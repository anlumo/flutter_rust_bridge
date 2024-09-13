// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.4.0.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../frb_generated.dart';
import 'dart:math';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<OpaqueStructWithDartCodeTwinNormal>>
abstract class OpaqueStructWithDartCodeTwinNormal
    implements RustOpaqueInterface {
  Future<int> normalMethodTwinNormal();

  static int get dartCodeGetter => 123;
}

class TranslatableStructWithDartCodeTwinNormal {
  final int a;

  const TranslatableStructWithDartCodeTwinNormal({
    required this.a,
  });

  Future<int> normalMethodTwinNormal() => RustLib.instance.api
          .crateApiDartCodeTranslatableStructWithDartCodeTwinNormalNormalMethodTwinNormal(
        that: this,
      );

  int dummyMethodThatNeedsDartImport() => Random().nextInt(10);

  @override
  bool operator ==(Object other) =>
      other is TranslatableStructWithDartCodeTwinNormal && a == other.a;

  @override
  int get hashCode => a.hashCode;

  int dartCodeMethod() => a * 2;
}
