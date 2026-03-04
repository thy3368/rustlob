; ModuleID = 'build_script_build.226d0e66a8b5160c-cgu.0'
source_filename = "build_script_build.226d0e66a8b5160c-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"alloc::boxed::Box<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>" = type { %"core::ptr::unique::Unique<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", %"alloc::alloc::Global" }
%"core::ptr::unique::Unique<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>" = type { %"core::ptr::non_null::NonNull<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", %"core::marker::PhantomData<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>" }
%"core::ptr::non_null::NonNull<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>" = type { { ptr, ptr } }
%"core::marker::PhantomData<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>" = type {}
%"alloc::alloc::Global" = type {}
%"core::mem::maybe_uninit::MaybeUninit<std::ffi::os_str::OsString>" = type { [3 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<core::option::Option<std::ffi::os_str::OsString>>" = type { [3 x i64] }

@alloc_69009fdc319497586282719e739ab5f8 = private unnamed_addr constant [136 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/collections/btree/navigate.rs\00", align 1
@alloc_1df1e5171bffdf21494df69d159bd444 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00\C6\00\00\00'\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs2XfqmVweRya_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs2XfqmVweRya_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs2XfqmVweRya_18build_script_build }>, align 8
@alloc_c7175637917997c04b44e3b2099eecca = private unnamed_addr constant [113 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/str.rs\00", align 1
@alloc_3f2fbfdca196a5b824209b380ee7ae1b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c7175637917997c04b44e3b2099eecca, [16 x i8] c"p\00\00\00\00\00\00\00\B1\00\00\00\16\00\00\00" }>, align 8
@alloc_ca673fb95acb8e58af271999e89294ae = private unnamed_addr constant [53 x i8] c"attempt to join into collection with len > usize::MAX", align 1
@alloc_60488e92c3d9250777708a132d567f7b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c7175637917997c04b44e3b2099eecca, [16 x i8] c"p\00\00\00\00\00\00\00\9A\00\00\00\0A\00\00\00" }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRINtNtB8_6option6OptionReENtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_71264e62a593ae064235a5eb90a16b4a = private unnamed_addr constant [15 x i8] c"freebsd-version", align 1
@alloc_e9aa3e56236bea0534a07b33b08bbbe6 = private unnamed_addr constant [2 x i8] c"10", align 1
@alloc_ae52c2733f312a4a903aef7e6436cb13 = private unnamed_addr constant [2 x i8] c"11", align 1
@alloc_3b059e5eb8e06e7498f909e7a08cef57 = private unnamed_addr constant [2 x i8] c"12", align 1
@alloc_d57d03743ee0b3cf85ca6cc66dce7f4d = private unnamed_addr constant [2 x i8] c"13", align 1
@alloc_e33ac00bdbd8c0cbb04273e924bd654c = private unnamed_addr constant [2 x i8] c"14", align 1
@alloc_27f411dcc9955beae922af37f2bb21f7 = private unnamed_addr constant [2 x i8] c"15", align 1
@alloc_e7b0dd178336291b9ad3b8b25bc77cb0 = private unnamed_addr constant [4 x i8] c"emcc", align 1
@alloc_53695a5ce3568835c4a92269d444b5c9 = private unnamed_addr constant [12 x i8] c"-dumpversion", align 1
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_57e2a3f3daa80a9da338a6fbc7fe2a99 = private unnamed_addr constant [46 x i8] c"Failed to get rustc version: missing RUSTC env", align 1
@alloc_af090edf81da88c0b13a8e9ba679e4b3 = private unnamed_addr constant [94 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libc-0.2.177/build.rs\00", align 1
@alloc_a0229857e2e413914482ab1a558bf8f4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\AF\00\00\00&\00\00\00" }>, align 8
@alloc_f36ce88bd5d4a921175f5521f484b675 = private unnamed_addr constant [13 x i8] c"RUSTC_WRAPPER", align 1
@alloc_0a95b2846250f640f3e914bc2bbe7701 = private unnamed_addr constant [7 x i8] c"--rustc", align 1
@alloc_a887f9858119cc7413062dc002c4d9ab = private unnamed_addr constant [9 x i8] c"--version", align 1
@alloc_c33e5af42b9b9e21f43a4fcb9c0ba190 = private unnamed_addr constant [27 x i8] c"Failed to get rustc version", align 1
@alloc_c8371de64884c804df3fe9a38c6ab64e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\C1\00\00\00\1F\00\00\00" }>, align 8
@alloc_dfa4fbb8607feef357360e24a0ecaa6f = private unnamed_addr constant [24 x i8] c"\15failed to run rustc: \C0\00", align 1
@alloc_bb62e3d175e305a454323ef105b47882 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\C3\00\00\00\05\00\00\00" }>, align 8
@alloc_55e278c996565db65fe0fb6e7409cbbb = private unnamed_addr constant [6 x i8] c"clippy", align 1
@alloc_ca36d7e792bb4bbd1a68749f90007ce8 = private unnamed_addr constant [7 x i8] c"rustc 1", align 1
@alloc_99ffb679049157f8e234fe49e0609c15 = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr @alloc_ca36d7e792bb4bbd1a68749f90007ce8, [8 x i8] c"\07\00\00\00\00\00\00\00" }>, align 8
@alloc_283d534c2070da3e4efc423f267bf494 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\E2\00\00\00\05\00\00\00" }>, align 8
@alloc_fb2e3f631cd125c14f7441a1357cf087 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\EF\00\00\00\17\00\00\00" }>, align 8
@alloc_e11c5658ee80a4da21f3f7793a294e3c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\F3\00\00\00\17\00\00\00" }>, align 8
@alloc_11cddd32ce3a2e81eed0afcf9afbb3a4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\F3\00\00\00\11\00\00\00" }>, align 8
@alloc_99ce08ad4fc15ca92c8576e66a043a53 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\DE\00\00\00\13\00\00\00" }>, align 8
@alloc_29af473fbfd8114b5ab99329593ea058 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\DA\00\00\00\08\00\00\00" }>, align 8
@alloc_742f06589122110502429e832b81e8bd = private unnamed_addr constant [32 x i8] c"cargo:rerun-if-changed=build.rs\0A", align 1
@alloc_509e3f14595a72dfc2af0a28f5824017 = private unnamed_addr constant [30 x i8] c"CARGO_FEATURE_RUSTC_DEP_OF_STD", align 1
@alloc_f73607afcba5e721c2712249402644b6 = private unnamed_addr constant [7 x i8] c"LIBC_CI", align 1
@alloc_1e1fc66c1706c6c7501acca2ae8010f4 = private unnamed_addr constant [20 x i8] c"CARGO_CFG_TARGET_ENV", align 1
@alloc_aa4687de82972c6f88dd4ebd068e3b63 = private unnamed_addr constant [19 x i8] c"CARGO_CFG_TARGET_OS", align 1
@alloc_6508c675143a2a16e0690055cd395724 = private unnamed_addr constant [30 x i8] c"CARGO_CFG_TARGET_POINTER_WIDTH", align 1
@alloc_0d3bcf6fb685f000bc18304ea76cbac4 = private unnamed_addr constant [21 x i8] c"CARGO_CFG_TARGET_ARCH", align 1
@alloc_b74b27f2b9f751849fcbc82dbd3a9d08 = private unnamed_addr constant [62 x i8] c"cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_FREEBSD_VERSION\0A", align 1
@alloc_aaa658f8720b91022cfd120b3be84301 = private unnamed_addr constant [34 x i8] c"RUST_LIBC_UNSTABLE_FREEBSD_VERSION", align 1
@alloc_d3ef88d2871426aa76206ddd2ecd76d5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00A\00\00\00$\00\00\00" }>, align 8
@alloc_8ef8a8c2c947634d07c01270a783d130 = private unnamed_addr constant [46 x i8] c")cargo:warning=setting FreeBSD version to \C0\01\0A\00", align 1
@alloc_7267420313fdc34f79da1c04bfca7409 = private unnamed_addr constant [9 x i8] c"freebsd10", align 1
@alloc_5581ed16f5c58ecd3f36713b9b396029 = private unnamed_addr constant [9 x i8] c"freebsd11", align 1
@alloc_55f07188386ace482603892e4768112d = private unnamed_addr constant [9 x i8] c"freebsd12", align 1
@alloc_028f45a065ad7442c332be763445b925 = private unnamed_addr constant [9 x i8] c"freebsd13", align 1
@alloc_358590eecf303ad391259af81e368788 = private unnamed_addr constant [9 x i8] c"freebsd14", align 1
@alloc_c8539d7d8992b0450a5874fa781e9124 = private unnamed_addr constant [9 x i8] c"freebsd15", align 1
@alloc_ccedf80c3ce4e46e2ff8efee35ec798b = private unnamed_addr constant [23 x i8] c"emscripten_old_stat_abi", align 1
@alloc_154439d6e8351f7172ea58cb90d2dd09 = private unnamed_addr constant [30 x i8] c"RUST_LIBC_UNSTABLE_MUSL_V1_2_3", align 1
@alloc_64de700dc0d3712bf4f0fd23fc9b97f6 = private unnamed_addr constant [58 x i8] c"cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_MUSL_V1_2_3\0A", align 1
@alloc_be0c7e2eb8d81d67a6db9a856123bb7e = private unnamed_addr constant [11 x i8] c"loongarch64", align 1
@alloc_830cd488b6068638e05ed5b0c299b4af = private unnamed_addr constant [4 x i8] c"ohos", align 1
@alloc_513019cde2cbfb4427cb8f1afc437e08 = private unnamed_addr constant [11 x i8] c"musl_v1_2_3", align 1
@alloc_508b13eade4b92efdda744da70d08ff7 = private unnamed_addr constant [36 x i8] c"RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64", align 1
@alloc_b745d31eb2902e488a48adfdc7a9757f = private unnamed_addr constant [64 x i8] c"cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_LINUX_TIME_BITS64\0A", align 1
@alloc_681b6f9e783332c8e0b8ad7b08df1498 = private unnamed_addr constant [17 x i8] c"linux_time_bits64", align 1
@alloc_e099490f9865495bf255e49aa607a840 = private unnamed_addr constant [67 x i8] c"cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS\0A", align 1
@alloc_3e0db014760956dcb5153ae64d55b081 = private unnamed_addr constant [60 x i8] c"cargo:rerun-if-env-changed=RUST_LIBC_UNSTABLE_GNU_TIME_BITS\0A", align 1
@alloc_772e61a39199df4134c467e272d2cf4b = private unnamed_addr constant [3 x i8] c"gnu", align 1
@alloc_70a1e7dc3879e83c39c209c1ae5f1722 = private unnamed_addr constant [5 x i8] c"linux", align 1
@alloc_8e020aace2b3cf2c6b8375c8868270b7 = private unnamed_addr constant [2 x i8] c"32", align 1
@alloc_22a6d0e24a3ac3ed7016f4ca447b0cea = private unnamed_addr constant [7 x i8] c"riscv32", align 1
@alloc_4a29a4faa0904cd7ff982831f2813e90 = private unnamed_addr constant [6 x i8] c"x86_64", align 1
@alloc_a81a2677393ac2707db2f683d48ac6b7 = private unnamed_addr constant [32 x i8] c"RUST_LIBC_UNSTABLE_GNU_TIME_BITS", align 1
@alloc_9329be348e7e4f3c8cc453f36256cbfd = private unnamed_addr constant [39 x i8] c"RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS", align 1
@alloc_14fc90d5f706773754d40e4dccd34450 = private unnamed_addr constant [92 x i8] c"Do not set both RUST_LIBC_UNSTABLE_GNU_TIME_BITS and RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS", align 1
@alloc_659c02f340995e0c4d05cbbdc231b829 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00s\00\00\00\1F\00\00\00" }>, align 8
@alloc_8092ccd99cb94b0213fd5864ca7ee6ea = private unnamed_addr constant [2 x i8] c"64", align 1
@alloc_4a184034f37022296f6ca89b4adb3768 = private unnamed_addr constant [68 x i8] c"Invalid value for RUST_LIBC_UNSTABLE_GNU_TIME_BITS, must be 32 or 64", align 1
@alloc_7cba2f35f97c1b56c6e6ba52f23cb86b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00w\00\00\00 \00\00\00" }>, align 8
@alloc_12b500c16d6393901618de0cf55c3e6c = private unnamed_addr constant [75 x i8] c"Invalid value for RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS, must be 32 or 64", align 1
@alloc_f8b23ce9691cdedbe96412eba55673b0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00y\00\00\00 \00\00\00" }>, align 8
@alloc_5b38d234a72f2da1e93ed696dcb5b073 = private unnamed_addr constant [118 x i8] c"Invalid value for RUST_LIBC_UNSTABLE_GNU_TIME_BITS or RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS, must be 32, 64 or unset", align 1
@alloc_1d8dfd7aeb6a932d2b8d6d6490c15ab7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00|\00\00\00\09\00\00\00" }>, align 8
@alloc_a4772b3acfc19af28fefe691db64c6aa = private unnamed_addr constant [15 x i8] c"gnu_time_bits64", align 1
@alloc_4052f5f320831d7a280bd8ee23d7c161 = private unnamed_addr constant [22 x i8] c"gnu_file_offset_bits64", align 1
@alloc_e051788150efb5e0f212c696366647c3 = private unnamed_addr constant [18 x i8] c"libc_deny_warnings", align 1
@alloc_e300d0c2c56fc656630ece49b293f3f6 = private unnamed_addr constant [17 x i8] c"libc_thread_local", align 1
@alloc_c1dd1d9f50ed06e24759135ae11c1cd7 = private unnamed_addr constant [13 x i8] c"espidf_time32", align 1
@alloc_0932325d29f8c848cece173911e7c4a6 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_ccedf80c3ce4e46e2ff8efee35ec798b, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_c1dd1d9f50ed06e24759135ae11c1cd7, [8 x i8] c"\0D\00\00\00\00\00\00\00", ptr @alloc_7267420313fdc34f79da1c04bfca7409, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_5581ed16f5c58ecd3f36713b9b396029, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_55f07188386ace482603892e4768112d, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_028f45a065ad7442c332be763445b925, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_358590eecf303ad391259af81e368788, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_c8539d7d8992b0450a5874fa781e9124, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_4052f5f320831d7a280bd8ee23d7c161, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_a4772b3acfc19af28fefe691db64c6aa, [8 x i8] c"\0F\00\00\00\00\00\00\00", ptr @alloc_e051788150efb5e0f212c696366647c3, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_e300d0c2c56fc656630ece49b293f3f6, [8 x i8] c"\11\00\00\00\00\00\00\00", ptr @alloc_681b6f9e783332c8e0b8ad7b08df1498, [8 x i8] c"\11\00\00\00\00\00\00\00", ptr @alloc_513019cde2cbfb4427cb8f1afc437e08, [8 x i8] c"\0B\00\00\00\00\00\00\00" }>, align 8
@alloc_8c8806985cff4e5eb0a771b7bf66c1ea = private unnamed_addr constant [32 x i8] c"\1Acargo:rustc-check-cfg=cfg(\C0\02)\0A\00", align 1
@alloc_2cddc5e59ad0ce52fa6a12317b7d9940 = private unnamed_addr constant [9 x i8] c"target_os", align 1
@alloc_06bdffecd12566b07b46a1d9c671b787 = private unnamed_addr constant [6 x i8] c"switch", align 1
@alloc_56682c411a884305a0498f0904259ddf = private unnamed_addr constant [3 x i8] c"aix", align 1
@alloc_45bb0232104d815c4dda9598abc92521 = private unnamed_addr constant [4 x i8] c"hurd", align 1
@alloc_7d89cad1193e3093f83db65de654886c = private unnamed_addr constant [5 x i8] c"rtems", align 1
@alloc_c681dba5e39d19fa023e5cc12642d541 = private unnamed_addr constant [8 x i8] c"visionos", align 1
@alloc_83b5f38e8216cdcf7d09bbba859e9e33 = private unnamed_addr constant [5 x i8] c"nuttx", align 1
@alloc_8556a45425763a509e2688076730be6f = private unnamed_addr constant [6 x i8] c"cygwin", align 1
@alloc_227cd49c7c33a6f341699cff0c19de8b = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_06bdffecd12566b07b46a1d9c671b787, [8 x i8] c"\06\00\00\00\00\00\00\00", ptr @alloc_56682c411a884305a0498f0904259ddf, [8 x i8] c"\03\00\00\00\00\00\00\00", ptr @alloc_830cd488b6068638e05ed5b0c299b4af, [8 x i8] c"\04\00\00\00\00\00\00\00", ptr @alloc_45bb0232104d815c4dda9598abc92521, [8 x i8] c"\04\00\00\00\00\00\00\00", ptr @alloc_7d89cad1193e3093f83db65de654886c, [8 x i8] c"\05\00\00\00\00\00\00\00", ptr @alloc_c681dba5e39d19fa023e5cc12642d541, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_83b5f38e8216cdcf7d09bbba859e9e33, [8 x i8] c"\05\00\00\00\00\00\00\00", ptr @alloc_8556a45425763a509e2688076730be6f, [8 x i8] c"\06\00\00\00\00\00\00\00" }>, align 8
@alloc_df599e29b3820982aef0645887d35e8c = private unnamed_addr constant [10 x i8] c"target_env", align 1
@alloc_369c643820514fbe33fb426e73a3da06 = private unnamed_addr constant [7 x i8] c"illumos", align 1
@alloc_7e4f24954a4fa587ee17e75c042a2c6a = private unnamed_addr constant [4 x i8] c"wasi", align 1
@alloc_9ece1e428f15509c5755d645816c31a7 = private unnamed_addr constant [12 x i8] c"nto71_iosock", align 1
@alloc_47a2c3621a8fa9e428c3400e4faf2bb1 = private unnamed_addr constant [5 x i8] c"nto80", align 1
@alloc_50543c6fc28a9aa73f9310524f136113 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_369c643820514fbe33fb426e73a3da06, [8 x i8] c"\07\00\00\00\00\00\00\00", ptr @alloc_7e4f24954a4fa587ee17e75c042a2c6a, [8 x i8] c"\04\00\00\00\00\00\00\00", ptr @alloc_56682c411a884305a0498f0904259ddf, [8 x i8] c"\03\00\00\00\00\00\00\00", ptr @alloc_830cd488b6068638e05ed5b0c299b4af, [8 x i8] c"\04\00\00\00\00\00\00\00", ptr @alloc_9ece1e428f15509c5755d645816c31a7, [8 x i8] c"\0C\00\00\00\00\00\00\00", ptr @alloc_47a2c3621a8fa9e428c3400e4faf2bb1, [8 x i8] c"\05\00\00\00\00\00\00\00" }>, align 8
@alloc_9259c4107c8646157225831547e51707 = private unnamed_addr constant [11 x i8] c"target_arch", align 1
@alloc_cbb73a85e2ed78c1dc2a615b03408878 = private unnamed_addr constant [8 x i8] c"mips32r6", align 1
@alloc_1cb3d6a6216aeea0aa93cb8b80a5f107 = private unnamed_addr constant [8 x i8] c"mips64r6", align 1
@alloc_a5dc2de5b3efc052edbd4e83ca0843da = private unnamed_addr constant [4 x i8] c"csky", align 1
@alloc_8ae71c71763656cbeae0e33c1cd3df64 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_be0c7e2eb8d81d67a6db9a856123bb7e, [8 x i8] c"\0B\00\00\00\00\00\00\00", ptr @alloc_cbb73a85e2ed78c1dc2a615b03408878, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_1cb3d6a6216aeea0aa93cb8b80a5f107, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_a5dc2de5b3efc052edbd4e83ca0843da, [8 x i8] c"\04\00\00\00\00\00\00\00" }>, align 8
@alloc_2bd9fa038d7fb2af8467fdc2c22fe0ae = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_2cddc5e59ad0ce52fa6a12317b7d9940, [8 x i8] c"\09\00\00\00\00\00\00\00", ptr @alloc_227cd49c7c33a6f341699cff0c19de8b, [8 x i8] c"\08\00\00\00\00\00\00\00", ptr @alloc_df599e29b3820982aef0645887d35e8c, [8 x i8] c"\0A\00\00\00\00\00\00\00", ptr @alloc_50543c6fc28a9aa73f9310524f136113, [8 x i8] c"\06\00\00\00\00\00\00\00", ptr @alloc_9259c4107c8646157225831547e51707, [8 x i8] c"\0B\00\00\00\00\00\00\00", ptr @alloc_8ae71c71763656cbeae0e33c1cd3df64, [8 x i8] c"\04\00\00\00\00\00\00\00" }>, align 8
@alloc_4e81f3446308e52f5d03e9e4175413e4 = private unnamed_addr constant [3 x i8] c"\22,\22", align 1
@alloc_f9b7a4a216e67c48cfcff7c8ca3d1ad4 = private unnamed_addr constant [45 x i8] c"\1Acargo:rustc-check-cfg=cfg(\C0\09,values(\22\C0\04\22))\0A\00", align 1
@alloc_c8a65b5fe9f8c8ff66f0add8177e4932 = private unnamed_addr constant [101 x i8] c"RUST_LIBC_UNSTABLE_GNU_FILE_OFFSET_BITS must be 64 or unset if RUST_LIBC_UNSTABLE_GNU_TIME_BITS is 64", align 1
@alloc_da02dd9cd40c96b9368bc58a598563e2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00\80\00\00\00\09\00\00\00" }>, align 8
@alloc_c63b2e5039c7d990f01e55018d57af8b = private unnamed_addr constant [38 x i8] c"FreeBSD older than 10 is not supported", align 1
@alloc_4aaeaad2afeac9a2adb9f1e49e255ee5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00K\00\00\00\18\00\00\00" }>, align 8
@alloc_dc81aac016627a2e79d483ac8b04c639 = private unnamed_addr constant [53 x i8] c"\12trying to set cfg \C0\1F, but it is not in ALLOWED_CFGS\00", align 1
@alloc_201211583369b7dd25e59c1fbc508159 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_af090edf81da88c0b13a8e9ba679e4b3, [16 x i8] c"]\00\00\00\00\00\00\00%\01\00\00\05\00\00\00" }>, align 8
@alloc_0f615e922801252a74ad4557d8ed2760 = private unnamed_addr constant [21 x i8] c"\10cargo:rustc-cfg=\C0\01\0A\00", align 1
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build, [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsc_NtNtCsjMrxcFdYDNN_4core3num5errorNtB5_13ParseIntErrorNtNtB9_3fmt5Debug3fmt }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP = external local_unnamed_addr global [256 x i8]
@alloc_7fe94be2e120ffbd80c490b1b3c481ee = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/pattern.rs\00", align 1
@alloc_37d2e53432a03a1f90b3e7253015eaf9 = private unnamed_addr constant [4 x i8] c"None", align 1
@vtable.6 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build }>, align 8
@alloc_9535bf4c204f3eb9b19ec2c83e446e52 = private unnamed_addr constant [4 x i8] c"Some", align 1
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8
@vtable.7 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build }>, align 8
@alloc_f62df14955f7d78bca139b0a7668683d = private unnamed_addr constant [13 x i8] c"ParseIntError", align 1
@alloc_a5d866b1768ad3f826bccdb004a1a8ae = private unnamed_addr constant [4 x i8] c"kind", align 1
@alloc_59ba7b9f7211443cd55a366616eef46a = private unnamed_addr constant [5 x i8] c"Empty", align 1
@alloc_00315c78e51d29fe6b3102a4c1ecf6ef = private unnamed_addr constant [12 x i8] c"InvalidDigit", align 1
@alloc_bd3a3f3879e0d5f64554753e977f58d4 = private unnamed_addr constant [11 x i8] c"PosOverflow", align 1
@alloc_0964bb2a4870637395c77a018495bd5c = private unnamed_addr constant [11 x i8] c"NegOverflow", align 1
@alloc_6566120a3a17f930e960a0863fcbd591 = private unnamed_addr constant [4 x i8] c"Zero", align 1
@switch.table._RNvCs2XfqmVweRya_18build_script_build4main = private unnamed_addr constant [5 x ptr] [ptr @alloc_7267420313fdc34f79da1c04bfca7409, ptr @alloc_5581ed16f5c58ecd3f36713b9b396029, ptr @alloc_55f07188386ace482603892e4768112d, ptr @alloc_028f45a065ad7442c332be763445b925, ptr @alloc_358590eecf303ad391259af81e368788], align 8
@switch.table._RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build = private unnamed_addr constant [5 x i64] [i64 5, i64 12, i64 11, i64 11, i64 4], align 8
@switch.table._RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build.88 = private unnamed_addr constant [5 x ptr] [ptr @alloc_59ba7b9f7211443cd55a366616eef46a, ptr @alloc_00315c78e51d29fe6b3102a4c1ecf6ef, ptr @alloc_bd3a3f3879e0d5f64554753e977f58d4, ptr @alloc_0964bb2a4870637395c77a018495bd5c, ptr @alloc_6566120a3a17f930e960a0863fcbd591], align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECs2XfqmVweRya_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_2 = load i64, ptr %_1, align 8, !range !3, !noundef !4
  %0 = icmp eq i64 %_2, 0
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %.val = load i64, ptr %1, align 8
  br i1 %0, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %2 = icmp eq i64 %.val, 0
  br i1 %2, label %bb1, label %bb1.sink.split

bb3:                                              ; preds = %start
  switch i64 %.val, label %bb1.sink.split [
    i64 -9223372036854775808, label %bb1
    i64 0, label %bb1
  ]

bb1.sink.split:                                   ; preds = %bb3, %bb2
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val3 = load ptr, ptr %3, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb1

bb1:                                              ; preds = %bb1.sink.split, %bb3, %bb3, %bb2
  ret void
}

; core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", ptr %_1.val, i64 %_3.sroa.0.09.i.i
  %2 = add nuw i64 %_3.sroa.0.09.i.i, 1
  %_6.val.i.i = load ptr, ptr %_6.i.i, align 8, !alias.scope !5
  %3 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %3, align 8, !alias.scope !5, !nonnull !4, !align !8, !noundef !4
  %4 = load ptr, ptr %_6.val7.i.i, align 8, !invariant.load !4, !noalias !5
  %.not.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i, label %bb3.i.i.i, label %is_not_null.i.i.i

is_not_null.i.i.i:                                ; preds = %bb5.i.i
  %5 = icmp ne ptr %_6.val.i.i, null
  tail call void @llvm.assume(i1 %5)
  invoke void %4(ptr noundef nonnull %_6.val.i.i)
          to label %bb3.i.i.i unwind label %cleanup.i.i.i, !noalias !5

bb3.i.i.i:                                        ; preds = %is_not_null.i.i.i, %bb5.i.i
  %6 = icmp ne ptr %_6.val.i.i, null
  tail call void @llvm.assume(i1 %6)
  %7 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 8
  %8 = load i64, ptr %7, align 8, !range !9, !invariant.load !4, !noalias !5
  %9 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 16
  %10 = load i64, ptr %9, align 8, !range !10, !invariant.load !4, !noalias !5
  %11 = add i64 %10, -1
  %12 = icmp sgt i64 %11, -1
  tail call void @llvm.assume(i1 %12)
  %13 = icmp eq i64 %8, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #22, !noalias !5
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i

cleanup.i.i.i:                                    ; preds = %is_not_null.i.i.i
  %14 = landingpad { ptr, i32 }
          cleanup
  %15 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !9, !invariant.load !4, !noalias !5
  %17 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !10, !invariant.load !4, !noalias !5
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  tail call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %bb4.i.i.preheader, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i

bb4.i.i.preheader:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i, %cleanup.i.i.i
  br label %bb4.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i: ; preds = %cleanup.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #22, !noalias !5
  br label %bb4.i.i.preheader

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4.i.i:                                          ; preds = %bb4.i.i.preheader, %bb3.i.i
  %_3.sroa.0.1.i.i = phi i64 [ %22, %bb3.i.i ], [ %2, %bb4.i.i.preheader ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %_1.val1
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", ptr %_1.val, i64 %_3.sroa.0.1.i.i
  %22 = add i64 %_3.sroa.0.1.i.i, 1
  %_4.val.i.i = load ptr, ptr %_4.i.i, align 8, !alias.scope !5
  %23 = getelementptr i8, ptr %_4.i.i, i64 8
  %_4.val6.i.i = load ptr, ptr %23, align 8, !alias.scope !5, !nonnull !4, !align !8, !noundef !4
; invoke core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #23
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !5

terminate.i.i:                                    ; preds = %bb3.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24, !noalias !5
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !9, !noundef !4
  %25 = icmp eq i64 %_1.val2, 0
  br i1 %25, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !9, !noundef !4
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs2XfqmVweRya_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs2XfqmVweRya_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs2XfqmVweRya_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs2XfqmVweRya_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = load ptr, ptr %_1.8.val, align 8, !invariant.load !4
  %.not = icmp eq ptr %1, null
  br i1 %.not, label %bb3, label %is_not_null

is_not_null:                                      ; preds = %start
  %2 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %2)
  invoke void %1(ptr noundef nonnull %_1.0.val)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %is_not_null, %start
  %3 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %5 = load i64, ptr %4, align 8, !range !9, !invariant.load !4
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !10, !invariant.load !4
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs2XfqmVweRya_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #22
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs2XfqmVweRya_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs2XfqmVweRya_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
  ret void

cleanup:                                          ; preds = %is_not_null
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %13 = load i64, ptr %12, align 8, !range !9, !invariant.load !4
  %14 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %15 = load i64, ptr %14, align 8, !range !10, !invariant.load !4
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4: ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #22
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %bb4, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %1, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i, %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2 = load i64, ptr %2, align 8
  %3 = icmp eq i64 %.val2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2XfqmVweRya_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2XfqmVweRya_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2XfqmVweRya_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !11, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val24.i = load i64, ptr %1, align 8, !alias.scope !11
  store i8 0, ptr %.val.i, align 1, !noalias !11
  %2 = icmp eq i64 %.val24.i, 0
  br i1 %2, label %bb20.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %.val24.i, i64 noundef 1) #22
  br label %bb20.i

bb20.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i, %start
; invoke <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
  invoke void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb20.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %_1.val.i.i = load i64, ptr %_1, align 8, !alias.scope !14
  %4 = icmp eq i64 %_1.val.i.i, 0
  br i1 %4, label %bb10.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i = load ptr, ptr %5, align 8, !alias.scope !14, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %_1.val.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb10.i

bb4.i.i:                                          ; preds = %bb20.i
  %_1.val2.i.i = load i64, ptr %_1, align 8, !alias.scope !14
  %6 = icmp eq i64 %_1.val2.i.i, 0
  br i1 %6, label %bb19.i, label %bb2.i.i.i4.i4.i.i

bb2.i.i.i4.i4.i.i:                                ; preds = %bb4.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3.i.i = load ptr, ptr %7, align 8, !alias.scope !14, !nonnull !4, !noundef !4
  %alloc_size.i.i.i.i5.i5.i.i = shl nuw i64 %_1.val2.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3.i.i, i64 noundef %alloc_size.i.i.i.i5.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb19.i

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #23
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
          to label %bb18.i unwind label %cleanup2.i

bb9.i:                                            ; preds = %cleanup2.i, %bb10.i
  %.pn10.i = phi { ptr, i32 } [ %14, %cleanup2.i ], [ %3, %bb10.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val27.i = load ptr, ptr %10, align 8, !alias.scope !11, !align !17, !noundef !4
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val28.i = load i64, ptr %11, align 8, !alias.scope !11
  %12 = icmp eq ptr %.val27.i, null
  br i1 %12, label %bb8.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb9.i
  store i8 0, ptr %.val27.i, align 1
  %13 = icmp eq i64 %.val28.i, 0
  br i1 %13, label %bb8.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val27.i, i64 noundef %.val28.i, i64 noundef 1) #22
  br label %bb8.i

cleanup2.i:                                       ; preds = %bb19.i
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %bb9.i

bb18.i:                                           ; preds = %bb19.i
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val31.i = load ptr, ptr %15, align 8, !alias.scope !11, !align !17, !noundef !4
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val32.i = load i64, ptr %16, align 8, !alias.scope !11
  %17 = icmp eq ptr %.val31.i, null
  br i1 %17, label %bb17.i, label %bb2.i50.i

bb2.i50.i:                                        ; preds = %bb18.i
  store i8 0, ptr %.val31.i, align 1
  %18 = icmp eq i64 %.val32.i, 0
  br i1 %18, label %bb17.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i: ; preds = %bb2.i50.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val31.i, i64 noundef %.val32.i, i64 noundef 1) #22
  br label %bb17.i

bb8.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i, %bb2.i.i, %bb9.i
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val25.i = load ptr, ptr %19, align 8, !alias.scope !11, !align !17, !noundef !4
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val26.i = load i64, ptr %20, align 8, !alias.scope !11
  %21 = icmp eq ptr %.val25.i, null
  br i1 %21, label %bb7.i, label %bb2.i54.i

bb2.i54.i:                                        ; preds = %bb8.i
  store i8 0, ptr %.val25.i, align 1
  %22 = icmp eq i64 %.val26.i, 0
  br i1 %22, label %bb7.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i: ; preds = %bb2.i54.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val25.i, i64 noundef %.val26.i, i64 noundef 1) #22
  br label %bb7.i

bb17.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i, %bb2.i50.i, %bb18.i
  %23 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val29.i = load ptr, ptr %23, align 8, !alias.scope !11, !align !17, !noundef !4
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val30.i = load i64, ptr %24, align 8, !alias.scope !11
  %25 = icmp eq ptr %.val29.i, null
  br i1 %25, label %bb16.i, label %bb2.i58.i

bb2.i58.i:                                        ; preds = %bb17.i
  store i8 0, ptr %.val29.i, align 1
  %26 = icmp eq i64 %.val30.i, 0
  br i1 %26, label %bb16.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i: ; preds = %bb2.i58.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val29.i, i64 noundef %.val30.i, i64 noundef 1) #22
  br label %bb16.i

bb7.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i, %bb2.i54.i, %bb8.i
  %27 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #23
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
          to label %bb15.i unwind label %cleanup5.i

bb6.i:                                            ; preds = %cleanup5.i, %bb7.i
  %.pn16.i = phi { ptr, i32 } [ %34, %cleanup5.i ], [ %.pn10.i, %bb7.i ]
  %29 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val33.i = load ptr, ptr %29, align 8, !alias.scope !11, !align !18, !noundef !4
  %30 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val34.i = load i64, ptr %30, align 8, !alias.scope !11
  %31 = icmp eq ptr %.val33.i, null
  %32 = icmp eq i64 %.val34.i, 0
  %or.cond.i.i = select i1 %31, i1 true, i1 %32
  br i1 %or.cond.i.i, label %bb5.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb6.i
  %33 = shl nuw nsw i64 %.val34.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val33.i, i64 noundef %33, i64 noundef 4) #22
  br label %bb5.i

cleanup5.i:                                       ; preds = %bb16.i
  %34 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i

bb15.i:                                           ; preds = %bb16.i
  %35 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val35.i = load ptr, ptr %35, align 8, !alias.scope !11, !align !18, !noundef !4
  %36 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val36.i = load i64, ptr %36, align 8, !alias.scope !11
  %37 = icmp eq ptr %.val35.i, null
  %38 = icmp eq i64 %.val36.i, 0
  %or.cond.i63.i = select i1 %37, i1 true, i1 %38
  br i1 %or.cond.i63.i, label %bb14.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i: ; preds = %bb15.i
  %39 = shl nuw nsw i64 %.val36.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val35.i, i64 noundef %39, i64 noundef 4) #22
  br label %bb14.i

bb5.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb6.i
  %40 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val41.i = load i32, ptr %40, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i.i = icmp eq i32 %.val41.i, 3
  br i1 %cond.i.i, label %bb2.i.i.i, label %bb4.i

bb2.i.i.i:                                        ; preds = %bb5.i
  %41 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val42.i = load i32, ptr %41, align 4, !alias.scope !11
  %_5.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val42.i) #22
  br label %bb4.i

bb14.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i, %bb15.i
  %42 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val47.i = load i32, ptr %42, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i68.i = icmp eq i32 %.val47.i, 3
  br i1 %cond.i68.i, label %bb2.i.i70.i, label %bb13.i

bb2.i.i70.i:                                      ; preds = %bb14.i
  %43 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val48.i = load i32, ptr %43, align 4, !alias.scope !11
  %_5.i.i.i.i.i71.i = tail call noundef i32 @close(i32 noundef %.val48.i) #22
  br label %bb13.i

bb4.i:                                            ; preds = %bb2.i.i.i, %bb5.i
  %44 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val39.i = load i32, ptr %44, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i73.i = icmp eq i32 %.val39.i, 3
  br i1 %cond.i73.i, label %bb2.i.i75.i, label %bb3.i

bb2.i.i75.i:                                      ; preds = %bb4.i
  %45 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val40.i = load i32, ptr %45, align 4, !alias.scope !11
  %_5.i.i.i.i.i76.i = tail call noundef i32 @close(i32 noundef %.val40.i) #22
  br label %bb3.i

bb13.i:                                           ; preds = %bb2.i.i70.i, %bb14.i
  %46 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val45.i = load i32, ptr %46, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i78.i = icmp eq i32 %.val45.i, 3
  br i1 %cond.i78.i, label %bb2.i.i80.i, label %bb12.i

bb2.i.i80.i:                                      ; preds = %bb13.i
  %47 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val46.i = load i32, ptr %47, align 4, !alias.scope !11
  %_5.i.i.i.i.i81.i = tail call noundef i32 @close(i32 noundef %.val46.i) #22
  br label %bb12.i

bb3.i:                                            ; preds = %bb2.i.i75.i, %bb4.i
  %48 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val37.i = load i32, ptr %48, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i83.i = icmp eq i32 %.val37.i, 3
  br i1 %cond.i83.i, label %bb2.i.i85.i, label %bb1.i

bb2.i.i85.i:                                      ; preds = %bb3.i
  %49 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val38.i = load i32, ptr %49, align 4, !alias.scope !11
  %_5.i.i.i.i.i86.i = tail call noundef i32 @close(i32 noundef %.val38.i) #22
  br label %bb1.i

bb12.i:                                           ; preds = %bb2.i.i80.i, %bb13.i
  %50 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val43.i = load i32, ptr %50, align 8, !range !19, !alias.scope !11, !noundef !4
  %cond.i88.i = icmp eq i32 %.val43.i, 3
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs2XfqmVweRya_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !11
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs2XfqmVweRya_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs2XfqmVweRya_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::io::error::Error>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
  %bits.i.i.i = ptrtoint ptr %_1.val to i64
  %_5.i.i.i = and i64 %bits.i.i.i, 3
  %switch.i.i = icmp eq i64 %_5.i.i.i, 1
  br i1 %switch.i.i, label %bb2.i2.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs2XfqmVweRya_18build_script_build.exit, !prof !20

bb2.i2.i.i:                                       ; preds = %start
  %0 = getelementptr i8, ptr %_1.val, i64 -1
  %1 = icmp ne ptr %0, null
  tail call void @llvm.assume(i1 %1)
  %_6.val.i.i.i.i = load ptr, ptr %0, align 8
  %2 = getelementptr i8, ptr %_1.val, i64 7
  %_6.val1.i.i.i.i = load ptr, ptr %2, align 8, !nonnull !4, !align !8, !noundef !4
  %3 = load ptr, ptr %_6.val1.i.i.i.i, align 8, !invariant.load !4
  %.not.i.i.i.i.i.i = icmp eq ptr %3, null
  br i1 %.not.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i:                          ; preds = %bb2.i2.i.i
  %4 = icmp ne ptr %_6.val.i.i.i.i, null
  tail call void @llvm.assume(i1 %4)
  invoke void %3(ptr noundef nonnull %_6.val.i.i.i.i)
          to label %bb3.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %is_not_null.i.i.i.i.i.i, %bb2.i2.i.i
  %5 = icmp ne ptr %_6.val.i.i.i.i, null
  tail call void @llvm.assume(i1 %5)
  %6 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %7 = load i64, ptr %6, align 8, !range !9, !invariant.load !4
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %9 = load i64, ptr %8, align 8, !range !10, !invariant.load !4
  %10 = add i64 %9, -1
  %11 = icmp sgt i64 %10, -1
  tail call void @llvm.assume(i1 %11)
  %12 = icmp eq i64 %7, 0
  br i1 %12, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) %9) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i

cleanup.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %15 = load i64, ptr %14, align 8, !range !9, !invariant.load !4
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %17 = load i64, ptr %16, align 8, !range !10, !invariant.load !4
  %18 = add i64 %17, -1
  %19 = icmp sgt i64 %18, -1
  tail call void @llvm.assume(i1 %19)
  %20 = icmp eq i64 %15, 0
  br i1 %20, label %bb1.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) %17) #22
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 24, i64 noundef 8) #22
  resume { ptr, i32 } %13

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 24, i64 noundef 8) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs2XfqmVweRya_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs2XfqmVweRya_18build_script_build.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i = alloca [24 x i8], align 8
  %_x.i.i = alloca [72 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !21)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_x.i.i), !noalias !27
  %self1.sroa.0.0.copyload.i.i = load ptr, ptr %_1, align 8, !alias.scope !27
  %.not.i.i = icmp eq ptr %self1.sroa.0.0.copyload.i.i, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self1.sroa.5.0.copyload.i.i = load i64, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !27
  %self1.sroa.4.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.sroa.4.0.copyload.i.i = load i64, ptr %self1.sroa.4.0.self.sroa_idx.i.i, align 8, !alias.scope !27
  %full_range.sroa.4.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 8
  store ptr null, ptr %full_range.sroa.4.0._x.sroa_idx.i.i, align 8, !noalias !27
  %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 16
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !27
  %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 24
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !27
  %full_range.sroa.6.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 40
  store ptr null, ptr %full_range.sroa.6.0._x.sroa_idx.i.i, align 8, !noalias !27
  %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 48
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !27
  %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 56
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !27
  br label %bb3.i.i

bb3.i.i:                                          ; preds = %bb1.i.i, %start
  %.sink9.i.i = phi i64 [ 1, %bb1.i.i ], [ 0, %start ]
  %self1.sroa.5.0.copyload.sink.i.i = phi i64 [ %self1.sroa.5.0.copyload.i.i, %bb1.i.i ], [ 0, %start ]
  store i64 %.sink9.i.i, ptr %_x.i.i, align 8, !noalias !27
  %0 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 32
  store i64 %.sink9.i.i, ptr %0, align 8, !noalias !27
  %1 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 64
  store i64 %self1.sroa.5.0.copyload.sink.i.i, ptr %1, align 8, !noalias !27
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !4
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

bb3.lr.ph.i.i.i.i:                                ; preds = %bb3.i.i
  %kv.sroa.22.0._2.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i, i64 16
  br label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb4.i.i.i.i, %bb3.lr.ph.i.i.i.i
  %3 = phi ptr [ %2, %bb3.lr.ph.i.i.i.i ], [ %7, %bb4.i.i.i.i ]
  %kv.sroa.22.0.copyload.i.i.i.i = load i64, ptr %kv.sroa.22.0._2.sroa_idx.i.i.i.i, align 8, !noalias !28
  %_5.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 8
  %key.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<std::ffi::os_str::OsString>", ptr %_5.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %_9.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 272
  %_17.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<core::option::Option<std::ffi::os_str::OsString>>", ptr %_9.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %key.val.i.i.i.i.i = load i64, ptr %key.i.i.i.i.i, align 8, !noalias !28
  %4 = icmp eq i64 %key.val.i.i.i.i.i, 0
  br i1 %4, label %bb8.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb3.i.i.i.i
  %5 = getelementptr i8, ptr %key.i.i.i.i.i, i64 8
  %key.val1.i.i.i.i.i = load ptr, ptr %5, align 8, !noalias !28, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %key.val1.i.i.i.i.i, i64 noundef %key.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !28
  br label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i
  %self1.val.i.i.i.i.i.i.i = load i64, ptr %_17.i.i.i.i.i, align 8, !range !33, !noalias !28, !noundef !4
  switch i64 %self1.val.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb8.i.i.i.i.i
  %6 = getelementptr i8, ptr %_17.i.i.i.i.i, i64 8
  %self1.val2.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !28, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self1.val2.i.i.i.i.i.i.i, i64 noundef %self1.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !28
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i, %bb8.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !4
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !27
  ret void
}

; core::panicking::assert_failed::<core::option::Option<&str>, core::option::Option<&str>>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedINtNtB4_6option6OptionReEBM_ECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %0) unnamed_addr #2 {
start:
  %right = alloca [8 x i8], align 8
  %left = alloca [8 x i8], align 8
  store ptr %0, ptr %left, align 8
  store ptr @alloc_99ffb679049157f8e234fe49e0609c15, ptr %right, align 8
; call core::panicking::assert_failed_inner
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking19assert_failed_inner(i8 noundef 0, ptr noundef nonnull align 1 %left, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noundef nonnull align 1 %right, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noundef nonnull @alloc_c33e5af42b9b9e21f43a4fcb9c0ba190, ptr nonnull inttoptr (i64 55 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_283d534c2070da3e4efc423f267bf494) #25
  unreachable
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs2XfqmVweRya_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #3 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #22, !srcloc !34
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !9, !alias.scope !35, !noundef !4
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !35
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !35
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !3, !noalias !35, !noundef !4
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !33, !noalias !35, !noundef !4
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !35
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !35
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #26
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !35, !nonnull !4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !35
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !35
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !35
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #5 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs2XfqmVweRya_18build_script_build(ptr noundef nonnull %_4) #27
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs2XfqmVweRya_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs2XfqmVweRya_18build_script_build(ptr noundef nonnull readonly %0) #27, !noalias !38
  ret i32 0
}

; build_script_build::rustc_version_cmd
; Function Attrs: uwtable
define internal fastcc void @_RNvCs2XfqmVweRya_18build_script_build17rustc_version_cmd(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(56) %_0, i1 noundef zeroext %is_clippy_driver) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [8 x i8], align 8
  %_2.i40 = alloca [200 x i8], align 8
  %_2.i25 = alloca [200 x i8], align 8
  %_2.i = alloca [200 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_26 = alloca [24 x i8], align 8
  %_21 = alloca [56 x i8], align 8
  %output = alloca [56 x i8], align 8
  %cmd1 = alloca [200 x i8], align 8
  %_5 = alloca [24 x i8], align 8
  %cmd = alloca [200 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5)
  %0 = load i64, ptr %_3, align 8, !range !33, !noundef !4
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb32, label %bb33, !prof !41

bb33:                                             ; preds = %start
  %rustc.sroa.7.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %rustc.sroa.7.0.copyload = load ptr, ptr %rustc.sroa.7.0._3.sroa_idx, align 8
  %rustc.sroa.8.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %rustc.sroa.8.0.copyload = load i64, ptr %rustc.sroa.8.0._3.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f36ce88bd5d4a921175f5521f484b675, i64 noundef 13)
          to label %bb2 unwind label %cleanup.thread

cleanup.thread:                                   ; preds = %bb33
  %1 = landingpad { ptr, i32 }
          cleanup
  br label %bb30

bb32:                                             ; preds = %start
; call core::option::expect_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_57e2a3f3daa80a9da338a6fbc7fe2a99, i64 noundef 46, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a0229857e2e413914482ab1a558bf8f4) #25
  unreachable

bb2:                                              ; preds = %bb33
  %2 = load i64, ptr %_5, align 8, !range !33, !noundef !4
  %.not8 = icmp eq i64 %2, -9223372036854775808
  br i1 %.not8, label %bb4, label %bb5

bb5:                                              ; preds = %bb2
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %_41 = load i64, ptr %3, align 8, !noundef !4
  %4 = icmp eq i64 %_41, 0
  br i1 %4, label %bb6, label %bb7

bb4:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i), !noalias !42
  %5 = icmp ne ptr %rustc.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %5)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustc.sroa.7.0.copyload, i64 noundef %rustc.sroa.8.0.copyload)
          to label %bb2.i unwind label %cleanup.i, !noalias !42

cleanup.i:                                        ; preds = %bb4
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = icmp eq i64 %0, 0
  br i1 %7, label %bb24, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !42
  br label %bb24

bb2.i:                                            ; preds = %bb4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd, ptr noundef nonnull align 8 dereferenceable(200) %_2.i, i64 200, i1 false), !noalias !46
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i), !noalias !42
  %8 = icmp eq i64 %0, 0
  br i1 %8, label %bb25, label %bb2.i.i.i4.i.i.i6.i

bb2.i.i.i4.i.i.i6.i:                              ; preds = %bb2.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !42
  br label %bb25

bb6:                                              ; preds = %bb5
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i25), !noalias !47
  %9 = icmp ne ptr %rustc.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %9)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i25, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustc.sroa.7.0.copyload, i64 noundef %rustc.sroa.8.0.copyload)
          to label %bb2.i32 unwind label %cleanup.i28, !noalias !47

cleanup.i28:                                      ; preds = %bb6
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = icmp eq i64 %0, 0
  br i1 %11, label %bb28, label %bb2.i.i.i4.i.i.i.i30

bb2.i.i.i4.i.i.i.i30:                             ; preds = %cleanup.i28
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !47
  br label %bb28

bb2.i32:                                          ; preds = %bb6
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd, ptr noundef nonnull align 8 dereferenceable(200) %_2.i25, i64 200, i1 false), !noalias !51
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i25), !noalias !47
  %12 = icmp eq i64 %0, 0
  br i1 %12, label %bb26, label %bb2.i.i.i4.i.i.i6.i34

bb2.i.i.i4.i.i.i6.i34:                            ; preds = %bb2.i32
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !47
  br label %bb26

bb7:                                              ; preds = %bb5
  %wrapper.sroa.3.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %wrapper.sroa.3.0.copyload = load ptr, ptr %wrapper.sroa.3.0._5.sroa_idx, align 8, !nonnull !4, !noundef !4
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd1)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i40), !noalias !52
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i40, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %wrapper.sroa.3.0.copyload, i64 noundef %_41)
          to label %bb2.i47 unwind label %cleanup.i43, !noalias !52

cleanup.i43:                                      ; preds = %bb7
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = icmp eq i64 %2, 0
  br i1 %14, label %bb30, label %bb2.i.i.i4.i.i.i.i45

bb2.i.i.i4.i.i.i.i45:                             ; preds = %cleanup.i43
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %wrapper.sroa.3.0.copyload, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !52
  br label %bb30

bb2.i47:                                          ; preds = %bb7
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd1, ptr noundef nonnull align 8 dereferenceable(200) %_2.i40, i64 200, i1 false), !noalias !56
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i40), !noalias !52
  %15 = icmp eq i64 %2, 0
  br i1 %15, label %bb9, label %bb2.i.i.i4.i.i.i6.i49

bb2.i.i.i4.i.i.i6.i49:                            ; preds = %bb2.i47
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %wrapper.sroa.3.0.copyload, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !52
  br label %bb9

bb26:                                             ; preds = %bb2.i32, %bb2.i.i.i4.i.i.i6.i34
  %16 = icmp eq i64 %2, 0
  br i1 %16, label %bb25, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb26
  %17 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_5.val23 = load ptr, ptr %17, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.val23, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb25

bb25:                                             ; preds = %bb2.i.i.i4.i.i.i6.i, %bb2.i, %bb2.i.i.i4.i.i.i, %bb26, %bb13
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a887f9858119cc7413062dc002c4d9ab, i64 noundef 9)
          to label %bb15 unwind label %cleanup3

bb22:                                             ; preds = %bb21, %cleanup.i61, %cleanup3
  %.pn16.pn = phi { ptr, i32 } [ %.pn16, %bb21 ], [ %18, %cleanup3 ], [ %28, %cleanup.i61 ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd) #23
          to label %bb24 unwind label %terminate

cleanup3:                                         ; preds = %bb25, %bb15
  %18 = landingpad { ptr, i32 }
          cleanup
  br label %bb22

bb9:                                              ; preds = %bb2.i.i.i4.i.i.i6.i49, %bb2.i47
  %19 = icmp ne ptr %rustc.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %19)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustc.sroa.7.0.copyload, i64 noundef %rustc.sroa.8.0.copyload)
          to label %bb2.i56 unwind label %cleanup.i53, !noalias !57

cleanup.i53:                                      ; preds = %bb9
  %20 = landingpad { ptr, i32 }
          cleanup
  %21 = icmp eq i64 %0, 0
  br i1 %21, label %cleanup4.body, label %bb2.i.i.i4.i.i.i.i54

bb2.i.i.i4.i.i.i.i54:                             ; preds = %cleanup.i53
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !57
  br label %cleanup4.body

bb2.i56:                                          ; preds = %bb9
  %22 = icmp eq i64 %0, 0
  br i1 %22, label %bb10, label %bb2.i.i.i4.i.i.i6.i57

bb2.i.i.i4.i.i.i6.i57:                            ; preds = %bb2.i56
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !57
  br label %bb10

cleanup4:                                         ; preds = %bb11
  %23 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup4.body

cleanup4.body:                                    ; preds = %cleanup.i53, %bb2.i.i.i4.i.i.i.i54, %cleanup4
  %eh.lpad-body58 = phi { ptr, i32 } [ %23, %cleanup4 ], [ %20, %bb2.i.i.i4.i.i.i.i54 ], [ %20, %cleanup.i53 ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd1) #23
          to label %bb24 unwind label %terminate

bb10:                                             ; preds = %bb2.i.i.i4.i.i.i6.i57, %bb2.i56
  br i1 %is_clippy_driver, label %bb11, label %bb13

bb13:                                             ; preds = %bb11, %bb10
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd, ptr noundef nonnull align 8 dereferenceable(200) %cmd1, i64 200, i1 false)
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd1)
  br label %bb25

bb11:                                             ; preds = %bb10
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0a95b2846250f640f3e914bc2bbe7701, i64 noundef 7)
          to label %bb13 unwind label %cleanup4

bb15:                                             ; preds = %bb25
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_21)
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_21, ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd)
          to label %bb16 unwind label %cleanup3

bb16:                                             ; preds = %bb15
  call void @llvm.experimental.noalias.scope.decl(metadata !60)
  call void @llvm.experimental.noalias.scope.decl(metadata !63)
  %24 = load i64, ptr %_21, align 8, !range !33, !alias.scope !63, !noalias !60, !noundef !4
  %25 = icmp eq i64 %24, -9223372036854775808
  br i1 %25, label %bb2.i60, label %bb17, !prof !41

bb2.i60:                                          ; preds = %bb16
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %e.i), !noalias !65
  %26 = getelementptr inbounds nuw i8, ptr %_21, i64 8
  %27 = load ptr, ptr %26, align 8, !alias.scope !63, !noalias !60, !nonnull !4, !noundef !4
  store ptr %27, ptr %e.i, align 8, !noalias !65
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c33e5af42b9b9e21f43a4fcb9c0ba190, i64 noundef 27, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c8371de64884c804df3fe9a38c6ab64e) #26
          to label %unreachable.i unwind label %cleanup.i61, !noalias !65

cleanup.i61:                                      ; preds = %bb2.i60
  %28 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(8) %e.i) #23
          to label %bb22 unwind label %terminate.i, !noalias !65

unreachable.i:                                    ; preds = %bb2.i60
  unreachable

terminate.i:                                      ; preds = %cleanup.i61
  %29 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24, !noalias !65
  unreachable

bb17:                                             ; preds = %bb16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %output, ptr noundef nonnull readonly align 8 dereferenceable(56) %_21, i64 56, i1 false), !alias.scope !65
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_21)
  %30 = getelementptr inbounds nuw i8, ptr %output, i64 48
  %_47 = load i32, ptr %30, align 8, !noundef !4
  %.not15 = icmp eq i32 %_47, 0
  br i1 %.not15, label %bb34, label %bb35

bb35:                                             ; preds = %bb17
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_26)
  %31 = getelementptr inbounds nuw i8, ptr %output, i64 32
  %_53 = load ptr, ptr %31, align 8, !nonnull !4, !noundef !4
  %32 = getelementptr inbounds nuw i8, ptr %output, i64 40
  %_52 = load i64, ptr %32, align 8, !noundef !4
; invoke <alloc::string::String>::from_utf8_lossy
  invoke void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_26, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_53, i64 noundef %_52)
          to label %bb18 unwind label %cleanup5

bb34:                                             ; preds = %bb17
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %output, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output)
; call core::ptr::drop_in_place::<std::process::Command>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd)
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd)
  ret void

bb21:                                             ; preds = %bb2.i.i.i4.i.i.i64, %cleanup6, %cleanup6, %cleanup5
  %.pn16 = phi { ptr, i32 } [ %33, %cleanup5 ], [ %34, %cleanup6 ], [ %34, %cleanup6 ], [ %34, %bb2.i.i.i4.i.i.i64 ]
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output) #23
  br label %bb22

cleanup5:                                         ; preds = %bb35
  %33 = landingpad { ptr, i32 }
          cleanup
  br label %bb21

bb18:                                             ; preds = %bb35
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_26, ptr %args, align 8
  %_29.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsb_NtCsdJPVW0sQgAG_5alloc6borrowINtB5_3CoweENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_29.sroa.4.0..sroa_idx, align 8
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_dfa4fbb8607feef357360e24a0ecaa6f, ptr noundef nonnull %args, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bb62e3d175e305a454323ef105b47882) #26
          to label %unreachable unwind label %cleanup6

cleanup6:                                         ; preds = %bb18
  %34 = landingpad { ptr, i32 }
          cleanup
  %_26.val = load i64, ptr %_26, align 8, !range !33, !noundef !4
  switch i64 %_26.val, label %bb2.i.i.i4.i.i.i64 [
    i64 -9223372036854775808, label %bb21
    i64 0, label %bb21
  ]

bb2.i.i.i4.i.i.i64:                               ; preds = %cleanup6
  %35 = getelementptr inbounds nuw i8, ptr %_26, i64 8
  %_26.val24 = load ptr, ptr %35, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_26.val24, i64 noundef %_26.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb21

unreachable:                                      ; preds = %bb18
  unreachable

terminate:                                        ; preds = %cleanup4.body, %bb22
  %36 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb28:                                             ; preds = %bb2.i.i.i4.i.i.i.i30, %cleanup.i28
  %37 = icmp eq i64 %2, 0
  br i1 %37, label %bb24, label %bb2.i.i.i4.i.i.i65

bb2.i.i.i4.i.i.i65:                               ; preds = %bb28
  %38 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_5.val21 = load ptr, ptr %38, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.val21, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb24

bb24:                                             ; preds = %cleanup.i, %bb2.i.i.i4.i.i.i.i, %cleanup4.body, %bb2.i.i.i4.i.i.i67, %bb30, %bb2.i.i.i4.i.i.i65, %bb28, %bb22
  %.pn16.pn.pn83 = phi { ptr, i32 } [ %.pn16.pn, %bb22 ], [ %10, %bb28 ], [ %10, %bb2.i.i.i4.i.i.i65 ], [ %.pn16.pn.pn84, %bb30 ], [ %.pn16.pn.pn84, %bb2.i.i.i4.i.i.i67 ], [ %eh.lpad-body58, %cleanup4.body ], [ %6, %bb2.i.i.i4.i.i.i.i ], [ %6, %cleanup.i ]
  resume { ptr, i32 } %.pn16.pn.pn83

bb30:                                             ; preds = %cleanup.i43, %bb2.i.i.i4.i.i.i.i45, %cleanup.thread
  %.pn16.pn.pn84 = phi { ptr, i32 } [ %1, %cleanup.thread ], [ %13, %cleanup.i43 ], [ %13, %bb2.i.i.i4.i.i.i.i45 ]
  %39 = icmp eq i64 %0, 0
  br i1 %39, label %bb24, label %bb2.i.i.i4.i.i.i67

bb2.i.i.i4.i.i.i67:                               ; preds = %bb30
  %40 = icmp ne ptr %rustc.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %40)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.7.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb24
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCs2XfqmVweRya_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %result.i = alloca [24 x i8], align 8
  %args1.i433 = alloca [16 x i8], align 8
  %cfg.i435 = alloca [16 x i8], align 8
  %args1.i423 = alloca [16 x i8], align 8
  %cfg.i425 = alloca [16 x i8], align 8
  %args1.i295 = alloca [16 x i8], align 8
  %cfg.i297 = alloca [16 x i8], align 8
  %args1.i270 = alloca [16 x i8], align 8
  %cfg.i272 = alloca [16 x i8], align 8
  %args1.i = alloca [16 x i8], align 8
  %cfg.i = alloca [16 x i8], align 8
  %_2.i.i176 = alloca [24 x i8], align 8
  %_7.i = alloca [200 x i8], align 8
  %_4.i177 = alloca [56 x i8], align 8
  %_2.i.i = alloca [24 x i8], align 8
  %_6.i = alloca [200 x i8], align 8
  %_4.i150 = alloca [56 x i8], align 8
  %_26.i = alloca [72 x i8], align 8
  %_18.i = alloca [16 x i8], align 8
  %pieces.i = alloca [72 x i8], align 8
  %_11.i = alloca [24 x i8], align 8
  %_9.i = alloca [56 x i8], align 8
  %_4.i = alloca [24 x i8], align 8
  %output.i = alloca [56 x i8], align 8
  %e.i = alloca [1 x i8], align 1
  %args5 = alloca [32 x i8], align 8
  %values = alloca [24 x i8], align 8
  %name = alloca [16 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %cfg = alloca [8 x i8], align 8
  %_100 = alloca [24 x i8], align 8
  %_98 = alloca [24 x i8], align 8
  %_84 = alloca [24 x i8], align 8
  %tb = alloca [24 x i8], align 8
  %_78 = alloca [24 x i8], align 8
  %_72 = alloca [32 x i8], align 8
  %_71 = alloca [32 x i8], align 8
  %_70 = alloca [64 x i8], align 8
  %defaultbits = alloca [24 x i8], align 8
  %_54 = alloca [32 x i8], align 8
  %_47 = alloca [32 x i8], align 8
  %args = alloca [16 x i8], align 8
  %vers = alloca [4 x i8], align 4
  %_21 = alloca [32 x i8], align 8
  %_17 = alloca [32 x i8], align 8
  %_15 = alloca [32 x i8], align 8
  %_13 = alloca [32 x i8], align 8
  %_11 = alloca [32 x i8], align 8
  %_9 = alloca [32 x i8], align 8
  %_7 = alloca [32 x i8], align 8
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_742f06589122110502429e832b81e8bd, ptr noundef nonnull inttoptr (i64 65 to ptr))
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_18.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output.i)
; call build_script_build::rustc_version_cmd
  call fastcc void @_RNvCs2XfqmVweRya_18build_script_build17rustc_version_cmd(ptr noalias noundef align 8 captures(none) dereferenceable(56) %output.i, i1 noundef zeroext false)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i)
  %0 = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  %_46.i = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %output.i, i64 16
  %_45.i = load i64, ptr %1, align 8, !noundef !4
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef %_45.i)
          to label %bb2.i81 unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb12.i, %bb37.i, %bb33.i, %bb31.i, %bb11.i, %bb26.i, %bb9.i, %bb4.i, %bb21.invoke.i, %start
  %_1.val4.i = phi ptr [ %_63.i, %bb37.i ], [ %_63.i, %bb33.i ], [ %_63.i, %bb31.i ], [ %_63.i, %bb11.i ], [ %_63.i, %bb12.i ], [ %_63.i, %bb26.i ], [ %_63.i, %bb9.i ], [ %_46.i, %bb4.i ], [ %_1.val4.i706, %bb21.invoke.i ], [ %_46.i, %start ]
  %2 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !66)
  %_1.val.i = load i64, ptr %output.i, align 8, !alias.scope !66
  %3 = icmp eq i64 %_1.val.i, 0
  br i1 %3, label %bb4.i478, label %bb2.i.i.i4.i.i477

bb2.i.i.i4.i.i477:                                ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !66
  br label %bb4.i478

bb4.i478:                                         ; preds = %bb2.i.i.i4.i.i477, %cleanup.i
  %4 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i = load i64, ptr %4, align 8, !alias.scope !66
  %5 = icmp eq i64 %.val2.i, 0
  br i1 %5, label %common.resume, label %bb2.i.i.i4.i7.i

bb2.i.i.i4.i7.i:                                  ; preds = %bb4.i478
  %6 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i479 = load ptr, ptr %6, align 8, !alias.scope !66, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i479, i64 noundef %.val2.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !66
  br label %common.resume

common.resume:                                    ; preds = %bb139, %bb2.i.i.i4.i.i, %bb2.i.i.i4.i7.i, %bb4.i478
  %common.resume.op = phi { ptr, i32 } [ %2, %bb4.i478 ], [ %2, %bb2.i.i.i4.i7.i ], [ %.pn39.pn.pn.pn, %bb2.i.i.i4.i.i ], [ %.pn39.pn.pn.pn, %bb139 ]
  resume { ptr, i32 } %common.resume.op

bb2.i81:                                          ; preds = %start
  %_47.i = load i64, ptr %_4.i, align 8, !range !3, !noundef !4
  %7 = trunc nuw i64 %_47.i to i1
  br i1 %7, label %bb21.i, label %bb22.i, !prof !41

bb21.i:                                           ; preds = %bb2.i81
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i)
  br label %bb21.invoke.i

bb21.invoke.i:                                    ; preds = %bb33.i.i, %bb23.i.i, %bb16.i.i, %bb7.i.i, %bb7.i.i, %bb17.i, %bb15.i, %bb32.i, %bb24.i, %bb21.i
  %_1.val4.i706 = phi ptr [ %_46.i, %bb21.i ], [ %_63.i, %bb24.i ], [ %_63.i, %bb32.i ], [ %_63.i, %bb15.i ], [ %_63.i, %bb17.i ], [ %_63.i, %bb7.i.i ], [ %_63.i, %bb7.i.i ], [ %_63.i, %bb16.i.i ], [ %_63.i, %bb23.i.i ], [ %_63.i, %bb33.i.i ]
  %8 = phi ptr [ @alloc_29af473fbfd8114b5ab99329593ea058, %bb21.i ], [ @alloc_99ce08ad4fc15ca92c8576e66a043a53, %bb24.i ], [ @alloc_fb2e3f631cd125c14f7441a1357cf087, %bb32.i ], [ @alloc_e11c5658ee80a4da21f3f7793a294e3c, %bb15.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb17.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb7.i.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb7.i.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb16.i.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb23.i.i ], [ @alloc_11cddd32ce3a2e81eed0afcf9afbb3a4, %bb33.i.i ]
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_c33e5af42b9b9e21f43a4fcb9c0ba190, ptr noundef nonnull inttoptr (i64 55 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %8) #26
          to label %bb21.cont.i unwind label %cleanup.i

bb21.cont.i:                                      ; preds = %bb21.invoke.i
  unreachable

bb22.i:                                           ; preds = %bb2.i81
  %9 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_48.0.i = load ptr, ptr %9, align 8, !nonnull !4, !align !17, !noundef !4
  %10 = getelementptr inbounds nuw i8, ptr %_4.i, i64 16
  %_48.1.i = load i64, ptr %10, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i)
  %_4.not.i.i = icmp samesign ult i64 %_48.1.i, 6
  br i1 %_4.not.i.i, label %bb9.i, label %bb23.i

bb23.i:                                           ; preds = %bb22.i
  %11 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) @alloc_55e278c996565db65fe0fb6e7409cbbb, ptr noundef nonnull readonly align 1 dereferenceable(6) %_48.0.i, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !69
  %12 = icmp eq i32 %11, 0
  br i1 %12, label %bb4.i, label %bb9.i

bb4.i:                                            ; preds = %bb23.i
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_9.i)
; invoke build_script_build::rustc_version_cmd
  invoke fastcc void @_RNvCs2XfqmVweRya_18build_script_build17rustc_version_cmd(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_9.i, i1 noundef zeroext true)
          to label %bb5.i unwind label %cleanup.i

bb9.i:                                            ; preds = %bb6.i, %bb23.i, %bb22.i
  %_62.i = phi i64 [ %_45.i, %bb22.i ], [ %_45.i, %bb23.i ], [ %_62.pre.i, %bb6.i ]
  %_63.i = phi ptr [ %_46.i, %bb22.i ], [ %_46.i, %bb23.i ], [ %_63.pre.i, %bb6.i ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_11.i)
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_11.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_63.i, i64 noundef %_62.i)
          to label %bb10.i unwind label %cleanup.i

bb5.i:                                            ; preds = %bb4.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !76)
  %_1.val.i.i = load i64, ptr %output.i, align 8, !alias.scope !76
  %13 = icmp eq i64 %_1.val.i.i, 0
  br i1 %13, label %bb4.i15.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb5.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_46.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !76
  br label %bb4.i15.i

bb4.i15.i:                                        ; preds = %bb2.i.i.i4.i.i.i, %bb5.i
  %14 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %14, align 8, !alias.scope !76
  %15 = icmp eq i64 %.val2.i.i, 0
  br i1 %15, label %bb6.i, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i15.i
  %16 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %16, align 8, !alias.scope !76, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !76
  br label %bb6.i

bb6.i:                                            ; preds = %bb2.i.i.i4.i7.i.i, %bb4.i15.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %output.i, ptr noundef nonnull align 8 dereferenceable(56) %_9.i, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_9.i)
  %_63.pre.i = load ptr, ptr %0, align 8
  %_62.pre.i = load i64, ptr %1, align 8
  br label %bb9.i

bb10.i:                                           ; preds = %bb9.i
  %_64.i = load i64, ptr %_11.i, align 8, !range !3, !noundef !4
  %17 = trunc nuw i64 %_64.i to i1
  br i1 %17, label %bb24.i, label %bb26.i, !prof !41

bb24.i:                                           ; preds = %bb10.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_11.i)
  br label %bb21.invoke.i

bb26.i:                                           ; preds = %bb10.i
  %18 = getelementptr inbounds nuw i8, ptr %_11.i, i64 8
  %_65.0.i = load ptr, ptr %18, align 8, !nonnull !4, !align !17, !noundef !4
  %19 = getelementptr inbounds nuw i8, ptr %_11.i, i64 16
  %_65.1.i = load i64, ptr %19, align 8, !noundef !4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_11.i)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %pieces.i)
  store i64 0, ptr %pieces.i, align 8
  %_75.sroa.4.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 8
  store i64 %_65.1.i, ptr %_75.sroa.4.0.pieces.sroa_idx.i, align 8
  %_75.sroa.5.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 16
  store ptr %_65.0.i, ptr %_75.sroa.5.0.pieces.sroa_idx.i, align 8
  %_75.sroa.5.sroa.4.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 24
  store i64 %_65.1.i, ptr %_75.sroa.5.sroa.4.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_75.sroa.5.sroa.5.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 32
  store i64 0, ptr %_75.sroa.5.sroa.5.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_75.sroa.5.sroa.6.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 40
  store i64 %_65.1.i, ptr %_75.sroa.5.sroa.6.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_75.sroa.5.sroa.7.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 48
  store <2 x i32> splat (i32 46), ptr %_75.sroa.5.sroa.7.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_75.sroa.5.sroa.9.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 56
  store i8 1, ptr %_75.sroa.5.sroa.9.0._75.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_75.sroa.6.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 64
  store i8 1, ptr %_75.sroa.6.0.pieces.sroa_idx.i, align 8
  %_75.sroa.7.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 65
  store i8 0, ptr %_75.sroa.7.0.pieces.sroa_idx.i, align 1
; invoke <core::str::iter::SplitInternal<char>>::next
  %20 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb27.i unwind label %cleanup.i

bb27.i:                                           ; preds = %bb26.i
  %21 = extractvalue { ptr, i64 } %20, 0
  %22 = extractvalue { ptr, i64 } %20, 1
  store ptr %21, ptr %_18.i, align 8
  %23 = getelementptr inbounds nuw i8, ptr %_18.i, i64 8
  store i64 %22, ptr %23, align 8
  %.not.i = icmp ne ptr %21, null
  %_3.not.i.i = icmp eq i64 %22, 7
  %or.cond.i = select i1 %.not.i, i1 %_3.not.i.i, i1 false
  br i1 %or.cond.i, label %bb30.i, label %bb12.i, !prof !79

bb12.i:                                           ; preds = %bb30.i, %bb27.i
; invoke core::panicking::assert_failed::<core::option::Option<&str>, core::option::Option<&str>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedINtNtB4_6option6OptionReEBM_ECs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %_18.i) #26
          to label %unreachable.i unwind label %cleanup.i

bb30.i:                                           ; preds = %bb27.i
  %24 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %21, ptr noundef nonnull dereferenceable(7) @alloc_ca36d7e792bb4bbd1a68749f90007ce8, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !80
  %25 = icmp eq i32 %24, 0
  br i1 %25, label %bb11.i, label %bb12.i, !prof !84

bb11.i:                                           ; preds = %bb30.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %26 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb31.i unwind label %cleanup.i

unreachable.i:                                    ; preds = %bb12.i
  unreachable

bb31.i:                                           ; preds = %bb11.i
  %27 = extractvalue { ptr, i64 } %26, 0
  %28 = extractvalue { ptr, i64 } %26, 1
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_26.i)
; invoke <core::str::iter::SplitInternal<char>>::next
  %29 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb32.i unwind label %cleanup.i

bb32.i:                                           ; preds = %bb31.i
  %30 = extractvalue { ptr, i64 } %29, 0
  %.not11.i = icmp eq ptr %30, null
  br i1 %.not11.i, label %bb21.invoke.i, label %bb33.i, !prof !41

bb33.i:                                           ; preds = %bb32.i
  %31 = extractvalue { ptr, i64 } %29, 1
  store i64 0, ptr %_26.i, align 8
  %_105.sroa.4.0._26.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 8
  store i64 %31, ptr %_105.sroa.4.0._26.sroa_idx.i, align 8
  %_105.sroa.5.0._26.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 16
  store ptr %30, ptr %_105.sroa.5.0._26.sroa_idx.i, align 8
  %_105.sroa.5.sroa.4.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 24
  store i64 %31, ptr %_105.sroa.5.sroa.4.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i, align 8
  %_105.sroa.5.sroa.5.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 32
  store i64 0, ptr %_105.sroa.5.sroa.5.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i, align 8
  %_105.sroa.5.sroa.6.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 40
  store i64 %31, ptr %_105.sroa.5.sroa.6.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i, align 8
  %_105.sroa.5.sroa.7.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 48
  store <2 x i32> splat (i32 45), ptr %_105.sroa.5.sroa.7.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i, align 8
  %_105.sroa.5.sroa.9.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 56
  store i8 1, ptr %_105.sroa.5.sroa.9.0._105.sroa.5.0._26.sroa_idx.sroa_idx.i, align 8
  %_105.sroa.6.0._26.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 64
  store i8 1, ptr %_105.sroa.6.0._26.sroa_idx.i, align 8
  %_105.sroa.7.0._26.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 65
  store i8 0, ptr %_105.sroa.7.0._26.sroa_idx.i, align 1
; invoke <core::str::iter::SplitInternal<char>>::next
  %32 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(72) %_26.i) #28
          to label %bb35.i unwind label %cleanup.i

bb35.i:                                           ; preds = %bb33.i
  %33 = extractvalue { ptr, i64 } %32, 0
  %.not.i.not.i = icmp eq ptr %33, null
  br i1 %.not.i.not.i, label %bb15.i, label %bb37.i

bb37.i:                                           ; preds = %bb35.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %34 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %_26.i)
          to label %bb15.i unwind label %cleanup.i

bb15.i:                                           ; preds = %bb37.i, %bb35.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_26.i)
  %.not13.i = icmp eq ptr %27, null
  br i1 %.not13.i, label %bb21.invoke.i, label %bb17.i, !prof !41

bb17.i:                                           ; preds = %bb15.i
  switch i64 %28, label %bb9thread-pre-split.i.i [
    i64 0, label %bb21.invoke.i
    i64 1, label %bb7.i.i
  ]

bb7.i.i:                                          ; preds = %bb17.i
  %35 = load i8, ptr %27, align 1, !alias.scope !85, !noundef !4
  switch i8 %35, label %bb9.i.i [
    i8 43, label %bb21.invoke.i
    i8 45, label %bb21.invoke.i
  ]

bb9thread-pre-split.i.i:                          ; preds = %bb17.i
  %.pr.i.i = load i8, ptr %27, align 1, !alias.scope !85
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb9thread-pre-split.i.i, %bb7.i.i
  %36 = phi i8 [ %.pr.i.i, %bb9thread-pre-split.i.i ], [ %35, %bb7.i.i ]
  %cond.i.i = icmp eq i8 %36, 43
  %rest.1.i.i = sext i1 %cond.i.i to i64
  %src.sroa.15.0.i.i = add nsw i64 %28, %rest.1.i.i
  %src.sroa.0.0.idx.i.i = zext i1 %cond.i.i to i64
  %src.sroa.0.0.i.i = getelementptr inbounds nuw i8, ptr %27, i64 %src.sroa.0.0.idx.i.i
  %_10.i.i = icmp samesign ult i64 %src.sroa.15.0.i.i, 9
  br i1 %_10.i.i, label %bb15.preheader.i.i, label %bb22.i.i

bb15.preheader.i.i:                               ; preds = %bb9.i.i
  %_13.not56.i.i = icmp eq i64 %src.sroa.15.0.i.i, 0
  br i1 %_13.not56.i.i, label %bb41.i, label %bb16.i.i

bb22.i.i:                                         ; preds = %bb9.i.i, %bb33.i.i
  %result.sroa.0.0.i.i = phi i32 [ %_60.0.i.i, %bb33.i.i ], [ 0, %bb9.i.i ]
  %src.sroa.15.1.i.i = phi i64 [ %rest.12.i.i, %bb33.i.i ], [ %src.sroa.15.0.i.i, %bb9.i.i ]
  %src.sroa.0.1.i.i = phi ptr [ %rest.01.i.i, %bb33.i.i ], [ %src.sroa.0.0.i.i, %bb9.i.i ]
  %_28.not.i.not.i = icmp eq i64 %src.sroa.15.1.i.i, 0
  br i1 %_28.not.i.not.i, label %bb41.i, label %bb23.i.i

bb23.i.i:                                         ; preds = %bb22.i.i
  %37 = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i.i, i32 10)
  %_57.1.i.i = extractvalue { i32, i1 } %37, 1
  br i1 %_57.1.i.i, label %bb21.invoke.i, label %bb33.i.i, !prof !41

bb33.i.i:                                         ; preds = %bb23.i.i
  %_57.0.i.i = extractvalue { i32, i1 } %37, 0
  %rest.12.i.i = add nsw i64 %src.sroa.15.1.i.i, -1
  %rest.01.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i, i64 1
  %38 = load i8, ptr %src.sroa.0.1.i.i, align 1, !alias.scope !85, !noundef !4
  %39 = zext i8 %38 to i32
  %40 = add nsw i32 %39, -48
  %_14.i.i.i = icmp ugt i32 %40, 9
  %_60.0.i.i = add i32 %40, %_57.0.i.i
  %_60.1.i.i = icmp ult i32 %_60.0.i.i, %_57.0.i.i
  %or.cond36.i = select i1 %_14.i.i.i, i1 true, i1 %_60.1.i.i
  br i1 %or.cond36.i, label %bb21.invoke.i, label %bb22.i.i, !prof !88

bb16.i.i:                                         ; preds = %bb15.preheader.i.i, %bb20.i.i
  %src.sroa.0.259.i.i = phi ptr [ %rest.04.i.i, %bb20.i.i ], [ %src.sroa.0.0.i.i, %bb15.preheader.i.i ]
  %src.sroa.15.258.i.i = phi i64 [ %rest.15.i.i, %bb20.i.i ], [ %src.sroa.15.0.i.i, %bb15.preheader.i.i ]
  %result.sroa.0.257.i.i = phi i32 [ %43, %bb20.i.i ], [ 0, %bb15.preheader.i.i ]
  %_19.i.i = load i8, ptr %src.sroa.0.259.i.i, align 1, !alias.scope !85, !noundef !4
  %_18.i.i = zext i8 %_19.i.i to i32
  %41 = add nsw i32 %_18.i.i, -48
  %_14.i47.i.i = icmp ugt i32 %41, 9
  br i1 %_14.i47.i.i, label %bb21.invoke.i, label %bb20.i.i

bb20.i.i:                                         ; preds = %bb16.i.i
  %42 = mul i32 %result.sroa.0.257.i.i, 10
  %rest.15.i.i = add nsw i64 %src.sroa.15.258.i.i, -1
  %rest.04.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i, i64 1
  %43 = add i32 %41, %42
  %_13.not.i.i = icmp eq i64 %rest.15.i.i, 0
  br i1 %_13.not.i.i, label %bb41.i, label %bb16.i.i

bb41.i:                                           ; preds = %bb22.i.i, %bb20.i.i, %bb15.preheader.i.i
  %_0.sroa.8.0.insert.insert.i.i = phi i32 [ 0, %bb15.preheader.i.i ], [ %43, %bb20.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !89)
  %_1.val.i21.i = load i64, ptr %output.i, align 8, !alias.scope !89
  %44 = icmp eq i64 %_1.val.i21.i, 0
  br i1 %44, label %bb4.i24.i, label %bb2.i.i.i4.i.i22.i

bb2.i.i.i4.i.i22.i:                               ; preds = %bb41.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_63.i, i64 noundef %_1.val.i21.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !89
  br label %bb4.i24.i

bb4.i24.i:                                        ; preds = %bb2.i.i.i4.i.i22.i, %bb41.i
  %45 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i25.i = load i64, ptr %45, align 8, !alias.scope !89
  %46 = icmp eq i64 %.val2.i25.i, 0
  br i1 %46, label %_RNvCs2XfqmVweRya_18build_script_build19rustc_minor_nightly.exit, label %bb2.i.i.i4.i7.i26.i

bb2.i.i.i4.i7.i26.i:                              ; preds = %bb4.i24.i
  %47 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i27.i = load ptr, ptr %47, align 8, !alias.scope !89, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i27.i, i64 noundef %.val2.i25.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !89
  br label %_RNvCs2XfqmVweRya_18build_script_build19rustc_minor_nightly.exit

_RNvCs2XfqmVweRya_18build_script_build19rustc_minor_nightly.exit: ; preds = %bb4.i24.i, %bb2.i.i.i4.i7.i26.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_18.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_7)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_509e3f14595a72dfc2af0a28f5824017, i64 noundef 30)
  %_201 = load i64, ptr %_7, align 8, !range !3, !noundef !4
  %rustc_dep_of_std = icmp eq i64 %_201, 0
  tail call void @llvm.experimental.noalias.scope.decl(metadata !92)
  %48 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  %.val.i = load i64, ptr %48, align 8, !alias.scope !92
  br i1 %rustc_dep_of_std, label %bb2.i82, label %bb3.i

bb2.i82:                                          ; preds = %_RNvCs2XfqmVweRya_18build_script_build19rustc_minor_nightly.exit
  %49 = icmp eq i64 %.val.i, 0
  br i1 %49, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit, label %bb1.sink.split.i

bb3.i:                                            ; preds = %_RNvCs2XfqmVweRya_18build_script_build19rustc_minor_nightly.exit
  switch i64 %.val.i, label %bb1.sink.split.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit
  ]

bb1.sink.split.i:                                 ; preds = %bb3.i, %bb2.i82
  %50 = getelementptr inbounds nuw i8, ptr %_7, i64 16
  %.val3.i = load ptr, ptr %50, align 8, !alias.scope !92, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i, i64 noundef %.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !92
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit: ; preds = %bb2.i82, %bb3.i, %bb3.i, %bb1.sink.split.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_7)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f73607afcba5e721c2712249402644b6, i64 noundef 7)
  %_202 = load i64, ptr %_9, align 8, !range !3, !noundef !4
  %libc_ci = icmp eq i64 %_202, 0
  tail call void @llvm.experimental.noalias.scope.decl(metadata !95)
  %51 = getelementptr inbounds nuw i8, ptr %_9, i64 8
  %.val.i84 = load i64, ptr %51, align 8, !alias.scope !95
  br i1 %libc_ci, label %bb2.i88, label %bb3.i85

bb2.i88:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit
  %52 = icmp eq i64 %.val.i84, 0
  br i1 %52, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89, label %bb1.sink.split.i86

bb3.i85:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit
  switch i64 %.val.i84, label %bb1.sink.split.i86 [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89
  ]

bb1.sink.split.i86:                               ; preds = %bb3.i85, %bb2.i88
  %53 = getelementptr inbounds nuw i8, ptr %_9, i64 16
  %.val3.i87 = load ptr, ptr %53, align 8, !alias.scope !95, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i87, i64 noundef %.val.i84, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !95
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89: ; preds = %bb2.i88, %bb3.i85, %bb3.i85, %bb1.sink.split.i86
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_11, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1e1fc66c1706c6c7501acca2ae8010f4, i64 noundef 20)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !98)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !101)
  %_2.i90 = load i64, ptr %_11, align 8, !range !3, !alias.scope !101, !noalias !98, !noundef !4
  %54 = trunc nuw i64 %_2.i90 to i1
  br i1 %54, label %bb3.i.i, label %bb7.i

bb7.i:                                            ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89
  %55 = getelementptr inbounds nuw i8, ptr %_11, i64 8
  %target_env.sroa.0.0.copyload = load i64, ptr %55, align 8, !alias.scope !103
  %target_env.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_11, i64 16
  %target_env.sroa.6.0.copyload = load ptr, ptr %target_env.sroa.6.0..sroa_idx, align 8, !alias.scope !103
  %target_env.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_11, i64 24
  %target_env.sroa.11.0.copyload = load i64, ptr %target_env.sroa.11.0..sroa_idx, align 8, !alias.scope !103
  br label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit

bb3.i.i:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build.exit89
  tail call void @llvm.experimental.noalias.scope.decl(metadata !104)
  %56 = getelementptr inbounds nuw i8, ptr %_11, i64 8
  %.val.i.i = load i64, ptr %56, align 8, !alias.scope !107, !noalias !98
  switch i64 %.val.i.i, label %bb1.sink.split.i.i [
    i64 -9223372036854775808, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit
    i64 0, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit
  ]

bb1.sink.split.i.i:                               ; preds = %bb3.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_11, i64 16
  %.val3.i.i92 = load ptr, ptr %57, align 8, !alias.scope !107, !noalias !98, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i92, i64 noundef %.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !108
  br label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit: ; preds = %bb7.i, %bb3.i.i, %bb3.i.i, %bb1.sink.split.i.i
  %target_env.sroa.11.0 = phi i64 [ 0, %bb1.sink.split.i.i ], [ 0, %bb3.i.i ], [ 0, %bb3.i.i ], [ %target_env.sroa.11.0.copyload, %bb7.i ]
  %target_env.sroa.6.0 = phi ptr [ inttoptr (i64 1 to ptr), %bb1.sink.split.i.i ], [ inttoptr (i64 1 to ptr), %bb3.i.i ], [ inttoptr (i64 1 to ptr), %bb3.i.i ], [ %target_env.sroa.6.0.copyload, %bb7.i ]
  %target_env.sroa.0.0 = phi i64 [ 0, %bb1.sink.split.i.i ], [ 0, %bb3.i.i ], [ 0, %bb3.i.i ], [ %target_env.sroa.0.0.copyload, %bb7.i ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_aa4687de82972c6f88dd4ebd068e3b63, i64 noundef 19)
          to label %bb9 unwind label %cleanup

bb139:                                            ; preds = %bb2.i.i.i4.i.i106, %bb138, %cleanup
  %.pn39.pn.pn.pn = phi { ptr, i32 } [ %60, %cleanup ], [ %.pn39.pn.pn, %bb138 ], [ %.pn39.pn.pn, %bb2.i.i.i4.i.i106 ]
  %58 = icmp eq i64 %target_env.sroa.0.0, 0
  br i1 %58, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb139
  %59 = icmp ne ptr %target_env.sroa.6.0, null
  call void @llvm.assume(i1 %59)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_env.sroa.6.0, i64 noundef %target_env.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %common.resume

cleanup:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit
  %60 = landingpad { ptr, i32 }
          cleanup
  br label %bb139

bb9:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build.exit
  tail call void @llvm.experimental.noalias.scope.decl(metadata !109)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !112)
  %_2.i94 = load i64, ptr %_13, align 8, !range !3, !alias.scope !112, !noalias !109, !noundef !4
  %61 = trunc nuw i64 %_2.i94 to i1
  br i1 %61, label %bb3.i.i97, label %bb7.i95

bb7.i95:                                          ; preds = %bb9
  %62 = getelementptr inbounds nuw i8, ptr %_13, i64 8
  %target_os.sroa.0.0.copyload = load i64, ptr %62, align 8, !alias.scope !114
  %target_os.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 16
  %target_os.sroa.6.0.copyload = load ptr, ptr %target_os.sroa.6.0..sroa_idx, align 8, !alias.scope !114
  %target_os.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 24
  %target_os.sroa.10.0.copyload = load i64, ptr %target_os.sroa.10.0..sroa_idx, align 8, !alias.scope !114
  %63 = icmp eq i64 %target_os.sroa.10.0.copyload, 5
  br label %bb10

bb3.i.i97:                                        ; preds = %bb9
  tail call void @llvm.experimental.noalias.scope.decl(metadata !115)
  %64 = getelementptr inbounds nuw i8, ptr %_13, i64 8
  %.val.i.i100 = load i64, ptr %64, align 8, !alias.scope !118, !noalias !109
  switch i64 %.val.i.i100, label %bb1.sink.split.i.i101 [
    i64 -9223372036854775808, label %bb10
    i64 0, label %bb10
  ]

bb1.sink.split.i.i101:                            ; preds = %bb3.i.i97
  %65 = getelementptr inbounds nuw i8, ptr %_13, i64 16
  %.val3.i.i102 = load ptr, ptr %65, align 8, !alias.scope !118, !noalias !109, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i102, i64 noundef %.val.i.i100, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !119
  br label %bb10

bb10:                                             ; preds = %bb1.sink.split.i.i101, %bb3.i.i97, %bb3.i.i97, %bb7.i95
  %target_os.sroa.10.0 = phi i1 [ false, %bb1.sink.split.i.i101 ], [ false, %bb3.i.i97 ], [ false, %bb3.i.i97 ], [ %63, %bb7.i95 ]
  %target_os.sroa.6.0 = phi ptr [ inttoptr (i64 1 to ptr), %bb1.sink.split.i.i101 ], [ inttoptr (i64 1 to ptr), %bb3.i.i97 ], [ inttoptr (i64 1 to ptr), %bb3.i.i97 ], [ %target_os.sroa.6.0.copyload, %bb7.i95 ]
  %target_os.sroa.0.0 = phi i64 [ 0, %bb1.sink.split.i.i101 ], [ 0, %bb3.i.i97 ], [ 0, %bb3.i.i97 ], [ %target_os.sroa.0.0.copyload, %bb7.i95 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_15)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_15, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6508c675143a2a16e0690055cd395724, i64 noundef 30)
          to label %bb11 unwind label %cleanup7

bb138:                                            ; preds = %bb2.i.i.i4.i.i120, %bb137, %cleanup7
  %.pn39.pn.pn = phi { ptr, i32 } [ %68, %cleanup7 ], [ %.pn39.pn, %bb137 ], [ %.pn39.pn, %bb2.i.i.i4.i.i120 ]
  %66 = icmp eq i64 %target_os.sroa.0.0, 0
  br i1 %66, label %bb139, label %bb2.i.i.i4.i.i106

bb2.i.i.i4.i.i106:                                ; preds = %bb138
  %67 = icmp ne ptr %target_os.sroa.6.0, null
  call void @llvm.assume(i1 %67)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_os.sroa.6.0, i64 noundef %target_os.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb139

cleanup7:                                         ; preds = %bb10
  %68 = landingpad { ptr, i32 }
          cleanup
  br label %bb138

bb11:                                             ; preds = %bb10
  tail call void @llvm.experimental.noalias.scope.decl(metadata !120)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !123)
  %_2.i108 = load i64, ptr %_15, align 8, !range !3, !alias.scope !123, !noalias !120, !noundef !4
  %69 = trunc nuw i64 %_2.i108 to i1
  br i1 %69, label %bb3.i.i111, label %bb7.i109

bb7.i109:                                         ; preds = %bb11
  %70 = getelementptr inbounds nuw i8, ptr %_15, i64 8
  %target_ptr_width.sroa.0.0.copyload = load i64, ptr %70, align 8, !alias.scope !125
  %target_ptr_width.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_15, i64 16
  %target_ptr_width.sroa.6.0.copyload = load ptr, ptr %target_ptr_width.sroa.6.0..sroa_idx, align 8, !alias.scope !125
  %target_ptr_width.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_15, i64 24
  %target_ptr_width.sroa.10.0.copyload = load i64, ptr %target_ptr_width.sroa.10.0..sroa_idx, align 8, !alias.scope !125
  %71 = icmp eq i64 %target_ptr_width.sroa.10.0.copyload, 2
  br label %bb12

bb3.i.i111:                                       ; preds = %bb11
  tail call void @llvm.experimental.noalias.scope.decl(metadata !126)
  %72 = getelementptr inbounds nuw i8, ptr %_15, i64 8
  %.val.i.i114 = load i64, ptr %72, align 8, !alias.scope !129, !noalias !120
  switch i64 %.val.i.i114, label %bb1.sink.split.i.i115 [
    i64 -9223372036854775808, label %bb12
    i64 0, label %bb12
  ]

bb1.sink.split.i.i115:                            ; preds = %bb3.i.i111
  %73 = getelementptr inbounds nuw i8, ptr %_15, i64 16
  %.val3.i.i116 = load ptr, ptr %73, align 8, !alias.scope !129, !noalias !120, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i116, i64 noundef %.val.i.i114, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !130
  br label %bb12

bb12:                                             ; preds = %bb1.sink.split.i.i115, %bb3.i.i111, %bb3.i.i111, %bb7.i109
  %target_ptr_width.sroa.10.0 = phi i1 [ false, %bb1.sink.split.i.i115 ], [ false, %bb3.i.i111 ], [ false, %bb3.i.i111 ], [ %71, %bb7.i109 ]
  %target_ptr_width.sroa.6.0 = phi ptr [ inttoptr (i64 1 to ptr), %bb1.sink.split.i.i115 ], [ inttoptr (i64 1 to ptr), %bb3.i.i111 ], [ inttoptr (i64 1 to ptr), %bb3.i.i111 ], [ %target_ptr_width.sroa.6.0.copyload, %bb7.i109 ]
  %target_ptr_width.sroa.0.0 = phi i64 [ 0, %bb1.sink.split.i.i115 ], [ 0, %bb3.i.i111 ], [ 0, %bb3.i.i111 ], [ %target_ptr_width.sroa.0.0.copyload, %bb7.i109 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_15)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_17)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_17, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0d3bcf6fb685f000bc18304ea76cbac4, i64 noundef 21)
          to label %bb13 unwind label %cleanup8

bb137:                                            ; preds = %bb2.i.i.i4.i.i132, %bb136, %cleanup8
  %.pn39.pn = phi { ptr, i32 } [ %76, %cleanup8 ], [ %.pn39, %bb136 ], [ %.pn39, %bb2.i.i.i4.i.i132 ]
  %74 = icmp eq i64 %target_ptr_width.sroa.0.0, 0
  br i1 %74, label %bb138, label %bb2.i.i.i4.i.i120

bb2.i.i.i4.i.i120:                                ; preds = %bb137
  %75 = icmp ne ptr %target_ptr_width.sroa.6.0, null
  call void @llvm.assume(i1 %75)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_ptr_width.sroa.6.0, i64 noundef %target_ptr_width.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb138

cleanup8:                                         ; preds = %bb12
  %76 = landingpad { ptr, i32 }
          cleanup
  br label %bb137

bb13:                                             ; preds = %bb12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !131)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !134)
  %_2.i122 = load i64, ptr %_17, align 8, !range !3, !alias.scope !134, !noalias !131, !noundef !4
  %77 = trunc nuw i64 %_2.i122 to i1
  br i1 %77, label %bb3.i.i125, label %bb7.i123

bb7.i123:                                         ; preds = %bb13
  %78 = getelementptr inbounds nuw i8, ptr %_17, i64 8
  %target_arch.sroa.0.0.copyload = load i64, ptr %78, align 8, !alias.scope !136
  %target_arch.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_17, i64 16
  %target_arch.sroa.6.0.copyload = load ptr, ptr %target_arch.sroa.6.0..sroa_idx, align 8, !alias.scope !136
  %target_arch.sroa.12.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_17, i64 24
  %target_arch.sroa.12.0.copyload = load i64, ptr %target_arch.sroa.12.0..sroa_idx, align 8, !alias.scope !136
  br label %bb14

bb3.i.i125:                                       ; preds = %bb13
  tail call void @llvm.experimental.noalias.scope.decl(metadata !137)
  %79 = getelementptr inbounds nuw i8, ptr %_17, i64 8
  %.val.i.i128 = load i64, ptr %79, align 8, !alias.scope !140, !noalias !131
  switch i64 %.val.i.i128, label %bb1.sink.split.i.i129 [
    i64 -9223372036854775808, label %bb14
    i64 0, label %bb14
  ]

bb1.sink.split.i.i129:                            ; preds = %bb3.i.i125
  %80 = getelementptr inbounds nuw i8, ptr %_17, i64 16
  %.val3.i.i130 = load ptr, ptr %80, align 8, !alias.scope !140, !noalias !131, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i130, i64 noundef %.val.i.i128, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !141
  br label %bb14

bb14:                                             ; preds = %bb1.sink.split.i.i129, %bb3.i.i125, %bb3.i.i125, %bb7.i123
  %target_arch.sroa.12.0 = phi i64 [ 0, %bb1.sink.split.i.i129 ], [ 0, %bb3.i.i125 ], [ 0, %bb3.i.i125 ], [ %target_arch.sroa.12.0.copyload, %bb7.i123 ]
  %target_arch.sroa.6.0 = phi ptr [ inttoptr (i64 1 to ptr), %bb1.sink.split.i.i129 ], [ inttoptr (i64 1 to ptr), %bb3.i.i125 ], [ inttoptr (i64 1 to ptr), %bb3.i.i125 ], [ %target_arch.sroa.6.0.copyload, %bb7.i123 ]
  %target_arch.sroa.0.0 = phi i64 [ 0, %bb1.sink.split.i.i129 ], [ 0, %bb3.i.i125 ], [ 0, %bb3.i.i125 ], [ %target_arch.sroa.0.0.copyload, %bb7.i123 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_17)
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_b74b27f2b9f751849fcbc82dbd3a9d08, ptr noundef nonnull inttoptr (i64 125 to ptr))
          to label %bb15 unwind label %cleanup9.loopexit.split-lp

bb136:                                            ; preds = %cleanup10, %bb2.i.i.i4.i.i146, %cleanup9.loopexit, %cleanup9.loopexit.split-lp, %bb158, %bb2.i.i.i4.i.i463, %cleanup23, %cleanup.i455, %bb2.i.i.i4.i.i456, %cleanup.body.i179, %bb17.i185, %bb2.i.i.i4.i361.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336
  %.pn39 = phi { ptr, i32 } [ %.pn35.pn.pn, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336 ], [ %.pn502, %bb158 ], [ %eh.lpad-body366.i, %cleanup.body.i179 ], [ %.pn435.i, %bb17.i185 ], [ %.pn435.i, %bb2.i.i.i4.i361.i ], [ %lpad.phi, %bb2.i.i.i4.i.i456 ], [ %lpad.phi, %cleanup.i455 ], [ %439, %cleanup23 ], [ %439, %bb2.i.i.i4.i.i463 ], [ %lpad.loopexit613, %cleanup9.loopexit ], [ %lpad.loopexit.split-lp614, %cleanup9.loopexit.split-lp ], [ %121, %bb2.i.i.i4.i.i146 ], [ %121, %cleanup10 ]
  %81 = icmp eq i64 %target_arch.sroa.0.0, 0
  br i1 %81, label %bb137, label %bb2.i.i.i4.i.i132

bb2.i.i.i4.i.i132:                                ; preds = %bb136
  %82 = icmp ne ptr %target_arch.sroa.6.0, null
  call void @llvm.assume(i1 %82)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_arch.sroa.6.0, i64 noundef %target_arch.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb137

cleanup9.loopexit:                                ; preds = %bb109.12, %bb109.11, %bb109.10, %bb109.9, %bb109.8, %bb109.7, %bb109.6, %bb109.5, %bb109.4, %bb109.3, %bb109.2, %bb109.1, %bb109, %bb107.preheader
  %lpad.loopexit613 = landingpad { ptr, i32 }
          cleanup
  br label %bb136

cleanup9.loopexit.split-lp:                       ; preds = %bb172.invoke, %bb30.invoke, %bb14, %bb41, %bb48, %bb51, %bb52, %bb31, %bb15, %bb33, %bb25.i241, %bb39, %bb36, %bb45, %bb46, %bb50, %bb102, %bb104, %bb52.i450
  %lpad.loopexit.split-lp614 = landingpad { ptr, i32 }
          cleanup
  br label %bb136

bb15:                                             ; preds = %bb14
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_21)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_21, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_aaa658f8720b91022cfd120b3be84301, i64 noundef 34)
          to label %bb16 unwind label %cleanup9.loopexit.split-lp

bb16:                                             ; preds = %bb15
  %_22 = load i64, ptr %_21, align 8, !range !3, !noundef !4
  %83 = trunc nuw i64 %_22 to i1
  br i1 %83, label %bb20, label %bb17

bb20:                                             ; preds = %bb16
  br i1 %libc_ci, label %bb22, label %bb3.i171

bb17:                                             ; preds = %bb16
  %84 = getelementptr inbounds nuw i8, ptr %_21, i64 8
  %version.sroa.0.0.copyload = load i64, ptr %84, align 8
  %version.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_21, i64 16
  %version.sroa.5.0.copyload = load ptr, ptr %version.sroa.5.0..sroa_idx, align 8, !nonnull !4, !noundef !4
  %version.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_21, i64 24
  %version.sroa.8.0.copyload = load i64, ptr %version.sroa.8.0..sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %vers)
  switch i64 %version.sroa.8.0.copyload, label %bb9thread-pre-split.i [
    i64 0, label %bb2.i
    i64 1, label %bb7.i136
  ]

bb7.i136:                                         ; preds = %bb17
  %85 = load i8, ptr %version.sroa.5.0.copyload, align 1, !alias.scope !142, !noundef !4
  switch i8 %85, label %bb9.i138 [
    i8 43, label %bb2.i
    i8 45, label %bb2.i
  ]

bb9thread-pre-split.i:                            ; preds = %bb17
  %.pr.i = load i8, ptr %version.sroa.5.0.copyload, align 1, !alias.scope !142
  br label %bb9.i138

bb9.i138:                                         ; preds = %bb9thread-pre-split.i, %bb7.i136
  %86 = phi i8 [ %.pr.i, %bb9thread-pre-split.i ], [ %85, %bb7.i136 ]
  switch i8 %86, label %bb48.i [
    i8 43, label %bb11.i143
    i8 45, label %bb10.i139
  ]

bb11.i143:                                        ; preds = %bb9.i138
  %rest.0.i = getelementptr inbounds nuw i8, ptr %version.sroa.5.0.copyload, i64 1
  %rest.1.i = add nsw i64 %version.sroa.8.0.copyload, -1
  br label %bb48.i

bb10.i139:                                        ; preds = %bb9.i138
  %rest.05.i = getelementptr inbounds nuw i8, ptr %version.sroa.5.0.copyload, i64 1
  %rest.16.i = add nsw i64 %version.sroa.8.0.copyload, -1
  %87 = icmp samesign ult i64 %version.sroa.8.0.copyload, 9
  br i1 %87, label %bb19.preheader.i, label %bb28.i

bb19.preheader.i:                                 ; preds = %bb10.i139
  %_27.not123.i = icmp eq i64 %rest.16.i, 0
  br i1 %_27.not123.i, label %bb18, label %bb20.i

bb28.i:                                           ; preds = %bb10.i139, %bb46.i
  %src.sroa.0.1122.i = phi ptr [ %rest.07.i, %bb46.i ], [ %rest.05.i, %bb10.i139 ]
  %src.sroa.26.1121.i = phi i64 [ %rest.18.i, %bb46.i ], [ %rest.16.i, %bb10.i139 ]
  %result.sroa.0.0120.i = phi i32 [ %_85.0.i, %bb46.i ], [ 0, %bb10.i139 ]
  %rest.07.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1122.i, i64 1
  %rest.18.i = add nsw i64 %src.sroa.26.1121.i, -1
  %88 = tail call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %result.sroa.0.0120.i, i32 10)
  %_81.0.i = extractvalue { i32, i1 } %88, 0
  %_81.1.i = extractvalue { i32, i1 } %88, 1
  br i1 %_81.1.i, label %bb2.i.sink.split, label %bb43.i, !prof !41

bb43.i:                                           ; preds = %bb28.i
  %89 = load i8, ptr %src.sroa.0.1122.i, align 1, !alias.scope !142, !noundef !4
  %90 = zext i8 %89 to i32
  %91 = add nsw i32 %90, -48
  %_14.i.i = icmp ult i32 %91, 10
  br i1 %_14.i.i, label %bb63.i, label %bb2.i

bb63.i:                                           ; preds = %bb43.i
  %92 = tail call { i32, i1 } @llvm.ssub.with.overflow.i32(i32 %_81.0.i, i32 %91)
  %_85.1.i = extractvalue { i32, i1 } %92, 1
  br i1 %_85.1.i, label %bb2.i, label %bb46.i, !prof !41

bb46.i:                                           ; preds = %bb63.i
  %_85.0.i = extractvalue { i32, i1 } %92, 0
  %_54.not.i = icmp eq i64 %rest.18.i, 0
  br i1 %_54.not.i, label %bb18, label %bb28.i

bb20.i:                                           ; preds = %bb19.preheader.i, %bb23.i142
  %src.sroa.0.2126.i = phi ptr [ %rest.010.i, %bb23.i142 ], [ %rest.05.i, %bb19.preheader.i ]
  %src.sroa.26.2125.i = phi i64 [ %rest.111.i, %bb23.i142 ], [ %rest.16.i, %bb19.preheader.i ]
  %result.sroa.0.2124.i = phi i32 [ %95, %bb23.i142 ], [ 0, %bb19.preheader.i ]
  %_34.i = load i8, ptr %src.sroa.0.2126.i, align 1, !alias.scope !142, !noundef !4
  %_33.i = zext i8 %_34.i to i32
  %93 = add nsw i32 %_33.i, -48
  %_14.i94.i = icmp ult i32 %93, 10
  br i1 %_14.i94.i, label %bb23.i142, label %bb2.i

bb23.i142:                                        ; preds = %bb20.i
  %94 = mul i32 %result.sroa.0.2124.i, 10
  %rest.111.i = add nsw i64 %src.sroa.26.2125.i, -1
  %rest.010.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.2126.i, i64 1
  %95 = sub i32 %94, %93
  %_27.not.i = icmp eq i64 %rest.111.i, 0
  br i1 %_27.not.i, label %bb18, label %bb20.i

bb48.i:                                           ; preds = %bb11.i143, %bb9.i138
  %src.sroa.26.0.i = phi i64 [ %rest.1.i, %bb11.i143 ], [ %version.sroa.8.0.copyload, %bb9.i138 ]
  %src.sroa.0.0.i = phi ptr [ %rest.0.i, %bb11.i143 ], [ %version.sroa.5.0.copyload, %bb9.i138 ]
  %96 = icmp samesign ult i64 %src.sroa.26.0.i, 8
  br i1 %96, label %bb13.preheader.i, label %bb25.i

bb13.preheader.i:                                 ; preds = %bb48.i
  %_14.not131.i = icmp eq i64 %src.sroa.26.0.i, 0
  br i1 %_14.not131.i, label %bb18, label %bb14.i

bb25.i:                                           ; preds = %bb48.i, %bb40.i
  %src.sroa.0.3130.i = phi ptr [ %rest.014.i, %bb40.i ], [ %src.sroa.0.0.i, %bb48.i ]
  %src.sroa.26.3129.i = phi i64 [ %rest.115.i, %bb40.i ], [ %src.sroa.26.0.i, %bb48.i ]
  %result.sroa.0.3128.i = phi i32 [ %_77.0.i, %bb40.i ], [ 0, %bb48.i ]
  %rest.014.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.3130.i, i64 1
  %rest.115.i = add nsw i64 %src.sroa.26.3129.i, -1
  %97 = tail call { i32, i1 } @llvm.smul.with.overflow.i32(i32 %result.sroa.0.3128.i, i32 10)
  %_73.0.i = extractvalue { i32, i1 } %97, 0
  %_73.1.i = extractvalue { i32, i1 } %97, 1
  br i1 %_73.1.i, label %bb2.i.sink.split, label %bb37.i144, !prof !41

bb37.i144:                                        ; preds = %bb25.i
  %98 = load i8, ptr %src.sroa.0.3130.i, align 1, !alias.scope !142, !noundef !4
  %99 = zext i8 %98 to i32
  %100 = add nsw i32 %99, -48
  %_14.i96.i = icmp ult i32 %100, 10
  br i1 %_14.i96.i, label %bb55.i, label %bb2.i

bb55.i:                                           ; preds = %bb37.i144
  %101 = tail call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %_73.0.i, i32 %100)
  %_77.1.i = extractvalue { i32, i1 } %101, 1
  br i1 %_77.1.i, label %bb2.i, label %bb40.i, !prof !41

bb40.i:                                           ; preds = %bb55.i
  %_77.0.i = extractvalue { i32, i1 } %101, 0
  %_40.not.i = icmp eq i64 %rest.115.i, 0
  br i1 %_40.not.i, label %bb18, label %bb25.i

bb14.i:                                           ; preds = %bb13.preheader.i
  %_21.i = load i8, ptr %src.sroa.0.0.i, align 1, !alias.scope !142, !noundef !4
  %_20.i = zext i8 %_21.i to i32
  %102 = add nsw i32 %_20.i, -48
  %_14.i100.i = icmp ult i32 %102, 10
  br i1 %_14.i100.i, label %bb18.i, label %bb2.i

bb18.i:                                           ; preds = %bb14.i
  %_14.not.i = icmp eq i64 %src.sroa.26.0.i, 1
  br i1 %_14.not.i, label %bb18, label %bb14.i.1

bb14.i.1:                                         ; preds = %bb18.i
  %rest.018.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 1
  %_21.i.1 = load i8, ptr %rest.018.i, align 1, !alias.scope !142, !noundef !4
  %_20.i.1 = zext i8 %_21.i.1 to i32
  %103 = add nsw i32 %_20.i.1, -48
  %_14.i100.i.1 = icmp ult i32 %103, 10
  br i1 %_14.i100.i.1, label %bb18.i.1, label %bb2.i

bb18.i.1:                                         ; preds = %bb14.i.1
  %104 = mul nuw nsw i32 %102, 10
  %105 = add nuw nsw i32 %103, %104
  %_14.not.i.1 = icmp eq i64 %src.sroa.26.0.i, 2
  br i1 %_14.not.i.1, label %bb18, label %bb14.i.2

bb14.i.2:                                         ; preds = %bb18.i.1
  %rest.018.i.1 = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 2
  %_21.i.2 = load i8, ptr %rest.018.i.1, align 1, !alias.scope !142, !noundef !4
  %_20.i.2 = zext i8 %_21.i.2 to i32
  %106 = add nsw i32 %_20.i.2, -48
  %_14.i100.i.2 = icmp ult i32 %106, 10
  br i1 %_14.i100.i.2, label %bb18.i.2, label %bb2.i

bb18.i.2:                                         ; preds = %bb14.i.2
  %107 = mul nuw nsw i32 %105, 10
  %108 = add nuw nsw i32 %106, %107
  %_14.not.i.2 = icmp eq i64 %src.sroa.26.0.i, 3
  br i1 %_14.not.i.2, label %bb18, label %bb14.i.3

bb14.i.3:                                         ; preds = %bb18.i.2
  %rest.018.i.2 = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 3
  %_21.i.3 = load i8, ptr %rest.018.i.2, align 1, !alias.scope !142, !noundef !4
  %_20.i.3 = zext i8 %_21.i.3 to i32
  %109 = add nsw i32 %_20.i.3, -48
  %_14.i100.i.3 = icmp ult i32 %109, 10
  br i1 %_14.i100.i.3, label %bb18.i.3, label %bb2.i

bb18.i.3:                                         ; preds = %bb14.i.3
  %110 = mul nuw nsw i32 %108, 10
  %111 = add nuw nsw i32 %109, %110
  %_14.not.i.3 = icmp eq i64 %src.sroa.26.0.i, 4
  br i1 %_14.not.i.3, label %bb18, label %bb14.i.4

bb14.i.4:                                         ; preds = %bb18.i.3
  %rest.018.i.3 = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 4
  %_21.i.4 = load i8, ptr %rest.018.i.3, align 1, !alias.scope !142, !noundef !4
  %_20.i.4 = zext i8 %_21.i.4 to i32
  %112 = add nsw i32 %_20.i.4, -48
  %_14.i100.i.4 = icmp ult i32 %112, 10
  br i1 %_14.i100.i.4, label %bb18.i.4, label %bb2.i

bb18.i.4:                                         ; preds = %bb14.i.4
  %113 = mul i32 %111, 10
  %114 = add i32 %112, %113
  %_14.not.i.4 = icmp eq i64 %src.sroa.26.0.i, 5
  br i1 %_14.not.i.4, label %bb18, label %bb14.i.5

bb14.i.5:                                         ; preds = %bb18.i.4
  %rest.018.i.4 = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 5
  %_21.i.5 = load i8, ptr %rest.018.i.4, align 1, !alias.scope !142, !noundef !4
  %_20.i.5 = zext i8 %_21.i.5 to i32
  %115 = add nsw i32 %_20.i.5, -48
  %_14.i100.i.5 = icmp ult i32 %115, 10
  br i1 %_14.i100.i.5, label %bb18.i.5, label %bb2.i

bb18.i.5:                                         ; preds = %bb14.i.5
  %116 = mul i32 %114, 10
  %117 = add i32 %115, %116
  %_14.not.i.5 = icmp eq i64 %src.sroa.26.0.i, 6
  br i1 %_14.not.i.5, label %bb18, label %bb14.i.6

bb14.i.6:                                         ; preds = %bb18.i.5
  %rest.018.i.5 = getelementptr inbounds nuw i8, ptr %src.sroa.0.0.i, i64 6
  %_21.i.6 = load i8, ptr %rest.018.i.5, align 1, !alias.scope !142, !noundef !4
  %_20.i.6 = zext i8 %_21.i.6 to i32
  %118 = add nsw i32 %_20.i.6, -48
  %_14.i100.i.6 = icmp ult i32 %118, 10
  br i1 %_14.i100.i.6, label %bb18.i.6, label %bb2.i

bb18.i.6:                                         ; preds = %bb14.i.6
  %119 = mul i32 %117, 10
  %120 = add i32 %118, %119
  br label %bb18

cleanup10:                                        ; preds = %bb2.i, %bb18
  %121 = landingpad { ptr, i32 }
          cleanup
  %122 = icmp eq i64 %version.sroa.0.0.copyload, 0
  br i1 %122, label %bb136, label %bb2.i.i.i4.i.i146

bb2.i.i.i4.i.i146:                                ; preds = %cleanup10
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %version.sroa.5.0.copyload, i64 noundef %version.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb136

bb2.i.sink.split:                                 ; preds = %bb28.i, %bb25.i
  %src.sroa.0.3130.i.lcssa.sink = phi ptr [ %src.sroa.0.3130.i, %bb25.i ], [ %src.sroa.0.1122.i, %bb28.i ]
  %.sink = phi i8 [ 2, %bb25.i ], [ 3, %bb28.i ]
  %123 = load i8, ptr %src.sroa.0.3130.i.lcssa.sink, align 1, !alias.scope !142, !noundef !4
  %124 = add i8 %123, -48
  %_14.i98.i = icmp ult i8 %124, 10
  %125 = select i1 %_14.i98.i, i8 %.sink, i8 1
  br label %bb2.i

bb2.i:                                            ; preds = %bb43.i, %bb63.i, %bb20.i, %bb37.i144, %bb55.i, %bb14.i, %bb14.i.1, %bb14.i.2, %bb14.i.3, %bb14.i.4, %bb14.i.5, %bb14.i.6, %bb2.i.sink.split, %bb17, %bb7.i136, %bb7.i136
  %_0.sroa.12.0.insert.insert.i.ph = phi i8 [ 1, %bb7.i136 ], [ 1, %bb7.i136 ], [ 0, %bb17 ], [ %125, %bb2.i.sink.split ], [ 1, %bb14.i.6 ], [ 1, %bb14.i.5 ], [ 1, %bb14.i.4 ], [ 1, %bb14.i.3 ], [ 1, %bb14.i.2 ], [ 1, %bb14.i.1 ], [ 1, %bb14.i ], [ 2, %bb55.i ], [ 1, %bb37.i144 ], [ 1, %bb20.i ], [ 3, %bb63.i ], [ 1, %bb43.i ]
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %e.i)
  store i8 %_0.sroa.12.0.insert.insert.i.ph, ptr %e.i, align 1
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d3ef88d2871426aa76206ddd2ecd76d5) #26
          to label %.noexc unwind label %cleanup10

.noexc:                                           ; preds = %bb2.i
  unreachable

bb18:                                             ; preds = %bb46.i, %bb23.i142, %bb40.i, %bb18.i, %bb18.i.1, %bb18.i.2, %bb18.i.3, %bb18.i.4, %bb18.i.5, %bb18.i.6, %bb13.preheader.i, %bb19.preheader.i
  %result.sroa.0.1.i = phi i32 [ 0, %bb13.preheader.i ], [ 0, %bb19.preheader.i ], [ %102, %bb18.i ], [ %105, %bb18.i.1 ], [ %108, %bb18.i.2 ], [ %111, %bb18.i.3 ], [ %114, %bb18.i.4 ], [ %117, %bb18.i.5 ], [ %120, %bb18.i.6 ], [ %_77.0.i, %bb40.i ], [ %95, %bb23.i142 ], [ %_85.0.i, %bb46.i ]
  store i32 %result.sroa.0.1.i, ptr %vers, align 4
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %vers, ptr %args, align 8
  %_31.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3implNtB9_7Display3fmt, ptr %_31.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8ef8a8c2c947634d07c01270a783d130, ptr noundef nonnull %args)
          to label %bb19 unwind label %cleanup10

bb19:                                             ; preds = %bb18
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  %126 = load i32, ptr %vers, align 4, !noundef !4
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %vers)
  %127 = icmp eq i64 %version.sroa.0.0.copyload, 0
  br i1 %127, label %bb141, label %bb2.i.i.i4.i.i148

bb2.i.i.i4.i.i148:                                ; preds = %bb19
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %version.sroa.5.0.copyload, i64 noundef %version.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb141

cleanup11:                                        ; preds = %bb37.i166, %bb22
  %128 = landingpad { ptr, i32 }
          cleanup
  br label %bb158

bb22:                                             ; preds = %bb20
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_4.i150)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_6.i)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_6.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_71264e62a593ae064235a5eb90a16b4a, i64 noundef 15)
          to label %.noexc167 unwind label %cleanup11

.noexc167:                                        ; preds = %bb22
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_4.i150, ptr noalias noundef nonnull align 8 dereferenceable(200) %_6.i)
          to label %bb2.i153 unwind label %cleanup.i151

cleanup.i151:                                     ; preds = %.noexc167
  %129 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i, %cleanup.i151
  %eh.lpad-body76.i = phi { ptr, i32 } [ %129, %cleanup.i151 ], [ %146, %bb1.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_6.i) #23
          to label %bb158 unwind label %terminate.i

bb2.i153:                                         ; preds = %.noexc167
  %130 = load i64, ptr %_4.i150, align 8, !range !33, !noundef !4
  %131 = icmp eq i64 %130, -9223372036854775808
  %132 = getelementptr inbounds nuw i8, ptr %_4.i150, i64 8
  br i1 %131, label %bb3.i.i164, label %bb38.i

bb3.i.i164:                                       ; preds = %bb2.i153
  call void @llvm.experimental.noalias.scope.decl(metadata !145)
  %_1.val.i.i165 = load ptr, ptr %132, align 8, !alias.scope !145, !nonnull !4, !noundef !4
  %bits.i.i.i.i.i = ptrtoint ptr %_1.val.i.i165 to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %bb37.i166, !prof !20

bb2.i2.i.i.i.i:                                   ; preds = %bb3.i.i164
  %133 = getelementptr i8, ptr %_1.val.i.i165, i64 -1
  %134 = icmp ne ptr %133, null
  call void @llvm.assume(i1 %134)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %133, align 8, !noalias !145
  %135 = getelementptr i8, ptr %_1.val.i.i165, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %135, align 8, !noalias !145, !nonnull !4, !align !8, !noundef !4
  %136 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !4, !noalias !145
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %136, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %137 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %137)
  invoke void %136(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i, !noalias !145

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %138 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %138)
  %139 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %140 = load i64, ptr %139, align 8, !range !9, !invariant.load !4, !noalias !145
  %141 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %142 = load i64, ptr %141, align 8, !range !10, !invariant.load !4, !noalias !145
  %143 = add i64 %142, -1
  %144 = icmp sgt i64 %143, -1
  call void @llvm.assume(i1 %144)
  %145 = icmp eq i64 %140, 0
  br i1 %145, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %140, i64 noundef range(i64 1, -9223372036854775807) %142) #22, !noalias !145
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %146 = landingpad { ptr, i32 }
          cleanup
  %147 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %148 = load i64, ptr %147, align 8, !range !9, !invariant.load !4, !noalias !145
  %149 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %150 = load i64, ptr %149, align 8, !range !10, !invariant.load !4, !noalias !145
  %151 = add i64 %150, -1
  %152 = icmp sgt i64 %151, -1
  call void @llvm.assume(i1 %152)
  %153 = icmp eq i64 %148, 0
  br i1 %153, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %148, i64 noundef range(i64 1, -9223372036854775807) %150) #22, !noalias !145
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %133, i64 noundef 24, i64 noundef 8) #22, !noalias !145
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %133, i64 noundef 24, i64 noundef 8) #22, !noalias !145
  br label %bb37.i166

bb38.i:                                           ; preds = %bb2.i153
  %_35.sroa.4.sroa.0.0.copyload.i = load ptr, ptr %132, align 8
  %_35.sroa.4.sroa.4.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i150, i64 16
  %_35.sroa.4.sroa.4.0.copyload.i = load i64, ptr %_35.sroa.4.sroa.4.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.4.sroa.5.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i150, i64 24
  %_35.sroa.4.sroa.5.0.copyload.i = load i64, ptr %_35.sroa.4.sroa.5.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.4.sroa.6.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i150, i64 32
  %_35.sroa.4.sroa.6.0.copyload.i = load ptr, ptr %_35.sroa.4.sroa.6.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.4.sroa.8.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i150, i64 48
  %_35.sroa.4.sroa.8.0.copyload.i = load i32, ptr %_35.sroa.4.sroa.8.0._35.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_4.i150)
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_6.i)
          to label %bb4.i155 unwind label %bb30.i154

bb37.i166:                                        ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i, %bb3.i.i164
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_4.i150)
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_6.i)
          to label %.noexc168 unwind label %cleanup11

.noexc168:                                        ; preds = %bb37.i166
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_6.i)
  br label %bb3.i171

bb4.i155:                                         ; preds = %bb38.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_6.i)
  %.not7.i = icmp eq i32 %_35.sroa.4.sroa.8.0.copyload.i, 0
  br i1 %.not7.i, label %bb39.i, label %bb40.i156

bb40.i156:                                        ; preds = %bb4.i155
  %154 = icmp eq i64 %130, 0
  br i1 %154, label %bb28.i158, label %bb2.i.i.i4.i.i157

bb2.i.i.i4.i.i157:                                ; preds = %bb40.i156
  %155 = icmp ne ptr %_35.sroa.4.sroa.0.0.copyload.i, null
  call void @llvm.assume(i1 %155)
  br label %bb28.sink.split.i

bb39.i:                                           ; preds = %bb4.i155
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !148
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_35.sroa.4.sroa.0.0.copyload.i, i64 noundef %_35.sroa.4.sroa.4.0.copyload.i)
          to label %bb1.i.i unwind label %cleanup.i.i, !noalias !148

cleanup.i.i:                                      ; preds = %bb39.i
  %156 = landingpad { ptr, i32 }
          cleanup
  %157 = icmp eq i64 %130, 0
  br i1 %157, label %bb29.i, label %bb29.sink.split.i

bb1.i.i:                                          ; preds = %bb39.i
  %_5.i.i = load i64, ptr %_2.i.i, align 8, !range !3, !noalias !148, !noundef !4
  %158 = trunc nuw i64 %_5.i.i to i1
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i), !noalias !148
  br i1 %158, label %bb44.i, label %bb47.i

bb44.i:                                           ; preds = %bb1.i.i
  %cond.i = icmp eq i64 %130, 0
  br i1 %cond.i, label %bb28.i158, label %bb28.sink.split.i

bb47.i:                                           ; preds = %bb1.i.i
  %_4.not.i.i160 = icmp samesign ult i64 %_35.sroa.4.sroa.4.0.copyload.i, 2
  br i1 %_4.not.i.i160, label %bb18.i163, label %bb48.i161

bb28.sink.split.i:                                ; preds = %bb44.i, %bb2.i.i.i4.i.i157
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.0.0.copyload.i, i64 noundef %130, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb28.i158

bb28.i158:                                        ; preds = %bb28.sink.split.i, %bb44.i, %bb40.i156
  %159 = icmp eq i64 %_35.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %159, label %bb3.i171, label %bb2.i.i.i4.i34.i

bb2.i.i.i4.i34.i:                                 ; preds = %bb28.i158
  %160 = icmp ne ptr %_35.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %160)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.6.0.copyload.i, i64 noundef %_35.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb3.i171

bb48.i161:                                        ; preds = %bb47.i
  %161 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_e9aa3e56236bea0534a07b33b08bbbe6, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !152
  %162 = icmp eq i32 %161, 0
  br i1 %162, label %bb18.i163, label %bb49.i

bb49.i:                                           ; preds = %bb48.i161
  %163 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_ae52c2733f312a4a903aef7e6436cb13, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !159
  %164 = icmp eq i32 %163, 0
  br i1 %164, label %bb18.i163, label %bb50.i

bb50.i:                                           ; preds = %bb49.i
  %165 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_3b059e5eb8e06e7498f909e7a08cef57, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !166
  %166 = icmp eq i32 %165, 0
  br i1 %166, label %bb18.i163, label %bb51.i

bb51.i:                                           ; preds = %bb50.i
  %167 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_d57d03743ee0b3cf85ca6cc66dce7f4d, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !173
  %168 = icmp eq i32 %167, 0
  br i1 %168, label %bb18.i163, label %bb52.i

bb52.i:                                           ; preds = %bb51.i
  %169 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_e33ac00bdbd8c0cbb04273e924bd654c, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !180
  %170 = icmp eq i32 %169, 0
  br i1 %170, label %bb18.i163, label %bb53.i

bb53.i:                                           ; preds = %bb52.i
  %171 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_27f411dcc9955beae922af37f2bb21f7, ptr noundef nonnull readonly align 1 dereferenceable(2) %_35.sroa.4.sroa.0.0.copyload.i, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !187
  %.fr.i = freeze i32 %171
  %172 = icmp eq i32 %.fr.i, 0
  br label %bb18.i163

bb18.i163:                                        ; preds = %bb53.i, %bb52.i, %bb51.i, %bb50.i, %bb49.i, %bb48.i161, %bb47.i
  %_0.sroa.11.2.i = phi i32 [ 10, %bb48.i161 ], [ 11, %bb49.i ], [ 12, %bb50.i ], [ 13, %bb51.i ], [ 14, %bb52.i ], [ 15, %bb53.i ], [ 15, %bb47.i ]
  %_0.sroa.0.2.i = phi i1 [ true, %bb48.i161 ], [ true, %bb49.i ], [ true, %bb50.i ], [ true, %bb51.i ], [ true, %bb52.i ], [ %172, %bb53.i ], [ false, %bb47.i ]
  %173 = icmp eq i64 %130, 0
  br i1 %173, label %bb19.i, label %bb2.i.i.i4.i.i62.i

bb2.i.i.i4.i.i62.i:                               ; preds = %bb18.i163
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.0.0.copyload.i, i64 noundef %130, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb19.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i.i62.i, %bb18.i163
  %174 = icmp eq i64 %_35.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %174, label %bb23, label %bb2.i.i.i4.i65.i

bb2.i.i.i4.i65.i:                                 ; preds = %bb19.i
  %175 = icmp ne ptr %_35.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %175)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.6.0.copyload.i, i64 noundef %_35.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br i1 %_0.sroa.0.2.i, label %182, label %bb3.i171

terminate.i:                                      ; preds = %cleanup.body.i
  %176 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb29.sink.split.i:                                ; preds = %bb2.i.i.i4.i74.i, %cleanup.i.i
  %.pn127.ph.i = phi { ptr, i32 } [ %179, %bb2.i.i.i4.i74.i ], [ %156, %cleanup.i.i ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.0.0.copyload.i, i64 noundef %130, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb29.i

bb29.i:                                           ; preds = %bb30.i154, %bb29.sink.split.i, %cleanup.i.i
  %.pn127.i = phi { ptr, i32 } [ %156, %cleanup.i.i ], [ %179, %bb30.i154 ], [ %.pn127.ph.i, %bb29.sink.split.i ]
  %177 = icmp eq i64 %_35.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %177, label %bb158, label %bb2.i.i.i4.i71.i

bb2.i.i.i4.i71.i:                                 ; preds = %bb29.i
  %178 = icmp ne ptr %_35.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %178)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.sroa.4.sroa.6.0.copyload.i, i64 noundef %_35.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb158

bb30.i154:                                        ; preds = %bb38.i
  %179 = landingpad { ptr, i32 }
          cleanup
  %180 = icmp eq i64 %130, 0
  br i1 %180, label %bb29.i, label %bb2.i.i.i4.i74.i

bb2.i.i.i4.i74.i:                                 ; preds = %bb30.i154
  %181 = icmp ne ptr %_35.sroa.4.sroa.0.0.copyload.i, null
  call void @llvm.assume(i1 %181)
  br label %bb29.sink.split.i

bb23:                                             ; preds = %bb19.i
  br i1 %_0.sroa.0.2.i, label %182, label %bb3.i171

182:                                              ; preds = %bb2.i.i.i4.i65.i, %bb23
  br label %bb3.i171

bb3.i171:                                         ; preds = %bb2.i.i.i4.i34.i, %bb28.i158, %.noexc168, %182, %bb23, %bb2.i.i.i4.i65.i, %bb20
  %which_freebsd.sroa.0.0505 = phi i32 [ 12, %bb20 ], [ %_0.sroa.11.2.i, %182 ], [ 12, %bb23 ], [ 12, %bb2.i.i.i4.i65.i ], [ 12, %.noexc168 ], [ 12, %bb28.i158 ], [ 12, %bb2.i.i.i4.i34.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !194)
  %183 = getelementptr inbounds nuw i8, ptr %_21, i64 8
  %.val.i170 = load i64, ptr %183, align 8, !alias.scope !194
  switch i64 %.val.i170, label %bb1.sink.split.i172 [
    i64 -9223372036854775808, label %bb141.thread
    i64 0, label %bb141.thread
  ]

bb1.sink.split.i172:                              ; preds = %bb3.i171
  %184 = getelementptr inbounds nuw i8, ptr %_21, i64 16
  %.val3.i173 = load ptr, ptr %184, align 8, !alias.scope !194, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i173, i64 noundef %.val.i170, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !194
  br label %bb141.thread

bb141.thread:                                     ; preds = %bb3.i171, %bb3.i171, %bb1.sink.split.i172
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_21)
  br label %bb32

bb141:                                            ; preds = %bb2.i.i.i4.i.i148, %bb19
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_21)
  %_36 = icmp slt i32 %126, 10
  br i1 %_36, label %bb31, label %bb32, !prof !197

bb32:                                             ; preds = %bb141.thread, %bb141
  %which_freebsd.sroa.0.0504715 = phi i32 [ %which_freebsd.sroa.0.0505, %bb141.thread ], [ %126, %bb141 ]
  %switch.tableidx = add i32 %which_freebsd.sroa.0.0504715, -10
  %185 = icmp ult i32 %switch.tableidx, 5
  br i1 %185, label %switch.lookup, label %bb30.invoke

bb31:                                             ; preds = %bb141
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_c63b2e5039c7d990f01e55018d57af8b, ptr noundef nonnull inttoptr (i64 77 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4aaeaad2afeac9a2adb9f1e49e255ee5) #26
          to label %unreachable unwind label %cleanup9.loopexit.split-lp

switch.lookup:                                    ; preds = %bb32
  %186 = zext nneg i32 %switch.tableidx to i64
  %switch.gep = getelementptr inbounds nuw [5 x ptr], ptr @switch.table._RNvCs2XfqmVweRya_18build_script_build4main, i64 0, i64 %186
  %switch.load = load ptr, ptr %switch.gep, align 8
  br label %bb30.invoke

bb30.invoke:                                      ; preds = %switch.lookup, %bb32
  %187 = phi ptr [ @alloc_c8539d7d8992b0450a5874fa781e9124, %bb32 ], [ %switch.load, %switch.lookup ]
; invoke build_script_build::set_cfg
  invoke fastcc void @_RNvCs2XfqmVweRya_18build_script_build7set_cfg(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %187, i64 noundef 9)
          to label %bb33 unwind label %cleanup9.loopexit.split-lp

bb33:                                             ; preds = %bb30.invoke
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_4.i177)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_7.i)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_7.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e7b0dd178336291b9ad3b8b25bc77cb0, i64 noundef 4)
          to label %.noexc253 unwind label %cleanup9.loopexit.split-lp

.noexc253:                                        ; preds = %bb33
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %_7.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_53695a5ce3568835c4a92269d444b5c9, i64 noundef 12)
          to label %bb2.i181 unwind label %cleanup.i178

cleanup.i178:                                     ; preds = %bb2.i181, %.noexc253
  %188 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i179

cleanup.body.i179:                                ; preds = %bb1.i.i.i.i.i.i249, %cleanup.i178
  %eh.lpad-body366.i = phi { ptr, i32 } [ %188, %cleanup.i178 ], [ %205, %bb1.i.i.i.i.i.i249 ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_7.i) #23
          to label %bb136 unwind label %terminate.i180

bb2.i181:                                         ; preds = %.noexc253
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_4.i177, ptr noalias noundef nonnull align 8 dereferenceable(200) %_7.i)
          to label %bb3.i182 unwind label %cleanup.i178

bb3.i182:                                         ; preds = %bb2.i181
  %189 = load i64, ptr %_4.i177, align 8, !range !33, !noundef !4
  %190 = icmp eq i64 %189, -9223372036854775808
  %191 = getelementptr inbounds nuw i8, ptr %_4.i177, i64 8
  br i1 %190, label %bb3.i.i236, label %bb26.i183

bb3.i.i236:                                       ; preds = %bb3.i182
  call void @llvm.experimental.noalias.scope.decl(metadata !198)
  %_1.val.i.i237 = load ptr, ptr %191, align 8, !alias.scope !198, !nonnull !4, !noundef !4
  %bits.i.i.i.i.i238 = ptrtoint ptr %_1.val.i.i237 to i64
  %_5.i.i.i.i.i239 = and i64 %bits.i.i.i.i.i238, 3
  %switch.i.i.i.i240 = icmp eq i64 %_5.i.i.i.i.i239, 1
  br i1 %switch.i.i.i.i240, label %bb2.i2.i.i.i.i242, label %bb25.i241, !prof !20

bb2.i2.i.i.i.i242:                                ; preds = %bb3.i.i236
  %192 = getelementptr i8, ptr %_1.val.i.i237, i64 -1
  %193 = icmp ne ptr %192, null
  call void @llvm.assume(i1 %193)
  %_6.val.i.i.i.i.i.i243 = load ptr, ptr %192, align 8, !noalias !198
  %194 = getelementptr i8, ptr %_1.val.i.i237, i64 7
  %_6.val1.i.i.i.i.i.i244 = load ptr, ptr %194, align 8, !noalias !198, !nonnull !4, !align !8, !noundef !4
  %195 = load ptr, ptr %_6.val1.i.i.i.i.i.i244, align 8, !invariant.load !4, !noalias !198
  %.not.i.i.i.i.i.i.i.i245 = icmp eq ptr %195, null
  br i1 %.not.i.i.i.i.i.i.i.i245, label %bb3.i.i.i.i.i.i.i.i250, label %is_not_null.i.i.i.i.i.i.i.i246

is_not_null.i.i.i.i.i.i.i.i246:                   ; preds = %bb2.i2.i.i.i.i242
  %196 = icmp ne ptr %_6.val.i.i.i.i.i.i243, null
  call void @llvm.assume(i1 %196)
  invoke void %195(ptr noundef nonnull %_6.val.i.i.i.i.i.i243)
          to label %bb3.i.i.i.i.i.i.i.i250 unwind label %cleanup.i.i.i.i.i.i.i.i247, !noalias !198

bb3.i.i.i.i.i.i.i.i250:                           ; preds = %is_not_null.i.i.i.i.i.i.i.i246, %bb2.i2.i.i.i.i242
  %197 = icmp ne ptr %_6.val.i.i.i.i.i.i243, null
  call void @llvm.assume(i1 %197)
  %198 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i244, i64 8
  %199 = load i64, ptr %198, align 8, !range !9, !invariant.load !4, !noalias !198
  %200 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i244, i64 16
  %201 = load i64, ptr %200, align 8, !range !10, !invariant.load !4, !noalias !198
  %202 = add i64 %201, -1
  %203 = icmp sgt i64 %202, -1
  call void @llvm.assume(i1 %203)
  %204 = icmp eq i64 %199, 0
  br i1 %204, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i252, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i251

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i251: ; preds = %bb3.i.i.i.i.i.i.i.i250
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i243, i64 noundef %199, i64 noundef range(i64 1, -9223372036854775807) %201) #22, !noalias !198
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i252

cleanup.i.i.i.i.i.i.i.i247:                       ; preds = %is_not_null.i.i.i.i.i.i.i.i246
  %205 = landingpad { ptr, i32 }
          cleanup
  %206 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i244, i64 8
  %207 = load i64, ptr %206, align 8, !range !9, !invariant.load !4, !noalias !198
  %208 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i244, i64 16
  %209 = load i64, ptr %208, align 8, !range !10, !invariant.load !4, !noalias !198
  %210 = add i64 %209, -1
  %211 = icmp sgt i64 %210, -1
  call void @llvm.assume(i1 %211)
  %212 = icmp eq i64 %207, 0
  br i1 %212, label %bb1.i.i.i.i.i.i249, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i248

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i248: ; preds = %cleanup.i.i.i.i.i.i.i.i247
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i243, i64 noundef %207, i64 noundef range(i64 1, -9223372036854775807) %209) #22, !noalias !198
  br label %bb1.i.i.i.i.i.i249

bb1.i.i.i.i.i.i249:                               ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i248, %cleanup.i.i.i.i.i.i.i.i247
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %192, i64 noundef 24, i64 noundef 8) #22, !noalias !198
  br label %cleanup.body.i179

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i252: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i251, %bb3.i.i.i.i.i.i.i.i250
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %192, i64 noundef 24, i64 noundef 8) #22, !noalias !198
  br label %bb25.i241

bb26.i183:                                        ; preds = %bb3.i182
  %_34.sroa.4.sroa.0.0.copyload.i = load ptr, ptr %191, align 8
  %_34.sroa.4.sroa.4.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i177, i64 16
  %_34.sroa.4.sroa.4.0.copyload.i = load i64, ptr %_34.sroa.4.sroa.4.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_34.sroa.4.sroa.5.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i177, i64 24
  %_34.sroa.4.sroa.5.0.copyload.i = load i64, ptr %_34.sroa.4.sroa.5.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_34.sroa.4.sroa.6.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i177, i64 32
  %_34.sroa.4.sroa.6.0.copyload.i = load ptr, ptr %_34.sroa.4.sroa.6.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  %_34.sroa.4.sroa.8.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.i177, i64 48
  %_34.sroa.4.sroa.8.0.copyload.i = load i32, ptr %_34.sroa.4.sroa.8.0._34.sroa.4.0._4.sroa_idx.sroa_idx.i, align 8
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_4.i177)
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_7.i)
          to label %bb5.i186 unwind label %bb18.i184

bb25.i241:                                        ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i252, %bb3.i.i236
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_4.i177)
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_7.i)
          to label %.noexc254 unwind label %cleanup9.loopexit.split-lp

.noexc254:                                        ; preds = %bb25.i241
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_7.i)
  br label %bb39

bb5.i186:                                         ; preds = %bb26.i183
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_7.i)
  %.not11.i187 = icmp eq i32 %_34.sroa.4.sroa.8.0.copyload.i, 0
  br i1 %.not11.i187, label %bb27.i192, label %bb28.i188

bb28.i188:                                        ; preds = %bb5.i186
  %213 = icmp eq i64 %189, 0
  br i1 %213, label %bb16.i, label %bb2.i.i.i4.i.i189

bb2.i.i.i4.i.i189:                                ; preds = %bb28.i188
  %214 = icmp ne ptr %_34.sroa.4.sroa.0.0.copyload.i, null
  call void @llvm.assume(i1 %214)
  br label %bb16.sink.split.i

bb27.i192:                                        ; preds = %bb5.i186
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i176), !noalias !201
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2.i.i176, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_34.sroa.4.sroa.0.0.copyload.i, i64 noundef %_34.sroa.4.sroa.4.0.copyload.i)
          to label %bb1.i.i194 unwind label %cleanup.i.i193, !noalias !201

cleanup.i.i193:                                   ; preds = %bb27.i192
  %215 = landingpad { ptr, i32 }
          cleanup
  %216 = icmp eq i64 %189, 0
  br i1 %216, label %bb17.i185, label %bb17.sink.split.i

bb1.i.i194:                                       ; preds = %bb27.i192
  %_5.i.i195 = load i64, ptr %_2.i.i176, align 8, !range !3, !noalias !201, !noundef !4
  %217 = trunc nuw i64 %_5.i.i195 to i1
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i176), !noalias !201
  br i1 %217, label %bb32.i234, label %bb35.i196

bb32.i234:                                        ; preds = %bb1.i.i194
  %cond.i235 = icmp eq i64 %189, 0
  br i1 %cond.i235, label %bb16.i, label %bb16.sink.split.i

bb35.i196:                                        ; preds = %bb1.i.i194
  call void @llvm.experimental.noalias.scope.decl(metadata !205)
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_34.sroa.4.sroa.0.0.copyload.i, i64 %_34.sroa.4.sroa.4.0.copyload.i
  %_6.i.i.i.i11.i.i.i.i = icmp samesign eq i64 %_34.sroa.4.sroa.4.0.copyload.i, 0
  br i1 %_6.i.i.i.i11.i.i.i.i, label %bb5.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb35.i196, %bb5.i.i.i.i
  %218 = phi i64 [ %225, %bb5.i.i.i.i ], [ 0, %bb35.i196 ]
  %_16.i26.i.i.i1012.i.i.i.i = phi ptr [ %subtracted.i.i.i.i.i.i, %bb5.i.i.i.i ], [ %_34.sroa.4.sroa.0.0.copyload.i, %bb35.i196 ]
  %219 = ptrtoint ptr %_16.i26.i.i.i1012.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1012.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1012.i.i.i.i, align 1, !alias.scope !205, !noalias !208, !noundef !4
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1012.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !208, !noundef !4
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %220 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb3.i.i.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1012.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !208, !noundef !4
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %221 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1012.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !208, !noundef !4
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %222 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i
  %subtracted.i.i.i.i.i.i = phi ptr [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %220, %bb4.i.i.i.i.i.i.i ], [ %221, %bb6.i.i.i.i.i.i.i ], [ %222, %bb8.i.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %223 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %223)
  %224 = ptrtoint ptr %subtracted.i.i.i.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %224, %219
  %225 = add i64 %_10.i.i.i.i.i.i, %218
  switch i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i [
    i32 32, label %bb5.i.i.i.i
    i32 13, label %bb5.i.i.i.i
    i32 12, label %bb5.i.i.i.i
    i32 11, label %bb5.i.i.i.i
    i32 10, label %bb5.i.i.i.i
    i32 9, label %bb5.i.i.i.i
  ]

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb3.i.i.i.i.i
  %_4.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_4.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i, label %bb5.i.i

bb5.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i.i.i
  %_3.i.i.i.i.i.i.i.i.i = lshr i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i.i.i, label %bb5.i.i [
    i32 0, label %bb6.i.i.i.i.i.i.i.i.i
    i32 22, label %bb4.i.i.i.i.i.i.i.i.i
    i32 32, label %bb7.i.i.i.i.i.i.i.i.i
    i32 48, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i
  ]

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb5.i.i.i.i.i.i.i.i
  %226 = icmp eq i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 5760
  br i1 %226, label %bb5.i.i.i.i, label %bb5.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb5.i.i.i.i.i.i.i.i
  %227 = and i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i.i.i = zext nneg i32 %227 to i64
  %228 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_8.i.i.i.i.i.i.i.i.i
  %_6.i.i.i.i.i.i.i.i.i = load i8, ptr %228, align 1, !noalias !222, !noundef !4
  %extract.t.i.i.i.i.i.i.i.i.i = trunc i8 %_6.i.i.i.i.i.i.i.i.i to i1
  br i1 %extract.t.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i, label %bb5.i.i

bb7.i.i.i.i.i.i.i.i.i:                            ; preds = %bb5.i.i.i.i.i.i.i.i
  %229 = and i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i.i.i = zext nneg i32 %229 to i64
  %230 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_14.i.i.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i.i.i = load i8, ptr %230, align 1, !noalias !222, !noundef !4
  %231 = and i8 %_12.i.i.i.i.i.i.i.i.i, 2
  %extract.t3.i.i.i.i.not.i.i.i.i.i = icmp eq i8 %231, 0
  br i1 %extract.t3.i.i.i.i.not.i.i.i.i.i, label %bb5.i.i, label %bb5.i.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i: ; preds = %bb5.i.i.i.i.i.i.i.i
  %232 = icmp eq i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 12288
  br i1 %232, label %bb5.i.i.i.i, label %bb5.i.i

bb5.i.i.i.i:                                      ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %subtracted.i.i.i.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb2.i43.i, label %bb14.i.i.i.i.i.i.i

bb5.i.i:                                          ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i, %bb35.i196
  %matcher.sroa.4.048.i.i = phi ptr [ %_34.sroa.4.sroa.0.0.copyload.i, %bb35.i196 ], [ %subtracted.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i ], [ %subtracted.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i ], [ %subtracted.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ], [ %subtracted.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %subtracted.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i.i ], [ %subtracted.i.i.i.i.i.i, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i ]
  %matcher.sroa.14.046.i.i = phi i64 [ 0, %bb35.i196 ], [ %225, %bb1.i.i.i.i.i.i.i.i ], [ %225, %bb5.i.i.i.i.i.i.i.i ], [ %225, %bb4.i.i.i.i.i.i.i.i.i ], [ %225, %bb6.i.i.i.i.i.i.i.i.i ], [ %225, %bb7.i.i.i.i.i.i.i.i.i ], [ %225, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i ]
  %i.sroa.0.0.i.i = phi i64 [ 0, %bb35.i196 ], [ %218, %bb1.i.i.i.i.i.i.i.i ], [ %218, %bb5.i.i.i.i.i.i.i.i ], [ %218, %bb4.i.i.i.i.i.i.i.i.i ], [ %218, %bb6.i.i.i.i.i.i.i.i.i ], [ %218, %bb7.i.i.i.i.i.i.i.i.i ], [ %218, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i.i.i ]
  %233 = icmp eq ptr %matcher.sroa.4.048.i.i, %_7.i.i.i.i.i
  br i1 %233, label %bb2.i43.i, label %bb17.i.i.i.i.i.i.i

bb17.i.i.i.i.i.i.i:                               ; preds = %bb5.i.i, %bb5.i.i11.i.i
  %_23.i25.i.i.i1213.i.i.i.i = phi ptr [ %_4.i.i.i.i.i.i, %bb5.i.i11.i.i ], [ %_7.i.i.i.i.i, %bb5.i.i ]
  %_23.i.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i.i, i64 -1
  %w.i.i.i.i.i4.i.i = load i8, ptr %_23.i.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !223, !noundef !4
  %_6.i.i.i.i.i5.i.i = icmp sgt i8 %w.i.i.i.i.i4.i.i, -1
  br i1 %_6.i.i.i.i.i5.i.i, label %bb3.i.i.i.i.i38.i.i, label %bb4.i.i.i.i.i6.i.i

bb4.i.i.i.i.i6.i.i:                               ; preds = %bb17.i.i.i.i.i.i.i
  %234 = icmp ne ptr %matcher.sroa.4.048.i.i, %_23.i.i.i.i.i.i.i.i
  call void @llvm.assume(i1 %234)
  %_23.i13.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i.i, i64 -2
  %z.i.i.i.i.i7.i.i = load i8, ptr %_23.i13.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !223, !noundef !4
  %_25.i.i.i.i.i8.i.i = and i8 %z.i.i.i.i.i7.i.i, 31
  %235 = zext nneg i8 %_25.i.i.i.i.i8.i.i to i32
  %_12.i.i.i.i.i.i.i = icmp slt i8 %z.i.i.i.i.i7.i.i, -64
  br i1 %_12.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i29.i.i, label %bb13.i.i.i.i.i.i.i

bb3.i.i.i.i.i38.i.i:                              ; preds = %bb17.i.i.i.i.i.i.i
  %_8.i.i.i.i.i.i.i = zext nneg i8 %w.i.i.i.i.i4.i.i to i32
  br label %bb3.i.i.i10.i.i

bb6.i.i.i.i.i29.i.i:                              ; preds = %bb4.i.i.i.i.i6.i.i
  %236 = icmp ne ptr %matcher.sroa.4.048.i.i, %_23.i13.i.i.i.i.i.i.i
  call void @llvm.assume(i1 %236)
  %_23.i19.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i.i, i64 -3
  %y.i.i.i.i.i30.i.i = load i8, ptr %_23.i19.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !223, !noundef !4
  %_29.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i30.i.i, 15
  %237 = zext nneg i8 %_29.i.i.i.i.i.i.i to i32
  %_16.i.i.i.i.i.i.i = icmp slt i8 %y.i.i.i.i.i30.i.i, -64
  br i1 %_16.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i33.i.i, label %bb11.i.i.i.i.i.i.i

bb13.i.i.i.i.i.i.i:                               ; preds = %bb11.i.i.i.i.i.i.i, %bb4.i.i.i.i.i6.i.i
  %_4.i14.i.i.i.i.i.i = phi ptr [ %_4.i15.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i ], [ %_23.i13.i.i.i.i.i.i.i, %bb4.i.i.i.i.i6.i.i ]
  %ch.sroa.0.0.i.i.i.i.i.i.i = phi i32 [ %242, %bb11.i.i.i.i.i.i.i ], [ %235, %bb4.i.i.i.i.i6.i.i ]
  %_40.i.i.i.i.i9.i.i = shl nuw nsw i32 %ch.sroa.0.0.i.i.i.i.i.i.i, 6
  %_42.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i4.i.i, 63
  %_41.i.i.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i.i.i to i32
  %238 = or disjoint i32 %_40.i.i.i.i.i9.i.i, %_41.i.i.i.i.i.i.i
  br label %bb3.i.i.i10.i.i

bb8.i.i.i.i.i33.i.i:                              ; preds = %bb6.i.i.i.i.i29.i.i
  %239 = icmp ne ptr %matcher.sroa.4.048.i.i, %_23.i19.i.i.i.i.i.i.i
  call void @llvm.assume(i1 %239)
  %_23.i25.i.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i.i.i, i64 -4
  %x.i.i.i.i.i34.i.i = load i8, ptr %_23.i25.i.i.i.i.i.i.i, align 1, !alias.scope !205, !noalias !223, !noundef !4
  %_33.i.i.i.i.i35.i.i = and i8 %x.i.i.i.i.i34.i.i, 7
  %240 = zext nneg i8 %_33.i.i.i.i.i35.i.i to i32
  %_34.i.i.i.i.i36.i.i = shl nuw nsw i32 %240, 6
  %_36.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i30.i.i, 63
  %_35.i.i.i.i.i37.i.i = zext nneg i8 %_36.i.i.i.i.i.i.i to i32
  %241 = or disjoint i32 %_34.i.i.i.i.i36.i.i, %_35.i.i.i.i.i37.i.i
  br label %bb11.i.i.i.i.i.i.i

bb11.i.i.i.i.i.i.i:                               ; preds = %bb8.i.i.i.i.i33.i.i, %bb6.i.i.i.i.i29.i.i
  %_4.i15.i.i.i.i.i.i = phi ptr [ %_23.i25.i.i.i.i.i.i.i, %bb8.i.i.i.i.i33.i.i ], [ %_23.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i29.i.i ]
  %ch.sroa.0.1.i.i.i.i.i.i.i = phi i32 [ %241, %bb8.i.i.i.i.i33.i.i ], [ %237, %bb6.i.i.i.i.i29.i.i ]
  %_37.i.i.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.1.i.i.i.i.i.i.i, 6
  %_39.i.i.i.i.i31.i.i = and i8 %z.i.i.i.i.i7.i.i, 63
  %_38.i.i.i.i.i32.i.i = zext nneg i8 %_39.i.i.i.i.i31.i.i to i32
  %242 = or disjoint i32 %_37.i.i.i.i.i.i.i, %_38.i.i.i.i.i32.i.i
  br label %bb13.i.i.i.i.i.i.i

bb3.i.i.i10.i.i:                                  ; preds = %bb13.i.i.i.i.i.i.i, %bb3.i.i.i.i.i38.i.i
  %_4.i.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i38.i.i ], [ %_4.i14.i.i.i.i.i.i, %bb13.i.i.i.i.i.i.i ]
  %_0.sroa.4.1.i.ph.i.i.i.i.i.i = phi i32 [ %_8.i.i.i.i.i.i.i, %bb3.i.i.i.i.i38.i.i ], [ %238, %bb13.i.i.i.i.i.i.i ]
  %243 = icmp samesign ult i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %243)
  switch i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i14.i.i [
    i32 32, label %bb5.i.i11.i.i
    i32 13, label %bb5.i.i11.i.i
    i32 12, label %bb5.i.i11.i.i
    i32 11, label %bb5.i.i11.i.i
    i32 10, label %bb5.i.i11.i.i
    i32 9, label %bb5.i.i11.i.i
  ]

bb1.i.i.i.i.i.i14.i.i:                            ; preds = %bb3.i.i.i10.i.i
  %_4.i.i.i.i.i.i15.i.i = icmp samesign ugt i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 127
  br i1 %_4.i.i.i.i.i.i15.i.i, label %bb5.i.i.i.i.i.i17.i.i, label %bb7.i37.i

bb5.i.i.i.i.i.i17.i.i:                            ; preds = %bb1.i.i.i.i.i.i14.i.i
  %_3.i.i.i.i.i.i.i18.i.i = lshr i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i18.i.i, label %bb7.i37.i [
    i32 0, label %bb6.i.i.i.i.i.i.i25.i.i
    i32 22, label %bb4.i.i.i.i.i.i.i24.i.i
    i32 32, label %bb7.i.i.i.i.i.i.i20.i.i
    i32 48, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i19.i.i
  ]

bb4.i.i.i.i.i.i.i24.i.i:                          ; preds = %bb5.i.i.i.i.i.i17.i.i
  %244 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 5760
  br i1 %244, label %bb5.i.i11.i.i, label %bb7.i37.i

bb6.i.i.i.i.i.i.i25.i.i:                          ; preds = %bb5.i.i.i.i.i.i17.i.i
  %245 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i26.i.i = zext nneg i32 %245 to i64
  %246 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_8.i.i.i.i.i.i.i26.i.i
  %_6.i.i.i.i.i.i.i27.i.i = load i8, ptr %246, align 1, !noalias !237, !noundef !4
  %extract.t.i.i.i.i.i.i.i28.i.i = trunc i8 %_6.i.i.i.i.i.i.i27.i.i to i1
  br i1 %extract.t.i.i.i.i.i.i.i28.i.i, label %bb5.i.i11.i.i, label %bb7.i37.i

bb7.i.i.i.i.i.i.i20.i.i:                          ; preds = %bb5.i.i.i.i.i.i17.i.i
  %247 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i21.i.i = zext nneg i32 %247 to i64
  %248 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_14.i.i.i.i.i.i.i21.i.i
  %_12.i.i.i.i.i.i.i22.i.i = load i8, ptr %248, align 1, !noalias !237, !noundef !4
  %249 = and i8 %_12.i.i.i.i.i.i.i22.i.i, 2
  %extract.t3.i.i.i.i.not.i.i.i23.i.i = icmp eq i8 %249, 0
  br i1 %extract.t3.i.i.i.i.not.i.i.i23.i.i, label %bb7.i37.i, label %bb5.i.i11.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i19.i.i: ; preds = %bb5.i.i.i.i.i.i17.i.i
  %250 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i.i.i, 12288
  br i1 %250, label %bb5.i.i11.i.i, label %bb7.i37.i

bb5.i.i11.i.i:                                    ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i19.i.i, %bb7.i.i.i.i.i.i.i20.i.i, %bb6.i.i.i.i.i.i.i25.i.i, %bb4.i.i.i.i.i.i.i24.i.i, %bb3.i.i.i10.i.i, %bb3.i.i.i10.i.i, %bb3.i.i.i10.i.i, %bb3.i.i.i10.i.i, %bb3.i.i.i10.i.i, %bb3.i.i.i10.i.i
  %251 = icmp eq ptr %matcher.sroa.4.048.i.i, %_4.i.i.i.i.i.i
  br i1 %251, label %bb2.i43.i, label %bb17.i.i.i.i.i.i.i

bb7.i37.i:                                        ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs2XfqmVweRya_18build_script_build.exit.i.i.i19.i.i, %bb7.i.i.i.i.i.i.i20.i.i, %bb6.i.i.i.i.i.i.i25.i.i, %bb4.i.i.i.i.i.i.i24.i.i, %bb5.i.i.i.i.i.i17.i.i, %bb1.i.i.i.i.i.i14.i.i
  %252 = ptrtoint ptr %_23.i25.i.i.i1213.i.i.i.i to i64
  %253 = ptrtoint ptr %matcher.sroa.4.048.i.i to i64
  %254 = sub i64 %matcher.sroa.14.046.i.i, %253
  %_15.i6.i.i.i.i = add i64 %254, %252
  br label %bb2.i43.i

bb16.sink.split.i:                                ; preds = %bb32.i234, %bb2.i.i.i4.i.i189
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.0.0.copyload.i, i64 noundef %189, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb16.i

bb16.i:                                           ; preds = %bb16.sink.split.i, %bb32.i234, %bb28.i188
  %255 = icmp eq i64 %_34.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %255, label %bb39, label %bb2.i.i.i4.i39.i

bb2.i.i.i4.i39.i:                                 ; preds = %bb16.i
  %256 = icmp ne ptr %_34.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %256)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.6.0.copyload.i, i64 noundef %_34.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb39

bb2.i43.i:                                        ; preds = %bb5.i.i.i.i, %bb5.i.i11.i.i, %bb7.i37.i, %bb5.i.i
  %i.sroa.0.059.i.i = phi i64 [ %i.sroa.0.0.i.i, %bb7.i37.i ], [ %i.sroa.0.0.i.i, %bb5.i.i ], [ %i.sroa.0.0.i.i, %bb5.i.i11.i.i ], [ 0, %bb5.i.i.i.i ]
  %j.sroa.0.1.i.i = phi i64 [ %_15.i6.i.i.i.i, %bb7.i37.i ], [ %matcher.sroa.14.046.i.i, %bb5.i.i ], [ %matcher.sroa.14.046.i.i, %bb5.i.i11.i.i ], [ 0, %bb5.i.i.i.i ]
  %new_len.i.i = sub nuw i64 %j.sroa.0.1.i.i, %i.sroa.0.059.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_34.sroa.4.sroa.0.0.copyload.i, i64 %i.sroa.0.059.i.i
  %_7.i.i.i = getelementptr inbounds nuw i8, ptr %_34.sroa.4.sroa.0.0.copyload.i, i64 %j.sroa.0.1.i.i
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i, %bb2.i43.i
  %pieces.sroa.12375.0.i = phi ptr [ %data.i.i, %bb2.i43.i ], [ %pieces.sroa.12375.1.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ]
  %pieces.sroa.30.0.i = phi i64 [ 0, %bb2.i43.i ], [ %263, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ]
  %257 = ptrtoint ptr %pieces.sroa.12375.0.i to i64
  %_6.i.i.i.i.i.i.i44.i = icmp eq ptr %pieces.sroa.12375.0.i, %_7.i.i.i
  br i1 %_6.i.i.i.i.i.i.i44.i, label %bb40.i197, label %bb14.i.i.i.i.i.i45.i

bb14.i.i.i.i.i.i45.i:                             ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i46.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.0.i, i64 1
  %x.i.i.i.i.i.i47.i = load i8, ptr %pieces.sroa.12375.0.i, align 1, !noalias !238, !noundef !4
  %_6.i.i.i.i.i.i48.i = icmp sgt i8 %x.i.i.i.i.i.i47.i, -1
  br i1 %_6.i.i.i.i.i.i48.i, label %bb3.i.i.i.i.i.i85.i, label %bb4.i.i.i.i.i.i49.i

bb4.i.i.i.i.i.i49.i:                              ; preds = %bb14.i.i.i.i.i.i45.i
  %_30.i.i.i.i.i.i50.i = and i8 %x.i.i.i.i.i.i47.i, 31
  %init.i.i.i.i.i.i51.i = zext nneg i8 %_30.i.i.i.i.i.i50.i to i32
  %_6.i10.i.i.i.i.i.i52.i = icmp ne ptr %_16.i.i.i.i.i.i.i46.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i52.i)
  %_16.i12.i.i.i.i.i.i53.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.0.i, i64 2
  %y.i.i.i.i.i.i54.i = load i8, ptr %_16.i.i.i.i.i.i.i46.i, align 1, !noalias !238, !noundef !4
  %_33.i.i.i.i.i.i55.i = shl nuw nsw i32 %init.i.i.i.i.i.i51.i, 6
  %_35.i.i.i.i.i.i56.i = and i8 %y.i.i.i.i.i.i54.i, 63
  %_34.i.i.i.i.i.i57.i = zext nneg i8 %_35.i.i.i.i.i.i56.i to i32
  %258 = or disjoint i32 %_33.i.i.i.i.i.i55.i, %_34.i.i.i.i.i.i57.i
  %_13.i.i.i.i.i.i58.i = icmp samesign ugt i8 %x.i.i.i.i.i.i47.i, -33
  br i1 %_13.i.i.i.i.i.i58.i, label %bb6.i.i.i.i.i.i65.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i

bb3.i.i.i.i.i.i85.i:                              ; preds = %bb14.i.i.i.i.i.i45.i
  %_7.i.i.i.i.i.i86.i = zext nneg i8 %x.i.i.i.i.i.i47.i to i32
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i

bb6.i.i.i.i.i.i65.i:                              ; preds = %bb4.i.i.i.i.i.i49.i
  %_6.i17.i.i.i.i.i.i66.i = icmp ne ptr %_16.i12.i.i.i.i.i.i53.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i66.i)
  %_16.i19.i.i.i.i.i.i67.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.0.i, i64 3
  %z.i.i.i.i.i.i68.i = load i8, ptr %_16.i12.i.i.i.i.i.i53.i, align 1, !noalias !238, !noundef !4
  %_38.i.i.i.i.i.i69.i = shl nuw nsw i32 %_34.i.i.i.i.i.i57.i, 6
  %_40.i.i.i.i.i.i70.i = and i8 %z.i.i.i.i.i.i68.i, 63
  %_39.i.i.i.i.i.i71.i = zext nneg i8 %_40.i.i.i.i.i.i70.i to i32
  %y_z.i.i.i.i.i.i72.i = or disjoint i32 %_38.i.i.i.i.i.i69.i, %_39.i.i.i.i.i.i71.i
  %_20.i.i.i.i.i.i73.i = shl nuw nsw i32 %init.i.i.i.i.i.i51.i, 12
  %259 = or disjoint i32 %y_z.i.i.i.i.i.i72.i, %_20.i.i.i.i.i.i73.i
  %_21.i.i.i.i.i.i74.i = icmp samesign ugt i8 %x.i.i.i.i.i.i47.i, -17
  br i1 %_21.i.i.i.i.i.i74.i, label %bb8.i.i.i.i.i.i75.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i

bb8.i.i.i.i.i.i75.i:                              ; preds = %bb6.i.i.i.i.i.i65.i
  %_6.i24.i.i.i.i.i.i76.i = icmp ne ptr %_16.i19.i.i.i.i.i.i67.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i76.i)
  %_16.i26.i.i.i.i.i.i77.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.0.i, i64 4
  %w.i.i.i.i.i.i78.i = load i8, ptr %_16.i19.i.i.i.i.i.i67.i, align 1, !noalias !238, !noundef !4
  %_26.i.i.i.i.i.i79.i = shl nuw nsw i32 %init.i.i.i.i.i.i51.i, 18
  %_25.i.i.i.i.i.i80.i = and i32 %_26.i.i.i.i.i.i79.i, 1835008
  %_43.i.i.i.i.i.i81.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i72.i, 6
  %_45.i.i.i.i.i.i82.i = and i8 %w.i.i.i.i.i.i78.i, 63
  %_44.i.i.i.i.i.i83.i = zext nneg i8 %_45.i.i.i.i.i.i82.i to i32
  %_27.i.i.i.i.i.i84.i = or disjoint i32 %_43.i.i.i.i.i.i81.i, %_44.i.i.i.i.i.i83.i
  %260 = or disjoint i32 %_27.i.i.i.i.i.i84.i, %_25.i.i.i.i.i.i80.i
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i

_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i: ; preds = %bb8.i.i.i.i.i.i75.i, %bb6.i.i.i.i.i.i65.i, %bb3.i.i.i.i.i.i85.i, %bb4.i.i.i.i.i.i49.i
  %pieces.sroa.12375.1.i = phi ptr [ %_16.i.i.i.i.i.i.i46.i, %bb3.i.i.i.i.i.i85.i ], [ %_16.i26.i.i.i.i.i.i77.i, %bb8.i.i.i.i.i.i75.i ], [ %_16.i19.i.i.i.i.i.i67.i, %bb6.i.i.i.i.i.i65.i ], [ %_16.i12.i.i.i.i.i.i53.i, %bb4.i.i.i.i.i.i49.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i60.i = phi i32 [ %_7.i.i.i.i.i.i86.i, %bb3.i.i.i.i.i.i85.i ], [ %260, %bb8.i.i.i.i.i.i75.i ], [ %259, %bb6.i.i.i.i.i.i65.i ], [ %258, %bb4.i.i.i.i.i.i49.i ]
  %261 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i60.i, 1114112
  call void @llvm.assume(i1 %261)
  %262 = ptrtoint ptr %pieces.sroa.12375.1.i to i64
  %_10.i.i.i.i.i61.i = sub i64 %262, %257
  %263 = add i64 %_10.i.i.i.i.i61.i, %pieces.sroa.30.0.i
  %264 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i60.i, -47
  %spec.select.i.not.i.i.i.i.i = icmp ult i32 %264, -2
  br i1 %spec.select.i.not.i.i.i.i.i, label %bb1.i.i.i.i, label %bb40.i197

bb40.i197:                                        ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i, %bb1.i.i.i.i
  %pieces.sroa.12375.2.i = phi ptr [ %pieces.sroa.12375.1.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ], [ %_7.i.i.i, %bb1.i.i.i.i ]
  %pieces.sroa.30.1.i = phi i64 [ %263, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ], [ %pieces.sroa.30.0.i, %bb1.i.i.i.i ]
  %pieces.sroa.36.0.i = phi i64 [ %263, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ], [ 0, %bb1.i.i.i.i ]
  %_0.sroa.4.1.i.i = phi i64 [ %pieces.sroa.30.0.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i.i ], [ %new_len.i.i, %bb1.i.i.i.i ]
  switch i64 %_0.sroa.4.1.i.i, label %bb9thread-pre-split.i.i232 [
    i64 0, label %bb44.i198
    i64 1, label %bb7.i89.i
  ]

bb7.i89.i:                                        ; preds = %bb40.i197
  %265 = load i8, ptr %data.i.i, align 1, !alias.scope !254, !noalias !257, !noundef !4
  switch i8 %265, label %bb9.i.i203 [
    i8 43, label %bb44.i198
    i8 45, label %bb44.i198
  ]

bb9thread-pre-split.i.i232:                       ; preds = %bb40.i197
  %.pr.i.i233 = load i8, ptr %data.i.i, align 1, !alias.scope !254, !noalias !257
  br label %bb9.i.i203

bb9.i.i203:                                       ; preds = %bb9thread-pre-split.i.i232, %bb7.i89.i
  %266 = phi i8 [ %.pr.i.i233, %bb9thread-pre-split.i.i232 ], [ %265, %bb7.i89.i ]
  %cond.i.i204 = icmp eq i8 %266, 43
  %rest.1.i.i205 = sext i1 %cond.i.i204 to i64
  %src.sroa.15.0.i.i206 = add nsw i64 %_0.sroa.4.1.i.i, %rest.1.i.i205
  %src.sroa.0.0.idx.i.i207 = zext i1 %cond.i.i204 to i64
  %src.sroa.0.0.i.i208 = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %src.sroa.0.0.idx.i.i207
  %_10.i.i209 = icmp samesign ult i64 %src.sroa.15.0.i.i206, 17
  br i1 %_10.i.i209, label %bb15.preheader.i.i223, label %bb22.i.i210

bb15.preheader.i.i223:                            ; preds = %bb9.i.i203
  %_13.not56.i.i224 = icmp eq i64 %src.sroa.15.0.i.i206, 0
  br i1 %_13.not56.i.i224, label %bb43.i222, label %bb16.i.i225

bb22.i.i210:                                      ; preds = %bb9.i.i203, %bb33.i.i216
  %result.sroa.0.0.i.i211 = phi i64 [ %_63.0.i.i, %bb33.i.i216 ], [ 0, %bb9.i.i203 ]
  %src.sroa.15.1.i.i212 = phi i64 [ %rest.12.i.i218, %bb33.i.i216 ], [ %src.sroa.15.0.i.i206, %bb9.i.i203 ]
  %src.sroa.0.1.i.i213 = phi ptr [ %rest.01.i.i219, %bb33.i.i216 ], [ %src.sroa.0.0.i.i208, %bb9.i.i203 ]
  %_30.not.i.i = icmp eq i64 %src.sroa.15.1.i.i212, 0
  br i1 %_30.not.i.i, label %bb43.i222, label %bb23.i.i214

bb23.i.i214:                                      ; preds = %bb22.i.i210
  %267 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i.i211, i64 10)
  %_60.1.i.i215 = extractvalue { i64, i1 } %267, 1
  br i1 %_60.1.i.i215, label %bb44.i198, label %bb33.i.i216, !prof !41

bb33.i.i216:                                      ; preds = %bb23.i.i214
  %_60.0.i.i217 = extractvalue { i64, i1 } %267, 0
  %rest.12.i.i218 = add nsw i64 %src.sroa.15.1.i.i212, -1
  %rest.01.i.i219 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i213, i64 1
  %268 = load i8, ptr %src.sroa.0.1.i.i213, align 1, !alias.scope !254, !noalias !257, !noundef !4
  %269 = zext i8 %268 to i32
  %270 = add nsw i32 %269, -48
  %_14.i.i.i220 = icmp ugt i32 %270, 9
  %271 = zext nneg i32 %270 to i64
  %_63.0.i.i = add i64 %_60.0.i.i217, %271
  %_63.1.i.i = icmp ult i64 %_63.0.i.i, %_60.0.i.i217
  %or.cond.i221 = select i1 %_14.i.i.i220, i1 true, i1 %_63.1.i.i
  br i1 %or.cond.i221, label %bb44.i198, label %bb22.i.i210, !prof !88

bb16.i.i225:                                      ; preds = %bb15.preheader.i.i223, %bb20.i.i230
  %src.sroa.0.259.i.i226 = phi ptr [ %rest.05.i.i, %bb20.i.i230 ], [ %src.sroa.0.0.i.i208, %bb15.preheader.i.i223 ]
  %src.sroa.15.258.i.i227 = phi i64 [ %rest.16.i.i, %bb20.i.i230 ], [ %src.sroa.15.0.i.i206, %bb15.preheader.i.i223 ]
  %result.sroa.0.257.i.i228 = phi i64 [ %274, %bb20.i.i230 ], [ 0, %bb15.preheader.i.i223 ]
  %_20.i.i = load i8, ptr %src.sroa.0.259.i.i226, align 1, !alias.scope !254, !noalias !257, !noundef !4
  %_19.i.i229 = zext i8 %_20.i.i to i32
  %272 = add nsw i32 %_19.i.i229, -48
  %_14.i46.i.i = icmp ult i32 %272, 10
  br i1 %_14.i46.i.i, label %bb20.i.i230, label %bb44.i198

bb20.i.i230:                                      ; preds = %bb16.i.i225
  %273 = mul i64 %result.sroa.0.257.i.i228, 10
  %rest.16.i.i = add nsw i64 %src.sroa.15.258.i.i227, -1
  %rest.05.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i226, i64 1
  %_24.i.i = zext nneg i32 %272 to i64
  %274 = add i64 %273, %_24.i.i
  %_13.not.i.i231 = icmp eq i64 %rest.16.i.i, 0
  br i1 %_13.not.i.i231, label %bb43.i222, label %bb16.i.i225

bb43.i222:                                        ; preds = %bb22.i.i210, %bb20.i.i230, %bb15.preheader.i.i223
  %_60.sroa.11395.0.i = phi i64 [ 0, %bb15.preheader.i.i223 ], [ %274, %bb20.i.i230 ], [ %result.sroa.0.0.i.i211, %bb22.i.i210 ]
  %275 = mul i64 %_60.sroa.11395.0.i, 10000
  br label %bb44.i198

bb44.i198:                                        ; preds = %bb33.i.i216, %bb23.i.i214, %bb16.i.i225, %bb43.i222, %bb7.i89.i, %bb7.i89.i, %bb40.i197
  %major.sroa.0.0.i = phi i64 [ %275, %bb43.i222 ], [ %_0.sroa.4.1.i.i, %bb40.i197 ], [ 0, %bb7.i89.i ], [ 0, %bb7.i89.i ], [ 0, %bb16.i.i225 ], [ 0, %bb23.i.i214 ], [ 0, %bb33.i.i216 ]
  br i1 %_6.i.i.i.i.i.i.i44.i, label %bb58.i, label %bb1.i.i.i101.i

bb1.i.i.i101.i:                                   ; preds = %bb44.i198, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i
  %pieces.sroa.12375.3.i = phi ptr [ %pieces.sroa.12375.4.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %pieces.sroa.12375.2.i, %bb44.i198 ]
  %pieces.sroa.30.2.i = phi i64 [ %282, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %pieces.sroa.30.1.i, %bb44.i198 ]
  %276 = ptrtoint ptr %pieces.sroa.12375.3.i to i64
  %_6.i.i.i.i.i.i.i103.i = icmp eq ptr %pieces.sroa.12375.3.i, %_7.i.i.i
  br i1 %_6.i.i.i.i.i.i.i103.i, label %bb45.i, label %bb14.i.i.i.i.i.i104.i

bb14.i.i.i.i.i.i104.i:                            ; preds = %bb1.i.i.i101.i
  %_16.i.i.i.i.i.i.i105.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.3.i, i64 1
  %x.i.i.i.i.i.i106.i = load i8, ptr %pieces.sroa.12375.3.i, align 1, !noalias !259, !noundef !4
  %_6.i.i.i.i.i.i107.i = icmp sgt i8 %x.i.i.i.i.i.i106.i, -1
  br i1 %_6.i.i.i.i.i.i107.i, label %bb3.i.i.i.i.i.i152.i, label %bb4.i.i.i.i.i.i108.i

bb4.i.i.i.i.i.i108.i:                             ; preds = %bb14.i.i.i.i.i.i104.i
  %_30.i.i.i.i.i.i109.i = and i8 %x.i.i.i.i.i.i106.i, 31
  %init.i.i.i.i.i.i110.i = zext nneg i8 %_30.i.i.i.i.i.i109.i to i32
  %_6.i10.i.i.i.i.i.i111.i = icmp ne ptr %_16.i.i.i.i.i.i.i105.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i111.i)
  %_16.i12.i.i.i.i.i.i112.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.3.i, i64 2
  %y.i.i.i.i.i.i113.i = load i8, ptr %_16.i.i.i.i.i.i.i105.i, align 1, !noalias !259, !noundef !4
  %_33.i.i.i.i.i.i114.i = shl nuw nsw i32 %init.i.i.i.i.i.i110.i, 6
  %_35.i.i.i.i.i.i115.i = and i8 %y.i.i.i.i.i.i113.i, 63
  %_34.i.i.i.i.i.i116.i = zext nneg i8 %_35.i.i.i.i.i.i115.i to i32
  %277 = or disjoint i32 %_33.i.i.i.i.i.i114.i, %_34.i.i.i.i.i.i116.i
  %_13.i.i.i.i.i.i117.i = icmp samesign ugt i8 %x.i.i.i.i.i.i106.i, -33
  br i1 %_13.i.i.i.i.i.i117.i, label %bb6.i.i.i.i.i.i132.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i

bb3.i.i.i.i.i.i152.i:                             ; preds = %bb14.i.i.i.i.i.i104.i
  %_7.i.i.i.i.i.i153.i = zext nneg i8 %x.i.i.i.i.i.i106.i to i32
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i

bb6.i.i.i.i.i.i132.i:                             ; preds = %bb4.i.i.i.i.i.i108.i
  %_6.i17.i.i.i.i.i.i133.i = icmp ne ptr %_16.i12.i.i.i.i.i.i112.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i133.i)
  %_16.i19.i.i.i.i.i.i134.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.3.i, i64 3
  %z.i.i.i.i.i.i135.i = load i8, ptr %_16.i12.i.i.i.i.i.i112.i, align 1, !noalias !259, !noundef !4
  %_38.i.i.i.i.i.i136.i = shl nuw nsw i32 %_34.i.i.i.i.i.i116.i, 6
  %_40.i.i.i.i.i.i137.i = and i8 %z.i.i.i.i.i.i135.i, 63
  %_39.i.i.i.i.i.i138.i = zext nneg i8 %_40.i.i.i.i.i.i137.i to i32
  %y_z.i.i.i.i.i.i139.i = or disjoint i32 %_38.i.i.i.i.i.i136.i, %_39.i.i.i.i.i.i138.i
  %_20.i.i.i.i.i.i140.i = shl nuw nsw i32 %init.i.i.i.i.i.i110.i, 12
  %278 = or disjoint i32 %y_z.i.i.i.i.i.i139.i, %_20.i.i.i.i.i.i140.i
  %_21.i.i.i.i.i.i141.i = icmp samesign ugt i8 %x.i.i.i.i.i.i106.i, -17
  br i1 %_21.i.i.i.i.i.i141.i, label %bb8.i.i.i.i.i.i142.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i

bb8.i.i.i.i.i.i142.i:                             ; preds = %bb6.i.i.i.i.i.i132.i
  %_6.i24.i.i.i.i.i.i143.i = icmp ne ptr %_16.i19.i.i.i.i.i.i134.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i143.i)
  %_16.i26.i.i.i.i.i.i144.i = getelementptr inbounds nuw i8, ptr %pieces.sroa.12375.3.i, i64 4
  %w.i.i.i.i.i.i145.i = load i8, ptr %_16.i19.i.i.i.i.i.i134.i, align 1, !noalias !259, !noundef !4
  %_26.i.i.i.i.i.i146.i = shl nuw nsw i32 %init.i.i.i.i.i.i110.i, 18
  %_25.i.i.i.i.i.i147.i = and i32 %_26.i.i.i.i.i.i146.i, 1835008
  %_43.i.i.i.i.i.i148.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i139.i, 6
  %_45.i.i.i.i.i.i149.i = and i8 %w.i.i.i.i.i.i145.i, 63
  %_44.i.i.i.i.i.i150.i = zext nneg i8 %_45.i.i.i.i.i.i149.i to i32
  %_27.i.i.i.i.i.i151.i = or disjoint i32 %_43.i.i.i.i.i.i148.i, %_44.i.i.i.i.i.i150.i
  %279 = or disjoint i32 %_27.i.i.i.i.i.i151.i, %_25.i.i.i.i.i.i147.i
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i

_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i: ; preds = %bb8.i.i.i.i.i.i142.i, %bb6.i.i.i.i.i.i132.i, %bb3.i.i.i.i.i.i152.i, %bb4.i.i.i.i.i.i108.i
  %pieces.sroa.12375.4.i = phi ptr [ %_16.i.i.i.i.i.i.i105.i, %bb3.i.i.i.i.i.i152.i ], [ %_16.i26.i.i.i.i.i.i144.i, %bb8.i.i.i.i.i.i142.i ], [ %_16.i19.i.i.i.i.i.i134.i, %bb6.i.i.i.i.i.i132.i ], [ %_16.i12.i.i.i.i.i.i112.i, %bb4.i.i.i.i.i.i108.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i120.i = phi i32 [ %_7.i.i.i.i.i.i153.i, %bb3.i.i.i.i.i.i152.i ], [ %279, %bb8.i.i.i.i.i.i142.i ], [ %278, %bb6.i.i.i.i.i.i132.i ], [ %277, %bb4.i.i.i.i.i.i108.i ]
  %280 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i120.i, 1114112
  call void @llvm.assume(i1 %280)
  %281 = ptrtoint ptr %pieces.sroa.12375.4.i to i64
  %_10.i.i.i.i.i121.i = sub i64 %281, %276
  %282 = add i64 %_10.i.i.i.i.i121.i, %pieces.sroa.30.2.i
  %283 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i120.i, -47
  %spec.select.i.not.i.i.i.i124.i = icmp ult i32 %283, -2
  br i1 %spec.select.i.not.i.i.i.i124.i, label %bb1.i.i.i101.i, label %bb45.i

bb45.i:                                           ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i, %bb1.i.i.i101.i
  %pieces.sroa.12375.5.i = phi ptr [ %pieces.sroa.12375.4.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %_7.i.i.i, %bb1.i.i.i101.i ]
  %pieces.sroa.30.3.i = phi i64 [ %282, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %pieces.sroa.30.2.i, %bb1.i.i.i101.i ]
  %pieces.sroa.36.1.i = phi i64 [ %282, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %pieces.sroa.36.0.i, %bb1.i.i.i101.i ]
  %pieces.sroa.30.2.pn.i = phi i64 [ %pieces.sroa.30.2.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i118.i ], [ %new_len.i.i, %bb1.i.i.i101.i ]
  %_0.sroa.0.1.i131.i = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %pieces.sroa.36.0.i
  %_0.sroa.4.1.i130.i = sub nuw i64 %pieces.sroa.30.2.pn.i, %pieces.sroa.36.0.i
  switch i64 %_0.sroa.4.1.i130.i, label %bb9thread-pre-split.i219.i [
    i64 0, label %bb51.i199
    i64 1, label %bb7.i171.i
  ]

bb7.i171.i:                                       ; preds = %bb45.i
  %284 = load i8, ptr %_0.sroa.0.1.i131.i, align 1, !alias.scope !275, !noalias !278, !noundef !4
  switch i8 %284, label %bb9.i174.i [
    i8 43, label %bb51.i199
    i8 45, label %bb51.i199
  ]

bb9thread-pre-split.i219.i:                       ; preds = %bb45.i
  %.pr.i220.i = load i8, ptr %_0.sroa.0.1.i131.i, align 1, !alias.scope !275, !noalias !278
  br label %bb9.i174.i

bb9.i174.i:                                       ; preds = %bb9thread-pre-split.i219.i, %bb7.i171.i
  %285 = phi i8 [ %.pr.i220.i, %bb9thread-pre-split.i219.i ], [ %284, %bb7.i171.i ]
  %cond.i175.i = icmp eq i8 %285, 43
  %rest.1.i176.i = sext i1 %cond.i175.i to i64
  %src.sroa.15.0.i177.i = add nsw i64 %_0.sroa.4.1.i130.i, %rest.1.i176.i
  %src.sroa.0.0.idx.i178.i = zext i1 %cond.i175.i to i64
  %src.sroa.0.0.i179.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i131.i, i64 %src.sroa.0.0.idx.i178.i
  %_10.i180.i = icmp samesign ult i64 %src.sroa.15.0.i177.i, 17
  br i1 %_10.i180.i, label %bb15.preheader.i203.i, label %bb22.i181.i

bb15.preheader.i203.i:                            ; preds = %bb9.i174.i
  %_13.not56.i204.i = icmp eq i64 %src.sroa.15.0.i177.i, 0
  br i1 %_13.not56.i204.i, label %bb50.i202, label %bb16.i205.i

bb22.i181.i:                                      ; preds = %bb9.i174.i, %bb33.i191.i
  %result.sroa.0.0.i182.i = phi i64 [ %_63.0.i195.i, %bb33.i191.i ], [ 0, %bb9.i174.i ]
  %src.sroa.15.1.i183.i = phi i64 [ %rest.12.i188.i, %bb33.i191.i ], [ %src.sroa.15.0.i177.i, %bb9.i174.i ]
  %src.sroa.0.1.i184.i = phi ptr [ %rest.01.i187.i, %bb33.i191.i ], [ %src.sroa.0.0.i179.i, %bb9.i174.i ]
  %_30.not.i185.i = icmp eq i64 %src.sroa.15.1.i183.i, 0
  br i1 %_30.not.i185.i, label %bb50.i202, label %bb23.i186.i

bb23.i186.i:                                      ; preds = %bb22.i181.i
  %286 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i182.i, i64 10)
  %_60.1.i190.i = extractvalue { i64, i1 } %286, 1
  br i1 %_60.1.i190.i, label %bb51.i199, label %bb33.i191.i, !prof !41

bb33.i191.i:                                      ; preds = %bb23.i186.i
  %_60.0.i189.i = extractvalue { i64, i1 } %286, 0
  %rest.12.i188.i = add nsw i64 %src.sroa.15.1.i183.i, -1
  %rest.01.i187.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i184.i, i64 1
  %287 = load i8, ptr %src.sroa.0.1.i184.i, align 1, !alias.scope !275, !noalias !278, !noundef !4
  %288 = zext i8 %287 to i32
  %289 = add nsw i32 %288, -48
  %_14.i.i192.i = icmp ugt i32 %289, 9
  %290 = zext nneg i32 %289 to i64
  %_63.0.i195.i = add i64 %_60.0.i189.i, %290
  %_63.1.i196.i = icmp ult i64 %_63.0.i195.i, %_60.0.i189.i
  %or.cond504.i = select i1 %_14.i.i192.i, i1 true, i1 %_63.1.i196.i
  br i1 %or.cond504.i, label %bb51.i199, label %bb22.i181.i, !prof !88

bb16.i205.i:                                      ; preds = %bb15.preheader.i203.i, %bb20.i213.i
  %src.sroa.0.259.i206.i = phi ptr [ %rest.05.i215.i, %bb20.i213.i ], [ %src.sroa.0.0.i179.i, %bb15.preheader.i203.i ]
  %src.sroa.15.258.i207.i = phi i64 [ %rest.16.i214.i, %bb20.i213.i ], [ %src.sroa.15.0.i177.i, %bb15.preheader.i203.i ]
  %result.sroa.0.257.i208.i = phi i64 [ %293, %bb20.i213.i ], [ 0, %bb15.preheader.i203.i ]
  %_20.i209.i = load i8, ptr %src.sroa.0.259.i206.i, align 1, !alias.scope !275, !noalias !278, !noundef !4
  %_19.i210.i = zext i8 %_20.i209.i to i32
  %291 = add nsw i32 %_19.i210.i, -48
  %_14.i46.i211.i = icmp ult i32 %291, 10
  br i1 %_14.i46.i211.i, label %bb20.i213.i, label %bb51.i199

bb20.i213.i:                                      ; preds = %bb16.i205.i
  %292 = mul i64 %result.sroa.0.257.i208.i, 10
  %rest.16.i214.i = add nsw i64 %src.sroa.15.258.i207.i, -1
  %rest.05.i215.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i206.i, i64 1
  %_24.i216.i = zext nneg i32 %291 to i64
  %293 = add i64 %292, %_24.i216.i
  %_13.not.i217.i = icmp eq i64 %rest.16.i214.i, 0
  br i1 %_13.not.i217.i, label %bb50.i202, label %bb16.i205.i

bb50.i202:                                        ; preds = %bb22.i181.i, %bb20.i213.i, %bb15.preheader.i203.i
  %_67.sroa.11396.0.i = phi i64 [ 0, %bb15.preheader.i203.i ], [ %293, %bb20.i213.i ], [ %result.sroa.0.0.i182.i, %bb22.i181.i ]
  %294 = mul i64 %_67.sroa.11396.0.i, 100
  br label %bb51.i199

bb51.i199:                                        ; preds = %bb33.i191.i, %bb23.i186.i, %bb16.i205.i, %bb50.i202, %bb7.i171.i, %bb7.i171.i, %bb45.i
  %minor.sroa.0.0.i = phi i64 [ %294, %bb50.i202 ], [ %_0.sroa.4.1.i130.i, %bb45.i ], [ 0, %bb7.i171.i ], [ 0, %bb7.i171.i ], [ 0, %bb16.i205.i ], [ 0, %bb23.i186.i ], [ 0, %bb33.i191.i ]
  br i1 %_6.i.i.i.i.i.i.i103.i, label %bb58.i, label %bb1.i.i.i231.i

bb1.i.i.i231.i:                                   ; preds = %bb51.i199, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i
  %295 = phi i64 [ %302, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i ], [ %pieces.sroa.30.3.i, %bb51.i199 ]
  %_16.i26.i.i.i4.i.i.i232.i = phi ptr [ %subtracted.i.i.i.i.i249.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i ], [ %pieces.sroa.12375.5.i, %bb51.i199 ]
  %296 = ptrtoint ptr %_16.i26.i.i.i4.i.i.i232.i to i64
  %_6.i.i.i.i.i.i.i233.i = icmp eq ptr %_16.i26.i.i.i4.i.i.i232.i, %_7.i.i.i
  br i1 %_6.i.i.i.i.i.i.i233.i, label %bb52.i200, label %bb14.i.i.i.i.i.i234.i

bb14.i.i.i.i.i.i234.i:                            ; preds = %bb1.i.i.i231.i
  %_16.i.i.i.i.i.i.i235.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i232.i, i64 1
  %x.i.i.i.i.i.i236.i = load i8, ptr %_16.i26.i.i.i4.i.i.i232.i, align 1, !noalias !280, !noundef !4
  %_6.i.i.i.i.i.i237.i = icmp sgt i8 %x.i.i.i.i.i.i236.i, -1
  br i1 %_6.i.i.i.i.i.i237.i, label %bb3.i.i.i.i.i.i282.i, label %bb4.i.i.i.i.i.i238.i

bb4.i.i.i.i.i.i238.i:                             ; preds = %bb14.i.i.i.i.i.i234.i
  %_30.i.i.i.i.i.i239.i = and i8 %x.i.i.i.i.i.i236.i, 31
  %init.i.i.i.i.i.i240.i = zext nneg i8 %_30.i.i.i.i.i.i239.i to i32
  %_6.i10.i.i.i.i.i.i241.i = icmp ne ptr %_16.i.i.i.i.i.i.i235.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i241.i)
  %_16.i12.i.i.i.i.i.i242.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i232.i, i64 2
  %y.i.i.i.i.i.i243.i = load i8, ptr %_16.i.i.i.i.i.i.i235.i, align 1, !noalias !280, !noundef !4
  %_33.i.i.i.i.i.i244.i = shl nuw nsw i32 %init.i.i.i.i.i.i240.i, 6
  %_35.i.i.i.i.i.i245.i = and i8 %y.i.i.i.i.i.i243.i, 63
  %_34.i.i.i.i.i.i246.i = zext nneg i8 %_35.i.i.i.i.i.i245.i to i32
  %297 = or disjoint i32 %_33.i.i.i.i.i.i244.i, %_34.i.i.i.i.i.i246.i
  %_13.i.i.i.i.i.i247.i = icmp samesign ugt i8 %x.i.i.i.i.i.i236.i, -33
  br i1 %_13.i.i.i.i.i.i247.i, label %bb6.i.i.i.i.i.i262.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i

bb3.i.i.i.i.i.i282.i:                             ; preds = %bb14.i.i.i.i.i.i234.i
  %_7.i.i.i.i.i.i283.i = zext nneg i8 %x.i.i.i.i.i.i236.i to i32
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i

bb6.i.i.i.i.i.i262.i:                             ; preds = %bb4.i.i.i.i.i.i238.i
  %_6.i17.i.i.i.i.i.i263.i = icmp ne ptr %_16.i12.i.i.i.i.i.i242.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i263.i)
  %_16.i19.i.i.i.i.i.i264.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i232.i, i64 3
  %z.i.i.i.i.i.i265.i = load i8, ptr %_16.i12.i.i.i.i.i.i242.i, align 1, !noalias !280, !noundef !4
  %_38.i.i.i.i.i.i266.i = shl nuw nsw i32 %_34.i.i.i.i.i.i246.i, 6
  %_40.i.i.i.i.i.i267.i = and i8 %z.i.i.i.i.i.i265.i, 63
  %_39.i.i.i.i.i.i268.i = zext nneg i8 %_40.i.i.i.i.i.i267.i to i32
  %y_z.i.i.i.i.i.i269.i = or disjoint i32 %_38.i.i.i.i.i.i266.i, %_39.i.i.i.i.i.i268.i
  %_20.i.i.i.i.i.i270.i = shl nuw nsw i32 %init.i.i.i.i.i.i240.i, 12
  %298 = or disjoint i32 %y_z.i.i.i.i.i.i269.i, %_20.i.i.i.i.i.i270.i
  %_21.i.i.i.i.i.i271.i = icmp samesign ugt i8 %x.i.i.i.i.i.i236.i, -17
  br i1 %_21.i.i.i.i.i.i271.i, label %bb8.i.i.i.i.i.i272.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i

bb8.i.i.i.i.i.i272.i:                             ; preds = %bb6.i.i.i.i.i.i262.i
  %_6.i24.i.i.i.i.i.i273.i = icmp ne ptr %_16.i19.i.i.i.i.i.i264.i, %_7.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i273.i)
  %_16.i26.i.i.i.i.i.i274.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i232.i, i64 4
  %w.i.i.i.i.i.i275.i = load i8, ptr %_16.i19.i.i.i.i.i.i264.i, align 1, !noalias !280, !noundef !4
  %_26.i.i.i.i.i.i276.i = shl nuw nsw i32 %init.i.i.i.i.i.i240.i, 18
  %_25.i.i.i.i.i.i277.i = and i32 %_26.i.i.i.i.i.i276.i, 1835008
  %_43.i.i.i.i.i.i278.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i269.i, 6
  %_45.i.i.i.i.i.i279.i = and i8 %w.i.i.i.i.i.i275.i, 63
  %_44.i.i.i.i.i.i280.i = zext nneg i8 %_45.i.i.i.i.i.i279.i to i32
  %_27.i.i.i.i.i.i281.i = or disjoint i32 %_43.i.i.i.i.i.i278.i, %_44.i.i.i.i.i.i280.i
  %299 = or disjoint i32 %_27.i.i.i.i.i.i281.i, %_25.i.i.i.i.i.i277.i
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i

_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i: ; preds = %bb8.i.i.i.i.i.i272.i, %bb6.i.i.i.i.i.i262.i, %bb3.i.i.i.i.i.i282.i, %bb4.i.i.i.i.i.i238.i
  %subtracted.i.i.i.i.i249.i = phi ptr [ %_16.i12.i.i.i.i.i.i242.i, %bb4.i.i.i.i.i.i238.i ], [ %_16.i19.i.i.i.i.i.i264.i, %bb6.i.i.i.i.i.i262.i ], [ %_16.i26.i.i.i.i.i.i274.i, %bb8.i.i.i.i.i.i272.i ], [ %_16.i.i.i.i.i.i.i235.i, %bb3.i.i.i.i.i.i282.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i250.i = phi i32 [ %297, %bb4.i.i.i.i.i.i238.i ], [ %298, %bb6.i.i.i.i.i.i262.i ], [ %299, %bb8.i.i.i.i.i.i272.i ], [ %_7.i.i.i.i.i.i283.i, %bb3.i.i.i.i.i.i282.i ]
  %300 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i250.i, 1114112
  call void @llvm.assume(i1 %300)
  %301 = ptrtoint ptr %subtracted.i.i.i.i.i249.i to i64
  %_10.i.i.i.i.i251.i = sub i64 %295, %296
  %302 = add i64 %_10.i.i.i.i.i251.i, %301
  %303 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i250.i, -47
  %spec.select.i.not.i.i.i.i254.i = icmp ult i32 %303, -2
  br i1 %spec.select.i.not.i.i.i.i254.i, label %bb1.i.i.i231.i, label %bb52.i200

bb52.i200:                                        ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i, %bb1.i.i.i231.i
  %.pn.i = phi i64 [ %295, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build.exit.i.i.i248.i ], [ %new_len.i.i, %bb1.i.i.i231.i ]
  %_0.sroa.0.1.i261.i = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %pieces.sroa.36.1.i
  %_0.sroa.4.1.i260.i = sub nuw i64 %.pn.i, %pieces.sroa.36.1.i
  switch i64 %_0.sroa.4.1.i260.i, label %bb9thread-pre-split.i349.i [
    i64 0, label %bb58.i
    i64 1, label %bb7.i301.i
  ]

bb7.i301.i:                                       ; preds = %bb52.i200
  %304 = load i8, ptr %_0.sroa.0.1.i261.i, align 1, !alias.scope !296, !noalias !299, !noundef !4
  switch i8 %304, label %bb9.i304.i [
    i8 43, label %bb58.i
    i8 45, label %bb58.i
  ]

bb9thread-pre-split.i349.i:                       ; preds = %bb52.i200
  %.pr.i350.i = load i8, ptr %_0.sroa.0.1.i261.i, align 1, !alias.scope !296, !noalias !299
  br label %bb9.i304.i

bb9.i304.i:                                       ; preds = %bb9thread-pre-split.i349.i, %bb7.i301.i
  %305 = phi i8 [ %.pr.i350.i, %bb9thread-pre-split.i349.i ], [ %304, %bb7.i301.i ]
  %cond.i305.i = icmp eq i8 %305, 43
  %rest.1.i306.i = sext i1 %cond.i305.i to i64
  %src.sroa.15.0.i307.i = add nsw i64 %_0.sroa.4.1.i260.i, %rest.1.i306.i
  %src.sroa.0.0.idx.i308.i = zext i1 %cond.i305.i to i64
  %src.sroa.0.0.i309.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i261.i, i64 %src.sroa.0.0.idx.i308.i
  %_10.i310.i = icmp samesign ult i64 %src.sroa.15.0.i307.i, 17
  br i1 %_10.i310.i, label %bb15.preheader.i333.i, label %bb22.i311.i

bb15.preheader.i333.i:                            ; preds = %bb9.i304.i
  %_13.not56.i334.i = icmp eq i64 %src.sroa.15.0.i307.i, 0
  br i1 %_13.not56.i334.i, label %bb58.i, label %bb16.i335.i

bb22.i311.i:                                      ; preds = %bb9.i304.i, %bb33.i321.i
  %result.sroa.0.0.i312.i = phi i64 [ %_63.0.i325.i, %bb33.i321.i ], [ 0, %bb9.i304.i ]
  %src.sroa.15.1.i313.i = phi i64 [ %rest.12.i318.i, %bb33.i321.i ], [ %src.sroa.15.0.i307.i, %bb9.i304.i ]
  %src.sroa.0.1.i314.i = phi ptr [ %rest.01.i317.i, %bb33.i321.i ], [ %src.sroa.0.0.i309.i, %bb9.i304.i ]
  %_30.not.i315.i = icmp eq i64 %src.sroa.15.1.i313.i, 0
  br i1 %_30.not.i315.i, label %bb58.i, label %bb23.i316.i

bb23.i316.i:                                      ; preds = %bb22.i311.i
  %306 = call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %result.sroa.0.0.i312.i, i64 10)
  %_60.1.i320.i = extractvalue { i64, i1 } %306, 1
  br i1 %_60.1.i320.i, label %bb58.i, label %bb33.i321.i, !prof !41

bb33.i321.i:                                      ; preds = %bb23.i316.i
  %_60.0.i319.i = extractvalue { i64, i1 } %306, 0
  %rest.12.i318.i = add nsw i64 %src.sroa.15.1.i313.i, -1
  %rest.01.i317.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i314.i, i64 1
  %307 = load i8, ptr %src.sroa.0.1.i314.i, align 1, !alias.scope !296, !noalias !299, !noundef !4
  %308 = zext i8 %307 to i32
  %309 = add nsw i32 %308, -48
  %_14.i.i322.i = icmp ugt i32 %309, 9
  %310 = zext nneg i32 %309 to i64
  %_63.0.i325.i = add i64 %_60.0.i319.i, %310
  %_63.1.i326.i = icmp ult i64 %_63.0.i325.i, %_60.0.i319.i
  %or.cond505.i = select i1 %_14.i.i322.i, i1 true, i1 %_63.1.i326.i
  br i1 %or.cond505.i, label %bb58.i, label %bb22.i311.i, !prof !88

bb16.i335.i:                                      ; preds = %bb15.preheader.i333.i, %bb20.i343.i
  %src.sroa.0.259.i336.i = phi ptr [ %rest.05.i345.i, %bb20.i343.i ], [ %src.sroa.0.0.i309.i, %bb15.preheader.i333.i ]
  %src.sroa.15.258.i337.i = phi i64 [ %rest.16.i344.i, %bb20.i343.i ], [ %src.sroa.15.0.i307.i, %bb15.preheader.i333.i ]
  %result.sroa.0.257.i338.i = phi i64 [ %313, %bb20.i343.i ], [ 0, %bb15.preheader.i333.i ]
  %_20.i339.i = load i8, ptr %src.sroa.0.259.i336.i, align 1, !alias.scope !296, !noalias !299, !noundef !4
  %_19.i340.i = zext i8 %_20.i339.i to i32
  %311 = add nsw i32 %_19.i340.i, -48
  %_14.i46.i341.i = icmp ult i32 %311, 10
  br i1 %_14.i46.i341.i, label %bb20.i343.i, label %bb58.i

bb20.i343.i:                                      ; preds = %bb16.i335.i
  %312 = mul i64 %result.sroa.0.257.i338.i, 10
  %rest.16.i344.i = add nsw i64 %src.sroa.15.258.i337.i, -1
  %rest.05.i345.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i336.i, i64 1
  %_24.i346.i = zext nneg i32 %311 to i64
  %313 = add i64 %312, %_24.i346.i
  %_13.not.i347.i = icmp eq i64 %rest.16.i344.i, 0
  br i1 %_13.not.i347.i, label %bb58.i, label %bb16.i335.i

bb58.i:                                           ; preds = %bb33.i321.i, %bb23.i316.i, %bb22.i311.i, %bb20.i343.i, %bb16.i335.i, %bb15.preheader.i333.i, %bb7.i301.i, %bb7.i301.i, %bb52.i200, %bb51.i199, %bb44.i198
  %minor.sroa.0.0489494.i = phi i64 [ %minor.sroa.0.0.i, %bb52.i200 ], [ %minor.sroa.0.0.i, %bb7.i301.i ], [ %minor.sroa.0.0.i, %bb7.i301.i ], [ %minor.sroa.0.0.i, %bb15.preheader.i333.i ], [ %minor.sroa.0.0.i, %bb51.i199 ], [ 0, %bb44.i198 ], [ %minor.sroa.0.0.i, %bb16.i335.i ], [ %minor.sroa.0.0.i, %bb20.i343.i ], [ %minor.sroa.0.0.i, %bb22.i311.i ], [ %minor.sroa.0.0.i, %bb23.i316.i ], [ %minor.sroa.0.0.i, %bb33.i321.i ]
  %patch.sroa.0.0.i = phi i64 [ %_0.sroa.4.1.i260.i, %bb52.i200 ], [ 0, %bb7.i301.i ], [ 0, %bb7.i301.i ], [ 0, %bb15.preheader.i333.i ], [ 0, %bb51.i199 ], [ 0, %bb44.i198 ], [ %313, %bb20.i343.i ], [ 0, %bb16.i335.i ], [ 0, %bb33.i321.i ], [ 0, %bb23.i316.i ], [ %result.sroa.0.0.i312.i, %bb22.i311.i ]
  %314 = icmp eq i64 %189, 0
  br i1 %314, label %bb7.i201, label %bb2.i.i.i4.i.i352.i

bb2.i.i.i4.i.i352.i:                              ; preds = %bb58.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.0.0.copyload.i, i64 noundef %189, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb7.i201

bb7.i201:                                         ; preds = %bb2.i.i.i4.i.i352.i, %bb58.i
  %_29.i = add i64 %minor.sroa.0.0489494.i, %major.sroa.0.0.i
  %_28.i = add i64 %_29.i, %patch.sroa.0.0.i
  %315 = icmp eq i64 %_34.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %315, label %bb34, label %bb2.i.i.i4.i355.i

bb2.i.i.i4.i355.i:                                ; preds = %bb7.i201
  %316 = icmp ne ptr %_34.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %316)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.6.0.copyload.i, i64 noundef %_34.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb34

terminate.i180:                                   ; preds = %cleanup.body.i179
  %317 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb17.sink.split.i:                                ; preds = %bb2.i.i.i4.i364.i, %cleanup.i.i193
  %.pn435.ph.i = phi { ptr, i32 } [ %320, %bb2.i.i.i4.i364.i ], [ %215, %cleanup.i.i193 ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.0.0.copyload.i, i64 noundef %189, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb17.i185

bb17.i185:                                        ; preds = %bb18.i184, %bb17.sink.split.i, %cleanup.i.i193
  %.pn435.i = phi { ptr, i32 } [ %215, %cleanup.i.i193 ], [ %320, %bb18.i184 ], [ %.pn435.ph.i, %bb17.sink.split.i ]
  %318 = icmp eq i64 %_34.sroa.4.sroa.5.0.copyload.i, 0
  br i1 %318, label %bb136, label %bb2.i.i.i4.i361.i

bb2.i.i.i4.i361.i:                                ; preds = %bb17.i185
  %319 = icmp ne ptr %_34.sroa.4.sroa.6.0.copyload.i, null
  call void @llvm.assume(i1 %319)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34.sroa.4.sroa.6.0.copyload.i, i64 noundef %_34.sroa.4.sroa.5.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb136

bb18.i184:                                        ; preds = %bb26.i183
  %320 = landingpad { ptr, i32 }
          cleanup
  %321 = icmp eq i64 %189, 0
  br i1 %321, label %bb17.i185, label %bb2.i.i.i4.i364.i

bb2.i.i.i4.i364.i:                                ; preds = %bb18.i184
  %322 = icmp ne ptr %_34.sroa.4.sroa.0.0.copyload.i, null
  call void @llvm.assume(i1 %322)
  br label %bb17.sink.split.i

bb34:                                             ; preds = %bb2.i.i.i4.i355.i, %bb7.i201
  %_44 = icmp ult i64 %_28.i, 30142
  br i1 %_44, label %bb36, label %bb39

bb39:                                             ; preds = %bb2.i.i.i4.i39.i, %bb16.i, %.noexc254, %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit, %bb34
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_47)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_47, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_154439d6e8351f7172ea58cb90d2dd09, i64 noundef 30)
          to label %bb40 unwind label %cleanup9.loopexit.split-lp

bb36:                                             ; preds = %bb34
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cfg.i)
  store ptr @alloc_ccedf80c3ce4e46e2ff8efee35ec798b, ptr %cfg.i, align 8, !noalias !301
  %323 = getelementptr inbounds nuw i8, ptr %cfg.i, i64 8
  store i64 23, ptr %323, align 8, !noalias !301
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i), !noalias !301
  store ptr %cfg.i, ptr %args1.i, align 8, !noalias !301
  %_15.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args1.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx.i, align 8, !noalias !301
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1.i)
          to label %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit unwind label %cleanup9.loopexit.split-lp

_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit: ; preds = %bb36
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i), !noalias !301
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cfg.i)
  br label %bb39

bb40:                                             ; preds = %bb39
  %_236 = load i64, ptr %_47, align 8, !range !3, !noundef !4
  %musl_v1_2_3 = icmp eq i64 %_236, 0
  call void @llvm.experimental.noalias.scope.decl(metadata !304)
  %324 = getelementptr inbounds nuw i8, ptr %_47, i64 8
  %.val.i261 = load i64, ptr %324, align 8, !alias.scope !304
  br i1 %musl_v1_2_3, label %bb2.i266, label %bb3.i262

bb2.i266:                                         ; preds = %bb40
  %325 = icmp eq i64 %.val.i261, 0
  br i1 %325, label %bb41, label %bb1.sink.split.i264

bb3.i262:                                         ; preds = %bb40
  switch i64 %.val.i261, label %bb1.sink.split.i264 [
    i64 -9223372036854775808, label %bb41
    i64 0, label %bb41
  ]

bb1.sink.split.i264:                              ; preds = %bb3.i262, %bb2.i266
  %326 = getelementptr inbounds nuw i8, ptr %_47, i64 16
  %.val3.i265 = load ptr, ptr %326, align 8, !alias.scope !304, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i265, i64 noundef %.val.i261, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !304
  br label %bb41

bb41:                                             ; preds = %bb1.sink.split.i264, %bb3.i262, %bb3.i262, %bb2.i266
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_47)
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_64de700dc0d3712bf4f0fd23fc9b97f6, ptr noundef nonnull inttoptr (i64 117 to ptr))
          to label %bb42 unwind label %cleanup9.loopexit.split-lp

bb42:                                             ; preds = %bb41
  br i1 %musl_v1_2_3, label %bb45, label %bb43

bb43:                                             ; preds = %bb42
  %_3.not.i = icmp eq i64 %target_arch.sroa.12.0, 11
  br i1 %_3.not.i, label %bb164, label %bb44

bb45:                                             ; preds = %bb165, %bb164, %bb42
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cfg.i272)
  store ptr @alloc_513019cde2cbfb4427cb8f1afc437e08, ptr %cfg.i272, align 8, !noalias !307
  %327 = getelementptr inbounds nuw i8, ptr %cfg.i272, i64 8
  store i64 11, ptr %327, align 8, !noalias !307
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i270), !noalias !307
  store ptr %cfg.i272, ptr %args1.i270, align 8, !noalias !307
  %_15.sroa.4.0..sroa_idx.i276 = getelementptr inbounds nuw i8, ptr %args1.i270, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx.i276, align 8, !noalias !307
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1.i270)
          to label %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit279 unwind label %cleanup9.loopexit.split-lp

_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit279: ; preds = %bb45
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i270), !noalias !307
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cfg.i272)
  br label %bb46

bb164:                                            ; preds = %bb43
  %328 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(11) %target_arch.sroa.6.0, ptr noundef nonnull dereferenceable(11) @alloc_be0c7e2eb8d81d67a6db9a856123bb7e, i64 range(i64 0, -9223372036854775808) 11), !alias.scope !310
  %329 = icmp eq i32 %328, 0
  br i1 %329, label %bb45, label %bb44

bb44:                                             ; preds = %bb43, %bb164
  %_3.not.i280 = icmp eq i64 %target_env.sroa.11.0, 4
  br i1 %_3.not.i280, label %bb165, label %bb46

bb165:                                            ; preds = %bb44
  %330 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %target_env.sroa.6.0, ptr noundef nonnull dereferenceable(4) @alloc_830cd488b6068638e05ed5b0c299b4af, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !314
  %331 = icmp eq i32 %330, 0
  br i1 %331, label %bb45, label %bb46

bb46:                                             ; preds = %bb44, %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit279, %bb165
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_54)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_54, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_508b13eade4b92efdda744da70d08ff7, i64 noundef 36)
          to label %bb47 unwind label %cleanup9.loopexit.split-lp

bb47:                                             ; preds = %bb46
  %_264 = load i64, ptr %_54, align 8, !range !3, !noundef !4
  %linux_time_bits64 = icmp eq i64 %_264, 0
  call void @llvm.experimental.noalias.scope.decl(metadata !318)
  %332 = getelementptr inbounds nuw i8, ptr %_54, i64 8
  %.val.i288 = load i64, ptr %332, align 8, !alias.scope !318
  br i1 %linux_time_bits64, label %bb2.i293, label %bb3.i289

bb2.i293:                                         ; preds = %bb47
  %333 = icmp eq i64 %.val.i288, 0
  br i1 %333, label %bb48, label %bb1.sink.split.i291

bb3.i289:                                         ; preds = %bb47
  switch i64 %.val.i288, label %bb1.sink.split.i291 [
    i64 -9223372036854775808, label %bb48
    i64 0, label %bb48
  ]

bb1.sink.split.i291:                              ; preds = %bb3.i289, %bb2.i293
  %334 = getelementptr inbounds nuw i8, ptr %_54, i64 16
  %.val3.i292 = load ptr, ptr %334, align 8, !alias.scope !318, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i292, i64 noundef %.val.i288, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !318
  br label %bb48

bb48:                                             ; preds = %bb1.sink.split.i291, %bb3.i289, %bb3.i289, %bb2.i293
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_54)
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_b745d31eb2902e488a48adfdc7a9757f, ptr noundef nonnull inttoptr (i64 129 to ptr))
          to label %bb49 unwind label %cleanup9.loopexit.split-lp

bb49:                                             ; preds = %bb48
  br i1 %linux_time_bits64, label %bb50, label %bb51

bb51:                                             ; preds = %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit304, %bb49
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_e099490f9865495bf255e49aa607a840, ptr noundef nonnull inttoptr (i64 135 to ptr))
          to label %bb52 unwind label %cleanup9.loopexit.split-lp

bb50:                                             ; preds = %bb49
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cfg.i297)
  store ptr @alloc_681b6f9e783332c8e0b8ad7b08df1498, ptr %cfg.i297, align 8, !noalias !321
  %335 = getelementptr inbounds nuw i8, ptr %cfg.i297, i64 8
  store i64 17, ptr %335, align 8, !noalias !321
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i295), !noalias !321
  store ptr %cfg.i297, ptr %args1.i295, align 8, !noalias !321
  %_15.sroa.4.0..sroa_idx.i301 = getelementptr inbounds nuw i8, ptr %args1.i295, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx.i301, align 8, !noalias !321
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1.i295)
          to label %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit304 unwind label %cleanup9.loopexit.split-lp

_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit304: ; preds = %bb50
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i295), !noalias !321
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cfg.i297)
  br label %bb51

bb52:                                             ; preds = %bb51
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_3e0db014760956dcb5153ae64d55b081, ptr noundef nonnull inttoptr (i64 121 to ptr))
          to label %bb53 unwind label %cleanup9.loopexit.split-lp

bb53:                                             ; preds = %bb52
  %_3.not.i305 = icmp eq i64 %target_env.sroa.11.0, 3
  br i1 %_3.not.i305, label %bb166, label %bb101

bb166:                                            ; preds = %bb53
  %336 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %target_env.sroa.6.0, ptr noundef nonnull dereferenceable(3) @alloc_772e61a39199df4134c467e272d2cf4b, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !324
  %337 = icmp eq i32 %336, 0
  %or.cond610 = select i1 %337, i1 %target_os.sroa.10.0, i1 false
  br i1 %or.cond610, label %bb167, label %bb101

bb101:                                            ; preds = %bb53, %bb100, %bb170, %bb169, %bb168, %bb167, %bb166
  br i1 %libc_ci, label %bb102, label %bb103

bb167:                                            ; preds = %bb166
  %338 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %target_os.sroa.6.0, ptr noundef nonnull dereferenceable(5) @alloc_70a1e7dc3879e83c39c209c1ae5f1722, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !328
  %339 = icmp eq i32 %338, 0
  %or.cond611 = select i1 %339, i1 %target_ptr_width.sroa.10.0, i1 false
  br i1 %or.cond611, label %bb168, label %bb101

bb168:                                            ; preds = %bb167
  %340 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %target_ptr_width.sroa.6.0, ptr noundef nonnull dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !332
  %341 = icmp eq i32 %340, 0
  br i1 %341, label %bb56, label %bb101

bb56:                                             ; preds = %bb168
  switch i64 %target_arch.sroa.12.0, label %bb58 [
    i64 7, label %bb169
    i64 6, label %bb170
  ]

bb169:                                            ; preds = %bb56
  %342 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %target_arch.sroa.6.0, ptr noundef nonnull dereferenceable(7) @alloc_22a6d0e24a3ac3ed7016f4ca447b0cea, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !336
  %343 = icmp eq i32 %342, 0
  br i1 %343, label %bb101, label %bb58

bb170:                                            ; preds = %bb56
  %344 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %target_arch.sroa.6.0, ptr noundef nonnull dereferenceable(6) @alloc_4a29a4faa0904cd7ff982831f2813e90, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !340
  %345 = icmp eq i32 %344, 0
  br i1 %345, label %bb101, label %bb58

bb58:                                             ; preds = %bb56, %bb169, %bb170
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %defaultbits)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22, !noalias !344
; call __rustc::__rust_alloc
  %346 = call noundef dereferenceable_or_null(2) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 2, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !344
  %347 = icmp eq ptr %346, null
  br i1 %347, label %bb172.invoke, label %bb173

bb172.invoke:                                     ; preds = %bb6.i.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i, %bb58
  %348 = phi i64 [ 1, %bb58 ], [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ 0, %bb6.i.i.i.i ]
  %349 = phi i64 [ 2, %bb58 ], [ %accum.sroa.0.0.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ %accum.sroa.0.0.i.i, %bb6.i.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %348, i64 %349) #26
          to label %bb172.cont unwind label %cleanup9.loopexit.split-lp

bb172.cont:                                       ; preds = %bb172.invoke
  unreachable

bb173:                                            ; preds = %bb58
  store i16 12851, ptr %346, align 1
  store i64 2, ptr %defaultbits, align 8
  %_339.sroa.4.0.defaultbits.sroa_idx = getelementptr inbounds nuw i8, ptr %defaultbits, i64 8
  store ptr %346, ptr %_339.sroa.4.0.defaultbits.sroa_idx, align 8
  %_339.sroa.6.0.defaultbits.sroa_idx = getelementptr inbounds nuw i8, ptr %defaultbits, i64 16
  store i64 2, ptr %_339.sroa.6.0.defaultbits.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_70)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_71)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_71, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a81a2677393ac2707db2f683d48ac6b7, i64 noundef 32)
          to label %bb59 unwind label %cleanup12

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336: ; preds = %bb153, %cleanup13, %bb2.i.i.i4.i.i475, %bb152, %bb2.i.i.i4.i.i417, %bb127, %bb151, %cleanup12
  %.pn35.pn.pn = phi { ptr, i32 } [ %350, %cleanup12 ], [ %.pn33537, %bb151 ], [ %.pn33537746, %bb153 ], [ %351, %cleanup13 ], [ %393, %bb127 ], [ %393, %bb2.i.i.i4.i.i417 ], [ %.pn33537, %bb152 ], [ %.pn33537, %bb2.i.i.i4.i.i475 ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %346, i64 noundef 2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb136

cleanup12:                                        ; preds = %bb173
  %350 = landingpad { ptr, i32 }
          cleanup
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb59:                                             ; preds = %bb173
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_72)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_72, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9329be348e7e4f3c8cc453f36256cbfd, i64 noundef 39)
          to label %bb60 unwind label %cleanup13

cleanup13:                                        ; preds = %bb59
  %351 = landingpad { ptr, i32 }
          cleanup
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %_71) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb60:                                             ; preds = %bb59
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_70, ptr noundef nonnull align 8 dereferenceable(32) %_71, i64 32, i1 false)
  %352 = getelementptr inbounds nuw i8, ptr %_70, i64 32
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %352, ptr noundef nonnull align 8 dereferenceable(32) %_72, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_72)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_71)
  %_75 = load i64, ptr %_70, align 8, !range !3, !noundef !4
  %353 = trunc nuw i64 %_75 to i1
  %_74 = load i64, ptr %352, align 8, !range !3, !noundef !4
  %354 = trunc nuw i64 %_74 to i1
  br i1 %353, label %bb63, label %bb62

bb63:                                             ; preds = %bb60
  br i1 %354, label %bb66, label %bb64

bb62:                                             ; preds = %bb60
  br i1 %354, label %bb65, label %bb67.invoke, !prof !347

bb65:                                             ; preds = %bb62
  %355 = getelementptr inbounds nuw i8, ptr %_70, i64 8
  %356 = getelementptr inbounds nuw i8, ptr %_70, i64 16
  %_372 = load ptr, ptr %356, align 8, !nonnull !4, !noundef !4
  %357 = getelementptr inbounds nuw i8, ptr %_70, i64 24
  %_371 = load i64, ptr %357, align 8, !noundef !4
  %_3.not.i339 = icmp eq i64 %_371, 2
  br i1 %_3.not.i339, label %bb175, label %bb67.invoke, !prof !348

bb67.invoke:                                      ; preds = %bb176, %bb65, %bb62
  %358 = phi ptr [ @alloc_14fc90d5f706773754d40e4dccd34450, %bb62 ], [ @alloc_4a184034f37022296f6ca89b4adb3768, %bb65 ], [ @alloc_4a184034f37022296f6ca89b4adb3768, %bb176 ]
  %359 = phi ptr [ inttoptr (i64 185 to ptr), %bb62 ], [ inttoptr (i64 137 to ptr), %bb65 ], [ inttoptr (i64 137 to ptr), %bb176 ]
  %360 = phi ptr [ @alloc_659c02f340995e0c4d05cbbdc231b829, %bb62 ], [ @alloc_7cba2f35f97c1b56c6e6ba52f23cb86b, %bb65 ], [ @alloc_7cba2f35f97c1b56c6e6ba52f23cb86b, %bb176 ]
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull %358, ptr noundef nonnull %359, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %360) #26
          to label %bb67.cont unwind label %bb155

bb67.cont:                                        ; preds = %bb67.invoke
  unreachable

bb157:                                            ; preds = %bb82, %bb66
  %lpad.thr_comm.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb156

unreachable:                                      ; preds = %bb31, %bb82
  unreachable

bb175:                                            ; preds = %bb65
  %361 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_372, ptr noundef nonnull dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !349
  %362 = icmp eq i32 %361, 0
  br i1 %362, label %bb70, label %bb176

bb70:                                             ; preds = %bb175
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %tb)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %tb, ptr noundef nonnull align 8 dereferenceable(24) %355, i64 24, i1 false)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_84)
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_84, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %tb)
          to label %bb72 unwind label %cleanup16

bb176:                                            ; preds = %bb175
  %363 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_372, ptr noundef nonnull dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !353
  %364 = icmp eq i32 %363, 0
  br i1 %364, label %bb75, label %bb67.invoke, !prof !84

bb75:                                             ; preds = %bb176
  %tb1.sroa.0.0.copyload = load i64, ptr %355, align 8
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_100, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %defaultbits)
          to label %bb77 unwind label %cleanup15

cleanup15:                                        ; preds = %bb75
  %365 = landingpad { ptr, i32 }
          cleanup
  %366 = icmp eq i64 %tb1.sroa.0.0.copyload, 0
  br i1 %366, label %bb153, label %bb2.i.i.i4.i.i349

bb2.i.i.i4.i.i349:                                ; preds = %cleanup15
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_372, i64 noundef %tb1.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb153

bb77:                                             ; preds = %bb75
  store i64 %tb1.sroa.0.0.copyload, ptr %_98, align 8
  %_90.sroa.5.0._98.sroa_idx = getelementptr inbounds nuw i8, ptr %_98, i64 8
  store ptr %_372, ptr %_90.sroa.5.0._98.sroa_idx, align 8
  %_90.sroa.6.0._98.sroa_idx = getelementptr inbounds nuw i8, ptr %_98, i64 16
  store i64 2, ptr %_90.sroa.6.0._98.sroa_idx, align 8
  br label %bb148.thread

bb148.thread:                                     ; preds = %bb77, %bb74
  %timebits.sroa.10.0.copyload = phi i64 [ %timebits.sroa.10.0.copyload.pre, %bb74 ], [ 2, %bb77 ]
  %timebits.sroa.5.0.copyload = phi ptr [ %timebits.sroa.5.0.copyload.pre, %bb74 ], [ %_372, %bb77 ]
  %timebits.sroa.0.0.copyload = phi i64 [ %timebits.sroa.0.0.copyload.pre, %bb74 ], [ %tb1.sroa.0.0.copyload, %bb77 ]
  %filebits.sroa.0.0.copyload = load i64, ptr %_100, align 8
  %filebits.sroa.5.0._100.sroa_idx = getelementptr inbounds nuw i8, ptr %_100, i64 8
  %filebits.sroa.5.0.copyload = load ptr, ptr %filebits.sroa.5.0._100.sroa_idx, align 8
  %filebits.sroa.10.0._100.sroa_idx = getelementptr inbounds nuw i8, ptr %_100, i64 16
  %filebits.sroa.10.0.copyload = load i64, ptr %filebits.sroa.10.0._100.sroa_idx, align 8
  br label %bb147

bb131:                                            ; preds = %bb2.i.i.i4.i.i353, %cleanup17, %cleanup16
  %.pn31 = phi { ptr, i32 } [ %369, %cleanup16 ], [ %370, %cleanup17 ], [ %370, %bb2.i.i.i4.i.i353 ]
  %tb.val = load i64, ptr %tb, align 8
  %367 = icmp eq i64 %tb.val, 0
  br i1 %367, label %bb153, label %bb2.i.i.i4.i.i351

bb2.i.i.i4.i.i351:                                ; preds = %bb131
  %368 = getelementptr inbounds nuw i8, ptr %tb, i64 8
  %tb.val73 = load ptr, ptr %368, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %tb.val73, i64 noundef %tb.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb153

cleanup16:                                        ; preds = %bb70
  %369 = landingpad { ptr, i32 }
          cleanup
  br label %bb131

bb72:                                             ; preds = %bb70
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_100, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %tb)
          to label %bb73 unwind label %cleanup17

cleanup17:                                        ; preds = %bb72
  %370 = landingpad { ptr, i32 }
          cleanup
  %_84.val = load i64, ptr %_84, align 8
  %371 = icmp eq i64 %_84.val, 0
  br i1 %371, label %bb131, label %bb2.i.i.i4.i.i353

bb2.i.i.i4.i.i353:                                ; preds = %cleanup17
  %372 = getelementptr inbounds nuw i8, ptr %_84, i64 8
  %_84.val74 = load ptr, ptr %372, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_84.val74, i64 noundef %_84.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb131

bb73:                                             ; preds = %bb72
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_98, ptr noundef nonnull align 8 dereferenceable(24) %_84, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_84)
  %tb.val75 = load i64, ptr %tb, align 8
  %373 = icmp eq i64 %tb.val75, 0
  br i1 %373, label %bb74, label %bb2.i.i.i4.i.i355

bb2.i.i.i4.i.i355:                                ; preds = %bb73
  %374 = getelementptr inbounds nuw i8, ptr %tb, i64 8
  %tb.val76 = load ptr, ptr %374, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %tb.val76, i64 noundef %tb.val75, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb74

bb74:                                             ; preds = %bb2.i.i.i4.i.i355, %bb73
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %tb)
  %timebits.sroa.0.0.copyload.pre = load i64, ptr %_98, align 8
  %timebits.sroa.5.0._98.sroa_idx.phi.trans.insert = getelementptr inbounds nuw i8, ptr %_98, i64 8
  %timebits.sroa.5.0.copyload.pre = load ptr, ptr %timebits.sroa.5.0._98.sroa_idx.phi.trans.insert, align 8
  %timebits.sroa.10.0._98.sroa_idx.phi.trans.insert = getelementptr inbounds nuw i8, ptr %_98, i64 16
  %timebits.sroa.10.0.copyload.pre = load i64, ptr %timebits.sroa.10.0._98.sroa_idx.phi.trans.insert, align 8
  br label %bb148.thread

bb66:                                             ; preds = %bb63
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_78)
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_78, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %defaultbits)
          to label %bb68 unwind label %bb157

bb64:                                             ; preds = %bb63
  %375 = getelementptr inbounds nuw i8, ptr %_70, i64 40
  %376 = getelementptr inbounds nuw i8, ptr %_70, i64 48
  %_363 = load ptr, ptr %376, align 8, !nonnull !4, !noundef !4
  %377 = getelementptr inbounds nuw i8, ptr %_70, i64 56
  %_362 = load i64, ptr %377, align 8, !noundef !4
  %_3.not.i357 = icmp eq i64 %_362, 2
  br i1 %_3.not.i357, label %bb174, label %bb82, !prof !348

bb174:                                            ; preds = %bb64
  %378 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_363, ptr noundef nonnull dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !357
  %379 = icmp eq i32 %378, 0
  br i1 %379, label %bb81, label %bb177

bb177:                                            ; preds = %bb174
  %380 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_363, ptr noundef nonnull dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !361
  %381 = icmp eq i32 %380, 0
  br i1 %381, label %bb81, label %bb82, !prof !84

bb82:                                             ; preds = %bb64, %bb177
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_12b500c16d6393901618de0cf55c3e6c, ptr noundef nonnull inttoptr (i64 151 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f8b23ce9691cdedbe96412eba55673b0) #26
          to label %unreachable unwind label %bb157

bb81:                                             ; preds = %bb177, %bb174
  %fb.sroa.0.0.copyload = load i64, ptr %375, align 8
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_98, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %defaultbits)
          to label %bb83 unwind label %cleanup18

cleanup18:                                        ; preds = %bb81
  %382 = landingpad { ptr, i32 }
          cleanup
  %383 = icmp eq i64 %fb.sroa.0.0.copyload, 0
  br i1 %383, label %bb156, label %bb2.i.i.i4.i.i367

bb2.i.i.i4.i.i367:                                ; preds = %cleanup18
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_363, i64 noundef %fb.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb156

bb83:                                             ; preds = %bb81
  store i64 %fb.sroa.0.0.copyload, ptr %_100, align 8
  %fb.sroa.5.0._100.sroa_idx = getelementptr inbounds nuw i8, ptr %_100, i64 8
  store ptr %_363, ptr %fb.sroa.5.0._100.sroa_idx, align 8
  %fb.sroa.6.0._100.sroa_idx = getelementptr inbounds nuw i8, ptr %_100, i64 16
  store i64 2, ptr %fb.sroa.6.0._100.sroa_idx, align 8
  br label %bb148

bb68:                                             ; preds = %bb66
; invoke <alloc::string::String as core::clone::Clone>::clone
  invoke void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_100, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %defaultbits)
          to label %bb69 unwind label %cleanup19

cleanup19:                                        ; preds = %bb68
  %384 = landingpad { ptr, i32 }
          cleanup
  %_78.val = load i64, ptr %_78, align 8
  %385 = icmp eq i64 %_78.val, 0
  br i1 %385, label %bb156, label %bb2.i.i.i4.i.i369

bb2.i.i.i4.i.i369:                                ; preds = %cleanup19
  %386 = getelementptr inbounds nuw i8, ptr %_78, i64 8
  %_78.val52 = load ptr, ptr %386, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_78.val52, i64 noundef %_78.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb156

bb69:                                             ; preds = %bb68
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_98, ptr noundef nonnull align 8 dereferenceable(24) %_78, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_78)
  %filebits.sroa.0.0.copyload559.pre = load i64, ptr %_100, align 8
  %filebits.sroa.5.0._100.sroa_idx560.phi.trans.insert = getelementptr inbounds nuw i8, ptr %_100, i64 8
  %filebits.sroa.5.0.copyload561.pre = load ptr, ptr %filebits.sroa.5.0._100.sroa_idx560.phi.trans.insert, align 8
  %filebits.sroa.10.0._100.sroa_idx562.phi.trans.insert = getelementptr inbounds nuw i8, ptr %_100, i64 16
  %filebits.sroa.10.0.copyload563.pre = load i64, ptr %filebits.sroa.10.0._100.sroa_idx562.phi.trans.insert, align 8
  br label %bb148

bb148:                                            ; preds = %bb83, %bb69
  %filebits.sroa.10.0.copyload563 = phi i64 [ %filebits.sroa.10.0.copyload563.pre, %bb69 ], [ 2, %bb83 ]
  %filebits.sroa.5.0.copyload561 = phi ptr [ %filebits.sroa.5.0.copyload561.pre, %bb69 ], [ %_363, %bb83 ]
  %filebits.sroa.0.0.copyload559 = phi i64 [ %filebits.sroa.0.0.copyload559.pre, %bb69 ], [ %fb.sroa.0.0.copyload, %bb83 ]
  %timebits.sroa.0.0.copyload554 = load i64, ptr %_98, align 8
  %timebits.sroa.5.0._98.sroa_idx555 = getelementptr inbounds nuw i8, ptr %_98, i64 8
  %timebits.sroa.5.0.copyload556 = load ptr, ptr %timebits.sroa.5.0._98.sroa_idx555, align 8
  %timebits.sroa.10.0._98.sroa_idx557 = getelementptr inbounds nuw i8, ptr %_98, i64 16
  %timebits.sroa.10.0.copyload558 = load i64, ptr %timebits.sroa.10.0._98.sroa_idx557, align 8
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %_70)
  %387 = trunc nuw i64 %_74 to i1
  br i1 %387, label %bb147, label %bb13.lr.ph.i

bb147:                                            ; preds = %bb148.thread, %bb148
  %timebits.sroa.0.0.copyload564740 = phi i64 [ %timebits.sroa.0.0.copyload, %bb148.thread ], [ %timebits.sroa.0.0.copyload554, %bb148 ]
  %timebits.sroa.5.0.copyload569736 = phi ptr [ %timebits.sroa.5.0.copyload, %bb148.thread ], [ %timebits.sroa.5.0.copyload556, %bb148 ]
  %timebits.sroa.10.0.copyload574734 = phi i64 [ %timebits.sroa.10.0.copyload, %bb148.thread ], [ %timebits.sroa.10.0.copyload558, %bb148 ]
  %filebits.sroa.0.0.copyload576730 = phi i64 [ %filebits.sroa.0.0.copyload, %bb148.thread ], [ %filebits.sroa.0.0.copyload559, %bb148 ]
  %filebits.sroa.5.0.copyload580726 = phi ptr [ %filebits.sroa.5.0.copyload, %bb148.thread ], [ %filebits.sroa.5.0.copyload561, %bb148 ]
  %filebits.sroa.10.0.copyload584724 = phi i64 [ %filebits.sroa.10.0.copyload, %bb148.thread ], [ %filebits.sroa.10.0.copyload563, %bb148 ]
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %352)
  br label %bb13.lr.ph.i

bb13.lr.ph.i:                                     ; preds = %bb147, %bb148
  %timebits.sroa.0.0.copyload564739 = phi i64 [ %timebits.sroa.0.0.copyload564740, %bb147 ], [ %timebits.sroa.0.0.copyload554, %bb148 ]
  %timebits.sroa.5.0.copyload569735 = phi ptr [ %timebits.sroa.5.0.copyload569736, %bb147 ], [ %timebits.sroa.5.0.copyload556, %bb148 ]
  %timebits.sroa.10.0.copyload574733 = phi i64 [ %timebits.sroa.10.0.copyload574734, %bb147 ], [ %timebits.sroa.10.0.copyload558, %bb148 ]
  %filebits.sroa.0.0.copyload576729 = phi i64 [ %filebits.sroa.0.0.copyload576730, %bb147 ], [ %filebits.sroa.0.0.copyload559, %bb148 ]
  %filebits.sroa.5.0.copyload580725 = phi ptr [ %filebits.sroa.5.0.copyload580726, %bb147 ], [ %filebits.sroa.5.0.copyload561, %bb148 ]
  %filebits.sroa.10.0.copyload584723 = phi i64 [ %filebits.sroa.10.0.copyload584724, %bb147 ], [ %filebits.sroa.10.0.copyload563, %bb148 ]
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %_70)
  %388 = icmp ne ptr %filebits.sroa.5.0.copyload580725, null
  call void @llvm.assume(i1 %388)
  %_3.not.i.i.i.i.i = icmp eq i64 %filebits.sroa.10.0.copyload584723, 2
  br i1 %_3.not.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb91.invoke

bb2.i.i.i.i.i:                                    ; preds = %bb13.lr.ph.i
  %389 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, ptr noundef nonnull readonly align 1 dereferenceable(2) %filebits.sroa.5.0.copyload580725, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !365, !noalias !372
  %390 = icmp eq i32 %389, 0
  br i1 %390, label %bb13.lr.ph.i378, label %bb2.i.i.i.i.i.1

bb2.i.i.i.i.i.1:                                  ; preds = %bb2.i.i.i.i.i
  %391 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, ptr noundef nonnull readonly align 1 dereferenceable(2) %filebits.sroa.5.0.copyload580725, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !365, !noalias !372
  %392 = icmp eq i32 %391, 0
  br i1 %392, label %bb13.lr.ph.i378, label %bb91.invoke

bb2.i.i.i4.i.i373:                                ; preds = %cleanup21
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %filebits.sroa.5.0.copyload580725, i64 noundef %filebits.sroa.0.0.copyload576729, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb127

cleanup21:                                        ; preds = %bb91.invoke, %bb96, %bb94, %bb93
  %393 = landingpad { ptr, i32 }
          cleanup
  %394 = icmp eq i64 %filebits.sroa.0.0.copyload576729, 0
  br i1 %394, label %bb127, label %bb2.i.i.i4.i.i373

bb13.lr.ph.i378:                                  ; preds = %bb2.i.i.i.i.i.1, %bb2.i.i.i.i.i
  %395 = icmp ne ptr %timebits.sroa.5.0.copyload569735, null
  call void @llvm.assume(i1 %395)
  %_3.not.i.i.i.i.i385 = icmp eq i64 %timebits.sroa.10.0.copyload574733, 2
  br i1 %_3.not.i.i.i.i.i385, label %bb2.i.i.i.i.i392, label %bb91.invoke

bb2.i.i.i.i.i392:                                 ; preds = %bb13.lr.ph.i378
  %396 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, ptr noundef nonnull readonly align 1 dereferenceable(2) %timebits.sroa.5.0.copyload569735, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !376, !noalias !383
  %397 = icmp eq i32 %396, 0
  br i1 %397, label %bb180, label %bb2.i.i.i.i.i392.1

bb2.i.i.i.i.i392.1:                               ; preds = %bb2.i.i.i.i.i392
  %398 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, ptr noundef nonnull readonly align 1 dereferenceable(2) %timebits.sroa.5.0.copyload569735, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !376, !noalias !383
  %399 = icmp eq i32 %398, 0
  br i1 %399, label %bb180, label %bb91.invoke

bb180:                                            ; preds = %bb2.i.i.i.i.i392, %bb2.i.i.i.i.i392.1
  %400 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %filebits.sroa.5.0.copyload580725, ptr noundef nonnull dereferenceable(2) @alloc_8e020aace2b3cf2c6b8375c8868270b7, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !387
  %401 = icmp eq i32 %400, 0
  br i1 %401, label %bb181, label %bb182

bb181:                                            ; preds = %bb180
  %402 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %timebits.sroa.5.0.copyload569735, ptr noundef nonnull dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !391
  %403 = icmp eq i32 %402, 0
  br i1 %403, label %bb91.invoke, label %bb182, !prof !395

bb91.invoke:                                      ; preds = %bb181, %bb13.lr.ph.i, %bb2.i.i.i.i.i.1, %bb13.lr.ph.i378, %bb2.i.i.i.i.i392.1
  %404 = phi ptr [ @alloc_5b38d234a72f2da1e93ed696dcb5b073, %bb2.i.i.i.i.i392.1 ], [ @alloc_5b38d234a72f2da1e93ed696dcb5b073, %bb13.lr.ph.i378 ], [ @alloc_5b38d234a72f2da1e93ed696dcb5b073, %bb2.i.i.i.i.i.1 ], [ @alloc_5b38d234a72f2da1e93ed696dcb5b073, %bb13.lr.ph.i ], [ @alloc_c8a65b5fe9f8c8ff66f0add8177e4932, %bb181 ]
  %405 = phi ptr [ inttoptr (i64 237 to ptr), %bb2.i.i.i.i.i392.1 ], [ inttoptr (i64 237 to ptr), %bb13.lr.ph.i378 ], [ inttoptr (i64 237 to ptr), %bb2.i.i.i.i.i.1 ], [ inttoptr (i64 237 to ptr), %bb13.lr.ph.i ], [ inttoptr (i64 203 to ptr), %bb181 ]
  %406 = phi ptr [ @alloc_1d8dfd7aeb6a932d2b8d6d6490c15ab7, %bb2.i.i.i.i.i392.1 ], [ @alloc_1d8dfd7aeb6a932d2b8d6d6490c15ab7, %bb13.lr.ph.i378 ], [ @alloc_1d8dfd7aeb6a932d2b8d6d6490c15ab7, %bb2.i.i.i.i.i.1 ], [ @alloc_1d8dfd7aeb6a932d2b8d6d6490c15ab7, %bb13.lr.ph.i ], [ @alloc_da02dd9cd40c96b9368bc58a598563e2, %bb181 ]
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull %404, ptr noundef nonnull %405, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %406) #26
          to label %bb91.cont unwind label %cleanup21

bb91.cont:                                        ; preds = %bb91.invoke
  unreachable

bb182:                                            ; preds = %bb180, %bb181
  %407 = call i32 @memcmp(ptr nonnull readonly align 1 %timebits.sroa.5.0.copyload569735, ptr nonnull @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) %timebits.sroa.10.0.copyload574733), !alias.scope !396
  %408 = icmp eq i32 %407, 0
  br i1 %408, label %bb93, label %bb183

bb93:                                             ; preds = %bb182
; invoke build_script_build::set_cfg
  invoke fastcc void @_RNvCs2XfqmVweRya_18build_script_build7set_cfg(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_681b6f9e783332c8e0b8ad7b08df1498, i64 noundef 17)
          to label %bb94 unwind label %cleanup21

bb94:                                             ; preds = %bb93
; invoke build_script_build::set_cfg
  invoke fastcc void @_RNvCs2XfqmVweRya_18build_script_build7set_cfg(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a4772b3acfc19af28fefe691db64c6aa, i64 noundef 15)
          to label %bb183 unwind label %cleanup21

bb183:                                            ; preds = %bb182, %bb94
  %409 = call i32 @memcmp(ptr nonnull readonly align 1 %filebits.sroa.5.0.copyload580725, ptr nonnull @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) %filebits.sroa.10.0.copyload584723), !alias.scope !400
  %410 = icmp eq i32 %409, 0
  br i1 %410, label %bb96, label %bb97

bb97:                                             ; preds = %bb96, %bb183
  %411 = icmp eq i64 %filebits.sroa.0.0.copyload576729, 0
  br i1 %411, label %bb98, label %bb2.i.i.i4.i.i415

bb2.i.i.i4.i.i415:                                ; preds = %bb97
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %filebits.sroa.5.0.copyload580725, i64 noundef %filebits.sroa.0.0.copyload576729, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb98

bb96:                                             ; preds = %bb183
; invoke build_script_build::set_cfg
  invoke fastcc void @_RNvCs2XfqmVweRya_18build_script_build7set_cfg(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4052f5f320831d7a280bd8ee23d7c161, i64 noundef 22)
          to label %bb97 unwind label %cleanup21

bb127:                                            ; preds = %bb2.i.i.i4.i.i373, %cleanup21
  %412 = icmp eq i64 %timebits.sroa.0.0.copyload564739, 0
  br i1 %412, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336, label %bb2.i.i.i4.i.i417

bb2.i.i.i4.i.i417:                                ; preds = %bb127
  %413 = icmp ne ptr %timebits.sroa.5.0.copyload569735, null
  call void @llvm.assume(i1 %413)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %timebits.sroa.5.0.copyload569735, i64 noundef %timebits.sroa.0.0.copyload564739, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb98:                                             ; preds = %bb2.i.i.i4.i.i415, %bb97
  %414 = icmp eq i64 %timebits.sroa.0.0.copyload564739, 0
  br i1 %414, label %bb100, label %bb2.i.i.i4.i.i419

bb2.i.i.i4.i.i419:                                ; preds = %bb98
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %timebits.sroa.5.0.copyload569735, i64 noundef %timebits.sroa.0.0.copyload564739, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb100

bb100:                                            ; preds = %bb98, %bb2.i.i.i4.i.i419
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %346, i64 noundef 2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %defaultbits)
  br label %bb101

bb103:                                            ; preds = %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit432, %bb101
  br i1 %rustc_dep_of_std, label %bb104, label %bb105

bb102:                                            ; preds = %bb101
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cfg.i425)
  store ptr @alloc_e051788150efb5e0f212c696366647c3, ptr %cfg.i425, align 8, !noalias !404
  %415 = getelementptr inbounds nuw i8, ptr %cfg.i425, i64 8
  store i64 18, ptr %415, align 8, !noalias !404
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i423), !noalias !404
  store ptr %cfg.i425, ptr %args1.i423, align 8, !noalias !404
  %_15.sroa.4.0..sroa_idx.i429 = getelementptr inbounds nuw i8, ptr %args1.i423, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx.i429, align 8, !noalias !404
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1.i423)
          to label %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit432 unwind label %cleanup9.loopexit.split-lp

_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit432: ; preds = %bb102
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i423), !noalias !404
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cfg.i425)
  br label %bb103

bb105:                                            ; preds = %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit443, %bb103
  %_127 = icmp ugt i32 %_0.sroa.8.0.insert.insert.i.i, 79
  br i1 %_127, label %bb107.preheader, label %bb120

bb107.preheader:                                  ; preds = %bb105
  %_136.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args2, i64 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr @alloc_0932325d29f8c848cece173911e7c4a6, ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109 unwind label %cleanup9.loopexit

bb104:                                            ; preds = %bb103
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cfg.i435)
  store ptr @alloc_e300d0c2c56fc656630ece49b293f3f6, ptr %cfg.i435, align 8, !noalias !407
  %416 = getelementptr inbounds nuw i8, ptr %cfg.i435, i64 8
  store i64 17, ptr %416, align 8, !noalias !407
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i433), !noalias !407
  store ptr %cfg.i435, ptr %args1.i433, align 8, !noalias !407
  %_15.sroa.4.0..sroa_idx.i440 = getelementptr inbounds nuw i8, ptr %args1.i433, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx.i440, align 8, !noalias !407
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1.i433)
          to label %_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit443 unwind label %cleanup9.loopexit.split-lp

_RNvCs2XfqmVweRya_18build_script_build7set_cfg.exit443: ; preds = %bb104
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i433), !noalias !407
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cfg.i435)
  br label %bb105

bb120:                                            ; preds = %bb119, %bb105
  %417 = icmp eq i64 %target_arch.sroa.0.0, 0
  br i1 %417, label %bb121, label %bb2.i.i.i4.i.i444

bb2.i.i.i4.i.i444:                                ; preds = %bb120
  %418 = icmp ne ptr %target_arch.sroa.6.0, null
  call void @llvm.assume(i1 %418)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_arch.sroa.6.0, i64 noundef %target_arch.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb121

bb109:                                            ; preds = %bb107.preheader
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 16), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.1 unwind label %cleanup9.loopexit

bb109.1:                                          ; preds = %bb109
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 32), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.2 unwind label %cleanup9.loopexit

bb109.2:                                          ; preds = %bb109.1
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 48), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.3 unwind label %cleanup9.loopexit

bb109.3:                                          ; preds = %bb109.2
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 64), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.4 unwind label %cleanup9.loopexit

bb109.4:                                          ; preds = %bb109.3
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 80), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.5 unwind label %cleanup9.loopexit

bb109.5:                                          ; preds = %bb109.4
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 96), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.6 unwind label %cleanup9.loopexit

bb109.6:                                          ; preds = %bb109.5
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 112), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.7 unwind label %cleanup9.loopexit

bb109.7:                                          ; preds = %bb109.6
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 128), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.8 unwind label %cleanup9.loopexit

bb109.8:                                          ; preds = %bb109.7
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 144), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.9 unwind label %cleanup9.loopexit

bb109.9:                                          ; preds = %bb109.8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 160), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.10 unwind label %cleanup9.loopexit

bb109.10:                                         ; preds = %bb109.9
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 176), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.11 unwind label %cleanup9.loopexit

bb109.11:                                         ; preds = %bb109.10
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 192), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.12 unwind label %cleanup9.loopexit

bb109.12:                                         ; preds = %bb109.11
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cfg)
  store ptr getelementptr inbounds nuw (i8, ptr @alloc_0932325d29f8c848cece173911e7c4a6, i64 208), ptr %cfg, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %cfg, ptr %args2, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_136.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8c8806985cff4e5eb0a771b7bf66c1ea, ptr noundef nonnull %args2)
          to label %bb109.13 unwind label %cleanup9.loopexit

bb109.13:                                         ; preds = %bb109.12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cfg)
  %419 = getelementptr inbounds nuw i8, ptr %name, i64 8
  %420 = getelementptr inbounds nuw i8, ptr %result.i, i64 8
  %421 = getelementptr inbounds nuw i8, ptr %result.i, i64 16
  %_547.sroa.5.0.values.sroa_idx = getelementptr inbounds nuw i8, ptr %values, i64 8
  %_547.sroa.6.0.values.sroa_idx = getelementptr inbounds nuw i8, ptr %values, i64 16
  %_155.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args5, i64 8
  %422 = getelementptr inbounds nuw i8, ptr %args5, i64 16
  %_156.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args5, i64 24
  br label %bb187

bb187:                                            ; preds = %bb109.13, %bb119
  %iter4.sroa.0.0.idx656 = phi i64 [ 0, %bb109.13 ], [ %iter4.sroa.0.0.add, %bb119 ]
  %iter4.sroa.0.0.ptr = getelementptr inbounds nuw i8, ptr @alloc_2bd9fa038d7fb2af8467fdc2c22fe0ae, i64 %iter4.sroa.0.0.idx656
  %iter4.sroa.0.0.add = add nuw nsw i64 %iter4.sroa.0.0.idx656, 32
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %name)
  %423 = load ptr, ptr %iter4.sroa.0.0.ptr, align 8, !nonnull !4, !align !17, !noundef !4
  %424 = getelementptr inbounds nuw i8, ptr %iter4.sroa.0.0.ptr, i64 8
  %425 = load i64, ptr %424, align 8, !noundef !4
  store ptr %423, ptr %name, align 8
  store i64 %425, ptr %419, align 8
  %426 = getelementptr inbounds nuw i8, ptr %iter4.sroa.0.0.ptr, i64 16
  %values.0 = load ptr, ptr %426, align 8, !nonnull !4, !align !8, !noundef !4
  %427 = getelementptr inbounds nuw i8, ptr %iter4.sroa.0.0.ptr, i64 24
  %values.1 = load i64, ptr %427, align 8, !noundef !4
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %values)
  call void @llvm.experimental.noalias.scope.decl(metadata !410)
  %_95.idx.i = shl nuw nsw i64 %values.1, 4
  %_95.i = getelementptr inbounds nuw i8, ptr %values.0, i64 %_95.idx.i
  %_103.i = icmp eq i64 %values.1, 0
  br i1 %_103.i, label %bb188, label %bb41.i446

bb41.i446:                                        ; preds = %bb187
  %gepdiff.i = add nsw i64 %_95.idx.i, -16
  %428 = lshr exact i64 %gepdiff.i, 4
  %429 = mul nuw nsw i64 %428, 3
  br label %bb1.i.i447

bb1.i.i447:                                       ; preds = %bb3.i.i448, %bb41.i446
  %_16.i6.i.i = phi ptr [ %values.0, %bb41.i446 ], [ %_16.i.i.i, %bb3.i.i448 ]
  %accum.sroa.0.0.i.i = phi i64 [ %429, %bb41.i446 ], [ %_4.0.i.i.i.i.i, %bb3.i.i448 ]
  %_6.i.i.i = icmp eq ptr %_16.i6.i.i, %_95.i
  br i1 %_6.i.i.i, label %bb53.i451, label %bb3.i.i448

bb3.i.i448:                                       ; preds = %bb1.i.i447
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i6.i.i, i64 16
  %430 = getelementptr i8, ptr %_16.i6.i.i, i64 8
  %.val3.i.i449 = load i64, ptr %430, align 8, !alias.scope !410, !noalias !413, !noundef !4
  %_4.0.i.i.i.i.i = add i64 %.val3.i.i449, %accum.sroa.0.0.i.i
  %_4.1.i.i.i.i.i = icmp ult i64 %_4.0.i.i.i.i.i, %accum.sroa.0.0.i.i
  br i1 %_4.1.i.i.i.i.i, label %bb52.i450, label %bb1.i.i447

bb53.i451:                                        ; preds = %bb1.i.i447
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %result.i), !noalias !417
  %_23.i.i.i.i = icmp eq i64 %accum.sroa.0.0.i.i, 0
  br i1 %_23.i.i.i.i, label %bb4.i452, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb53.i451
  %or.cond.not.i.i.i = icmp sgt i64 %accum.sroa.0.0.i.i, 0
  br i1 %or.cond.not.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i, label %bb172.invoke, !prof !418

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i: ; preds = %bb6.i.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22, !noalias !419
; call __rustc::__rust_alloc
  %431 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %accum.sroa.0.0.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !419
  %432 = icmp eq ptr %431, null
  br i1 %432, label %bb172.invoke, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i
  %433 = ptrtoint ptr %431 to i64
  br label %bb4.i452

bb52.i450:                                        ; preds = %bb3.i.i448
; invoke core::option::expect_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ca673fb95acb8e58af271999e89294ae, i64 noundef 53, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60488e92c3d9250777708a132d567f7b) #25
          to label %.noexc460 unwind label %cleanup9.loopexit.split-lp

.noexc460:                                        ; preds = %bb52.i450
  unreachable

cleanup.i455.loopexit:                            ; preds = %bb1.i.i.i.i458
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i455

cleanup.i455.loopexit.split-lp:                   ; preds = %bb3.i116.invoke.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i455

cleanup.i455:                                     ; preds = %cleanup.i455.loopexit.split-lp, %cleanup.i455.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup.i455.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.i455.loopexit.split-lp ]
  %result.val.i = load i64, ptr %result.i, align 8, !noalias !417
  %434 = icmp eq i64 %result.val.i, 0
  br i1 %434, label %bb136, label %bb2.i.i.i4.i.i456

bb2.i.i.i4.i.i456:                                ; preds = %cleanup.i455
  %result.val99.i = load ptr, ptr %420, align 8, !noalias !417, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %result.val99.i, i64 noundef %result.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !417
  br label %bb136

bb4.i452:                                         ; preds = %bb10.i.i.i, %bb53.i451
  %_4.sroa.10.0.i.i = phi i64 [ %433, %bb10.i.i.i ], [ 1, %bb53.i451 ]
  %435 = inttoptr i64 %_4.sroa.10.0.i.i to ptr
  store i64 %accum.sroa.0.0.i.i, ptr %result.i, align 8, !noalias !417
  store ptr %435, ptr %420, align 8, !noalias !417
  store i64 0, ptr %421, align 8, !noalias !417
  %slice.0.val.i = load ptr, ptr %values.0, align 8, !alias.scope !410, !noalias !422, !nonnull !4, !align !17, !noundef !4
  %436 = getelementptr i8, ptr %values.0, i64 8
  %slice.0.val101.i = load i64, ptr %436, align 8, !alias.scope !410, !noalias !422, !noundef !4
  call void @llvm.experimental.noalias.scope.decl(metadata !423)
  call void @llvm.experimental.noalias.scope.decl(metadata !426)
  %_7.i.i.i.i = icmp ugt i64 %slice.0.val101.i, %accum.sroa.0.0.i.i
  br i1 %_7.i.i.i.i, label %bb1.i.i.i.i458, label %bb55.i453, !prof !41

bb1.i.i.i.i458:                                   ; preds = %bb4.i452
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef 0, i64 noundef %slice.0.val101.i)
          to label %.noexc.i unwind label %cleanup.i455.loopexit

.noexc.i:                                         ; preds = %bb1.i.i.i.i458
  %len.pre.i.i.i = load i64, ptr %421, align 8, !alias.scope !429, !noalias !417
  %_10.i.i.pre.i = load ptr, ptr %420, align 8, !alias.scope !429, !noalias !417
  br label %bb55.i453

bb55.i453:                                        ; preds = %.noexc.i, %bb4.i452
  %_10.i.i.i = phi ptr [ %435, %bb4.i452 ], [ %_10.i.i.pre.i, %.noexc.i ]
  %len.i.i.i = phi i64 [ 0, %bb4.i452 ], [ %len.pre.i.i.i, %.noexc.i ]
  %_9.i.i.i = icmp sgt i64 %len.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i)
  %dst.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i, i64 %len.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i.i, ptr nonnull readonly align 1 %slice.0.val.i, i64 %slice.0.val101.i, i1 false), !noalias !430
  %437 = add i64 %len.i.i.i, %slice.0.val101.i
  %_157.i = icmp sgt i64 %437, -1
  call void @llvm.assume(i1 %_157.i)
  %index.i = sub nsw i64 %accum.sroa.0.0.i.i, %437
  %_2826.i = icmp eq i64 %values.1, 1
  br i1 %_2826.i, label %bb122.i, label %bb128.preheader.i

bb128.preheader.i:                                ; preds = %bb55.i453
  %iter_uninit.sroa.0.35.i = getelementptr inbounds nuw i8, ptr %values.0, i64 16
  %_159.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i, i64 %437
  br label %bb128.i

bb122.i.loopexit:                                 ; preds = %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i
  %_547.sroa.5.0.copyload499.pre = load ptr, ptr %420, align 8, !noalias !410
  br label %bb122.i

bb122.i:                                          ; preds = %bb122.i.loopexit, %bb55.i453
  %_547.sroa.5.0.copyload499 = phi ptr [ %_10.i.i.i, %bb55.i453 ], [ %_547.sroa.5.0.copyload499.pre, %bb122.i.loopexit ]
  %target.sroa.26.4.lcssa.i = phi i64 [ %index.i, %bb55.i453 ], [ %len.i.i112.i, %bb122.i.loopexit ]
  %result_len.i = sub i64 %accum.sroa.0.0.i.i, %target.sroa.26.4.lcssa.i
  %_547.sroa.0.0.copyload498 = load i64, ptr %result.i, align 8, !noalias !410
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %result.i), !noalias !417
  br label %bb188

bb128.i:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i, %bb128.preheader.i
  %iter_uninit.sroa.0.310.i = phi ptr [ %iter_uninit.sroa.0.3.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i ], [ %iter_uninit.sroa.0.35.i, %bb128.preheader.i ]
  %slice.0.pn9.i = phi ptr [ %iter_uninit.sroa.0.310.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i ], [ %values.0, %bb128.preheader.i ]
  %target.sroa.26.48.i = phi i64 [ %len.i.i112.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i ], [ %index.i, %bb128.preheader.i ]
  %target.sroa.0.47.i = phi ptr [ %data.i.i111.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i ], [ %_159.i, %bb128.preheader.i ]
  %iter_uninit.sroa.0.3.val.i = load ptr, ptr %iter_uninit.sroa.0.310.i, align 8, !alias.scope !410, !noalias !422, !nonnull !4, !align !17, !noundef !4
  %438 = getelementptr i8, ptr %slice.0.pn9.i, i64 24
  %iter_uninit.sroa.0.3.val100.i = load i64, ptr %438, align 8, !alias.scope !410, !noalias !422, !noundef !4
  %_6.not.i.i = icmp ult i64 %target.sroa.26.48.i, 3
  br i1 %_6.not.i.i, label %bb3.i116.invoke.i, label %bb132.i, !prof !41

bb132.i:                                          ; preds = %bb128.i
  %len.i.i104.i = add nsw i64 %target.sroa.26.48.i, -3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %target.sroa.0.47.i, ptr noundef nonnull align 1 dereferenceable(3) @alloc_4e81f3446308e52f5d03e9e4175413e4, i64 range(i64 0, -9223372036854775808) 3, i1 false), !alias.scope !431, !noalias !417
  %_6.not.i109.i = icmp ugt i64 %iter_uninit.sroa.0.3.val100.i, %len.i.i104.i
  br i1 %_6.not.i109.i, label %bb3.i116.invoke.i, label %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i, !prof !41

bb3.i116.invoke.i:                                ; preds = %bb132.i, %bb128.i
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3f2fbfdca196a5b824209b380ee7ae1b) #25
          to label %bb3.i116.cont.i unwind label %cleanup.i455.loopexit.split-lp, !noalias !417

bb3.i116.cont.i:                                  ; preds = %bb3.i116.invoke.i
  unreachable

_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build.exit122.i: ; preds = %bb132.i
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %target.sroa.0.47.i, i64 3
  %data.i.i111.i = getelementptr inbounds nuw i8, ptr %data.i.i.i, i64 %iter_uninit.sroa.0.3.val100.i
  %len.i.i112.i = sub nuw nsw i64 %len.i.i104.i, %iter_uninit.sroa.0.3.val100.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %data.i.i.i, ptr nonnull readonly align 1 %iter_uninit.sroa.0.3.val.i, i64 range(i64 0, -9223372036854775808) %iter_uninit.sroa.0.3.val100.i, i1 false), !alias.scope !435, !noalias !417
  %iter_uninit.sroa.0.3.i = getelementptr inbounds nuw i8, ptr %iter_uninit.sroa.0.310.i, i64 16
  %_282.i = icmp eq ptr %iter_uninit.sroa.0.3.i, %_95.i
  br i1 %_282.i, label %bb122.i.loopexit, label %bb128.i

bb188:                                            ; preds = %bb122.i, %bb187
  %_547.sroa.0.0 = phi i64 [ %_547.sroa.0.0.copyload498, %bb122.i ], [ 0, %bb187 ]
  %_547.sroa.5.0 = phi ptr [ %_547.sroa.5.0.copyload499, %bb122.i ], [ inttoptr (i64 1 to ptr), %bb187 ]
  %_547.sroa.6.0 = phi i64 [ %result_len.i, %bb122.i ], [ 0, %bb187 ]
  store i64 %_547.sroa.0.0, ptr %values, align 8
  store ptr %_547.sroa.5.0, ptr %_547.sroa.5.0.values.sroa_idx, align 8
  store i64 %_547.sroa.6.0, ptr %_547.sroa.6.0.values.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args5)
  store ptr %name, ptr %args5, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_155.sroa.4.0..sroa_idx, align 8
  store ptr %values, ptr %422, align 8
  store ptr @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_156.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_f9b7a4a216e67c48cfcff7c8ca3d1ad4, ptr noundef nonnull %args5)
          to label %bb115 unwind label %cleanup23

cleanup23:                                        ; preds = %bb188
  %439 = landingpad { ptr, i32 }
          cleanup
  %values.val = load i64, ptr %values, align 8
  %440 = icmp eq i64 %values.val, 0
  br i1 %440, label %bb136, label %bb2.i.i.i4.i.i463

bb2.i.i.i4.i.i463:                                ; preds = %cleanup23
  %values.val63 = load ptr, ptr %_547.sroa.5.0.values.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %values.val63, i64 noundef %values.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb136

bb115:                                            ; preds = %bb188
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args5)
  %values.val64 = load i64, ptr %values, align 8
  %441 = icmp eq i64 %values.val64, 0
  br i1 %441, label %bb119, label %bb2.i.i.i4.i.i465

bb2.i.i.i4.i.i465:                                ; preds = %bb115
  %values.val65 = load ptr, ptr %_547.sroa.5.0.values.sroa_idx, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %values.val65, i64 noundef %values.val64, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb119

bb119:                                            ; preds = %bb2.i.i.i4.i.i465, %bb115
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %values)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %name)
  %_539 = icmp eq i64 %iter4.sroa.0.0.add, 96
  br i1 %_539, label %bb120, label %bb187

bb121:                                            ; preds = %bb2.i.i.i4.i.i444, %bb120
  %442 = icmp eq i64 %target_ptr_width.sroa.0.0, 0
  br i1 %442, label %bb122, label %bb2.i.i.i4.i.i467

bb2.i.i.i4.i.i467:                                ; preds = %bb121
  %443 = icmp ne ptr %target_ptr_width.sroa.6.0, null
  call void @llvm.assume(i1 %443)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_ptr_width.sroa.6.0, i64 noundef %target_ptr_width.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb122

bb122:                                            ; preds = %bb2.i.i.i4.i.i467, %bb121
  %444 = icmp eq i64 %target_os.sroa.0.0, 0
  br i1 %444, label %bb123, label %bb2.i.i.i4.i.i469

bb2.i.i.i4.i.i469:                                ; preds = %bb122
  %445 = icmp ne ptr %target_os.sroa.6.0, null
  call void @llvm.assume(i1 %445)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_os.sroa.6.0, i64 noundef %target_os.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb123

bb123:                                            ; preds = %bb2.i.i.i4.i.i469, %bb122
  %446 = icmp eq i64 %target_env.sroa.0.0, 0
  br i1 %446, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit472, label %bb2.i.i.i4.i.i471

bb2.i.i.i4.i.i471:                                ; preds = %bb123
  %447 = icmp ne ptr %target_env.sroa.6.0, null
  call void @llvm.assume(i1 %447)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_env.sroa.6.0, i64 noundef %target_env.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit472

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit472: ; preds = %bb123, %bb2.i.i.i4.i.i471
  ret void

bb156:                                            ; preds = %bb2.i.i.i4.i.i369, %cleanup19, %bb2.i.i.i4.i.i367, %cleanup18, %bb157
  %.pn33548 = phi { ptr, i32 } [ %lpad.thr_comm.split-lp, %bb157 ], [ %382, %cleanup18 ], [ %382, %bb2.i.i.i4.i.i367 ], [ %384, %cleanup19 ], [ %384, %bb2.i.i.i4.i.i369 ]
  %_183.sroa.0.0.off0547 = phi i1 [ true, %bb157 ], [ false, %cleanup18 ], [ false, %bb2.i.i.i4.i.i367 ], [ true, %cleanup19 ], [ true, %bb2.i.i.i4.i.i369 ]
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %_70) #23
  br label %bb150

bb150:                                            ; preds = %bb156, %bb2.i.i.i4.i.i473, %bb155
  %.pn33537 = phi { ptr, i32 } [ %.pn33548, %bb156 ], [ %lpad.thr_comm, %bb155 ], [ %lpad.thr_comm, %bb2.i.i.i4.i.i473 ]
  %_183.sroa.0.0.off0534 = phi i1 [ %_183.sroa.0.0.off0547, %bb156 ], [ true, %bb155 ], [ true, %bb2.i.i.i4.i.i473 ]
  %448 = icmp eq i64 %_74, 0
  br i1 %448, label %bb151, label %bb153

bb155:                                            ; preds = %bb67.invoke
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
  %449 = getelementptr inbounds nuw i8, ptr %_70, i64 8
  %.val50 = load i64, ptr %449, align 8
  %450 = icmp eq i64 %.val50, 0
  br i1 %450, label %bb150, label %bb2.i.i.i4.i.i473

bb2.i.i.i4.i.i473:                                ; preds = %bb155
  %451 = getelementptr inbounds nuw i8, ptr %_70, i64 16
  %.val51 = load ptr, ptr %451, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val51, i64 noundef %.val50, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb150

bb151:                                            ; preds = %bb150
  br i1 %_183.sroa.0.0.off0534, label %bb152, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb153:                                            ; preds = %bb2.i.i.i4.i.i351, %bb131, %bb2.i.i.i4.i.i349, %cleanup15, %bb150
  %.pn33537746 = phi { ptr, i32 } [ %.pn33537, %bb150 ], [ %.pn31, %bb2.i.i.i4.i.i351 ], [ %.pn31, %bb131 ], [ %365, %bb2.i.i.i4.i.i349 ], [ %365, %cleanup15 ]
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %352) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb152:                                            ; preds = %bb151
  %452 = getelementptr inbounds nuw i8, ptr %_70, i64 40
  %.val = load i64, ptr %452, align 8
  %453 = icmp eq i64 %.val, 0
  br i1 %453, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336, label %bb2.i.i.i4.i.i475

bb2.i.i.i4.i.i475:                                ; preds = %bb152
  %454 = getelementptr inbounds nuw i8, ptr %_70, i64 48
  %.val49 = load ptr, ptr %454, align 8, !nonnull !4, !noundef !4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val49, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2XfqmVweRya_18build_script_build.exit336

bb158:                                            ; preds = %cleanup.body.i, %bb29.i, %bb2.i.i.i4.i71.i, %cleanup11
  %.pn502 = phi { ptr, i32 } [ %128, %cleanup11 ], [ %eh.lpad-body76.i, %cleanup.body.i ], [ %.pn127.i, %bb29.i ], [ %.pn127.i, %bb2.i.i.i4.i71.i ]
; call core::ptr::drop_in_place::<core::result::Result<alloc::string::String, std::env::VarError>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %_21) #23
  br label %bb136
}

; build_script_build::set_cfg
; Function Attrs: uwtable
define internal fastcc void @_RNvCs2XfqmVweRya_18build_script_build7set_cfg(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef range(i64 9, 24) %1) unnamed_addr #0 personality ptr @rust_eh_personality {
bb13.lr.ph.i:
  %args1 = alloca [16 x i8], align 8
  %args = alloca [16 x i8], align 8
  %cfg = alloca [16 x i8], align 8
  store ptr %0, ptr %cfg, align 8
  %2 = getelementptr inbounds nuw i8, ptr %cfg, i64 8
  store i64 %1, ptr %2, align 8
  switch i64 %1, label %bb1.backedge.i.13 [
    i64 23, label %bb2.i.i.i.i.i
    i64 13, label %bb2.i.i.i.i.i.1
    i64 9, label %bb2.i.i.i.i.i.2
    i64 22, label %bb2.i.i.i.i.i.8
    i64 15, label %bb2.i.i.i.i.i.9
    i64 18, label %bb2.i.i.i.i.i.10
    i64 17, label %bb2.i.i.i.i.i.11
    i64 11, label %bb2.i.i.i.i.i.13
  ]

bb2.i.i.i.i.i:                                    ; preds = %bb13.lr.ph.i
  %3 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(23) @alloc_ccedf80c3ce4e46e2ff8efee35ec798b, ptr noundef nonnull readonly align 1 dereferenceable(23) %0, i64 range(i64 0, -9223372036854775808) 23), !alias.scope !439, !noalias !446
  %4 = icmp eq i32 %3, 0
  br i1 %4, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.1:                                  ; preds = %bb13.lr.ph.i
  %5 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(13) @alloc_c1dd1d9f50ed06e24759135ae11c1cd7, ptr noundef nonnull readonly align 1 dereferenceable(13) %0, i64 range(i64 0, -9223372036854775808) 13), !alias.scope !439, !noalias !446
  %6 = icmp eq i32 %5, 0
  br i1 %6, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.2:                                  ; preds = %bb13.lr.ph.i
  %7 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_7267420313fdc34f79da1c04bfca7409, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %8 = icmp eq i32 %7, 0
  br i1 %8, label %bb1, label %bb2.i.i.i.i.i.3

bb2.i.i.i.i.i.3:                                  ; preds = %bb2.i.i.i.i.i.2
  %9 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_5581ed16f5c58ecd3f36713b9b396029, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %10 = icmp eq i32 %9, 0
  br i1 %10, label %bb1, label %bb2.i.i.i.i.i.4

bb2.i.i.i.i.i.4:                                  ; preds = %bb2.i.i.i.i.i.3
  %11 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_55f07188386ace482603892e4768112d, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %12 = icmp eq i32 %11, 0
  br i1 %12, label %bb1, label %bb2.i.i.i.i.i.5

bb2.i.i.i.i.i.5:                                  ; preds = %bb2.i.i.i.i.i.4
  %13 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_028f45a065ad7442c332be763445b925, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %14 = icmp eq i32 %13, 0
  br i1 %14, label %bb1, label %bb2.i.i.i.i.i.6

bb2.i.i.i.i.i.6:                                  ; preds = %bb2.i.i.i.i.i.5
  %15 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_358590eecf303ad391259af81e368788, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %bb1, label %bb2.i.i.i.i.i.7

bb2.i.i.i.i.i.7:                                  ; preds = %bb2.i.i.i.i.i.6
  %17 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_c8539d7d8992b0450a5874fa781e9124, ptr noundef nonnull readonly align 1 dereferenceable(9) %0, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !439, !noalias !446
  %18 = icmp eq i32 %17, 0
  br i1 %18, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.8:                                  ; preds = %bb13.lr.ph.i
  %19 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(22) @alloc_4052f5f320831d7a280bd8ee23d7c161, ptr noundef nonnull readonly align 1 dereferenceable(22) %0, i64 range(i64 0, -9223372036854775808) 22), !alias.scope !439, !noalias !446
  %20 = icmp eq i32 %19, 0
  br i1 %20, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.9:                                  ; preds = %bb13.lr.ph.i
  %21 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(15) @alloc_a4772b3acfc19af28fefe691db64c6aa, ptr noundef nonnull readonly align 1 dereferenceable(15) %0, i64 range(i64 0, -9223372036854775808) 15), !alias.scope !439, !noalias !446
  %22 = icmp eq i32 %21, 0
  br i1 %22, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.10:                                 ; preds = %bb13.lr.ph.i
  %23 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_e051788150efb5e0f212c696366647c3, ptr noundef nonnull readonly align 1 dereferenceable(18) %0, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !439, !noalias !446
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.11:                                 ; preds = %bb13.lr.ph.i
  %25 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(17) @alloc_e300d0c2c56fc656630ece49b293f3f6, ptr noundef nonnull readonly align 1 dereferenceable(17) %0, i64 range(i64 0, -9223372036854775808) 17), !alias.scope !439, !noalias !446
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %bb1, label %bb2.i.i.i.i.i.12

bb2.i.i.i.i.i.12:                                 ; preds = %bb2.i.i.i.i.i.11
  %27 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(17) @alloc_681b6f9e783332c8e0b8ad7b08df1498, ptr noundef nonnull readonly align 1 dereferenceable(17) %0, i64 range(i64 0, -9223372036854775808) 17), !alias.scope !439, !noalias !446
  %28 = icmp eq i32 %27, 0
  br i1 %28, label %bb1, label %bb1.backedge.i.13

bb2.i.i.i.i.i.13:                                 ; preds = %bb13.lr.ph.i
  %29 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(11) @alloc_513019cde2cbfb4427cb8f1afc437e08, ptr noundef nonnull readonly align 1 dereferenceable(11) %0, i64 range(i64 0, -9223372036854775808) 11), !alias.scope !439, !noalias !446
  %30 = icmp eq i32 %29, 0
  br i1 %30, label %bb1, label %bb1.backedge.i.13

bb1.backedge.i.13:                                ; preds = %bb13.lr.ph.i, %bb2.i.i.i.i.i, %bb2.i.i.i.i.i.7, %bb2.i.i.i.i.i.1, %bb2.i.i.i.i.i.8, %bb2.i.i.i.i.i.9, %bb2.i.i.i.i.i.12, %bb2.i.i.i.i.i.10, %bb2.i.i.i.i.i.13
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %cfg, ptr %args, align 8
  %_9.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_9.sroa.4.0..sroa_idx, align 8
; call core::panicking::panic_fmt
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_dc81aac016627a2e79d483ac8b04c639, ptr noundef nonnull %args, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_201211583369b7dd25e59c1fbc508159) #25
  unreachable

bb1:                                              ; preds = %bb2.i.i.i.i.i.13, %bb2.i.i.i.i.i.12, %bb2.i.i.i.i.i.11, %bb2.i.i.i.i.i.10, %bb2.i.i.i.i.i.9, %bb2.i.i.i.i.i.8, %bb2.i.i.i.i.i.7, %bb2.i.i.i.i.i.6, %bb2.i.i.i.i.i.5, %bb2.i.i.i.i.i.4, %bb2.i.i.i.i.i.3, %bb2.i.i.i.i.i.2, %bb2.i.i.i.i.i.1, %bb2.i.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1)
  store ptr %cfg, ptr %args1, align 8
  %_15.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build, ptr %_15.sroa.4.0..sroa_idx, align 8
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0f615e922801252a74ad4557d8ed2760, ptr noundef nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1)
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs2XfqmVweRya_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #6 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %or.cond.not = icmp sgt i64 %cap, 0
  br i1 %or.cond.not, label %bb14, label %bb11, !prof !418

bb14:                                             ; preds = %bb6.i
  %0 = icmp eq i64 %self.0.val, 0
  br i1 %0, label %bb4.i.i11, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit

bb14.thread:                                      ; preds = %start
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb9, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit: ; preds = %bb14.thread, %bb14
  %2 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %2)
  %cond.i.i = icmp uge i64 %cap, %self.0.val
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %self.0.val, i64 noundef range(i64 1, -9223372036854775807) 1, i64 noundef %cap) #22
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22
; call __rustc::__rust_alloc
  %3 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %cap, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit ], [ %3, %bb4.i.i11 ]
  %4 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 1, ptr %5, align 8
  br label %bb11

bb9:                                              ; preds = %bb14.thread, %bb7
  %raw_ptr.i.i.pn22 = phi ptr [ %raw_ptr.i.i.pn, %bb7 ], [ inttoptr (i64 1 to ptr), %bb14.thread ]
  %_27.sroa.7.01121 = phi i64 [ %cap, %bb7 ], [ 0, %bb14.thread ]
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn22, ptr %6, align 8
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb9, %bb8
  %.sink23 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %bb6.i ]
  %_27.sroa.7.01121.sink = phi i64 [ %_27.sroa.7.01121, %bb9 ], [ %cap, %bb8 ], [ 0, %bb6.i ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %bb6.i ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink23
  store i64 %_27.sroa.7.01121.sink, ptr %7, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <core::str::iter::SplitInternal<char>>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs2XfqmVweRya_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #5 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !450, !noundef !4
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !4, !align !17, !noundef !4
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1 = load i64, ptr %2, align 8, !noundef !4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !451)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i = load i64, ptr %4, align 8, !alias.scope !451, !noalias !454, !noundef !4
  %_38.not.i = icmp ugt i64 %index2.i, %_4.val1
  %.promoted.i = load i64, ptr %3, align 8, !alias.scope !451, !noalias !454
  %_4325.i = icmp ult i64 %index2.i, %.promoted.i
  %or.cond26.i = or i1 %_38.not.i, %_4325.i
  br i1 %or.cond26.i, label %bb1.i, label %bb12.lr.ph.i

bb12.lr.ph.i:                                     ; preds = %bb2
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_48.i = load i8, ptr %5, align 8, !alias.scope !451, !noalias !454, !noundef !4
  %_12.i = zext i8 %_48.i to i64
  %6 = getelementptr i8, ptr %_10.i, i64 %_12.i
  %_49.i = getelementptr i8, ptr %6, i64 -1
  %_65.i = icmp ult i8 %_48.i, 5
  %last_byte.us.pre.i = load i8, ptr %_49.i, align 1, !alias.scope !451, !noalias !454
  br i1 %_65.i, label %bb12.us.i, label %bb12.i, !prof !456

bb12.us.i:                                        ; preds = %bb12.lr.ph.i, %bb9.us.i
  %7 = phi i64 [ %16, %bb9.us.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.us.i = sub nuw i64 %index2.i, %7
  %_46.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %7
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i), !noalias !457
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %10, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %9, align 1, !alias.scope !458, !noalias !457, !noundef !4
  %_8.i.us.i = icmp eq i8 %_9.i.us.i, %last_byte.us.pre.i
  br i1 %_8.i.us.i, label %bb4.i.us.i, label %bb9.i.us.i

bb9.i.us.i:                                       ; preds = %bb7.i.us.i
  %10 = add nuw nsw i64 %i.sroa.0.05.i.us.i, 1
  %exitcond.not.i.us.i = icmp eq i64 %10, %new_len.us.i
  br i1 %exitcond.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb4.i.us.i:                                       ; preds = %bb9.i.us.i, %bb7.i.us.i, %bb5.preheader.i.us.i
  %i.sroa.0.0.lcssa.i.us.i = phi i64 [ 0, %bb5.preheader.i.us.i ], [ %i.sroa.0.05.i.us.i, %bb7.i.us.i ], [ %new_len.us.i, %bb9.i.us.i ]
  %_0.sroa.0.1.i.us.i = phi i64 [ 0, %bb5.preheader.i.us.i ], [ 1, %bb7.i.us.i ], [ 0, %bb9.i.us.i ]
  %11 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us.i, 0
  %12 = insertvalue { i64, i64 } %11, i64 %i.sroa.0.0.lcssa.i.us.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i: ; preds = %bb4.i.us.i, %bb2.i.us.i
  %.merged.i.us.i = phi { i64, i64 } [ %12, %bb4.i.us.i ], [ %8, %bb2.i.us.i ]
  %13 = extractvalue { i64, i64 } %.merged.i.us.i, 0
  %14 = trunc nuw i64 %13 to i1
  br i1 %14, label %bb4.us.i, label %bb10.i

bb4.us.i:                                         ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  %15 = extractvalue { i64, i64 } %.merged.i.us.i, 1
  %_16.us.i = add i64 %7, 1
  %16 = add i64 %_16.us.i, %15
  store i64 %16, ptr %3, align 8, !alias.scope !451, !noalias !454
  %_17.not.us.i = icmp ult i64 %16, %_12.i
  %_54.not.us.i = icmp ugt i64 %16, %_4.val1
  %or.cond.i = or i1 %_17.not.us.i, %_54.not.us.i
  br i1 %or.cond.i, label %bb9.us.i, label %bb19.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %found_char.us.i = sub nuw i64 %16, %_12.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %found_char.us.i
  %17 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us.i, ptr nonnull readonly align 1 %_10.i, i64 range(i64 0, -9223372036854775808) %_12.i), !alias.scope !461, !noalias !454
  %18 = icmp eq i32 %17, 0
  br i1 %18, label %bb7, label %bb9.us.i

bb9.us.i:                                         ; preds = %bb19.us.i, %bb4.us.i
  %_43.us.i = icmp ult i64 %index2.i, %16
  br i1 %_43.us.i, label %bb1.i, label %bb12.us.i

bb12.i:                                           ; preds = %bb12.lr.ph.i, %bb9.i
  %19 = phi i64 [ %28, %bb9.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.i = sub nuw i64 %index2.i, %19
  %_46.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %19
  %_3.i.i = icmp samesign ult i64 %new_len.i, 16
  br i1 %_3.i.i, label %bb5.preheader.i.i, label %bb2.i.i

bb5.preheader.i.i:                                ; preds = %bb12.i
  %_64.not.i.i = icmp eq i64 %new_len.i, 0
  br i1 %_64.not.i.i, label %bb4.i.i, label %bb7.i.i

bb2.i.i:                                          ; preds = %bb12.i
; call core::slice::memchr::memchr_aligned
  %20 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i), !noalias !457
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i

bb4.i.i:                                          ; preds = %bb9.i.i, %bb7.i.i, %bb5.preheader.i.i
  %i.sroa.0.0.lcssa.i.i = phi i64 [ 0, %bb5.preheader.i.i ], [ %i.sroa.0.05.i.i, %bb7.i.i ], [ %new_len.i, %bb9.i.i ]
  %_0.sroa.0.1.i.i = phi i64 [ 0, %bb5.preheader.i.i ], [ 1, %bb7.i.i ], [ 0, %bb9.i.i ]
  %21 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.i, 0
  %22 = insertvalue { i64, i64 } %21, i64 %i.sroa.0.0.lcssa.i.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i

bb7.i.i:                                          ; preds = %bb5.preheader.i.i, %bb9.i.i
  %i.sroa.0.05.i.i = phi i64 [ %24, %bb9.i.i ], [ 0, %bb5.preheader.i.i ]
  %23 = getelementptr inbounds nuw i8, ptr %_46.i, i64 %i.sroa.0.05.i.i
  %_9.i.i = load i8, ptr %23, align 1, !alias.scope !458, !noalias !457, !noundef !4
  %_8.i.i = icmp eq i8 %_9.i.i, %last_byte.us.pre.i
  br i1 %_8.i.i, label %bb4.i.i, label %bb9.i.i

bb9.i.i:                                          ; preds = %bb7.i.i
  %24 = add nuw nsw i64 %i.sroa.0.05.i.i, 1
  %exitcond.not.i.i = icmp eq i64 %24, %new_len.i
  br i1 %exitcond.not.i.i, label %bb4.i.i, label %bb7.i.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i: ; preds = %bb4.i.i, %bb2.i.i
  %.merged.i.i = phi { i64, i64 } [ %22, %bb4.i.i ], [ %20, %bb2.i.i ]
  %25 = extractvalue { i64, i64 } %.merged.i.i, 0
  %26 = trunc nuw i64 %25 to i1
  br i1 %26, label %bb4.i, label %bb10.i

bb4.i:                                            ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i
  %27 = extractvalue { i64, i64 } %.merged.i.i, 1
  %_16.i = add i64 %19, 1
  %28 = add i64 %_16.i, %27
  store i64 %28, ptr %3, align 8, !alias.scope !451, !noalias !454
  %_17.not.i = icmp ult i64 %28, %_12.i
  %_54.not.i = icmp ugt i64 %28, %_4.val1
  %or.cond70.i = or i1 %_17.not.i, %_54.not.i
  br i1 %or.cond70.i, label %bb9.i, label %bb25.i

bb10.i:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  store i64 %index2.i, ptr %3, align 8, !alias.scope !451, !noalias !454
  br label %bb1.i

bb9.i:                                            ; preds = %bb4.i
  %_43.i = icmp ult i64 %index2.i, %28
  br i1 %_43.i, label %bb1.i, label %bb12.i

bb25.i:                                           ; preds = %bb4.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #25, !noalias !457
  unreachable

bb7:                                              ; preds = %bb19.us.i
  %i = load i64, ptr %self, align 8, !noundef !4
  %new_len = sub nuw i64 %found_char.us.i, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %16, ptr %self, align 8
  br label %bb9

bb1.i:                                            ; preds = %bb9.i, %bb9.us.i, %bb2, %bb10.i
  store i8 1, ptr %0, align 1, !alias.scope !465
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %30 = load i8, ptr %29, align 8, !range !450, !alias.scope !465, !noundef !4
  %_3.i = trunc nuw i8 %30 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !465
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !465
  %_4.not.i = icmp ne i64 %i1.pre.i, %i.pre.i
  %or.cond.not.i = select i1 %_3.i, i1 true, i1 %_4.not.i
  %new_len.i4 = sub nuw i64 %i1.pre.i, %i.pre.i
  %data.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i.pre.i
  %_0.sroa.3.0.i = select i1 %or.cond.not.i, i64 %new_len.i4, i64 undef
  %_0.sroa.0.0.i = select i1 %or.cond.not.i, ptr %data.i, ptr null
  br label %bb9

bb9:                                              ; preds = %bb1.i, %bb7, %start
  %_0.sroa.4.1 = phi i64 [ undef, %start ], [ %new_len, %bb7 ], [ %_0.sroa.3.0.i, %bb1.i ]
  %_0.sroa.0.1 = phi ptr [ null, %start ], [ %data, %bb7 ], [ %_0.sroa.0.0.i, %bb1.i ]
  %31 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.1, 0
  %32 = insertvalue { ptr, i64 } %31, i64 %_0.sroa.4.1, 1
  ret { ptr, i64 } %32
}

; <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
; Function Attrs: uwtable
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs2XfqmVweRya_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !4
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !468)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !471)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !474, !noalias !475
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !474, !noalias !475
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !474, !noalias !475
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !474, !noalias !475
  store i64 0, ptr %self, align 8, !alias.scope !474, !noalias !475
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit

bb7.i.i:                                          ; preds = %bb1
  %.not.i.i = icmp eq ptr %self1.sroa.5.sroa.0.0.copyload.i.i, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb2.i

bb3.i.i:                                          ; preds = %bb7.i.i
  %3 = icmp ne ptr %self1.sroa.5.sroa.5.0.copyload.i.i, null
  tail call void @llvm.assume(i1 %3)
  %4 = icmp eq i64 %self1.sroa.5.sroa.6.0.copyload.i.i, 0
  br i1 %4, label %bb2.i, label %bb10.i.i

bb10.i.i:                                         ; preds = %bb3.i.i, %bb10.i.i
  %root2.sroa.0.011.i.i = phi ptr [ %5, %bb10.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ]
  %root.sroa.0.010.i.i = phi i64 [ %6, %bb10.i.i ], [ %self1.sroa.5.sroa.6.0.copyload.i.i, %bb3.i.i ]
  %_19.i.i = getelementptr inbounds nuw i8, ptr %root2.sroa.0.011.i.i, i64 544
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !477, !nonnull !4, !noundef !4
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !478, !noundef !4
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #22, !noalias !483
  %11 = load ptr, ptr %10, align 8, !noalias !478, !noundef !4
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #22, !noalias !483
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !484)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !487)
  %_3.i.i = load i64, ptr %self, align 8, !range !3, !alias.scope !490, !noalias !491, !noundef !4
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !490, !noalias !491, !noundef !4
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !493, !noalias !496
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !493, !noalias !496
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !490, !noalias !491, !noundef !4
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !490, !noalias !491, !nonnull !4, !noundef !4
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !490, !noalias !491
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !498, !nonnull !4, !noundef !4
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !499)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !500, !noundef !4
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !507, !noundef !4
  %.not.i.i.i.i.i.i = icmp eq ptr %23, null
  br i1 %.not.i.i.i.i.i.i, label %bb3.i.i.i, label %bb7.i.i.i.i

bb12.loopexit.i.i.i.i:                            ; preds = %bb7.i.i.i.i
  %_20.i.i.i.i.i.i = zext i16 %28 to i64
  br label %bb12.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %bb12.loopexit.i.i.i.i, %bb7.i
  %edge.sroa.8.0.lcssa.i.i.i.i = phi i64 [ %value.sroa.3.0.copyload.i.i, %bb7.i ], [ %_20.i.i.i.i.i.i, %bb12.loopexit.i.i.i.i ]
  %edge.sroa.5.0.lcssa.i.i.i.i = phi i64 [ %value.sroa.2.0.copyload.i.i, %bb7.i ], [ %_18.i.i.i.i.i.i, %bb12.loopexit.i.i.i.i ]
  %edge.sroa.0.0.lcssa.i.i.i.i = phi ptr [ %value.sroa.0.0.copyload.i.i, %bb7.i ], [ %23, %bb12.loopexit.i.i.i.i ]
  %24 = icmp eq i64 %edge.sroa.5.0.lcssa.i.i.i.i, 0
  br i1 %24, label %bb2.i.i.i.i.i, label %bb3.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %_11.i.i.i.i.i = add nuw nsw i64 %edge.sroa.8.0.lcssa.i.i.i.i, 1
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !512, !nonnull !4, !noundef !4
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !507
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #22, !noalias !516
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !500, !noundef !4
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #22, !noalias !516
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #25
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !517

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #26, !noalias !518
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !493, !noalias !496
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !493, !noalias !496
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !493, !noalias !496
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build.exit
  ret void
}

; <&core::option::Option<&str> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRINtNtB8_6option6OptionReENtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %__self_0.i = alloca [8 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !nonnull !4, !align !8, !noundef !4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !519)
  %0 = load ptr, ptr %_3, align 8, !alias.scope !519, !noalias !522, !align !17, !noundef !4
  %.not.i = icmp eq ptr %0, null
  br i1 %.not.i, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_0.i), !noalias !524
  store ptr %_3, ptr %__self_0.i, align 8, !noalias !524
; call <core::fmt::Formatter>::debug_tuple_field1_finish
  %1 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9535bf4c204f3eb9b19ec2c83e446e52, i64 noundef 4, ptr noundef nonnull align 1 %__self_0.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_0.i), !noalias !524
  br label %_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build.exit

bb3.i:                                            ; preds = %start
; call <core::fmt::Formatter>::write_str
  %2 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_37d2e53432a03a1f90b3e7253015eaf9, i64 noundef 4), !noalias !519
  br label %_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build.exit

_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build.exit: ; preds = %bb2.i, %bb3.i
  %_0.sroa.0.0.in.i = phi i1 [ %1, %bb2.i ], [ %2, %bb3.i ]
  ret i1 %_0.sroa.0.0.in.i
}

; <&core::num::error::IntErrorKind as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !4, !align !17, !noundef !4
  %_3.val = load i8, ptr %_3, align 1, !range !525, !noundef !4
  %0 = zext nneg i8 %_3.val to i64
  %switch.gep = getelementptr inbounds nuw [5 x i64], ptr @switch.table._RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build, i64 0, i64 %0
  %switch.load = load i64, ptr %switch.gep, align 8
  %1 = zext nneg i8 %_3.val to i64
  %switch.gep1 = getelementptr inbounds nuw [5 x ptr], ptr @switch.table._RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtB8_3num5error12IntErrorKindNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build.88, i64 0, i64 %1
  %switch.load2 = load ptr, ptr %switch.gep1, align 8
; call <core::fmt::Formatter>::write_str
  %_0.i = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %switch.load2, i64 noundef %switch.load)
  ret i1 %_0.i
}

; <&&str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_5Debug3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !4, !align !8, !noundef !4
  %_3.val = load ptr, ptr %_3, align 8, !nonnull !4, !align !17, !noundef !4
  %0 = getelementptr i8, ptr %_3, i64 8
  %_3.val1 = load i64, ptr %0, align 8, !noundef !4
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.val, i64 noundef %_3.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <&&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !4, !align !8, !noundef !4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !526)
  %_3.0.i = load ptr, ptr %_3, align 8, !alias.scope !526, !noalias !529, !nonnull !4, !align !17, !noundef !4
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_3.1.i = load i64, ptr %0, align 8, !alias.scope !526, !noalias !529, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0.i, i64 noundef %_3.1.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f), !noalias !526
  ret i1 %_0.i
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !4, !align !17, !noundef !4
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <alloc::borrow::Cow<str> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXsb_NtCsdJPVW0sQgAG_5alloc6borrowINtB5_3CoweENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtCs2XfqmVweRya_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8.i = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7.i = load i64, ptr %1, align 8, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8.i, i64 noundef %_7.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <core::num::error::ParseIntError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsc_NtNtCsjMrxcFdYDNN_4core3num5errorNtB5_13ParseIntErrorNtNtB9_3fmt5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #5 {
start:
  %_5 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5)
  store ptr %self, ptr %_5, align 8
; call <core::fmt::Formatter>::debug_struct_field1_finish
  %_0 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f62df14955f7d78bca139b0a7668683d, i64 noundef 13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a5d866b1768ad3f826bccdb004a1a8ae, i64 noundef 4, ptr noundef nonnull align 1 %_5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.7)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5)
  ret i1 %_0
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #5 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load ptr, ptr %0, align 8, !nonnull !4, !noundef !4
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7 = load i64, ptr %1, align 8, !noundef !4
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8, i64 noundef %_7, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #8

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #10

; <std::sys::process::unix::common::Command>::arg
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef align 8 dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #11

; <std::sys::process::unix::common::Command>::new
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr dead_on_unwind noalias noundef writable sret([200 x i8]) align 8 captures(none) dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; std::env::_var
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; std::env::_var_os
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #12

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; core::panicking::assert_failed_inner
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking19assert_failed_inner(i8 noundef range(i8 0, 3), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noundef, ptr, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #2

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #13

; <std::process::Command>::output
; Function Attrs: uwtable
declare void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr dead_on_unwind noalias noundef writable sret([56 x i8]) align 8 captures(none) dereferenceable(56), ptr noalias noundef align 8 dereferenceable(200)) unnamed_addr #0

; <alloc::string::String>::from_utf8_lossy
; Function Attrs: uwtable
declare void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; <i32 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3implNtB9_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <alloc::string::String as core::clone::Clone>::clone
; Function Attrs: uwtable
declare void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #14

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #15

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #16

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #12

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.smul.with.overflow.i32(i32, i32) #12

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.ssub.with.overflow.i32(i32, i32) #12

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #12

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #17

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #18

; <core::fmt::Formatter>::debug_tuple_field1_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::fmt::Formatter>::debug_struct_field1_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #19 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCs2XfqmVweRya_18build_script_build4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #20

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #21

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #11 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #13 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #18 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #19 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #21 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #22 = { nounwind }
attributes #23 = { cold }
attributes #24 = { cold noreturn nounwind }
attributes #25 = { noinline noreturn }
attributes #26 = { noreturn }
attributes #27 = { noinline }
attributes #28 = { inlinehint }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{i64 0, i64 2}
!4 = !{}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs2XfqmVweRya_18build_script_build: %_1.0"}
!7 = distinct !{!7, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs2XfqmVweRya_18build_script_build"}
!8 = !{i64 8}
!9 = !{i64 0, i64 -9223372036854775808}
!10 = !{i64 1, i64 0}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs2XfqmVweRya_18build_script_build: %_1"}
!13 = distinct !{!13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs2XfqmVweRya_18build_script_build"}
!14 = !{!15, !12}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs2XfqmVweRya_18build_script_build: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs2XfqmVweRya_18build_script_build"}
!17 = !{i64 1}
!18 = !{i64 4}
!19 = !{i32 0, i32 6}
!20 = !{!"branch_weights", i32 2000, i32 6001}
!21 = !{!22}
!22 = distinct !{!22, !23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build: %_1"}
!23 = distinct !{!23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build"}
!24 = !{!25}
!25 = distinct !{!25, !26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs2XfqmVweRya_18build_script_build: %self"}
!26 = distinct !{!26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs2XfqmVweRya_18build_script_build"}
!27 = !{!25, !22}
!28 = !{!29, !31, !25, !22}
!29 = distinct !{!29, !30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs2XfqmVweRya_18build_script_build: %self"}
!30 = distinct !{!30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs2XfqmVweRya_18build_script_build"}
!31 = distinct !{!31, !32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build: %_1"}
!32 = distinct !{!32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs2XfqmVweRya_18build_script_build"}
!33 = !{i64 0, i64 -9223372036854775807}
!34 = !{i64 20924470785627791}
!35 = !{!36}
!36 = distinct !{!36, !37, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs2XfqmVweRya_18build_script_build: %self"}
!37 = distinct !{!37, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs2XfqmVweRya_18build_script_build"}
!38 = !{!39}
!39 = distinct !{!39, !40, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs2XfqmVweRya_18build_script_build: %_1"}
!40 = distinct !{!40, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs2XfqmVweRya_18build_script_build"}
!41 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!42 = !{!43, !45}
!43 = distinct !{!43, !44, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %_0"}
!44 = distinct !{!44, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build"}
!45 = distinct !{!45, !44, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %program"}
!46 = !{!45}
!47 = !{!48, !50}
!48 = distinct !{!48, !49, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %_0"}
!49 = distinct !{!49, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build"}
!50 = distinct !{!50, !49, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %program"}
!51 = !{!50}
!52 = !{!53, !55}
!53 = distinct !{!53, !54, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %_0"}
!54 = distinct !{!54, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build"}
!55 = distinct !{!55, !54, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %program"}
!56 = !{!55}
!57 = !{!58}
!58 = distinct !{!58, !59, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build: %arg"}
!59 = distinct !{!59, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECs2XfqmVweRya_18build_script_build"}
!60 = !{!61}
!61 = distinct !{!61, !62, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtBL_2io5error5ErrorE6expectCs2XfqmVweRya_18build_script_build: %t"}
!62 = distinct !{!62, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtBL_2io5error5ErrorE6expectCs2XfqmVweRya_18build_script_build"}
!63 = !{!64}
!64 = distinct !{!64, !62, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtBL_2io5error5ErrorE6expectCs2XfqmVweRya_18build_script_build: %self"}
!65 = !{!61, !64}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build: %_1"}
!68 = distinct !{!68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build"}
!69 = !{!70, !72, !73, !75}
!70 = distinct !{!70, !71, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!71 = distinct !{!71, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!72 = distinct !{!72, !71, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!73 = distinct !{!73, !74, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!74 = distinct !{!74, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!75 = distinct !{!75, !74, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!76 = !{!77}
!77 = distinct !{!77, !78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build: %_1"}
!78 = distinct !{!78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build"}
!79 = !{!"branch_weights", i32 -102759400, i32 4193255}
!80 = !{!81, !83}
!81 = distinct !{!81, !82, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!82 = distinct !{!82, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!83 = distinct !{!83, !82, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!84 = !{!"branch_weights", !"expected", i32 -2147483648, i32 0}
!85 = !{!86}
!86 = distinct !{!86, !87, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!87 = distinct !{!87, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!88 = !{!"branch_weights", i32 2002, i32 2000}
!89 = !{!90}
!90 = distinct !{!90, !91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build: %_1"}
!91 = distinct !{!91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs2XfqmVweRya_18build_script_build"}
!92 = !{!93}
!93 = distinct !{!93, !94, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!94 = distinct !{!94, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!95 = !{!96}
!96 = distinct !{!96, !97, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!97 = distinct !{!97, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!98 = !{!99}
!99 = distinct !{!99, !100, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %x"}
!100 = distinct !{!100, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build"}
!101 = !{!102}
!102 = distinct !{!102, !100, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %self"}
!103 = !{!99, !102}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!106 = distinct !{!106, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!107 = !{!105, !102}
!108 = !{!105, !99, !102}
!109 = !{!110}
!110 = distinct !{!110, !111, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %x"}
!111 = distinct !{!111, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build"}
!112 = !{!113}
!113 = distinct !{!113, !111, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %self"}
!114 = !{!110, !113}
!115 = !{!116}
!116 = distinct !{!116, !117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!117 = distinct !{!117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!118 = !{!116, !113}
!119 = !{!116, !110, !113}
!120 = !{!121}
!121 = distinct !{!121, !122, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %x"}
!122 = distinct !{!122, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build"}
!123 = !{!124}
!124 = distinct !{!124, !122, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %self"}
!125 = !{!121, !124}
!126 = !{!127}
!127 = distinct !{!127, !128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!128 = distinct !{!128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!129 = !{!127, !124}
!130 = !{!127, !121, !124}
!131 = !{!132}
!132 = distinct !{!132, !133, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %x"}
!133 = distinct !{!133, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build"}
!134 = !{!135}
!135 = distinct !{!135, !133, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCs2XfqmVweRya_18build_script_build: %self"}
!136 = !{!132, !135}
!137 = !{!138}
!138 = distinct !{!138, !139, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!139 = distinct !{!139, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!140 = !{!138, !135}
!141 = !{!138, !132, !135}
!142 = !{!143}
!143 = distinct !{!143, !144, !"_RNvMsp_NtCsjMrxcFdYDNN_4core3numl16from_ascii_radix: argument 0"}
!144 = distinct !{!144, !"_RNvMsp_NtCsjMrxcFdYDNN_4core3numl16from_ascii_radix"}
!145 = !{!146}
!146 = distinct !{!146, !147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build: %_1"}
!147 = distinct !{!147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build"}
!148 = !{!149, !151}
!149 = distinct !{!149, !150, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8: %_0"}
!150 = distinct !{!150, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8"}
!151 = distinct !{!151, !150, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8: %vec"}
!152 = !{!153, !155, !156, !158}
!153 = distinct !{!153, !154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!154 = distinct !{!154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!155 = distinct !{!155, !154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!156 = distinct !{!156, !157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!157 = distinct !{!157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!158 = distinct !{!158, !157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!159 = !{!160, !162, !163, !165}
!160 = distinct !{!160, !161, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!161 = distinct !{!161, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!162 = distinct !{!162, !161, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!163 = distinct !{!163, !164, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!164 = distinct !{!164, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!165 = distinct !{!165, !164, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!166 = !{!167, !169, !170, !172}
!167 = distinct !{!167, !168, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!168 = distinct !{!168, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!169 = distinct !{!169, !168, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!170 = distinct !{!170, !171, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!171 = distinct !{!171, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!172 = distinct !{!172, !171, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!173 = !{!174, !176, !177, !179}
!174 = distinct !{!174, !175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!175 = distinct !{!175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!176 = distinct !{!176, !175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!177 = distinct !{!177, !178, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!178 = distinct !{!178, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!179 = distinct !{!179, !178, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!180 = !{!181, !183, !184, !186}
!181 = distinct !{!181, !182, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!182 = distinct !{!182, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!183 = distinct !{!183, !182, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!184 = distinct !{!184, !185, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!185 = distinct !{!185, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!186 = distinct !{!186, !185, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!187 = !{!188, !190, !191, !193}
!188 = distinct !{!188, !189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!189 = distinct !{!189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!190 = distinct !{!190, !189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!191 = distinct !{!191, !192, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %self.0"}
!192 = distinct !{!192, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build"}
!193 = distinct !{!193, !192, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs2XfqmVweRya_18build_script_build: %needle.0"}
!194 = !{!195}
!195 = distinct !{!195, !196, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!196 = distinct !{!196, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!197 = !{!"branch_weights", !"expected", i32 6447283, i32 2141036365}
!198 = !{!199}
!199 = distinct !{!199, !200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build: %_1"}
!200 = distinct !{!200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs2XfqmVweRya_18build_script_build"}
!201 = !{!202, !204}
!202 = distinct !{!202, !203, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8: %_0"}
!203 = distinct !{!203, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8"}
!204 = distinct !{!204, !203, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String9from_utf8: %vec"}
!205 = !{!206}
!206 = distinct !{!206, !207, !"_RINvMNtCsjMrxcFdYDNN_4core3stre12trim_matchesNvMNtNtB5_4char7methodsc13is_whitespaceECs2XfqmVweRya_18build_script_build: %self.0"}
!207 = distinct !{!207, !"_RINvMNtCsjMrxcFdYDNN_4core3stre12trim_matchesNvMNtNtB5_4char7methodsc13is_whitespaceECs2XfqmVweRya_18build_script_build"}
!208 = !{!209, !211, !213, !215, !216, !218, !219, !221}
!209 = distinct !{!209, !210, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build: %bytes"}
!210 = distinct !{!210, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build"}
!211 = distinct !{!211, !212, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!212 = distinct !{!212, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!213 = distinct !{!213, !214, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %_0"}
!214 = distinct !{!214, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build"}
!215 = distinct !{!215, !214, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %self"}
!216 = distinct !{!216, !217, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build: %_0"}
!217 = distinct !{!217, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build"}
!218 = distinct !{!218, !217, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build: %self"}
!219 = distinct !{!219, !220, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build: %_0"}
!220 = distinct !{!220, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build"}
!221 = distinct !{!221, !220, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_8Searcher11next_rejectCs2XfqmVweRya_18build_script_build: %self"}
!222 = !{!213, !215, !216, !218, !219, !221, !206}
!223 = !{!224, !226, !228, !230, !231, !233, !234, !236}
!224 = distinct !{!224, !225, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build: %bytes"}
!225 = distinct !{!225, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build"}
!226 = distinct !{!226, !227, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back: %self"}
!227 = distinct !{!227, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back"}
!228 = distinct !{!228, !229, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs2XfqmVweRya_18build_script_build: %_0"}
!229 = distinct !{!229, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs2XfqmVweRya_18build_script_build"}
!230 = distinct !{!230, !229, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs2XfqmVweRya_18build_script_build: %self"}
!231 = distinct !{!231, !232, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build: %_0"}
!232 = distinct !{!232, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build"}
!233 = distinct !{!233, !232, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build: %self"}
!234 = distinct !{!234, !235, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build: %_0"}
!235 = distinct !{!235, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build"}
!236 = distinct !{!236, !235, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs2XfqmVweRya_18build_script_build: %self"}
!237 = !{!228, !230, !231, !233, !234, !236, !206}
!238 = !{!239, !241, !243, !245, !246, !248, !249, !251, !252}
!239 = distinct !{!239, !240, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build: %bytes"}
!240 = distinct !{!240, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build"}
!241 = distinct !{!241, !242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!242 = distinct !{!242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!243 = distinct !{!243, !244, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %_0"}
!244 = distinct !{!244, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build"}
!245 = distinct !{!245, !244, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %self"}
!246 = distinct !{!246, !247, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!247 = distinct !{!247, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!248 = distinct !{!248, !247, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!249 = distinct !{!249, !250, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!250 = distinct !{!250, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!251 = distinct !{!251, !250, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!252 = distinct !{!252, !253, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build: %self"}
!253 = distinct !{!253, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build"}
!254 = !{!255}
!255 = distinct !{!255, !256, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: argument 1"}
!256 = distinct !{!256, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix"}
!257 = !{!258}
!258 = distinct !{!258, !256, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: %_0"}
!259 = !{!260, !262, !264, !266, !267, !269, !270, !272, !273}
!260 = distinct !{!260, !261, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build: %bytes"}
!261 = distinct !{!261, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build"}
!262 = distinct !{!262, !263, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!263 = distinct !{!263, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!264 = distinct !{!264, !265, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %_0"}
!265 = distinct !{!265, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build"}
!266 = distinct !{!266, !265, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %self"}
!267 = distinct !{!267, !268, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!268 = distinct !{!268, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!269 = distinct !{!269, !268, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!270 = distinct !{!270, !271, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!271 = distinct !{!271, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!272 = distinct !{!272, !271, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!273 = distinct !{!273, !274, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build: %self"}
!274 = distinct !{!274, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build"}
!275 = !{!276}
!276 = distinct !{!276, !277, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: argument 1"}
!277 = distinct !{!277, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix"}
!278 = !{!279}
!279 = distinct !{!279, !277, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: %_0"}
!280 = !{!281, !283, !285, !287, !288, !290, !291, !293, !294}
!281 = distinct !{!281, !282, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build: %bytes"}
!282 = distinct !{!282, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2XfqmVweRya_18build_script_build"}
!283 = distinct !{!283, !284, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!284 = distinct !{!284, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!285 = distinct !{!285, !286, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %_0"}
!286 = distinct !{!286, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build"}
!287 = distinct !{!287, !286, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherAcj2_ENtB5_8Searcher4nextCs2XfqmVweRya_18build_script_build: %self"}
!288 = distinct !{!288, !289, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!289 = distinct !{!289, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!290 = distinct !{!290, !289, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherAcj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!291 = distinct !{!291, !292, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %_0"}
!292 = distinct !{!292, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build"}
!293 = distinct !{!293, !292, !"_RNvXsc_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_17CharArraySearcherKj2_ENtB5_8Searcher10next_matchCs2XfqmVweRya_18build_script_build: %self"}
!294 = distinct !{!294, !295, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build: %self"}
!295 = distinct !{!295, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalAcj2_E4nextCs2XfqmVweRya_18build_script_build"}
!296 = !{!297}
!297 = distinct !{!297, !298, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: argument 1"}
!298 = distinct !{!298, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix"}
!299 = !{!300}
!300 = distinct !{!300, !298, !"_RNvMsD_NtCsjMrxcFdYDNN_4core3numy16from_ascii_radix: %_0"}
!301 = !{!302}
!302 = distinct !{!302, !303, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg: argument 0"}
!303 = distinct !{!303, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg"}
!304 = !{!305}
!305 = distinct !{!305, !306, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!306 = distinct !{!306, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!307 = !{!308}
!308 = distinct !{!308, !309, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg: argument 0"}
!309 = distinct !{!309, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg"}
!310 = !{!311, !313}
!311 = distinct !{!311, !312, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!312 = distinct !{!312, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!313 = distinct !{!313, !312, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!314 = !{!315, !317}
!315 = distinct !{!315, !316, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!316 = distinct !{!316, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!317 = distinct !{!317, !316, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!318 = !{!319}
!319 = distinct !{!319, !320, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build: %_1"}
!320 = distinct !{!320, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs2XfqmVweRya_18build_script_build"}
!321 = !{!322}
!322 = distinct !{!322, !323, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg: argument 0"}
!323 = distinct !{!323, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg"}
!324 = !{!325, !327}
!325 = distinct !{!325, !326, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!326 = distinct !{!326, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!327 = distinct !{!327, !326, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!328 = !{!329, !331}
!329 = distinct !{!329, !330, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!330 = distinct !{!330, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!331 = distinct !{!331, !330, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!332 = !{!333, !335}
!333 = distinct !{!333, !334, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!334 = distinct !{!334, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!335 = distinct !{!335, !334, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!336 = !{!337, !339}
!337 = distinct !{!337, !338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!338 = distinct !{!338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!339 = distinct !{!339, !338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!340 = !{!341, !343}
!341 = distinct !{!341, !342, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!342 = distinct !{!342, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!343 = distinct !{!343, !342, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!344 = !{!345}
!345 = distinct !{!345, !346, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs2XfqmVweRya_18build_script_build: %_0"}
!346 = distinct !{!346, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs2XfqmVweRya_18build_script_build"}
!347 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!348 = !{!"branch_weights", i32 2146410443, i32 1073205}
!349 = !{!350, !352}
!350 = distinct !{!350, !351, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!351 = distinct !{!351, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!352 = distinct !{!352, !351, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!353 = !{!354, !356}
!354 = distinct !{!354, !355, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!355 = distinct !{!355, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!356 = distinct !{!356, !355, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!357 = !{!358, !360}
!358 = distinct !{!358, !359, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!359 = distinct !{!359, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!360 = distinct !{!360, !359, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!361 = !{!362, !364}
!362 = distinct !{!362, !363, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!363 = distinct !{!363, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!364 = distinct !{!364, !363, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!365 = !{!366, !368, !369, !371}
!366 = distinct !{!366, !367, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!367 = distinct !{!367, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!368 = distinct !{!368, !367, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!369 = distinct !{!369, !370, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!370 = distinct !{!370, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!371 = distinct !{!371, !370, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!372 = !{!373, !375}
!373 = distinct !{!373, !374, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: %self"}
!374 = distinct !{!374, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build"}
!375 = distinct !{!375, !374, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: argument 1"}
!376 = !{!377, !379, !380, !382}
!377 = distinct !{!377, !378, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!378 = distinct !{!378, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!379 = distinct !{!379, !378, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!380 = distinct !{!380, !381, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!381 = distinct !{!381, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!382 = distinct !{!382, !381, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!383 = !{!384, !386}
!384 = distinct !{!384, !385, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: %self"}
!385 = distinct !{!385, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build"}
!386 = distinct !{!386, !385, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: argument 1"}
!387 = !{!388, !390}
!388 = distinct !{!388, !389, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!389 = distinct !{!389, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!390 = distinct !{!390, !389, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!391 = !{!392, !394}
!392 = distinct !{!392, !393, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!393 = distinct !{!393, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!394 = distinct !{!394, !393, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!395 = !{!"branch_weights", !"expected", i32 2146410, i32 2145337238}
!396 = !{!397, !399}
!397 = distinct !{!397, !398, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!398 = distinct !{!398, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!399 = distinct !{!399, !398, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!400 = !{!401, !403}
!401 = distinct !{!401, !402, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!402 = distinct !{!402, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!403 = distinct !{!403, !402, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!404 = !{!405}
!405 = distinct !{!405, !406, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg: argument 0"}
!406 = distinct !{!406, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg"}
!407 = !{!408}
!408 = distinct !{!408, !409, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg: argument 0"}
!409 = distinct !{!409, !"_RNvCs2XfqmVweRya_18build_script_build7set_cfg"}
!410 = !{!411}
!411 = distinct !{!411, !412, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECs2XfqmVweRya_18build_script_build: %slice.0"}
!412 = distinct !{!412, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECs2XfqmVweRya_18build_script_build"}
!413 = !{!414, !416}
!414 = distinct !{!414, !415, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterReENtNtNtNtBa_4iter6traits8iterator8Iterator8try_foldjNCINvNtNtBS_8adapters3map12map_try_foldRBJ_jjINtNtBa_6option6OptionjENCNCINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehBJ_E00NvMs9_NtBa_3numj11checked_addE0B2k_ECs2XfqmVweRya_18build_script_build: %self"}
!415 = distinct !{!415, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterReENtNtNtNtBa_4iter6traits8iterator8Iterator8try_foldjNCINvNtNtBS_8adapters3map12map_try_foldRBJ_jjINtNtBa_6option6OptionjENCNCINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehBJ_E00NvMs9_NtBa_3numj11checked_addE0B2k_ECs2XfqmVweRya_18build_script_build"}
!416 = distinct !{!416, !412, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECs2XfqmVweRya_18build_script_build: %_0"}
!417 = !{!416, !411}
!418 = !{!"branch_weights", i32 2000, i32 6004}
!419 = !{!420, !416, !411}
!420 = distinct !{!420, !421, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs2XfqmVweRya_18build_script_build: %_0"}
!421 = distinct !{!421, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs2XfqmVweRya_18build_script_build"}
!422 = !{!416}
!423 = !{!424}
!424 = distinct !{!424, !425, !"_RNvXs2_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhEE11spec_extendCs2XfqmVweRya_18build_script_build: %self"}
!425 = distinct !{!425, !"_RNvXs2_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhEE11spec_extendCs2XfqmVweRya_18build_script_build"}
!426 = !{!427}
!427 = distinct !{!427, !428, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2XfqmVweRya_18build_script_build: %self"}
!428 = distinct !{!428, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2XfqmVweRya_18build_script_build"}
!429 = !{!427, !424}
!430 = !{!427, !424, !416, !411}
!431 = !{!432, !434}
!432 = distinct !{!432, !433, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build: %dest.0"}
!433 = distinct !{!433, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build"}
!434 = distinct !{!434, !433, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build: %src.0"}
!435 = !{!436, !438}
!436 = distinct !{!436, !437, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build: %dest.0"}
!437 = distinct !{!437, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build"}
!438 = distinct !{!438, !437, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECs2XfqmVweRya_18build_script_build: %src.0"}
!439 = !{!440, !442, !443, !445}
!440 = distinct !{!440, !441, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!441 = distinct !{!441, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!442 = distinct !{!442, !441, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!443 = distinct !{!443, !444, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!444 = distinct !{!444, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!445 = distinct !{!445, !444, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!446 = !{!447, !449}
!447 = distinct !{!447, !448, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: %self"}
!448 = distinct !{!448, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build"}
!449 = distinct !{!449, !448, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs2XfqmVweRya_18build_script_build: argument 1"}
!450 = !{i8 0, i8 2}
!451 = !{!452}
!452 = distinct !{!452, !453, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!453 = distinct !{!453, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!454 = !{!455}
!455 = distinct !{!455, !453, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!456 = !{!"branch_weights", i32 4000000, i32 4001}
!457 = !{!455, !452}
!458 = !{!459}
!459 = distinct !{!459, !460, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!460 = distinct !{!460, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!461 = !{!462, !464}
!462 = distinct !{!462, !463, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %self.0"}
!463 = distinct !{!463, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build"}
!464 = distinct !{!464, !463, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs2XfqmVweRya_18build_script_build: %other.0"}
!465 = !{!466}
!466 = distinct !{!466, !467, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs2XfqmVweRya_18build_script_build: %self"}
!467 = distinct !{!467, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs2XfqmVweRya_18build_script_build"}
!468 = !{!469}
!469 = distinct !{!469, !470, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %self"}
!470 = distinct !{!470, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build"}
!471 = !{!472}
!472 = distinct !{!472, !473, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs2XfqmVweRya_18build_script_build: %self"}
!473 = distinct !{!473, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs2XfqmVweRya_18build_script_build"}
!474 = !{!472, !469}
!475 = !{!476}
!476 = distinct !{!476, !473, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs2XfqmVweRya_18build_script_build: %_0"}
!477 = !{!476, !472, !469}
!478 = !{!479, !481, !469}
!479 = distinct !{!479, !480, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs2XfqmVweRya_18build_script_build: %_0"}
!480 = distinct !{!480, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs2XfqmVweRya_18build_script_build"}
!481 = distinct !{!481, !482, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %ret"}
!482 = distinct !{!482, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build"}
!483 = !{!481, !469}
!484 = !{!485}
!485 = distinct !{!485, !486, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %self"}
!486 = distinct !{!486, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build"}
!487 = !{!488}
!488 = distinct !{!488, !489, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs2XfqmVweRya_18build_script_build: %self"}
!489 = distinct !{!489, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs2XfqmVweRya_18build_script_build"}
!490 = !{!488, !485}
!491 = !{!492}
!492 = distinct !{!492, !486, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %_0"}
!493 = !{!494, !485}
!494 = distinct !{!494, !495, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs2XfqmVweRya_18build_script_build: %v"}
!495 = distinct !{!495, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs2XfqmVweRya_18build_script_build"}
!496 = !{!497, !492}
!497 = distinct !{!497, !495, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs2XfqmVweRya_18build_script_build: %ret"}
!498 = !{!488, !492, !485}
!499 = !{!494}
!500 = !{!501, !503, !504, !506, !497, !494, !492, !485}
!501 = distinct !{!501, !502, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %_0"}
!502 = distinct !{!502, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build"}
!503 = distinct !{!503, !502, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %self"}
!504 = distinct !{!504, !505, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs2XfqmVweRya_18build_script_build: %val"}
!505 = distinct !{!505, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs2XfqmVweRya_18build_script_build"}
!506 = distinct !{!506, !505, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs2XfqmVweRya_18build_script_build: %leaf_edge"}
!507 = !{!508, !510, !501, !503, !504, !506, !497, !494, !492, !485}
!508 = distinct !{!508, !509, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs2XfqmVweRya_18build_script_build: %_0"}
!509 = distinct !{!509, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs2XfqmVweRya_18build_script_build"}
!510 = distinct !{!510, !511, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build: %ret"}
!511 = distinct !{!511, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs2XfqmVweRya_18build_script_build"}
!512 = !{!513, !515, !501, !503, !504, !506, !497, !494, !492, !485}
!513 = distinct !{!513, !514, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs2XfqmVweRya_18build_script_build: %_0"}
!514 = distinct !{!514, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs2XfqmVweRya_18build_script_build"}
!515 = distinct !{!515, !514, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs2XfqmVweRya_18build_script_build: %self"}
!516 = !{!510, !501, !503, !504, !506, !497, !494, !492, !485}
!517 = !{!497, !494, !492, !485}
!518 = !{!492, !485}
!519 = !{!520}
!520 = distinct !{!520, !521, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build: %self"}
!521 = distinct !{!521, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build"}
!522 = !{!523}
!523 = distinct !{!523, !521, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionReENtNtB7_3fmt5Debug3fmtCs2XfqmVweRya_18build_script_build: %f"}
!524 = !{!520, !523}
!525 = !{i8 0, i8 5}
!526 = !{!527}
!527 = distinct !{!527, !528, !"_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build: %self"}
!528 = distinct !{!528, !"_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build"}
!529 = !{!530}
!530 = distinct !{!530, !528, !"_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs2XfqmVweRya_18build_script_build: %f"}
