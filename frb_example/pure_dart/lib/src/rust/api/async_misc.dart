// ignore_for_file: invalid_use_of_internal_member, unused_import

import '../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<void> funcAsyncVoid({dynamic hint}) =>
    RustLib.instance.api.funcAsyncVoid(hint: hint);

Future<int> funcAsyncSimpleAdd(
        {required int a, required int b, dynamic hint}) =>
    RustLib.instance.api.funcAsyncSimpleAdd(a: a, b: b, hint: hint);
