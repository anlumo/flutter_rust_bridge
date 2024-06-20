// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.41.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:uuid/uuid.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`

Future<UuidValue> handleUuidTwinRustAsync({required UuidValue id}) => RustLib
    .instance.api
    .crateApiPseudoManualUuidTypeTwinRustAsyncHandleUuidTwinRustAsync(id: id);

Future<List<UuidValue>> handleUuidsTwinRustAsync(
        {required List<UuidValue> ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinRustAsyncHandleUuidsTwinRustAsync(
            ids: ids);

Future<FeatureUuidTwinRustAsync> handleNestedUuidsTwinRustAsync(
        {required FeatureUuidTwinRustAsync ids}) =>
    RustLib.instance.api
        .crateApiPseudoManualUuidTypeTwinRustAsyncHandleNestedUuidsTwinRustAsync(
            ids: ids);

class FeatureUuidTwinRustAsync {
  final UuidValue one;

  const FeatureUuidTwinRustAsync({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is FeatureUuidTwinRustAsync &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
