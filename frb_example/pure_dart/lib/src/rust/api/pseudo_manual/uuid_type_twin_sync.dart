// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`

UuidValue handleUuidTwinSync({required UuidValue id}) => RustLib.instance.api
    .crateApiPseudoManualUuidTypeTwinSyncHandleUuidTwinSync(id: id);

List<UuidValue> handleUuidsTwinSync({required List<UuidValue> ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncHandleUuidsTwinSync(ids: ids);

FeatureUuidTwinSync handleNestedUuidsTwinSync(
        {required FeatureUuidTwinSync ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinSyncHandleNestedUuidsTwinSync(
            ids: ids);

class FeatureUuidTwinSync {
  final UuidValue one;

  const FeatureUuidTwinSync({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinSync &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
