use crate::codegen::generator::wire::dart::spec_generator::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::misc::gen_wire2api_simple_type_cast;
use crate::codegen::generator::wire::dart::spec_generator::transfer::dco::decoder::ty::WireDartTransferDcoGeneratorDecoderTrait;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::IrType::*;
use crate::library::codegen::ir::ty::IrTypeTrait;

impl<'a> WireDartTransferDcoGeneratorDecoderTrait for BoxedWireDartTransferDcoGenerator<'a> {
    fn generate_impl_wire2api_body(&self) -> String {
        match &*self.ir.inner {
            StructRef(_)
            | DartOpaque(_)
            | RustOpaque(_)
            | EnumRef(_)
            | Primitive(IrTypePrimitive::I64 | IrTypePrimitive::U64 | IrTypePrimitive::Usize)
            | Delegate(IrTypeDelegate::Array(_) | IrTypeDelegate::PrimitiveEnum { .. }) => {
                format!("return _wire2api_{}(raw);", self.ir.inner.safe_ident())
            }
            // TODO merge with above
            Delegate(IrTypeDelegate::Time(time)) => {
                format!("return _wire2api_Chrono_{}(raw);", time)
            }
            _ => gen_wire2api_simple_type_cast(self.ir.clone().into(), self.context),
        }
    }
}
