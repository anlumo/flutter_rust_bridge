// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 1.82.4.

#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]

// Section: imports

use flutter_rust_bridge::{Handler, IntoIntoDart};

// Section: executor

#[cfg(not(target_family = "wasm"))]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER:
    flutter_rust_bridge::DefaultHandler<flutter_rust_bridge::for_generated::SimpleThreadPool>
    = flutter_rust_bridge::DefaultHandler::new_simple(Default::default());
}

#[cfg(target_family = "wasm")]
thread_local! {
    pub static THREAD_POOL: flutter_rust_bridge::for_generated::SimpleThreadPool = Default::default();
}

#[cfg(target_family = "wasm")]
flutter_rust_bridge::for_generated::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER:
    flutter_rust_bridge::DefaultHandler<&'static std::thread::LocalKey<flutter_rust_bridge::for_generated::SimpleThreadPool>>
    = flutter_rust_bridge::DefaultHandler::new_simple(&THREAD_POOL);
}

// Section: wire_funcs

fn wire_hello_impl(port_: i64, ptr_: *const u8, rust_vec_len_: i32, data_len_: i32) {
    FLUTTER_RUST_BRIDGE_HANDLER
        .wrap_normal::<flutter_rust_bridge::for_generated::SseCodec, _, _, _, i32, _>(
            flutter_rust_bridge::for_generated::TaskInfo {
                debug_name: "hello",
                port: Some(port_),
                mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
            },
            move || {
                TODO_generate_func_call_decode;
                move |context| Result::<_, ()>::Ok(crate::api::minimal::hello(api_a, api_b))
            },
        )
}
fn wire_minimal_adder_impl(
    port_: flutter_rust_bridge::for_generated::MessagePort,
    a: impl CstDecode<i32> + core::panic::UnwindSafe,
    b: impl CstDecode<i32> + core::panic::UnwindSafe,
) {
    FLUTTER_RUST_BRIDGE_HANDLER
        .wrap_normal::<flutter_rust_bridge::for_generated::DcoCodec, _, _, _, i32, _>(
            flutter_rust_bridge::for_generated::TaskInfo {
                debug_name: "minimal_adder",
                port: Some(port_),
                mode: flutter_rust_bridge::for_generated::FfiCallMode::Normal,
            },
            move || {
                let api_a = a.cst_decode();
                let api_b = b.cst_decode();
                move |context| Result::<_, ()>::Ok(crate::api::minimal::minimal_adder(api_a, api_b))
            },
        )
}

// Section: dart2rust

pub trait CstDecode<T> {
    fn cst_decode(self) -> T;
}

impl<T, S> CstDecode<Option<T>> for *mut S
where
    *mut S: CstDecode<T>,
{
    fn cst_decode(self) -> Option<T> {
        (!self.is_null()).then(|| self.cst_decode())
    }
}
impl CstDecode<i32> for i32 {
    fn cst_decode(self) -> i32 {
        self
    }
}
pub trait SseDecode {
    fn sse_decode(deserializer: SseDeserializer) -> T;
}

impl SseDecode for i32 {
    fn sse_decode(deserializer: SseDeserializer) -> Self {
        return deserializer.buffer.getInt32();
    }
}

// Section: rust2dart

pub trait SseEncode {
    fn sse_encode(self, serializer: SseSerializer);
}

impl SseEncode for i32 {
    fn sse_encode(self, serializer: SseSerializer) {
        serializer.buffer.putInt32(self);
    }
}

#[cfg(not(target_family = "wasm"))]
#[path = "frb_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "frb_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;
