"""
@generated
cargo-raze generated Bazel file.

DO NOT EDIT! Replaced on runs of cargo-raze
"""

load("@bazel_tools//tools/build_defs/repo:git.bzl", "new_git_repository")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")  # buildifier: disable=load
load("@bazel_tools//tools/build_defs/repo:utils.bzl", "maybe")  # buildifier: disable=load

def raze_fetch_remote_crates():
    """This function defines a collection of repos and should be called in a WORKSPACE file"""
    maybe(
        http_archive,
        name = "raze__adler32__1_2_0",
        url = "https://crates.io/api/v1/crates/adler32/1.2.0/download",
        type = "tar.gz",
        sha256 = "aae1277d39aeec15cb388266ecc24b11c80469deae6067e17a1a7aa9e5c1f234",
        strip_prefix = "adler32-1.2.0",
        build_file = Label("//cargo/remote:BUILD.adler32-1.2.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__arrayref__0_3_6",
        url = "https://crates.io/api/v1/crates/arrayref/0.3.6/download",
        type = "tar.gz",
        sha256 = "a4c527152e37cf757a3f78aae5a06fbeefdb07ccc535c980a3208ee3060dd544",
        strip_prefix = "arrayref-0.3.6",
        build_file = Label("//cargo/remote:BUILD.arrayref-0.3.6.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__arrayvec__0_5_2",
        url = "https://crates.io/api/v1/crates/arrayvec/0.5.2/download",
        type = "tar.gz",
        sha256 = "23b62fc65de8e4e7f52534fb52b0f3ed04746ae267519eef2a83941e8085068b",
        strip_prefix = "arrayvec-0.5.2",
        build_file = Label("//cargo/remote:BUILD.arrayvec-0.5.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__atty__0_2_14",
        url = "https://crates.io/api/v1/crates/atty/0.2.14/download",
        type = "tar.gz",
        sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8",
        strip_prefix = "atty-0.2.14",
        build_file = Label("//cargo/remote:BUILD.atty-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__autocfg__1_0_1",
        url = "https://crates.io/api/v1/crates/autocfg/1.0.1/download",
        type = "tar.gz",
        sha256 = "cdb031dd78e28731d87d56cc8ffef4a8f36ca26c38fe2de700543e627f8a464a",
        strip_prefix = "autocfg-1.0.1",
        build_file = Label("//cargo/remote:BUILD.autocfg-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__base64__0_13_0",
        url = "https://crates.io/api/v1/crates/base64/0.13.0/download",
        type = "tar.gz",
        sha256 = "904dfeac50f3cdaba28fc6f57fdcddb75f49ed61346676a78c4ffe55877802fd",
        strip_prefix = "base64-0.13.0",
        build_file = Label("//cargo/remote:BUILD.base64-0.13.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__blake2b_simd__0_5_11",
        url = "https://crates.io/api/v1/crates/blake2b_simd/0.5.11/download",
        type = "tar.gz",
        sha256 = "afa748e348ad3be8263be728124b24a24f268266f6f5d58af9d75f6a40b5c587",
        strip_prefix = "blake2b_simd-0.5.11",
        build_file = Label("//cargo/remote:BUILD.blake2b_simd-0.5.11.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bumpalo__3_4_0",
        url = "https://crates.io/api/v1/crates/bumpalo/3.4.0/download",
        type = "tar.gz",
        sha256 = "2e8c087f005730276d1096a652e92a8bacee2e2472bcc9715a74d2bec38b5820",
        strip_prefix = "bumpalo-3.4.0",
        build_file = Label("//cargo/remote:BUILD.bumpalo-3.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__byteorder__1_3_4",
        url = "https://crates.io/api/v1/crates/byteorder/1.3.4/download",
        type = "tar.gz",
        sha256 = "08c48aae112d48ed9f069b33538ea9e3e90aa263cfa3d1c24309612b1f7472de",
        strip_prefix = "byteorder-1.3.4",
        build_file = Label("//cargo/remote:BUILD.byteorder-1.3.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bzip2__0_3_3",
        url = "https://crates.io/api/v1/crates/bzip2/0.3.3/download",
        type = "tar.gz",
        sha256 = "42b7c3cbf0fa9c1b82308d57191728ca0256cb821220f4e2fd410a72ade26e3b",
        strip_prefix = "bzip2-0.3.3",
        build_file = Label("//cargo/remote:BUILD.bzip2-0.3.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__bzip2_sys__0_1_9_1_0_8",
        url = "https://crates.io/api/v1/crates/bzip2-sys/0.1.9+1.0.8/download",
        type = "tar.gz",
        sha256 = "ad3b39a260062fca31f7b0b12f207e8f2590a67d32ec7d59c20484b07ea7285e",
        strip_prefix = "bzip2-sys-0.1.9+1.0.8",
        build_file = Label("//cargo/remote:BUILD.bzip2-sys-0.1.9+1.0.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cc__1_0_66",
        url = "https://crates.io/api/v1/crates/cc/1.0.66/download",
        type = "tar.gz",
        sha256 = "4c0496836a84f8d0495758516b8621a622beb77c0fed418570e50764093ced48",
        strip_prefix = "cc-1.0.66",
        build_file = Label("//cargo/remote:BUILD.cc-1.0.66.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__0_1_10",
        url = "https://crates.io/api/v1/crates/cfg-if/0.1.10/download",
        type = "tar.gz",
        sha256 = "4785bdd1c96b2a846b2bd7cc02e86b6b3dbf14e7e53446c4f54c92a361040822",
        strip_prefix = "cfg-if-0.1.10",
        build_file = Label("//cargo/remote:BUILD.cfg-if-0.1.10.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__cfg_if__1_0_0",
        url = "https://crates.io/api/v1/crates/cfg-if/1.0.0/download",
        type = "tar.gz",
        sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd",
        strip_prefix = "cfg-if-1.0.0",
        build_file = Label("//cargo/remote:BUILD.cfg-if-1.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__chrono__0_4_19",
        url = "https://crates.io/api/v1/crates/chrono/0.4.19/download",
        type = "tar.gz",
        sha256 = "670ad68c9088c2a963aaa298cb369688cf3f9465ce5e2d4ca10e6e0098a1ce73",
        strip_prefix = "chrono-0.4.19",
        build_file = Label("//cargo/remote:BUILD.chrono-0.4.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__constant_time_eq__0_1_5",
        url = "https://crates.io/api/v1/crates/constant_time_eq/0.1.5/download",
        type = "tar.gz",
        sha256 = "245097e9a4535ee1e3e3931fcfcd55a796a44c643e8596ff6566d68f09b87bbc",
        strip_prefix = "constant_time_eq-0.1.5",
        build_file = Label("//cargo/remote:BUILD.constant_time_eq-0.1.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crc32fast__1_2_1",
        url = "https://crates.io/api/v1/crates/crc32fast/1.2.1/download",
        type = "tar.gz",
        sha256 = "81156fece84ab6a9f2afdb109ce3ae577e42b1228441eded99bd77f627953b1a",
        strip_prefix = "crc32fast-1.2.1",
        build_file = Label("//cargo/remote:BUILD.crc32fast-1.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_channel__0_4_4",
        url = "https://crates.io/api/v1/crates/crossbeam-channel/0.4.4/download",
        type = "tar.gz",
        sha256 = "b153fe7cbef478c567df0f972e02e6d736db11affe43dfc9c56a9374d1adfb87",
        strip_prefix = "crossbeam-channel-0.4.4",
        build_file = Label("//cargo/remote:BUILD.crossbeam-channel-0.4.4.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_utils__0_7_2",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.7.2/download",
        type = "tar.gz",
        sha256 = "c3c7c73a2d1e9fc0886a08b93e98eb643461230d5f1925e4036204d5f2e261a8",
        strip_prefix = "crossbeam-utils-0.7.2",
        build_file = Label("//cargo/remote:BUILD.crossbeam-utils-0.7.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__crossbeam_utils__0_8_1",
        url = "https://crates.io/api/v1/crates/crossbeam-utils/0.8.1/download",
        type = "tar.gz",
        sha256 = "02d96d1e189ef58269ebe5b97953da3274d83a93af647c2ddd6f9dab28cedb8d",
        strip_prefix = "crossbeam-utils-0.8.1",
        build_file = Label("//cargo/remote:BUILD.crossbeam-utils-0.8.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dirs__2_0_2",
        url = "https://crates.io/api/v1/crates/dirs/2.0.2/download",
        type = "tar.gz",
        sha256 = "13aea89a5c93364a98e9b37b2fa237effbb694d5cfe01c5b70941f7eb087d5e3",
        strip_prefix = "dirs-2.0.2",
        build_file = Label("//cargo/remote:BUILD.dirs-2.0.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dirs__3_0_1",
        url = "https://crates.io/api/v1/crates/dirs/3.0.1/download",
        type = "tar.gz",
        sha256 = "142995ed02755914747cc6ca76fc7e4583cd18578746716d0508ea6ed558b9ff",
        strip_prefix = "dirs-3.0.1",
        build_file = Label("//cargo/remote:BUILD.dirs-3.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__dirs_sys__0_3_5",
        url = "https://crates.io/api/v1/crates/dirs-sys/0.3.5/download",
        type = "tar.gz",
        sha256 = "8e93d7f5705de3e49895a2b5e0b8855a1c27f080192ae9c32a6432d50741a57a",
        strip_prefix = "dirs-sys-0.3.5",
        build_file = Label("//cargo/remote:BUILD.dirs-sys-0.3.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__flate2__1_0_14",
        url = "https://crates.io/api/v1/crates/flate2/1.0.14/download",
        type = "tar.gz",
        sha256 = "2cfff41391129e0a856d6d822600b8d71179d46879e310417eb9c762eb178b42",
        strip_prefix = "flate2-1.0.14",
        build_file = Label("//cargo/remote:BUILD.flate2-1.0.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__getrandom__0_1_16",
        url = "https://crates.io/api/v1/crates/getrandom/0.1.16/download",
        type = "tar.gz",
        sha256 = "8fc3cb4d91f53b50155bdcfd23f6a4c39ae1969c2ae85982b135750cccaf5fce",
        strip_prefix = "getrandom-0.1.16",
        build_file = Label("//cargo/remote:BUILD.getrandom-0.1.16.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__hermit_abi__0_1_17",
        url = "https://crates.io/api/v1/crates/hermit-abi/0.1.17/download",
        type = "tar.gz",
        sha256 = "5aca5565f760fb5b220e499d72710ed156fdb74e631659e99377d9ebfbd13ae8",
        strip_prefix = "hermit-abi-0.1.17",
        build_file = Label("//cargo/remote:BUILD.hermit-abi-0.1.17.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__js_sys__0_3_46",
        url = "https://crates.io/api/v1/crates/js-sys/0.3.46/download",
        type = "tar.gz",
        sha256 = "cf3d7383929f7c9c7c2d0fa596f325832df98c3704f2c60553080f7127a58175",
        strip_prefix = "js-sys-0.3.46",
        build_file = Label("//cargo/remote:BUILD.js-sys-0.3.46.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__lazy_static__1_4_0",
        url = "https://crates.io/api/v1/crates/lazy_static/1.4.0/download",
        type = "tar.gz",
        sha256 = "e2abad23fbc42b3700f2f279844dc832adb2b2eb069b2df918f455c4e18cc646",
        strip_prefix = "lazy_static-1.4.0",
        build_file = Label("//cargo/remote:BUILD.lazy_static-1.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__libc__0_2_81",
        url = "https://crates.io/api/v1/crates/libc/0.2.81/download",
        type = "tar.gz",
        sha256 = "1482821306169ec4d07f6aca392a4681f66c75c9918aa49641a2595db64053cb",
        strip_prefix = "libc-0.2.81",
        build_file = Label("//cargo/remote:BUILD.libc-0.2.81.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__log__0_4_13",
        url = "https://crates.io/api/v1/crates/log/0.4.13/download",
        type = "tar.gz",
        sha256 = "fcf3805d4480bb5b86070dcfeb9e2cb2ebc148adb753c5cca5f884d1d65a42b2",
        strip_prefix = "log-0.4.13",
        build_file = Label("//cargo/remote:BUILD.log-0.4.13.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__maybe_uninit__2_0_0",
        url = "https://crates.io/api/v1/crates/maybe-uninit/2.0.0/download",
        type = "tar.gz",
        sha256 = "60302e4db3a61da70c0cb7991976248362f30319e88850c487b9b95bbf059e00",
        strip_prefix = "maybe-uninit-2.0.0",
        build_file = Label("//cargo/remote:BUILD.maybe-uninit-2.0.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__miniz_oxide__0_3_7",
        url = "https://crates.io/api/v1/crates/miniz_oxide/0.3.7/download",
        type = "tar.gz",
        sha256 = "791daaae1ed6889560f8c4359194f56648355540573244a5448a83ba1ecc7435",
        strip_prefix = "miniz_oxide-0.3.7",
        build_file = Label("//cargo/remote:BUILD.miniz_oxide-0.3.7.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_integer__0_1_44",
        url = "https://crates.io/api/v1/crates/num-integer/0.1.44/download",
        type = "tar.gz",
        sha256 = "d2cc698a63b549a70bc047073d2949cce27cd1c7b0a4a862d08a8031bc2801db",
        strip_prefix = "num-integer-0.1.44",
        build_file = Label("//cargo/remote:BUILD.num-integer-0.1.44.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__num_traits__0_2_14",
        url = "https://crates.io/api/v1/crates/num-traits/0.2.14/download",
        type = "tar.gz",
        sha256 = "9a64b1ec5cda2586e284722486d802acf1f7dbdc623e2bfc57e65ca1cd099290",
        strip_prefix = "num-traits-0.2.14",
        build_file = Label("//cargo/remote:BUILD.num-traits-0.2.14.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__pkg_config__0_3_19",
        url = "https://crates.io/api/v1/crates/pkg-config/0.3.19/download",
        type = "tar.gz",
        sha256 = "3831453b3449ceb48b6d9c7ad7c96d5ea673e9b470a1dc578c2ce6521230884c",
        strip_prefix = "pkg-config-0.3.19",
        build_file = Label("//cargo/remote:BUILD.pkg-config-0.3.19.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__proc_macro2__1_0_24",
        url = "https://crates.io/api/v1/crates/proc-macro2/1.0.24/download",
        type = "tar.gz",
        sha256 = "1e0704ee1a7e00d7bb417d0770ea303c1bccbabf0ef1667dae92b5967f5f8a71",
        strip_prefix = "proc-macro2-1.0.24",
        build_file = Label("//cargo/remote:BUILD.proc-macro2-1.0.24.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__quote__1_0_8",
        url = "https://crates.io/api/v1/crates/quote/1.0.8/download",
        type = "tar.gz",
        sha256 = "991431c3519a3f36861882da93630ce66b52918dcf1b8e2fd66b397fc96f28df",
        strip_prefix = "quote-1.0.8",
        build_file = Label("//cargo/remote:BUILD.quote-1.0.8.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__redox_syscall__0_1_57",
        url = "https://crates.io/api/v1/crates/redox_syscall/0.1.57/download",
        type = "tar.gz",
        sha256 = "41cc0f7e4d5d4544e8861606a285bb08d3e70712ccc7d2b84d7c0ccfaf4b05ce",
        strip_prefix = "redox_syscall-0.1.57",
        build_file = Label("//cargo/remote:BUILD.redox_syscall-0.1.57.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__redox_users__0_3_5",
        url = "https://crates.io/api/v1/crates/redox_users/0.3.5/download",
        type = "tar.gz",
        sha256 = "de0737333e7a9502c789a36d7c7fa6092a49895d4faa31ca5df163857ded2e9d",
        strip_prefix = "redox_users-0.3.5",
        build_file = Label("//cargo/remote:BUILD.redox_users-0.3.5.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__rust_argon2__0_8_3",
        url = "https://crates.io/api/v1/crates/rust-argon2/0.8.3/download",
        type = "tar.gz",
        sha256 = "4b18820d944b33caa75a71378964ac46f58517c92b6ae5f762636247c09e78fb",
        strip_prefix = "rust-argon2-0.8.3",
        build_file = Label("//cargo/remote:BUILD.rust-argon2-0.8.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slog__2_7_0",
        url = "https://crates.io/api/v1/crates/slog/2.7.0/download",
        type = "tar.gz",
        sha256 = "8347046d4ebd943127157b94d63abb990fcf729dc4e9978927fdf4ac3c998d06",
        strip_prefix = "slog-2.7.0",
        build_file = Label("//cargo/remote:BUILD.slog-2.7.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slog_async__2_5_0",
        url = "https://crates.io/api/v1/crates/slog-async/2.5.0/download",
        type = "tar.gz",
        sha256 = "51b3336ce47ce2f96673499fc07eb85e3472727b9a7a2959964b002c2ce8fbbb",
        strip_prefix = "slog-async-2.5.0",
        build_file = Label("//cargo/remote:BUILD.slog-async-2.5.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__slog_term__2_6_0",
        url = "https://crates.io/api/v1/crates/slog-term/2.6.0/download",
        type = "tar.gz",
        sha256 = "bab1d807cf71129b05ce36914e1dbb6fbfbdecaf686301cb457f4fa967f9f5b6",
        strip_prefix = "slog-term-2.6.0",
        build_file = Label("//cargo/remote:BUILD.slog-term-2.6.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__syn__1_0_58",
        url = "https://crates.io/api/v1/crates/syn/1.0.58/download",
        type = "tar.gz",
        sha256 = "cc60a3d73ea6594cd712d830cc1f0390fd71542d8c8cd24e70cc54cdfd5e05d5",
        strip_prefix = "syn-1.0.58",
        build_file = Label("//cargo/remote:BUILD.syn-1.0.58.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__take_mut__0_2_2",
        url = "https://crates.io/api/v1/crates/take_mut/0.2.2/download",
        type = "tar.gz",
        sha256 = "f764005d11ee5f36500a149ace24e00e3da98b0158b3e2d53a7495660d3f4d60",
        strip_prefix = "take_mut-0.2.2",
        build_file = Label("//cargo/remote:BUILD.take_mut-0.2.2.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__term__0_6_1",
        url = "https://crates.io/api/v1/crates/term/0.6.1/download",
        type = "tar.gz",
        sha256 = "c0863a3345e70f61d613eab32ee046ccd1bcc5f9105fe402c61fcd0c13eeb8b5",
        strip_prefix = "term-0.6.1",
        build_file = Label("//cargo/remote:BUILD.term-0.6.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror__1_0_23",
        url = "https://crates.io/api/v1/crates/thiserror/1.0.23/download",
        type = "tar.gz",
        sha256 = "76cc616c6abf8c8928e2fdcc0dbfab37175edd8fb49a4641066ad1364fdab146",
        strip_prefix = "thiserror-1.0.23",
        build_file = Label("//cargo/remote:BUILD.thiserror-1.0.23.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thiserror_impl__1_0_23",
        url = "https://crates.io/api/v1/crates/thiserror-impl/1.0.23/download",
        type = "tar.gz",
        sha256 = "9be73a2caec27583d0046ef3796c3794f868a5bc813db689eed00c7631275cd1",
        strip_prefix = "thiserror-impl-1.0.23",
        build_file = Label("//cargo/remote:BUILD.thiserror-impl-1.0.23.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__thread_local__1_0_1",
        url = "https://crates.io/api/v1/crates/thread_local/1.0.1/download",
        type = "tar.gz",
        sha256 = "d40c6d1b69745a6ec6fb1ca717914848da4b44ae29d9b3080cbee91d72a69b14",
        strip_prefix = "thread_local-1.0.1",
        build_file = Label("//cargo/remote:BUILD.thread_local-1.0.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__time__0_1_44",
        url = "https://crates.io/api/v1/crates/time/0.1.44/download",
        type = "tar.gz",
        sha256 = "6db9e6914ab8b1ae1c260a4ae7a49b6c5611b40328a735b21862567685e73255",
        strip_prefix = "time-0.1.44",
        build_file = Label("//cargo/remote:BUILD.time-0.1.44.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__unicode_xid__0_2_1",
        url = "https://crates.io/api/v1/crates/unicode-xid/0.2.1/download",
        type = "tar.gz",
        sha256 = "f7fe0bb3479651439c9112f72b6c505038574c9fbb575ed1bf3b797fa39dd564",
        strip_prefix = "unicode-xid-0.2.1",
        build_file = Label("//cargo/remote:BUILD.unicode-xid-0.2.1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_10_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.10.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "1a143597ca7c7793eff794def352d41792a93c481eb1042423ff7ff72ba2c31f",
        strip_prefix = "wasi-0.10.0+wasi-snapshot-preview1",
        build_file = Label("//cargo/remote:BUILD.wasi-0.10.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasi__0_9_0_wasi_snapshot_preview1",
        url = "https://crates.io/api/v1/crates/wasi/0.9.0+wasi-snapshot-preview1/download",
        type = "tar.gz",
        sha256 = "cccddf32554fecc6acb585f82a32a72e28b48f8c4c1883ddfeeeaa96f7d8e519",
        strip_prefix = "wasi-0.9.0+wasi-snapshot-preview1",
        build_file = Label("//cargo/remote:BUILD.wasi-0.9.0+wasi-snapshot-preview1.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen/0.2.69/download",
        type = "tar.gz",
        sha256 = "3cd364751395ca0f68cafb17666eee36b63077fb5ecd972bbcd74c90c4bf736e",
        strip_prefix = "wasm-bindgen-0.2.69",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_backend__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-backend/0.2.69/download",
        type = "tar.gz",
        sha256 = "1114f89ab1f4106e5b55e688b828c0ab0ea593a1ea7c094b141b14cbaaec2d62",
        strip_prefix = "wasm-bindgen-backend-0.2.69",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-backend-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro/0.2.69/download",
        type = "tar.gz",
        sha256 = "7a6ac8995ead1f084a8dea1e65f194d0973800c7f571f6edd70adf06ecf77084",
        strip_prefix = "wasm-bindgen-macro-0.2.69",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_macro_support__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-macro-support/0.2.69/download",
        type = "tar.gz",
        sha256 = "b5a48c72f299d80557c7c62e37e7225369ecc0c963964059509fbafe917c7549",
        strip_prefix = "wasm-bindgen-macro-support-0.2.69",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-macro-support-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__wasm_bindgen_shared__0_2_69",
        url = "https://crates.io/api/v1/crates/wasm-bindgen-shared/0.2.69/download",
        type = "tar.gz",
        sha256 = "7e7811dd7f9398f14cc76efd356f98f03aa30419dea46aa810d71e819fc97158",
        strip_prefix = "wasm-bindgen-shared-0.2.69",
        build_file = Label("//cargo/remote:BUILD.wasm-bindgen-shared-0.2.69.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__web_sys__0_3_46",
        url = "https://crates.io/api/v1/crates/web-sys/0.3.46/download",
        type = "tar.gz",
        sha256 = "222b1ef9334f92a21d3fb53dc3fd80f30836959a90f9274a626d7e06315ba3c3",
        strip_prefix = "web-sys-0.3.46",
        build_file = Label("//cargo/remote:BUILD.web-sys-0.3.46.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__whoami__1_0_3",
        url = "https://crates.io/api/v1/crates/whoami/1.0.3/download",
        type = "tar.gz",
        sha256 = "d595b2e146f36183d6a590b8d41568e2bc84c922267f43baf61c956330eeb436",
        strip_prefix = "whoami-1.0.3",
        build_file = Label("//cargo/remote:BUILD.whoami-1.0.3.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi__0_3_9",
        url = "https://crates.io/api/v1/crates/winapi/0.3.9/download",
        type = "tar.gz",
        sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419",
        strip_prefix = "winapi-0.3.9",
        build_file = Label("//cargo/remote:BUILD.winapi-0.3.9.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_i686_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-i686-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6",
        strip_prefix = "winapi-i686-pc-windows-gnu-0.4.0",
        build_file = Label("//cargo/remote:BUILD.winapi-i686-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__winapi_x86_64_pc_windows_gnu__0_4_0",
        url = "https://crates.io/api/v1/crates/winapi-x86_64-pc-windows-gnu/0.4.0/download",
        type = "tar.gz",
        sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f",
        strip_prefix = "winapi-x86_64-pc-windows-gnu-0.4.0",
        build_file = Label("//cargo/remote:BUILD.winapi-x86_64-pc-windows-gnu-0.4.0.bazel"),
    )

    maybe(
        http_archive,
        name = "raze__zip__0_5_9",
        url = "https://crates.io/api/v1/crates/zip/0.5.9/download",
        type = "tar.gz",
        sha256 = "cc2896475a242c41366941faa27264df2cb935185a92e059a004d0048feb2ac5",
        strip_prefix = "zip-0.5.9",
        build_file = Label("//cargo/remote:BUILD.zip-0.5.9.bazel"),
    )
