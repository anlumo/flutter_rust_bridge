// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.18.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
import 'package:freezed_annotation/freezed_annotation.dart' hide protected;
part 'event_listener_twin_rust_async.freezed.dart';

Stream<EventTwinRustAsync> registerEventListenerTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.registerEventListenerTwinRustAsync(hint: hint);

Future<void> closeEventListenerTwinRustAsync({dynamic hint}) =>
    RustLib.instance.api.closeEventListenerTwinRustAsync(hint: hint);

Future<void> createEventTwinRustAsync(
        {required String address, required String payload, dynamic hint}) =>
    RustLib.instance.api.createEventTwinRustAsync(
        address: address, payload: payload, hint: hint);

@freezed
class EventTwinRustAsync with _$EventTwinRustAsync {
  const EventTwinRustAsync._();
  const factory EventTwinRustAsync({
    required String address,
    required String payload,
  }) = _EventTwinRustAsync;
  Future<String> asStringTwinRustAsync({dynamic hint}) =>
      RustLib.instance.api.eventTwinRustAsyncAsStringTwinRustAsync(
        that: this,
      );
}
