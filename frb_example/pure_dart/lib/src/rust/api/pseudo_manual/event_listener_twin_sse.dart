// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'event_listener_twin_sse.freezed.dart';

Future<Stream<EventTwinSse>> registerEventListenerTwinSse() =>
    RustLib.instance.api
        .crateApiPseudoManualEventListenerTwinSseRegisterEventListenerTwinSse();

Future<void> closeEventListenerTwinSse() => RustLib.instance.api
    .crateApiPseudoManualEventListenerTwinSseCloseEventListenerTwinSse();

Future<void> createEventTwinSse(
        {required String address, required String payload}) =>
    RustLib.instance.api
        .crateApiPseudoManualEventListenerTwinSseCreateEventTwinSse(
            address: address, payload: payload);

@freezed
class EventTwinSse with _$EventTwinSse {
  const EventTwinSse._();
  const factory EventTwinSse({
    required String address,
    required String payload,
  }) = _EventTwinSse;
  Future<String> asStringTwinSse() => RustLib.instance.api
          .crateApiPseudoManualEventListenerTwinSseEventTwinSseAsStringTwinSse(
        that: this,
      );
}

class EVENTS {
  final void privateField;

  const EVENTS({
    required this.privateField,
  });

  Future<void> deref() =>
      RustLib.instance.api.crateApiPseudoManualEventListenerTwinSseEventsDeref(
        that: this,
      );

  Future<void> deref() =>
      RustLib.instance.api.crateApiPseudoManualEventListenerTwinSseEventsDeref(
        that: this,
      );

  Future<void> deref() =>
      RustLib.instance.api.crateApiPseudoManualEventListenerTwinSseEventsDeref(
        that: this,
      );

  Future<void> deref() =>
      RustLib.instance.api.crateApiPseudoManualEventListenerTwinSseEventsDeref(
        that: this,
      );

  static Future<void> initialize({required EVENTS lazy}) => RustLib.instance.api
      .crateApiPseudoManualEventListenerTwinSseEventsInitialize(lazy: lazy);

  static Future<void> initialize({required EVENTS lazy}) => RustLib.instance.api
      .crateApiPseudoManualEventListenerTwinSseEventsInitialize(lazy: lazy);

  static Future<void> initialize({required EVENTS lazy}) => RustLib.instance.api
      .crateApiPseudoManualEventListenerTwinSseEventsInitialize(lazy: lazy);

  static Future<void> initialize({required EVENTS lazy}) => RustLib.instance.api
      .crateApiPseudoManualEventListenerTwinSseEventsInitialize(lazy: lazy);

  @override
  int get hashCode => privateField.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is EVENTS &&
          runtimeType == other.runtimeType &&
          privateField == other.privateField;
}
