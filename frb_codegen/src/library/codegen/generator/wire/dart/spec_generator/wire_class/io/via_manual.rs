use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::generator::wire::dart::spec_generator::wire_class::io::common::generate_wire_class_header;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::utils::basic_code::DartBasicHeaderCode;
use itertools::Itertools;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    rust_extern_funcs: &[ExternFunc],
) -> anyhow::Result<WireDartOutputCode> {
    let wire_class_name = &config.dart_output_class_name_pack.wire_class_name;
    let wire_class_header = generate_wire_class_header(wire_class_name);
    let class_body = rust_extern_funcs.iter().map(|f| generate_func(f)).join("");

    let body = format!(
        "
        {wire_class_header}
            {wire_class_name}(ffi.DynamicLibrary dynamicLibrary);
            {class_body}
        }}
        "
    );
    Ok(WireDartOutputCode {
        header: DartBasicHeaderCode {
            import: "import 'dart:ffi' as ffi;\n".to_owned(),
            ..Default::default()
        },
        body,
        ..Default::default()
    })
}

fn generate_func(func: &ExternFunc) -> String {
    // Only know how to generate this currently
    if !func.needs_ffigen
        && func.return_type.is_none()
        && func.params.len() == 1
        && func.params[0]
            == (ExternFuncParam {
                name: "ptr".to_string(),
                rust_type: "*const std::ffi::c_void".to_owned(),
                dart_type: "dynamic".to_string(),
            })
    {
        let name = &func.partial_func_name;
        format!(
            "
            void {name}(
              ffi.Pointer<ffi.Void> ptr,
            ) {{
              return _{name}(
                ptr,
              );
            }}

            late final _{name}Ptr = _lookup<ffi.NativeFunction<ffi.Void Function(ffi.Pointer<ffi.Void>)>>('{name}');
            late final _{name} = _{name}Ptr.asFunction<void Function(ffi.Pointer<ffi.Void>)>();
            "
        )
    } else {
        unreachable!(
            "Do not understand how to generate this func without ffigen yet (func={func:?})"
        )
    }
}
