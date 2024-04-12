use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFuncInput, IrFuncOwnerInfo};
use crate::codegen::ir::func::{IrFuncOwnerInfoMethod, OwnershipMode};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{FunctionParser, FunctionPartialInfo};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::rust_auto_opaque::split_ownership_from_ty;
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use crate::if_then_some;
use anyhow::{bail, ensure, Context};
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        sig_input: &FnArg,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let (ty_syn_raw, name) = match sig_input {
            FnArg::Typed(ref pat_type) => {
                (*pat_type.ty.clone(), parse_name_from_pat_type(pat_type)?)
            }
            FnArg::Receiver(ref receiver) => {
                let method = if_then_some!(let IrFuncOwnerInfo::Method(method) = owner, method)
                    .context("`self` must happen within methods")?;
                (syntheize_receiver_type(receiver, method), "that".to_owned())
            }
        };

        let (ty_syn_without_ownership, ownership_mode_raw) = split_ownership_from_ty(ty_syn_raw);

        let ty_without_ownership =
            (self.type_parser).parse_type(ty_syn_without_ownership, context)?;

        let (ty, ownership_mode) = match ty_without_ownership {
            IrType::RustAutoOpaque(ty_raw) => (
                self.type_parser.transform_rust_auto_opaque(
                    &ty_raw,
                    |raw| format!("{}{raw}", ownership_mode_raw.prefix()),
                    context,
                )?,
                None,
            ),
            _ => (ty_without_ownership, Some(ownership_mode_raw)),
        };

        if let IrType::StructRef(s) = &ty {
            if s.get(self.type_parser).ignore {
                return Ok(FunctionPartialInfo {
                    ignore_func: true,
                    ..Default::default()
                });
            }
        }

        let attributes = FrbAttributes::parse(attrs)?;
        let ty = auto_add_boxed(ty);
        Ok(FunctionPartialInfo {
            inputs: vec![IrFuncInput {
                inner: IrField {
                    name: IrIdent::new(name),
                    ty,
                    is_final: true,
                    comments: parse_comments(attrs),
                    default: attributes.default_value(),
                    settings: IrFieldSettings::default(),
                },
                ownership_mode,
            }],
            ..Default::default()
        })
    }
}

fn auto_add_boxed(ty: IrType) -> IrType {
    if ty.is_struct_or_enum_or_record() {
        Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty),
        })
    } else {
        ty
    }
}

fn parse_name_from_pat_type(pat_type: &PatType) -> anyhow::Result<String> {
    if_then_some!(let Pat::Ident(ref pat_ident) = *pat_type.pat, pat_ident)
        .map(|pat_ident| format!("{}", pat_ident.ident))
        .with_context(|| quote::quote!(#pat_type).to_string())
}

fn syntheize_receiver_type(receiver: &Receiver, method: &IrFuncOwnerInfoMethod) -> Type {
    let ty_str = format!(
        "{}{}",
        parse_receiver_ownership_mode(receiver).prefix(),
        method.owner_ty_name().name.to_owned()
    );
    parse_str::<Type>(&ty_str)
}

fn parse_receiver_ownership_mode(receiver: &Receiver) -> OwnershipMode {
    if receiver.reference.is_some() {
        if receiver.mutability.is_some() {
            OwnershipMode::RefMut
        } else {
            OwnershipMode::Ref
        }
    } else {
        OwnershipMode::Owned
    }
}
