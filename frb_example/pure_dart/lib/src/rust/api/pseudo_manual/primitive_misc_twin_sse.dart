// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<int> primitiveTypesTwinSse(
        {required int myI32,
        required PlatformInt64 myI64,
        required double myF64,
        required bool myBool}) =>
    RustLib.instance.api
        .crateApiPseudoManualPrimitiveMiscTwinSsePrimitiveTypesTwinSse(
            myI32: myI32, myI64: myI64, myF64: myF64, myBool: myBool);

Future<int> primitiveU32TwinSse({required int myU32}) => RustLib.instance.api
    .crateApiPseudoManualPrimitiveMiscTwinSsePrimitiveU32TwinSse(myU32: myU32);
