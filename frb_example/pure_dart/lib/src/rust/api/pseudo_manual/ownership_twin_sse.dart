// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`

Future<String> borrowStringTwinSse({required String arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualOwnershipTwinSseBorrowStringTwinSse(arg: arg);

Future<String> borrowStrTwinSse({required String arg}) => RustLib.instance.api
    .crateApiPseudoManualOwnershipTwinSseBorrowStrTwinSse(arg: arg);

Future<int> borrowI32TwinSse({required int arg}) => RustLib.instance.api
    .crateApiPseudoManualOwnershipTwinSseBorrowI32TwinSse(arg: arg);

Future<Uint8List> borrowSliceU8TwinSse({required List<int> arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualOwnershipTwinSseBorrowSliceU8TwinSse(arg: arg);

Future<List<String>> borrowSliceStringTwinSse({required List<String> arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualOwnershipTwinSseBorrowSliceStringTwinSse(arg: arg);

Future<SimpleStructForBorrowTwinSse> borrowStructTwinSse(
        {required SimpleStructForBorrowTwinSse arg}) =>
    RustLib.instance.api
        .crateApiPseudoManualOwnershipTwinSseBorrowStructTwinSse(arg: arg);

class SimpleStructForBorrowTwinSse {
  final String one;

  const SimpleStructForBorrowTwinSse({
    required this.one,
  });

  @override
  int get hashCode => one.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is SimpleStructForBorrowTwinSse &&
          runtimeType == other.runtimeType &&
          one == other.one;
}
