// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'misc_example_twin_sync_sse.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'enumeration_twin_sync_sse.freezed.dart';

EnumSimpleTwinSyncSse funcEnumSimpleTwinSyncSse(
        {required EnumSimpleTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseFuncEnumSimpleTwinSyncSse(
            arg: arg);

EnumWithItemMixedTwinSyncSse funcEnumWithItemMixedTwinSyncSse(
        {required EnumWithItemMixedTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseFuncEnumWithItemMixedTwinSyncSse(
            arg: arg);

EnumWithItemTupleTwinSyncSse funcEnumWithItemTupleTwinSyncSse(
        {required EnumWithItemTupleTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseFuncEnumWithItemTupleTwinSyncSse(
            arg: arg);

EnumWithItemStructTwinSyncSse funcEnumWithItemStructTwinSyncSse(
        {required EnumWithItemStructTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseFuncEnumWithItemStructTwinSyncSse(
            arg: arg);

EnumWithDiscriminantTwinSyncSse funcEnumWithDiscriminantTwinSyncSse(
        {required EnumWithDiscriminantTwinSyncSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseFuncEnumWithDiscriminantTwinSyncSse(
            arg: arg);

Uint8List printNoteTwinSyncSse({required NoteTwinSyncSse note}) => RustLib
    .instance.api
    .crateApiPseudoManualEnumerationTwinSyncSsePrintNoteTwinSyncSse(note: note);

WeekdaysTwinSyncSse? handleReturnEnumTwinSyncSse({required String input}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseHandleReturnEnumTwinSyncSse(
            input: input);

WeekdaysTwinSyncSse handleEnumParameterTwinSyncSse(
        {required WeekdaysTwinSyncSse weekday}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseHandleEnumParameterTwinSyncSse(
            weekday: weekday);

MeasureTwinSyncSse? multiplyByTenTwinSyncSse(
        {required MeasureTwinSyncSse measure}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseMultiplyByTenTwinSyncSse(
            measure: measure);

KitchenSinkTwinSyncSse handleEnumStructTwinSyncSse(
        {required KitchenSinkTwinSyncSse val}) =>
    RustLib.instance.api
        .crateApiPseudoManualEnumerationTwinSyncSseHandleEnumStructTwinSyncSse(
            val: val);

@freezed
sealed class DistanceTwinSyncSse with _$DistanceTwinSyncSse {
  const DistanceTwinSyncSse._();

  const factory DistanceTwinSyncSse.unknown() = DistanceTwinSyncSse_Unknown;
  const factory DistanceTwinSyncSse.map(
    double field0,
  ) = DistanceTwinSyncSse_Map;
}

enum EnumSimpleTwinSyncSse {
  a,
  b,
  ;
}

enum EnumWithDiscriminantTwinSyncSse {
  oneHundred,
  fifty,
  ;
}

@freezed
sealed class EnumWithItemMixedTwinSyncSse with _$EnumWithItemMixedTwinSyncSse {
  const EnumWithItemMixedTwinSyncSse._();

  const factory EnumWithItemMixedTwinSyncSse.a() =
      EnumWithItemMixedTwinSyncSse_A;
  const factory EnumWithItemMixedTwinSyncSse.b(
    Uint8List field0,
  ) = EnumWithItemMixedTwinSyncSse_B;
  const factory EnumWithItemMixedTwinSyncSse.c({
    required String cField,
  }) = EnumWithItemMixedTwinSyncSse_C;
}

@freezed
sealed class EnumWithItemStructTwinSyncSse
    with _$EnumWithItemStructTwinSyncSse {
  const EnumWithItemStructTwinSyncSse._();

  const factory EnumWithItemStructTwinSyncSse.a({
    required Uint8List aField,
  }) = EnumWithItemStructTwinSyncSse_A;
  const factory EnumWithItemStructTwinSyncSse.b({
    required Int32List bField,
  }) = EnumWithItemStructTwinSyncSse_B;
}

@freezed
sealed class EnumWithItemTupleTwinSyncSse with _$EnumWithItemTupleTwinSyncSse {
  const EnumWithItemTupleTwinSyncSse._();

  const factory EnumWithItemTupleTwinSyncSse.a(
    Uint8List field0,
  ) = EnumWithItemTupleTwinSyncSse_A;
  const factory EnumWithItemTupleTwinSyncSse.b(
    Int32List field0,
  ) = EnumWithItemTupleTwinSyncSse_B;
}

@freezed
sealed class KitchenSinkTwinSyncSse with _$KitchenSinkTwinSyncSse {
  const KitchenSinkTwinSyncSse._();

  /// Comment on variant
  const factory KitchenSinkTwinSyncSse.empty() = KitchenSinkTwinSyncSse_Empty;
  const factory KitchenSinkTwinSyncSse.primitives({
    /// Dart field comment
    @Default(-1) int int32,
    required double float64,
    required bool boolean,
  }) = KitchenSinkTwinSyncSse_Primitives;
  const factory KitchenSinkTwinSyncSse.nested(
    int field0, [
    @Default(KitchenSinkTwinSyncSse.empty()) KitchenSinkTwinSyncSse field1,
  ]) = KitchenSinkTwinSyncSse_Nested;
  const factory KitchenSinkTwinSyncSse.optional([
    /// Comment on anonymous field
    @Default(-1) int? field0,
    int? field1,
  ]) = KitchenSinkTwinSyncSse_Optional;
  const factory KitchenSinkTwinSyncSse.buffer(
    Uint8List field0,
  ) = KitchenSinkTwinSyncSse_Buffer;
  const factory KitchenSinkTwinSyncSse.enums([
    @Default(WeekdaysTwinSyncSse.sunday) WeekdaysTwinSyncSse field0,
  ]) = KitchenSinkTwinSyncSse_Enums;
}

@freezed
sealed class MeasureTwinSyncSse with _$MeasureTwinSyncSse {
  const MeasureTwinSyncSse._();

  const factory MeasureTwinSyncSse.speed(
    SpeedTwinSyncSse field0,
  ) = MeasureTwinSyncSse_Speed;
  const factory MeasureTwinSyncSse.distance(
    DistanceTwinSyncSse field0,
  ) = MeasureTwinSyncSse_Distance;
}

class NoteTwinSyncSse {
  final WeekdaysTwinSyncSse day;
  final String body;

  const NoteTwinSyncSse({
    this.day = WeekdaysTwinSyncSse.sunday,
    required this.body,
  });

  @override
  int get hashCode => day.hashCode ^ body.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NoteTwinSyncSse &&
          runtimeType == other.runtimeType &&
          day == other.day &&
          body == other.body;
}

@freezed
sealed class SpeedTwinSyncSse with _$SpeedTwinSyncSse {
  const SpeedTwinSyncSse._();

  const factory SpeedTwinSyncSse.unknown() = SpeedTwinSyncSse_Unknown;
  const factory SpeedTwinSyncSse.gps(
    double field0,
  ) = SpeedTwinSyncSse_GPS;
}
