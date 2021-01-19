load("@io_bazel_rules_rust//proto:toolchain.bzl", "GRPC_COMPILE_DEPS")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_clippy", "rust_doc", "rust_library", "rust_test")

rust_doc(
      name = "bazel_doc",
      dep = ":bazel",
  )

rust_library(
    name = "bazel",
    srcs = [
        "archive_utils.rs",
        "bazel_startup_options.rs",
        "exit_code.rs",
        "lib.rs",
        "option_processor.rs",
        "rc_file.rs",
        "server_process_info.rs",
        "startup_options.rs",
        "workspace_layout.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:dirs",
        "//cargo:slog",
        "//cargo:slog_async",
        "//cargo:slog_term",
        "//cargo:whoami",
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
