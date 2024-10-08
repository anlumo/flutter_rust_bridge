// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`

UuidValue handleUuidTwinSyncSse({required UuidValue id}) => RustLib.instance.api
    .crateApiPseudoManualUuidTypeTwinSyncSseHandleUuidTwinSyncSse(id: id);

List<UuidValue> handleUuidsTwinSyncSse({required List<UuidValue> ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncSseHandleUuidsTwinSyncSse(
            ids: ids);

FeatureUuidTwinSyncSse handleNestedUuidsTwinSyncSse(
        {required FeatureUuidTwinSyncSse ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncSseHandleNestedUuidsTwinSyncSse(
            ids: ids);

class FeatureUuidTwinSyncSse {
  final UuidValue one;

  const FeatureUuidTwinSyncSse({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinSyncSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
