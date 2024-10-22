// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../auxiliary/new_module_system/sub_module.dart';
import '../../auxiliary/old_module_system/sub_module.dart';
import '../../auxiliary/sample_types.dart';
import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

Future<bool> useImportedStructTwinRustAsyncSse({required MyStruct myStruct}) =>
    RustLib.instance.api
        .crateApiPseudoManualExternalTypeInCrateTwinRustAsyncSseUseImportedStructTwinRustAsyncSse(
            myStruct: myStruct);

Future<bool> useImportedEnumTwinRustAsyncSse({required MyEnum myEnum}) => RustLib
    .instance.api
    .crateApiPseudoManualExternalTypeInCrateTwinRustAsyncSseUseImportedEnumTwinRustAsyncSse(
        myEnum: myEnum);

Future<OldSimpleStruct> callOldModuleSystemTwinRustAsyncSse() => RustLib
    .instance.api
    .crateApiPseudoManualExternalTypeInCrateTwinRustAsyncSseCallOldModuleSystemTwinRustAsyncSse();

Future<NewSimpleStruct> callNewModuleSystemTwinRustAsyncSse() => RustLib
    .instance.api
    .crateApiPseudoManualExternalTypeInCrateTwinRustAsyncSseCallNewModuleSystemTwinRustAsyncSse();
