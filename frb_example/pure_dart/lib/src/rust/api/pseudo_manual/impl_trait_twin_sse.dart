// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.37.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

class StructWithTraitTwinSse {
  final int value;

  const StructWithTraitTwinSse({
    required this.value,
  });

  static Future<StructWithTraitTwinSse> simpleTraitFnTwinSse() => RustLib
      .instance.api
      .crateApiPseudoManualImplTraitTwinSseStructWithTraitTwinSseSimpleTraitFnTwinSse();

  @override
  int get hashCode => value.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is StructWithTraitTwinSse &&
          runtimeType == other.runtimeType &&
          value == other.value;
}
