import 'dart:convert';
import 'dart:ffi' as ffi;
import 'dart:ffi';
export 'dart:ffi' show NativePort, DynamicLibrary;
import 'dart:typed_data';
import 'io_dartcobject.dart';
import 'stub.dart' show FlutterRustBridgeWireBase;
export 'stub.dart'
    show castInt, castNativeBigInt, FlutterRustBridgeWireBase, WasmModule;

/// Abstraction over a Dart SendPort and a JS MessagePort.
typedef NativePortType = int;
typedef ExternalLibrary = ffi.DynamicLibrary;
typedef DartPostCObject = ffi.Pointer<
    ffi.NativeFunction<ffi.Bool Function(ffi.Int64, ffi.Pointer<ffi.Void>)>>;

extension StoreDartPostCObjectExt on FlutterRustBridgeWireBase {
  void storeDartPostCObject() {
    store_dart_post_cobject(ffi.NativeApi.postCObject.cast());
  }
}

class DartApiDl {
  static int? _initCode;
  final int Function(ffi.Pointer<ffi.Void>) _initFn;
  DartApiDl(this._initFn);

  void initApi() {
    _initCode ??= _initFn(ffi.NativeApi.initializeApiDLData);
    if (_initCode != 0) {
      throw 'Failed to initialize Dart API. Code: $_initCode';
    }
  }
}

// NOTE for maintainer: Please manually keep in sync with [WireSyncReturnStruct] in Rust
/// This class is only for internal usage.
class WireSyncReturnStruct extends ffi.Struct {
  /// Not to be used by normal users, but has to be public for generated code
  external DartCObject data;

  @ffi.Bool()
  external bool isSuccess;

  external ffi.Pointer<ffi.Void> ptr;

  dynamic intoDart() => data.intoDart();
}

extension DartCObjectWireSyncReturn on DartCObject {
  dynamic intoDart() {
    switch (ty) {
      case DartCObjectType.DartNull:
        return null;
      case DartCObjectType.DartBool:
        return value.as_bool;
      case DartCObjectType.DartInt32:
        return value.as_int32;
      case DartCObjectType.DartInt64:
        return value.as_int64;
      case DartCObjectType.DartDouble:
        return value.as_double;

      case DartCObjectType.DartString:
        // `DartCObject` strings being encoded with std::ffi::CString assert us nul-termination.
        // See [allo-isolate's String::into_dart](https://github.com/sunshine-protocol/allo-isolate/blob/71b9760993d64ef46794176ca276d1cc637b2599/src/into_dart.rs#L106)
        // and [std::ffi::CString](https://doc.rust-lang.org/nightly/std/ffi/struct.CString.html)
        int len = 0;
        while (value.as_string.elementAt(len).value != 0) {
          len++;
        }
        return utf8.decode(value.as_string.cast<ffi.Uint8>().asTypedList(len));

      case DartCObjectType.DartArray:
        return List.generate(value.as_array.length,
            (i) => value.as_array.values.elementAt(i).value.ref.intoDart());

      case DartCObjectType.DartTypedData:
        return _typedDataIntoDart(
          value.as_typed_data.ty,
          value.as_typed_data.values,
          value.as_typed_data.length,
          copy: true,
        );

      case DartCObjectType.DartExternalTypedData:
        final externalTypedData = _typedDataIntoDart(
          value.as_external_typed_data.ty,
          value.as_external_typed_data.data,
          value.as_external_typed_data.length,
          copy: false,
        );
        _externalTypedDataFinalizer.attach(
            externalTypedData, value.as_external_typed_data);
        return externalTypedData;

      case DartCObjectType.DartSendPort:
      case DartCObjectType.DartCapability:
      case DartCObjectType.DartNativePointer:
      case DartCObjectType.DartUnsupported:
      case DartCObjectType.DartNumberOfTypes:
      default:
        throw "Can't read invalid data type $ty";
    }
  }

  static dynamic _typedDataIntoDart(
    int ty,
    ffi.Pointer<ffi.Uint8> typedValues,
    int nValues, {
    required bool copy,
  }) {
    switch (ty) {
      case DartTypedDataType.ByteData:
        final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
        final bytes = copy ? view : Uint8List.fromList(view);
        return ByteData.view(bytes.buffer);
      case DartTypedDataType.Int8:
        final view = typedValues.cast<ffi.Int8>().asTypedList(nValues);
        return copy ? Int8List.fromList(view) : view;
      case DartTypedDataType.Uint8:
        final view = typedValues.cast<ffi.Uint8>().asTypedList(nValues);
        return copy ? Uint8List.fromList(view) : view;
      case DartTypedDataType.Int16:
        final view = typedValues.cast<ffi.Int16>().asTypedList(nValues);
        return copy ? Int16List.fromList(view) : view;
      case DartTypedDataType.Uint16:
        final view = typedValues.cast<ffi.Uint16>().asTypedList(nValues);
        return copy ? Uint16List.fromList(view) : view;
      case DartTypedDataType.Int32:
        final view = typedValues.cast<ffi.Int32>().asTypedList(nValues);
        return copy ? Int32List.fromList(view) : view;
      case DartTypedDataType.Uint32:
        final view = typedValues.cast<ffi.Uint32>().asTypedList(nValues);
        return copy ? Uint32List.fromList(view) : view;
      case DartTypedDataType.Int64:
        final view = typedValues.cast<ffi.Int64>().asTypedList(nValues);
        return copy ? Int64List.fromList(view) : view;
      case DartTypedDataType.Uint64:
        final view = typedValues.cast<ffi.Uint64>().asTypedList(nValues);
        return copy ? Uint64List.fromList(view) : view;
      case DartTypedDataType.Float32:
        final view = typedValues.cast<ffi.Float>().asTypedList(nValues);
        return copy ? Float32List.fromList(view) : view;
      case DartTypedDataType.Float64:
        final view = typedValues.cast<ffi.Double>().asTypedList(nValues);
        return copy ? Float64List.fromList(view) : view;

      case DartTypedDataType.Uint8Clamped:
      case DartTypedDataType.Float32x4:
      case DartTypedDataType.Invalid:
      default:
        throw "Can't read invalid typed data type $ty";
    }
  }

  static final _externalTypedDataFinalizer =
      Finalizer<DartNativeExternalTypedData>((externalTypedData) {
    final handleFinalizer = externalTypedData.callback
        .cast<ffi.NativeFunction<NativeExternalTypedDataFinalizer>>()
        .asFunction<DartExternalTypedDataFinalizer>();
    handleFinalizer(externalTypedData.data, externalTypedData.peer);
  });
}

typedef NativeExternalTypedDataFinalizer = ffi.Void Function(
    ffi.Pointer<ffi.Uint8>, ffi.Pointer<ffi.Void>);
typedef DartExternalTypedDataFinalizer = void Function(
    ffi.Pointer<ffi.Uint8>, ffi.Pointer<ffi.Void>);

typedef PlatformPointer = ffi.Pointer<ffi.Void>;
typedef OpaqueTypeFinalizer = NativeFinalizer;

/// An opaque pointer to a native C or Rust type.
/// Recipients of this type should call [dispose] at least once during runtime.
/// If passed to a native function after being [dispose]d, an exception will be thrown.
class FrbOpaqueBase implements Finalizable {
  static PlatformPointer initPtr(int ptr) => ffi.Pointer.fromAddress(ptr);
  static PlatformPointer nullPtr() => ffi.Pointer.fromAddress(0);
  static bool isStalePtr(PlatformPointer ptr) => ptr.address == 0;
  static void finalizerAttach(FrbOpaqueBase opaque, PlatformPointer ptr,
          int size, OpaqueTypeFinalizer finalizer) =>
      finalizer.attach(opaque, ptr, detach: opaque, externalSize: size);
}
