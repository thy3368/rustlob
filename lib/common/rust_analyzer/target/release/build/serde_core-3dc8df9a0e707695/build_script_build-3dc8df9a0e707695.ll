; ModuleID = 'build_script_build.c97b3b305a7f618a-cgu.0'
source_filename = "build_script_build.c97b3b305a7f618a-cgu.0"
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
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCshitNtYJEXnW_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CshitNtYJEXnW_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CshitNtYJEXnW_18build_script_build }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_a887f9858119cc7413062dc002c4d9ab = private unnamed_addr constant [9 x i8] c"--version", align 1
@alloc_ca36d7e792bb4bbd1a68749f90007ce8 = private unnamed_addr constant [7 x i8] c"rustc 1", align 1
@alloc_742f06589122110502429e832b81e8bd = private unnamed_addr constant [32 x i8] c"cargo:rerun-if-changed=build.rs\0A", align 1
@alloc_ebcdb5f66b6f511cde89ece546cbdd6d = private unnamed_addr constant [7 x i8] c"OUT_DIR", align 1
@alloc_b778b1d91909edbc73f307a98fdd149a = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_core-1.0.228/build.rs\00", align 1
@alloc_5da8f4824d8fe348ca85a158334c7f7b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b778b1d91909edbc73f307a98fdd149a, [16 x i8] c"c\00\00\00\00\00\00\00\15\00\00\008\00\00\00" }>, align 8
@alloc_e0596ffb4aa18b25c9e2716c2c3baf39 = private unnamed_addr constant [23 x i8] c"CARGO_PKG_VERSION_PATCH", align 1
@alloc_898e0353ee0a600f00e4e151a27db227 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b778b1d91909edbc73f307a98fdd149a, [16 x i8] c"c\00\00\00\00\00\00\00\16\00\00\00=\00\00\00" }>, align 8
@alloc_a98ee8a049a1c25292c235225752f6c8 = private unnamed_addr constant [89 x i8] c"#[doc(hidden)]\0Apub mod __private$$ {\0A    #[doc(hidden)]\0A    pub use crate::private::*;\0A}\0A", align 1
@alloc_cb0b3bff64b3ed8f2f2081deb871df05 = private unnamed_addr constant [2 x i8] c"$$", align 1
@alloc_589d53fa4508e5cc88338e60f7602ef2 = private unnamed_addr constant [10 x i8] c"private.rs", align 1
@alloc_d7a955ea474cc242b6aa56db3ecaf3a9 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b778b1d91909edbc73f307a98fdd149a, [16 x i8] c"c\00\00\00\00\00\00\00\18\00\00\003\00\00\00" }>, align 8
@alloc_bc9e042c014e8fa11bdfddb773364e8c = private unnamed_addr constant [56 x i8] c"cargo:rustc-check-cfg=cfg(if_docsrs_then_no_serde_core)\0A", align 1
@alloc_38a8aab5c2b62e9751be68fe1ccf5613 = private unnamed_addr constant [40 x i8] c"cargo:rustc-check-cfg=cfg(no_core_cstr)\0A", align 1
@alloc_68978439aeb0698832af016bfe1612cd = private unnamed_addr constant [41 x i8] c"cargo:rustc-check-cfg=cfg(no_core_error)\0A", align 1
@alloc_4c52a9725234699023cc0b6fe3df2bc7 = private unnamed_addr constant [39 x i8] c"cargo:rustc-check-cfg=cfg(no_core_net)\0A", align 1
@alloc_aa136c7cc6a3cdf54ac82998fd4794de = private unnamed_addr constant [50 x i8] c"cargo:rustc-check-cfg=cfg(no_core_num_saturating)\0A", align 1
@alloc_7bab126fe70079b94cb91837d5266928 = private unnamed_addr constant [51 x i8] c"cargo:rustc-check-cfg=cfg(no_diagnostic_namespace)\0A", align 1
@alloc_cf61cd25720d051afd7dbe390badc285 = private unnamed_addr constant [43 x i8] c"cargo:rustc-check-cfg=cfg(no_serde_derive)\0A", align 1
@alloc_0bc122f91da9d875d8b43d249e797349 = private unnamed_addr constant [41 x i8] c"cargo:rustc-check-cfg=cfg(no_std_atomic)\0A", align 1
@alloc_11b9a1cc79cd309ab8e05e189e755542 = private unnamed_addr constant [43 x i8] c"cargo:rustc-check-cfg=cfg(no_std_atomic64)\0A", align 1
@alloc_1979714a80cbc333f412aa0c87203b40 = private unnamed_addr constant [48 x i8] c"cargo:rustc-check-cfg=cfg(no_target_has_atomic)\0A", align 1
@alloc_dcbc225a8ec7dbfaaef714ff8a7176fb = private unnamed_addr constant [6 x i8] c"TARGET", align 1
@alloc_f632b4be46da6b0210f55a14255fdbb7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b778b1d91909edbc73f307a98fdd149a, [16 x i8] c"c\00\00\00\00\00\00\00,\00\00\00%\00\00\00" }>, align 8
@alloc_48f3a6bb4813fb278bc6f6204d79600d = private unnamed_addr constant [24 x i8] c"asmjs-unknown-emscripten", align 1
@alloc_6aff5e31c3101a31407cfa339159573c = private unnamed_addr constant [25 x i8] c"wasm32-unknown-emscripten", align 1
@alloc_aee7a68eb29be7f2c6b56df8eb17cb8f = private unnamed_addr constant [37 x i8] c"cargo:rustc-cfg=no_target_has_atomic\0A", align 1
@alloc_4a29a4faa0904cd7ff982831f2813e90 = private unnamed_addr constant [6 x i8] c"x86_64", align 1
@alloc_bd1f56fe9bb71b27893bc9d878ecf35c = private unnamed_addr constant [4 x i8] c"i686", align 1
@alloc_a1239a7299ff261e47dc48b0b02c8ca1 = private unnamed_addr constant [7 x i8] c"aarch64", align 1
@alloc_fa1130f2f45123ef906740f12b430906 = private unnamed_addr constant [9 x i8] c"powerpc64", align 1
@alloc_7397f20c1cb53576ae7e84fd06ee7a1e = private unnamed_addr constant [7 x i8] c"sparc64", align 1
@alloc_72e190fc6a3e093db1bf5535129d517c = private unnamed_addr constant [8 x i8] c"mips64el", align 1
@alloc_8f8dc58223e03021a07a335aedb98959 = private unnamed_addr constant [7 x i8] c"riscv64", align 1
@alloc_13e1e7fbee4ecb7b0b0c4fa67467351d = private unnamed_addr constant [32 x i8] c"cargo:rustc-cfg=no_std_atomic64\0A", align 1
@alloc_2d02ffc40637a25bbd278af8d6191a7f = private unnamed_addr constant [30 x i8] c"cargo:rustc-cfg=no_std_atomic\0A", align 1
@alloc_7f378ac6f88a3c5251674bf739a6a81b = private unnamed_addr constant [32 x i8] c"cargo:rustc-cfg=no_serde_derive\0A", align 1
@alloc_f80a9e92383c9e0ada01c4be923fcd98 = private unnamed_addr constant [29 x i8] c"cargo:rustc-cfg=no_core_cstr\0A", align 1
@alloc_214341b8cbf974b5fa4df5e3a1822866 = private unnamed_addr constant [39 x i8] c"cargo:rustc-cfg=no_core_num_saturating\0A", align 1
@alloc_17c7a5f1c85e31bcbd0baf8c45cbf4c9 = private unnamed_addr constant [28 x i8] c"cargo:rustc-cfg=no_core_net\0A", align 1
@alloc_c2f517cef8b40692abde0001fd67d43c = private unnamed_addr constant [40 x i8] c"cargo:rustc-cfg=no_diagnostic_namespace\0A", align 1
@alloc_9dcee10c03036d5ac2dd73d4efbb97a5 = private unnamed_addr constant [30 x i8] c"cargo:rustc-cfg=no_core_error\0A", align 1
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsk_NtCs5sEH5CPMdak_3std3envNtB5_8VarErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECshitNtYJEXnW_18build_script_build, [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8
@alloc_1c5ece773fe9d8a26ac674de79674b77 = private unnamed_addr constant [10 x i8] c"NotPresent", align 1
@vtable.5 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtB6_5Debug3fmtCshitNtYJEXnW_18build_script_build }>, align 8
@alloc_19adf04fb909e90136daf37b5ff22508 = private unnamed_addr constant [10 x i8] c"NotUnicode", align 1
@alloc_559c4f386b668c946885822cef1a587d = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00h\04\00\00$\00\00\00" }>, align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECshitNtYJEXnW_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
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
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i ], [ 0, %start ]
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
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #22, !noalias !4
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i

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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #22, !noalias !4
  br label %bb4.i.i.preheader

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #23
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !4

terminate.i.i:                                    ; preds = %bb3.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24, !noalias !4
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %25 = icmp eq i64 %_1.val2, 0
  br i1 %25, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECshitNtYJEXnW_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECshitNtYJEXnW_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECshitNtYJEXnW_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECshitNtYJEXnW_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCshitNtYJEXnW_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #22
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCshitNtYJEXnW_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCshitNtYJEXnW_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #22
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<std::env::VarError>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !10, !noundef !3
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb1
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %bb4, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i, %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2 = load i64, ptr %2, align 8
  %3 = icmp eq i64 %.val2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECshitNtYJEXnW_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECshitNtYJEXnW_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECshitNtYJEXnW_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !11, !nonnull !3, !noundef !3
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
  %_1.val1.i.i = load ptr, ptr %5, align 8, !alias.scope !14, !nonnull !3, !noundef !3
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
  %_1.val3.i.i = load ptr, ptr %7, align 8, !alias.scope !14, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i5.i.i = shl nuw i64 %_1.val2.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3.i.i, i64 noundef %alloc_size.i.i.i.i5.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #22
  br label %bb19.i

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #23
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
          to label %bb18.i unwind label %cleanup2.i

bb9.i:                                            ; preds = %cleanup2.i, %bb10.i
  %.pn10.i = phi { ptr, i32 } [ %14, %cleanup2.i ], [ %3, %bb10.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val27.i = load ptr, ptr %10, align 8, !alias.scope !11, !align !17, !noundef !3
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
  %.val31.i = load ptr, ptr %15, align 8, !alias.scope !11, !align !17, !noundef !3
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
  %.val25.i = load ptr, ptr %19, align 8, !alias.scope !11, !align !17, !noundef !3
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
  %.val29.i = load ptr, ptr %23, align 8, !alias.scope !11, !align !17, !noundef !3
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #23
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
          to label %bb15.i unwind label %cleanup5.i

bb6.i:                                            ; preds = %cleanup5.i, %bb7.i
  %.pn16.i = phi { ptr, i32 } [ %34, %cleanup5.i ], [ %.pn10.i, %bb7.i ]
  %29 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val33.i = load ptr, ptr %29, align 8, !alias.scope !11, !align !18, !noundef !3
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
  %.val35.i = load ptr, ptr %35, align 8, !alias.scope !11, !align !18, !noundef !3
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
  %.val41.i = load i32, ptr %40, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i.i = icmp eq i32 %.val41.i, 3
  br i1 %cond.i.i, label %bb2.i.i.i, label %bb4.i

bb2.i.i.i:                                        ; preds = %bb5.i
  %41 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val42.i = load i32, ptr %41, align 4, !alias.scope !11
  %_5.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val42.i) #22
  br label %bb4.i

bb14.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i, %bb15.i
  %42 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val47.i = load i32, ptr %42, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i68.i = icmp eq i32 %.val47.i, 3
  br i1 %cond.i68.i, label %bb2.i.i70.i, label %bb13.i

bb2.i.i70.i:                                      ; preds = %bb14.i
  %43 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val48.i = load i32, ptr %43, align 4, !alias.scope !11
  %_5.i.i.i.i.i71.i = tail call noundef i32 @close(i32 noundef %.val48.i) #22
  br label %bb13.i

bb4.i:                                            ; preds = %bb2.i.i.i, %bb5.i
  %44 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val39.i = load i32, ptr %44, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i73.i = icmp eq i32 %.val39.i, 3
  br i1 %cond.i73.i, label %bb2.i.i75.i, label %bb3.i

bb2.i.i75.i:                                      ; preds = %bb4.i
  %45 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val40.i = load i32, ptr %45, align 4, !alias.scope !11
  %_5.i.i.i.i.i76.i = tail call noundef i32 @close(i32 noundef %.val40.i) #22
  br label %bb3.i

bb13.i:                                           ; preds = %bb2.i.i70.i, %bb14.i
  %46 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val45.i = load i32, ptr %46, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i78.i = icmp eq i32 %.val45.i, 3
  br i1 %cond.i78.i, label %bb2.i.i80.i, label %bb12.i

bb2.i.i80.i:                                      ; preds = %bb13.i
  %47 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val46.i = load i32, ptr %47, align 4, !alias.scope !11
  %_5.i.i.i.i.i81.i = tail call noundef i32 @close(i32 noundef %.val46.i) #22
  br label %bb12.i

bb3.i:                                            ; preds = %bb2.i.i75.i, %bb4.i
  %48 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val37.i = load i32, ptr %48, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i83.i = icmp eq i32 %.val37.i, 3
  br i1 %cond.i83.i, label %bb2.i.i85.i, label %bb1.i

bb2.i.i85.i:                                      ; preds = %bb3.i
  %49 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val38.i = load i32, ptr %49, align 4, !alias.scope !11
  %_5.i.i.i.i.i86.i = tail call noundef i32 @close(i32 noundef %.val38.i) #22
  br label %bb1.i

bb12.i:                                           ; preds = %bb2.i.i80.i, %bb13.i
  %50 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val43.i = load i32, ptr %50, align 8, !range !19, !alias.scope !11, !noundef !3
  %cond.i88.i = icmp eq i32 %.val43.i, 3
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECshitNtYJEXnW_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !11
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECshitNtYJEXnW_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECshitNtYJEXnW_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::io::error::Error>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECshitNtYJEXnW_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
  %bits.i.i.i = ptrtoint ptr %_1.val to i64
  %_5.i.i.i = and i64 %bits.i.i.i, 3
  %switch.i.i = icmp eq i64 %_5.i.i.i, 1
  br i1 %switch.i.i, label %bb2.i2.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECshitNtYJEXnW_18build_script_build.exit, !prof !20

bb2.i2.i.i:                                       ; preds = %start
  %0 = getelementptr i8, ptr %_1.val, i64 -1
  %1 = icmp ne ptr %0, null
  tail call void @llvm.assume(i1 %1)
  %_6.val.i.i.i.i = load ptr, ptr %0, align 8
  %2 = getelementptr i8, ptr %_1.val, i64 7
  %_6.val1.i.i.i.i = load ptr, ptr %2, align 8, !nonnull !3, !align !7, !noundef !3
  %3 = load ptr, ptr %_6.val1.i.i.i.i, align 8, !invariant.load !3
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
  %7 = load i64, ptr %6, align 8, !range !8, !invariant.load !3
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %9 = load i64, ptr %8, align 8, !range !9, !invariant.load !3
  %10 = add i64 %9, -1
  %11 = icmp sgt i64 %10, -1
  tail call void @llvm.assume(i1 %11)
  %12 = icmp eq i64 %7, 0
  br i1 %12, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) %9) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i

cleanup.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %15 = load i64, ptr %14, align 8, !range !8, !invariant.load !3
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %17 = load i64, ptr %16, align 8, !range !9, !invariant.load !3
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

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 24, i64 noundef 8) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECshitNtYJEXnW_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECshitNtYJEXnW_18build_script_build.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !3
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %key.val1.i.i.i.i.i, i64 noundef %key.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !28
  br label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i
  %self1.val.i.i.i.i.i.i.i = load i64, ptr %_17.i.i.i.i.i, align 8, !range !10, !noalias !28, !noundef !3
  switch i64 %self1.val.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb8.i.i.i.i.i
  %6 = getelementptr i8, ptr %_17.i.i.i.i.i, i64 8
  %self1.val2.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !28, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self1.val2.i.i.i.i.i.i.i, i64 noundef %self1.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !28
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i, %bb8.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !27
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !28, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !28
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !27
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshitNtYJEXnW_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #22, !srcloc !33
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional) unnamed_addr #3 personality ptr @rust_eh_personality {
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
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !34
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !37, !noalias !34, !noundef !3
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !10, !noalias !34, !noundef !3
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !34
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #25
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !34, !nonnull !3, !noundef !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !34
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !34
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !34
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CshitNtYJEXnW_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #4 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshitNtYJEXnW_18build_script_build(ptr noundef nonnull %_4) #26
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCshitNtYJEXnW_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECshitNtYJEXnW_18build_script_build(ptr noundef nonnull readonly %0) #26, !noalias !38
  ret i32 0
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCshitNtYJEXnW_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i.i = alloca [200 x i8], align 8
  %pieces.i = alloca [72 x i8], align 8
  %_14.i = alloca [24 x i8], align 8
  %_10.i = alloca [200 x i8], align 8
  %_7.i = alloca [56 x i8], align 8
  %output.i = alloca [56 x i8], align 8
  %_2.i66 = alloca [24 x i8], align 8
  %_98.i = alloca [104 x i8], align 8
  %result.i = alloca [24 x i8], align 8
  %e.i34 = alloca [8 x i8], align 8
  %e.i25 = alloca [24 x i8], align 8
  %e.i = alloca [24 x i8], align 8
  %_40 = alloca [32 x i8], align 8
  %_12 = alloca [24 x i8], align 8
  %_7 = alloca [32 x i8], align 8
  %_5 = alloca [24 x i8], align 8
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_742f06589122110502429e832b81e8bd, ptr noundef nonnull inttoptr (i64 65 to ptr))
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ebcdb5f66b6f511cde89ece546cbdd6d, i64 noundef 7)
  %0 = load i64, ptr %_5, align 8, !range !10, !noundef !3
  %.not12 = icmp eq i64 %0, -9223372036854775808
  br i1 %.not12, label %bb81, label %bb82, !prof !41

bb82:                                             ; preds = %start
  %_4.sroa.4.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_4.sroa.4.0.copyload = load ptr, ptr %_4.sroa.4.0._5.sroa_idx, align 8
  %_4.sroa.5.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %_4.sroa.5.0.copyload = load i64, ptr %_4.sroa.5.0._5.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_7)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e0596ffb4aa18b25c9e2716c2c3baf39, i64 noundef 23)
          to label %bb3 unwind label %cleanup

bb81:                                             ; preds = %start
; call core::option::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5da8f4824d8fe348ca85a158334c7f7b) #27
  unreachable

bb77:                                             ; preds = %bb2.i.i.i4.i.i, %bb76, %cleanup.i28, %cleanup.i28, %bb2.i.i.i4.i.i.i.i57, %cleanup
  %.pn23 = phi { ptr, i32 } [ %3, %cleanup ], [ %6, %bb2.i.i.i4.i.i.i.i57 ], [ %6, %cleanup.i28 ], [ %6, %cleanup.i28 ], [ %.pn.pn, %bb76 ], [ %.pn.pn, %bb2.i.i.i4.i.i ]
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %bb78, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb77
  %2 = icmp ne ptr %_4.sroa.4.0.copyload, null
  call void @llvm.assume(i1 %2)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.4.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb78

cleanup:                                          ; preds = %bb82
  %3 = landingpad { ptr, i32 }
          cleanup
  br label %bb77

bb3:                                              ; preds = %bb82
  call void @llvm.experimental.noalias.scope.decl(metadata !42)
  call void @llvm.experimental.noalias.scope.decl(metadata !45)
  %_2.i26 = load i64, ptr %_7, align 8, !range !37, !alias.scope !45, !noalias !47, !noundef !3
  %4 = trunc nuw i64 %_2.i26 to i1
  br i1 %4, label %bb2.i27, label %bb4, !prof !41

bb2.i27:                                          ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i25), !noalias !49
  %5 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e.i25, ptr noundef nonnull readonly align 8 dereferenceable(24) %5, i64 24, i1 false), !noalias !47
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i25, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_898e0353ee0a600f00e4e151a27db227) #25
          to label %unreachable.i31 unwind label %cleanup.i28, !noalias !50

cleanup.i28:                                      ; preds = %bb2.i27
  %6 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !51)
  %7 = load i64, ptr %e.i25, align 8, !range !10, !alias.scope !51, !noalias !50, !noundef !3
  switch i64 %7, label %bb2.i.i.i4.i.i.i.i57 [
    i64 -9223372036854775808, label %bb77
    i64 0, label %bb77
  ]

bb2.i.i.i4.i.i.i.i57:                             ; preds = %cleanup.i28
  %8 = getelementptr inbounds nuw i8, ptr %e.i25, i64 8
  %_1.val1.i = load ptr, ptr %8, align 8, !alias.scope !51, !noalias !50, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !54
  br label %bb77

unreachable.i31:                                  ; preds = %bb2.i27
  unreachable

bb4:                                              ; preds = %bb3
  %9 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  %patch_version.sroa.0.0.copyload = load i64, ptr %9, align 8, !alias.scope !50, !noalias !55
  %patch_version.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 16
  %patch_version.sroa.7.0.copyload = load ptr, ptr %patch_version.sroa.7.0..sroa_idx, align 8, !alias.scope !50, !noalias !55, !nonnull !3, !noundef !3
  %patch_version.sroa.11.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_7, i64 24
  %patch_version.sroa.11.0.copyload = load i64, ptr %patch_version.sroa.11.0..sroa_idx, align 8, !alias.scope !50, !noalias !55
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_7)
  %_28.not.i = icmp ult i64 %patch_version.sroa.11.0.copyload, 2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %result.i), !noalias !56
  br i1 %_28.not.i, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCshitNtYJEXnW_18build_script_build.exit.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i: ; preds = %bb4
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22, !noalias !60
; call __rustc::__rust_alloc
  %10 = call noundef dereferenceable_or_null(89) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 89, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !60
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb3.i.i, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCshitNtYJEXnW_18build_script_build.exit.i

bb3.i.i:                                          ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 89) #25
          to label %.noexc unwind label %cleanup1

.noexc:                                           ; preds = %bb3.i.i
  unreachable

_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCshitNtYJEXnW_18build_script_build.exit.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i, %bb4
  %self2.i.i41.i.peel = phi i64 [ 89, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ 0, %bb4 ]
  %_4.sroa.10.0.i.i = phi ptr [ %10, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ inttoptr (i64 1 to ptr), %bb4 ]
  store i64 %self2.i.i41.i.peel, ptr %result.i, align 8, !noalias !56
  %_94.sroa.4.0.result.sroa_idx.i = getelementptr inbounds nuw i8, ptr %result.i, i64 8
  store ptr %_4.sroa.10.0.i.i, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !noalias !56
  %_94.sroa.5.0.result.sroa_idx.i = getelementptr inbounds nuw i8, ptr %result.i, i64 16
  store i64 0, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !noalias !56
  call void @llvm.lifetime.start.p0(i64 104, ptr nonnull %_98.i), !noalias !56
; invoke <core::str::pattern::StrSearcher>::new
  invoke void @_RNvMsu_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcher3new(ptr noalias noundef nonnull sret([104 x i8]) align 8 captures(none) dereferenceable(104) %_98.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a98ee8a049a1c25292c235225752f6c8, i64 noundef 89, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cb0b3bff64b3ed8f2f2081deb871df05, i64 noundef 2)
          to label %bb40.i unwind label %cleanup9.i, !noalias !56

bb25.i:                                           ; preds = %cleanup10.loopexit.split.i.loopexit, %cleanup10.loopexit.split.i.loopexit.split-lp, %cleanup10.loopexit.split-lp.i, %cleanup10.loopexit.split.us.i, %cleanup9.i
  %.pn.i = phi { ptr, i32 } [ %13, %cleanup9.i ], [ %lpad.loopexit.split-lp.i, %cleanup10.loopexit.split-lp.i ], [ %lpad.loopexit.us.i, %cleanup10.loopexit.split.us.i ], [ %lpad.loopexit, %cleanup10.loopexit.split.i.loopexit ], [ %lpad.loopexit.split-lp, %cleanup10.loopexit.split.i.loopexit.split-lp ]
  %result.val.i = load i64, ptr %result.i, align 8, !noalias !56
  %12 = icmp eq i64 %result.val.i, 0
  br i1 %12, label %bb76, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb25.i
  %result.val32.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !noalias !56, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %result.val32.i, i64 noundef %result.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !56
  br label %bb76

cleanup9.i:                                       ; preds = %bb1.i.i.i, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCshitNtYJEXnW_18build_script_build.exit.i
  %13 = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

bb40.i:                                           ; preds = %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCshitNtYJEXnW_18build_script_build.exit.i
  %_97.sroa.0.0.copyload.i = load i64, ptr %_98.i, align 8, !noalias !56
  %_97.sroa.4.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 8
  %_97.sroa.4.0.copyload.i = load i64, ptr %_97.sroa.4.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.6.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 24
  %_97.sroa.6.0.copyload.i = load i64, ptr %_97.sroa.6.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.7.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 32
  %_97.sroa.7.0.copyload.i = load i64, ptr %_97.sroa.7.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.8.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 40
  %_97.sroa.8.0.copyload.i = load i64, ptr %_97.sroa.8.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.10.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 56
  %_97.sroa.10.0.copyload.i = load i64, ptr %_97.sroa.10.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.12.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 72
  %_97.sroa.12.0.copyload.i = load ptr, ptr %_97.sroa.12.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.13.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 80
  %_97.sroa.13.0.copyload.i = load i64, ptr %_97.sroa.13.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.14.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 88
  %_97.sroa.14.0.copyload.i = load ptr, ptr %_97.sroa.14.0._98.sroa_idx.i, align 8, !noalias !56
  %_97.sroa.15.0._98.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_98.i, i64 96
  %_97.sroa.15.0.copyload.i = load i64, ptr %_97.sroa.15.0._98.sroa_idx.i, align 8, !noalias !56
  call void @llvm.lifetime.end.p0(i64 104, ptr nonnull %_98.i), !noalias !56
  %14 = trunc nuw i64 %_97.sroa.0.0.copyload.i to i1
  %15 = icmp ne ptr %_97.sroa.12.0.copyload.i, null
  %_48.i.i.i = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_97.sroa.13.0.copyload.i
  %16 = icmp ne ptr %_97.sroa.14.0.copyload.i, null
  %needle_last.i71.i = add nsw i64 %_97.sroa.15.0.copyload.i, -1
  br i1 %14, label %bb40.split.us.i, label %bb40.split.i

bb40.split.us.i:                                  ; preds = %bb40.i
  call void @llvm.assume(i1 %15)
  call void @llvm.assume(i1 %16)
  %17 = sub i64 %_97.sroa.15.0.copyload.i, %_97.sroa.6.0.copyload.i
  %umax40.i.us.i = call i64 @llvm.umax.i64(i64 %_97.sroa.4.0.copyload.i, i64 range(i64 0, -9223372036854775808) %_97.sroa.15.0.copyload.i)
  %18 = add i64 %_97.sroa.4.0.copyload.i, -1
  %_43.i.us.first_iter.i = icmp ult i64 %18, %_97.sroa.15.0.copyload.i
  %_43.i.us.first_iter.i.fr = freeze i1 %_43.i.us.first_iter.i
  %_70.i.not.us.i = icmp eq i64 %_97.sroa.4.0.copyload.i, 0
  br label %bb20.us.i

bb20.us.i:                                        ; preds = %bb50.us.i, %bb40.split.us.i
  %_10.i58.us162.i = phi ptr [ %_4.sroa.10.0.i.i, %bb40.split.us.i ], [ %_10.i58.us.i, %bb50.us.i ]
  %len.i.i40.us.i = phi i64 [ 0, %bb40.split.us.i ], [ %41, %bb50.us.i ]
  %iter.sroa.18.0.us.i = phi i64 [ %_97.sroa.8.0.copyload.i, %bb40.split.us.i ], [ %iter.sroa.18.3.us.i, %bb50.us.i ]
  %iter.sroa.306.0.us.i = phi i64 [ %_97.sroa.10.0.copyload.i, %bb40.split.us.i ], [ %iter.sroa.306.3.us.i, %bb50.us.i ]
  %last_end.sroa.0.0.us.i = phi i64 [ 0, %bb40.split.us.i ], [ %iter.sroa.18.3.us.i, %bb50.us.i ]
  %is_long.i.us.i = icmp eq i64 %iter.sroa.306.0.us.i, -1
  %index24.i.us.i = add i64 %iter.sroa.18.0.us.i, %needle_last.i71.i
  %_5325.i.us.i = icmp ult i64 %index24.i.us.i, %_97.sroa.13.0.copyload.i
  br i1 %is_long.i.us.i, label %bb8.i.us.i, label %bb9.i.us.i

bb9.i.us.i:                                       ; preds = %bb20.us.i
  call void @llvm.experimental.noalias.scope.decl(metadata !63)
  call void @llvm.experimental.noalias.scope.decl(metadata !66)
  br i1 %_5325.i.us.i, label %bb39.i82.us.i, label %bb56.i

bb39.i82.us.i:                                    ; preds = %bb9.i.us.i, %bb37.sink.split.i.us.i
  %v229.i83.us.i = phi i64 [ %.sink.i.us.i, %bb37.sink.split.i.us.i ], [ %iter.sroa.306.0.us.i, %bb9.i.us.i ]
  %index26.i84.us.i = phi i64 [ %index.i109.us.i, %bb37.sink.split.i.us.i ], [ %index24.i.us.i, %bb9.i.us.i ]
  %19 = phi i64 [ %.ph.i.us.i, %bb37.sink.split.i.us.i ], [ %iter.sroa.18.0.us.i, %bb9.i.us.i ]
  %_55.i85.us.i = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %index26.i84.us.i
  %tail_byte.i86.us.i = load i8, ptr %_55.i85.us.i, align 1, !alias.scope !63, !noalias !68, !noundef !3
  %_60.i87.us.i = and i8 %tail_byte.i86.us.i, 63
  %_59.i88.us.i = zext nneg i8 %_60.i87.us.i to i64
  %20 = shl nuw i64 1, %_59.i88.us.i
  %21 = and i64 %20, %_97.sroa.7.0.copyload.i
  %22 = icmp eq i64 %21, 0
  br i1 %22, label %bb10.i129.us.i, label %bb9.i89.us.i

bb9.i89.us.i:                                     ; preds = %bb39.i82.us.i
  %_0.sroa.0.0.i.i90.us.i = call i64 @llvm.umax.i64(i64 %v229.i83.us.i, i64 %_97.sroa.4.0.copyload.i)
  %umax40.i91.us.i = call i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i90.us.i, i64 range(i64 0, -9223372036854775808) %_97.sroa.15.0.copyload.i)
  br label %bb16.i92.us.i

bb16.i92.us.i:                                    ; preds = %bb18.i100.us.i, %bb9.i89.us.i
  %iter.sroa.0.0.i93.us.i = phi i64 [ %_0.sroa.0.0.i.i90.us.i, %bb9.i89.us.i ], [ %_65.i101.us.i, %bb18.i100.us.i ]
  %exitcond.not.i94.us.i = icmp eq i64 %iter.sroa.0.0.i93.us.i, %umax40.i91.us.i
  br i1 %exitcond.not.i94.us.i, label %bb26.i112.us.i, label %bb42.i95.us.i

bb42.i95.us.i:                                    ; preds = %bb16.i92.us.i
  %_28.i96.us.i = add i64 %iter.sroa.0.0.i93.us.i, %19
  %_30.i97.us.i = icmp ult i64 %_28.i96.us.i, %_97.sroa.13.0.copyload.i
  br i1 %_30.i97.us.i, label %bb18.i100.us.i, label %panic8.i98.i

bb18.i100.us.i:                                   ; preds = %bb42.i95.us.i
  %_65.i101.us.i = add i64 %iter.sroa.0.0.i93.us.i, 1
  %23 = getelementptr inbounds nuw i8, ptr %_97.sroa.14.0.copyload.i, i64 %iter.sroa.0.0.i93.us.i
  %_25.i102.us.i = load i8, ptr %23, align 1, !alias.scope !66, !noalias !71, !noundef !3
  %24 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_28.i96.us.i
  %_27.i103.us.i = load i8, ptr %24, align 1, !alias.scope !63, !noalias !68, !noundef !3
  %_24.not.i104.us.i = icmp eq i8 %_25.i102.us.i, %_27.i103.us.i
  br i1 %_24.not.i104.us.i, label %bb16.i92.us.i, label %bb19.i105.us.i

bb19.i105.us.i:                                   ; preds = %bb18.i100.us.i
  %reass.sub = sub i64 %19, %_97.sroa.4.0.copyload.i
  %_31.i107.us.i = add i64 %reass.sub, 1
  %25 = add i64 %_31.i107.us.i, %iter.sroa.0.0.i93.us.i
  br label %bb37.sink.split.i.us.i

bb26.i112.us.i:                                   ; preds = %bb16.i92.us.i, %bb28.i124.us.i
  %iter3.sroa.2.0.i113.us.i = phi i64 [ %_73.i117.us.i, %bb28.i124.us.i ], [ %_97.sroa.4.0.copyload.i, %bb16.i92.us.i ]
  %_70.i114.us.i = icmp ult i64 %v229.i83.us.i, %iter3.sroa.2.0.i113.us.i
  br i1 %_70.i114.us.i, label %bb46.i116.us.i, label %bb49.us.i

bb46.i116.us.i:                                   ; preds = %bb26.i112.us.i
  %_73.i117.us.i = add i64 %iter3.sroa.2.0.i113.us.i, -1
  %_43.i118.us.i = icmp ult i64 %_73.i117.us.i, %_97.sroa.15.0.copyload.i
  br i1 %_43.i118.us.i, label %bb27.i120.us.i, label %panic.i.invoke.i

bb27.i120.us.i:                                   ; preds = %bb46.i116.us.i
  %_45.i121.us.i = add i64 %_73.i117.us.i, %19
  %_47.i122.us.i = icmp ult i64 %_45.i121.us.i, %_97.sroa.13.0.copyload.i
  br i1 %_47.i122.us.i, label %bb28.i124.us.i, label %panic.i.invoke.i

bb28.i124.us.i:                                   ; preds = %bb27.i120.us.i
  %26 = getelementptr inbounds nuw i8, ptr %_97.sroa.14.0.copyload.i, i64 %_73.i117.us.i
  %_42.i125.us.i = load i8, ptr %26, align 1, !alias.scope !66, !noalias !71, !noundef !3
  %27 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_45.i121.us.i
  %_44.i126.us.i = load i8, ptr %27, align 1, !alias.scope !63, !noalias !68, !noundef !3
  %_41.not.i127.us.i = icmp eq i8 %_42.i125.us.i, %_44.i126.us.i
  br i1 %_41.not.i127.us.i, label %bb26.i112.us.i, label %bb29.i128.us.i

bb29.i128.us.i:                                   ; preds = %bb28.i124.us.i
  %28 = add i64 %19, %_97.sroa.6.0.copyload.i
  br label %bb37.sink.split.i.us.i

bb10.i129.us.i:                                   ; preds = %bb39.i82.us.i
  %29 = add i64 %19, %_97.sroa.15.0.copyload.i
  br label %bb37.sink.split.i.us.i

bb37.sink.split.i.us.i:                           ; preds = %bb10.i129.us.i, %bb29.i128.us.i, %bb19.i105.us.i
  %.sink.i.us.i = phi i64 [ %17, %bb29.i128.us.i ], [ 0, %bb19.i105.us.i ], [ 0, %bb10.i129.us.i ]
  %.ph.i.us.i = phi i64 [ %28, %bb29.i128.us.i ], [ %25, %bb19.i105.us.i ], [ %29, %bb10.i129.us.i ]
  %index.i109.us.i = add i64 %.ph.i.us.i, %needle_last.i71.i
  %_53.i110.us.i = icmp ult i64 %index.i109.us.i, %_97.sroa.13.0.copyload.i
  br i1 %_53.i110.us.i, label %bb39.i82.us.i, label %bb56.i

bb8.i.us.i:                                       ; preds = %bb20.us.i
  call void @llvm.experimental.noalias.scope.decl(metadata !72)
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  br i1 %_5325.i.us.i, label %bb39.i.us.i, label %bb56.i

bb39.i.us.i:                                      ; preds = %bb8.i.us.i, %bb37.i.us.i
  %index26.i.us.i = phi i64 [ %index.i.us.i, %bb37.i.us.i ], [ %index24.i.us.i, %bb8.i.us.i ]
  %30 = phi i64 [ %43, %bb37.i.us.i ], [ %iter.sroa.18.0.us.i, %bb8.i.us.i ]
  %_55.i.us.i = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %index26.i.us.i
  %tail_byte.i.us.i = load i8, ptr %_55.i.us.i, align 1, !alias.scope !72, !noalias !77, !noundef !3
  %_60.i.us.i = and i8 %tail_byte.i.us.i, 63
  %_59.i.us.i = zext nneg i8 %_60.i.us.i to i64
  %31 = shl nuw i64 1, %_59.i.us.i
  %32 = and i64 %31, %_97.sroa.7.0.copyload.i
  %33 = icmp eq i64 %32, 0
  br i1 %33, label %bb10.i67.us.i, label %bb16.i.us.i

bb16.i.us.i:                                      ; preds = %bb39.i.us.i, %bb18.i.us.i
  %iter.sroa.0.0.i.us.i = phi i64 [ %_65.i.us.i, %bb18.i.us.i ], [ %_97.sroa.4.0.copyload.i, %bb39.i.us.i ]
  %exitcond.not.i.us.i = icmp eq i64 %iter.sroa.0.0.i.us.i, %umax40.i.us.i
  br i1 %exitcond.not.i.us.i, label %bb26.i.us.i.preheader, label %bb42.i.us.i

bb26.i.us.i.preheader:                            ; preds = %bb16.i.us.i
  br i1 %_43.i.us.first_iter.i.fr, label %bb26.i.us.i.us, label %bb26.i.us.i.preheader.split

bb26.i.us.i.us:                                   ; preds = %bb26.i.us.i.preheader, %bb28.i.us.i.us
  %iter3.sroa.2.0.i.us.i.us = phi i64 [ %_73.i.us.i.us, %bb28.i.us.i.us ], [ %_97.sroa.4.0.copyload.i, %bb26.i.us.i.preheader ]
  %_70.i.not.us.i.us = icmp eq i64 %iter3.sroa.2.0.i.us.i.us, 0
  br i1 %_70.i.not.us.i.us, label %bb49.us.i, label %bb46.i.us.i.us

bb46.i.us.i.us:                                   ; preds = %bb26.i.us.i.us
  %_73.i.us.i.us = add i64 %iter3.sroa.2.0.i.us.i.us, -1
  %_45.i.us.i.us = add i64 %_73.i.us.i.us, %30
  %_47.i.us.i.us = icmp ult i64 %_45.i.us.i.us, %_97.sroa.13.0.copyload.i
  br i1 %_47.i.us.i.us, label %bb28.i.us.i.us, label %panic.i.invoke.i

bb28.i.us.i.us:                                   ; preds = %bb46.i.us.i.us
  %34 = getelementptr inbounds nuw i8, ptr %_97.sroa.14.0.copyload.i, i64 %_73.i.us.i.us
  %_42.i.us.i.us = load i8, ptr %34, align 1, !alias.scope !75, !noalias !80, !noundef !3
  %35 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_45.i.us.i.us
  %_44.i.us.i.us = load i8, ptr %35, align 1, !alias.scope !72, !noalias !77, !noundef !3
  %_41.not.i.us.i.us = icmp eq i8 %_42.i.us.i.us, %_44.i.us.i.us
  br i1 %_41.not.i.us.i.us, label %bb26.i.us.i.us, label %bb29.i.us.i.split.us

bb29.i.us.i.split.us:                             ; preds = %bb28.i.us.i.us
  %36 = add i64 %30, %_97.sroa.6.0.copyload.i
  br label %bb37.i.us.i

bb26.i.us.i.preheader.split:                      ; preds = %bb26.i.us.i.preheader
  br i1 %_70.i.not.us.i, label %bb49.us.i, label %panic.i.invoke.i

bb42.i.us.i:                                      ; preds = %bb16.i.us.i
  %_28.i.us.i = add i64 %iter.sroa.0.0.i.us.i, %30
  %_30.i.us.i = icmp ult i64 %_28.i.us.i, %_97.sroa.13.0.copyload.i
  br i1 %_30.i.us.i, label %bb18.i.us.i, label %panic8.i.i

bb18.i.us.i:                                      ; preds = %bb42.i.us.i
  %_65.i.us.i = add i64 %iter.sroa.0.0.i.us.i, 1
  %37 = getelementptr inbounds nuw i8, ptr %_97.sroa.14.0.copyload.i, i64 %iter.sroa.0.0.i.us.i
  %_25.i.us.i = load i8, ptr %37, align 1, !alias.scope !75, !noalias !80, !noundef !3
  %38 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_28.i.us.i
  %_27.i.us.i = load i8, ptr %38, align 1, !alias.scope !72, !noalias !77, !noundef !3
  %_24.not.i.us.i = icmp eq i8 %_25.i.us.i, %_27.i.us.i
  br i1 %_24.not.i.us.i, label %bb16.i.us.i, label %bb19.i.us.i

bb19.i.us.i:                                      ; preds = %bb18.i.us.i
  %reass.sub298 = sub i64 %30, %_97.sroa.4.0.copyload.i
  %_31.i.us.i = add i64 %reass.sub298, 1
  %39 = add i64 %_31.i.us.i, %iter.sroa.0.0.i.us.i
  br label %bb37.i.us.i

bb49.us.i:                                        ; preds = %bb26.i112.us.i, %bb26.i.us.i.us, %bb26.i.us.i.preheader.split
  %iter.sroa.306.3.us.i = phi i64 [ -1, %bb26.i.us.i.preheader.split ], [ -1, %bb26.i.us.i.us ], [ 0, %bb26.i112.us.i ]
  %self5.sroa.7.4.us.i = phi i64 [ %30, %bb26.i.us.i.preheader.split ], [ %30, %bb26.i.us.i.us ], [ %19, %bb26.i112.us.i ]
  %iter.sroa.18.3.us.i = add i64 %self5.sroa.7.4.us.i, %_97.sroa.15.0.copyload.i
  %data13.us.i = getelementptr inbounds nuw i8, ptr @alloc_a98ee8a049a1c25292c235225752f6c8, i64 %last_end.sroa.0.0.us.i
  %gepdiff.us.i = sub nuw nsw i64 %self5.sroa.7.4.us.i, %last_end.sroa.0.0.us.i
  call void @llvm.experimental.noalias.scope.decl(metadata !81)
  %self2.i.i41.us.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !84, !noalias !56, !noundef !3
  %_9.i.i42.us.i = sub i64 %self2.i.i41.us.i, %len.i.i40.us.i
  %_7.i.i43.us.i = icmp ugt i64 %gepdiff.us.i, %_9.i.i42.us.i
  br i1 %_7.i.i43.us.i, label %bb1.i.i48.us.i, label %bb51.us.i, !prof !41

bb1.i.i48.us.i:                                   ; preds = %bb49.us.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %len.i.i40.us.i, i64 noundef %gepdiff.us.i)
          to label %.noexc50.us.i unwind label %cleanup10.loopexit.split.us.i

.noexc50.us.i:                                    ; preds = %bb1.i.i48.us.i
  %len.pre.i49.us.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  %_10.i46.us.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  %self2.i.i53.us.pre.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !87, !noalias !56
  br label %bb51.us.i

bb51.us.i:                                        ; preds = %.noexc50.us.i, %bb49.us.i
  %_10.i58.us161.i = phi ptr [ %_10.i58.us162.i, %bb49.us.i ], [ %_10.i46.us.pre.i, %.noexc50.us.i ]
  %self2.i.i53.us.i = phi i64 [ %self2.i.i41.us.i, %bb49.us.i ], [ %self2.i.i53.us.pre.i, %.noexc50.us.i ]
  %len.i44.us.i = phi i64 [ %len.i.i40.us.i, %bb49.us.i ], [ %len.pre.i49.us.i, %.noexc50.us.i ]
  %_9.i45.us.i = icmp sgt i64 %len.i44.us.i, -1
  call void @llvm.assume(i1 %_9.i45.us.i)
  %dst.i47.us.i = getelementptr inbounds nuw i8, ptr %_10.i58.us161.i, i64 %len.i44.us.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i47.us.i, ptr nonnull readonly align 1 %data13.us.i, i64 %gepdiff.us.i, i1 false), !noalias !92
  %40 = add i64 %len.i44.us.i, %gepdiff.us.i
  store i64 %40, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  call void @llvm.experimental.noalias.scope.decl(metadata !93)
  %_9.i.i54.us.i = sub i64 %self2.i.i53.us.i, %40
  %_7.i.i55.us.i = icmp ugt i64 %patch_version.sroa.11.0.copyload, %_9.i.i54.us.i
  br i1 %_7.i.i55.us.i, label %bb1.i.i60.us.i, label %bb50.us.i, !prof !41

bb1.i.i60.us.i:                                   ; preds = %bb51.us.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %40, i64 noundef %patch_version.sroa.11.0.copyload)
          to label %.noexc62.us.i unwind label %cleanup10.loopexit.split.us.i

.noexc62.us.i:                                    ; preds = %bb1.i.i60.us.i
  %len.pre.i61.us.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !93, !noalias !56
  %_10.i58.us.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !93, !noalias !56
  br label %bb50.us.i

bb50.us.i:                                        ; preds = %.noexc62.us.i, %bb51.us.i
  %_10.i58.us.i = phi ptr [ %_10.i58.us161.i, %bb51.us.i ], [ %_10.i58.us.pre.i, %.noexc62.us.i ]
  %len.i56.us.i = phi i64 [ %40, %bb51.us.i ], [ %len.pre.i61.us.i, %.noexc62.us.i ]
  %_9.i57.us.i = icmp sgt i64 %len.i56.us.i, -1
  call void @llvm.assume(i1 %_9.i57.us.i)
  %dst.i59.us.i = getelementptr inbounds nuw i8, ptr %_10.i58.us.i, i64 %len.i56.us.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i59.us.i, ptr nonnull readonly align 1 %patch_version.sroa.7.0.copyload, i64 %patch_version.sroa.11.0.copyload, i1 false), !noalias !94
  %41 = add i64 %len.i56.us.i, %patch_version.sroa.11.0.copyload
  store i64 %41, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !93, !noalias !56
  br label %bb20.us.i

bb10.i67.us.i:                                    ; preds = %bb39.i.us.i
  %42 = add i64 %30, %_97.sroa.15.0.copyload.i
  br label %bb37.i.us.i

bb37.i.us.i:                                      ; preds = %bb10.i67.us.i, %bb29.i.us.i.split.us, %bb19.i.us.i
  %43 = phi i64 [ %39, %bb19.i.us.i ], [ %36, %bb29.i.us.i.split.us ], [ %42, %bb10.i67.us.i ]
  %index.i.us.i = add i64 %43, %needle_last.i71.i
  %_53.i.us.i = icmp ult i64 %index.i.us.i, %_97.sroa.13.0.copyload.i
  br i1 %_53.i.us.i, label %bb39.i.us.i, label %bb56.i

cleanup10.loopexit.split.us.i:                    ; preds = %bb1.i.i48.us.i, %bb1.i.i60.us.i
  %lpad.loopexit.us.i = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

bb40.split.i:                                     ; preds = %bb40.i
  %44 = and i64 %_97.sroa.6.0.copyload.i, 65536
  %_4.i.i.not.i = icmp eq i64 %44, 0
  br i1 %_4.i.i.not.i, label %bb5.i.lr.ph.i.lr.ph.i, label %bb56.i

bb5.i.lr.ph.i.lr.ph.i:                            ; preds = %bb40.split.i
  call void @llvm.assume(i1 %15)
  %extract.t.i = trunc i64 %_97.sroa.6.0.copyload.i to i1
  %45 = icmp eq i64 %_97.sroa.4.0.copyload.i, 0
  br i1 %45, label %bb19.i.i.peel.i.peel, label %bb5.i.i.i.peel.i.peel

bb5.i.i.i.peel.i.peel:                            ; preds = %bb5.i.lr.ph.i.lr.ph.i
  %_8.not.i.i.i.peel.i.peel = icmp ult i64 %_97.sroa.4.0.copyload.i, %_97.sroa.13.0.copyload.i
  br i1 %_8.not.i.i.i.peel.i.peel, label %bb9.i.i.i.peel.i.peel, label %bb6.i.i.i35.peel.i.peel

bb6.i.i.i35.peel.i.peel:                          ; preds = %bb5.i.i.i.peel.i.peel
  %46 = icmp eq i64 %_97.sroa.4.0.copyload.i, %_97.sroa.13.0.copyload.i
  br i1 %46, label %bb19.i.i.peel.i.peel, label %bb18.i.i.i

bb9.i.i.i.peel.i.peel:                            ; preds = %bb5.i.i.i.peel.i.peel
  %47 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_97.sroa.4.0.copyload.i
  %self1.i.i.i.peel.i.peel = load i8, ptr %47, align 1, !alias.scope !95, !noalias !98, !noundef !3
  %48 = icmp sgt i8 %self1.i.i.i.peel.i.peel, -65
  br i1 %48, label %bb19.i.i.peel.i.peel, label %bb18.i.i.i

bb19.i.i.peel.i.peel:                             ; preds = %bb9.i.i.i.peel.i.peel, %bb6.i.i.i35.peel.i.peel, %bb5.i.lr.ph.i.lr.ph.i
  %data.i.i.i.peel.i.peel = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %_97.sroa.4.0.copyload.i
  %_6.i.i.i.i.peel.i.peel = icmp samesign eq i64 %_97.sroa.4.0.copyload.i, %_97.sroa.13.0.copyload.i
  br i1 %_6.i.i.i.i.peel.i.peel, label %bb21.i.i.i.peel, label %bb14.i.i.i.peel.i.peel

bb14.i.i.i.peel.i.peel:                           ; preds = %bb19.i.i.peel.i.peel
  %x.i.i.i.peel.i.peel = load i8, ptr %data.i.i.i.peel.i.peel, align 1, !noalias !105, !noundef !3
  %_6.i.i.i.peel.i.peel = icmp sgt i8 %x.i.i.i.peel.i.peel, -1
  br i1 %_6.i.i.i.peel.i.peel, label %bb3.i.i.i.peel.i.peel, label %bb4.i.i.i.peel.i.peel

bb4.i.i.i.peel.i.peel:                            ; preds = %bb14.i.i.i.peel.i.peel
  %_16.i.i.i.i.peel.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.peel, i64 1
  %_30.i.i.i.peel.i.peel = and i8 %x.i.i.i.peel.i.peel, 31
  %init.i.i.i.peel.i.peel = zext nneg i8 %_30.i.i.i.peel.i.peel to i32
  %_6.i10.i.i.i.peel.i.peel = icmp ne ptr %_16.i.i.i.i.peel.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.peel.i.peel)
  %y.i.i.i.peel.i.peel = load i8, ptr %_16.i.i.i.i.peel.i.peel, align 1, !noalias !105, !noundef !3
  %_33.i.i.i.peel.i.peel = shl nuw nsw i32 %init.i.i.i.peel.i.peel, 6
  %_35.i.i.i.peel.i.peel = and i8 %y.i.i.i.peel.i.peel, 63
  %_34.i.i.i.peel.i.peel = zext nneg i8 %_35.i.i.i.peel.i.peel to i32
  %49 = or disjoint i32 %_33.i.i.i.peel.i.peel, %_34.i.i.i.peel.i.peel
  %_13.i.i.i.peel.i.peel = icmp samesign ugt i8 %x.i.i.i.peel.i.peel, -33
  br i1 %_13.i.i.i.peel.i.peel, label %bb6.i21.i.i.peel.i.peel, label %bb22.i.i.peel.i.peel

bb6.i21.i.i.peel.i.peel:                          ; preds = %bb4.i.i.i.peel.i.peel
  %_16.i12.i.i.i.peel.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.peel, i64 2
  %_6.i17.i.i.i.peel.i.peel = icmp ne ptr %_16.i12.i.i.i.peel.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.peel.i.peel)
  %z.i.i.i.peel.i.peel = load i8, ptr %_16.i12.i.i.i.peel.i.peel, align 1, !noalias !105, !noundef !3
  %_38.i.i.i.peel.i.peel = shl nuw nsw i32 %_34.i.i.i.peel.i.peel, 6
  %_40.i.i.i.peel.i.peel = and i8 %z.i.i.i.peel.i.peel, 63
  %_39.i.i.i.peel.i.peel = zext nneg i8 %_40.i.i.i.peel.i.peel to i32
  %y_z.i.i.i.peel.i.peel = or disjoint i32 %_38.i.i.i.peel.i.peel, %_39.i.i.i.peel.i.peel
  %_20.i.i.i.peel.i.peel = shl nuw nsw i32 %init.i.i.i.peel.i.peel, 12
  %50 = or disjoint i32 %y_z.i.i.i.peel.i.peel, %_20.i.i.i.peel.i.peel
  %_21.i.i.i.peel.i.peel = icmp samesign ugt i8 %x.i.i.i.peel.i.peel, -17
  br i1 %_21.i.i.i.peel.i.peel, label %bb8.i.i.i.peel.i.peel, label %bb22.i.i.peel.i.peel

bb8.i.i.i.peel.i.peel:                            ; preds = %bb6.i21.i.i.peel.i.peel
  %_16.i19.i.i.i.peel.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i.peel, i64 3
  %_6.i24.i.i.i.peel.i.peel = icmp ne ptr %_16.i19.i.i.i.peel.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.peel.i.peel)
  %w.i.i.i.peel.i.peel = load i8, ptr %_16.i19.i.i.i.peel.i.peel, align 1, !noalias !105, !noundef !3
  %_26.i.i.i.peel.i.peel = shl nuw nsw i32 %init.i.i.i.peel.i.peel, 18
  %_25.i.i.i.peel.i.peel = and i32 %_26.i.i.i.peel.i.peel, 1835008
  %_43.i.i.i.peel.i.peel = shl nuw nsw i32 %y_z.i.i.i.peel.i.peel, 6
  %_45.i.i.i.peel.i.peel = and i8 %w.i.i.i.peel.i.peel, 63
  %_44.i.i.i.peel.i.peel = zext nneg i8 %_45.i.i.i.peel.i.peel to i32
  %_27.i.i.i.peel.i.peel = or disjoint i32 %_43.i.i.i.peel.i.peel, %_44.i.i.i.peel.i.peel
  %51 = or disjoint i32 %_27.i.i.i.peel.i.peel, %_25.i.i.i.peel.i.peel
  br label %bb22.i.i.peel.i.peel

bb3.i.i.i.peel.i.peel:                            ; preds = %bb14.i.i.i.peel.i.peel
  %_7.i.i.i.peel.i.peel = zext nneg i8 %x.i.i.i.peel.i.peel to i32
  br label %bb22.i.i.peel.i.peel

bb22.i.i.peel.i.peel:                             ; preds = %bb3.i.i.i.peel.i.peel, %bb8.i.i.i.peel.i.peel, %bb6.i21.i.i.peel.i.peel, %bb4.i.i.i.peel.i.peel
  %_0.sroa.4.0.i.ph.i.i.peel.i.peel = phi i32 [ %49, %bb4.i.i.i.peel.i.peel ], [ %50, %bb6.i21.i.i.peel.i.peel ], [ %51, %bb8.i.i.i.peel.i.peel ], [ %_7.i.i.i.peel.i.peel, %bb3.i.i.i.peel.i.peel ]
  %52 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.peel, 1114112
  call void @llvm.assume(i1 %52)
  br i1 %extract.t.i, label %bb49.i.peel, label %bb39.i.i.peel.i.peel

bb39.i.i.peel.i.peel:                             ; preds = %bb22.i.i.peel.i.peel
  %_59.i.i.peel.i.peel = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.peel, 128
  br i1 %_59.i.i.peel.i.peel, label %bb5.i.i.i.peel, label %bb26.i.i.peel.i.peel

bb26.i.i.peel.i.peel:                             ; preds = %bb39.i.i.peel.i.peel
  %_60.i.i.peel.i.peel = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.peel, 2048
  br i1 %_60.i.i.peel.i.peel, label %bb5.i.i.i.peel, label %bb27.i.i.peel.i.peel

bb27.i.i.peel.i.peel:                             ; preds = %bb26.i.i.peel.i.peel
  %_61.i.i.peel.i.peel = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i.peel, 65536
  %..i.i.peel.i.peel = select i1 %_61.i.i.peel.i.peel, i64 3, i64 4
  br label %bb5.i.i.i.peel

bb5.i.i.i.peel:                                   ; preds = %bb27.i.i.peel.i.peel, %bb26.i.i.peel.i.peel, %bb39.i.i.peel.i.peel
  %_14.sroa.0.0.i.i.peel.i.peel = phi i64 [ 1, %bb39.i.i.peel.i.peel ], [ %..i.i.peel.i.peel, %bb27.i.i.peel.i.peel ], [ 2, %bb26.i.i.peel.i.peel ]
  %53 = add i64 %_14.sroa.0.0.i.i.peel.i.peel, %_97.sroa.4.0.copyload.i
  %54 = icmp eq i64 %53, 0
  br i1 %54, label %bb19.i.i.i.peel, label %bb5.i.i.i.i.peel

bb5.i.i.i.i.peel:                                 ; preds = %bb5.i.i.i.peel
  %_8.not.i.i.i.i.peel = icmp ult i64 %53, %_97.sroa.13.0.copyload.i
  br i1 %_8.not.i.i.i.i.peel, label %bb9.i.i.i.i.peel, label %bb6.i.i.i35.i.peel

bb6.i.i.i35.i.peel:                               ; preds = %bb5.i.i.i.i.peel
  %55 = icmp eq i64 %53, %_97.sroa.13.0.copyload.i
  br i1 %55, label %bb19.i.i.i.peel, label %bb18.i.i.i

bb9.i.i.i.i.peel:                                 ; preds = %bb5.i.i.i.i.peel
  %56 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %53
  %self1.i.i.i.i.peel = load i8, ptr %56, align 1, !alias.scope !95, !noalias !98, !noundef !3
  %57 = icmp sgt i8 %self1.i.i.i.i.peel, -65
  br i1 %57, label %bb19.i.i.i.peel, label %bb18.i.i.i

bb19.i.i.i.peel:                                  ; preds = %bb9.i.i.i.i.peel, %bb6.i.i.i35.i.peel, %bb5.i.i.i.peel
  %data.i.i.i.i.peel = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %53
  %_6.i.i.i.i.i.peel = icmp samesign eq i64 %53, %_97.sroa.13.0.copyload.i
  br i1 %_6.i.i.i.i.i.peel, label %bb49.i.peel, label %bb14.i.i.i.i.peel

bb14.i.i.i.i.peel:                                ; preds = %bb19.i.i.i.peel
  %x.i.i.i.i.peel = load i8, ptr %data.i.i.i.i.peel, align 1, !noalias !105, !noundef !3
  %_6.i.i.i.i.peel = icmp sgt i8 %x.i.i.i.i.peel, -1
  br i1 %_6.i.i.i.i.peel, label %bb3.i.i.i.i.peel, label %bb4.i.i.i.i.peel

bb4.i.i.i.i.peel:                                 ; preds = %bb14.i.i.i.i.peel
  %_16.i.i.i.i.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.peel, i64 1
  %_30.i.i.i.i.peel = and i8 %x.i.i.i.i.peel, 31
  %init.i.i.i.i.peel = zext nneg i8 %_30.i.i.i.i.peel to i32
  %_6.i10.i.i.i.i.peel = icmp ne ptr %_16.i.i.i.i.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.peel)
  %y.i.i.i.i.peel = load i8, ptr %_16.i.i.i.i.i.peel, align 1, !noalias !105, !noundef !3
  %_33.i.i.i.i.peel = shl nuw nsw i32 %init.i.i.i.i.peel, 6
  %_35.i.i.i.i.peel = and i8 %y.i.i.i.i.peel, 63
  %_34.i.i.i.i.peel = zext nneg i8 %_35.i.i.i.i.peel to i32
  %58 = or disjoint i32 %_33.i.i.i.i.peel, %_34.i.i.i.i.peel
  %_13.i.i.i.i.peel = icmp samesign ugt i8 %x.i.i.i.i.peel, -33
  br i1 %_13.i.i.i.i.peel, label %bb6.i21.i.i.i.peel, label %bb49.loopexit.loopexit.i.peel

bb6.i21.i.i.i.peel:                               ; preds = %bb4.i.i.i.i.peel
  %_16.i12.i.i.i.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.peel, i64 2
  %_6.i17.i.i.i.i.peel = icmp ne ptr %_16.i12.i.i.i.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.peel)
  %z.i.i.i.i.peel = load i8, ptr %_16.i12.i.i.i.i.peel, align 1, !noalias !105, !noundef !3
  %_38.i.i.i.i.peel = shl nuw nsw i32 %_34.i.i.i.i.peel, 6
  %_40.i.i.i.i.peel = and i8 %z.i.i.i.i.peel, 63
  %_39.i.i.i.i.peel = zext nneg i8 %_40.i.i.i.i.peel to i32
  %y_z.i.i.i.i.peel = or disjoint i32 %_38.i.i.i.i.peel, %_39.i.i.i.i.peel
  %_20.i.i.i.i.peel = shl nuw nsw i32 %init.i.i.i.i.peel, 12
  %59 = or disjoint i32 %y_z.i.i.i.i.peel, %_20.i.i.i.i.peel
  %_21.i.i.i.i.peel = icmp samesign ugt i8 %x.i.i.i.i.peel, -17
  br i1 %_21.i.i.i.i.peel, label %bb8.i.i.i.i.peel, label %bb49.loopexit.loopexit.i.peel

bb8.i.i.i.i.peel:                                 ; preds = %bb6.i21.i.i.i.peel
  %_16.i19.i.i.i.i.peel = getelementptr inbounds nuw i8, ptr %data.i.i.i.i.peel, i64 3
  %_6.i24.i.i.i.i.peel = icmp ne ptr %_16.i19.i.i.i.i.peel, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.peel)
  %w.i.i.i.i.peel = load i8, ptr %_16.i19.i.i.i.i.peel, align 1, !noalias !105, !noundef !3
  %_26.i.i.i.i.peel = shl nuw nsw i32 %init.i.i.i.i.peel, 18
  %_25.i.i.i.i.peel = and i32 %_26.i.i.i.i.peel, 1835008
  %_43.i.i.i.i.peel = shl nuw nsw i32 %y_z.i.i.i.i.peel, 6
  %_45.i.i.i.i.peel = and i8 %w.i.i.i.i.peel, 63
  %_44.i.i.i.i.peel = zext nneg i8 %_45.i.i.i.i.peel to i32
  %_27.i.i.i.i.peel = or disjoint i32 %_43.i.i.i.i.peel, %_44.i.i.i.i.peel
  %60 = or disjoint i32 %_27.i.i.i.i.peel, %_25.i.i.i.i.peel
  br label %bb49.loopexit.loopexit.i.peel

bb3.i.i.i.i.peel:                                 ; preds = %bb14.i.i.i.i.peel
  %_7.i.i.i.i.peel = zext nneg i8 %x.i.i.i.i.peel to i32
  br label %bb49.loopexit.loopexit.i.peel

bb49.loopexit.loopexit.i.peel:                    ; preds = %bb3.i.i.i.i.peel, %bb8.i.i.i.i.peel, %bb6.i21.i.i.i.peel, %bb4.i.i.i.i.peel
  %_0.sroa.4.0.i.ph.i.i.i.peel = phi i32 [ %58, %bb4.i.i.i.i.peel ], [ %59, %bb6.i21.i.i.i.peel ], [ %60, %bb8.i.i.i.i.peel ], [ %_7.i.i.i.i.peel, %bb3.i.i.i.i.peel ]
  %61 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.peel, 1114112
  call void @llvm.assume(i1 %61)
  br label %bb49.i.peel

bb21.i.i.i.peel:                                  ; preds = %bb19.i.i.peel.i.peel
  br i1 %extract.t.i, label %bb49.i.peel, label %bb56.i

bb49.i.peel:                                      ; preds = %bb21.i.i.i.peel, %bb49.loopexit.loopexit.i.peel, %bb19.i.i.i.peel, %bb22.i.i.peel.i.peel
  %iter.sroa.4.1145.i.peel = phi i64 [ %_97.sroa.4.0.copyload.i, %bb21.i.i.i.peel ], [ %_97.sroa.4.0.copyload.i, %bb22.i.i.peel.i.peel ], [ %53, %bb49.loopexit.loopexit.i.peel ], [ %_97.sroa.13.0.copyload.i, %bb19.i.i.i.peel ]
  call void @llvm.experimental.noalias.scope.decl(metadata !108)
  %_7.i.i43.i.peel = icmp ugt i64 %iter.sroa.4.1145.i.peel, %self2.i.i41.i.peel
  br i1 %_7.i.i43.i.peel, label %bb1.i.i48.i.peel, label %bb51.i.peel, !prof !41

bb1.i.i48.i.peel:                                 ; preds = %bb49.i.peel
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef 0, i64 noundef %iter.sroa.4.1145.i.peel)
          to label %.noexc50.i.peel unwind label %cleanup10.loopexit.split.i.loopexit.split-lp

.noexc50.i.peel:                                  ; preds = %bb1.i.i48.i.peel
  %len.pre.i49.i.peel = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !108, !noalias !56
  %_10.i46.pre.i.peel = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !108, !noalias !56
  %self2.i.i53.pre.i.peel = load i64, ptr %result.i, align 8, !range !8, !alias.scope !110, !noalias !56
  br label %bb51.i.peel

bb51.i.peel:                                      ; preds = %.noexc50.i.peel, %bb49.i.peel
  %_10.i58154.i.peel = phi ptr [ %_4.sroa.10.0.i.i, %bb49.i.peel ], [ %_10.i46.pre.i.peel, %.noexc50.i.peel ]
  %self2.i.i53.i.peel = phi i64 [ %self2.i.i41.i.peel, %bb49.i.peel ], [ %self2.i.i53.pre.i.peel, %.noexc50.i.peel ]
  %len.i44.i.peel = phi i64 [ 0, %bb49.i.peel ], [ %len.pre.i49.i.peel, %.noexc50.i.peel ]
  %_9.i45.i.peel = icmp sgt i64 %len.i44.i.peel, -1
  call void @llvm.assume(i1 %_9.i45.i.peel)
  %dst.i47.i.peel = getelementptr inbounds nuw i8, ptr %_10.i58154.i.peel, i64 %len.i44.i.peel
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i47.i.peel, ptr nonnull readonly align 1 @alloc_a98ee8a049a1c25292c235225752f6c8, i64 %iter.sroa.4.1145.i.peel, i1 false), !noalias !112
  %62 = add i64 %len.i44.i.peel, %iter.sroa.4.1145.i.peel
  store i64 %62, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !108, !noalias !56
  %_9.i.i54.i.peel = sub i64 %self2.i.i53.i.peel, %62
  %_7.i.i55.i.peel = icmp ugt i64 %patch_version.sroa.11.0.copyload, %_9.i.i54.i.peel
  br i1 %_7.i.i55.i.peel, label %bb1.i.i60.i.peel, label %bb50.i.peel, !prof !41

bb1.i.i60.i.peel:                                 ; preds = %bb51.i.peel
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %62, i64 noundef %patch_version.sroa.11.0.copyload)
          to label %.noexc62.i.peel unwind label %cleanup10.loopexit.split.i.loopexit.split-lp

.noexc62.i.peel:                                  ; preds = %bb1.i.i60.i.peel
  %len.pre.i61.i.peel = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !113, !noalias !56
  %_10.i58.pre.i.peel = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !113, !noalias !56
  br label %bb50.i.peel

bb50.i.peel:                                      ; preds = %.noexc62.i.peel, %bb51.i.peel
  %_10.i58.i.peel = phi ptr [ %_10.i58154.i.peel, %bb51.i.peel ], [ %_10.i58.pre.i.peel, %.noexc62.i.peel ]
  %len.i56.i.peel = phi i64 [ %62, %bb51.i.peel ], [ %len.pre.i61.i.peel, %.noexc62.i.peel ]
  %_9.i57.i.peel = icmp sgt i64 %len.i56.i.peel, -1
  call void @llvm.assume(i1 %_9.i57.i.peel)
  %dst.i59.i.peel = getelementptr inbounds nuw i8, ptr %_10.i58.i.peel, i64 %len.i56.i.peel
  br label %bb5.i.lr.ph.i.i

bb5.i.lr.ph.i.i:                                  ; preds = %bb50.i, %bb50.i.peel
  %dst.i59.i.sink = phi ptr [ %dst.i59.i, %bb50.i ], [ %dst.i59.i.peel, %bb50.i.peel ]
  %_10.i58155.i = phi ptr [ %_10.i58.i, %bb50.i ], [ %_10.i58.i.peel, %bb50.i.peel ]
  %len.i56.i.peel.pn = phi i64 [ %len.i56.i, %bb50.i ], [ %len.i56.i.peel, %bb50.i.peel ]
  %last_end.sroa.0.0103.i = phi i64 [ %iter.sroa.4.1145.i, %bb50.i ], [ %iter.sroa.4.1145.i.peel, %bb50.i.peel ]
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i59.i.sink, ptr nonnull readonly align 1 %patch_version.sroa.7.0.copyload, i64 %patch_version.sroa.11.0.copyload, i1 false), !noalias !114
  %len.i.i40.i = add i64 %len.i56.i.peel.pn, %patch_version.sroa.11.0.copyload
  store i64 %len.i.i40.i, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !115, !noalias !56
  %63 = icmp eq i64 %last_end.sroa.0.0103.i, 0
  br i1 %63, label %bb19.i.i.peel.i, label %bb5.i.i.i.peel.i

bb5.i.i.i.peel.i:                                 ; preds = %bb5.i.lr.ph.i.i
  %_8.not.i.i.i.peel.i = icmp ult i64 %last_end.sroa.0.0103.i, %_97.sroa.13.0.copyload.i
  br i1 %_8.not.i.i.i.peel.i, label %bb9.i.i.i.peel.i, label %bb6.i.i.i35.peel.i

bb6.i.i.i35.peel.i:                               ; preds = %bb5.i.i.i.peel.i
  %64 = icmp eq i64 %last_end.sroa.0.0103.i, %_97.sroa.13.0.copyload.i
  br i1 %64, label %bb19.i.i.peel.i, label %bb18.i.i.i

bb9.i.i.i.peel.i:                                 ; preds = %bb5.i.i.i.peel.i
  %65 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %last_end.sroa.0.0103.i
  %self1.i.i.i.peel.i = load i8, ptr %65, align 1, !alias.scope !95, !noalias !98, !noundef !3
  %66 = icmp sgt i8 %self1.i.i.i.peel.i, -65
  br i1 %66, label %bb19.i.i.peel.i, label %bb18.i.i.i

bb19.i.i.peel.i:                                  ; preds = %bb9.i.i.i.peel.i, %bb6.i.i.i35.peel.i, %bb5.i.lr.ph.i.i
  %data.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %last_end.sroa.0.0103.i
  %_6.i.i.i.i.peel.i = icmp samesign eq i64 %last_end.sroa.0.0103.i, %_97.sroa.13.0.copyload.i
  br i1 %_6.i.i.i.i.peel.i, label %bb56.i, label %bb14.i.i.i.peel.i

bb14.i.i.i.peel.i:                                ; preds = %bb19.i.i.peel.i
  %x.i.i.i.peel.i = load i8, ptr %data.i.i.i.peel.i, align 1, !noalias !105, !noundef !3
  %_6.i.i.i.peel.i = icmp sgt i8 %x.i.i.i.peel.i, -1
  br i1 %_6.i.i.i.peel.i, label %bb5.i.i.i, label %bb4.i.i.i.peel.i

bb4.i.i.i.peel.i:                                 ; preds = %bb14.i.i.i.peel.i
  %_16.i.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 1
  %_30.i.i.i.peel.i = and i8 %x.i.i.i.peel.i, 31
  %init.i.i.i.peel.i = zext nneg i8 %_30.i.i.i.peel.i to i32
  %_6.i10.i.i.i.peel.i = icmp ne ptr %_16.i.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.peel.i)
  %y.i.i.i.peel.i = load i8, ptr %_16.i.i.i.i.peel.i, align 1, !noalias !105, !noundef !3
  %_33.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 6
  %_35.i.i.i.peel.i = and i8 %y.i.i.i.peel.i, 63
  %_34.i.i.i.peel.i = zext nneg i8 %_35.i.i.i.peel.i to i32
  %67 = or disjoint i32 %_33.i.i.i.peel.i, %_34.i.i.i.peel.i
  %_13.i.i.i.peel.i = icmp samesign ugt i8 %x.i.i.i.peel.i, -33
  br i1 %_13.i.i.i.peel.i, label %bb6.i21.i.i.peel.i, label %bb39.i.i.peel.i

bb6.i21.i.i.peel.i:                               ; preds = %bb4.i.i.i.peel.i
  %_16.i12.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 2
  %_6.i17.i.i.i.peel.i = icmp ne ptr %_16.i12.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.peel.i)
  %z.i.i.i.peel.i = load i8, ptr %_16.i12.i.i.i.peel.i, align 1, !noalias !105, !noundef !3
  %_38.i.i.i.peel.i = shl nuw nsw i32 %_34.i.i.i.peel.i, 6
  %_40.i.i.i.peel.i = and i8 %z.i.i.i.peel.i, 63
  %_39.i.i.i.peel.i = zext nneg i8 %_40.i.i.i.peel.i to i32
  %y_z.i.i.i.peel.i = or disjoint i32 %_38.i.i.i.peel.i, %_39.i.i.i.peel.i
  %_20.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 12
  %68 = or disjoint i32 %y_z.i.i.i.peel.i, %_20.i.i.i.peel.i
  %_21.i.i.i.peel.i = icmp samesign ugt i8 %x.i.i.i.peel.i, -17
  br i1 %_21.i.i.i.peel.i, label %bb8.i.i.i.peel.i, label %bb39.i.i.peel.i

bb8.i.i.i.peel.i:                                 ; preds = %bb6.i21.i.i.peel.i
  %_16.i19.i.i.i.peel.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.peel.i, i64 3
  %_6.i24.i.i.i.peel.i = icmp ne ptr %_16.i19.i.i.i.peel.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.peel.i)
  %w.i.i.i.peel.i = load i8, ptr %_16.i19.i.i.i.peel.i, align 1, !noalias !105, !noundef !3
  %_26.i.i.i.peel.i = shl nuw nsw i32 %init.i.i.i.peel.i, 18
  %_25.i.i.i.peel.i = and i32 %_26.i.i.i.peel.i, 1835008
  %_43.i.i.i.peel.i = shl nuw nsw i32 %y_z.i.i.i.peel.i, 6
  %_45.i.i.i.peel.i = and i8 %w.i.i.i.peel.i, 63
  %_44.i.i.i.peel.i = zext nneg i8 %_45.i.i.i.peel.i to i32
  %_27.i.i.i.peel.i = or disjoint i32 %_43.i.i.i.peel.i, %_44.i.i.i.peel.i
  %69 = or disjoint i32 %_27.i.i.i.peel.i, %_25.i.i.i.peel.i
  br label %bb39.i.i.peel.i

bb39.i.i.peel.i:                                  ; preds = %bb4.i.i.i.peel.i, %bb6.i21.i.i.peel.i, %bb8.i.i.i.peel.i
  %_0.sroa.4.0.i.ph.i.i.peel.i = phi i32 [ %67, %bb4.i.i.i.peel.i ], [ %68, %bb6.i21.i.i.peel.i ], [ %69, %bb8.i.i.i.peel.i ]
  %70 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 1114112
  call void @llvm.assume(i1 %70)
  %_59.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 128
  br i1 %_59.i.i.peel.i, label %bb5.i.i.i, label %bb26.i.i.peel.i

bb26.i.i.peel.i:                                  ; preds = %bb39.i.i.peel.i
  %_60.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 2048
  br i1 %_60.i.i.peel.i, label %bb5.i.i.i, label %bb27.i.i.peel.i

bb27.i.i.peel.i:                                  ; preds = %bb26.i.i.peel.i
  %_61.i.i.peel.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.peel.i, 65536
  %..i.i.peel.i = select i1 %_61.i.i.peel.i, i64 3, i64 4
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb14.i.i.i.peel.i, %bb27.i.i.peel.i, %bb26.i.i.peel.i, %bb39.i.i.peel.i
  %_14.sroa.0.0.i.i.peel.i = phi i64 [ 1, %bb39.i.i.peel.i ], [ %..i.i.peel.i, %bb27.i.i.peel.i ], [ 2, %bb26.i.i.peel.i ], [ 1, %bb14.i.i.i.peel.i ]
  %71 = add i64 %_14.sroa.0.0.i.i.peel.i, %last_end.sroa.0.0103.i
  %72 = icmp eq i64 %71, 0
  br i1 %72, label %bb19.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb5.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %71, %_97.sroa.13.0.copyload.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i35.i

bb6.i.i.i35.i:                                    ; preds = %bb5.i.i.i.i
  %73 = icmp eq i64 %71, %_97.sroa.13.0.copyload.i
  br i1 %73, label %bb19.i.i.i, label %bb18.i.i.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %74 = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %71
  %self1.i.i.i.i = load i8, ptr %74, align 1, !alias.scope !95, !noalias !98, !noundef !3
  %75 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %75, label %bb19.i.i.i, label %bb18.i.i.i

bb19.i.i.i:                                       ; preds = %bb9.i.i.i.i, %bb6.i.i.i35.i, %bb5.i.i.i
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %_97.sroa.12.0.copyload.i, i64 %71
  %_6.i.i.i.i.i = icmp samesign eq i64 %71, %_97.sroa.13.0.copyload.i
  br i1 %_6.i.i.i.i.i, label %bb49.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb19.i.i.i
  %x.i.i.i.i = load i8, ptr %data.i.i.i.i, align 1, !noalias !105, !noundef !3
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i, i64 1
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !105, !noundef !3
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %76 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i21.i.i.i, label %bb49.loopexit.loopexit.i

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb49.loopexit.loopexit.i

bb6.i21.i.i.i:                                    ; preds = %bb4.i.i.i.i
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i, i64 2
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !105, !noundef !3
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %77 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb49.loopexit.loopexit.i

bb8.i.i.i.i:                                      ; preds = %bb6.i21.i.i.i
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i, i64 3
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_48.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !105, !noundef !3
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %78 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb49.loopexit.loopexit.i

bb18.i.i.i:                                       ; preds = %bb9.i.i.i.i, %bb6.i.i.i35.i, %bb9.i.i.i.peel.i, %bb6.i.i.i35.peel.i, %bb9.i.i.i.i.peel, %bb6.i.i.i35.i.peel, %bb9.i.i.i.peel.i.peel, %bb6.i.i.i35.peel.i.peel
  %iter.sroa.4.1.lcssa.i = phi i64 [ %_97.sroa.4.0.copyload.i, %bb6.i.i.i35.peel.i.peel ], [ %_97.sroa.4.0.copyload.i, %bb9.i.i.i.peel.i.peel ], [ %53, %bb6.i.i.i35.i.peel ], [ %53, %bb9.i.i.i.i.peel ], [ %71, %bb6.i.i.i35.i ], [ %71, %bb9.i.i.i.i ], [ %last_end.sroa.0.0103.i, %bb6.i.i.i35.peel.i ], [ %last_end.sroa.0.0103.i, %bb9.i.i.i.peel.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_97.sroa.12.0.copyload.i, i64 noundef %_97.sroa.13.0.copyload.i, i64 noundef %iter.sroa.4.1.lcssa.i, i64 noundef %_97.sroa.13.0.copyload.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_559c4f386b668c946885822cef1a587d) #27
          to label %.noexc.i unwind label %cleanup10.loopexit.split-lp.i, !noalias !56

.noexc.i:                                         ; preds = %bb18.i.i.i
  unreachable

panic8.i98.i:                                     ; preds = %bb42.i95.us.i
  %79 = add i64 %19, %_0.sroa.0.0.i.i90.us.i
  %umax.i99.i = call i64 @llvm.umax.i64(i64 range(i64 0, -9223372036854775808) %_97.sroa.13.0.copyload.i, i64 %79)
  br label %panic.i.invoke.i

panic.i.invoke.i:                                 ; preds = %bb26.i.us.i.preheader.split, %bb27.i120.us.i, %bb46.i116.us.i, %bb46.i.us.i.us, %panic8.i.i, %panic8.i98.i
  %80 = phi i64 [ %umax.i.i, %panic8.i.i ], [ %umax.i99.i, %panic8.i98.i ], [ %_45.i.us.i.us, %bb46.i.us.i.us ], [ %_45.i121.us.i, %bb27.i120.us.i ], [ %_73.i117.us.i, %bb46.i116.us.i ], [ %18, %bb26.i.us.i.preheader.split ]
  %81 = phi i64 [ %_97.sroa.13.0.copyload.i, %panic8.i.i ], [ %_97.sroa.13.0.copyload.i, %panic8.i98.i ], [ %_97.sroa.13.0.copyload.i, %bb46.i.us.i.us ], [ %_97.sroa.13.0.copyload.i, %bb27.i120.us.i ], [ %_97.sroa.15.0.copyload.i, %bb46.i116.us.i ], [ %_97.sroa.15.0.copyload.i, %bb26.i.us.i.preheader.split ]
  %82 = phi ptr [ @alloc_cfc145f12794171662ae0bd5e97799ce, %panic8.i.i ], [ @alloc_cfc145f12794171662ae0bd5e97799ce, %panic8.i98.i ], [ @alloc_759b6db6182a2ae5f8169b55f322d553, %bb46.i.us.i.us ], [ @alloc_759b6db6182a2ae5f8169b55f322d553, %bb27.i120.us.i ], [ @alloc_3c3a438693b52af6c6b31c2cc77620da, %bb46.i116.us.i ], [ @alloc_3c3a438693b52af6c6b31c2cc77620da, %bb26.i.us.i.preheader.split ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %80, i64 noundef range(i64 0, -9223372036854775808) %81, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %82) #27
          to label %panic.i.cont.i unwind label %cleanup10.loopexit.split-lp.i, !noalias !56

panic.i.cont.i:                                   ; preds = %panic.i.invoke.i
  unreachable

panic8.i.i:                                       ; preds = %bb42.i.us.i
  %83 = add i64 %30, %_97.sroa.4.0.copyload.i
  %umax.i.i = call i64 @llvm.umax.i64(i64 range(i64 0, -9223372036854775808) %_97.sroa.13.0.copyload.i, i64 %83)
  br label %panic.i.invoke.i

cleanup10.loopexit.split.i.loopexit:              ; preds = %bb1.i.i48.i, %bb1.i.i60.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

cleanup10.loopexit.split.i.loopexit.split-lp:     ; preds = %bb1.i.i48.i.peel, %bb1.i.i60.i.peel
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

cleanup10.loopexit.split-lp.i:                    ; preds = %panic.i.invoke.i, %bb18.i.i.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %bb25.i

bb56.i:                                           ; preds = %bb19.i.i.peel.i, %bb8.i.us.i, %bb9.i.us.i, %bb37.sink.split.i.us.i, %bb37.i.us.i, %bb21.i.i.i.peel, %bb40.split.i
  %_10.i38167.i = phi ptr [ %_4.sroa.10.0.i.i, %bb40.split.i ], [ %_4.sroa.10.0.i.i, %bb21.i.i.i.peel ], [ %_10.i58.us162.i, %bb37.i.us.i ], [ %_10.i58.us162.i, %bb37.sink.split.i.us.i ], [ %_10.i58.us162.i, %bb9.i.us.i ], [ %_10.i58.us162.i, %bb8.i.us.i ], [ %_10.i58155.i, %bb19.i.i.peel.i ]
  %len.i.i.i = phi i64 [ 0, %bb40.split.i ], [ 0, %bb21.i.i.i.peel ], [ %len.i.i40.us.i, %bb37.i.us.i ], [ %len.i.i40.us.i, %bb37.sink.split.i.us.i ], [ %len.i.i40.us.i, %bb9.i.us.i ], [ %len.i.i40.us.i, %bb8.i.us.i ], [ %len.i.i40.i, %bb19.i.i.peel.i ]
  %last_end.sroa.0.077.i = phi i64 [ 0, %bb40.split.i ], [ 0, %bb21.i.i.i.peel ], [ %last_end.sroa.0.0.us.i, %bb37.i.us.i ], [ %last_end.sroa.0.0.us.i, %bb37.sink.split.i.us.i ], [ %last_end.sroa.0.0.us.i, %bb9.i.us.i ], [ %last_end.sroa.0.0.us.i, %bb8.i.us.i ], [ %_97.sroa.13.0.copyload.i, %bb19.i.i.peel.i ]
  %gepdiff38.i = sub nuw nsw i64 89, %last_end.sroa.0.077.i
  call void @llvm.experimental.noalias.scope.decl(metadata !116)
  %self2.i.i.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !119, !noalias !56, !noundef !3
  %_9.i.i.i = sub i64 %self2.i.i.i, %len.i.i.i
  %_7.i.i.i = icmp ugt i64 %gepdiff38.i, %_9.i.i.i
  br i1 %_7.i.i.i, label %bb1.i.i.i, label %bb5, !prof !41

bb1.i.i.i:                                        ; preds = %bb56.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %len.i.i.i, i64 noundef %gepdiff38.i)
          to label %.noexc39.i unwind label %cleanup9.i

.noexc39.i:                                       ; preds = %bb1.i.i.i
  %len.pre.i.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !116, !noalias !56
  %_10.i38.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !116, !noalias !56
  br label %bb5

bb49.loopexit.loopexit.i:                         ; preds = %bb8.i.i.i.i, %bb6.i21.i.i.i, %bb3.i.i.i.i, %bb4.i.i.i.i
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %76, %bb4.i.i.i.i ], [ %77, %bb6.i21.i.i.i ], [ %78, %bb8.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i.i ]
  %84 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  call void @llvm.assume(i1 %84)
  br label %bb49.i

bb49.i:                                           ; preds = %bb49.loopexit.loopexit.i, %bb19.i.i.i
  %iter.sroa.4.1145.i = phi i64 [ %71, %bb49.loopexit.loopexit.i ], [ %_97.sroa.13.0.copyload.i, %bb19.i.i.i ]
  %data13.i = getelementptr inbounds nuw i8, ptr @alloc_a98ee8a049a1c25292c235225752f6c8, i64 %last_end.sroa.0.0103.i
  %gepdiff.i = sub nuw nsw i64 %iter.sroa.4.1145.i, %last_end.sroa.0.0103.i
  call void @llvm.experimental.noalias.scope.decl(metadata !81)
  %self2.i.i41.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !84, !noalias !56, !noundef !3
  %_9.i.i42.i = sub i64 %self2.i.i41.i, %len.i.i40.i
  %_7.i.i43.i = icmp ugt i64 %gepdiff.i, %_9.i.i42.i
  br i1 %_7.i.i43.i, label %bb1.i.i48.i, label %bb51.i, !prof !41

bb1.i.i48.i:                                      ; preds = %bb49.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %len.i.i40.i, i64 noundef %gepdiff.i)
          to label %.noexc50.i unwind label %cleanup10.loopexit.split.i.loopexit

.noexc50.i:                                       ; preds = %bb1.i.i48.i
  %len.pre.i49.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  %_10.i46.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  %self2.i.i53.pre.i = load i64, ptr %result.i, align 8, !range !8, !alias.scope !87, !noalias !56
  br label %bb51.i

bb51.i:                                           ; preds = %.noexc50.i, %bb49.i
  %_10.i58154.i = phi ptr [ %_10.i58155.i, %bb49.i ], [ %_10.i46.pre.i, %.noexc50.i ]
  %self2.i.i53.i = phi i64 [ %self2.i.i41.i, %bb49.i ], [ %self2.i.i53.pre.i, %.noexc50.i ]
  %len.i44.i = phi i64 [ %len.i.i40.i, %bb49.i ], [ %len.pre.i49.i, %.noexc50.i ]
  %_9.i45.i = icmp sgt i64 %len.i44.i, -1
  call void @llvm.assume(i1 %_9.i45.i)
  %dst.i47.i = getelementptr inbounds nuw i8, ptr %_10.i58154.i, i64 %len.i44.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i47.i, ptr nonnull readonly align 1 %data13.i, i64 %gepdiff.i, i1 false), !noalias !92
  %85 = add i64 %len.i44.i, %gepdiff.i
  store i64 %85, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !81, !noalias !56
  call void @llvm.experimental.noalias.scope.decl(metadata !93)
  %_9.i.i54.i = sub i64 %self2.i.i53.i, %85
  %_7.i.i55.i = icmp ugt i64 %patch_version.sroa.11.0.copyload, %_9.i.i54.i
  br i1 %_7.i.i55.i, label %bb1.i.i60.i, label %bb50.i, !prof !41

bb1.i.i60.i:                                      ; preds = %bb51.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(24) %result.i, i64 noundef %85, i64 noundef %patch_version.sroa.11.0.copyload)
          to label %.noexc62.i unwind label %cleanup10.loopexit.split.i.loopexit

.noexc62.i:                                       ; preds = %bb1.i.i60.i
  %len.pre.i61.i = load i64, ptr %_94.sroa.5.0.result.sroa_idx.i, align 8, !alias.scope !93, !noalias !56
  %_10.i58.pre.i = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !alias.scope !93, !noalias !56
  br label %bb50.i

bb50.i:                                           ; preds = %.noexc62.i, %bb51.i
  %_10.i58.i = phi ptr [ %_10.i58154.i, %bb51.i ], [ %_10.i58.pre.i, %.noexc62.i ]
  %len.i56.i = phi i64 [ %85, %bb51.i ], [ %len.pre.i61.i, %.noexc62.i ]
  %_9.i57.i = icmp sgt i64 %len.i56.i, -1
  call void @llvm.assume(i1 %_9.i57.i)
  %dst.i59.i = getelementptr inbounds nuw i8, ptr %_10.i58.i, i64 %len.i56.i
  br label %bb5.i.lr.ph.i.i, !llvm.loop !122

bb76:                                             ; preds = %bb2.i.i.i4.i.i139, %bb79, %cleanup3, %bb2.i.i.i4.i.i93, %bb2.i.i.i4.i.i.i.i88, %cleanup.i, %cleanup.i, %cleanup1.i, %cleanup.body.i, %bb2.i.i.i4.i.i.i.i.i68, %cleanup.i.i, %cleanup.i36, %bb2.i.i.i4.i.i.i.i.i, %bb7.i, %bb80.thread167, %cleanup1, %bb2.i.i.i4.i.i.i, %bb25.i
  %.pn.pn = phi { ptr, i32 } [ %87, %cleanup1 ], [ %.pn.i, %bb2.i.i.i4.i.i.i ], [ %.pn.i, %bb25.i ], [ %lpad.thr_comm, %bb80.thread167 ], [ %93, %bb7.i ], [ %93, %bb2.i.i.i4.i.i.i.i.i ], [ %98, %cleanup.i36 ], [ %101, %bb2.i.i.i4.i.i.i.i.i68 ], [ %101, %cleanup.i.i ], [ %129, %cleanup1.i ], [ %eh.lpad-body.i, %cleanup.body.i ], [ %165, %cleanup.i ], [ %165, %cleanup.i ], [ %165, %bb2.i.i.i4.i.i.i.i88 ], [ %169, %bb2.i.i.i4.i.i93 ], [ %169, %cleanup3 ], [ %lpad.thr_comm.split-lp, %bb79 ], [ %lpad.thr_comm.split-lp, %bb2.i.i.i4.i.i139 ]
  %86 = icmp eq i64 %patch_version.sroa.0.0.copyload, 0
  br i1 %86, label %bb77, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb76
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %patch_version.sroa.7.0.copyload, i64 noundef %patch_version.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb77

cleanup1:                                         ; preds = %bb3.i.i
  %87 = landingpad { ptr, i32 }
          cleanup
  br label %bb76

bb5:                                              ; preds = %.noexc39.i, %bb56.i
  %_10.i38.i = phi ptr [ %_10.i38167.i, %bb56.i ], [ %_10.i38.pre.i, %.noexc39.i ]
  %len.i.i = phi i64 [ %len.i.i.i, %bb56.i ], [ %len.pre.i.i, %.noexc39.i ]
  %data.i = getelementptr inbounds nuw i8, ptr @alloc_a98ee8a049a1c25292c235225752f6c8, i64 %last_end.sroa.0.077.i
  %_9.i.i = icmp sgt i64 %len.i.i, -1
  call void @llvm.assume(i1 %_9.i.i)
  %dst.i.i = getelementptr inbounds nuw i8, ptr %_10.i38.i, i64 %len.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i, ptr nonnull readonly align 1 %data.i, i64 %gepdiff38.i, i1 false), !noalias !124
  %module.sroa.0.0.copyload141 = load i64, ptr %result.i, align 8, !noalias !125
  %module.sroa.6.0.copyload142 = load ptr, ptr %_94.sroa.4.0.result.sroa_idx.i, align 8, !noalias !125
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %result.i), !noalias !56
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_12)
  %88 = icmp ne ptr %_4.sroa.4.0.copyload, null
  call void @llvm.assume(i1 %88)
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_12, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_4.sroa.4.0.copyload, i64 noundef %_4.sroa.5.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_589d53fa4508e5cc88338e60f7602ef2, i64 noundef 10)
          to label %bb6 unwind label %bb79

bb80.thread167:                                   ; preds = %bb22, %bb21, %bb20, %bb19, %bb18, %bb17, %bb16, %bb15, %bb14, %bb13, %bb8, %bb27.i, %bb24
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
  br label %bb76

bb6:                                              ; preds = %bb5
  %89 = add nuw i64 %len.i.i, %gepdiff38.i
  call void @llvm.experimental.noalias.scope.decl(metadata !126)
  %90 = getelementptr inbounds nuw i8, ptr %_12, i64 8
  %path.val9.i = load ptr, ptr %90, align 8, !alias.scope !126, !noalias !129, !nonnull !3, !noundef !3
  %91 = getelementptr inbounds nuw i8, ptr %_12, i64 16
  %path.val10.i = load i64, ptr %91, align 8, !alias.scope !126, !noalias !129, !noundef !3
  %92 = icmp ne ptr %module.sroa.6.0.copyload142, null
  call void @llvm.assume(i1 %92)
; invoke std::fs::write::inner
  %_0.i = invoke noundef ptr @_RNvNvNtCs5sEH5CPMdak_3std2fs5write5inner(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %path.val9.i, i64 noundef %path.val10.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %module.sroa.6.0.copyload142, i64 noundef %89)
          to label %bb3.i unwind label %cleanup.i61, !noalias !131

cleanup.i61:                                      ; preds = %bb6
  %93 = landingpad { ptr, i32 }
          cleanup
  %94 = icmp eq i64 %module.sroa.0.0.copyload141, 0
  br i1 %94, label %bb7.i, label %bb2.i.i.i4.i.i.i62

bb2.i.i.i4.i.i.i62:                               ; preds = %cleanup.i61
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %module.sroa.6.0.copyload142, i64 noundef %module.sroa.0.0.copyload141, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !131
  br label %bb7.i

bb3.i:                                            ; preds = %bb6
  %95 = icmp eq i64 %module.sroa.0.0.copyload141, 0
  br i1 %95, label %bb4.i, label %bb2.i.i.i4.i.i13.i

bb2.i.i.i4.i.i13.i:                               ; preds = %bb3.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %module.sroa.6.0.copyload142, i64 noundef %module.sroa.0.0.copyload141, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !131
  br label %bb4.i

bb7.i:                                            ; preds = %bb2.i.i.i4.i.i.i62, %cleanup.i61
  %path.val.i = load i64, ptr %_12, align 8, !alias.scope !126, !noalias !129
  %96 = icmp eq i64 %path.val.i, 0
  br i1 %96, label %bb76, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb7.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %path.val9.i, i64 noundef %path.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !131
  br label %bb76

bb4.i:                                            ; preds = %bb2.i.i.i4.i.i13.i, %bb3.i
  %path.val4.i = load i64, ptr %_12, align 8, !alias.scope !126, !noalias !129
  %97 = icmp eq i64 %path.val4.i, 0
  br i1 %97, label %bb7, label %bb2.i.i.i4.i.i.i.i15.i

bb2.i.i.i4.i.i.i.i15.i:                           ; preds = %bb4.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %path.val9.i, i64 noundef %path.val4.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !131
  br label %bb7

bb7:                                              ; preds = %bb2.i.i.i4.i.i.i.i15.i, %bb4.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12)
  %.not.i = icmp eq ptr %_0.i, null
  br i1 %.not.i, label %bb8, label %bb2.i35, !prof !132

bb2.i35:                                          ; preds = %bb7
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %e.i34)
  store ptr %_0.i, ptr %e.i34, align 8
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i34, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d7a955ea474cc242b6aa56db3ecaf3a9) #25
          to label %unreachable.i39 unwind label %cleanup.i36

cleanup.i36:                                      ; preds = %bb2.i35
  %98 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(8) %e.i34) #23
          to label %bb76 unwind label %terminate.i37

unreachable.i39:                                  ; preds = %bb2.i35
  unreachable

terminate.i37:                                    ; preds = %cleanup.i36
  %99 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb8:                                              ; preds = %bb7
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i66)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i66, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5)
          to label %.noexc77 unwind label %bb80.thread167

.noexc77:                                         ; preds = %bb8
  %100 = load i64, ptr %_2.i66, align 8, !range !10, !noundef !3
  %.not.i67 = icmp eq i64 %100, -9223372036854775808
  br i1 %.not.i67, label %bb20.i, label %bb21.i

bb21.i:                                           ; preds = %.noexc77
  %_26.sroa.5.0._2.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i66, i64 8
  %_26.sroa.5.0.copyload.i = load ptr, ptr %_26.sroa.5.0._2.sroa_idx.i, align 8, !nonnull !3, !noundef !3
  %_26.sroa.6.0._2.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i66, i64 16
  %_26.sroa.6.0.copyload.i = load i64, ptr %_26.sroa.6.0._2.sroa_idx.i, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i66)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_7.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_10.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i.i), !noalias !133
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_26.sroa.5.0.copyload.i, i64 noundef %_26.sroa.6.0.copyload.i)
          to label %bb2.i.i unwind label %cleanup.i.i, !noalias !133

cleanup.i.i:                                      ; preds = %bb21.i
  %101 = landingpad { ptr, i32 }
          cleanup
  %102 = icmp eq i64 %100, 0
  br i1 %102, label %bb76, label %bb2.i.i.i4.i.i.i.i.i68

bb2.i.i.i4.i.i.i.i.i68:                           ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_26.sroa.5.0.copyload.i, i64 noundef %100, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !133
  br label %bb76

bb2.i.i:                                          ; preds = %bb21.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %_10.i, ptr noundef nonnull align 8 dereferenceable(200) %_2.i.i, i64 200, i1 false), !noalias !137
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i.i), !noalias !133
  %103 = icmp eq i64 %100, 0
  br i1 %103, label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build.exit.i, label %bb2.i.i.i4.i.i.i6.i.i

bb2.i.i.i4.i.i.i6.i.i:                            ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_26.sroa.5.0.copyload.i, i64 noundef %100, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !133
  br label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build.exit.i

_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build.exit.i: ; preds = %bb2.i.i.i4.i.i.i6.i.i, %bb2.i.i
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %_10.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a887f9858119cc7413062dc002c4d9ab, i64 noundef 9)
          to label %bb4.i71 unwind label %cleanup.i69

bb20.i:                                           ; preds = %.noexc77
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i66)
  br label %bb11

cleanup.i69:                                      ; preds = %bb4.i71, %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build.exit.i
  %104 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i, %cleanup.i69
  %eh.lpad-body.i = phi { ptr, i32 } [ %104, %cleanup.i69 ], [ %121, %bb1.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_10.i) #23
          to label %bb76 unwind label %terminate.i70

bb4.i71:                                          ; preds = %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build.exit.i
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_7.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_10.i)
          to label %bb5.i72 unwind label %cleanup.i69

bb5.i72:                                          ; preds = %bb4.i71
  %105 = load i64, ptr %_7.i, align 8, !range !10, !noundef !3
  %106 = icmp eq i64 %105, -9223372036854775808
  %107 = getelementptr inbounds nuw i8, ptr %_7.i, i64 8
  br i1 %106, label %bb3.i.i76, label %bb28.i

bb3.i.i76:                                        ; preds = %bb5.i72
  call void @llvm.experimental.noalias.scope.decl(metadata !138)
  %_1.val.i21.i = load ptr, ptr %107, align 8, !alias.scope !138, !nonnull !3, !noundef !3
  %bits.i.i.i.i.i = ptrtoint ptr %_1.val.i21.i to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %bb27.i, !prof !20

bb2.i2.i.i.i.i:                                   ; preds = %bb3.i.i76
  %108 = getelementptr i8, ptr %_1.val.i21.i, i64 -1
  %109 = icmp ne ptr %108, null
  call void @llvm.assume(i1 %109)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %108, align 8, !noalias !138
  %110 = getelementptr i8, ptr %_1.val.i21.i, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %110, align 8, !noalias !138, !nonnull !3, !align !7, !noundef !3
  %111 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !3, !noalias !138
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %111, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %112 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %112)
  invoke void %111(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i, !noalias !138

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %113 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %113)
  %114 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %115 = load i64, ptr %114, align 8, !range !8, !invariant.load !3, !noalias !138
  %116 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %117 = load i64, ptr %116, align 8, !range !9, !invariant.load !3, !noalias !138
  %118 = add i64 %117, -1
  %119 = icmp sgt i64 %118, -1
  call void @llvm.assume(i1 %119)
  %120 = icmp eq i64 %115, 0
  br i1 %120, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %115, i64 noundef range(i64 1, -9223372036854775807) %117) #22, !noalias !138
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %121 = landingpad { ptr, i32 }
          cleanup
  %122 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %123 = load i64, ptr %122, align 8, !range !8, !invariant.load !3, !noalias !138
  %124 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %125 = load i64, ptr %124, align 8, !range !9, !invariant.load !3, !noalias !138
  %126 = add i64 %125, -1
  %127 = icmp sgt i64 %126, -1
  call void @llvm.assume(i1 %127)
  %128 = icmp eq i64 %123, 0
  br i1 %128, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %123, i64 noundef range(i64 1, -9223372036854775807) %125) #22, !noalias !138
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %108, i64 noundef 24, i64 noundef 8) #22, !noalias !138
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %108, i64 noundef 24, i64 noundef 8) #22, !noalias !138
  br label %bb27.i

bb28.i:                                           ; preds = %bb5.i72
  %val.sroa.4.0.output.sroa_idx.i = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val.sroa.4.0.output.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %107, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_7.i)
  store i64 %105, ptr %output.i, align 8
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_10.i)
          to label %bb6.i unwind label %cleanup1.i

bb27.i:                                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECshitNtYJEXnW_18build_script_build.exit.i.i.i.i.i, %bb3.i.i76
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_7.i)
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_10.i)
          to label %.noexc81 unwind label %bb80.thread167

.noexc81:                                         ; preds = %bb27.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_10.i)
  br label %bb14.i

bb14.i:                                           ; preds = %bb2.i.i.i4.i7.i18.i, %bb4.i16.i, %.noexc81
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br label %bb11

cleanup1.i:                                       ; preds = %bb9.i, %bb31.i, %bb6.i, %bb28.i
  %129 = landingpad { ptr, i32 }
          cleanup
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i) #23
  br label %bb76

bb6.i:                                            ; preds = %bb28.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_10.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_14.i)
  %_35.i = load ptr, ptr %val.sroa.4.0.output.sroa_idx.i, align 8, !nonnull !3, !noundef !3
  %130 = getelementptr inbounds nuw i8, ptr %output.i, i64 16
  %_34.i = load i64, ptr %130, align 8, !noundef !3
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_14.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_35.i, i64 noundef %_34.i)
          to label %bb7.i73 unwind label %cleanup1.i

bb7.i73:                                          ; preds = %bb6.i
  %_36.i = load i64, ptr %_14.i, align 8, !range !37, !noundef !3
  %131 = trunc nuw i64 %_36.i to i1
  br i1 %131, label %bb29.i, label %bb31.i

bb29.i:                                           ; preds = %bb7.i73
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_14.i)
  br label %bb12.i

bb31.i:                                           ; preds = %bb7.i73
  %132 = getelementptr inbounds nuw i8, ptr %_14.i, i64 8
  %_37.0.i = load ptr, ptr %132, align 8, !nonnull !3, !align !17, !noundef !3
  %133 = getelementptr inbounds nuw i8, ptr %_14.i, i64 16
  %_37.1.i = load i64, ptr %133, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_14.i)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %pieces.i)
  store i64 0, ptr %pieces.i, align 8
  %_39.sroa.4.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 8
  store i64 %_37.1.i, ptr %_39.sroa.4.0.pieces.sroa_idx.i, align 8
  %_39.sroa.5.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 16
  store ptr %_37.0.i, ptr %_39.sroa.5.0.pieces.sroa_idx.i, align 8
  %_39.sroa.5.sroa.4.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 24
  store i64 %_37.1.i, ptr %_39.sroa.5.sroa.4.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_39.sroa.5.sroa.5.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 32
  store i64 0, ptr %_39.sroa.5.sroa.5.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_39.sroa.5.sroa.6.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 40
  store i64 %_37.1.i, ptr %_39.sroa.5.sroa.6.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_39.sroa.5.sroa.7.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 48
  store <2 x i32> splat (i32 46), ptr %_39.sroa.5.sroa.7.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_39.sroa.5.sroa.9.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 56
  store i8 1, ptr %_39.sroa.5.sroa.9.0._39.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_39.sroa.6.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 64
  store i8 1, ptr %_39.sroa.6.0.pieces.sroa_idx.i, align 8
  %_39.sroa.7.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 65
  store i8 0, ptr %_39.sroa.7.0.pieces.sroa_idx.i, align 1
; invoke <core::str::iter::SplitInternal<char>>::next
  %134 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb32.i unwind label %cleanup1.i

bb32.i:                                           ; preds = %bb31.i
  %135 = extractvalue { ptr, i64 } %134, 0
  %.not8.i = icmp ne ptr %135, null
  %136 = extractvalue { ptr, i64 } %134, 1
  %_3.not.i.i = icmp eq i64 %136, 7
  %or.cond43.i = select i1 %.not8.i, i1 %_3.not.i.i, i1 false
  br i1 %or.cond43.i, label %bb35.i, label %bb8.i74

bb8.i74:                                          ; preds = %bb35.i, %bb32.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  br label %bb12.i

bb35.i:                                           ; preds = %bb32.i
  %137 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %135, ptr noundef nonnull dereferenceable(7) @alloc_ca36d7e792bb4bbd1a68749f90007ce8, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !141
  %138 = icmp eq i32 %137, 0
  br i1 %138, label %bb9.i, label %bb8.i74

bb9.i:                                            ; preds = %bb35.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %139 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb36.i unwind label %cleanup1.i

bb36.i:                                           ; preds = %bb9.i
  %140 = extractvalue { ptr, i64 } %139, 0
  %.not9.i = icmp eq ptr %140, null
  br i1 %.not9.i, label %bb37.i, label %bb38.i

bb38.i:                                           ; preds = %bb36.i
  %141 = extractvalue { ptr, i64 } %139, 1
  switch i64 %141, label %bb9thread-pre-split.i.i [
    i64 0, label %bb39.i
    i64 1, label %bb7.i.i
  ]

bb7.i.i:                                          ; preds = %bb38.i
  %142 = load i8, ptr %140, align 1, !alias.scope !145, !noundef !3
  switch i8 %142, label %bb9.i.i [
    i8 43, label %bb39.i
    i8 45, label %bb39.i
  ]

bb9thread-pre-split.i.i:                          ; preds = %bb38.i
  %.pr.i.i = load i8, ptr %140, align 1, !alias.scope !145
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb9thread-pre-split.i.i, %bb7.i.i
  %143 = phi i8 [ %.pr.i.i, %bb9thread-pre-split.i.i ], [ %142, %bb7.i.i ]
  %cond.i.i = icmp eq i8 %143, 43
  %rest.1.i.i = sext i1 %cond.i.i to i64
  %src.sroa.15.0.i.i = add nsw i64 %141, %rest.1.i.i
  %src.sroa.0.0.idx.i.i = zext i1 %cond.i.i to i64
  %src.sroa.0.0.i.i = getelementptr inbounds nuw i8, ptr %140, i64 %src.sroa.0.0.idx.i.i
  %_10.i.i = icmp samesign ult i64 %src.sroa.15.0.i.i, 9
  br i1 %_10.i.i, label %bb15.preheader.i.i, label %bb22.i.i

bb15.preheader.i.i:                               ; preds = %bb9.i.i
  %_13.not56.i.i = icmp eq i64 %src.sroa.15.0.i.i, 0
  br i1 %_13.not56.i.i, label %bb39.i, label %bb16.i.i

bb22.i.i:                                         ; preds = %bb9.i.i, %bb33.i.i
  %result.sroa.0.0.i.i = phi i32 [ %_60.0.i.i, %bb33.i.i ], [ 0, %bb9.i.i ]
  %src.sroa.15.1.i.i = phi i64 [ %rest.12.i.i, %bb33.i.i ], [ %src.sroa.15.0.i.i, %bb9.i.i ]
  %src.sroa.0.1.i.i = phi ptr [ %rest.01.i.i, %bb33.i.i ], [ %src.sroa.0.0.i.i, %bb9.i.i ]
  %_28.not.i.not.i = icmp eq i64 %src.sroa.15.1.i.i, 0
  br i1 %_28.not.i.not.i, label %bb39.i, label %bb23.i.i

bb23.i.i:                                         ; preds = %bb22.i.i
  %144 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i.i, i32 10)
  %_57.1.i.i = extractvalue { i32, i1 } %144, 1
  br i1 %_57.1.i.i, label %bb39.i, label %bb33.i.i, !prof !41

bb33.i.i:                                         ; preds = %bb23.i.i
  %_57.0.i.i = extractvalue { i32, i1 } %144, 0
  %rest.12.i.i = add nsw i64 %src.sroa.15.1.i.i, -1
  %rest.01.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i, i64 1
  %145 = load i8, ptr %src.sroa.0.1.i.i, align 1, !alias.scope !145, !noundef !3
  %146 = zext i8 %145 to i32
  %147 = add nsw i32 %146, -48
  %_14.i.i.i = icmp ugt i32 %147, 9
  %_60.0.i.i = add i32 %147, %_57.0.i.i
  %_60.1.i.i = icmp ult i32 %_60.0.i.i, %_57.0.i.i
  %or.cond.i = select i1 %_14.i.i.i, i1 true, i1 %_60.1.i.i
  br i1 %or.cond.i, label %bb39.i, label %bb22.i.i, !prof !148

bb16.i.i:                                         ; preds = %bb15.preheader.i.i, %bb20.i.i
  %src.sroa.0.259.i.i = phi ptr [ %rest.04.i.i, %bb20.i.i ], [ %src.sroa.0.0.i.i, %bb15.preheader.i.i ]
  %src.sroa.15.258.i.i = phi i64 [ %rest.15.i.i, %bb20.i.i ], [ %src.sroa.15.0.i.i, %bb15.preheader.i.i ]
  %result.sroa.0.257.i.i = phi i32 [ %150, %bb20.i.i ], [ 0, %bb15.preheader.i.i ]
  %_19.i.i = load i8, ptr %src.sroa.0.259.i.i, align 1, !alias.scope !145, !noundef !3
  %_18.i.i = zext i8 %_19.i.i to i32
  %148 = add nsw i32 %_18.i.i, -48
  %_14.i47.i.i = icmp ult i32 %148, 10
  br i1 %_14.i47.i.i, label %bb20.i.i, label %bb39.i

bb20.i.i:                                         ; preds = %bb16.i.i
  %149 = mul i32 %result.sroa.0.257.i.i, 10
  %rest.15.i.i = add nsw i64 %src.sroa.15.258.i.i, -1
  %rest.04.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i, i64 1
  %150 = add i32 %148, %149
  %_13.not.i.i = icmp eq i64 %rest.15.i.i, 0
  br i1 %_13.not.i.i, label %bb39.i, label %bb16.i.i

bb37.i:                                           ; preds = %bb36.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br label %bb11

bb39.i:                                           ; preds = %bb33.i.i, %bb23.i.i, %bb22.i.i, %bb20.i.i, %bb16.i.i, %bb15.preheader.i.i, %bb7.i.i, %bb7.i.i, %bb38.i
  %not._0.sroa.0.0.i.i = phi i1 [ true, %bb15.preheader.i.i ], [ false, %bb7.i.i ], [ false, %bb7.i.i ], [ false, %bb38.i ], [ %_14.i47.i.i, %bb16.i.i ], [ %_14.i47.i.i, %bb20.i.i ], [ %_28.not.i.not.i, %bb22.i.i ], [ %_28.not.i.not.i, %bb23.i.i ], [ %_28.not.i.not.i, %bb33.i.i ]
  %151 = phi i32 [ 0, %bb15.preheader.i.i ], [ undef, %bb7.i.i ], [ undef, %bb7.i.i ], [ undef, %bb38.i ], [ %150, %bb20.i.i ], [ undef, %bb16.i.i ], [ undef, %bb33.i.i ], [ undef, %bb23.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !149)
  %152 = icmp eq i64 %105, 0
  br i1 %152, label %bb4.i.i, label %bb2.i.i.i4.i.i.i75

bb2.i.i.i4.i.i.i75:                               ; preds = %bb39.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.i, i64 noundef %105, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !149
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i75, %bb39.i
  %153 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %153, align 8, !alias.scope !149
  %154 = icmp eq i64 %.val2.i.i, 0
  br i1 %154, label %bb9, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i.i
  %155 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %155, align 8, !alias.scope !149, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !149
  br label %bb9

bb12.i:                                           ; preds = %bb8.i74, %bb29.i
  call void @llvm.experimental.noalias.scope.decl(metadata !152)
  %156 = icmp eq i64 %105, 0
  br i1 %156, label %bb4.i16.i, label %bb2.i.i.i4.i.i14.i

bb2.i.i.i4.i.i14.i:                               ; preds = %bb12.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_35.i, i64 noundef %105, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !152
  br label %bb4.i16.i

bb4.i16.i:                                        ; preds = %bb2.i.i.i4.i.i14.i, %bb12.i
  %157 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i17.i = load i64, ptr %157, align 8, !alias.scope !152
  %158 = icmp eq i64 %.val2.i17.i, 0
  br i1 %158, label %bb14.i, label %bb2.i.i.i4.i7.i18.i

bb2.i.i.i4.i7.i18.i:                              ; preds = %bb4.i16.i
  %159 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i19.i = load ptr, ptr %159, align 8, !alias.scope !152, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i19.i, i64 noundef %.val2.i17.i, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !152
  br label %bb14.i

terminate.i70:                                    ; preds = %cleanup.body.i
  %160 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #24
  unreachable

bb9:                                              ; preds = %bb4.i.i, %bb2.i.i.i4.i7.i.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br i1 %not._0.sroa.0.0.i.i, label %bb12, label %bb11

bb12:                                             ; preds = %bb9
  %_18 = icmp ugt i32 %151, 76
  br i1 %_18, label %bb13, label %bb24

bb11:                                             ; preds = %bb20.i, %bb37.i, %bb14.i, %bb9
  %161 = icmp eq i64 %patch_version.sroa.0.0.copyload, 0
  br i1 %161, label %bb72, label %bb2.i.i.i4.i.i82

bb2.i.i.i4.i.i82:                                 ; preds = %bb11
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %patch_version.sroa.7.0.copyload, i64 noundef %patch_version.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb72

bb72:                                             ; preds = %bb2.i.i.i4.i.i82, %bb11
  %162 = icmp eq i64 %0, 0
  br i1 %162, label %bb74, label %bb74.sink.split

bb74.sink.split:                                  ; preds = %bb72, %bb70
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.4.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb74

bb74:                                             ; preds = %bb74.sink.split, %bb70, %bb72
  ret void

bb24:                                             ; preds = %bb22, %bb12
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_40)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_40, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dcbc225a8ec7dbfaaef714ff8a7176fb, i64 noundef 6)
          to label %bb25 unwind label %bb80.thread167

bb13:                                             ; preds = %bb12
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_bc9e042c014e8fa11bdfddb773364e8c, ptr noundef nonnull inttoptr (i64 113 to ptr))
          to label %bb14 unwind label %bb80.thread167

bb14:                                             ; preds = %bb13
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_38a8aab5c2b62e9751be68fe1ccf5613, ptr noundef nonnull inttoptr (i64 81 to ptr))
          to label %bb15 unwind label %bb80.thread167

bb15:                                             ; preds = %bb14
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_68978439aeb0698832af016bfe1612cd, ptr noundef nonnull inttoptr (i64 83 to ptr))
          to label %bb16 unwind label %bb80.thread167

bb16:                                             ; preds = %bb15
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_4c52a9725234699023cc0b6fe3df2bc7, ptr noundef nonnull inttoptr (i64 79 to ptr))
          to label %bb17 unwind label %bb80.thread167

bb17:                                             ; preds = %bb16
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_aa136c7cc6a3cdf54ac82998fd4794de, ptr noundef nonnull inttoptr (i64 101 to ptr))
          to label %bb18 unwind label %bb80.thread167

bb18:                                             ; preds = %bb17
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_7bab126fe70079b94cb91837d5266928, ptr noundef nonnull inttoptr (i64 103 to ptr))
          to label %bb19 unwind label %bb80.thread167

bb19:                                             ; preds = %bb18
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_cf61cd25720d051afd7dbe390badc285, ptr noundef nonnull inttoptr (i64 87 to ptr))
          to label %bb20 unwind label %bb80.thread167

bb20:                                             ; preds = %bb19
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0bc122f91da9d875d8b43d249e797349, ptr noundef nonnull inttoptr (i64 83 to ptr))
          to label %bb21 unwind label %bb80.thread167

bb21:                                             ; preds = %bb20
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_11b9a1cc79cd309ab8e05e189e755542, ptr noundef nonnull inttoptr (i64 87 to ptr))
          to label %bb22 unwind label %bb80.thread167

bb22:                                             ; preds = %bb21
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_1979714a80cbc333f412aa0c87203b40, ptr noundef nonnull inttoptr (i64 97 to ptr))
          to label %bb24 unwind label %bb80.thread167

bb25:                                             ; preds = %bb24
  call void @llvm.experimental.noalias.scope.decl(metadata !155)
  call void @llvm.experimental.noalias.scope.decl(metadata !158)
  %_2.i = load i64, ptr %_40, align 8, !range !37, !alias.scope !158, !noalias !160, !noundef !3
  %163 = trunc nuw i64 %_2.i to i1
  br i1 %163, label %bb2.i, label %bb26, !prof !41

bb2.i:                                            ; preds = %bb25
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i), !noalias !162
  %164 = getelementptr inbounds nuw i8, ptr %_40, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %e.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %164, i64 24, i1 false), !noalias !160
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f632b4be46da6b0210f55a14255fdbb7) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !163

cleanup.i:                                        ; preds = %bb2.i
  %165 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !164)
  %166 = load i64, ptr %e.i, align 8, !range !10, !alias.scope !164, !noalias !163, !noundef !3
  switch i64 %166, label %bb2.i.i.i4.i.i.i.i88 [
    i64 -9223372036854775808, label %bb76
    i64 0, label %bb76
  ]

bb2.i.i.i4.i.i.i.i88:                             ; preds = %cleanup.i
  %167 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  %_1.val1.i89 = load ptr, ptr %167, align 8, !alias.scope !164, !noalias !163, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i89, i64 noundef %166, i64 noundef range(i64 1, -9223372036854775807) 1) #22, !noalias !167
  br label %bb76

unreachable.i:                                    ; preds = %bb2.i
  unreachable

bb26:                                             ; preds = %bb25
  %168 = getelementptr inbounds nuw i8, ptr %_40, i64 8
  %target.sroa.0.0.copyload = load i64, ptr %168, align 8, !alias.scope !163, !noalias !168
  %target.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_40, i64 16
  %target.sroa.5.0.copyload = load ptr, ptr %target.sroa.5.0..sroa_idx, align 8, !alias.scope !163, !noalias !168, !nonnull !3, !noundef !3
  %target.sroa.16.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_40, i64 24
  %target.sroa.16.0.copyload = load i64, ptr %target.sroa.16.0..sroa_idx, align 8, !alias.scope !163, !noalias !168
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_40)
  switch i64 %target.sroa.16.0.copyload, label %bb29 [
    i64 24, label %bb83
    i64 25, label %bb2.i98
  ]

cleanup3:                                         ; preds = %bb66, %bb63, %bb60, %bb57, %bb54, %bb51, %bb47, %bb43, %bb30
  %169 = landingpad { ptr, i32 }
          cleanup
  %170 = icmp eq i64 %target.sroa.0.0.copyload, 0
  br i1 %170, label %bb76, label %bb2.i.i.i4.i.i93

bb2.i.i.i4.i.i93:                                 ; preds = %cleanup3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %target.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb76

bb83:                                             ; preds = %bb26
  %171 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(24) %target.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(24) @alloc_48f3a6bb4813fb278bc6f6204d79600d, i64 range(i64 0, -9223372036854775808) 24), !alias.scope !169
  %172 = icmp eq i32 %171, 0
  br label %bb29

bb2.i98:                                          ; preds = %bb26
  %173 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(25) %target.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(25) @alloc_6aff5e31c3101a31407cfa339159573c, i64 range(i64 0, -9223372036854775808) 25), !alias.scope !173
  %174 = icmp eq i32 %173, 0
  br label %bb29

bb29:                                             ; preds = %bb83, %bb26, %bb2.i98
  %emscripten.sroa.0.0.off0 = phi i1 [ %174, %bb2.i98 ], [ %172, %bb83 ], [ false, %bb26 ]
  %_43 = icmp ult i32 %151, 60
  br i1 %_43, label %bb30, label %bb50

bb50:                                             ; preds = %bb29
  %_66 = icmp eq i32 %151, 60
  br i1 %_66, label %bb51, label %bb53

bb30:                                             ; preds = %bb29
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_aee7a68eb29be7f2c6b56df8eb17cb8f, ptr noundef nonnull inttoptr (i64 75 to ptr))
          to label %bb31 unwind label %cleanup3

bb31:                                             ; preds = %bb30
  %_4.not.i = icmp samesign ult i64 %target.sroa.16.0.copyload, 6
  br i1 %_4.not.i, label %bb32, label %bb85

bb85:                                             ; preds = %bb31
  %175 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) @alloc_4a29a4faa0904cd7ff982831f2813e90, ptr noundef nonnull readonly align 1 dereferenceable(6) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !177
  %176 = icmp eq i32 %175, 0
  br i1 %176, label %bb37, label %bb86

bb32:                                             ; preds = %bb31
  %_4.not.i103 = icmp samesign ult i64 %target.sroa.16.0.copyload, 4
  br i1 %_4.not.i103, label %bb91.thread, label %bb86

bb37:                                             ; preds = %bb90, %bb89, %bb88, %bb87, %bb86, %bb85
  %177 = icmp samesign ult i32 %151, 34
  br i1 %177, label %bb43, label %bb45

bb86:                                             ; preds = %bb85, %bb32
  %178 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) @alloc_bd1f56fe9bb71b27893bc9d878ecf35c, ptr noundef nonnull readonly align 1 dereferenceable(4) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !184
  %179 = icmp eq i32 %178, 0
  br i1 %179, label %bb37, label %bb33

bb33:                                             ; preds = %bb86
  %_4.not.i108 = icmp samesign ult i64 %target.sroa.16.0.copyload, 7
  br i1 %_4.not.i108, label %bb91.thread, label %bb87

bb87:                                             ; preds = %bb33
  %180 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) @alloc_a1239a7299ff261e47dc48b0b02c8ca1, ptr noundef nonnull readonly align 1 dereferenceable(7) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !191
  %181 = icmp eq i32 %180, 0
  br i1 %181, label %bb37, label %bb34

bb34:                                             ; preds = %bb87
  %_4.not.i113 = icmp samesign ult i64 %target.sroa.16.0.copyload, 9
  br i1 %_4.not.i113, label %bb89, label %bb88

bb88:                                             ; preds = %bb34
  %182 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) @alloc_fa1130f2f45123ef906740f12b430906, ptr noundef nonnull readonly align 1 dereferenceable(9) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !198
  %183 = icmp eq i32 %182, 0
  br i1 %183, label %bb37, label %bb89

bb91.thread:                                      ; preds = %bb32, %bb33
  %184 = icmp samesign ult i32 %151, 34
  br label %bb43

bb89:                                             ; preds = %bb88, %bb34
  %185 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) @alloc_7397f20c1cb53576ae7e84fd06ee7a1e, ptr noundef nonnull readonly align 1 dereferenceable(7) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !205
  %186 = icmp eq i32 %185, 0
  br i1 %186, label %bb37, label %bb36

bb36:                                             ; preds = %bb89
  %_4.not.i123 = icmp eq i64 %target.sroa.16.0.copyload, 7
  br i1 %_4.not.i123, label %bb91, label %bb90

bb90:                                             ; preds = %bb36
  %187 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(8) @alloc_72e190fc6a3e093db1bf5535129d517c, ptr noundef nonnull readonly align 1 dereferenceable(8) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 8), !alias.scope !212
  %188 = icmp eq i32 %187, 0
  br i1 %188, label %bb37, label %bb91

bb91:                                             ; preds = %bb36, %bb90
  %189 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) @alloc_8f8dc58223e03021a07a335aedb98959, ptr noundef nonnull readonly align 1 dereferenceable(7) %target.sroa.5.0.copyload, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !219
  %190 = icmp eq i32 %189, 0
  %spec.select = select i1 %190, i1 true, i1 %emscripten.sroa.0.0.off0
  %191 = icmp samesign ult i32 %151, 34
  %.not = xor i1 %191, true
  %or.cond = and i1 %190, %.not
  br i1 %or.cond, label %bb45, label %bb43

bb43:                                             ; preds = %bb91.thread, %bb37, %bb91
  %_61.sroa.0.0.off0 = phi i1 [ true, %bb37 ], [ %191, %bb91 ], [ %184, %bb91.thread ]
  %emscripten.sroa.0.2.off0 = phi i1 [ true, %bb37 ], [ %spec.select, %bb91 ], [ %emscripten.sroa.0.0.off0, %bb91.thread ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_13e1e7fbee4ecb7b0b0c4fa67467351d, ptr noundef nonnull inttoptr (i64 65 to ptr))
          to label %bb45 unwind label %cleanup3

bb45:                                             ; preds = %bb43, %bb37, %bb91
  %_61.sroa.0.1.off0 = phi i1 [ %_61.sroa.0.0.off0, %bb43 ], [ false, %bb37 ], [ %191, %bb91 ]
  %emscripten.sroa.0.3.off0 = phi i1 [ %emscripten.sroa.0.2.off0, %bb43 ], [ true, %bb37 ], [ %spec.select, %bb91 ]
  %.not5 = xor i1 %_61.sroa.0.1.off0, true
  %or.cond7 = and i1 %emscripten.sroa.0.3.off0, %.not5
  br i1 %or.cond7, label %bb51, label %bb47

bb47:                                             ; preds = %bb45
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_2d02ffc40637a25bbd278af8d6191a7f, ptr noundef nonnull inttoptr (i64 61 to ptr))
          to label %bb51 unwind label %cleanup3

bb53:                                             ; preds = %bb50
  %_69 = icmp ult i32 %151, 64
  br i1 %_69, label %bb54, label %bb56

bb51:                                             ; preds = %bb45, %bb47, %bb50
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_7f378ac6f88a3c5251674bf739a6a81b, ptr noundef nonnull inttoptr (i64 65 to ptr))
          to label %bb54 unwind label %cleanup3

bb56:                                             ; preds = %bb53
  %_72 = icmp ult i32 %151, 74
  br i1 %_72, label %bb57, label %bb59

bb54:                                             ; preds = %bb51, %bb53
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_f80a9e92383c9e0ada01c4be923fcd98, ptr noundef nonnull inttoptr (i64 59 to ptr))
          to label %bb57 unwind label %cleanup3

bb59:                                             ; preds = %bb56
  %_75 = icmp ult i32 %151, 77
  br i1 %_75, label %bb60, label %bb62

bb57:                                             ; preds = %bb54, %bb56
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_214341b8cbf974b5fa4df5e3a1822866, ptr noundef nonnull inttoptr (i64 79 to ptr))
          to label %bb60 unwind label %cleanup3

bb62:                                             ; preds = %bb59
  %_78 = icmp eq i32 %151, 77
  br i1 %_78, label %bb63, label %bb65

bb60:                                             ; preds = %bb57, %bb59
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_17c7a5f1c85e31bcbd0baf8c45cbf4c9, ptr noundef nonnull inttoptr (i64 57 to ptr))
          to label %bb63 unwind label %cleanup3

bb65:                                             ; preds = %bb62
  %_81 = icmp ult i32 %151, 81
  br i1 %_81, label %bb66, label %bb68

bb63:                                             ; preds = %bb60, %bb62
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c2f517cef8b40692abde0001fd67d43c, ptr noundef nonnull inttoptr (i64 81 to ptr))
          to label %bb66 unwind label %cleanup3

bb68:                                             ; preds = %bb66, %bb65
  %192 = icmp eq i64 %target.sroa.0.0.copyload, 0
  br i1 %192, label %bb69, label %bb2.i.i.i4.i.i133

bb2.i.i.i4.i.i133:                                ; preds = %bb68
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %target.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb69

bb66:                                             ; preds = %bb63, %bb65
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_9dcee10c03036d5ac2dd73d4efbb97a5, ptr noundef nonnull inttoptr (i64 61 to ptr))
          to label %bb68 unwind label %cleanup3

bb69:                                             ; preds = %bb2.i.i.i4.i.i133, %bb68
  %193 = icmp eq i64 %patch_version.sroa.0.0.copyload, 0
  br i1 %193, label %bb70, label %bb2.i.i.i4.i.i135

bb2.i.i.i4.i.i135:                                ; preds = %bb69
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %patch_version.sroa.7.0.copyload, i64 noundef %patch_version.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb70

bb70:                                             ; preds = %bb2.i.i.i4.i.i135, %bb69
  %194 = icmp eq i64 %0, 0
  br i1 %194, label %bb74, label %bb74.sink.split

bb79:                                             ; preds = %bb5
  %lpad.thr_comm.split-lp = landingpad { ptr, i32 }
          cleanup
  %195 = icmp eq i64 %module.sroa.0.0.copyload141, 0
  br i1 %195, label %bb76, label %bb2.i.i.i4.i.i139

bb2.i.i.i4.i.i139:                                ; preds = %bb79
  %196 = icmp ne ptr %module.sroa.6.0.copyload142, null
  call void @llvm.assume(i1 %196)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %module.sroa.6.0.copyload142, i64 noundef %module.sroa.0.0.copyload141, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb76

bb78:                                             ; preds = %bb2.i.i.i4.i.i.i.i, %bb77
  resume { ptr, i32 } %.pn23
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCshitNtYJEXnW_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #5 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %or.cond.not = icmp sgt i64 %cap, 0
  br i1 %or.cond.not, label %bb14, label %bb11, !prof !226

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
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCshitNtYJEXnW_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #4 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !227, !noundef !3
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !3, !align !17, !noundef !3
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1 = load i64, ptr %2, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !228)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i = load i64, ptr %4, align 8, !alias.scope !228, !noalias !231, !noundef !3
  %_38.not.i = icmp ugt i64 %index2.i, %_4.val1
  %.promoted.i = load i64, ptr %3, align 8, !alias.scope !228, !noalias !231
  %_4325.i = icmp ult i64 %index2.i, %.promoted.i
  %or.cond26.i = or i1 %_38.not.i, %_4325.i
  br i1 %or.cond26.i, label %bb1.i, label %bb12.lr.ph.i

bb12.lr.ph.i:                                     ; preds = %bb2
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_48.i = load i8, ptr %5, align 8, !alias.scope !228, !noalias !231, !noundef !3
  %_12.i = zext i8 %_48.i to i64
  %6 = getelementptr i8, ptr %_10.i, i64 %_12.i
  %_49.i = getelementptr i8, ptr %6, i64 -1
  %_65.i = icmp ult i8 %_48.i, 5
  %last_byte.us.pre.i = load i8, ptr %_49.i, align 1, !alias.scope !228, !noalias !231
  br i1 %_65.i, label %bb12.us.i, label %bb12.i, !prof !233

bb12.us.i:                                        ; preds = %bb12.lr.ph.i, %bb9.us.i
  %7 = phi i64 [ %16, %bb9.us.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.us.i = sub nuw i64 %index2.i, %7
  %_46.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %7
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i), !noalias !234
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %10, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %9, align 1, !alias.scope !235, !noalias !234, !noundef !3
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
  store i64 %16, ptr %3, align 8, !alias.scope !228, !noalias !231
  %_17.not.us.i = icmp ult i64 %16, %_12.i
  %_54.not.us.i = icmp ugt i64 %16, %_4.val1
  %or.cond.i = or i1 %_17.not.us.i, %_54.not.us.i
  br i1 %or.cond.i, label %bb9.us.i, label %bb19.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %found_char.us.i = sub nuw i64 %16, %_12.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %found_char.us.i
  %17 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us.i, ptr nonnull readonly align 1 %_10.i, i64 range(i64 0, -9223372036854775808) %_12.i), !alias.scope !238, !noalias !231
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
  %20 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i), !noalias !234
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
  %_9.i.i = load i8, ptr %23, align 1, !alias.scope !235, !noalias !234, !noundef !3
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
  store i64 %28, ptr %3, align 8, !alias.scope !228, !noalias !231
  %_17.not.i = icmp ult i64 %28, %_12.i
  %_54.not.i = icmp ugt i64 %28, %_4.val1
  %or.cond70.i = or i1 %_17.not.i, %_54.not.i
  br i1 %or.cond70.i, label %bb9.i, label %bb25.i

bb10.i:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  store i64 %index2.i, ptr %3, align 8, !alias.scope !228, !noalias !231
  br label %bb1.i

bb9.i:                                            ; preds = %bb4.i
  %_43.i = icmp ult i64 %index2.i, %28
  br i1 %_43.i, label %bb1.i, label %bb12.i

bb25.i:                                           ; preds = %bb4.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #27, !noalias !234
  unreachable

bb7:                                              ; preds = %bb19.us.i
  %i = load i64, ptr %self, align 8, !noundef !3
  %new_len = sub nuw i64 %found_char.us.i, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %16, ptr %self, align 8
  br label %bb9

bb1.i:                                            ; preds = %bb9.i, %bb9.us.i, %bb2, %bb10.i
  store i8 1, ptr %0, align 1, !alias.scope !242
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %30 = load i8, ptr %29, align 8, !range !227, !alias.scope !242, !noundef !3
  %_3.i = trunc nuw i8 %30 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !242
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !242
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
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCshitNtYJEXnW_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !3
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !245)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !248)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !251, !noalias !252
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !251, !noalias !252
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !251, !noalias !252
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !251, !noalias !252
  store i64 0, ptr %self, align 8, !alias.scope !251, !noalias !252
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit

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
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !254, !nonnull !3, !noundef !3
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !255, !noundef !3
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #22, !noalias !260
  %11 = load ptr, ptr %10, align 8, !noalias !255, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #22, !noalias !260
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !261)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !264)
  %_3.i.i = load i64, ptr %self, align 8, !range !37, !alias.scope !267, !noalias !268, !noundef !3
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !267, !noalias !268, !noundef !3
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !270, !noalias !273
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !270, !noalias !273
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !267, !noalias !268, !noundef !3
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !267, !noalias !268, !nonnull !3, !noundef !3
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !267, !noalias !268
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !275, !nonnull !3, !noundef !3
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !276)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !277, !noundef !3
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !284, !noundef !3
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
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !289, !nonnull !3, !noundef !3
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !284
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #22, !noalias !293
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !277, !noundef !3
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #22, !noalias !293
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #27
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !294

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #25, !noalias !295
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !270, !noalias !273
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !270, !noalias !273
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !270, !noalias !273
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build.exit
  ret void
}

; <&std::ffi::os_str::OsString as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtB6_5Debug3fmtCshitNtYJEXnW_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !3, !align !7, !noundef !3
; call <std::ffi::os_str::OsString as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXs9_NtNtCs5sEH5CPMdak_3std3ffi6os_strNtB5_8OsStringNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <std::env::VarError as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsk_NtCs5sEH5CPMdak_3std3envNtB5_8VarErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %__self_0 = alloca [8 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !10, !noundef !3
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_0)
  store ptr %self, ptr %__self_0, align 8
; call <core::fmt::Formatter>::debug_tuple_field1_finish
  %1 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_19adf04fb909e90136daf37b5ff22508, i64 noundef 10, ptr noundef nonnull align 1 %__self_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.5)
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

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #6

; <std::path::Path>::_join
; Function Attrs: uwtable
declare void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #7

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #8

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #9

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #10

; <std::sys::process::unix::common::Command>::arg
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef align 8 dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::new
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr dead_on_unwind noalias noundef writable sret([200 x i8]) align 8 captures(none) dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; std::fs::write::inner
; Function Attrs: uwtable
declare noundef ptr @_RNvNvNtCs5sEH5CPMdak_3std2fs5write5inner(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; std::env::_var
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; std::env::_var_os
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #12

; <std::process::Command>::output
; Function Attrs: uwtable
declare void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr dead_on_unwind noalias noundef writable sret([56 x i8]) align 8 captures(none) dereferenceable(56), ptr noalias noundef align 8 dereferenceable(200)) unnamed_addr #0

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #13

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #14

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #15

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #10

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #10

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #16

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #17

; <std::ffi::os_str::OsString as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs9_NtNtCs5sEH5CPMdak_3std3ffi6os_strNtB5_8OsStringNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

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

; <core::str::pattern::StrSearcher>::new
; Function Attrs: uwtable
declare void @_RNvMsu_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcher3new(ptr dead_on_unwind noalias noundef writable sret([104 x i8]) align 8 captures(none) dereferenceable(104), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #10

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #19 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCshitNtYJEXnW_18build_script_build4main, ptr %_7.i, align 8
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
attributes #2 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #8 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #17 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #18 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #19 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #21 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #22 = { nounwind }
attributes #23 = { cold }
attributes #24 = { cold noreturn nounwind }
attributes #25 = { noreturn }
attributes #26 = { noinline }
attributes #27 = { noinline noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECshitNtYJEXnW_18build_script_build: %_1.0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECshitNtYJEXnW_18build_script_build"}
!7 = !{i64 8}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{i64 0, i64 -9223372036854775807}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECshitNtYJEXnW_18build_script_build: %_1"}
!13 = distinct !{!13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECshitNtYJEXnW_18build_script_build"}
!14 = !{!15, !12}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECshitNtYJEXnW_18build_script_build: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECshitNtYJEXnW_18build_script_build"}
!17 = !{i64 1}
!18 = !{i64 4}
!19 = !{i32 0, i32 6}
!20 = !{!"branch_weights", i32 2000, i32 6001}
!21 = !{!22}
!22 = distinct !{!22, !23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build: %_1"}
!23 = distinct !{!23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build"}
!24 = !{!25}
!25 = distinct !{!25, !26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCshitNtYJEXnW_18build_script_build: %self"}
!26 = distinct !{!26, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCshitNtYJEXnW_18build_script_build"}
!27 = !{!25, !22}
!28 = !{!29, !31, !25, !22}
!29 = distinct !{!29, !30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCshitNtYJEXnW_18build_script_build: %self"}
!30 = distinct !{!30, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCshitNtYJEXnW_18build_script_build"}
!31 = distinct !{!31, !32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build: %_1"}
!32 = distinct !{!32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECshitNtYJEXnW_18build_script_build"}
!33 = !{i64 19035411024420984}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCshitNtYJEXnW_18build_script_build: %self"}
!36 = distinct !{!36, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCshitNtYJEXnW_18build_script_build"}
!37 = !{i64 0, i64 2}
!38 = !{!39}
!39 = distinct !{!39, !40, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CshitNtYJEXnW_18build_script_build: %_1"}
!40 = distinct !{!40, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CshitNtYJEXnW_18build_script_build"}
!41 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!42 = !{!43}
!43 = distinct !{!43, !44, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: %t"}
!44 = distinct !{!44, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build"}
!45 = !{!46}
!46 = distinct !{!46, !44, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: %self"}
!47 = !{!43, !48}
!48 = distinct !{!48, !44, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: argument 2"}
!49 = !{!43, !46, !48}
!50 = !{!43, !46}
!51 = !{!52}
!52 = distinct !{!52, !53, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build: %_1"}
!53 = distinct !{!53, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build"}
!54 = !{!52, !43, !46}
!55 = !{!48}
!56 = !{!57, !59}
!57 = distinct !{!57, !58, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceReECshitNtYJEXnW_18build_script_build: %_0"}
!58 = distinct !{!58, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceReECshitNtYJEXnW_18build_script_build"}
!59 = distinct !{!59, !58, !"_RINvMs3_NtCsdJPVW0sQgAG_5alloc3stre7replaceReECshitNtYJEXnW_18build_script_build: %to.0"}
!60 = !{!61, !57, !59}
!61 = distinct !{!61, !62, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshitNtYJEXnW_18build_script_build: %_0"}
!62 = distinct !{!62, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCshitNtYJEXnW_18build_script_build"}
!63 = !{!64}
!64 = distinct !{!64, !65, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %haystack.0"}
!65 = distinct !{!65, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build"}
!66 = !{!67}
!67 = distinct !{!67, !65, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %needle.0"}
!68 = !{!69, !70, !67, !57, !59}
!69 = distinct !{!69, !65, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %_0"}
!70 = distinct !{!70, !65, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %self"}
!71 = !{!69, !70, !64, !57, !59}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %haystack.0"}
!74 = distinct !{!74, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build"}
!75 = !{!76}
!76 = distinct !{!76, !74, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %needle.0"}
!77 = !{!78, !79, !76, !57, !59}
!78 = distinct !{!78, !74, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %_0"}
!79 = distinct !{!79, !74, !"_RINvMsx_NtNtCsjMrxcFdYDNN_4core3str7patternNtB6_14TwoWaySearcher4nextNtB6_9MatchOnlyECshitNtYJEXnW_18build_script_build: %self"}
!80 = !{!78, !79, !73, !57, !59}
!81 = !{!82}
!82 = distinct !{!82, !83, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build: %self"}
!83 = distinct !{!83, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build"}
!84 = !{!85, !82}
!85 = distinct !{!85, !86, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build: %self"}
!86 = distinct !{!86, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build"}
!87 = !{!88, !90}
!88 = distinct !{!88, !89, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build: %self"}
!89 = distinct !{!89, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build"}
!90 = distinct !{!90, !91, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build: %self"}
!91 = distinct !{!91, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build"}
!92 = !{!82, !57, !59}
!93 = !{!90}
!94 = !{!90, !57}
!95 = !{!96}
!96 = distinct !{!96, !97, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!97 = distinct !{!97, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!98 = !{!99, !101, !102, !104, !57, !59}
!99 = distinct !{!99, !100, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next: %otherwise"}
!100 = distinct !{!100, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next"}
!101 = distinct !{!101, !100, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher4next: %self"}
!102 = distinct !{!102, !103, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match: %_0"}
!103 = distinct !{!103, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match"}
!104 = distinct !{!104, !103, !"_RNvXsv_NtNtCsjMrxcFdYDNN_4core3str7patternNtB5_11StrSearcherNtB5_8Searcher10next_match: %self"}
!105 = !{!106, !99, !101, !102, !104, !57, !59}
!106 = distinct !{!106, !107, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECshitNtYJEXnW_18build_script_build: %bytes"}
!107 = distinct !{!107, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECshitNtYJEXnW_18build_script_build"}
!108 = !{!109}
!109 = distinct !{!109, !83, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build: %self:Peel0"}
!110 = !{!88, !111}
!111 = distinct !{!111, !91, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build: %self:Peel0"}
!112 = !{!109, !57, !59}
!113 = !{!111}
!114 = !{!57}
!115 = !{!90, !111}
!116 = !{!117}
!117 = distinct !{!117, !118, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build: %self"}
!118 = distinct !{!118, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCshitNtYJEXnW_18build_script_build"}
!119 = !{!120, !117}
!120 = distinct !{!120, !121, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build: %self"}
!121 = distinct !{!121, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCshitNtYJEXnW_18build_script_build"}
!122 = distinct !{!122, !123}
!123 = !{!"llvm.loop.peeled.count", i32 1}
!124 = !{!117, !57, !59}
!125 = !{!59}
!126 = !{!127}
!127 = distinct !{!127, !128, !"_RINvNtCs5sEH5CPMdak_3std2fs5writeNtNtB4_4path7PathBufNtNtCsdJPVW0sQgAG_5alloc6string6StringECshitNtYJEXnW_18build_script_build: %path"}
!128 = distinct !{!128, !"_RINvNtCs5sEH5CPMdak_3std2fs5writeNtNtB4_4path7PathBufNtNtCsdJPVW0sQgAG_5alloc6string6StringECshitNtYJEXnW_18build_script_build"}
!129 = !{!130}
!130 = distinct !{!130, !128, !"_RINvNtCs5sEH5CPMdak_3std2fs5writeNtNtB4_4path7PathBufNtNtCsdJPVW0sQgAG_5alloc6string6StringECshitNtYJEXnW_18build_script_build: %contents"}
!131 = !{!127, !130}
!132 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!133 = !{!134, !136}
!134 = distinct !{!134, !135, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build: %_0"}
!135 = distinct !{!135, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build"}
!136 = distinct !{!136, !135, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECshitNtYJEXnW_18build_script_build: %program"}
!137 = !{!136}
!138 = !{!139}
!139 = distinct !{!139, !140, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECshitNtYJEXnW_18build_script_build: %_1"}
!140 = distinct !{!140, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECshitNtYJEXnW_18build_script_build"}
!141 = !{!142, !144}
!142 = distinct !{!142, !143, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!143 = distinct !{!143, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!144 = distinct !{!144, !143, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!145 = !{!146}
!146 = distinct !{!146, !147, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!147 = distinct !{!147, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!148 = !{!"branch_weights", i32 2002, i32 2000}
!149 = !{!150}
!150 = distinct !{!150, !151, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build: %_1"}
!151 = distinct !{!151, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build"}
!152 = !{!153}
!153 = distinct !{!153, !154, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build: %_1"}
!154 = distinct !{!154, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECshitNtYJEXnW_18build_script_build"}
!155 = !{!156}
!156 = distinct !{!156, !157, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: %t"}
!157 = distinct !{!157, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build"}
!158 = !{!159}
!159 = distinct !{!159, !157, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: %self"}
!160 = !{!156, !161}
!161 = distinct !{!161, !157, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorE6unwrapCshitNtYJEXnW_18build_script_build: argument 2"}
!162 = !{!156, !159, !161}
!163 = !{!156, !159}
!164 = !{!165}
!165 = distinct !{!165, !166, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build: %_1"}
!166 = distinct !{!166, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env8VarErrorECshitNtYJEXnW_18build_script_build"}
!167 = !{!165, !156, !159}
!168 = !{!161}
!169 = !{!170, !172}
!170 = distinct !{!170, !171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!171 = distinct !{!171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!172 = distinct !{!172, !171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!173 = !{!174, !176}
!174 = distinct !{!174, !175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!175 = distinct !{!175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!176 = distinct !{!176, !175, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!177 = !{!178, !180, !181, !183}
!178 = distinct !{!178, !179, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!179 = distinct !{!179, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!180 = distinct !{!180, !179, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!181 = distinct !{!181, !182, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!182 = distinct !{!182, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!183 = distinct !{!183, !182, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!184 = !{!185, !187, !188, !190}
!185 = distinct !{!185, !186, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!186 = distinct !{!186, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!187 = distinct !{!187, !186, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!188 = distinct !{!188, !189, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!189 = distinct !{!189, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!190 = distinct !{!190, !189, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!191 = !{!192, !194, !195, !197}
!192 = distinct !{!192, !193, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!193 = distinct !{!193, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!194 = distinct !{!194, !193, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!195 = distinct !{!195, !196, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!196 = distinct !{!196, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!197 = distinct !{!197, !196, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!198 = !{!199, !201, !202, !204}
!199 = distinct !{!199, !200, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!200 = distinct !{!200, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!201 = distinct !{!201, !200, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!202 = distinct !{!202, !203, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!203 = distinct !{!203, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!204 = distinct !{!204, !203, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!205 = !{!206, !208, !209, !211}
!206 = distinct !{!206, !207, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!207 = distinct !{!207, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!208 = distinct !{!208, !207, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!209 = distinct !{!209, !210, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!210 = distinct !{!210, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!211 = distinct !{!211, !210, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!212 = !{!213, !215, !216, !218}
!213 = distinct !{!213, !214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!214 = distinct !{!214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!215 = distinct !{!215, !214, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!216 = distinct !{!216, !217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!217 = distinct !{!217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!218 = distinct !{!218, !217, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!219 = !{!220, !222, !223, !225}
!220 = distinct !{!220, !221, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!221 = distinct !{!221, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!222 = distinct !{!222, !221, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!223 = distinct !{!223, !224, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %self.0"}
!224 = distinct !{!224, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build"}
!225 = distinct !{!225, !224, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCshitNtYJEXnW_18build_script_build: %needle.0"}
!226 = !{!"branch_weights", i32 2000, i32 6004}
!227 = !{i8 0, i8 2}
!228 = !{!229}
!229 = distinct !{!229, !230, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!230 = distinct !{!230, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!231 = !{!232}
!232 = distinct !{!232, !230, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!233 = !{!"branch_weights", i32 4000000, i32 4001}
!234 = !{!232, !229}
!235 = !{!236}
!236 = distinct !{!236, !237, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!237 = distinct !{!237, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!238 = !{!239, !241}
!239 = distinct !{!239, !240, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %self.0"}
!240 = distinct !{!240, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build"}
!241 = distinct !{!241, !240, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCshitNtYJEXnW_18build_script_build: %other.0"}
!242 = !{!243}
!243 = distinct !{!243, !244, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCshitNtYJEXnW_18build_script_build: %self"}
!244 = distinct !{!244, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCshitNtYJEXnW_18build_script_build"}
!245 = !{!246}
!246 = distinct !{!246, !247, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %self"}
!247 = distinct !{!247, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build"}
!248 = !{!249}
!249 = distinct !{!249, !250, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCshitNtYJEXnW_18build_script_build: %self"}
!250 = distinct !{!250, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCshitNtYJEXnW_18build_script_build"}
!251 = !{!249, !246}
!252 = !{!253}
!253 = distinct !{!253, !250, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCshitNtYJEXnW_18build_script_build: %_0"}
!254 = !{!253, !249, !246}
!255 = !{!256, !258, !246}
!256 = distinct !{!256, !257, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCshitNtYJEXnW_18build_script_build: %_0"}
!257 = distinct !{!257, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCshitNtYJEXnW_18build_script_build"}
!258 = distinct !{!258, !259, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %ret"}
!259 = distinct !{!259, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build"}
!260 = !{!258, !246}
!261 = !{!262}
!262 = distinct !{!262, !263, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %self"}
!263 = distinct !{!263, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build"}
!264 = !{!265}
!265 = distinct !{!265, !266, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCshitNtYJEXnW_18build_script_build: %self"}
!266 = distinct !{!266, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCshitNtYJEXnW_18build_script_build"}
!267 = !{!265, !262}
!268 = !{!269}
!269 = distinct !{!269, !263, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %_0"}
!270 = !{!271, !262}
!271 = distinct !{!271, !272, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECshitNtYJEXnW_18build_script_build: %v"}
!272 = distinct !{!272, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECshitNtYJEXnW_18build_script_build"}
!273 = !{!274, !269}
!274 = distinct !{!274, !272, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECshitNtYJEXnW_18build_script_build: %ret"}
!275 = !{!265, !269, !262}
!276 = !{!271}
!277 = !{!278, !280, !281, !283, !274, !271, !269, !262}
!278 = distinct !{!278, !279, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %_0"}
!279 = distinct !{!279, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build"}
!280 = distinct !{!280, !279, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %self"}
!281 = distinct !{!281, !282, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CshitNtYJEXnW_18build_script_build: %val"}
!282 = distinct !{!282, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CshitNtYJEXnW_18build_script_build"}
!283 = distinct !{!283, !282, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0CshitNtYJEXnW_18build_script_build: %leaf_edge"}
!284 = !{!285, !287, !278, !280, !281, !283, !274, !271, !269, !262}
!285 = distinct !{!285, !286, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCshitNtYJEXnW_18build_script_build: %_0"}
!286 = distinct !{!286, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCshitNtYJEXnW_18build_script_build"}
!287 = distinct !{!287, !288, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build: %ret"}
!288 = distinct !{!288, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECshitNtYJEXnW_18build_script_build"}
!289 = !{!290, !292, !278, !280, !281, !283, !274, !271, !269, !262}
!290 = distinct !{!290, !291, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCshitNtYJEXnW_18build_script_build: %_0"}
!291 = distinct !{!291, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCshitNtYJEXnW_18build_script_build"}
!292 = distinct !{!292, !291, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCshitNtYJEXnW_18build_script_build: %self"}
!293 = !{!287, !278, !280, !281, !283, !274, !271, !269, !262}
!294 = !{!274, !271, !269, !262}
!295 = !{!269, !262}
