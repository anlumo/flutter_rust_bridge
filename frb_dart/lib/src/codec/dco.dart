import 'package:flutter_rust_bridge/src/codec/base.dart';
import 'package:flutter_rust_bridge/src/generalized_frb_rust_binding/_io.dart';
import 'package:flutter_rust_bridge/src/manual_impl/manual_impl.dart';
import 'package:flutter_rust_bridge/src/platform_types/platform_types.dart';

/// {@macro flutter_rust_bridge.only_for_generated_code}
class DcoCodec<S, E extends Object> extends BaseCodec<S, E, WireSyncReturnDco> {
  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final S Function(dynamic) parseSuccessData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  final E Function(dynamic)? parseErrorData;

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  const DcoCodec({
    required this.parseSuccessData,
    required this.parseErrorData,
  });

  /// {@macro flutter_rust_bridge.only_for_generated_code}
  @override
  S decode(dynamic raw) {
    final rawList = raw as List<dynamic>;
    switch (_Rust2DartAction.values[rawList[0]]) {
      case _Rust2DartAction.success:
        assert(rawList.length == 2);
        return parseSuccessData(rawList[1]);

      case _Rust2DartAction.error:
        assert(rawList.length == 2);
        final parseErrorData = this.parseErrorData;
        if (parseErrorData == null) {
          throw Exception(
              'transformRust2DartMessage received error message, but no parseErrorData to parse it. '
              'Raw data: $raw');
        }
        throw parseErrorData(rawList[1]);

      case _Rust2DartAction.panic:
        assert(rawList.length == 2);
        throw dcoDecodePanicError(rawList[1]);

      case _Rust2DartAction.closeStream:
        assert(rawList.length == 1);
        throw CloseStreamException();

      default:
        throw Exception('Unsupported message (raw=$raw)');
    }
  }

  @override
  void freeWireSync(WireSyncReturnDco raw,
          GeneralizedFrbRustBinding generalizedFrbRustBinding) =>
      generalizedFrbRustBinding.freeWireSyncReturnDco(raw);
}

/// NOTE: Please keep in sync with the Rust side
enum _Rust2DartAction { success, error, closeStream, panic }
