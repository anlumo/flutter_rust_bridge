// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
import 'rust_opaque_sync_twin_sse_moi.dart';
import 'rust_opaque_twin_moi.dart';
part 'rust_opaque_twin_sse_moi.freezed.dart';

Future<HideDataTwinSseMoi> createOpaqueTwinSseMoi() => RustLib.instance.api
    .crateApiPseudoManualRustOpaqueTwinSseMoiCreateOpaqueTwinSseMoi();

Future<HideDataTwinSseMoi?> createOptionOpaqueTwinSseMoi(
        {HideDataTwinSseMoi? opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiCreateOptionOpaqueTwinSseMoi(
            opaque: opaque);

Future<EnumOpaqueTwinSseMoiArray5> createArrayOpaqueEnumTwinSseMoi() => RustLib
    .instance.api
    .crateApiPseudoManualRustOpaqueTwinSseMoiCreateArrayOpaqueEnumTwinSseMoi();

Future<String> runEnumOpaqueTwinSseMoi(
        {required EnumOpaqueTwinSseMoi opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiRunEnumOpaqueTwinSseMoi(
            opaque: opaque);

Future<String> runOpaqueTwinSseMoi({required HideDataTwinSseMoi opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiRunOpaqueTwinSseMoi(
            opaque: opaque);

Future<String> runOpaqueWithDelayTwinSseMoi(
        {required HideDataTwinSseMoi opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiRunOpaqueWithDelayTwinSseMoi(
            opaque: opaque);

Future<HideDataTwinSseMoiArray2> opaqueArrayTwinSseMoi() => RustLib.instance.api
    .crateApiPseudoManualRustOpaqueTwinSseMoiOpaqueArrayTwinSseMoi();

Future<String> runNonCloneTwinSseMoi({required NonCloneDataTwinSseMoi clone}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiRunNonCloneTwinSseMoi(
            clone: clone);

Future<void> opaqueArrayRunTwinSseMoi(
        {required HideDataTwinSseMoiArray2 data}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiOpaqueArrayRunTwinSseMoi(
            data: data);

Future<List<HideDataTwinSseMoi>> opaqueVecTwinSseMoi() => RustLib.instance.api
    .crateApiPseudoManualRustOpaqueTwinSseMoiOpaqueVecTwinSseMoi();

Future<void> opaqueVecRunTwinSseMoi({required List<HideDataTwinSseMoi> data}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiOpaqueVecRunTwinSseMoi(
            data: data);

Future<OpaqueNestedTwinSseMoi> createNestedOpaqueTwinSseMoi() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiCreateNestedOpaqueTwinSseMoi();

Future<void> runNestedOpaqueTwinSseMoi(
        {required OpaqueNestedTwinSseMoi opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiRunNestedOpaqueTwinSseMoi(
            opaque: opaque);

Future<String> unwrapRustOpaqueTwinSseMoi(
        {required HideDataTwinSseMoi opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiUnwrapRustOpaqueTwinSseMoi(
            opaque: opaque);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
Future<FrbOpaqueReturnTwinSseMoi> frbGeneratorTestTwinSseMoi() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinSseMoiFrbGeneratorTestTwinSseMoi();

// Rust type: RustOpaqueMoi<Box < dyn DartDebugTwinSseMoi >>
abstract class BoxDartDebugTwinSseMoi implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<FrbOpaqueReturnTwinSseMoi>
abstract class FrbOpaqueReturnTwinSseMoi implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<HideDataTwinSseMoi>
abstract class HideDataTwinSseMoi implements RustOpaqueInterface {}

class HideDataTwinSseMoiArray2 extends NonGrowableListView<HideDataTwinSseMoi> {
  static const arraySize = 2;

  @internal
  List<HideDataTwinSseMoi> get inner => _inner;
  final List<HideDataTwinSseMoi> _inner;

  HideDataTwinSseMoiArray2(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  HideDataTwinSseMoiArray2.init(HideDataTwinSseMoi fill)
      : this(List<HideDataTwinSseMoi>.filled(arraySize, fill));
}

// Rust type: RustOpaqueMoi<Mutex < HideDataTwinSseMoi >>
abstract class MutexHideDataTwinSseMoi implements RustOpaqueInterface {}

// Rust type: RustOpaqueMoi<RwLock < HideDataTwinSseMoi >>
abstract class RwLockHideDataTwinSseMoi implements RustOpaqueInterface {}

@freezed
sealed class EnumOpaqueTwinSseMoi with _$EnumOpaqueTwinSseMoi {
  const EnumOpaqueTwinSseMoi._();

  const factory EnumOpaqueTwinSseMoi.struct(
    HideDataTwinSseMoi field0,
  ) = EnumOpaqueTwinSseMoi_Struct;
  const factory EnumOpaqueTwinSseMoi.primitive(
    I16 field0,
  ) = EnumOpaqueTwinSseMoi_Primitive;
  const factory EnumOpaqueTwinSseMoi.traitObj(
    BoxDartDebugTwinSseMoi field0,
  ) = EnumOpaqueTwinSseMoi_TraitObj;
  const factory EnumOpaqueTwinSseMoi.mutex(
    MutexHideDataTwinSseMoi field0,
  ) = EnumOpaqueTwinSseMoi_Mutex;
  const factory EnumOpaqueTwinSseMoi.rwLock(
    RwLockHideDataTwinSseMoi field0,
  ) = EnumOpaqueTwinSseMoi_RwLock;
  const factory EnumOpaqueTwinSseMoi.nothing() = EnumOpaqueTwinSseMoi_Nothing;
}

class EnumOpaqueTwinSseMoiArray5
    extends NonGrowableListView<EnumOpaqueTwinSseMoi> {
  static const arraySize = 5;

  @internal
  List<EnumOpaqueTwinSseMoi> get inner => _inner;
  final List<EnumOpaqueTwinSseMoi> _inner;

  EnumOpaqueTwinSseMoiArray5(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  EnumOpaqueTwinSseMoiArray5.init(EnumOpaqueTwinSseMoi fill)
      : this(List<EnumOpaqueTwinSseMoi>.filled(arraySize, fill));
}

/// [`HideDataTwinSseMoi`] has private fields.
class OpaqueNestedTwinSseMoi {
  final HideDataTwinSseMoi first;
  final HideDataTwinSseMoi second;

  const OpaqueNestedTwinSseMoi({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNestedTwinSseMoi &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
