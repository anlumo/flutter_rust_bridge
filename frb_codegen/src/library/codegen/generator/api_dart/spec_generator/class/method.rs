use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_function_dart_return_type,
};
use crate::codegen::ir::func::{
    IrFunc, IrFuncMode, IrFuncOwnerInfo, IrFuncOwnerInfoMethod, IrFuncOwnerInfoMethodMode,
};
use crate::codegen::ir::namespace::NamespacedName;
use crate::if_then_some;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use itertools::Itertools;

pub(crate) struct GeneratedApiMethods {
    pub methods: Vec<String>,
    pub has_default_dart_constructor: bool,
}

pub(crate) fn generate_api_methods(
    generalized_class_name: &NamespacedName,
    context: ApiDartGeneratorContext,
) -> GeneratedApiMethods {
    let infos = (context.ir_pack.funcs.iter())
        .filter(|f| {
            matches!(&f.owner, IrFuncOwnerInfo::Method(IrFuncOwnerInfoMethod{ enum_or_struct_name, .. }) if enum_or_struct_name == generalized_class_name)
        })
        .map(|func| generate_api_method(func, context))
        .collect_vec();

    let has_default_dart_constructor =
        (infos.iter()).any(|x| x.1 == Some(DefaultConstructorMode::DartConstructor));
    let methods = infos.into_iter().map(|x| x.0).collect_vec();
    GeneratedApiMethods {
        methods,
        has_default_dart_constructor,
    }
}

fn generate_api_method(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
) -> (String, Option<DefaultConstructorMode>) {
    let method_info =
        if_then_some!(let IrFuncOwnerInfo::Method(info) = &func.owner , info).unwrap();
    let is_static_method = method_info.mode == IrFuncOwnerInfoMethodMode::Static;

    let default_constructor_mode = DefaultConstructorMode::parse(func, method_info);

    // skip the first as it's the method 'self'
    let skip_count = usize::from(!is_static_method);

    let params = generate_params(func, context, skip_count);
    let comments = generate_comments(func, default_constructor_mode);
    let signature =
        generate_signature(func, context, method_info, params, default_constructor_mode);
    let arg_names = generate_arg_names(func, is_static_method, skip_count).concat();
    let implementation = generate_implementation(func, context, is_static_method, arg_names);

    (
        format!("{comments}{signature}=>{implementation};\n\n"),
        default_constructor_mode,
    )
}

fn generate_comments(
    func: &IrFunc,
    default_constructor_mode: Option<DefaultConstructorMode>,
) -> String {
    let mut ans = String::new();
    if default_constructor_mode == Some(DefaultConstructorMode::StaticMethod) {
        ans += "  // HINT: Make it `#[frb(sync)]` to let it become the default constructor of Dart class.\n";
    }
    ans += &generate_dart_comments(&func.comments);
    ans
}

fn generate_params(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    skip_count: usize,
) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .skip(skip_count)
        .map(|input| {
            let required = generate_field_required_modifier(input);
            let default = generate_field_default(input, false, context.config.dart_enums_style);
            let type_str = ApiDartGenerator::new(input.ty.clone(), context).dart_api_type();
            let name_str = input.name.dart_style();
            format!("{required}{type_str} {name_str} {default}")
        })
        .collect_vec();

    ans.push("dynamic hint".to_string());

    ans
}

fn generate_signature(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    method_info: &IrFuncOwnerInfoMethod,
    func_params: Vec<String>,
    default_constructor_mode: Option<DefaultConstructorMode>,
) -> String {
    let is_static_method = method_info.mode == IrFuncOwnerInfoMethodMode::Static;
    let maybe_static = if is_static_method { "static" } else { "" };
    let return_type = generate_function_dart_return_type(
        &func.mode,
        &ApiDartGenerator::new(func.output.clone(), context).dart_api_type(),
    );
    let method_name = if default_constructor_mode.is_some() {
        format!("newInstance")
    } else {
        method_info.actual_method_name.to_case(Case::Camel)
    };
    let (func_params, maybe_getter) = if func.getter {
        ("".to_owned(), "get")
    } else {
        (format!("({{ {} }})", func_params.join(",")), "")
    };

    if default_constructor_mode == Some(DefaultConstructorMode::DartConstructor) {
        return format!("factory {return_type}({func_params})");
    }

    format!("{maybe_static} {return_type} {maybe_getter} {method_name}{func_params}")
}

fn generate_arg_names(func: &IrFunc, is_static_method: bool, skip_count: usize) -> Vec<String> {
    let mut ans = func
        .inputs
        .iter()
        .skip(skip_count)
        .map(|input| format!("{}:{},", input.name.dart_style(), input.name.dart_style()))
        .collect_vec();
    if is_static_method {
        ans.push("hint: hint".to_string());
    }
    ans
}

fn generate_implementation(
    func: &IrFunc,
    context: ApiDartGeneratorContext,
    is_static_method: bool,
    arg_names: String,
) -> String {
    let dart_entrypoint_class_name = &context.config.dart_entrypoint_class_name;
    let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

    let func_name = func.name.name.clone().to_case(Case::Camel);

    if is_static_method {
        format!("{dart_api_instance}.{func_name}({arg_names})")
    } else {
        let extra_arg_name = func.inputs[0].name.dart_style();
        format!("{dart_api_instance}.{func_name}({extra_arg_name}: this, {arg_names})")
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
enum DefaultConstructorMode {
    DartConstructor,
    StaticMethod,
}

impl DefaultConstructorMode {
    fn parse(func: &IrFunc, method_info: &IrFuncOwnerInfoMethod) -> Option<Self> {
        if method_info.actual_method_name == "new" {
            if method_info.mode == IrFuncOwnerInfoMethodMode::Static
                && func.mode == IrFuncMode::Sync
            {
                Some(Self::DartConstructor)
            } else {
                Some(Self::StaticMethod)
            }
        } else {
            None
        }
    }
}
