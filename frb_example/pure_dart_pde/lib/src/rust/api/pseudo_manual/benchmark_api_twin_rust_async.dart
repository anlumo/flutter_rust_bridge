// This file is automatically generated, so please do not edit it.
// @generated by `flutter_rust_bridge`@ 2.5.1.

import 'dart:io';

// ignore_for_file: invalid_use_of_internal_member, unused_import, unnecessary_import

import '../../frb_generated.dart';
import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';

// These functions are ignored because they are not marked as `pub`: `create_binary_tree_map`, `create_binary_tree_protobuf`, `create_binary_tree`, `create_blob`
// These function are ignored because they are on traits that is not defined in current crate (put an empty `#[frb]` on it to unignore): `clone`, `clone`, `deref`, `deref`, `fmt`, `fmt`, `initialize`, `initialize`

Future<void> benchmarkVoidTwinRustAsync() => RustLib.instance.api
    .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkVoidTwinRustAsync();

Future<int> benchmarkInputBytesTwinRustAsync({required List<int> bytes}) => RustLib
    .instance.api
    .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkInputBytesTwinRustAsync(
        bytes: bytes);

Future<Uint8List> benchmarkOutputBytesTwinRustAsync({required int size}) => RustLib
    .instance.api
    .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkOutputBytesTwinRustAsync(
        size: size);

Future<void> benchmarkBinaryTreeInputTwinRustAsync(
        {required BenchmarkBinaryTreeTwinRustAsync tree}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeInputTwinRustAsync(
            tree: tree);

Future<BenchmarkBinaryTreeTwinRustAsync> benchmarkBinaryTreeOutputTwinRustAsync(
        {required int depth}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeOutputTwinRustAsync(
            depth: depth);

Future<void> benchmarkBinaryTreeInputProtobufTwinRustAsync(
        {required List<int> raw}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeInputProtobufTwinRustAsync(
            raw: raw);

Future<Uint8List> benchmarkBinaryTreeOutputProtobufTwinRustAsync(
        {required int depth}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeOutputProtobufTwinRustAsync(
            depth: depth);

Future<void> benchmarkBinaryTreeInputJsonTwinRustAsync({required String raw}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeInputJsonTwinRustAsync(
            raw: raw);

Future<String> benchmarkBinaryTreeOutputJsonTwinRustAsync(
        {required int depth}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBinaryTreeOutputJsonTwinRustAsync(
            depth: depth);

Future<void> benchmarkBlobInputTwinRustAsync(
        {required BenchmarkBlobTwinRustAsync blob}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobInputTwinRustAsync(
            blob: blob);

Future<BenchmarkBlobTwinRustAsync> benchmarkBlobOutputTwinRustAsync(
        {required int size}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobOutputTwinRustAsync(
            size: size);

Future<void> benchmarkBlobInputProtobufTwinRustAsync(
        {required List<int> raw}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobInputProtobufTwinRustAsync(
            raw: raw);

Future<Uint8List> benchmarkBlobOutputProtobufTwinRustAsync(
        {required int size}) =>
    RustLib.instance.api
        .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobOutputProtobufTwinRustAsync(
            size: size);

Future<void> benchmarkBlobInputJsonTwinRustAsync({required String raw}) => RustLib
    .instance.api
    .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobInputJsonTwinRustAsync(
        raw: raw);

Future<String> benchmarkBlobOutputJsonTwinRustAsync({required int size}) => RustLib
    .instance.api
    .crateApiPseudoManualBenchmarkApiTwinRustAsyncBenchmarkBlobOutputJsonTwinRustAsync(
        size: size);

class BenchmarkBinaryTreeTwinRustAsync {
  final String name;
  final BenchmarkBinaryTreeTwinRustAsync? left;
  final BenchmarkBinaryTreeTwinRustAsync? right;

  const BenchmarkBinaryTreeTwinRustAsync({
    required this.name,
    this.left,
    this.right,
  });

  @override
  int get hashCode => name.hashCode ^ left.hashCode ^ right.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BenchmarkBinaryTreeTwinRustAsync &&
          runtimeType == other.runtimeType &&
          name == other.name &&
          left == other.left &&
          right == other.right;
}

class BenchmarkBlobTwinRustAsync {
  final Uint8List first;
  final Uint8List second;
  final Uint8List third;

  const BenchmarkBlobTwinRustAsync({
    required this.first,
    required this.second,
    required this.third,
  });

  @override
  int get hashCode => first.hashCode ^ second.hashCode ^ third.hashCode;

  @override
  bool operator ==(Object other) =>
      identical(this, other) ||
      other is BenchmarkBlobTwinRustAsync &&
          runtimeType == other.runtimeType &&
          first == other.first &&
          second == other.second &&
          third == other.third;
}
