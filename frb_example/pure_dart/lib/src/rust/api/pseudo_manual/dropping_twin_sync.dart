// This file is automatically generated, so please do not edit it.
// Generated by `flutter_rust_bridge`@ 2.0.0-dev.38.

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// Rust type: RustOpaqueNom<flutter_rust_bridge::for_generated::RustAutoOpaqueInner<DroppableTwinSync>>
abstract class DroppableTwinSync implements RustOpaqueInterface {
  Stream<int> createStreamTwinSync();

  static int getDropCountTwinSync() => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncDroppableTwinSyncGetDropCountTwinSync();

  static DroppableTwinSync newTwinSync() => RustLib.instance.api
      .crateApiPseudoManualDroppingTwinSyncDroppableTwinSyncNewTwinSync();

  void simpleMethodTwinSync();
}
