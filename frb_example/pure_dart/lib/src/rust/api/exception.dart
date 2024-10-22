// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'exception.freezed.dart';

Future<int> funcReturnErrorTwinNormal() =>
    RustLib.instance.api.crateApiExceptionFuncReturnErrorTwinNormal();

Future<int> funcTypeFalliblePanicTwinNormal() =>
    RustLib.instance.api.crateApiExceptionFuncTypeFalliblePanicTwinNormal();

Future<int> funcTypeInfalliblePanicTwinNormal() =>
    RustLib.instance.api.crateApiExceptionFuncTypeInfalliblePanicTwinNormal();

Future<int> customEnumErrorReturnOkTwinNormal({required int arg}) =>
    RustLib.instance.api
        .crateApiExceptionCustomEnumErrorReturnOkTwinNormal(arg: arg);

Future<void> customEnumErrorPanicTwinNormal() =>
    RustLib.instance.api.crateApiExceptionCustomEnumErrorPanicTwinNormal();

Future<int> customEnumErrorReturnErrorTwinNormal() => RustLib.instance.api
    .crateApiExceptionCustomEnumErrorReturnErrorTwinNormal();

Future<void> customNestedErrorReturnErrorTwinNormal(
        {required CustomNestedErrorOuterTwinNormal arg}) =>
    RustLib.instance.api
        .crateApiExceptionCustomNestedErrorReturnErrorTwinNormal(arg: arg);

Future<void> customStructErrorReturnErrorTwinNormal(
        {required CustomStructErrorTwinNormal arg}) =>
    RustLib.instance.api
        .crateApiExceptionCustomStructErrorReturnErrorTwinNormal(arg: arg);

Future<int> returnErrCustomErrorTwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnErrCustomErrorTwinNormal();

Future<int> returnOkCustomErrorTwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnOkCustomErrorTwinNormal();

Future<int> returnErrorVariantTwinNormal({required int variant}) =>
    RustLib.instance.api
        .crateApiExceptionReturnErrorVariantTwinNormal(variant: variant);

Future<void> returnCustomNestedError1TwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnCustomNestedError1TwinNormal();

Future<void> returnCustomNestedError1Variant1TwinNormal() =>
    RustLib.instance.api
        .crateApiExceptionReturnCustomNestedError1Variant1TwinNormal();

Future<void> returnCustomNestedError2TwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnCustomNestedError2TwinNormal();

Future<void> returnCustomStructErrorTwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnCustomStructErrorTwinNormal();

Future<int> returnCustomStructOkTwinNormal() =>
    RustLib.instance.api.crateApiExceptionReturnCustomStructOkTwinNormal();

Future<void> throwAnyhowTwinNormal() =>
    RustLib.instance.api.crateApiExceptionThrowAnyhowTwinNormal();

Future<void> panicWithCustomResultTwinNormal() =>
    RustLib.instance.api.crateApiExceptionPanicWithCustomResultTwinNormal();

Future<Stream<String>> streamSinkThrowAnyhowTwinNormal() =>
    RustLib.instance.api.crateApiExceptionStreamSinkThrowAnyhowTwinNormal();

@freezed
sealed class CustomEnumErrorTwinNormal
    with _$CustomEnumErrorTwinNormal
    implements FrbException {
  const CustomEnumErrorTwinNormal._();

  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.one({
    required String message,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_One;
  @Implements<FrbBacktracedException>()
  const factory CustomEnumErrorTwinNormal.two({
    required int message,
    required String backtrace,
  }) = CustomEnumErrorTwinNormal_Two;
}

@freezed
sealed class CustomErrorTwinNormal
    with _$CustomErrorTwinNormal
    implements FrbException {
  const CustomErrorTwinNormal._();

  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinNormal.error0({
    required String e,
    required String backtrace,
  }) = CustomErrorTwinNormal_Error0;
  @Implements<FrbBacktracedException>()
  const factory CustomErrorTwinNormal.error1({
    required int e,
    required String backtrace,
  }) = CustomErrorTwinNormal_Error1;
}

@freezed
sealed class CustomNestedError1TwinNormal
    with _$CustomNestedError1TwinNormal
    implements FrbException {
  const CustomNestedError1TwinNormal._();

  const factory CustomNestedError1TwinNormal.customNested1(
    String field0,
  ) = CustomNestedError1TwinNormal_CustomNested1;
  const factory CustomNestedError1TwinNormal.errorNested(
    CustomNestedError2TwinNormal field0,
  ) = CustomNestedError1TwinNormal_ErrorNested;
}

@freezed
sealed class CustomNestedError2TwinNormal with _$CustomNestedError2TwinNormal {
  const CustomNestedError2TwinNormal._();

  const factory CustomNestedError2TwinNormal.customNested2(
    String field0,
  ) = CustomNestedError2TwinNormal_CustomNested2;
  const factory CustomNestedError2TwinNormal.customNested2Number(
    int field0,
  ) = CustomNestedError2TwinNormal_CustomNested2Number;
}

@freezed
sealed class CustomNestedErrorInnerTwinNormal
    with _$CustomNestedErrorInnerTwinNormal {
  const CustomNestedErrorInnerTwinNormal._();

  const factory CustomNestedErrorInnerTwinNormal.three(
    String field0,
  ) = CustomNestedErrorInnerTwinNormal_Three;
  const factory CustomNestedErrorInnerTwinNormal.four(
    int field0,
  ) = CustomNestedErrorInnerTwinNormal_Four;
}

@freezed
sealed class CustomNestedErrorOuterTwinNormal
    with _$CustomNestedErrorOuterTwinNormal {
  const CustomNestedErrorOuterTwinNormal._();

  const factory CustomNestedErrorOuterTwinNormal.one(
    String field0,
  ) = CustomNestedErrorOuterTwinNormal_One;
  const factory CustomNestedErrorOuterTwinNormal.two(
    CustomNestedErrorInnerTwinNormal field0,
  ) = CustomNestedErrorOuterTwinNormal_Two;
}

class CustomStructErrorAnotherTwinNormal implements FrbException {
  final String message;

  const CustomStructErrorAnotherTwinNormal({
    required this.message,
  });

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorAnotherTwinNormal &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class CustomStructErrorTwinNormal {
  final String a;

  const CustomStructErrorTwinNormal({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructErrorTwinNormal &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class CustomStructTwinNormal {
  final String message;

  const CustomStructTwinNormal({
    required this.message,
  });

  static Future<CustomStructTwinNormal> newTwinNormal(
          {required String message}) =>
      RustLib.instance.api.crateApiExceptionCustomStructTwinNormalNewTwinNormal(
          message: message);

  Future<void> nonstaticReturnCustomStructErrorTwinNormal() =>
      RustLib.instance.api
          .crateApiExceptionCustomStructTwinNormalNonstaticReturnCustomStructErrorTwinNormal(
        that: this,
      );

  Future<int> nonstaticReturnCustomStructOkTwinNormal() => RustLib.instance.api
          .crateApiExceptionCustomStructTwinNormalNonstaticReturnCustomStructOkTwinNormal(
        that: this,
      );

  static Future<void> staticReturnCustomStructErrorTwinNormal() => RustLib
      .instance.api
      .crateApiExceptionCustomStructTwinNormalStaticReturnCustomStructErrorTwinNormal();

  static Future<int> staticReturnCustomStructOkTwinNormal() => RustLib
      .instance.api
      .crateApiExceptionCustomStructTwinNormalStaticReturnCustomStructOkTwinNormal();

  @override
  int get hashCode => message.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is CustomStructTwinNormal &&
          runtimeType == other.runtimeType &&
          message == other.message;
}

class SomeStructTwinNormal {
  final int value;

  const SomeStructTwinNormal({
    required this.value,
  });

  static Future<SomeStructTwinNormal> newTwinNormal({required int value}) =>
      RustLib.instance.api
          .crateApiExceptionSomeStructTwinNormalNewTwinNormal(value: value);

  Future<int> nonStaticReturnErrCustomErrorTwinNormal() => RustLib.instance.api
          .crateApiExceptionSomeStructTwinNormalNonStaticReturnErrCustomErrorTwinNormal(
        that: this,
      );

  Future<int> nonStaticReturnOkCustomErrorTwinNormal() => RustLib.instance.api
          .crateApiExceptionSomeStructTwinNormalNonStaticReturnOkCustomErrorTwinNormal(
        that: this,
      );

  static Future<int> staticReturnErrCustomErrorTwinNormal() => RustLib
      .instance.api
      .crateApiExceptionSomeStructTwinNormalStaticReturnErrCustomErrorTwinNormal();

  static Future<int> staticReturnOkCustomErrorTwinNormal() => RustLib
      .instance.api
      .crateApiExceptionSomeStructTwinNormalStaticReturnOkCustomErrorTwinNormal();

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SomeStructTwinNormal &&
          runtimeType == other.runtimeType &&
          value == other.value;
}
