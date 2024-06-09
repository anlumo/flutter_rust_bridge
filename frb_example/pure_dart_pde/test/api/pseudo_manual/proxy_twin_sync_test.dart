// NOTE: This file is mimicking how a human developer writes tests,
// and is auto-generated from `proxy_test.dart` by frb_internal
// Please do not modify manually, but modify the origin and re-run frb_internal generator

// AUTO-GENERATED FROM frb_example/pure_dart, DO NOT EDIT

import 'package:frb_example_pure_dart_pde/src/rust/api/pseudo_manual/proxy_twin_sync.dart';
import 'package:frb_example_pure_dart_pde/src/rust/frb_generated.dart';
import 'package:test/test.dart';

Future<void> main({bool skipRustLibInit = false}) async {
  if (!skipRustLibInit) await RustLib.init();

  test('simple proxy', () async {
    final node = await MyNodeTwinSync.createTwinSync();
    final paramOne = await node.paramOneTwinSync();
    final paramTwo = await node.paramTwoTwinSync();
    expect(await paramOne.myMethodTwinSync(), 'aa');
    expect(await paramTwo.myMethodTwinSync(), 'bb');
  });
}
