load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

_srcs = [
    "archive_utils.rs",
    "exit_code.rs",
    "lib.rs",
    "rc_file.rs",
    "workspace_layout.rs",
]

rust_library(
    name = "bazel",
    srcs = _srcs,
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:dirs",
        "//cargo:slog",
        "//cargo:slog_async",
        "//cargo:slog_term",
        "//cargo:zip",
    ],
)

rust_test(
    name = "bazel_test",
    srcs = _srcs,
    data = glob(["testdata/**"]),
    deps = [
        ":bazel",
        "//cargo:slog",
        "//cargo:slog_term",
    ],
)
