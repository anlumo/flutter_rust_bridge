// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.39.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> minimalAdder({required int a, required int b}) =>
    RustLib.instance.api.crateApiMinimalMinimalAdder(a: a, b: b);

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<LifetimeTesterOneTwinNormal>>
abstract class LifetimeTesterOneTwinNormal implements RustOpaqueInterface {
  Future<LifetimeTesterTwoTwinNormal> computeTwo();
}

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<flutter_rust_bridge :: for_generated :: Lifetimeable < LifetimeTesterTwoTwinNormal < 'static > >>>
abstract class FlutterRustBridgeForGeneratedLifetimeableLifetimeTesterTwoTwinNormal
    implements RustOpaqueInterface {}
