load("@io_bazel_rules_rust//rust:rust.bzl", "rust_binary", "rust_clippy")

rust_binary(
    name = "bazel",
    srcs = ["bazel.rs"],
    visibility = ["//src:__pkg__"],
    deps = [
        "//src/main/rust:bazel",
    ],
)

rust_clippy(
    name = "bazel_clippy_test",
    testonly = True,
    deps = [":bazel"],
)
