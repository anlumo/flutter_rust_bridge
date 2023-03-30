// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.69.0.
// ignore_for_file: non_constant_identifier_names, unused_element, duplicate_ignore, directives_ordering, curly_braces_in_flow_control_structures, unnecessary_lambdas, slash_for_doc_comments, prefer_const_literals_to_create_immutables, implicit_dynamic_list_literal, duplicate_import, unused_import, unnecessary_import, prefer_single_quotes, prefer_const_constructors, use_super_parameters, always_use_package_imports, annotate_overrides, invalid_use_of_protected_member, constant_identifier_names, invalid_use_of_internal_member, prefer_is_empty, unnecessary_const

import 'dart:convert';
import 'dart:async';
import 'package:meta/meta.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge.dart';
import 'package:uuid/uuid.dart';
import 'bridge_generated_shares.rs.dart';
export 'bridge_generated_shares.rs.dart';

class BridgeGeneratedSharesPlatform extends FlutterRustBridgeBase<BridgeGeneratedSharesWire>
    with FlutterRustBridgeSetupMixin {
  BridgeGeneratedSharesPlatform(FutureOr<WasmModule> dylib) : super(BridgeGeneratedSharesWire(dylib)) {
    setupMixinConstructor();
  }
  Future<void> setup() => inner.init;

// Section: api2wire

  @protected
  String api2wire_String(String raw) {
    return raw;
  }

  @protected
  List<dynamic> api2wire_box_autoadd_cross_shared_struct(CrossSharedStruct raw) {
    return api2wire_cross_shared_struct(raw);
  }

  @protected
  List<dynamic> api2wire_box_autoadd_shared_struct(SharedStruct raw) {
    return api2wire_shared_struct(raw);
  }

  @protected
  List<dynamic> api2wire_cross_shared_struct(CrossSharedStruct raw) {
    return [api2wire_String(raw.name)];
  }

  @protected
  List<dynamic> api2wire_shared_struct(SharedStruct raw) {
    return [api2wire_i32(raw.id), api2wire_f64(raw.num), api2wire_String(raw.name)];
  }

  @protected
  Object api2wire_u64(int raw) {
    return castNativeBigInt(raw);
  }

  @protected
  Uint8List api2wire_uint_8_list(Uint8List raw) {
    return raw;
  }
// Section: finalizer
}

// Section: WASM wire module

@JS('wasm_bindgen')
external BridgeGeneratedSharesWasmModule get wasmModule;

@JS()
@anonymous
class BridgeGeneratedSharesWasmModule implements WasmModule {
  external Object /* Promise */ call([String? moduleName]);
  external BridgeGeneratedSharesWasmModule bind(dynamic thisArg, String moduleName);
}

// Section: WASM wire connector

class BridgeGeneratedSharesWire extends FlutterRustBridgeWasmWireBase<BridgeGeneratedSharesWasmModule> {
  BridgeGeneratedSharesWire(FutureOr<WasmModule> module)
      : super(WasmModule.cast<BridgeGeneratedSharesWasmModule>(module));
}
