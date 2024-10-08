// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<MyAudioParamTwinNormal>>
abstract class MyAudioParamTwinNormal implements RustOpaqueInterface {
  static Future<MyAudioParamTwinNormal> createTwinNormal(
          {required String value}) =>
      RustLib.instance.api
          .crateApiProxyMyAudioParamTwinNormalCreateTwinNormal(value: value);

  Future<String> myMethodTwinNormal();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<MyNodeTwinNormal>>
abstract class MyNodeTwinNormal implements RustOpaqueInterface {
  static Future<MyNodeTwinNormal> createTwinNormal() =>
      RustLib.instance.api.crateApiProxyMyNodeTwinNormalCreateTwinNormal();

  Future<MyAudioParamTwinNormal> paramOneTwinNormal();

  Future<MyAudioParamTwinNormal> paramTwoTwinNormal();
}
