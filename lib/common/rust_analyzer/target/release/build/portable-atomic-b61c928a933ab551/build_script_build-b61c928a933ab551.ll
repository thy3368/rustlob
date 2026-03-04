; ModuleID = 'build_script_build.fac099121c4e46b2-cgu.0'
source_filename = "build_script_build.fac099121c4e46b2-cgu.0"
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
@alloc_7fe94be2e120ffbd80c490b1b3c481ee = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/pattern.rs\00", align 1
@alloc_3c3a438693b52af6c6b31c2cc77620da = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\E4\05\00\00\14\00\00\00" }>, align 8
@alloc_759b6db6182a2ae5f8169b55f322d553 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\E4\05\00\00!\00\00\00" }>, align 8
@alloc_cfc145f12794171662ae0bd5e97799ce = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\D8\05\00\00!\00\00\00" }>, align 8
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCslwKqnJYeWCA_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CslwKqnJYeWCA_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CslwKqnJYeWCA_18build_script_build }>, align 8
@alloc_c7175637917997c04b44e3b2099eecca = private unnamed_addr constant [113 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/str.rs\00", align 1
@alloc_3f2fbfdca196a5b824209b380ee7ae1b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c7175637917997c04b44e3b2099eecca, [16 x i8] c"p\00\00\00\00\00\00\00\B1\00\00\00\16\00\00\00" }>, align 8
@alloc_ca673fb95acb8e58af271999e89294ae = private unnamed_addr constant [53 x i8] c"attempt to join into collection with len > usize::MAX", align 1
@alloc_60488e92c3d9250777708a132d567f7b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c7175637917997c04b44e3b2099eecca, [16 x i8] c"p\00\00\00\00\00\00\00\9A\00\00\00\0A\00\00\00" }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_1d8e296db13ccc3871d7443d7beb0b2f = private unnamed_addr constant [14 x i8] c"LLVM version: ", align 1
@alloc_ed740c9299d7724b54ca9f50b8396b41 = private unnamed_addr constant [13 x i8] c"commit-date: ", align 1
@alloc_0c3cbb61e273e9decf597461734e2fad = private unnamed_addr constant [5 x i8] c"thumb", align 1
@alloc_3e61858716336dfbf74d71ab75462972 = private unnamed_addr constant [6 x i8] c"apple-", align 1
@alloc_b7185c5006b7edc9741130ed35f40cf6 = private unnamed_addr constant [9 x i8] c"release: ", align 1
@alloc_c522aef10584118522ad11d67bba8829 = private unnamed_addr constant [106 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/portable-atomic-1.11.1/version.rs\00", align 1
@alloc_954cd85162064372f588eb6b0cd0c8e9 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c522aef10584118522ad11d67bba8829, [16 x i8] c"i\00\00\00\00\00\00\00U\00\00\00\22\00\00\00" }>, align 8
@alloc_dda1ee2b88b89b9cdac753eef7988035 = private unnamed_addr constant [1 x i8] c"0", align 1
@alloc_5a70f0d82a4daf23c5a2b55427a97995 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c522aef10584118522ad11d67bba8829, [16 x i8] c"i\00\00\00\00\00\00\00d\00\00\00&\00\00\00" }>, align 8
@alloc_07f3eec4949a8d39db630a4a477c65b3 = private unnamed_addr constant [23 x i8] c"CARGO_ENCODED_RUSTFLAGS", align 1
@alloc_13ae1be13e6486d5c310a371cde21787 = private unnamed_addr constant [2 x i8] c"-C", align 1
@alloc_28b6e12cbb2608b0ebceb37ca6d5945a = private unnamed_addr constant [11 x i8] c"target-cpu=", align 1
@alloc_987495d299690d318d890dac34727c75 = private unnamed_addr constant [104 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/portable-atomic-1.11.1/build.rs\00", align 1
@alloc_b9a6efcfeb17b646c43fe2783a9632a7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00%\02\00\00$\00\00\00" }>, align 8
@alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26 = private unnamed_addr constant [11 x i8] c"RUSTC_STAGE", align 1
@alloc_9bc7f23b9f8ca5426d0539d95c9ebfc5 = private unnamed_addr constant [2 x i8] c"-Z", align 1
@alloc_3f5a3f432cc037101cfa0283db269c57 = private unnamed_addr constant [15 x i8] c"allow-features=", align 1
@alloc_b6e2898a10ad3df62e798ba959034459 = private unnamed_addr constant [15 x i8] c"target-feature=", align 1
@alloc_1545dd372b2bbe07420f0d25b9c1ecbe = private unnamed_addr constant [54 x i8] c"0cargo:rustc-cfg=portable_atomic_target_feature=\22\C0\02\22\0A\00", align 1
@alloc_70a1e7dc3879e83c39c209c1ae5f1722 = private unnamed_addr constant [5 x i8] c"linux", align 1
@alloc_14c43fe6be9850e9c6ac099b83b2e4e2 = private unnamed_addr constant [7 x i8] c"unknown", align 1
@alloc_742f06589122110502429e832b81e8bd = private unnamed_addr constant [32 x i8] c"cargo:rerun-if-changed=build.rs\0A", align 1
@alloc_8724dc04cf16cdfa205f2534ce59cc5f = private unnamed_addr constant [40 x i8] c"cargo:rerun-if-changed=src/gen/build.rs\0A", align 1
@alloc_9529fcb5bab6980e7c00a1d3aa85ad20 = private unnamed_addr constant [34 x i8] c"cargo:rerun-if-changed=version.rs\0A", align 1
@alloc_dcbc225a8ec7dbfaaef714ff8a7176fb = private unnamed_addr constant [6 x i8] c"TARGET", align 1
@alloc_1f011f14125487663894f05c404929a9 = private unnamed_addr constant [14 x i8] c"TARGET not set", align 1
@alloc_84ab129184af97b3019d993fd0b7c805 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00\1E\00\00\00'\00\00\00" }>, align 8
@alloc_0d3bcf6fb685f000bc18304ea76cbac4 = private unnamed_addr constant [21 x i8] c"CARGO_CFG_TARGET_ARCH", align 1
@alloc_86b01a9b169de421b373ac989e740d38 = private unnamed_addr constant [29 x i8] c"CARGO_CFG_TARGET_ARCH not set", align 1
@alloc_453d9fa5d8e2ccad25a908c230c60eb0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00\1F\00\00\00;\00\00\00" }>, align 8
@alloc_aa4687de82972c6f88dd4ebd068e3b63 = private unnamed_addr constant [19 x i8] c"CARGO_CFG_TARGET_OS", align 1
@alloc_b7c7e69e15013ee480a39c67f380d6cc = private unnamed_addr constant [27 x i8] c"CARGO_CFG_TARGET_OS not set", align 1
@alloc_57f734e358c6dd0a67f2db5255d1a5b4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00 \00\00\007\00\00\00" }>, align 8
@alloc_0afe6f0504dd6bb7e0912d0ebba4969c = private unnamed_addr constant [29 x i8] c"PORTABLE_ATOMIC_DENY_WARNINGS", align 1
@alloc_c6d8f7bad6fe84b0a1fb1ad39cccc87c = private unnamed_addr constant [33 x i8] c"unable to determine rustc version", align 1
@alloc_f56fcc3b591a05dcb59c62f94980d9e5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00&\00\00\00\11\00\00\00" }>, align 8
@alloc_2f2a7b3b6dd8e6a4ec2f251266b0daff = private unnamed_addr constant <{ [13 x i8], [3 x i8] }> <{ [13 x i8] c"W\00\00\00\00\00\00\00\12\00\00\00\00", [3 x i8] undef }>, align 4
@alloc_9b5fa806b1b67f7699cdf95aedfa7081 = private unnamed_addr constant [104 x i8] c"bcargo:warning=portable-atomic: unable to determine rustc version; assuming latest stable rustc (1.\C0\02)\0A\00", align 1
@alloc_092aad2bc9b61c4864788da0a3d09052 = private unnamed_addr constant [143 x i8] c"cargo:rustc-check-cfg=cfg(target_feature,values(\22lsfe\22,\22fast-serialization\22,\22load-store-on-cond\22,\22distinct-ops\22,\22miscellaneous-extensions-3\22))\0A", align 1
@alloc_3c9e784e7e4614c4b58cf1fcdd1319d2 = private unnamed_addr constant [1227 x i8] c"cargo:rustc-check-cfg=cfg(portable_atomic_disable_fiq,portable_atomic_force_amo,portable_atomic_ll_sc_rmw,portable_atomic_atomic_intrinsics,portable_atomic_no_asm,portable_atomic_no_asm_maybe_uninit,portable_atomic_no_atomic_64,portable_atomic_no_atomic_cas,portable_atomic_no_atomic_load_store,portable_atomic_no_atomic_min_max,portable_atomic_no_cfg_target_has_atomic,portable_atomic_no_cmpxchg16b_intrinsic,portable_atomic_no_cmpxchg16b_target_feature,portable_atomic_no_const_mut_refs,portable_atomic_no_const_raw_ptr_deref,portable_atomic_no_const_transmute,portable_atomic_no_core_unwind_safe,portable_atomic_no_diagnostic_namespace,portable_atomic_no_offset_of,portable_atomic_no_strict_provenance,portable_atomic_no_stronger_failure_ordering,portable_atomic_no_track_caller,portable_atomic_no_unsafe_op_in_unsafe_fn,portable_atomic_pre_llvm_15,portable_atomic_pre_llvm_16,portable_atomic_pre_llvm_18,portable_atomic_pre_llvm_20,portable_atomic_s_mode,portable_atomic_sanitize_thread,portable_atomic_target_feature,portable_atomic_unsafe_assume_single_core,portable_atomic_unstable_asm,portable_atomic_unstable_asm_experimental_arch,portable_atomic_unstable_cfg_target_has_atomic,portable_atomic_unstable_isa_attribute)\0A", align 1
@alloc_9870bcf023b2efb7b8b18a97902cd1ef = private unnamed_addr constant [259 x i8] c"cargo:rustc-check-cfg=cfg(portable_atomic_target_feature,values(\22cmpxchg16b\22,\22distinct-ops\22,\22fast-serialization\22,\22load-store-on-cond\22,\22lse\22,\22lse128\22,\22lse2\22,\22lsfe\22,\22mclass\22,\22miscellaneous-extensions-3\22,\22quadword-atomics\22,\22rcpc3\22,\22v6\22,\22zaamo\22,\22zabha\22,\22zacas\22))\0A", align 1
@alloc_d47a354a607bf77ebae5daba39357d52 = private unnamed_addr constant [51 x i8] c"cargo:rerun-if-env-changed=CARGO_ENCODED_RUSTFLAGS\0A", align 1
@alloc_26aa35ac1fd69f511512a9380c2223a9 = private unnamed_addr constant [37 x i8] c"cargo:rerun-if-env-changed=RUSTFLAGS\0A", align 1
@alloc_5cdf6a79c62811efa8c483f97a5cc917 = private unnamed_addr constant [49 x i8] c"cargo:rerun-if-env-changed=CARGO_BUILD_RUSTFLAGS\0A", align 1
@alloc_c22f04a0140b0ae1314c125358058abd = private unnamed_addr constant [55 x i8] c"(cargo:rerun-if-env-changed=CARGO_TARGET_\C0\0B_RUSTFLAGS\0A\00", align 1
@alloc_1fed9b908bc75f0483fa23e0a5540e7b = private unnamed_addr constant [50 x i8] c"cargo:rustc-cfg=portable_atomic_no_atomic_min_max\0A", align 1
@alloc_ad12ff8fc6e049ff439008f27922a354 = private unnamed_addr constant [48 x i8] c"cargo:rustc-cfg=portable_atomic_no_track_caller\0A", align 1
@alloc_c1341c958305d557d0587cd9fe34f2c5 = private unnamed_addr constant [58 x i8] c"cargo:rustc-cfg=portable_atomic_no_unsafe_op_in_unsafe_fn\0A", align 1
@alloc_6833eaf1a79a5bbdf426dcbf1dc565a9 = private unnamed_addr constant [51 x i8] c"cargo:rustc-cfg=portable_atomic_no_const_transmute\0A", align 1
@alloc_4eea02266a7f159030b82ab70c4915d3 = private unnamed_addr constant [52 x i8] c"cargo:rustc-cfg=portable_atomic_no_core_unwind_safe\0A", align 1
@alloc_36b7da1b4ea2bacdb83a39b716c9e7e2 = private unnamed_addr constant [55 x i8] c"cargo:rustc-cfg=portable_atomic_no_const_raw_ptr_deref\0A", align 1
@alloc_11a9245b51bd70ca041d4c95e4e22ec5 = private unnamed_addr constant [61 x i8] c"cargo:rustc-cfg=portable_atomic_no_stronger_failure_ordering\0A", align 1
@alloc_42144296fbd1e57a517c011ba5e34db7 = private unnamed_addr constant [52 x i8] c"cargo:rustc-cfg=portable_atomic_no_asm_maybe_uninit\0A", align 1
@alloc_5d94a3a01eeede3af4fbbee4226859e6 = private unnamed_addr constant [45 x i8] c"cargo:rustc-cfg=portable_atomic_no_offset_of\0A", align 1
@alloc_5831c002d8b741107bcdaec0fae41316 = private unnamed_addr constant [56 x i8] c"cargo:rustc-cfg=portable_atomic_no_diagnostic_namespace\0A", align 1
@alloc_2e9b5516734e64b9676c629692898b2f = private unnamed_addr constant [50 x i8] c"cargo:rustc-cfg=portable_atomic_no_const_mut_refs\0A", align 1
@alloc_638a7631d4f759bf1aef9d06869234dd = private unnamed_addr constant [53 x i8] c"cargo:rustc-cfg=portable_atomic_no_strict_provenance\0A", align 1
@alloc_fa1130f2f45123ef906740f12b430906 = private unnamed_addr constant [9 x i8] c"powerpc64", align 1
@alloc_2c4353bbb852c5a5c07b390c882774d5 = private unnamed_addr constant [50 x i8] c"cargo:rustc-cfg=portable_atomic_atomic_intrinsics\0A", align 1
@alloc_77091ef4013986fd40216f126dabc12f = private unnamed_addr constant [7 x i8] c"arm64ec", align 1
@alloc_5a449a0bdb20d0cd84a204297ad784b7 = private unnamed_addr constant [5 x i8] c"s390x", align 1
@alloc_d56840a3539592a0b21e632750debc01 = private unnamed_addr constant [21 x i8] c"asm_experimental_arch", align 1
@alloc_a416d606bb0533c9dea6d062122086f5 = private unnamed_addr constant [63 x i8] c"cargo:rustc-cfg=portable_atomic_unstable_asm_experimental_arch\0A", align 1
@alloc_4a7c5e08e7b916cb34cb4d73d8036fc8 = private unnamed_addr constant [39 x i8] c"cargo:rustc-cfg=portable_atomic_no_asm\0A", align 1
@alloc_fba164e1a5f38e2d8018bcf65720d955 = private unnamed_addr constant [3 x i8] c"x86", align 1
@alloc_4a29a4faa0904cd7ff982831f2813e90 = private unnamed_addr constant [6 x i8] c"x86_64", align 1
@alloc_fb30e4df55a4a7c53eb78eab864290eb = private unnamed_addr constant [3 x i8] c"asm", align 1
@alloc_1406cfd7adf5bf37af423094d906069c = private unnamed_addr constant [45 x i8] c"cargo:rustc-cfg=portable_atomic_unstable_asm\0A", align 1
@alloc_50c6ea2673204147b4c7c6266b4cb1e4 = private unnamed_addr constant [21 x i8] c"cfg_target_has_atomic", align 1
@alloc_2c12234dbe8e690864c2c0a679e3cd05 = private unnamed_addr constant [57 x i8] c"cargo:rustc-cfg=portable_atomic_no_cfg_target_has_atomic\0A", align 1
@alloc_0a8846dfe80e6e9ba5109a5921697c29 = private unnamed_addr constant [46 x i8] c"cargo:rustc-cfg=portable_atomic_no_atomic_cas\0A", align 1
@alloc_8e16d26229e79ba7f22bd47d524058bd = private unnamed_addr constant [45 x i8] c"cargo:rustc-cfg=portable_atomic_no_atomic_64\0A", align 1
@alloc_9e6ac5f3f7d9693fbc40f6727b337014 = private unnamed_addr constant [63 x i8] c"cargo:rustc-cfg=portable_atomic_unstable_cfg_target_has_atomic\0A", align 1
@alloc_f0dd653f0ec1ecef8fb1fc4d13d313c0 = private unnamed_addr constant [53 x i8] c"cargo:rustc-cfg=portable_atomic_no_atomic_load_store\0A", align 1
@alloc_94cefc1b63dffcfbd691138662a4abf1 = private unnamed_addr constant [44 x i8] c"cargo:rustc-cfg=portable_atomic_pre_llvm_20\0A", align 1
@alloc_d611028f9649b3783e472c0e81960e47 = private unnamed_addr constant [44 x i8] c"cargo:rustc-cfg=portable_atomic_pre_llvm_18\0A", align 1
@alloc_9050225349f21addcf6eebdb0ba383e4 = private unnamed_addr constant [44 x i8] c"cargo:rustc-cfg=portable_atomic_pre_llvm_16\0A", align 1
@alloc_8d616064d0ee2b338ed7caf95f46cce6 = private unnamed_addr constant [44 x i8] c"cargo:rustc-cfg=portable_atomic_pre_llvm_15\0A", align 1
@alloc_1bde45392581cd9043bc066293d5f001 = private unnamed_addr constant [18 x i8] c"CARGO_CFG_SANITIZE", align 1
@alloc_0ccf5eeb19a73b85efabd846cfd6625c = private unnamed_addr constant [6 x i8] c"thread", align 1
@alloc_d1324d222df80959476a7b359dea6188 = private unnamed_addr constant [48 x i8] c"cargo:rustc-cfg=portable_atomic_sanitize_thread\0A", align 1
@alloc_708437d7a9a3b1bed2b2fbb27ca99947 = private unnamed_addr constant [7 x i8] c"aarch64", align 1
@alloc_d9036dbef1cc78d0c3562113c2babf56 = private unnamed_addr constant [3 x i8] c"arm", align 1
@alloc_82a51f3810ec0ab0cde96eb28e0a8f16 = private unnamed_addr constant [7 x i8] c"riscv32", align 1
@alloc_f566f2e0543c30db53174190aea17def = private unnamed_addr constant [7 x i8] c"riscv64", align 1
@alloc_fcc17c744c1d97967e2fd259ede87770 = private unnamed_addr constant [4 x i8] c"arch", align 1
@alloc_6381f5c37a2a38a20bd4bf05e9496d96 = private unnamed_addr constant [4 x i8] c"z196", align 1
@alloc_2a818f10c905893af1e371c88f7d0169 = private unnamed_addr constant [5 x i8] c"zEC12", align 1
@alloc_fb3f24020003f3819dcc1e59fb1e917c = private unnamed_addr constant [3 x i8] c"z13", align 1
@alloc_3cb0e3a0d99149dd91748ce9aa02491e = private unnamed_addr constant [3 x i8] c"z14", align 1
@alloc_b287530f63e31393d0bcb611dd2ef350 = private unnamed_addr constant [3 x i8] c"z15", align 1
@alloc_6d1d74389733cf20cb433539a0c6ad2d = private unnamed_addr constant [3 x i8] c"z16", align 1
@alloc_1b0f75a7cb5e484bd78be04806919b14 = private unnamed_addr constant [18 x i8] c"fast-serialization", align 1
@alloc_b685305cf7511b16b4281b53804013c9 = private unnamed_addr constant [18 x i8] c"load-store-on-cond", align 1
@alloc_fb9a0f244e913c69309d6ba532bf7b0c = private unnamed_addr constant [12 x i8] c"distinct-ops", align 1
@alloc_d66b25e4b56c5d1a43b7571f92983e4a = private unnamed_addr constant [26 x i8] c"miscellaneous-extensions-3", align 1
@alloc_ac7dbe20bf8ff4864d2af9d0f6d43781 = private unnamed_addr constant [3 x i8] c"pwr", align 1
@alloc_599e99bb598e572a5316e19a07ba6538 = private unnamed_addr constant [6 x i8] c"future", align 1
@alloc_34414b8d2a3366cb3bf8126f7ed8d762 = private unnamed_addr constant [7 x i8] c"ppc64le", align 1
@alloc_0fa298ef16b44f409ee8019af1deb36d = private unnamed_addr constant [23 x i8] c"CARGO_CFG_TARGET_ENDIAN", align 1
@alloc_48b8864556065fe7b21fa91d6bd43bb1 = private unnamed_addr constant [31 x i8] c"CARGO_CFG_TARGET_ENDIAN not set", align 1
@alloc_766b25183a26c8232979e8e97ad1aa67 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00\98\01\00\00\1A\00\00\00" }>, align 8
@alloc_4ab12fe6a700d822e8674db3fe2ff0c9 = private unnamed_addr constant [6 x i8] c"little", align 1
@alloc_42a2d297b45bdf881e84f224970518a9 = private unnamed_addr constant [16 x i8] c"quadword-atomics", align 1
@alloc_5a97a820ee50c370898f0445e27bf764 = private unnamed_addr constant [5 x i8] c"zacas", align 1
@alloc_1f93ada5207dde6875e7ab77a5bb8dbe = private unnamed_addr constant [5 x i8] c"zabha", align 1
@alloc_9e015abacf20cb399b4f0a804bc0e15e = private unnamed_addr constant [5 x i8] c"zaamo", align 1
@alloc_983b9f659255e384bca2078a0c05baf3 = private unnamed_addr constant [55 x i8] c"cargo:rustc-cfg=portable_atomic_unstable_isa_attribute\0A", align 1
@alloc_9808848e4b0f0dfce430f2ce907c2ec4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00>\01\00\00[\00\00\00" }>, align 8
@alloc_5d5c3f5b03c6d3586ba34aa6bd6df864 = private unnamed_addr constant [2 x i8] c"eb", align 1
@alloc_ee08f4448ec37f2c785795512d1d6371 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00@\01\00\005\00\00\00" }>, align 8
@alloc_b0cad33fb230cb36b707608c08d32517 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00A\01\00\005\00\00\00" }>, align 8
@alloc_b655ffc265eabd5b7c4618f72368ce58 = private unnamed_addr constant [2 x i8] c"v7", align 1
@alloc_876e28ca4ef18e92a168b8b6c0258020 = private unnamed_addr constant [3 x i8] c"v7a", align 1
@alloc_3871c7a1d848c9c811d08e9ba480e39d = private unnamed_addr constant [6 x i8] c"v7neon", align 1
@alloc_8a148dfbf624d86ad479275878ea55de = private unnamed_addr constant [3 x i8] c"v7s", align 1
@alloc_620d80f9f3d64bd5a4c2be88c9b298d5 = private unnamed_addr constant [3 x i8] c"v7k", align 1
@alloc_98dd345a480634a67ab490269b3092e8 = private unnamed_addr constant [2 x i8] c"v8", align 1
@alloc_8bc0c6258d5b796bc8931ca41d054010 = private unnamed_addr constant [3 x i8] c"v8a", align 1
@alloc_766309e861a41c77d8e1ec8d2bf7d3d1 = private unnamed_addr constant [2 x i8] c"v9", align 1
@alloc_731bb66c58a2091aa6837f3ec33ccbd0 = private unnamed_addr constant [3 x i8] c"v9a", align 1
@alloc_46d8ec96465309b380b25451978e5b89 = private unnamed_addr constant [3 x i8] c"v7r", align 1
@alloc_8e5d4739a815f492da01ca41d793f679 = private unnamed_addr constant [3 x i8] c"v8r", align 1
@alloc_896376205399067eda0bb4872a2b2e1a = private unnamed_addr constant [3 x i8] c"v9r", align 1
@alloc_39d6b4d2e142be3718c9abbba15e676b = private unnamed_addr constant [3 x i8] c"v6m", align 1
@alloc_6f0318d77bf4620e64f2b718a0fed714 = private unnamed_addr constant [4 x i8] c"v7em", align 1
@alloc_6fe9401313774126d244c9df8f281e77 = private unnamed_addr constant [3 x i8] c"v7m", align 1
@alloc_094ba35790031d8096d74b82ea129258 = private unnamed_addr constant [3 x i8] c"v8m", align 1
@alloc_7ef794223a3c79a90832fd404d7462c3 = private unnamed_addr constant [21 x i8] c"arm-linux-androideabi", align 1
@alloc_efe094782a10f2611a22702571dbac01 = private unnamed_addr constant [27 x i8] c"armeb-unknown-linux-gnueabi", align 1
@alloc_e01da671082aa58f6e6c733a052567ea = private unnamed_addr constant [3 x i8] c"v4t", align 1
@alloc_cc3bb8f2b93f3b6ad6e65fe09616c7dd = private unnamed_addr constant [4 x i8] c"v5te", align 1
@alloc_6785c718eb5d42ff6741fe7de88e57ea = private unnamed_addr constant [2 x i8] c"v6", align 1
@alloc_c8a1a35b0978064484413724586714e9 = private unnamed_addr constant [3 x i8] c"v6k", align 1
@alloc_fbcccdd8daac3e36c025f53437f4178b = private unnamed_addr constant [29 x i8] c"\1Aunrecognized Arm subarch: \C0\00", align 1
@alloc_83de034d7ccdc84ce78692abaea10898 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_987495d299690d318d890dac34727c75, [16 x i8] c"g\00\00\00\00\00\00\00U\01\00\00\1D\00\00\00" }>, align 8
@alloc_cf6e7c65e8353dc4e21daa402c81c63c = private unnamed_addr constant [62 x i8] c"9cargo:warning=portable-atomic: unrecognized Arm subarch: \C0\01\0A\00", align 1
@alloc_da78d7c202eb238c4cdf0045a21de1ef = private unnamed_addr constant [6 x i8] c"mclass", align 1
@alloc_8a32b15b0e78e52f3e29895e5f677744 = private unnamed_addr constant [5 x i8] c"macos", align 1
@alloc_ff4976f557f9331cf80413f038ea83af = private unnamed_addr constant [4 x i8] c"lse2", align 1
@alloc_96fd63ed221128cfefca2794b4bdbd59 = private unnamed_addr constant [6 x i8] c"lse128", align 1
@alloc_d21745838991a4cb2caa18b5ddb8dc43 = private unnamed_addr constant [5 x i8] c"rcpc3", align 1
@alloc_e3abda62d4f863fc9e691690c373f274 = private unnamed_addr constant [3 x i8] c"lse", align 1
@alloc_d1ae99d9a9853d638c28f3dedea5a4ac = private unnamed_addr constant [4 x i8] c"lsfe", align 1
@alloc_43a58bdcc2572eb4426ba7ac54f3fdc3 = private unnamed_addr constant [23 x i8] c"CARGO_CFG_TARGET_VENDOR", align 1
@alloc_f5fa10d2bd50b965d2515db045847aab = private unnamed_addr constant [5 x i8] c"apple", align 1
@alloc_236339e905ce8c9e5823d1ccff1bb370 = private unnamed_addr constant [42 x i8] c"cargo:rustc-cfg=portable_atomic_ll_sc_rmw\0A", align 1
@alloc_ee297b014715a704a78587d10e2d5209 = private unnamed_addr constant [61 x i8] c"cargo:rustc-cfg=portable_atomic_no_cmpxchg16b_target_feature\0A", align 1
@alloc_e1f6422a9d16410b776670e5fce196a5 = private unnamed_addr constant [56 x i8] c"cargo:rustc-cfg=portable_atomic_no_cmpxchg16b_intrinsic\0A", align 1
@alloc_79633b065602e14b572fa6ec5d7a5307 = private unnamed_addr constant [10 x i8] c"cmpxchg16b", align 1
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsk_NtCs5sEH5CPMdak_3std3envNtB5_8VarErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@alloc_1a49731e8f6ceed5b16a30308d388517 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c522aef10584118522ad11d67bba8829, [16 x i8] c"i\00\00\00\00\00\00\00?\00\00\00\1E\00\00\00" }>, align 8
@alloc_c6e9e0ae8c6937b3491e661c604ec907 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c522aef10584118522ad11d67bba8829, [16 x i8] c"i\00\00\00\00\00\00\00A\00\00\00&\00\00\00" }>, align 8
@alloc_d563101362ed4a06747b9210d55c4c5b = private unnamed_addr constant [15 x i8] c"RUSTC_BOOTSTRAP", align 1
@alloc_24b05860d599286ecb26af7d40c66ae9 = private unnamed_addr constant [2 x i8] c"-1", align 1
@alloc_22ec252afd5f5781ca8ee9b115d4a0d6 = private unnamed_addr constant [7 x i8] c"nightly", align 1
@alloc_12dcbe319bdb437b2d068742d0ee3321 = private unnamed_addr constant [3 x i8] c"dev", align 1
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_f36ce88bd5d4a921175f5521f484b675 = private unnamed_addr constant [13 x i8] c"RUSTC_WRAPPER", align 1
@alloc_9994c89853702f537928cd873913af0d = private unnamed_addr constant [3 x i8] c"-vV", align 1
@alloc_60041a2996b9889a1b040c54c616ffa4 = private unnamed_addr constant [18 x i8] c"armebv7r-none-eabi", align 1
@alloc_fc4b0cb0b48da575f740f3b061247e0f = private unnamed_addr constant [20 x i8] c"armebv7r-none-eabihf", align 1
@alloc_3e1a5023273d14d3525522c2099080bd = private unnamed_addr constant [28 x i8] c"armv4t-unknown-linux-gnueabi", align 1
@alloc_d854eb1f96568158508a1fe06acc479b = private unnamed_addr constant [29 x i8] c"armv5te-unknown-linux-gnueabi", align 1
@alloc_ab31f0ee4c349cacd5df9f59f007a5ed = private unnamed_addr constant [30 x i8] c"armv5te-unknown-linux-musleabi", align 1
@alloc_82d439c4b4b9541b054ad05c5b792d5a = private unnamed_addr constant [32 x i8] c"armv5te-unknown-linux-uclibceabi", align 1
@alloc_c1e04960e461045b24da3143b9bbe57e = private unnamed_addr constant [19 x i8] c"armv6k-nintendo-3ds", align 1
@alloc_828ae20c2ae220c38b3245d3e4d32e82 = private unnamed_addr constant [16 x i8] c"armv7r-none-eabi", align 1
@alloc_7b4c0892dcb20ad38838433c6bd13c7a = private unnamed_addr constant [18 x i8] c"armv7r-none-eabihf", align 1
@alloc_073a41cfae022637d06fd85a5e605fab = private unnamed_addr constant [25 x i8] c"avr-unknown-gnu-atmega328", align 1
@alloc_077b257855cc8302e2e37029c956e3c3 = private unnamed_addr constant [26 x i8] c"hexagon-unknown-linux-musl", align 1
@alloc_9bc964707defe2160c729e1bb0b7c1e9 = private unnamed_addr constant [22 x i8] c"m68k-unknown-linux-gnu", align 1
@alloc_22b33466e0dd64c3b2e2ee15ff8d195c = private unnamed_addr constant [22 x i8] c"mips-unknown-linux-gnu", align 1
@alloc_3f54fddd3386141ea42568fa33ba0fc0 = private unnamed_addr constant [23 x i8] c"mips-unknown-linux-musl", align 1
@alloc_53bb287215d6b7ca5d0e83638974f2dc = private unnamed_addr constant [25 x i8] c"mips-unknown-linux-uclibc", align 1
@alloc_670ec80a76e3dda8c0e4bb964f752aae = private unnamed_addr constant [15 x i8] c"mipsel-sony-psp", align 1
@alloc_4275caebb106965f8ef5bb2811245ced = private unnamed_addr constant [24 x i8] c"mipsel-unknown-linux-gnu", align 1
@alloc_a306aa267a229f9192f581f5c4b3f8a5 = private unnamed_addr constant [25 x i8] c"mipsel-unknown-linux-musl", align 1
@alloc_5b2e73936d9ed25120ae3cd1b1d86750 = private unnamed_addr constant [27 x i8] c"mipsel-unknown-linux-uclibc", align 1
@alloc_addf2b8c5bb0c785f64e828bb16710c9 = private unnamed_addr constant [19 x i8] c"mipsel-unknown-none", align 1
@alloc_f92ab8510275e5e42dd14554da4d5f0d = private unnamed_addr constant [29 x i8] c"mipsisa32r6-unknown-linux-gnu", align 1
@alloc_0c0bdd2040e1d3b535a15670bed8e2ee = private unnamed_addr constant [31 x i8] c"mipsisa32r6el-unknown-linux-gnu", align 1
@alloc_9a3299e3caae06e665af83a2a067eb68 = private unnamed_addr constant [15 x i8] c"msp430-none-elf", align 1
@alloc_5e3a4f7a117d2e7bbe7456c524bb0006 = private unnamed_addr constant [23 x i8] c"powerpc-unknown-freebsd", align 1
@alloc_edfd6be1acee3e3854f172faab285c38 = private unnamed_addr constant [25 x i8] c"powerpc-unknown-linux-gnu", align 1
@alloc_1f2300ead6011ba2b5b331f2b76eb516 = private unnamed_addr constant [28 x i8] c"powerpc-unknown-linux-gnuspe", align 1
@alloc_baac72b63c55b5842a31e2c00a7d0883 = private unnamed_addr constant [26 x i8] c"powerpc-unknown-linux-musl", align 1
@alloc_0d8d35c4ec871ff7e6c083e1cc59762d = private unnamed_addr constant [22 x i8] c"powerpc-unknown-netbsd", align 1
@alloc_ad7f1342255ebef4377308b28b295ab3 = private unnamed_addr constant [23 x i8] c"powerpc-unknown-openbsd", align 1
@alloc_4c78f2281b95c444848389f653bfe1aa = private unnamed_addr constant [19 x i8] c"powerpc-wrs-vxworks", align 1
@alloc_2e417c01fc50dde593c75f17df443aac = private unnamed_addr constant [23 x i8] c"powerpc-wrs-vxworks-spe", align 1
@alloc_ba03a0b15f58439199c9c1fb605fb91b = private unnamed_addr constant [27 x i8] c"riscv32gc-unknown-linux-gnu", align 1
@alloc_1689bd3b01b5acc98a70228e04d83498 = private unnamed_addr constant [28 x i8] c"riscv32gc-unknown-linux-musl", align 1
@alloc_ecbfecbf042f5d02ed1d945b2da736d2 = private unnamed_addr constant [25 x i8] c"riscv32i-unknown-none-elf", align 1
@alloc_b7513f3459de499a3785fd3ab62ad6ab = private unnamed_addr constant [28 x i8] c"riscv32imac-unknown-none-elf", align 1
@alloc_4aac2c751578ff8b2a7da0743e6f1498 = private unnamed_addr constant [21 x i8] c"riscv32imc-esp-espidf", align 1
@alloc_fd1b38744aa12ca58723e85c5996f9f0 = private unnamed_addr constant [27 x i8] c"riscv32imc-unknown-none-elf", align 1
@alloc_603fc8e29c06fe139773a6b784e3cc49 = private unnamed_addr constant [18 x i8] c"thumbv4t-none-eabi", align 1
@alloc_0f444773a3c04e447512a602928cccf9 = private unnamed_addr constant [18 x i8] c"thumbv6m-none-eabi", align 1
@alloc_3619ae5c84ee9a44040d6ebb9b6b060d = private unnamed_addr constant [19 x i8] c"thumbv7em-none-eabi", align 1
@alloc_d90900e16a07082031ab037fc65b138f = private unnamed_addr constant [21 x i8] c"thumbv7em-none-eabihf", align 1
@alloc_a6ec35165b5132684f6e014918578670 = private unnamed_addr constant [18 x i8] c"thumbv7m-none-eabi", align 1
@alloc_26b0241c8fc21259fc82bc5f08a04026 = private unnamed_addr constant [23 x i8] c"thumbv8m.base-none-eabi", align 1
@alloc_eb24842ce519f5e77fce191c09f10d66 = private unnamed_addr constant [23 x i8] c"thumbv8m.main-none-eabi", align 1
@alloc_571391e7b49d723924917da68f3d8a4d = private unnamed_addr constant [25 x i8] c"thumbv8m.main-none-eabihf", align 1
@alloc_6929424002fbe1deb331ef746021f140 = private unnamed_addr constant <{ ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8], ptr, [8 x i8] }> <{ ptr @alloc_7ef794223a3c79a90832fd404d7462c3, [8 x i8] c"\15\00\00\00\00\00\00\00", ptr @alloc_60041a2996b9889a1b040c54c616ffa4, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_fc4b0cb0b48da575f740f3b061247e0f, [8 x i8] c"\14\00\00\00\00\00\00\00", ptr @alloc_3e1a5023273d14d3525522c2099080bd, [8 x i8] c"\1C\00\00\00\00\00\00\00", ptr @alloc_d854eb1f96568158508a1fe06acc479b, [8 x i8] c"\1D\00\00\00\00\00\00\00", ptr @alloc_ab31f0ee4c349cacd5df9f59f007a5ed, [8 x i8] c"\1E\00\00\00\00\00\00\00", ptr @alloc_82d439c4b4b9541b054ad05c5b792d5a, [8 x i8] c" \00\00\00\00\00\00\00", ptr @alloc_c1e04960e461045b24da3143b9bbe57e, [8 x i8] c"\13\00\00\00\00\00\00\00", ptr @alloc_828ae20c2ae220c38b3245d3e4d32e82, [8 x i8] c"\10\00\00\00\00\00\00\00", ptr @alloc_7b4c0892dcb20ad38838433c6bd13c7a, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_073a41cfae022637d06fd85a5e605fab, [8 x i8] c"\19\00\00\00\00\00\00\00", ptr @alloc_077b257855cc8302e2e37029c956e3c3, [8 x i8] c"\1A\00\00\00\00\00\00\00", ptr @alloc_9bc964707defe2160c729e1bb0b7c1e9, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_22b33466e0dd64c3b2e2ee15ff8d195c, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_3f54fddd3386141ea42568fa33ba0fc0, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_53bb287215d6b7ca5d0e83638974f2dc, [8 x i8] c"\19\00\00\00\00\00\00\00", ptr @alloc_670ec80a76e3dda8c0e4bb964f752aae, [8 x i8] c"\0F\00\00\00\00\00\00\00", ptr @alloc_4275caebb106965f8ef5bb2811245ced, [8 x i8] c"\18\00\00\00\00\00\00\00", ptr @alloc_a306aa267a229f9192f581f5c4b3f8a5, [8 x i8] c"\19\00\00\00\00\00\00\00", ptr @alloc_5b2e73936d9ed25120ae3cd1b1d86750, [8 x i8] c"\1B\00\00\00\00\00\00\00", ptr @alloc_addf2b8c5bb0c785f64e828bb16710c9, [8 x i8] c"\13\00\00\00\00\00\00\00", ptr @alloc_f92ab8510275e5e42dd14554da4d5f0d, [8 x i8] c"\1D\00\00\00\00\00\00\00", ptr @alloc_0c0bdd2040e1d3b535a15670bed8e2ee, [8 x i8] c"\1F\00\00\00\00\00\00\00", ptr @alloc_9a3299e3caae06e665af83a2a067eb68, [8 x i8] c"\0F\00\00\00\00\00\00\00", ptr @alloc_5e3a4f7a117d2e7bbe7456c524bb0006, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_edfd6be1acee3e3854f172faab285c38, [8 x i8] c"\19\00\00\00\00\00\00\00", ptr @alloc_1f2300ead6011ba2b5b331f2b76eb516, [8 x i8] c"\1C\00\00\00\00\00\00\00", ptr @alloc_baac72b63c55b5842a31e2c00a7d0883, [8 x i8] c"\1A\00\00\00\00\00\00\00", ptr @alloc_0d8d35c4ec871ff7e6c083e1cc59762d, [8 x i8] c"\16\00\00\00\00\00\00\00", ptr @alloc_ad7f1342255ebef4377308b28b295ab3, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_4c78f2281b95c444848389f653bfe1aa, [8 x i8] c"\13\00\00\00\00\00\00\00", ptr @alloc_2e417c01fc50dde593c75f17df443aac, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_ba03a0b15f58439199c9c1fb605fb91b, [8 x i8] c"\1B\00\00\00\00\00\00\00", ptr @alloc_1689bd3b01b5acc98a70228e04d83498, [8 x i8] c"\1C\00\00\00\00\00\00\00", ptr @alloc_ecbfecbf042f5d02ed1d945b2da736d2, [8 x i8] c"\19\00\00\00\00\00\00\00", ptr @alloc_b7513f3459de499a3785fd3ab62ad6ab, [8 x i8] c"\1C\00\00\00\00\00\00\00", ptr @alloc_4aac2c751578ff8b2a7da0743e6f1498, [8 x i8] c"\15\00\00\00\00\00\00\00", ptr @alloc_fd1b38744aa12ca58723e85c5996f9f0, [8 x i8] c"\1B\00\00\00\00\00\00\00", ptr @alloc_603fc8e29c06fe139773a6b784e3cc49, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_0f444773a3c04e447512a602928cccf9, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_3619ae5c84ee9a44040d6ebb9b6b060d, [8 x i8] c"\13\00\00\00\00\00\00\00", ptr @alloc_d90900e16a07082031ab037fc65b138f, [8 x i8] c"\15\00\00\00\00\00\00\00", ptr @alloc_a6ec35165b5132684f6e014918578670, [8 x i8] c"\12\00\00\00\00\00\00\00", ptr @alloc_26b0241c8fc21259fc82bc5f08a04026, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_eb24842ce519f5e77fce191c09f10d66, [8 x i8] c"\17\00\00\00\00\00\00\00", ptr @alloc_571391e7b49d723924917da68f3d8a4d, [8 x i8] c"\19\00\00\00\00\00\00\00" }>, align 8
@alloc_aa31e40b84e5bd72eb2b84fd23b8ce8b = private unnamed_addr constant [18 x i8] c"bpfeb-unknown-none", align 1
@alloc_edb56cace5b9083f410aa7986257c472 = private unnamed_addr constant [18 x i8] c"bpfel-unknown-none", align 1
@alloc_63b8bcb9c0020832e6fa0013d77e86c3 = private unnamed_addr constant [15 x i8] c"mipsel-sony-psx", align 1
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8
@alloc_1c5ece773fe9d8a26ac674de79674b77 = private unnamed_addr constant [10 x i8] c"NotPresent", align 1
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtB6_5Debug3fmtCslwKqnJYeWCA_18build_script_build }>, align 8
@alloc_19adf04fb909e90136daf37b5ff22508 = private unnamed_addr constant [10 x i8] c"NotUnicode", align 1
@alloc_559c4f386b668c946885822cef1a587d = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00h\04\00\00$\00\00\00" }>, align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECslwKqnJYeWCA_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", ptr %_1.val, i64 %_3.sroa.0.09.i.i
  %2 = add nuw i64 %_3.sroa.0.09.i.i, 1
  %_6.val.i.i = load ptr, ptr %_6.i.i, align 8, !alias.scope !4
  %3 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %3, align 8, !alias.scope !4, !nonnull !3, !align !7, !noundef !3
  %4 = load ptr, ptr %_6.val7.i.i, align 8, !invariant.load !3, !noalias !4
  %.not.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i, label %bb3.i.i.i, label %is_not_null.i.i.i

is_not_null.i.i.i:                                ; preds = %bb5.i.i
  %5 = icmp ne ptr %_6.val.i.i, null
  tail call void @llvm.assume(i1 %5)
  invoke void %4(ptr noundef nonnull %_6.val.i.i)
          to label %bb3.i.i.i unwind label %cleanup.i.i.i, !noalias !4

bb3.i.i.i:                                        ; preds = %is_not_null.i.i.i, %bb5.i.i
  %6 = icmp ne ptr %_6.val.i.i, null
  tail call void @llvm.assume(i1 %6)
  %7 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 8
  %8 = load i64, ptr %7, align 8, !range !8, !invariant.load !3, !noalias !4
  %9 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 16
  %10 = load i64, ptr %9, align 8, !range !9, !invariant.load !3, !noalias !4
  %11 = add i64 %10, -1
  %12 = icmp sgt i64 %11, -1
  tail call void @llvm.assume(i1 %12)
  %13 = icmp eq i64 %8, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #23, !noalias !4
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i

cleanup.i.i.i:                                    ; preds = %is_not_null.i.i.i
  %14 = landingpad { ptr, i32 }
          cleanup
  %15 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !8, !invariant.load !3, !noalias !4
  %17 = getelementptr inbounds nuw i8, ptr %_6.val7.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !9, !invariant.load !3, !noalias !4
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  tail call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %bb4.i.i.preheader, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i

bb4.i.i.preheader:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i, %cleanup.i.i.i
  br label %bb4.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i: ; preds = %cleanup.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #23, !noalias !4
  br label %bb4.i.i.preheader

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4.i.i:                                          ; preds = %bb4.i.i.preheader, %bb3.i.i
  %_3.sroa.0.1.i.i = phi i64 [ %22, %bb3.i.i ], [ %2, %bb4.i.i.preheader ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %_1.val1
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw %"alloc::boxed::Box<dyn core::ops::function::FnMut() -> core::result::Result<(), std::io::error::Error> + core::marker::Send + core::marker::Sync>", ptr %_1.val, i64 %_3.sroa.0.1.i.i
  %22 = add i64 %_3.sroa.0.1.i.i, 1
  %_4.val.i.i = load ptr, ptr %_4.i.i, align 8, !alias.scope !4
  %23 = getelementptr i8, ptr %_4.i.i, i64 8
  %_4.val6.i.i = load ptr, ptr %23, align 8, !alias.scope !4, !nonnull !3, !align !7, !noundef !3
; invoke core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #24
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !4

terminate.i.i:                                    ; preds = %bb3.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #25, !noalias !4
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %25 = icmp eq i64 %_1.val2, 0
  br i1 %25, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECslwKqnJYeWCA_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECslwKqnJYeWCA_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECslwKqnJYeWCA_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECslwKqnJYeWCA_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = load ptr, ptr %_1.8.val, align 8, !invariant.load !3
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
  %5 = load i64, ptr %4, align 8, !range !8, !invariant.load !3
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !9, !invariant.load !3
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCslwKqnJYeWCA_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #23
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCslwKqnJYeWCA_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCslwKqnJYeWCA_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
  ret void

cleanup:                                          ; preds = %is_not_null
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %13 = load i64, ptr %12, align 8, !range !8, !invariant.load !3
  %14 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %15 = load i64, ptr %14, align 8, !range !9, !invariant.load !3
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4: ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #23
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBN_7sources4once4OnceB1G_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8, !range !10, !noundef !3
  switch i64 %_1.val, label %bb2.i.i.i4.i.i.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4
    i64 -9223372036854775808, label %bb4
    i64 0, label %bb4
  ]

bb2.i.i.i4.i.i.i.i.i.i.i:                         ; preds = %start
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i, %start, %start, %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val3 = load i64, ptr %1, align 8, !range !10, !noundef !3
  switch i64 %.val3, label %bb2.i.i.i4.i.i.i.i.i.i.i.i5 [
    i64 -9223372036854775807, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECslwKqnJYeWCA_18build_script_build.exit6
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECslwKqnJYeWCA_18build_script_build.exit6
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECslwKqnJYeWCA_18build_script_build.exit6
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i5:                      ; preds = %bb4
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val4 = load ptr, ptr %2, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val4, i64 noundef %.val3, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECslwKqnJYeWCA_18build_script_build.exit6

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECslwKqnJYeWCA_18build_script_build.exit6: ; preds = %bb4, %bb4, %bb4, %bb2.i.i.i4.i.i.i.i.i.i.i.i5
  ret void
}

; core::ptr::drop_in_place::<std::env::VarError>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !11, !noundef !3
  switch i64 %0, label %bb2.i.i.i4.i.i.i [
    i64 -9223372036854775808, label %bb1
    i64 0, label %bb1
  ]

bb1:                                              ; preds = %start, %start, %bb2.i.i.i4.i.i.i
  ret void

bb2.i.i.i4.i.i.i:                                 ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb1
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %bb4, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i, %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2 = load i64, ptr %2, align 8
  %3 = icmp eq i64 %.val2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECslwKqnJYeWCA_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECslwKqnJYeWCA_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECslwKqnJYeWCA_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !12, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val24.i = load i64, ptr %1, align 8, !alias.scope !12
  store i8 0, ptr %.val.i, align 1, !noalias !12
  %2 = icmp eq i64 %.val24.i, 0
  br i1 %2, label %bb20.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %.val24.i, i64 noundef 1) #23
  br label %bb20.i

bb20.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i, %start
; invoke <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
  invoke void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb20.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %_1.val.i.i = load i64, ptr %_1, align 8, !alias.scope !15
  %4 = icmp eq i64 %_1.val.i.i, 0
  br i1 %4, label %bb10.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i = load ptr, ptr %5, align 8, !alias.scope !15, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %_1.val.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb10.i

bb4.i.i:                                          ; preds = %bb20.i
  %_1.val2.i.i = load i64, ptr %_1, align 8, !alias.scope !15
  %6 = icmp eq i64 %_1.val2.i.i, 0
  br i1 %6, label %bb19.i, label %bb2.i.i.i4.i4.i.i

bb2.i.i.i4.i4.i.i:                                ; preds = %bb4.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3.i.i = load ptr, ptr %7, align 8, !alias.scope !15, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i5.i.i = shl nuw i64 %_1.val2.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3.i.i, i64 noundef %alloc_size.i.i.i.i5.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb19.i

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #24
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
          to label %bb18.i unwind label %cleanup2.i

bb9.i:                                            ; preds = %cleanup2.i, %bb10.i
  %.pn10.i = phi { ptr, i32 } [ %14, %cleanup2.i ], [ %3, %bb10.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val27.i = load ptr, ptr %10, align 8, !alias.scope !12, !align !18, !noundef !3
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val28.i = load i64, ptr %11, align 8, !alias.scope !12
  %12 = icmp eq ptr %.val27.i, null
  br i1 %12, label %bb8.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb9.i
  store i8 0, ptr %.val27.i, align 1
  %13 = icmp eq i64 %.val28.i, 0
  br i1 %13, label %bb8.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val27.i, i64 noundef %.val28.i, i64 noundef 1) #23
  br label %bb8.i

cleanup2.i:                                       ; preds = %bb19.i
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %bb9.i

bb18.i:                                           ; preds = %bb19.i
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val31.i = load ptr, ptr %15, align 8, !alias.scope !12, !align !18, !noundef !3
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val32.i = load i64, ptr %16, align 8, !alias.scope !12
  %17 = icmp eq ptr %.val31.i, null
  br i1 %17, label %bb17.i, label %bb2.i50.i

bb2.i50.i:                                        ; preds = %bb18.i
  store i8 0, ptr %.val31.i, align 1
  %18 = icmp eq i64 %.val32.i, 0
  br i1 %18, label %bb17.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i: ; preds = %bb2.i50.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val31.i, i64 noundef %.val32.i, i64 noundef 1) #23
  br label %bb17.i

bb8.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i, %bb2.i.i, %bb9.i
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val25.i = load ptr, ptr %19, align 8, !alias.scope !12, !align !18, !noundef !3
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val26.i = load i64, ptr %20, align 8, !alias.scope !12
  %21 = icmp eq ptr %.val25.i, null
  br i1 %21, label %bb7.i, label %bb2.i54.i

bb2.i54.i:                                        ; preds = %bb8.i
  store i8 0, ptr %.val25.i, align 1
  %22 = icmp eq i64 %.val26.i, 0
  br i1 %22, label %bb7.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i: ; preds = %bb2.i54.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val25.i, i64 noundef %.val26.i, i64 noundef 1) #23
  br label %bb7.i

bb17.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i, %bb2.i50.i, %bb18.i
  %23 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val29.i = load ptr, ptr %23, align 8, !alias.scope !12, !align !18, !noundef !3
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val30.i = load i64, ptr %24, align 8, !alias.scope !12
  %25 = icmp eq ptr %.val29.i, null
  br i1 %25, label %bb16.i, label %bb2.i58.i

bb2.i58.i:                                        ; preds = %bb17.i
  store i8 0, ptr %.val29.i, align 1
  %26 = icmp eq i64 %.val30.i, 0
  br i1 %26, label %bb16.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i: ; preds = %bb2.i58.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val29.i, i64 noundef %.val30.i, i64 noundef 1) #23
  br label %bb16.i

bb7.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i, %bb2.i54.i, %bb8.i
  %27 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #24
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
          to label %bb15.i unwind label %cleanup5.i

bb6.i:                                            ; preds = %cleanup5.i, %bb7.i
  %.pn16.i = phi { ptr, i32 } [ %34, %cleanup5.i ], [ %.pn10.i, %bb7.i ]
  %29 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val33.i = load ptr, ptr %29, align 8, !alias.scope !12, !align !19, !noundef !3
  %30 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val34.i = load i64, ptr %30, align 8, !alias.scope !12
  %31 = icmp eq ptr %.val33.i, null
  %32 = icmp eq i64 %.val34.i, 0
  %or.cond.i.i = select i1 %31, i1 true, i1 %32
  br i1 %or.cond.i.i, label %bb5.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb6.i
  %33 = shl nuw nsw i64 %.val34.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val33.i, i64 noundef %33, i64 noundef 4) #23
  br label %bb5.i

cleanup5.i:                                       ; preds = %bb16.i
  %34 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i

bb15.i:                                           ; preds = %bb16.i
  %35 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val35.i = load ptr, ptr %35, align 8, !alias.scope !12, !align !19, !noundef !3
  %36 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val36.i = load i64, ptr %36, align 8, !alias.scope !12
  %37 = icmp eq ptr %.val35.i, null
  %38 = icmp eq i64 %.val36.i, 0
  %or.cond.i63.i = select i1 %37, i1 true, i1 %38
  br i1 %or.cond.i63.i, label %bb14.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i: ; preds = %bb15.i
  %39 = shl nuw nsw i64 %.val36.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val35.i, i64 noundef %39, i64 noundef 4) #23
  br label %bb14.i

bb5.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb6.i
  %40 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val41.i = load i32, ptr %40, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i.i = icmp eq i32 %.val41.i, 3
  br i1 %cond.i.i, label %bb2.i.i.i, label %bb4.i

bb2.i.i.i:                                        ; preds = %bb5.i
  %41 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val42.i = load i32, ptr %41, align 4, !alias.scope !12
  %_5.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val42.i) #23
  br label %bb4.i

bb14.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i, %bb15.i
  %42 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val47.i = load i32, ptr %42, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i68.i = icmp eq i32 %.val47.i, 3
  br i1 %cond.i68.i, label %bb2.i.i70.i, label %bb13.i

bb2.i.i70.i:                                      ; preds = %bb14.i
  %43 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val48.i = load i32, ptr %43, align 4, !alias.scope !12
  %_5.i.i.i.i.i71.i = tail call noundef i32 @close(i32 noundef %.val48.i) #23
  br label %bb13.i

bb4.i:                                            ; preds = %bb2.i.i.i, %bb5.i
  %44 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val39.i = load i32, ptr %44, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i73.i = icmp eq i32 %.val39.i, 3
  br i1 %cond.i73.i, label %bb2.i.i75.i, label %bb3.i

bb2.i.i75.i:                                      ; preds = %bb4.i
  %45 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val40.i = load i32, ptr %45, align 4, !alias.scope !12
  %_5.i.i.i.i.i76.i = tail call noundef i32 @close(i32 noundef %.val40.i) #23
  br label %bb3.i

bb13.i:                                           ; preds = %bb2.i.i70.i, %bb14.i
  %46 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val45.i = load i32, ptr %46, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i78.i = icmp eq i32 %.val45.i, 3
  br i1 %cond.i78.i, label %bb2.i.i80.i, label %bb12.i

bb2.i.i80.i:                                      ; preds = %bb13.i
  %47 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val46.i = load i32, ptr %47, align 4, !alias.scope !12
  %_5.i.i.i.i.i81.i = tail call noundef i32 @close(i32 noundef %.val46.i) #23
  br label %bb12.i

bb3.i:                                            ; preds = %bb2.i.i75.i, %bb4.i
  %48 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val37.i = load i32, ptr %48, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i83.i = icmp eq i32 %.val37.i, 3
  br i1 %cond.i83.i, label %bb2.i.i85.i, label %bb1.i

bb2.i.i85.i:                                      ; preds = %bb3.i
  %49 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val38.i = load i32, ptr %49, align 4, !alias.scope !12
  %_5.i.i.i.i.i86.i = tail call noundef i32 @close(i32 noundef %.val38.i) #23
  br label %bb1.i

bb12.i:                                           ; preds = %bb2.i.i80.i, %bb13.i
  %50 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val43.i = load i32, ptr %50, align 8, !range !20, !alias.scope !12, !noundef !3
  %cond.i88.i = icmp eq i32 %.val43.i, 3
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECslwKqnJYeWCA_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !12
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECslwKqnJYeWCA_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #25
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECslwKqnJYeWCA_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !3
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

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
  %key.val1.i.i.i.i.i = load ptr, ptr %5, align 8, !noalias !28, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %key.val1.i.i.i.i.i, i64 noundef %key.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !28
  br label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i
  %self1.val.i.i.i.i.i.i.i = load i64, ptr %_17.i.i.i.i.i, align 8, !range !11, !noalias !28, !noundef !3
  switch i64 %self1.val.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb8.i.i.i.i.i
  %6 = getelementptr i8, ptr %_17.i.i.i.i.i, i64 8
  %self1.val2.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !28, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self1.val2.i.i.i.i.i.i.i, i64 noundef %self1.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !28
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i, %bb8.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !27
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECslwKqnJYeWCA_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #23, !srcloc !33
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !34)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !8, !alias.scope !34, !noundef !3
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 %v16.i)
  %0 = icmp eq i64 %elem_layout.1, 1
  %v1.sroa.0.0.i = select i1 %0, i64 8, i64 4
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 %v1.sroa.0.0.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  %1 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !34
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1), !noalias !34
  %_35.i = load i64, ptr %self3.i, align 8, !range !37, !noalias !34, !noundef !3
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %3, align 8, !range !11, !noalias !34, !noundef !3
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !34
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #26
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %3, align 8, !noalias !34, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !34
  %5 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !34
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CslwKqnJYeWCA_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #4 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECslwKqnJYeWCA_18build_script_build(ptr noundef nonnull %_4) #27
  ret i32 0
}

; <build_script_build::version::Version>::parse::{closure#2}
; Function Attrs: inlinehint uwtable
define internal fastcc { i32, i32 } @_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parses0_0B6_(ptr readonly captures(address_is_null) %_1.0.val) unnamed_addr #4 personality ptr @rust_eh_personality {
bb2.i.i.i.i.lr.ph.i:
  %_5.i52 = alloca [24 x i8], align 8
  %_5.i = alloca [24 x i8], align 8
  %_5.i.i.i.i.i = alloca [24 x i8], align 8
  %digits = alloca [80 x i8], align 16
  %_5 = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_5)
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_29.0 = load ptr, ptr %_1.0.val, align 8, !nonnull !3, !align !18, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1.0.val, i64 8
  %_29.1 = load i64, ptr %1, align 8, !noundef !3
  store i64 0, ptr %_5, align 8
  %_30.sroa.4.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store i64 %_29.1, ptr %_30.sroa.4.0._5.sroa_idx, align 8
  %_30.sroa.5.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 16
  store ptr %_29.0, ptr %_30.sroa.5.0._5.sroa_idx, align 8
  %_30.sroa.5.sroa.4.0._30.sroa.5.0._5.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 24
  store i64 %_29.1, ptr %_30.sroa.5.sroa.4.0._30.sroa.5.0._5.sroa_idx.sroa_idx, align 8
  %_30.sroa.5.sroa.5.0._30.sroa.5.0._5.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 32
  store i64 0, ptr %_30.sroa.5.sroa.5.0._30.sroa.5.0._5.sroa_idx.sroa_idx, align 8
  %_30.sroa.5.sroa.6.0._30.sroa.5.0._5.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 40
  store i64 %_29.1, ptr %_30.sroa.5.sroa.6.0._30.sroa.5.0._5.sroa_idx.sroa_idx, align 8
  %_30.sroa.5.sroa.7.0._30.sroa.5.0._5.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 48
  store <2 x i32> splat (i32 10), ptr %_30.sroa.5.sroa.7.0._30.sroa.5.0._5.sroa_idx.sroa_idx, align 8
  %_30.sroa.5.sroa.9.0._30.sroa.5.0._5.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 56
  store i8 1, ptr %_30.sroa.5.sroa.9.0._30.sroa.5.0._5.sroa_idx.sroa_idx, align 8
  %_30.sroa.6.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 64
  store i8 0, ptr %_30.sroa.6.0._5.sroa_idx, align 8
  %_30.sroa.7.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 65
  store i8 0, ptr %_30.sroa.7.0._5.sroa_idx, align 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !38)
  %2 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i, i64 16
  br label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses0_00E0B1n_.exit.i, %bb2.i.i.i.i.lr.ph.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !44)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !47)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !50)
  %_4.val.i.i.i.i.i = load ptr, ptr %_30.sroa.5.0._5.sroa_idx, align 8, !alias.scope !53, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !53
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_30.sroa.5.0._5.sroa_idx) #28
  %_7.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i, align 8, !range !37, !noalias !53, !noundef !3
  %3 = trunc nuw i64 %_7.i.i.i.i.i to i1
  br i1 %3, label %bb7.i.i.i.i.i, label %bb6.i.i.i.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i
  %b.i.i.i.i.i = load i64, ptr %2, align 8, !noalias !53, !noundef !3
  %i.i.i.i.i.i = load i64, ptr %_5, align 8, !alias.scope !53, !noundef !3
  %new_len.i.i.i.i.i = sub nuw i64 %b.i.i.i.i.i, %i.i.i.i.i.i
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i.i.i, i64 %i.i.i.i.i.i
  store i64 %b.i.i.i.i.i, ptr %_5, align 8, !alias.scope !53
  br label %bb5.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i
  %4 = load i8, ptr %_30.sroa.7.0._5.sroa_idx, align 1, !range !54, !alias.scope !55, !noundef !3
  %_2.i.i.i.i.i.i = trunc nuw i8 %4 to i1
  br i1 %_2.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i, label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i
  store i8 1, ptr %_30.sroa.7.0._5.sroa_idx, align 1, !alias.scope !55
  %5 = load i8, ptr %_30.sroa.6.0._5.sroa_idx, align 8, !range !54, !alias.scope !55, !noundef !3
  %_3.i.i.i.i.i.i = trunc nuw i8 %5 to i1
  %i.pre.i.i.i.i.i.i = load i64, ptr %_5, align 8, !alias.scope !55
  %i1.pre.i.i.i.i.i.i = load i64, ptr %_30.sroa.4.0._5.sroa_idx, align 8, !alias.scope !55
  %_4.not.i.i.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i
  %or.cond.not.i.i.i.i.i.i = select i1 %_3.i.i.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i.i.i
  br i1 %or.cond.not.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i
  %_10.val.i.i.i.i.i.i = load ptr, ptr %_30.sroa.5.0._5.sroa_idx, align 8, !alias.scope !55, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i
  %data.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i.i, i64 %i.pre.i.i.i.i.i.i
  br label %bb5.i.i.i

_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i: ; preds = %bb1.i.i.i.i.i.i, %bb6.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !53
  br label %bb14

bb5.i.i.i:                                        ; preds = %bb4.i.i.i.i.i.i, %bb7.i.i.i.i.i
  %_0.sroa.4.0.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i, %bb7.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i.i = phi ptr [ %data.i.i.i.i.i, %bb7.i.i.i.i.i ], [ %data.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !53
  %6 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i.i.i.i.i, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %_0.sroa.4.0.i.i.i.i.i, 1
  %_5.not.i.i.i.i.i.i.i = icmp eq i64 %_0.sroa.4.0.i.i.i.i.i, 0
  br i1 %_5.not.i.i.i.i.i.i.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i: ; preds = %bb5.i.i.i
  %8 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i, i64 %_0.sroa.4.0.i.i.i.i.i
  %_16.i.i.i.i.i.i.i = getelementptr i8, ptr %8, i64 -1
  %rhsc.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !58, !noalias !67
  %rhsc.i.i.fr.i.i.i.i.i = freeze i8 %rhsc.i.i.i.i.i.i.i
  %9 = icmp eq i8 %rhsc.i.i.fr.i.i.i.i.i, 10
  %i.i.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i, -1
  br i1 %9, label %bb1.i.i.i.i.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i
  %10 = insertvalue { ptr, i64 } %7, i64 %i.i.i.i.i.i.i, 1
  %_5.not.i.i12.i.i.i.i.i = icmp eq i64 %i.i.i.i.i.i.i, 0
  br i1 %_5.not.i.i12.i.i.i.i.i, label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i
  %11 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i, i64 %i.i.i.i.i.i.i
  %_16.i.i14.i.i.i.i.i = getelementptr i8, ptr %11, i64 -1
  %rhsc.i.i16.i.i.i.i.i = load i8, ptr %_16.i.i14.i.i.i.i.i, align 1, !alias.scope !70, !noalias !75
  %rhsc.i.i16.fr.i.i.i.i.i = freeze i8 %rhsc.i.i16.i.i.i.i.i
  %12 = icmp eq i8 %rhsc.i.i16.fr.i.i.i.i.i, 13
  %i.i17.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i, -2
  %spec.select.i19.i.i.i.i.i = select i1 %12, ptr %_0.sroa.0.0.i.i.i.i.i, ptr null
  br label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i

_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i: ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i, %bb1.i.i.i.i.i
  %i4.i20.i.i.i.i.i = phi i64 [ %i.i17.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i ], [ -1, %bb1.i.i.i.i.i ]
  %13 = phi ptr [ %spec.select.i19.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i ], [ null, %bb1.i.i.i.i.i ]
  %14 = insertvalue { ptr, i64 } poison, ptr %13, 0
  %15 = insertvalue { ptr, i64 } %14, i64 %i4.i20.i.i.i.i.i, 1
  %.not11.i.i.i.i.i = icmp eq ptr %13, null
  %..i.i.i.i.i = select i1 %.not11.i.i.i.i.i, { ptr, i64 } %10, { ptr, i64 } %15
  br label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i

_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i, %bb5.i.i.i
  %.merged.i.i.i.i.i = phi { ptr, i64 } [ %..i.i.i.i.i, %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i ], [ %7, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i ], [ %7, %bb5.i.i.i ]
  %_7.0.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i, 0
  %_7.1.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i, 1
  %.not.i = icmp eq ptr %_7.0.i.i.i, null
  br i1 %.not.i, label %bb14, label %bb3.i

bb3.i:                                            ; preds = %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i
  %_4.not.i.i.i.i = icmp samesign ult i64 %_7.1.i.i.i, 14
  br i1 %_4.not.i.i.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses0_00E0B1n_.exit.i, label %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses0_00B8_.exit.i.i

_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses0_00B8_.exit.i.i: ; preds = %bb3.i
  %16 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(14) @alloc_1d8e296db13ccc3871d7443d7beb0b2f, ptr noundef nonnull readonly align 1 dereferenceable(14) %_7.0.i.i.i, i64 range(i64 0, -9223372036854775808) 14), !alias.scope !78, !noalias !38
  %.fr.i.i = freeze i32 %16
  %17 = icmp eq i32 %.fr.i.i, 0
  br i1 %17, label %bb9, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses0_00E0B1n_.exit.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses0_00E0B1n_.exit.i: ; preds = %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses0_00B8_.exit.i.i, %bb3.i
  %18 = load i8, ptr %_30.sroa.7.0._5.sroa_idx, align 1, !range !54, !alias.scope !85, !noundef !3
  %_2.i.i.i.i.i = trunc nuw i8 %18 to i1
  br i1 %_2.i.i.i.i.i, label %bb14, label %bb2.i.i.i.i.i

bb9:                                              ; preds = %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses0_00B8_.exit.i.i
  %_8.not.i.not = icmp eq i64 %_7.1.i.i.i, 14
  br i1 %_8.not.i.not, label %bb18, label %bb9.i

bb9.i:                                            ; preds = %bb9
  %19 = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i, i64 14
  %self1.i = load i8, ptr %19, align 1, !alias.scope !90, !noundef !3
  %20 = icmp sgt i8 %self1.i, -65
  br i1 %20, label %bb18, label %bb12

bb14:                                             ; preds = %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses0_00E0B1n_.exit.i, %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_5)
  br label %bb6

bb12:                                             ; preds = %bb9.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_7.0.i.i.i, i64 noundef %_7.1.i.i.i, i64 noundef 14, i64 noundef %_7.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_954cd85162064372f588eb6b0cd0c8e9) #29
  unreachable

bb6:                                              ; preds = %bb5, %bb50, %bb14
  %_0.sroa.8.0 = phi i32 [ undef, %bb5 ], [ %_0.sroa.8.0.insert.insert.i, %bb50 ], [ undef, %bb14 ]
  %_0.sroa.0.0 = phi i32 [ 0, %bb5 ], [ 1, %bb50 ], [ 0, %bb14 ]
  %21 = insertvalue { i32, i32 } poison, i32 %_0.sroa.0.0, 0
  %22 = insertvalue { i32, i32 } %21, i32 %_0.sroa.8.0, 1
  ret { i32, i32 } %22

bb18:                                             ; preds = %bb9, %bb9.i
  %new_len.i = add i64 %_7.1.i.i.i, -14
  %data.i = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i, i64 14
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_5)
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %digits)
  %_50.sroa.4.0.digits.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 8
  %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 16
  store i64 %new_len.i, ptr %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 16
  %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 24
  store ptr %data.i, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8
  %_50.sroa.4.sroa.5.sroa.4.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 32
  store i64 %new_len.i, ptr %_50.sroa.4.sroa.5.sroa.4.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx, align 16
  %_50.sroa.4.sroa.5.sroa.5.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 40
  store i64 0, ptr %_50.sroa.4.sroa.5.sroa.5.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx, align 8
  %_50.sroa.4.sroa.5.sroa.6.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 48
  store i64 %new_len.i, ptr %_50.sroa.4.sroa.5.sroa.6.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx, align 16
  %_50.sroa.4.sroa.5.sroa.7.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 56
  store <2 x i32> splat (i32 46), ptr %_50.sroa.4.sroa.5.sroa.7.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx, align 8
  %_50.sroa.4.sroa.5.sroa.9.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 64
  store i8 1, ptr %_50.sroa.4.sroa.5.sroa.9.0._50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx, align 16
  %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 72
  store i8 1, ptr %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8
  %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %digits, i64 73
  store i8 0, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1
  store <2 x i64> <i64 2, i64 0>, ptr %digits, align 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !93)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !93
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef align 8 dereferenceable(48) %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx) #28
  %_7.i = load i64, ptr %_5.i, align 8, !range !37, !noalias !93, !noundef !3
  %23 = trunc nuw i64 %_7.i to i1
  br i1 %23, label %bb7.i, label %bb6.i37

bb7.i:                                            ; preds = %bb18
  %24 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %a.i = load i64, ptr %24, align 8, !noalias !93, !noundef !3
  %25 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  %b.i = load i64, ptr %25, align 8, !noalias !93, !noundef !3
  %i.i = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !93, !noundef !3
  %new_len.i40 = sub nuw i64 %a.i, %i.i
  %data.i41 = getelementptr inbounds nuw i8, ptr %data.i, i64 %i.i
  store i64 %b.i, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !93
  br label %bb24

bb6.i37:                                          ; preds = %bb18
  %26 = load i8, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !96, !noundef !3
  %_2.i.i = trunc nuw i8 %26 to i1
  br i1 %_2.i.i, label %bb8.i.thread, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i37
  store i8 1, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !alias.scope !96
  %27 = load i8, ptr %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !96, !noundef !3
  %_3.i.i = trunc nuw i8 %27 to i1
  %i.pre.i.i = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !96
  %i1.pre.i.i = load i64, ptr %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 16, !alias.scope !96
  %_4.not.i.i = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i
  br i1 %or.cond.not.i.i, label %bb4.i.i, label %bb8.i.thread

bb4.i.i:                                          ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !alias.scope !96, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb24

bb8.i.thread:                                     ; preds = %bb6.i37, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !93
  br label %bb5

bb24:                                             ; preds = %bb7.i, %bb4.i.i
  %_0.sroa.4.0.i = phi i64 [ %new_len.i40, %bb7.i ], [ %new_len.i.i, %bb4.i.i ]
  %_0.sroa.0.0.i38 = phi ptr [ %data.i41, %bb7.i ], [ %data.i.i, %bb4.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !93
  switch i64 %_0.sroa.4.0.i, label %bb9thread-pre-split.i [
    i64 0, label %bb5
    i64 1, label %bb7.i48
  ]

bb7.i48:                                          ; preds = %bb24
  %28 = load i8, ptr %_0.sroa.0.0.i38, align 1, !alias.scope !99, !noundef !3
  switch i8 %28, label %bb9.i50 [
    i8 43, label %bb5
    i8 45, label %bb5
  ]

bb9thread-pre-split.i:                            ; preds = %bb24
  %.pr.i = load i8, ptr %_0.sroa.0.0.i38, align 1, !alias.scope !99
  br label %bb9.i50

bb9.i50:                                          ; preds = %bb9thread-pre-split.i, %bb7.i48
  %29 = phi i8 [ %.pr.i, %bb9thread-pre-split.i ], [ %28, %bb7.i48 ]
  %cond.i = icmp eq i8 %29, 43
  %rest.1.i = sext i1 %cond.i to i64
  %src.sroa.15.0.i = add nsw i64 %_0.sroa.4.0.i, %rest.1.i
  %src.sroa.0.0.idx.i = zext i1 %cond.i to i64
  %src.sroa.0.0.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i38, i64 %src.sroa.0.0.idx.i
  %_10.i51 = icmp samesign ult i64 %src.sroa.15.0.i, 9
  br i1 %_10.i51, label %bb15.preheader.i, label %bb22.i

bb15.preheader.i:                                 ; preds = %bb9.i50
  %_13.not56.i = icmp eq i64 %src.sroa.15.0.i, 0
  br i1 %_13.not56.i, label %bb27, label %bb16.i

bb22.i:                                           ; preds = %bb9.i50, %bb33.i
  %result.sroa.0.0.i = phi i32 [ %_60.0.i, %bb33.i ], [ 0, %bb9.i50 ]
  %src.sroa.15.1.i = phi i64 [ %rest.12.i, %bb33.i ], [ %src.sroa.15.0.i, %bb9.i50 ]
  %src.sroa.0.1.i = phi ptr [ %rest.01.i, %bb33.i ], [ %src.sroa.0.0.i, %bb9.i50 ]
  %_28.not.i.not = icmp eq i64 %src.sroa.15.1.i, 0
  br i1 %_28.not.i.not, label %bb27, label %bb23.i

bb23.i:                                           ; preds = %bb22.i
  %30 = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i, i32 10)
  %_57.1.i = extractvalue { i32, i1 } %30, 1
  br i1 %_57.1.i, label %bb5, label %bb33.i, !prof !102

bb33.i:                                           ; preds = %bb23.i
  %_57.0.i = extractvalue { i32, i1 } %30, 0
  %rest.12.i = add nsw i64 %src.sroa.15.1.i, -1
  %rest.01.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i, i64 1
  %31 = load i8, ptr %src.sroa.0.1.i, align 1, !alias.scope !99, !noundef !3
  %32 = zext i8 %31 to i32
  %33 = add nsw i32 %32, -48
  %_14.i.i = icmp ugt i32 %33, 9
  %_60.0.i = add i32 %33, %_57.0.i
  %_60.1.i = icmp ult i32 %_60.0.i, %_57.0.i
  %or.cond = select i1 %_14.i.i, i1 true, i1 %_60.1.i
  br i1 %or.cond, label %bb5, label %bb22.i, !prof !103

bb16.i:                                           ; preds = %bb15.preheader.i, %bb20.i
  %src.sroa.0.259.i = phi ptr [ %rest.04.i, %bb20.i ], [ %src.sroa.0.0.i, %bb15.preheader.i ]
  %src.sroa.15.258.i = phi i64 [ %rest.15.i, %bb20.i ], [ %src.sroa.15.0.i, %bb15.preheader.i ]
  %result.sroa.0.257.i = phi i32 [ %36, %bb20.i ], [ 0, %bb15.preheader.i ]
  %_19.i = load i8, ptr %src.sroa.0.259.i, align 1, !alias.scope !99, !noundef !3
  %_18.i = zext i8 %_19.i to i32
  %34 = add nsw i32 %_18.i, -48
  %_14.i47.i = icmp ugt i32 %34, 9
  br i1 %_14.i47.i, label %bb5, label %bb20.i

bb20.i:                                           ; preds = %bb16.i
  %35 = mul i32 %result.sroa.0.257.i, 10
  %rest.15.i = add nsw i64 %src.sroa.15.258.i, -1
  %rest.04.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i, i64 1
  %36 = add i32 %34, %35
  %_13.not.i = icmp eq i64 %rest.15.i, 0
  br i1 %_13.not.i, label %bb27, label %bb16.i

bb27:                                             ; preds = %bb22.i, %bb20.i, %bb15.preheader.i
  %_0.sroa.8.0.insert.insert.i = phi i32 [ 0, %bb15.preheader.i ], [ %36, %bb20.i ], [ %result.sroa.0.0.i, %bb22.i ]
  %37 = load i64, ptr %digits, align 16, !noundef !3
  switch i64 %37, label %bb29 [
    i64 0, label %bb5
    i64 1, label %bb30
  ]

bb29:                                             ; preds = %bb27
  %38 = add i64 %37, -1
  store i64 %38, ptr %digits, align 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !104)
  %39 = load i8, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !104, !noundef !3
  %_2.i53 = trunc nuw i8 %39 to i1
  br i1 %_2.i53, label %bb5, label %bb2.i54

bb2.i54:                                          ; preds = %bb29
  %_4.val.i56 = load ptr, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !alias.scope !104, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i52), !noalias !104
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i52, ptr noalias noundef align 8 dereferenceable(48) %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx) #28
  %_7.i57 = load i64, ptr %_5.i52, align 8, !range !37, !noalias !104, !noundef !3
  %40 = trunc nuw i64 %_7.i57 to i1
  br i1 %40, label %bb7.i77, label %bb6.i58

bb7.i77:                                          ; preds = %bb2.i54
  %41 = getelementptr inbounds nuw i8, ptr %_5.i52, i64 8
  %a.i78 = load i64, ptr %41, align 8, !noalias !104, !noundef !3
  %42 = getelementptr inbounds nuw i8, ptr %_5.i52, i64 16
  %b.i79 = load i64, ptr %42, align 8, !noalias !104, !noundef !3
  %i.i80 = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !104, !noundef !3
  %new_len.i81 = sub nuw i64 %a.i78, %i.i80
  %data.i82 = getelementptr inbounds nuw i8, ptr %_4.val.i56, i64 %i.i80
  store i64 %b.i79, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !104
  br label %bb28

bb6.i58:                                          ; preds = %bb2.i54
  %43 = load i8, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !107, !noundef !3
  %_2.i.i59 = trunc nuw i8 %43 to i1
  br i1 %_2.i.i59, label %bb28.thread52, label %bb1.i.i60

bb1.i.i60:                                        ; preds = %bb6.i58
  store i8 1, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !alias.scope !107
  %44 = load i8, ptr %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !107, !noundef !3
  %_3.i.i61 = trunc nuw i8 %44 to i1
  %i.pre.i.i62 = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !107
  %i1.pre.i.i64 = load i64, ptr %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 16, !alias.scope !107
  %_4.not.i.i65 = icmp ne i64 %i1.pre.i.i64, %i.pre.i.i62
  %or.cond.not.i.i66 = select i1 %_3.i.i61, i1 true, i1 %_4.not.i.i65
  br i1 %or.cond.not.i.i66, label %bb4.i.i73, label %bb28.thread52

bb4.i.i73:                                        ; preds = %bb1.i.i60
  %_10.val.i.i74 = load ptr, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !alias.scope !107, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i75 = sub nuw i64 %i1.pre.i.i64, %i.pre.i.i62
  %data.i.i76 = getelementptr inbounds nuw i8, ptr %_10.val.i.i74, i64 %i.pre.i.i62
  br label %bb28

bb30:                                             ; preds = %bb27
  store i64 0, ptr %digits, align 16
  %45 = load i8, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !110, !noundef !3
  %_2.i84 = trunc nuw i8 %45 to i1
  br i1 %_2.i84, label %bb5, label %bb1.i85

bb1.i85:                                          ; preds = %bb30
  store i8 1, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !alias.scope !110
  %46 = load i8, ptr %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !110, !noundef !3
  %_3.i86 = trunc nuw i8 %46 to i1
  %i.pre.i87 = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !110
  %i1.pre.i89 = load i64, ptr %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 16, !alias.scope !110
  %_4.not.i90 = icmp ne i64 %i1.pre.i89, %i.pre.i87
  %or.cond.not.i91 = select i1 %_3.i86, i1 true, i1 %_4.not.i90
  br i1 %or.cond.not.i91, label %bb28.thread45, label %bb5

bb28.thread45:                                    ; preds = %bb1.i85
  %_10.val.i97 = load ptr, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !alias.scope !110, !nonnull !3, !align !18, !noundef !3
  %new_len.i98 = sub nuw i64 %i1.pre.i89, %i.pre.i87
  %data.i99 = getelementptr inbounds nuw i8, ptr %_10.val.i97, i64 %i.pre.i87
  br label %bb35

bb28.thread52:                                    ; preds = %bb6.i58, %bb1.i.i60
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i52), !noalias !104
  br label %bb5

bb28:                                             ; preds = %bb7.i77, %bb4.i.i73
  %_0.sroa.4.0.i68 = phi i64 [ %new_len.i81, %bb7.i77 ], [ %new_len.i.i75, %bb4.i.i73 ]
  %_0.sroa.0.0.i69 = phi ptr [ %data.i82, %bb7.i77 ], [ %data.i.i76, %bb4.i.i73 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i52), !noalias !104
  br label %bb35

bb35:                                             ; preds = %bb28, %bb28.thread45
  %_0.sroa.4.1.i71.pn50 = phi i64 [ %new_len.i98, %bb28.thread45 ], [ %_0.sroa.4.0.i68, %bb28 ]
  %_0.sroa.0.1.i72.pn49 = phi ptr [ %data.i99, %bb28.thread45 ], [ %_0.sroa.0.0.i69, %bb28 ]
  switch i64 %_0.sroa.4.1.i71.pn50, label %bb9thread-pre-split.i145 [
    i64 0, label %bb5
    i64 1, label %bb7.i101
  ]

bb7.i101:                                         ; preds = %bb35
  %47 = load i8, ptr %_0.sroa.0.1.i72.pn49, align 1, !alias.scope !113, !noundef !3
  switch i8 %47, label %bb9.i105 [
    i8 43, label %bb5
    i8 45, label %bb5
  ]

bb9thread-pre-split.i145:                         ; preds = %bb35
  %.pr.i146 = load i8, ptr %_0.sroa.0.1.i72.pn49, align 1, !alias.scope !113
  br label %bb9.i105

bb9.i105:                                         ; preds = %bb9thread-pre-split.i145, %bb7.i101
  %48 = phi i8 [ %.pr.i146, %bb9thread-pre-split.i145 ], [ %47, %bb7.i101 ]
  %cond.i106 = icmp eq i8 %48, 43
  %rest.1.i107 = sext i1 %cond.i106 to i64
  %src.sroa.15.0.i108 = add nsw i64 %_0.sroa.4.1.i71.pn50, %rest.1.i107
  %src.sroa.0.0.idx.i109 = zext i1 %cond.i106 to i64
  %src.sroa.0.0.i110 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i72.pn49, i64 %src.sroa.0.0.idx.i109
  %_10.i111 = icmp samesign ult i64 %src.sroa.15.0.i108, 9
  br i1 %_10.i111, label %bb15.preheader.i132, label %bb22.i112

bb15.preheader.i132:                              ; preds = %bb9.i105
  %_13.not56.i133 = icmp eq i64 %src.sroa.15.0.i108, 0
  br i1 %_13.not56.i133, label %bb38, label %bb16.i134

bb22.i112:                                        ; preds = %bb9.i105, %bb33.i122
  %result.sroa.0.0.i113 = phi i32 [ %_60.0.i125, %bb33.i122 ], [ 0, %bb9.i105 ]
  %src.sroa.15.1.i114 = phi i64 [ %rest.12.i119, %bb33.i122 ], [ %src.sroa.15.0.i108, %bb9.i105 ]
  %src.sroa.0.1.i115 = phi ptr [ %rest.01.i118, %bb33.i122 ], [ %src.sroa.0.0.i110, %bb9.i105 ]
  %_28.not.i116 = icmp eq i64 %src.sroa.15.1.i114, 0
  br i1 %_28.not.i116, label %bb38, label %bb23.i117

bb23.i117:                                        ; preds = %bb22.i112
  %49 = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i113, i32 10)
  %_57.1.i121 = extractvalue { i32, i1 } %49, 1
  br i1 %_57.1.i121, label %bb5, label %bb33.i122, !prof !102

bb33.i122:                                        ; preds = %bb23.i117
  %_57.0.i120 = extractvalue { i32, i1 } %49, 0
  %rest.12.i119 = add nsw i64 %src.sroa.15.1.i114, -1
  %rest.01.i118 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i115, i64 1
  %50 = load i8, ptr %src.sroa.0.1.i115, align 1, !alias.scope !113, !noundef !3
  %51 = zext i8 %50 to i32
  %52 = add nsw i32 %51, -48
  %_14.i.i123 = icmp ugt i32 %52, 9
  %_60.0.i125 = add i32 %52, %_57.0.i120
  %_60.1.i126 = icmp ult i32 %_60.0.i125, %_57.0.i120
  %or.cond14 = select i1 %_14.i.i123, i1 true, i1 %_60.1.i126
  br i1 %or.cond14, label %bb5, label %bb22.i112, !prof !103

bb16.i134:                                        ; preds = %bb15.preheader.i132, %bb20.i141
  %src.sroa.0.259.i135 = phi ptr [ %rest.04.i143, %bb20.i141 ], [ %src.sroa.0.0.i110, %bb15.preheader.i132 ]
  %src.sroa.15.258.i136 = phi i64 [ %rest.15.i142, %bb20.i141 ], [ %src.sroa.15.0.i108, %bb15.preheader.i132 ]
  %_19.i138 = load i8, ptr %src.sroa.0.259.i135, align 1, !alias.scope !113, !noundef !3
  %53 = add i8 %_19.i138, -48
  %_14.i47.i140 = icmp ult i8 %53, 10
  br i1 %_14.i47.i140, label %bb20.i141, label %bb5

bb20.i141:                                        ; preds = %bb16.i134
  %rest.15.i142 = add nsw i64 %src.sroa.15.258.i136, -1
  %rest.04.i143 = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i135, i64 1
  %_13.not.i144 = icmp eq i64 %rest.15.i142, 0
  br i1 %_13.not.i144, label %bb38, label %bb16.i134

bb38:                                             ; preds = %bb22.i112, %bb20.i141, %bb15.preheader.i132
  %54 = load i64, ptr %digits, align 16, !noundef !3
  switch i64 %54, label %bb40 [
    i64 0, label %bb7.i165
    i64 1, label %bb41
  ]

bb40:                                             ; preds = %bb38
  %55 = add i64 %54, -1
  store i64 %55, ptr %digits, align 16
; call <core::str::iter::SplitInternal<char>>::next
  %56 = call fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %_50.sroa.4.0.digits.sroa_idx) #28
  br label %bb39

bb41:                                             ; preds = %bb38
  store i64 0, ptr %digits, align 16
  %57 = load i8, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !116, !noundef !3
  %_2.i148 = trunc nuw i8 %57 to i1
  br i1 %_2.i148, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164, label %bb1.i149

bb1.i149:                                         ; preds = %bb41
  store i8 1, ptr %_50.sroa.4.sroa.7.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 1, !alias.scope !116
  %58 = load i8, ptr %_50.sroa.4.sroa.6.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !116, !noundef !3
  %_3.i150 = trunc nuw i8 %58 to i1
  %i.pre.i151 = load i64, ptr %_50.sroa.4.0.digits.sroa_idx, align 8, !alias.scope !116
  %i1.pre.i153 = load i64, ptr %_50.sroa.4.sroa.4.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 16, !alias.scope !116
  %_4.not.i154 = icmp ne i64 %i1.pre.i153, %i.pre.i151
  %or.cond.not.i155 = select i1 %_3.i150, i1 true, i1 %_4.not.i154
  br i1 %or.cond.not.i155, label %bb4.i159, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164

bb4.i159:                                         ; preds = %bb1.i149
  %_10.val.i161 = load ptr, ptr %_50.sroa.4.sroa.5.0._50.sroa.4.0.digits.sroa_idx.sroa_idx, align 8, !alias.scope !116, !nonnull !3, !align !18, !noundef !3
  %new_len.i162 = sub nuw i64 %i1.pre.i153, %i.pre.i151
  %data.i163 = getelementptr inbounds nuw i8, ptr %_10.val.i161, i64 %i.pre.i151
  br label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164

_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164: ; preds = %bb41, %bb1.i149, %bb4.i159
  %_0.sroa.3.0.i157 = phi i64 [ %new_len.i162, %bb4.i159 ], [ undef, %bb41 ], [ undef, %bb1.i149 ]
  %_0.sroa.0.0.i158 = phi ptr [ %data.i163, %bb4.i159 ], [ null, %bb41 ], [ null, %bb1.i149 ]
  %59 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i158, 0
  %60 = insertvalue { ptr, i64 } %59, i64 %_0.sroa.3.0.i157, 1
  br label %bb39

bb39:                                             ; preds = %bb40, %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164
  %.pn30 = phi { ptr, i64 } [ %56, %bb40 ], [ %60, %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit164 ]
  %_26.sroa.0.0 = extractvalue { ptr, i64 } %.pn30, 0
  %.not32 = icmp eq ptr %_26.sroa.0.0, null
  %_26.sroa.6.0 = extractvalue { ptr, i64 } %.pn30, 1
  br i1 %.not32, label %bb7.i165, label %bb45

bb45:                                             ; preds = %bb39
  switch i64 %_26.sroa.6.0, label %bb9thread-pre-split.i209 [
    i64 0, label %bb5
    i64 1, label %bb7.i165
  ]

bb7.i165:                                         ; preds = %bb39, %bb38, %bb45
  %_25.sroa.0.09 = phi ptr [ %_26.sroa.0.0, %bb45 ], [ @alloc_dda1ee2b88b89b9cdac753eef7988035, %bb38 ], [ @alloc_dda1ee2b88b89b9cdac753eef7988035, %bb39 ]
  %61 = load i8, ptr %_25.sroa.0.09, align 1, !alias.scope !119, !noundef !3
  switch i8 %61, label %bb9.i169 [
    i8 43, label %bb5
    i8 45, label %bb5
  ]

bb9thread-pre-split.i209:                         ; preds = %bb45
  %.pr.i210 = load i8, ptr %_26.sroa.0.0, align 1, !alias.scope !119
  br label %bb9.i169

bb9.i169:                                         ; preds = %bb9thread-pre-split.i209, %bb7.i165
  %_25.sroa.3.011 = phi i64 [ %_26.sroa.6.0, %bb9thread-pre-split.i209 ], [ 1, %bb7.i165 ]
  %_25.sroa.0.010 = phi ptr [ %_26.sroa.0.0, %bb9thread-pre-split.i209 ], [ %_25.sroa.0.09, %bb7.i165 ]
  %62 = phi i8 [ %.pr.i210, %bb9thread-pre-split.i209 ], [ %61, %bb7.i165 ]
  %cond.i170 = icmp eq i8 %62, 43
  %rest.1.i171 = sext i1 %cond.i170 to i64
  %src.sroa.15.0.i172 = add nsw i64 %_25.sroa.3.011, %rest.1.i171
  %src.sroa.0.0.idx.i173 = zext i1 %cond.i170 to i64
  %src.sroa.0.0.i174 = getelementptr inbounds nuw i8, ptr %_25.sroa.0.010, i64 %src.sroa.0.0.idx.i173
  %_10.i175 = icmp samesign ult i64 %src.sroa.15.0.i172, 9
  br i1 %_10.i175, label %bb15.preheader.i196, label %bb22.i176

bb15.preheader.i196:                              ; preds = %bb9.i169
  %_13.not56.i197 = icmp eq i64 %src.sroa.15.0.i172, 0
  br i1 %_13.not56.i197, label %bb50, label %bb16.i198

bb22.i176:                                        ; preds = %bb9.i169, %bb33.i186
  %result.sroa.0.0.i177 = phi i32 [ %_60.0.i189, %bb33.i186 ], [ 0, %bb9.i169 ]
  %src.sroa.15.1.i178 = phi i64 [ %rest.12.i183, %bb33.i186 ], [ %src.sroa.15.0.i172, %bb9.i169 ]
  %src.sroa.0.1.i179 = phi ptr [ %rest.01.i182, %bb33.i186 ], [ %src.sroa.0.0.i174, %bb9.i169 ]
  %_28.not.i180 = icmp eq i64 %src.sroa.15.1.i178, 0
  br i1 %_28.not.i180, label %bb50, label %bb23.i181

bb23.i181:                                        ; preds = %bb22.i176
  %63 = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i177, i32 10)
  %_57.1.i185 = extractvalue { i32, i1 } %63, 1
  br i1 %_57.1.i185, label %bb5, label %bb33.i186, !prof !102

bb33.i186:                                        ; preds = %bb23.i181
  %_57.0.i184 = extractvalue { i32, i1 } %63, 0
  %rest.12.i183 = add nsw i64 %src.sroa.15.1.i178, -1
  %rest.01.i182 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i179, i64 1
  %64 = load i8, ptr %src.sroa.0.1.i179, align 1, !alias.scope !119, !noundef !3
  %65 = zext i8 %64 to i32
  %66 = add nsw i32 %65, -48
  %_14.i.i187 = icmp ugt i32 %66, 9
  %_60.0.i189 = add i32 %66, %_57.0.i184
  %_60.1.i190 = icmp ult i32 %_60.0.i189, %_57.0.i184
  %or.cond15 = select i1 %_14.i.i187, i1 true, i1 %_60.1.i190
  br i1 %or.cond15, label %bb5, label %bb22.i176, !prof !103

bb16.i198:                                        ; preds = %bb15.preheader.i196, %bb20.i205
  %src.sroa.0.259.i199 = phi ptr [ %rest.04.i207, %bb20.i205 ], [ %src.sroa.0.0.i174, %bb15.preheader.i196 ]
  %src.sroa.15.258.i200 = phi i64 [ %rest.15.i206, %bb20.i205 ], [ %src.sroa.15.0.i172, %bb15.preheader.i196 ]
  %_19.i202 = load i8, ptr %src.sroa.0.259.i199, align 1, !alias.scope !119, !noundef !3
  %67 = add i8 %_19.i202, -48
  %_14.i47.i204 = icmp ult i8 %67, 10
  br i1 %_14.i47.i204, label %bb20.i205, label %bb5

bb20.i205:                                        ; preds = %bb16.i198
  %rest.15.i206 = add nsw i64 %src.sroa.15.258.i200, -1
  %rest.04.i207 = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i199, i64 1
  %_13.not.i208 = icmp eq i64 %rest.15.i206, 0
  br i1 %_13.not.i208, label %bb50, label %bb16.i198

bb50:                                             ; preds = %bb22.i176, %bb20.i205, %bb15.preheader.i196
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %digits)
  br label %bb6

bb5:                                              ; preds = %bb23.i, %bb33.i, %bb16.i, %bb33.i122, %bb23.i117, %bb16.i134, %bb33.i186, %bb23.i181, %bb16.i198, %bb1.i85, %bb30, %bb29, %bb7.i48, %bb7.i48, %bb24, %bb28.thread52, %bb8.i.thread, %bb45, %bb7.i165, %bb7.i165, %bb35, %bb7.i101, %bb7.i101, %bb27
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %digits)
  br label %bb6
}

; <build_script_build::version::Version>::parse::{closure#3}
; Function Attrs: inlinehint uwtable
define internal fastcc range(i48 0, -65534) i48 @_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parses1_0B6_(ptr readonly captures(address_is_null) %_1.0.val) unnamed_addr #4 personality ptr @rust_eh_personality {
bb2.i.i.i.i.lr.ph.i:
  %_5.i45 = alloca [24 x i8], align 8
  %_5.i = alloca [24 x i8], align 8
  %_5.i.i.i.i.i = alloca [24 x i8], align 8
  %_6 = alloca [72 x i8], align 8
  %commit_date = alloca [80 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %commit_date)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_6)
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_33.0 = load ptr, ptr %_1.0.val, align 8, !nonnull !3, !align !18, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1.0.val, i64 8
  %_33.1 = load i64, ptr %1, align 8, !noundef !3
  store i64 0, ptr %_6, align 8
  %_34.sroa.4.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  store i64 %_33.1, ptr %_34.sroa.4.0._6.sroa_idx, align 8
  %_34.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  store ptr %_33.0, ptr %_34.sroa.5.0._6.sroa_idx, align 8
  %_34.sroa.5.sroa.4.0._34.sroa.5.0._6.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 24
  store i64 %_33.1, ptr %_34.sroa.5.sroa.4.0._34.sroa.5.0._6.sroa_idx.sroa_idx, align 8
  %_34.sroa.5.sroa.5.0._34.sroa.5.0._6.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 32
  store i64 0, ptr %_34.sroa.5.sroa.5.0._34.sroa.5.0._6.sroa_idx.sroa_idx, align 8
  %_34.sroa.5.sroa.6.0._34.sroa.5.0._6.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 40
  store i64 %_33.1, ptr %_34.sroa.5.sroa.6.0._34.sroa.5.0._6.sroa_idx.sroa_idx, align 8
  %_34.sroa.5.sroa.7.0._34.sroa.5.0._6.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 48
  store <2 x i32> splat (i32 10), ptr %_34.sroa.5.sroa.7.0._34.sroa.5.0._6.sroa_idx.sroa_idx, align 8
  %_34.sroa.5.sroa.9.0._34.sroa.5.0._6.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 56
  store i8 1, ptr %_34.sroa.5.sroa.9.0._34.sroa.5.0._6.sroa_idx.sroa_idx, align 8
  %_34.sroa.6.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 64
  store i8 0, ptr %_34.sroa.6.0._6.sroa_idx, align 8
  %_34.sroa.7.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 65
  store i8 0, ptr %_34.sroa.7.0._6.sroa_idx, align 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !122)
  %2 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i, i64 16
  br label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses1_00E0B1n_.exit.i, %bb2.i.i.i.i.lr.ph.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !125)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !128)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !131)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !134)
  %_4.val.i.i.i.i.i = load ptr, ptr %_34.sroa.5.0._6.sroa_idx, align 8, !alias.scope !137, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !137
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_34.sroa.5.0._6.sroa_idx) #28
  %_7.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i, align 8, !range !37, !noalias !137, !noundef !3
  %3 = trunc nuw i64 %_7.i.i.i.i.i to i1
  br i1 %3, label %bb7.i.i.i.i.i, label %bb6.i.i.i.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i
  %b.i.i.i.i.i = load i64, ptr %2, align 8, !noalias !137, !noundef !3
  %i.i.i.i.i.i = load i64, ptr %_6, align 8, !alias.scope !137, !noundef !3
  %new_len.i.i.i.i.i = sub nuw i64 %b.i.i.i.i.i, %i.i.i.i.i.i
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i.i.i, i64 %i.i.i.i.i.i
  store i64 %b.i.i.i.i.i, ptr %_6, align 8, !alias.scope !137
  br label %bb5.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i
  %4 = load i8, ptr %_34.sroa.7.0._6.sroa_idx, align 1, !range !54, !alias.scope !138, !noundef !3
  %_2.i.i.i.i.i.i = trunc nuw i8 %4 to i1
  br i1 %_2.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i, label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i
  store i8 1, ptr %_34.sroa.7.0._6.sroa_idx, align 1, !alias.scope !138
  %5 = load i8, ptr %_34.sroa.6.0._6.sroa_idx, align 8, !range !54, !alias.scope !138, !noundef !3
  %_3.i.i.i.i.i.i = trunc nuw i8 %5 to i1
  %i.pre.i.i.i.i.i.i = load i64, ptr %_6, align 8, !alias.scope !138
  %i1.pre.i.i.i.i.i.i = load i64, ptr %_34.sroa.4.0._6.sroa_idx, align 8, !alias.scope !138
  %_4.not.i.i.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i
  %or.cond.not.i.i.i.i.i.i = select i1 %_3.i.i.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i.i.i
  br i1 %or.cond.not.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i
  %_10.val.i.i.i.i.i.i = load ptr, ptr %_34.sroa.5.0._6.sroa_idx, align 8, !alias.scope !138, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i
  %data.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i.i, i64 %i.pre.i.i.i.i.i.i
  br label %bb5.i.i.i

_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i: ; preds = %bb1.i.i.i.i.i.i, %bb6.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !137
  br label %bb20

bb5.i.i.i:                                        ; preds = %bb4.i.i.i.i.i.i, %bb7.i.i.i.i.i
  %_0.sroa.4.0.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i, %bb7.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i.i = phi ptr [ %data.i.i.i.i.i, %bb7.i.i.i.i.i ], [ %data.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i), !noalias !137
  %6 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i.i.i.i.i, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %_0.sroa.4.0.i.i.i.i.i, 1
  %_5.not.i.i.i.i.i.i.i = icmp eq i64 %_0.sroa.4.0.i.i.i.i.i, 0
  br i1 %_5.not.i.i.i.i.i.i.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i: ; preds = %bb5.i.i.i
  %8 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i, i64 %_0.sroa.4.0.i.i.i.i.i
  %_16.i.i.i.i.i.i.i = getelementptr i8, ptr %8, i64 -1
  %rhsc.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !141, !noalias !150
  %rhsc.i.i.fr.i.i.i.i.i = freeze i8 %rhsc.i.i.i.i.i.i.i
  %9 = icmp eq i8 %rhsc.i.i.fr.i.i.i.i.i, 10
  %i.i.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i, -1
  br i1 %9, label %bb1.i.i.i.i.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i
  %10 = insertvalue { ptr, i64 } %7, i64 %i.i.i.i.i.i.i, 1
  %_5.not.i.i12.i.i.i.i.i = icmp eq i64 %i.i.i.i.i.i.i, 0
  br i1 %_5.not.i.i12.i.i.i.i.i, label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i
  %11 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i, i64 %i.i.i.i.i.i.i
  %_16.i.i14.i.i.i.i.i = getelementptr i8, ptr %11, i64 -1
  %rhsc.i.i16.i.i.i.i.i = load i8, ptr %_16.i.i14.i.i.i.i.i, align 1, !alias.scope !153, !noalias !158
  %rhsc.i.i16.fr.i.i.i.i.i = freeze i8 %rhsc.i.i16.i.i.i.i.i
  %12 = icmp eq i8 %rhsc.i.i16.fr.i.i.i.i.i, 13
  %i.i17.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i, -2
  %spec.select.i19.i.i.i.i.i = select i1 %12, ptr %_0.sroa.0.0.i.i.i.i.i, ptr null
  br label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i

_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i: ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i, %bb1.i.i.i.i.i
  %i4.i20.i.i.i.i.i = phi i64 [ %i.i17.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i ], [ -1, %bb1.i.i.i.i.i ]
  %13 = phi ptr [ %spec.select.i19.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i ], [ null, %bb1.i.i.i.i.i ]
  %14 = insertvalue { ptr, i64 } poison, ptr %13, 0
  %15 = insertvalue { ptr, i64 } %14, i64 %i4.i20.i.i.i.i.i, 1
  %.not11.i.i.i.i.i = icmp eq ptr %13, null
  %..i.i.i.i.i = select i1 %.not11.i.i.i.i.i, { ptr, i64 } %10, { ptr, i64 } %15
  br label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i

_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i, %bb5.i.i.i
  %.merged.i.i.i.i.i = phi { ptr, i64 } [ %..i.i.i.i.i, %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i ], [ %7, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i ], [ %7, %bb5.i.i.i ]
  %_7.0.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i, 0
  %_7.1.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i, 1
  %.not.i = icmp eq ptr %_7.0.i.i.i, null
  br i1 %.not.i, label %bb20, label %bb3.i

bb3.i:                                            ; preds = %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i
  %_4.not.i.i.i.i = icmp samesign ult i64 %_7.1.i.i.i, 13
  br i1 %_4.not.i.i.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses1_00E0B1n_.exit.i, label %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses1_00B8_.exit.i.i

_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses1_00B8_.exit.i.i: ; preds = %bb3.i
  %16 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(13) @alloc_ed740c9299d7724b54ca9f50b8396b41, ptr noundef nonnull readonly align 1 dereferenceable(13) %_7.0.i.i.i, i64 range(i64 0, -9223372036854775808) 13), !alias.scope !161, !noalias !122
  %.fr.i.i = freeze i32 %16
  %17 = icmp eq i32 %.fr.i.i, 0
  br i1 %17, label %bb15, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses1_00E0B1n_.exit.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses1_00E0B1n_.exit.i: ; preds = %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses1_00B8_.exit.i.i, %bb3.i
  %18 = load i8, ptr %_34.sroa.7.0._6.sroa_idx, align 1, !range !54, !alias.scope !168, !noundef !3
  %_2.i.i.i.i.i = trunc nuw i8 %18 to i1
  br i1 %_2.i.i.i.i.i, label %bb20, label %bb2.i.i.i.i.i

bb15:                                             ; preds = %_RNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB6_7Version5parses1_00B8_.exit.i.i
  %_8.not.i.not = icmp eq i64 %_7.1.i.i.i, 13
  br i1 %_8.not.i.not, label %bb24, label %bb9.i

bb9.i:                                            ; preds = %bb15
  %19 = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i, i64 13
  %self1.i = load i8, ptr %19, align 1, !alias.scope !173, !noundef !3
  %20 = icmp sgt i8 %self1.i, -65
  br i1 %20, label %bb24, label %bb18

bb20:                                             ; preds = %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1l_7Version5parses1_00E0B1n_.exit.i, %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_6)
  br label %bb12

bb18:                                             ; preds = %bb9.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_7.0.i.i.i, i64 noundef %_7.1.i.i.i, i64 noundef 13, i64 noundef %_7.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5a70f0d82a4daf23c5a2b55427a97995) #29
  unreachable

bb24:                                             ; preds = %bb15, %bb9.i
  %new_len.i = add i64 %_7.1.i.i.i, -13
  %data.i = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i, i64 13
  %_54.sroa.4.0.commit_date.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 8
  %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 16
  store i64 %new_len.i, ptr %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 16
  %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 24
  store ptr %data.i, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8
  %_54.sroa.4.sroa.5.sroa.4.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 32
  store i64 %new_len.i, ptr %_54.sroa.4.sroa.5.sroa.4.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx, align 16
  %_54.sroa.4.sroa.5.sroa.5.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 40
  store i64 0, ptr %_54.sroa.4.sroa.5.sroa.5.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx, align 8
  %_54.sroa.4.sroa.5.sroa.6.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 48
  store i64 %new_len.i, ptr %_54.sroa.4.sroa.5.sroa.6.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx, align 16
  %_54.sroa.4.sroa.5.sroa.7.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 56
  store <2 x i32> splat (i32 45), ptr %_54.sroa.4.sroa.5.sroa.7.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx, align 8
  %_54.sroa.4.sroa.5.sroa.9.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 64
  store i8 1, ptr %_54.sroa.4.sroa.5.sroa.9.0._54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx.sroa_idx, align 16
  %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 72
  store i8 1, ptr %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8
  %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %commit_date, i64 73
  store i8 0, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_6)
  store <2 x i64> <i64 2, i64 0>, ptr %commit_date, align 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !176)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !176
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef align 8 dereferenceable(48) %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx) #28
  %_7.i = load i64, ptr %_5.i, align 8, !range !37, !noalias !176, !noundef !3
  %21 = trunc nuw i64 %_7.i to i1
  br i1 %21, label %bb7.i, label %bb6.i30

bb7.i:                                            ; preds = %bb24
  %22 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %a.i = load i64, ptr %22, align 8, !noalias !176, !noundef !3
  %23 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  %b.i = load i64, ptr %23, align 8, !noalias !176, !noundef !3
  %i.i = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !176, !noundef !3
  %new_len.i33 = sub nuw i64 %a.i, %i.i
  %data.i34 = getelementptr inbounds nuw i8, ptr %data.i, i64 %i.i
  store i64 %b.i, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !176
  br label %bb30

bb6.i30:                                          ; preds = %bb24
  %24 = load i8, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !179, !noundef !3
  %_2.i.i = trunc nuw i8 %24 to i1
  br i1 %_2.i.i, label %bb8.i.thread, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i30
  store i8 1, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !alias.scope !179
  %25 = load i8, ptr %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !179, !noundef !3
  %_3.i.i = trunc nuw i8 %25 to i1
  %i.pre.i.i = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !179
  %i1.pre.i.i = load i64, ptr %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 16, !alias.scope !179
  %_4.not.i.i = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i
  br i1 %or.cond.not.i.i, label %bb4.i.i, label %bb8.i.thread

bb4.i.i:                                          ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !alias.scope !179, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb30

bb8.i.thread:                                     ; preds = %bb6.i30, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !176
  br label %bb12

bb30:                                             ; preds = %bb7.i, %bb4.i.i
  %_0.sroa.4.0.i = phi i64 [ %new_len.i33, %bb7.i ], [ %new_len.i.i, %bb4.i.i ]
  %_0.sroa.0.0.i31 = phi ptr [ %data.i34, %bb7.i ], [ %data.i.i, %bb4.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !176
  switch i64 %_0.sroa.4.0.i, label %bb9thread-pre-split.i [
    i64 0, label %bb12
    i64 1, label %bb7.i41
  ]

bb7.i41:                                          ; preds = %bb30
  %26 = load i8, ptr %_0.sroa.0.0.i31, align 1, !alias.scope !182, !noundef !3
  switch i8 %26, label %bb9.i43 [
    i8 43, label %bb12
    i8 45, label %bb12
  ]

bb9thread-pre-split.i:                            ; preds = %bb30
  %.pr.i = load i8, ptr %_0.sroa.0.0.i31, align 1, !alias.scope !182
  br label %bb9.i43

bb9.i43:                                          ; preds = %bb9thread-pre-split.i, %bb7.i41
  %27 = phi i8 [ %.pr.i, %bb9thread-pre-split.i ], [ %26, %bb7.i41 ]
  %cond.i = icmp eq i8 %27, 43
  %rest.1.i = sext i1 %cond.i to i64
  %src.sroa.15.0.i = add nsw i64 %_0.sroa.4.0.i, %rest.1.i
  %src.sroa.0.0.idx.i = zext i1 %cond.i to i64
  %src.sroa.0.0.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i31, i64 %src.sroa.0.0.idx.i
  %_10.i44 = icmp samesign ult i64 %src.sroa.15.0.i, 5
  br i1 %_10.i44, label %bb15.preheader.i, label %bb22.i

bb15.preheader.i:                                 ; preds = %bb9.i43
  %_13.not55.i = icmp eq i64 %src.sroa.15.0.i, 0
  br i1 %_13.not55.i, label %bb33, label %bb16.i

bb22.i:                                           ; preds = %bb9.i43, %bb33.i
  %result.sroa.0.0.i = phi i16 [ %_63.0.i, %bb33.i ], [ 0, %bb9.i43 ]
  %src.sroa.15.1.i = phi i64 [ %rest.12.i, %bb33.i ], [ %src.sroa.15.0.i, %bb9.i43 ]
  %src.sroa.0.1.i = phi ptr [ %rest.01.i, %bb33.i ], [ %src.sroa.0.0.i, %bb9.i43 ]
  %_30.not.i = icmp eq i64 %src.sroa.15.1.i, 0
  br i1 %_30.not.i, label %bb33, label %bb23.i

bb23.i:                                           ; preds = %bb22.i
  %28 = tail call { i16, i1 } @llvm.umul.with.overflow.i16(i16 %result.sroa.0.0.i, i16 10)
  %_60.1.i = extractvalue { i16, i1 } %28, 1
  br i1 %_60.1.i, label %bb12, label %bb33.i, !prof !102

bb33.i:                                           ; preds = %bb23.i
  %_60.0.i = extractvalue { i16, i1 } %28, 0
  %rest.12.i = add nsw i64 %src.sroa.15.1.i, -1
  %rest.01.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i, i64 1
  %29 = load i8, ptr %src.sroa.0.1.i, align 1, !alias.scope !182, !noundef !3
  %30 = zext i8 %29 to i32
  %31 = add nsw i32 %30, -48
  %_14.i.i = icmp ugt i32 %31, 9
  %32 = trunc nuw nsw i32 %31 to i16
  %_63.0.i = add i16 %_60.0.i, %32
  %_63.1.i = icmp ult i16 %_63.0.i, %_60.0.i
  %or.cond11 = select i1 %_14.i.i, i1 true, i1 %_63.1.i
  br i1 %or.cond11, label %bb12, label %bb22.i, !prof !103

bb16.i:                                           ; preds = %bb15.preheader.i, %bb20.i
  %src.sroa.0.258.i = phi ptr [ %rest.05.i, %bb20.i ], [ %src.sroa.0.0.i, %bb15.preheader.i ]
  %src.sroa.15.257.i = phi i64 [ %rest.16.i, %bb20.i ], [ %src.sroa.15.0.i, %bb15.preheader.i ]
  %result.sroa.0.256.i = phi i16 [ %35, %bb20.i ], [ 0, %bb15.preheader.i ]
  %_20.i = load i8, ptr %src.sroa.0.258.i, align 1, !alias.scope !182, !noundef !3
  %_19.i = zext i8 %_20.i to i32
  %33 = add nsw i32 %_19.i, -48
  %_14.i46.i = icmp ult i32 %33, 10
  br i1 %_14.i46.i, label %bb20.i, label %bb12

bb20.i:                                           ; preds = %bb16.i
  %34 = mul i16 %result.sroa.0.256.i, 10
  %rest.16.i = add nsw i64 %src.sroa.15.257.i, -1
  %rest.05.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.258.i, i64 1
  %_24.i = trunc nuw nsw i32 %33 to i16
  %35 = add i16 %34, %_24.i
  %_13.not.i = icmp eq i64 %rest.16.i, 0
  br i1 %_13.not.i, label %bb33, label %bb16.i

bb33:                                             ; preds = %bb22.i, %bb20.i, %bb15.preheader.i
  %result.sroa.0.1.i = phi i16 [ 0, %bb15.preheader.i ], [ %35, %bb20.i ], [ %result.sroa.0.0.i, %bb22.i ]
  %36 = zext i16 %result.sroa.0.1.i to i32
  %37 = load i64, ptr %commit_date, align 16, !noundef !3
  switch i64 %37, label %bb35 [
    i64 0, label %bb12
    i64 1, label %bb36
  ]

bb35:                                             ; preds = %bb33
  %38 = add i64 %37, -1
  store i64 %38, ptr %commit_date, align 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !185)
  %39 = load i8, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !185, !noundef !3
  %_2.i46 = trunc nuw i8 %39 to i1
  br i1 %_2.i46, label %bb12, label %bb2.i47

bb2.i47:                                          ; preds = %bb35
  %_4.val.i49 = load ptr, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !alias.scope !185, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i45), !noalias !185
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i45, ptr noalias noundef align 8 dereferenceable(48) %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx) #28
  %_7.i50 = load i64, ptr %_5.i45, align 8, !range !37, !noalias !185, !noundef !3
  %40 = trunc nuw i64 %_7.i50 to i1
  br i1 %40, label %bb7.i70, label %bb6.i51

bb7.i70:                                          ; preds = %bb2.i47
  %41 = getelementptr inbounds nuw i8, ptr %_5.i45, i64 8
  %a.i71 = load i64, ptr %41, align 8, !noalias !185, !noundef !3
  %42 = getelementptr inbounds nuw i8, ptr %_5.i45, i64 16
  %b.i72 = load i64, ptr %42, align 8, !noalias !185, !noundef !3
  %i.i73 = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !185, !noundef !3
  %new_len.i74 = sub nuw i64 %a.i71, %i.i73
  %data.i75 = getelementptr inbounds nuw i8, ptr %_4.val.i49, i64 %i.i73
  store i64 %b.i72, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !185
  br label %bb34

bb6.i51:                                          ; preds = %bb2.i47
  %43 = load i8, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !188, !noundef !3
  %_2.i.i52 = trunc nuw i8 %43 to i1
  br i1 %_2.i.i52, label %bb34.thread45, label %bb1.i.i53

bb1.i.i53:                                        ; preds = %bb6.i51
  store i8 1, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !alias.scope !188
  %44 = load i8, ptr %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !188, !noundef !3
  %_3.i.i54 = trunc nuw i8 %44 to i1
  %i.pre.i.i55 = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !188
  %i1.pre.i.i57 = load i64, ptr %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 16, !alias.scope !188
  %_4.not.i.i58 = icmp ne i64 %i1.pre.i.i57, %i.pre.i.i55
  %or.cond.not.i.i59 = select i1 %_3.i.i54, i1 true, i1 %_4.not.i.i58
  br i1 %or.cond.not.i.i59, label %bb4.i.i66, label %bb34.thread45

bb4.i.i66:                                        ; preds = %bb1.i.i53
  %_10.val.i.i67 = load ptr, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !alias.scope !188, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i68 = sub nuw i64 %i1.pre.i.i57, %i.pre.i.i55
  %data.i.i69 = getelementptr inbounds nuw i8, ptr %_10.val.i.i67, i64 %i.pre.i.i55
  br label %bb34

bb36:                                             ; preds = %bb33
  store i64 0, ptr %commit_date, align 16
  %45 = load i8, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !191, !noundef !3
  %_2.i77 = trunc nuw i8 %45 to i1
  br i1 %_2.i77, label %bb12, label %bb1.i78

bb1.i78:                                          ; preds = %bb36
  store i8 1, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !alias.scope !191
  %46 = load i8, ptr %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !191, !noundef !3
  %_3.i79 = trunc nuw i8 %46 to i1
  %i.pre.i80 = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !191
  %i1.pre.i82 = load i64, ptr %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 16, !alias.scope !191
  %_4.not.i83 = icmp ne i64 %i1.pre.i82, %i.pre.i80
  %or.cond.not.i84 = select i1 %_3.i79, i1 true, i1 %_4.not.i83
  br i1 %or.cond.not.i84, label %bb34.thread38, label %bb12

bb34.thread38:                                    ; preds = %bb1.i78
  %_10.val.i90 = load ptr, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !alias.scope !191, !nonnull !3, !align !18, !noundef !3
  %new_len.i91 = sub nuw i64 %i1.pre.i82, %i.pre.i80
  %data.i92 = getelementptr inbounds nuw i8, ptr %_10.val.i90, i64 %i.pre.i80
  br label %bb41

bb34.thread45:                                    ; preds = %bb6.i51, %bb1.i.i53
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i45), !noalias !185
  br label %bb12

bb34:                                             ; preds = %bb7.i70, %bb4.i.i66
  %_0.sroa.4.0.i61 = phi i64 [ %new_len.i74, %bb7.i70 ], [ %new_len.i.i68, %bb4.i.i66 ]
  %_0.sroa.0.0.i62 = phi ptr [ %data.i75, %bb7.i70 ], [ %data.i.i69, %bb4.i.i66 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i45), !noalias !185
  br label %bb41

bb41:                                             ; preds = %bb34, %bb34.thread38
  %_0.sroa.4.1.i64.pn43 = phi i64 [ %new_len.i91, %bb34.thread38 ], [ %_0.sroa.4.0.i61, %bb34 ]
  %_0.sroa.0.1.i65.pn42 = phi ptr [ %data.i92, %bb34.thread38 ], [ %_0.sroa.0.0.i62, %bb34 ]
  switch i64 %_0.sroa.4.1.i64.pn43, label %bb9thread-pre-split.i129 [
    i64 0, label %bb12
    i64 1, label %bb7.i94
  ]

bb7.i94:                                          ; preds = %bb41
  %47 = load i8, ptr %_0.sroa.0.1.i65.pn42, align 1, !alias.scope !194, !noundef !3
  switch i8 %47, label %bb9.i95 [
    i8 43, label %bb12
    i8 45, label %bb12
  ]

bb9thread-pre-split.i129:                         ; preds = %bb41
  %.pr.i130 = load i8, ptr %_0.sroa.0.1.i65.pn42, align 1, !alias.scope !194
  br label %bb9.i95

bb9.i95:                                          ; preds = %bb9thread-pre-split.i129, %bb7.i94
  %48 = phi i8 [ %.pr.i130, %bb9thread-pre-split.i129 ], [ %47, %bb7.i94 ]
  %cond.i96 = icmp eq i8 %48, 43
  %rest.1.i97 = sext i1 %cond.i96 to i64
  %src.sroa.15.0.i98 = add nsw i64 %_0.sroa.4.1.i64.pn43, %rest.1.i97
  %src.sroa.0.0.idx.i99 = zext i1 %cond.i96 to i64
  %src.sroa.0.0.i100 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i65.pn42, i64 %src.sroa.0.0.idx.i99
  %_10.i101 = icmp samesign ult i64 %src.sroa.15.0.i98, 3
  br i1 %_10.i101, label %bb15.preheader.i119, label %bb22.i102

bb15.preheader.i119:                              ; preds = %bb9.i95
  %_13.not52.i = icmp eq i64 %src.sroa.15.0.i98, 0
  br i1 %_13.not52.i, label %bb44, label %bb16.i120

bb22.i102:                                        ; preds = %bb9.i95, %bb33.i111
  %result.sroa.0.0.i103 = phi i8 [ %_63.0.i114, %bb33.i111 ], [ 0, %bb9.i95 ]
  %src.sroa.15.1.i104 = phi i64 [ %rest.12.i108, %bb33.i111 ], [ %src.sroa.15.0.i98, %bb9.i95 ]
  %src.sroa.0.1.i105 = phi ptr [ %rest.01.i107, %bb33.i111 ], [ %src.sroa.0.0.i100, %bb9.i95 ]
  %_30.not.not.i = icmp eq i64 %src.sroa.15.1.i104, 0
  br i1 %_30.not.not.i, label %bb44, label %bb23.i106

bb23.i106:                                        ; preds = %bb22.i102
  %49 = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %result.sroa.0.0.i103, i8 10)
  %_60.1.i110 = extractvalue { i8, i1 } %49, 1
  br i1 %_60.1.i110, label %bb12, label %bb33.i111, !prof !102

bb33.i111:                                        ; preds = %bb23.i106
  %_60.0.i109 = extractvalue { i8, i1 } %49, 0
  %rest.12.i108 = add nsw i64 %src.sroa.15.1.i104, -1
  %rest.01.i107 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i105, i64 1
  %50 = load i8, ptr %src.sroa.0.1.i105, align 1, !alias.scope !194, !noundef !3
  %51 = zext i8 %50 to i32
  %52 = add nsw i32 %51, -48
  %_14.i.i112 = icmp ugt i32 %52, 9
  %53 = trunc nuw nsw i32 %52 to i8
  %_63.0.i114 = add i8 %_60.0.i109, %53
  %_63.1.i115 = icmp ult i8 %_63.0.i114, %_60.0.i109
  %or.cond9 = select i1 %_14.i.i112, i1 true, i1 %_63.1.i115
  br i1 %or.cond9, label %bb12, label %bb22.i102, !prof !103

bb16.i120:                                        ; preds = %bb15.preheader.i119, %bb20.i124
  %src.sroa.0.255.i = phi ptr [ %rest.05.i126, %bb20.i124 ], [ %src.sroa.0.0.i100, %bb15.preheader.i119 ]
  %src.sroa.15.254.i = phi i64 [ %rest.16.i125, %bb20.i124 ], [ %src.sroa.15.0.i98, %bb15.preheader.i119 ]
  %result.sroa.0.253.i = phi i8 [ %56, %bb20.i124 ], [ 0, %bb15.preheader.i119 ]
  %_20.i121 = load i8, ptr %src.sroa.0.255.i, align 1, !alias.scope !194, !noundef !3
  %_19.i122 = zext i8 %_20.i121 to i32
  %54 = add nsw i32 %_19.i122, -48
  %_14.i46.i123 = icmp ugt i32 %54, 9
  br i1 %_14.i46.i123, label %bb12, label %bb20.i124

bb20.i124:                                        ; preds = %bb16.i120
  %55 = mul i8 %result.sroa.0.253.i, 10
  %rest.16.i125 = add nsw i64 %src.sroa.15.254.i, -1
  %rest.05.i126 = getelementptr inbounds nuw i8, ptr %src.sroa.0.255.i, i64 1
  %_24.i127 = trunc nuw nsw i32 %54 to i8
  %56 = add i8 %55, %_24.i127
  %_13.not.i128 = icmp eq i64 %rest.16.i125, 0
  br i1 %_13.not.i128, label %bb44, label %bb16.i120

bb44:                                             ; preds = %bb22.i102, %bb20.i124, %bb15.preheader.i119
  %_0.sroa.8.0.i = phi i8 [ 0, %bb15.preheader.i119 ], [ %56, %bb20.i124 ], [ %result.sroa.0.0.i103, %bb22.i102 ]
  %57 = load i64, ptr %commit_date, align 16, !noundef !3
  switch i64 %57, label %bb46 [
    i64 0, label %bb12
    i64 1, label %bb47
  ]

bb46:                                             ; preds = %bb44
  %58 = add i64 %57, -1
  store i64 %58, ptr %commit_date, align 16
; call <core::str::iter::SplitInternal<char>>::next
  %59 = call fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %_54.sroa.4.0.commit_date.sroa_idx) #28
  br label %bb45

bb47:                                             ; preds = %bb44
  %60 = load i8, ptr %_54.sroa.4.sroa.7.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 1, !range !54, !alias.scope !197, !noundef !3
  %_2.i131 = trunc nuw i8 %60 to i1
  br i1 %_2.i131, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147, label %bb1.i132

bb1.i132:                                         ; preds = %bb47
  %61 = load i8, ptr %_54.sroa.4.sroa.6.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !range !54, !alias.scope !197, !noundef !3
  %_3.i133 = trunc nuw i8 %61 to i1
  %i.pre.i134 = load i64, ptr %_54.sroa.4.0.commit_date.sroa_idx, align 8, !alias.scope !197
  %i1.pre.i136 = load i64, ptr %_54.sroa.4.sroa.4.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 16, !alias.scope !197
  %_4.not.i137 = icmp ne i64 %i1.pre.i136, %i.pre.i134
  %or.cond.not.i138 = select i1 %_3.i133, i1 true, i1 %_4.not.i137
  br i1 %or.cond.not.i138, label %bb4.i142, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147

bb4.i142:                                         ; preds = %bb1.i132
  %_10.val.i144 = load ptr, ptr %_54.sroa.4.sroa.5.0._54.sroa.4.0.commit_date.sroa_idx.sroa_idx, align 8, !alias.scope !197, !nonnull !3, !align !18, !noundef !3
  %new_len.i145 = sub nuw i64 %i1.pre.i136, %i.pre.i134
  %data.i146 = getelementptr inbounds nuw i8, ptr %_10.val.i144, i64 %i.pre.i134
  br label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147

_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147: ; preds = %bb47, %bb1.i132, %bb4.i142
  %_0.sroa.3.0.i140 = phi i64 [ %new_len.i145, %bb4.i142 ], [ undef, %bb47 ], [ undef, %bb1.i132 ]
  %_0.sroa.0.0.i141 = phi ptr [ %data.i146, %bb4.i142 ], [ null, %bb47 ], [ null, %bb1.i132 ]
  %62 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i141, 0
  %63 = insertvalue { ptr, i64 } %62, i64 %_0.sroa.3.0.i140, 1
  br label %bb45

bb45:                                             ; preds = %bb46, %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147
  %.pn24 = phi { ptr, i64 } [ %59, %bb46 ], [ %63, %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit147 ]
  %_26.sroa.0.0 = extractvalue { ptr, i64 } %.pn24, 0
  %.not26 = icmp eq ptr %_26.sroa.0.0, null
  br i1 %.not26, label %bb12, label %bb52

bb52:                                             ; preds = %bb45
  %_26.sroa.7.0 = extractvalue { ptr, i64 } %.pn24, 1
; call <u8>::from_ascii_radix
  %64 = tail call fastcc { i1, i8 } @_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_26.sroa.0.0, i64 noundef %_26.sroa.7.0) #28
  %65 = extractvalue { i1, i8 } %64, 0
  %66 = extractvalue { i1, i8 } %64, 1
  br i1 %65, label %bb12, label %bb55

bb55:                                             ; preds = %bb52
  %_29 = icmp ugt i8 %_0.sroa.8.0.i, 12
  %_30 = icmp ugt i8 %66, 31
  %or.cond = or i1 %_29, %_30
  br i1 %or.cond, label %bb12, label %bb3

bb3:                                              ; preds = %bb55
  %_31.sroa.5.0.insert.ext = zext nneg i8 %66 to i32
  %_31.sroa.5.0.insert.shift = shl nuw nsw i32 %_31.sroa.5.0.insert.ext, 24
  %_31.sroa.4.0.insert.ext = zext nneg i8 %_0.sroa.8.0.i to i32
  %_31.sroa.4.0.insert.shift = shl nuw nsw i32 %_31.sroa.4.0.insert.ext, 16
  %_31.sroa.4.0.insert.insert = or disjoint i32 %_31.sroa.5.0.insert.shift, %_31.sroa.4.0.insert.shift
  %_31.sroa.0.0.insert.insert = or disjoint i32 %_31.sroa.4.0.insert.insert, %36
  br label %bb12

bb12:                                             ; preds = %bb33.i, %bb23.i, %bb16.i, %bb33.i111, %bb23.i106, %bb16.i120, %bb20, %bb33, %bb45, %bb44, %bb52, %bb55, %bb41, %bb7.i94, %bb7.i94, %bb8.i.thread, %bb34.thread45, %bb30, %bb7.i41, %bb7.i41, %bb35, %bb36, %bb1.i78, %bb3
  %_0.sroa.10.0 = phi i32 [ %_31.sroa.0.0.insert.insert, %bb3 ], [ undef, %bb1.i78 ], [ undef, %bb36 ], [ undef, %bb35 ], [ undef, %bb7.i41 ], [ undef, %bb7.i41 ], [ undef, %bb30 ], [ undef, %bb34.thread45 ], [ undef, %bb8.i.thread ], [ undef, %bb7.i94 ], [ undef, %bb7.i94 ], [ undef, %bb41 ], [ undef, %bb55 ], [ undef, %bb52 ], [ undef, %bb44 ], [ undef, %bb45 ], [ undef, %bb33 ], [ undef, %bb20 ], [ undef, %bb16.i120 ], [ undef, %bb23.i106 ], [ undef, %bb33.i111 ], [ undef, %bb16.i ], [ undef, %bb23.i ], [ undef, %bb33.i ]
  %_0.sroa.0.4 = phi i16 [ 1, %bb3 ], [ 0, %bb1.i78 ], [ 0, %bb36 ], [ 0, %bb35 ], [ 0, %bb7.i41 ], [ 0, %bb7.i41 ], [ 0, %bb30 ], [ 0, %bb34.thread45 ], [ 0, %bb8.i.thread ], [ 0, %bb7.i94 ], [ 0, %bb7.i94 ], [ 0, %bb41 ], [ 0, %bb55 ], [ 0, %bb52 ], [ 0, %bb44 ], [ 0, %bb45 ], [ 0, %bb33 ], [ 0, %bb20 ], [ 0, %bb16.i120 ], [ 0, %bb23.i106 ], [ 0, %bb33.i111 ], [ 0, %bb16.i ], [ 0, %bb23.i ], [ 0, %bb33.i ]
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %commit_date)
  %_0.sroa.10.0.insert.ext = zext i32 %_0.sroa.10.0 to i48
  %_0.sroa.10.0.insert.shift = shl nuw i48 %_0.sroa.10.0.insert.ext, 16
  %_0.sroa.0.0.insert.ext = zext nneg i16 %_0.sroa.0.4 to i48
  %_0.sroa.0.0.insert.insert = or disjoint i48 %_0.sroa.10.0.insert.shift, %_0.sroa.0.0.insert.ext
  ret i48 %_0.sroa.0.0.insert.insert
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCslwKqnJYeWCA_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECslwKqnJYeWCA_18build_script_build(ptr noundef nonnull readonly %0) #27, !noalias !200
  ret i32 0
}

; build_script_build::target_cpu
; Function Attrs: uwtable
define internal fastcc void @_RNvCslwKqnJYeWCA_18build_script_build10target_cpu(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5.i = alloca [24 x i8], align 8
  %iter = alloca [72 x i8], align 8
  %rustflags1 = alloca [24 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
  %0 = load i64, ptr %_3, align 8, !range !11, !noundef !3
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb16, label %bb17

bb17:                                             ; preds = %start
  %_19.sroa.5.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_19.sroa.5.0.copyload = load ptr, ptr %_19.sroa.5.0._3.sroa_idx, align 8, !nonnull !3, !noundef !3
  %_19.sroa.6.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_19.sroa.6.0.copyload = load i64, ptr %_19.sroa.6.0._3.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rustflags1)
; invoke <alloc::string::String>::from_utf8_lossy
  invoke void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %rustflags1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_19.sroa.5.0.copyload, i64 noundef %_19.sroa.6.0.copyload)
          to label %bb2.i.lr.ph unwind label %cleanup

bb16:                                             ; preds = %start
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb12

bb12:                                             ; preds = %bb2.i.i.i4.i.i.i40, %bb10, %bb16
  ret void

bb14:                                             ; preds = %bb2.i.i.i4.i.i.i28, %cleanup2, %cleanup2, %cleanup
  %.pn = phi { ptr, i32 } [ %2, %cleanup ], [ %lpad.phi, %cleanup2 ], [ %lpad.phi, %cleanup2 ], [ %lpad.phi, %bb2.i.i.i4.i.i.i28 ]
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %bb15, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb14
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_19.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb15

cleanup:                                          ; preds = %bb17
  %2 = landingpad { ptr, i32 }
          cleanup
  br label %bb14

cleanup2.loopexit:                                ; preds = %bb2.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2

cleanup2.loopexit.split-lp:                       ; preds = %bb7.i34.invoke, %bb28
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2

cleanup2:                                         ; preds = %cleanup2.loopexit.split-lp, %cleanup2.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup2.loopexit ], [ %lpad.loopexit.split-lp, %cleanup2.loopexit.split-lp ]
  switch i64 %4, label %bb2.i.i.i4.i.i.i28 [
    i64 -9223372036854775808, label %bb14
    i64 0, label %bb14
  ]

bb2.i.i.i4.i.i.i28:                               ; preds = %cleanup2
  %3 = icmp ne ptr %_34, null
  call void @llvm.assume(i1 %3)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb14

bb2.i.lr.ph:                                      ; preds = %bb17
  %4 = load i64, ptr %rustflags1, align 8, !range !11, !noundef !3
  %5 = getelementptr inbounds nuw i8, ptr %rustflags1, i64 8
  %_34 = load ptr, ptr %5, align 8
  %6 = getelementptr inbounds nuw i8, ptr %rustflags1, i64 16
  %_33 = load i64, ptr %6, align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter)
  store i64 0, ptr %iter, align 8
  %_7.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %_33, ptr %_7.sroa.2.0.iter.sroa_idx, align 8
  %_7.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %_34, ptr %_7.sroa.3.0.iter.sroa_idx, align 8
  %_7.sroa.3.sroa.2.0._7.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store i64 %_33, ptr %_7.sroa.3.sroa.2.0._7.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_7.sroa.3.sroa.3.0._7.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 0, ptr %_7.sroa.3.sroa.3.0._7.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_7.sroa.3.sroa.4.0._7.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  store i64 %_33, ptr %_7.sroa.3.sroa.4.0._7.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_7.sroa.3.sroa.5.0._7.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store <2 x i32> splat (i32 31), ptr %_7.sroa.3.sroa.5.0._7.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_7.sroa.3.sroa.7.0._7.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 56
  store i8 1, ptr %_7.sroa.3.sroa.7.0._7.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_7.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i8 1, ptr %_7.sroa.4.0.iter.sroa_idx, align 8
  %_7.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 65
  store i8 0, ptr %_7.sroa.5.0.iter.sroa_idx, align 1
  %7 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %8 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  br label %bb2.i

bb2.i:                                            ; preds = %bb2.i.lr.ph, %bb7
  %cpu.sroa.0.083 = phi ptr [ null, %bb2.i.lr.ph ], [ %spec.select21, %bb7 ]
  %cpu.sroa.4.082 = phi i64 [ undef, %bb2.i.lr.ph ], [ %spec.select20, %bb7 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !203)
  %_4.val.i = load ptr, ptr %_7.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !203, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !203
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef align 8 dereferenceable(48) %_7.sroa.3.0.iter.sroa_idx) #28
          to label %.noexc unwind label %cleanup2.loopexit

.noexc:                                           ; preds = %bb2.i
  %_7.i = load i64, ptr %_5.i, align 8, !range !37, !noalias !203, !noundef !3
  %9 = trunc nuw i64 %_7.i to i1
  br i1 %9, label %bb7.i, label %bb6.i

bb7.i:                                            ; preds = %.noexc
  %a.i = load i64, ptr %7, align 8, !noalias !203, !noundef !3
  %b.i = load i64, ptr %8, align 8, !noalias !203, !noundef !3
  %i.i = load i64, ptr %iter, align 8, !alias.scope !203, !noundef !3
  %new_len.i = sub nuw i64 %a.i, %i.i
  %data.i = getelementptr inbounds nuw i8, ptr %_4.val.i, i64 %i.i
  store i64 %b.i, ptr %iter, align 8, !alias.scope !203
  br label %bb4

bb6.i:                                            ; preds = %.noexc
  %10 = load i8, ptr %_7.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !206, !noundef !3
  %_2.i.i = trunc nuw i8 %10 to i1
  br i1 %_2.i.i, label %bb23.thread60, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i
  store i8 1, ptr %_7.sroa.5.0.iter.sroa_idx, align 1, !alias.scope !206
  %11 = load i8, ptr %_7.sroa.4.0.iter.sroa_idx, align 8, !range !54, !alias.scope !206, !noundef !3
  %_3.i.i = trunc nuw i8 %11 to i1
  %i.pre.i.i = load i64, ptr %iter, align 8, !alias.scope !206
  %i1.pre.i.i = load i64, ptr %_7.sroa.2.0.iter.sroa_idx, align 8, !alias.scope !206
  %_4.not.i.i = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i
  br i1 %or.cond.not.i.i, label %bb4.i.i, label %bb23.thread60

bb4.i.i:                                          ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_7.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !206, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb4

bb23.thread60:                                    ; preds = %bb6.i, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !203
  br label %bb5

bb4:                                              ; preds = %bb4.i.i, %bb7.i
  %_0.sroa.4.0.i = phi i64 [ %new_len.i, %bb7.i ], [ %new_len.i.i, %bb4.i.i ]
  %_0.sroa.0.0.i = phi ptr [ %data.i, %bb7.i ], [ %data.i.i, %bb4.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !203
  call void @llvm.experimental.noalias.scope.decl(metadata !209)
  %_4.not.i.i29 = icmp samesign ult i64 %_0.sroa.4.0.i, 2
  br i1 %_4.not.i.i29, label %bb6, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb4
  %12 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_13ae1be13e6486d5c310a371cde21787, ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i, i64 range(i64 2, 16) 2), !alias.scope !212
  %13 = icmp eq i32 %12, 0
  br i1 %13, label %bb1.i, label %bb6

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i
  %_8.not.i.i.not = icmp eq i64 %_0.sroa.4.0.i, 2
  br i1 %_8.not.i.i.not, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb9.i.i

bb9.i.i:                                          ; preds = %bb1.i
  %14 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  %self1.i.i = load i8, ptr %14, align 1, !alias.scope !220, !noalias !209, !noundef !3
  %15 = icmp sgt i8 %self1.i.i, -65
  br i1 %15, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb7.i34.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i: ; preds = %bb9.i.i, %bb1.i
  %new_len.i.i32 = add i64 %_0.sroa.4.0.i, -2
  %data.i.i33 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  br label %bb6

bb7.i34.invoke:                                   ; preds = %bb9.i.i51, %bb9.i.i
  %16 = phi ptr [ %_0.sroa.0.0.i, %bb9.i.i ], [ %spec.select19, %bb9.i.i51 ]
  %17 = phi i64 [ %_0.sroa.4.0.i, %bb9.i.i ], [ %spec.select, %bb9.i.i51 ]
  %18 = phi i64 [ 2, %bb9.i.i ], [ 11, %bb9.i.i51 ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %16, i64 noundef %17, i64 noundef %18, i64 noundef %17, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9a6efcfeb17b646c43fe2783a9632a7) #29
          to label %bb7.i34.cont unwind label %cleanup2.loopexit.split-lp

bb7.i34.cont:                                     ; preds = %bb7.i34.invoke
  unreachable

bb5:                                              ; preds = %bb7, %bb23.thread60
  %cpu.sroa.4.078 = phi i64 [ %cpu.sroa.4.082, %bb23.thread60 ], [ %spec.select20, %bb7 ]
  %cpu.sroa.0.073 = phi ptr [ %cpu.sroa.0.083, %bb23.thread60 ], [ %spec.select21, %bb7 ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter)
  %.not15 = icmp eq ptr %cpu.sroa.0.073, null
  br i1 %.not15, label %bb25, label %bb26

bb26:                                             ; preds = %bb5
  %_23.i.i = icmp eq i64 %cpu.sroa.4.078, 0
  br i1 %_23.i.i, label %bb29, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb26
  %or.cond.not = icmp sgt i64 %cpu.sroa.4.078, 0
  br i1 %or.cond.not, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i, label %bb28, !prof !223

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i: ; preds = %bb6.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !224
; call __rustc::__rust_alloc
  %19 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %cpu.sroa.4.078, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !224
  %20 = icmp eq ptr %19, null
  br i1 %20, label %bb28, label %bb10.i

bb10.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %21 = ptrtoint ptr %19 to i64
  br label %bb29

bb25:                                             ; preds = %bb5
  store i64 -9223372036854775808, ptr %_0, align 8
  br label %bb24

bb24:                                             ; preds = %bb29, %bb25
  switch i64 %4, label %bb2.i.i.i4.i.i.i38 [
    i64 -9223372036854775808, label %bb10
    i64 0, label %bb10
  ]

bb2.i.i.i4.i.i.i38:                               ; preds = %bb24
  %22 = icmp ne ptr %_34, null
  call void @llvm.assume(i1 %22)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_34, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb10

bb28:                                             ; preds = %bb6.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %_52.sroa.4.0.ph = phi i64 [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i ], [ 0, %bb6.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_52.sroa.4.0.ph, i64 %cpu.sroa.4.078) #26
          to label %unreachable unwind label %cleanup2.loopexit.split-lp

bb29:                                             ; preds = %bb26, %bb10.i
  %_52.sroa.10.0 = phi i64 [ %21, %bb10.i ], [ 1, %bb26 ]
  %23 = inttoptr i64 %_52.sroa.10.0 to ptr
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %23, ptr nonnull align 1 %cpu.sroa.0.073, i64 %cpu.sroa.4.078, i1 false)
  store i64 %cpu.sroa.4.078, ptr %_0, align 8
  %_42.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %23, ptr %_42.sroa.4.0._0.sroa_idx, align 8
  %_42.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %cpu.sroa.4.078, ptr %_42.sroa.5.0._0.sroa_idx, align 8
  br label %bb24

bb10:                                             ; preds = %bb2.i.i.i4.i.i.i38, %bb24, %bb24
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rustflags1)
  %24 = icmp eq i64 %0, 0
  br i1 %24, label %bb12, label %bb2.i.i.i4.i.i.i40

bb2.i.i.i4.i.i.i40:                               ; preds = %bb10
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_19.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb12

unreachable:                                      ; preds = %bb28
  unreachable

bb6:                                              ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i, %bb4
  %_0.sroa.4.0.i30 = phi i64 [ %new_len.i.i32, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ undef, %bb4 ]
  %_0.sroa.0.0.i31 = phi ptr [ %data.i.i33, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ null, %bb4 ]
  %.not16 = icmp eq ptr %_0.sroa.0.0.i31, null
  %spec.select = select i1 %.not16, i64 %_0.sroa.4.0.i, i64 %_0.sroa.4.0.i30
  %spec.select19 = select i1 %.not16, ptr %_0.sroa.0.0.i, ptr %_0.sroa.0.0.i31
  call void @llvm.experimental.noalias.scope.decl(metadata !227)
  %_4.not.i.i42 = icmp samesign ult i64 %spec.select, 11
  br i1 %_4.not.i.i42, label %bb7, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43: ; preds = %bb6
  %25 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(11) @alloc_28b6e12cbb2608b0ebceb37ca6d5945a, ptr noundef nonnull readonly align 1 dereferenceable(11) %spec.select19, i64 range(i64 2, 16) 11), !alias.scope !230
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %bb1.i46, label %bb7

bb1.i46:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43
  %_8.not.i.i47.not = icmp eq i64 %spec.select, 11
  br i1 %_8.not.i.i47.not, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48, label %bb9.i.i51

bb9.i.i51:                                        ; preds = %bb1.i46
  %27 = getelementptr inbounds nuw i8, ptr %spec.select19, i64 11
  %self1.i.i52 = load i8, ptr %27, align 1, !alias.scope !238, !noalias !227, !noundef !3
  %28 = icmp sgt i8 %self1.i.i52, -65
  br i1 %28, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48, label %bb7.i34.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48: ; preds = %bb9.i.i51, %bb1.i46
  %new_len.i.i49 = add i64 %spec.select, -11
  %data.i.i50 = getelementptr inbounds nuw i8, ptr %spec.select19, i64 11
  br label %bb7

bb7:                                              ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43, %bb6
  %_0.sroa.4.0.i44 = phi i64 [ %new_len.i.i49, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48 ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43 ], [ undef, %bb6 ]
  %_0.sroa.0.0.i45 = phi ptr [ %data.i.i50, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i48 ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i43 ], [ null, %bb6 ]
  %.not18 = icmp eq ptr %_0.sroa.0.0.i45, null
  %spec.select20 = select i1 %.not18, i64 %cpu.sroa.4.082, i64 %_0.sroa.4.0.i44
  %spec.select21 = select i1 %.not18, ptr %cpu.sroa.0.083, ptr %_0.sroa.0.0.i45
  %29 = load i8, ptr %_7.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !241, !noundef !3
  %_2.i = trunc nuw i8 %29 to i1
  br i1 %_2.i, label %bb5, label %bb2.i

bb15:                                             ; preds = %bb2.i.i.i4.i.i.i, %bb14
  resume { ptr, i32 } %.pn
}

; build_script_build::strip_prefix
; Function Attrs: uwtable
define internal fastcc { ptr, i64 } @_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, ptr noalias noundef nonnull readonly align 1 captures(none) %pat.0, i64 noundef range(i64 2, 16) %pat.1) unnamed_addr #0 {
start:
  %_4.not.i = icmp samesign ult i64 %s.1, %pat.1
  br i1 %_4.not.i, label %bb3, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit: ; preds = %start
  %0 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(1) %pat.0, ptr noundef nonnull readonly align 1 dereferenceable(1) %s.0, i64 range(i64 2, 16) %pat.1), !alias.scope !243
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %bb1, label %bb3

bb1:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit
  %_8.not.i = icmp ult i64 %pat.1, %s.1
  br i1 %_8.not.i, label %bb9.i, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit

bb9.i:                                            ; preds = %bb1
  %2 = getelementptr inbounds nuw i8, ptr %s.0, i64 %pat.1
  %self1.i = load i8, ptr %2, align 1, !alias.scope !250, !noundef !3
  %3 = icmp sgt i8 %self1.i, -65
  br i1 %3, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit, label %bb7

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit: ; preds = %bb1, %bb9.i
  %new_len.i = sub nuw i64 %s.1, %pat.1
  %data.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %pat.1
  br label %bb3

bb3:                                              ; preds = %start, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit
  %_0.sroa.4.0 = phi i64 [ %new_len.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit ], [ undef, %start ]
  %_0.sroa.0.0 = phi ptr [ %data.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit ], [ null, %start ]
  %4 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0, 0
  %5 = insertvalue { ptr, i64 } %4, i64 %_0.sroa.4.0, 1
  ret { ptr, i64 } %5

bb7:                                              ; preds = %bb9.i
; call core::str::slice_error_fail
  tail call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %pat.1, i64 noundef %s.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9a6efcfeb17b646c43fe2783a9632a7) #29
  unreachable
}

; build_script_build::is_allowed_feature
; Function Attrs: uwtable
define internal fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature(ptr noalias noundef nonnull readonly align 1 captures(none) %0, i64 noundef range(i64 3, 22) %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5.i.i.i = alloca [24 x i8], align 8
  %_5.i = alloca [24 x i8], align 8
  %_19 = alloca [72 x i8], align 8
  %iter = alloca [72 x i8], align 8
  %_8 = alloca [24 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11)
  %2 = load i64, ptr %_2, align 8, !range !11, !noundef !3
  switch i64 %2, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit30
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %_2.val25 = load ptr, ptr %3, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val25, i64 noundef %2, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit: ; preds = %start, %bb2.i.i.i4.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  br label %bb18

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit30: ; preds = %start
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
  %4 = load i64, ptr %_3, align 8, !range !11, !noundef !3
  %.not9 = icmp eq i64 %4, -9223372036854775808
  br i1 %.not9, label %bb22, label %bb7

bb18:                                             ; preds = %bb22, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit
  %allowed.sroa.0.0.off0 = phi i1 [ false, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit ], [ %allowed.sroa.0.1.off0, %bb22 ]
  ret i1 %allowed.sroa.0.0.off0

bb7:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit30
  %rustflags.sroa.5.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %rustflags.sroa.5.0.copyload = load ptr, ptr %rustflags.sroa.5.0._3.sroa_idx, align 8, !nonnull !3, !noundef !3
  %rustflags.sroa.8.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %rustflags.sroa.8.0.copyload = load i64, ptr %rustflags.sroa.8.0._3.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_8)
; invoke <alloc::string::String>::from_utf8_lossy
  invoke void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_8, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.8.0.copyload)
          to label %bb2.i.lr.ph unwind label %cleanup

bb22:                                             ; preds = %bb2.i.i.i4.i.i.i42, %bb16, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit30
  %allowed.sroa.0.1.off0 = phi i1 [ true, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit30 ], [ %allowed.sroa.0.2.off077, %bb16 ], [ %allowed.sroa.0.2.off077, %bb2.i.i.i4.i.i.i42 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  br label %bb18

bb20:                                             ; preds = %bb2.i.i.i4.i.i.i31, %cleanup1, %cleanup1, %cleanup
  %.pn = phi { ptr, i32 } [ %6, %cleanup ], [ %lpad.phi, %cleanup1 ], [ %lpad.phi, %cleanup1 ], [ %lpad.phi, %bb2.i.i.i4.i.i.i31 ]
  %5 = icmp eq i64 %4, 0
  br i1 %5, label %bb21, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb20
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb21

cleanup:                                          ; preds = %bb7
  %6 = landingpad { ptr, i32 }
          cleanup
  br label %bb20

cleanup1.loopexit:                                ; preds = %bb2.i.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup1

cleanup1.loopexit.split-lp.loopexit:              ; preds = %bb2.i
  %lpad.loopexit69 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup1

cleanup1.loopexit.split-lp.loopexit.split-lp:     ; preds = %bb7.i55.invoke
  %lpad.loopexit.split-lp70 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup1

cleanup1:                                         ; preds = %cleanup1.loopexit.split-lp.loopexit, %cleanup1.loopexit.split-lp.loopexit.split-lp, %cleanup1.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup1.loopexit ], [ %lpad.loopexit69, %cleanup1.loopexit.split-lp.loopexit ], [ %lpad.loopexit.split-lp70, %cleanup1.loopexit.split-lp.loopexit.split-lp ]
  switch i64 %8, label %bb2.i.i.i4.i.i.i31 [
    i64 -9223372036854775808, label %bb20
    i64 0, label %bb20
  ]

bb2.i.i.i4.i.i.i31:                               ; preds = %cleanup1
  %7 = icmp ne ptr %_37, null
  call void @llvm.assume(i1 %7)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_37, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb20

bb2.i.lr.ph:                                      ; preds = %bb7
  %8 = load i64, ptr %_8, align 8, !range !11, !noundef !3
  %9 = getelementptr inbounds nuw i8, ptr %_8, i64 8
  %_37 = load ptr, ptr %9, align 8
  %10 = getelementptr inbounds nuw i8, ptr %_8, i64 16
  %_36 = load i64, ptr %10, align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter)
  store i64 0, ptr %iter, align 8
  %_6.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %_36, ptr %_6.sroa.2.0.iter.sroa_idx, align 8
  %_6.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %_37, ptr %_6.sroa.3.0.iter.sroa_idx, align 8
  %_6.sroa.3.sroa.2.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store i64 %_36, ptr %_6.sroa.3.sroa.2.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.3.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 0, ptr %_6.sroa.3.sroa.3.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.4.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  store i64 %_36, ptr %_6.sroa.3.sroa.4.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.5.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store <2 x i32> splat (i32 31), ptr %_6.sroa.3.sroa.5.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.7.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 56
  store i8 1, ptr %_6.sroa.3.sroa.7.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i8 1, ptr %_6.sroa.4.0.iter.sroa_idx, align 8
  %_6.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 65
  store i8 0, ptr %_6.sroa.5.0.iter.sroa_idx, align 1
  %11 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %12 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  %_44.sroa.4.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 8
  %_44.sroa.5.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 16
  %_44.sroa.5.sroa.4.0._44.sroa.5.0._19.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 24
  %_44.sroa.5.sroa.5.0._44.sroa.5.0._19.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 32
  %_44.sroa.5.sroa.6.0._44.sroa.5.0._19.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 40
  %_44.sroa.5.sroa.7.0._44.sroa.5.0._19.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 48
  %_44.sroa.5.sroa.9.0._44.sroa.5.0._19.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 56
  %_44.sroa.6.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 64
  %_44.sroa.7.0._19.sroa_idx = getelementptr inbounds nuw i8, ptr %_19, i64 65
  %13 = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 8
  %14 = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 16
  br label %bb2.i

bb2.i:                                            ; preds = %bb2.i.lr.ph, %bb15
  %allowed.sroa.0.2.off085 = phi i1 [ true, %bb2.i.lr.ph ], [ %allowed.sroa.0.3.off0, %bb15 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !253)
  %_4.val.i = load ptr, ptr %_6.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !253, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !253
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef align 8 dereferenceable(48) %_6.sroa.3.0.iter.sroa_idx) #28
          to label %.noexc unwind label %cleanup1.loopexit.split-lp.loopexit

.noexc:                                           ; preds = %bb2.i
  %_7.i = load i64, ptr %_5.i, align 8, !range !37, !noalias !253, !noundef !3
  %15 = trunc nuw i64 %_7.i to i1
  br i1 %15, label %bb7.i, label %bb6.i

bb7.i:                                            ; preds = %.noexc
  %a.i = load i64, ptr %11, align 8, !noalias !253, !noundef !3
  %b.i = load i64, ptr %12, align 8, !noalias !253, !noundef !3
  %i.i = load i64, ptr %iter, align 8, !alias.scope !253, !noundef !3
  %new_len.i = sub nuw i64 %a.i, %i.i
  %data.i = getelementptr inbounds nuw i8, ptr %_4.val.i, i64 %i.i
  store i64 %b.i, ptr %iter, align 8, !alias.scope !253
  br label %bb10

bb6.i:                                            ; preds = %.noexc
  %16 = load i8, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !256, !noundef !3
  %_2.i.i = trunc nuw i8 %16 to i1
  br i1 %_2.i.i, label %bb28.thread62, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i
  store i8 1, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !alias.scope !256
  %17 = load i8, ptr %_6.sroa.4.0.iter.sroa_idx, align 8, !range !54, !alias.scope !256, !noundef !3
  %_3.i.i = trunc nuw i8 %17 to i1
  %i.pre.i.i = load i64, ptr %iter, align 8, !alias.scope !256
  %i1.pre.i.i = load i64, ptr %_6.sroa.2.0.iter.sroa_idx, align 8, !alias.scope !256
  %_4.not.i.i = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i
  br i1 %or.cond.not.i.i, label %bb4.i.i, label %bb28.thread62

bb4.i.i:                                          ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_6.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !256, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb10

bb28.thread62:                                    ; preds = %bb6.i, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !253
  br label %bb11

bb10:                                             ; preds = %bb4.i.i, %bb7.i
  %_0.sroa.4.0.i = phi i64 [ %new_len.i, %bb7.i ], [ %new_len.i.i, %bb4.i.i ]
  %_0.sroa.0.0.i = phi ptr [ %data.i, %bb7.i ], [ %data.i.i, %bb4.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !253
  call void @llvm.experimental.noalias.scope.decl(metadata !259)
  %_4.not.i.i32 = icmp samesign ult i64 %_0.sroa.4.0.i, 2
  br i1 %_4.not.i.i32, label %bb12, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb10
  %18 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_9bc7f23b9f8ca5426d0539d95c9ebfc5, ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i, i64 range(i64 2, 16) 2), !alias.scope !262
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %bb1.i, label %bb12

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i
  %_8.not.i.i.not = icmp eq i64 %_0.sroa.4.0.i, 2
  br i1 %_8.not.i.i.not, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb9.i.i

bb9.i.i:                                          ; preds = %bb1.i
  %20 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  %self1.i.i = load i8, ptr %20, align 1, !alias.scope !270, !noalias !259, !noundef !3
  %21 = icmp sgt i8 %self1.i.i, -65
  br i1 %21, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb7.i55.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i: ; preds = %bb9.i.i, %bb1.i
  %new_len.i.i35 = add i64 %_0.sroa.4.0.i, -2
  %data.i.i36 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  br label %bb12

bb11:                                             ; preds = %bb15, %bb28.thread62
  %allowed.sroa.0.2.off077 = phi i1 [ %allowed.sroa.0.2.off085, %bb28.thread62 ], [ %allowed.sroa.0.3.off0, %bb15 ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter)
  switch i64 %8, label %bb2.i.i.i4.i.i.i40 [
    i64 -9223372036854775808, label %bb16
    i64 0, label %bb16
  ]

bb2.i.i.i4.i.i.i40:                               ; preds = %bb11
  %22 = icmp ne ptr %_37, null
  call void @llvm.assume(i1 %22)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_37, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb16

bb16:                                             ; preds = %bb2.i.i.i4.i.i.i40, %bb11, %bb11
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_8)
  %23 = icmp eq i64 %4, 0
  br i1 %23, label %bb22, label %bb2.i.i.i4.i.i.i42

bb2.i.i.i4.i.i.i42:                               ; preds = %bb16
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb22

bb12:                                             ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i, %bb10
  %_0.sroa.4.0.i33 = phi i64 [ %new_len.i.i35, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ undef, %bb10 ]
  %_0.sroa.0.0.i34 = phi ptr [ %data.i.i36, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ null, %bb10 ]
  %.not16 = icmp eq ptr %_0.sroa.0.0.i34, null
  %spec.select = select i1 %.not16, i64 %_0.sroa.4.0.i, i64 %_0.sroa.4.0.i33
  %spec.select19 = select i1 %.not16, ptr %_0.sroa.0.0.i, ptr %_0.sroa.0.0.i34
  call void @llvm.experimental.noalias.scope.decl(metadata !273)
  %_4.not.i.i44 = icmp samesign ult i64 %spec.select, 15
  br i1 %_4.not.i.i44, label %bb15, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i45

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i45: ; preds = %bb12
  %24 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(15) @alloc_3f5a3f432cc037101cfa0283db269c57, ptr noundef nonnull readonly align 1 dereferenceable(15) %spec.select19, i64 range(i64 2, 16) 15), !alias.scope !276
  %25 = icmp eq i32 %24, 0
  br i1 %25, label %bb1.i48, label %bb15

bb1.i48:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i45
  %_8.not.i.i49.not = icmp eq i64 %spec.select, 15
  br i1 %_8.not.i.i49.not, label %bb2.i.i.lr.ph.i, label %bb9.i.i53

bb9.i.i53:                                        ; preds = %bb1.i48
  %26 = getelementptr inbounds nuw i8, ptr %spec.select19, i64 15
  %self1.i.i54 = load i8, ptr %26, align 1, !alias.scope !284, !noalias !273, !noundef !3
  %27 = icmp sgt i8 %self1.i.i54, -65
  br i1 %27, label %bb2.i.i.lr.ph.i, label %bb7.i55.invoke

bb7.i55.invoke:                                   ; preds = %bb9.i.i53, %bb9.i.i
  %28 = phi ptr [ %_0.sroa.0.0.i, %bb9.i.i ], [ %spec.select19, %bb9.i.i53 ]
  %29 = phi i64 [ %_0.sroa.4.0.i, %bb9.i.i ], [ %spec.select, %bb9.i.i53 ]
  %30 = phi i64 [ 2, %bb9.i.i ], [ 15, %bb9.i.i53 ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %28, i64 noundef %29, i64 noundef %30, i64 noundef %29, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9a6efcfeb17b646c43fe2783a9632a7) #29
          to label %bb7.i55.cont unwind label %cleanup1.loopexit.split-lp.loopexit.split-lp

bb7.i55.cont:                                     ; preds = %bb7.i55.invoke
  unreachable

bb15:                                             ; preds = %bb12, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i45, %bb34
  %allowed.sroa.0.3.off0 = phi i1 [ %_0.sroa.0.0.off0.i, %bb34 ], [ %allowed.sroa.0.2.off085, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i45 ], [ %allowed.sroa.0.2.off085, %bb12 ]
  %31 = load i8, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !287, !noundef !3
  %_2.i = trunc nuw i8 %31 to i1
  br i1 %_2.i, label %bb11, label %bb2.i

bb2.i.i.lr.ph.i:                                  ; preds = %bb9.i.i53, %bb1.i48
  %new_len.i.i51 = add i64 %spec.select, -15
  %data.i.i52 = getelementptr inbounds nuw i8, ptr %spec.select19, i64 15
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_19)
  store i64 0, ptr %_19, align 8
  store i64 %new_len.i.i51, ptr %_44.sroa.4.0._19.sroa_idx, align 8
  store ptr %data.i.i52, ptr %_44.sroa.5.0._19.sroa_idx, align 8
  store i64 %new_len.i.i51, ptr %_44.sroa.5.sroa.4.0._44.sroa.5.0._19.sroa_idx.sroa_idx, align 8
  store i64 0, ptr %_44.sroa.5.sroa.5.0._44.sroa.5.0._19.sroa_idx.sroa_idx, align 8
  store i64 %new_len.i.i51, ptr %_44.sroa.5.sroa.6.0._44.sroa.5.0._19.sroa_idx.sroa_idx, align 8
  store <2 x i32> splat (i32 44), ptr %_44.sroa.5.sroa.7.0._44.sroa.5.0._19.sroa_idx.sroa_idx, align 8
  store i8 1, ptr %_44.sroa.5.sroa.9.0._44.sroa.5.0._19.sroa_idx.sroa_idx, align 8
  store i8 1, ptr %_44.sroa.6.0._19.sroa_idx, align 8
  store i8 0, ptr %_44.sroa.7.0._19.sroa_idx, align 1
  call void @llvm.experimental.noalias.scope.decl(metadata !289)
  br label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb1.backedge.i, %bb2.i.i.lr.ph.i
  call void @llvm.experimental.noalias.scope.decl(metadata !292)
  call void @llvm.experimental.noalias.scope.decl(metadata !295)
  %_4.val.i.i.i = load ptr, ptr %_44.sroa.5.0._19.sroa_idx, align 8, !alias.scope !298, !noalias !299, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !301
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_44.sroa.5.0._19.sroa_idx) #28
          to label %.noexc58 unwind label %cleanup1.loopexit

.noexc58:                                         ; preds = %bb2.i.i.i
  %_7.i.i.i = load i64, ptr %_5.i.i.i, align 8, !range !37, !noalias !301, !noundef !3
  %32 = trunc nuw i64 %_7.i.i.i to i1
  br i1 %32, label %bb7.i.i.i, label %bb6.i.i.i

bb7.i.i.i:                                        ; preds = %.noexc58
  %a.i.i.i = load i64, ptr %13, align 8, !noalias !301, !noundef !3
  %b.i.i.i = load i64, ptr %14, align 8, !noalias !301, !noundef !3
  %i.i.i.i = load i64, ptr %_19, align 8, !alias.scope !298, !noalias !299, !noundef !3
  %new_len.i.i.i = sub nuw i64 %a.i.i.i, %i.i.i.i
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i, i64 %i.i.i.i
  store i64 %b.i.i.i, ptr %_19, align 8, !alias.scope !298, !noalias !299
  br label %bb3.i

bb6.i.i.i:                                        ; preds = %.noexc58
  %33 = load i8, ptr %_44.sroa.7.0._19.sroa_idx, align 1, !range !54, !alias.scope !302, !noalias !299, !noundef !3
  %_2.i.i.i.i = trunc nuw i8 %33 to i1
  br i1 %_2.i.i.i.i, label %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread5.i, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb6.i.i.i
  store i8 1, ptr %_44.sroa.7.0._19.sroa_idx, align 1, !alias.scope !302, !noalias !299
  %34 = load i8, ptr %_44.sroa.6.0._19.sroa_idx, align 8, !range !54, !alias.scope !302, !noalias !299, !noundef !3
  %_3.i.i.i.i = trunc nuw i8 %34 to i1
  %i.pre.i.i.i.i = load i64, ptr %_19, align 8, !alias.scope !302, !noalias !299
  %i1.pre.i.i.i.i = load i64, ptr %_44.sroa.4.0._19.sroa_idx, align 8, !alias.scope !302, !noalias !299
  %_4.not.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  %or.cond.not.i.i.i.i = select i1 %_3.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i
  br i1 %or.cond.not.i.i.i.i, label %bb4.i.i.i.i, label %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread5.i

bb4.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  %_10.val.i.i.i.i = load ptr, ptr %_44.sroa.5.0._19.sroa_idx, align 8, !alias.scope !302, !noalias !299, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i, i64 %i.pre.i.i.i.i
  br label %bb3.i

_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread5.i: ; preds = %bb1.i.i.i.i, %bb6.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !301
  br label %bb34

bb3.i:                                            ; preds = %bb4.i.i.i.i, %bb7.i.i.i
  %_0.sroa.4.0.i.i.i = phi i64 [ %new_len.i.i.i, %bb7.i.i.i ], [ %new_len.i.i.i.i, %bb4.i.i.i.i ]
  %_0.sroa.0.0.i.i.i = phi ptr [ %data.i.i.i, %bb7.i.i.i ], [ %data.i.i.i.i, %bb4.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !301
  %_3.not.i.i.i.i = icmp eq i64 %_0.sroa.4.0.i.i.i, %1
  br i1 %_3.not.i.i.i.i, label %bb2.i.i.i.i, label %bb1.backedge.i

bb2.i.i.i.i:                                      ; preds = %bb3.i
  %35 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(1) %_0.sroa.0.0.i.i.i, ptr noundef nonnull readonly align 1 dereferenceable(1) %0, i64 range(i64 0, -9223372036854775808) %1), !alias.scope !305, !noalias !309
  %36 = icmp eq i32 %35, 0
  br i1 %36, label %bb34, label %bb1.backedge.i

bb1.backedge.i:                                   ; preds = %bb2.i.i.i.i, %bb3.i
  %37 = load i8, ptr %_44.sroa.7.0._19.sroa_idx, align 1, !range !54, !alias.scope !310, !noalias !299, !noundef !3
  %_2.i.i.i = trunc nuw i8 %37 to i1
  br i1 %_2.i.i.i, label %bb34, label %bb2.i.i.i

bb34:                                             ; preds = %bb2.i.i.i.i, %bb1.backedge.i, %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread5.i
  %_0.sroa.0.0.off0.i = phi i1 [ false, %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread5.i ], [ false, %bb1.backedge.i ], [ true, %bb2.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_19)
  br label %bb15

bb21:                                             ; preds = %bb2.i.i.i4.i.i.i, %bb20
  resume { ptr, i32 } %.pn
}

; build_script_build::target_feature_fallback
; Function Attrs: uwtable
define internal fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef range(i64 2, 27) %1, i1 noundef zeroext %2) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5.i = alloca [24 x i8], align 8
  %args = alloca [16 x i8], align 8
  %iter = alloca [72 x i8], align 8
  %_8 = alloca [24 x i8], align 8
  %_3 = alloca [24 x i8], align 8
  %name = alloca [16 x i8], align 8
  store ptr %0, ptr %name, align 8
  %3 = getelementptr inbounds nuw i8, ptr %name, i64 8
  store i64 %1, ptr %3, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
  %4 = load i64, ptr %_3, align 8, !range !11, !noundef !3
  %.not = icmp eq i64 %4, -9223372036854775808
  br i1 %.not, label %bb28, label %bb2

bb2:                                              ; preds = %start
  %rustflags.sroa.5.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %rustflags.sroa.5.0.copyload = load ptr, ptr %rustflags.sroa.5.0._3.sroa_idx, align 8, !nonnull !3, !noundef !3
  %rustflags.sroa.8.0._3.sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %rustflags.sroa.8.0.copyload = load i64, ptr %rustflags.sroa.8.0._3.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_8)
; invoke <alloc::string::String>::from_utf8_lossy
  invoke void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_8, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.8.0.copyload)
          to label %bb2.i.lr.ph unwind label %cleanup

bb28:                                             ; preds = %start
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  br i1 %2, label %bb23, label %bb25

bb27:                                             ; preds = %bb2.i.i.i4.i.i.i47, %cleanup2, %cleanup2, %cleanup
  %.pn = phi { ptr, i32 } [ %6, %cleanup ], [ %lpad.phi, %cleanup2 ], [ %lpad.phi, %cleanup2 ], [ %lpad.phi, %bb2.i.i.i4.i.i.i47 ]
  %5 = icmp eq i64 %4, 0
  br i1 %5, label %bb29, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb27
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb29

cleanup:                                          ; preds = %bb2
  %6 = landingpad { ptr, i32 }
          cleanup
  br label %bb27

cleanup2.loopexit:                                ; preds = %bb2.i.us.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2

cleanup2.loopexit.split-lp.loopexit:              ; preds = %bb2.i
  %lpad.loopexit160 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2

cleanup2.loopexit.split-lp.loopexit.split-lp:     ; preds = %bb7.i71.invoke
  %lpad.loopexit.split-lp161 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2

cleanup2:                                         ; preds = %cleanup2.loopexit.split-lp.loopexit, %cleanup2.loopexit.split-lp.loopexit.split-lp, %cleanup2.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup2.loopexit ], [ %lpad.loopexit160, %cleanup2.loopexit.split-lp.loopexit ], [ %lpad.loopexit.split-lp161, %cleanup2.loopexit.split-lp.loopexit.split-lp ]
  switch i64 %8, label %bb2.i.i.i4.i.i.i47 [
    i64 -9223372036854775808, label %bb27
    i64 0, label %bb27
  ]

bb2.i.i.i4.i.i.i47:                               ; preds = %cleanup2
  %7 = icmp ne ptr %_56, null
  call void @llvm.assume(i1 %7)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_56, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb27

bb2.i.lr.ph:                                      ; preds = %bb2
  %8 = load i64, ptr %_8, align 8, !range !11, !noundef !3
  %9 = getelementptr inbounds nuw i8, ptr %_8, i64 8
  %_56 = load ptr, ptr %9, align 8
  %10 = getelementptr inbounds nuw i8, ptr %_8, i64 16
  %_55 = load i64, ptr %10, align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter)
  store i64 0, ptr %iter, align 8
  %_6.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %_55, ptr %_6.sroa.2.0.iter.sroa_idx, align 8
  %_6.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %_56, ptr %_6.sroa.3.0.iter.sroa_idx, align 8
  %_6.sroa.3.sroa.2.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store i64 %_55, ptr %_6.sroa.3.sroa.2.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.3.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 0, ptr %_6.sroa.3.sroa.3.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.4.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  store i64 %_55, ptr %_6.sroa.3.sroa.4.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.5.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store <2 x i32> splat (i32 31), ptr %_6.sroa.3.sroa.5.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.3.sroa.7.0._6.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 56
  store i8 1, ptr %_6.sroa.3.sroa.7.0._6.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_6.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i8 1, ptr %_6.sroa.4.0.iter.sroa_idx, align 8
  %_6.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 65
  store i8 0, ptr %_6.sroa.5.0.iter.sroa_idx, align 1
  %11 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %12 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  br label %bb2.i

bb2.i:                                            ; preds = %bb2.i.lr.ph, %bb20
  %has_target_feature.sroa.0.1.off0180 = phi i1 [ %2, %bb2.i.lr.ph ], [ %has_target_feature.sroa.0.2.off0, %bb20 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !313)
  %_4.val.i = load ptr, ptr %_6.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !313, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !313
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef align 8 dereferenceable(48) %_6.sroa.3.0.iter.sroa_idx) #28
          to label %.noexc unwind label %cleanup2.loopexit.split-lp.loopexit

.noexc:                                           ; preds = %bb2.i
  %_7.i = load i64, ptr %_5.i, align 8, !range !37, !noalias !313, !noundef !3
  %13 = trunc nuw i64 %_7.i to i1
  br i1 %13, label %bb7.i, label %bb6.i

bb7.i:                                            ; preds = %.noexc
  %a.i = load i64, ptr %11, align 8, !noalias !313, !noundef !3
  %b.i = load i64, ptr %12, align 8, !noalias !313, !noundef !3
  %i.i = load i64, ptr %iter, align 8, !alias.scope !313, !noundef !3
  %new_len.i = sub nuw i64 %a.i, %i.i
  %data.i = getelementptr inbounds nuw i8, ptr %_4.val.i, i64 %i.i
  store i64 %b.i, ptr %iter, align 8, !alias.scope !313
  br label %bb5

bb6.i:                                            ; preds = %.noexc
  %14 = load i8, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !316, !noundef !3
  %_2.i.i = trunc nuw i8 %14 to i1
  br i1 %_2.i.i, label %bb35.thread127, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i
  store i8 1, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !alias.scope !316
  %15 = load i8, ptr %_6.sroa.4.0.iter.sroa_idx, align 8, !range !54, !alias.scope !316, !noundef !3
  %_3.i.i = trunc nuw i8 %15 to i1
  %i.pre.i.i = load i64, ptr %iter, align 8, !alias.scope !316
  %i1.pre.i.i = load i64, ptr %_6.sroa.2.0.iter.sroa_idx, align 8, !alias.scope !316
  %_4.not.i.i = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i
  br i1 %or.cond.not.i.i, label %bb4.i.i, label %bb35.thread127

bb4.i.i:                                          ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_6.sroa.3.0.iter.sroa_idx, align 8, !alias.scope !316, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb5

bb35.thread127:                                   ; preds = %bb6.i, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !313
  br label %bb6

bb5:                                              ; preds = %bb4.i.i, %bb7.i
  %_0.sroa.4.0.i = phi i64 [ %new_len.i, %bb7.i ], [ %new_len.i.i, %bb4.i.i ]
  %_0.sroa.0.0.i = phi ptr [ %data.i, %bb7.i ], [ %data.i.i, %bb4.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !313
  call void @llvm.experimental.noalias.scope.decl(metadata !319)
  %_4.not.i.i48 = icmp samesign ult i64 %_0.sroa.4.0.i, 2
  br i1 %_4.not.i.i48, label %bb7, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb5
  %16 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_13ae1be13e6486d5c310a371cde21787, ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i, i64 range(i64 2, 16) 2), !alias.scope !322
  %17 = icmp eq i32 %16, 0
  br i1 %17, label %bb1.i, label %bb7

bb1.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i
  %_8.not.i.i.not = icmp eq i64 %_0.sroa.4.0.i, 2
  br i1 %_8.not.i.i.not, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb9.i.i

bb9.i.i:                                          ; preds = %bb1.i
  %18 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  %self1.i.i = load i8, ptr %18, align 1, !alias.scope !330, !noalias !319, !noundef !3
  %19 = icmp sgt i8 %self1.i.i, -65
  br i1 %19, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, label %bb7.i71.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i: ; preds = %bb9.i.i, %bb1.i
  %new_len.i.i51 = add i64 %_0.sroa.4.0.i, -2
  %data.i.i52 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i, i64 2
  br label %bb7

bb6:                                              ; preds = %bb20, %bb35.thread127
  %has_target_feature.sroa.0.1.off0170 = phi i1 [ %has_target_feature.sroa.0.1.off0180, %bb35.thread127 ], [ %has_target_feature.sroa.0.2.off0, %bb20 ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter)
  switch i64 %8, label %bb2.i.i.i4.i.i.i56 [
    i64 -9223372036854775808, label %bb21
    i64 0, label %bb21
  ]

bb2.i.i.i4.i.i.i56:                               ; preds = %bb6
  %20 = icmp ne ptr %_56, null
  call void @llvm.assume(i1 %20)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_56, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb21

bb21:                                             ; preds = %bb2.i.i.i4.i.i.i56, %bb6, %bb6
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_8)
  %21 = icmp eq i64 %4, 0
  br i1 %21, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59, label %bb2.i.i.i4.i.i.i58

bb2.i.i.i4.i.i.i58:                               ; preds = %bb21
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59: ; preds = %bb21, %bb2.i.i.i4.i.i.i58
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3)
  br i1 %has_target_feature.sroa.0.1.off0170, label %bb23, label %bb25

bb7:                                              ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i, %bb5
  %_0.sroa.4.0.i49 = phi i64 [ %new_len.i.i51, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ undef, %bb5 ]
  %_0.sroa.0.0.i50 = phi ptr [ %data.i.i52, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i ], [ null, %bb5 ]
  %.not33 = icmp eq ptr %_0.sroa.0.0.i50, null
  %spec.select = select i1 %.not33, i64 %_0.sroa.4.0.i, i64 %_0.sroa.4.0.i49
  %spec.select.fr = freeze i64 %spec.select
  %spec.select38 = select i1 %.not33, ptr %_0.sroa.0.0.i, ptr %_0.sroa.0.0.i50
  call void @llvm.experimental.noalias.scope.decl(metadata !333)
  %_4.not.i.i60 = icmp samesign ult i64 %spec.select.fr, 15
  br i1 %_4.not.i.i60, label %bb20, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i61

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i61: ; preds = %bb7
  %22 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(15) @alloc_b6e2898a10ad3df62e798ba959034459, ptr noundef nonnull readonly align 1 dereferenceable(15) %spec.select38, i64 range(i64 2, 16) 15), !alias.scope !336
  %23 = icmp eq i32 %22, 0
  br i1 %23, label %bb1.i64, label %bb20

bb1.i64:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i61
  %_8.not.i.i65.not = icmp eq i64 %spec.select.fr, 15
  br i1 %_8.not.i.i65.not, label %bb39, label %bb9.i.i69

bb9.i.i69:                                        ; preds = %bb1.i64
  %24 = getelementptr inbounds nuw i8, ptr %spec.select38, i64 15
  %self1.i.i70 = load i8, ptr %24, align 1, !alias.scope !344, !noalias !333, !noundef !3
  %25 = icmp sgt i8 %self1.i.i70, -65
  br i1 %25, label %bb39, label %bb7.i71.invoke

bb7.i71.invoke:                                   ; preds = %bb9.i.i69, %bb9.i.i
  %26 = phi ptr [ %_0.sroa.0.0.i, %bb9.i.i ], [ %spec.select38, %bb9.i.i69 ]
  %27 = phi i64 [ %_0.sroa.4.0.i, %bb9.i.i ], [ %spec.select.fr, %bb9.i.i69 ]
  %28 = phi i64 [ 2, %bb9.i.i ], [ 15, %bb9.i.i69 ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %26, i64 noundef %27, i64 noundef %28, i64 noundef %27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9a6efcfeb17b646c43fe2783a9632a7) #29
          to label %bb7.i71.cont unwind label %cleanup2.loopexit.split-lp.loopexit.split-lp

bb7.i71.cont:                                     ; preds = %bb7.i71.invoke
  unreachable

bb20:                                             ; preds = %bb19, %bb7, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i61
  %has_target_feature.sroa.0.2.off0 = phi i1 [ %has_target_feature.sroa.0.1.off0180, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i61 ], [ %has_target_feature.sroa.0.1.off0180, %bb7 ], [ %has_target_feature.sroa.0.4.off0, %bb19 ]
  %29 = load i8, ptr %_6.sroa.5.0.iter.sroa_idx, align 1, !range !54, !alias.scope !347, !noundef !3
  %_2.i = trunc nuw i8 %29 to i1
  br i1 %_2.i, label %bb6, label %bb2.i

bb39:                                             ; preds = %bb1.i64, %bb9.i.i69
  %new_len.i.i67 = add i64 %spec.select.fr, -15
  %data.i.i68 = getelementptr inbounds nuw i8, ptr %spec.select38, i64 15
  br label %bb2.i76

bb2.i76:                                          ; preds = %bb39, %bb19
  %has_target_feature.sroa.0.3.off0178 = phi i1 [ %has_target_feature.sroa.0.1.off0180, %bb39 ], [ %has_target_feature.sroa.0.4.off0, %bb19 ]
  %30 = phi i64 [ 0, %bb39 ], [ %.ph144, %bb19 ]
  %_18.sroa.3.sroa.3.0.copyload121177 = phi i64 [ 0, %bb39 ], [ %_18.sroa.3.sroa.3.0.copyload120.ph, %bb19 ]
  %_4325.i = icmp ult i64 %new_len.i.i67, %_18.sroa.3.sroa.3.0.copyload121177
  br i1 %_4325.i, label %bb11, label %bb12.us.i

bb12.us.i:                                        ; preds = %bb2.i76, %bb9.us.i
  %31 = phi i64 [ %40, %bb9.us.i ], [ %_18.sroa.3.sroa.3.0.copyload121177, %bb2.i76 ]
  %new_len.us.i = sub nuw i64 %new_len.i.i67, %31
  %_46.us.i = getelementptr inbounds nuw i8, ptr %data.i.i68, i64 %31
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; invoke core::slice::memchr::memchr_aligned
  %32 = invoke { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef 44, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i)
          to label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i unwind label %cleanup2.loopexit

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %34, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %33 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %33, align 1, !alias.scope !349, !noalias !352, !noundef !3
  %_8.i.us.i = icmp eq i8 %_9.i.us.i, 44
  br i1 %_8.i.us.i, label %bb4.i.us.i, label %bb9.i.us.i

bb9.i.us.i:                                       ; preds = %bb7.i.us.i
  %34 = add nuw nsw i64 %i.sroa.0.05.i.us.i, 1
  %exitcond.not.i.us.i = icmp eq i64 %34, %new_len.us.i
  br i1 %exitcond.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb4.i.us.i:                                       ; preds = %bb9.i.us.i, %bb7.i.us.i, %bb5.preheader.i.us.i
  %i.sroa.0.0.lcssa.i.us.i = phi i64 [ 0, %bb5.preheader.i.us.i ], [ %i.sroa.0.05.i.us.i, %bb7.i.us.i ], [ %new_len.us.i, %bb9.i.us.i ]
  %_0.sroa.0.1.i.us.i = phi i64 [ 0, %bb5.preheader.i.us.i ], [ 1, %bb7.i.us.i ], [ 0, %bb9.i.us.i ]
  %35 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us.i, 0
  %36 = insertvalue { i64, i64 } %35, i64 %i.sroa.0.0.lcssa.i.us.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i: ; preds = %bb2.i.us.i, %bb4.i.us.i
  %.merged.i.us.i = phi { i64, i64 } [ %36, %bb4.i.us.i ], [ %32, %bb2.i.us.i ]
  %37 = extractvalue { i64, i64 } %.merged.i.us.i, 0
  %38 = trunc nuw i64 %37 to i1
  br i1 %38, label %bb4.us.i, label %bb11

bb4.us.i:                                         ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  %39 = extractvalue { i64, i64 } %.merged.i.us.i, 1
  %_16.us.i = add i64 %31, 1
  %40 = add i64 %_16.us.i, %39
  %41 = add i64 %39, %31
  %or.cond.i.not = icmp ult i64 %41, %new_len.i.i67
  br i1 %or.cond.i.not, label %bb19.us.i, label %bb9.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %data.i.i68, i64 %41
  %lhsc = load i8, ptr %_62.us.i, align 1
  %42 = icmp eq i8 %lhsc, 44
  br i1 %42, label %bb11, label %bb9.us.i

bb9.us.i:                                         ; preds = %bb19.us.i, %bb4.us.i
  %_43.us.i = icmp ult i64 %new_len.i.i67, %40
  br i1 %_43.us.i, label %bb11, label %bb12.us.i

bb11:                                             ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i, %bb9.us.i, %bb19.us.i, %bb2.i76
  %_18.sroa.3.sroa.3.0.copyload120.ph = phi i64 [ %_18.sroa.3.sroa.3.0.copyload121177, %bb2.i76 ], [ %40, %bb9.us.i ], [ %new_len.i.i67, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i ], [ %40, %bb19.us.i ]
  %.ph.off0 = phi i1 [ true, %bb2.i76 ], [ true, %bb9.us.i ], [ true, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i ], [ false, %bb19.us.i ]
  %.ph144 = phi i64 [ %30, %bb2.i76 ], [ %30, %bb9.us.i ], [ %30, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i ], [ %40, %bb19.us.i ]
  %new_len.i.i67.pn = phi i64 [ %new_len.i.i67, %bb2.i76 ], [ %new_len.i.i67, %bb9.us.i ], [ %new_len.i.i67, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i ], [ %41, %bb19.us.i ]
  %_69.not = icmp eq i64 %new_len.i.i67.pn, %30
  br i1 %_69.not, label %bb19, label %bb51

bb51:                                             ; preds = %bb11
  %_0.sroa.0.1.i93.ph = getelementptr inbounds nuw i8, ptr %data.i.i68, i64 %30
  %43 = xor i64 %30, -1
  %_76 = add i64 %new_len.i.i67.pn, %43
  %_78 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i93.ph, i64 1
  %44 = load i8, ptr %_0.sroa.0.1.i93.ph, align 1, !noundef !3
  switch i8 %44, label %bb19 [
    i8 43, label %bb14
    i8 45, label %bb13
  ]

bb19:                                             ; preds = %bb14, %bb2.i106, %bb49, %bb13, %bb11, %bb51
  %has_target_feature.sroa.0.4.off0 = phi i1 [ %has_target_feature.sroa.0.3.off0178, %bb51 ], [ %has_target_feature.sroa.0.3.off0178, %bb11 ], [ %has_target_feature.sroa.0.3.off0178, %bb13 ], [ %spec.select158, %bb49 ], [ %47, %bb2.i106 ], [ %has_target_feature.sroa.0.3.off0178, %bb14 ]
  br i1 %.ph.off0, label %bb20, label %bb2.i76

bb14:                                             ; preds = %bb51
  %_3.not.i = icmp eq i64 %_76, %1
  br i1 %_3.not.i, label %bb2.i106, label %bb19

bb2.i106:                                         ; preds = %bb14
  %45 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(1) %_78, ptr noundef nonnull readonly align 1 dereferenceable(1) %0, i64 range(i64 0, -9223372036854775808) %1), !alias.scope !356
  %46 = icmp eq i32 %45, 0
  %47 = select i1 %46, i1 true, i1 %has_target_feature.sroa.0.3.off0178
  br label %bb19

bb13:                                             ; preds = %bb51
  %_3.not.i107 = icmp eq i64 %_76, %1
  br i1 %_3.not.i107, label %bb49, label %bb19

bb49:                                             ; preds = %bb13
  %48 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(1) %_78, ptr noundef nonnull readonly align 1 dereferenceable(1) %0, i64 range(i64 0, -9223372036854775808) %1), !alias.scope !360
  %.fr = freeze i32 %48
  %49 = icmp ne i32 %.fr, 0
  %spec.select158 = select i1 %49, i1 %has_target_feature.sroa.0.3.off0178, i1 false
  br label %bb19

bb29:                                             ; preds = %bb2.i.i.i4.i.i.i, %bb27
  resume { ptr, i32 } %.pn

bb25:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59, %bb23, %bb28
  %has_target_feature.sroa.0.0.off0122 = phi i1 [ false, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59 ], [ true, %bb23 ], [ false, %bb28 ]
  ret i1 %has_target_feature.sroa.0.0.off0122

bb23:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build.exit59, %bb28
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %name, ptr %args, align 8
  %_39.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCslwKqnJYeWCA_18build_script_build, ptr %_39.sroa.4.0..sroa_idx, align 8
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_1545dd372b2bbe07420f0d25b9c1ecbe, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br label %bb25
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCslwKqnJYeWCA_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5.i937 = alloca [24 x i8], align 8
  %_5.i919 = alloca [24 x i8], align 8
  %_14.i = alloca [104 x i8], align 8
  %result.i.i = alloca [24 x i8], align 8
  %_5.i.i.i.i.i.i570 = alloca [24 x i8], align 8
  %_5.i.i.i.i = alloca [24 x i8], align 8
  %_19.i.i = alloca [72 x i8], align 8
  %vector.i.i = alloca [24 x i8], align 8
  %_3.i571 = alloca [72 x i8], align 8
  %result.i = alloca [24 x i8], align 8
  %_5.i162.i.i = alloca [24 x i8], align 8
  %_5.i111.i.i = alloca [24 x i8], align 8
  %_5.i62.i.i = alloca [24 x i8], align 8
  %_5.i.i.i = alloca [24 x i8], align 8
  %_5.i.i.i.i.i.i.i = alloca [24 x i8], align 8
  %_31.i.i = alloca [24 x i8], align 8
  %digits.i.i = alloca [80 x i8], align 16
  %_6.i.i = alloca [72 x i8], align 8
  %release.i.i = alloca [80 x i8], align 16
  %verbose_version.i.i = alloca [16 x i8], align 8
  %_5.sroa.0.i.i = alloca i64, align 8
  %iter.i.i = alloca [48 x i8], align 8
  %_2.i.i = alloca [200 x i8], align 8
  %_27.i = alloca [24 x i8], align 8
  %_21.i = alloca [56 x i8], align 8
  %output.i = alloca [56 x i8], align 8
  %cmd.i = alloca [200 x i8], align 8
  %rustc1.i = alloca [48 x i8], align 8
  %_7.i = alloca [24 x i8], align 8
  %_6.i = alloca [24 x i8], align 8
  %_3.i185 = alloca [24 x i8], align 8
  %e.i175 = alloca [24 x i8], align 8
  %e.i163 = alloca [24 x i8], align 8
  %e.i = alloca [24 x i8], align 8
  %_350 = alloca [24 x i8], align 8
  %_346 = alloca [32 x i8], align 8
  %_345 = alloca [24 x i8], align 8
  %_331 = alloca [24 x i8], align 8
  %args4 = alloca [16 x i8], align 8
  %args3 = alloca [16 x i8], align 8
  %_297 = alloca [24 x i8], align 8
  %_250 = alloca [72 x i8], align 8
  %_247 = alloca [72 x i8], align 8
  %_232 = alloca [24 x i8], align 8
  %_230 = alloca [32 x i8], align 8
  %_217 = alloca [32 x i8], align 8
  %_193 = alloca [32 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %target_upper = alloca [24 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_21 = alloca [24 x i8], align 8
  %_16 = alloca [32 x i8], align 8
  %_13 = alloca [32 x i8], align 8
  %_10 = alloca [32 x i8], align 8
  %_7 = alloca [16 x i8], align 8
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_742f06589122110502429e832b81e8bd, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8724dc04cf16cdfa205f2534ce59cc5f, ptr noundef nonnull inttoptr (i64 81 to ptr))
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9529fcb5bab6980e7c00a1d3aa85ad20, ptr noundef nonnull inttoptr (i64 69 to ptr))
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10)
; call std::env::_var
  call void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_10, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dcbc225a8ec7dbfaaef714ff8a7176fb, i64 noundef 6)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !364)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !367)
  %_3.i = load i64, ptr %_10, align 8, !range !37, !alias.scope !367, !noalias !369, !noundef !3
  %0 = trunc nuw i64 %_3.i to i1
  br i1 %0, label %bb2.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build.exit, !prof !102

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i), !noalias !372
  %1 = getelementptr inbounds nuw i8, ptr %_10, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %1, i64 24, i1 false), !noalias !369
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1f011f14125487663894f05c404929a9, i64 noundef 14, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_84ab129184af97b3019d993fd0b7c805) #26
          to label %unreachable.i unwind label %cleanup.i, !noalias !373

cleanup.i:                                        ; preds = %bb2.i
  %2 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !374)
  %3 = load i64, ptr %e.i, align 8, !range !11, !alias.scope !374, !noalias !372, !noundef !3
  switch i64 %3, label %bb2.i.i.i4.i.i.i.i.i [
    i64 -9223372036854775808, label %common.resume
    i64 0, label %common.resume
  ]

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i
  %4 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  %_1.val1.i.i = load ptr, ptr %4, align 8, !alias.scope !374, !noalias !372, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %3, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !377
  br label %common.resume

unreachable.i:                                    ; preds = %bb2.i
  unreachable

common.resume:                                    ; preds = %bb386, %bb2.i.i.i4.i.i, %cleanup.i, %cleanup.i, %bb2.i.i.i4.i.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %2, %bb2.i.i.i4.i.i.i.i.i ], [ %2, %cleanup.i ], [ %2, %cleanup.i ], [ %.pn117, %bb2.i.i.i4.i.i ], [ %.pn117, %bb386 ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build.exit: ; preds = %start
  %5 = getelementptr inbounds nuw i8, ptr %_10, i64 8
  %_9.sroa.0.0.copyload = load i64, ptr %5, align 8, !alias.scope !373, !noalias !378
  %_9.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10, i64 16
  %_9.sroa.5.0.copyload = load ptr, ptr %_9.sroa.5.0..sroa_idx, align 8, !alias.scope !373, !noalias !378, !nonnull !3, !noundef !3
  %_9.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10, i64 24
  %_9.sroa.8.0.copyload = load i64, ptr %_9.sroa.8.0..sroa_idx, align 8, !alias.scope !373, !noalias !378
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10)
  store ptr %_9.sroa.5.0.copyload, ptr %_7, align 8
  %6 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  store i64 %_9.sroa.8.0.copyload, ptr %6, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0d3bcf6fb685f000bc18304ea76cbac4, i64 noundef 21)
          to label %bb6 unwind label %cleanup

bb386:                                            ; preds = %bb2.i.i.i4.i.i173, %bb385, %cleanup.i166, %cleanup.i166, %bb2.i.i.i4.i.i.i.i.i168, %cleanup
  %.pn117 = phi { ptr, i32 } [ %8, %cleanup ], [ %11, %bb2.i.i.i4.i.i.i.i.i168 ], [ %11, %cleanup.i166 ], [ %11, %cleanup.i166 ], [ %.pn115, %bb385 ], [ %.pn115, %bb2.i.i.i4.i.i173 ]
  %7 = icmp eq i64 %_9.sroa.0.0.copyload, 0
  br i1 %7, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb386
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9.sroa.5.0.copyload, i64 noundef %_9.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %common.resume

cleanup:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build.exit
  %8 = landingpad { ptr, i32 }
          cleanup
  br label %bb386

bb6:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build.exit
  tail call void @llvm.experimental.noalias.scope.decl(metadata !379)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !382)
  %_3.i164 = load i64, ptr %_13, align 8, !range !37, !alias.scope !382, !noalias !384, !noundef !3
  %9 = trunc nuw i64 %_3.i164 to i1
  br i1 %9, label %bb2.i165, label %bb7, !prof !102

bb2.i165:                                         ; preds = %bb6
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i163), !noalias !387
  %10 = getelementptr inbounds nuw i8, ptr %_13, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e.i163, ptr noundef nonnull readonly align 8 dereferenceable(24) %10, i64 24, i1 false), !noalias !384
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_86b01a9b169de421b373ac989e740d38, i64 noundef 29, ptr noundef nonnull align 1 %e.i163, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_453d9fa5d8e2ccad25a908c230c60eb0) #26
          to label %unreachable.i170 unwind label %cleanup.i166, !noalias !388

cleanup.i166:                                     ; preds = %bb2.i165
  %11 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !389)
  %12 = load i64, ptr %e.i163, align 8, !range !11, !alias.scope !389, !noalias !387, !noundef !3
  switch i64 %12, label %bb2.i.i.i4.i.i.i.i.i168 [
    i64 -9223372036854775808, label %bb386
    i64 0, label %bb386
  ]

bb2.i.i.i4.i.i.i.i.i168:                          ; preds = %cleanup.i166
  %13 = getelementptr inbounds nuw i8, ptr %e.i163, i64 8
  %_1.val1.i.i169 = load ptr, ptr %13, align 8, !alias.scope !389, !noalias !387, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i169, i64 noundef %12, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !392
  br label %bb386

unreachable.i170:                                 ; preds = %bb2.i165
  unreachable

bb7:                                              ; preds = %bb6
  %14 = getelementptr inbounds nuw i8, ptr %_13, i64 8
  %_12.sroa.0.0.copyload = load i64, ptr %14, align 8, !alias.scope !388, !noalias !393
  %_12.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 16
  %_12.sroa.5.0.copyload = load ptr, ptr %_12.sroa.5.0..sroa_idx, align 8, !alias.scope !388, !noalias !393, !nonnull !3, !noundef !3
  %_12.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 24
  %_12.sroa.8.0.copyload = load i64, ptr %_12.sroa.8.0..sroa_idx, align 8, !alias.scope !388, !noalias !393
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_16)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_16, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_aa4687de82972c6f88dd4ebd068e3b63, i64 noundef 19)
          to label %bb8 unwind label %cleanup6

bb385:                                            ; preds = %bb2.i.i.i4.i.i191, %bb384, %cleanup.i178, %cleanup.i178, %bb2.i.i.i4.i.i.i.i.i180, %cleanup6
  %.pn115 = phi { ptr, i32 } [ %16, %cleanup6 ], [ %19, %bb2.i.i.i4.i.i.i.i.i180 ], [ %19, %cleanup.i178 ], [ %19, %cleanup.i178 ], [ %.pn, %bb384 ], [ %.pn, %bb2.i.i.i4.i.i191 ]
  %15 = icmp eq i64 %_12.sroa.0.0.copyload, 0
  br i1 %15, label %bb386, label %bb2.i.i.i4.i.i173

bb2.i.i.i4.i.i173:                                ; preds = %bb385
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_12.sroa.5.0.copyload, i64 noundef %_12.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb386

cleanup6:                                         ; preds = %bb7
  %16 = landingpad { ptr, i32 }
          cleanup
  br label %bb385

bb8:                                              ; preds = %bb7
  tail call void @llvm.experimental.noalias.scope.decl(metadata !394)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !397)
  %_3.i176 = load i64, ptr %_16, align 8, !range !37, !alias.scope !397, !noalias !399, !noundef !3
  %17 = trunc nuw i64 %_3.i176 to i1
  br i1 %17, label %bb2.i177, label %bb9, !prof !102

bb2.i177:                                         ; preds = %bb8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i175), !noalias !402
  %18 = getelementptr inbounds nuw i8, ptr %_16, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e.i175, ptr noundef nonnull readonly align 8 dereferenceable(24) %18, i64 24, i1 false), !noalias !399
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b7c7e69e15013ee480a39c67f380d6cc, i64 noundef 27, ptr noundef nonnull align 1 %e.i175, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_57f734e358c6dd0a67f2db5255d1a5b4) #26
          to label %unreachable.i182 unwind label %cleanup.i178, !noalias !403

cleanup.i178:                                     ; preds = %bb2.i177
  %19 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !404)
  %20 = load i64, ptr %e.i175, align 8, !range !11, !alias.scope !404, !noalias !402, !noundef !3
  switch i64 %20, label %bb2.i.i.i4.i.i.i.i.i180 [
    i64 -9223372036854775808, label %bb385
    i64 0, label %bb385
  ]

bb2.i.i.i4.i.i.i.i.i180:                          ; preds = %cleanup.i178
  %21 = getelementptr inbounds nuw i8, ptr %e.i175, i64 8
  %_1.val1.i.i181 = load ptr, ptr %21, align 8, !alias.scope !404, !noalias !402, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i181, i64 noundef %20, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !407
  br label %bb385

unreachable.i182:                                 ; preds = %bb2.i177
  unreachable

bb9:                                              ; preds = %bb8
  %22 = getelementptr inbounds nuw i8, ptr %_16, i64 8
  %_15.sroa.0.0.copyload = load i64, ptr %22, align 8, !alias.scope !403, !noalias !408
  %_15.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_16, i64 16
  %_15.sroa.5.0.copyload = load ptr, ptr %_15.sroa.5.0..sroa_idx, align 8, !alias.scope !403, !noalias !408, !nonnull !3, !noundef !3
  %_15.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_16, i64 24
  %_15.sroa.8.0.copyload = load i64, ptr %_15.sroa.8.0..sroa_idx, align 8, !alias.scope !403, !noalias !408
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_16)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i185), !noalias !409
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3.i185, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5)
          to label %.noexc unwind label %cleanup7

.noexc:                                           ; preds = %bb9
  %23 = load i64, ptr %_3.i185, align 8, !range !11, !noalias !409, !noundef !3
  %.not.i = icmp eq i64 %23, -9223372036854775808
  br i1 %.not.i, label %bb31.i, label %bb32.i

bb32.i:                                           ; preds = %.noexc
  %_33.sroa.5.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i185, i64 8
  %_33.sroa.5.0.copyload.i = load ptr, ptr %_33.sroa.5.0._3.sroa_idx.i, align 8, !noalias !409
  %_33.sroa.6.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i185, i64 16
  %_33.sroa.6.0.copyload.i = load i64, ptr %_33.sroa.6.0._3.sroa_idx.i, align 8, !noalias !409
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i185), !noalias !409
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i), !noalias !409
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_6.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
          to label %bb3.i unwind label %bb29.i, !noalias !409

bb31.i:                                           ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i185), !noalias !409
  br label %bb12

bb3.i:                                            ; preds = %bb32.i
  %24 = load i64, ptr %_6.i, align 8, !range !11, !noalias !409, !noundef !3
  switch i64 %24, label %bb2.i.i.i4.i.i.i.i.i187 [
    i64 -9223372036854775808, label %bb11.i.i.thread
    i64 0, label %bb5.i186
  ]

bb2.i.i.i4.i.i.i.i.i187:                          ; preds = %bb3.i
  %25 = getelementptr inbounds nuw i8, ptr %_6.i, i64 8
  %_6.val17.i = load ptr, ptr %25, align 8, !noalias !409, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val17.i, i64 noundef %24, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !409
  br label %bb5.i186

bb5.i186:                                         ; preds = %bb2.i.i.i4.i.i.i.i.i187, %bb3.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i), !noalias !409
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_7.i), !noalias !409
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_7.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f36ce88bd5d4a921175f5521f484b675, i64 noundef 13)
          to label %bb6.i unwind label %bb29.i, !noalias !409

bb6.i:                                            ; preds = %bb5.i186
  call void @llvm.experimental.noalias.scope.decl(metadata !412)
  %26 = load i64, ptr %_7.i, align 8, !range !11, !alias.scope !412, !noalias !415, !noundef !3
  %.not.i.i = icmp eq i64 %26, -9223372036854775808
  br i1 %.not.i.i, label %bb11.i.i.thread1243, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb6.i
  %x.sroa.7.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_7.i, i64 8
  %x.sroa.7.0.copyload.i.i = load ptr, ptr %x.sroa.7.0.self.sroa_idx.i.i, align 8, !alias.scope !412, !noalias !415
  %x.sroa.9.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_7.i, i64 16
  %x.sroa.9.0.copyload.i.i = load i64, ptr %x.sroa.9.0.self.sroa_idx.i.i, align 8, !alias.scope !412, !noalias !415
  %_3.i.not.i.i = icmp eq i64 %x.sroa.9.0.copyload.i.i, 0
  br i1 %_3.i.not.i.i, label %bb4.i.i, label %bb11.i.i

bb4.i.i:                                          ; preds = %bb2.i.i
  %27 = icmp eq i64 %26, 0
  br i1 %27, label %bb11.i.i.thread1243, label %bb2.i.i.i4.i.i.i5.i.i

bb2.i.i.i4.i.i.i5.i.i:                            ; preds = %bb4.i.i
  %28 = icmp ne ptr %x.sroa.7.0.copyload.i.i, null
  call void @llvm.assume(i1 %28)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload.i.i, i64 noundef %26, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !417
  br label %bb11.i.i.thread1243

bb11.i.i.thread:                                  ; preds = %bb3.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i), !noalias !409
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i

bb11.i.i.thread1243:                              ; preds = %bb6.i, %bb2.i.i.i4.i.i.i5.i.i, %bb4.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7.i), !noalias !409
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i

bb11.i.i:                                         ; preds = %bb2.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7.i), !noalias !409
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %rustc1.i), !noalias !409
  call void @llvm.experimental.noalias.scope.decl(metadata !418)
  %_5.sroa.4.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 8
  store ptr %x.sroa.7.0.copyload.i.i, ptr %_5.sroa.4.0._0.sroa_idx.i.i, align 8, !alias.scope !421, !noalias !423
  %_5.sroa.5.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 16
  store i64 %x.sroa.9.0.copyload.i.i, ptr %_5.sroa.5.0._0.sroa_idx.i.i, align 8, !alias.scope !421, !noalias !423
  %29 = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 24
  store i64 %23, ptr %29, align 8, !alias.scope !425, !noalias !426
  %_10.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 32
  store ptr %_33.sroa.5.0.copyload.i, ptr %_10.sroa.4.0..sroa_idx.i, align 8, !alias.scope !425, !noalias !426
  %_10.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 40
  store i64 %_33.sroa.6.0.copyload.i, ptr %_10.sroa.5.0..sroa_idx.i, align 8, !alias.scope !425, !noalias !426
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd.i), !noalias !409
  br label %bb36.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i: ; preds = %bb11.i.i.thread1243, %bb11.i.i.thread
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %rustc1.i), !noalias !409
  %30 = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 24
  store i64 %23, ptr %30, align 8, !alias.scope !427, !noalias !426
  %_10.sroa.4.0..sroa_idx.i1249 = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 32
  store ptr %_33.sroa.5.0.copyload.i, ptr %_10.sroa.4.0..sroa_idx.i1249, align 8, !alias.scope !427, !noalias !426
  %_10.sroa.5.0..sroa_idx.i1250 = getelementptr inbounds nuw i8, ptr %rustc1.i, i64 40
  store i64 %_33.sroa.6.0.copyload.i, ptr %_10.sroa.5.0..sroa_idx.i1250, align 8, !alias.scope !427, !noalias !426
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd.i), !noalias !409
  store i64 -9223372036854775808, ptr %30, align 8, !alias.scope !430, !noalias !437
  br label %bb36.i

bb36.i:                                           ; preds = %bb11.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i
  %.sink.i = phi i64 [ -9223372036854775807, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i ], [ -9223372036854775808, %bb11.i.i ]
  %_14.sroa.0.0._14.sroa.0.0..i = phi i64 [ %23, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i ], [ %26, %bb11.i.i ]
  %_14.sroa.7.1.i = phi ptr [ %_33.sroa.5.0.copyload.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i ], [ %x.sroa.7.0.copyload.i.i, %bb11.i.i ]
  %_14.sroa.8.1.i = phi i64 [ %_33.sroa.6.0.copyload.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit5.i.i ], [ %x.sroa.9.0.copyload.i.i, %bb11.i.i ]
  store i64 %.sink.i, ptr %rustc1.i, align 8, !alias.scope !439, !noalias !442
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i.i), !noalias !444
  %31 = icmp ne ptr %_14.sroa.7.1.i, null
  call void @llvm.assume(i1 %31)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_14.sroa.7.1.i, i64 noundef %_14.sroa.8.1.i)
          to label %bb2.i26.i unwind label %cleanup.i.i, !noalias !444

cleanup.i.i:                                      ; preds = %bb36.i
  %32 = landingpad { ptr, i32 }
          cleanup
  %33 = icmp eq i64 %_14.sroa.0.0._14.sroa.0.0..i, 0
  br i1 %33, label %bb27.i, label %bb2.i.i.i4.i.i.i.i25.i

bb2.i.i.i4.i.i.i.i25.i:                           ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_14.sroa.7.1.i, i64 noundef %_14.sroa.0.0._14.sroa.0.0..i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !444
  br label %bb27.i

bb2.i26.i:                                        ; preds = %bb36.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd.i, ptr noundef nonnull align 8 dereferenceable(200) %_2.i.i, i64 200, i1 false), !noalias !448
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i.i), !noalias !444
  %34 = icmp eq i64 %_14.sroa.0.0._14.sroa.0.0..i, 0
  br i1 %34, label %bb12.i, label %bb2.i.i.i4.i.i.i6.i.i

bb2.i.i.i4.i.i.i6.i.i:                            ; preds = %bb2.i26.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_14.sroa.7.1.i, i64 noundef %_14.sroa.0.0._14.sroa.0.0..i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !444
  br label %bb12.i

bb12.i:                                           ; preds = %bb2.i.i.i4.i.i.i6.i.i, %bb2.i26.i
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %iter.i.i), !noalias !449
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %iter.i.i, ptr noundef nonnull align 8 dereferenceable(48) %rustc1.i, i64 48, i1 false), !noalias !409
  %iter.promoted.i.i = load i64, ptr %iter.i.i, align 8, !alias.scope !453, !noalias !458
  %x.sroa.7.0.opt.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8
  %x.sroa.7.0.copyload7.i.i.i.i = load ptr, ptr %x.sroa.7.0.opt.sroa_idx.i.i.i.i, align 8, !noalias !449
  %x.sroa.8.0.opt.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16
  %x.sroa.8.0.copyload8.i.i.i.i = load i64, ptr %x.sroa.8.0.opt.sroa_idx.i.i.i.i, align 8, !noalias !449
  %_57.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24
  %_5.sroa.8.0._57.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 32
  %_5.sroa.8.0.copyload11.i.i = load ptr, ptr %_5.sroa.8.0._57.i.sroa_idx.i.i, align 8, !noalias !449
  %_5.sroa.9.0._57.i.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 40
  %_5.sroa.9.0.copyload12.i.i = load i64, ptr %_5.sroa.9.0._57.i.sroa_idx.i.i, align 8, !noalias !449
  br label %bb2.i27.i

bb2.i27.i:                                        ; preds = %bb9.i.i, %bb12.i
  %_1.val.i24.i.i = phi i64 [ %iter.promoted.i.i, %bb12.i ], [ %_1.val.i25.i.i, %bb9.i.i ]
  %spec.store.select.i.i23.i.i = phi i64 [ %iter.promoted.i.i, %bb12.i ], [ %spec.store.select.i.i21.i.i, %bb9.i.i ]
  %_5.sroa.9.0.i.i = phi i64 [ undef, %bb12.i ], [ %_5.sroa.9.219.i.i, %bb9.i.i ]
  %_5.sroa.8.0.i.i = phi ptr [ undef, %bb12.i ], [ %_5.sroa.8.220.i.i, %bb9.i.i ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5.sroa.0.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !461)
  call void @llvm.experimental.noalias.scope.decl(metadata !462)
  %.not.i.i.i.i = icmp eq i64 %spec.store.select.i.i23.i.i, -9223372036854775807
  br i1 %.not.i.i.i.i, label %bb2.i.i.i.i, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build.exit.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build.exit.i.i.i: ; preds = %bb2.i27.i
  %.not3.i.i.i.i = icmp eq i64 %spec.store.select.i.i23.i.i, -9223372036854775808
  %spec.store.select.i.i.i.i = select i1 %.not3.i.i.i.i, i64 -9223372036854775807, i64 -9223372036854775808
  store i64 %spec.store.select.i.i.i.i, ptr %iter.i.i, align 8, !alias.scope !453, !noalias !458
  call void @llvm.experimental.noalias.scope.decl(metadata !463)
  call void @llvm.experimental.noalias.scope.decl(metadata !466)
  call void @llvm.experimental.noalias.scope.decl(metadata !468)
  br i1 %.not3.i.i.i.i, label %bb2.i.i.i.i, label %bb3.thread.i.i

bb3.thread.i.i:                                   ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build.exit.i.i.i
  store i64 %spec.store.select.i.i23.i.i, ptr %_5.sroa.0.i.i, align 8, !alias.scope !470, !noalias !471
  br label %bb7.i.i

bb2.i.i.i.i:                                      ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build.exit.i.i.i, %bb2.i27.i
  %_1.val.i.i.i = phi i64 [ -9223372036854775807, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build.exit.i.i.i ], [ %_1.val.i24.i.i, %bb2.i27.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !472)
  %35 = load i64, ptr %_57.i.i.i, align 8, !range !10, !alias.scope !475, !noalias !477, !noundef !3
  %.not.i.i.i.i.i = icmp eq i64 %35, -9223372036854775807
  br i1 %.not.i.i.i.i.i, label %bb3.i29.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i
  store i64 %35, ptr %_5.sroa.0.i.i, align 8, !alias.scope !478, !noalias !482
  br label %bb3.i29.i

bb12.i.i:                                         ; preds = %bb2.i.i.i4.i.i.i.i28.i, %cleanup1.i.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBN_7sources4once4OnceB1G_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(48) %iter.i.i) #24, !noalias !483
  br label %bb25.i

bb3.i29.i:                                        ; preds = %bb5.i.i.i.i.i, %bb2.i.i.i.i
  %_5.sroa.9.1.i.i = phi i64 [ %_5.sroa.9.0.i.i, %bb2.i.i.i.i ], [ %_5.sroa.9.0.copyload12.i.i, %bb5.i.i.i.i.i ]
  %_5.sroa.8.1.i.i = phi ptr [ %_5.sroa.8.0.i.i, %bb2.i.i.i.i ], [ %_5.sroa.8.0.copyload11.i.i, %bb5.i.i.i.i.i ]
  %_1.sink.i.i.i.i.i = phi ptr [ %_5.sroa.0.i.i, %bb2.i.i.i.i ], [ %_57.i.i.i, %bb5.i.i.i.i.i ]
  store i64 -9223372036854775808, ptr %_1.sink.i.i.i.i.i, align 8, !alias.scope !484, !noalias !482
  %_5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i.i = load i64, ptr %_5.sroa.0.i.i, align 8, !noalias !449
  %.not.i30.i = icmp eq i64 %_5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i.i, -9223372036854775808
  br i1 %.not.i30.i, label %bb6.i.i, label %bb7.i.i

bb6.i.i:                                          ; preds = %bb3.i29.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !485)
  switch i64 %_1.val.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i.i
    i64 -9223372036854775808, label %bb4.i.i.i
    i64 0, label %bb4.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i:                   ; preds = %bb6.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload7.i.i.i.i, i64 noundef %_1.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !488
  br label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i, %bb6.i.i, %bb6.i.i, %bb6.i.i
  %.val3.i.i.i = load i64, ptr %_57.i.i.i, align 8, !range !10, !alias.scope !485, !noalias !449, !noundef !3
  switch i64 %.val3.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i5.i.i.i [
    i64 -9223372036854775807, label %bb13.i
    i64 -9223372036854775808, label %bb13.i
    i64 0, label %bb13.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i5.i.i.i:                ; preds = %bb4.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.0.copyload11.i.i, i64 noundef %.val3.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !488
  br label %bb13.i

cleanup1.i.i:                                     ; preds = %bb7.i.i
  %36 = landingpad { ptr, i32 }
          cleanup
  %37 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.18.i.i, 0
  br i1 %37, label %bb12.i.i, label %bb2.i.i.i4.i.i.i.i28.i

bb2.i.i.i4.i.i.i.i28.i:                           ; preds = %cleanup1.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.220.i.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.18.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !483
  br label %bb12.i.i

bb7.i.i:                                          ; preds = %bb3.i29.i, %bb3.thread.i.i
  %_1.val.i25.i.i = phi i64 [ -9223372036854775808, %bb3.thread.i.i ], [ %_1.val.i.i.i, %bb3.i29.i ]
  %spec.store.select.i.i21.i.i = phi i64 [ -9223372036854775808, %bb3.thread.i.i ], [ -9223372036854775807, %bb3.i29.i ]
  %_5.sroa.8.220.i.i = phi ptr [ %x.sroa.7.0.copyload7.i.i.i.i, %bb3.thread.i.i ], [ %_5.sroa.8.1.i.i, %bb3.i29.i ]
  %_5.sroa.9.219.i.i = phi i64 [ %x.sroa.8.0.copyload8.i.i.i.i, %bb3.thread.i.i ], [ %_5.sroa.9.1.i.i, %bb3.i29.i ]
  %_5.sroa.0.0._5.sroa.0.0.18.i.i = phi i64 [ %spec.store.select.i.i23.i.i, %bb3.thread.i.i ], [ %_5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i.i, %bb3.i29.i ]
  %38 = icmp ne ptr %_5.sroa.8.220.i.i, null
  call void @llvm.assume(i1 %38)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.8.220.i.i, i64 noundef %_5.sroa.9.219.i.i)
          to label %bb8.i.i unwind label %cleanup1.i.i, !noalias !483

bb8.i.i:                                          ; preds = %bb7.i.i
  %39 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.18.i.i, 0
  br i1 %39, label %bb9.i.i, label %bb2.i.i.i4.i.i.i8.i.i

bb2.i.i.i4.i.i.i8.i.i:                            ; preds = %bb8.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.220.i.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.18.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !483
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i8.i.i, %bb8.i.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i.i)
  br label %bb2.i27.i

bb25.i:                                           ; preds = %cleanup5.i, %bb1.i.i.i.i.i.i.i, %cleanup4.i, %bb12.i.i
  %.pn.i = phi { ptr, i32 } [ %lpad.phi.i, %cleanup5.i ], [ %36, %bb12.i.i ], [ %40, %cleanup4.i ], [ %57, %bb1.i.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd.i) #24
          to label %bb384 unwind label %terminate.i, !noalias !409

cleanup4.i:                                       ; preds = %bb14.i, %bb13.i
  %40 = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

bb13.i:                                           ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i5.i.i.i, %bb4.i.i.i, %bb4.i.i.i, %bb4.i.i.i
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %iter.i.i), !noalias !449
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output.i), !noalias !409
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_21.i), !noalias !409
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9994c89853702f537928cd873913af0d, i64 noundef 3)
          to label %bb14.i unwind label %cleanup4.i, !noalias !409

bb14.i:                                           ; preds = %bb13.i
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_21.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd.i)
          to label %bb15.i unwind label %cleanup4.i, !noalias !409

bb15.i:                                           ; preds = %bb14.i
  %41 = load i64, ptr %_21.i, align 8, !range !11, !noalias !409, !noundef !3
  %42 = icmp eq i64 %41, -9223372036854775808
  br i1 %42, label %bb3.i36.i, label %bb43.i

bb3.i36.i:                                        ; preds = %bb15.i
  call void @llvm.experimental.noalias.scope.decl(metadata !489)
  %43 = getelementptr inbounds nuw i8, ptr %_21.i, i64 8
  %.val.i.i = load ptr, ptr %43, align 8, !alias.scope !489, !noalias !409, !nonnull !3, !noundef !3
  %bits.i.i.i.i.i.i = ptrtoint ptr %.val.i.i to i64
  %_5.i.i.i.i.i.i = and i64 %bits.i.i.i.i.i.i, 3
  %switch.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i.i, label %bb2.i2.i.i.i.i.i, label %bb42.i, !prof !492

bb2.i2.i.i.i.i.i:                                 ; preds = %bb3.i36.i
  %44 = getelementptr i8, ptr %.val.i.i, i64 -1
  %45 = icmp ne ptr %44, null
  call void @llvm.assume(i1 %45)
  %_6.val.i.i.i.i.i.i.i = load ptr, ptr %44, align 8, !noalias !493
  %46 = getelementptr i8, ptr %.val.i.i, i64 7
  %_6.val1.i.i.i.i.i.i.i = load ptr, ptr %46, align 8, !noalias !493, !nonnull !3, !align !7, !noundef !3
  %47 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i, align 8, !invariant.load !3, !noalias !493
  %.not.i.i.i.i.i.i.i.i.i = icmp eq ptr %47, null
  br i1 %.not.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i2.i.i.i.i.i
  %48 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %48)
  invoke void %47(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i, !noalias !493

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i
  %49 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %49)
  %50 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %51 = load i64, ptr %50, align 8, !range !8, !invariant.load !3, !noalias !493
  %52 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %53 = load i64, ptr %52, align 8, !range !9, !invariant.load !3, !noalias !493
  %54 = add i64 %53, -1
  %55 = icmp sgt i64 %54, -1
  call void @llvm.assume(i1 %55)
  %56 = icmp eq i64 %51, 0
  br i1 %56, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %51, i64 noundef range(i64 1, -9223372036854775807) %53) #23, !noalias !493
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i:                        ; preds = %is_not_null.i.i.i.i.i.i.i.i.i
  %57 = landingpad { ptr, i32 }
          cleanup
  %58 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %59 = load i64, ptr %58, align 8, !range !8, !invariant.load !3, !noalias !493
  %60 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %61 = load i64, ptr %60, align 8, !range !9, !invariant.load !3, !noalias !493
  %62 = add i64 %61, -1
  %63 = icmp sgt i64 %62, -1
  call void @llvm.assume(i1 %63)
  %64 = icmp eq i64 %59, 0
  br i1 %64, label %bb1.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %59, i64 noundef range(i64 1, -9223372036854775807) %61) #23, !noalias !493
  br label %bb1.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %44, i64 noundef 24, i64 noundef 8) #23, !noalias !493
  br label %bb25.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %44, i64 noundef 24, i64 noundef 8) #23, !noalias !493
  br label %bb42.i

bb43.i:                                           ; preds = %bb15.i
  %_46.sroa.4.0._21.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_21.i, i64 8
  %val2.sroa.4.0.output.sroa_idx.i = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val2.sroa.4.0.output.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_46.sroa.4.0._21.sroa_idx.i, i64 48, i1 false), !noalias !409
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_21.i), !noalias !409
  store i64 %41, ptr %output.i, align 8, !noalias !409
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_27.i), !noalias !409
  %_53.i = load ptr, ptr %val2.sroa.4.0.output.sroa_idx.i, align 8, !noalias !409, !nonnull !3, !noundef !3
  %65 = getelementptr inbounds nuw i8, ptr %output.i, i64 16
  %_52.i = load i64, ptr %65, align 8, !noalias !409, !noundef !3
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_27.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_53.i, i64 noundef %_52.i)
          to label %bb16.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

bb42.i:                                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i, %bb3.i36.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_21.i), !noalias !409
  br label %bb20.i

bb20.i:                                           ; preds = %bb2.i.i.i4.i7.i.i, %bb4.i39.i, %bb42.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i), !noalias !409
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd.i)
          to label %.noexc188 unwind label %cleanup7

.noexc188:                                        ; preds = %bb20.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd.i), !noalias !409
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %rustc1.i), !noalias !409
  br label %bb12

cleanup5.loopexit.i:                              ; preds = %bb2.i.i.i.i.i.i.i
  %lpad.loopexit.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup5.i

cleanup5.loopexit.split-lp.i:                     ; preds = %bb15.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i, %bb85.i.i, %bb75.i.i, %bb2.i164.i.i, %bb55.i.i, %bb2.i64.i.i, %bb42.i.i, %bb37.i.i, %bb31.i.i, %bb43.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup5.i

cleanup5.i:                                       ; preds = %cleanup5.loopexit.split-lp.i, %cleanup5.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit.i, %cleanup5.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup5.loopexit.split-lp.i ]
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i) #24, !noalias !409
  br label %bb25.i

bb16.i:                                           ; preds = %bb43.i
  %_54.i = load i64, ptr %_27.i, align 8, !range !37, !noalias !409, !noundef !3
  %66 = trunc nuw i64 %_54.i to i1
  br i1 %66, label %bb44.i, label %bb45.i

bb44.i:                                           ; preds = %bb16.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_27.i), !noalias !409
  call void @llvm.experimental.noalias.scope.decl(metadata !494)
  %67 = icmp eq i64 %41, 0
  br i1 %67, label %bb4.i39.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb44.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_53.i, i64 noundef %41, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !497
  br label %bb4.i39.i

bb4.i39.i:                                        ; preds = %bb2.i.i.i4.i.i.i, %bb44.i
  %68 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %68, align 8, !alias.scope !494, !noalias !409
  %69 = icmp eq i64 %.val2.i.i, 0
  br i1 %69, label %bb20.i, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i39.i
  %70 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %70, align 8, !alias.scope !494, !noalias !409, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !497
  br label %bb20.i

bb45.i:                                           ; preds = %bb16.i
  %71 = getelementptr inbounds nuw i8, ptr %_27.i, i64 8
  %_55.0.i = load ptr, ptr %71, align 8, !noalias !409, !nonnull !3, !align !18, !noundef !3
  %72 = getelementptr inbounds nuw i8, ptr %_27.i, i64 16
  %_55.1.i = load i64, ptr %72, align 8, !noalias !409, !noundef !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_27.i), !noalias !409
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %verbose_version.i.i), !noalias !409
  store ptr %_55.0.i, ptr %verbose_version.i.i, align 8, !noalias !498
  %73 = getelementptr inbounds nuw i8, ptr %verbose_version.i.i, i64 8
  store i64 %_55.1.i, ptr %73, align 8, !noalias !498
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %release.i.i), !noalias !498
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_6.i.i), !noalias !498
  store i64 0, ptr %_6.i.i, align 8, !noalias !498
  %_54.sroa.4.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 8
  store i64 %_55.1.i, ptr %_54.sroa.4.0._6.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 16
  store ptr %_55.0.i, ptr %_54.sroa.5.0._6.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.sroa.4.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 24
  store i64 %_55.1.i, ptr %_54.sroa.5.sroa.4.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.sroa.5.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 32
  store i64 0, ptr %_54.sroa.5.sroa.5.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.sroa.6.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 40
  store i64 %_55.1.i, ptr %_54.sroa.5.sroa.6.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.sroa.7.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 48
  store <2 x i32> splat (i32 10), ptr %_54.sroa.5.sroa.7.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.5.sroa.9.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 56
  store i8 1, ptr %_54.sroa.5.sroa.9.0._54.sroa.5.0._6.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.6.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 64
  store i8 0, ptr %_54.sroa.6.0._6.sroa_idx.i.i, align 8, !noalias !498
  %_54.sroa.7.0._6.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 65
  store i8 0, ptr %_54.sroa.7.0._6.sroa_idx.i.i, align 1, !noalias !498
  call void @llvm.experimental.noalias.scope.decl(metadata !502)
  %74 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i.i.i, i64 16
  br label %bb2.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1j_7Version5parse0E0B1l_.exit.i.i.i, %bb45.i
  call void @llvm.experimental.noalias.scope.decl(metadata !505)
  call void @llvm.experimental.noalias.scope.decl(metadata !508)
  call void @llvm.experimental.noalias.scope.decl(metadata !511)
  call void @llvm.experimental.noalias.scope.decl(metadata !514)
  %_4.val.i.i.i.i.i.i.i = load ptr, ptr %_54.sroa.5.0._6.sroa_idx.i.i, align 8, !alias.scope !517, !noalias !498, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i.i), !noalias !518
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_54.sroa.5.0._6.sroa_idx.i.i) #28
          to label %.noexc.i unwind label %cleanup5.loopexit.i, !noalias !409

.noexc.i:                                         ; preds = %bb2.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i.i.i, align 8, !range !37, !noalias !518, !noundef !3
  %75 = trunc nuw i64 %_7.i.i.i.i.i.i.i to i1
  br i1 %75, label %bb7.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i:                                ; preds = %.noexc.i
  %b.i.i.i.i.i.i.i = load i64, ptr %74, align 8, !noalias !518, !noundef !3
  %i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !517, !noalias !498, !noundef !3
  %new_len.i.i.i.i.i.i.i = sub nuw i64 %b.i.i.i.i.i.i.i, %i.i.i.i.i.i.i.i
  %data.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i.i.i.i.i, i64 %i.i.i.i.i.i.i.i
  store i64 %b.i.i.i.i.i.i.i, ptr %_6.i.i, align 8, !alias.scope !517, !noalias !498
  br label %bb5.i.i.i.i40.i

bb6.i.i.i.i.i.i.i:                                ; preds = %.noexc.i
  %76 = load i8, ptr %_54.sroa.7.0._6.sroa_idx.i.i, align 1, !range !54, !alias.scope !519, !noalias !498, !noundef !3
  %_2.i.i.i.i.i.i.i.i = trunc nuw i8 %76 to i1
  br i1 %_2.i.i.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb6.i.i.i.i.i.i.i
  store i8 1, ptr %_54.sroa.7.0._6.sroa_idx.i.i, align 1, !alias.scope !519, !noalias !498
  %77 = load i8, ptr %_54.sroa.6.0._6.sroa_idx.i.i, align 8, !range !54, !alias.scope !519, !noalias !498, !noundef !3
  %_3.i.i.i.i.i.i.i.i = trunc nuw i8 %77 to i1
  %i.pre.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !519, !noalias !498
  %i1.pre.i.i.i.i.i.i.i.i = load i64, ptr %_54.sroa.4.0._6.sroa_idx.i.i, align 8, !alias.scope !519, !noalias !498
  %_4.not.i.i.i.i.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i.i.i
  %or.cond.not.i.i.i.i.i.i.i.i = select i1 %_3.i.i.i.i.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i.i.i.i.i
  br i1 %or.cond.not.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i, label %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb1.i.i.i.i.i.i.i.i
  %_10.val.i.i.i.i.i.i.i.i = load ptr, ptr %_54.sroa.5.0._6.sroa_idx.i.i, align 8, !alias.scope !519, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i.i.i
  %data.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i.i.i.i, i64 %i.pre.i.i.i.i.i.i.i.i
  br label %bb5.i.i.i.i40.i

_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i.i), !noalias !518
  br label %bb33.i.i

bb5.i.i.i.i40.i:                                  ; preds = %bb4.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i
  %_0.sroa.4.0.i.i.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i.i.i.i = phi ptr [ %data.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i ], [ %data.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i.i), !noalias !518
  %78 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i.i.i.i.i.i.i, 0
  %79 = insertvalue { ptr, i64 } %78, i64 %_0.sroa.4.0.i.i.i.i.i.i.i, 1
  %_5.not.i.i.i.i.i.i.i.i.i = icmp eq i64 %_0.sroa.4.0.i.i.i.i.i.i.i, 0
  br i1 %_5.not.i.i.i.i.i.i.i.i.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i.i.i: ; preds = %bb5.i.i.i.i40.i
  %80 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i.i.i, i64 %_0.sroa.4.0.i.i.i.i.i.i.i
  %_16.i.i.i.i.i.i.i.i.i = getelementptr i8, ptr %80, i64 -1
  %rhsc.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !522, !noalias !531
  %rhsc.i.i.fr.i.i.i.i.i.i.i = freeze i8 %rhsc.i.i.i.i.i.i.i.i.i
  %81 = icmp eq i8 %rhsc.i.i.fr.i.i.i.i.i.i.i, 10
  %i.i.i.i.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i.i.i, -1
  br i1 %81, label %bb1.i.i.i.i.i.i46.i, label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i

bb1.i.i.i.i.i.i46.i:                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i.i.i
  %82 = insertvalue { ptr, i64 } %79, i64 %i.i.i.i.i.i.i.i.i, 1
  %_5.not.i.i12.i.i.i.i.i.i.i = icmp eq i64 %i.i.i.i.i.i.i.i.i, 0
  br i1 %_5.not.i.i12.i.i.i.i.i.i.i, label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i.i.i: ; preds = %bb1.i.i.i.i.i.i46.i
  %83 = getelementptr i8, ptr %_0.sroa.0.0.i.i.i.i.i.i.i, i64 %i.i.i.i.i.i.i.i.i
  %_16.i.i14.i.i.i.i.i.i.i = getelementptr i8, ptr %83, i64 -1
  %rhsc.i.i16.i.i.i.i.i.i.i = load i8, ptr %_16.i.i14.i.i.i.i.i.i.i, align 1, !alias.scope !534, !noalias !539
  %rhsc.i.i16.fr.i.i.i.i.i.i.i = freeze i8 %rhsc.i.i16.i.i.i.i.i.i.i
  %84 = icmp eq i8 %rhsc.i.i16.fr.i.i.i.i.i.i.i, 13
  %i.i17.i.i.i.i.i.i.i = add i64 %_0.sroa.4.0.i.i.i.i.i.i.i, -2
  %spec.select.i19.i.i.i.i.i.i.i = select i1 %84, ptr %_0.sroa.0.0.i.i.i.i.i.i.i, ptr null
  br label %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i.i.i

_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i.i.i: ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i46.i
  %i4.i20.i.i.i.i.i.i.i = phi i64 [ %i.i17.i.i.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i.i.i ], [ -1, %bb1.i.i.i.i.i.i46.i ]
  %85 = phi ptr [ %spec.select.i19.i.i.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i13.i.i.i.i.i.i.i ], [ null, %bb1.i.i.i.i.i.i46.i ]
  %86 = insertvalue { ptr, i64 } poison, ptr %85, 0
  %87 = insertvalue { ptr, i64 } %86, i64 %i4.i20.i.i.i.i.i.i.i, 1
  %.not11.i.i.i.i.i.i.i = icmp eq ptr %85, null
  %..i.i.i.i.i.i.i = select i1 %.not11.i.i.i.i.i.i.i, { ptr, i64 } %82, { ptr, i64 } %87
  br label %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i

_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i: ; preds = %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i.i.i, %bb5.i.i.i.i40.i
  %.merged.i.i.i.i.i.i.i = phi { ptr, i64 } [ %..i.i.i.i.i.i.i, %_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of.exit21.i.i.i.i.i.i.i ], [ %79, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i.i.i.i.i ], [ %79, %bb5.i.i.i.i40.i ]
  %_7.0.i.i.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i.i.i, 0
  %_7.1.i.i.i.i.i = extractvalue { ptr, i64 } %.merged.i.i.i.i.i.i.i, 1
  %.not.i.i41.i = icmp eq ptr %_7.0.i.i.i.i.i, null
  br i1 %.not.i.i41.i, label %bb33.i.i, label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i
  %_4.not.i.i.i.i.i.i = icmp samesign ult i64 %_7.1.i.i.i.i.i, 9
  br i1 %_4.not.i.i.i.i.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1j_7Version5parse0E0B1l_.exit.i.i.i, label %_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parse0B6_.exit.i.i.i.i

_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parse0B6_.exit.i.i.i.i: ; preds = %bb3.i.i.i
  %88 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_b7185c5006b7edc9741130ed35f40cf6, ptr noundef nonnull readonly align 1 dereferenceable(9) %_7.0.i.i.i.i.i, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !542, !noalias !549
  %.fr.i.i.i.i = freeze i32 %88
  %89 = icmp eq i32 %.fr.i.i.i.i, 0
  br i1 %89, label %bb28.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1j_7Version5parse0E0B1l_.exit.i.i.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1j_7Version5parse0E0B1l_.exit.i.i.i: ; preds = %_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parse0B6_.exit.i.i.i.i, %bb3.i.i.i
  %90 = load i8, ptr %_54.sroa.7.0._6.sroa_idx.i.i, align 1, !range !54, !alias.scope !550, !noalias !498, !noundef !3
  %_2.i.i.i.i.i.i.i = trunc nuw i8 %90 to i1
  br i1 %_2.i.i.i.i.i.i.i, label %bb33.i.i, label %bb2.i.i.i.i.i.i.i

bb28.i.i:                                         ; preds = %_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parse0B6_.exit.i.i.i.i
  %_8.not.i.not.i.i = icmp eq i64 %_7.1.i.i.i.i.i, 9
  br i1 %_8.not.i.not.i.i, label %bb37.i.i, label %bb9.i.i.i

bb9.i.i.i:                                        ; preds = %bb28.i.i
  %91 = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i.i.i, i64 9
  %self1.i.i.i = load i8, ptr %91, align 1, !alias.scope !555, !noalias !558, !noundef !3
  %92 = icmp sgt i8 %self1.i.i.i, -65
  br i1 %92, label %bb37.i.i, label %bb31.i.i

bb33.i.i:                                         ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1j_7Version5parse0E0B1l_.exit.i.i.i, %_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next.exit.i.i.i, %_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread7.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_6.i.i), !noalias !498
  br label %bb17.i

bb31.i.i:                                         ; preds = %bb9.i.i.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_7.0.i.i.i.i.i, i64 noundef %_7.1.i.i.i.i.i, i64 noundef 9, i64 noundef %_7.1.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1a49731e8f6ceed5b16a30308d388517) #29
          to label %.noexc47.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc47.i:                                       ; preds = %bb31.i.i
  unreachable

bb37.i.i:                                         ; preds = %bb9.i.i.i, %bb28.i.i
  %new_len.i.i.i = add i64 %_7.1.i.i.i.i.i, -9
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %_7.0.i.i.i.i.i, i64 9
  %_74.sroa.4.0.release.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 8
  %_74.sroa.4.sroa.4.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 16
  store i64 %new_len.i.i.i, ptr %_74.sroa.4.sroa.4.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 24
  store ptr %data.i.i.i, ptr %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_74.sroa.4.sroa.5.sroa.4.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 32
  store i64 %new_len.i.i.i, ptr %_74.sroa.4.sroa.5.sroa.4.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_74.sroa.4.sroa.5.sroa.5.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 40
  store i64 0, ptr %_74.sroa.4.sroa.5.sroa.5.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_74.sroa.4.sroa.5.sroa.6.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 48
  store i64 %new_len.i.i.i, ptr %_74.sroa.4.sroa.5.sroa.6.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_74.sroa.4.sroa.5.sroa.7.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 56
  store <2 x i32> splat (i32 45), ptr %_74.sroa.4.sroa.5.sroa.7.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_74.sroa.4.sroa.5.sroa.9.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 64
  store i8 1, ptr %_74.sroa.4.sroa.5.sroa.9.0._74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_74.sroa.4.sroa.6.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 72
  store i8 1, ptr %_74.sroa.4.sroa.6.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %release.i.i, i64 73
  store i8 0, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !noalias !498
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_6.i.i), !noalias !498
  store <2 x i64> <i64 1, i64 0>, ptr %release.i.i, align 16, !noalias !498
  call void @llvm.experimental.noalias.scope.decl(metadata !559)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !562
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i) #28
          to label %.noexc48.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc48.i:                                       ; preds = %bb37.i.i
  %_7.i.i.i = load i64, ptr %_5.i.i.i, align 8, !range !37, !noalias !562, !noundef !3
  %93 = trunc nuw i64 %_7.i.i.i to i1
  br i1 %93, label %bb7.i.i.i, label %bb6.i51.i.i

bb7.i.i.i:                                        ; preds = %.noexc48.i
  %94 = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 8
  %a.i.i.i = load i64, ptr %94, align 8, !noalias !562, !noundef !3
  %95 = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 16
  %b.i.i.i = load i64, ptr %95, align 8, !noalias !562, !noundef !3
  %i.i.i.i = load i64, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !559, !noalias !498, !noundef !3
  %new_len.i54.i.i = sub nuw i64 %a.i.i.i, %i.i.i.i
  %data.i55.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i, i64 %i.i.i.i
  store i64 %b.i.i.i, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !559, !noalias !498
  br label %bb43.i.i

bb6.i51.i.i:                                      ; preds = %.noexc48.i
  %96 = load i8, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !563, !noalias !498, !noundef !3
  %_2.i.i.i.i = trunc nuw i8 %96 to i1
  br i1 %_2.i.i.i.i, label %bb42.i.i, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb6.i51.i.i
  store i8 1, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !563, !noalias !498
  %97 = load i8, ptr %_74.sroa.4.sroa.6.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !563, !noalias !498, !noundef !3
  %_3.i.i.i.i = trunc nuw i8 %97 to i1
  %i.pre.i.i.i.i = load i64, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !563, !noalias !498
  %i1.pre.i.i.i.i = load i64, ptr %_74.sroa.4.sroa.4.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !563, !noalias !498
  %_4.not.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  %or.cond.not.i.i.i.i = select i1 %_3.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i
  br i1 %or.cond.not.i.i.i.i, label %bb4.i.i.i.i, label %bb42.i.i

bb4.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  %_10.val.i.i.i.i = load ptr, ptr %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !563, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i, %i.pre.i.i.i.i
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i, i64 %i.pre.i.i.i.i
  br label %bb43.i.i

bb42.i.i:                                         ; preds = %bb1.i.i.i.i, %bb6.i51.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !562
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c6e9e0ae8c6937b3491e661c604ec907) #29
          to label %.noexc49.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc49.i:                                       ; preds = %bb42.i.i
  unreachable

bb43.i.i:                                         ; preds = %bb4.i.i.i.i, %bb7.i.i.i
  %i.pre.i97.i.i = phi i64 [ %b.i.i.i, %bb7.i.i.i ], [ %i.pre.i.i.i.i, %bb4.i.i.i.i ]
  %_0.sroa.4.0.i.i.i = phi i64 [ %new_len.i54.i.i, %bb7.i.i.i ], [ %new_len.i.i.i.i, %bb4.i.i.i.i ]
  %_0.sroa.0.0.i52.i.i = phi ptr [ %data.i55.i.i, %bb7.i.i.i ], [ %data.i.i.i.i, %bb4.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i), !noalias !562
  %98 = load i64, ptr %release.i.i, align 16, !noalias !498, !noundef !3
  switch i64 %98, label %bb45.i.i [
    i64 0, label %bb55.i.i
    i64 1, label %bb46.i.i
  ]

bb45.i.i:                                         ; preds = %bb43.i.i
  %99 = add i64 %98, -1
  store i64 %99, ptr %release.i.i, align 16, !noalias !498
  call void @llvm.experimental.noalias.scope.decl(metadata !566)
  %100 = load i8, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !566, !noalias !498, !noundef !3
  %_2.i63.i.i = trunc nuw i8 %100 to i1
  br i1 %_2.i63.i.i, label %bb44.i.i, label %bb2.i64.i.i

bb2.i64.i.i:                                      ; preds = %bb45.i.i
  %_4.val.i66.i.i = load ptr, ptr %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !566, !noalias !498, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i62.i.i), !noalias !569
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i62.i.i, ptr noalias noundef align 8 dereferenceable(48) %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i) #28
          to label %.noexc50.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc50.i:                                       ; preds = %bb2.i64.i.i
  %_7.i67.i.i = load i64, ptr %_5.i62.i.i, align 8, !range !37, !noalias !569, !noundef !3
  %101 = trunc nuw i64 %_7.i67.i.i to i1
  br i1 %101, label %bb7.i87.i.i, label %bb6.i68.i.i

bb7.i87.i.i:                                      ; preds = %.noexc50.i
  %102 = getelementptr inbounds nuw i8, ptr %_5.i62.i.i, i64 8
  %a.i88.i.i = load i64, ptr %102, align 8, !noalias !569, !noundef !3
  %103 = getelementptr inbounds nuw i8, ptr %_5.i62.i.i, i64 16
  %b.i89.i.i = load i64, ptr %103, align 8, !noalias !569, !noundef !3
  %i.i90.i.i = load i64, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !566, !noalias !498, !noundef !3
  %new_len.i91.i.i = sub nuw i64 %a.i88.i.i, %i.i90.i.i
  %data.i92.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i66.i.i, i64 %i.i90.i.i
  store i64 %b.i89.i.i, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !566, !noalias !498
  br label %bb8.i77.i.i

bb6.i68.i.i:                                      ; preds = %.noexc50.i
  %104 = load i8, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !570, !noalias !498, !noundef !3
  %_2.i.i69.i.i = trunc nuw i8 %104 to i1
  br i1 %_2.i.i69.i.i, label %bb8.i77.i.i, label %bb1.i.i70.i.i

bb1.i.i70.i.i:                                    ; preds = %bb6.i68.i.i
  store i8 1, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !570, !noalias !498
  %105 = load i8, ptr %_74.sroa.4.sroa.6.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !570, !noalias !498, !noundef !3
  %_3.i.i71.i.i = trunc nuw i8 %105 to i1
  %i.pre.i.i72.i.i = load i64, ptr %_74.sroa.4.0.release.sroa_idx.i.i, align 8, !alias.scope !570, !noalias !498
  %i1.pre.i.i74.i.i = load i64, ptr %_74.sroa.4.sroa.4.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !570, !noalias !498
  %_4.not.i.i75.i.i = icmp ne i64 %i1.pre.i.i74.i.i, %i.pre.i.i72.i.i
  %or.cond.not.i.i76.i.i = select i1 %_3.i.i71.i.i, i1 true, i1 %_4.not.i.i75.i.i
  br i1 %or.cond.not.i.i76.i.i, label %bb4.i.i83.i.i, label %bb8.i77.i.i

bb4.i.i83.i.i:                                    ; preds = %bb1.i.i70.i.i
  %_10.val.i.i84.i.i = load ptr, ptr %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !570, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i85.i.i = sub nuw i64 %i1.pre.i.i74.i.i, %i.pre.i.i72.i.i
  %data.i.i86.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i84.i.i, i64 %i.pre.i.i72.i.i
  br label %bb8.i77.i.i

bb8.i77.i.i:                                      ; preds = %bb4.i.i83.i.i, %bb1.i.i70.i.i, %bb6.i68.i.i, %bb7.i87.i.i
  %_0.sroa.4.0.i78.i.i = phi i64 [ %new_len.i91.i.i, %bb7.i87.i.i ], [ %new_len.i.i85.i.i, %bb4.i.i83.i.i ], [ undef, %bb6.i68.i.i ], [ undef, %bb1.i.i70.i.i ]
  %_0.sroa.0.0.i79.i.i = phi ptr [ %data.i92.i.i, %bb7.i87.i.i ], [ %data.i.i86.i.i, %bb4.i.i83.i.i ], [ null, %bb6.i68.i.i ], [ null, %bb1.i.i70.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i62.i.i), !noalias !569
  br label %bb44.i.i

bb46.i.i:                                         ; preds = %bb43.i.i
  store i64 0, ptr %release.i.i, align 16, !noalias !498
  %106 = load i8, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !573, !noalias !498, !noundef !3
  %_2.i94.i.i = trunc nuw i8 %106 to i1
  br i1 %_2.i94.i.i, label %bb44.i.i, label %bb1.i95.i.i

bb1.i95.i.i:                                      ; preds = %bb46.i.i
  store i8 1, ptr %_74.sroa.4.sroa.7.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !573, !noalias !498
  %107 = load i8, ptr %_74.sroa.4.sroa.6.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !573, !noalias !498, !noundef !3
  %_3.i96.i.i = trunc nuw i8 %107 to i1
  %i1.pre.i99.i.i = load i64, ptr %_74.sroa.4.sroa.4.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !573, !noalias !498
  %_4.not.i100.i.i = icmp ne i64 %i1.pre.i99.i.i, %i.pre.i97.i.i
  %or.cond.not.i101.i.i = select i1 %_3.i96.i.i, i1 true, i1 %_4.not.i100.i.i
  br i1 %or.cond.not.i101.i.i, label %bb4.i105.i.i, label %bb44.i.i

bb4.i105.i.i:                                     ; preds = %bb1.i95.i.i
  %_10.val.i107.i.i = load ptr, ptr %_74.sroa.4.sroa.5.0._74.sroa.4.0.release.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !573, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i108.i.i = sub nuw i64 %i1.pre.i99.i.i, %i.pre.i97.i.i
  %data.i109.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i107.i.i, i64 %i.pre.i97.i.i
  br label %bb44.i.i

bb44.i.i:                                         ; preds = %bb4.i105.i.i, %bb1.i95.i.i, %bb46.i.i, %bb8.i77.i.i, %bb45.i.i
  %_0.sroa.0.1.i82.pn.i.i = phi ptr [ %_0.sroa.0.0.i79.i.i, %bb8.i77.i.i ], [ null, %bb45.i.i ], [ %data.i109.i.i, %bb4.i105.i.i ], [ null, %bb46.i.i ], [ null, %bb1.i95.i.i ]
  %_0.sroa.4.1.i81.pn.i.i = phi i64 [ %_0.sroa.4.0.i78.i.i, %bb8.i77.i.i ], [ undef, %bb45.i.i ], [ %new_len.i108.i.i, %bb4.i105.i.i ], [ undef, %bb46.i.i ], [ undef, %bb1.i95.i.i ]
  %.not33.i.i = icmp eq ptr %_0.sroa.0.1.i82.pn.i.i, null
  %spec.select.i.i = select i1 %.not33.i.i, i64 0, i64 %_0.sroa.4.1.i81.pn.i.i
  %spec.select316.i.i = select i1 %.not33.i.i, ptr inttoptr (i64 1 to ptr), ptr %_0.sroa.0.1.i82.pn.i.i
  br label %bb55.i.i

bb55.i.i:                                         ; preds = %bb44.i.i, %bb43.i.i
  %channel.sroa.4.0.i.i = phi i64 [ %98, %bb43.i.i ], [ %spec.select.i.i, %bb44.i.i ]
  %channel.sroa.0.0.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb43.i.i ], [ %spec.select316.i.i, %bb44.i.i ]
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %digits.i.i), !noalias !498
  %_86.sroa.4.0.digits.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 8
  %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 16
  store i64 %_0.sroa.4.0.i.i.i, ptr %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 24
  store ptr %_0.sroa.0.0.i52.i.i, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_86.sroa.4.sroa.5.sroa.4.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 32
  store i64 %_0.sroa.4.0.i.i.i, ptr %_86.sroa.4.sroa.5.sroa.4.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_86.sroa.4.sroa.5.sroa.5.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 40
  store i64 0, ptr %_86.sroa.4.sroa.5.sroa.5.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_86.sroa.4.sroa.5.sroa.6.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 48
  store i64 %_0.sroa.4.0.i.i.i, ptr %_86.sroa.4.sroa.5.sroa.6.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_86.sroa.4.sroa.5.sroa.7.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 56
  store <2 x i32> splat (i32 46), ptr %_86.sroa.4.sroa.5.sroa.7.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_86.sroa.4.sroa.5.sroa.9.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 64
  store i8 1, ptr %_86.sroa.4.sroa.5.sroa.9.0._86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.sroa_idx.i.i, align 16, !noalias !498
  %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 72
  store i8 1, ptr %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !noalias !498
  %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %digits.i.i, i64 73
  store i8 0, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !noalias !498
  store <2 x i64> <i64 2, i64 0>, ptr %digits.i.i, align 16, !noalias !498
  call void @llvm.experimental.noalias.scope.decl(metadata !576)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i111.i.i), !noalias !579
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i111.i.i, ptr noalias noundef align 8 dereferenceable(48) %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i) #28
          to label %.noexc51.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc51.i:                                       ; preds = %bb55.i.i
  %_7.i116.i.i = load i64, ptr %_5.i111.i.i, align 8, !range !37, !noalias !579, !noundef !3
  %108 = trunc nuw i64 %_7.i116.i.i to i1
  br i1 %108, label %bb8.i126.i.i, label %bb6.i117.i.i

bb6.i117.i.i:                                     ; preds = %.noexc51.i
  %109 = load i8, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !580, !noalias !498, !noundef !3
  %_2.i.i118.i.i = trunc nuw i8 %109 to i1
  br i1 %_2.i.i118.i.i, label %bb8.i126.thread.i.i, label %bb1.i.i119.i.i

bb1.i.i119.i.i:                                   ; preds = %bb6.i117.i.i
  store i8 1, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !580, !noalias !498
  %110 = load i8, ptr %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !580, !noalias !498, !noundef !3
  %_3.i.i120.i.i = trunc nuw i8 %110 to i1
  %i.pre.i.i121.i.i = load i64, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !580, !noalias !498
  %i1.pre.i.i123.i.i = load i64, ptr %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !580, !noalias !498
  %_4.not.i.i124.i.i = icmp ne i64 %i1.pre.i.i123.i.i, %i.pre.i.i121.i.i
  %or.cond.not.i.i125.i.i = select i1 %_3.i.i120.i.i, i1 true, i1 %_4.not.i.i124.i.i
  br i1 %or.cond.not.i.i125.i.i, label %bb8.i126.thread340.i.i, label %bb8.i126.thread.i.i

bb8.i126.thread340.i.i:                           ; preds = %bb1.i.i119.i.i
  %_10.val.i.i133.i.i = load ptr, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !580, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i134.i.i = sub nuw i64 %i1.pre.i.i123.i.i, %i.pre.i.i121.i.i
  %data.i.i135.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i133.i.i, i64 %i.pre.i.i121.i.i
  br label %bb61.i.i

bb8.i126.thread.i.i:                              ; preds = %bb1.i.i119.i.i, %bb6.i117.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i111.i.i), !noalias !579
  br label %bb21.i.i

bb8.i126.i.i:                                     ; preds = %.noexc51.i
  %111 = getelementptr inbounds nuw i8, ptr %_5.i111.i.i, i64 8
  %a.i137.i.i = load i64, ptr %111, align 8, !noalias !579, !noundef !3
  %112 = getelementptr inbounds nuw i8, ptr %_5.i111.i.i, i64 16
  %b.i138.i.i = load i64, ptr %112, align 8, !noalias !579, !noundef !3
  %i.i139.i.i = load i64, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !576, !noalias !498, !noundef !3
  %new_len.i140.i.i = sub nuw i64 %a.i137.i.i, %i.i139.i.i
  %data.i141.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i52.i.i, i64 %i.i139.i.i
  store i64 %b.i138.i.i, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !576, !noalias !498
  br label %bb61.i.i

bb61.i.i:                                         ; preds = %bb8.i126.i.i, %bb8.i126.thread340.i.i
  %i.pre.i197.i.i = phi i64 [ %i.pre.i.i121.i.i, %bb8.i126.thread340.i.i ], [ %b.i138.i.i, %bb8.i126.i.i ]
  %_0.sroa.0.0.i128345.i.i = phi ptr [ %data.i.i135.i.i, %bb8.i126.thread340.i.i ], [ %data.i141.i.i, %bb8.i126.i.i ]
  %_0.sroa.4.0.i127344.i.i = phi i64 [ %new_len.i.i134.i.i, %bb8.i126.thread340.i.i ], [ %new_len.i140.i.i, %bb8.i126.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i111.i.i), !noalias !579
  %_3.not.i.i.i = icmp eq i64 %_0.sroa.4.0.i127344.i.i, 1
  br i1 %_3.not.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build.exit.i.i, label %bb21.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build.exit.i.i: ; preds = %bb61.i.i
  %lhsc.i.i = load i8, ptr %_0.sroa.0.0.i128345.i.i, align 1, !noalias !558
  %113 = icmp eq i8 %lhsc.i.i, 49
  br i1 %113, label %bb3.i42.i, label %bb21.i.i

bb21.i.i:                                         ; preds = %bb33.i.i.i, %bb23.i.i.i, %bb16.i.i.i, %bb33.i253.i.i, %bb23.i248.i.i, %bb16.i265.i.i, %bb7.i232.i.i, %bb7.i232.i.i, %bb80.i.i, %bb7.i211.i.i, %bb7.i211.i.i, %bb70.i.i, %bb63.thread356.i.i, %bb1.i195.i.i, %bb65.i.i, %bb64.i.i, %bb3.i42.i, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build.exit.i.i, %bb61.i.i, %bb8.i126.thread.i.i
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %digits.i.i), !noalias !498
  br label %bb17.i

bb3.i42.i:                                        ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build.exit.i.i
  %114 = load i64, ptr %digits.i.i, align 16, !noalias !498, !noundef !3
  switch i64 %114, label %bb64.i.i [
    i64 0, label %bb21.i.i
    i64 1, label %bb65.i.i
  ]

bb64.i.i:                                         ; preds = %bb3.i42.i
  %115 = add i64 %114, -1
  store i64 %115, ptr %digits.i.i, align 16, !noalias !498
  call void @llvm.experimental.noalias.scope.decl(metadata !583)
  %116 = load i8, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !583, !noalias !498, !noundef !3
  %_2.i163.i.i = trunc nuw i8 %116 to i1
  br i1 %_2.i163.i.i, label %bb21.i.i, label %bb2.i164.i.i

bb2.i164.i.i:                                     ; preds = %bb64.i.i
  %_4.val.i166.i.i = load ptr, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !583, !noalias !498, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i162.i.i), !noalias !586
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i162.i.i, ptr noalias noundef align 8 dereferenceable(48) %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i) #28
          to label %.noexc52.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc52.i:                                       ; preds = %bb2.i164.i.i
  %_7.i167.i.i = load i64, ptr %_5.i162.i.i, align 8, !range !37, !noalias !586, !noundef !3
  %117 = trunc nuw i64 %_7.i167.i.i to i1
  br i1 %117, label %bb7.i187.i.i, label %bb6.i168.i.i

bb7.i187.i.i:                                     ; preds = %.noexc52.i
  %118 = getelementptr inbounds nuw i8, ptr %_5.i162.i.i, i64 8
  %a.i188.i.i = load i64, ptr %118, align 8, !noalias !586, !noundef !3
  %119 = getelementptr inbounds nuw i8, ptr %_5.i162.i.i, i64 16
  %b.i189.i.i = load i64, ptr %119, align 8, !noalias !586, !noundef !3
  %i.i190.i.i = load i64, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !583, !noalias !498, !noundef !3
  %new_len.i191.i.i = sub nuw i64 %a.i188.i.i, %i.i190.i.i
  %data.i192.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i166.i.i, i64 %i.i190.i.i
  store i64 %b.i189.i.i, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !583, !noalias !498
  br label %bb63.i.i

bb6.i168.i.i:                                     ; preds = %.noexc52.i
  %120 = load i8, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !587, !noalias !498, !noundef !3
  %_2.i.i169.i.i = trunc nuw i8 %120 to i1
  br i1 %_2.i.i169.i.i, label %bb63.thread356.i.i, label %bb1.i.i170.i.i

bb1.i.i170.i.i:                                   ; preds = %bb6.i168.i.i
  store i8 1, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !587, !noalias !498
  %121 = load i8, ptr %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !587, !noalias !498, !noundef !3
  %_3.i.i171.i.i = trunc nuw i8 %121 to i1
  %i.pre.i.i172.i.i = load i64, ptr %_86.sroa.4.0.digits.sroa_idx.i.i, align 8, !alias.scope !587, !noalias !498
  %i1.pre.i.i174.i.i = load i64, ptr %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !587, !noalias !498
  %_4.not.i.i175.i.i = icmp ne i64 %i1.pre.i.i174.i.i, %i.pre.i.i172.i.i
  %or.cond.not.i.i176.i.i = select i1 %_3.i.i171.i.i, i1 true, i1 %_4.not.i.i175.i.i
  br i1 %or.cond.not.i.i176.i.i, label %bb4.i.i183.i.i, label %bb63.thread356.i.i

bb4.i.i183.i.i:                                   ; preds = %bb1.i.i170.i.i
  %_10.val.i.i184.i.i = load ptr, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !587, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i185.i.i = sub nuw i64 %i1.pre.i.i174.i.i, %i.pre.i.i172.i.i
  %data.i.i186.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i184.i.i, i64 %i.pre.i.i172.i.i
  br label %bb63.i.i

bb65.i.i:                                         ; preds = %bb3.i42.i
  store i64 0, ptr %digits.i.i, align 16, !noalias !498
  %122 = load i8, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !590, !noalias !498, !noundef !3
  %_2.i194.i.i = trunc nuw i8 %122 to i1
  br i1 %_2.i194.i.i, label %bb21.i.i, label %bb1.i195.i.i

bb1.i195.i.i:                                     ; preds = %bb65.i.i
  store i8 1, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !590, !noalias !498
  %123 = load i8, ptr %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !590, !noalias !498, !noundef !3
  %_3.i196.i.i = trunc nuw i8 %123 to i1
  %i1.pre.i199.i.i = load i64, ptr %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !590, !noalias !498
  %_4.not.i200.i.i = icmp ne i64 %i1.pre.i199.i.i, %i.pre.i197.i.i
  %or.cond.not.i201.i.i = select i1 %_3.i196.i.i, i1 true, i1 %_4.not.i200.i.i
  br i1 %or.cond.not.i201.i.i, label %bb63.thread349.i.i, label %bb21.i.i

bb63.thread349.i.i:                               ; preds = %bb1.i195.i.i
  %_10.val.i207.i.i = load ptr, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !590, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i208.i.i = sub nuw i64 %i1.pre.i199.i.i, %i.pre.i197.i.i
  %data.i209.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i207.i.i, i64 %i.pre.i197.i.i
  br label %bb70.i.i

bb63.thread356.i.i:                               ; preds = %bb1.i.i170.i.i, %bb6.i168.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i162.i.i), !noalias !586
  br label %bb21.i.i

bb63.i.i:                                         ; preds = %bb4.i.i183.i.i, %bb7.i187.i.i
  %i.pre.i218.i111.i = phi i64 [ %b.i189.i.i, %bb7.i187.i.i ], [ %i.pre.i.i172.i.i, %bb4.i.i183.i.i ]
  %_0.sroa.4.0.i178.i.i = phi i64 [ %new_len.i191.i.i, %bb7.i187.i.i ], [ %new_len.i.i185.i.i, %bb4.i.i183.i.i ]
  %_0.sroa.0.0.i179.i.i = phi ptr [ %data.i192.i.i, %bb7.i187.i.i ], [ %data.i.i186.i.i, %bb4.i.i183.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i162.i.i), !noalias !586
  br label %bb70.i.i

bb70.i.i:                                         ; preds = %bb63.i.i, %bb63.thread349.i.i
  %i.pre.i218.i.i = phi i64 [ %i.pre.i197.i.i, %bb63.thread349.i.i ], [ %i.pre.i218.i111.i, %bb63.i.i ]
  %_0.sroa.4.1.i181.pn354.i.i = phi i64 [ %new_len.i208.i.i, %bb63.thread349.i.i ], [ %_0.sroa.4.0.i178.i.i, %bb63.i.i ]
  %_0.sroa.0.1.i182.pn353.i.i = phi ptr [ %data.i209.i.i, %bb63.thread349.i.i ], [ %_0.sroa.0.0.i179.i.i, %bb63.i.i ]
  switch i64 %_0.sroa.4.1.i181.pn354.i.i, label %bb9thread-pre-split.i.i.i [
    i64 0, label %bb21.i.i
    i64 1, label %bb7.i211.i.i
  ]

bb7.i211.i.i:                                     ; preds = %bb70.i.i
  %124 = load i8, ptr %_0.sroa.0.1.i182.pn353.i.i, align 1, !alias.scope !593, !noalias !558, !noundef !3
  switch i8 %124, label %bb9.i213.i.i [
    i8 43, label %bb21.i.i
    i8 45, label %bb21.i.i
  ]

bb9thread-pre-split.i.i.i:                        ; preds = %bb70.i.i
  %.pr.i.i.i = load i8, ptr %_0.sroa.0.1.i182.pn353.i.i, align 1, !alias.scope !593, !noalias !558
  br label %bb9.i213.i.i

bb9.i213.i.i:                                     ; preds = %bb9thread-pre-split.i.i.i, %bb7.i211.i.i
  %125 = phi i8 [ %.pr.i.i.i, %bb9thread-pre-split.i.i.i ], [ %124, %bb7.i211.i.i ]
  %cond.i.i.i = icmp eq i8 %125, 43
  %rest.1.i.i.i = sext i1 %cond.i.i.i to i64
  %src.sroa.15.0.i.i.i = add nsw i64 %_0.sroa.4.1.i181.pn354.i.i, %rest.1.i.i.i
  %src.sroa.0.0.idx.i.i.i = zext i1 %cond.i.i.i to i64
  %src.sroa.0.0.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i182.pn353.i.i, i64 %src.sroa.0.0.idx.i.i.i
  %_10.i214.i.i = icmp samesign ult i64 %src.sroa.15.0.i.i.i, 9
  br i1 %_10.i214.i.i, label %bb15.preheader.i.i.i, label %bb22.i.i.i

bb15.preheader.i.i.i:                             ; preds = %bb9.i213.i.i
  %_13.not56.i.i.i = icmp eq i64 %src.sroa.15.0.i.i.i, 0
  br i1 %_13.not56.i.i.i, label %bb73.i.i, label %bb16.i.i.i

bb22.i.i.i:                                       ; preds = %bb9.i213.i.i, %bb33.i.i.i
  %result.sroa.0.0.i.i.i = phi i32 [ %_60.0.i.i.i, %bb33.i.i.i ], [ 0, %bb9.i213.i.i ]
  %src.sroa.15.1.i.i.i = phi i64 [ %rest.12.i.i.i, %bb33.i.i.i ], [ %src.sroa.15.0.i.i.i, %bb9.i213.i.i ]
  %src.sroa.0.1.i.i.i = phi ptr [ %rest.01.i.i.i, %bb33.i.i.i ], [ %src.sroa.0.0.i.i.i, %bb9.i213.i.i ]
  %_28.not.i.not.i.i = icmp eq i64 %src.sroa.15.1.i.i.i, 0
  br i1 %_28.not.i.not.i.i, label %bb73.i.i, label %bb23.i.i.i

bb23.i.i.i:                                       ; preds = %bb22.i.i.i
  %126 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i.i.i, i32 10)
  %_57.1.i.i.i = extractvalue { i32, i1 } %126, 1
  br i1 %_57.1.i.i.i, label %bb21.i.i, label %bb33.i.i.i, !prof !102

bb33.i.i.i:                                       ; preds = %bb23.i.i.i
  %_57.0.i.i.i = extractvalue { i32, i1 } %126, 0
  %rest.12.i.i.i = add nsw i64 %src.sroa.15.1.i.i.i, -1
  %rest.01.i.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i.i, i64 1
  %127 = load i8, ptr %src.sroa.0.1.i.i.i, align 1, !alias.scope !593, !noalias !558, !noundef !3
  %128 = zext i8 %127 to i32
  %129 = add nsw i32 %128, -48
  %_14.i.i.i.i = icmp ugt i32 %129, 9
  %_60.0.i.i.i = add i32 %129, %_57.0.i.i.i
  %_60.1.i.i.i = icmp ult i32 %_60.0.i.i.i, %_57.0.i.i.i
  %or.cond.i.i = select i1 %_14.i.i.i.i, i1 true, i1 %_60.1.i.i.i
  br i1 %or.cond.i.i, label %bb21.i.i, label %bb22.i.i.i, !prof !103

bb16.i.i.i:                                       ; preds = %bb15.preheader.i.i.i, %bb20.i.i.i
  %src.sroa.0.259.i.i.i = phi ptr [ %rest.04.i.i.i, %bb20.i.i.i ], [ %src.sroa.0.0.i.i.i, %bb15.preheader.i.i.i ]
  %src.sroa.15.258.i.i.i = phi i64 [ %rest.15.i.i.i, %bb20.i.i.i ], [ %src.sroa.15.0.i.i.i, %bb15.preheader.i.i.i ]
  %result.sroa.0.257.i.i.i = phi i32 [ %132, %bb20.i.i.i ], [ 0, %bb15.preheader.i.i.i ]
  %_19.i.i.i = load i8, ptr %src.sroa.0.259.i.i.i, align 1, !alias.scope !593, !noalias !558, !noundef !3
  %_18.i.i.i = zext i8 %_19.i.i.i to i32
  %130 = add nsw i32 %_18.i.i.i, -48
  %_14.i47.i.i.i = icmp ugt i32 %130, 9
  br i1 %_14.i47.i.i.i, label %bb21.i.i, label %bb20.i.i.i

bb20.i.i.i:                                       ; preds = %bb16.i.i.i
  %131 = mul i32 %result.sroa.0.257.i.i.i, 10
  %rest.15.i.i.i = add nsw i64 %src.sroa.15.258.i.i.i, -1
  %rest.04.i.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i.i, i64 1
  %132 = add i32 %130, %131
  %_13.not.i.i.i = icmp eq i64 %rest.15.i.i.i, 0
  br i1 %_13.not.i.i.i, label %bb73.i.i, label %bb16.i.i.i

bb73.i.i:                                         ; preds = %bb22.i.i.i, %bb20.i.i.i, %bb15.preheader.i.i.i
  %_0.sroa.8.0.insert.insert.i.i.i = phi i32 [ 0, %bb15.preheader.i.i.i ], [ %132, %bb20.i.i.i ], [ %result.sroa.0.0.i.i.i, %bb22.i.i.i ]
  %133 = load i64, ptr %digits.i.i, align 16, !noalias !498, !noundef !3
  switch i64 %133, label %bb75.i.i [
    i64 0, label %bb7.i232.i.i
    i64 1, label %bb76.i.i
  ]

bb75.i.i:                                         ; preds = %bb73.i.i
  %134 = add i64 %133, -1
  store i64 %134, ptr %digits.i.i, align 16, !noalias !498
; invoke <core::str::iter::SplitInternal<char>>::next
  %135 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %_86.sroa.4.0.digits.sroa_idx.i.i) #28
          to label %bb74.i.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

bb76.i.i:                                         ; preds = %bb73.i.i
  store i64 0, ptr %digits.i.i, align 16, !noalias !498
  %136 = load i8, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !range !54, !alias.scope !596, !noalias !498, !noundef !3
  %_2.i215.i.i = trunc nuw i8 %136 to i1
  br i1 %_2.i215.i.i, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i, label %bb1.i216.i.i

bb1.i216.i.i:                                     ; preds = %bb76.i.i
  store i8 1, ptr %_86.sroa.4.sroa.7.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 1, !alias.scope !596, !noalias !498
  %137 = load i8, ptr %_86.sroa.4.sroa.6.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !range !54, !alias.scope !596, !noalias !498, !noundef !3
  %_3.i217.i.i = trunc nuw i8 %137 to i1
  %i1.pre.i220.i.i = load i64, ptr %_86.sroa.4.sroa.4.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 16, !alias.scope !596, !noalias !498
  %_4.not.i221.i.i = icmp ne i64 %i1.pre.i220.i.i, %i.pre.i218.i.i
  %or.cond.not.i222.i.i = select i1 %_3.i217.i.i, i1 true, i1 %_4.not.i221.i.i
  br i1 %or.cond.not.i222.i.i, label %bb4.i226.i.i, label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i

bb4.i226.i.i:                                     ; preds = %bb1.i216.i.i
  %_10.val.i228.i.i = load ptr, ptr %_86.sroa.4.sroa.5.0._86.sroa.4.0.digits.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !596, !noalias !498, !nonnull !3, !align !18, !noundef !3
  %new_len.i229.i.i = sub nuw i64 %i1.pre.i220.i.i, %i.pre.i218.i.i
  %data.i230.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i228.i.i, i64 %i.pre.i218.i.i
  br label %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i

_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i: ; preds = %bb4.i226.i.i, %bb1.i216.i.i, %bb76.i.i
  %_0.sroa.3.0.i224.i.i = phi i64 [ %new_len.i229.i.i, %bb4.i226.i.i ], [ undef, %bb76.i.i ], [ undef, %bb1.i216.i.i ]
  %_0.sroa.0.0.i225.i.i = phi ptr [ %data.i230.i.i, %bb4.i226.i.i ], [ null, %bb76.i.i ], [ null, %bb1.i216.i.i ]
  %138 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i225.i.i, 0
  %139 = insertvalue { ptr, i64 } %138, i64 %_0.sroa.3.0.i224.i.i, 1
  br label %bb74.i.i

bb74.i.i:                                         ; preds = %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i, %bb75.i.i
  %.pn40.i.i = phi { ptr, i64 } [ %139, %_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build.exit231.i.i ], [ %135, %bb75.i.i ]
  %_28.sroa.0.0.i.i = extractvalue { ptr, i64 } %.pn40.i.i, 0
  %.not42.i.i = icmp eq ptr %_28.sroa.0.0.i.i, null
  %_28.sroa.6.0.i.i = extractvalue { ptr, i64 } %.pn40.i.i, 1
  br i1 %.not42.i.i, label %bb7.i232.i.i, label %bb80.i.i

bb80.i.i:                                         ; preds = %bb74.i.i
  switch i64 %_28.sroa.6.0.i.i, label %bb9thread-pre-split.i276.i.i [
    i64 0, label %bb21.i.i
    i64 1, label %bb7.i232.i.i
  ]

bb7.i232.i.i:                                     ; preds = %bb80.i.i, %bb74.i.i, %bb73.i.i
  %_27.sroa.0.0307.i.i = phi ptr [ %_28.sroa.0.0.i.i, %bb80.i.i ], [ @alloc_dda1ee2b88b89b9cdac753eef7988035, %bb73.i.i ], [ @alloc_dda1ee2b88b89b9cdac753eef7988035, %bb74.i.i ]
  %140 = load i8, ptr %_27.sroa.0.0307.i.i, align 1, !alias.scope !599, !noalias !558, !noundef !3
  switch i8 %140, label %bb9.i236.i.i [
    i8 43, label %bb21.i.i
    i8 45, label %bb21.i.i
  ]

bb9thread-pre-split.i276.i.i:                     ; preds = %bb80.i.i
  %.pr.i277.i.i = load i8, ptr %_28.sroa.0.0.i.i, align 1, !alias.scope !599, !noalias !558
  br label %bb9.i236.i.i

bb9.i236.i.i:                                     ; preds = %bb9thread-pre-split.i276.i.i, %bb7.i232.i.i
  %_27.sroa.3.0309.i.i = phi i64 [ %_28.sroa.6.0.i.i, %bb9thread-pre-split.i276.i.i ], [ 1, %bb7.i232.i.i ]
  %_27.sroa.0.0308.i.i = phi ptr [ %_28.sroa.0.0.i.i, %bb9thread-pre-split.i276.i.i ], [ %_27.sroa.0.0307.i.i, %bb7.i232.i.i ]
  %141 = phi i8 [ %.pr.i277.i.i, %bb9thread-pre-split.i276.i.i ], [ %140, %bb7.i232.i.i ]
  %cond.i237.i.i = icmp eq i8 %141, 43
  %rest.1.i238.i.i = sext i1 %cond.i237.i.i to i64
  %src.sroa.15.0.i239.i.i = add nsw i64 %_27.sroa.3.0309.i.i, %rest.1.i238.i.i
  %src.sroa.0.0.idx.i240.i.i = zext i1 %cond.i237.i.i to i64
  %src.sroa.0.0.i241.i.i = getelementptr inbounds nuw i8, ptr %_27.sroa.0.0308.i.i, i64 %src.sroa.0.0.idx.i240.i.i
  %_10.i242.i.i = icmp samesign ult i64 %src.sroa.15.0.i239.i.i, 9
  br i1 %_10.i242.i.i, label %bb15.preheader.i263.i.i, label %bb22.i243.i.i

bb15.preheader.i263.i.i:                          ; preds = %bb9.i236.i.i
  %_13.not56.i264.i.i = icmp eq i64 %src.sroa.15.0.i239.i.i, 0
  br i1 %_13.not56.i264.i.i, label %bb85.i.i, label %bb16.i265.i.i

bb22.i243.i.i:                                    ; preds = %bb9.i236.i.i, %bb33.i253.i.i
  %result.sroa.0.0.i244.i.i = phi i32 [ %_60.0.i256.i.i, %bb33.i253.i.i ], [ 0, %bb9.i236.i.i ]
  %src.sroa.15.1.i245.i.i = phi i64 [ %rest.12.i250.i.i, %bb33.i253.i.i ], [ %src.sroa.15.0.i239.i.i, %bb9.i236.i.i ]
  %src.sroa.0.1.i246.i.i = phi ptr [ %rest.01.i249.i.i, %bb33.i253.i.i ], [ %src.sroa.0.0.i241.i.i, %bb9.i236.i.i ]
  %_28.not.i247.i.i = icmp eq i64 %src.sroa.15.1.i245.i.i, 0
  br i1 %_28.not.i247.i.i, label %bb85.i.i, label %bb23.i248.i.i

bb23.i248.i.i:                                    ; preds = %bb22.i243.i.i
  %142 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i244.i.i, i32 10)
  %_57.1.i252.i.i = extractvalue { i32, i1 } %142, 1
  br i1 %_57.1.i252.i.i, label %bb21.i.i, label %bb33.i253.i.i, !prof !102

bb33.i253.i.i:                                    ; preds = %bb23.i248.i.i
  %_57.0.i251.i.i = extractvalue { i32, i1 } %142, 0
  %rest.12.i250.i.i = add nsw i64 %src.sroa.15.1.i245.i.i, -1
  %rest.01.i249.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i246.i.i, i64 1
  %143 = load i8, ptr %src.sroa.0.1.i246.i.i, align 1, !alias.scope !599, !noalias !558, !noundef !3
  %144 = zext i8 %143 to i32
  %145 = add nsw i32 %144, -48
  %_14.i.i254.i.i = icmp ugt i32 %145, 9
  %_60.0.i256.i.i = add i32 %145, %_57.0.i251.i.i
  %_60.1.i257.i.i = icmp ult i32 %_60.0.i256.i.i, %_57.0.i251.i.i
  %or.cond317.i.i = select i1 %_14.i.i254.i.i, i1 true, i1 %_60.1.i257.i.i
  br i1 %or.cond317.i.i, label %bb21.i.i, label %bb22.i243.i.i, !prof !103

bb16.i265.i.i:                                    ; preds = %bb15.preheader.i263.i.i, %bb20.i272.i.i
  %src.sroa.0.259.i266.i.i = phi ptr [ %rest.04.i274.i.i, %bb20.i272.i.i ], [ %src.sroa.0.0.i241.i.i, %bb15.preheader.i263.i.i ]
  %src.sroa.15.258.i267.i.i = phi i64 [ %rest.15.i273.i.i, %bb20.i272.i.i ], [ %src.sroa.15.0.i239.i.i, %bb15.preheader.i263.i.i ]
  %_19.i269.i.i = load i8, ptr %src.sroa.0.259.i266.i.i, align 1, !alias.scope !599, !noalias !558, !noundef !3
  %146 = add i8 %_19.i269.i.i, -48
  %_14.i47.i271.i.i = icmp ult i8 %146, 10
  br i1 %_14.i47.i271.i.i, label %bb20.i272.i.i, label %bb21.i.i

bb20.i272.i.i:                                    ; preds = %bb16.i265.i.i
  %rest.15.i273.i.i = add nsw i64 %src.sroa.15.258.i267.i.i, -1
  %rest.04.i274.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i266.i.i, i64 1
  %_13.not.i275.i.i = icmp eq i64 %rest.15.i273.i.i, 0
  br i1 %_13.not.i275.i.i, label %bb85.i.i, label %bb16.i265.i.i

bb85.i.i:                                         ; preds = %bb22.i243.i.i, %bb20.i272.i.i, %bb15.preheader.i263.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_31.i.i), !noalias !498
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_31.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d563101362ed4a06747b9210d55c4c5b, i64 noundef 15)
          to label %.noexc54.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc54.i:                                       ; preds = %bb85.i.i
  %147 = load i64, ptr %_31.i.i, align 8, !range !11, !noalias !498, !noundef !3
  %.not43.i.i = icmp ne i64 %147, -9223372036854775808
  %148 = getelementptr inbounds nuw i8, ptr %_31.i.i, i64 16
  %_123.i.i = load i64, ptr %148, align 8, !noalias !498
  %_3.not.i279.i.i = icmp eq i64 %_123.i.i, 2
  %or.cond370.i.i = select i1 %.not43.i.i, i1 %_3.not.i279.i.i, i1 false
  br i1 %or.cond370.i.i, label %bb87.i.i, label %bb5.i43.i

bb5.i43.i:                                        ; preds = %bb87.i.i, %.noexc54.i
  switch i64 %channel.sroa.4.0.i.i, label %bb12.i44.i [
    i64 7, label %bb86.i.i
    i64 3, label %bb2.i296.i.i
  ]

bb87.i.i:                                         ; preds = %.noexc54.i
  %149 = getelementptr inbounds nuw i8, ptr %_31.i.i, i64 8
  %_124.i.i = load ptr, ptr %149, align 8, !noalias !498, !nonnull !3, !noundef !3
  %150 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_124.i.i, ptr noundef nonnull dereferenceable(2) @alloc_24b05860d599286ecb26af7d40c66ae9, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !602, !noalias !558
  %151 = icmp eq i32 %150, 0
  br i1 %151, label %bb12.i44.i, label %bb5.i43.i

bb12.i44.i:                                       ; preds = %bb2.i296.i.i, %bb86.i.i, %bb87.i.i, %bb5.i43.i
  %nightly.sroa.0.0.off0.i.i = phi i1 [ false, %bb87.i.i ], [ %160, %bb2.i296.i.i ], [ %158, %bb86.i.i ], [ false, %bb5.i43.i ]
  switch i64 %147, label %bb2.i.i.i4.i.i.i.i291.i.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i
  ]

bb2.i.i.i4.i.i.i.i291.i.i:                        ; preds = %bb12.i44.i
  %152 = getelementptr inbounds nuw i8, ptr %_31.i.i, i64 8
  %_31.val47.i.i = load ptr, ptr %152, align 8, !noalias !498, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.val47.i.i, i64 noundef %147, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !558
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i: ; preds = %bb2.i.i.i4.i.i.i.i291.i.i, %bb12.i44.i, %bb12.i44.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_31.i.i), !noalias !498
; invoke <build_script_build::version::Version>::parse::{closure#2}
  %153 = invoke fastcc { i32, i32 } @_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parses0_0B6_(ptr nonnull %verbose_version.i.i) #28
          to label %.noexc55.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc55.i:                                       ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECslwKqnJYeWCA_18build_script_build.exit292.i.i
  %154 = extractvalue { i32, i32 } %153, 0
  %155 = trunc i32 %154 to i1
  %156 = extractvalue { i32, i32 } %153, 1
  %llvm_major.sroa.0.0.i.i = select i1 %155, i32 %156, i32 0
  br i1 %nightly.sroa.0.0.off0.i.i, label %bb15.i.i, label %bb18.i.i

bb86.i.i:                                         ; preds = %bb5.i43.i
  %157 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %channel.sroa.0.0.i.i, ptr noundef nonnull dereferenceable(7) @alloc_22ec252afd5f5781ca8ee9b115d4a0d6, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !606, !noalias !558
  %158 = icmp eq i32 %157, 0
  br label %bb12.i44.i

bb2.i296.i.i:                                     ; preds = %bb5.i43.i
  %159 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %channel.sroa.0.0.i.i, ptr noundef nonnull dereferenceable(3) @alloc_12dcbe319bdb437b2d068742d0ee3321, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !610, !noalias !558
  %160 = icmp eq i32 %159, 0
  br label %bb12.i44.i

bb15.i.i:                                         ; preds = %.noexc55.i
; invoke <build_script_build::version::Version>::parse::{closure#3}
  %161 = invoke fastcc i48 @_RNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB4_7Version5parses1_0B6_(ptr nonnull %verbose_version.i.i) #28
          to label %.noexc56.i unwind label %cleanup5.loopexit.split-lp.i, !noalias !409

.noexc56.i:                                       ; preds = %bb15.i.i
  %162 = trunc i48 %161 to i1
  %.sroa.026.2.extract.shift.i.i = lshr i48 %161, 16
  %.sroa.026.2.extract.trunc.i.i = trunc nuw i48 %.sroa.026.2.extract.shift.i.i to i32
  %_48.sroa.0.0.i.i = select i1 %162, i32 %.sroa.026.2.extract.trunc.i.i, i32 0
  br label %bb18.i.i

bb18.i.i:                                         ; preds = %.noexc56.i, %.noexc55.i
  %.sink333.i.i = phi i32 [ %_48.sroa.0.0.i.i, %.noexc56.i ], [ 0, %.noexc55.i ]
  %.sink.i45.i = phi i8 [ 1, %.noexc56.i ], [ 0, %.noexc55.i ]
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %digits.i.i), !noalias !498
  br label %bb17.i

bb17.i:                                           ; preds = %bb18.i.i, %bb21.i.i, %bb33.i.i
  %_18.sroa.6.0 = phi i8 [ 2, %bb33.i.i ], [ 2, %bb21.i.i ], [ %.sink.i45.i, %bb18.i.i ]
  %_18.sroa.5.0 = phi i32 [ undef, %bb33.i.i ], [ undef, %bb21.i.i ], [ %llvm_major.sroa.0.0.i.i, %bb18.i.i ]
  %_18.sroa.4.0 = phi i32 [ undef, %bb33.i.i ], [ undef, %bb21.i.i ], [ %.sink333.i.i, %bb18.i.i ]
  %_18.sroa.0.0 = phi i32 [ undef, %bb33.i.i ], [ undef, %bb21.i.i ], [ %_0.sroa.8.0.insert.insert.i.i.i, %bb18.i.i ]
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %release.i.i), !noalias !498
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %verbose_version.i.i), !noalias !409
  call void @llvm.experimental.noalias.scope.decl(metadata !614)
  %163 = icmp eq i64 %41, 0
  br i1 %163, label %bb4.i60.i, label %bb2.i.i.i4.i.i58.i

bb2.i.i.i4.i.i58.i:                               ; preds = %bb17.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_53.i, i64 noundef %41, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !617
  br label %bb4.i60.i

bb4.i60.i:                                        ; preds = %bb2.i.i.i4.i.i58.i, %bb17.i
  %164 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i61.i = load i64, ptr %164, align 8, !alias.scope !614, !noalias !409
  %165 = icmp eq i64 %.val2.i61.i, 0
  br i1 %165, label %bb18.i, label %bb2.i.i.i4.i7.i62.i

bb2.i.i.i4.i7.i62.i:                              ; preds = %bb4.i60.i
  %166 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i63.i = load ptr, ptr %166, align 8, !alias.scope !614, !noalias !409, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i63.i, i64 noundef %.val2.i61.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !617
  br label %bb18.i

bb18.i:                                           ; preds = %bb2.i.i.i4.i7.i62.i, %bb4.i60.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i), !noalias !409
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd.i)
          to label %bb10 unwind label %cleanup7

terminate.i:                                      ; preds = %bb25.i
  %167 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #25, !noalias !409
  unreachable

bb27.i:                                           ; preds = %bb2.i.i.i4.i.i.i.i25.i, %cleanup.i.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBN_7sources4once4OnceB1G_EEECslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 dereferenceable(48) %rustc1.i) #24, !noalias !409
  br label %bb384

bb29.i:                                           ; preds = %bb5.i186, %bb32.i
  %168 = landingpad { ptr, i32 }
          cleanup
  %169 = icmp eq i64 %23, 0
  br i1 %169, label %bb384, label %bb2.i.i.i4.i.i.i65.i

bb2.i.i.i4.i.i.i65.i:                             ; preds = %bb29.i
  %170 = icmp ne ptr %_33.sroa.5.0.copyload.i, null
  call void @llvm.assume(i1 %170)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_33.sroa.5.0.copyload.i, i64 noundef %23, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !409
  br label %bb384

bb384:                                            ; preds = %bb2.i.i.i4.i.i828, %cleanup12, %bb2.i.i.i4.i.i792, %cleanup11, %bb2.i.i.i4.i.i687, %cleanup10, %bb2.i.i.i4.i.i603, %cleanup9, %cleanup3.i.i, %bb2.i.i.i4.i.i.i573, %cleanup.body.i, %bb2.i.i.i4.i.i583, %bb2.i.i.i4.i.i213, %cleanup8, %bb25.i203, %bb2.i.i.i4.i.i.i205, %bb2.i.i.i4.i.i.i65.i, %bb29.i, %bb27.i, %bb25.i, %cleanup7
  %.pn = phi { ptr, i32 } [ %168, %bb29.i ], [ %168, %bb2.i.i.i4.i.i.i65.i ], [ %32, %bb27.i ], [ %.pn.i, %bb25.i ], [ %172, %cleanup7 ], [ %.pn.i204, %bb2.i.i.i4.i.i.i205 ], [ %.pn.i204, %bb25.i203 ], [ %227, %cleanup8 ], [ %227, %bb2.i.i.i4.i.i213 ], [ %372, %bb2.i.i.i4.i.i.i573 ], [ %372, %cleanup3.i.i ], [ %eh.lpad-body.i, %bb2.i.i.i4.i.i583 ], [ %eh.lpad-body.i, %cleanup.body.i ], [ %410, %cleanup9 ], [ %410, %bb2.i.i.i4.i.i603 ], [ %492, %cleanup10 ], [ %492, %bb2.i.i.i4.i.i687 ], [ %539, %cleanup11 ], [ %539, %bb2.i.i.i4.i.i792 ], [ %562, %cleanup12 ], [ %562, %bb2.i.i.i4.i.i828 ]
  %171 = icmp eq i64 %_15.sroa.0.0.copyload, 0
  br i1 %171, label %bb385, label %bb2.i.i.i4.i.i191

bb2.i.i.i4.i.i191:                                ; preds = %bb384
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_15.sroa.5.0.copyload, i64 noundef %_15.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb385

cleanup7:                                         ; preds = %bb434.thread.invoke, %bb229.invoke, %bb234.invoke, %bb7.i916.invoke, %bb412, %bb247, %bb284, %bb2.i939, %bb2.i921, %bb345, %bb200, %bb3.i.i.i594, %bb167, %bb12, %bb18.i, %bb20.i, %bb9, %bb223, %bb251, %bb246, %bb420, %bb242, %bb241, %bb416, %bb309, %bb299, %bb297, %bb323, %bb316, %bb347, %bb332, %bb369, %bb368, %bb389, %bb218, %bb190, %bb188, %bb186, %bb184, %bb180, %bb163, %bb166, %bb161, %bb124, %bb120, %bb118, %bb140, %bb143, %bb138, %bb151, %bb149, %bb105, %bb94, %bb89, %bb84, %bb78, %bb75, %bb70, %bb65, %bb60, %bb55, %bb50, %bb45, %bb40, %bb31, %bb30, %bb29, %bb23, %bb22, %bb21, %bb18, %bb16
  %172 = landingpad { ptr, i32 }
          cleanup
  br label %bb384

bb10:                                             ; preds = %bb18.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd.i), !noalias !409
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %rustc1.i), !noalias !409
  %.not = icmp eq i8 %_18.sroa.6.0, 2
  br i1 %.not, label %bb12, label %bb20

bb12:                                             ; preds = %.noexc188, %bb31.i, %bb10
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_21)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_21, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0afe6f0504dd6bb7e0912d0ebba4969c, i64 noundef 29)
          to label %bb14 unwind label %cleanup7

bb14:                                             ; preds = %bb12
  %173 = load i64, ptr %_21, align 8, !range !11, !noundef !3
  switch i64 %173, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %bb18
    i64 0, label %bb16
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb14
  %174 = getelementptr inbounds nuw i8, ptr %_21, i64 8
  %_21.val161 = load ptr, ptr %174, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_21.val161, i64 noundef %173, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb16

bb16:                                             ; preds = %bb14, %bb2.i.i.i4.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_21)
; invoke std::panicking::begin_panic::<&str>
  invoke void @_RINvNtCs5sEH5CPMdak_3std9panicking11begin_panicReEB4_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c6d8f7bad6fe84b0a1fb1ad39cccc87c, i64 noundef 33, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f56fcc3b591a05dcb59c62f94980d9e5) #26
          to label %unreachable unwind label %cleanup7

unreachable:                                      ; preds = %bb297, %bb16
  unreachable

bb18:                                             ; preds = %bb14
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_21)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr @alloc_2f2a7b3b6dd8e6a4ec2f251266b0daff, ptr %args, align 8
  %_27.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt, ptr %_27.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9b5fa806b1b67f7699cdf95aedfa7081, ptr noundef nonnull %args)
          to label %bb20.thread unwind label %cleanup7

bb20.thread:                                      ; preds = %bb18
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br label %bb21

bb20:                                             ; preds = %bb10
  %version1.sroa.4.4.extract.trunc = trunc i32 %_18.sroa.4.0 to i16
  %version1.sroa.4.6.extract.shift = lshr i32 %_18.sroa.4.0, 16
  %version1.sroa.4.6.extract.trunc = trunc i32 %version1.sroa.4.6.extract.shift to i8
  %version1.sroa.4.7.extract.shift = lshr i32 %_18.sroa.4.0, 24
  %version1.sroa.4.7.extract.trunc = trunc nuw i32 %version1.sroa.4.7.extract.shift to i8
  %_29 = icmp ugt i32 %_18.sroa.0.0, 79
  %extract.t1848 = trunc nuw i8 %_18.sroa.6.0 to i1
  br i1 %_29, label %bb21, label %bb26.thread

bb21:                                             ; preds = %bb20.thread, %bb20
  %version.sroa.0.01275 = phi i32 [ 87, %bb20.thread ], [ %_18.sroa.0.0, %bb20 ]
  %version.sroa.36.01273 = phi i16 [ 0, %bb20.thread ], [ %version1.sroa.4.4.extract.trunc, %bb20 ]
  %version.sroa.64.01271 = phi i8 [ 0, %bb20.thread ], [ %version1.sroa.4.6.extract.trunc, %bb20 ]
  %version.sroa.92.01269 = phi i8 [ 0, %bb20.thread ], [ %version1.sroa.4.7.extract.trunc, %bb20 ]
  %version.sroa.120.01267 = phi i32 [ 18, %bb20.thread ], [ %_18.sroa.5.0, %bb20 ]
  %version.sroa.128.01265.off0 = phi i1 [ false, %bb20.thread ], [ %extract.t1848, %bb20 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_092aad2bc9b61c4864788da0a3d09052, ptr noundef nonnull inttoptr (i64 287 to ptr))
          to label %bb22 unwind label %cleanup7

bb26:                                             ; preds = %bb23
  br i1 %version.sroa.128.01265.off0, label %bb1.i366, label %bb8.i363

bb26.thread:                                      ; preds = %bb20
  %_9.i = icmp eq i32 %_18.sroa.0.0, 79
  br i1 %extract.t1848, label %bb3.i199, label %bb8.i

bb8.i:                                            ; preds = %bb26.thread
  br i1 %_9.i, label %bb89, label %bb29

bb3.i199:                                         ; preds = %bb26.thread
  br i1 %_9.i, label %bb4.i, label %bb29

bb4.i:                                            ; preds = %bb3.i199
  %175 = icmp eq i16 %version1.sroa.4.4.extract.trunc, 2024
  %176 = icmp ugt i16 %version1.sroa.4.4.extract.trunc, 2023
  br i1 %175, label %bb11.i, label %bb27

bb11.i:                                           ; preds = %bb4.i
  %177 = icmp eq i8 %version1.sroa.4.6.extract.trunc, 4
  %178 = icmp ugt i8 %version1.sroa.4.6.extract.trunc, 3
  br i1 %177, label %bb12.i200, label %bb27

bb12.i200:                                        ; preds = %bb11.i
  %179 = icmp ugt i32 %_18.sroa.4.0, 167772159
  br i1 %179, label %bb89, label %bb29

bb22:                                             ; preds = %bb21
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_3c9e784e7e4614c4b58cf1fcdd1319d2, ptr noundef nonnull inttoptr (i64 2455 to ptr))
          to label %bb23 unwind label %cleanup7

bb23:                                             ; preds = %bb22
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9870bcf023b2efb7b8b18a97902cd1ef, ptr noundef nonnull inttoptr (i64 519 to ptr))
          to label %bb26 unwind label %cleanup7

bb27:                                             ; preds = %bb11.i, %bb4.i
  %_0.sroa.0.0.shrunk.i = phi i1 [ %178, %bb11.i ], [ %176, %bb4.i ]
  br i1 %_0.sroa.0.0.shrunk.i, label %bb89, label %bb29

bb29:                                             ; preds = %bb3.i199, %bb12.i200, %bb8.i, %bb27
  %version.sroa.64.012702018 = phi i8 [ %version1.sroa.4.6.extract.trunc, %bb3.i199 ], [ 4, %bb12.i200 ], [ %version1.sroa.4.6.extract.trunc, %bb8.i ], [ %version1.sroa.4.6.extract.trunc, %bb27 ]
  %version.sroa.36.012721991 = phi i16 [ %version1.sroa.4.4.extract.trunc, %bb3.i199 ], [ 2024, %bb12.i200 ], [ %version1.sroa.4.4.extract.trunc, %bb8.i ], [ %version1.sroa.4.4.extract.trunc, %bb27 ]
  %version.sroa.0.012741949 = phi i32 [ %_18.sroa.0.0, %bb3.i199 ], [ 79, %bb12.i200 ], [ %_18.sroa.0.0, %bb8.i ], [ 79, %bb27 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_d47a354a607bf77ebae5daba39357d52, ptr noundef nonnull inttoptr (i64 103 to ptr))
          to label %bb30 unwind label %cleanup7

bb30:                                             ; preds = %bb29
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_26aa35ac1fd69f511512a9380c2223a9, ptr noundef nonnull inttoptr (i64 75 to ptr))
          to label %bb31 unwind label %cleanup7

bb31:                                             ; preds = %bb30
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_5cdf6a79c62811efa8c483f97a5cc917, ptr noundef nonnull inttoptr (i64 99 to ptr))
          to label %bb32 unwind label %cleanup7

bb32:                                             ; preds = %bb31
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %target_upper)
  call void @llvm.experimental.noalias.scope.decl(metadata !618)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %result.i), !noalias !621
  store i64 0, ptr %result.i, align 8, !noalias !621
  %_94.sroa.4.0.result.sroa_idx.i = getelementptr inbounds nuw i8, ptr %result.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !noalias !621
  %_94.sroa.5.0.result.sroa_idx.i = getelementptr inbounds nuw i8, ptr %result.i, i64 16
  store i64 0, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !noalias !621
  %_7.i.i.i.i = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 %_9.sroa.8.0.copyload
  br label %bb20.i202

bb25.i203:                                        ; preds = %cleanup10.i, %cleanup9.i
  %.pn.i204 = phi { ptr, i32 } [ %191, %cleanup10.i ], [ %181, %cleanup9.i ]
  %result.val.i = load i64, ptr %result.i, align 8, !noalias !621
  %180 = icmp eq i64 %result.val.i, 0
  br i1 %180, label %bb384, label %bb2.i.i.i4.i.i.i205

bb2.i.i.i4.i.i.i205:                              ; preds = %bb25.i203
  %result.val28.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !noalias !621, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %result.val28.i, i64 noundef %result.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !623
  br label %bb384

cleanup9.i:                                       ; preds = %bb1.i.i30.i
  %181 = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i203

bb20.i202:                                        ; preds = %bb50.i, %bb32
  %_10.i4924.i = phi ptr [ inttoptr (i64 1 to ptr), %bb32 ], [ %_10.i49.i, %bb50.i ]
  %len.i.i.i = phi i64 [ 0, %bb32 ], [ %193, %bb50.i ]
  %iter.sroa.5.0.i = phi ptr [ %_9.sroa.5.0.copyload, %bb32 ], [ %iter.sroa.5.2.i, %bb50.i ]
  %iter.sroa.11.0.i = phi i64 [ 0, %bb32 ], [ %189, %bb50.i ]
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i, %bb20.i202
  %182 = phi i64 [ %189, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i ], [ %iter.sroa.11.0.i, %bb20.i202 ]
  %_16.i26.i.i.i4.i.i.i = phi ptr [ %iter.sroa.5.2.i, %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i ], [ %iter.sroa.5.0.i, %bb20.i202 ]
  %183 = ptrtoint ptr %_16.i26.i.i.i4.i.i.i to i64
  %_6.i.i.i.i.i.i.i = icmp eq ptr %_16.i26.i.i.i4.i.i.i, %_7.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i, label %bb56.i, label %bb14.i.i.i.i.i.i

bb14.i.i.i.i.i.i:                                 ; preds = %bb1.i.i.i
  %_16.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i, i64 1
  %x.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i4.i.i.i, align 1, !alias.scope !618, !noalias !624, !noundef !3
  %_6.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_30.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i, %_7.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i, i64 2
  %y.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !618, !noalias !624, !noundef !3
  %_33.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i to i32
  %184 = or disjoint i32 %_33.i.i.i.i.i.i, %_34.i.i.i.i.i.i
  %_13.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i to i32
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i, %_7.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i, i64 3
  %z.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i, align 1, !alias.scope !618, !noalias !624, !noundef !3
  %_38.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i, %_39.i.i.i.i.i.i
  %_20.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 12
  %185 = or disjoint i32 %y_z.i.i.i.i.i.i, %_20.i.i.i.i.i.i
  %_21.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i, %_7.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i4.i.i.i, i64 4
  %w.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i, align 1, !alias.scope !618, !noalias !624, !noundef !3
  %_26.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i, %_44.i.i.i.i.i.i
  %186 = or disjoint i32 %_27.i.i.i.i.i.i, %_25.i.i.i.i.i.i
  br label %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i

_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i: ; preds = %bb8.i.i.i.i.i.i, %bb6.i.i.i.i.i.i, %bb3.i.i.i.i.i.i, %bb4.i.i.i.i.i.i
  %iter.sroa.5.2.i = phi ptr [ %_16.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i.i, %bb8.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ], [ %186, %bb8.i.i.i.i.i.i ], [ %185, %bb6.i.i.i.i.i.i ], [ %184, %bb4.i.i.i.i.i.i ]
  %187 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %187)
  %188 = ptrtoint ptr %iter.sroa.5.2.i to i64
  %_10.i.i.i.i.i = sub i64 %188, %183
  %189 = add i64 %_10.i.i.i.i.i, %182
  %190 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i, -47
  %_0.sroa.0.0.off0.i.i.i.i.i.i = icmp ult i32 %190, -2
  br i1 %_0.sroa.0.0.off0.i.i.i.i.i.i, label %bb1.i.i.i, label %bb49.i

cleanup10.i:                                      ; preds = %bb1.i.i51.i, %bb1.i.i39.i
  %191 = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i203

bb56.i:                                           ; preds = %bb1.i.i.i
  %gepdiff12.i = sub nuw nsw i64 %_9.sroa.8.0.copyload, %iter.sroa.11.0.i
  call void @llvm.experimental.noalias.scope.decl(metadata !638)
  %self2.i.i.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !641, !noalias !621, !noundef !3
  %_9.i.i.i = sub i64 %self2.i.i.i, %len.i.i.i
  %_7.i.i.i207 = icmp ugt i64 %gepdiff12.i, %_9.i.i.i
  br i1 %_7.i.i.i207, label %bb1.i.i30.i, label %bb33, !prof !102

bb1.i.i30.i:                                      ; preds = %bb56.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %len.i.i.i, i64 noundef %gepdiff12.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc.i208 unwind label %cleanup9.i, !noalias !623

.noexc.i208:                                      ; preds = %bb1.i.i30.i
  %len.pre.i.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !638, !noalias !621
  br label %bb33

bb49.i:                                           ; preds = %_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_.exit.i.i.i
  %data13.i = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 %iter.sroa.11.0.i
  %gepdiff.i = sub nuw nsw i64 %182, %iter.sroa.11.0.i
  call void @llvm.experimental.noalias.scope.decl(metadata !644)
  %self2.i.i32.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !647, !noalias !621, !noundef !3
  %_9.i.i33.i = sub i64 %self2.i.i32.i, %len.i.i.i
  %_7.i.i34.i = icmp ugt i64 %gepdiff.i, %_9.i.i33.i
  br i1 %_7.i.i34.i, label %bb1.i.i39.i, label %bb51.i, !prof !102

bb1.i.i39.i:                                      ; preds = %bb49.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %len.i.i.i, i64 noundef %gepdiff.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc41.i unwind label %cleanup10.i, !noalias !623

.noexc41.i:                                       ; preds = %bb1.i.i39.i
  %len.pre.i40.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !644, !noalias !621
  %_10.i37.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !644, !noalias !621
  %self2.i.i44.pre.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !650, !noalias !621
  br label %bb51.i

bb51.i:                                           ; preds = %.noexc41.i, %bb49.i
  %_10.i4923.i = phi ptr [ %_10.i4924.i, %bb49.i ], [ %_10.i37.pre.i, %.noexc41.i ]
  %self2.i.i44.i = phi i64 [ %self2.i.i32.i, %bb49.i ], [ %self2.i.i44.pre.i, %.noexc41.i ]
  %len.i35.i = phi i64 [ %len.i.i.i, %bb49.i ], [ %len.pre.i40.i, %.noexc41.i ]
  %_9.i36.i = icmp sgt i64 %len.i35.i, -1
  call void @llvm.assume(i1 %_9.i36.i)
  %dst.i38.i = getelementptr inbounds nuw i8, ptr %_10.i4923.i, i64 %len.i35.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i38.i, ptr nonnull readonly align 1 %data13.i, i64 %gepdiff.i, i1 false), !noalias !655
  %192 = add i64 %len.i35.i, %gepdiff.i
  store i64 %192, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !644, !noalias !621
  call void @llvm.experimental.noalias.scope.decl(metadata !656)
  %_7.i.i46.i = icmp eq i64 %self2.i.i44.i, %192
  br i1 %_7.i.i46.i, label %bb1.i.i51.i, label %bb50.i, !prof !102

bb1.i.i51.i:                                      ; preds = %bb51.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %self2.i.i44.i, i64 noundef 1, i64 noundef 1, i64 noundef 1)
          to label %.noexc53.i unwind label %cleanup10.i, !noalias !623

.noexc53.i:                                       ; preds = %bb1.i.i51.i
  %len.pre.i52.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !656, !noalias !621
  %_10.i49.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !656, !noalias !621
  br label %bb50.i

bb50.i:                                           ; preds = %.noexc53.i, %bb51.i
  %_10.i49.i = phi ptr [ %_10.i4923.i, %bb51.i ], [ %_10.i49.pre.i, %.noexc53.i ]
  %len.i47.i = phi i64 [ %192, %bb51.i ], [ %len.pre.i52.i, %.noexc53.i ]
  %_9.i48.i = icmp sgt i64 %len.i47.i, -1
  call void @llvm.assume(i1 %_9.i48.i)
  %dst.i50.i = getelementptr inbounds nuw i8, ptr %_10.i49.i, i64 %len.i47.i
  store i8 95, ptr %dst.i50.i, align 1, !noalias !657
  %193 = add nuw i64 %len.i47.i, 1
  store i64 %193, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !656, !noalias !621
  br label %bb20.i202

bb33:                                             ; preds = %.noexc.i208, %bb56.i
  %len.i.i = phi i64 [ %len.i.i.i, %bb56.i ], [ %len.pre.i.i, %.noexc.i208 ]
  %data.i = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 %iter.sroa.11.0.i
  %_9.i.i = icmp sgt i64 %len.i.i, -1
  call void @llvm.assume(i1 %_9.i.i)
  %_10.i.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !638, !noalias !621, !nonnull !3, !noundef !3
  %dst.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 %len.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i, ptr nonnull readonly align 1 %data.i, i64 %gepdiff12.i, i1 false), !noalias !658
  %194 = add i64 %len.i.i, %gepdiff12.i
  store i64 %194, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !638, !noalias !621
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %target_upper, ptr noundef nonnull align 8 dereferenceable(24) %result.i, i64 24, i1 false), !noalias !618
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %result.i), !noalias !621
  %195 = getelementptr inbounds nuw i8, ptr %target_upper, i64 8
  %_497 = load ptr, ptr %195, align 8, !nonnull !3, !noundef !3
  %196 = getelementptr inbounds nuw i8, ptr %target_upper, i64 16
  %_496 = load i64, ptr %196, align 8, !noundef !3
  %_63.not.i = icmp eq i64 %_496, 0
  br i1 %_63.not.i, label %bb34, label %iter.check

iter.check:                                       ; preds = %bb33
  %min.iters.check = icmp ult i64 %_496, 8
  br i1 %min.iters.check, label %bb3.i211.preheader, label %vector.main.loop.iter.check

vector.main.loop.iter.check:                      ; preds = %iter.check
  %min.iters.check2594 = icmp ult i64 %_496, 64
  br i1 %min.iters.check2594, label %vec.epilog.ph, label %vector.ph

vector.ph:                                        ; preds = %vector.main.loop.iter.check
  %n.vec = and i64 %_496, -64
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %197 = getelementptr inbounds nuw i8, ptr %_497, i64 %index
  %198 = getelementptr inbounds nuw i8, ptr %197, i64 16
  %199 = getelementptr inbounds nuw i8, ptr %197, i64 32
  %200 = getelementptr inbounds nuw i8, ptr %197, i64 48
  %wide.load = load <16 x i8>, ptr %197, align 1, !alias.scope !659
  %wide.load2595 = load <16 x i8>, ptr %198, align 1, !alias.scope !659
  %wide.load2596 = load <16 x i8>, ptr %199, align 1, !alias.scope !659
  %wide.load2597 = load <16 x i8>, ptr %200, align 1, !alias.scope !659
  %201 = add <16 x i8> %wide.load, splat (i8 -97)
  %202 = add <16 x i8> %wide.load2595, splat (i8 -97)
  %203 = add <16 x i8> %wide.load2596, splat (i8 -97)
  %204 = add <16 x i8> %wide.load2597, splat (i8 -97)
  %205 = icmp ult <16 x i8> %201, splat (i8 26)
  %206 = icmp ult <16 x i8> %202, splat (i8 26)
  %207 = icmp ult <16 x i8> %203, splat (i8 26)
  %208 = icmp ult <16 x i8> %204, splat (i8 26)
  %209 = select <16 x i1> %205, <16 x i8> splat (i8 32), <16 x i8> zeroinitializer
  %210 = select <16 x i1> %206, <16 x i8> splat (i8 32), <16 x i8> zeroinitializer
  %211 = select <16 x i1> %207, <16 x i8> splat (i8 32), <16 x i8> zeroinitializer
  %212 = select <16 x i1> %208, <16 x i8> splat (i8 32), <16 x i8> zeroinitializer
  %213 = xor <16 x i8> %209, %wide.load
  %214 = xor <16 x i8> %210, %wide.load2595
  %215 = xor <16 x i8> %211, %wide.load2596
  %216 = xor <16 x i8> %212, %wide.load2597
  store <16 x i8> %213, ptr %197, align 1, !alias.scope !659
  store <16 x i8> %214, ptr %198, align 1, !alias.scope !659
  store <16 x i8> %215, ptr %199, align 1, !alias.scope !659
  store <16 x i8> %216, ptr %200, align 1, !alias.scope !659
  %index.next = add nuw i64 %index, 64
  %217 = icmp eq i64 %index.next, %n.vec
  br i1 %217, label %middle.block, label %vector.body, !llvm.loop !662

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %_496, %n.vec
  br i1 %cmp.n, label %bb34, label %vec.epilog.iter.check

vec.epilog.iter.check:                            ; preds = %middle.block
  %n.vec.remaining = and i64 %_496, 56
  %min.epilog.iters.check = icmp eq i64 %n.vec.remaining, 0
  br i1 %min.epilog.iters.check, label %bb3.i211.preheader, label %vec.epilog.ph

vec.epilog.ph:                                    ; preds = %vec.epilog.iter.check, %vector.main.loop.iter.check
  %vec.epilog.resume.val = phi i64 [ %n.vec, %vec.epilog.iter.check ], [ 0, %vector.main.loop.iter.check ]
  %n.vec2599 = and i64 %_496, -8
  br label %vec.epilog.vector.body

vec.epilog.vector.body:                           ; preds = %vec.epilog.vector.body, %vec.epilog.ph
  %index2600 = phi i64 [ %vec.epilog.resume.val, %vec.epilog.ph ], [ %index.next2602, %vec.epilog.vector.body ]
  %218 = getelementptr inbounds nuw i8, ptr %_497, i64 %index2600
  %wide.load2601 = load <8 x i8>, ptr %218, align 1, !alias.scope !659
  %219 = add <8 x i8> %wide.load2601, splat (i8 -97)
  %220 = icmp ult <8 x i8> %219, splat (i8 26)
  %221 = select <8 x i1> %220, <8 x i8> splat (i8 32), <8 x i8> zeroinitializer
  %222 = xor <8 x i8> %221, %wide.load2601
  store <8 x i8> %222, ptr %218, align 1, !alias.scope !659
  %index.next2602 = add nuw i64 %index2600, 8
  %223 = icmp eq i64 %index.next2602, %n.vec2599
  br i1 %223, label %vec.epilog.middle.block, label %vec.epilog.vector.body, !llvm.loop !665

vec.epilog.middle.block:                          ; preds = %vec.epilog.vector.body
  %cmp.n2603 = icmp eq i64 %_496, %n.vec2599
  br i1 %cmp.n2603, label %bb34, label %bb3.i211.preheader

bb3.i211.preheader:                               ; preds = %vec.epilog.iter.check, %vec.epilog.middle.block, %iter.check
  %i.sroa.0.04.i.ph = phi i64 [ 0, %iter.check ], [ %n.vec, %vec.epilog.iter.check ], [ %n.vec2599, %vec.epilog.middle.block ]
  br label %bb3.i211

bb3.i211:                                         ; preds = %bb3.i211.preheader, %bb3.i211
  %i.sroa.0.04.i = phi i64 [ %226, %bb3.i211 ], [ %i.sroa.0.04.i.ph, %bb3.i211.preheader ]
  %byte.i = getelementptr inbounds nuw i8, ptr %_497, i64 %i.sroa.0.04.i
  %_13.i = load i8, ptr %byte.i, align 1, !alias.scope !659, !noundef !3
  %224 = add i8 %_13.i, -97
  %225 = icmp ult i8 %224, 26
  %_17.sroa.0.0.off0.i = select i1 %225, i8 32, i8 0
  %_12.i = xor i8 %_17.sroa.0.0.off0.i, %_13.i
  store i8 %_12.i, ptr %byte.i, align 1, !alias.scope !659
  %226 = add nuw i64 %i.sroa.0.04.i, 1
  %exitcond.not.i = icmp eq i64 %226, %_496
  br i1 %exitcond.not.i, label %bb34, label %bb3.i211, !llvm.loop !666

cleanup8:                                         ; preds = %bb34
  %227 = landingpad { ptr, i32 }
          cleanup
  %target_upper.val = load i64, ptr %target_upper, align 8
  %228 = icmp eq i64 %target_upper.val, 0
  br i1 %228, label %bb384, label %bb2.i.i.i4.i.i213

bb2.i.i.i4.i.i213:                                ; preds = %cleanup8
  %target_upper.val153 = load ptr, ptr %195, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_upper.val153, i64 noundef %target_upper.val, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb384

bb34:                                             ; preds = %bb3.i211, %middle.block, %vec.epilog.middle.block, %bb33
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %target_upper, ptr %args2, align 8
  %_52.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args2, i64 8
  store ptr @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_52.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c22f04a0140b0ae1314c125358058abd, ptr noundef nonnull %args2)
          to label %bb35 unwind label %cleanup8

bb35:                                             ; preds = %bb34
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  %target_upper.val154 = load i64, ptr %target_upper, align 8
  %229 = icmp eq i64 %target_upper.val154, 0
  br i1 %229, label %bb37, label %bb2.i.i.i4.i.i215

bb2.i.i.i4.i.i215:                                ; preds = %bb35
  %target_upper.val155 = load ptr, ptr %195, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_upper.val155, i64 noundef %target_upper.val154, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb37

bb37:                                             ; preds = %bb35, %bb2.i.i.i4.i.i215
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %target_upper)
  br i1 %extract.t1848, label %bb1.i222, label %bb8.i219

bb8.i219:                                         ; preds = %bb37
  %230 = icmp ugt i32 %version.sroa.0.012741949, 44
  br i1 %230, label %bb8.i235, label %bb40

bb1.i222:                                         ; preds = %bb37
  %_7.i223 = icmp ugt i32 %version.sroa.0.012741949, 45
  br i1 %_7.i223, label %bb1.i238, label %bb3.i224

bb3.i224:                                         ; preds = %bb1.i222
  %_9.i225 = icmp eq i32 %version.sroa.0.012741949, 45
  br i1 %_9.i225, label %bb4.i226, label %bb40

bb4.i226:                                         ; preds = %bb3.i224
  %231 = icmp eq i16 %version.sroa.36.012721991, 2020
  %232 = icmp ugt i16 %version.sroa.36.012721991, 2019
  br i1 %231, label %bb11.i228, label %bb38

bb11.i228:                                        ; preds = %bb4.i226
  %233 = icmp eq i8 %version.sroa.64.012702018, 5
  %234 = icmp ugt i8 %version.sroa.64.012702018, 4
  br i1 %233, label %bb12.i230, label %bb38

bb12.i230:                                        ; preds = %bb11.i228
  %235 = icmp ugt i32 %_18.sroa.4.0, 486539263
  br i1 %235, label %bb45, label %bb40

bb38:                                             ; preds = %bb11.i228, %bb4.i226
  %_0.sroa.0.0.shrunk.i221 = phi i1 [ %234, %bb11.i228 ], [ %232, %bb4.i226 ]
  br i1 %_0.sroa.0.0.shrunk.i221, label %bb45, label %bb40

bb40:                                             ; preds = %bb3.i224, %bb12.i230, %bb8.i219, %bb38
  %version.sroa.64.012702015 = phi i8 [ %version.sroa.64.012702018, %bb3.i224 ], [ 5, %bb12.i230 ], [ %version.sroa.64.012702018, %bb8.i219 ], [ %version.sroa.64.012702018, %bb38 ]
  %version.sroa.36.012721988 = phi i16 [ %version.sroa.36.012721991, %bb3.i224 ], [ 2020, %bb12.i230 ], [ %version.sroa.36.012721991, %bb8.i219 ], [ %version.sroa.36.012721991, %bb38 ]
  %version.sroa.0.012741954 = phi i32 [ %version.sroa.0.012741949, %bb3.i224 ], [ 45, %bb12.i230 ], [ %version.sroa.0.012741949, %bb8.i219 ], [ 45, %bb38 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_1fed9b908bc75f0483fa23e0a5540e7b, ptr noundef nonnull inttoptr (i64 101 to ptr))
          to label %bb45 unwind label %cleanup7

bb8.i235:                                         ; preds = %bb8.i219
  %.not1850 = icmp eq i32 %version.sroa.0.012741949, 45
  br i1 %.not1850, label %bb45, label %bb8.i251

bb1.i238:                                         ; preds = %bb1.i222
  %_7.i239.not = icmp eq i32 %version.sroa.0.012741949, 46
  br i1 %_7.i239.not, label %bb4.i242, label %bb1.i254

bb4.i242:                                         ; preds = %bb1.i238
  %236 = icmp eq i16 %version.sroa.36.012721991, 2020
  %237 = icmp ugt i16 %version.sroa.36.012721991, 2019
  br i1 %236, label %bb11.i244, label %bb43

bb11.i244:                                        ; preds = %bb4.i242
  %238 = icmp eq i8 %version.sroa.64.012702018, 7
  %239 = icmp ugt i8 %version.sroa.64.012702018, 6
  br i1 %238, label %bb12.i246, label %bb43

bb12.i246:                                        ; preds = %bb11.i244
  %.not1851 = icmp ult i32 %_18.sroa.4.0, 16777216
  br i1 %.not1851, label %bb45, label %bb50

bb43:                                             ; preds = %bb11.i244, %bb4.i242
  %_0.sroa.0.0.shrunk.i237 = phi i1 [ %239, %bb11.i244 ], [ %237, %bb4.i242 ]
  br i1 %_0.sroa.0.0.shrunk.i237, label %bb50, label %bb45

bb45:                                             ; preds = %bb38, %bb12.i230, %bb40, %bb12.i246, %bb8.i235, %bb43
  %version.sroa.128.01264.off02102 = phi i1 [ true, %bb38 ], [ true, %bb12.i230 ], [ %extract.t1848, %bb40 ], [ true, %bb12.i246 ], [ false, %bb8.i235 ], [ true, %bb43 ]
  %version.sroa.92.012682029 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb38 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i230 ], [ %version1.sroa.4.7.extract.trunc, %bb40 ], [ 0, %bb12.i246 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i235 ], [ %version1.sroa.4.7.extract.trunc, %bb43 ]
  %version.sroa.64.012702014 = phi i8 [ %version.sroa.64.012702018, %bb38 ], [ 5, %bb12.i230 ], [ %version.sroa.64.012702015, %bb40 ], [ 7, %bb12.i246 ], [ %version.sroa.64.012702018, %bb8.i235 ], [ %version.sroa.64.012702018, %bb43 ]
  %version.sroa.36.012721987 = phi i16 [ %version.sroa.36.012721991, %bb38 ], [ 2020, %bb12.i230 ], [ %version.sroa.36.012721988, %bb40 ], [ 2020, %bb12.i246 ], [ %version.sroa.36.012721991, %bb8.i235 ], [ %version.sroa.36.012721991, %bb43 ]
  %version.sroa.0.012741953 = phi i32 [ 45, %bb38 ], [ 45, %bb12.i230 ], [ %version.sroa.0.012741954, %bb40 ], [ 46, %bb12.i246 ], [ 45, %bb8.i235 ], [ 46, %bb43 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_ad12ff8fc6e049ff439008f27922a354, ptr noundef nonnull inttoptr (i64 97 to ptr))
          to label %bb50 unwind label %cleanup7

bb8.i251:                                         ; preds = %bb8.i235
  %240 = icmp ugt i32 %version.sroa.0.012741949, 51
  br i1 %240, label %bb8.i267, label %bb50

bb1.i254:                                         ; preds = %bb1.i238
  %_7.i255 = icmp ugt i32 %version.sroa.0.012741949, 52
  br i1 %_7.i255, label %bb1.i270, label %bb3.i256

bb3.i256:                                         ; preds = %bb1.i254
  %_9.i257 = icmp eq i32 %version.sroa.0.012741949, 52
  br i1 %_9.i257, label %bb4.i258, label %bb50

bb4.i258:                                         ; preds = %bb3.i256
  %241 = icmp eq i16 %version.sroa.36.012721991, 2021
  %242 = icmp ugt i16 %version.sroa.36.012721991, 2020
  br i1 %241, label %bb11.i260, label %bb48

bb11.i260:                                        ; preds = %bb4.i258
  %243 = icmp eq i8 %version.sroa.64.012702018, 3
  %244 = icmp ugt i8 %version.sroa.64.012702018, 2
  br i1 %243, label %bb12.i262, label %bb48

bb12.i262:                                        ; preds = %bb11.i260
  %245 = icmp ugt i32 %_18.sroa.4.0, 167772159
  br i1 %245, label %bb55, label %bb50

bb48:                                             ; preds = %bb11.i260, %bb4.i258
  %_0.sroa.0.0.shrunk.i253 = phi i1 [ %244, %bb11.i260 ], [ %242, %bb4.i258 ]
  br i1 %_0.sroa.0.0.shrunk.i253, label %bb55, label %bb50

bb50:                                             ; preds = %bb43, %bb12.i246, %bb45, %bb3.i256, %bb12.i262, %bb8.i251, %bb48
  %version.sroa.128.01264.off02101 = phi i1 [ true, %bb43 ], [ true, %bb12.i246 ], [ %version.sroa.128.01264.off02102, %bb45 ], [ true, %bb3.i256 ], [ true, %bb12.i262 ], [ false, %bb8.i251 ], [ true, %bb48 ]
  %version.sroa.92.012682028 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb43 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i246 ], [ %version.sroa.92.012682029, %bb45 ], [ %version1.sroa.4.7.extract.trunc, %bb3.i256 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i262 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i251 ], [ %version1.sroa.4.7.extract.trunc, %bb48 ]
  %version.sroa.64.012702013 = phi i8 [ %version.sroa.64.012702018, %bb43 ], [ 7, %bb12.i246 ], [ %version.sroa.64.012702014, %bb45 ], [ %version.sroa.64.012702018, %bb3.i256 ], [ 3, %bb12.i262 ], [ %version.sroa.64.012702018, %bb8.i251 ], [ %version.sroa.64.012702018, %bb48 ]
  %version.sroa.36.012721986 = phi i16 [ %version.sroa.36.012721991, %bb43 ], [ 2020, %bb12.i246 ], [ %version.sroa.36.012721987, %bb45 ], [ %version.sroa.36.012721991, %bb3.i256 ], [ 2021, %bb12.i262 ], [ %version.sroa.36.012721991, %bb8.i251 ], [ %version.sroa.36.012721991, %bb48 ]
  %version.sroa.0.012741952 = phi i32 [ 46, %bb43 ], [ 46, %bb12.i246 ], [ %version.sroa.0.012741953, %bb45 ], [ %version.sroa.0.012741949, %bb3.i256 ], [ 52, %bb12.i262 ], [ %version.sroa.0.012741949, %bb8.i251 ], [ 52, %bb48 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c1341c958305d557d0587cd9fe34f2c5, ptr noundef nonnull inttoptr (i64 117 to ptr))
          to label %bb55 unwind label %cleanup7

bb8.i267:                                         ; preds = %bb8.i251
  %246 = icmp ugt i32 %version.sroa.0.012741949, 55
  br i1 %246, label %bb8.i299, label %bb55

bb1.i270:                                         ; preds = %bb1.i254
  %_7.i271 = icmp ugt i32 %version.sroa.0.012741949, 56
  br i1 %_7.i271, label %bb1.i302, label %bb3.i272

bb3.i272:                                         ; preds = %bb1.i270
  %_9.i273 = icmp eq i32 %version.sroa.0.012741949, 56
  br i1 %_9.i273, label %bb4.i274, label %bb55

bb4.i274:                                         ; preds = %bb3.i272
  %247 = icmp eq i16 %version.sroa.36.012721991, 2021
  %248 = icmp ugt i16 %version.sroa.36.012721991, 2020
  br i1 %247, label %bb11.i276, label %bb53

bb11.i276:                                        ; preds = %bb4.i274
  %249 = icmp eq i8 %version.sroa.64.012702018, 7
  %250 = icmp ugt i8 %version.sroa.64.012702018, 6
  br i1 %249, label %bb12.i278, label %bb53

bb12.i278:                                        ; preds = %bb11.i276
  %251 = icmp ugt i32 %_18.sroa.4.0, 469762047
  br i1 %251, label %bb60, label %bb55

bb53:                                             ; preds = %bb11.i276, %bb4.i274
  %_0.sroa.0.0.shrunk.i269 = phi i1 [ %250, %bb11.i276 ], [ %248, %bb4.i274 ]
  br i1 %_0.sroa.0.0.shrunk.i269, label %bb4.i290, label %bb55

bb55:                                             ; preds = %bb48, %bb12.i262, %bb50, %bb3.i272, %bb12.i278, %bb8.i267, %bb53
  %version.sroa.128.01264.off02100 = phi i1 [ true, %bb48 ], [ true, %bb12.i262 ], [ %version.sroa.128.01264.off02101, %bb50 ], [ true, %bb3.i272 ], [ true, %bb12.i278 ], [ false, %bb8.i267 ], [ true, %bb53 ]
  %version.sroa.92.012682026 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb48 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i262 ], [ %version.sroa.92.012682028, %bb50 ], [ %version1.sroa.4.7.extract.trunc, %bb3.i272 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i278 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i267 ], [ %version1.sroa.4.7.extract.trunc, %bb53 ]
  %version.sroa.64.012702012 = phi i8 [ %version.sroa.64.012702018, %bb48 ], [ 3, %bb12.i262 ], [ %version.sroa.64.012702013, %bb50 ], [ %version.sroa.64.012702018, %bb3.i272 ], [ 7, %bb12.i278 ], [ %version.sroa.64.012702018, %bb8.i267 ], [ %version.sroa.64.012702018, %bb53 ]
  %version.sroa.36.012721985 = phi i16 [ %version.sroa.36.012721991, %bb48 ], [ 2021, %bb12.i262 ], [ %version.sroa.36.012721986, %bb50 ], [ %version.sroa.36.012721991, %bb3.i272 ], [ 2021, %bb12.i278 ], [ %version.sroa.36.012721991, %bb8.i267 ], [ %version.sroa.36.012721991, %bb53 ]
  %version.sroa.0.012741951 = phi i32 [ 52, %bb48 ], [ 52, %bb12.i262 ], [ %version.sroa.0.012741952, %bb50 ], [ %version.sroa.0.012741949, %bb3.i272 ], [ 56, %bb12.i278 ], [ %version.sroa.0.012741949, %bb8.i267 ], [ 56, %bb53 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_6833eaf1a79a5bbdf426dcbf1dc565a9, ptr noundef nonnull inttoptr (i64 103 to ptr))
          to label %bb57 unwind label %cleanup7

bb57:                                             ; preds = %bb55
  br i1 %version.sroa.128.01264.off02100, label %bb3.i288, label %bb8.i283

bb8.i283:                                         ; preds = %bb57
  %252 = icmp samesign ugt i32 %version.sroa.0.012741951, 55
  br i1 %252, label %bb65, label %bb60

bb3.i288:                                         ; preds = %bb57
  %_9.i289 = icmp eq i32 %version.sroa.0.012741951, 56
  br i1 %_9.i289, label %bb4.i290, label %bb60

bb4.i290:                                         ; preds = %bb53, %bb3.i288
  %version.sroa.92.012682027 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb53 ], [ %version.sroa.92.012682026, %bb3.i288 ]
  %version.sroa.64.012702011 = phi i8 [ %version.sroa.64.012702018, %bb53 ], [ %version.sroa.64.012702012, %bb3.i288 ]
  %version.sroa.36.012721984 = phi i16 [ %version.sroa.36.012721991, %bb53 ], [ %version.sroa.36.012721985, %bb3.i288 ]
  %253 = icmp eq i16 %version.sroa.36.012721984, 2021
  %254 = icmp ugt i16 %version.sroa.36.012721984, 2020
  br i1 %253, label %bb11.i292, label %bb58

bb11.i292:                                        ; preds = %bb4.i290
  %255 = icmp eq i8 %version.sroa.64.012702011, 8
  %256 = icmp ugt i8 %version.sroa.64.012702011, 7
  br i1 %255, label %bb12.i294, label %bb58

bb12.i294:                                        ; preds = %bb11.i292
  %.not1852 = icmp eq i8 %version.sroa.92.012682027, 0
  br i1 %.not1852, label %bb60, label %bb65

bb58:                                             ; preds = %bb11.i292, %bb4.i290
  %_0.sroa.0.0.shrunk.i285 = phi i1 [ %256, %bb11.i292 ], [ %254, %bb4.i290 ]
  br i1 %_0.sroa.0.0.shrunk.i285, label %bb65, label %bb60

bb60:                                             ; preds = %bb12.i278, %bb3.i288, %bb12.i294, %bb8.i283, %bb58
  %version.sroa.128.01264.off02083 = phi i1 [ true, %bb3.i288 ], [ true, %bb12.i294 ], [ false, %bb8.i283 ], [ true, %bb58 ], [ true, %bb12.i278 ]
  %version.sroa.92.012682037 = phi i8 [ %version.sroa.92.012682026, %bb3.i288 ], [ 0, %bb12.i294 ], [ %version.sroa.92.012682026, %bb8.i283 ], [ %version.sroa.92.012682027, %bb58 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i278 ]
  %version.sroa.64.012701995 = phi i8 [ %version.sroa.64.012702012, %bb3.i288 ], [ 8, %bb12.i294 ], [ %version.sroa.64.012702012, %bb8.i283 ], [ %version.sroa.64.012702011, %bb58 ], [ 7, %bb12.i278 ]
  %version.sroa.36.012721968 = phi i16 [ %version.sroa.36.012721985, %bb3.i288 ], [ 2021, %bb12.i294 ], [ %version.sroa.36.012721985, %bb8.i283 ], [ %version.sroa.36.012721984, %bb58 ], [ 2021, %bb12.i278 ]
  %version.sroa.0.012741961 = phi i32 [ %version.sroa.0.012741951, %bb3.i288 ], [ 56, %bb12.i294 ], [ %version.sroa.0.012741951, %bb8.i283 ], [ 56, %bb58 ], [ 56, %bb12.i278 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_4eea02266a7f159030b82ab70c4915d3, ptr noundef nonnull inttoptr (i64 105 to ptr))
          to label %bb65 unwind label %cleanup7

bb8.i299:                                         ; preds = %bb8.i267
  %257 = icmp ugt i32 %version.sroa.0.012741949, 57
  br i1 %257, label %bb8.i315, label %bb65

bb1.i302:                                         ; preds = %bb1.i270
  %_7.i303 = icmp ugt i32 %version.sroa.0.012741949, 58
  br i1 %_7.i303, label %bb1.i318, label %bb3.i304

bb3.i304:                                         ; preds = %bb1.i302
  %_9.i305 = icmp eq i32 %version.sroa.0.012741949, 58
  br i1 %_9.i305, label %bb4.i306, label %bb65

bb4.i306:                                         ; preds = %bb3.i304
  %258 = icmp eq i16 %version.sroa.36.012721991, 2021
  %259 = icmp ugt i16 %version.sroa.36.012721991, 2020
  br i1 %258, label %bb11.i308, label %bb63

bb11.i308:                                        ; preds = %bb4.i306
  %260 = icmp eq i8 %version.sroa.64.012702018, 11
  %261 = icmp ugt i8 %version.sroa.64.012702018, 10
  br i1 %260, label %bb12.i310, label %bb63

bb12.i310:                                        ; preds = %bb11.i308
  %262 = icmp ugt i32 %_18.sroa.4.0, 234881023
  br i1 %262, label %bb70, label %bb65

bb63:                                             ; preds = %bb11.i308, %bb4.i306
  %_0.sroa.0.0.shrunk.i301 = phi i1 [ %261, %bb11.i308 ], [ %259, %bb4.i306 ]
  br i1 %_0.sroa.0.0.shrunk.i301, label %bb70, label %bb65

bb65:                                             ; preds = %bb58, %bb12.i294, %bb60, %bb8.i283, %bb3.i304, %bb12.i310, %bb8.i299, %bb63
  %version.sroa.128.01264.off02098 = phi i1 [ true, %bb58 ], [ true, %bb12.i294 ], [ %version.sroa.128.01264.off02083, %bb60 ], [ false, %bb8.i283 ], [ true, %bb3.i304 ], [ true, %bb12.i310 ], [ false, %bb8.i299 ], [ true, %bb63 ]
  %version.sroa.92.012682036 = phi i8 [ %version.sroa.92.012682027, %bb58 ], [ %version.sroa.92.012682027, %bb12.i294 ], [ %version.sroa.92.012682037, %bb60 ], [ %version.sroa.92.012682026, %bb8.i283 ], [ %version1.sroa.4.7.extract.trunc, %bb3.i304 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i310 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i299 ], [ %version1.sroa.4.7.extract.trunc, %bb63 ]
  %version.sroa.64.012702010 = phi i8 [ %version.sroa.64.012702011, %bb58 ], [ 8, %bb12.i294 ], [ %version.sroa.64.012701995, %bb60 ], [ %version.sroa.64.012702012, %bb8.i283 ], [ %version.sroa.64.012702018, %bb3.i304 ], [ 11, %bb12.i310 ], [ %version.sroa.64.012702018, %bb8.i299 ], [ %version.sroa.64.012702018, %bb63 ]
  %version.sroa.36.012721983 = phi i16 [ %version.sroa.36.012721984, %bb58 ], [ 2021, %bb12.i294 ], [ %version.sroa.36.012721968, %bb60 ], [ %version.sroa.36.012721985, %bb8.i283 ], [ %version.sroa.36.012721991, %bb3.i304 ], [ 2021, %bb12.i310 ], [ %version.sroa.36.012721991, %bb8.i299 ], [ %version.sroa.36.012721991, %bb63 ]
  %version.sroa.0.012741960 = phi i32 [ 56, %bb58 ], [ 56, %bb12.i294 ], [ %version.sroa.0.012741961, %bb60 ], [ 56, %bb8.i283 ], [ 57, %bb3.i304 ], [ 58, %bb12.i310 ], [ %version.sroa.0.012741949, %bb8.i299 ], [ 58, %bb63 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_36b7da1b4ea2bacdb83a39b716c9e7e2, ptr noundef nonnull inttoptr (i64 111 to ptr))
          to label %bb70 unwind label %cleanup7

bb8.i315:                                         ; preds = %bb8.i299
  %263 = icmp ugt i32 %version.sroa.0.012741949, 63
  br i1 %263, label %bb8.i331, label %bb70

bb1.i318:                                         ; preds = %bb1.i302
  %_7.i319 = icmp ugt i32 %version.sroa.0.012741949, 64
  br i1 %_7.i319, label %bb1.i334, label %bb3.i320

bb3.i320:                                         ; preds = %bb1.i318
  %_9.i321 = icmp eq i32 %version.sroa.0.012741949, 64
  br i1 %_9.i321, label %bb4.i322, label %bb70

bb4.i322:                                         ; preds = %bb3.i320
  %264 = icmp eq i16 %version.sroa.36.012721991, 2022
  %265 = icmp ugt i16 %version.sroa.36.012721991, 2021
  br i1 %264, label %bb11.i324, label %bb68

bb11.i324:                                        ; preds = %bb4.i322
  %266 = icmp eq i8 %version.sroa.64.012702018, 7
  %267 = icmp ugt i8 %version.sroa.64.012702018, 6
  br i1 %266, label %bb12.i326, label %bb68

bb12.i326:                                        ; preds = %bb11.i324
  %268 = icmp ugt i32 %_18.sroa.4.0, 301989887
  br i1 %268, label %bb75, label %bb70

bb68:                                             ; preds = %bb11.i324, %bb4.i322
  %_0.sroa.0.0.shrunk.i317 = phi i1 [ %267, %bb11.i324 ], [ %265, %bb4.i322 ]
  br i1 %_0.sroa.0.0.shrunk.i317, label %bb75, label %bb70

bb70:                                             ; preds = %bb63, %bb12.i310, %bb65, %bb3.i320, %bb12.i326, %bb8.i315, %bb68
  %version.sroa.128.01264.off02097 = phi i1 [ true, %bb63 ], [ true, %bb12.i310 ], [ %version.sroa.128.01264.off02098, %bb65 ], [ true, %bb3.i320 ], [ true, %bb12.i326 ], [ false, %bb8.i315 ], [ true, %bb68 ]
  %version.sroa.92.012682035 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb63 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i310 ], [ %version.sroa.92.012682036, %bb65 ], [ %version1.sroa.4.7.extract.trunc, %bb3.i320 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i326 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i315 ], [ %version1.sroa.4.7.extract.trunc, %bb68 ]
  %version.sroa.64.012702009 = phi i8 [ %version.sroa.64.012702018, %bb63 ], [ 11, %bb12.i310 ], [ %version.sroa.64.012702010, %bb65 ], [ %version.sroa.64.012702018, %bb3.i320 ], [ 7, %bb12.i326 ], [ %version.sroa.64.012702018, %bb8.i315 ], [ %version.sroa.64.012702018, %bb68 ]
  %version.sroa.36.012721982 = phi i16 [ %version.sroa.36.012721991, %bb63 ], [ 2021, %bb12.i310 ], [ %version.sroa.36.012721983, %bb65 ], [ %version.sroa.36.012721991, %bb3.i320 ], [ 2022, %bb12.i326 ], [ %version.sroa.36.012721991, %bb8.i315 ], [ %version.sroa.36.012721991, %bb68 ]
  %version.sroa.0.012741959 = phi i32 [ 58, %bb63 ], [ 58, %bb12.i310 ], [ %version.sroa.0.012741960, %bb65 ], [ %version.sroa.0.012741949, %bb3.i320 ], [ 64, %bb12.i326 ], [ %version.sroa.0.012741949, %bb8.i315 ], [ 64, %bb68 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_11a9245b51bd70ca041d4c95e4e22ec5, ptr noundef nonnull inttoptr (i64 123 to ptr))
          to label %bb75 unwind label %cleanup7

bb8.i331:                                         ; preds = %bb8.i315
  %269 = icmp ugt i32 %version.sroa.0.012741949, 73
  br i1 %269, label %bb77, label %bb75

bb1.i334:                                         ; preds = %bb1.i318
  %_7.i335 = icmp ugt i32 %version.sroa.0.012741949, 74
  br i1 %_7.i335, label %bb77, label %bb3.i336

bb3.i336:                                         ; preds = %bb1.i334
  %_9.i337 = icmp eq i32 %version.sroa.0.012741949, 74
  br i1 %_9.i337, label %bb4.i338, label %bb75

bb4.i338:                                         ; preds = %bb3.i336
  %270 = icmp eq i16 %version.sroa.36.012721991, 2023
  %271 = icmp ugt i16 %version.sroa.36.012721991, 2022
  br i1 %270, label %bb11.i340, label %bb73

bb11.i340:                                        ; preds = %bb4.i338
  %272 = icmp eq i8 %version.sroa.64.012702018, 8
  %273 = icmp ugt i8 %version.sroa.64.012702018, 7
  br i1 %272, label %bb12.i342, label %bb73

bb12.i342:                                        ; preds = %bb11.i340
  %274 = icmp ugt i32 %_18.sroa.4.0, 385875967
  br i1 %274, label %bb78, label %bb75

bb73:                                             ; preds = %bb11.i340, %bb4.i338
  %_0.sroa.0.0.shrunk.i333 = phi i1 [ %273, %bb11.i340 ], [ %271, %bb4.i338 ]
  br i1 %_0.sroa.0.0.shrunk.i333, label %bb78, label %bb75

bb75:                                             ; preds = %bb68, %bb12.i326, %bb70, %bb3.i336, %bb12.i342, %bb8.i331, %bb73
  %version.sroa.128.01264.off02096 = phi i1 [ true, %bb68 ], [ true, %bb12.i326 ], [ %version.sroa.128.01264.off02097, %bb70 ], [ true, %bb3.i336 ], [ true, %bb12.i342 ], [ false, %bb8.i331 ], [ true, %bb73 ]
  %version.sroa.92.012682034 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb68 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i326 ], [ %version.sroa.92.012682035, %bb70 ], [ %version1.sroa.4.7.extract.trunc, %bb3.i336 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i342 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i331 ], [ %version1.sroa.4.7.extract.trunc, %bb73 ]
  %version.sroa.64.012702008 = phi i8 [ %version.sroa.64.012702018, %bb68 ], [ 7, %bb12.i326 ], [ %version.sroa.64.012702009, %bb70 ], [ %version.sroa.64.012702018, %bb3.i336 ], [ 8, %bb12.i342 ], [ %version.sroa.64.012702018, %bb8.i331 ], [ %version.sroa.64.012702018, %bb73 ]
  %version.sroa.36.012721981 = phi i16 [ %version.sroa.36.012721991, %bb68 ], [ 2022, %bb12.i326 ], [ %version.sroa.36.012721982, %bb70 ], [ %version.sroa.36.012721991, %bb3.i336 ], [ 2023, %bb12.i342 ], [ %version.sroa.36.012721991, %bb8.i331 ], [ %version.sroa.36.012721991, %bb73 ]
  %version.sroa.0.012741958 = phi i32 [ 64, %bb68 ], [ 64, %bb12.i326 ], [ %version.sroa.0.012741959, %bb70 ], [ %version.sroa.0.012741949, %bb3.i336 ], [ 74, %bb12.i342 ], [ %version.sroa.0.012741949, %bb8.i331 ], [ 74, %bb73 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_42144296fbd1e57a517c011ba5e34db7, ptr noundef nonnull inttoptr (i64 105 to ptr))
          to label %bb78 unwind label %cleanup7

bb77:                                             ; preds = %bb1.i334, %bb8.i331
  %_86 = icmp ult i32 %version.sroa.0.012741949, 77
  br i1 %_86, label %bb78, label %bb81

bb78:                                             ; preds = %bb73, %bb75, %bb12.i342, %bb77
  %version.sroa.128.01264.off02095 = phi i1 [ true, %bb73 ], [ %version.sroa.128.01264.off02096, %bb75 ], [ true, %bb12.i342 ], [ %extract.t1848, %bb77 ]
  %version.sroa.92.012682032 = phi i8 [ %version1.sroa.4.7.extract.trunc, %bb73 ], [ %version.sroa.92.012682034, %bb75 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i342 ], [ %version1.sroa.4.7.extract.trunc, %bb77 ]
  %version.sroa.64.012702007 = phi i8 [ %version.sroa.64.012702018, %bb73 ], [ %version.sroa.64.012702008, %bb75 ], [ 8, %bb12.i342 ], [ %version.sroa.64.012702018, %bb77 ]
  %version.sroa.36.012721980 = phi i16 [ %version.sroa.36.012721991, %bb73 ], [ %version.sroa.36.012721981, %bb75 ], [ 2023, %bb12.i342 ], [ %version.sroa.36.012721991, %bb77 ]
  %version.sroa.0.012741957 = phi i32 [ 74, %bb73 ], [ %version.sroa.0.012741958, %bb75 ], [ 74, %bb12.i342 ], [ %version.sroa.0.012741949, %bb77 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_5d94a3a01eeede3af4fbbee4226859e6, ptr noundef nonnull inttoptr (i64 91 to ptr))
          to label %bb81 unwind label %cleanup7

bb81:                                             ; preds = %bb78, %bb77
  %version.sroa.128.01264.off02094 = phi i1 [ %version.sroa.128.01264.off02095, %bb78 ], [ %extract.t1848, %bb77 ]
  %version.sroa.92.012682031 = phi i8 [ %version.sroa.92.012682032, %bb78 ], [ %version1.sroa.4.7.extract.trunc, %bb77 ]
  %version.sroa.64.012702006 = phi i8 [ %version.sroa.64.012702007, %bb78 ], [ %version.sroa.64.012702018, %bb77 ]
  %version.sroa.36.012721979 = phi i16 [ %version.sroa.36.012721980, %bb78 ], [ %version.sroa.36.012721991, %bb77 ]
  %version.sroa.0.012741956 = phi i32 [ %version.sroa.0.012741957, %bb78 ], [ %version.sroa.0.012741949, %bb77 ]
  br i1 %version.sroa.128.01264.off02094, label %bb1.i350, label %bb8.i347

bb8.i347:                                         ; preds = %bb81
  %275 = icmp ugt i32 %version.sroa.0.012741956, 77
  br i1 %275, label %bb8.i363, label %bb84

bb1.i350:                                         ; preds = %bb81
  %_7.i351 = icmp ugt i32 %version.sroa.0.012741956, 78
  br i1 %_7.i351, label %bb1.i366, label %bb3.i352

bb3.i352:                                         ; preds = %bb1.i350
  %_9.i353 = icmp eq i32 %version.sroa.0.012741956, 78
  br i1 %_9.i353, label %bb4.i354, label %bb84

bb4.i354:                                         ; preds = %bb3.i352
  %276 = icmp eq i16 %version.sroa.36.012721979, 2024
  %277 = icmp ugt i16 %version.sroa.36.012721979, 2023
  br i1 %276, label %bb11.i356, label %bb82

bb11.i356:                                        ; preds = %bb4.i354
  %278 = icmp eq i8 %version.sroa.64.012702006, 3
  %279 = icmp ugt i8 %version.sroa.64.012702006, 2
  br i1 %278, label %bb12.i358, label %bb82

bb12.i358:                                        ; preds = %bb11.i356
  %280 = icmp ugt i8 %version.sroa.92.012682031, 7
  br i1 %280, label %bb89, label %bb84

bb82:                                             ; preds = %bb11.i356, %bb4.i354
  %_0.sroa.0.0.shrunk.i349 = phi i1 [ %279, %bb11.i356 ], [ %277, %bb4.i354 ]
  br i1 %_0.sroa.0.0.shrunk.i349, label %bb89, label %bb84

bb84:                                             ; preds = %bb3.i352, %bb12.i358, %bb8.i347, %bb82
  %version.sroa.0.0127419562255 = phi i32 [ %version.sroa.0.012741956, %bb3.i352 ], [ 78, %bb12.i358 ], [ %version.sroa.0.012741956, %bb8.i347 ], [ 78, %bb82 ]
  %version.sroa.36.0127219792253 = phi i16 [ %version.sroa.36.012721979, %bb3.i352 ], [ 2024, %bb12.i358 ], [ %version.sroa.36.012721979, %bb8.i347 ], [ %version.sroa.36.012721979, %bb82 ]
  %version.sroa.64.0127020062251 = phi i8 [ %version.sroa.64.012702006, %bb3.i352 ], [ 3, %bb12.i358 ], [ %version.sroa.64.012702006, %bb8.i347 ], [ %version.sroa.64.012702006, %bb82 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_5831c002d8b741107bcdaec0fae41316, ptr noundef nonnull inttoptr (i64 113 to ptr))
          to label %bb89 unwind label %cleanup7

bb8.i363:                                         ; preds = %bb26, %bb8.i347
  %version.sroa.120.012662080 = phi i32 [ %_18.sroa.5.0, %bb8.i347 ], [ %version.sroa.120.01267, %bb26 ]
  %version.sroa.92.012682040 = phi i8 [ %version.sroa.92.012682031, %bb8.i347 ], [ %version.sroa.92.01269, %bb26 ]
  %version.sroa.64.012702019 = phi i8 [ %version.sroa.64.012702006, %bb8.i347 ], [ %version.sroa.64.01271, %bb26 ]
  %version.sroa.36.012721992 = phi i16 [ %version.sroa.36.012721979, %bb8.i347 ], [ %version.sroa.36.01273, %bb26 ]
  %version.sroa.0.012741965 = phi i32 [ %version.sroa.0.012741956, %bb8.i347 ], [ %version.sroa.0.01275, %bb26 ]
  %281 = icmp ugt i32 %version.sroa.0.012741965, 82
  br i1 %281, label %bb8.i379, label %bb89

bb1.i366:                                         ; preds = %bb26, %bb1.i350
  %version.sroa.120.01266206722472267 = phi i32 [ %_18.sroa.5.0, %bb1.i350 ], [ %version.sroa.120.01267, %bb26 ]
  %version.sroa.92.01268203122492266 = phi i8 [ %version.sroa.92.012682031, %bb1.i350 ], [ %version.sroa.92.01269, %bb26 ]
  %version.sroa.64.01270200622502265 = phi i8 [ %version.sroa.64.012702006, %bb1.i350 ], [ %version.sroa.64.01271, %bb26 ]
  %version.sroa.36.01272197922522264 = phi i16 [ %version.sroa.36.012721979, %bb1.i350 ], [ %version.sroa.36.01273, %bb26 ]
  %version.sroa.0.01274195622542263 = phi i32 [ %version.sroa.0.012741956, %bb1.i350 ], [ %version.sroa.0.01275, %bb26 ]
  %_7.i367 = icmp ugt i32 %version.sroa.0.01274195622542263, 83
  br i1 %_7.i367, label %bb1.i382, label %bb3.i368

bb3.i368:                                         ; preds = %bb1.i366
  %_9.i369 = icmp eq i32 %version.sroa.0.01274195622542263, 83
  br i1 %_9.i369, label %bb4.i370, label %bb89

bb4.i370:                                         ; preds = %bb3.i368
  %282 = icmp eq i16 %version.sroa.36.01272197922522264, 2024
  %283 = icmp ugt i16 %version.sroa.36.01272197922522264, 2023
  br i1 %282, label %bb11.i372, label %bb87

bb11.i372:                                        ; preds = %bb4.i370
  %284 = icmp eq i8 %version.sroa.64.01270200622502265, 9
  %285 = icmp ugt i8 %version.sroa.64.01270200622502265, 8
  br i1 %284, label %bb12.i374, label %bb87

bb12.i374:                                        ; preds = %bb11.i372
  %286 = icmp ugt i8 %version.sroa.92.01268203122492266, 14
  br i1 %286, label %bb94, label %bb89

bb87:                                             ; preds = %bb11.i372, %bb4.i370
  %_0.sroa.0.0.shrunk.i365 = phi i1 [ %285, %bb11.i372 ], [ %283, %bb4.i370 ]
  br i1 %_0.sroa.0.0.shrunk.i365, label %bb94, label %bb89

bb89:                                             ; preds = %bb12.i200, %bb27, %bb8.i, %bb82, %bb12.i358, %bb84, %bb3.i368, %bb12.i374, %bb8.i363, %bb87
  %version.sroa.128.01264.off02093 = phi i1 [ true, %bb82 ], [ true, %bb12.i358 ], [ %version.sroa.128.01264.off02094, %bb84 ], [ true, %bb3.i368 ], [ true, %bb12.i374 ], [ false, %bb8.i363 ], [ true, %bb87 ], [ false, %bb8.i ], [ true, %bb27 ], [ true, %bb12.i200 ]
  %version.sroa.120.012662066 = phi i32 [ %_18.sroa.5.0, %bb82 ], [ %_18.sroa.5.0, %bb12.i358 ], [ %_18.sroa.5.0, %bb84 ], [ %version.sroa.120.01266206722472267, %bb3.i368 ], [ %version.sroa.120.01266206722472267, %bb12.i374 ], [ %version.sroa.120.012662080, %bb8.i363 ], [ %version.sroa.120.01266206722472267, %bb87 ], [ %_18.sroa.5.0, %bb8.i ], [ %_18.sroa.5.0, %bb27 ], [ %_18.sroa.5.0, %bb12.i200 ]
  %version.sroa.92.012682038 = phi i8 [ %version.sroa.92.012682031, %bb82 ], [ %version.sroa.92.012682031, %bb12.i358 ], [ %version.sroa.92.012682031, %bb84 ], [ %version.sroa.92.01268203122492266, %bb3.i368 ], [ %version.sroa.92.01268203122492266, %bb12.i374 ], [ %version.sroa.92.012682040, %bb8.i363 ], [ %version.sroa.92.01268203122492266, %bb87 ], [ %version1.sroa.4.7.extract.trunc, %bb8.i ], [ %version1.sroa.4.7.extract.trunc, %bb27 ], [ %version1.sroa.4.7.extract.trunc, %bb12.i200 ]
  %version.sroa.64.012702005 = phi i8 [ %version.sroa.64.012702006, %bb82 ], [ 3, %bb12.i358 ], [ %version.sroa.64.0127020062251, %bb84 ], [ %version.sroa.64.01270200622502265, %bb3.i368 ], [ 9, %bb12.i374 ], [ %version.sroa.64.012702019, %bb8.i363 ], [ %version.sroa.64.01270200622502265, %bb87 ], [ %version1.sroa.4.6.extract.trunc, %bb8.i ], [ %version1.sroa.4.6.extract.trunc, %bb27 ], [ 4, %bb12.i200 ]
  %version.sroa.36.012721978 = phi i16 [ %version.sroa.36.012721979, %bb82 ], [ 2024, %bb12.i358 ], [ %version.sroa.36.0127219792253, %bb84 ], [ %version.sroa.36.01272197922522264, %bb3.i368 ], [ 2024, %bb12.i374 ], [ %version.sroa.36.012721992, %bb8.i363 ], [ %version.sroa.36.01272197922522264, %bb87 ], [ %version1.sroa.4.4.extract.trunc, %bb8.i ], [ %version1.sroa.4.4.extract.trunc, %bb27 ], [ 2024, %bb12.i200 ]
  %version.sroa.0.012741964 = phi i32 [ 78, %bb82 ], [ 78, %bb12.i358 ], [ %version.sroa.0.0127419562255, %bb84 ], [ %version.sroa.0.01274195622542263, %bb3.i368 ], [ 83, %bb12.i374 ], [ %version.sroa.0.012741965, %bb8.i363 ], [ 83, %bb87 ], [ 79, %bb8.i ], [ 79, %bb27 ], [ 79, %bb12.i200 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_2e9b5516734e64b9676c629692898b2f, ptr noundef nonnull inttoptr (i64 101 to ptr))
          to label %bb94 unwind label %cleanup7

bb8.i379:                                         ; preds = %bb8.i363
  %.not1853 = icmp eq i32 %version.sroa.0.012741965, 83
  br i1 %.not1853, label %bb94, label %bb126

bb1.i382:                                         ; preds = %bb1.i366
  %_7.i383.not = icmp eq i32 %version.sroa.0.01274195622542263, 84
  br i1 %_7.i383.not, label %bb4.i386, label %bb1.i430

bb4.i386:                                         ; preds = %bb1.i382
  %287 = icmp eq i16 %version.sroa.36.01272197922522264, 2024
  %288 = icmp ugt i16 %version.sroa.36.01272197922522264, 2023
  br i1 %287, label %bb11.i388, label %bb92

bb11.i388:                                        ; preds = %bb4.i386
  %289 = icmp eq i8 %version.sroa.64.01270200622502265, 10
  %290 = icmp ugt i8 %version.sroa.64.01270200622502265, 9
  br i1 %289, label %bb12.i390, label %bb92

bb12.i390:                                        ; preds = %bb11.i388
  %291 = icmp ugt i8 %version.sroa.92.01268203122492266, 20
  br i1 %291, label %bb102, label %bb94

bb92:                                             ; preds = %bb11.i388, %bb4.i386
  %_0.sroa.0.0.shrunk.i381 = phi i1 [ %290, %bb11.i388 ], [ %288, %bb4.i386 ]
  br i1 %_0.sroa.0.0.shrunk.i381, label %bb102, label %bb94

bb94:                                             ; preds = %bb87, %bb12.i374, %bb89, %bb12.i390, %bb8.i379, %bb92
  %version.sroa.128.01264.off02092 = phi i1 [ true, %bb87 ], [ true, %bb12.i374 ], [ %version.sroa.128.01264.off02093, %bb89 ], [ true, %bb12.i390 ], [ false, %bb8.i379 ], [ true, %bb92 ]
  %version.sroa.120.012662065 = phi i32 [ %version.sroa.120.01266206722472267, %bb87 ], [ %version.sroa.120.01266206722472267, %bb12.i374 ], [ %version.sroa.120.012662066, %bb89 ], [ %version.sroa.120.01266206722472267, %bb12.i390 ], [ %version.sroa.120.012662080, %bb8.i379 ], [ %version.sroa.120.01266206722472267, %bb92 ]
  %version.sroa.92.012682039 = phi i8 [ %version.sroa.92.01268203122492266, %bb87 ], [ %version.sroa.92.01268203122492266, %bb12.i374 ], [ %version.sroa.92.012682038, %bb89 ], [ %version.sroa.92.01268203122492266, %bb12.i390 ], [ %version.sroa.92.012682040, %bb8.i379 ], [ %version.sroa.92.01268203122492266, %bb92 ]
  %version.sroa.64.012702004 = phi i8 [ %version.sroa.64.01270200622502265, %bb87 ], [ 9, %bb12.i374 ], [ %version.sroa.64.012702005, %bb89 ], [ 10, %bb12.i390 ], [ %version.sroa.64.012702019, %bb8.i379 ], [ %version.sroa.64.01270200622502265, %bb92 ]
  %version.sroa.36.012721977 = phi i16 [ %version.sroa.36.01272197922522264, %bb87 ], [ 2024, %bb12.i374 ], [ %version.sroa.36.012721978, %bb89 ], [ 2024, %bb12.i390 ], [ %version.sroa.36.012721992, %bb8.i379 ], [ %version.sroa.36.01272197922522264, %bb92 ]
  %version.sroa.0.012741963 = phi i32 [ 83, %bb87 ], [ 83, %bb12.i374 ], [ %version.sroa.0.012741964, %bb89 ], [ 84, %bb12.i390 ], [ 83, %bb8.i379 ], [ 84, %bb92 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_638a7631d4f759bf1aef9d06869234dd, ptr noundef nonnull inttoptr (i64 107 to ptr))
          to label %bb96 unwind label %cleanup7

bb96:                                             ; preds = %bb94
  br i1 %version.sroa.128.01264.off02092, label %bb1.i414, label %bb8.i395

bb8.i395:                                         ; preds = %bb96
  %292 = icmp samesign ugt i32 %version.sroa.0.012741963, 58
  br i1 %292, label %bb126, label %bb124

bb1.i398:                                         ; preds = %bb3.i416
  %_7.i399 = icmp samesign ugt i32 %version.sroa.0.012741963, 59
  br i1 %_7.i399, label %bb126, label %bb3.i400

bb3.i400:                                         ; preds = %bb1.i398
  %_9.i401 = icmp eq i32 %version.sroa.0.012741963, 59
  br i1 %_9.i401, label %bb4.i402, label %bb1.i532

bb4.i402:                                         ; preds = %bb3.i400
  %293 = icmp eq i16 %version.sroa.36.012721977, 2021
  %294 = icmp ugt i16 %version.sroa.36.012721977, 2020
  br i1 %293, label %bb11.i404, label %bb110

bb11.i404:                                        ; preds = %bb4.i402
  %295 = icmp eq i8 %version.sroa.64.012702004, 12
  %296 = icmp ugt i8 %version.sroa.64.012702004, 11
  br i1 %295, label %bb12.i406, label %bb110

bb12.i406:                                        ; preds = %bb11.i404
  %297 = icmp ugt i8 %version.sroa.92.012682039, 14
  br i1 %297, label %bb126, label %bb114

bb1.i414:                                         ; preds = %bb96
  %_7.i415 = icmp samesign ugt i32 %version.sroa.0.012741963, 64
  br i1 %_7.i415, label %bb102, label %bb3.i416

bb3.i416:                                         ; preds = %bb1.i414
  %_9.i417 = icmp eq i32 %version.sroa.0.012741963, 64
  br i1 %_9.i417, label %bb4.i418, label %bb1.i398

bb4.i418:                                         ; preds = %bb3.i416
  %298 = icmp eq i16 %version.sroa.36.012721977, 2022
  %299 = icmp ugt i16 %version.sroa.36.012721977, 2021
  br i1 %298, label %bb11.i420, label %bb98

bb11.i420:                                        ; preds = %bb4.i418
  %300 = icmp eq i8 %version.sroa.64.012702004, 6
  %301 = icmp ugt i8 %version.sroa.64.012702004, 5
  br i1 %300, label %bb12.i422, label %bb98

bb12.i422:                                        ; preds = %bb11.i420
  %302 = icmp ugt i8 %version.sroa.92.012682039, 28
  br i1 %302, label %bb102, label %bb126

bb98:                                             ; preds = %bb11.i420, %bb4.i418
  %_0.sroa.0.0.shrunk.i413 = phi i1 [ %301, %bb11.i420 ], [ %299, %bb4.i418 ]
  br i1 %_0.sroa.0.0.shrunk.i413, label %bb102, label %bb126

bb1.i430:                                         ; preds = %bb1.i382
  %_7.i431 = icmp ugt i32 %version.sroa.0.01274195622542263, 89
  br i1 %_7.i431, label %bb126, label %bb3.i432

bb3.i432:                                         ; preds = %bb1.i430
  %_9.i433 = icmp eq i32 %version.sroa.0.01274195622542263, 89
  br i1 %_9.i433, label %bb4.i434, label %bb102

bb4.i434:                                         ; preds = %bb3.i432
  %303 = icmp eq i16 %version.sroa.36.01272197922522264, 2025
  %304 = icmp ugt i16 %version.sroa.36.01272197922522264, 2024
  br i1 %303, label %bb11.i436, label %bb100

bb11.i436:                                        ; preds = %bb4.i434
  %305 = icmp eq i8 %version.sroa.64.01270200622502265, 5
  %306 = icmp ugt i8 %version.sroa.64.01270200622502265, 4
  br i1 %305, label %bb12.i438, label %bb100

bb12.i438:                                        ; preds = %bb11.i436
  %307 = icmp ugt i8 %version.sroa.92.01268203122492266, 29
  br i1 %307, label %bb126, label %bb102

bb100:                                            ; preds = %bb11.i436, %bb4.i434
  %_0.sroa.0.0.shrunk.i429 = phi i1 [ %306, %bb11.i436 ], [ %304, %bb4.i434 ]
  br i1 %_0.sroa.0.0.shrunk.i429, label %bb126, label %bb102

bb102:                                            ; preds = %bb92, %bb12.i390, %bb12.i422, %bb98, %bb1.i414, %bb3.i432, %bb12.i438, %bb100
  %version.sroa.120.012662064 = phi i32 [ %version.sroa.120.01266206722472267, %bb92 ], [ %version.sroa.120.01266206722472267, %bb12.i390 ], [ %version.sroa.120.012662065, %bb12.i422 ], [ %version.sroa.120.012662065, %bb98 ], [ %version.sroa.120.012662065, %bb1.i414 ], [ %version.sroa.120.01266206722472267, %bb3.i432 ], [ %version.sroa.120.01266206722472267, %bb12.i438 ], [ %version.sroa.120.01266206722472267, %bb100 ]
  %version.sroa.92.012682043 = phi i8 [ %version.sroa.92.01268203122492266, %bb92 ], [ %version.sroa.92.01268203122492266, %bb12.i390 ], [ %version.sroa.92.012682039, %bb12.i422 ], [ %version.sroa.92.012682039, %bb98 ], [ %version.sroa.92.012682039, %bb1.i414 ], [ %version.sroa.92.01268203122492266, %bb3.i432 ], [ %version.sroa.92.01268203122492266, %bb12.i438 ], [ %version.sroa.92.01268203122492266, %bb100 ]
  %version.sroa.64.012702003 = phi i8 [ %version.sroa.64.01270200622502265, %bb92 ], [ 10, %bb12.i390 ], [ 6, %bb12.i422 ], [ %version.sroa.64.012702004, %bb98 ], [ %version.sroa.64.012702004, %bb1.i414 ], [ %version.sroa.64.01270200622502265, %bb3.i432 ], [ 5, %bb12.i438 ], [ %version.sroa.64.01270200622502265, %bb100 ]
  %version.sroa.36.012721976 = phi i16 [ %version.sroa.36.01272197922522264, %bb92 ], [ 2024, %bb12.i390 ], [ 2022, %bb12.i422 ], [ %version.sroa.36.012721977, %bb98 ], [ %version.sroa.36.012721977, %bb1.i414 ], [ %version.sroa.36.01272197922522264, %bb3.i432 ], [ 2025, %bb12.i438 ], [ %version.sroa.36.01272197922522264, %bb100 ]
  %version.sroa.0.012741967 = phi i32 [ 84, %bb92 ], [ 84, %bb12.i390 ], [ 64, %bb12.i422 ], [ 64, %bb98 ], [ %version.sroa.0.012741963, %bb1.i414 ], [ %version.sroa.0.01274195622542263, %bb3.i432 ], [ 89, %bb12.i438 ], [ 89, %bb100 ]
  %_3.not.i = icmp eq i64 %_12.sroa.8.0.copyload, 9
  br i1 %_3.not.i, label %bb390, label %bb105

bb390:                                            ; preds = %bb102
  %308 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(9) @alloc_fa1130f2f45123ef906740f12b430906, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !667
  %309 = icmp eq i32 %308, 0
  %_108 = icmp ult i32 %version.sroa.120.012662064, 15
  %or.cond.not = select i1 %309, i1 %_108, i1 false
  br i1 %or.cond.not, label %bb395, label %bb105

bb105:                                            ; preds = %bb102, %bb390
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_2c4353bbb852c5a5c07b390c882774d5, ptr noundef nonnull inttoptr (i64 101 to ptr))
          to label %bb126 unwind label %cleanup7

bb110:                                            ; preds = %bb11.i404, %bb4.i402
  %_0.sroa.0.0.shrunk.i397 = phi i1 [ %296, %bb11.i404 ], [ %294, %bb4.i402 ]
  br i1 %_0.sroa.0.0.shrunk.i397, label %bb126, label %bb114

bb126:                                            ; preds = %bb105, %bb1.i430, %bb98, %bb100, %bb12.i422, %bb12.i438, %bb8.i379, %bb1.i398, %bb12.i406, %bb8.i395, %bb110
  %version.sroa.128.01264.off02089 = phi i1 [ true, %bb110 ], [ false, %bb8.i395 ], [ true, %bb12.i406 ], [ true, %bb1.i398 ], [ false, %bb8.i379 ], [ true, %bb12.i438 ], [ true, %bb12.i422 ], [ true, %bb100 ], [ true, %bb98 ], [ true, %bb1.i430 ], [ true, %bb105 ]
  %version.sroa.120.012662062 = phi i32 [ %version.sroa.120.012662065, %bb110 ], [ %version.sroa.120.012662065, %bb8.i395 ], [ %version.sroa.120.012662065, %bb12.i406 ], [ %version.sroa.120.012662065, %bb1.i398 ], [ %version.sroa.120.012662080, %bb8.i379 ], [ %version.sroa.120.01266206722472267, %bb12.i438 ], [ %version.sroa.120.012662065, %bb12.i422 ], [ %version.sroa.120.01266206722472267, %bb100 ], [ %version.sroa.120.012662065, %bb98 ], [ %version.sroa.120.01266206722472267, %bb1.i430 ], [ %version.sroa.120.012662064, %bb105 ]
  %version.sroa.92.012682044 = phi i8 [ %version.sroa.92.012682039, %bb110 ], [ %version.sroa.92.012682039, %bb8.i395 ], [ %version.sroa.92.012682039, %bb12.i406 ], [ %version.sroa.92.012682039, %bb1.i398 ], [ %version.sroa.92.012682040, %bb8.i379 ], [ %version.sroa.92.01268203122492266, %bb12.i438 ], [ %version.sroa.92.012682039, %bb12.i422 ], [ %version.sroa.92.01268203122492266, %bb100 ], [ %version.sroa.92.012682039, %bb98 ], [ %version.sroa.92.01268203122492266, %bb1.i430 ], [ %version.sroa.92.012682043, %bb105 ]
  %version.sroa.64.012702001 = phi i8 [ %version.sroa.64.012702004, %bb110 ], [ %version.sroa.64.012702004, %bb8.i395 ], [ 12, %bb12.i406 ], [ %version.sroa.64.012702004, %bb1.i398 ], [ %version.sroa.64.012702019, %bb8.i379 ], [ 5, %bb12.i438 ], [ 6, %bb12.i422 ], [ %version.sroa.64.01270200622502265, %bb100 ], [ %version.sroa.64.012702004, %bb98 ], [ %version.sroa.64.01270200622502265, %bb1.i430 ], [ %version.sroa.64.012702003, %bb105 ]
  %version.sroa.36.012721974 = phi i16 [ %version.sroa.36.012721977, %bb110 ], [ %version.sroa.36.012721977, %bb8.i395 ], [ 2021, %bb12.i406 ], [ %version.sroa.36.012721977, %bb1.i398 ], [ %version.sroa.36.012721992, %bb8.i379 ], [ 2025, %bb12.i438 ], [ 2022, %bb12.i422 ], [ %version.sroa.36.01272197922522264, %bb100 ], [ %version.sroa.36.012721977, %bb98 ], [ %version.sroa.36.01272197922522264, %bb1.i430 ], [ %version.sroa.36.012721976, %bb105 ]
  %version.sroa.0.0127613671387 = phi i32 [ 59, %bb110 ], [ %version.sroa.0.012741963, %bb8.i395 ], [ 59, %bb12.i406 ], [ %version.sroa.0.012741963, %bb1.i398 ], [ %version.sroa.0.012741965, %bb8.i379 ], [ 89, %bb12.i438 ], [ 64, %bb12.i422 ], [ 89, %bb100 ], [ 64, %bb98 ], [ %version.sroa.0.01274195622542263, %bb1.i430 ], [ %version.sroa.0.012741967, %bb105 ]
  switch i64 %_12.sroa.8.0.copyload, label %bb155 [
    i64 7, label %bb393
    i64 5, label %bb394
    i64 9, label %bb395
  ]

bb393:                                            ; preds = %bb126
  %310 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_77091ef4013986fd40216f126dabc12f, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !671
  %311 = icmp eq i32 %310, 0
  br i1 %311, label %bb130, label %bb155

bb130:                                            ; preds = %bb394, %bb393
  br i1 %version.sroa.128.01264.off02089, label %bb1.i458, label %bb8.i455

bb8.i455:                                         ; preds = %bb130
  %312 = icmp ugt i32 %version.sroa.0.0127613671387, 83
  br i1 %312, label %bb13.lr.ph.i630, label %bb143

bb1.i458:                                         ; preds = %bb130
  %_7.i459 = icmp ugt i32 %version.sroa.0.0127613671387, 84
  br i1 %_7.i459, label %bb13.lr.ph.i630, label %bb3.i460

bb3.i460:                                         ; preds = %bb1.i458
  %_9.i461 = icmp eq i32 %version.sroa.0.0127613671387, 84
  br i1 %_9.i461, label %bb4.i462, label %bb134

bb4.i462:                                         ; preds = %bb3.i460
  %313 = icmp eq i16 %version.sroa.36.012721974, 2024
  %314 = icmp ugt i16 %version.sroa.36.012721974, 2023
  br i1 %313, label %bb11.i464, label %bb131

bb11.i464:                                        ; preds = %bb4.i462
  %315 = icmp eq i8 %version.sroa.64.012702001, 11
  %316 = icmp ugt i8 %version.sroa.64.012702001, 10
  br i1 %315, label %bb12.i466, label %bb131

bb12.i466:                                        ; preds = %bb11.i464
  %317 = icmp ugt i8 %version.sroa.92.012682044, 9
  br i1 %317, label %bb13.lr.ph.i630, label %bb134

bb394:                                            ; preds = %bb126
  %318 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_5a449a0bdb20d0cd84a204297ad784b7, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !675
  %319 = icmp eq i32 %318, 0
  br i1 %319, label %bb130, label %bb155

bb395:                                            ; preds = %bb390, %bb126
  %version.sroa.0.01276136713872287 = phi i32 [ %version.sroa.0.0127613671387, %bb126 ], [ %version.sroa.0.012741967, %bb390 ]
  %version.sroa.36.0127219742286 = phi i16 [ %version.sroa.36.012721974, %bb126 ], [ %version.sroa.36.012721976, %bb390 ]
  %version.sroa.64.0127020012285 = phi i8 [ %version.sroa.64.012702001, %bb126 ], [ %version.sroa.64.012702003, %bb390 ]
  %version.sroa.92.0126820442284 = phi i8 [ %version.sroa.92.012682044, %bb126 ], [ %version.sroa.92.012682043, %bb390 ]
  %version.sroa.120.0126620622283 = phi i32 [ %version.sroa.120.012662062, %bb126 ], [ %version.sroa.120.012662064, %bb390 ]
  %version.sroa.128.01264.off020892282 = phi i1 [ %version.sroa.128.01264.off02089, %bb126 ], [ true, %bb390 ]
  %320 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(9) @alloc_fa1130f2f45123ef906740f12b430906, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !679
  %321 = icmp eq i32 %320, 0
  %or.cond19 = and i1 %version.sroa.128.01264.off020892282, %321
  br i1 %or.cond19, label %bb1.i495, label %bb155

bb155:                                            ; preds = %bb126, %bb394, %bb393, %bb124, %bb143, %bb395
  %version.sroa.128.01264.off02087 = phi i1 [ %version.sroa.128.01264.off02092, %bb124 ], [ %version.sroa.128.01264.off02089, %bb143 ], [ %version.sroa.128.01264.off020892282, %bb395 ], [ %version.sroa.128.01264.off02089, %bb393 ], [ %version.sroa.128.01264.off02089, %bb394 ], [ %version.sroa.128.01264.off02089, %bb126 ]
  %version.sroa.120.012662060 = phi i32 [ %version.sroa.120.012662065, %bb124 ], [ %version.sroa.120.012662062, %bb143 ], [ %version.sroa.120.0126620622283, %bb395 ], [ %version.sroa.120.012662062, %bb393 ], [ %version.sroa.120.012662062, %bb394 ], [ %version.sroa.120.012662062, %bb126 ]
  %version.sroa.92.012682048 = phi i8 [ %version.sroa.92.012682039, %bb124 ], [ %version.sroa.92.012682044, %bb143 ], [ %version.sroa.92.0126820442284, %bb395 ], [ %version.sroa.92.012682044, %bb393 ], [ %version.sroa.92.012682044, %bb394 ], [ %version.sroa.92.012682044, %bb126 ]
  %version.sroa.64.012701999 = phi i8 [ %version.sroa.64.012702000, %bb124 ], [ %version.sroa.64.012702001, %bb143 ], [ %version.sroa.64.0127020012285, %bb395 ], [ %version.sroa.64.012702001, %bb393 ], [ %version.sroa.64.012702001, %bb394 ], [ %version.sroa.64.012702001, %bb126 ]
  %version.sroa.36.012721972 = phi i16 [ %version.sroa.36.012721973, %bb124 ], [ %version.sroa.36.012721974, %bb143 ], [ %version.sroa.36.0127219742286, %bb395 ], [ %version.sroa.36.012721974, %bb393 ], [ %version.sroa.36.012721974, %bb394 ], [ %version.sroa.36.012721974, %bb126 ]
  %version.sroa.0.01277 = phi i32 [ %version.sroa.0.01276136713911393, %bb124 ], [ %version.sroa.0.01276136713872288, %bb143 ], [ %version.sroa.0.01276136713872287, %bb395 ], [ %version.sroa.0.0127613671387, %bb393 ], [ %version.sroa.0.0127613671387, %bb394 ], [ %version.sroa.0.0127613671387, %bb126 ]
  br i1 %version.sroa.128.01264.off02087, label %bb1.i479, label %bb8.i476

bb8.i476:                                         ; preds = %bb155
  %322 = icmp ugt i32 %version.sroa.0.01277, 59
  br i1 %322, label %bb13.lr.ph.i630, label %bb166

bb1.i479:                                         ; preds = %bb150, %bb151, %bb140, %bb155
  %version.sroa.128.01264.off02086 = phi i1 [ true, %bb155 ], [ true, %bb140 ], [ %version.sroa.128.01264.off020892282, %bb151 ], [ %version.sroa.128.01264.off020892282, %bb150 ]
  %version.sroa.120.012662059 = phi i32 [ %version.sroa.120.012662060, %bb155 ], [ %version.sroa.120.012662062, %bb140 ], [ %version.sroa.120.0126620622283, %bb151 ], [ %version.sroa.120.0126620622283, %bb150 ]
  %version.sroa.92.012682046 = phi i8 [ %version.sroa.92.012682048, %bb155 ], [ %version.sroa.92.012682044, %bb140 ], [ %version.sroa.92.0126820442284, %bb151 ], [ %version.sroa.92.0126820442284, %bb150 ]
  %version.sroa.64.012701998 = phi i8 [ %version.sroa.64.012701999, %bb155 ], [ %version.sroa.64.012702001, %bb140 ], [ %version.sroa.64.0127020012285, %bb151 ], [ %version.sroa.64.0127020012285, %bb150 ]
  %version.sroa.36.012721971 = phi i16 [ %version.sroa.36.012721972, %bb155 ], [ %version.sroa.36.012721974, %bb140 ], [ %version.sroa.36.0127219742286, %bb151 ], [ %version.sroa.36.0127219742286, %bb150 ]
  %version.sroa.0.012771404 = phi i32 [ %version.sroa.0.01277, %bb155 ], [ %version.sroa.0.0127613671387, %bb140 ], [ %version.sroa.0.01276136713872287, %bb151 ], [ %version.sroa.0.01276136713872287, %bb150 ]
  %_7.i480 = icmp ugt i32 %version.sroa.0.012771404, 60
  br i1 %_7.i480, label %bb13.lr.ph.i630, label %bb3.i481

bb3.i481:                                         ; preds = %bb1.i479
  %_9.i482 = icmp eq i32 %version.sroa.0.012771404, 60
  br i1 %_9.i482, label %bb4.i483, label %bb1.i558

bb4.i483:                                         ; preds = %bb148, %bb3.i481
  %version.sroa.128.01264.off0208623062471 = phi i1 [ %version.sroa.128.01264.off02086, %bb3.i481 ], [ %version.sroa.128.01264.off020892282, %bb148 ]
  %version.sroa.120.01266205923072469 = phi i32 [ %version.sroa.120.012662059, %bb3.i481 ], [ %version.sroa.120.0126620622283, %bb148 ]
  %version.sroa.92.01268204623082467 = phi i8 [ %version.sroa.92.012682046, %bb3.i481 ], [ %version.sroa.92.0126820442284, %bb148 ]
  %version.sroa.64.01270199823092466 = phi i8 [ %version.sroa.64.012701998, %bb3.i481 ], [ %version.sroa.64.0127020012285, %bb148 ]
  %version.sroa.36.01272197123102464 = phi i16 [ %version.sroa.36.012721971, %bb3.i481 ], [ %version.sroa.36.0127219742286, %bb148 ]
  %323 = icmp eq i16 %version.sroa.36.01272197123102464, 2022
  %324 = icmp ugt i16 %version.sroa.36.01272197123102464, 2021
  br i1 %323, label %bb11.i485, label %bb156

bb11.i485:                                        ; preds = %bb4.i483
  %325 = icmp eq i8 %version.sroa.64.01270199823092466, 2
  %326 = icmp ugt i8 %version.sroa.64.01270199823092466, 1
  br i1 %325, label %bb12.i487, label %bb156

bb12.i487:                                        ; preds = %bb12.i503, %bb11.i485
  %version.sroa.128.01264.off020862306247124992517 = phi i1 [ %version.sroa.128.01264.off0208623062471, %bb11.i485 ], [ %version.sroa.128.01264.off020892282, %bb12.i503 ]
  %version.sroa.120.0126620592307246925012516 = phi i32 [ %version.sroa.120.01266205923072469, %bb11.i485 ], [ %version.sroa.120.0126620622283, %bb12.i503 ]
  %version.sroa.92.0126820462308246725022515 = phi i8 [ %version.sroa.92.01268204623082467, %bb11.i485 ], [ %version.sroa.92.0126820442284, %bb12.i503 ]
  %327 = icmp ugt i8 %version.sroa.92.0126820462308246725022515, 9
  br i1 %327, label %bb13.lr.ph.i630, label %bb161

bb1.i495:                                         ; preds = %bb395
  %_7.i496 = icmp ugt i32 %version.sroa.0.01276136713872287, 60
  br i1 %_7.i496, label %bb149, label %bb3.i497

bb3.i497:                                         ; preds = %bb1.i495
  %_9.i498 = icmp eq i32 %version.sroa.0.01276136713872287, 60
  br i1 %_9.i498, label %bb4.i499, label %bb161

bb4.i499:                                         ; preds = %bb3.i497
  %328 = icmp eq i16 %version.sroa.36.0127219742286, 2022
  %329 = icmp ugt i16 %version.sroa.36.0127219742286, 2021
  br i1 %328, label %bb11.i501, label %bb148

bb11.i501:                                        ; preds = %bb4.i499
  %330 = icmp eq i8 %version.sroa.64.0127020012285, 2
  %331 = icmp ugt i8 %version.sroa.64.0127020012285, 1
  br i1 %330, label %bb12.i503, label %bb148

bb12.i503:                                        ; preds = %bb11.i501
  %332 = icmp ugt i8 %version.sroa.92.0126820442284, 11
  br i1 %332, label %bb149, label %bb12.i487

bb148:                                            ; preds = %bb11.i501, %bb4.i499
  %_0.sroa.0.0.shrunk.i494 = phi i1 [ %331, %bb11.i501 ], [ %329, %bb4.i499 ]
  br i1 %_0.sroa.0.0.shrunk.i494, label %bb149, label %bb4.i483

bb149:                                            ; preds = %bb1.i495, %bb12.i503, %bb148
; invoke build_script_build::is_allowed_feature
  %_144 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d56840a3539592a0b21e632750debc01, i64 noundef 21)
          to label %bb150 unwind label %cleanup7

bb150:                                            ; preds = %bb149
  br i1 %_144, label %bb151, label %bb1.i479

bb151:                                            ; preds = %bb150
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a416d606bb0533c9dea6d062122086f5, ptr noundef nonnull inttoptr (i64 127 to ptr))
          to label %bb1.i479 unwind label %cleanup7

bb131:                                            ; preds = %bb11.i464, %bb4.i462
  %_0.sroa.0.0.shrunk.i457 = phi i1 [ %316, %bb11.i464 ], [ %314, %bb4.i462 ]
  br i1 %_0.sroa.0.0.shrunk.i457, label %bb13.lr.ph.i630, label %bb134

bb143:                                            ; preds = %bb3.i518, %bb12.i524, %bb8.i455, %bb136, %bb139
  %version.sroa.0.01276136713872288 = phi i32 [ %version.sroa.0.0127613671387, %bb3.i518 ], [ 71, %bb12.i524 ], [ %version.sroa.0.0127613671387, %bb8.i455 ], [ 71, %bb136 ], [ %version.sroa.0.0127613671387, %bb139 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_4a7c5e08e7b916cb34cb4d73d8036fc8, ptr noundef nonnull inttoptr (i64 79 to ptr))
          to label %bb155 unwind label %cleanup7

bb134:                                            ; preds = %bb3.i460, %bb12.i466, %bb131
  %_3.not.i506 = icmp eq i64 %_12.sroa.8.0.copyload, 5
  br i1 %_3.not.i506, label %bb396, label %bb138

bb396:                                            ; preds = %bb134
  %333 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_5a449a0bdb20d0cd84a204297ad784b7, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !683
  %334 = icmp ne i32 %333, 0
  %_7.i517 = icmp samesign ugt i32 %version.sroa.0.0127613671387, 71
  %or.cond = select i1 %334, i1 true, i1 %_7.i517
  br i1 %or.cond, label %bb138, label %bb3.i518

bb3.i518:                                         ; preds = %bb396
  %_9.i519 = icmp eq i32 %version.sroa.0.0127613671387, 71
  br i1 %_9.i519, label %bb4.i520, label %bb143

bb4.i520:                                         ; preds = %bb3.i518
  %335 = icmp eq i16 %version.sroa.36.012721974, 2023
  %336 = icmp ugt i16 %version.sroa.36.012721974, 2022
  br i1 %335, label %bb11.i522, label %bb136

bb11.i522:                                        ; preds = %bb4.i520
  %337 = icmp eq i8 %version.sroa.64.012702001, 5
  %338 = icmp ugt i8 %version.sroa.64.012702001, 4
  br i1 %337, label %bb12.i524, label %bb136

bb12.i524:                                        ; preds = %bb11.i522
  %339 = icmp ugt i8 %version.sroa.92.012682044, 7
  br i1 %339, label %bb138, label %bb143

bb138:                                            ; preds = %bb134, %bb12.i524, %bb136, %bb396
; invoke build_script_build::is_allowed_feature
  %_136 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d56840a3539592a0b21e632750debc01, i64 noundef 21)
          to label %bb139 unwind label %cleanup7

bb136:                                            ; preds = %bb11.i522, %bb4.i520
  %_0.sroa.0.0.shrunk.i515 = phi i1 [ %338, %bb11.i522 ], [ %336, %bb4.i520 ]
  br i1 %_0.sroa.0.0.shrunk.i515, label %bb138, label %bb143

bb139:                                            ; preds = %bb138
  br i1 %_136, label %bb140, label %bb143

bb140:                                            ; preds = %bb139
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a416d606bb0533c9dea6d062122086f5, ptr noundef nonnull inttoptr (i64 127 to ptr))
          to label %bb1.i479 unwind label %cleanup7

bb124:                                            ; preds = %bb391, %bb3.i534, %bb12.i540, %bb8.i395, %bb392, %bb120, %bb113, %bb119
  %version.sroa.64.012702000 = phi i8 [ %version.sroa.64.012702004, %bb392 ], [ %version.sroa.64.012702004, %bb120 ], [ %version.sroa.64.012702004, %bb113 ], [ %version.sroa.64.012702004, %bb119 ], [ %version.sroa.64.012702004, %bb8.i395 ], [ 6, %bb12.i540 ], [ %version.sroa.64.012702004, %bb3.i534 ], [ %version.sroa.64.012702004, %bb391 ]
  %version.sroa.36.012721973 = phi i16 [ %version.sroa.36.012721977, %bb392 ], [ %version.sroa.36.012721977, %bb120 ], [ %version.sroa.36.012721977, %bb113 ], [ %version.sroa.36.012721977, %bb119 ], [ %version.sroa.36.012721977, %bb8.i395 ], [ 2020, %bb12.i540 ], [ %version.sroa.36.012721977, %bb3.i534 ], [ %version.sroa.36.012721977, %bb391 ]
  %version.sroa.0.01276136713911393 = phi i32 [ %version.sroa.0.012741963, %bb392 ], [ %version.sroa.0.012741963, %bb120 ], [ 46, %bb113 ], [ %version.sroa.0.012741963, %bb119 ], [ %version.sroa.0.012741963, %bb8.i395 ], [ 46, %bb12.i540 ], [ %version.sroa.0.012741963, %bb3.i534 ], [ %version.sroa.0.012741963, %bb391 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_4a7c5e08e7b916cb34cb4d73d8036fc8, ptr noundef nonnull inttoptr (i64 79 to ptr))
          to label %bb155 unwind label %cleanup7

bb1.i532:                                         ; preds = %bb3.i400
  %_7.i533 = icmp samesign ugt i32 %version.sroa.0.012741963, 46
  br i1 %_7.i533, label %bb114, label %bb3.i534

bb3.i534:                                         ; preds = %bb1.i532
  %_9.i535 = icmp eq i32 %version.sroa.0.012741963, 46
  br i1 %_9.i535, label %bb4.i536, label %bb124

bb4.i536:                                         ; preds = %bb3.i534
  %340 = icmp eq i16 %version.sroa.36.012721977, 2020
  %341 = icmp ugt i16 %version.sroa.36.012721977, 2019
  br i1 %340, label %bb11.i538, label %bb113

bb11.i538:                                        ; preds = %bb4.i536
  %342 = icmp eq i8 %version.sroa.64.012702004, 6
  %343 = icmp ugt i8 %version.sroa.64.012702004, 5
  br i1 %342, label %bb12.i540, label %bb113

bb12.i540:                                        ; preds = %bb11.i538
  %344 = icmp ugt i8 %version.sroa.92.012682039, 19
  br i1 %344, label %bb114, label %bb124

bb113:                                            ; preds = %bb11.i538, %bb4.i536
  %_0.sroa.0.0.shrunk.i531 = phi i1 [ %343, %bb11.i538 ], [ %341, %bb4.i536 ]
  br i1 %_0.sroa.0.0.shrunk.i531, label %bb114, label %bb124

bb114:                                            ; preds = %bb110, %bb12.i406, %bb1.i532, %bb12.i540, %bb113
  switch i64 %_12.sroa.8.0.copyload, label %bb118 [
    i64 3, label %bb391
    i64 6, label %bb392
  ]

bb391:                                            ; preds = %bb114
  %345 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_fba164e1a5f38e2d8018bcf65720d955, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !687
  %346 = icmp ne i32 %345, 0
  %_120.old = icmp ugt i32 %version.sroa.120.012662065, 9
  %or.cond1856 = select i1 %346, i1 true, i1 %_120.old
  br i1 %or.cond1856, label %bb118, label %bb124

bb392:                                            ; preds = %bb114
  %347 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_4a29a4faa0904cd7ff982831f2813e90, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !691
  %348 = icmp eq i32 %347, 0
  %_120 = icmp ult i32 %version.sroa.120.012662065, 10
  %or.cond20.not = select i1 %348, i1 %_120, i1 false
  br i1 %or.cond20.not, label %bb124, label %bb118

bb118:                                            ; preds = %bb114, %bb391, %bb392
; invoke build_script_build::is_allowed_feature
  %_122 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_fb30e4df55a4a7c53eb78eab864290eb, i64 noundef 3)
          to label %bb119 unwind label %cleanup7

bb119:                                            ; preds = %bb118
  br i1 %_122, label %bb120, label %bb124

bb120:                                            ; preds = %bb119
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_1406cfd7adf5bf37af423094d906069c, ptr noundef nonnull inttoptr (i64 91 to ptr))
          to label %bb124 unwind label %cleanup7

bb156:                                            ; preds = %bb11.i485, %bb4.i483
  %_0.sroa.0.0.shrunk.i478 = phi i1 [ %326, %bb11.i485 ], [ %324, %bb4.i483 ]
  br i1 %_0.sroa.0.0.shrunk.i478, label %bb13.lr.ph.i630, label %bb161

bb166:                                            ; preds = %bb3.i560, %bb12.i566, %bb8.i476, %bb160, %bb162
  %version.sroa.128.01264.off02084 = phi i1 [ %version.sroa.128.01264.off02086, %bb160 ], [ %version.sroa.128.01264.off0208623062470, %bb162 ], [ false, %bb8.i476 ], [ %version.sroa.128.01264.off02086, %bb12.i566 ], [ %version.sroa.128.01264.off02086, %bb3.i560 ]
  %version.sroa.120.012662057 = phi i32 [ %version.sroa.120.012662059, %bb160 ], [ %version.sroa.120.01266205923072468, %bb162 ], [ %version.sroa.120.012662060, %bb8.i476 ], [ %version.sroa.120.012662059, %bb12.i566 ], [ %version.sroa.120.012662059, %bb3.i560 ]
  %version.sroa.92.012682053 = phi i8 [ %version.sroa.92.012682046, %bb160 ], [ %version.sroa.92.012682054, %bb162 ], [ %version.sroa.92.012682048, %bb8.i476 ], [ %version.sroa.92.012682046, %bb12.i566 ], [ %version.sroa.92.012682046, %bb3.i560 ]
  %version.sroa.64.012701996 = phi i8 [ %version.sroa.64.012701998, %bb160 ], [ %version.sroa.64.01270199823092465, %bb162 ], [ %version.sroa.64.012701999, %bb8.i476 ], [ 10, %bb12.i566 ], [ %version.sroa.64.012701998, %bb3.i560 ]
  %version.sroa.36.012721969 = phi i16 [ %version.sroa.36.012721971, %bb160 ], [ %version.sroa.36.01272197123102463, %bb162 ], [ %version.sroa.36.012721972, %bb8.i476 ], [ 2019, %bb12.i566 ], [ %version.sroa.36.012721971, %bb3.i560 ]
  %version.sroa.0.01277140514361439 = phi i32 [ 40, %bb160 ], [ %version.sroa.0.0127714051436.ph14421445, %bb162 ], [ %version.sroa.0.01277, %bb8.i476 ], [ 40, %bb12.i566 ], [ %version.sroa.0.012771404, %bb3.i560 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_2c12234dbe8e690864c2c0a679e3cd05, ptr noundef nonnull inttoptr (i64 115 to ptr))
          to label %bb167 unwind label %cleanup7

bb1.i558:                                         ; preds = %bb3.i481
  %_7.i559 = icmp samesign ugt i32 %version.sroa.0.012771404, 40
  br i1 %_7.i559, label %bb161, label %bb3.i560

bb3.i560:                                         ; preds = %bb1.i558
  %_9.i561 = icmp eq i32 %version.sroa.0.012771404, 40
  br i1 %_9.i561, label %bb4.i562, label %bb166

bb4.i562:                                         ; preds = %bb3.i560
  %349 = icmp eq i16 %version.sroa.36.012721971, 2019
  %350 = icmp ugt i16 %version.sroa.36.012721971, 2018
  br i1 %349, label %bb11.i564, label %bb160

bb11.i564:                                        ; preds = %bb4.i562
  %351 = icmp eq i8 %version.sroa.64.012701998, 10
  %352 = icmp ugt i8 %version.sroa.64.012701998, 9
  br i1 %351, label %bb12.i566, label %bb160

bb12.i566:                                        ; preds = %bb11.i564
  %353 = icmp ugt i8 %version.sroa.92.012682046, 12
  br i1 %353, label %bb161, label %bb166

bb160:                                            ; preds = %bb11.i564, %bb4.i562
  %_0.sroa.0.0.shrunk.i557 = phi i1 [ %352, %bb11.i564 ], [ %350, %bb4.i562 ]
  br i1 %_0.sroa.0.0.shrunk.i557, label %bb161, label %bb166

bb161:                                            ; preds = %bb3.i497, %bb156, %bb12.i487, %bb1.i558, %bb12.i566, %bb160
  %version.sroa.128.01264.off0208623062470 = phi i1 [ %version.sroa.128.01264.off02086, %bb160 ], [ %version.sroa.128.01264.off02086, %bb12.i566 ], [ %version.sroa.128.01264.off02086, %bb1.i558 ], [ %version.sroa.128.01264.off020862306247124992517, %bb12.i487 ], [ %version.sroa.128.01264.off0208623062471, %bb156 ], [ %version.sroa.128.01264.off020892282, %bb3.i497 ]
  %version.sroa.120.01266205923072468 = phi i32 [ %version.sroa.120.012662059, %bb160 ], [ %version.sroa.120.012662059, %bb12.i566 ], [ %version.sroa.120.012662059, %bb1.i558 ], [ %version.sroa.120.0126620592307246925012516, %bb12.i487 ], [ %version.sroa.120.01266205923072469, %bb156 ], [ %version.sroa.120.0126620622283, %bb3.i497 ]
  %version.sroa.64.01270199823092465 = phi i8 [ %version.sroa.64.012701998, %bb160 ], [ 10, %bb12.i566 ], [ %version.sroa.64.012701998, %bb1.i558 ], [ 2, %bb12.i487 ], [ %version.sroa.64.01270199823092466, %bb156 ], [ %version.sroa.64.0127020012285, %bb3.i497 ]
  %version.sroa.36.01272197123102463 = phi i16 [ %version.sroa.36.012721971, %bb160 ], [ 2019, %bb12.i566 ], [ %version.sroa.36.012721971, %bb1.i558 ], [ 2022, %bb12.i487 ], [ %version.sroa.36.01272197123102464, %bb156 ], [ %version.sroa.36.0127219742286, %bb3.i497 ]
  %version.sroa.92.012682054 = phi i8 [ %version.sroa.92.012682046, %bb160 ], [ %version.sroa.92.012682046, %bb12.i566 ], [ %version.sroa.92.012682046, %bb1.i558 ], [ %version.sroa.92.0126820462308246725022515, %bb12.i487 ], [ %version.sroa.92.01268204623082467, %bb156 ], [ %version.sroa.92.0126820442284, %bb3.i497 ]
  %version.sroa.0.0127714051436.ph14421445 = phi i32 [ 40, %bb160 ], [ 40, %bb12.i566 ], [ %version.sroa.0.012771404, %bb1.i558 ], [ 60, %bb12.i487 ], [ 60, %bb156 ], [ 59, %bb3.i497 ]
; invoke build_script_build::is_allowed_feature
  %_152 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_50c6ea2673204147b4c7c6266b4cb1e4, i64 noundef 21)
          to label %bb162 unwind label %cleanup7

bb162:                                            ; preds = %bb161
  br i1 %_152, label %bb163, label %bb166

bb163:                                            ; preds = %bb162
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9e6ac5f3f7d9693fbc40f6727b337014, ptr noundef nonnull inttoptr (i64 127 to ptr))
          to label %bb13.lr.ph.i630 unwind label %cleanup7

bb167:                                            ; preds = %bb166
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_3.i571)
  store i64 0, ptr %_3.i571, align 8, !noalias !695
  %_9.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 8
  store i64 %_9.sroa.8.0.copyload, ptr %_9.sroa.4.0._3.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 16
  store ptr %_9.sroa.5.0.copyload, ptr %_9.sroa.5.0._3.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.sroa.4.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 24
  store i64 %_9.sroa.8.0.copyload, ptr %_9.sroa.5.sroa.4.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.sroa.5.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 32
  store i64 0, ptr %_9.sroa.5.sroa.5.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.sroa.6.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 40
  store i64 %_9.sroa.8.0.copyload, ptr %_9.sroa.5.sroa.6.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.sroa.7.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 48
  store <2 x i32> splat (i32 45), ptr %_9.sroa.5.sroa.7.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.5.sroa.9.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 56
  store i8 1, ptr %_9.sroa.5.sroa.9.0._9.sroa.5.0._3.sroa_idx.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.6.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 64
  store i8 1, ptr %_9.sroa.6.0._3.sroa_idx.i, align 8, !noalias !695
  %_9.sroa.7.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i571, i64 65
  store i8 0, ptr %_9.sroa.7.0._3.sroa_idx.i, align 1, !noalias !695
  call void @llvm.experimental.noalias.scope.decl(metadata !699)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i), !noalias !702
  call void @llvm.experimental.noalias.scope.decl(metadata !704)
  call void @llvm.experimental.noalias.scope.decl(metadata !707)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i), !noalias !710
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i.i, ptr noalias noundef align 8 dereferenceable(48) %_9.sroa.5.0._3.sroa_idx.i) #28
          to label %.noexc597 unwind label %cleanup7

.noexc597:                                        ; preds = %bb167
  %_7.i.i.i.i572 = load i64, ptr %_5.i.i.i.i, align 8, !range !37, !noalias !710, !noundef !3
  %354 = trunc nuw i64 %_7.i.i.i.i572 to i1
  br i1 %354, label %bb7.i.i.i.i, label %bb6.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %.noexc597
  %355 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i, i64 8
  %a.i.i.i.i = load i64, ptr %355, align 8, !noalias !710, !noundef !3
  %356 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i, i64 16
  %b.i.i.i.i = load i64, ptr %356, align 8, !noalias !710, !noundef !3
  %i.i.i.i.i = load i64, ptr %_3.i571, align 8, !alias.scope !711, !noalias !712, !noundef !3
  %new_len.i.i.i.i595 = sub nuw i64 %a.i.i.i.i, %i.i.i.i.i
  %data.i.i.i.i596 = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 %i.i.i.i.i
  store i64 %b.i.i.i.i, ptr %_3.i571, align 8, !alias.scope !711, !noalias !712
  br label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %.noexc597
  %357 = load i8, ptr %_9.sroa.7.0._3.sroa_idx.i, align 1, !range !54, !alias.scope !713, !noalias !712, !noundef !3
  %_2.i.i.i.i.i = trunc nuw i8 %357 to i1
  br i1 %_2.i.i.i.i.i, label %bb1.backedge.i.3, label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i
  store i8 1, ptr %_9.sroa.7.0._3.sroa_idx.i, align 1, !alias.scope !713, !noalias !712
  %358 = load i8, ptr %_9.sroa.6.0._3.sroa_idx.i, align 8, !range !54, !alias.scope !713, !noalias !712, !noundef !3
  %_3.i.i.i.i.i = trunc nuw i8 %358 to i1
  %i.pre.i.i.i.i.i = load i64, ptr %_3.i571, align 8, !alias.scope !713, !noalias !712
  %i1.pre.i.i.i.i.i = load i64, ptr %_9.sroa.4.0._3.sroa_idx.i, align 8, !alias.scope !713, !noalias !712
  %_4.not.i.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i.i, %i.pre.i.i.i.i.i
  %or.cond.not.i.i.i.i.i = select i1 %_3.i.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i.i
  br i1 %or.cond.not.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb1.backedge.i.3

bb4.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i.i
  %_10.val.i.i.i.i.i = load ptr, ptr %_9.sroa.5.0._3.sroa_idx.i, align 8, !alias.scope !713, !noalias !712, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i.i = sub nuw i64 %i1.pre.i.i.i.i.i, %i.pre.i.i.i.i.i
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i, i64 %i.pre.i.i.i.i.i
  br label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i: ; preds = %bb4.i.i.i.i.i, %bb7.i.i.i.i
  %_0.sroa.4.0.i.i.i.i = phi i64 [ %new_len.i.i.i.i595, %bb7.i.i.i.i ], [ %new_len.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i = phi ptr [ %data.i.i.i.i596, %bb7.i.i.i.i ], [ %data.i.i.i.i.i, %bb4.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i), !noalias !710
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !716
; call __rustc::__rust_alloc
  %359 = call noundef align 8 dereferenceable_or_null(64) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 64, i64 noundef range(i64 1, 9) 8) #23, !noalias !716
  %360 = icmp eq ptr %359, null
  br i1 %360, label %bb3.i.i.i594, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCslwKqnJYeWCA_18build_script_build.exit.i.i

bb3.i.i.i594:                                     ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 8, i64 64) #26
          to label %.noexc598 unwind label %cleanup7

.noexc598:                                        ; preds = %bb3.i.i.i594
  unreachable

_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCslwKqnJYeWCA_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
  store ptr %_0.sroa.0.0.i.i.i.i, ptr %359, align 8, !noalias !719
  %361 = getelementptr inbounds nuw i8, ptr %359, i64 8
  store i64 %_0.sroa.4.0.i.i.i.i, ptr %361, align 8, !noalias !719
  store i64 4, ptr %vector.i.i, align 8, !noalias !702
  %vector1.sroa.4.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 8
  store ptr %359, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !702
  %vector1.sroa.6.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !noalias !702
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_19.i.i), !noalias !702
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %_19.i.i, ptr noundef nonnull align 8 dereferenceable(72) %_3.i571, i64 72, i1 false), !noalias !712
  call void @llvm.experimental.noalias.scope.decl(metadata !720)
  call void @llvm.experimental.noalias.scope.decl(metadata !723)
  call void @llvm.experimental.noalias.scope.decl(metadata !725)
  call void @llvm.experimental.noalias.scope.decl(metadata !728)
  %362 = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 65
  %363 = load i8, ptr %362, align 1, !range !54, !alias.scope !730, !noalias !735, !noundef !3
  %_2.i.i13.i.i.i.i = trunc nuw i8 %363 to i1
  br i1 %_2.i.i13.i.i.i.i, label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i, label %bb2.i.i.lr.ph.i.i.i.i

_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i: ; preds = %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCslwKqnJYeWCA_18build_script_build.exit.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_19.i.i), !noalias !702
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i), !noalias !702
  br label %bb2.i579

bb2.i.i.lr.ph.i.i.i.i:                            ; preds = %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCslwKqnJYeWCA_18build_script_build.exit.i.i
  %_4.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 16
  %364 = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 64
  %.phi.trans.insert.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 8
  %365 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i.i570, i64 8
  %366 = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i.i.i570, i64 16
  br label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb8.i.i9.i.i, %bb2.i.i.lr.ph.i.i.i.i
  %_23.i.i20.i.i = phi ptr [ %_23.i.i.i.i, %bb8.i.i9.i.i ], [ %359, %bb2.i.i.lr.ph.i.i.i.i ]
  %len.i.i.i.i = phi i64 [ %new_len.i.i10.i.i, %bb8.i.i9.i.i ], [ 1, %bb2.i.i.lr.ph.i.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !736)
  call void @llvm.experimental.noalias.scope.decl(metadata !738)
  %_4.val.i.i.i.i.i.i = load ptr, ptr %_4.i.i.i.i.i.i, align 8, !alias.scope !740, !noalias !735, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i570), !noalias !741
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i.i.i.i.i.i570, ptr noalias noundef align 8 dereferenceable(48) %_4.i.i.i.i.i.i) #28
          to label %.noexc.i.i unwind label %cleanup3.i.i, !noalias !719

.noexc.i.i:                                       ; preds = %bb2.i.i.i.i.i.i
  %_7.i.i.i.i.i.i574 = load i64, ptr %_5.i.i.i.i.i.i570, align 8, !range !37, !noalias !741, !noundef !3
  %367 = trunc nuw i64 %_7.i.i.i.i.i.i574 to i1
  br i1 %367, label %bb7.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i575

bb7.i.i.i.i.i.i:                                  ; preds = %.noexc.i.i
  %a.i.i.i.i.i.i = load i64, ptr %365, align 8, !noalias !741, !noundef !3
  %b.i.i.i.i.i.i = load i64, ptr %366, align 8, !noalias !741, !noundef !3
  %i.i.i.i.i.i.i = load i64, ptr %_19.i.i, align 8, !alias.scope !740, !noalias !735, !noundef !3
  %new_len.i.i.i.i.i.i = sub nuw i64 %a.i.i.i.i.i.i, %i.i.i.i.i.i.i
  %data.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_4.val.i.i.i.i.i.i, i64 %i.i.i.i.i.i.i
  store i64 %b.i.i.i.i.i.i, ptr %_19.i.i, align 8, !alias.scope !740, !noalias !735
  br label %bb3.i.i.i.i

bb6.i.i.i.i.i.i575:                               ; preds = %.noexc.i.i
  %368 = load i8, ptr %362, align 1, !range !54, !alias.scope !742, !noalias !735, !noundef !3
  %_2.i.i.i.i.i.i.i576 = trunc nuw i8 %368 to i1
  br i1 %_2.i.i.i.i.i.i.i576, label %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread9.i.i.i.i, label %bb1.i.i.i.i.i.i.i577

bb1.i.i.i.i.i.i.i577:                             ; preds = %bb6.i.i.i.i.i.i575
  store i8 1, ptr %362, align 1, !alias.scope !742, !noalias !735
  %369 = load i8, ptr %364, align 8, !range !54, !alias.scope !742, !noalias !735, !noundef !3
  %_3.i.i.i.i.i.i.i = trunc nuw i8 %369 to i1
  %i.pre.i.i.i.i.i.i.i = load i64, ptr %_19.i.i, align 8, !alias.scope !742, !noalias !735
  %i1.pre.i.i.i.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i.i.i.i, align 8, !alias.scope !742, !noalias !735
  %_4.not.i.i.i.i.i.i.i = icmp ne i64 %i1.pre.i.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i.i
  %or.cond.not.i.i.i.i.i.i.i = select i1 %_3.i.i.i.i.i.i.i, i1 true, i1 %_4.not.i.i.i.i.i.i.i
  br i1 %or.cond.not.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i, label %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread9.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb1.i.i.i.i.i.i.i577
  %_10.val.i.i.i.i.i.i.i = load ptr, ptr %_4.i.i.i.i.i.i, align 8, !alias.scope !742, !noalias !735, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i.i.i.i.i.i592 = sub nuw i64 %i1.pre.i.i.i.i.i.i.i, %i.pre.i.i.i.i.i.i.i
  %data.i.i.i.i.i.i.i593 = getelementptr inbounds nuw i8, ptr %_10.val.i.i.i.i.i.i.i, i64 %i.pre.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i

_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread9.i.i.i.i: ; preds = %bb1.i.i.i.i.i.i.i577, %bb6.i.i.i.i.i.i575
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i570), !noalias !741
  br label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i

bb3.i.i.i.i:                                      ; preds = %bb4.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i
  %_0.sroa.4.0.i.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i.i, %bb7.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i.i.i592, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.0.0.i.i.i.i.i.i = phi ptr [ %data.i.i.i.i.i.i, %bb7.i.i.i.i.i.i ], [ %data.i.i.i.i.i.i.i593, %bb4.i.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i.i.i570), !noalias !741
  %_19.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i, 576460752303423488
  call void @llvm.assume(i1 %_19.i.i.i.i)
  %self1.i.i.i.i = load i64, ptr %vector.i.i, align 8, !range !8, !alias.scope !745, !noalias !746, !noundef !3
  %_8.i.i.i.i = icmp eq i64 %len.i.i.i.i, %self1.i.i.i.i
  br i1 %_8.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i, label %bb8.i.i9.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i: ; preds = %bb3.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i, i64 noundef %len.i.i.i.i, i64 noundef range(i64 1, 0) 1, i64 noundef 8, i64 noundef 16)
          to label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.bb8.i.i9_crit_edge.i.i unwind label %cleanup3.i.i, !noalias !719

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.bb8.i.i9_crit_edge.i.i: ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i
  %_23.i.i.pre.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !alias.scope !745, !noalias !746
  br label %bb8.i.i9.i.i

bb8.i.i9.i.i:                                     ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.bb8.i.i9_crit_edge.i.i, %bb3.i.i.i.i
  %_23.i.i.i.i = phi ptr [ %_23.i.i.pre.i.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.bb8.i.i9_crit_edge.i.i ], [ %_23.i.i20.i.i, %bb3.i.i.i.i ]
  %dst.i.i.i.i = getelementptr inbounds nuw { ptr, i64 }, ptr %_23.i.i.i.i, i64 %len.i.i.i.i
  store ptr %_0.sroa.0.0.i.i.i.i.i.i, ptr %dst.i.i.i.i, align 8, !noalias !747
  %370 = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i, i64 8
  store i64 %_0.sroa.4.0.i.i.i.i.i.i, ptr %370, align 8, !noalias !747
  %new_len.i.i10.i.i = add nuw nsw i64 %len.i.i.i.i, 1
  store i64 %new_len.i.i10.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !alias.scope !745, !noalias !746
  %371 = load i8, ptr %362, align 1, !range !54, !alias.scope !748, !noalias !735, !noundef !3
  %_2.i.i.i.i.i.i = trunc nuw i8 %371 to i1
  br i1 %_2.i.i.i.i.i.i, label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i, label %bb2.i.i.i.i.i.i

cleanup3.i.i:                                     ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecReE7reserveCslwKqnJYeWCA_18build_script_build.exit.i.i.i.i, %bb2.i.i.i.i.i.i
  %372 = landingpad { ptr, i32 }
          cleanup
  %vector.val.i.i = load i64, ptr %vector.i.i, align 8, !noalias !702
  %373 = icmp eq i64 %vector.val.i.i, 0
  br i1 %373, label %bb384, label %bb2.i.i.i4.i.i.i573

bb2.i.i.i4.i.i.i573:                              ; preds = %cleanup3.i.i
  %vector.val7.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !702, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %vector.val.i.i, 4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %vector.val7.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !719
  br label %bb384

_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb8.i.i9.i.i, %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread9.i.i.i.i
  %parts.sroa.12.0.copyload.i = phi i64 [ %len.i.i.i.i, %_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build.exit.thread9.i.i.i.i ], [ %new_len.i.i10.i.i, %bb8.i.i9.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_19.i.i), !noalias !702
  %parts.sroa.0.0.copyload.i = load i64, ptr %vector.i.i, align 8, !noalias !751
  %parts.sroa.6.0.copyload.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !751
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i), !noalias !702
  %_17.i578 = icmp samesign ugt i64 %parts.sroa.12.0.copyload.i, 2
  br i1 %_17.i578, label %bb8.i589, label %bb2.i579

bb8.i589:                                         ; preds = %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i
  %374 = getelementptr i8, ptr %parts.sroa.6.0.copyload.i, i64 40
  %_18.val4.i = load i64, ptr %374, align 8, !noalias !752, !noundef !3
  %_3.not.i.i.i.i = icmp eq i64 %_18.val4.i, 5
  br i1 %_3.not.i.i.i.i, label %bb10.i, label %bb41.i.i

bb2.i579:                                         ; preds = %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i
  %parts.sroa.6.0.copyload82.i = phi ptr [ %359, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i ], [ %parts.sroa.6.0.copyload.i, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i ]
  %parts.sroa.0.0.copyload80.i = phi i64 [ 4, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i ], [ %parts.sroa.0.0.copyload.i, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i ]
  %parts.sroa.12.0.copyload78.i = phi i64 [ 1, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.thread.i ], [ %parts.sroa.12.0.copyload.i, %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build.exit.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !753)
  br label %bb41.i.i

bb41.i.i:                                         ; preds = %bb11.i590, %bb10.i, %bb2.i579, %bb8.i589
  %parts.sroa.6.0.copyload81.i = phi ptr [ %parts.sroa.6.0.copyload82.i, %bb2.i579 ], [ %parts.sroa.6.0.copyload.i, %bb8.i589 ], [ %parts.sroa.6.0.copyload.i, %bb10.i ], [ %parts.sroa.6.0.copyload.i, %bb11.i590 ]
  %parts.sroa.0.0.copyload79.i = phi i64 [ %parts.sroa.0.0.copyload80.i, %bb2.i579 ], [ %parts.sroa.0.0.copyload.i, %bb8.i589 ], [ %parts.sroa.0.0.copyload.i, %bb10.i ], [ %parts.sroa.0.0.copyload.i, %bb11.i590 ]
  %parts.sroa.12.0.copyload77.i = phi i64 [ %parts.sroa.12.0.copyload78.i, %bb2.i579 ], [ %parts.sroa.12.0.copyload.i, %bb8.i589 ], [ %parts.sroa.12.0.copyload.i, %bb10.i ], [ %parts.sroa.12.0.copyload.i, %bb11.i590 ]
  %_95.idx.i50.i = shl nuw nsw i64 %parts.sroa.12.0.copyload77.i, 4
  %_95.i51.i = getelementptr inbounds nuw i8, ptr %parts.sroa.6.0.copyload81.i, i64 %_95.idx.i50.i
  %gepdiff.i.i = add nsw i64 %_95.idx.i50.i, -16
  %375 = lshr exact i64 %gepdiff.i.i, 4
  br label %bb1.i.i.i580

bb1.i.i.i580:                                     ; preds = %bb3.i.i7.i, %bb41.i.i
  %_16.i6.i.i.i = phi ptr [ %parts.sroa.6.0.copyload81.i, %bb41.i.i ], [ %_16.i.i.i.i, %bb3.i.i7.i ]
  %accum.sroa.0.0.i.i.i = phi i64 [ %375, %bb41.i.i ], [ %_4.0.i.i.i.i.i.i, %bb3.i.i7.i ]
  %_6.i.i.i.i = icmp eq ptr %_16.i6.i.i.i, %_95.i51.i
  br i1 %_6.i.i.i.i, label %bb53.i.i, label %bb3.i.i7.i

bb3.i.i7.i:                                       ; preds = %bb1.i.i.i580
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i6.i.i.i, i64 16
  %376 = getelementptr i8, ptr %_16.i6.i.i.i, i64 8
  %.val3.i.i.i581 = load i64, ptr %376, align 8, !alias.scope !753, !noalias !756, !noundef !3
  %_4.0.i.i.i.i.i.i = add i64 %.val3.i.i.i581, %accum.sroa.0.0.i.i.i
  %_4.1.i.i.i.i.i.i = icmp ult i64 %_4.0.i.i.i.i.i.i, %accum.sroa.0.0.i.i.i
  br i1 %_4.1.i.i.i.i.i.i, label %bb52.i.i, label %bb1.i.i.i580

bb53.i.i:                                         ; preds = %bb1.i.i.i580
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %result.i.i), !noalias !760
  %_23.i.i.i.i.i = icmp eq i64 %accum.sroa.0.0.i.i.i, 0
  br i1 %_23.i.i.i.i.i, label %bb4.i.i585, label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb53.i.i
  %or.cond.not.i.i = icmp sgt i64 %accum.sroa.0.0.i.i.i, 0
  br i1 %or.cond.not.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i, label %bb3.i102.i.i, !prof !223

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i: ; preds = %bb6.i.i.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !761
; call __rustc::__rust_alloc
  %377 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %accum.sroa.0.0.i.i.i, i64 noundef range(i64 1, 9) 1) #23, !noalias !761
  %378 = icmp eq ptr %377, null
  br i1 %378, label %bb3.i102.i.i, label %bb4.i.i585

bb3.i102.i.i:                                     ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i, %bb6.i.i.i.i.i
  %_4.sroa.4.0.ph.i.i.i = phi i64 [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i ], [ 0, %bb6.i.i.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i.i, i64 %accum.sroa.0.0.i.i.i) #26
          to label %.noexc.i584 unwind label %cleanup.i582, !noalias !752

.noexc.i584:                                      ; preds = %bb3.i102.i.i
  unreachable

bb52.i.i:                                         ; preds = %bb3.i.i7.i
; invoke core::option::expect_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ca673fb95acb8e58af271999e89294ae, i64 noundef 53, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_60488e92c3d9250777708a132d567f7b) #29
          to label %.noexc15.i unwind label %cleanup.i582, !noalias !752

.noexc15.i:                                       ; preds = %bb52.i.i
  unreachable

cleanup.i.i588:                                   ; preds = %bb3.i116.invoke.i.i, %bb1.i.i.i.i13.i
  %379 = landingpad { ptr, i32 }
          cleanup
  %result.val.i.i = load i64, ptr %result.i.i, align 8, !noalias !760
  %380 = icmp eq i64 %result.val.i.i, 0
  br i1 %380, label %cleanup.body.i, label %bb2.i.i.i4.i.i12.i

bb2.i.i.i4.i.i12.i:                               ; preds = %cleanup.i.i588
  %result.val99.i.i = load ptr, ptr %381, align 8, !noalias !760, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %result.val99.i.i, i64 noundef %result.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !764
  br label %cleanup.body.i

bb4.i.i585:                                       ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i, %bb53.i.i
  %_4.sroa.10.0.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb53.i.i ], [ %377, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i8.i ]
  store i64 %accum.sroa.0.0.i.i.i, ptr %result.i.i, align 8, !noalias !760
  %381 = getelementptr inbounds nuw i8, ptr %result.i.i, i64 8
  store ptr %_4.sroa.10.0.i.i.i, ptr %381, align 8, !noalias !760
  %382 = getelementptr inbounds nuw i8, ptr %result.i.i, i64 16
  store i64 0, ptr %382, align 8, !noalias !760
  %slice.0.val.i.i = load ptr, ptr %parts.sroa.6.0.copyload81.i, align 8, !alias.scope !753, !noalias !765, !nonnull !3, !align !18, !noundef !3
  %383 = getelementptr i8, ptr %parts.sroa.6.0.copyload81.i, i64 8
  %slice.0.val101.i.i = load i64, ptr %383, align 8, !alias.scope !753, !noalias !765, !noundef !3
  call void @llvm.experimental.noalias.scope.decl(metadata !766)
  call void @llvm.experimental.noalias.scope.decl(metadata !769)
  %_7.i.i.i.i.i = icmp ugt i64 %slice.0.val101.i.i, %accum.sroa.0.0.i.i.i
  br i1 %_7.i.i.i.i.i, label %bb1.i.i.i.i13.i, label %bb55.i.i586, !prof !102

bb1.i.i.i.i13.i:                                  ; preds = %bb4.i.i585
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i.i, i64 noundef 0, i64 noundef %slice.0.val101.i.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc.i14.i unwind label %cleanup.i.i588, !noalias !764

.noexc.i14.i:                                     ; preds = %bb1.i.i.i.i13.i
  %len.pre.i.i.i.i = load i64, ptr %382, align 8, !alias.scope !772, !noalias !760
  %_10.i.i.pre.i.i = load ptr, ptr %381, align 8, !alias.scope !772, !noalias !760
  br label %bb55.i.i586

bb55.i.i586:                                      ; preds = %.noexc.i14.i, %bb4.i.i585
  %_10.i.i.i.i = phi ptr [ %_4.sroa.10.0.i.i.i, %bb4.i.i585 ], [ %_10.i.i.pre.i.i, %.noexc.i14.i ]
  %len.i.i.i9.i = phi i64 [ 0, %bb4.i.i585 ], [ %len.pre.i.i.i.i, %.noexc.i14.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i9.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %dst.i.i.i10.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i, i64 %len.i.i.i9.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i.i10.i, ptr nonnull readonly align 1 %slice.0.val.i.i, i64 %slice.0.val101.i.i, i1 false), !noalias !773
  %384 = add i64 %len.i.i.i9.i, %slice.0.val101.i.i
  %_157.i.i = icmp sgt i64 %384, -1
  call void @llvm.assume(i1 %_157.i.i)
  %index.i.i = sub nsw i64 %accum.sroa.0.0.i.i.i, %384
  %_2206.i.i = icmp eq i64 %parts.sroa.12.0.copyload77.i, 1
  br i1 %_2206.i.i, label %bb12.i587, label %bb96.preheader.i.i

bb96.preheader.i.i:                               ; preds = %bb55.i.i586
  %iter_uninit.sroa.0.15.i.i = getelementptr inbounds nuw i8, ptr %parts.sroa.6.0.copyload81.i, i64 16
  %_159.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i, i64 %384
  br label %bb96.i.i

bb90.i.loopexit.i:                                ; preds = %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i
  %_31.sroa.5.0.copyload24.pre.i = load ptr, ptr %381, align 8, !noalias !774
  br label %bb12.i587

bb96.i.i:                                         ; preds = %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i, %bb96.preheader.i.i
  %iter_uninit.sroa.0.110.i.i = phi ptr [ %iter_uninit.sroa.0.1.i.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i ], [ %iter_uninit.sroa.0.15.i.i, %bb96.preheader.i.i ]
  %slice.0.pn9.i.i = phi ptr [ %iter_uninit.sroa.0.110.i.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i ], [ %parts.sroa.6.0.copyload81.i, %bb96.preheader.i.i ]
  %target.sroa.26.28.i.i = phi i64 [ %len.i.i112.i.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i ], [ %index.i.i, %bb96.preheader.i.i ]
  %target.sroa.0.27.i.i = phi ptr [ %data.i.i111.i.i, %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i ], [ %_159.i.i, %bb96.preheader.i.i ]
  %iter_uninit.sroa.0.1.val.i.i = load ptr, ptr %iter_uninit.sroa.0.110.i.i, align 8, !alias.scope !753, !noalias !765, !nonnull !3, !align !18, !noundef !3
  %385 = getelementptr i8, ptr %slice.0.pn9.i.i, i64 24
  %iter_uninit.sroa.0.1.val100.i.i = load i64, ptr %385, align 8, !alias.scope !753, !noalias !765, !noundef !3
  %_6.not.i.i.i = icmp eq i64 %target.sroa.26.28.i.i, 0
  br i1 %_6.not.i.i.i, label %bb3.i116.invoke.i.i, label %bb100.i.i, !prof !102

bb100.i.i:                                        ; preds = %bb96.i.i
  %len.i.i104.i.i = add nsw i64 %target.sroa.26.28.i.i, -1
  store i8 45, ptr %target.sroa.0.27.i.i, align 1, !alias.scope !775, !noalias !764
  %_6.not.i109.i.i = icmp ugt i64 %iter_uninit.sroa.0.1.val100.i.i, %len.i.i104.i.i
  br i1 %_6.not.i109.i.i, label %bb3.i116.invoke.i.i, label %_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i, !prof !102

bb3.i116.invoke.i.i:                              ; preds = %bb100.i.i, %bb96.i.i
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3f2fbfdca196a5b824209b380ee7ae1b) #29
          to label %bb3.i116.cont.i.i unwind label %cleanup.i.i588, !noalias !764

bb3.i116.cont.i.i:                                ; preds = %bb3.i116.invoke.i.i
  unreachable

_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build.exit122.i.i: ; preds = %bb100.i.i
  %data.i.i.i11.i = getelementptr inbounds nuw i8, ptr %target.sroa.0.27.i.i, i64 1
  %data.i.i111.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i11.i, i64 %iter_uninit.sroa.0.1.val100.i.i
  %len.i.i112.i.i = sub nuw nsw i64 %len.i.i104.i.i, %iter_uninit.sroa.0.1.val100.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %data.i.i.i11.i, ptr nonnull readonly align 1 %iter_uninit.sroa.0.1.val.i.i, i64 range(i64 0, -9223372036854775808) %iter_uninit.sroa.0.1.val100.i.i, i1 false), !alias.scope !779, !noalias !764
  %iter_uninit.sroa.0.1.i.i = getelementptr inbounds nuw i8, ptr %iter_uninit.sroa.0.110.i.i, i64 16
  %_220.i.i = icmp eq ptr %iter_uninit.sroa.0.1.i.i, %_95.i51.i
  br i1 %_220.i.i, label %bb90.i.loopexit.i, label %bb96.i.i

cleanup.i582:                                     ; preds = %bb52.i.i, %bb3.i102.i.i
  %386 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %cleanup.i582, %bb2.i.i.i4.i.i12.i, %cleanup.i.i588
  %eh.lpad-body.i = phi { ptr, i32 } [ %386, %cleanup.i582 ], [ %379, %bb2.i.i.i4.i.i12.i ], [ %379, %cleanup.i.i588 ]
  %387 = icmp eq i64 %parts.sroa.0.0.copyload79.i, 0
  br i1 %387, label %bb384, label %bb2.i.i.i4.i.i583

bb2.i.i.i4.i.i583:                                ; preds = %cleanup.body.i
  %alloc_size.i.i.i.i5.i.i = shl nuw i64 %parts.sroa.0.0.copyload79.i, 4
  %388 = icmp ne ptr %parts.sroa.6.0.copyload81.i, null
  call void @llvm.assume(i1 %388)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %parts.sroa.6.0.copyload81.i, i64 noundef %alloc_size.i.i.i.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !752
  br label %bb384

bb10.i:                                           ; preds = %bb8.i589
  %_18.i = getelementptr inbounds nuw i8, ptr %parts.sroa.6.0.copyload.i, i64 32
  %_18.val.i = load ptr, ptr %_18.i, align 8, !noalias !752, !nonnull !3, !align !18, !noundef !3
  %389 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_18.val.i, ptr noundef nonnull readonly align 1 dereferenceable(5) @alloc_70a1e7dc3879e83c39c209c1ae5f1722, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !783, !noalias !752
  %390 = icmp eq i32 %389, 0
  br i1 %390, label %bb11.i590, label %bb41.i.i

bb11.i590:                                        ; preds = %bb10.i
  %_6.i591 = getelementptr inbounds nuw i8, ptr %parts.sroa.6.0.copyload.i, i64 16
  store ptr @alloc_14c43fe6be9850e9c6ac099b83b2e4e2, ptr %_6.i591, align 8, !noalias !752
  %391 = getelementptr inbounds nuw i8, ptr %parts.sroa.6.0.copyload.i, i64 24
  store i64 7, ptr %391, align 8, !noalias !752
  br label %bb41.i.i

bb12.i587:                                        ; preds = %bb90.i.loopexit.i, %bb55.i.i586
  %_31.sroa.5.0.copyload24.i = phi ptr [ %_10.i.i.i.i, %bb55.i.i586 ], [ %_31.sroa.5.0.copyload24.pre.i, %bb90.i.loopexit.i ]
  %target.sroa.26.2.lcssa.i.i = phi i64 [ %index.i.i, %bb55.i.i586 ], [ %len.i.i112.i.i, %bb90.i.loopexit.i ]
  %result_len.i.i = sub i64 %accum.sroa.0.0.i.i.i, %target.sroa.26.2.lcssa.i.i
  %_31.sroa.0.0.copyload23.i = load i64, ptr %result.i.i, align 8, !noalias !774
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %result.i.i), !noalias !760
  %392 = icmp eq i64 %parts.sroa.0.0.copyload79.i, 0
  br i1 %392, label %bb13.lr.ph.i, label %bb2.i.i.i4.i18.i

bb2.i.i.i4.i18.i:                                 ; preds = %bb12.i587
  %alloc_size.i.i.i.i5.i19.i = shl nuw i64 %parts.sroa.0.0.copyload79.i, 4
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %parts.sroa.6.0.copyload81.i, i64 noundef %alloc_size.i.i.i.i5.i19.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !752
  br label %bb13.lr.ph.i

bb13.lr.ph.i:                                     ; preds = %bb12.i587, %bb2.i.i.i4.i18.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_3.i571)
  %393 = icmp ne ptr %_31.sroa.5.0.copyload24.i, null
  call void @llvm.assume(i1 %393)
  switch i64 %result_len.i.i, label %bb13.lr.ph.i608 [
    i64 25, label %bb2.i.i.i.i.i
    i64 18, label %bb2.i.i.i.i.i.1
    i64 15, label %bb2.i.i.i.i.i.3
    i64 27, label %bb2.i.i.i.i.i.5
  ]

bb2.i.i.i.i.i:                                    ; preds = %bb13.lr.ph.i
  %394 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(25) @alloc_073a41cfae022637d06fd85a5e605fab, ptr noundef nonnull readonly align 1 dereferenceable(25) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 25), !alias.scope !790, !noalias !797
  %395 = icmp eq i32 %394, 0
  br i1 %395, label %bb169, label %bb2.i.i.i.i.i.4

bb2.i.i.i.i.i.1:                                  ; preds = %bb13.lr.ph.i
  %396 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_aa31e40b84e5bd72eb2b84fd23b8ce8b, ptr noundef nonnull readonly align 1 dereferenceable(18) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !790, !noalias !797
  %397 = icmp eq i32 %396, 0
  br i1 %397, label %bb169, label %bb2.i.i.i.i.i.2

bb2.i.i.i.i.i.2:                                  ; preds = %bb2.i.i.i.i.i.1
  %398 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_edb56cace5b9083f410aa7986257c472, ptr noundef nonnull readonly align 1 dereferenceable(18) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !790, !noalias !797
  %399 = icmp eq i32 %398, 0
  br i1 %399, label %bb169, label %bb2.i.i.i.i.i.6

bb2.i.i.i.i.i.3:                                  ; preds = %bb13.lr.ph.i
  %400 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(15) @alloc_9a3299e3caae06e665af83a2a067eb68, ptr noundef nonnull readonly align 1 dereferenceable(15) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 15), !alias.scope !790, !noalias !797
  %401 = icmp eq i32 %400, 0
  br i1 %401, label %bb169, label %bb13.lr.ph.i608

bb1.backedge.i.3:                                 ; preds = %bb6.i.i.i.i, %bb1.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i.i.i.i), !noalias !710
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i), !noalias !702
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_3.i571)
  br label %bb13.lr.ph.i608

bb2.i.i.i.i.i.4:                                  ; preds = %bb2.i.i.i.i.i
  %402 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(25) @alloc_ecbfecbf042f5d02ed1d945b2da736d2, ptr noundef nonnull readonly align 1 dereferenceable(25) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 25), !alias.scope !790, !noalias !797
  %403 = icmp eq i32 %402, 0
  br i1 %403, label %bb169, label %bb13.lr.ph.i608

bb2.i.i.i.i.i.5:                                  ; preds = %bb13.lr.ph.i
  %404 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(27) @alloc_fd1b38744aa12ca58723e85c5996f9f0, ptr noundef nonnull readonly align 1 dereferenceable(27) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 27), !alias.scope !790, !noalias !797
  %405 = icmp eq i32 %404, 0
  br i1 %405, label %bb169, label %bb13.lr.ph.i608

bb2.i.i.i.i.i.6:                                  ; preds = %bb2.i.i.i.i.i.2
  %406 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_603fc8e29c06fe139773a6b784e3cc49, ptr noundef nonnull readonly align 1 dereferenceable(18) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !790, !noalias !797
  %407 = icmp eq i32 %406, 0
  br i1 %407, label %bb169, label %bb2.i.i.i.i.i.7

bb2.i.i.i.i.i.7:                                  ; preds = %bb2.i.i.i.i.i.6
  %408 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_0f444773a3c04e447512a602928cccf9, ptr noundef nonnull readonly align 1 dereferenceable(18) %_31.sroa.5.0.copyload24.i, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !790, !noalias !797
  %409 = icmp eq i32 %408, 0
  br i1 %409, label %bb169, label %bb13.lr.ph.i608

cleanup9:                                         ; preds = %bb173, %bb169
  %_159.sroa.6.02323 = phi ptr [ %_159.sroa.6.02320, %bb173 ], [ %_31.sroa.5.0.copyload24.i, %bb169 ]
  %_159.sroa.0.02319 = phi i64 [ %_159.sroa.0.02316, %bb173 ], [ %_31.sroa.0.0.copyload23.i, %bb169 ]
  %410 = landingpad { ptr, i32 }
          cleanup
  %411 = icmp eq i64 %_159.sroa.0.02319, 0
  br i1 %411, label %bb384, label %bb2.i.i.i4.i.i603

bb2.i.i.i4.i.i603:                                ; preds = %cleanup9
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_159.sroa.6.02323, i64 noundef %_159.sroa.0.02319, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb384

bb169:                                            ; preds = %bb2.i.i.i.i.i.7, %bb2.i.i.i.i.i.6, %bb2.i.i.i.i.i.5, %bb2.i.i.i.i.i.4, %bb2.i.i.i.i.i.3, %bb2.i.i.i.i.i.2, %bb2.i.i.i.i.i.1, %bb2.i.i.i.i.i
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0a8846dfe80e6e9ba5109a5921697c29, ptr noundef nonnull inttoptr (i64 93 to ptr))
          to label %bb13.lr.ph.i608 unwind label %cleanup9

bb13.lr.ph.i608:                                  ; preds = %bb13.lr.ph.i, %bb1.backedge.i.3, %bb2.i.i.i.i.i.4, %bb2.i.i.i.i.i.5, %bb2.i.i.i.i.i.3, %bb2.i.i.i.i.i.7, %bb169
  %_159.sroa.11.02324 = phi i64 [ %result_len.i.i, %bb169 ], [ 18, %bb2.i.i.i.i.i.7 ], [ 15, %bb2.i.i.i.i.i.3 ], [ 27, %bb2.i.i.i.i.i.5 ], [ 0, %bb1.backedge.i.3 ], [ 25, %bb2.i.i.i.i.i.4 ], [ %result_len.i.i, %bb13.lr.ph.i ]
  %_159.sroa.6.02320 = phi ptr [ %_31.sroa.5.0.copyload24.i, %bb169 ], [ %_31.sroa.5.0.copyload24.i, %bb2.i.i.i.i.i.7 ], [ %_31.sroa.5.0.copyload24.i, %bb2.i.i.i.i.i.3 ], [ %_31.sroa.5.0.copyload24.i, %bb2.i.i.i.i.i.5 ], [ inttoptr (i64 1 to ptr), %bb1.backedge.i.3 ], [ %_31.sroa.5.0.copyload24.i, %bb2.i.i.i.i.i.4 ], [ %_31.sroa.5.0.copyload24.i, %bb13.lr.ph.i ]
  %_159.sroa.0.02316 = phi i64 [ %_31.sroa.0.0.copyload23.i, %bb169 ], [ %_31.sroa.0.0.copyload23.i, %bb2.i.i.i.i.i.7 ], [ %_31.sroa.0.0.copyload23.i, %bb2.i.i.i.i.i.3 ], [ %_31.sroa.0.0.copyload23.i, %bb2.i.i.i.i.i.5 ], [ 0, %bb1.backedge.i.3 ], [ %_31.sroa.0.0.copyload23.i, %bb2.i.i.i.i.i.4 ], [ %_31.sroa.0.0.copyload23.i, %bb13.lr.ph.i ]
  br label %bb13.i611

bb13.i611:                                        ; preds = %bb1.backedge.i616, %bb13.lr.ph.i608
  %_2224.i612.idx = phi i64 [ 0, %bb13.lr.ph.i608 ], [ %_2224.i612.add, %bb1.backedge.i616 ]
  %_2224.i612.ptr = getelementptr inbounds nuw i8, ptr @alloc_6929424002fbe1deb331ef746021f140, i64 %_2224.i612.idx
  %_2224.i612.add = add nuw nsw i64 %_2224.i612.idx, 16
  %412 = getelementptr i8, ptr %_2224.i612.ptr, i64 8
  %ptr.val1.i614 = load i64, ptr %412, align 8, !noalias !801, !noundef !3
  %_3.not.i.i.i.i.i615 = icmp eq i64 %ptr.val1.i614, %_159.sroa.11.02324
  br i1 %_3.not.i.i.i.i.i615, label %bb2.i.i.i.i.i622, label %bb1.backedge.i616

bb2.i.i.i.i.i622:                                 ; preds = %bb13.i611
  %ptr.val.i623 = load ptr, ptr %_2224.i612.ptr, align 8, !noalias !801, !nonnull !3, !align !18, !noundef !3
  %413 = call i32 @memcmp(ptr nonnull readonly align 1 %ptr.val.i623, ptr nonnull readonly align 1 %_159.sroa.6.02320, i64 range(i64 0, -9223372036854775808) %_159.sroa.11.02324), !alias.scope !805, !noalias !801
  %414 = icmp eq i32 %413, 0
  br i1 %414, label %bb173, label %bb1.backedge.i616

bb1.backedge.i616:                                ; preds = %bb2.i.i.i.i.i622, %bb13.i611
  %_12.not.i617 = icmp eq i64 %_2224.i612.add, 736
  br i1 %_12.not.i617, label %bb176, label %bb13.i611

bb173:                                            ; preds = %bb2.i.i.i.i.i622
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8e16d26229e79ba7f22bd47d524058bd, ptr noundef nonnull inttoptr (i64 91 to ptr))
          to label %bb176 unwind label %cleanup9

bb176:                                            ; preds = %bb1.backedge.i616, %bb173
  %415 = icmp eq i64 %_159.sroa.0.02316, 0
  br i1 %415, label %bb13.lr.ph.i630, label %bb2.i.i.i4.i.i625

bb2.i.i.i4.i.i625:                                ; preds = %bb176
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_159.sroa.6.02320, i64 noundef %_159.sroa.0.02316, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb13.lr.ph.i630

bb13.lr.ph.i630:                                  ; preds = %bb131, %bb12.i466, %bb1.i458, %bb8.i455, %bb1.i479, %bb156, %bb163, %bb8.i476, %bb12.i487, %bb2.i.i.i4.i.i625, %bb176
  %version.sroa.128.01264.off02085 = phi i1 [ %version.sroa.128.01264.off0208623062470, %bb163 ], [ %version.sroa.128.01264.off0208623062471, %bb156 ], [ false, %bb8.i476 ], [ %version.sroa.128.01264.off020862306247124992517, %bb12.i487 ], [ %version.sroa.128.01264.off02084, %bb2.i.i.i4.i.i625 ], [ %version.sroa.128.01264.off02084, %bb176 ], [ %version.sroa.128.01264.off02086, %bb1.i479 ], [ false, %bb8.i455 ], [ true, %bb1.i458 ], [ true, %bb12.i466 ], [ true, %bb131 ]
  %version.sroa.120.012662058 = phi i32 [ %version.sroa.120.01266205923072468, %bb163 ], [ %version.sroa.120.01266205923072469, %bb156 ], [ %version.sroa.120.012662060, %bb8.i476 ], [ %version.sroa.120.0126620592307246925012516, %bb12.i487 ], [ %version.sroa.120.012662057, %bb2.i.i.i4.i.i625 ], [ %version.sroa.120.012662057, %bb176 ], [ %version.sroa.120.012662059, %bb1.i479 ], [ %version.sroa.120.012662062, %bb8.i455 ], [ %version.sroa.120.012662062, %bb1.i458 ], [ %version.sroa.120.012662062, %bb12.i466 ], [ %version.sroa.120.012662062, %bb131 ]
  %version.sroa.92.012682055 = phi i8 [ %version.sroa.92.012682054, %bb163 ], [ %version.sroa.92.01268204623082467, %bb156 ], [ %version.sroa.92.012682048, %bb8.i476 ], [ %version.sroa.92.0126820462308246725022515, %bb12.i487 ], [ %version.sroa.92.012682053, %bb2.i.i.i4.i.i625 ], [ %version.sroa.92.012682053, %bb176 ], [ %version.sroa.92.012682046, %bb1.i479 ], [ %version.sroa.92.012682044, %bb8.i455 ], [ %version.sroa.92.012682044, %bb1.i458 ], [ %version.sroa.92.012682044, %bb12.i466 ], [ %version.sroa.92.012682044, %bb131 ]
  %version.sroa.64.012701997 = phi i8 [ %version.sroa.64.01270199823092465, %bb163 ], [ %version.sroa.64.01270199823092466, %bb156 ], [ %version.sroa.64.012701999, %bb8.i476 ], [ 2, %bb12.i487 ], [ %version.sroa.64.012701996, %bb2.i.i.i4.i.i625 ], [ %version.sroa.64.012701996, %bb176 ], [ %version.sroa.64.012701998, %bb1.i479 ], [ %version.sroa.64.012702001, %bb8.i455 ], [ %version.sroa.64.012702001, %bb1.i458 ], [ 11, %bb12.i466 ], [ %version.sroa.64.012702001, %bb131 ]
  %version.sroa.36.012721970 = phi i16 [ %version.sroa.36.01272197123102463, %bb163 ], [ %version.sroa.36.01272197123102464, %bb156 ], [ %version.sroa.36.012721972, %bb8.i476 ], [ 2022, %bb12.i487 ], [ %version.sroa.36.012721969, %bb2.i.i.i4.i.i625 ], [ %version.sroa.36.012721969, %bb176 ], [ %version.sroa.36.012721971, %bb1.i479 ], [ %version.sroa.36.012721974, %bb8.i455 ], [ %version.sroa.36.012721974, %bb1.i458 ], [ 2024, %bb12.i466 ], [ %version.sroa.36.012721974, %bb131 ]
  %version.sroa.0.0127714051432 = phi i32 [ %version.sroa.0.0127714051436.ph14421445, %bb163 ], [ 60, %bb156 ], [ %version.sroa.0.01277, %bb8.i476 ], [ 60, %bb12.i487 ], [ %version.sroa.0.01277140514361439, %bb2.i.i.i4.i.i625 ], [ %version.sroa.0.01277140514361439, %bb176 ], [ %version.sroa.0.012771404, %bb1.i479 ], [ %version.sroa.0.0127613671387, %bb8.i455 ], [ %version.sroa.0.0127613671387, %bb1.i458 ], [ 84, %bb12.i466 ], [ 84, %bb131 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !812)
  %_3.val2.i.i631 = load i64, ptr %6, align 8, !alias.scope !812, !noalias !815, !noundef !3
  %_3.val.i.i632 = load ptr, ptr %_7, align 8, !alias.scope !812, !noalias !815, !nonnull !3, !align !18
  switch i64 %_3.val2.i.i631, label %bb183 [
    i64 18, label %bb2.i.i.i.i.i644
    i64 15, label %bb2.i.i.i.i.i644.2
  ]

bb2.i.i.i.i.i644:                                 ; preds = %bb13.lr.ph.i630
  %416 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_aa31e40b84e5bd72eb2b84fd23b8ce8b, ptr noundef nonnull readonly align 1 dereferenceable(18) %_3.val.i.i632, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !817, !noalias !824
  %417 = icmp eq i32 %416, 0
  br i1 %417, label %bb180, label %bb2.i.i.i.i.i644.1

bb2.i.i.i.i.i644.1:                               ; preds = %bb2.i.i.i.i.i644
  %418 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(18) @alloc_edb56cace5b9083f410aa7986257c472, ptr noundef nonnull readonly align 1 dereferenceable(18) %_3.val.i.i632, i64 range(i64 0, -9223372036854775808) 18), !alias.scope !817, !noalias !824
  %419 = icmp eq i32 %418, 0
  br i1 %419, label %bb180, label %bb183

bb2.i.i.i.i.i644.2:                               ; preds = %bb13.lr.ph.i630
  %420 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(15) @alloc_63b8bcb9c0020832e6fa0013d77e86c3, ptr noundef nonnull readonly align 1 dereferenceable(15) %_3.val.i.i632, i64 range(i64 0, -9223372036854775808) 15), !alias.scope !817, !noalias !824
  %421 = icmp eq i32 %420, 0
  br i1 %421, label %bb180, label %bb183

bb180:                                            ; preds = %bb2.i.i.i.i.i644.2, %bb2.i.i.i.i.i644.1, %bb2.i.i.i.i.i644
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_f0dd653f0ec1ecef8fb1fc4d13d313c0, ptr noundef nonnull inttoptr (i64 107 to ptr))
          to label %bb183 unwind label %cleanup7

bb183:                                            ; preds = %bb13.lr.ph.i630, %bb2.i.i.i.i.i644.1, %bb2.i.i.i.i.i644.2, %bb180
  %_175 = icmp ult i32 %version.sroa.120.012662058, 20
  br i1 %_175, label %bb184, label %bb199

bb184:                                            ; preds = %bb183
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_94cefc1b63dffcfbd691138662a4abf1, ptr noundef nonnull inttoptr (i64 89 to ptr))
          to label %bb185 unwind label %cleanup7

bb199:                                            ; preds = %bb190, %bb189, %bb187, %bb185, %bb183
  br i1 %version.sroa.128.01264.off02085, label %bb200, label %bb207

bb185:                                            ; preds = %bb184
  %_179 = icmp samesign ult i32 %version.sroa.120.012662058, 18
  br i1 %_179, label %bb186, label %bb199

bb186:                                            ; preds = %bb185
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_d611028f9649b3783e472c0e81960e47, ptr noundef nonnull inttoptr (i64 89 to ptr))
          to label %bb187 unwind label %cleanup7

bb187:                                            ; preds = %bb186
  %_183 = icmp samesign ult i32 %version.sroa.120.012662058, 16
  br i1 %_183, label %bb188, label %bb199

bb188:                                            ; preds = %bb187
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9050225349f21addcf6eebdb0ba383e4, ptr noundef nonnull inttoptr (i64 89 to ptr))
          to label %bb189 unwind label %cleanup7

bb189:                                            ; preds = %bb188
  %_187.not = icmp eq i32 %version.sroa.120.012662058, 15
  br i1 %_187.not, label %bb199, label %bb190

bb190:                                            ; preds = %bb189
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8d616064d0ee2b338ed7caf95f46cce6, ptr noundef nonnull inttoptr (i64 89 to ptr))
          to label %bb199 unwind label %cleanup7

bb207:                                            ; preds = %bb205.thread, %bb205, %bb2.i.i.i4.i.i689, %bb199
  switch i64 %_12.sroa.8.0.copyload, label %bb372 [
    i64 6, label %bb401
    i64 7, label %bb402
    i64 3, label %bb404
    i64 9, label %bb407
    i64 5, label %bb408
  ]

bb200:                                            ; preds = %bb199
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_193)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_193, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1bde45392581cd9043bc066293d5f001, i64 noundef 18)
          to label %bb201 unwind label %cleanup7

bb201:                                            ; preds = %bb200
  call void @llvm.experimental.noalias.scope.decl(metadata !825)
  call void @llvm.experimental.noalias.scope.decl(metadata !828)
  %_2.i = load i64, ptr %_193, align 8, !range !37, !alias.scope !828, !noalias !825, !noundef !3
  %422 = trunc nuw i64 %_2.i to i1
  br i1 %422, label %bb3.i.i, label %bb202

bb3.i.i:                                          ; preds = %bb201
  call void @llvm.experimental.noalias.scope.decl(metadata !830)
  %423 = getelementptr inbounds nuw i8, ptr %_193, i64 8
  %424 = load i64, ptr %423, align 8, !range !11, !alias.scope !833, !noalias !825, !noundef !3
  switch i64 %424, label %bb1.sink.split.i.i [
    i64 -9223372036854775808, label %bb205.thread
    i64 0, label %bb205.thread
  ]

bb1.sink.split.i.i:                               ; preds = %bb3.i.i
  %425 = getelementptr inbounds nuw i8, ptr %_193, i64 16
  %_1.val1.i.i.i = load ptr, ptr %425, align 8, !alias.scope !836, !noalias !825, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i, i64 noundef %424, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !837
  br label %bb205.thread

bb205.thread:                                     ; preds = %bb1.sink.split.i.i, %bb3.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_193)
  br label %bb207

bb202:                                            ; preds = %bb201
  %426 = getelementptr inbounds nuw i8, ptr %_193, i64 8
  %sanitize.sroa.0.0.copyload = load i64, ptr %426, align 8, !alias.scope !838
  %sanitize.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_193, i64 16
  %sanitize.sroa.6.0.copyload = load ptr, ptr %sanitize.sroa.6.0..sroa_idx, align 8, !alias.scope !838
  %sanitize.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_193, i64 24
  %sanitize.sroa.10.0.copyload = load i64, ptr %sanitize.sroa.10.0..sroa_idx, align 8, !alias.scope !838
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_193)
  %427 = icmp ugt i64 %sanitize.sroa.10.0.copyload, 6
  br i1 %427, label %bb7.i660, label %bb3.i656

bb3.i656:                                         ; preds = %bb202
  %_3.not.i.i = icmp eq i64 %sanitize.sroa.10.0.copyload, 6
  br i1 %_3.not.i.i, label %bb400, label %bb205

bb7.i660:                                         ; preds = %bb202
  call void @llvm.lifetime.start.p0(i64 104, ptr nonnull %_14.i), !noalias !839
; invoke <core::str::pattern::StrSearcher>::new
  invoke void @_RNvMsu_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcher3new(ptr noalias noundef nonnull sret([104 x i8]) align 8 captures(none) dereferenceable(104) %_14.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %sanitize.sroa.6.0.copyload, i64 noundef %sanitize.sroa.10.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0ccf5eeb19a73b85efabd846cfd6625c, i64 noundef 6)
          to label %.noexc679 unwind label %cleanup10

.noexc679:                                        ; preds = %bb7.i660
  call void @llvm.experimental.noalias.scope.decl(metadata !842)
  %_2.i.i661 = load i64, ptr %_14.i, align 8, !range !37, !alias.scope !842, !noalias !845, !noundef !3
  %428 = trunc nuw i64 %_2.i.i661 to i1
  br i1 %428, label %bb2.i2.i, label %bb3.i.critedge.i.i

bb2.i2.i:                                         ; preds = %.noexc679
  %searcher.i.i = getelementptr inbounds nuw i8, ptr %_14.i, i64 8
  %429 = getelementptr inbounds nuw i8, ptr %_14.i, i64 56
  %_10.i.i669 = load i64, ptr %429, align 8, !alias.scope !842, !noalias !845, !noundef !3
  %is_long.i.i = icmp eq i64 %_10.i.i669, -1
  %430 = getelementptr inbounds nuw i8, ptr %_14.i, i64 72
  %self.03.i.i = load ptr, ptr %430, align 8, !alias.scope !842, !noalias !845, !nonnull !3, !align !18, !noundef !3
  %431 = getelementptr inbounds nuw i8, ptr %_14.i, i64 80
  %self.14.i.i = load i64, ptr %431, align 8, !alias.scope !842, !noalias !845, !noundef !3
  %432 = getelementptr inbounds nuw i8, ptr %_14.i, i64 88
  %self.05.i.i = load ptr, ptr %432, align 8, !alias.scope !842, !noalias !845, !nonnull !3, !align !18, !noundef !3
  %433 = getelementptr inbounds nuw i8, ptr %_14.i, i64 96
  %self.16.i.i = load i64, ptr %433, align 8, !alias.scope !842, !noalias !845, !noundef !3
  %434 = getelementptr inbounds nuw i8, ptr %_14.i, i64 40
  %needle_last.i.i = add nsw i64 %self.16.i.i, -1
  br i1 %is_long.i.i, label %bb8.i.i671, label %bb9.i.i670

bb3.i.critedge.i.i:                               ; preds = %.noexc679
  call void @llvm.experimental.noalias.scope.decl(metadata !847)
  %435 = getelementptr inbounds nuw i8, ptr %_14.i, i64 26
  %436 = load i8, ptr %435, align 2, !range !54, !alias.scope !850, !noalias !851, !noundef !3
  %_4.i.i.i = trunc nuw i8 %436 to i1
  br i1 %_4.i.i.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816, label %bb5.i.lr.ph.i.i

bb5.i.lr.ph.i.i:                                  ; preds = %bb3.i.critedge.i.i
  %437 = getelementptr inbounds nuw i8, ptr %_14.i, i64 8
  %.promoted.i.i = load i64, ptr %437, align 8, !alias.scope !842, !noalias !845
  %438 = getelementptr inbounds nuw i8, ptr %_14.i, i64 24
  %439 = getelementptr inbounds nuw i8, ptr %_14.i, i64 72
  %self.0.i.i.i = load ptr, ptr %439, align 8, !alias.scope !850, !noalias !851, !nonnull !3, !align !18, !noundef !3
  %440 = getelementptr inbounds nuw i8, ptr %_14.i, i64 80
  %self.1.i.i.i = load i64, ptr %440, align 8, !alias.scope !850, !noalias !851, !noundef !3
  %_48.i.i.i = getelementptr inbounds nuw i8, ptr %self.0.i.i.i, i64 %self.1.i.i.i
  %.promoted29.i.i = load i8, ptr %438, align 8, !alias.scope !850, !noalias !851
  %extract.t.i.i = trunc i8 %.promoted29.i.i to i1
  %441 = icmp eq i64 %.promoted.i.i, 0
  br i1 %441, label %bb19.i.i.peel.i, label %bb5.i.i.i.peel.i

bb5.i.i.i.peel.i:                                 ; preds = %bb5.i.lr.ph.i.i
  %_8.not.i.i.i.peel.i = icmp ult i64 %.promoted.i.i, %self.1.i.i.i
  br i1 %_8.not.i.i.i.peel.i, label %bb9.i.i.i.peel.i, label %bb6.i.i.i.peel.i

bb6.i.i.i.peel.i:                                 ; preds = %bb5.i.i.i.peel.i
  %442 = icmp eq i64 %.promoted.i.i, %self.1.i.i.i
  br i1 %442, label %bb19.i.i.peel.i, label %bb18.i.i.i

bb9.i.i.i.peel.i:                                 ; preds = %bb5.i.i.i.peel.i
  %443 = getelementptr inbounds nuw i8, ptr %self.0.i.i.i, i64 %.promoted.i.i
  %self1.i.i.i.peel.i = load i8, ptr %443, align 1, !alias.scope !853, !noalias !856, !noundef !3
  %444 = icmp sgt i8 %self1.i.i.i.peel.i, -65
  br i1 %444, label %bb19.i.i.peel.i, label %bb18.i.i.i

bb19.i.i.peel.i:                                  ; preds = %bb9.i.i.i.peel.i, %bb6.i.i.i.peel.i, %bb5.i.lr.ph.i.i
  %data.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %self.0.i.i.i, i64 %.promoted.i.i
  %_6.i.i.i.i.peel.i = icmp samesign eq i64 %.promoted.i.i, %self.1.i.i.i
  br i1 %_6.i.i.i.i.peel.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i, label %bb14.i.i.i.peel.i

bb14.i.i.i.peel.i:                                ; preds = %bb19.i.i.peel.i
  %x.i.i.i.peel.i = load i8, ptr %data.i.i.i.peel.i, align 1, !noalias !858, !noundef !3
  %_6.i.i.i.peel.i = icmp sgt i8 %x.i.i.i.peel.i, -1
  br i1 %_6.i.i.i.peel.i, label %bb3.i.i.i.peel.i, label %bb4.i.i.i.peel.i

bb4.i.i.i.peel.i:                                 ; preds = %bb14.i.i.i.peel.i
  %_16.i.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 1
  %_30.i.i.i.peel.i = and i8 %x.i.i.i.peel.i, 31
  %init.i.i.i.peel.i = zext nneg i8 %_30.i.i.i.peel.i to i32
  %_6.i10.i.i.i.peel.i = icmp ne ptr %_16.i.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.peel.i)
  %y.i.i.i.peel.i = load i8, ptr %_16.i.i.i.i.peel.i, align 1, !noalias !858, !noundef !3
  %_33.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 6
  %_35.i.i.i.peel.i = and i8 %y.i.i.i.peel.i, 63
  %_34.i.i.i.peel.i = zext nneg i8 %_35.i.i.i.peel.i to i32
  %445 = or disjoint i32 %_33.i.i.i.peel.i, %_34.i.i.i.peel.i
  %_13.i.i.i.peel.i = icmp samesign ugt i8 %x.i.i.i.peel.i, -33
  br i1 %_13.i.i.i.peel.i, label %bb6.i21.i.i.peel.i, label %bb22.i.i.peel.i

bb6.i21.i.i.peel.i:                               ; preds = %bb4.i.i.i.peel.i
  %_16.i12.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 2
  %_6.i17.i.i.i.peel.i = icmp ne ptr %_16.i12.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.peel.i)
  %z.i.i.i.peel.i = load i8, ptr %_16.i12.i.i.i.peel.i, align 1, !noalias !858, !noundef !3
  %_38.i.i.i.peel.i = shl nuw nsw i32 %_34.i.i.i.peel.i, 6
  %_40.i.i.i.peel.i = and i8 %z.i.i.i.peel.i, 63
  %_39.i.i.i.peel.i = zext nneg i8 %_40.i.i.i.peel.i to i32
  %y_z.i.i.i.peel.i = or disjoint i32 %_38.i.i.i.peel.i, %_39.i.i.i.peel.i
  %_20.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 12
  %446 = or disjoint i32 %y_z.i.i.i.peel.i, %_20.i.i.i.peel.i
  %_21.i.i.i.peel.i = icmp samesign ugt i8 %x.i.i.i.peel.i, -17
  br i1 %_21.i.i.i.peel.i, label %bb8.i.i.i.peel.i, label %bb22.i.i.peel.i

bb8.i.i.i.peel.i:                                 ; preds = %bb6.i21.i.i.peel.i
  %_16.i19.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 3
  %_6.i24.i.i.i.peel.i = icmp ne ptr %_16.i19.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.peel.i)
  %w.i.i.i.peel.i = load i8, ptr %_16.i19.i.i.i.peel.i, align 1, !noalias !858, !noundef !3
  %_26.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 18
  %_25.i.i.i.peel.i = and i32 %_26.i.i.i.peel.i, 1835008
  %_43.i.i.i.peel.i = shl nuw nsw i32 %y_z.i.i.i.peel.i, 6
  %_45.i.i.i.peel.i = and i8 %w.i.i.i.peel.i, 63
  %_44.i.i.i.peel.i = zext nneg i8 %_45.i.i.i.peel.i to i32
  %_27.i.i.i.peel.i = or disjoint i32 %_43.i.i.i.peel.i, %_44.i.i.i.peel.i
  %447 = or disjoint i32 %_27.i.i.i.peel.i, %_25.i.i.i.peel.i
  br label %bb22.i.i.peel.i

bb3.i.i.i.peel.i:                                 ; preds = %bb14.i.i.i.peel.i
  %_7.i.i.i.peel.i = zext nneg i8 %x.i.i.i.peel.i to i32
  br label %bb22.i.i.peel.i

bb22.i.i.peel.i:                                  ; preds = %bb3.i.i.i.peel.i, %bb8.i.i.i.peel.i, %bb6.i21.i.i.peel.i, %bb4.i.i.i.peel.i
  %_0.sroa.4.0.i.ph.i.i.peel.i = phi i32 [ %445, %bb4.i.i.i.peel.i ], [ %446, %bb6.i21.i.i.peel.i ], [ %447, %bb8.i.i.i.peel.i ], [ %_7.i.i.i.peel.i, %bb3.i.i.i.peel.i ]
  %448 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 1114112
  call void @llvm.assume(i1 %448)
  br i1 %extract.t.i.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread, label %bb39.i.i.peel.i

bb39.i.i.peel.i:                                  ; preds = %bb22.i.i.peel.i
  %_59.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 128
  br i1 %_59.i.i.peel.i, label %bb5.i.i.i, label %bb26.i.i.peel.i

bb26.i.i.peel.i:                                  ; preds = %bb39.i.i.peel.i
  %_60.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 2048
  br i1 %_60.i.i.peel.i, label %bb5.i.i.i, label %bb27.i.i.peel.i

bb27.i.i.peel.i:                                  ; preds = %bb26.i.i.peel.i
  %_61.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 65536
  %..i.i.peel.i = select i1 %_61.i.i.peel.i, i64 3, i64 4
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb27.i.i.peel.i, %bb26.i.i.peel.i, %bb39.i.i.peel.i
  %_14.sroa.0.0.i.i.peel.i = phi i64 [ 1, %bb39.i.i.peel.i ], [ %..i.i.peel.i, %bb27.i.i.peel.i ], [ 2, %bb26.i.i.peel.i ]
  %449 = add i64 %_14.sroa.0.0.i.i.peel.i, %.promoted.i.i
  %450 = icmp eq i64 %449, 0
  br i1 %450, label %bb19.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb5.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %449, %self.1.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i662

bb6.i.i.i.i662:                                   ; preds = %bb5.i.i.i.i
  %451 = icmp eq i64 %449, %self.1.i.i.i
  br i1 %451, label %bb19.i.i.i, label %bb18.i.i.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %452 = getelementptr inbounds nuw i8, ptr %self.0.i.i.i, i64 %449
  %self1.i.i.i.i668 = load i8, ptr %452, align 1, !alias.scope !853, !noalias !861, !noundef !3
  %453 = icmp sgt i8 %self1.i.i.i.i668, -65
  br i1 %453, label %bb19.i.i.i, label %bb18.i.i.i

bb19.i.i.i:                                       ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i662, %bb5.i.i.i
  %data.i.i.i.i663 = getelementptr inbounds nuw i8, ptr %self.0.i.i.i, i64 %449
  %_6.i.i.i.i.i = icmp samesign eq i64 %449, %self.1.i.i.i
  br i1 %_6.i.i.i.i.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb19.i.i.i
  %x.i.i.i.i = load i8, ptr %data.i.i.i.i663, align 1, !noalias !862, !noundef !3
  %_6.i.i.i.i664 = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i664, label %bb3.i.i.i.i666, label %bb4.i.i.i.i665

bb4.i.i.i.i665:                                   ; preds = %bb14.i.i.i.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i663, i64 1
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !862, !noundef !3
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %454 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i21.i.i.i, label %bb7.loopexit.i.loopexit.i

bb3.i.i.i.i666:                                   ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i667 = zext nneg i8 %x.i.i.i.i to i32
  br label %bb7.loopexit.i.loopexit.i

bb6.i21.i.i.i:                                    ; preds = %bb4.i.i.i.i665
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i663, i64 2
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !862, !noundef !3
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %455 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb7.loopexit.i.loopexit.i

bb8.i.i.i.i:                                      ; preds = %bb6.i21.i.i.i
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i663, i64 3
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !862, !noundef !3
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %456 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb7.loopexit.i.loopexit.i

bb18.i.i.i:                                       ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i662, %bb9.i.i.i.peel.i, %bb6.i.i.i.peel.i
  %.lcssa90.i = phi i64 [ %.promoted.i.i, %bb6.i.i.i.peel.i ], [ %.promoted.i.i, %bb9.i.i.i.peel.i ], [ %449, %bb6.i.i.i.i662 ], [ %449, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0.i.i.i, i64 noundef %self.1.i.i.i, i64 noundef %.lcssa90.i, i64 noundef %self.1.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_559c4f386b668c946885822cef1a587d) #29
          to label %.noexc680 unwind label %cleanup10

.noexc680:                                        ; preds = %bb18.i.i.i
  unreachable

bb7.loopexit.i.loopexit.i:                        ; preds = %bb8.i.i.i.i, %bb6.i21.i.i.i, %bb3.i.i.i.i666, %bb4.i.i.i.i665
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %454, %bb4.i.i.i.i665 ], [ %455, %bb6.i21.i.i.i ], [ %456, %bb8.i.i.i.i ], [ %_7.i.i.i.i667, %bb3.i.i.i.i666 ]
  %457 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  call void @llvm.assume(i1 %457)
  br label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread

bb9.i.i670:                                       ; preds = %bb2.i2.i
  call void @llvm.experimental.noalias.scope.decl(metadata !863)
  call void @llvm.experimental.noalias.scope.decl(metadata !866)
  call void @llvm.experimental.noalias.scope.decl(metadata !868)
  %.promoted.i8.i = load i64, ptr %434, align 8, !alias.scope !863, !noalias !870
  %index24.i9.i = add i64 %.promoted.i8.i, %needle_last.i.i
  %_5325.i10.i = icmp ult i64 %index24.i9.i, %self.14.i.i
  br i1 %_5325.i10.i, label %bb39.lr.ph.i13.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816

bb39.lr.ph.i13.i:                                 ; preds = %bb9.i.i670
  %458 = getelementptr inbounds nuw i8, ptr %_14.i, i64 32
  %_58.i14.i = load i64, ptr %458, align 8, !alias.scope !863, !noalias !870, !noundef !3
  %v1.i15.i = load i64, ptr %searcher.i.i, align 8, !alias.scope !863, !noalias !870
  %459 = getelementptr inbounds nuw i8, ptr %_14.i, i64 24
  %_48.i16.i = load i64, ptr %459, align 8, !alias.scope !863, !noalias !870
  %460 = sub i64 %self.16.i.i, %_48.i16.i
  br label %bb39.i18.i

bb39.i18.i:                                       ; preds = %bb37.sink.split.i.i, %bb39.lr.ph.i13.i
  %461 = phi i64 [ %.promoted.i8.i, %bb39.lr.ph.i13.i ], [ %.ph.i.i, %bb37.sink.split.i.i ]
  %v229.i19.i = phi i64 [ %_10.i.i669, %bb39.lr.ph.i13.i ], [ %.sink.i.i, %bb37.sink.split.i.i ]
  %index26.i20.i = phi i64 [ %index24.i9.i, %bb39.lr.ph.i13.i ], [ %index.i45.i, %bb37.sink.split.i.i ]
  %_55.i21.i = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %index26.i20.i
  %tail_byte.i22.i = load i8, ptr %_55.i21.i, align 1, !alias.scope !866, !noalias !872, !noundef !3
  %_60.i23.i = and i8 %tail_byte.i22.i, 63
  %_59.i24.i = zext nneg i8 %_60.i23.i to i64
  %462 = shl nuw i64 1, %_59.i24.i
  %463 = and i64 %462, %_58.i14.i
  %464 = icmp eq i64 %463, 0
  br i1 %464, label %bb10.i65.i, label %bb9.i25.i

bb10.i65.i:                                       ; preds = %bb39.i18.i
  %465 = add i64 %461, %self.16.i.i
  br label %bb37.sink.split.i.i

bb9.i25.i:                                        ; preds = %bb39.i18.i
  %_0.sroa.0.0.i.i26.i = call i64 @llvm.umax.i64(i64 %v229.i19.i, i64 %v1.i15.i)
  %umax40.i27.i = call i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i26.i, i64 range(i64 0, -9223372036854775808) %self.16.i.i)
  br label %bb16.i28.i

bb37.sink.split.i.i:                              ; preds = %bb19.i41.i, %bb29.i64.i, %bb10.i65.i
  %.sink.i.i = phi i64 [ %460, %bb29.i64.i ], [ 0, %bb19.i41.i ], [ 0, %bb10.i65.i ]
  %.ph.i.i = phi i64 [ %468, %bb29.i64.i ], [ %472, %bb19.i41.i ], [ %465, %bb10.i65.i ]
  %index.i45.i = add i64 %.ph.i.i, %needle_last.i.i
  %_53.i46.i = icmp ult i64 %index.i45.i, %self.14.i.i
  br i1 %_53.i46.i, label %bb39.i18.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816

bb16.i28.i:                                       ; preds = %bb18.i36.i, %bb9.i25.i
  %iter.sroa.0.0.i29.i = phi i64 [ %_0.sroa.0.0.i.i26.i, %bb9.i25.i ], [ %_65.i37.i, %bb18.i36.i ]
  %exitcond.not.i30.i = icmp eq i64 %iter.sroa.0.0.i29.i, %umax40.i27.i
  br i1 %exitcond.not.i30.i, label %bb26.i48.i, label %bb42.i31.i

bb42.i31.i:                                       ; preds = %bb16.i28.i
  %_28.i32.i = add i64 %iter.sroa.0.0.i29.i, %461
  %_30.i33.i = icmp ult i64 %_28.i32.i, %self.14.i.i
  br i1 %_30.i33.i, label %bb18.i36.i, label %panic8.i34.i

bb26.i48.i:                                       ; preds = %bb16.i28.i, %bb28.i60.i
  %iter3.sroa.2.0.i49.i = phi i64 [ %_73.i53.i, %bb28.i60.i ], [ %v1.i15.i, %bb16.i28.i ]
  %_70.i50.i = icmp ult i64 %v229.i19.i, %iter3.sroa.2.0.i49.i
  br i1 %_70.i50.i, label %bb46.i52.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread

bb46.i52.i:                                       ; preds = %bb26.i48.i
  %_73.i53.i = add i64 %iter3.sroa.2.0.i49.i, -1
  %_43.i54.i = icmp ult i64 %_73.i53.i, %self.16.i.i
  br i1 %_43.i54.i, label %bb27.i56.i, label %panic8.i.i.invoke

bb27.i56.i:                                       ; preds = %bb46.i52.i
  %_45.i57.i = add i64 %_73.i53.i, %461
  %_47.i58.i = icmp ult i64 %_45.i57.i, %self.14.i.i
  br i1 %_47.i58.i, label %bb28.i60.i, label %panic8.i.i.invoke

bb28.i60.i:                                       ; preds = %bb27.i56.i
  %466 = getelementptr inbounds nuw i8, ptr %self.05.i.i, i64 %_73.i53.i
  %_42.i61.i = load i8, ptr %466, align 1, !alias.scope !868, !noalias !873, !noundef !3
  %467 = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %_45.i57.i
  %_44.i62.i = load i8, ptr %467, align 1, !alias.scope !866, !noalias !872, !noundef !3
  %_41.not.i63.i = icmp eq i8 %_42.i61.i, %_44.i62.i
  br i1 %_41.not.i63.i, label %bb26.i48.i, label %bb29.i64.i

bb29.i64.i:                                       ; preds = %bb28.i60.i
  %468 = add i64 %461, %_48.i16.i
  br label %bb37.sink.split.i.i

bb18.i36.i:                                       ; preds = %bb42.i31.i
  %_65.i37.i = add i64 %iter.sroa.0.0.i29.i, 1
  %469 = getelementptr inbounds nuw i8, ptr %self.05.i.i, i64 %iter.sroa.0.0.i29.i
  %_25.i38.i = load i8, ptr %469, align 1, !alias.scope !868, !noalias !873, !noundef !3
  %470 = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %_28.i32.i
  %_27.i39.i = load i8, ptr %470, align 1, !alias.scope !866, !noalias !872, !noundef !3
  %_24.not.i40.i = icmp eq i8 %_25.i38.i, %_27.i39.i
  br i1 %_24.not.i40.i, label %bb16.i28.i, label %bb19.i41.i

panic8.i34.i:                                     ; preds = %bb42.i31.i
  %471 = add i64 %_0.sroa.0.0.i.i26.i, %461
  %umax.i35.i = call i64 @llvm.umax.i64(i64 range(i64 0, -9223372036854775808) %self.14.i.i, i64 %471)
  br label %panic8.i.i.invoke

bb19.i41.i:                                       ; preds = %bb18.i36.i
  %reass.sub = sub i64 %461, %v1.i15.i
  %_31.i43.i = add i64 %reass.sub, 1
  %472 = add i64 %_31.i43.i, %iter.sroa.0.0.i29.i
  br label %bb37.sink.split.i.i

bb8.i.i671:                                       ; preds = %bb2.i2.i
  call void @llvm.experimental.noalias.scope.decl(metadata !874)
  call void @llvm.experimental.noalias.scope.decl(metadata !877)
  call void @llvm.experimental.noalias.scope.decl(metadata !879)
  %.promoted.i3.i = load i64, ptr %434, align 8, !alias.scope !874, !noalias !881
  %index24.i.i = add i64 %.promoted.i3.i, %needle_last.i.i
  %_5325.i.i = icmp ult i64 %index24.i.i, %self.14.i.i
  br i1 %_5325.i.i, label %bb39.lr.ph.i.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816

bb39.lr.ph.i.i:                                   ; preds = %bb8.i.i671
  %473 = getelementptr inbounds nuw i8, ptr %_14.i, i64 32
  %_58.i.i = load i64, ptr %473, align 8, !alias.scope !874, !noalias !881, !noundef !3
  %v1.i.i = load i64, ptr %searcher.i.i, align 8, !alias.scope !874, !noalias !881
  %v1.i.i.fr = freeze i64 %v1.i.i
  %474 = getelementptr inbounds nuw i8, ptr %_14.i, i64 24
  %_48.i.i = load i64, ptr %474, align 8, !alias.scope !874, !noalias !881
  %umax40.i.i = call i64 @llvm.umax.i64(i64 %v1.i.i.fr, i64 range(i64 0, -9223372036854775808) %self.16.i.i)
  %475 = add i64 %v1.i.i.fr, -1
  %_43.i.first_iter.i = icmp ult i64 %475, %self.16.i.i
  br label %bb39.i.i

bb39.i.i:                                         ; preds = %bb37.i.i675, %bb39.lr.ph.i.i
  %476 = phi i64 [ %.promoted.i3.i, %bb39.lr.ph.i.i ], [ %481, %bb37.i.i675 ]
  %index26.i.i = phi i64 [ %index24.i.i, %bb39.lr.ph.i.i ], [ %index.i.i676, %bb37.i.i675 ]
  %_55.i.i = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %index26.i.i
  %tail_byte.i.i = load i8, ptr %_55.i.i, align 1, !alias.scope !877, !noalias !883, !noundef !3
  %_60.i.i = and i8 %tail_byte.i.i, 63
  %_59.i.i = zext nneg i8 %_60.i.i to i64
  %477 = shl nuw i64 1, %_59.i.i
  %478 = and i64 %477, %_58.i.i
  %479 = icmp eq i64 %478, 0
  br i1 %479, label %bb10.i6.i, label %bb16.i.i

bb10.i6.i:                                        ; preds = %bb39.i.i
  %480 = add i64 %476, %self.16.i.i
  br label %bb37.i.i675

bb37.i.i675:                                      ; preds = %bb19.i.i, %bb29.i.i.split.us, %bb10.i6.i
  %481 = phi i64 [ %491, %bb19.i.i ], [ %484, %bb29.i.i.split.us ], [ %480, %bb10.i6.i ]
  %index.i.i676 = add i64 %481, %needle_last.i.i
  %_53.i.i = icmp ult i64 %index.i.i676, %self.14.i.i
  br i1 %_53.i.i, label %bb39.i.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816

bb16.i.i:                                         ; preds = %bb39.i.i, %bb18.i.i673
  %iter.sroa.0.0.i.i = phi i64 [ %_65.i.i, %bb18.i.i673 ], [ %v1.i.i.fr, %bb39.i.i ]
  %exitcond.not.i.i = icmp eq i64 %iter.sroa.0.0.i.i, %umax40.i.i
  br i1 %exitcond.not.i.i, label %bb26.i.i.preheader, label %bb42.i.i672

bb26.i.i.preheader:                               ; preds = %bb16.i.i
  br i1 %_43.i.first_iter.i, label %bb26.i.i.us, label %bb26.i.i

bb26.i.i.us:                                      ; preds = %bb26.i.i.preheader, %bb28.i.i678.us
  %iter3.sroa.2.0.i.i.us = phi i64 [ %_73.i.i.us, %bb28.i.i678.us ], [ %v1.i.i.fr, %bb26.i.i.preheader ]
  %_70.i.not.i.us = icmp eq i64 %iter3.sroa.2.0.i.i.us, 0
  br i1 %_70.i.not.i.us, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread, label %bb46.i.i677.us

bb46.i.i677.us:                                   ; preds = %bb26.i.i.us
  %_73.i.i.us = add i64 %iter3.sroa.2.0.i.i.us, -1
  %_45.i.i.us = add i64 %_73.i.i.us, %476
  %_47.i.i.us = icmp ult i64 %_45.i.i.us, %self.14.i.i
  br i1 %_47.i.i.us, label %bb28.i.i678.us, label %panic8.i.i.invoke

bb28.i.i678.us:                                   ; preds = %bb46.i.i677.us
  %482 = getelementptr inbounds nuw i8, ptr %self.05.i.i, i64 %_73.i.i.us
  %_42.i.i.us = load i8, ptr %482, align 1, !alias.scope !879, !noalias !884, !noundef !3
  %483 = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %_45.i.i.us
  %_44.i.i.us = load i8, ptr %483, align 1, !alias.scope !877, !noalias !883, !noundef !3
  %_41.not.i.i.us = icmp eq i8 %_42.i.i.us, %_44.i.i.us
  br i1 %_41.not.i.i.us, label %bb26.i.i.us, label %bb29.i.i.split.us

bb29.i.i.split.us:                                ; preds = %bb28.i.i678.us
  %484 = add i64 %476, %_48.i.i
  br label %bb37.i.i675

bb42.i.i672:                                      ; preds = %bb16.i.i
  %_28.i.i = add i64 %iter.sroa.0.0.i.i, %476
  %_30.i.i = icmp ult i64 %_28.i.i, %self.14.i.i
  br i1 %_30.i.i, label %bb18.i.i673, label %panic8.i.i

bb26.i.i:                                         ; preds = %bb26.i.i.preheader
  %_70.i.not.i = icmp eq i64 %v1.i.i.fr, 0
  br i1 %_70.i.not.i, label %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread, label %panic8.i.i.invoke

bb18.i.i673:                                      ; preds = %bb42.i.i672
  %_65.i.i = add i64 %iter.sroa.0.0.i.i, 1
  %485 = getelementptr inbounds nuw i8, ptr %self.05.i.i, i64 %iter.sroa.0.0.i.i
  %_25.i.i = load i8, ptr %485, align 1, !alias.scope !879, !noalias !884, !noundef !3
  %486 = getelementptr inbounds nuw i8, ptr %self.03.i.i, i64 %_28.i.i
  %_27.i.i = load i8, ptr %486, align 1, !alias.scope !877, !noalias !883, !noundef !3
  %_24.not.i.i = icmp eq i8 %_25.i.i, %_27.i.i
  br i1 %_24.not.i.i, label %bb16.i.i, label %bb19.i.i

panic8.i.i:                                       ; preds = %bb42.i.i672
  %487 = add i64 %476, %v1.i.i.fr
  %umax.i.i = call i64 @llvm.umax.i64(i64 range(i64 0, -9223372036854775808) %self.14.i.i, i64 %487)
  br label %panic8.i.i.invoke

panic8.i.i.invoke:                                ; preds = %bb27.i56.i, %bb46.i52.i, %bb46.i.i677.us, %bb26.i.i, %panic8.i34.i, %panic8.i.i
  %488 = phi i64 [ %umax.i.i, %panic8.i.i ], [ %umax.i35.i, %panic8.i34.i ], [ %475, %bb26.i.i ], [ %_45.i.i.us, %bb46.i.i677.us ], [ %_73.i53.i, %bb46.i52.i ], [ %_45.i57.i, %bb27.i56.i ]
  %489 = phi i64 [ %self.14.i.i, %panic8.i.i ], [ %self.14.i.i, %panic8.i34.i ], [ %self.16.i.i, %bb26.i.i ], [ %self.14.i.i, %bb46.i.i677.us ], [ %self.16.i.i, %bb46.i52.i ], [ %self.14.i.i, %bb27.i56.i ]
  %490 = phi ptr [ @alloc_cfc145f12794171662ae0bd5e97799ce, %panic8.i.i ], [ @alloc_cfc145f12794171662ae0bd5e97799ce, %panic8.i34.i ], [ @alloc_3c3a438693b52af6c6b31c2cc77620da, %bb26.i.i ], [ @alloc_759b6db6182a2ae5f8169b55f322d553, %bb46.i.i677.us ], [ @alloc_3c3a438693b52af6c6b31c2cc77620da, %bb46.i52.i ], [ @alloc_759b6db6182a2ae5f8169b55f322d553, %bb27.i56.i ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %488, i64 noundef range(i64 0, -9223372036854775808) %489, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %490) #29
          to label %panic8.i.i.cont unwind label %cleanup10

panic8.i.i.cont:                                  ; preds = %panic8.i.i.invoke
  unreachable

bb19.i.i:                                         ; preds = %bb18.i.i673
  %reass.sub1899 = sub i64 %476, %v1.i.i.fr
  %_31.i.i674 = add i64 %reass.sub1899, 1
  %491 = add i64 %_31.i.i674, %iter.sroa.0.0.i.i
  br label %bb37.i.i675

_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread: ; preds = %bb26.i48.i, %bb26.i.i.us, %bb26.i.i, %bb19.i.i.i, %bb7.loopexit.i.loopexit.i, %bb22.i.i.peel.i
  call void @llvm.lifetime.end.p0(i64 104, ptr nonnull %_14.i), !noalias !839
  br label %bb203

_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816: ; preds = %bb37.sink.split.i.i, %bb37.i.i675, %bb3.i.critedge.i.i, %bb9.i.i670, %bb8.i.i671
  call void @llvm.lifetime.end.p0(i64 104, ptr nonnull %_14.i), !noalias !839
  br label %bb205

_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i: ; preds = %bb19.i.i.peel.i
  call void @llvm.lifetime.end.p0(i64 104, ptr nonnull %_14.i), !noalias !839
  br i1 %extract.t.i.i, label %bb203, label %bb205

cleanup10:                                        ; preds = %panic8.i.i.invoke, %bb18.i.i.i, %bb7.i660, %bb203
  %492 = landingpad { ptr, i32 }
          cleanup
  %493 = icmp eq i64 %sanitize.sroa.0.0.copyload, 0
  br i1 %493, label %bb384, label %bb2.i.i.i4.i.i687

bb2.i.i.i4.i.i687:                                ; preds = %cleanup10
  %494 = icmp ne ptr %sanitize.sroa.6.0.copyload, null
  call void @llvm.assume(i1 %494)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %sanitize.sroa.6.0.copyload, i64 noundef %sanitize.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb384

bb400:                                            ; preds = %bb3.i656
  %495 = call i32 @memcmp(ptr noundef nonnull dereferenceable(6) @alloc_0ccf5eeb19a73b85efabd846cfd6625c, ptr noundef nonnull readonly align 1 dereferenceable(6) %sanitize.sroa.6.0.copyload, i64 6), !alias.scope !885
  %496 = icmp eq i32 %495, 0
  br i1 %496, label %bb203, label %bb205

bb205:                                            ; preds = %bb3.i656, %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread1816, %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i, %bb203, %bb400
  %497 = icmp eq i64 %sanitize.sroa.0.0.copyload, 0
  br i1 %497, label %bb207, label %bb2.i.i.i4.i.i689

bb2.i.i.i4.i.i689:                                ; preds = %bb205
  %498 = icmp ne ptr %sanitize.sroa.6.0.copyload, null
  call void @llvm.assume(i1 %498)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %sanitize.sroa.6.0.copyload, i64 noundef %sanitize.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb207

bb203:                                            ; preds = %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i.thread, %_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match.exit.i, %bb400
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_d1324d222df80959476a7b359dea6188, ptr noundef nonnull inttoptr (i64 97 to ptr))
          to label %bb205 unwind label %cleanup10

bb401:                                            ; preds = %bb207
  %499 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_4a29a4faa0904cd7ff982831f2813e90, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !889
  %500 = icmp eq i32 %499, 0
  br i1 %500, label %bb208, label %bb372

bb208:                                            ; preds = %bb401
  br i1 %version.sroa.128.01264.off02085, label %bb1.i701, label %bb8.i698

bb8.i698:                                         ; preds = %bb208
  %501 = icmp ugt i32 %version.sroa.0.0127714051432, 68
  br i1 %501, label %bb372, label %bb223

bb1.i701:                                         ; preds = %bb208
  %_7.i702 = icmp ugt i32 %version.sroa.0.0127714051432, 69
  br i1 %_7.i702, label %bb1.i1147, label %bb3.i703

bb3.i703:                                         ; preds = %bb1.i701
  %_9.i704 = icmp eq i32 %version.sroa.0.0127714051432, 69
  br i1 %_9.i704, label %bb4.i705, label %bb223

bb4.i705:                                         ; preds = %bb3.i703
  %502 = icmp eq i16 %version.sroa.36.012721970, 2023
  %503 = icmp ugt i16 %version.sroa.36.012721970, 2022
  br i1 %502, label %bb11.i707, label %bb221

bb11.i707:                                        ; preds = %bb4.i705
  %504 = icmp eq i8 %version.sroa.64.012701997, 2
  %505 = icmp ugt i8 %version.sroa.64.012701997, 1
  br i1 %504, label %bb12.i709, label %bb221

bb12.i709:                                        ; preds = %bb11.i707
  %506 = icmp ugt i8 %version.sroa.92.012682055, 27
  br i1 %506, label %bb229.invoke, label %bb223

bb402:                                            ; preds = %bb207
  %507 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_708437d7a9a3b1bed2b2fbb27ca99947, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !893
  %508 = icmp eq i32 %507, 0
  br i1 %508, label %bb220, label %bb403

bb220:                                            ; preds = %bb403, %bb402
  br i1 %version.sroa.128.01264.off02085, label %bb1.i722, label %bb240

bb1.i722:                                         ; preds = %bb220
  %_7.i723 = icmp ugt i32 %version.sroa.0.0127714051432, 82
  br i1 %_7.i723, label %bb246, label %bb3.i724

bb3.i724:                                         ; preds = %bb1.i722
  %_9.i725 = icmp eq i32 %version.sroa.0.0127714051432, 82
  br i1 %_9.i725, label %bb4.i726, label %bb240

bb4.i726:                                         ; preds = %bb3.i724
  %509 = icmp eq i16 %version.sroa.36.012721970, 2024
  %510 = icmp ugt i16 %version.sroa.36.012721970, 2023
  %511 = icmp eq i8 %version.sroa.64.012701997, 8
  %512 = icmp ugt i8 %version.sroa.64.012701997, 7
  %513 = icmp ugt i8 %version.sroa.92.012682055, 28
  %spec.select1842 = select i1 %511, i1 %513, i1 %512
  %_0.sroa.0.0.shrunk.i721 = select i1 %509, i1 %spec.select1842, i1 %510
  br i1 %_0.sroa.0.0.shrunk.i721, label %bb246, label %bb240

bb403:                                            ; preds = %bb402
  %514 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_77091ef4013986fd40216f126dabc12f, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !897
  %515 = icmp eq i32 %514, 0
  br i1 %515, label %bb220, label %bb405

bb404:                                            ; preds = %bb207
  %516 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_d9036dbef1cc78d0c3562113c2babf56, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !901
  %517 = icmp eq i32 %516, 0
  br i1 %517, label %bb212, label %bb372

bb212:                                            ; preds = %bb404
  br i1 %version.sroa.128.01264.off02085, label %bb1.i874, label %bb423

bb405:                                            ; preds = %bb403
  %518 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_82a51f3810ec0ab0cde96eb28e0a8f16, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !905
  %519 = icmp eq i32 %518, 0
  br i1 %519, label %bb219, label %bb406

bb219:                                            ; preds = %bb406, %bb405
  br i1 %version.sroa.128.01264.off02085, label %bb1.i753, label %bb316

bb1.i753:                                         ; preds = %bb219
  %_7.i754 = icmp ugt i32 %version.sroa.0.0127714051432, 87
  br i1 %_7.i754, label %bb319, label %bb3.i755

bb3.i755:                                         ; preds = %bb1.i753
  %_9.i756 = icmp eq i32 %version.sroa.0.0127714051432, 87
  br i1 %_9.i756, label %bb4.i757, label %bb316

bb4.i757:                                         ; preds = %bb3.i755
  %520 = icmp eq i16 %version.sroa.36.012721970, 2025
  %521 = icmp ugt i16 %version.sroa.36.012721970, 2024
  %522 = icmp eq i8 %version.sroa.64.012701997, 2
  %523 = icmp ugt i8 %version.sroa.64.012701997, 1
  %524 = icmp ugt i8 %version.sroa.92.012682055, 24
  %spec.select1843 = select i1 %522, i1 %524, i1 %523
  %_0.sroa.0.0.shrunk.i752 = select i1 %520, i1 %spec.select1843, i1 %521
  br i1 %_0.sroa.0.0.shrunk.i752, label %bb319, label %bb316

bb406:                                            ; preds = %bb405
  %525 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_f566f2e0543c30db53174190aea17def, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !909
  %526 = icmp eq i32 %525, 0
  br i1 %526, label %bb219, label %bb372

bb407:                                            ; preds = %bb207
  %527 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(9) @alloc_fa1130f2f45123ef906740f12b430906, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !913
  %528 = icmp eq i32 %527, 0
  br i1 %528, label %bb216, label %bb372

bb216:                                            ; preds = %bb407
  br i1 %version.sroa.128.01264.off02085, label %bb1.i779, label %bb332

bb1.i779:                                         ; preds = %bb216
  %_7.i780 = icmp ugt i32 %version.sroa.0.0127714051432, 83
  br i1 %_7.i780, label %bb372, label %bb3.i781

bb3.i781:                                         ; preds = %bb1.i779
  %_9.i782 = icmp eq i32 %version.sroa.0.0127714051432, 83
  br i1 %_9.i782, label %bb4.i783, label %bb332

bb4.i783:                                         ; preds = %bb3.i781
  %529 = icmp eq i16 %version.sroa.36.012721970, 2024
  %530 = icmp ugt i16 %version.sroa.36.012721970, 2023
  %531 = icmp eq i8 %version.sroa.64.012701997, 9
  %532 = icmp ugt i8 %version.sroa.64.012701997, 8
  %533 = icmp ugt i8 %version.sroa.92.012682055, 26
  %spec.select1844 = select i1 %531, i1 %533, i1 %532
  %_0.sroa.0.0.shrunk.i778 = select i1 %529, i1 %spec.select1844, i1 %530
  br i1 %_0.sroa.0.0.shrunk.i778, label %bb372, label %bb332

bb408:                                            ; preds = %bb207
  %534 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_12.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_5a449a0bdb20d0cd84a204297ad784b7, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !917
  %535 = icmp eq i32 %534, 0
  br i1 %535, label %bb218, label %bb372

bb372:                                            ; preds = %bb229.invoke, %bb234.invoke, %bb1.i779, %bb207, %bb401, %bb404, %bb406, %bb407, %bb1.i1147, %bb8.i698, %bb12.i1155, %bb227, %bb1.i874, %bb12.i882, %bb260, %bb319, %bb4.i783, %bb231, %bb257, %bb408
  %536 = icmp eq i64 %_15.sroa.0.0.copyload, 0
  br i1 %536, label %bb373, label %bb2.i.i.i4.i.i790

bb2.i.i.i4.i.i790:                                ; preds = %bb372
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_15.sroa.5.0.copyload, i64 noundef %_15.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb373

bb218:                                            ; preds = %bb408
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_350)
; invoke build_script_build::target_cpu
  invoke fastcc void @_RNvCslwKqnJYeWCA_18build_script_build10target_cpu(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_350)
          to label %bb352 unwind label %cleanup7

bb352:                                            ; preds = %bb218
  %537 = load i64, ptr %_350, align 8, !range !11, !noundef !3
  %.not93 = icmp eq i64 %537, -9223372036854775808
  br i1 %.not93, label %bb389, label %bb353

bb353:                                            ; preds = %bb352
  %cpu5.sroa.5.0._350.sroa_idx = getelementptr inbounds nuw i8, ptr %_350, i64 8
  %cpu5.sroa.5.0.copyload = load ptr, ptr %cpu5.sroa.5.0._350.sroa_idx, align 8, !nonnull !3, !noundef !3
  %cpu5.sroa.9.0._350.sroa_idx = getelementptr inbounds nuw i8, ptr %_350, i64 16
  %cpu5.sroa.9.0.copyload = load i64, ptr %cpu5.sroa.9.0._350.sroa_idx, align 8
; invoke build_script_build::strip_prefix
  %538 = invoke fastcc { ptr, i64 } @_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %cpu5.sroa.5.0.copyload, i64 noundef %cpu5.sroa.9.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_fcc17c744c1d97967e2fd259ede87770, i64 noundef 4)
          to label %bb354 unwind label %cleanup11

bb389:                                            ; preds = %bb366, %bb2.i.i.i4.i.i799, %bb352
  %arch9_features.sroa.0.0.off0 = phi i1 [ false, %bb352 ], [ %arch9_features.sroa.0.2.off0, %bb2.i.i.i4.i.i799 ], [ %arch9_features.sroa.0.2.off0, %bb366 ]
  %arch13_features.sroa.0.0.off0 = phi i1 [ false, %bb352 ], [ %arch13_features.sroa.0.2.off0, %bb2.i.i.i4.i.i799 ], [ %arch13_features.sroa.0.2.off0, %bb366 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_350)
; invoke build_script_build::target_feature_fallback
  %_367 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1b0f75a7cb5e484bd78be04806919b14, i64 noundef 18, i1 noundef zeroext %arch9_features.sroa.0.0.off0)
          to label %bb368 unwind label %cleanup7

cleanup11:                                        ; preds = %bb353
  %539 = landingpad { ptr, i32 }
          cleanup
  %540 = icmp eq i64 %537, 0
  br i1 %540, label %bb384, label %bb2.i.i.i4.i.i792

bb2.i.i.i4.i.i792:                                ; preds = %cleanup11
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %cpu5.sroa.5.0.copyload, i64 noundef %537, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb384

bb354:                                            ; preds = %bb353
  %541 = extractvalue { ptr, i64 } %538, 0
  %.not94 = icmp eq ptr %541, null
  br i1 %.not94, label %bb358, label %bb355

bb355:                                            ; preds = %bb354
  %542 = extractvalue { ptr, i64 } %538, 1
; call <u32>::from_ascii_radix
  %543 = call fastcc i64 @_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %541, i64 noundef %542)
  %544 = trunc i64 %543 to i1
  %545 = icmp ugt i64 %543, 38654705663
  %546 = icmp ugt i64 %543, 55834574847
  %not. = xor i1 %544, true
  %arch9_features.sroa.0.1.off0 = and i1 %545, %not.
  %arch13_features.sroa.0.1.off0 = and i1 %546, %not.
  br label %bb366

bb358:                                            ; preds = %bb354
  switch i64 %cpu5.sroa.9.0.copyload, label %bb483.thread [
    i64 4, label %bb478
    i64 5, label %bb479
    i64 3, label %bb480
  ]

bb366:                                            ; preds = %bb483.thread, %bb478, %bb479, %bb480, %bb481, %bb364, %bb483, %bb355
  %arch9_features.sroa.0.2.off0 = phi i1 [ %arch9_features.sroa.0.1.off0, %bb355 ], [ true, %bb364 ], [ false, %bb483 ], [ true, %bb481 ], [ true, %bb480 ], [ true, %bb479 ], [ true, %bb478 ], [ false, %bb483.thread ]
  %arch13_features.sroa.0.2.off0 = phi i1 [ %arch13_features.sroa.0.1.off0, %bb355 ], [ true, %bb364 ], [ false, %bb483 ], [ false, %bb481 ], [ false, %bb480 ], [ false, %bb479 ], [ false, %bb478 ], [ false, %bb483.thread ]
  %547 = icmp eq i64 %537, 0
  br i1 %547, label %bb389, label %bb2.i.i.i4.i.i799

bb2.i.i.i4.i.i799:                                ; preds = %bb366
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %cpu5.sroa.5.0.copyload, i64 noundef %537, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb389

bb478:                                            ; preds = %bb358
  %548 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(4) @alloc_6381f5c37a2a38a20bd4bf05e9496d96, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !921
  %549 = icmp eq i32 %548, 0
  br i1 %549, label %bb366, label %bb483.thread

bb479:                                            ; preds = %bb358
  %550 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_2a818f10c905893af1e371c88f7d0169, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !925
  %551 = icmp eq i32 %550, 0
  br i1 %551, label %bb366, label %bb483.thread

bb480:                                            ; preds = %bb358
  %552 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_fb3f24020003f3819dcc1e59fb1e917c, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !929
  %553 = icmp eq i32 %552, 0
  br i1 %553, label %bb366, label %bb481

bb481:                                            ; preds = %bb480
  %554 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_3cb0e3a0d99149dd91748ce9aa02491e, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !933
  %555 = icmp eq i32 %554, 0
  br i1 %555, label %bb366, label %bb482

bb482:                                            ; preds = %bb481
  %556 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_b287530f63e31393d0bcb611dd2ef350, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !937
  %557 = icmp eq i32 %556, 0
  br i1 %557, label %bb364, label %bb483

bb483.thread:                                     ; preds = %bb479, %bb358, %bb478
  br label %bb366

bb364:                                            ; preds = %bb483, %bb482
  br label %bb366

bb483:                                            ; preds = %bb482
  %558 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %cpu5.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(3) @alloc_6d1d74389733cf20cb433539a0c6ad2d, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !941
  %559 = icmp eq i32 %558, 0
  br i1 %559, label %bb364, label %bb366

bb368:                                            ; preds = %bb389
; invoke build_script_build::target_feature_fallback
  %_369 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b685305cf7511b16b4281b53804013c9, i64 noundef 18, i1 noundef zeroext %arch9_features.sroa.0.0.off0)
          to label %bb369 unwind label %cleanup7

bb369:                                            ; preds = %bb368
; invoke build_script_build::target_feature_fallback
  %_371 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_fb9a0f244e913c69309d6ba532bf7b0c, i64 noundef 12, i1 noundef zeroext %arch9_features.sroa.0.0.off0)
          to label %bb234.invoke unwind label %cleanup7

bb332:                                            ; preds = %bb216, %bb3.i781, %bb4.i783
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_331)
; invoke build_script_build::target_cpu
  invoke fastcc void @_RNvCslwKqnJYeWCA_18build_script_build10target_cpu(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_331)
          to label %bb333 unwind label %cleanup7

bb333:                                            ; preds = %bb332
  %560 = load i64, ptr %_331, align 8, !range !11, !noundef !3
  %.not99 = icmp eq i64 %560, -9223372036854775808
  br i1 %.not99, label %bb345, label %bb334

bb334:                                            ; preds = %bb333
  %cpu.sroa.5.0._331.sroa_idx = getelementptr inbounds nuw i8, ptr %_331, i64 8
  %cpu.sroa.5.0.copyload = load ptr, ptr %cpu.sroa.5.0._331.sroa_idx, align 8, !nonnull !3, !noundef !3
  %cpu.sroa.10.0._331.sroa_idx = getelementptr inbounds nuw i8, ptr %_331, i64 16
  %cpu.sroa.10.0.copyload = load i64, ptr %cpu.sroa.10.0._331.sroa_idx, align 8
; invoke build_script_build::strip_prefix
  %561 = invoke fastcc { ptr, i64 } @_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %cpu.sroa.5.0.copyload, i64 noundef %cpu.sroa.10.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ac7dbe20bf8ff4864d2af9d0f6d43781, i64 noundef 3)
          to label %bb335 unwind label %cleanup12

bb345:                                            ; preds = %bb333
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_345)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_346)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_346, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0fa298ef16b44f409ee8019af1deb36d, i64 noundef 23)
          to label %bb347 unwind label %cleanup7

cleanup12:                                        ; preds = %bb334
  %562 = landingpad { ptr, i32 }
          cleanup
  %563 = icmp eq i64 %560, 0
  br i1 %563, label %bb384, label %bb2.i.i.i4.i.i828

bb2.i.i.i4.i.i828:                                ; preds = %cleanup12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %cpu.sroa.5.0.copyload, i64 noundef %560, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb384

bb335:                                            ; preds = %bb334
  %564 = extractvalue { ptr, i64 } %561, 0
  %565 = extractvalue { ptr, i64 } %561, 1
  %.not100 = icmp eq ptr %564, null
  br i1 %.not100, label %bb340, label %bb336

bb336:                                            ; preds = %bb335
  %_5.not.i.i = icmp eq i64 %565, 0
  br i1 %_5.not.i.i, label %bb337, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb336
  %566 = getelementptr i8, ptr %564, i64 %565
  %_16.i.i = getelementptr i8, ptr %566, i64 -1
  %rhsc.i.i = load i8, ptr %_16.i.i, align 1, !alias.scope !945, !noalias !950
  %567 = icmp eq i8 %rhsc.i.i, 120
  %_5.i = add i64 %565, -1
  %spec.select1845 = select i1 %567, i64 %_5.i, i64 undef
  %spec.select1846 = select i1 %567, ptr %564, ptr null
  br label %bb337

bb340:                                            ; preds = %bb335
  switch i64 %cpu.sroa.10.0.copyload, label %bb344 [
    i64 6, label %bb474
    i64 7, label %bb2.i842
  ]

bb337:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i, %bb336
  %_0.sroa.4.0.i = phi i64 [ undef, %bb336 ], [ %spec.select1845, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i ]
  %_0.sroa.0.0.i = phi ptr [ null, %bb336 ], [ %spec.select1846, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build.exit.i ]
  %.not101 = icmp eq ptr %_0.sroa.0.0.i, null
  %spec.select = select i1 %.not101, ptr %564, ptr %_0.sroa.0.0.i
  %spec.select119 = select i1 %.not101, i64 %565, i64 %_0.sroa.4.0.i
; call <u32>::from_ascii_radix
  %568 = call fastcc i64 @_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %spec.select, i64 noundef %spec.select119)
  %569 = trunc i64 %568 to i1
  %570 = icmp ugt i64 %568, 34359738367
  %not.1855 = xor i1 %569, true
  %spec.select120 = and i1 %570, %not.1855
  br label %bb344

bb344:                                            ; preds = %bb474, %bb340, %bb2.i842, %bb337
  %pwr8_features.sroa.0.1.off0 = phi i1 [ %spec.select120, %bb337 ], [ %575, %bb2.i842 ], [ %573, %bb474 ], [ false, %bb340 ]
  %571 = icmp eq i64 %560, 0
  br i1 %571, label %bb388, label %bb2.i.i.i4.i.i837

bb2.i.i.i4.i.i837:                                ; preds = %bb344
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %cpu.sroa.5.0.copyload, i64 noundef %560, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb388

bb474:                                            ; preds = %bb340
  %572 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %cpu.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_599e99bb598e572a5316e19a07ba6538, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !952
  %573 = icmp eq i32 %572, 0
  br label %bb344

bb2.i842:                                         ; preds = %bb340
  %574 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %cpu.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_34414b8d2a3366cb3bf8126f7ed8d762, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !956
  %575 = icmp eq i32 %574, 0
  br label %bb344

bb388:                                            ; preds = %bb344, %bb2.i.i.i4.i.i837, %bb349
  %pwr8_features.sroa.0.2.off0 = phi i1 [ %_0.sroa.0.0.off0.i846, %bb349 ], [ %pwr8_features.sroa.0.1.off0, %bb2.i.i.i4.i.i837 ], [ %pwr8_features.sroa.0.1.off0, %bb344 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_331)
  br label %bb234.invoke

bb347:                                            ; preds = %bb345
; invoke <core::result::Result<alloc::string::String, std::env::VarError>>::expect
  invoke fastcc void @_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_345, ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %_346, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_48b8864556065fe7b21fa91d6bd43bb1, i64 noundef 31, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_766b25183a26c8232979e8e97ad1aa67)
          to label %bb348 unwind label %cleanup7

bb348:                                            ; preds = %bb347
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_346)
  %576 = getelementptr inbounds nuw i8, ptr %_345, i64 8
  %_1011 = load ptr, ptr %576, align 8, !nonnull !3, !noundef !3
  %577 = getelementptr inbounds nuw i8, ptr %_345, i64 16
  %_1010 = load i64, ptr %577, align 8, !noundef !3
  %_3.not.i844 = icmp eq i64 %_1010, 6
  br i1 %_3.not.i844, label %bb2.i847, label %bb476

bb2.i847:                                         ; preds = %bb348
  %578 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_1011, ptr noundef nonnull dereferenceable(6) @alloc_4ab12fe6a700d822e8674db3fe2ff0c9, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !960
  %579 = icmp eq i32 %578, 0
  br label %bb476

bb476:                                            ; preds = %bb2.i847, %bb348
  %_0.sroa.0.0.off0.i846 = phi i1 [ %579, %bb2.i847 ], [ false, %bb348 ]
  %_345.val140 = load i64, ptr %_345, align 8
  %580 = icmp eq i64 %_345.val140, 0
  br i1 %580, label %bb349, label %bb2.i.i.i4.i.i851

bb2.i.i.i4.i.i851:                                ; preds = %bb476
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1011, i64 noundef %_345.val140, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb349

bb349:                                            ; preds = %bb2.i.i.i4.i.i851, %bb476
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_345)
  br label %bb388

bb316:                                            ; preds = %bb219, %bb3.i755, %bb4.i757
; invoke build_script_build::target_feature_fallback
  %_322 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5a97a820ee50c370898f0445e27bf764, i64 noundef 5, i1 noundef zeroext false)
          to label %bb318 unwind label %cleanup7

bb318:                                            ; preds = %bb316
  br i1 %version.sroa.128.01264.off02085, label %bb1.i858, label %bb322

bb1.i858:                                         ; preds = %bb318
  %_7.i859 = icmp ugt i32 %version.sroa.0.0127714051432, 83
  br i1 %_7.i859, label %bb319, label %bb3.i860

bb3.i860:                                         ; preds = %bb1.i858
  %_9.i861 = icmp eq i32 %version.sroa.0.0127714051432, 83
  br i1 %_9.i861, label %bb4.i862, label %bb322

bb4.i862:                                         ; preds = %bb3.i860
  %581 = icmp eq i16 %version.sroa.36.012721970, 2024
  %582 = icmp ugt i16 %version.sroa.36.012721970, 2023
  br i1 %581, label %bb11.i864, label %bb319

bb11.i864:                                        ; preds = %bb4.i862
  %583 = icmp eq i8 %version.sroa.64.012701997, 10
  %584 = icmp ugt i8 %version.sroa.64.012701997, 9
  %585 = icmp ne i8 %version.sroa.92.012682055, 0
  %spec.select1847 = select i1 %583, i1 %585, i1 %584
  br label %bb319

bb319:                                            ; preds = %bb1.i753, %bb4.i757, %bb11.i864, %bb4.i862, %bb1.i858
  %zaamo.sroa.0.0.off01549 = phi i1 [ %_322, %bb1.i858 ], [ %_322, %bb4.i862 ], [ %_322, %bb11.i864 ], [ false, %bb4.i757 ], [ false, %bb1.i753 ]
  %_0.sroa.0.0.shrunk.i857 = phi i1 [ true, %bb1.i858 ], [ %582, %bb4.i862 ], [ %spec.select1847, %bb11.i864 ], [ true, %bb4.i757 ], [ true, %bb1.i753 ]
  %or.cond29 = and i1 %version.sroa.128.01264.off02085, %_0.sroa.0.0.shrunk.i857
  br i1 %or.cond29, label %bb372, label %bb322

bb322:                                            ; preds = %bb318, %bb3.i860, %bb319
  %zaamo.sroa.0.0.off015491556 = phi i1 [ %zaamo.sroa.0.0.off01549, %bb319 ], [ %_322, %bb3.i860 ], [ %_322, %bb318 ]
  %_325 = icmp ugt i32 %version.sroa.120.012662058, 18
  br i1 %_325, label %bb323, label %bb234.invoke

bb323:                                            ; preds = %bb322
; invoke build_script_build::target_feature_fallback
  %_327 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1f93ada5207dde6875e7ab77a5bb8dbe, i64 noundef 5, i1 noundef zeroext false)
          to label %bb324 unwind label %cleanup7

bb324:                                            ; preds = %bb323
  %586 = or i1 %zaamo.sroa.0.0.off015491556, %_327
  br label %bb234.invoke

bb1.i874:                                         ; preds = %bb212
  %_7.i875 = icmp ugt i32 %version.sroa.0.0127714051432, 67
  br i1 %_7.i875, label %bb372, label %bb3.i876

bb3.i876:                                         ; preds = %bb1.i874
  %_9.i877 = icmp eq i32 %version.sroa.0.0127714051432, 67
  br i1 %_9.i877, label %bb4.i878, label %bb229.invoke

bb4.i878:                                         ; preds = %bb3.i876
  %587 = icmp eq i16 %version.sroa.36.012721970, 2022
  %588 = icmp ugt i16 %version.sroa.36.012721970, 2021
  br i1 %587, label %bb11.i880, label %bb260

bb11.i880:                                        ; preds = %bb4.i878
  %589 = icmp eq i8 %version.sroa.64.012701997, 11
  %590 = icmp ugt i8 %version.sroa.64.012701997, 10
  br i1 %589, label %bb12.i882, label %bb260

bb12.i882:                                        ; preds = %bb11.i880
  %591 = icmp ugt i8 %version.sroa.92.012682055, 4
  br i1 %591, label %bb372, label %bb229.invoke

bb260:                                            ; preds = %bb11.i880, %bb4.i878
  %_0.sroa.0.0.shrunk.i873 = phi i1 [ %590, %bb11.i880 ], [ %588, %bb4.i878 ]
  br i1 %_0.sroa.0.0.shrunk.i873, label %bb372, label %bb229.invoke

bb423:                                            ; preds = %bb212
  call void @llvm.experimental.noalias.scope.decl(metadata !964)
  %_4.not.i.i = icmp samesign ult i64 %_9.sroa.8.0.copyload, 3
  br i1 %_4.not.i.i, label %bb2.i895, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb423
  %592 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) @alloc_d9036dbef1cc78d0c3562113c2babf56, ptr noundef nonnull readonly align 1 dereferenceable(3) %_9.sroa.5.0.copyload, i64 range(i64 2, 16) 3), !alias.scope !967
  %593 = icmp eq i32 %592, 0
  br i1 %593, label %bb1.i888, label %bb2.i895

bb1.i888:                                         ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i
  %_8.not.i.i.not = icmp eq i64 %_9.sroa.8.0.copyload, 3
  br i1 %_8.not.i.i.not, label %bb265, label %bb9.i.i889

bb9.i.i889:                                       ; preds = %bb1.i888
  %594 = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 3
  %self1.i.i = load i8, ptr %594, align 1, !alias.scope !975, !noalias !964, !noundef !3
  %595 = icmp sgt i8 %self1.i.i, -65
  br i1 %595, label %bb265, label %bb7.i916.invoke

bb265:                                            ; preds = %bb1.i888, %bb9.i.i889
  %new_len.i.i = add i64 %_9.sroa.8.0.copyload, -3
  %data.i.i = getelementptr inbounds nuw i8, ptr %_9.sroa.5.0.copyload, i64 3
  br label %bb425

bb2.i895:                                         ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i, %bb423
  %_7.val1562 = load ptr, ptr %_7, align 8, !nonnull !3, !noundef !3
  %_7.val1621563 = load i64, ptr %6, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !978)
  %_4.not.i.i.i.i896 = icmp samesign ult i64 %_7.val1621563, 5
  br i1 %_4.not.i.i.i.i896, label %bb434.thread.invoke, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i: ; preds = %bb2.i895
  %596 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) @alloc_0c3cbb61e273e9decf597461734e2fad, ptr noundef nonnull readonly align 1 dereferenceable(5) %_7.val1562, i64 range(i64 2, 16) 5), !alias.scope !981, !noalias !989
  %597 = icmp eq i32 %596, 0
  br i1 %597, label %bb1.i.i.i897, label %bb434.thread.invoke

bb1.i.i.i897:                                     ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i
  %_8.not.i.i.not.i.i = icmp eq i64 %_7.val1621563, 5
  br i1 %_8.not.i.i.not.i.i, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb9.i.i.i.i898

bb9.i.i.i.i898:                                   ; preds = %bb1.i.i.i897
  %598 = getelementptr inbounds nuw i8, ptr %_7.val1562, i64 5
  %self1.i.i.i.i899 = load i8, ptr %598, align 1, !alias.scope !992, !noalias !995, !noundef !3
  %599 = icmp sgt i8 %self1.i.i.i.i899, -65
  br i1 %599, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb7.i916.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i: ; preds = %bb9.i.i.i.i898, %bb1.i.i.i897
  %new_len.i.i.i.i901 = add i64 %_7.val1621563, -5
  %data.i.i.i.i902 = getelementptr inbounds nuw i8, ptr %_7.val1562, i64 5
  br label %bb425

bb425:                                            ; preds = %bb265, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i
  %self.0.pn.i = phi ptr [ %data.i.i, %bb265 ], [ %data.i.i.i.i902, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i ]
  %self.1.pn.i = phi i64 [ %new_len.i.i, %bb265 ], [ %new_len.i.i.i.i901, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !996)
  %_4.not.i.i904 = icmp samesign ult i64 %self.1.pn.i, 2
  br i1 %_4.not.i.i904, label %bb2.i921, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905: ; preds = %bb425
  %600 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_5d5c3f5b03c6d3586ba34aa6bd6df864, ptr noundef nonnull readonly align 1 dereferenceable(2) %self.0.pn.i, i64 range(i64 2, 16) 2), !alias.scope !999
  %601 = icmp eq i32 %600, 0
  br i1 %601, label %bb1.i909, label %bb2.i921

bb1.i909:                                         ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905
  %_8.not.i.i910.not = icmp eq i64 %self.1.pn.i, 2
  br i1 %_8.not.i.i910.not, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911, label %bb9.i.i914

bb9.i.i914:                                       ; preds = %bb1.i909
  %602 = getelementptr inbounds nuw i8, ptr %self.0.pn.i, i64 2
  %self1.i.i915 = load i8, ptr %602, align 1, !alias.scope !1007, !noalias !996, !noundef !3
  %603 = icmp sgt i8 %self1.i.i915, -65
  br i1 %603, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911, label %bb7.i916.invoke

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911: ; preds = %bb9.i.i914, %bb1.i909
  %new_len.i.i912 = add i64 %self.1.pn.i, -2
  %data.i.i913 = getelementptr inbounds nuw i8, ptr %self.0.pn.i, i64 2
  br label %bb2.i921

bb7.i916.invoke:                                  ; preds = %bb9.i.i914, %bb9.i.i.i.i898, %bb9.i.i889
  %604 = phi ptr [ %_9.sroa.5.0.copyload, %bb9.i.i889 ], [ %_7.val1562, %bb9.i.i.i.i898 ], [ %self.0.pn.i, %bb9.i.i914 ]
  %605 = phi i64 [ %_9.sroa.8.0.copyload, %bb9.i.i889 ], [ %_7.val1621563, %bb9.i.i.i.i898 ], [ %self.1.pn.i, %bb9.i.i914 ]
  %606 = phi i64 [ 3, %bb9.i.i889 ], [ 5, %bb9.i.i.i.i898 ], [ 2, %bb9.i.i914 ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %604, i64 noundef %605, i64 noundef %606, i64 noundef %605, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b9a6efcfeb17b646c43fe2783a9632a7) #29
          to label %bb7.i916.cont unwind label %cleanup7

bb7.i916.cont:                                    ; preds = %bb7.i916.invoke
  unreachable

bb2.i921:                                         ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905, %bb425
  %_0.sroa.4.0.i907 = phi i64 [ %new_len.i.i912, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911 ], [ undef, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905 ], [ undef, %bb425 ]
  %_0.sroa.0.0.i908 = phi ptr [ %data.i.i913, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i911 ], [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i905 ], [ null, %bb425 ]
  %.not109 = icmp eq ptr %_0.sroa.0.0.i908, null
  %spec.select121 = select i1 %.not109, i64 %self.1.pn.i, i64 %_0.sroa.4.0.i907
  %spec.select122 = select i1 %.not109, ptr %self.0.pn.i, ptr %_0.sroa.0.0.i908
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_247)
  store i64 0, ptr %_247, align 8
  %_891.sroa.4.0._247.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 8
  store i64 %spec.select121, ptr %_891.sroa.4.0._247.sroa_idx, align 8
  %_891.sroa.5.0._247.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 16
  store ptr %spec.select122, ptr %_891.sroa.5.0._247.sroa_idx, align 8
  %_891.sroa.5.sroa.4.0._891.sroa.5.0._247.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 24
  store i64 %spec.select121, ptr %_891.sroa.5.sroa.4.0._891.sroa.5.0._247.sroa_idx.sroa_idx, align 8
  %_891.sroa.5.sroa.5.0._891.sroa.5.0._247.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 32
  store i64 0, ptr %_891.sroa.5.sroa.5.0._891.sroa.5.0._247.sroa_idx.sroa_idx, align 8
  %_891.sroa.5.sroa.6.0._891.sroa.5.0._247.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 40
  store i64 %spec.select121, ptr %_891.sroa.5.sroa.6.0._891.sroa.5.0._247.sroa_idx.sroa_idx, align 8
  %_891.sroa.5.sroa.7.0._891.sroa.5.0._247.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 48
  store <2 x i32> splat (i32 45), ptr %_891.sroa.5.sroa.7.0._891.sroa.5.0._247.sroa_idx.sroa_idx, align 8
  %_891.sroa.5.sroa.9.0._891.sroa.5.0._247.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 56
  store i8 1, ptr %_891.sroa.5.sroa.9.0._891.sroa.5.0._247.sroa_idx.sroa_idx, align 8
  %_891.sroa.6.0._247.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 64
  store i8 1, ptr %_891.sroa.6.0._247.sroa_idx, align 8
  %_891.sroa.7.0._247.sroa_idx = getelementptr inbounds nuw i8, ptr %_247, i64 65
  store i8 0, ptr %_891.sroa.7.0._247.sroa_idx, align 1
  call void @llvm.experimental.noalias.scope.decl(metadata !1010)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i919), !noalias !1010
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i919, ptr noalias noundef align 8 dereferenceable(48) %_891.sroa.5.0._247.sroa_idx) #28
          to label %.noexc936 unwind label %cleanup7

.noexc936:                                        ; preds = %bb2.i921
  %_7.i922 = load i64, ptr %_5.i919, align 8, !range !37, !noalias !1010, !noundef !3
  %607 = trunc nuw i64 %_7.i922 to i1
  br i1 %607, label %bb7.i934, label %bb6.i923

bb7.i934:                                         ; preds = %.noexc936
  %608 = getelementptr inbounds nuw i8, ptr %_5.i919, i64 8
  %a.i = load i64, ptr %608, align 8, !noalias !1010, !noundef !3
  %i.i = load i64, ptr %_247, align 8, !alias.scope !1010, !noundef !3
  %new_len.i = sub nuw i64 %a.i, %i.i
  %data.i935 = getelementptr inbounds nuw i8, ptr %spec.select122, i64 %i.i
  br label %bb2.i939

bb6.i923:                                         ; preds = %.noexc936
  %609 = load i8, ptr %_891.sroa.7.0._247.sroa_idx, align 1, !range !54, !alias.scope !1013, !noundef !3
  %_2.i.i924 = trunc nuw i8 %609 to i1
  br i1 %_2.i.i924, label %bb431, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb6.i923
  %610 = load i8, ptr %_891.sroa.6.0._247.sroa_idx, align 8, !range !54, !alias.scope !1013, !noundef !3
  %_3.i.i = trunc nuw i8 %610 to i1
  %i.pre.i.i = load i64, ptr %_247, align 8, !alias.scope !1013
  %i1.pre.i.i = load i64, ptr %_891.sroa.4.0._247.sroa_idx, align 8, !alias.scope !1013
  %_4.not.i.i925 = icmp ne i64 %i1.pre.i.i, %i.pre.i.i
  %or.cond.not.i.i926 = select i1 %_3.i.i, i1 true, i1 %_4.not.i.i925
  br i1 %or.cond.not.i.i926, label %bb4.i.i931, label %bb431

bb4.i.i931:                                       ; preds = %bb1.i.i
  %_10.val.i.i = load ptr, ptr %_891.sroa.5.0._247.sroa_idx, align 8, !alias.scope !1013, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i932 = sub nuw i64 %i1.pre.i.i, %i.pre.i.i
  %data.i.i933 = getelementptr inbounds nuw i8, ptr %_10.val.i.i, i64 %i.pre.i.i
  br label %bb2.i939

bb431:                                            ; preds = %bb6.i923, %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i919), !noalias !1010
  br label %bb434.thread.invoke

bb2.i939:                                         ; preds = %bb7.i934, %bb4.i.i931
  %_0.sroa.4.0.i928 = phi i64 [ %new_len.i, %bb7.i934 ], [ %new_len.i.i932, %bb4.i.i931 ]
  %_0.sroa.0.0.i929 = phi ptr [ %data.i935, %bb7.i934 ], [ %data.i.i933, %bb4.i.i931 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i919), !noalias !1010
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_247)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_250)
  store i64 0, ptr %_250, align 8
  %_898.sroa.4.0._250.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 8
  store i64 %_0.sroa.4.0.i928, ptr %_898.sroa.4.0._250.sroa_idx, align 8
  %_898.sroa.5.0._250.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 16
  store ptr %_0.sroa.0.0.i929, ptr %_898.sroa.5.0._250.sroa_idx, align 8
  %_898.sroa.5.sroa.4.0._898.sroa.5.0._250.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 24
  store i64 %_0.sroa.4.0.i928, ptr %_898.sroa.5.sroa.4.0._898.sroa.5.0._250.sroa_idx.sroa_idx, align 8
  %_898.sroa.5.sroa.5.0._898.sroa.5.0._250.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 32
  store i64 0, ptr %_898.sroa.5.sroa.5.0._898.sroa.5.0._250.sroa_idx.sroa_idx, align 8
  %_898.sroa.5.sroa.6.0._898.sroa.5.0._250.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 40
  store i64 %_0.sroa.4.0.i928, ptr %_898.sroa.5.sroa.6.0._898.sroa.5.0._250.sroa_idx.sroa_idx, align 8
  %_898.sroa.5.sroa.7.0._898.sroa.5.0._250.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 48
  store <2 x i32> splat (i32 46), ptr %_898.sroa.5.sroa.7.0._898.sroa.5.0._250.sroa_idx.sroa_idx, align 8
  %_898.sroa.5.sroa.9.0._898.sroa.5.0._250.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 56
  store i8 1, ptr %_898.sroa.5.sroa.9.0._898.sroa.5.0._250.sroa_idx.sroa_idx, align 8
  %_898.sroa.6.0._250.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 64
  store i8 1, ptr %_898.sroa.6.0._250.sroa_idx, align 8
  %_898.sroa.7.0._250.sroa_idx = getelementptr inbounds nuw i8, ptr %_250, i64 65
  store i8 0, ptr %_898.sroa.7.0._250.sroa_idx, align 1
  call void @llvm.experimental.noalias.scope.decl(metadata !1016)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i937), !noalias !1016
; invoke <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  invoke fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5.i937, ptr noalias noundef align 8 dereferenceable(48) %_898.sroa.5.0._250.sroa_idx) #28
          to label %.noexc968 unwind label %cleanup7

.noexc968:                                        ; preds = %bb2.i939
  %_7.i942 = load i64, ptr %_5.i937, align 8, !range !37, !noalias !1016, !noundef !3
  %611 = trunc nuw i64 %_7.i942 to i1
  br i1 %611, label %bb434, label %bb6.i943

bb6.i943:                                         ; preds = %.noexc968
  %612 = load i8, ptr %_898.sroa.7.0._250.sroa_idx, align 1, !range !54, !alias.scope !1019, !noundef !3
  %_2.i.i944 = trunc nuw i8 %612 to i1
  br i1 %_2.i.i944, label %bb434.thread, label %bb1.i.i945

bb1.i.i945:                                       ; preds = %bb6.i943
  store i8 1, ptr %_898.sroa.7.0._250.sroa_idx, align 1, !alias.scope !1019
  %613 = load i8, ptr %_898.sroa.6.0._250.sroa_idx, align 8, !range !54, !alias.scope !1019, !noundef !3
  %_3.i.i946 = trunc nuw i8 %613 to i1
  %i.pre.i.i947 = load i64, ptr %_250, align 8, !alias.scope !1019
  %i1.pre.i.i949 = load i64, ptr %_898.sroa.4.0._250.sroa_idx, align 8, !alias.scope !1019
  %_4.not.i.i950 = icmp ne i64 %i1.pre.i.i949, %i.pre.i.i947
  %or.cond.not.i.i951 = select i1 %_3.i.i946, i1 true, i1 %_4.not.i.i950
  br i1 %or.cond.not.i.i951, label %bb434.thread1578, label %bb434.thread

bb434.thread1578:                                 ; preds = %bb1.i.i945
  %_10.val.i.i959 = load ptr, ptr %_898.sroa.5.0._250.sroa_idx, align 8, !alias.scope !1019, !nonnull !3, !align !18, !noundef !3
  %new_len.i.i960 = sub nuw i64 %i1.pre.i.i949, %i.pre.i.i947
  %data.i.i961 = getelementptr inbounds nuw i8, ptr %_10.val.i.i959, i64 %i.pre.i.i947
  br label %bb436

bb434.thread:                                     ; preds = %bb6.i943, %bb1.i.i945
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i937), !noalias !1016
  br label %bb434.thread.invoke

bb434.thread.invoke:                              ; preds = %bb2.i895, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i, %bb431, %bb434.thread
  %614 = phi ptr [ @alloc_b0cad33fb230cb36b707608c08d32517, %bb434.thread ], [ @alloc_ee08f4448ec37f2c785795512d1d6371, %bb431 ], [ @alloc_9808848e4b0f0dfce430f2ce907c2ec4, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build.exit.i.i.i ], [ @alloc_9808848e4b0f0dfce430f2ce907c2ec4, %bb2.i895 ]
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %614) #26
          to label %bb434.thread.cont unwind label %cleanup7

bb434.thread.cont:                                ; preds = %bb434.thread.invoke
  unreachable

bb434:                                            ; preds = %.noexc968
  %615 = getelementptr inbounds nuw i8, ptr %_5.i937, i64 8
  %a.i963 = load i64, ptr %615, align 8, !noalias !1016, !noundef !3
  %i.i965 = load i64, ptr %_250, align 8, !alias.scope !1016, !noundef !3
  %new_len.i966 = sub nuw i64 %a.i963, %i.i965
  %data.i967 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i929, i64 %i.i965
  br label %bb436

bb436:                                            ; preds = %bb434, %bb434.thread1578
  %_0.sroa.0.0.i9541583 = phi ptr [ %data.i.i961, %bb434.thread1578 ], [ %data.i967, %bb434 ]
  %_0.sroa.4.0.i9531582 = phi i64 [ %new_len.i.i960, %bb434.thread1578 ], [ %new_len.i966, %bb434 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i937), !noalias !1016
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_250)
  %_3.not.i970 = icmp eq i64 %_0.sroa.4.0.i9531582, 2
  br i1 %_3.not.i970, label %bb437, label %bb269

bb437:                                            ; preds = %bb436
  %616 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(2) @alloc_b655ffc265eabd5b7c4618f72368ce58, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1022
  %617 = icmp eq i32 %616, 0
  br i1 %617, label %bb460, label %bb443

bb269:                                            ; preds = %bb436
  switch i64 %_0.sroa.4.0.i9531582, label %bb268 [
    i64 3, label %bb439
    i64 6, label %bb440
    i64 4, label %bb451
  ]

bb484:                                            ; preds = %bb457, %bb455, %bb454
  %_4.not.i = icmp eq i64 %_0.sroa.4.0.i9531582, 1
  br i1 %_4.not.i, label %bb309, label %bb460

bb439:                                            ; preds = %bb269
  %618 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_876e28ca4ef18e92a168b8b6c0258020, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1026
  %619 = icmp eq i32 %618, 0
  br i1 %619, label %bb460, label %bb441

bb440:                                            ; preds = %bb269
  %620 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(6) @alloc_3871c7a1d848c9c811d08e9ba480e39d, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !1030
  %621 = icmp eq i32 %620, 0
  br i1 %621, label %bb460, label %bb268

bb441:                                            ; preds = %bb439
  %622 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_8a148dfbf624d86ad479275878ea55de, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1034
  %623 = icmp eq i32 %622, 0
  br i1 %623, label %bb460, label %bb442

bb442:                                            ; preds = %bb441
  %624 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_620d80f9f3d64bd5a4c2be88c9b298d5, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1038
  %625 = icmp eq i32 %624, 0
  br i1 %625, label %bb460, label %bb444

bb443:                                            ; preds = %bb437
  %626 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(2) @alloc_98dd345a480634a67ab490269b3092e8, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1042
  %627 = icmp eq i32 %626, 0
  br i1 %627, label %bb460, label %bb445

bb444:                                            ; preds = %bb442
  %628 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_8bc0c6258d5b796bc8931ca41d054010, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1046
  %629 = icmp eq i32 %628, 0
  br i1 %629, label %bb460, label %bb446

bb445:                                            ; preds = %bb443
  %630 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(2) @alloc_766309e861a41c77d8e1ec8d2bf7d3d1, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1050
  %631 = icmp eq i32 %630, 0
  br i1 %631, label %bb460, label %bb268

bb446:                                            ; preds = %bb444
  %632 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_731bb66c58a2091aa6837f3ec33ccbd0, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1054
  %633 = icmp eq i32 %632, 0
  br i1 %633, label %bb460, label %bb447

bb447:                                            ; preds = %bb446
  %634 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_46d8ec96465309b380b25451978e5b89, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1058
  %635 = icmp eq i32 %634, 0
  br i1 %635, label %bb460, label %bb448

bb448:                                            ; preds = %bb447
  %636 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_8e5d4739a815f492da01ca41d793f679, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1062
  %637 = icmp eq i32 %636, 0
  br i1 %637, label %bb460, label %bb449

bb449:                                            ; preds = %bb448
  %638 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_896376205399067eda0bb4872a2b2e1a, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1066
  %639 = icmp eq i32 %638, 0
  br i1 %639, label %bb460, label %bb450

bb450:                                            ; preds = %bb449
  %640 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_39d6b4d2e142be3718c9abbba15e676b, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1070
  %641 = icmp eq i32 %640, 0
  br i1 %641, label %bb460, label %bb452

bb451:                                            ; preds = %bb269
  %642 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(4) @alloc_6f0318d77bf4620e64f2b718a0fed714, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !1074
  %643 = icmp eq i32 %642, 0
  br i1 %643, label %bb460, label %bb268

bb452:                                            ; preds = %bb450
  %644 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_6fe9401313774126d244c9df8f281e77, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1078
  %645 = icmp eq i32 %644, 0
  br i1 %645, label %bb460, label %bb453

bb453:                                            ; preds = %bb452
  %646 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(3) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(3) @alloc_094ba35790031d8096d74b82ea129258, i64 range(i64 0, -9223372036854775808) 3), !alias.scope !1082
  %647 = icmp eq i32 %646, 0
  br i1 %647, label %bb460, label %bb268

bb268:                                            ; preds = %bb451, %bb269, %bb445, %bb440, %bb453
  %_3.not.i975158615991602160816101618162016241626163016321638164016441646165216541658166016641666167016721676167916891692170117041712 = phi i1 [ true, %bb453 ], [ false, %bb440 ], [ false, %bb445 ], [ false, %bb269 ], [ false, %bb451 ]
  %_3.not.i1038168016881693170017061711 = phi i1 [ false, %bb453 ], [ false, %bb440 ], [ false, %bb445 ], [ false, %bb269 ], [ true, %bb451 ]
  switch i64 %_9.sroa.8.0.copyload, label %bb294 [
    i64 21, label %bb438
    i64 27, label %bb458
  ]

bb438:                                            ; preds = %bb268
  %648 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(21) %_9.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(21) @alloc_7ef794223a3c79a90832fd404d7462c3, i64 range(i64 0, -9223372036854775808) 21), !alias.scope !1086
  %649 = icmp eq i32 %648, 0
  br i1 %649, label %bb460, label %bb294

bb458:                                            ; preds = %bb268
  %650 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(27) %_9.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(27) @alloc_efe094782a10f2611a22702571dbac01, i64 range(i64 0, -9223372036854775808) 27), !alias.scope !1090
  %651 = icmp eq i32 %650, 0
  br i1 %651, label %bb460, label %bb294

bb294:                                            ; preds = %bb268, %bb438, %bb458
  %_3.not.i1063 = icmp eq i64 %_0.sroa.4.0.i9531582, 0
  br i1 %_3.not.i1063, label %bb460, label %bb286

bb286:                                            ; preds = %bb294
  br i1 %_3.not.i975158615991602160816101618162016241626163016321638164016441646165216541658166016641666167016721676167916891692170117041712, label %bb454, label %bb287

bb454:                                            ; preds = %bb286
  %652 = call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.0.i9541583, ptr nonnull @alloc_e01da671082aa58f6e6c733a052567ea, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.0.i9531582), !alias.scope !1094
  %653 = icmp eq i32 %652, 0
  br i1 %653, label %bb484, label %bb287

bb287:                                            ; preds = %bb286, %bb454
  br i1 %_3.not.i1038168016881693170017061711, label %bb455, label %bb288

bb455:                                            ; preds = %bb287
  %654 = call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.0.i9541583, ptr nonnull @alloc_cc3bb8f2b93f3b6ad6e65fe09616c7dd, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.0.i9531582), !alias.scope !1098
  %655 = icmp eq i32 %654, 0
  br i1 %655, label %bb484, label %bb288

bb288:                                            ; preds = %bb287, %bb455
  br i1 %_3.not.i970, label %bb456, label %bb289

bb456:                                            ; preds = %bb288
  %656 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_0.sroa.0.0.i9541583, ptr noundef nonnull dereferenceable(2) @alloc_6785c718eb5d42ff6741fe7de88e57ea, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1102
  %657 = icmp eq i32 %656, 0
  br i1 %657, label %bb460, label %bb289

bb289:                                            ; preds = %bb288, %bb456
  br i1 %_3.not.i975158615991602160816101618162016241626163016321638164016441646165216541658166016641666167016721676167916891692170117041712, label %bb457, label %bb284

bb457:                                            ; preds = %bb289
  %658 = call i32 @memcmp(ptr nonnull readonly align 1 %_0.sroa.0.0.i9541583, ptr nonnull @alloc_c8a1a35b0978064484413724586714e9, i64 range(i64 0, -9223372036854775808) %_0.sroa.4.0.i9531582), !alias.scope !1106
  %659 = icmp eq i32 %658, 0
  br i1 %659, label %bb484, label %bb284

bb284:                                            ; preds = %bb289, %bb457
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_297)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_297, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0afe6f0504dd6bb7e0912d0ebba4969c, i64 noundef 29)
          to label %bb295 unwind label %cleanup7

bb295:                                            ; preds = %bb284
  %660 = load i64, ptr %_297, align 8, !range !11, !noundef !3
  switch i64 %660, label %bb2.i.i.i4.i.i.i.i1091 [
    i64 -9223372036854775808, label %bb299
    i64 0, label %bb297
  ]

bb2.i.i.i4.i.i.i.i1091:                           ; preds = %bb295
  %661 = getelementptr inbounds nuw i8, ptr %_297, i64 8
  %_297.val158 = load ptr, ptr %661, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_297.val158, i64 noundef %660, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb297

bb297:                                            ; preds = %bb295, %bb2.i.i.i4.i.i.i.i1091
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_297)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args3)
  store ptr %_7, ptr %args3, align 8
  %_302.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args3, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCslwKqnJYeWCA_18build_script_build, ptr %_302.sroa.4.0..sroa_idx, align 8
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_fbcccdd8daac3e36c025f53437f4178b, ptr noundef nonnull %args3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_83de034d7ccdc84ce78692abaea10898) #26
          to label %unreachable unwind label %cleanup7

bb299:                                            ; preds = %bb295
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_297)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args4)
  store ptr %_7, ptr %args4, align 8
  %_308.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args4, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCslwKqnJYeWCA_18build_script_build, ptr %_308.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_cf6e7c65e8353dc4e21daa402c81c63c, ptr noundef nonnull %args4)
          to label %bb300 unwind label %cleanup7

bb300:                                            ; preds = %bb299
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args4)
  br label %bb309

bb309:                                            ; preds = %bb484, %bb4.i1107, %bb460, %bb461, %bb462, %bb300
  %v6.sroa.0.0.off0 = phi i1 [ false, %bb300 ], [ true, %bb462 ], [ true, %bb461 ], [ true, %bb460 ], [ %669, %bb4.i1107 ], [ false, %bb484 ]
  %mclass.sroa.0.1.off0 = phi i1 [ false, %bb300 ], [ %mclass.sroa.0.0.off01591, %bb462 ], [ %mclass.sroa.0.0.off01591, %bb461 ], [ %mclass.sroa.0.0.off01591, %bb460 ], [ %mclass.sroa.0.0.off01591, %bb4.i1107 ], [ false, %bb484 ]
; invoke build_script_build::target_feature_fallback
  %_317 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6785c718eb5d42ff6741fe7de88e57ea, i64 noundef 2, i1 noundef zeroext %v6.sroa.0.0.off0)
          to label %bb234.invoke unwind label %cleanup7

bb460:                                            ; preds = %bb294, %bb450, %bb451, %bb452, %bb453, %bb456, %bb449, %bb448, %bb447, %bb446, %bb445, %bb444, %bb443, %bb442, %bb441, %bb440, %bb439, %bb458, %bb438, %bb437, %bb484
  %subarch.sroa.0.11593 = phi ptr [ %_0.sroa.0.0.i9541583, %bb484 ], [ @alloc_98dd345a480634a67ab490269b3092e8, %bb458 ], [ @alloc_cc3bb8f2b93f3b6ad6e65fe09616c7dd, %bb438 ], [ %_0.sroa.0.0.i9541583, %bb437 ], [ %_0.sroa.0.0.i9541583, %bb439 ], [ %_0.sroa.0.0.i9541583, %bb440 ], [ %_0.sroa.0.0.i9541583, %bb441 ], [ %_0.sroa.0.0.i9541583, %bb442 ], [ %_0.sroa.0.0.i9541583, %bb443 ], [ %_0.sroa.0.0.i9541583, %bb444 ], [ %_0.sroa.0.0.i9541583, %bb445 ], [ %_0.sroa.0.0.i9541583, %bb446 ], [ %_0.sroa.0.0.i9541583, %bb447 ], [ %_0.sroa.0.0.i9541583, %bb448 ], [ %_0.sroa.0.0.i9541583, %bb449 ], [ %_0.sroa.0.0.i9541583, %bb456 ], [ %_0.sroa.0.0.i9541583, %bb453 ], [ %_0.sroa.0.0.i9541583, %bb452 ], [ %_0.sroa.0.0.i9541583, %bb451 ], [ %_0.sroa.0.0.i9541583, %bb450 ], [ @alloc_6785c718eb5d42ff6741fe7de88e57ea, %bb294 ]
  %mclass.sroa.0.0.off01591 = phi i1 [ false, %bb484 ], [ false, %bb458 ], [ false, %bb438 ], [ false, %bb437 ], [ false, %bb439 ], [ false, %bb440 ], [ false, %bb441 ], [ false, %bb442 ], [ false, %bb443 ], [ false, %bb444 ], [ false, %bb445 ], [ false, %bb446 ], [ false, %bb447 ], [ false, %bb448 ], [ false, %bb449 ], [ false, %bb456 ], [ true, %bb453 ], [ true, %bb452 ], [ true, %bb451 ], [ true, %bb450 ], [ false, %bb294 ]
  %662 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_6785c718eb5d42ff6741fe7de88e57ea, ptr noundef nonnull readonly align 1 dereferenceable(2) %subarch.sroa.0.11593, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1110
  %663 = icmp eq i32 %662, 0
  br i1 %663, label %bb309, label %bb461

bb461:                                            ; preds = %bb460
  %664 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_b655ffc265eabd5b7c4618f72368ce58, ptr noundef nonnull readonly align 1 dereferenceable(2) %subarch.sroa.0.11593, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1117
  %665 = icmp eq i32 %664, 0
  br i1 %665, label %bb309, label %bb462

bb462:                                            ; preds = %bb461
  %666 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_98dd345a480634a67ab490269b3092e8, ptr noundef nonnull readonly align 1 dereferenceable(2) %subarch.sroa.0.11593, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1124
  %667 = icmp eq i32 %666, 0
  br i1 %667, label %bb309, label %bb4.i1107

bb4.i1107:                                        ; preds = %bb462
  %668 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_766309e861a41c77d8e1ec8d2bf7d3d1, ptr noundef nonnull readonly align 1 dereferenceable(2) %subarch.sroa.0.11593, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !1131
  %669 = icmp eq i32 %668, 0
  br label %bb309

bb240:                                            ; preds = %bb220, %bb3.i724, %bb4.i726
  %_3.not.i1111 = icmp eq i64 %_15.sroa.8.0.copyload, 5
  br i1 %_3.not.i1111, label %bb2.i1114, label %bb416

bb2.i1114:                                        ; preds = %bb240
  %670 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_15.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_8a32b15b0e78e52f3e29895e5f677744, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !1138
  %671 = icmp eq i32 %670, 0
  br label %bb416

bb416:                                            ; preds = %bb2.i1114, %bb240
  %_0.sroa.0.0.off0.i1113 = phi i1 [ %671, %bb2.i1114 ], [ false, %bb240 ]
; invoke build_script_build::target_feature_fallback
  %_223 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ff4976f557f9331cf80413f038ea83af, i64 noundef 4, i1 noundef zeroext %_0.sroa.0.0.off0.i1113)
          to label %bb241 unwind label %cleanup7

bb241:                                            ; preds = %bb416
; invoke build_script_build::target_feature_fallback
  %_224 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_96fd63ed221128cfefca2794b4bdbd59, i64 noundef 6, i1 noundef zeroext false)
          to label %bb242 unwind label %cleanup7

bb242:                                            ; preds = %bb241
  %672 = or i1 %_0.sroa.0.0.off0.i1113, %_224
; invoke build_script_build::target_feature_fallback
  %_225 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d21745838991a4cb2caa18b5ddb8dc43, i64 noundef 5, i1 noundef zeroext false)
          to label %bb243 unwind label %cleanup7

bb243:                                            ; preds = %bb242
  %_858 = icmp ugt i32 %version.sroa.0.0127714051432, 60
  %or.cond33 = select i1 %version.sroa.128.01264.off02085, i1 true, i1 %_858
  br i1 %or.cond33, label %bb246, label %bb420

bb420:                                            ; preds = %bb243
; invoke build_script_build::target_feature_fallback
  %_226 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e3abda62d4f863fc9e691690c373f274, i64 noundef 3, i1 noundef zeroext %672)
          to label %bb246 unwind label %cleanup7

bb246:                                            ; preds = %bb1.i722, %bb4.i726, %bb243, %bb420
; invoke build_script_build::target_feature_fallback
  %_227 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d1ae99d9a9853d638c28f3dedea5a4ac, i64 noundef 4, i1 noundef zeroext false)
          to label %bb247 unwind label %cleanup7

bb247:                                            ; preds = %bb246
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_230)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_230, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_43a58bdcc2572eb4426ba7ac54f3fdc3, i64 noundef 23)
          to label %bb248 unwind label %cleanup7

bb248:                                            ; preds = %bb247
  call void @llvm.experimental.noalias.scope.decl(metadata !1142)
  call void @llvm.experimental.noalias.scope.decl(metadata !1145)
  %_2.i1118 = load i64, ptr %_230, align 8, !range !37, !alias.scope !1145, !noalias !1142, !noundef !3
  %673 = trunc nuw i64 %_2.i1118 to i1
  br i1 %673, label %bb3.i.i1121, label %bb249

bb3.i.i1121:                                      ; preds = %bb248
  call void @llvm.experimental.noalias.scope.decl(metadata !1147)
  %674 = getelementptr inbounds nuw i8, ptr %_230, i64 8
  %675 = load i64, ptr %674, align 8, !range !11, !alias.scope !1150, !noalias !1142, !noundef !3
  switch i64 %675, label %bb1.sink.split.i.i1124 [
    i64 -9223372036854775808, label %bb250.thread
    i64 0, label %bb250.thread
  ]

bb1.sink.split.i.i1124:                           ; preds = %bb3.i.i1121
  %676 = getelementptr inbounds nuw i8, ptr %_230, i64 16
  %_1.val1.i.i.i1125 = load ptr, ptr %676, align 8, !alias.scope !1153, !noalias !1142, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i1125, i64 noundef %675, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !1154
  br label %bb250.thread

bb250.thread:                                     ; preds = %bb1.sink.split.i.i1124, %bb3.i.i1121, %bb3.i.i1121
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_230)
  br label %bb251

bb249:                                            ; preds = %bb248
  %677 = getelementptr inbounds nuw i8, ptr %_230, i64 8
  %_229.sroa.0.0.copyload = load i64, ptr %677, align 8, !alias.scope !1155
  %_229.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_230, i64 16
  %_229.sroa.6.0.copyload = load ptr, ptr %_229.sroa.6.0..sroa_idx, align 8, !alias.scope !1155
  %_229.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_230, i64 24
  %_229.sroa.10.0.copyload = load i64, ptr %_229.sroa.10.0..sroa_idx, align 8, !alias.scope !1155
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_230)
  %_3.not.i1127 = icmp eq i64 %_229.sroa.10.0.copyload, 5
  br i1 %_3.not.i1127, label %bb421, label %bb421.thread1826

bb421:                                            ; preds = %bb249
  %678 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_229.sroa.6.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_f5fa10d2bd50b965d2515db045847aab, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !1156
  %679 = icmp eq i32 %678, 0
  %680 = icmp eq i64 %_229.sroa.0.0.copyload, 0
  br i1 %680, label %bb250, label %bb2.i.i.i4.i.i1134

bb421.thread1826:                                 ; preds = %bb249
  %681 = icmp eq i64 %_229.sroa.0.0.copyload, 0
  br i1 %681, label %bb251, label %bb2.i.i.i4.i.i1134.thread

bb2.i.i.i4.i.i1134.thread:                        ; preds = %bb421.thread1826
  %682 = icmp ne ptr %_229.sroa.6.0.copyload, null
  call void @llvm.assume(i1 %682)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_229.sroa.6.0.copyload, i64 noundef %_229.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb251

bb2.i.i.i4.i.i1134:                               ; preds = %bb421
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_229.sroa.6.0.copyload, i64 noundef %_229.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br i1 %679, label %bb229.invoke, label %bb251

bb250:                                            ; preds = %bb421
  br i1 %679, label %bb229.invoke, label %bb251

bb251:                                            ; preds = %bb421.thread1826, %bb2.i.i.i4.i.i1134.thread, %bb2.i.i.i4.i.i1134, %bb250.thread, %bb250
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_232)
; invoke build_script_build::target_cpu
  invoke fastcc void @_RNvCslwKqnJYeWCA_18build_script_build10target_cpu(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_232)
          to label %bb252 unwind label %cleanup7

bb252:                                            ; preds = %bb251
  call void @llvm.experimental.noalias.scope.decl(metadata !1160)
  %683 = load i64, ptr %_232, align 8, !range !11, !alias.scope !1160, !noundef !3
  %.not.i1136 = icmp eq i64 %683, -9223372036854775808
  br i1 %.not.i1136, label %bb257, label %bb3.i1137

bb3.i1137:                                        ; preds = %bb252
  %_7.sroa.4.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_232, i64 8
  %_7.sroa.4.0.copyload.i = load ptr, ptr %_7.sroa.4.0.self.sroa_idx.i, align 8, !alias.scope !1160, !nonnull !3, !noundef !3
  %_7.sroa.5.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_232, i64 16
  %_7.sroa.5.0.copyload.i = load i64, ptr %_7.sroa.5.0.self.sroa_idx.i, align 8, !alias.scope !1160
  %_4.not.i.i.i = icmp samesign ult i64 %_7.sroa.5.0.copyload.i, 6
  br i1 %_4.not.i.i.i, label %bb4.i.i1139.thread, label %bb4.i.i1139

bb4.i.i1139:                                      ; preds = %bb3.i1137
  %684 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) @alloc_3e61858716336dfbf74d71ab75462972, ptr noundef nonnull readonly align 1 dereferenceable(6) %_7.sroa.4.0.copyload.i, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !1163, !noalias !1170
  %685 = icmp eq i32 %684, 0
  %686 = icmp eq i64 %683, 0
  br i1 %686, label %bb253, label %bb2.i.i.i4.i.i4.i.i

bb4.i.i1139.thread:                               ; preds = %bb3.i1137
  %687 = icmp eq i64 %683, 0
  br i1 %687, label %bb257, label %bb2.i.i.i4.i.i4.i.i.thread

bb2.i.i.i4.i.i4.i.i.thread:                       ; preds = %bb4.i.i1139.thread
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_7.sroa.4.0.copyload.i, i64 noundef %683, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !1170
  br label %bb257

bb2.i.i.i4.i.i4.i.i:                              ; preds = %bb4.i.i1139
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_7.sroa.4.0.copyload.i, i64 noundef %683, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !1170
  br i1 %685, label %bb254, label %bb257

bb253:                                            ; preds = %bb4.i.i1139
  br i1 %685, label %bb254, label %bb257

bb257:                                            ; preds = %bb4.i.i1139.thread, %bb2.i.i.i4.i.i4.i.i.thread, %bb252, %bb2.i.i.i4.i.i4.i.i, %bb253
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_232)
  br label %bb372

bb254:                                            ; preds = %bb2.i.i.i4.i.i4.i.i, %bb253
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_232)
  br label %bb229.invoke

bb221:                                            ; preds = %bb11.i707, %bb4.i705
  %_0.sroa.0.0.shrunk.i700 = phi i1 [ %505, %bb11.i707 ], [ %503, %bb4.i705 ]
  br i1 %_0.sroa.0.0.shrunk.i700, label %bb229.invoke, label %bb223

bb223:                                            ; preds = %bb3.i703, %bb12.i709, %bb8.i698, %bb221
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_ee297b014715a704a78587d10e2d5209, ptr noundef nonnull inttoptr (i64 123 to ptr))
          to label %bb225 unwind label %cleanup7

bb225:                                            ; preds = %bb223
  br i1 %version.sroa.128.01264.off02085, label %bb229.invoke, label %bb231

bb231:                                            ; preds = %bb225
  %_842 = icmp samesign ugt i32 %version.sroa.0.0127714051432, 68
  br i1 %_842, label %bb372, label %bb412

bb1.i1147:                                        ; preds = %bb1.i701
  %_7.i1148.not = icmp eq i32 %version.sroa.0.0127714051432, 70
  br i1 %_7.i1148.not, label %bb4.i1151, label %bb372

bb4.i1151:                                        ; preds = %bb1.i1147
  %688 = icmp eq i16 %version.sroa.36.012721970, 2023
  %689 = icmp ugt i16 %version.sroa.36.012721970, 2022
  br i1 %688, label %bb11.i1153, label %bb227

bb11.i1153:                                       ; preds = %bb4.i1151
  %690 = icmp eq i8 %version.sroa.64.012701997, 3
  %691 = icmp ugt i8 %version.sroa.64.012701997, 2
  br i1 %690, label %bb12.i1155, label %bb227

bb12.i1155:                                       ; preds = %bb11.i1153
  %692 = icmp ugt i8 %version.sroa.92.012682055, 22
  br i1 %692, label %bb372, label %bb229.invoke

bb227:                                            ; preds = %bb11.i1153, %bb4.i1151
  %_0.sroa.0.0.shrunk.i1146 = phi i1 [ %691, %bb11.i1153 ], [ %689, %bb4.i1151 ]
  br i1 %_0.sroa.0.0.shrunk.i1146, label %bb372, label %bb229.invoke

bb229.invoke:                                     ; preds = %bb227, %bb12.i1155, %bb225, %bb12.i709, %bb221, %bb250, %bb254, %bb2.i.i.i4.i.i1134, %bb260, %bb12.i882, %bb3.i876
  %693 = phi ptr [ @alloc_983b9f659255e384bca2078a0c05baf3, %bb3.i876 ], [ @alloc_983b9f659255e384bca2078a0c05baf3, %bb12.i882 ], [ @alloc_983b9f659255e384bca2078a0c05baf3, %bb260 ], [ @alloc_236339e905ce8c9e5823d1ccff1bb370, %bb2.i.i.i4.i.i1134 ], [ @alloc_236339e905ce8c9e5823d1ccff1bb370, %bb254 ], [ @alloc_236339e905ce8c9e5823d1ccff1bb370, %bb250 ], [ @alloc_e1f6422a9d16410b776670e5fce196a5, %bb221 ], [ @alloc_e1f6422a9d16410b776670e5fce196a5, %bb12.i709 ], [ @alloc_e1f6422a9d16410b776670e5fce196a5, %bb225 ], [ @alloc_e1f6422a9d16410b776670e5fce196a5, %bb12.i1155 ], [ @alloc_e1f6422a9d16410b776670e5fce196a5, %bb227 ]
  %694 = phi ptr [ inttoptr (i64 111 to ptr), %bb3.i876 ], [ inttoptr (i64 111 to ptr), %bb12.i882 ], [ inttoptr (i64 111 to ptr), %bb260 ], [ inttoptr (i64 85 to ptr), %bb2.i.i.i4.i.i1134 ], [ inttoptr (i64 85 to ptr), %bb254 ], [ inttoptr (i64 85 to ptr), %bb250 ], [ inttoptr (i64 113 to ptr), %bb221 ], [ inttoptr (i64 113 to ptr), %bb12.i709 ], [ inttoptr (i64 113 to ptr), %bb225 ], [ inttoptr (i64 113 to ptr), %bb12.i1155 ], [ inttoptr (i64 113 to ptr), %bb227 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull %693, ptr noundef nonnull %694)
          to label %bb372 unwind label %cleanup7

bb412:                                            ; preds = %bb231
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_217)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_217, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_43a58bdcc2572eb4426ba7ac54f3fdc3, i64 noundef 23)
          to label %bb232 unwind label %cleanup7

bb232:                                            ; preds = %bb412
  call void @llvm.experimental.noalias.scope.decl(metadata !1173)
  call void @llvm.experimental.noalias.scope.decl(metadata !1176)
  %_2.i1160 = load i64, ptr %_217, align 8, !range !37, !alias.scope !1176, !noalias !1173, !noundef !3
  %695 = trunc nuw i64 %_2.i1160 to i1
  br i1 %695, label %bb3.i.i1163, label %bb233

bb3.i.i1163:                                      ; preds = %bb232
  call void @llvm.experimental.noalias.scope.decl(metadata !1178)
  %696 = getelementptr inbounds nuw i8, ptr %_217, i64 8
  %697 = load i64, ptr %696, align 8, !range !11, !alias.scope !1181, !noalias !1173, !noundef !3
  switch i64 %697, label %bb1.sink.split.i.i1166 [
    i64 -9223372036854775808, label %bb413.thread
    i64 0, label %bb413.thread
  ]

bb1.sink.split.i.i1166:                           ; preds = %bb3.i.i1163
  %698 = getelementptr inbounds nuw i8, ptr %_217, i64 16
  %_1.val1.i.i.i1167 = load ptr, ptr %698, align 8, !alias.scope !1184, !noalias !1173, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i1167, i64 noundef %697, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !1185
  br label %bb413.thread

bb413.thread:                                     ; preds = %bb3.i.i1163, %bb3.i.i1163, %bb1.sink.split.i.i1166
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_217)
  br label %bb234.invoke

bb233:                                            ; preds = %bb232
  %699 = getelementptr inbounds nuw i8, ptr %_217, i64 8
  %_216.sroa.0.0.copyload = load i64, ptr %699, align 8, !alias.scope !1186
  %_216.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_217, i64 16
  %_216.sroa.6.0.copyload = load ptr, ptr %_216.sroa.6.0..sroa_idx, align 8, !alias.scope !1186
  %_216.sroa.10.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_217, i64 24
  %_216.sroa.10.0.copyload = load i64, ptr %_216.sroa.10.0..sroa_idx, align 8, !alias.scope !1186
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_217)
  %_3.not.i1169 = icmp eq i64 %_216.sroa.10.0.copyload, 5
  br i1 %_3.not.i1169, label %bb2.i1172, label %bb413

bb2.i1172:                                        ; preds = %bb233
  %700 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(5) %_216.sroa.6.0.copyload, ptr noundef nonnull dereferenceable(5) @alloc_f5fa10d2bd50b965d2515db045847aab, i64 range(i64 0, -9223372036854775808) 5), !alias.scope !1187
  %701 = icmp eq i32 %700, 0
  br label %bb413

bb413:                                            ; preds = %bb2.i1172, %bb233
  %_0.sroa.0.0.off0.i1171 = phi i1 [ %701, %bb2.i1172 ], [ false, %bb233 ]
  %702 = icmp eq i64 %_216.sroa.0.0.copyload, 0
  br i1 %702, label %bb234.invoke, label %bb2.i.i.i4.i.i1176

bb2.i.i.i4.i.i1176:                               ; preds = %bb413
  %703 = icmp ne ptr %_216.sroa.6.0.copyload, null
  call void @llvm.assume(i1 %703)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_216.sroa.6.0.copyload, i64 noundef %_216.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb234.invoke

bb234.invoke:                                     ; preds = %bb413.thread, %bb413, %bb2.i.i.i4.i.i1176, %bb309, %bb324, %bb322, %bb369, %bb388
  %704 = phi ptr [ @alloc_42a2d297b45bdf881e84f224970518a9, %bb388 ], [ @alloc_d66b25e4b56c5d1a43b7571f92983e4a, %bb369 ], [ @alloc_9e015abacf20cb399b4f0a804bc0e15e, %bb322 ], [ @alloc_9e015abacf20cb399b4f0a804bc0e15e, %bb324 ], [ @alloc_da78d7c202eb238c4cdf0045a21de1ef, %bb309 ], [ @alloc_79633b065602e14b572fa6ec5d7a5307, %bb2.i.i.i4.i.i1176 ], [ @alloc_79633b065602e14b572fa6ec5d7a5307, %bb413 ], [ @alloc_79633b065602e14b572fa6ec5d7a5307, %bb413.thread ]
  %705 = phi i64 [ 16, %bb388 ], [ 26, %bb369 ], [ 5, %bb322 ], [ 5, %bb324 ], [ 6, %bb309 ], [ 10, %bb2.i.i.i4.i.i1176 ], [ 10, %bb413 ], [ 10, %bb413.thread ]
  %706 = phi i1 [ %pwr8_features.sroa.0.2.off0, %bb388 ], [ %arch13_features.sroa.0.0.off0, %bb369 ], [ %zaamo.sroa.0.0.off015491556, %bb322 ], [ %586, %bb324 ], [ %mclass.sroa.0.1.off0, %bb309 ], [ %_0.sroa.0.0.off0.i1171, %bb2.i.i.i4.i.i1176 ], [ %_0.sroa.0.0.off0.i1171, %bb413 ], [ false, %bb413.thread ]
; invoke build_script_build::target_feature_fallback
  %707 = invoke fastcc noundef zeroext i1 @_RNvCslwKqnJYeWCA_18build_script_build23target_feature_fallback(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %704, i64 noundef %705, i1 noundef zeroext %706)
          to label %bb372 unwind label %cleanup7

bb373:                                            ; preds = %bb2.i.i.i4.i.i790, %bb372
  %708 = icmp eq i64 %_12.sroa.0.0.copyload, 0
  br i1 %708, label %bb374, label %bb2.i.i.i4.i.i1178

bb2.i.i.i4.i.i1178:                               ; preds = %bb373
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_12.sroa.5.0.copyload, i64 noundef %_12.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb374

bb374:                                            ; preds = %bb2.i.i.i4.i.i1178, %bb373
  %709 = icmp eq i64 %_9.sroa.0.0.copyload, 0
  br i1 %709, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECslwKqnJYeWCA_18build_script_build.exit1181, label %bb2.i.i.i4.i.i1180

bb2.i.i.i4.i.i1180:                               ; preds = %bb374
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_9.sroa.5.0.copyload, i64 noundef %_9.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECslwKqnJYeWCA_18build_script_build.exit1181

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECslwKqnJYeWCA_18build_script_build.exit1181: ; preds = %bb374, %bb2.i.i.i4.i.i1180
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7)
  ret void
}

; <core::result::Result<alloc::string::String, std::env::VarError>>::expect
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %t, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %msg.0, i64 noundef range(i64 14, 32) %msg.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %0) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %e = alloca [24 x i8], align 8
  %_3 = load i64, ptr %self, align 8, !range !37, !noundef !3
  %1 = trunc nuw i64 %_3 to i1
  br i1 %1, label %bb2, label %bb3, !prof !102

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e)
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e, ptr noundef nonnull align 8 dereferenceable(24) %2, i64 24, i1 false)
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %msg.0, i64 noundef %msg.1, ptr noundef nonnull align 1 %e, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %0) #26
          to label %unreachable unwind label %cleanup

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 8
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %t, ptr noundef nonnull align 8 dereferenceable(24) %3, i64 24, i1 false)
  ret void

cleanup:                                          ; preds = %bb2
  %4 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !1191)
  %5 = load i64, ptr %e, align 8, !range !11, !alias.scope !1191, !noundef !3
  switch i64 %5, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %bb5
    i64 0, label %bb5
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup
  %6 = getelementptr inbounds nuw i8, ptr %e, i64 8
  %_1.val1.i = load ptr, ptr %6, align 8, !alias.scope !1191, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !1191
  br label %bb5

unreachable:                                      ; preds = %bb2
  unreachable

bb5:                                              ; preds = %bb2.i.i.i4.i.i.i.i, %cleanup, %cleanup
  resume { ptr, i32 } %4
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCslwKqnJYeWCA_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1) unnamed_addr #5 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %_15.i = add nsw i64 %elem_layout.0, -1
  %_17.i = add nuw nsw i64 %_15.i, %elem_layout.1
  %_19.i = sub nsw i64 0, %elem_layout.0
  %new_size.i = and i64 %_17.i, %_19.i
  %_24.i = add i64 %cap, -1
  %0 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %new_size.i, i64 %_24.i)
  %_27.0.i = extractvalue { i64, i1 } %0, 0
  %_27.1.i = extractvalue { i64, i1 } %0, 1
  %_33.i = sub nuw i64 -9223372036854775808, %elem_layout.0
  %_32.i = icmp ugt i64 %_27.0.i, %_33.i
  %or.cond.i = select i1 %_27.1.i, i1 true, i1 %_32.i
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !103

bb11.i:                                           ; preds = %bb6.i
  %new_size2.i = add nuw i64 %_27.0.i, %elem_layout.1
  %_40.i = icmp ugt i64 %new_size2.i, %_33.i
  br i1 %_40.i, label %bb11, label %bb14

bb14:                                             ; preds = %bb11.i
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb4.i.i11, label %bb3.i.i

bb14.thread:                                      ; preds = %start
  %2 = icmp eq i64 %self.0.val, 0
  br i1 %2, label %bb7.thread, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb14.thread, %bb14
  %_27.sroa.7.01321 = phi i64 [ %new_size2.i, %bb14 ], [ 0, %bb14.thread ]
  %3 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %3)
  %alloc_size.i23 = mul nuw i64 %elem_layout.1, %self.0.val
  %cond.i.i = icmp uge i64 %_27.sroa.7.01321, %alloc_size.i23
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %_27.sroa.7.01321) #23
  br label %bb7

bb7.thread:                                       ; preds = %bb14.thread
  %_16.i.i = inttoptr i64 %elem_layout.0 to ptr
  br label %bb9

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #23
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %bb3.i.i
  %_27.sroa.7.012 = phi i64 [ %_27.sroa.7.01321, %bb3.i.i ], [ %new_size2.i, %bb4.i.i11 ]
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3.i.i ], [ %4, %bb4.i.i11 ]
  %5 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %5, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %elem_layout.0, ptr %6, align 8
  br label %bb11

bb9:                                              ; preds = %bb7.thread, %bb7
  %raw_ptr.i.i.pn31 = phi ptr [ %_16.i.i, %bb7.thread ], [ %raw_ptr.i.i.pn, %bb7 ]
  %_27.sroa.7.01230 = phi i64 [ 0, %bb7.thread ], [ %_27.sroa.7.012, %bb7 ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn31, ptr %7, align 8
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb11.i, %bb9, %bb8
  %.sink32 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %bb11.i ], [ 8, %bb6.i ]
  %_27.sroa.7.01230.sink = phi i64 [ %_27.sroa.7.01230, %bb9 ], [ %_27.sroa.7.012, %bb8 ], [ 0, %bb11.i ], [ 0, %bb6.i ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %bb11.i ], [ 1, %bb6.i ]
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink32
  store i64 %_27.sroa.7.01230.sink, ptr %8, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <u32>::from_ascii_radix
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(argmem: read) uwtable
define internal fastcc range(i64 0, -4294967294) i64 @_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(none) %0, i64 noundef range(i64 0, -9223372036854775808) %1) unnamed_addr #6 {
start:
  switch i64 %1, label %bb9thread-pre-split [
    i64 0, label %bb28
    i64 1, label %bb7
  ]

bb28:                                             ; preds = %bb33, %bb40, %bb16, %bb31, %bb7, %bb7, %start, %bb25
  %_0.sroa.0.0 = phi i64 [ 0, %bb25 ], [ 1, %start ], [ 1, %bb7 ], [ 1, %bb7 ], [ 1, %bb31 ], [ 1, %bb16 ], [ 1, %bb40 ], [ 1, %bb33 ]
  %_0.sroa.8.0.insert.insert = phi i64 [ %5, %bb25 ], [ %1, %start ], [ 256, %bb7 ], [ 256, %bb7 ], [ %spec.select, %bb31 ], [ 256, %bb16 ], [ 256, %bb33 ], [ 512, %bb40 ]
  %_0.sroa.0.0.insert.insert = or disjoint i64 %_0.sroa.8.0.insert.insert, %_0.sroa.0.0
  ret i64 %_0.sroa.0.0.insert.insert

bb7:                                              ; preds = %start
  %2 = load i8, ptr %0, align 1, !noundef !3
  switch i8 %2, label %bb9 [
    i8 43, label %bb28
    i8 45, label %bb28
  ]

bb9thread-pre-split:                              ; preds = %start
  %.pr = load i8, ptr %0, align 1
  br label %bb9

bb9:                                              ; preds = %bb9thread-pre-split, %bb7
  %3 = phi i8 [ %.pr, %bb9thread-pre-split ], [ %2, %bb7 ]
  %cond = icmp eq i8 %3, 43
  %rest.1 = sext i1 %cond to i64
  %src.sroa.15.0 = add nsw i64 %1, %rest.1
  %src.sroa.0.0.idx = zext i1 %cond to i64
  %src.sroa.0.0 = getelementptr inbounds nuw i8, ptr %0, i64 %src.sroa.0.0.idx
  %_10 = icmp samesign ult i64 %src.sroa.15.0, 9
  br i1 %_10, label %bb15.preheader, label %bb22

bb15.preheader:                                   ; preds = %bb9
  %_13.not56 = icmp eq i64 %src.sroa.15.0, 0
  br i1 %_13.not56, label %bb25, label %bb16

bb22:                                             ; preds = %bb9, %bb40
  %result.sroa.0.0 = phi i32 [ %_60.0, %bb40 ], [ 0, %bb9 ]
  %src.sroa.15.1 = phi i64 [ %rest.12, %bb40 ], [ %src.sroa.15.0, %bb9 ]
  %src.sroa.0.1 = phi ptr [ %rest.01, %bb40 ], [ %src.sroa.0.0, %bb9 ]
  %_28.not = icmp eq i64 %src.sroa.15.1, 0
  br i1 %_28.not, label %bb25, label %bb23

bb25:                                             ; preds = %bb22, %bb20, %bb15.preheader
  %result.sroa.0.1 = phi i32 [ 0, %bb15.preheader ], [ %13, %bb20 ], [ %result.sroa.0.0, %bb22 ]
  %4 = zext i32 %result.sroa.0.1 to i64
  %5 = shl nuw i64 %4, 32
  br label %bb28

bb23:                                             ; preds = %bb22
  %rest.01 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1, i64 1
  %rest.12 = add nsw i64 %src.sroa.15.1, -1
  %6 = tail call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0, i32 10)
  %_57.0 = extractvalue { i32, i1 } %6, 0
  %_57.1 = extractvalue { i32, i1 } %6, 1
  %7 = load i8, ptr %src.sroa.0.1, align 1, !noundef !3
  br i1 %_57.1, label %bb31, label %bb33, !prof !102

bb33:                                             ; preds = %bb23
  %8 = zext i8 %7 to i32
  %9 = add nsw i32 %8, -48
  %_14.i = icmp ult i32 %9, 10
  br i1 %_14.i, label %bb40, label %bb28

bb31:                                             ; preds = %bb23
  %10 = add i8 %7, -48
  %_14.i45 = icmp ult i8 %10, 10
  %spec.select = select i1 %_14.i45, i64 512, i64 256
  br label %bb28

bb40:                                             ; preds = %bb33
  %_60.0 = add i32 %9, %_57.0
  %_60.1 = icmp ult i32 %_60.0, %_57.0
  br i1 %_60.1, label %bb28, label %bb22, !prof !102

bb16:                                             ; preds = %bb15.preheader, %bb20
  %src.sroa.0.259 = phi ptr [ %rest.04, %bb20 ], [ %src.sroa.0.0, %bb15.preheader ]
  %src.sroa.15.258 = phi i64 [ %rest.15, %bb20 ], [ %src.sroa.15.0, %bb15.preheader ]
  %result.sroa.0.257 = phi i32 [ %13, %bb20 ], [ 0, %bb15.preheader ]
  %_19 = load i8, ptr %src.sroa.0.259, align 1, !noundef !3
  %_18 = zext i8 %_19 to i32
  %11 = add nsw i32 %_18, -48
  %_14.i47 = icmp ult i32 %11, 10
  br i1 %_14.i47, label %bb20, label %bb28

bb20:                                             ; preds = %bb16
  %12 = mul i32 %result.sroa.0.257, 10
  %rest.15 = add nsw i64 %src.sroa.15.258, -1
  %rest.04 = getelementptr inbounds nuw i8, ptr %src.sroa.0.259, i64 1
  %13 = add i32 %11, %12
  %_13.not = icmp eq i64 %rest.15, 0
  br i1 %_13.not, label %bb25, label %bb16
}

; <core::str::iter::SplitInternal<char>>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #4 {
start:
  %_5 = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !54, !noundef !3
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !3, !align !18, !noundef !3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
  call fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_5, ptr noalias noundef align 8 dereferenceable(48) %_4) #28
  %_7 = load i64, ptr %_5, align 8, !range !37, !noundef !3
  %2 = trunc nuw i64 %_7 to i1
  br i1 %2, label %bb7, label %bb6

bb7:                                              ; preds = %bb2
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %a = load i64, ptr %3, align 8, !noundef !3
  %4 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %b = load i64, ptr %4, align 8, !noundef !3
  %i = load i64, ptr %self, align 8, !noundef !3
  %new_len = sub nuw i64 %a, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %b, ptr %self, align 8
  br label %bb8

bb6:                                              ; preds = %bb2
  %5 = load i8, ptr %0, align 1, !range !54, !alias.scope !1194, !noundef !3
  %_2.i = trunc nuw i8 %5 to i1
  br i1 %_2.i, label %bb8, label %bb1.i

bb1.i:                                            ; preds = %bb6
  store i8 1, ptr %0, align 1, !alias.scope !1194
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %7 = load i8, ptr %6, align 8, !range !54, !alias.scope !1194, !noundef !3
  %_3.i = trunc nuw i8 %7 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !1194
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !1194
  %_4.not.i = icmp ne i64 %i1.pre.i, %i.pre.i
  %or.cond.not.i = select i1 %_3.i, i1 true, i1 %_4.not.i
  br i1 %or.cond.not.i, label %bb4.i, label %bb8

bb4.i:                                            ; preds = %bb1.i
  %_10.val.i = load ptr, ptr %_4, align 8, !alias.scope !1194, !nonnull !3, !align !18, !noundef !3
  %new_len.i = sub nuw i64 %i1.pre.i, %i.pre.i
  %data.i = getelementptr inbounds nuw i8, ptr %_10.val.i, i64 %i.pre.i
  br label %bb8

bb8:                                              ; preds = %bb4.i, %bb1.i, %bb6, %bb7
  %_0.sroa.4.0 = phi i64 [ %new_len, %bb7 ], [ %new_len.i, %bb4.i ], [ undef, %bb6 ], [ undef, %bb1.i ]
  %_0.sroa.0.0 = phi ptr [ %data, %bb7 ], [ %data.i, %bb4.i ], [ null, %bb6 ], [ null, %bb1.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  br label %bb9

bb9:                                              ; preds = %start, %bb8
  %_0.sroa.4.1 = phi i64 [ %_0.sroa.4.0, %bb8 ], [ undef, %start ]
  %_0.sroa.0.1 = phi ptr [ %_0.sroa.0.0, %bb8 ], [ null, %start ]
  %8 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.1, 0
  %9 = insertvalue { ptr, i64 } %8, i64 %_0.sroa.4.1, 1
  ret { ptr, i64 } %9
}

; <u8>::from_ascii_radix
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(argmem: read) uwtable
define internal fastcc { i1, i8 } @_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix(ptr noalias noundef nonnull readonly align 1 captures(none) %0, i64 noundef range(i64 0, -9223372036854775808) %1) unnamed_addr #6 {
start:
  switch i64 %1, label %bb9thread-pre-split [
    i64 0, label %bb28
    i64 1, label %bb7
  ]

bb28:                                             ; preds = %bb33, %bb40, %bb22, %bb20, %bb16, %bb15.preheader, %bb31, %bb7, %bb7, %start
  %_0.sroa.8.0 = phi i8 [ 0, %start ], [ 1, %bb7 ], [ 1, %bb7 ], [ %spec.select, %bb31 ], [ 0, %bb15.preheader ], [ %14, %bb20 ], [ 1, %bb16 ], [ 2, %bb40 ], [ 1, %bb33 ], [ %result.sroa.0.0, %bb22 ]
  %_0.sroa.0.0.off0 = phi i1 [ true, %start ], [ true, %bb7 ], [ true, %bb7 ], [ true, %bb31 ], [ false, %bb15.preheader ], [ %_14.i46, %bb16 ], [ %_14.i46, %bb20 ], [ %_30.not.not.not, %bb22 ], [ %_30.not.not.not, %bb40 ], [ %_30.not.not.not, %bb33 ]
  %2 = insertvalue { i1, i8 } poison, i1 %_0.sroa.0.0.off0, 0
  %3 = insertvalue { i1, i8 } %2, i8 %_0.sroa.8.0, 1
  ret { i1, i8 } %3

bb7:                                              ; preds = %start
  %4 = load i8, ptr %0, align 1, !noundef !3
  switch i8 %4, label %bb9 [
    i8 43, label %bb28
    i8 45, label %bb28
  ]

bb9thread-pre-split:                              ; preds = %start
  %.pr = load i8, ptr %0, align 1
  br label %bb9

bb9:                                              ; preds = %bb9thread-pre-split, %bb7
  %5 = phi i8 [ %.pr, %bb9thread-pre-split ], [ %4, %bb7 ]
  %cond = icmp eq i8 %5, 43
  %rest.1 = sext i1 %cond to i64
  %src.sroa.15.0 = add nsw i64 %1, %rest.1
  %src.sroa.0.0.idx = zext i1 %cond to i64
  %src.sroa.0.0 = getelementptr inbounds nuw i8, ptr %0, i64 %src.sroa.0.0.idx
  %_10 = icmp samesign ult i64 %src.sroa.15.0, 3
  br i1 %_10, label %bb15.preheader, label %bb22

bb15.preheader:                                   ; preds = %bb9
  %_13.not52 = icmp eq i64 %src.sroa.15.0, 0
  br i1 %_13.not52, label %bb28, label %bb16

bb22:                                             ; preds = %bb9, %bb40
  %result.sroa.0.0 = phi i8 [ %_63.0, %bb40 ], [ 0, %bb9 ]
  %src.sroa.15.1 = phi i64 [ %rest.12, %bb40 ], [ %src.sroa.15.0, %bb9 ]
  %src.sroa.0.1 = phi ptr [ %rest.01, %bb40 ], [ %src.sroa.0.0, %bb9 ]
  %_30.not.not.not = icmp ne i64 %src.sroa.15.1, 0
  br i1 %_30.not.not.not, label %bb23, label %bb28

bb23:                                             ; preds = %bb22
  %rest.01 = getelementptr inbounds nuw i8, ptr %src.sroa.0.1, i64 1
  %rest.12 = add nsw i64 %src.sroa.15.1, -1
  %6 = tail call { i8, i1 } @llvm.umul.with.overflow.i8(i8 %result.sroa.0.0, i8 10)
  %_60.0 = extractvalue { i8, i1 } %6, 0
  %_60.1 = extractvalue { i8, i1 } %6, 1
  %7 = load i8, ptr %src.sroa.0.1, align 1, !noundef !3
  br i1 %_60.1, label %bb31, label %bb33, !prof !102

bb33:                                             ; preds = %bb23
  %8 = zext i8 %7 to i32
  %9 = add nsw i32 %8, -48
  %_14.i = icmp ult i32 %9, 10
  br i1 %_14.i, label %bb40, label %bb28

bb31:                                             ; preds = %bb23
  %10 = add i8 %7, -48
  %_14.i44 = icmp ult i8 %10, 10
  %spec.select = select i1 %_14.i44, i8 2, i8 1
  br label %bb28

bb40:                                             ; preds = %bb33
  %11 = trunc nuw nsw i32 %9 to i8
  %_63.0 = add i8 %_60.0, %11
  %_63.1 = icmp ult i8 %_63.0, %_60.0
  br i1 %_63.1, label %bb28, label %bb22, !prof !102

bb16:                                             ; preds = %bb15.preheader, %bb20
  %src.sroa.0.255 = phi ptr [ %rest.05, %bb20 ], [ %src.sroa.0.0, %bb15.preheader ]
  %src.sroa.15.254 = phi i64 [ %rest.16, %bb20 ], [ %src.sroa.15.0, %bb15.preheader ]
  %result.sroa.0.253 = phi i8 [ %14, %bb20 ], [ 0, %bb15.preheader ]
  %_20 = load i8, ptr %src.sroa.0.255, align 1, !noundef !3
  %_19 = zext i8 %_20 to i32
  %12 = add nsw i32 %_19, -48
  %_14.i46 = icmp ugt i32 %12, 9
  br i1 %_14.i46, label %bb28, label %bb20

bb20:                                             ; preds = %bb16
  %13 = mul i8 %result.sroa.0.253, 10
  %rest.16 = add nsw i64 %src.sroa.15.254, -1
  %rest.05 = getelementptr inbounds nuw i8, ptr %src.sroa.0.255, i64 1
  %_24 = trunc nuw nsw i32 %12 to i8
  %14 = add i8 %13, %_24
  %_13.not = icmp eq i64 %rest.16, 0
  br i1 %_13.not, label %bb28, label %bb16
}

; <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
; Function Attrs: uwtable
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCslwKqnJYeWCA_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !3
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1197)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1200)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !1203, !noalias !1204
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !1203, !noalias !1204
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !1203, !noalias !1204
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !1203, !noalias !1204
  store i64 0, ptr %self, align 8, !alias.scope !1203, !noalias !1204
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit

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
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !1206, !nonnull !3, !noundef !3
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !1207, !noundef !3
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #23, !noalias !1212
  %11 = load ptr, ptr %10, align 8, !noalias !1207, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #23, !noalias !1212
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1213)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1216)
  %_3.i.i = load i64, ptr %self, align 8, !range !37, !alias.scope !1219, !noalias !1220, !noundef !3
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !1219, !noalias !1220, !noundef !3
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !1222, !noalias !1225
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !1222, !noalias !1225
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !1219, !noalias !1220, !noundef !3
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !1219, !noalias !1220, !nonnull !3, !noundef !3
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !1219, !noalias !1220
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !1227, !nonnull !3, !noundef !3
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1228)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !1229, !noundef !3
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !1236, !noundef !3
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
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !1241, !nonnull !3, !noundef !3
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !1236
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #23, !noalias !1245
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !1229, !noundef !3
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #23, !noalias !1245
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #29
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !1246

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #26, !noalias !1247
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !1222, !noalias !1225
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !1222, !noalias !1225
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !1222, !noalias !1225
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build.exit
  ret void
}

; <&std::ffi::os_str::OsString as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtB6_5Debug3fmtCslwKqnJYeWCA_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !3, !align !7, !noundef !3
; call <std::ffi::os_str::OsString as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXs9_NtNtCs5sEH5CPMdak_3std3ffi6os_strNtB5_8OsStringNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCslwKqnJYeWCA_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !3, !align !18, !noundef !3
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !3
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <core::str::pattern::CharSearcher as core::str::pattern::Searcher>::next_match
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(48) %self) unnamed_addr #4 {
start:
  %self.0 = load ptr, ptr %self, align 8, !nonnull !3, !align !18, !noundef !3
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %index2 = load i64, ptr %2, align 8, !noundef !3
  %self.1 = load i64, ptr %0, align 8
  %self.1.fr = freeze i64 %self.1
  %_38.not = icmp ugt i64 %index2, %self.1.fr
  %.promoted = load i64, ptr %1, align 8
  %_4325 = icmp ult i64 %index2, %.promoted
  %or.cond26 = or i1 %_4325, %_38.not
  br i1 %or.cond26, label %bb11, label %bb12.lr.ph

bb12.lr.ph:                                       ; preds = %start
  %_10 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %_48 = load i8, ptr %3, align 8, !noundef !3
  %_12 = zext i8 %_48 to i64
  %4 = getelementptr i8, ptr %_10, i64 %_12
  %_49 = getelementptr i8, ptr %4, i64 -1
  %_65 = icmp ult i8 %_48, 5
  %last_byte.us.pre = load i8, ptr %_49, align 1
  br i1 %_65, label %bb12.us, label %bb12, !prof !1248

bb12.us:                                          ; preds = %bb12.lr.ph, %bb9.us
  %5 = phi i64 [ %14, %bb9.us ], [ %.promoted, %bb12.lr.ph ]
  %new_len.us = sub nuw i64 %index2, %5
  %_46.us = getelementptr inbounds nuw i8, ptr %self.0, i64 %5
  %_3.i.us = icmp samesign ult i64 %new_len.us, 16
  br i1 %_3.i.us, label %bb5.preheader.i.us, label %bb2.i.us

bb2.i.us:                                         ; preds = %bb12.us
; call core::slice::memchr::memchr_aligned
  %6 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us, i64 noundef range(i64 0, -9223372036854775808) %new_len.us)
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us

bb5.preheader.i.us:                               ; preds = %bb12.us
  %_64.not.i.us = icmp eq i64 %new_len.us, 0
  br i1 %_64.not.i.us, label %bb4.i.us, label %bb7.i.us

bb7.i.us:                                         ; preds = %bb5.preheader.i.us, %bb9.i.us
  %i.sroa.0.05.i.us = phi i64 [ %8, %bb9.i.us ], [ 0, %bb5.preheader.i.us ]
  %7 = getelementptr inbounds nuw i8, ptr %_46.us, i64 %i.sroa.0.05.i.us
  %_9.i.us = load i8, ptr %7, align 1, !alias.scope !1249, !noundef !3
  %_8.i.us = icmp eq i8 %_9.i.us, %last_byte.us.pre
  br i1 %_8.i.us, label %bb4.i.us, label %bb9.i.us

bb9.i.us:                                         ; preds = %bb7.i.us
  %8 = add nuw nsw i64 %i.sroa.0.05.i.us, 1
  %exitcond.not.i.us = icmp eq i64 %8, %new_len.us
  br i1 %exitcond.not.i.us, label %bb4.i.us, label %bb7.i.us

bb4.i.us:                                         ; preds = %bb7.i.us, %bb9.i.us, %bb5.preheader.i.us
  %i.sroa.0.0.lcssa.i.us = phi i64 [ 0, %bb5.preheader.i.us ], [ %new_len.us, %bb9.i.us ], [ %i.sroa.0.05.i.us, %bb7.i.us ]
  %_0.sroa.0.1.i.us = phi i64 [ 0, %bb5.preheader.i.us ], [ 0, %bb9.i.us ], [ 1, %bb7.i.us ]
  %9 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us, 0
  %10 = insertvalue { i64, i64 } %9, i64 %i.sroa.0.0.lcssa.i.us, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us: ; preds = %bb4.i.us, %bb2.i.us
  %.merged.i.us = phi { i64, i64 } [ %10, %bb4.i.us ], [ %6, %bb2.i.us ]
  %11 = extractvalue { i64, i64 } %.merged.i.us, 0
  %12 = trunc nuw i64 %11 to i1
  br i1 %12, label %bb4.us, label %bb10

bb4.us:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us
  %13 = extractvalue { i64, i64 } %.merged.i.us, 1
  %_16.us = add i64 %5, 1
  %14 = add i64 %_16.us, %13
  store i64 %14, ptr %1, align 8
  %_17.not.us = icmp ult i64 %14, %_12
  %_54.not.us = icmp ugt i64 %14, %self.1.fr
  %or.cond = or i1 %_17.not.us, %_54.not.us
  br i1 %or.cond, label %bb9.us, label %bb19.us

bb19.us:                                          ; preds = %bb4.us
  %found_char.us = sub nuw i64 %14, %_12
  %_62.us = getelementptr inbounds nuw i8, ptr %self.0, i64 %found_char.us
  %15 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us, ptr nonnull readonly align 1 %_10, i64 range(i64 0, -9223372036854775808) %_12), !alias.scope !1252
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %bb6, label %bb9.us

bb9.us:                                           ; preds = %bb19.us, %bb4.us
  %_43.us = icmp ult i64 %index2, %14
  br i1 %_43.us, label %bb11, label %bb12.us

bb12:                                             ; preds = %bb12.lr.ph, %bb9
  %17 = phi i64 [ %26, %bb9 ], [ %.promoted, %bb12.lr.ph ]
  %new_len = sub nuw i64 %index2, %17
  %_46 = getelementptr inbounds nuw i8, ptr %self.0, i64 %17
  %_3.i = icmp samesign ult i64 %new_len, 16
  br i1 %_3.i, label %bb5.preheader.i, label %bb2.i

bb5.preheader.i:                                  ; preds = %bb12
  %_64.not.i = icmp eq i64 %new_len, 0
  br i1 %_64.not.i, label %bb4.i, label %bb7.i

bb2.i:                                            ; preds = %bb12
; call core::slice::memchr::memchr_aligned
  %18 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46, i64 noundef range(i64 0, -9223372036854775808) %new_len)
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit

bb4.i:                                            ; preds = %bb9.i, %bb7.i, %bb5.preheader.i
  %i.sroa.0.0.lcssa.i = phi i64 [ 0, %bb5.preheader.i ], [ %new_len, %bb9.i ], [ %i.sroa.0.05.i, %bb7.i ]
  %_0.sroa.0.1.i = phi i64 [ 0, %bb5.preheader.i ], [ 0, %bb9.i ], [ 1, %bb7.i ]
  %19 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i, 0
  %20 = insertvalue { i64, i64 } %19, i64 %i.sroa.0.0.lcssa.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit

bb7.i:                                            ; preds = %bb5.preheader.i, %bb9.i
  %i.sroa.0.05.i = phi i64 [ %22, %bb9.i ], [ 0, %bb5.preheader.i ]
  %21 = getelementptr inbounds nuw i8, ptr %_46, i64 %i.sroa.0.05.i
  %_9.i = load i8, ptr %21, align 1, !alias.scope !1249, !noundef !3
  %_8.i = icmp eq i8 %_9.i, %last_byte.us.pre
  br i1 %_8.i, label %bb4.i, label %bb9.i

bb9.i:                                            ; preds = %bb7.i
  %22 = add nuw nsw i64 %i.sroa.0.05.i, 1
  %exitcond.not.i = icmp eq i64 %22, %new_len
  br i1 %exitcond.not.i, label %bb4.i, label %bb7.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit: ; preds = %bb2.i, %bb4.i
  %.merged.i = phi { i64, i64 } [ %20, %bb4.i ], [ %18, %bb2.i ]
  %23 = extractvalue { i64, i64 } %.merged.i, 0
  %24 = trunc nuw i64 %23 to i1
  br i1 %24, label %bb4, label %bb10

bb4:                                              ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit
  %25 = extractvalue { i64, i64 } %.merged.i, 1
  %_16 = add i64 %17, 1
  %26 = add i64 %_16, %25
  store i64 %26, ptr %1, align 8
  %_17.not = icmp ult i64 %26, %_12
  %_54.not = icmp ugt i64 %26, %self.1.fr
  %or.cond70 = or i1 %_17.not, %_54.not
  br i1 %or.cond70, label %bb9, label %bb25

bb10:                                             ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us
  store i64 %index2, ptr %1, align 8
  br label %bb11

bb9:                                              ; preds = %bb4
  %_43 = icmp ult i64 %index2, %26
  br i1 %_43, label %bb11, label %bb12

bb25:                                             ; preds = %bb4
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #29
  unreachable

bb6:                                              ; preds = %bb19.us
  %27 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %found_char.us, ptr %27, align 8
  %28 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %14, ptr %28, align 8
  br label %bb11

bb11:                                             ; preds = %bb9, %bb9.us, %start, %bb10, %bb6
  %.sink = phi i64 [ 0, %bb10 ], [ 1, %bb6 ], [ 0, %start ], [ 0, %bb9.us ], [ 0, %bb9 ]
  store i64 %.sink, ptr %_0, align 8
  ret void
}

; <std::env::VarError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsk_NtCs5sEH5CPMdak_3std3envNtB5_8VarErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %__self_0 = alloca [8 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !3
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_0)
  store ptr %self, ptr %__self_0, align 8
; call <core::fmt::Formatter>::debug_tuple_field1_finish
  %1 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_19adf04fb909e90136daf37b5ff22508, i64 noundef 10, ptr noundef nonnull align 1 %__self_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_0)
  br label %bb5

bb3:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %2 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1c5ece773fe9d8a26ac674de79674b77, i64 noundef 10)
  br label %bb5

bb5:                                              ; preds = %bb2, %bb3
  %_0.sroa.0.0.in = phi i1 [ %1, %bb2 ], [ %2, %bb3 ]
  ret i1 %_0.sroa.0.0.in
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7 = load i64, ptr %1, align 8, !noundef !3
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8, i64 noundef %_7, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #8

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #10

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; <std::sys::process::unix::common::Command>::arg
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef align 8 dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::new
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr dead_on_unwind noalias noundef writable sret([200 x i8]) align 8 captures(none) dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

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
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #13

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #14

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; <alloc::string::String>::from_utf8_lossy
; Function Attrs: uwtable
declare void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; std::panicking::begin_panic::<&str>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RINvNtCs5sEH5CPMdak_3std9panicking11begin_panicReEB4_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; <u32 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #15

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #16

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #17

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #13

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i8, i1 } @llvm.umul.with.overflow.i8(i8, i8) #13

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i16, i1 } @llvm.umul.with.overflow.i16(i16, i16) #13

; <std::process::Command>::output
; Function Attrs: uwtable
declare void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr dead_on_unwind noalias noundef writable sret([56 x i8]) align 8 captures(none) dereferenceable(56), ptr noalias noundef align 8 dereferenceable(200)) unnamed_addr #0

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #18

; <std::ffi::os_str::OsString as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs9_NtNtCs5sEH5CPMdak_3std3ffi6os_strNtB5_8OsStringNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #19

; <core::fmt::Formatter>::debug_tuple_field1_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::str::pattern::StrSearcher>::new
; Function Attrs: uwtable
declare void @_RNvMsu_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcher3new(ptr dead_on_unwind noalias noundef writable sret([104 x i8]) align 8 captures(none) dereferenceable(104), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #20 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCslwKqnJYeWCA_18build_script_build4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #21

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #22

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { inlinehint nofree norecurse nosync nounwind memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #11 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #14 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #19 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #20 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #21 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #22 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #23 = { nounwind }
attributes #24 = { cold }
attributes #25 = { cold noreturn nounwind }
attributes #26 = { noreturn }
attributes #27 = { noinline }
attributes #28 = { inlinehint }
attributes #29 = { noinline noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECslwKqnJYeWCA_18build_script_build: %_1.0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECslwKqnJYeWCA_18build_script_build"}
!7 = !{i64 8}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{i64 0, i64 -9223372036854775806}
!11 = !{i64 0, i64 -9223372036854775807}
!12 = !{!13}
!13 = distinct !{!13, !14, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECslwKqnJYeWCA_18build_script_build: %_1"}
!14 = distinct !{!14, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECslwKqnJYeWCA_18build_script_build"}
!15 = !{!16, !13}
!16 = distinct !{!16, !17, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECslwKqnJYeWCA_18build_script_build: %_1"}
!17 = distinct !{!17, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECslwKqnJYeWCA_18build_script_build"}
!18 = !{i64 1}
!19 = !{i64 4}
!20 = !{i32 0, i32 6}
!21 = !{!22}
!22 = distinct !{!22, !23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build: %_1"}
!23 = distinct !{!23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build"}
!24 = !{!25}
!25 = distinct !{!25, !26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCslwKqnJYeWCA_18build_script_build: %self"}
!26 = distinct !{!26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCslwKqnJYeWCA_18build_script_build"}
!27 = !{!25, !22}
!28 = !{!29, !31, !25, !22}
!29 = distinct !{!29, !30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCslwKqnJYeWCA_18build_script_build: %self"}
!30 = distinct !{!30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCslwKqnJYeWCA_18build_script_build"}
!31 = distinct !{!31, !32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build: %_1"}
!32 = distinct !{!32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECslwKqnJYeWCA_18build_script_build"}
!33 = !{i64 19861341825568678}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCslwKqnJYeWCA_18build_script_build: %self"}
!36 = distinct !{!36, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCslwKqnJYeWCA_18build_script_build"}
!37 = !{i64 0, i64 2}
!38 = !{!39}
!39 = distinct !{!39, !40, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB20_7Version5parses0_00E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB22_: %self"}
!40 = distinct !{!40, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB20_7Version5parses0_00E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB22_"}
!41 = !{!42}
!42 = distinct !{!42, !43, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!43 = distinct !{!43, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!44 = !{!45}
!45 = distinct !{!45, !46, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!46 = distinct !{!46, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!47 = !{!48}
!48 = distinct !{!48, !49, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!49 = distinct !{!49, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self"}
!52 = distinct !{!52, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build"}
!53 = !{!51, !48, !45, !42, !39}
!54 = !{i8 0, i8 2}
!55 = !{!56, !51, !48, !45, !42, !39}
!56 = distinct !{!56, !57, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!57 = distinct !{!57, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!58 = !{!59, !61, !63, !65}
!59 = distinct !{!59, !60, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!60 = distinct !{!60, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!61 = distinct !{!61, !62, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!62 = distinct !{!62, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!63 = distinct !{!63, !64, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call: argument 0"}
!64 = distinct !{!64, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call"}
!65 = distinct !{!65, !66, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut: argument 0"}
!66 = distinct !{!66, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut"}
!67 = !{!68, !69, !45, !42, !39}
!68 = distinct !{!68, !60, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!69 = distinct !{!69, !62, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!70 = !{!71, !73, !63, !65}
!71 = distinct !{!71, !72, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!72 = distinct !{!72, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!73 = distinct !{!73, !74, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!74 = distinct !{!74, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!75 = !{!76, !77, !45, !42, !39}
!76 = distinct !{!76, !72, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!77 = distinct !{!77, !74, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!78 = !{!79, !81, !82, !84}
!79 = distinct !{!79, !80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!80 = distinct !{!80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!81 = distinct !{!81, !80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!82 = distinct !{!82, !83, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!83 = distinct !{!83, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!84 = distinct !{!84, !83, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!85 = !{!86, !87, !88, !89, !39}
!86 = distinct !{!86, !52, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!87 = distinct !{!87, !49, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!88 = distinct !{!88, !46, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!89 = distinct !{!89, !43, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self:h.rot"}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!92 = distinct !{!92, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!93 = !{!94}
!94 = distinct !{!94, !95, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!95 = distinct !{!95, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!96 = !{!97, !94}
!97 = distinct !{!97, !98, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!98 = distinct !{!98, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!99 = !{!100}
!100 = distinct !{!100, !101, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!101 = distinct !{!101, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!102 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!103 = !{!"branch_weights", i32 2002, i32 2000}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!106 = distinct !{!106, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!107 = !{!108, !105}
!108 = distinct !{!108, !109, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!109 = distinct !{!109, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!112 = distinct !{!112, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!115 = distinct !{!115, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!116 = !{!117}
!117 = distinct !{!117, !118, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!118 = distinct !{!118, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!119 = !{!120}
!120 = distinct !{!120, !121, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!121 = distinct !{!121, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!122 = !{!123}
!123 = distinct !{!123, !124, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB20_7Version5parses1_00E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB22_: %self"}
!124 = distinct !{!124, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB20_7Version5parses1_00E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB22_"}
!125 = !{!126}
!126 = distinct !{!126, !127, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!127 = distinct !{!127, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!128 = !{!129}
!129 = distinct !{!129, !130, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!130 = distinct !{!130, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!131 = !{!132}
!132 = distinct !{!132, !133, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!133 = distinct !{!133, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!134 = !{!135}
!135 = distinct !{!135, !136, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self"}
!136 = distinct !{!136, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build"}
!137 = !{!135, !132, !129, !126, !123}
!138 = !{!139, !135, !132, !129, !126, !123}
!139 = distinct !{!139, !140, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!140 = distinct !{!140, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!141 = !{!142, !144, !146, !148}
!142 = distinct !{!142, !143, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!143 = distinct !{!143, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!144 = distinct !{!144, !145, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!145 = distinct !{!145, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!146 = distinct !{!146, !147, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call: argument 0"}
!147 = distinct !{!147, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call"}
!148 = distinct !{!148, !149, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut: argument 0"}
!149 = distinct !{!149, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut"}
!150 = !{!151, !152, !129, !126, !123}
!151 = distinct !{!151, !143, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!152 = distinct !{!152, !145, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!153 = !{!154, !156, !146, !148}
!154 = distinct !{!154, !155, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!155 = distinct !{!155, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!156 = distinct !{!156, !157, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!157 = distinct !{!157, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!158 = !{!159, !160, !129, !126, !123}
!159 = distinct !{!159, !155, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!160 = distinct !{!160, !157, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!161 = !{!162, !164, !165, !167}
!162 = distinct !{!162, !163, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!163 = distinct !{!163, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!164 = distinct !{!164, !163, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!165 = distinct !{!165, !166, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!166 = distinct !{!166, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!167 = distinct !{!167, !166, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!168 = !{!169, !170, !171, !172, !123}
!169 = distinct !{!169, !136, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!170 = distinct !{!170, !133, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!171 = distinct !{!171, !130, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!172 = distinct !{!172, !127, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self:h.rot"}
!173 = !{!174}
!174 = distinct !{!174, !175, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!175 = distinct !{!175, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!176 = !{!177}
!177 = distinct !{!177, !178, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!178 = distinct !{!178, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!179 = !{!180, !177}
!180 = distinct !{!180, !181, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!181 = distinct !{!181, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!182 = !{!183}
!183 = distinct !{!183, !184, !"_RNvMsz_NtCsjMrxcFdYDNN_4core3numt16from_ascii_radix: argument 0"}
!184 = distinct !{!184, !"_RNvMsz_NtCsjMrxcFdYDNN_4core3numt16from_ascii_radix"}
!185 = !{!186}
!186 = distinct !{!186, !187, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!187 = distinct !{!187, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!188 = !{!189, !186}
!189 = distinct !{!189, !190, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!190 = distinct !{!190, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!191 = !{!192}
!192 = distinct !{!192, !193, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!193 = distinct !{!193, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!194 = !{!195}
!195 = distinct !{!195, !196, !"_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix: argument 0"}
!196 = distinct !{!196, !"_RNvMsx_NtCsjMrxcFdYDNN_4core3numh16from_ascii_radix"}
!197 = !{!198}
!198 = distinct !{!198, !199, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!199 = distinct !{!199, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!200 = !{!201}
!201 = distinct !{!201, !202, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CslwKqnJYeWCA_18build_script_build: %_1"}
!202 = distinct !{!202, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CslwKqnJYeWCA_18build_script_build"}
!203 = !{!204}
!204 = distinct !{!204, !205, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!205 = distinct !{!205, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!206 = !{!207, !204}
!207 = distinct !{!207, !208, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!208 = distinct !{!208, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!209 = !{!210}
!210 = distinct !{!210, !211, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!211 = distinct !{!211, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!212 = !{!213, !215, !216, !218, !219, !210}
!213 = distinct !{!213, !214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!214 = distinct !{!214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!215 = distinct !{!215, !214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!216 = distinct !{!216, !217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!217 = distinct !{!217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!218 = distinct !{!218, !217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!219 = distinct !{!219, !211, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!220 = !{!221, !219}
!221 = distinct !{!221, !222, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!222 = distinct !{!222, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!223 = !{!"branch_weights", i32 2000, i32 6004}
!224 = !{!225}
!225 = distinct !{!225, !226, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build: %_0"}
!226 = distinct !{!226, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build"}
!227 = !{!228}
!228 = distinct !{!228, !229, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!229 = distinct !{!229, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!230 = !{!231, !233, !234, !236, !237, !228}
!231 = distinct !{!231, !232, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!232 = distinct !{!232, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!233 = distinct !{!233, !232, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!234 = distinct !{!234, !235, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!235 = distinct !{!235, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!236 = distinct !{!236, !235, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!237 = distinct !{!237, !229, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!238 = !{!239, !237}
!239 = distinct !{!239, !240, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!240 = distinct !{!240, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!241 = !{!242}
!242 = distinct !{!242, !205, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!243 = !{!244, !246, !247, !249}
!244 = distinct !{!244, !245, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!245 = distinct !{!245, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!246 = distinct !{!246, !245, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!247 = distinct !{!247, !248, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!248 = distinct !{!248, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!249 = distinct !{!249, !248, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!250 = !{!251}
!251 = distinct !{!251, !252, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!252 = distinct !{!252, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!253 = !{!254}
!254 = distinct !{!254, !255, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!255 = distinct !{!255, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!256 = !{!257, !254}
!257 = distinct !{!257, !258, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!258 = distinct !{!258, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!259 = !{!260}
!260 = distinct !{!260, !261, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!261 = distinct !{!261, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!262 = !{!263, !265, !266, !268, !269, !260}
!263 = distinct !{!263, !264, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!264 = distinct !{!264, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!265 = distinct !{!265, !264, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!266 = distinct !{!266, !267, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!267 = distinct !{!267, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!268 = distinct !{!268, !267, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!269 = distinct !{!269, !261, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!270 = !{!271, !269}
!271 = distinct !{!271, !272, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!272 = distinct !{!272, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!273 = !{!274}
!274 = distinct !{!274, !275, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!275 = distinct !{!275, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!276 = !{!277, !279, !280, !282, !283, !274}
!277 = distinct !{!277, !278, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!278 = distinct !{!278, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!279 = distinct !{!279, !278, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!280 = distinct !{!280, !281, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!281 = distinct !{!281, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!282 = distinct !{!282, !281, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!283 = distinct !{!283, !275, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!284 = !{!285, !283}
!285 = distinct !{!285, !286, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!286 = distinct !{!286, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!287 = !{!288}
!288 = distinct !{!288, !255, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!289 = !{!290}
!290 = distinct !{!290, !291, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvBK_3any5checkReNCNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature0E0INtNtNtBa_3ops12control_flow11ControlFlowuEEB1Z_: %self"}
!291 = distinct !{!291, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvBK_3any5checkReNCNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature0E0INtNtNtBa_3ops12control_flow11ControlFlowuEEB1Z_"}
!292 = !{!293}
!293 = distinct !{!293, !294, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!294 = distinct !{!294, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!295 = !{!296}
!296 = distinct !{!296, !297, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!297 = distinct !{!297, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!298 = !{!296, !293, !290}
!299 = !{!300}
!300 = distinct !{!300, !291, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNvBK_3any5checkReNCNvCslwKqnJYeWCA_18build_script_build18is_allowed_feature0E0INtNtNtBa_3ops12control_flow11ControlFlowuEEB1Z_: argument 1"}
!301 = !{!296, !293, !290, !300}
!302 = !{!303, !296, !293, !290}
!303 = distinct !{!303, !304, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!304 = distinct !{!304, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!305 = !{!306, !308}
!306 = distinct !{!306, !307, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!307 = distinct !{!307, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!308 = distinct !{!308, !307, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!309 = !{!290, !300}
!310 = !{!311, !312, !290}
!311 = distinct !{!311, !297, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!312 = distinct !{!312, !294, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!313 = !{!314}
!314 = distinct !{!314, !315, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!315 = distinct !{!315, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!316 = !{!317, !314}
!317 = distinct !{!317, !318, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!318 = distinct !{!318, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!319 = !{!320}
!320 = distinct !{!320, !321, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!321 = distinct !{!321, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!322 = !{!323, !325, !326, !328, !329, !320}
!323 = distinct !{!323, !324, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!324 = distinct !{!324, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!325 = distinct !{!325, !324, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!326 = distinct !{!326, !327, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!327 = distinct !{!327, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!328 = distinct !{!328, !327, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!329 = distinct !{!329, !321, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!330 = !{!331, !329}
!331 = distinct !{!331, !332, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!332 = distinct !{!332, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!333 = !{!334}
!334 = distinct !{!334, !335, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!335 = distinct !{!335, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!336 = !{!337, !339, !340, !342, !343, !334}
!337 = distinct !{!337, !338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!338 = distinct !{!338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!339 = distinct !{!339, !338, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!340 = distinct !{!340, !341, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!341 = distinct !{!341, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!342 = distinct !{!342, !341, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!343 = distinct !{!343, !335, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!344 = !{!345, !343}
!345 = distinct !{!345, !346, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!346 = distinct !{!346, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!347 = !{!348}
!348 = distinct !{!348, !315, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!349 = !{!350}
!350 = distinct !{!350, !351, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!351 = distinct !{!351, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!352 = !{!353, !355}
!353 = distinct !{!353, !354, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!354 = distinct !{!354, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!355 = distinct !{!355, !354, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!356 = !{!357, !359}
!357 = distinct !{!357, !358, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!358 = distinct !{!358, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!359 = distinct !{!359, !358, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!360 = !{!361, !363}
!361 = distinct !{!361, !362, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!362 = distinct !{!362, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!363 = distinct !{!363, !362, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!364 = !{!365}
!365 = distinct !{!365, !366, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %t"}
!366 = distinct !{!366, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build"}
!367 = !{!368}
!368 = distinct !{!368, !366, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %self"}
!369 = !{!365, !370, !371}
!370 = distinct !{!370, !366, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %msg.0"}
!371 = distinct !{!371, !366, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: argument 3"}
!372 = !{!365, !368, !370, !371}
!373 = !{!365, !368}
!374 = !{!375}
!375 = distinct !{!375, !376, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!376 = distinct !{!376, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!377 = !{!375, !365, !368}
!378 = !{!370, !371}
!379 = !{!380}
!380 = distinct !{!380, !381, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %t"}
!381 = distinct !{!381, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build"}
!382 = !{!383}
!383 = distinct !{!383, !381, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %self"}
!384 = !{!380, !385, !386}
!385 = distinct !{!385, !381, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %msg.0"}
!386 = distinct !{!386, !381, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: argument 3"}
!387 = !{!380, !383, !385, !386}
!388 = !{!380, !383}
!389 = !{!390}
!390 = distinct !{!390, !391, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!391 = distinct !{!391, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!392 = !{!390, !380, !383}
!393 = !{!385, !386}
!394 = !{!395}
!395 = distinct !{!395, !396, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %t"}
!396 = distinct !{!396, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build"}
!397 = !{!398}
!398 = distinct !{!398, !396, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %self"}
!399 = !{!395, !400, !401}
!400 = distinct !{!400, !396, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: %msg.0"}
!401 = distinct !{!401, !396, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6expectCslwKqnJYeWCA_18build_script_build: argument 3"}
!402 = !{!395, !398, !400, !401}
!403 = !{!395, !398}
!404 = !{!405}
!405 = distinct !{!405, !406, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!406 = distinct !{!406, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!407 = !{!405, !395, !398}
!408 = !{!400, !401}
!409 = !{!410}
!410 = distinct !{!410, !411, !"_RNvNtCslwKqnJYeWCA_18build_script_build7version13rustc_version: %_0"}
!411 = distinct !{!411, !"_RNvNtCslwKqnJYeWCA_18build_script_build7version13rustc_version"}
!412 = !{!413}
!413 = distinct !{!413, !414, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvNtCslwKqnJYeWCA_18build_script_build7version13rustc_version0EB1E_: %self"}
!414 = distinct !{!414, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvNtCslwKqnJYeWCA_18build_script_build7version13rustc_version0EB1E_"}
!415 = !{!416, !410}
!416 = distinct !{!416, !414, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvNtCslwKqnJYeWCA_18build_script_build7version13rustc_version0EB1E_: %_0"}
!417 = !{!416, !413, !410}
!418 = !{!419}
!419 = distinct !{!419, !420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build: %other"}
!420 = distinct !{!420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build"}
!421 = !{!422}
!422 = distinct !{!422, !420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build: %_0"}
!423 = !{!424, !419, !410}
!424 = distinct !{!424, !420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build: %self"}
!425 = !{!422, !419}
!426 = !{!424, !410}
!427 = !{!422, !428, !429}
!428 = distinct !{!428, !420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build: %other:thread"}
!429 = distinct !{!429, !420, !"_RINvYINtNtCsjMrxcFdYDNN_4core6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB8_4iter6traits8iterator8Iterator5chainINtNtNtB1w_7sources4once4OnceBH_EECslwKqnJYeWCA_18build_script_build: %other:thread"}
!430 = !{!431, !433, !434, !436}
!431 = distinct !{!431, !432, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build: %_0"}
!432 = distinct !{!432, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build"}
!433 = distinct !{!433, !432, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build: %_1"}
!434 = distinct !{!434, !435, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %x"}
!435 = distinct !{!435, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build"}
!436 = distinct !{!436, !435, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %f"}
!437 = !{!438, !410}
!438 = distinct !{!438, !435, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %self"}
!439 = !{!440}
!440 = distinct !{!440, !441, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build: %opt"}
!441 = distinct !{!441, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build"}
!442 = !{!443, !410}
!443 = distinct !{!443, !441, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build: %_0"}
!444 = !{!445, !447, !410}
!445 = distinct !{!445, !446, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build: %_0"}
!446 = distinct !{!446, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build"}
!447 = distinct !{!447, !446, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECslwKqnJYeWCA_18build_script_build: %program"}
!448 = !{!447, !410}
!449 = !{!450, !452, !410}
!450 = distinct !{!450, !451, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEINtNtNtBX_7sources4once4OnceB26_EEB26_ECslwKqnJYeWCA_18build_script_build: %self"}
!451 = distinct !{!451, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEINtNtNtBX_7sources4once4OnceB26_EEB26_ECslwKqnJYeWCA_18build_script_build"}
!452 = distinct !{!452, !451, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEINtNtNtBX_7sources4once4OnceB26_EEB26_ECslwKqnJYeWCA_18build_script_build: %args"}
!453 = !{!454, !456}
!454 = distinct !{!454, !455, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build: %opt"}
!455 = distinct !{!455, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build"}
!456 = distinct !{!456, !457, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtB8_7sources4once4OnceB1n_EENtNtNtB8_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!457 = distinct !{!457, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtB8_7sources4once4OnceB1n_EENtNtNtB8_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!458 = !{!459, !460, !450, !452, !410}
!459 = distinct !{!459, !455, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECslwKqnJYeWCA_18build_script_build: %_0"}
!460 = distinct !{!460, !457, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtB8_7sources4once4OnceB1n_EENtNtNtB8_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %_0"}
!461 = !{!460}
!462 = !{!456}
!463 = !{!464}
!464 = distinct !{!464, !465, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %x"}
!465 = distinct !{!465, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build"}
!466 = !{!467}
!467 = distinct !{!467, !465, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %self"}
!468 = !{!469}
!469 = distinct !{!469, !465, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECslwKqnJYeWCA_18build_script_build: %f"}
!470 = !{!464, !467, !460}
!471 = !{!469, !456, !450, !452, !410}
!472 = !{!473}
!473 = distinct !{!473, !474, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build: %_0"}
!474 = distinct !{!474, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build"}
!475 = !{!476, !469, !456}
!476 = distinct !{!476, !474, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBa_7sources4once4OnceB1p_EENtNtNtBa_6traits8iterator8Iterator4next0CslwKqnJYeWCA_18build_script_build: %_1"}
!477 = !{!473, !464, !467, !460, !450, !452, !410}
!478 = !{!479, !481, !473, !476, !464, !469, !460, !456}
!479 = distinct !{!479, !480, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %_0"}
!480 = distinct !{!480, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!481 = distinct !{!481, !480, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!482 = !{!467, !450, !452, !410}
!483 = !{!452, !410}
!484 = !{!473, !476, !464, !469, !460, !456}
!485 = !{!486}
!486 = distinct !{!486, !487, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBN_7sources4once4OnceB1G_EEECslwKqnJYeWCA_18build_script_build: %_1"}
!487 = distinct !{!487, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtNtNtBN_7sources4once4OnceB1G_EEECslwKqnJYeWCA_18build_script_build"}
!488 = !{!486, !452, !410}
!489 = !{!490}
!490 = distinct !{!490, !491, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECslwKqnJYeWCA_18build_script_build: %_1"}
!491 = distinct !{!491, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECslwKqnJYeWCA_18build_script_build"}
!492 = !{!"branch_weights", i32 2000, i32 6001}
!493 = !{!490, !410}
!494 = !{!495}
!495 = distinct !{!495, !496, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build: %_1"}
!496 = distinct !{!496, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build"}
!497 = !{!495, !410}
!498 = !{!499, !501, !410}
!499 = distinct !{!499, !500, !"_RNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB2_7Version5parse: %_0"}
!500 = distinct !{!500, !"_RNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB2_7Version5parse"}
!501 = distinct !{!501, !500, !"_RNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB2_7Version5parse: argument 1"}
!502 = !{!503}
!503 = distinct !{!503, !504, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1Y_7Version5parse0E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB20_: %self"}
!504 = distinct !{!504, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvNvBH_4find5checkReNCNvMNtCslwKqnJYeWCA_18build_script_build7versionNtB1Y_7Version5parse0E0INtNtNtB9_3ops12control_flow11ControlFlowB1R_EEB20_"}
!505 = !{!506}
!506 = distinct !{!506, !507, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!507 = distinct !{!507, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!508 = !{!509}
!509 = distinct !{!509, !510, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!510 = distinct !{!510, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!511 = !{!512}
!512 = distinct !{!512, !513, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!513 = distinct !{!513, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!514 = !{!515}
!515 = distinct !{!515, !516, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self"}
!516 = distinct !{!516, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build"}
!517 = !{!515, !512, !509, !506, !503}
!518 = !{!515, !512, !509, !506, !503, !499, !501, !410}
!519 = !{!520, !515, !512, !509, !506, !503}
!520 = distinct !{!520, !521, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!521 = distinct !{!521, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!522 = !{!523, !525, !527, !529}
!523 = distinct !{!523, !524, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!524 = distinct !{!524, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!525 = distinct !{!525, !526, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!526 = distinct !{!526, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!527 = distinct !{!527, !528, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call: argument 0"}
!528 = distinct !{!528, !"_RNvXs3_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function2FnTReEE4call"}
!529 = distinct !{!529, !530, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut: argument 0"}
!530 = distinct !{!530, !"_RNvXs4_NtCsjMrxcFdYDNN_4core3strNtB5_8LinesMapINtNtNtB7_3ops8function5FnMutTReEE8call_mut"}
!531 = !{!532, !533, !509, !506, !503, !499, !410}
!532 = distinct !{!532, !524, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!533 = distinct !{!533, !526, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!534 = !{!535, !537, !527, !529}
!535 = distinct !{!535, !536, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!536 = distinct !{!536, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!537 = distinct !{!537, !538, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %haystack.0"}
!538 = distinct !{!538, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of"}
!539 = !{!540, !541, !509, !506, !503, !499, !410}
!540 = distinct !{!540, !536, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!541 = distinct !{!541, !538, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15strip_suffix_of: %self.0"}
!542 = !{!543, !545, !546, !548}
!543 = distinct !{!543, !544, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!544 = distinct !{!544, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!545 = distinct !{!545, !544, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!546 = distinct !{!546, !547, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!547 = distinct !{!547, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!548 = distinct !{!548, !547, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!549 = !{!503, !499, !410}
!550 = !{!551, !552, !553, !554, !503}
!551 = distinct !{!551, !516, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE14next_inclusiveCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!552 = distinct !{!552, !513, !"_RNvXsH_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_14SplitInclusivecENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!553 = distinct !{!553, !510, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtBb_3str4iter14SplitInclusivecENtB11_8LinesMapENtNtNtB9_6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!554 = distinct !{!554, !507, !"_RNvXss_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_5LinesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self:h.rot"}
!555 = !{!556}
!556 = distinct !{!556, !557, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!557 = distinct !{!557, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!558 = !{!499, !410}
!559 = !{!560}
!560 = distinct !{!560, !561, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!561 = distinct !{!561, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!562 = !{!560, !499, !501, !410}
!563 = !{!564, !560}
!564 = distinct !{!564, !565, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!565 = distinct !{!565, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!566 = !{!567}
!567 = distinct !{!567, !568, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!568 = distinct !{!568, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!569 = !{!567, !499, !501, !410}
!570 = !{!571, !567}
!571 = distinct !{!571, !572, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!572 = distinct !{!572, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!573 = !{!574}
!574 = distinct !{!574, !575, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!575 = distinct !{!575, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!576 = !{!577}
!577 = distinct !{!577, !578, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!578 = distinct !{!578, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!579 = !{!577, !499, !501, !410}
!580 = !{!581, !577}
!581 = distinct !{!581, !582, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!582 = distinct !{!582, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!583 = !{!584}
!584 = distinct !{!584, !585, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!585 = distinct !{!585, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!586 = !{!584, !499, !501, !410}
!587 = !{!588, !584}
!588 = distinct !{!588, !589, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!589 = distinct !{!589, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!590 = !{!591}
!591 = distinct !{!591, !592, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!592 = distinct !{!592, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!593 = !{!594}
!594 = distinct !{!594, !595, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!595 = distinct !{!595, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!596 = !{!597}
!597 = distinct !{!597, !598, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!598 = distinct !{!598, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!599 = !{!600}
!600 = distinct !{!600, !601, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!601 = distinct !{!601, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!602 = !{!603, !605}
!603 = distinct !{!603, !604, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!604 = distinct !{!604, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!605 = distinct !{!605, !604, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!606 = !{!607, !609}
!607 = distinct !{!607, !608, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!608 = distinct !{!608, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!609 = distinct !{!609, !608, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!610 = !{!611, !613}
!611 = distinct !{!611, !612, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!612 = distinct !{!612, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!613 = distinct !{!613, !612, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!614 = !{!615}
!615 = distinct !{!615, !616, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build: %_1"}
!616 = distinct !{!616, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECslwKqnJYeWCA_18build_script_build"}
!617 = !{!615, !410}
!618 = !{!619}
!619 = distinct !{!619, !620, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceNCNvCslwKqnJYeWCA_18build_script_build4main0EBJ_: %self.0"}
!620 = distinct !{!620, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceNCNvCslwKqnJYeWCA_18build_script_build4main0EBJ_"}
!621 = !{!622, !619}
!622 = distinct !{!622, !620, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceNCNvCslwKqnJYeWCA_18build_script_build4main0EBJ_: %_0"}
!623 = !{!622}
!624 = !{!625, !627, !629, !631, !632, !634, !635, !637, !622}
!625 = distinct !{!625, !626, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECslwKqnJYeWCA_18build_script_build: %bytes"}
!626 = distinct !{!626, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECslwKqnJYeWCA_18build_script_build"}
!627 = distinct !{!627, !628, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!628 = distinct !{!628, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!629 = distinct !{!629, !630, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_: %_0"}
!630 = distinct !{!630, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_"}
!631 = distinct !{!631, !630, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher4nextB19_: %self"}
!632 = distinct !{!632, !633, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB13_: %_0"}
!633 = distinct !{!633, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB13_"}
!634 = distinct !{!634, !633, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB13_: %self"}
!635 = distinct !{!635, !636, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB1b_: %_0"}
!636 = distinct !{!636, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB1b_"}
!637 = distinct !{!637, !636, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCNvCslwKqnJYeWCA_18build_script_build4main0ENtB5_8Searcher10next_matchB1b_: %self"}
!638 = !{!639}
!639 = distinct !{!639, !640, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build: %self"}
!640 = distinct !{!640, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build"}
!641 = !{!642, !639}
!642 = distinct !{!642, !643, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build: %self"}
!643 = distinct !{!643, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build"}
!644 = !{!645}
!645 = distinct !{!645, !646, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build: %self"}
!646 = distinct !{!646, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build"}
!647 = !{!648, !645}
!648 = distinct !{!648, !649, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build: %self"}
!649 = distinct !{!649, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build"}
!650 = !{!651, !653}
!651 = distinct !{!651, !652, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build: %self"}
!652 = distinct !{!652, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCslwKqnJYeWCA_18build_script_build"}
!653 = distinct !{!653, !654, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build: %self"}
!654 = distinct !{!654, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build"}
!655 = !{!645, !622}
!656 = !{!653}
!657 = !{!653, !622}
!658 = !{!639, !622}
!659 = !{!660}
!660 = distinct !{!660, !661, !"_RNvMNtCsjMrxcFdYDNN_4core3stre20make_ascii_uppercase: %self.0"}
!661 = distinct !{!661, !"_RNvMNtCsjMrxcFdYDNN_4core3stre20make_ascii_uppercase"}
!662 = distinct !{!662, !663, !664}
!663 = !{!"llvm.loop.isvectorized", i32 1}
!664 = !{!"llvm.loop.unroll.runtime.disable"}
!665 = distinct !{!665, !663, !664}
!666 = distinct !{!666, !664, !663}
!667 = !{!668, !670}
!668 = distinct !{!668, !669, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!669 = distinct !{!669, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!670 = distinct !{!670, !669, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!671 = !{!672, !674}
!672 = distinct !{!672, !673, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!673 = distinct !{!673, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!674 = distinct !{!674, !673, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!675 = !{!676, !678}
!676 = distinct !{!676, !677, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!677 = distinct !{!677, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!678 = distinct !{!678, !677, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!679 = !{!680, !682}
!680 = distinct !{!680, !681, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!681 = distinct !{!681, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!682 = distinct !{!682, !681, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!683 = !{!684, !686}
!684 = distinct !{!684, !685, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!685 = distinct !{!685, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!686 = distinct !{!686, !685, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!687 = !{!688, !690}
!688 = distinct !{!688, !689, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!689 = distinct !{!689, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!690 = distinct !{!690, !689, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!691 = !{!692, !694}
!692 = distinct !{!692, !693, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!693 = distinct !{!693, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!694 = distinct !{!694, !693, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!695 = !{!696, !698}
!696 = distinct !{!696, !697, !"_RNvCslwKqnJYeWCA_18build_script_build27convert_custom_linux_target: %_0"}
!697 = distinct !{!697, !"_RNvCslwKqnJYeWCA_18build_script_build27convert_custom_linux_target"}
!698 = distinct !{!698, !697, !"_RNvCslwKqnJYeWCA_18build_script_build27convert_custom_linux_target: %target.0"}
!699 = !{!700}
!700 = distinct !{!700, !701, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build: %iterator"}
!701 = distinct !{!701, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build"}
!702 = !{!703, !700, !696, !698}
!703 = distinct !{!703, !701, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecReEINtB2_18SpecFromIterNestedB11_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE9from_iterCslwKqnJYeWCA_18build_script_build: %_0"}
!704 = !{!705}
!705 = distinct !{!705, !706, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!706 = distinct !{!706, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!707 = !{!708}
!708 = distinct !{!708, !709, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!709 = distinct !{!709, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!710 = !{!708, !705, !703, !700, !696, !698}
!711 = !{!708, !705, !700}
!712 = !{!703, !696, !698}
!713 = !{!714, !708, !705, !700}
!714 = distinct !{!714, !715, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!715 = distinct !{!715, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!716 = !{!717, !703, !700, !696}
!717 = distinct !{!717, !718, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build: %_0"}
!718 = distinct !{!718, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build"}
!719 = !{!703, !700, !696}
!720 = !{!721}
!721 = distinct !{!721, !722, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecReEINtB2_10SpecExtendBR_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE11spec_extendCslwKqnJYeWCA_18build_script_build: %self"}
!722 = distinct !{!722, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecReEINtB2_10SpecExtendBR_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE11spec_extendCslwKqnJYeWCA_18build_script_build"}
!723 = !{!724}
!724 = distinct !{!724, !722, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecReEINtB2_10SpecExtendBR_INtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEE11spec_extendCslwKqnJYeWCA_18build_script_build: %iter"}
!725 = !{!726}
!726 = distinct !{!726, !727, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecReE16extend_desugaredINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEECslwKqnJYeWCA_18build_script_build: %self"}
!727 = distinct !{!727, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecReE16extend_desugaredINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEECslwKqnJYeWCA_18build_script_build"}
!728 = !{!729}
!729 = distinct !{!729, !727, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecReE16extend_desugaredINtNtNtCsjMrxcFdYDNN_4core3str4iter5SplitcEECslwKqnJYeWCA_18build_script_build: %iterator"}
!730 = !{!731, !733, !729, !724}
!731 = distinct !{!731, !732, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:pre.rot"}
!732 = distinct !{!732, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!733 = distinct !{!733, !734, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:pre.rot"}
!734 = distinct !{!734, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build"}
!735 = !{!726, !721, !703, !700, !696, !698}
!736 = !{!737}
!737 = distinct !{!737, !734, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self"}
!738 = !{!739}
!739 = distinct !{!739, !732, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!740 = !{!739, !737, !729, !724}
!741 = !{!739, !737, !726, !729, !721, !724, !703, !700, !696, !698}
!742 = !{!743, !739, !737, !729, !724}
!743 = distinct !{!743, !744, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!744 = distinct !{!744, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!745 = !{!726, !721}
!746 = !{!729, !724, !703, !700, !696, !698}
!747 = !{!726, !729, !721, !724, !703, !700, !696}
!748 = !{!749, !750, !729, !724}
!749 = distinct !{!749, !732, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!750 = distinct !{!750, !734, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCslwKqnJYeWCA_18build_script_build: %self:h.rot"}
!751 = !{!700, !696, !698}
!752 = !{!696}
!753 = !{!754}
!754 = distinct !{!754, !755, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECslwKqnJYeWCA_18build_script_build: %slice.0"}
!755 = distinct !{!755, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECslwKqnJYeWCA_18build_script_build"}
!756 = !{!757, !759, !696}
!757 = distinct !{!757, !758, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterReENtNtNtNtBa_4iter6traits8iterator8Iterator8try_foldjNCINvNtNtBS_8adapters3map12map_try_foldRBJ_jjINtNtBa_6option6OptionjENCNCINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehBJ_E00NvMs9_NtBa_3numj11checked_addE0B2k_ECslwKqnJYeWCA_18build_script_build: %self"}
!758 = distinct !{!758, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterReENtNtNtNtBa_4iter6traits8iterator8Iterator8try_foldjNCINvNtNtBS_8adapters3map12map_try_foldRBJ_jjINtNtBa_6option6OptionjENCNCINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehBJ_E00NvMs9_NtBa_3numj11checked_addE0B2k_ECslwKqnJYeWCA_18build_script_build"}
!759 = distinct !{!759, !755, !"_RINvNtCsdJPVW0sQgAG_5alloc3str17join_generic_copyehReECslwKqnJYeWCA_18build_script_build: %_0"}
!760 = !{!759, !754, !696, !698}
!761 = !{!762, !759, !754, !696}
!762 = distinct !{!762, !763, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build: %_0"}
!763 = distinct !{!763, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCslwKqnJYeWCA_18build_script_build"}
!764 = !{!759, !754, !696}
!765 = !{!759, !696}
!766 = !{!767}
!767 = distinct !{!767, !768, !"_RNvXs2_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhEE11spec_extendCslwKqnJYeWCA_18build_script_build: %self"}
!768 = distinct !{!768, !"_RNvXs2_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB7_3VechEINtB5_10SpecExtendRhINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhEE11spec_extendCslwKqnJYeWCA_18build_script_build"}
!769 = !{!770}
!770 = distinct !{!770, !771, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build: %self"}
!771 = distinct !{!771, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCslwKqnJYeWCA_18build_script_build"}
!772 = !{!770, !767}
!773 = !{!770, !767, !759, !754, !696}
!774 = !{!754, !696, !698}
!775 = !{!776, !778}
!776 = distinct !{!776, !777, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build: %dest.0"}
!777 = distinct !{!777, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build"}
!778 = distinct !{!778, !777, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build: %src.0"}
!779 = !{!780, !782}
!780 = distinct !{!780, !781, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build: %dest.0"}
!781 = distinct !{!781, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build"}
!782 = distinct !{!782, !781, !"_RINvNtCsjMrxcFdYDNN_4core5slice20copy_from_slice_implINtNtNtB4_3mem12maybe_uninit11MaybeUninithEECslwKqnJYeWCA_18build_script_build: %src.0"}
!783 = !{!784, !786, !787, !789}
!784 = distinct !{!784, !785, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!785 = distinct !{!785, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!786 = distinct !{!786, !785, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!787 = distinct !{!787, !788, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!788 = distinct !{!788, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!789 = distinct !{!789, !788, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!790 = !{!791, !793, !794, !796}
!791 = distinct !{!791, !792, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!792 = distinct !{!792, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!793 = distinct !{!793, !792, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!794 = distinct !{!794, !795, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!795 = distinct !{!795, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!796 = distinct !{!796, !795, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!797 = !{!798, !800}
!798 = distinct !{!798, !799, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: %self"}
!799 = distinct !{!799, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build"}
!800 = distinct !{!800, !799, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: argument 1"}
!801 = !{!802, !804}
!802 = distinct !{!802, !803, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: %self"}
!803 = distinct !{!803, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build"}
!804 = distinct !{!804, !803, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: argument 1"}
!805 = !{!806, !808, !809, !811}
!806 = distinct !{!806, !807, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!807 = distinct !{!807, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!808 = distinct !{!808, !807, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!809 = distinct !{!809, !810, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!810 = distinct !{!810, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!811 = distinct !{!811, !810, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!812 = !{!813}
!813 = distinct !{!813, !814, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: argument 1"}
!814 = distinct !{!814, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build"}
!815 = !{!816}
!816 = distinct !{!816, !814, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECslwKqnJYeWCA_18build_script_build: %self"}
!817 = !{!818, !820, !821, !823}
!818 = distinct !{!818, !819, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!819 = distinct !{!819, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!820 = distinct !{!820, !819, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!821 = distinct !{!821, !822, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!822 = distinct !{!822, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!823 = distinct !{!823, !822, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!824 = !{!816, !813}
!825 = !{!826}
!826 = distinct !{!826, !827, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %x"}
!827 = distinct !{!827, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build"}
!828 = !{!829}
!829 = distinct !{!829, !827, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %self"}
!830 = !{!831}
!831 = distinct !{!831, !832, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build: %_1"}
!832 = distinct !{!832, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build"}
!833 = !{!834, !831, !829}
!834 = distinct !{!834, !835, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!835 = distinct !{!835, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!836 = !{!831, !829}
!837 = !{!831, !826, !829}
!838 = !{!826, !829}
!839 = !{!840}
!840 = distinct !{!840, !841, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15is_contained_in: %haystack.0"}
!841 = distinct !{!841, !"_RNvXst_NtNtCsjMrxcFdYDNN_4core3str7patternReNtB5_7Pattern15is_contained_in"}
!842 = !{!843}
!843 = distinct !{!843, !844, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match: %self"}
!844 = distinct !{!844, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match"}
!845 = !{!846, !840}
!846 = distinct !{!846, !844, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match: %_0"}
!847 = !{!848}
!848 = distinct !{!848, !849, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next: %self"}
!849 = distinct !{!849, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next"}
!850 = !{!848, !843}
!851 = !{!852, !846, !840}
!852 = distinct !{!852, !849, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next: %otherwise"}
!853 = !{!854}
!854 = distinct !{!854, !855, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!855 = distinct !{!855, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!856 = !{!852, !857, !846, !843}
!857 = distinct !{!857, !849, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next: %self:Peel0"}
!858 = !{!859, !852, !857, !846, !843}
!859 = distinct !{!859, !860, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECslwKqnJYeWCA_18build_script_build: %bytes"}
!860 = distinct !{!860, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECslwKqnJYeWCA_18build_script_build"}
!861 = !{!852, !848, !846, !843}
!862 = !{!859, !852, !848, !846, !843}
!863 = !{!864}
!864 = distinct !{!864, !865, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %self"}
!865 = distinct !{!865, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build"}
!866 = !{!867}
!867 = distinct !{!867, !865, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %haystack.0"}
!868 = !{!869}
!869 = distinct !{!869, !865, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %needle.0"}
!870 = !{!871, !867, !869, !840}
!871 = distinct !{!871, !865, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %_0"}
!872 = !{!871, !864, !869}
!873 = !{!871, !864, !867}
!874 = !{!875}
!875 = distinct !{!875, !876, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %self"}
!876 = distinct !{!876, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build"}
!877 = !{!878}
!878 = distinct !{!878, !876, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %haystack.0"}
!879 = !{!880}
!880 = distinct !{!880, !876, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %needle.0"}
!881 = !{!882, !878, !880, !840}
!882 = distinct !{!882, !876, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECslwKqnJYeWCA_18build_script_build: %_0"}
!883 = !{!882, !875, !880}
!884 = !{!882, !875, !878}
!885 = !{!886, !888}
!886 = distinct !{!886, !887, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!887 = distinct !{!887, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!888 = distinct !{!888, !887, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!889 = !{!890, !892}
!890 = distinct !{!890, !891, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!891 = distinct !{!891, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!892 = distinct !{!892, !891, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!893 = !{!894, !896}
!894 = distinct !{!894, !895, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!895 = distinct !{!895, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!896 = distinct !{!896, !895, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!897 = !{!898, !900}
!898 = distinct !{!898, !899, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!899 = distinct !{!899, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!900 = distinct !{!900, !899, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!901 = !{!902, !904}
!902 = distinct !{!902, !903, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!903 = distinct !{!903, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!904 = distinct !{!904, !903, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!905 = !{!906, !908}
!906 = distinct !{!906, !907, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!907 = distinct !{!907, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!908 = distinct !{!908, !907, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!909 = !{!910, !912}
!910 = distinct !{!910, !911, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!911 = distinct !{!911, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!912 = distinct !{!912, !911, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!913 = !{!914, !916}
!914 = distinct !{!914, !915, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!915 = distinct !{!915, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!916 = distinct !{!916, !915, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!917 = !{!918, !920}
!918 = distinct !{!918, !919, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!919 = distinct !{!919, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!920 = distinct !{!920, !919, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!921 = !{!922, !924}
!922 = distinct !{!922, !923, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!923 = distinct !{!923, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!924 = distinct !{!924, !923, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!925 = !{!926, !928}
!926 = distinct !{!926, !927, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!927 = distinct !{!927, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!928 = distinct !{!928, !927, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!929 = !{!930, !932}
!930 = distinct !{!930, !931, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!931 = distinct !{!931, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!932 = distinct !{!932, !931, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!933 = !{!934, !936}
!934 = distinct !{!934, !935, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!935 = distinct !{!935, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!936 = distinct !{!936, !935, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!937 = !{!938, !940}
!938 = distinct !{!938, !939, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!939 = distinct !{!939, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!940 = distinct !{!940, !939, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!941 = !{!942, !944}
!942 = distinct !{!942, !943, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!943 = distinct !{!943, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!944 = distinct !{!944, !943, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!945 = !{!946, !948}
!946 = distinct !{!946, !947, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!947 = distinct !{!947, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build"}
!948 = distinct !{!948, !949, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_suffix: %s.0"}
!949 = distinct !{!949, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_suffix"}
!950 = !{!951}
!951 = distinct !{!951, !947, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh9ends_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!952 = !{!953, !955}
!953 = distinct !{!953, !954, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!954 = distinct !{!954, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!955 = distinct !{!955, !954, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!956 = !{!957, !959}
!957 = distinct !{!957, !958, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!958 = distinct !{!958, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!959 = distinct !{!959, !958, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!960 = !{!961, !963}
!961 = distinct !{!961, !962, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!962 = distinct !{!962, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!963 = distinct !{!963, !962, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!964 = !{!965}
!965 = distinct !{!965, !966, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!966 = distinct !{!966, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!967 = !{!968, !970, !971, !973, !974, !965}
!968 = distinct !{!968, !969, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!969 = distinct !{!969, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!970 = distinct !{!970, !969, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!971 = distinct !{!971, !972, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!972 = distinct !{!972, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!973 = distinct !{!973, !972, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!974 = distinct !{!974, !966, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!975 = !{!976, !974}
!976 = distinct !{!976, !977, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!977 = distinct !{!977, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!978 = !{!979}
!979 = distinct !{!979, !980, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!980 = distinct !{!980, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!981 = !{!982, !984, !985, !987, !988, !979}
!982 = distinct !{!982, !983, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!983 = distinct !{!983, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!984 = distinct !{!984, !983, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!985 = distinct !{!985, !986, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!986 = distinct !{!986, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!987 = distinct !{!987, !986, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!988 = distinct !{!988, !980, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!989 = !{!990}
!990 = distinct !{!990, !991, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionReE7or_elseNCNvCslwKqnJYeWCA_18build_script_build4mains0_0EBX_: %self.0"}
!991 = distinct !{!991, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionReE7or_elseNCNvCslwKqnJYeWCA_18build_script_build4mains0_0EBX_"}
!992 = !{!993, !988}
!993 = distinct !{!993, !994, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!994 = distinct !{!994, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!995 = !{!979, !990}
!996 = !{!997}
!997 = distinct !{!997, !998, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %pat.0"}
!998 = distinct !{!998, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix"}
!999 = !{!1000, !1002, !1003, !1005, !1006, !997}
!1000 = distinct !{!1000, !1001, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1001 = distinct !{!1001, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1002 = distinct !{!1002, !1001, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1003 = distinct !{!1003, !1004, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1004 = distinct !{!1004, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1005 = distinct !{!1005, !1004, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1006 = distinct !{!1006, !998, !"_RNvCslwKqnJYeWCA_18build_script_build12strip_prefix: %s.0"}
!1007 = !{!1008, !1006}
!1008 = distinct !{!1008, !1009, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1009 = distinct !{!1009, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1010 = !{!1011}
!1011 = distinct !{!1011, !1012, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!1012 = distinct !{!1012, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!1013 = !{!1014, !1011}
!1014 = distinct !{!1014, !1015, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!1015 = distinct !{!1015, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!1016 = !{!1017}
!1017 = distinct !{!1017, !1018, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build: %self"}
!1018 = distinct !{!1018, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCslwKqnJYeWCA_18build_script_build"}
!1019 = !{!1020, !1017}
!1020 = distinct !{!1020, !1021, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!1021 = distinct !{!1021, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!1022 = !{!1023, !1025}
!1023 = distinct !{!1023, !1024, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1024 = distinct !{!1024, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1025 = distinct !{!1025, !1024, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1026 = !{!1027, !1029}
!1027 = distinct !{!1027, !1028, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1028 = distinct !{!1028, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1029 = distinct !{!1029, !1028, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1030 = !{!1031, !1033}
!1031 = distinct !{!1031, !1032, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1032 = distinct !{!1032, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1033 = distinct !{!1033, !1032, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1034 = !{!1035, !1037}
!1035 = distinct !{!1035, !1036, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1036 = distinct !{!1036, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1037 = distinct !{!1037, !1036, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1038 = !{!1039, !1041}
!1039 = distinct !{!1039, !1040, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1040 = distinct !{!1040, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1041 = distinct !{!1041, !1040, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1042 = !{!1043, !1045}
!1043 = distinct !{!1043, !1044, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1044 = distinct !{!1044, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1045 = distinct !{!1045, !1044, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1046 = !{!1047, !1049}
!1047 = distinct !{!1047, !1048, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1048 = distinct !{!1048, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1049 = distinct !{!1049, !1048, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1050 = !{!1051, !1053}
!1051 = distinct !{!1051, !1052, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1052 = distinct !{!1052, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1053 = distinct !{!1053, !1052, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1054 = !{!1055, !1057}
!1055 = distinct !{!1055, !1056, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1056 = distinct !{!1056, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1057 = distinct !{!1057, !1056, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1058 = !{!1059, !1061}
!1059 = distinct !{!1059, !1060, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1060 = distinct !{!1060, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1061 = distinct !{!1061, !1060, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1062 = !{!1063, !1065}
!1063 = distinct !{!1063, !1064, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1064 = distinct !{!1064, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1065 = distinct !{!1065, !1064, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1066 = !{!1067, !1069}
!1067 = distinct !{!1067, !1068, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1068 = distinct !{!1068, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1069 = distinct !{!1069, !1068, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1070 = !{!1071, !1073}
!1071 = distinct !{!1071, !1072, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1072 = distinct !{!1072, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1073 = distinct !{!1073, !1072, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1074 = !{!1075, !1077}
!1075 = distinct !{!1075, !1076, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1076 = distinct !{!1076, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1077 = distinct !{!1077, !1076, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1078 = !{!1079, !1081}
!1079 = distinct !{!1079, !1080, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1080 = distinct !{!1080, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1081 = distinct !{!1081, !1080, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1082 = !{!1083, !1085}
!1083 = distinct !{!1083, !1084, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1084 = distinct !{!1084, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1085 = distinct !{!1085, !1084, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1086 = !{!1087, !1089}
!1087 = distinct !{!1087, !1088, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1088 = distinct !{!1088, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1089 = distinct !{!1089, !1088, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1090 = !{!1091, !1093}
!1091 = distinct !{!1091, !1092, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1092 = distinct !{!1092, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1093 = distinct !{!1093, !1092, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1094 = !{!1095, !1097}
!1095 = distinct !{!1095, !1096, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1096 = distinct !{!1096, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1097 = distinct !{!1097, !1096, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1098 = !{!1099, !1101}
!1099 = distinct !{!1099, !1100, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1100 = distinct !{!1100, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1101 = distinct !{!1101, !1100, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1102 = !{!1103, !1105}
!1103 = distinct !{!1103, !1104, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1104 = distinct !{!1104, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1105 = distinct !{!1105, !1104, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1106 = !{!1107, !1109}
!1107 = distinct !{!1107, !1108, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1108 = distinct !{!1108, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1109 = distinct !{!1109, !1108, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1110 = !{!1111, !1113, !1114, !1116}
!1111 = distinct !{!1111, !1112, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1112 = distinct !{!1112, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1113 = distinct !{!1113, !1112, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1114 = distinct !{!1114, !1115, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1115 = distinct !{!1115, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1116 = distinct !{!1116, !1115, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1117 = !{!1118, !1120, !1121, !1123}
!1118 = distinct !{!1118, !1119, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1119 = distinct !{!1119, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1120 = distinct !{!1120, !1119, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1121 = distinct !{!1121, !1122, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1122 = distinct !{!1122, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1123 = distinct !{!1123, !1122, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1124 = !{!1125, !1127, !1128, !1130}
!1125 = distinct !{!1125, !1126, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1126 = distinct !{!1126, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1127 = distinct !{!1127, !1126, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1128 = distinct !{!1128, !1129, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1129 = distinct !{!1129, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1130 = distinct !{!1130, !1129, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1131 = !{!1132, !1134, !1135, !1137}
!1132 = distinct !{!1132, !1133, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1133 = distinct !{!1133, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1134 = distinct !{!1134, !1133, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1135 = distinct !{!1135, !1136, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1136 = distinct !{!1136, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1137 = distinct !{!1137, !1136, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1138 = !{!1139, !1141}
!1139 = distinct !{!1139, !1140, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1140 = distinct !{!1140, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1141 = distinct !{!1141, !1140, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1142 = !{!1143}
!1143 = distinct !{!1143, !1144, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %x"}
!1144 = distinct !{!1144, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build"}
!1145 = !{!1146}
!1146 = distinct !{!1146, !1144, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %self"}
!1147 = !{!1148}
!1148 = distinct !{!1148, !1149, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build: %_1"}
!1149 = distinct !{!1149, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build"}
!1150 = !{!1151, !1148, !1146}
!1151 = distinct !{!1151, !1152, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!1152 = distinct !{!1152, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!1153 = !{!1148, !1146}
!1154 = !{!1148, !1143, !1146}
!1155 = !{!1143, !1146}
!1156 = !{!1157, !1159}
!1157 = distinct !{!1157, !1158, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1158 = distinct !{!1158, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1159 = distinct !{!1159, !1158, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1160 = !{!1161}
!1161 = distinct !{!1161, !1162, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtCsdJPVW0sQgAG_5alloc6string6StringE6map_orbNCNvCslwKqnJYeWCA_18build_script_build4mains_0EB1x_: %self"}
!1162 = distinct !{!1162, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtCsdJPVW0sQgAG_5alloc6string6StringE6map_orbNCNvCslwKqnJYeWCA_18build_script_build4mains_0EB1x_"}
!1163 = !{!1164, !1166, !1167, !1169}
!1164 = distinct !{!1164, !1165, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1165 = distinct !{!1165, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1166 = distinct !{!1166, !1165, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1167 = distinct !{!1167, !1168, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %self.0"}
!1168 = distinct !{!1168, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build"}
!1169 = distinct !{!1169, !1168, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCslwKqnJYeWCA_18build_script_build: %needle.0"}
!1170 = !{!1171, !1161}
!1171 = distinct !{!1171, !1172, !"_RNCNvCslwKqnJYeWCA_18build_script_build4mains_0B3_: %cpu"}
!1172 = distinct !{!1172, !"_RNCNvCslwKqnJYeWCA_18build_script_build4mains_0B3_"}
!1173 = !{!1174}
!1174 = distinct !{!1174, !1175, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %x"}
!1175 = distinct !{!1175, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build"}
!1176 = !{!1177}
!1177 = distinct !{!1177, !1175, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE17unwrap_or_defaultCslwKqnJYeWCA_18build_script_build: %self"}
!1178 = !{!1179}
!1179 = distinct !{!1179, !1180, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build: %_1"}
!1180 = distinct !{!1180, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECslwKqnJYeWCA_18build_script_build"}
!1181 = !{!1182, !1179, !1177}
!1182 = distinct !{!1182, !1183, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!1183 = distinct !{!1183, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!1184 = !{!1179, !1177}
!1185 = !{!1179, !1174, !1177}
!1186 = !{!1174, !1177}
!1187 = !{!1188, !1190}
!1188 = distinct !{!1188, !1189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1189 = distinct !{!1189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1190 = distinct !{!1190, !1189, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
!1191 = !{!1192}
!1192 = distinct !{!1192, !1193, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build: %_1"}
!1193 = distinct !{!1193, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECslwKqnJYeWCA_18build_script_build"}
!1194 = !{!1195}
!1195 = distinct !{!1195, !1196, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build: %self"}
!1196 = distinct !{!1196, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCslwKqnJYeWCA_18build_script_build"}
!1197 = !{!1198}
!1198 = distinct !{!1198, !1199, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %self"}
!1199 = distinct !{!1199, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build"}
!1200 = !{!1201}
!1201 = distinct !{!1201, !1202, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCslwKqnJYeWCA_18build_script_build: %self"}
!1202 = distinct !{!1202, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCslwKqnJYeWCA_18build_script_build"}
!1203 = !{!1201, !1198}
!1204 = !{!1205}
!1205 = distinct !{!1205, !1202, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCslwKqnJYeWCA_18build_script_build: %_0"}
!1206 = !{!1205, !1201, !1198}
!1207 = !{!1208, !1210, !1198}
!1208 = distinct !{!1208, !1209, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCslwKqnJYeWCA_18build_script_build: %_0"}
!1209 = distinct !{!1209, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCslwKqnJYeWCA_18build_script_build"}
!1210 = distinct !{!1210, !1211, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %ret"}
!1211 = distinct !{!1211, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build"}
!1212 = !{!1210, !1198}
!1213 = !{!1214}
!1214 = distinct !{!1214, !1215, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %self"}
!1215 = distinct !{!1215, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build"}
!1216 = !{!1217}
!1217 = distinct !{!1217, !1218, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCslwKqnJYeWCA_18build_script_build: %self"}
!1218 = distinct !{!1218, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCslwKqnJYeWCA_18build_script_build"}
!1219 = !{!1217, !1214}
!1220 = !{!1221}
!1221 = distinct !{!1221, !1215, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %_0"}
!1222 = !{!1223, !1214}
!1223 = distinct !{!1223, !1224, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECslwKqnJYeWCA_18build_script_build: %v"}
!1224 = distinct !{!1224, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECslwKqnJYeWCA_18build_script_build"}
!1225 = !{!1226, !1221}
!1226 = distinct !{!1226, !1224, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECslwKqnJYeWCA_18build_script_build: %ret"}
!1227 = !{!1217, !1221, !1214}
!1228 = !{!1223}
!1229 = !{!1230, !1232, !1233, !1235, !1226, !1223, !1221, !1214}
!1230 = distinct !{!1230, !1231, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %_0"}
!1231 = distinct !{!1231, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build"}
!1232 = distinct !{!1232, !1231, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %self"}
!1233 = distinct !{!1233, !1234, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CslwKqnJYeWCA_18build_script_build: %val"}
!1234 = distinct !{!1234, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CslwKqnJYeWCA_18build_script_build"}
!1235 = distinct !{!1235, !1234, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CslwKqnJYeWCA_18build_script_build: %leaf_edge"}
!1236 = !{!1237, !1239, !1230, !1232, !1233, !1235, !1226, !1223, !1221, !1214}
!1237 = distinct !{!1237, !1238, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCslwKqnJYeWCA_18build_script_build: %_0"}
!1238 = distinct !{!1238, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCslwKqnJYeWCA_18build_script_build"}
!1239 = distinct !{!1239, !1240, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build: %ret"}
!1240 = distinct !{!1240, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECslwKqnJYeWCA_18build_script_build"}
!1241 = !{!1242, !1244, !1230, !1232, !1233, !1235, !1226, !1223, !1221, !1214}
!1242 = distinct !{!1242, !1243, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCslwKqnJYeWCA_18build_script_build: %_0"}
!1243 = distinct !{!1243, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCslwKqnJYeWCA_18build_script_build"}
!1244 = distinct !{!1244, !1243, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCslwKqnJYeWCA_18build_script_build: %self"}
!1245 = !{!1239, !1230, !1232, !1233, !1235, !1226, !1223, !1221, !1214}
!1246 = !{!1226, !1223, !1221, !1214}
!1247 = !{!1221, !1214}
!1248 = !{!"branch_weights", i32 4000000, i32 4001}
!1249 = !{!1250}
!1250 = distinct !{!1250, !1251, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!1251 = distinct !{!1251, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!1252 = !{!1253, !1255}
!1253 = distinct !{!1253, !1254, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %self.0"}
!1254 = distinct !{!1254, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build"}
!1255 = distinct !{!1255, !1254, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCslwKqnJYeWCA_18build_script_build: %other.0"}
