use crate::generator::dart::ty::*;
use crate::ir::*;
use crate::target::{Acc, Target};
use crate::type_dart_generator_struct;
use crate::utils::misc::ShareMode;

type_dart_generator_struct!(TypeOptionalGenerator, IrTypeOptional);

impl TypeDartGeneratorTrait for TypeOptionalGenerator<'_> {
    fn api2wire_body(
        &self,
        _shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Acc<Option<String>> {
        let prefix = if !self.context.ir_file.shared
            && self
                .context
                .ir_file
                .is_type_shared_by_safe_ident(&self.ir.inner)
                == ShareMode::Shared
        {
            "_sharedPlatform.api2wire"
        } else {
            "api2wire"
        };
        Acc::new(|target| match target {
            Target::Io | Target::Wasm => Some(format!(
                "return raw == null ? {} : {prefix}_{}(raw);",
                if target.is_wasm() {
                    "null"
                } else {
                    "ffi.nullptr"
                },
                self.ir.inner.safe_ident()
            )),
            _ => None,
        })
    }

    fn api_fill_to_wire_body(
        &self,
        _shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Option<String> {
        if !self.ir.needs_initialization() || self.ir.is_list() || self.ir.is_boxed_primitive() {
            return None;
        }

        let prefix = if self.context.config.shared {
            ""
        } else {
            match self
                .context
                .ir_file
                .is_type_shared_by_safe_ident(&self.ir.inner)
            {
                ShareMode::Unique => "_",
                ShareMode::Shared => "_sharedPlatform.",
            }
        };

        Some(format!(
            "if (apiObj != null) {}api_fill_to_wire_{}(apiObj, wireObj);",
            prefix,
            self.ir.inner.safe_ident()
        ))
    }

    fn wire2api_body(&self) -> String {
        let use_shared_instance = !self.context.ir_file.shared
            && self
                .context
                .ir_file
                .is_type_shared_by_safe_ident(&self.ir.inner)
                == ShareMode::Shared;

        if use_shared_instance {
            format!(
                "return raw == null ? null : _sharedImpl.wire2api_{}(raw);",
                self.ir.inner.safe_ident()
            )
        } else {
            let prefix = if self.context.config.shared { "" } else { "_" };
            format!(
                "return raw == null ? null : {}wire2api_{}(raw);",
                prefix,
                self.ir.inner.safe_ident()
            )
        }
    }
}
