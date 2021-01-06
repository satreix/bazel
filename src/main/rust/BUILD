load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library", "rust_test")

_srcs = [
    "lib.rs",
    "rc_file.rs",
    "workspace_layout.rs",
]

rust_library(
    name = "bazel",
    srcs = _srcs,
    visibility = ["//visibility:public"],
    deps = [
        "//cargo:slog",
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
