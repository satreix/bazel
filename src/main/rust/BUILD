load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

rust_library(
    name = "bazel",
    srcs = [
        "lib.rs",
        "rc_file.rs",
    ],
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:slog",
    ],
)

rust_test(
    name = "bazel_test",
    srcs = ["rc_file.rs"],
    data = glob(["testdata/**"]),
    deps = [
        ":bazel",
        "//cargo:slog",
        "//cargo:slog_term",
    ],
)
