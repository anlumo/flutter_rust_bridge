// Section: imports

use super::*;
use crate::api::pseudo_manual::rust_opaque_twin_sync::*;
use crate::api::rust_opaque::*;
use crate::api::rust_opaque_sync::*;
use flutter_rust_bridge::rust2dart::IntoIntoDart;
use flutter_rust_bridge::wasm_bindgen;
use flutter_rust_bridge::wasm_bindgen::prelude::*;
use flutter_rust_bridge::Handler;

// Section: impl_wire2api

impl<T> Wire2Api<Option<T>> for flutter_rust_bridge::wasm_bindgen::JsValue
where
    JsValue: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null() && !self.is_undefined()).then(|| self.wire2api())
    }
}
impl Wire2Api<chrono::Duration> for i64 {
    fn wire2api(self) -> chrono::Duration {
        chrono::Duration::milliseconds(self)
    }
}
impl Wire2Api<Vec<chrono::Duration>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::Duration> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for Box<[i64]> {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        let vec: Vec<i64> = self.wire2api();
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<flutter_rust_bridge::DartOpaque> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> flutter_rust_bridge::DartOpaque {
        let arr = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        unsafe { flutter_rust_bridge::DartOpaque::new(arr.get(0), arr.get(1)) }
    }
}
impl Wire2Api<String> for String {
    fn wire2api(self) -> String {
        self
    }
}
impl Wire2Api<Vec<String>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<String> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<uuid::Uuid> for Box<[u8]> {
    fn wire2api(self) -> uuid::Uuid {
        let single: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuid_ref(single.as_slice())
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for Box<[u8]> {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        let multiple: Vec<u8> = self.wire2api();
        flutter_rust_bridge::wire2api_uuids(multiple)
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> for Box<[u8]> {
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::ATwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::ATwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::ATwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::ATwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::AbcTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::AbcTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::misc_example::AbcTwinNormal::A(self_.get(1).wire2api()),
            1 => crate::api::misc_example::AbcTwinNormal::B(self_.get(1).wire2api()),
            2 => crate::api::misc_example::AbcTwinNormal::C(self_.get(1).wire2api()),
            3 => crate::api::misc_example::AbcTwinNormal::JustInt(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::A(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::B(
                self_.get(1).wire2api(),
            ),
            2 => crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::C(
                self_.get(1).wire2api(),
            ),
            3 => crate::api::pseudo_manual::misc_example_twin_sync::AbcTwinSync::JustInt(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv {
            vars: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::mirror_twin_sync::ApplicationSettings {
            name: self_.get(0).wire2api(),
            version: self_.get(1).wire2api(),
            mode: self_.get(2).wire2api(),
            env: self_.get(3).wire2api(),
            env_optional: self_.get(4).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::AttributeTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::optional::AttributeTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::optional::AttributeTwinNormal {
            key: self_.get(0).wire2api(),
            value: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync {
            key: self_.get(0).wire2api(),
            value: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::BTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::BTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::BTwinNormal {
            b: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::BTwinSync {
            b: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::BlobTwinNormal> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> crate::api::array::BlobTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::BlobTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::BlobTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::array_twin_sync::BlobTwinSync(self_.get(0).wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for Box<[u8]> {
    fn wire2api(self) -> Box<[u8; 1600]> {
        Wire2Api::<[u8; 1600]>::wire2api(self).into()
    }
}
impl Wire2Api<crate::api::misc_example::CTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::CTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::CTwinNormal {
            c: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::CTwinSync {
            c: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::method::ConcatenateWithTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::method::ConcatenateWithTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::method::ConcatenateWithTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::method_twin_sync::ConcatenateWithTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorInnerTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorInnerTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::exception::CustomNestedErrorInnerTwinNormal::Three(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::exception::CustomNestedErrorInnerTwinNormal::Four(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
                    0 => { crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Three( self_.get(1).wire2api()) },
1 => { crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorInnerTwinSync::Four( self_.get(1).wire2api()) },
                    _ => unreachable!(),
                }
    }
}
impl Wire2Api<crate::api::exception::CustomNestedErrorOuterTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::exception::CustomNestedErrorOuterTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::exception::CustomNestedErrorOuterTwinNormal::One(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::exception::CustomNestedErrorOuterTwinNormal::Two(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => {
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::One(
                    self_.get(1).wire2api(),
                )
            }
            1 => {
                crate::api::pseudo_manual::exception_twin_sync::CustomNestedErrorOuterTwinSync::Two(
                    self_.get(1).wire2api(),
                )
            }
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructErrorTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::exception::CustomStructErrorTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::exception::CustomStructErrorTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::exception_twin_sync::CustomStructErrorTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::exception::CustomStructTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::exception::CustomStructTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::exception::CustomStructTwinNormal {
            message: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::exception_twin_sync::CustomStructTwinSync {
            message: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::attribute::CustomizedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::attribute::CustomizedTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::attribute::CustomizedTwinNormal {
            final_field: self_.get(0).wire2api(),
            non_final_field: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::attribute_twin_sync::CustomizedTwinSync {
            final_field: self_.get(0).wire2api(),
            non_final_field: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::dart_opaque::DartOpaqueNestedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::dart_opaque::DartOpaqueNestedTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::dart_opaque::DartOpaqueNestedTwinNormal {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::dart_opaque_twin_sync::DartOpaqueNestedTwinSync {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::DistanceTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::DistanceTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::DistanceTwinNormal::Unknown,
            1 => crate::api::enumeration::DistanceTwinNormal::Map(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync::Unknown,
            1 => crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync::Map(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::misc_type::EmptyTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_type::EmptyTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::misc_type::EmptyTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_type_twin_sync::EmptyTwinSync {}
    }
}
impl Wire2Api<crate::api::dart_opaque::EnumDartOpaqueTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::dart_opaque::EnumDartOpaqueTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::dart_opaque::EnumDartOpaqueTwinNormal::Primitive(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::dart_opaque::EnumDartOpaqueTwinNormal::Opaque(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => {
                crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync::Primitive(
                    self_.get(1).wire2api(),
                )
            }
            1 => crate::api::pseudo_manual::dart_opaque_twin_sync::EnumDartOpaqueTwinSync::Opaque(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::rust_opaque::EnumOpaqueTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::rust_opaque::EnumOpaqueTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::rust_opaque::EnumOpaqueTwinNormal::Struct(self_.get(1).wire2api()),
            1 => crate::api::rust_opaque::EnumOpaqueTwinNormal::Primitive(self_.get(1).wire2api()),
            2 => crate::api::rust_opaque::EnumOpaqueTwinNormal::TraitObj(self_.get(1).wire2api()),
            3 => crate::api::rust_opaque::EnumOpaqueTwinNormal::Mutex(self_.get(1).wire2api()),
            4 => crate::api::rust_opaque::EnumOpaqueTwinNormal::RwLock(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Struct(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Primitive(
                self_.get(1).wire2api(),
            ),
            2 => crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::TraitObj(
                self_.get(1).wire2api(),
            ),
            3 => crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::Mutex(
                self_.get(1).wire2api(),
            ),
            4 => crate::api::pseudo_manual::rust_opaque_twin_sync::EnumOpaqueTwinSync::RwLock(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemMixedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemMixedTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemMixedTwinNormal::A,
            1 => crate::api::enumeration::EnumWithItemMixedTwinNormal::B(self_.get(1).wire2api()),
            2 => crate::api::enumeration::EnumWithItemMixedTwinNormal::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::A,
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::B(
                self_.get(1).wire2api(),
            ),
            2 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemMixedTwinSync::C {
                c_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemStructTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemStructTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemStructTwinNormal::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => crate::api::enumeration::EnumWithItemStructTwinNormal::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::A {
                a_field: self_.get(1).wire2api(),
            },
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemStructTwinSync::B {
                b_field: self_.get(1).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::EnumWithItemTupleTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::EnumWithItemTupleTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::EnumWithItemTupleTwinNormal::A(self_.get(1).wire2api()),
            1 => crate::api::enumeration::EnumWithItemTupleTwinNormal::B(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::A(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::pseudo_manual::enumeration_twin_sync::EnumWithItemTupleTwinSync::B(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::event_listener::EventTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::event_listener::EventTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::event_listener::EventTwinNormal {
            address: self_.get(0).wire2api(),
            payload: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::event_listener_twin_sync::EventTwinSync {
            address: self_.get(0).wire2api(),
            payload: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::optional::ExoticOptionalsTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::optional::ExoticOptionalsTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            14,
            "Expected 14 elements, got {}",
            self_.length()
        );
        crate::api::optional::ExoticOptionalsTwinNormal {
            int32: self_.get(0).wire2api(),
            int64: self_.get(1).wire2api(),
            float64: self_.get(2).wire2api(),
            boolean: self_.get(3).wire2api(),
            zerocopy: self_.get(4).wire2api(),
            int8list: self_.get(5).wire2api(),
            uint8list: self_.get(6).wire2api(),
            int32list: self_.get(7).wire2api(),
            float32list: self_.get(8).wire2api(),
            float64list: self_.get(9).wire2api(),
            attributes: self_.get(10).wire2api(),
            attributes_nullable: self_.get(11).wire2api(),
            nullable_attributes: self_.get(12).wire2api(),
            newtypeint: self_.get(13).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            14,
            "Expected 14 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync {
            int32: self_.get(0).wire2api(),
            int64: self_.get(1).wire2api(),
            float64: self_.get(2).wire2api(),
            boolean: self_.get(3).wire2api(),
            zerocopy: self_.get(4).wire2api(),
            int8list: self_.get(5).wire2api(),
            uint8list: self_.get(6).wire2api(),
            int32list: self_.get(7).wire2api(),
            float32list: self_.get(8).wire2api(),
            float64list: self_.get(9).wire2api(),
            attributes: self_.get(10).wire2api(),
            attributes_nullable: self_.get(11).wire2api(),
            nullable_attributes: self_.get(12).wire2api(),
            newtypeint: self_.get(13).wire2api(),
        }
    }
}
impl Wire2Api<[f64; 16]> for Box<[f64]> {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::chrono_type::FeatureChronoTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::chrono_type::FeatureChronoTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::chrono_type::FeatureChronoTwinNormal {
            utc: self_.get(0).wire2api(),
            local: self_.get(1).wire2api(),
            duration: self_.get(2).wire2api(),
            naive: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::chrono_type_twin_sync::FeatureChronoTwinSync {
            utc: self_.get(0).wire2api(),
            local: self_.get(1).wire2api(),
            duration: self_.get(2).wire2api(),
            naive: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::uuid_type::FeatureUuidTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::uuid_type::FeatureUuidTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::uuid_type::FeatureUuidTwinNormal {
            one: self_.get(0).wire2api(),
            many: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::uuid_type_twin_sync::FeatureUuidTwinSync {
            one: self_.get(0).wire2api(),
            many: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::FeedIdTwinNormal> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> crate::api::array::FeedIdTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::FeedIdTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::array_twin_sync::FeedIdTwinSync(self_.get(0).wire2api())
    }
}
impl Wire2Api<[i32; 2]> for Box<[i32]> {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::enumeration::KitchenSinkTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::KitchenSinkTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::KitchenSinkTwinNormal::Empty,
            1 => crate::api::enumeration::KitchenSinkTwinNormal::Primitives {
                int32: self_.get(1).wire2api(),
                float64: self_.get(2).wire2api(),
                boolean: self_.get(3).wire2api(),
            },
            2 => crate::api::enumeration::KitchenSinkTwinNormal::Nested(
                self_.get(1).wire2api(),
                self_.get(2).wire2api(),
            ),
            3 => crate::api::enumeration::KitchenSinkTwinNormal::Optional(
                self_.get(1).wire2api(),
                self_.get(2).wire2api(),
            ),
            4 => crate::api::enumeration::KitchenSinkTwinNormal::Buffer(self_.get(1).wire2api()),
            5 => crate::api::enumeration::KitchenSinkTwinNormal::Enums(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Empty,
            1 => {
                crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Primitives {
                    int32: self_.get(1).wire2api(),
                    float64: self_.get(2).wire2api(),
                    boolean: self_.get(3).wire2api(),
                }
            }
            2 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Nested(
                self_.get(1).wire2api(),
                self_.get(2).wire2api(),
            ),
            3 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Optional(
                self_.get(1).wire2api(),
                self_.get(2).wire2api(),
            ),
            4 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Buffer(
                self_.get(1).wire2api(),
            ),
            5 => crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync::Enums(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::DartOpaque>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<flutter_rust_bridge::DartOpaque> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnvVar> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::optional::AttributeTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::optional::AttributeTwinNormal> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<bool>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<bool> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::auxiliary::sample_types::MySize>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::auxiliary::sample_types::MySize> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::MyTreeNodeTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::misc_example::MyTreeNodeTwinNormal> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Vec<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<String>>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<Option<String>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::optional::AttributeTwinNormal>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<Option<crate::api::optional::AttributeTwinNormal>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Vec<Option<crate::api::pseudo_manual::optional_twin_sync::AttributeTwinSync>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<i32>>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<Option<i32>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::misc_example::WeekdaysTwinNormal>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<Option<crate::api::misc_example::WeekdaysTwinNormal>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Vec<Option<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<Option<Vec<i32>>>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<Option<Vec<i32>>> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i16>> for Box<[i16]> {
    fn wire2api(self) -> Vec<i16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i64>> for Box<[i64]> {
    fn wire2api(self) -> Vec<i64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<i8>> for Box<[i8]> {
    fn wire2api(self) -> Vec<i8> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u16>> for Box<[u16]> {
    fn wire2api(self) -> Vec<u16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u64>> for Box<[u64]> {
    fn wire2api(self) -> Vec<u64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u8>> for Box<[u8]> {
    fn wire2api(self) -> Vec<u8> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<(String, i32)>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<(String, i32)> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::array::TestIdTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::array::TestIdTwinNormal> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::misc_example::WeekdaysTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::misc_example::WeekdaysTwinNormal> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Vec<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync> {
        self.dyn_into::<flutter_rust_bridge::JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<crate::api::inside_macro::MacroStruct>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::inside_macro::MacroStruct {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::inside_macro::MacroStruct {
            data: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::MeasureTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::MeasureTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::MeasureTwinNormal::Speed(self_.get(1).wire2api()),
            1 => crate::api::enumeration::MeasureTwinNormal::Distance(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync::Speed(
                self_.get(1).wire2api(),
            ),
            1 => crate::api::pseudo_manual::enumeration_twin_sync::MeasureTwinSync::Distance(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::array::MessageIdTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::array::MessageIdTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::MessageIdTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::array_twin_sync::MessageIdTwinSync(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::misc_example::MyNestedStructTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::MyNestedStructTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::MyNestedStructTwinNormal {
            tree_node: self_.get(0).wire2api(),
            weekday: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::MyNestedStructTwinSync {
            tree_node: self_.get(0).wire2api(),
            weekday: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MySize>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::auxiliary::sample_types::MySize {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::auxiliary::sample_types::MySize {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MySizeFreezedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::MySizeFreezedTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::MySizeFreezedTwinNormal {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync {
            width: self_.get(0).wire2api(),
            height: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyStruct>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::auxiliary::sample_types::MyStruct {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::auxiliary::sample_types::MyStruct {
            content: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::MyTreeNodeTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::MyTreeNodeTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::MyTreeNodeTwinNormal {
            value_i32: self_.get(0).wire2api(),
            value_vec_u8: self_.get(1).wire2api(),
            value_boolean: self_.get(2).wire2api(),
            children: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::MyTreeNodeTwinSync {
            value_i32: self_.get(0).wire2api(),
            value_vec_u8: self_.get(1).wire2api(),
            value_boolean: self_.get(2).wire2api(),
            children: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::newtype_pattern::NewTypeIntTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::newtype_pattern::NewTypeIntTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::newtype_pattern::NewTypeIntTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::newtype_pattern_twin_sync::NewTypeIntTwinSync(
            self_.get(0).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::enumeration::NoteTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::NoteTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::enumeration::NoteTwinNormal {
            day: self_.get(0).wire2api(),
            body: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::enumeration_twin_sync::NoteTwinSync {
            day: self_.get(0).wire2api(),
            body: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Numbers>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Numbers {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::mirror_twin_sync::Numbers(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::rust_opaque::OpaqueNestedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::rust_opaque::OpaqueNestedTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::rust_opaque::OpaqueNestedTwinNormal {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::rust_opaque_twin_sync::OpaqueNestedTwinSync {
            first: self_.get(0).wire2api(),
            second: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Option<String>> for Option<String> {
    fn wire2api(self) -> Option<String> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f32>>> for Option<Box<[f32]>> {
    fn wire2api(self) -> Option<Vec<f32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<f64>>> for Option<Box<[f64]>> {
    fn wire2api(self) -> Option<Vec<f64>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i32>>> for Option<Box<[i32]>> {
    fn wire2api(self) -> Option<Vec<i32>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<i8>>> for Option<Box<[i8]>> {
    fn wire2api(self) -> Option<Vec<i8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<Option<Vec<u8>>> for Option<Box<[u8]>> {
    fn wire2api(self) -> Option<Vec<u8>> {
        self.map(Wire2Api::wire2api)
    }
}
impl Wire2Api<crate::api::optional::OptVecsTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::optional::OptVecsTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::optional::OptVecsTwinNormal {
            i32: self_.get(0).wire2api(),
            enums: self_.get(1).wire2api(),
            strings: self_.get(2).wire2api(),
            buffers: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            4,
            "Expected 4 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::optional_twin_sync::OptVecsTwinSync {
            i32: self_.get(0).wire2api(),
            enums: self_.get(1).wire2api(),
            strings: self_.get(2).wire2api(),
            buffers: self_.get(3).wire2api(),
        }
    }
}
impl Wire2Api<(String, i32)> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> (String, i32) {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        (self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::Sequences>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::Sequences {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::mirror_twin_sync::Sequences(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::exception::SomeStructTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::exception::SomeStructTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::exception::SomeStructTwinNormal {
            value: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::exception_twin_sync::SomeStructTwinSync {
            value: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::enumeration::SpeedTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::SpeedTwinNormal {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::enumeration::SpeedTwinNormal::Unknown,
            1 => crate::api::enumeration::SpeedTwinNormal::GPS(self_.get(1).wire2api()),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync {
        let self_ = self.unchecked_into::<flutter_rust_bridge::JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync::Unknown,
            1 => crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync::GPS(
                self_.get(1).wire2api(),
            ),
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<crate::api::comment::StructWithCommentsTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::comment::StructWithCommentsTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::comment::StructWithCommentsTwinNormal {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::comment_twin_sync::StructWithCommentsTwinSync {
            field_with_comments: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::misc_example::StructWithEnumTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::StructWithEnumTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::misc_example::StructWithEnumTwinNormal {
            abc1: self_.get(0).wire2api(),
            abc2: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::misc_example_twin_sync::StructWithEnumTwinSync {
            abc1: self_.get(0).wire2api(),
            abc2: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithOneFieldTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::structure::StructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithOneFieldTwinNormal {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithOneFieldTwinSync {
            a: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithTwoFieldTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::structure::StructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithTwoFieldTwinNormal {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithTwoFieldTwinSync {
            a: self_.get(0).wire2api(),
            b: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::structure::StructWithZeroFieldTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::structure::StructWithZeroFieldTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::structure::StructWithZeroFieldTwinNormal {}
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            0,
            "Expected 0 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::StructWithZeroFieldTwinSync {}
    }
}
impl Wire2Api<crate::api::method::SumWithTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::method::SumWithTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::method::SumWithTwinNormal {
            x: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::method_twin_sync::SumWithTwinSync {
            x: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::array::TestIdTwinNormal> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> crate::api::array::TestIdTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::array::TestIdTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithOneFieldTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithOneFieldTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::structure::TupleStructWithOneFieldTwinNormal(self_.get(0).wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithOneFieldTwinSync(
            self_.get(0).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::structure::TupleStructWithTwoFieldTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::structure::TupleStructWithTwoFieldTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::structure::TupleStructWithTwoFieldTwinNormal(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
        )
    }
}
impl Wire2Api<crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::structure_twin_sync::TupleStructWithTwoFieldTwinSync(
            self_.get(0).wire2api(),
            self_.get(1).wire2api(),
        )
    }
}
impl Wire2Api<[u8; 1600]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for Box<[u8]> {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<crate::api::attribute::UserIdTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::attribute::UserIdTwinNormal {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::attribute::UserIdTwinNormal {
            value: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync {
        let self_ = self.dyn_into::<flutter_rust_bridge::JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            1,
            "Expected 1 elements, got {}",
            self_.length()
        );
        crate::api::pseudo_manual::attribute_twin_sync::UserIdTwinSync {
            value: self_.get(0).wire2api(),
        }
    }
}
impl Wire2Api<chrono::Duration> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> chrono::Duration {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::Duration>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<chrono::Duration> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Local>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Local> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<chrono::NaiveDateTime> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> chrono::NaiveDateTime {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<Vec<chrono::NaiveDateTime>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<chrono::NaiveDateTime> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::BigInt64Array>()
            .to_vec()
            .into_iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<chrono::DateTime<chrono::Utc>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> chrono::DateTime<chrono::Utc> {
        Wire2Api::<i64>::wire2api(self).wire2api()
    }
}
impl Wire2Api<[flutter_rust_bridge::DartOpaque; 1]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [flutter_rust_bridge::DartOpaque; 1] {
        let vec: Vec<flutter_rust_bridge::DartOpaque> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Mutex<HideData>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Mutex<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<RwLock<HideData>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<RwLock<HideData>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinNormal>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinNormal>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinSync>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<Box<dyn DartDebugTwinSync>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<[flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2]>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> [flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>; 2] {
        let vec: Vec<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::HideData>> =
            self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<i32>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> flutter_rust_bridge::RustOpaque<i32> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonCloneData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> flutter_rust_bridge::RustOpaque<crate::auxiliary::sample_types::NonSendHideData> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe {
            flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
        }
    }
}
impl Wire2Api<String> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> String {
        self.as_string().expect("non-UTF-8 string, or not a string")
    }
}
impl Wire2Api<uuid::Uuid> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> uuid::Uuid {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<Vec<uuid::Uuid>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<uuid::Uuid> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint8Array>()
            .to_vec()
            .into_boxed_slice()
            .wire2api()
    }
}
impl Wire2Api<flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> flutter_rust_bridge::ZeroCopyBuffer<Vec<u8>> {
        flutter_rust_bridge::ZeroCopyBuffer(self.wire2api())
    }
}
impl Wire2Api<crate::api::pseudo_manual::mirror_twin_sync::ApplicationMode>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::mirror_twin_sync::ApplicationMode {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<bool> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> bool {
        self.is_truthy()
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::mirror_twin_sync::ApplicationEnv> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::array::BlobTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::array::BlobTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::array_twin_sync::BlobTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<bool>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<bool> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::enumeration::DistanceTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::enumeration::DistanceTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::DistanceTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::optional::ExoticOptionalsTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::optional::ExoticOptionalsTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::optional_twin_sync::ExoticOptionalsTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<f64>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<f64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i32>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<i32> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i64>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<i64> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<i8>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<i8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::enumeration::KitchenSinkTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::enumeration::KitchenSinkTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::KitchenSinkTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::auxiliary::sample_types::MySize>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::auxiliary::sample_types::MySize> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::misc_example::MySizeFreezedTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::misc_example::MySizeFreezedTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(
        self,
    ) -> Box<crate::api::pseudo_manual::misc_example_twin_sync::MySizeFreezedTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::enumeration::SpeedTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::enumeration::SpeedTwinNormal> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::enumeration_twin_sync::SpeedTwinSync> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<u8>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<u8> {
        Box::new(self.wire2api())
    }
}
impl Wire2Api<Box<[u8; 1600]>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Box<[u8; 1600]> {
        let vec: Vec<u8> = self.wire2api();
        Box::new(flutter_rust_bridge::support::from_vec_to_array(vec))
    }
}
impl Wire2Api<Box<crate::api::misc_example::WeekdaysTwinNormal>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::misc_example::WeekdaysTwinNormal> {
        let ptr: Box<i32> = self.wire2api();
        Box::new(ptr.wire2api())
    }
}
impl Wire2Api<Box<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> Box<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync> {
        let ptr: Box<i32> = self.wire2api();
        Box::new(ptr.wire2api())
    }
}
impl Wire2Api<crate::api::enumeration::EnumSimpleTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::enumeration::EnumSimpleTwinNormal {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<crate::api::pseudo_manual::enumeration_twin_sync::EnumSimpleTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::enumeration_twin_sync::EnumSimpleTwinSync {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<f32> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[f64; 16]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [f64; 16] {
        let vec: Vec<f64> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<i16> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> i16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<i32> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[i32; 2]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [i32; 2] {
        let vec: Vec<i32> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<i64> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> i64 {
        ::std::convert::TryInto::try_into(
            self.dyn_into::<flutter_rust_bridge::js_sys::BigInt>()
                .unwrap(),
        )
        .unwrap()
    }
}
impl Wire2Api<i8> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> i8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<f32>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<f64>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<i16>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<i16> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Int16Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<i32>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Int32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<i64>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<i64> {
        let buf = self
            .dyn_into::<flutter_rust_bridge::js_sys::BigInt64Array>()
            .unwrap();
        let buf = flutter_rust_bridge::js_sys::Uint8Array::new(&buf.buffer());
        flutter_rust_bridge::support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<i8>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<i8> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Int8Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<u16>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<u16> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint16Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<u32>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<Vec<u64>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<u64> {
        let buf = self
            .dyn_into::<flutter_rust_bridge::js_sys::BigUint64Array>()
            .unwrap();
        let buf = flutter_rust_bridge::js_sys::Uint8Array::new(&buf.buffer());
        flutter_rust_bridge::support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
impl Wire2Api<Vec<u8>> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> Vec<u8> {
        self.unchecked_into::<flutter_rust_bridge::js_sys::Uint8Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<crate::auxiliary::sample_types::MyEnum>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::auxiliary::sample_types::MyEnum {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<[crate::api::array::TestIdTwinNormal; 4]>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> [crate::api::array::TestIdTwinNormal; 4] {
        let vec: Vec<crate::api::array::TestIdTwinNormal> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync; 4]>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> [crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync; 4] {
        let vec: Vec<crate::api::pseudo_manual::array_twin_sync::TestIdTwinSync> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<u16> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(
            self.dyn_into::<flutter_rust_bridge::js_sys::BigInt>()
                .unwrap(),
        )
        .unwrap()
    }
}
impl Wire2Api<u8> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> u8 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[u8; 1600]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [u8; 1600] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 32]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [u8; 32] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<[u8; 8]> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> [u8; 8] {
        let vec: Vec<u8> = self.wire2api();
        flutter_rust_bridge::support::from_vec_to_array(vec)
    }
}
impl Wire2Api<usize> for flutter_rust_bridge::wasm_bindgen::JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<crate::api::misc_example::WeekdaysTwinNormal>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::misc_example::WeekdaysTwinNormal {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}
impl Wire2Api<crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync>
    for flutter_rust_bridge::wasm_bindgen::JsValue
{
    fn wire2api(self) -> crate::api::pseudo_manual::misc_example_twin_sync::WeekdaysTwinSync {
        (self.unchecked_into_f64() as i32).wire2api()
    }
}

#[wasm_bindgen]
pub fn wire_boxed_blob_twin_normal(port_: flutter_rust_bridge::MessagePort, blob: Box<[u8]>) {
    wire_boxed_blob_twin_normal_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_func_test_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_test_id_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_get_array_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_array_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_complex_array_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_complex_array_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_last_number_twin_normal(port_: flutter_rust_bridge::MessagePort, array: Box<[f64]>) {
    wire_last_number_twin_normal_impl(port_, array)
}

#[wasm_bindgen]
pub fn wire_nested_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_nested_id_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_new_msgid_twin_normal(port_: flutter_rust_bridge::MessagePort, id: Box<[u8]>) {
    wire_new_msgid_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_feed_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    id: Box<[u8]>,
) {
    wire_return_boxed_feed_id_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_raw_feed_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_return_boxed_raw_feed_id_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_use_boxed_blob_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    blob: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_use_boxed_blob_twin_normal_impl(port_, blob)
}

#[wasm_bindgen]
pub fn wire_use_msgid_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_use_msgid_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_func_async_simple_add(port_: flutter_rust_bridge::MessagePort, a: i32, b: i32) {
    wire_func_async_simple_add_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_func_async_void(port_: flutter_rust_bridge::MessagePort) {
    wire_func_async_void_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_customized_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    val: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_customized_struct_twin_normal_impl(port_, val)
}

#[wasm_bindgen]
pub fn wire_next_user_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    user_id: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_next_user_id_twin_normal_impl(port_, user_id)
}

#[wasm_bindgen]
pub fn wire_benchmark_input_bytes_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    bytes: Box<[u8]>,
) {
    wire_benchmark_input_bytes_twin_normal_impl(port_, bytes)
}

#[wasm_bindgen]
pub fn wire_benchmark_output_bytes_twin_normal(port_: flutter_rust_bridge::MessagePort, size: i32) {
    wire_benchmark_output_bytes_twin_normal_impl(port_, size)
}

#[wasm_bindgen]
pub fn wire_benchmark_void_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_benchmark_void_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_datetime_local_twin_normal(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_datetime_local_twin_normal_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_datetime_utc_twin_normal(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_datetime_utc_twin_normal_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_duration_twin_normal(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_duration_twin_normal_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_handle_durations_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    durations: Box<[i64]>,
    since: i64,
) {
    wire_handle_durations_twin_normal_impl(port_, durations, since)
}

#[wasm_bindgen]
pub fn wire_handle_timestamps_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    timestamps: Box<[i64]>,
    epoch: i64,
) {
    wire_handle_timestamps_twin_normal_impl(port_, timestamps, epoch)
}

#[wasm_bindgen]
pub fn wire_how_long_does_it_take_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    mine: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_how_long_does_it_take_twin_normal_impl(port_, mine)
}

#[wasm_bindgen]
pub fn wire_naivedatetime_twin_normal(port_: flutter_rust_bridge::MessagePort, d: i64) {
    wire_naivedatetime_twin_normal_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_optional_empty_datetime_utc_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    d: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_optional_empty_datetime_utc_twin_normal_impl(port_, d)
}

#[wasm_bindgen]
pub fn wire_test_chrono_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_chrono_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_precise_chrono_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_precise_chrono_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_instance_method_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_StructWithCommentsTwinNormal_instance_method_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinNormal_static_method_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_StructWithCommentsTwinNormal_static_method_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_slash_star_star_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_triple_slash_multi_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_function_with_comments_triple_slash_single_line_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_dart_dynamic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_dart_dynamic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_async_accept_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_async_accept_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_enum_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_create_enum_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_nested_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque1: flutter_rust_bridge::wasm_bindgen::JsValue,
    opaque2: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_create_nested_dart_opaque_twin_normal_impl(port_, opaque1, opaque2)
}

#[wasm_bindgen]
pub fn wire_drop_static_dart_opaque_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_drop_static_dart_opaque_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_enum_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_get_enum_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_get_nested_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_get_nested_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_get_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_array_get_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_array_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_get_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_option_get_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_option_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_get_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_vec_get_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_loop_back_vec_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_panic_unwrap_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_panic_unwrap_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_set_static_dart_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_set_static_dart_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_return_non_droppable_dart_opaque_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_non_droppable_dart_opaque_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_accept_dart_opaque_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_accept_dart_opaque_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_loopback_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_loopback_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_option_dart_opaque_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_dart_opaque_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_sync_option_loopback_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_loopback_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_dart_opaque_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_dart_opaque_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_normal(port_: flutter_rust_bridge::MessagePort, arg: i32) {
    wire_func_enum_simple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_enum_with_item_mixed_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_enum_with_item_struct_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_enum_with_item_tuple_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_enum_parameter_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    weekday: i32,
) {
    wire_handle_enum_parameter_twin_normal_impl(port_, weekday)
}

#[wasm_bindgen]
pub fn wire_handle_enum_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    val: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_enum_struct_twin_normal_impl(port_, val)
}

#[wasm_bindgen]
pub fn wire_handle_return_enum_twin_normal(port_: flutter_rust_bridge::MessagePort, input: String) {
    wire_handle_return_enum_twin_normal_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    measure: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_multiply_by_ten_twin_normal_impl(port_, measure)
}

#[wasm_bindgen]
pub fn wire_print_note_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    note: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_print_note_twin_normal_impl(port_, note)
}

#[wasm_bindgen]
pub fn wire_EventTwinNormal_as_string_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_EventTwinNormal_as_string_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_close_event_listener_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_close_event_listener_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_event_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    address: String,
    payload: String,
) {
    wire_create_event_twin_normal_impl(port_, address, payload)
}

#[wasm_bindgen]
pub fn wire_register_event_listener_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_register_event_listener_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinNormal_new_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    message: String,
) {
    wire_CustomStructTwinNormal_new_twin_normal_impl(port_, message)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_CustomStructTwinNormal_nonstatic_return_custom_struct_error_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_CustomStructTwinNormal_nonstatic_return_custom_struct_ok_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_CustomStructTwinNormal_static_return_custom_struct_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_CustomStructTwinNormal_static_return_custom_struct_ok_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinNormal_new_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    value: u32,
) {
    wire_SomeStructTwinNormal_new_twin_normal_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_SomeStructTwinNormal_non_static_return_err_custom_error_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_SomeStructTwinNormal_non_static_return_ok_custom_error_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_SomeStructTwinNormal_static_return_err_custom_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_SomeStructTwinNormal_static_return_ok_custom_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_custom_enum_error_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_custom_enum_error_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u32,
) {
    wire_custom_enum_error_return_ok_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_custom_nested_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_custom_struct_error_return_error_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_type_fallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_type_infallible_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_panic_with_custom_result_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_panic_with_custom_result_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_custom_nested_error_1_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1_variant1_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_return_custom_nested_error_1_variant1_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_2_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_custom_nested_error_2_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_custom_struct_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_ok_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_custom_struct_ok_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_err_custom_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_err_custom_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_return_error_variant_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    variant: u32,
) {
    wire_return_error_variant_twin_normal_impl(port_, variant)
}

#[wasm_bindgen]
pub fn wire_return_ok_custom_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_return_ok_custom_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_stream_sink_throw_anyhow_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_stream_sink_throw_anyhow_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_sync_return_custom_struct_error_twin_normal(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_return_custom_struct_error_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_throw_anyhow_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_throw_anyhow_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_call_new_module_system_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_call_new_module_system_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_call_old_module_system_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_call_old_module_system_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_use_imported_enum_twin_normal(port_: flutter_rust_bridge::MessagePort, my_enum: i32) {
    wire_use_imported_enum_twin_normal_impl(port_, my_enum)
}

#[wasm_bindgen]
pub fn wire_use_imported_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    my_struct: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_use_imported_struct_twin_normal_impl(port_, my_struct)
}

#[wasm_bindgen]
pub fn wire_another_macro_struct_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_another_macro_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_macro_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_macro_struct_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    a: String,
    b: String,
) {
    wire_ConcatenateWithTwinNormal_concatenate_static_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_concatenate_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    b: String,
) {
    wire_ConcatenateWithTwinNormal_concatenate_twin_normal_impl(port_, that, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_single_arg_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinNormal_handle_some_static_stream_sink_twin_normal_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_ConcatenateWithTwinNormal_handle_some_stream_sink_at_1_twin_normal_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinNormal_handle_some_stream_sink_twin_normal_impl(port_, that, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinNormal_new_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    a: String,
) {
    wire_ConcatenateWithTwinNormal_new_twin_normal_impl(port_, a)
}

#[wasm_bindgen]
pub fn wire_SumWithTwinNormal_sum_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    y: u32,
    z: u32,
) {
    wire_SumWithTwinNormal_sum_twin_normal_impl(port_, that, y, z)
}

#[wasm_bindgen]
pub fn wire_get_sum_array_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    a: u32,
    b: u32,
    c: u32,
) {
    wire_get_sum_array_twin_normal_impl(port_, a, b, c)
}

#[wasm_bindgen]
pub fn wire_get_sum_struct_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_sum_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_stream_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_stream_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_vec_stream_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_vec_stream_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_first_number_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    nums: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_first_number_twin_normal_impl(port_, nums)
}

#[wasm_bindgen]
pub fn wire_first_sequence_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    seqs: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_first_sequence_twin_normal_impl(port_, seqs)
}

#[wasm_bindgen]
pub fn wire_get_app_settings_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_app_settings_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_fallible_app_settings_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_fallible_app_settings_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_message_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_get_message_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_is_app_embedded_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    app_settings: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_is_app_embedded_twin_normal_impl(port_, app_settings)
}

#[wasm_bindgen]
pub fn wire_mirror_struct_stream_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_struct_stream_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_mirror_tuple_stream_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_tuple_stream_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_repeat_number_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    num: i32,
    times: usize,
) {
    wire_repeat_number_twin_normal_impl(port_, num, times)
}

#[wasm_bindgen]
pub fn wire_repeat_sequence_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    seq: i32,
    times: usize,
) {
    wire_repeat_sequence_twin_normal_impl(port_, seq, times)
}

#[wasm_bindgen]
pub fn wire_test_contains_mirrored_sub_struct_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_contains_mirrored_sub_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_fallible_of_raw_string_mirrored_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_test_fallible_of_raw_string_mirrored_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_list_of_nested_enums_mirrored_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_test_list_of_nested_enums_mirrored_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_list_of_raw_nested_string_mirrored_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_test_list_of_raw_nested_string_mirrored_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_nested_raw_string_mirrored_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_nested_raw_string_mirrored_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_enum_mirrored_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    nested: bool,
) {
    wire_test_raw_string_enum_mirrored_twin_normal_impl(port_, nested)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_mirrored_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_raw_string_mirrored_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_big_buffers_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_handle_big_buffers_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    s: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_complex_struct_twin_normal_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_nested_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    s: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_nested_struct_twin_normal_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_string_twin_normal(port_: flutter_rust_bridge::MessagePort, s: String) {
    wire_handle_string_twin_normal_impl(port_, s)
}

#[wasm_bindgen]
pub fn wire_handle_struct_sync_freezed_twin_normal(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
    boxed: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_sync_freezed_twin_normal_impl(arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
    boxed: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_struct_twin_normal_impl(port_, arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_vec_u8_twin_normal(port_: flutter_rust_bridge::MessagePort, v: Box<[u8]>) {
    wire_handle_vec_u8_twin_normal_impl(port_, v)
}

#[wasm_bindgen]
pub fn wire_list_of_primitive_enums_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    weekdays: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_list_of_primitive_enums_twin_normal_impl(port_, weekdays)
}

#[wasm_bindgen]
pub fn wire_test_abc_enum_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    abc: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_test_abc_enum_twin_normal_impl(port_, abc)
}

#[wasm_bindgen]
pub fn wire_test_struct_with_enum_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    se: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_test_struct_with_enum_twin_normal_impl(port_, se)
}

#[wasm_bindgen]
pub fn wire_empty_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    empty: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_empty_struct_twin_normal_impl(port_, empty)
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_return_unit_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_string_twin_normal(port_: flutter_rust_bridge::MessagePort, arg: String) {
    wire_func_string_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    l: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_list_of_struct_twin_normal_impl(port_, l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    names: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_string_list_twin_normal_impl(port_, names)
}

#[wasm_bindgen]
pub fn wire_handle_newtype_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_newtype_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_increment_boxed_optional_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_increment_boxed_optional_twin_normal_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    i8box: flutter_rust_bridge::wasm_bindgen::JsValue,
    u8box: flutter_rust_bridge::wasm_bindgen::JsValue,
    i32box: flutter_rust_bridge::wasm_bindgen::JsValue,
    i64box: flutter_rust_bridge::wasm_bindgen::JsValue,
    f64box: flutter_rust_bridge::wasm_bindgen::JsValue,
    boolbox: flutter_rust_bridge::wasm_bindgen::JsValue,
    structbox: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_option_box_arguments_twin_normal_impl(
        port_, i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[wasm_bindgen]
pub fn wire_handle_optional_increment_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_optional_increment_twin_normal_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_handle_optional_return_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    left: f64,
    right: f64,
) {
    wire_handle_optional_return_twin_normal_impl(port_, left, right)
}

#[wasm_bindgen]
pub fn wire_handle_optional_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    document: Option<String>,
) {
    wire_handle_optional_struct_twin_normal_impl(port_, document)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_opts_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_vec_of_opts_twin_normal_impl(port_, opt)
}

#[wasm_bindgen]
pub fn wire_sync_option_null_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_null_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_primitive_optional_types_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    my_i32: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_i64: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_f64: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_bool: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_primitive_optional_types_twin_normal_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_primitive_twin_normal(port_: flutter_rust_bridge::MessagePort, n: i32) {
    wire_handle_vec_of_primitive_twin_normal_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_handle_zero_copy_vec_of_primitive_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    n: i32,
) {
    wire_handle_zero_copy_vec_of_primitive_twin_normal_impl(port_, n)
}

#[wasm_bindgen]
pub fn wire_primitive_types_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) {
    wire_primitive_types_twin_normal_impl(port_, my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_primitive_u32_twin_normal(port_: flutter_rust_bridge::MessagePort, my_u32: u32) {
    wire_primitive_u32_twin_normal_impl(port_, my_u32)
}

#[wasm_bindgen]
pub fn wire_boxed_blob_twin_sync(blob: Box<[u8]>) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_boxed_blob_twin_sync_impl(blob)
}

#[wasm_bindgen]
pub fn wire_func_test_id_twin_sync(
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_test_id_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_get_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_array_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_get_complex_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_complex_array_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_last_number_twin_sync(
    array: Box<[f64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_last_number_twin_sync_impl(array)
}

#[wasm_bindgen]
pub fn wire_nested_id_twin_sync(
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_nested_id_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_new_msgid_twin_sync(id: Box<[u8]>) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_new_msgid_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_feed_id_twin_sync(
    id: Box<[u8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_boxed_feed_id_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_return_boxed_raw_feed_id_twin_sync(
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_boxed_raw_feed_id_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_use_boxed_blob_twin_sync(
    blob: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_boxed_blob_twin_sync_impl(blob)
}

#[wasm_bindgen]
pub fn wire_use_msgid_twin_sync(
    id: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_msgid_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_handle_customized_struct_twin_sync(
    val: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_customized_struct_twin_sync_impl(val)
}

#[wasm_bindgen]
pub fn wire_next_user_id_twin_sync(
    user_id: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_next_user_id_twin_sync_impl(user_id)
}

#[wasm_bindgen]
pub fn wire_benchmark_input_bytes_twin_sync(
    bytes: Box<[u8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_input_bytes_twin_sync_impl(bytes)
}

#[wasm_bindgen]
pub fn wire_benchmark_output_bytes_twin_sync(
    size: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_output_bytes_twin_sync_impl(size)
}

#[wasm_bindgen]
pub fn wire_benchmark_void_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_benchmark_void_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_datetime_local_twin_sync(d: i64) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_datetime_local_twin_sync_impl(d)
}

#[wasm_bindgen]
pub fn wire_datetime_utc_twin_sync(d: i64) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_datetime_utc_twin_sync_impl(d)
}

#[wasm_bindgen]
pub fn wire_duration_twin_sync(d: i64) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_duration_twin_sync_impl(d)
}

#[wasm_bindgen]
pub fn wire_handle_durations_twin_sync(
    durations: Box<[i64]>,
    since: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_durations_twin_sync_impl(durations, since)
}

#[wasm_bindgen]
pub fn wire_handle_timestamps_twin_sync(
    timestamps: Box<[i64]>,
    epoch: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_timestamps_twin_sync_impl(timestamps, epoch)
}

#[wasm_bindgen]
pub fn wire_how_long_does_it_take_twin_sync(
    mine: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_how_long_does_it_take_twin_sync_impl(mine)
}

#[wasm_bindgen]
pub fn wire_naivedatetime_twin_sync(d: i64) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_naivedatetime_twin_sync_impl(d)
}

#[wasm_bindgen]
pub fn wire_optional_empty_datetime_utc_twin_sync(
    d: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_optional_empty_datetime_utc_twin_sync_impl(d)
}

#[wasm_bindgen]
pub fn wire_test_chrono_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_chrono_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_precise_chrono_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_precise_chrono_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_instance_method_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_instance_method_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_StructWithCommentsTwinSync_static_method_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_StructWithCommentsTwinSync_static_method_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_slash_star_star_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_slash_star_star_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_multi_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_multi_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_function_with_comments_triple_slash_single_line_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_function_with_comments_triple_slash_single_line_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_dart_dynamic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_dart_dynamic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_async_accept_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_async_accept_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_create_enum_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_enum_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_create_nested_dart_opaque_twin_sync(
    opaque1: flutter_rust_bridge::wasm_bindgen::JsValue,
    opaque2: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_nested_dart_opaque_twin_sync_impl(opaque1, opaque2)
}

#[wasm_bindgen]
pub fn wire_drop_static_dart_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_drop_static_dart_opaque_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_get_enum_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_enum_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_get_nested_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_nested_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_get_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_array_get_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_array_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_array_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_get_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_option_get_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_option_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_option_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_get_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_vec_get_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_loop_back_vec_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_loop_back_vec_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_panic_unwrap_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_panic_unwrap_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_set_static_dart_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_set_static_dart_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_func_enum_simple_twin_sync(arg: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_simple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_mixed_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_mixed_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_struct_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_struct_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_enum_with_item_tuple_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_enum_with_item_tuple_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_handle_enum_parameter_twin_sync(
    weekday: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_enum_parameter_twin_sync_impl(weekday)
}

#[wasm_bindgen]
pub fn wire_handle_enum_struct_twin_sync(
    val: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_enum_struct_twin_sync_impl(val)
}

#[wasm_bindgen]
pub fn wire_handle_return_enum_twin_sync(
    input: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_return_enum_twin_sync_impl(input)
}

#[wasm_bindgen]
pub fn wire_multiply_by_ten_twin_sync(
    measure: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_multiply_by_ten_twin_sync_impl(measure)
}

#[wasm_bindgen]
pub fn wire_print_note_twin_sync(
    note: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_print_note_twin_sync_impl(note)
}

#[wasm_bindgen]
pub fn wire_EventTwinSync_as_string_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_EventTwinSync_as_string_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_close_event_listener_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_close_event_listener_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_create_event_twin_sync(
    address: String,
    payload: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_event_twin_sync_impl(address, payload)
}

#[wasm_bindgen]
pub fn wire_register_event_listener_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_register_event_listener_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinSync_new_twin_sync(
    message: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_new_twin_sync_impl(message)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_nonstatic_return_custom_struct_error_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_nonstatic_return_custom_struct_ok_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_static_return_custom_struct_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_CustomStructTwinSync_static_return_custom_struct_ok_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinSync_new_twin_sync(
    value: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_new_twin_sync_impl(value)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_non_static_return_err_custom_error_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_non_static_return_ok_custom_error_twin_sync_impl(that)
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_static_return_err_custom_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SomeStructTwinSync_static_return_ok_custom_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_custom_enum_error_return_ok_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_enum_error_return_ok_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_nested_error_return_error_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_nested_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_custom_struct_error_return_error_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_custom_struct_error_return_error_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_return_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_return_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_fallible_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_fallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_type_infallible_panic_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_type_infallible_panic_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_panic_with_custom_result_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_panic_with_custom_result_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_return_custom_nested_error_1_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_1_variant1_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_nested_error_1_variant1_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_nested_error_2_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_return_custom_nested_error_2_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_struct_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_custom_struct_ok_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_custom_struct_ok_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_err_custom_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_err_custom_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_return_error_variant_twin_sync(
    variant: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_error_variant_twin_sync_impl(variant)
}

#[wasm_bindgen]
pub fn wire_return_ok_custom_error_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_return_ok_custom_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_stream_sink_throw_anyhow_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_stream_sink_throw_anyhow_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_sync_return_custom_struct_error_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_return_custom_struct_error_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_throw_anyhow_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_throw_anyhow_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_call_new_module_system_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_call_new_module_system_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_call_old_module_system_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_call_old_module_system_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_use_imported_enum_twin_sync(
    my_enum: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_imported_enum_twin_sync_impl(my_enum)
}

#[wasm_bindgen]
pub fn wire_use_imported_struct_twin_sync(
    my_struct: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_use_imported_struct_twin_sync_impl(my_struct)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_concatenate_static_twin_sync(
    a: String,
    b: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_concatenate_static_twin_sync_impl(a, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_concatenate_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    b: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_concatenate_twin_sync_impl(that, b)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_single_arg_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinSync_handle_some_static_stream_sink_twin_sync_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_ConcatenateWithTwinSync_handle_some_stream_sink_at_1_twin_sync_impl(port_, that)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync(
    port_: flutter_rust_bridge::MessagePort,
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    key: u32,
    max: u32,
) {
    wire_ConcatenateWithTwinSync_handle_some_stream_sink_twin_sync_impl(port_, that, key, max)
}

#[wasm_bindgen]
pub fn wire_ConcatenateWithTwinSync_new_twin_sync(
    a: String,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_ConcatenateWithTwinSync_new_twin_sync_impl(a)
}

#[wasm_bindgen]
pub fn wire_SumWithTwinSync_sum_twin_sync(
    that: flutter_rust_bridge::wasm_bindgen::JsValue,
    y: u32,
    z: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_SumWithTwinSync_sum_twin_sync_impl(that, y, z)
}

#[wasm_bindgen]
pub fn wire_get_sum_array_twin_sync(
    a: u32,
    b: u32,
    c: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_sum_array_twin_sync_impl(a, b, c)
}

#[wasm_bindgen]
pub fn wire_get_sum_struct_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_sum_struct_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_app_settings_stream_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_stream_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_app_settings_vec_stream_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_app_settings_vec_stream_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_first_number_twin_sync(
    nums: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_first_number_twin_sync_impl(nums)
}

#[wasm_bindgen]
pub fn wire_first_sequence_twin_sync(
    seqs: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_first_sequence_twin_sync_impl(seqs)
}

#[wasm_bindgen]
pub fn wire_get_app_settings_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_app_settings_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_get_fallible_app_settings_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_fallible_app_settings_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_get_message_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_get_message_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_is_app_embedded_twin_sync(
    app_settings: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_is_app_embedded_twin_sync_impl(app_settings)
}

#[wasm_bindgen]
pub fn wire_mirror_struct_stream_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_struct_stream_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_mirror_tuple_stream_twin_sync(port_: flutter_rust_bridge::MessagePort) {
    wire_mirror_tuple_stream_twin_sync_impl(port_)
}

#[wasm_bindgen]
pub fn wire_repeat_number_twin_sync(
    num: i32,
    times: usize,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_repeat_number_twin_sync_impl(num, times)
}

#[wasm_bindgen]
pub fn wire_repeat_sequence_twin_sync(
    seq: i32,
    times: usize,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_repeat_sequence_twin_sync_impl(seq, times)
}

#[wasm_bindgen]
pub fn wire_test_contains_mirrored_sub_struct_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_contains_mirrored_sub_struct_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_fallible_of_raw_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_fallible_of_raw_string_mirrored_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_list_of_nested_enums_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_list_of_nested_enums_mirrored_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_list_of_raw_nested_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_list_of_raw_nested_string_mirrored_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_nested_raw_string_mirrored_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_nested_raw_string_mirrored_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_raw_string_enum_mirrored_twin_sync(
    nested: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_raw_string_enum_mirrored_twin_sync_impl(nested)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_mirrored_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_raw_string_mirrored_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_handle_big_buffers_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_big_buffers_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_handle_complex_struct_twin_sync(
    s: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_complex_struct_twin_sync_impl(s)
}

#[wasm_bindgen]
pub fn wire_handle_nested_struct_twin_sync(
    s: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_nested_struct_twin_sync_impl(s)
}

#[wasm_bindgen]
pub fn wire_handle_string_twin_sync(s: String) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_string_twin_sync_impl(s)
}

#[wasm_bindgen]
pub fn wire_handle_struct_sync_freezed_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
    boxed: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_sync_freezed_twin_sync_impl(arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_struct_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
    boxed: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_struct_twin_sync_impl(arg, boxed)
}

#[wasm_bindgen]
pub fn wire_handle_vec_u8_twin_sync(v: Box<[u8]>) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_u8_twin_sync_impl(v)
}

#[wasm_bindgen]
pub fn wire_list_of_primitive_enums_twin_sync(
    weekdays: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_list_of_primitive_enums_twin_sync_impl(weekdays)
}

#[wasm_bindgen]
pub fn wire_test_abc_enum_twin_sync(
    abc: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_abc_enum_twin_sync_impl(abc)
}

#[wasm_bindgen]
pub fn wire_test_struct_with_enum_twin_sync(
    se: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_struct_with_enum_twin_sync_impl(se)
}

#[wasm_bindgen]
pub fn wire_empty_struct_twin_sync(
    empty: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_empty_struct_twin_sync_impl(empty)
}

#[wasm_bindgen]
pub fn wire_func_return_unit_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_return_unit_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_func_string_twin_sync(arg: String) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_string_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_handle_list_of_struct_twin_sync(
    l: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_list_of_struct_twin_sync_impl(l)
}

#[wasm_bindgen]
pub fn wire_handle_string_list_twin_sync(
    names: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_string_list_twin_sync_impl(names)
}

#[wasm_bindgen]
pub fn wire_handle_newtype_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_newtype_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_optional_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_primitive_optional_types_twin_sync(
    my_i32: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_i64: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_f64: flutter_rust_bridge::wasm_bindgen::JsValue,
    my_bool: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_optional_types_twin_sync_impl(my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_bool_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f32_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_f64_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i16_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i32_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i64_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_i8_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u16_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u32_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u64_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_optional_primitive_type_u8_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_optional_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_handle_increment_boxed_optional_twin_sync(
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_increment_boxed_optional_twin_sync_impl(opt)
}

#[wasm_bindgen]
pub fn wire_handle_option_box_arguments_twin_sync(
    i8box: flutter_rust_bridge::wasm_bindgen::JsValue,
    u8box: flutter_rust_bridge::wasm_bindgen::JsValue,
    i32box: flutter_rust_bridge::wasm_bindgen::JsValue,
    i64box: flutter_rust_bridge::wasm_bindgen::JsValue,
    f64box: flutter_rust_bridge::wasm_bindgen::JsValue,
    boolbox: flutter_rust_bridge::wasm_bindgen::JsValue,
    structbox: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_option_box_arguments_twin_sync_impl(
        i8box, u8box, i32box, i64box, f64box, boolbox, structbox,
    )
}

#[wasm_bindgen]
pub fn wire_handle_optional_increment_twin_sync(
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_increment_twin_sync_impl(opt)
}

#[wasm_bindgen]
pub fn wire_handle_optional_return_twin_sync(
    left: f64,
    right: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_return_twin_sync_impl(left, right)
}

#[wasm_bindgen]
pub fn wire_handle_optional_struct_twin_sync(
    document: Option<String>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_optional_struct_twin_sync_impl(document)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_opts_twin_sync(
    opt: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_of_opts_twin_sync_impl(opt)
}

#[wasm_bindgen]
pub fn wire_sync_option_null_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_null_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: bool,
) {
    wire_example_primitive_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: f32,
) {
    wire_example_primitive_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: f64,
) {
    wire_example_primitive_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i16,
) {
    wire_example_primitive_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i32,
) {
    wire_example_primitive_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i64,
) {
    wire_example_primitive_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: i8,
) {
    wire_example_primitive_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u16,
) {
    wire_example_primitive_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u32,
) {
    wire_example_primitive_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u64,
) {
    wire_example_primitive_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: u8,
) {
    wire_example_primitive_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_example_primitive_list_type_bool_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[f32]>,
) {
    wire_example_primitive_list_type_f32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[f64]>,
) {
    wire_example_primitive_list_type_f64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i16]>,
) {
    wire_example_primitive_list_type_i16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i32]>,
) {
    wire_example_primitive_list_type_i32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i64]>,
) {
    wire_example_primitive_list_type_i64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[i8]>,
) {
    wire_example_primitive_list_type_i8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u16]>,
) {
    wire_example_primitive_list_type_u16_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u32]>,
) {
    wire_example_primitive_list_type_u32_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u64]>,
) {
    wire_example_primitive_list_type_u64_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: Box<[u8]>,
) {
    wire_example_primitive_list_type_u8_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_handle_vec_of_primitive_twin_sync(
    n: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_vec_of_primitive_twin_sync_impl(n)
}

#[wasm_bindgen]
pub fn wire_handle_zero_copy_vec_of_primitive_twin_sync(
    n: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_zero_copy_vec_of_primitive_twin_sync_impl(n)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_bool_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f32_twin_sync(
    arg: Box<[f32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_f64_twin_sync(
    arg: Box<[f64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i16_twin_sync(
    arg: Box<[i16]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i32_twin_sync(
    arg: Box<[i32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i64_twin_sync(
    arg: Box<[i64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_i8_twin_sync(
    arg: Box<[i8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u16_twin_sync(
    arg: Box<[u16]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u32_twin_sync(
    arg: Box<[u32]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u64_twin_sync(
    arg: Box<[u64]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_list_type_u8_twin_sync(
    arg: Box<[u8]>,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_list_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_primitive_types_twin_sync(
    my_i32: i32,
    my_i64: i64,
    my_f64: f64,
    my_bool: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_types_twin_sync_impl(my_i32, my_i64, my_f64, my_bool)
}

#[wasm_bindgen]
pub fn wire_primitive_u32_twin_sync(my_u32: u32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_primitive_u32_twin_sync_impl(my_u32)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_bool_twin_sync(
    arg: bool,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_bool_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f32_twin_sync(
    arg: f32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_f64_twin_sync(
    arg: f64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_f64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i16_twin_sync(
    arg: i16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i32_twin_sync(
    arg: i32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i64_twin_sync(
    arg: i64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_i8_twin_sync(
    arg: i8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_i8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u16_twin_sync(
    arg: u16,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u16_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u32_twin_sync(
    arg: u32,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u32_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u64_twin_sync(
    arg: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u64_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_example_primitive_type_u8_twin_sync(
    arg: u8,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_example_primitive_type_u8_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_test_more_than_just_one_raw_string_struct_twin_sync(
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_more_than_just_one_raw_string_struct_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_test_raw_string_item_struct_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn
{
    wire_test_raw_string_item_struct_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_create_array_opaque_enum_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_array_opaque_enum_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_create_nested_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_nested_opaque_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_create_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_opaque_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_create_option_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_option_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_create_sync_opaque_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_create_sync_opaque_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_frb_generator_test_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_frb_generator_test_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_opaque_array_run_twin_sync(
    data: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_array_run_twin_sync_impl(data)
}

#[wasm_bindgen]
pub fn wire_opaque_array_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_array_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_opaque_vec_run_twin_sync(
    data: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_vec_run_twin_sync_impl(data)
}

#[wasm_bindgen]
pub fn wire_opaque_vec_twin_sync() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_opaque_vec_twin_sync_impl()
}

#[wasm_bindgen]
pub fn wire_run_enum_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_enum_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_run_nested_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_nested_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_run_non_clone_twin_sync(
    clone: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_non_clone_twin_sync_impl(clone)
}

#[wasm_bindgen]
pub fn wire_run_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_run_opaque_with_delay_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_run_opaque_with_delay_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_rust_opaque_twin_sync(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_unwrap_rust_opaque_twin_sync_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_sync(a: i32, b: i32) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_simple_adder_twin_sync_impl(a, b)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_struct_with_zero_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_one_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_sync(
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_func_tuple_struct_with_two_field_twin_sync_impl(arg)
}

#[wasm_bindgen]
pub fn wire_test_tuple_2_twin_sync(
    value: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_tuple_2_twin_sync_impl(value)
}

#[wasm_bindgen]
pub fn wire_test_tuple_twin_sync(
    value: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_test_tuple_twin_sync_impl(value)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_id_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_alias_id_twin_sync_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_model_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_alias_model_twin_sync_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_type_nest_alias_id_twin_sync(
    input: u64,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_type_nest_alias_id_twin_sync_impl(input)
}

#[wasm_bindgen]
pub fn wire_handle_nested_uuids_twin_sync(
    ids: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_nested_uuids_twin_sync_impl(ids)
}

#[wasm_bindgen]
pub fn wire_handle_uuid_twin_sync(id: Box<[u8]>) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_uuid_twin_sync_impl(id)
}

#[wasm_bindgen]
pub fn wire_handle_uuids_twin_sync(ids: Box<[u8]>) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_handle_uuids_twin_sync_impl(ids)
}

#[wasm_bindgen]
pub fn wire_test_more_than_just_one_raw_string_struct_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
) {
    wire_test_more_than_just_one_raw_string_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_test_raw_string_item_struct_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_test_raw_string_item_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_array_opaque_enum_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_create_array_opaque_enum_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_nested_opaque_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_create_nested_opaque_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_opaque_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_create_opaque_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_create_option_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_create_option_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_create_sync_opaque_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_create_sync_opaque_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_frb_generator_test_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_frb_generator_test_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_array_run_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    data: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_opaque_array_run_twin_normal_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_opaque_array_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_opaque_array_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_opaque_vec_run_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    data: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_opaque_vec_run_twin_normal_impl(port_, data)
}

#[wasm_bindgen]
pub fn wire_opaque_vec_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_opaque_vec_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_run_enum_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_run_enum_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_nested_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_run_nested_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_non_clone_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    clone: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_run_non_clone_twin_normal_impl(port_, clone)
}

#[wasm_bindgen]
pub fn wire_run_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_run_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_run_opaque_with_delay_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_run_opaque_with_delay_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_unwrap_rust_opaque_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_unwrap_rust_opaque_twin_normal_impl(port_, opaque)
}

#[wasm_bindgen]
pub fn wire_frb_sync_generator_test_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_frb_sync_generator_test_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_create_non_clone_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_non_clone_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_create_opaque_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_opaque_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_create_sync_opaque_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_create_sync_opaque_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_option_rust_opaque_twin_normal() -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_option_rust_opaque_twin_normal_impl()
}

#[wasm_bindgen]
pub fn wire_sync_run_opaque_twin_normal(
    opaque: flutter_rust_bridge::wasm_bindgen::JsValue,
) -> flutter_rust_bridge::support::WireSyncReturn {
    wire_sync_run_opaque_twin_normal_impl(opaque)
}

#[wasm_bindgen]
pub fn wire_simple_adder_twin_normal(port_: flutter_rust_bridge::MessagePort, a: i32, b: i32) {
    wire_simple_adder_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_func_stream_realistic_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: String,
) {
    wire_func_stream_realistic_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_error_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_stream_return_error_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_return_panic_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_func_stream_return_panic_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_func_stream_sink_arg_position_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    a: u32,
    b: u32,
) {
    wire_func_stream_sink_arg_position_twin_normal_impl(port_, a, b)
}

#[wasm_bindgen]
pub fn wire_handle_stream_of_struct_twin_normal(port_: flutter_rust_bridge::MessagePort) {
    wire_handle_stream_of_struct_twin_normal_impl(port_)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_1_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_handle_stream_sink_at_1_twin_normal_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_2_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_handle_stream_sink_at_2_twin_normal_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_handle_stream_sink_at_3_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    key: u32,
    max: u32,
) {
    wire_handle_stream_sink_at_3_twin_normal_impl(port_, key, max)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_one_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_two_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_struct_with_zero_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_struct_with_zero_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_one_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_tuple_struct_with_one_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_func_tuple_struct_with_two_field_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    arg: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_func_tuple_struct_with_two_field_twin_normal_impl(port_, arg)
}

#[wasm_bindgen]
pub fn wire_test_tuple_2_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    value: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_test_tuple_2_twin_normal_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_test_tuple_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    value: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_test_tuple_twin_normal_impl(port_, value)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_id_twin_normal(port_: flutter_rust_bridge::MessagePort, input: u64) {
    wire_handle_type_alias_id_twin_normal_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_alias_model_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    input: u64,
) {
    wire_handle_type_alias_model_twin_normal_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_type_nest_alias_id_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    input: u64,
) {
    wire_handle_type_nest_alias_id_twin_normal_impl(port_, input)
}

#[wasm_bindgen]
pub fn wire_handle_nested_uuids_twin_normal(
    port_: flutter_rust_bridge::MessagePort,
    ids: flutter_rust_bridge::wasm_bindgen::JsValue,
) {
    wire_handle_nested_uuids_twin_normal_impl(port_, ids)
}

#[wasm_bindgen]
pub fn wire_handle_uuid_twin_normal(port_: flutter_rust_bridge::MessagePort, id: Box<[u8]>) {
    wire_handle_uuid_twin_normal_impl(port_, id)
}

#[wasm_bindgen]
pub fn wire_handle_uuids_twin_normal(port_: flutter_rust_bridge::MessagePort, ids: Box<[u8]>) {
    wire_handle_uuids_twin_normal_impl(port_, ids)
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_MutexHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_MutexHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Mutex<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_RwLockHideData(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_RwLockHideData(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<RwLock<HideData>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_box_dynDartDebugTwinNormal(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinNormal>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_box_dynDartDebugTwinNormal(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinNormal>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_box_dynDartDebugTwinSync(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinSync>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_box_dynDartDebugTwinSync(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<Box<dyn DartDebugTwinSync>>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_frb_opaque_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_frb_opaque_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueReturn>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_frb_opaque_sync_return(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_frb_opaque_sync_return(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::FrbOpaqueSyncReturn>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_hide_data(ptr: *const std::ffi::c_void) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::HideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<i32>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_i_32(ptr: *const std::ffi::c_void) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<i32>::increment_strong_count(ptr as _);
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_non_clone_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_non_clone_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonCloneData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}

#[wasm_bindgen]
pub fn drop_opaque_RustOpaque_non_send_hide_data(ptr: *const std::ffi::c_void) {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::decrement_strong_count(
            ptr as _,
        );
    }
}

#[wasm_bindgen]
pub fn share_opaque_RustOpaque_non_send_hide_data(
    ptr: *const std::ffi::c_void,
) -> *const std::ffi::c_void {
    unsafe {
        std::sync::Arc::<crate::auxiliary::sample_types::NonSendHideData>::increment_strong_count(
            ptr as _,
        );
        ptr
    }
}
