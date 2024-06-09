// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.38.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import '../dart_opaque.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'dart_opaque_twin_sync_sse.freezed.dart';

String asyncAcceptDartOpaqueTwinSyncSse({required Object opaque}) => RustLib
    .instance.api
    .crateApiPseudoManualDartOpaqueTwinSyncSseAsyncAcceptDartOpaqueTwinSyncSse(
        opaque: opaque);

Object loopBackTwinSyncSse({required Object opaque}) => RustLib.instance.api
    .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackTwinSyncSse(
        opaque: opaque);

Object? loopBackOptionTwinSyncSse({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackOptionTwinSyncSse(
            opaque: opaque);

ObjectArray1 loopBackArrayTwinSyncSse({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackArrayTwinSyncSse(
            opaque: opaque);

List<Object> loopBackVecTwinSyncSse({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackVecTwinSyncSse(
            opaque: opaque);

void loopBackOptionGetTwinSyncSse({Object? opaque}) => RustLib.instance.api
    .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackOptionGetTwinSyncSse(
        opaque: opaque);

void loopBackArrayGetTwinSyncSse({required ObjectArray1 opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackArrayGetTwinSyncSse(
            opaque: opaque);

void loopBackVecGetTwinSyncSse({required List<Object> opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseLoopBackVecGetTwinSyncSse(
            opaque: opaque);

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
void panicUnwrapDartOpaqueTwinSyncSse({required Object opaque}) => RustLib
    .instance.api
    .crateApiPseudoManualDartOpaqueTwinSyncSsePanicUnwrapDartOpaqueTwinSyncSse(
        opaque: opaque);

DartOpaqueNestedTwinSyncSse createNestedDartOpaqueTwinSyncSse(
        {required Object opaque1, required Object opaque2}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseCreateNestedDartOpaqueTwinSyncSse(
            opaque1: opaque1, opaque2: opaque2);

void getNestedDartOpaqueTwinSyncSse(
        {required DartOpaqueNestedTwinSyncSse opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseGetNestedDartOpaqueTwinSyncSse(
            opaque: opaque);

EnumDartOpaqueTwinSyncSse createEnumDartOpaqueTwinSyncSse(
        {required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseCreateEnumDartOpaqueTwinSyncSse(
            opaque: opaque);

void getEnumDartOpaqueTwinSyncSse(
        {required EnumDartOpaqueTwinSyncSse opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseGetEnumDartOpaqueTwinSyncSse(
            opaque: opaque);

void setStaticDartOpaqueTwinSyncSse(
        {required int id, required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseSetStaticDartOpaqueTwinSyncSse(
            id: id, opaque: opaque);

void dropStaticDartOpaqueTwinSyncSse({required int id}) => RustLib.instance.api
    .crateApiPseudoManualDartOpaqueTwinSyncSseDropStaticDartOpaqueTwinSyncSse(
        id: id);

List<Object> cloneDartOpaqueTwinSyncSse({required Object opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualDartOpaqueTwinSyncSseCloneDartOpaqueTwinSyncSse(
            opaque: opaque);

class DartOpaqueNestedTwinSyncSse {
  final Object first;
  final Object second;

  const DartOpaqueNestedTwinSyncSse({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is DartOpaqueNestedTwinSyncSse &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}

@freezed
sealed class EnumDartOpaqueTwinSyncSse with _$EnumDartOpaqueTwinSyncSse {
  const EnumDartOpaqueTwinSyncSse._();

  const factory EnumDartOpaqueTwinSyncSse.primitive(
    int field0,
  ) = EnumDartOpaqueTwinSyncSse_Primitive;
  const factory EnumDartOpaqueTwinSyncSse.opaque(
    Object field0,
  ) = EnumDartOpaqueTwinSyncSse_Opaque;
}
