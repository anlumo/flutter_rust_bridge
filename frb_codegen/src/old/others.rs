use std::fs;
use std::ops::Add;
use std::path::Path;

use anyhow::{anyhow, Context, Result};
use lazy_static::lazy_static;
use log::{info, warn};
use pathdiff::diff_paths;

lazy_static! {
    pub static ref EXTRA_EXTERN_FUNC_NAMES: Vec<String> = vec![
        "store_dart_post_cobject".to_owned(),
        "get_dart_object".to_owned(),
        "drop_dart_object".to_owned(),
        "new_dart_opaque".to_owned()
    ];
}

pub fn modify_dart_wire_content(content_raw: &str, dart_wire_class_name: &str) -> String {
    let content = content_raw.replace(
        &format!("class {dart_wire_class_name} {{",),
        &format!(
            "class {dart_wire_class_name} implements FlutterRustBridgeWireBase {{
            @internal
            late final dartApi = DartApiDl(init_frb_dart_api_dl);",
        ),
    );

    content
        .replace("final class DartCObject extends ffi.Opaque {}", "")
        /*.replace(
            "class _Dart_Handle extends ffi.Opaque {}",
            "base class _Dart_Handle extends ffi.Opaque {}",
        )*/
        .replace("typedef WireSyncReturn = ffi.Pointer<DartCObject>;", "")
}

#[derive(Default)]
pub struct DartBasicCode {
    pub import: String,
    pub part: String,
    pub body: String,
}

impl Add for &DartBasicCode {
    type Output = DartBasicCode;

    fn add(self, rhs: Self) -> Self::Output {
        DartBasicCode {
            import: format!("{}\n{}", self.import, rhs.import),
            part: format!("{}\n{}", self.part, rhs.part),
            body: format!("{}\n{}", self.body, rhs.body),
        }
    }
}

impl Add<&DartBasicCode> for DartBasicCode {
    type Output = DartBasicCode;

    fn add(self, rhs: &DartBasicCode) -> Self::Output {
        (&self).add(rhs)
    }
}

impl DartBasicCode {
    pub fn to_text(&self) -> String {
        format!("{}\n{}\n{}", self.import, self.part, self.body)
    }
}

pub fn extract_dart_wire_content(content: &str) -> DartBasicCode {
    let (mut imports, mut body) = (Vec::new(), Vec::new());
    for line in content.split('\n') {
        (if line.starts_with("import ") {
            &mut imports
        } else {
            &mut body
        })
        .push(line);
    }
    DartBasicCode {
        import: imports.join("\n"),
        part: "".to_string(),
        body: body.join("\n"),
    }
}

pub fn sanity_check(generated_dart_wire_code: &str, dart_wire_class_name: &str) -> crate::Result {
    if !generated_dart_wire_code.contains(dart_wire_class_name) {
        bail!(
            "Nothing is generated for dart wire class. \
            Maybe you forget to put code like `mod the_generated_bridge_code;` to your `lib.rs`?",
        );
    }
    Ok(())
}
