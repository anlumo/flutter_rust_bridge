import 'dart:async';
import 'dart:js_util';

import 'package:flutter_rust_bridge/src/main_components/wire/wire.dart';
import 'package:flutter_rust_bridge/src/platform_types/_web.dart';
import 'package:flutter_rust_bridge/src/wasm_module/_web.dart';

// TODO rm?
/// {@macro flutter_rust_bridge.only_for_generated_code}
class BaseWasmWire<T extends WasmModule> extends BaseWire {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final Future<T> init;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  BaseWasmWire(ExternalLibrary externalLibrary)
      : init = Future.value(externalLibrary.wasmModule).then((module) => promiseToFuture(module()));
}
