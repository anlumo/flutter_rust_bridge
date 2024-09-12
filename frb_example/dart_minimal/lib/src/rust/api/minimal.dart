// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.3.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import '../lib.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `fmt`

Future<int> minimalAdder({required int a, required int b}) =>
    RustLib.instance.api.crateApiMinimalMinimalAdder(a: a, b: b);

class StructWithRustAutoOpaqueFieldWithManyDerive {
  final String content;

  const StructWithRustAutoOpaqueFieldWithManyDerive({
    required this.content,
  });

  Future<void> f() => RustLib.instance.api
          .crateApiMinimalStructWithRustAutoOpaqueFieldWithManyDeriveF(
        that: this,
      );

  @override
  int get hashCode => content.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithRustAutoOpaqueFieldWithManyDerive &&
          runtimeType == other.runtimeType &&
          content == other.content;
}
