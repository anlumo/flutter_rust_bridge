use crate::codegen::generator::wire::rust::base::*;
use crate::library::codegen::ir::ty::IrTypeTrait;

mod boxed;
mod dart_opaque;
mod delegate;
mod dynamic;
mod enumeration;
mod general_list;
mod optional;
mod optional_list;
mod primitive;
mod primitive_list;
mod record;
mod rust_opaque;
mod structure;
mod unencodable;

use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::IrType;
use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub(crate) trait WireRustGeneratorCommonTrait {
    fn wrapper_struct_name(&self) -> Option<String> {
        None
    }
}
