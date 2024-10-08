// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:collection/collection.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'rust_opaque_twin_rust_async.freezed.dart';

Future<HideDataTwinRustAsync> createOpaqueTwinRustAsync() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncCreateOpaqueTwinRustAsync();

Future<HideDataTwinRustAsync?> createOptionOpaqueTwinRustAsync(
        {HideDataTwinRustAsync? opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncCreateOptionOpaqueTwinRustAsync(
            opaque: opaque);

Future<EnumOpaqueTwinRustAsyncArray5> createArrayOpaqueEnumTwinRustAsync() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncCreateArrayOpaqueEnumTwinRustAsync();

Future<String> runEnumOpaqueTwinRustAsync(
        {required EnumOpaqueTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncRunEnumOpaqueTwinRustAsync(
            opaque: opaque);

Future<String> runOpaqueTwinRustAsync(
        {required HideDataTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncRunOpaqueTwinRustAsync(
            opaque: opaque);

Future<String> runOpaqueWithDelayTwinRustAsync(
        {required HideDataTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncRunOpaqueWithDelayTwinRustAsync(
            opaque: opaque);

Future<HideDataTwinRustAsyncArray2> opaqueArrayTwinRustAsync() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncOpaqueArrayTwinRustAsync();

Future<String> runNonCloneTwinRustAsync(
        {required NonCloneDataTwinRustAsync clone}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncRunNonCloneTwinRustAsync(
            clone: clone);

Future<void> opaqueArrayRunTwinRustAsync(
        {required HideDataTwinRustAsyncArray2 data}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncOpaqueArrayRunTwinRustAsync(
            data: data);

Future<List<HideDataTwinRustAsync>> opaqueVecTwinRustAsync() =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncOpaqueVecTwinRustAsync();

Future<void> opaqueVecRunTwinRustAsync(
        {required List<HideDataTwinRustAsync> data}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncOpaqueVecRunTwinRustAsync(
            data: data);

Future<OpaqueNestedTwinRustAsync> createNestedOpaqueTwinRustAsync() => RustLib
    .instance.api
    .crateApiPseudoManualRustOpaqueTwinRustAsyncCreateNestedOpaqueTwinRustAsync();

Future<void> runNestedOpaqueTwinRustAsync(
        {required OpaqueNestedTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncRunNestedOpaqueTwinRustAsync(
            opaque: opaque);

Future<String> unwrapRustOpaqueTwinRustAsync(
        {required HideDataTwinRustAsync opaque}) =>
    RustLib.instance.api
        .crateApiPseudoManualRustOpaqueTwinRustAsyncUnwrapRustOpaqueTwinRustAsync(
            opaque: opaque);

/// Function to check the code generator.
/// FrbOpaqueReturn must be only return type.
/// FrbOpaqueReturn must not be used as an argument.
Future<FrbOpaqueReturnTwinRustAsync> frbGeneratorTestTwinRustAsync() => RustLib
    .instance.api
    .crateApiPseudoManualRustOpaqueTwinRustAsyncFrbGeneratorTestTwinRustAsync();

// Rust type: RustOpaqueNom<Box < dyn DartDebugTwinRustAsync >>
abstract class BoxDartDebugTwinRustAsync implements RustOpaqueInterface {}

// Rust type: RustOpaqueNom<FrbOpaqueReturnTwinRustAsync>
abstract class FrbOpaqueReturnTwinRustAsync implements RustOpaqueInterface {}

// Rust type: RustOpaqueNom<HideDataTwinRustAsync>
abstract class HideDataTwinRustAsync implements RustOpaqueInterface {}

class HideDataTwinRustAsyncArray2
    extends NonGrowableListView<HideDataTwinRustAsync> {
  static const arraySize = 2;

  @internal
  List<HideDataTwinRustAsync> get inner => _inner;
  final List<HideDataTwinRustAsync> _inner;

  HideDataTwinRustAsyncArray2(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  HideDataTwinRustAsyncArray2.init(HideDataTwinRustAsync fill)
      : this(List<HideDataTwinRustAsync>.filled(arraySize, fill));
}

// Rust type: RustOpaqueNom<Mutex < HideDataTwinRustAsync >>
abstract class MutexHideDataTwinRustAsync implements RustOpaqueInterface {}

// Rust type: RustOpaqueNom<NonCloneDataTwinRustAsync>
abstract class NonCloneDataTwinRustAsync implements RustOpaqueInterface {}

// Rust type: RustOpaqueNom<RwLock < HideDataTwinRustAsync >>
abstract class RwLockHideDataTwinRustAsync implements RustOpaqueInterface {}

// Rust type: RustOpaqueNom<i32>
abstract class I32 implements RustOpaqueInterface {}

@freezed
sealed class EnumOpaqueTwinRustAsync with _$EnumOpaqueTwinRustAsync {
  const EnumOpaqueTwinRustAsync._();

  const factory EnumOpaqueTwinRustAsync.struct(
    HideDataTwinRustAsync field0,
  ) = EnumOpaqueTwinRustAsync_Struct;
  const factory EnumOpaqueTwinRustAsync.primitive(
    I32 field0,
  ) = EnumOpaqueTwinRustAsync_Primitive;
  const factory EnumOpaqueTwinRustAsync.traitObj(
    BoxDartDebugTwinRustAsync field0,
  ) = EnumOpaqueTwinRustAsync_TraitObj;
  const factory EnumOpaqueTwinRustAsync.mutex(
    MutexHideDataTwinRustAsync field0,
  ) = EnumOpaqueTwinRustAsync_Mutex;
  const factory EnumOpaqueTwinRustAsync.rwLock(
    RwLockHideDataTwinRustAsync field0,
  ) = EnumOpaqueTwinRustAsync_RwLock;
  const factory EnumOpaqueTwinRustAsync.nothing() =
      EnumOpaqueTwinRustAsync_Nothing;
}

class EnumOpaqueTwinRustAsyncArray5
    extends NonGrowableListView<EnumOpaqueTwinRustAsync> {
  static const arraySize = 5;

  @internal
  List<EnumOpaqueTwinRustAsync> get inner => _inner;
  final List<EnumOpaqueTwinRustAsync> _inner;

  EnumOpaqueTwinRustAsyncArray5(this._inner)
      : assert(_inner.length == arraySize),
        super(_inner);

  EnumOpaqueTwinRustAsyncArray5.init(EnumOpaqueTwinRustAsync fill)
      : this(List<EnumOpaqueTwinRustAsync>.filled(arraySize, fill));
}

/// [`HideDataTwinRustAsync`] has private fields.
class OpaqueNestedTwinRustAsync {
  final HideDataTwinRustAsync first;
  final HideDataTwinRustAsync second;

  const OpaqueNestedTwinRustAsync({
    required this.first,
    required this.second,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is OpaqueNestedTwinRustAsync &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second;
}
