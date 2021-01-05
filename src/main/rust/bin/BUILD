load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary")

rust_binary(
    name = "bazel",
    srcs = ["bazel.rs"],
    deps = [
        "//src/main/rust:bazel",
        "//cargo:dirs",
        "//cargo:slog",
        "//cargo:slog_async",
        "//cargo:slog_term",
    ],
)
