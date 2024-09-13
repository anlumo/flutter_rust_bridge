// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

StructWithZeroFieldTwinSyncSse funcStructWithZeroFieldTwinSyncSse(
        {required StructWithZeroFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncStructWithZeroFieldTwinSyncSse(
            arg: arg);

StructWithOneFieldTwinSyncSse funcStructWithOneFieldTwinSyncSse(
        {required StructWithOneFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncStructWithOneFieldTwinSyncSse(
            arg: arg);

StructWithTwoFieldTwinSyncSse funcStructWithTwoFieldTwinSyncSse(
        {required StructWithTwoFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncStructWithTwoFieldTwinSyncSse(
            arg: arg);

TupleStructWithOneFieldTwinSyncSse funcTupleStructWithOneFieldTwinSyncSse(
        {required TupleStructWithOneFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncTupleStructWithOneFieldTwinSyncSse(
            arg: arg);

TupleStructWithTwoFieldTwinSyncSse funcTupleStructWithTwoFieldTwinSyncSse(
        {required TupleStructWithTwoFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncTupleStructWithTwoFieldTwinSyncSse(
            arg: arg);

StructWithFieldRenameTwinSyncSse funcForStructWithFieldRenameTwinSyncSse(
        {required StructWithFieldRenameTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncForStructWithFieldRenameTwinSyncSse(
            arg: arg);

StructWithDartKeywordFieldTwinSyncSse funcForStructWithDartKeywordFieldTwinSyncSse(
        {required StructWithDartKeywordFieldTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualStructureTwinSyncSseFuncForStructWithDartKeywordFieldTwinSyncSse(
            arg: arg);

class StructWithDartKeywordFieldTwinSyncSse {
  final int class_;
  final PlatformInt64 interface_;

  const StructWithDartKeywordFieldTwinSyncSse({
    required this.class_,
    required this.interface_,
  });

  @override
  int get hashCode => class_.hashCode ^ interface_.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithDartKeywordFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          class_ == other.class_ &&
          interface_ == other.interface_;
}

class StructWithFieldRenameTwinSyncSse {
  final int renamed_field;

  const StructWithFieldRenameTwinSyncSse({
    required this.renamed_field,
  });

  @override
  int get hashCode => renamed_field.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithFieldRenameTwinSyncSse &&
          runtimeType == other.runtimeType &&
          renamed_field == other.renamed_field;
}

class StructWithOneFieldTwinSyncSse {
  final int a;

  const StructWithOneFieldTwinSyncSse({
    required this.a,
  });

  @override
  int get hashCode => a.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithOneFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a;
}

class StructWithTwoFieldTwinSyncSse {
  final int a;
  final int b;

  const StructWithTwoFieldTwinSyncSse({
    required this.a,
    required this.b,
  });

  @override
  int get hashCode => a.hashCode ^ b.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithTwoFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          a == other.a &&
          b == other.b;
}

class StructWithZeroFieldTwinSyncSse {
  const StructWithZeroFieldTwinSyncSse();

  @override
  int get hashCode => 0;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithZeroFieldTwinSyncSse &&
          runtimeType == other.runtimeType;
}

class TupleStructWithOneFieldTwinSyncSse {
  final int field0;

  const TupleStructWithOneFieldTwinSyncSse({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TupleStructWithOneFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}

class TupleStructWithTwoFieldTwinSyncSse {
  final int field0;
  final int field1;

  const TupleStructWithTwoFieldTwinSyncSse({
    required this.field0,
    required this.field1,
  });

  @override
  int get hashCode => field0.hashCode ^ field1.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is TupleStructWithTwoFieldTwinSyncSse &&
          runtimeType == other.runtimeType &&
          field0 == other.field0 &&
          field1 == other.field1;
}
