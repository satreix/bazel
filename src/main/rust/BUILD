load("@io_bazel_rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_clippy", "rust_library", "rust_test")

rust_library(
    name = "bazel",
    srcs = [
        "archive_utils.rs",
        "bazel_startup_options.rs",
        "exit_code.rs",
        "lib.rs",
        "rc_file.rs",
        "startup_options.rs",
        "workspace_layout.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:dirs",
        "//cargo:slog",
        "//cargo:slog_async",
        "//cargo:slog_term",
        "//cargo:zip",
        "//src/main/protobuf:command_server_rust_proto",
    ] + GRPC_COMPILE_DEPS,
)

rust_test(
    name = "bazel_test",
    crate = ":bazel",
    data = glob(["testdata/**"]),
)

rust_clippy(
    name = "bazel_clippy_test",
    testonly = True,
    deps = [":bazel"],
)
