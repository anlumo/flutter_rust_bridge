// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `deref`, `drop`, `initialize`

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DroppableTwinSync>>
abstract class DroppableTwinSync implements RustOpaqueInterface {
  Stream<int> createStreamTwinSync();

  static int getDropCountTwinSync() => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncDroppableTwinSyncGetDropCountTwinSync();

  static DroppableTwinSync newTwinSync() => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncDroppableTwinSyncNewTwinSync();

  void simpleMethodTwinSync();
}
