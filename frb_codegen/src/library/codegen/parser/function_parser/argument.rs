use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::IrFuncMode;
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::IrType;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{
    type_to_string, FunctionParser, FunctionPartialInfo, STREAM_SINK_IDENT,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::ParserResult;
use anyhow::Context;
use syn::*;

/// Represents the type of an argument to a function
#[derive(Debug, Clone)]
pub(super) enum FuncArg {
    StreamSinkType(IrType),
    Type(IrType),
}

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        i: usize,
        sig_input: &FnArg,
    ) -> ParserResult<FunctionPartialInfo> {
        Ok(if let FnArg::Typed(ref pat_type) = sig_input {
            let name = if let Pat::Ident(ref pat_ident) = *pat_type.pat {
                format!("{}", pat_ident.ident)
            } else {
                return Err(super::error::Error::UnexpectedPattern(
                    quote::quote!(#pat_type).to_string().into(),
                ));
            };
            let arg_type = self.parse_fn_arg_type(&pat_type.ty)?.with_context(|| {
                format!(
                    "Failed to parse function argument type `{}`",
                    type_to_string(&pat_type.ty)
                )
            })?;
            match arg_type {
                FuncArg::StreamSinkType(ty) => {
                    output = Some(ty);
                    mode = Some(IrFuncMode::Stream { argument_index: i });
                    fallible = match &sig.output {
                        ReturnType::Default => false,
                        ReturnType::Type(_, ty) => {
                            !matches!(self.parse_fn_output_type(ty)?, Some(FuncOutput::Type(_)))
                        }
                    }
                }
                FuncArg::Type(ty) => {
                    let attributes = FrbAttributes::parse(&pat_type.attrs)?;
                    inputs.push(IrField {
                        name: IrIdent::new(name),
                        ty,
                        is_final: true,
                        comments: parse_comments(&pat_type.attrs),
                        default: attributes.default_value(),
                        settings: IrFieldSettings::default(),
                    });
                }
            }
        } else {
            return Err(super::error::Error::UnexpectedSigInput(
                quote::quote!(#sig_input).to_string().into(),
            ));
        })
    }

    /// Attempts to parse the type from an argument of a function signature. There is a special
    /// case for top-level `StreamSink` types.
    fn parse_fn_arg_type(&mut self, ty: &Type) -> anyhow::Result<Option<FuncArg>> {
        Ok(match ty {
            Type::Path(TypePath { path, .. }) => {
                if let Some(ans) = self.parse_fn_arg_type_stream_sink(path)? {
                    Some(ans)
                } else {
                    Some(FuncArg::Type(self.type_parser.parse_type(ty)?))
                }
            }
            Type::Array(_) => Some(FuncArg::Type(self.type_parser.parse_type(ty)?)),
            _ => None,
        })
    }

    fn parse_fn_arg_type_stream_sink(&mut self, path: &Path) -> anyhow::Result<Option<FuncArg>> {
        let last_segment = path.segments.last().unwrap();
        Ok(if last_segment.ident == STREAM_SINK_IDENT {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if args.len() == 1 =>
                {
                    // Unwrap is safe here because args.len() == 1
                    match args.last().unwrap() {
                        GenericArgument::Type(t) => {
                            Some(FuncArg::StreamSinkType(self.type_parser.parse_type(t)?))
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        })
    }
}
