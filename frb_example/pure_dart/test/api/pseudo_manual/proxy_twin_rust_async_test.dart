// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

import 'package:frb_example_pure_dart/src/rust/api/pseudo_manual/proxy_twin_rust_async.dart';
import 'package:frb_example_pure_dart/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('simple proxy', () async {
    final node = await MyNodeTwinRustAsync.createTwinRustAsync();
    final paramOne = await node.paramOneTwinRustAsync();
    final paramTwo = await node.paramTwoTwinRustAsync();
    expect(await paramOne.myMethodTwinRustAsync(), 'aa');
    expect(await paramTwo.myMethodTwinRustAsync(), 'bb');
  });
}
