// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'package:meta/meta.dart' as meta;
part 'attribute_twin_rust_async.freezed.dart';

// These types are ignored because they are not used by any `pub` functions: `IgnoredStructTwinRustAsync`, `StructWithOnlyIgnoredMethodTwinRustAsync`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `fmt`
// These functions are ignored (category: IgnoreBecauseExplicitAttribute): `func_should_not_exist_in_dart_twin_rust_async`, `method_should_not_exist_in_dart_twin_rust_async`
// These functions are ignored (category: IgnoreBecauseOwnerTyShouldIgnore): `method_should_not_exist_in_dart_twin_rust_async`

Future<void> handleCustomizedStructTwinRustAsync(
        {required CustomizedTwinRustAsync val}) =>
    RustLib.instance.api
        .crateApiPseudoManualAttributeTwinRustAsyncHandleCustomizedStructTwinRustAsync(
            val: val);

Future<UserIdTwinRustAsync> nextUserIdTwinRustAsync(
        {UserIdTwinRustAsync userId = const UserIdTwinRustAsync()}) =>
    RustLib.instance.api
        .crateApiPseudoManualAttributeTwinRustAsyncNextUserIdTwinRustAsync(
            userId: userId);

class CustomizedTwinRustAsync {
  final String finalField;
  String? nonFinalField;

  CustomizedTwinRustAsync({
    required this.finalField,
    this.nonFinalField,
  });

  @override
  int get hashCode => finalField.hashCode ^ nonFinalField.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomizedTwinRustAsync &&
          runtimeType == other.runtimeType &&
          finalField == other.finalField &&
          nonFinalField == other.nonFinalField;
}

/// Example for @freezed and @meta.immutable
@freezed
@meta.immutable
class UserIdTwinRustAsync with _$UserIdTwinRustAsync {
  const factory UserIdTwinRustAsync({
    @Default(0) int value,
  }) = _UserIdTwinRustAsync;
}
