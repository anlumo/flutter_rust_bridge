// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.41.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import '../dart_opaque.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'dart_opaque_twin_rust_async.freezed.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `deref`, `initialize`

Future<String> asyncAcceptDartOpaqueTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncAsyncAcceptDartOpaqueTwinRustAsync(
            opaque: opaque);

Future<Object> loopBackTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackTwinRustAsync(
            opaque: opaque);

Future<Object?> loopBackOptionTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackOptionTwinRustAsync(
            opaque: opaque);

Future<ObjectArray1> loopBackArrayTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackArrayTwinRustAsync(
            opaque: opaque);

Future<List<Object>> loopBackVecTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackVecTwinRustAsync(
            opaque: opaque);

Future<void> loopBackOptionGetTwinRustAsync({Object? opaque}) => RustLib
    .instance.api
    .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackOptionGetTwinRustAsync(
        opaque: opaque);

Future<void> loopBackArrayGetTwinRustAsync({required ObjectArray1 opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackArrayGetTwinRustAsync(
            opaque: opaque);

Future<void> loopBackVecGetTwinRustAsync({required List<Object> opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncLoopBackVecGetTwinRustAsync(
            opaque: opaque);

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
Future<void> panicUnwrapDartOpaqueTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncPanicUnwrapDartOpaqueTwinRustAsync(
            opaque: opaque);

Future<DartOpaqueNestedTwinRustAsync> createNestedDartOpaqueTwinRustAsync(
        {required Object opaque1, required Object opaque2}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncCreateNestedDartOpaqueTwinRustAsync(
            opaque1: opaque1, opaque2: opaque2);

Future<void> getNestedDartOpaqueTwinRustAsync(
        {required DartOpaqueNestedTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncGetNestedDartOpaqueTwinRustAsync(
            opaque: opaque);

Future<EnumDartOpaqueTwinRustAsync> createEnumDartOpaqueTwinRustAsync(
        {required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncCreateEnumDartOpaqueTwinRustAsync(
            opaque: opaque);

Future<void> getEnumDartOpaqueTwinRustAsync(
        {required EnumDartOpaqueTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncGetEnumDartOpaqueTwinRustAsync(
            opaque: opaque);

Future<void> setStaticDartOpaqueTwinRustAsync(
        {required int id, required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncSetStaticDartOpaqueTwinRustAsync(
            id: id, opaque: opaque);

Future<void> dropStaticDartOpaqueTwinRustAsync({required int id}) => RustLib
    .instance.api
    .crateApiPseudoManualDartOpaqueTwinRustAsyncDropStaticDartOpaqueTwinRustAsync(
        id: id);

Future<List<Object>> cloneDartOpaqueTwinRustAsync({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinRustAsyncCloneDartOpaqueTwinRustAsync(
            opaque: opaque);

class DartOpaqueNestedTwinRustAsync {
  final Object first;
  final Object second;

  const DartOpaqueNestedTwinRustAsync({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DartOpaqueNestedTwinRustAsync &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}

@freezed
sealed class EnumDartOpaqueTwinRustAsync with _$EnumDartOpaqueTwinRustAsync {
  const EnumDartOpaqueTwinRustAsync._();

  const factory EnumDartOpaqueTwinRustAsync.primitive(
    int field0,
  ) = EnumDartOpaqueTwinRustAsync_Primitive;
  const factory EnumDartOpaqueTwinRustAsync.opaque(
    Object field0,
  ) = EnumDartOpaqueTwinRustAsync_Opaque;
}
