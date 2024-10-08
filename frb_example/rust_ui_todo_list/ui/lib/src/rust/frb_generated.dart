// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: unused_import, unused_element, unnecessary_import, duplicate_ignore, invalid_use_of_internal_member, annotate_overrides, non_constant_identifier_names, curly_braces_in_flow_control_structures, prefer_const_literals_to_create_immutables, unused_field
// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.0.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import 'app.dart';
import 'dart:async';
import 'dart:convert';
import 'frb_generated.dart';
import 'frb_generated.io.dart'
    if (dart.library.js_interop) 'frb_generated.web.dart';
import 'package:flutter/material.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

/// Main entrypoint of the Rust API
class RustLib extends BaseEntrypoint<RustLibApi, RustLibApiImpl, RustLibWire> {
  @internal
  static final instance = RustLib._();

  RustLib._();

  /// Initialize flutter_rust_bridge
  static Future<void> init({
    RustLibApi? api,
    BaseHandler? handler,
    ExternalLibrary? externalLibrary,
  }) async {
    await instance.initImpl(
      api: api,
      handler: handler,
      externalLibrary: externalLibrary,
    );
  }

  /// Initialize flutter_rust_bridge in mock mode.
  /// No libraries for FFI are loaded.
  static void initMock({
    required RustLibApi api,
  }) {
    instance.initMockImpl(
      api: api,
    );
  }

  /// Dispose flutter_rust_bridge
  ///
  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
  /// is automatically disposed when the app stops.
  static void dispose() => instance.disposeImpl();

  @override
  ApiImplConstructor<RustLibApiImpl, RustLibWire> get apiImplConstructor =>
      RustLibApiImpl.new;

  @override
  WireConstructor<RustLibWire> get wireConstructor =>
      RustLibWire.fromExternalLibrary;

  @override
  Future<void> executeRustInitializers() async {}

  @override
  ExternalLibraryLoaderConfig get defaultExternalLibraryLoaderConfig =>
      kDefaultExternalLibraryLoaderConfig;

  @override
  String get codegenVersion => '2.5.0';

  @override
  int get rustContentHash => -1253672655;

  static const kDefaultExternalLibraryLoaderConfig =
      ExternalLibraryLoaderConfig(
    stem: 'rust_lib_frb_example_rust_ui_todo_list',
    ioDirectory: '../target/release/',
    webPrefix: 'pkg/',
  );
}

abstract class RustLibApi extends BaseApi {
  void crateAppRustStateAdd({required RustState that});

  Filter crateAppRustStateAutoAccessorGetFilter({required RustState that});

  String crateAppRustStateAutoAccessorGetInputText({required RustState that});

  void crateAppRustStateAutoAccessorSetFilter(
      {required RustState that, required Filter filter});

  void crateAppRustStateAutoAccessorSetInputText(
      {required RustState that, required String inputText});

  List<Item> crateAppRustStateFilteredItems({required RustState that});

  RustState crateAppRustStateNew();

  void crateAppRustStateRemove({required RustState that, required int id});

  void crateAppRustStateSetBaseState(
      {required RustState that, required BaseRustState baseState});

  void crateAppRustStateToggle({required RustState that, required int id});

  Stream<void> crateFrbGeneratedBaseRustStateCreateNotifyUiStream(
      {required BaseRustState that});

  BaseRustState crateFrbGeneratedBaseRustStateEmpty();

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_BaseRustState;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_BaseRustState;

  CrossPlatformFinalizerArg
      get rust_arc_decrement_strong_count_BaseRustStatePtr;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_RustState;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_RustState;

  CrossPlatformFinalizerArg get rust_arc_decrement_strong_count_RustStatePtr;
}

class RustLibApiImpl extends RustLibApiImplPlatform implements RustLibApi {
  RustLibApiImpl({
    required super.handler,
    required super.wire,
    required super.generalizedFrbRustBinding,
    required super.portManager,
  });

  @override
  void crateAppRustStateAdd({required RustState that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 1)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateAddConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateAddConstMeta => const TaskConstMeta(
        debugName: "RustState_add",
        argNames: ["that"],
      );

  @override
  Filter crateAppRustStateAutoAccessorGetFilter({required RustState that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 2)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_filter,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateAutoAccessorGetFilterConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateAutoAccessorGetFilterConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_auto_accessor_get_filter",
        argNames: ["that"],
      );

  @override
  String crateAppRustStateAutoAccessorGetInputText({required RustState that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 3)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_String,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateAutoAccessorGetInputTextConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateAutoAccessorGetInputTextConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_auto_accessor_get_input_text",
        argNames: ["that"],
      );

  @override
  void crateAppRustStateAutoAccessorSetFilter(
      {required RustState that, required Filter filter}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        sse_encode_filter(filter, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 4)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateAutoAccessorSetFilterConstMeta,
      argValues: [that, filter],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateAutoAccessorSetFilterConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_auto_accessor_set_filter",
        argNames: ["that", "filter"],
      );

  @override
  void crateAppRustStateAutoAccessorSetInputText(
      {required RustState that, required String inputText}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        sse_encode_String(inputText, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 5)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateAutoAccessorSetInputTextConstMeta,
      argValues: [that, inputText],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateAutoAccessorSetInputTextConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_auto_accessor_set_input_text",
        argNames: ["that", "inputText"],
      );

  @override
  List<Item> crateAppRustStateFilteredItems({required RustState that}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 6)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_list_item,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateFilteredItemsConstMeta,
      argValues: [that],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateFilteredItemsConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_filtered_items",
        argNames: ["that"],
      );

  @override
  RustState crateAppRustStateNew() {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 7)!;
      },
      codec: SseCodec(
        decodeSuccessData:
            sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateNewConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateNewConstMeta => const TaskConstMeta(
        debugName: "RustState_new",
        argNames: [],
      );

  @override
  void crateAppRustStateRemove({required RustState that, required int id}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        sse_encode_i_32(id, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 8)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateRemoveConstMeta,
      argValues: [that, id],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateRemoveConstMeta => const TaskConstMeta(
        debugName: "RustState_remove",
        argNames: ["that", "id"],
      );

  @override
  void crateAppRustStateSetBaseState(
      {required RustState that, required BaseRustState baseState}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
            baseState, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 9)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateSetBaseStateConstMeta,
      argValues: [that, baseState],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateSetBaseStateConstMeta =>
      const TaskConstMeta(
        debugName: "RustState_set_base_state",
        argNames: ["that", "baseState"],
      );

  @override
  void crateAppRustStateToggle({required RustState that, required int id}) {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
            that, serializer);
        sse_encode_i_32(id, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 10)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateAppRustStateToggleConstMeta,
      argValues: [that, id],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateAppRustStateToggleConstMeta => const TaskConstMeta(
        debugName: "RustState_toggle",
        argNames: ["that", "id"],
      );

  @override
  Stream<void> crateFrbGeneratedBaseRustStateCreateNotifyUiStream(
      {required BaseRustState that}) {
    final sink = RustStreamSink<void>();
    handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
            that, serializer);
        sse_encode_StreamSink_unit_Sse(sink, serializer);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 11)!;
      },
      codec: SseCodec(
        decodeSuccessData: sse_decode_unit,
        decodeErrorData: null,
      ),
      constMeta: kCrateFrbGeneratedBaseRustStateCreateNotifyUiStreamConstMeta,
      argValues: [that, sink],
      apiImpl: this,
    ));
    return sink.stream;
  }

  TaskConstMeta
      get kCrateFrbGeneratedBaseRustStateCreateNotifyUiStreamConstMeta =>
          const TaskConstMeta(
            debugName: "BaseRustState_create_notify_ui_stream",
            argNames: ["that", "sink"],
          );

  @override
  BaseRustState crateFrbGeneratedBaseRustStateEmpty() {
    return handler.executeSync(SyncTask(
      callFfi: () {
        final serializer = SseSerializer(generalizedFrbRustBinding);
        return pdeCallFfi(generalizedFrbRustBinding, serializer, funcId: 12)!;
      },
      codec: SseCodec(
        decodeSuccessData:
            sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState,
        decodeErrorData: null,
      ),
      constMeta: kCrateFrbGeneratedBaseRustStateEmptyConstMeta,
      argValues: [],
      apiImpl: this,
    ));
  }

  TaskConstMeta get kCrateFrbGeneratedBaseRustStateEmptyConstMeta =>
      const TaskConstMeta(
        debugName: "BaseRustState_empty",
        argNames: [],
      );

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_BaseRustState => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_BaseRustState => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState;

  RustArcIncrementStrongCountFnType
      get rust_arc_increment_strong_count_RustState => wire
          .rust_arc_increment_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState;

  RustArcDecrementStrongCountFnType
      get rust_arc_decrement_strong_count_RustState => wire
          .rust_arc_decrement_strong_count_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState;

  @protected
  AnyhowException dco_decode_AnyhowException(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return AnyhowException(raw as String);
  }

  @protected
  BaseRustState
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustState
      dco_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return RustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  BaseRustState
      dco_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustState
      dco_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return RustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustState
      dco_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return RustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  BaseRustState
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustState
      dco_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return RustStateImpl.frbInternalDcoDecode(raw as List<dynamic>);
  }

  @protected
  RustStreamSink<void> dco_decode_StreamSink_unit_Sse(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    throw UnimplementedError();
  }

  @protected
  String dco_decode_String(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as String;
  }

  @protected
  bool dco_decode_bool(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as bool;
  }

  @protected
  Filter dco_decode_filter(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return Filter.values[raw as int];
  }

  @protected
  int dco_decode_i_32(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  Item dco_decode_item(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    final arr = raw as List<dynamic>;
    if (arr.length != 3)
      throw Exception('unexpected arr length: expect 3 but see ${arr.length}');
    return Item(
      id: dco_decode_i_32(arr[0]),
      content: dco_decode_String(arr[1]),
      completed: dco_decode_bool(arr[2]),
    );
  }

  @protected
  List<Item> dco_decode_list_item(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return (raw as List<dynamic>).map(dco_decode_item).toList();
  }

  @protected
  Uint8List dco_decode_list_prim_u_8_strict(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as Uint8List;
  }

  @protected
  int dco_decode_u_8(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return raw as int;
  }

  @protected
  void dco_decode_unit(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return;
  }

  @protected
  BigInt dco_decode_usize(dynamic raw) {
    // Codec=Dco (DartCObject based), see doc to use other codecs
    return dcoDecodeU64(raw);
  }

  @protected
  AnyhowException sse_decode_AnyhowException(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_String(deserializer);
    return AnyhowException(inner);
  }

  @protected
  BaseRustState
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustState
      sse_decode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return RustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  BaseRustState
      sse_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustState
      sse_decode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return RustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustState
      sse_decode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return RustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  BaseRustState
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return BaseRustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustState
      sse_decode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return RustStateImpl.frbInternalSseDecode(
        sse_decode_usize(deserializer), sse_decode_i_32(deserializer));
  }

  @protected
  RustStreamSink<void> sse_decode_StreamSink_unit_Sse(
      SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    throw UnimplementedError('Unreachable ()');
  }

  @protected
  String sse_decode_String(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_list_prim_u_8_strict(deserializer);
    return utf8.decoder.convert(inner);
  }

  @protected
  bool sse_decode_bool(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8() != 0;
  }

  @protected
  Filter sse_decode_filter(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var inner = sse_decode_i_32(deserializer);
    return Filter.values[inner];
  }

  @protected
  int sse_decode_i_32(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getInt32();
  }

  @protected
  Item sse_decode_item(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var var_id = sse_decode_i_32(deserializer);
    var var_content = sse_decode_String(deserializer);
    var var_completed = sse_decode_bool(deserializer);
    return Item(id: var_id, content: var_content, completed: var_completed);
  }

  @protected
  List<Item> sse_decode_list_item(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs

    var len_ = sse_decode_i_32(deserializer);
    var ans_ = <Item>[];
    for (var idx_ = 0; idx_ < len_; ++idx_) {
      ans_.add(sse_decode_item(deserializer));
    }
    return ans_;
  }

  @protected
  Uint8List sse_decode_list_prim_u_8_strict(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    var len_ = sse_decode_i_32(deserializer);
    return deserializer.buffer.getUint8List(len_);
  }

  @protected
  int sse_decode_u_8(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getUint8();
  }

  @protected
  void sse_decode_unit(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  BigInt sse_decode_usize(SseDeserializer deserializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    return deserializer.buffer.getBigUint64();
  }

  @protected
  void sse_encode_AnyhowException(
      AnyhowException self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(self.message, serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          BaseRustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as BaseRustStateImpl).frbInternalSseEncode(move: true),
        serializer);
  }

  @protected
  void
      sse_encode_Auto_Owned_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          RustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as RustStateImpl).frbInternalSseEncode(move: true), serializer);
  }

  @protected
  void
      sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          BaseRustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as BaseRustStateImpl).frbInternalSseEncode(move: false),
        serializer);
  }

  @protected
  void
      sse_encode_Auto_RefMut_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          RustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as RustStateImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void
      sse_encode_Auto_Ref_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          RustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as RustStateImpl).frbInternalSseEncode(move: false), serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerBaseRustState(
          BaseRustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as BaseRustStateImpl).frbInternalSseEncode(move: null),
        serializer);
  }

  @protected
  void
      sse_encode_RustOpaque_flutter_rust_bridgefor_generatedRustAutoOpaqueInnerRustState(
          RustState self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_usize(
        (self as RustStateImpl).frbInternalSseEncode(move: null), serializer);
  }

  @protected
  void sse_encode_StreamSink_unit_Sse(
      RustStreamSink<void> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_String(
        self.setupAndSerialize(
            codec: SseCodec(
          decodeSuccessData: sse_decode_unit,
          decodeErrorData: sse_decode_AnyhowException,
        )),
        serializer);
  }

  @protected
  void sse_encode_String(String self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_list_prim_u_8_strict(utf8.encoder.convert(self), serializer);
  }

  @protected
  void sse_encode_bool(bool self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self ? 1 : 0);
  }

  @protected
  void sse_encode_filter(Filter self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.index, serializer);
  }

  @protected
  void sse_encode_i_32(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putInt32(self);
  }

  @protected
  void sse_encode_item(Item self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.id, serializer);
    sse_encode_String(self.content, serializer);
    sse_encode_bool(self.completed, serializer);
  }

  @protected
  void sse_encode_list_item(List<Item> self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    for (final item in self) {
      sse_encode_item(item, serializer);
    }
  }

  @protected
  void sse_encode_list_prim_u_8_strict(
      Uint8List self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    sse_encode_i_32(self.length, serializer);
    serializer.buffer.putUint8List(self);
  }

  @protected
  void sse_encode_u_8(int self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putUint8(self);
  }

  @protected
  void sse_encode_unit(void self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
  }

  @protected
  void sse_encode_usize(BigInt self, SseSerializer serializer) {
    // Codec=Sse (Serialization based), see doc to use other codecs
    serializer.buffer.putBigUint64(self);
  }
}

// Section: extra_from_parser

Future<void> runRustApp({
  required Widget Function(RustState state) body,
  required RustState Function() state,
}) async {
  await RustLib.init();
  runApp(_MyApp(body: body, state: state()));
}

// improve typing later
class _MyApp extends StatefulWidget {
  final Widget Function(RustState state) body;
  final RustState state;

  const _MyApp({
    required this.body,
    required this.state,
  });

  @override
  State<_MyApp> createState() => _MyAppState();
}

class _MyAppState extends State<_MyApp> {
  late final BaseRustState baseState;

  @override
  void initState() {
    super.initState();
    baseState = BaseRustState(onMutate: () {
      if (mounted) setState(() {});
    });
    widget.state.setBaseState(baseState: baseState);
  }

  @override
  void dispose() {
    baseState.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    // We can allow users to customize MaterialApp/Scaffold by exposing another argument
    // like `Widget Function() app`; but for simplicity let's customize the `body` by default.
    return MaterialApp(
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.blue),
        useMaterial3: true,
      ),
      home: Scaffold(body: widget.body(widget.state)),
    );
  }
}

/// Adapted from https://github.com/mobxjs/mobx.dart/issues/750
// TODO: Move to support library instead of inlining here
class SyncTextField extends StatefulWidget {
  final String text;

  // forward
  final ValueChanged<String>? onChanged;
  final InputDecoration? decoration;
  final ValueChanged<String>? onSubmitted;

  const SyncTextField({
    super.key,
    required this.text,
    this.onChanged,
    this.decoration,
    this.onSubmitted,
  });

  @override
  State<SyncTextField> createState() => _SyncTextFieldState();
}

class _SyncTextFieldState extends State<SyncTextField> {
  late final TextEditingController _controller;

  @override
  void initState() {
    super.initState();
    _controller = TextEditingController();
    _controller.text = widget.text;
  }

  @override
  void didUpdateWidget(covariant SyncTextField oldWidget) {
    super.didUpdateWidget(oldWidget);
    if (oldWidget.text != widget.text) _controller.text = widget.text;
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return TextField(
      controller: _controller,
      // forward
      onChanged: widget.onChanged,
      decoration: widget.decoration,
      onSubmitted: widget.onSubmitted,
    );
  }
}

// These functions are ignored because they are not marked as `pub`: `on_mutation`

// Rust type: RustOpaqueMoi<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<BaseRustState>>
abstract class BaseRustState implements RustOpaqueInterface {
  Stream<void> createNotifyUiStream();

  static BaseRustState empty() =>
      RustLib.instance.api.crateFrbGeneratedBaseRustStateEmpty();

  factory BaseRustState({required void Function() onMutate}) {
    final object = BaseRustState.empty();
    object.createNotifyUiStream().listen((_) => onMutate());
    return object;
  }
}

@sealed
class BaseRustStateImpl extends RustOpaque implements BaseRustState {
  // Not to be used by end users
  BaseRustStateImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  BaseRustStateImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_BaseRustState,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_BaseRustState,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_BaseRustStatePtr,
  );

  Stream<void> createNotifyUiStream() =>
      RustLib.instance.api.crateFrbGeneratedBaseRustStateCreateNotifyUiStream(
        that: this,
      );
}

@sealed
class RustStateImpl extends RustOpaque implements RustState {
  // Not to be used by end users
  RustStateImpl.frbInternalDcoDecode(List<dynamic> wire)
      : super.frbInternalDcoDecode(wire, _kStaticData);

  // Not to be used by end users
  RustStateImpl.frbInternalSseDecode(BigInt ptr, int externalSizeOnNative)
      : super.frbInternalSseDecode(ptr, externalSizeOnNative, _kStaticData);

  static final _kStaticData = RustArcStaticData(
    rustArcIncrementStrongCount:
        RustLib.instance.api.rust_arc_increment_strong_count_RustState,
    rustArcDecrementStrongCount:
        RustLib.instance.api.rust_arc_decrement_strong_count_RustState,
    rustArcDecrementStrongCountPtr:
        RustLib.instance.api.rust_arc_decrement_strong_count_RustStatePtr,
  );

  void add() => RustLib.instance.api.crateAppRustStateAdd(
        that: this,
      );

  Filter get filter =>
      RustLib.instance.api.crateAppRustStateAutoAccessorGetFilter(
        that: this,
      );

  String get inputText =>
      RustLib.instance.api.crateAppRustStateAutoAccessorGetInputText(
        that: this,
      );

  set filter(Filter filter) => RustLib.instance.api
      .crateAppRustStateAutoAccessorSetFilter(that: this, filter: filter);

  set inputText(String inputText) =>
      RustLib.instance.api.crateAppRustStateAutoAccessorSetInputText(
          that: this, inputText: inputText);

  List<Item> filteredItems() =>
      RustLib.instance.api.crateAppRustStateFilteredItems(
        that: this,
      );

  void remove({required int id}) =>
      RustLib.instance.api.crateAppRustStateRemove(that: this, id: id);

  void setBaseState({required BaseRustState baseState}) => RustLib.instance.api
      .crateAppRustStateSetBaseState(that: this, baseState: baseState);

  void toggle({required int id}) =>
      RustLib.instance.api.crateAppRustStateToggle(that: this, id: id);
}
