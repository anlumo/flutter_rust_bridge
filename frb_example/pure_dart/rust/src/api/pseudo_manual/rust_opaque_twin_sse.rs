// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `rust_opaque.rs` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// FRB_INTERNAL_GENERATOR: {"enableAll": true}

pub use crate::auxiliary::sample_types::{HideDataRaw, NonCloneDataRaw, NonSendHideDataRaw};
use anyhow::Result;
use flutter_rust_bridge::{opaque_dyn, RustOpaque};
use std::fmt::Debug;
use std::ops::Deref;
pub use std::sync::{Mutex, RwLock};

pub struct HideDataTwinSse(HideDataRaw);
pub struct NonCloneDataTwinSse(NonCloneDataRaw);
pub struct NonSendHideDataTwinSse(NonSendHideDataRaw);

/// Structure for testing the RustOpaque code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
pub struct FrbOpaqueReturnTwinSse;

/// Opaque types
pub trait DartDebugTwinSse: Debug + Send + Sync {}
impl<T: Debug + Send + Sync> DartDebugTwinSse for T {}

pub enum EnumOpaqueTwinSse {
    Struct(crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>),
    Primitive(crate::frb_generated::RustOpaqueMoi<i32>),
    TraitObj(crate::frb_generated::RustOpaqueMoi<Box<dyn DartDebugTwinSse>>),
    Mutex(crate::frb_generated::RustOpaqueMoi<Mutex<HideDataTwinSse>>),
    RwLock(crate::frb_generated::RustOpaqueMoi<RwLock<HideDataTwinSse>>),
}

/// [`HideDataTwinSse`] has private fields.
pub struct OpaqueNestedTwinSse {
    pub first: crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>,
    pub second: crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>,
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_opaque_twin_sse() -> crate::frb_generated::RustOpaqueMoi<HideDataTwinSse> {
    RustOpaque::new(HideDataTwinSse::new())
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_option_opaque_twin_sse(
    opaque: Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>>,
) -> Option<crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>> {
    opaque
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_opaque_twin_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>> {
//     SyncReturn(RustOpaque::new(HideDataTwinSse::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn create_array_opaque_enum_twin_sse() -> [EnumOpaqueTwinSse; 5] {
    [
        EnumOpaqueTwinSse::Struct(RustOpaque::new(HideDataTwinSse::new())),
        EnumOpaqueTwinSse::Primitive(RustOpaque::new(42)),
        EnumOpaqueTwinSse::TraitObj(opaque_dyn!("String")),
        EnumOpaqueTwinSse::Mutex(RustOpaque::new(Mutex::new(HideDataTwinSse::new()))),
        EnumOpaqueTwinSse::RwLock(RustOpaque::new(RwLock::new(HideDataTwinSse::new()))),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_enum_opaque_twin_sse(opaque: EnumOpaqueTwinSse) -> String {
    match opaque {
        EnumOpaqueTwinSse::Struct(s) => s.hide_data(),
        EnumOpaqueTwinSse::Primitive(p) => format!("{:?}", p.deref()),
        EnumOpaqueTwinSse::TraitObj(t) => format!("{:?}", t.deref()),
        EnumOpaqueTwinSse::Mutex(m) => {
            format!("{:?}", m.lock().unwrap().hide_data())
        }
        EnumOpaqueTwinSse::RwLock(r) => {
            format!("{:?}", r.read().unwrap().hide_data())
        }
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_twin_sse(opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>) -> String {
    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_opaque_with_delay_twin_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>,
) -> String {
    // If WASM + main thread (i.e. "sync"), the `sleep` cannot be used, which is a Rust / WASM limit.
    // (But if on native, or on WASM + async mode, it is OK)
    #[cfg(not(target_family = "wasm"))]
    std::thread::sleep(std::time::Duration::from_millis(1000));

    opaque.hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_twin_sse() -> [crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>; 2] {
    [
        RustOpaque::new(HideDataTwinSse::new()),
        RustOpaque::new(HideDataTwinSse::new()),
    ]
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_non_clone_twin_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSse>> {
//     SyncReturn(RustOpaque::new(NonCloneDataTwinSse::new()))
// }

#[allow(clippy::redundant_clone)]
#[flutter_rust_bridge::frb(serialize)]
pub fn run_non_clone_twin_sse(
    clone: crate::frb_generated::RustOpaqueMoi<NonCloneDataTwinSse>,
) -> String {
    // Tests whether `.clone()` works even without the generic type wrapped by it
    // implementing Clone.
    clone.clone().hide_data()
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_sync_opaque_twin_sse() -> crate::frb_generated::RustOpaqueMoi<NonSendHideDataTwinSse>
{
    RustOpaque::new(NonSendHideDataTwinSse::new())
}

// TODO about sync
// #[flutter_rust_bridge::frb(serialize)] pub fn sync_create_sync_opaque_twin_sse() -> SyncReturn<crate::frb_generated::RustOpaqueMoi<NonSendHideDataTwinSse>> {
//     SyncReturn(RustOpaque::new(NonSendHideDataTwinSse::new()))
// }

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_array_run_twin_sse(data: [crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>; 2]) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_twin_sse() -> Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>> {
    vec![
        RustOpaque::new(HideDataTwinSse::new()),
        RustOpaque::new(HideDataTwinSse::new()),
    ]
}

#[flutter_rust_bridge::frb(serialize)]
pub fn opaque_vec_run_twin_sse(data: Vec<crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>>) {
    for i in data {
        i.hide_data();
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn create_nested_opaque_twin_sse() -> OpaqueNestedTwinSse {
    OpaqueNestedTwinSse {
        first: RustOpaque::new(HideDataTwinSse::new()),
        second: RustOpaque::new(HideDataTwinSse::new()),
    }
}

#[flutter_rust_bridge::frb(serialize)]
pub fn run_nested_opaque_twin_sse(opaque: OpaqueNestedTwinSse) {
    opaque.first.hide_data();
    opaque.second.hide_data();
}

#[flutter_rust_bridge::frb(serialize)]
pub fn unwrap_rust_opaque_twin_sse(
    opaque: crate::frb_generated::RustOpaqueMoi<HideDataTwinSse>,
) -> Result<String> {
    let data: HideDataTwinSse = opaque
        .try_unwrap()
        .map_err(|_| anyhow::anyhow!("opaque type is shared"))?;
    Ok(data.hide_data())
}

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
#[flutter_rust_bridge::frb(serialize)]
pub fn frb_generator_test_twin_sse() -> crate::frb_generated::RustOpaqueMoi<FrbOpaqueReturnTwinSse>
{
    panic!("dummy code");
}
