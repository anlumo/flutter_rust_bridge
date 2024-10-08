// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'basic_twin_rust_async.freezed.dart';

Future<int> exampleBasicTypeI8TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeI8TwinRustAsync(
            arg: arg, expect: expect);

Future<int> exampleBasicTypeI16TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeI16TwinRustAsync(
            arg: arg, expect: expect);

Future<int> exampleBasicTypeI32TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeI32TwinRustAsync(
            arg: arg, expect: expect);

Future<PlatformInt64> exampleBasicTypeI64TwinRustAsync(
        {required PlatformInt64 arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeI64TwinRustAsync(
            arg: arg, expect: expect);

Future<BigInt> exampleBasicTypeI128TwinRustAsync(
        {required BigInt arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeI128TwinRustAsync(
            arg: arg, expect: expect);

Future<int> exampleBasicTypeU8TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeU8TwinRustAsync(
            arg: arg, expect: expect);

Future<int> exampleBasicTypeU16TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeU16TwinRustAsync(
            arg: arg, expect: expect);

Future<int> exampleBasicTypeU32TwinRustAsync(
        {required int arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeU32TwinRustAsync(
            arg: arg, expect: expect);

Future<BigInt> exampleBasicTypeU64TwinRustAsync(
        {required BigInt arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeU64TwinRustAsync(
            arg: arg, expect: expect);

Future<BigInt> exampleBasicTypeU128TwinRustAsync(
        {required BigInt arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeU128TwinRustAsync(
            arg: arg, expect: expect);

Future<PlatformInt64> exampleBasicTypeIsizeTwinRustAsync(
        {required PlatformInt64 arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeIsizeTwinRustAsync(
            arg: arg, expect: expect);

Future<BigInt> exampleBasicTypeUsizeTwinRustAsync(
        {required BigInt arg, required String expect}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeUsizeTwinRustAsync(
            arg: arg, expect: expect);

Future<double> exampleBasicTypeF32TwinRustAsync({required double arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeF32TwinRustAsync(
            arg: arg);

Future<double> exampleBasicTypeF64TwinRustAsync({required double arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeF64TwinRustAsync(
            arg: arg);

Future<bool> exampleBasicTypeBoolTwinRustAsync({required bool arg}) => RustLib
    .instance.api
    .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeBoolTwinRustAsync(
        arg: arg);

Future<String> exampleBasicTypeStringTwinRustAsync({required String arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeStringTwinRustAsync(
            arg: arg);

Future<Uint8List> exampleBasicTypeBytesTwinRustAsync(
        {required List<int> arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeBytesTwinRustAsync(
            arg: arg);

Future<BasicPrimitiveEnumTwinRustAsync>
    exampleBasicTypeBasicPrimitiveEnumTwinRustAsyncTwinRustAsync(
            {required BasicPrimitiveEnumTwinRustAsync arg}) =>
        RustLib.instance.api
            .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeBasicPrimitiveEnumTwinRustAsyncTwinRustAsync(
                arg: arg);

Future<BasicGeneralEnumTwinRustAsync>
    exampleBasicTypeBasicGeneralEnumTwinRustAsyncTwinRustAsync(
            {required BasicGeneralEnumTwinRustAsync arg}) =>
        RustLib.instance.api
            .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeBasicGeneralEnumTwinRustAsyncTwinRustAsync(
                arg: arg);

Future<BasicStructTwinRustAsync>
    exampleBasicTypeBasicStructTwinRustAsyncTwinRustAsync(
            {required BasicStructTwinRustAsync arg}) =>
        RustLib.instance.api
            .crateApiPseudoManualBasicTwinRustAsyncExampleBasicTypeBasicStructTwinRustAsyncTwinRustAsync(
                arg: arg);

@freezed
sealed class BasicGeneralEnumTwinRustAsync
    with _$BasicGeneralEnumTwinRustAsync {
  const BasicGeneralEnumTwinRustAsync._();

  const factory BasicGeneralEnumTwinRustAsync.apple({
    required String field,
  }) = BasicGeneralEnumTwinRustAsync_Apple;
  const factory BasicGeneralEnumTwinRustAsync.orange() =
      BasicGeneralEnumTwinRustAsync_Orange;
}

enum BasicPrimitiveEnumTwinRustAsync {
  apple,
  orange,
  ;
}

class BasicStructTwinRustAsync {
  final String? apple;
  final int? orange;

  const BasicStructTwinRustAsync({
    this.apple,
    this.orange,
  });

  @override
  int get hashCode => apple.hashCode ^ orange.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BasicStructTwinRustAsync &&
          runtimeType == other.runtimeType &&
          apple == other.apple &&
          orange == other.orange;
}
