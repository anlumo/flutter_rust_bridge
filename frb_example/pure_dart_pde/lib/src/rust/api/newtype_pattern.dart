// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`

Future<NewTypeIntTwinNormal> handleNewtypeTwinNormal(
        {required NewTypeIntTwinNormal arg}) =>
    RustLib.instance.api
        .crateApiNewtypePatternHandleNewtypeTwinNormal(arg: arg);

class NewTypeIntTwinNormal {
  final PlatformInt64 field0;

  const NewTypeIntTwinNormal({
    required this.field0,
  });

  @override
  int get hashCode => field0.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is NewTypeIntTwinNormal &&
          runtimeType == other.runtimeType &&
          field0 == other.field0;
}
