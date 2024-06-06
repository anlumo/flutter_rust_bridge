use crate::codegen::generator::api_dart::spec_generator::class::method::{
    generate_api_methods, GenerateApiMethodConfig,
};
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for TraitDefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_api_type = &self.mir.name.name;
        let methods = generate_api_methods(
            &self.mir.name,
            self.context,
            &GenerateApiMethodConfig {
                generate_static: false,
                generate_non_static: true,
                generate_impl: false,
            },
            dart_api_type,
        );
        let methods_str = &methods.code;

        Some(ApiDartGeneratedClass {
            namespace: self.mir.name.namespace.clone(),
            class_name: self.mir.name.name.clone(),
            code: format!(
                "
                abstract class {dart_api_type} {{
                    {methods_str}
                }}
                "
            ),
            needs_freezed: false,
            header: Default::default(),
        })
    }
}
