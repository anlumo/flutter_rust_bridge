// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.18.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import '../dart_opaque.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'dart_opaque_twin_sync_sse.freezed.dart';

String asyncAcceptDartOpaqueTwinSyncSse(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .asyncAcceptDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

Object loopBackTwinSyncSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackTwinSyncSse(opaque: opaque, hint: hint);

Object? loopBackOptionTwinSyncSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackOptionTwinSyncSse(opaque: opaque, hint: hint);

ObjectArray1 loopBackArrayTwinSyncSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackArrayTwinSyncSse(opaque: opaque, hint: hint);

List<Object> loopBackVecTwinSyncSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVecTwinSyncSse(opaque: opaque, hint: hint);

void loopBackOptionGetTwinSyncSse({Object? opaque, dynamic hint}) =>
    RustLib.instance.api
        .loopBackOptionGetTwinSyncSse(opaque: opaque, hint: hint);

void loopBackArrayGetTwinSyncSse(
        {required ObjectArray1 opaque, dynamic hint}) =>
    RustLib.instance.api
        .loopBackArrayGetTwinSyncSse(opaque: opaque, hint: hint);

void loopBackVecGetTwinSyncSse({required List<Object> opaque, dynamic hint}) =>
    RustLib.instance.api.loopBackVecGetTwinSyncSse(opaque: opaque, hint: hint);

/// [DartWrapObject] cannot be obtained
/// on a thread other than the thread it was created on.
void panicUnwrapDartOpaqueTwinSyncSse({required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .panicUnwrapDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

DartOpaqueNestedTwinSyncSse createNestedDartOpaqueTwinSyncSse(
        {required Object opaque1, required Object opaque2, dynamic hint}) =>
    RustLib.instance.api.createNestedDartOpaqueTwinSyncSse(
        opaque1: opaque1, opaque2: opaque2, hint: hint);

void getNestedDartOpaqueTwinSyncSse(
        {required DartOpaqueNestedTwinSyncSse opaque, dynamic hint}) =>
    RustLib.instance.api
        .getNestedDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

EnumDartOpaqueTwinSyncSse createEnumDartOpaqueTwinSyncSse(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .createEnumDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

void getEnumDartOpaqueTwinSyncSse(
        {required EnumDartOpaqueTwinSyncSse opaque, dynamic hint}) =>
    RustLib.instance.api
        .getEnumDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

void setStaticDartOpaqueTwinSyncSse(
        {required int id, required Object opaque, dynamic hint}) =>
    RustLib.instance.api
        .setStaticDartOpaqueTwinSyncSse(id: id, opaque: opaque, hint: hint);

void dropStaticDartOpaqueTwinSyncSse({required int id, dynamic hint}) =>
    RustLib.instance.api.dropStaticDartOpaqueTwinSyncSse(id: id, hint: hint);

List<Object> cloneDartOpaqueTwinSyncSse(
        {required Object opaque, dynamic hint}) =>
    RustLib.instance.api.cloneDartOpaqueTwinSyncSse(opaque: opaque, hint: hint);

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
  const factory EnumDartOpaqueTwinSyncSse.primitive(
    int field0,
  ) = EnumDartOpaqueTwinSyncSse_Primitive;
  const factory EnumDartOpaqueTwinSyncSse.opaque(
    Object field0,
  ) = EnumDartOpaqueTwinSyncSse_Opaque;
}
