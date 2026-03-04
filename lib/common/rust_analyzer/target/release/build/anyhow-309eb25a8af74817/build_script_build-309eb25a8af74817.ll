; ModuleID = 'build_script_build.80962978fffc9592-cgu.0'
source_filename = "build_script_build.80962978fffc9592-cgu.0"
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
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCsb2sIUxn0KTS_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Csb2sIUxn0KTS_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Csb2sIUxn0KTS_18build_script_build }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_193ab55f01318f0887536940a400dd6a = private unnamed_addr constant [71 x i8] c"\16Environment variable $\C0- is not set during execution of build script\0A\00", align 1
@alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26 = private unnamed_addr constant [11 x i8] c"RUSTC_STAGE", align 1
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_ebcdb5f66b6f511cde89ece546cbdd6d = private unnamed_addr constant [7 x i8] c"OUT_DIR", align 1
@alloc_6b9c4d547a268aa1e418a0468c721e03 = private unnamed_addr constant [5 x i8] c"probe", align 1
@alloc_ec4612a7974a19b5164a1467c0223b7b = private unnamed_addr constant [3 x i8] c"src", align 1
@alloc_9b952d500051094ec12d4caeaadde05b = private unnamed_addr constant [10 x i8] c"nightly.rs", align 1
@alloc_ec0fb45a03827c6e6fbcf8024afc16d6 = private unnamed_addr constant [26 x i8] c"\11Failed to create \C0\02: \C0\01\0A\00", align 1
@alloc_f36ce88bd5d4a921175f5521f484b675 = private unnamed_addr constant [13 x i8] c"RUSTC_WRAPPER", align 1
@alloc_fbe0d85396ee55e48aae2aa2891c1dc3 = private unnamed_addr constant [23 x i8] c"RUSTC_WORKSPACE_WRAPPER", align 1
@alloc_d563101362ed4a06747b9210d55c4c5b = private unnamed_addr constant [15 x i8] c"RUSTC_BOOTSTRAP", align 1
@alloc_74b6a34a01881648f3efb1322e7c9ba4 = private unnamed_addr constant [24 x i8] c"--cfg=anyhow_build_probe", align 1
@alloc_31c405a4038c01b5a14020c6d50bb4ce = private unnamed_addr constant [14 x i8] c"--edition=2018", align 1
@alloc_1e60bf274ec782b0083f842c06658d23 = private unnamed_addr constant [19 x i8] c"--crate-name=anyhow", align 1
@alloc_a6e356e753364954471dcbf409cc4c4e = private unnamed_addr constant [16 x i8] c"--crate-type=lib", align 1
@alloc_9930910b9e2bf161f6d41704390848d2 = private unnamed_addr constant [17 x i8] c"--cap-lints=allow", align 1
@alloc_4d01ae01a4b8e52c5a54208511747587 = private unnamed_addr constant [24 x i8] c"--emit=dep-info,metadata", align 1
@alloc_8bbf703e0ecc0326ac386a57604275da = private unnamed_addr constant [9 x i8] c"--out-dir", align 1
@alloc_dcbc225a8ec7dbfaaef714ff8a7176fb = private unnamed_addr constant [6 x i8] c"TARGET", align 1
@alloc_c20974c698c079af35c03642327d3f4f = private unnamed_addr constant [8 x i8] c"--target", align 1
@alloc_07f3eec4949a8d39db630a4a477c65b3 = private unnamed_addr constant [23 x i8] c"CARGO_ENCODED_RUSTFLAGS", align 1
@alloc_62e6ec0e1c3bfea4ae2f14deaee8dee9 = private unnamed_addr constant [28 x i8] c"\13Failed to clean up \C0\02: \C0\01\0A\00", align 1
@alloc_a887f9858119cc7413062dc002c4d9ab = private unnamed_addr constant [9 x i8] c"--version", align 1
@alloc_ca36d7e792bb4bbd1a68749f90007ce8 = private unnamed_addr constant [7 x i8] c"rustc 1", align 1
@alloc_a5ada7cbbef10488e9785c51c0789d49 = private unnamed_addr constant [38 x i8] c"cargo:rerun-if-changed=src/nightly.rs\0A", align 1
@alloc_a0f0604af2246bca35707903288f4f53 = private unnamed_addr constant [30 x i8] c"cargo:rustc-cfg=std_backtrace\0A", align 1
@alloc_e181ada66eb53f56ba6935f91cca5e48 = private unnamed_addr constant [44 x i8] c"cargo:rustc-cfg=error_generic_member_access\0A", align 1
@alloc_c4fe0d46c3935d35a63bc8de9de91c71 = private unnamed_addr constant [43 x i8] c"cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP\0A", align 1
@alloc_7d35a291c18dd0521a5e644c84ccaf0c = private unnamed_addr constant [46 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_build_probe)\0A", align 1
@alloc_72df481b03b7eb46c2809fd337b26da7 = private unnamed_addr constant [50 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_nightly_testing)\0A", align 1
@alloc_5dfc04774f16d076cbbf5aa4aa0b112a = private unnamed_addr constant [56 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_clippy_format_args)\0A", align 1
@alloc_be9f510b6961f26854e9160336556929 = private unnamed_addr constant [48 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_core_error)\0A", align 1
@alloc_a5780797e07fb9d8b2064b159c0780ee = private unnamed_addr constant [54 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_core_unwind_safe)\0A", align 1
@alloc_fcc25653342892fcd6eef156552226a0 = private unnamed_addr constant [58 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_fmt_arguments_as_str)\0A", align 1
@alloc_fed985032aec30b656cbc8da62e5d8b4 = private unnamed_addr constant [49 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_ptr_addr_of)\0A", align 1
@alloc_1ac038b884c8e490cbaf26d8798c71a3 = private unnamed_addr constant [65 x i8] c"cargo:rustc-check-cfg=cfg(anyhow_no_unsafe_op_in_unsafe_fn_lint)\0A", align 1
@alloc_3e2d38849520ab73d6bdac75533cb117 = private unnamed_addr constant [55 x i8] c"cargo:rustc-check-cfg=cfg(error_generic_member_access)\0A", align 1
@alloc_311235f6f62f0542f667345a272b98e8 = private unnamed_addr constant [41 x i8] c"cargo:rustc-check-cfg=cfg(std_backtrace)\0A", align 1
@alloc_5699aa2caa76aa64718f60b150ec0ee6 = private unnamed_addr constant [38 x i8] c"cargo:rustc-cfg=anyhow_no_ptr_addr_of\0A", align 1
@alloc_7351d367884d97b1604bb68c0a75d428 = private unnamed_addr constant [47 x i8] c"cargo:rustc-cfg=anyhow_no_fmt_arguments_as_str\0A", align 1
@alloc_286391b658cb8dcdee605b2b79832083 = private unnamed_addr constant [54 x i8] c"cargo:rustc-cfg=anyhow_no_unsafe_op_in_unsafe_fn_lint\0A", align 1
@alloc_27cc1f87a94a5f6e40c716f850700584 = private unnamed_addr constant [43 x i8] c"cargo:rustc-cfg=anyhow_no_core_unwind_safe\0A", align 1
@alloc_6df1e55893960c0368c61ff3e9d5b943 = private unnamed_addr constant [37 x i8] c"cargo:rustc-cfg=anyhow_no_core_error\0A", align 1
@alloc_afd5ca28658b6cdb1db5eabbd0000b2d = private unnamed_addr constant [45 x i8] c"cargo:rustc-cfg=anyhow_no_clippy_format_args\0A", align 1
@alloc_7fe94be2e120ffbd80c490b1b3c481ee = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/pattern.rs\00", align 1
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECsb2sIUxn0KTS_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
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
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i ], [ 0, %start ]
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
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #17, !noalias !4
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i

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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #17, !noalias !4
  br label %bb4.i.i.preheader

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #18
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !4

terminate.i.i:                                    ; preds = %bb3.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19, !noalias !4
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %25 = icmp eq i64 %_1.val2, 0
  br i1 %25, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #17
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECsb2sIUxn0KTS_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #17
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #17
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::option::IntoIter<std::ffi::os_str::OsString>>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  %1 = load i64, ptr %0, align 8, !range !13, !alias.scope !10, !noundef !3
  %2 = icmp eq i64 %1, -9223372036854775806
  br i1 %2, label %bb4, label %bb2.i

bb2.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14)
  switch i64 %1, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i
    i64 -9223372036854775808, label %bb4.i.i
    i64 0, label %bb4.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i:                     ; preds = %bb2.i
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %_1.val4.i.i = load ptr, ptr %3, align 8, !alias.scope !17, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4.i.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !17
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i, %bb2.i, %bb2.i, %bb2.i
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 48
  %.val2.i.i = load i64, ptr %4, align 8, !range !18, !alias.scope !17, !noundef !3
  switch i64 %.val2.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i [
    i64 -9223372036854775807, label %bb4
    i64 -9223372036854775808, label %bb4
    i64 0, label %bb4
  ]

bb2.i.i.i4.i.i.i.i.i.i.i7.i.i:                    ; preds = %bb4.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  %.val3.i.i = load ptr, ptr %5, align 8, !alias.scope !17, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !17
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i, %bb4.i.i, %bb4.i.i, %bb4.i.i, %start
  %_1.val2 = load i64, ptr %_1, align 8, !range !18, !noundef !3
  switch i64 %_1.val2, label %bb2.i.i.i4.i.i.i.i.i.i.i.i4 [
    i64 -9223372036854775807, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECsb2sIUxn0KTS_18build_script_build.exit5
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECsb2sIUxn0KTS_18build_script_build.exit5
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECsb2sIUxn0KTS_18build_script_build.exit5
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i4:                      ; preds = %bb4
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3, i64 noundef %_1.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECsb2sIUxn0KTS_18build_script_build.exit5

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECsb2sIUxn0KTS_18build_script_build.exit5: ; preds = %bb4, %bb4, %bb4, %bb2.i.i.i4.i.i.i.i.i.i.i.i4
  ret void
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %bb4, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i, %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2 = load i64, ptr %2, align 8
  %3 = icmp eq i64 %.val2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECsb2sIUxn0KTS_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECsb2sIUxn0KTS_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECsb2sIUxn0KTS_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !19, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val24.i = load i64, ptr %1, align 8, !alias.scope !19
  store i8 0, ptr %.val.i, align 1, !noalias !19
  %2 = icmp eq i64 %.val24.i, 0
  br i1 %2, label %bb20.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %.val24.i, i64 noundef 1) #17
  br label %bb20.i

bb20.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i, %start
; invoke <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
  invoke void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb20.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %_1.val.i.i = load i64, ptr %_1, align 8, !alias.scope !22
  %4 = icmp eq i64 %_1.val.i.i, 0
  br i1 %4, label %bb10.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i = load ptr, ptr %5, align 8, !alias.scope !22, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %_1.val.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #17
  br label %bb10.i

bb4.i.i:                                          ; preds = %bb20.i
  %_1.val2.i.i = load i64, ptr %_1, align 8, !alias.scope !22
  %6 = icmp eq i64 %_1.val2.i.i, 0
  br i1 %6, label %bb19.i, label %bb2.i.i.i4.i4.i.i

bb2.i.i.i4.i4.i.i:                                ; preds = %bb4.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3.i.i = load ptr, ptr %7, align 8, !alias.scope !22, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i5.i.i = shl nuw i64 %_1.val2.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3.i.i, i64 noundef %alloc_size.i.i.i.i5.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #17
  br label %bb19.i

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #18
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
          to label %bb18.i unwind label %cleanup2.i

bb9.i:                                            ; preds = %cleanup2.i, %bb10.i
  %.pn10.i = phi { ptr, i32 } [ %14, %cleanup2.i ], [ %3, %bb10.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val27.i = load ptr, ptr %10, align 8, !alias.scope !19, !align !25, !noundef !3
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val28.i = load i64, ptr %11, align 8, !alias.scope !19
  %12 = icmp eq ptr %.val27.i, null
  br i1 %12, label %bb8.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb9.i
  store i8 0, ptr %.val27.i, align 1
  %13 = icmp eq i64 %.val28.i, 0
  br i1 %13, label %bb8.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val27.i, i64 noundef %.val28.i, i64 noundef 1) #17
  br label %bb8.i

cleanup2.i:                                       ; preds = %bb19.i
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %bb9.i

bb18.i:                                           ; preds = %bb19.i
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val31.i = load ptr, ptr %15, align 8, !alias.scope !19, !align !25, !noundef !3
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val32.i = load i64, ptr %16, align 8, !alias.scope !19
  %17 = icmp eq ptr %.val31.i, null
  br i1 %17, label %bb17.i, label %bb2.i50.i

bb2.i50.i:                                        ; preds = %bb18.i
  store i8 0, ptr %.val31.i, align 1
  %18 = icmp eq i64 %.val32.i, 0
  br i1 %18, label %bb17.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i: ; preds = %bb2.i50.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val31.i, i64 noundef %.val32.i, i64 noundef 1) #17
  br label %bb17.i

bb8.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i, %bb2.i.i, %bb9.i
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val25.i = load ptr, ptr %19, align 8, !alias.scope !19, !align !25, !noundef !3
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val26.i = load i64, ptr %20, align 8, !alias.scope !19
  %21 = icmp eq ptr %.val25.i, null
  br i1 %21, label %bb7.i, label %bb2.i54.i

bb2.i54.i:                                        ; preds = %bb8.i
  store i8 0, ptr %.val25.i, align 1
  %22 = icmp eq i64 %.val26.i, 0
  br i1 %22, label %bb7.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i: ; preds = %bb2.i54.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val25.i, i64 noundef %.val26.i, i64 noundef 1) #17
  br label %bb7.i

bb17.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i, %bb2.i50.i, %bb18.i
  %23 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val29.i = load ptr, ptr %23, align 8, !alias.scope !19, !align !25, !noundef !3
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val30.i = load i64, ptr %24, align 8, !alias.scope !19
  %25 = icmp eq ptr %.val29.i, null
  br i1 %25, label %bb16.i, label %bb2.i58.i

bb2.i58.i:                                        ; preds = %bb17.i
  store i8 0, ptr %.val29.i, align 1
  %26 = icmp eq i64 %.val30.i, 0
  br i1 %26, label %bb16.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i: ; preds = %bb2.i58.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val29.i, i64 noundef %.val30.i, i64 noundef 1) #17
  br label %bb16.i

bb7.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i, %bb2.i54.i, %bb8.i
  %27 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #18
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
          to label %bb15.i unwind label %cleanup5.i

bb6.i:                                            ; preds = %cleanup5.i, %bb7.i
  %.pn16.i = phi { ptr, i32 } [ %34, %cleanup5.i ], [ %.pn10.i, %bb7.i ]
  %29 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val33.i = load ptr, ptr %29, align 8, !alias.scope !19, !align !26, !noundef !3
  %30 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val34.i = load i64, ptr %30, align 8, !alias.scope !19
  %31 = icmp eq ptr %.val33.i, null
  %32 = icmp eq i64 %.val34.i, 0
  %or.cond.i.i = select i1 %31, i1 true, i1 %32
  br i1 %or.cond.i.i, label %bb5.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb6.i
  %33 = shl nuw nsw i64 %.val34.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val33.i, i64 noundef %33, i64 noundef 4) #17
  br label %bb5.i

cleanup5.i:                                       ; preds = %bb16.i
  %34 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i

bb15.i:                                           ; preds = %bb16.i
  %35 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val35.i = load ptr, ptr %35, align 8, !alias.scope !19, !align !26, !noundef !3
  %36 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val36.i = load i64, ptr %36, align 8, !alias.scope !19
  %37 = icmp eq ptr %.val35.i, null
  %38 = icmp eq i64 %.val36.i, 0
  %or.cond.i63.i = select i1 %37, i1 true, i1 %38
  br i1 %or.cond.i63.i, label %bb14.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i: ; preds = %bb15.i
  %39 = shl nuw nsw i64 %.val36.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val35.i, i64 noundef %39, i64 noundef 4) #17
  br label %bb14.i

bb5.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb6.i
  %40 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val41.i = load i32, ptr %40, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i.i = icmp eq i32 %.val41.i, 3
  br i1 %cond.i.i, label %bb2.i.i.i, label %bb4.i

bb2.i.i.i:                                        ; preds = %bb5.i
  %41 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val42.i = load i32, ptr %41, align 4, !alias.scope !19
  %_5.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val42.i) #17
  br label %bb4.i

bb14.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i, %bb15.i
  %42 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val47.i = load i32, ptr %42, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i68.i = icmp eq i32 %.val47.i, 3
  br i1 %cond.i68.i, label %bb2.i.i70.i, label %bb13.i

bb2.i.i70.i:                                      ; preds = %bb14.i
  %43 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val48.i = load i32, ptr %43, align 4, !alias.scope !19
  %_5.i.i.i.i.i71.i = tail call noundef i32 @close(i32 noundef %.val48.i) #17
  br label %bb13.i

bb4.i:                                            ; preds = %bb2.i.i.i, %bb5.i
  %44 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val39.i = load i32, ptr %44, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i73.i = icmp eq i32 %.val39.i, 3
  br i1 %cond.i73.i, label %bb2.i.i75.i, label %bb3.i

bb2.i.i75.i:                                      ; preds = %bb4.i
  %45 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val40.i = load i32, ptr %45, align 4, !alias.scope !19
  %_5.i.i.i.i.i76.i = tail call noundef i32 @close(i32 noundef %.val40.i) #17
  br label %bb3.i

bb13.i:                                           ; preds = %bb2.i.i70.i, %bb14.i
  %46 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val45.i = load i32, ptr %46, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i78.i = icmp eq i32 %.val45.i, 3
  br i1 %cond.i78.i, label %bb2.i.i80.i, label %bb12.i

bb2.i.i80.i:                                      ; preds = %bb13.i
  %47 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val46.i = load i32, ptr %47, align 4, !alias.scope !19
  %_5.i.i.i.i.i81.i = tail call noundef i32 @close(i32 noundef %.val46.i) #17
  br label %bb12.i

bb3.i:                                            ; preds = %bb2.i.i75.i, %bb4.i
  %48 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val37.i = load i32, ptr %48, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i83.i = icmp eq i32 %.val37.i, 3
  br i1 %cond.i83.i, label %bb2.i.i85.i, label %bb1.i

bb2.i.i85.i:                                      ; preds = %bb3.i
  %49 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val38.i = load i32, ptr %49, align 4, !alias.scope !19
  %_5.i.i.i.i.i86.i = tail call noundef i32 @close(i32 noundef %.val38.i) #17
  br label %bb1.i

bb12.i:                                           ; preds = %bb2.i.i80.i, %bb13.i
  %50 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val43.i = load i32, ptr %50, align 8, !range !27, !alias.scope !19, !noundef !3
  %cond.i88.i = icmp eq i32 %.val43.i, 3
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECsb2sIUxn0KTS_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !19
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECsb2sIUxn0KTS_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::io::error::Error>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsb2sIUxn0KTS_18build_script_build(ptr %_1.0.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %bits.i.i.i = ptrtoint ptr %_1.0.val to i64
  %_5.i.i.i = and i64 %bits.i.i.i, 3
  %switch.i.i = icmp eq i64 %_5.i.i.i, 1
  br i1 %switch.i.i, label %bb2.i2.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsb2sIUxn0KTS_18build_script_build.exit, !prof !28

bb2.i2.i.i:                                       ; preds = %start
  %1 = getelementptr i8, ptr %_1.0.val, i64 -1
  %2 = icmp ne ptr %1, null
  tail call void @llvm.assume(i1 %2)
  %_6.val.i.i.i.i = load ptr, ptr %1, align 8
  %3 = getelementptr i8, ptr %_1.0.val, i64 7
  %_6.val1.i.i.i.i = load ptr, ptr %3, align 8, !nonnull !3, !align !7, !noundef !3
  %4 = load ptr, ptr %_6.val1.i.i.i.i, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i:                          ; preds = %bb2.i2.i.i
  %5 = icmp ne ptr %_6.val.i.i.i.i, null
  tail call void @llvm.assume(i1 %5)
  invoke void %4(ptr noundef nonnull %_6.val.i.i.i.i)
          to label %bb3.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %is_not_null.i.i.i.i.i.i, %bb2.i2.i.i
  %6 = icmp ne ptr %_6.val.i.i.i.i, null
  tail call void @llvm.assume(i1 %6)
  %7 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %8 = load i64, ptr %7, align 8, !range !8, !invariant.load !3
  %9 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %10 = load i64, ptr %9, align 8, !range !9, !invariant.load !3
  %11 = add i64 %10, -1
  %12 = icmp sgt i64 %11, -1
  tail call void @llvm.assume(i1 %12)
  %13 = icmp eq i64 %8, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i

cleanup.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i
  %14 = landingpad { ptr, i32 }
          cleanup
  %15 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !8, !invariant.load !3
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !9, !invariant.load !3
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  tail call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %bb1.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #17
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef 24, i64 noundef 8) #17
  resume { ptr, i32 } %14

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef 24, i64 noundef 8) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsb2sIUxn0KTS_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i = alloca [24 x i8], align 8
  %_x.i.i = alloca [72 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !32)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_x.i.i), !noalias !35
  %self1.sroa.0.0.copyload.i.i = load ptr, ptr %_1, align 8, !alias.scope !35
  %.not.i.i = icmp eq ptr %self1.sroa.0.0.copyload.i.i, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self1.sroa.5.0.copyload.i.i = load i64, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !35
  %self1.sroa.4.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.sroa.4.0.copyload.i.i = load i64, ptr %self1.sroa.4.0.self.sroa_idx.i.i, align 8, !alias.scope !35
  %full_range.sroa.4.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 8
  store ptr null, ptr %full_range.sroa.4.0._x.sroa_idx.i.i, align 8, !noalias !35
  %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 16
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !35
  %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 24
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !35
  %full_range.sroa.6.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 40
  store ptr null, ptr %full_range.sroa.6.0._x.sroa_idx.i.i, align 8, !noalias !35
  %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 48
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !35
  %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 56
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !35
  br label %bb3.i.i

bb3.i.i:                                          ; preds = %bb1.i.i, %start
  %.sink9.i.i = phi i64 [ 1, %bb1.i.i ], [ 0, %start ]
  %self1.sroa.5.0.copyload.sink.i.i = phi i64 [ %self1.sroa.5.0.copyload.i.i, %bb1.i.i ], [ 0, %start ]
  store i64 %.sink9.i.i, ptr %_x.i.i, align 8, !noalias !35
  %0 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 32
  store i64 %.sink9.i.i, ptr %0, align 8, !noalias !35
  %1 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 64
  store i64 %self1.sroa.5.0.copyload.sink.i.i, ptr %1, align 8, !noalias !35
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !36
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !35
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !36, !noundef !3
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

bb3.lr.ph.i.i.i.i:                                ; preds = %bb3.i.i
  %kv.sroa.22.0._2.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i, i64 16
  br label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb4.i.i.i.i, %bb3.lr.ph.i.i.i.i
  %3 = phi ptr [ %2, %bb3.lr.ph.i.i.i.i ], [ %7, %bb4.i.i.i.i ]
  %kv.sroa.22.0.copyload.i.i.i.i = load i64, ptr %kv.sroa.22.0._2.sroa_idx.i.i.i.i, align 8, !noalias !36
  %_5.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 8
  %key.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<std::ffi::os_str::OsString>", ptr %_5.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %_9.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 272
  %_17.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<core::option::Option<std::ffi::os_str::OsString>>", ptr %_9.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %key.val.i.i.i.i.i = load i64, ptr %key.i.i.i.i.i, align 8, !noalias !36
  %4 = icmp eq i64 %key.val.i.i.i.i.i, 0
  br i1 %4, label %bb8.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb3.i.i.i.i
  %5 = getelementptr i8, ptr %key.i.i.i.i.i, i64 8
  %key.val1.i.i.i.i.i = load ptr, ptr %5, align 8, !noalias !36, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %key.val1.i.i.i.i.i, i64 noundef %key.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !36
  br label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i
  %self1.val.i.i.i.i.i.i.i = load i64, ptr %_17.i.i.i.i.i, align 8, !range !41, !noalias !36, !noundef !3
  switch i64 %self1.val.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb8.i.i.i.i.i
  %6 = getelementptr i8, ptr %_17.i.i.i.i.i, i64 8
  %self1.val2.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !36, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self1.val2.i.i.i.i.i.i.i, i64 noundef %self1.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !36
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i, %bb8.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !36
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !36
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !35
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !36, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !36
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !35
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsb2sIUxn0KTS_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #17, !srcloc !42
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Csb2sIUxn0KTS_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #3 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsb2sIUxn0KTS_18build_script_build(ptr noundef nonnull %_4) #20
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCsb2sIUxn0KTS_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsb2sIUxn0KTS_18build_script_build(ptr noundef nonnull readonly %0) #20, !noalias !43
  ret i32 0
}

; build_script_build::compile_probe
; Function Attrs: uwtable
define internal fastcc noundef zeroext i1 @_RNvCsb2sIUxn0KTS_18build_script_build13compile_probe(i1 noundef zeroext %rustc_bootstrap) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %x.sroa.0.i.i.i = alloca i64, align 8
  %_5.sroa.0.i = alloca i64, align 8
  %iter.i = alloca [72 x i8], align 8
  %_2.i113 = alloca [200 x i8], align 8
  %_3.i = alloca [4 x i8], align 2
  %_7.i74 = alloca [16 x i8], align 8
  %_2.i75 = alloca [24 x i8], align 8
  %key.i76 = alloca [16 x i8], align 8
  %_7.i = alloca [16 x i8], align 8
  %_2.i = alloca [24 x i8], align 8
  %key.i = alloca [16 x i8], align 8
  %args3 = alloca [32 x i8], align 8
  %_87 = alloca [16 x i8], align 8
  %err2 = alloca [8 x i8], align 8
  %_75 = alloca [16 x i8], align 8
  %iter = alloca [72 x i8], align 8
  %_64 = alloca [32 x i8], align 8
  %_57 = alloca [24 x i8], align 8
  %cmd = alloca [200 x i8], align 8
  %rustc1 = alloca [72 x i8], align 8
  %_29 = alloca [24 x i8], align 8
  %_27 = alloca [24 x i8], align 8
  %args = alloca [32 x i8], align 8
  %_19 = alloca [16 x i8], align 8
  %err = alloca [8 x i8], align 8
  %probefile = alloca [24 x i8], align 8
  %out_subdir = alloca [24 x i8], align 8
  %_2 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11)
  %0 = load i64, ptr %_2, align 8, !range !41, !noundef !3
  switch i64 %0, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit73
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_2, i64 8
  %_2.val59 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val59, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %start, %bb2.i.i.i4.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  br label %bb68

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit73: ; preds = %start
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i)
  store ptr @alloc_806c1ac911172019779ceab530bc1f0e, ptr %key.i, align 8, !noalias !46
  %2 = getelementptr inbounds nuw i8, ptr %key.i, i64 8
  store i64 5, ptr %2, align 8, !noalias !46
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i), !noalias !46
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5), !noalias !50
  %3 = load i64, ptr %_2.i, align 8, !range !41, !noalias !46, !noundef !3
  %.not.i = icmp eq i64 %3, -9223372036854775808
  br i1 %.not.i, label %bb3.i, label %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit

bb3.i:                                            ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit73
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i), !noalias !46
  store ptr %key.i, ptr %_7.i, align 8, !noalias !46
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_7.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsb2sIUxn0KTS_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i, align 8, !noalias !46
; call std::io::stdio::_eprint
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i), !noalias !50
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i), !noalias !46
; call std::process::exit
  call void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21, !noalias !50
  unreachable

_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit: ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit73
  %rustc.sroa.5.0._2.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i, i64 8
  %rustc.sroa.5.0.copyload210 = load ptr, ptr %rustc.sroa.5.0._2.i.sroa_idx, align 8, !noalias !51
  %rustc.sroa.6.0._2.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i, i64 16
  %rustc.sroa.6.0.copyload211 = load i64, ptr %rustc.sroa.6.0._2.i.sroa_idx, align 8, !noalias !51
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !46
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i76)
  store ptr @alloc_ebcdb5f66b6f511cde89ece546cbdd6d, ptr %key.i76, align 8, !noalias !52
  %4 = getelementptr inbounds nuw i8, ptr %key.i76, i64 8
  store i64 7, ptr %4, align 8, !noalias !52
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i75), !noalias !52
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i75, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ebcdb5f66b6f511cde89ece546cbdd6d, i64 noundef 7)
          to label %.noexc unwind label %bb93.thread

.noexc:                                           ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit
  %5 = load i64, ptr %_2.i75, align 8, !range !41, !noalias !52, !noundef !3
  %.not.i77 = icmp eq i64 %5, -9223372036854775808
  br i1 %.not.i77, label %bb3.i78, label %bb7

bb3.i78:                                          ; preds = %.noexc
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i74), !noalias !52
  store ptr %key.i76, ptr %_7.i74, align 8, !noalias !52
  %_8.sroa.4.0..sroa_idx.i79 = getelementptr inbounds nuw i8, ptr %_7.i74, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsb2sIUxn0KTS_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i79, align 8, !noalias !52
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i74)
          to label %.noexc80 unwind label %bb93.thread

.noexc80:                                         ; preds = %bb3.i78
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i74), !noalias !52
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %.noexc81 unwind label %bb93.thread

.noexc81:                                         ; preds = %.noexc80
  unreachable

bb68:                                             ; preds = %bb66, %bb2.i.i.i4.i.i.i201, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit
  %success.sroa.0.0.off0 = phi i1 [ false, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit ], [ %success.sroa.0.1.off0, %bb2.i.i.i4.i.i.i201 ], [ %success.sroa.0.1.off0, %bb66 ]
  ret i1 %success.sroa.0.0.off0

bb93:                                             ; preds = %bb2.i.i.i4.i.i.i, %bb74
  br i1 %_98.sroa.0.2.off0, label %bb92, label %bb75

bb93.thread:                                      ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit, %bb3.i78, %.noexc80
  %6 = landingpad { ptr, i32 }
          cleanup
  br label %bb92

bb7:                                              ; preds = %.noexc
  %out_dir.sroa.5.0._2.i75.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i75, i64 8
  %out_dir.sroa.5.0.copyload = load ptr, ptr %out_dir.sroa.5.0._2.i75.sroa_idx, align 8, !noalias !56, !nonnull !3, !noundef !3
  %out_dir.sroa.8.0._2.i75.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i75, i64 16
  %out_dir.sroa.8.0.copyload = load i64, ptr %out_dir.sroa.8.0._2.i75.sroa_idx, align 8, !noalias !56
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i75), !noalias !52
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i76)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %out_subdir)
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %out_subdir, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %out_dir.sroa.5.0.copyload, i64 noundef %out_dir.sroa.8.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6b9c4d547a268aa1e418a0468c721e03, i64 noundef 5)
          to label %bb8 unwind label %cleanup4

bb74:                                             ; preds = %bb2.i.i.i4.i.i.i.i86, %bb73, %cleanup4
  %_98.sroa.0.2.off0 = phi i1 [ true, %cleanup4 ], [ %_98.sroa.0.4.off0, %bb73 ], [ %_98.sroa.0.4.off0, %bb2.i.i.i4.i.i.i.i86 ]
  %.pn46.pn.pn = phi { ptr, i32 } [ %8, %cleanup4 ], [ %.pn46.pn, %bb73 ], [ %.pn46.pn, %bb2.i.i.i4.i.i.i.i86 ]
  %7 = icmp eq i64 %5, 0
  br i1 %7, label %bb93, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb74
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_dir.sroa.5.0.copyload, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb93

cleanup4:                                         ; preds = %bb7
  %8 = landingpad { ptr, i32 }
          cleanup
  br label %bb74

bb8:                                              ; preds = %bb7
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %probefile)
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %probefile, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ec4612a7974a19b5164a1467c0223b7b, i64 noundef 3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9b952d500051094ec12d4caeaadde05b, i64 noundef 10)
          to label %bb9 unwind label %cleanup5

bb73:                                             ; preds = %bb91.thread258, %bb2.i.i.i4.i.i.i.i205, %bb90, %bb91, %cleanup5
  %_98.sroa.0.4.off0 = phi i1 [ false, %bb91 ], [ true, %cleanup5 ], [ %_98.sroa.0.5.off0227, %bb90 ], [ %_98.sroa.0.5.off0227, %bb2.i.i.i4.i.i.i.i205 ], [ false, %bb91.thread258 ]
  %.pn46.pn = phi { ptr, i32 } [ %.pn41, %bb91 ], [ %11, %cleanup5 ], [ %.pn46228, %bb90 ], [ %.pn46228, %bb2.i.i.i4.i.i.i.i205 ], [ %49, %bb91.thread258 ]
  %out_subdir.val = load i64, ptr %out_subdir, align 8
  %9 = icmp eq i64 %out_subdir.val, 0
  br i1 %9, label %bb74, label %bb2.i.i.i4.i.i.i.i86

bb2.i.i.i4.i.i.i.i86:                             ; preds = %bb73
  %10 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 8
  %out_subdir.val60 = load ptr, ptr %10, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_subdir.val60, i64 noundef %out_subdir.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb74

cleanup5:                                         ; preds = %bb8
  %11 = landingpad { ptr, i32 }
          cleanup
  br label %bb73

bb9:                                              ; preds = %bb8
  call void @llvm.experimental.noalias.scope.decl(metadata !57)
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_3.i), !noalias !57
  store i16 511, ptr %_3.i, align 2, !noalias !57
  %12 = getelementptr inbounds nuw i8, ptr %_3.i, i64 2
  store i8 0, ptr %12, align 2, !noalias !57
  %13 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 8
  %_2.val.i.i = load ptr, ptr %13, align 8, !alias.scope !57, !nonnull !3, !noundef !3
  %14 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 16
  %_2.val1.i.i = load i64, ptr %14, align 8, !alias.scope !57, !noundef !3
; invoke <std::fs::DirBuilder>::_create
  %_0.i.i87 = invoke noundef ptr @_RNvMsF_NtCs5sEH5CPMdak_3std2fsNtB5_10DirBuilder7__create(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(4) %_3.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i, i64 noundef %_2.val1.i.i)
          to label %bb10 unwind label %cleanup6

bb91:                                             ; preds = %bb71
  br i1 %_97.sroa.0.4.off0, label %bb90, label %bb73

cleanup6:                                         ; preds = %bb76, %bb9
  %15 = landingpad { ptr, i32 }
          cleanup
  br label %bb90

bb10:                                             ; preds = %bb9
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_3.i), !noalias !57
  %.not19 = icmp eq ptr %_0.i.i87, null
  br i1 %.not19, label %bb76, label %bb11

bb11:                                             ; preds = %bb10
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err)
  store ptr %_0.i.i87, ptr %err, align 8
; call <std::io::error::Error>::kind
  %_14 = call fastcc noundef i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr nonnull %_0.i.i87)
  %_118.not = icmp eq i8 %_14, 12
  br i1 %_118.not, label %bb15, label %bb13

bb76:                                             ; preds = %bb16, %bb10
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_27)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_27, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f36ce88bd5d4a921175f5521f484b675, i64 noundef 13)
          to label %bb17 unwind label %cleanup6

cleanup7:                                         ; preds = %bb14, %bb13
  %16 = landingpad { ptr, i32 }
          cleanup
  %err.val66 = load ptr, ptr %err, align 8, !nonnull !3, !noundef !3
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsb2sIUxn0KTS_18build_script_build(ptr nonnull %err.val66) #18
          to label %bb90 unwind label %terminate

bb15:                                             ; preds = %bb11
  %bits.i.i.i.i = ptrtoint ptr %_0.i.i87 to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb16, !prof !28

bb2.i2.i.i.i:                                     ; preds = %bb15
  %17 = getelementptr i8, ptr %_0.i.i87, i64 -1
  %18 = icmp ne ptr %17, null
  call void @llvm.assume(i1 %18)
  %_6.val.i.i.i.i.i = load ptr, ptr %17, align 8
  %19 = getelementptr i8, ptr %_0.i.i87, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %19, align 8, !nonnull !3, !align !7, !noundef !3
  %20 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i = icmp eq ptr %20, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %21 = icmp ne ptr %_6.val.i.i.i.i.i, null
  call void @llvm.assume(i1 %21)
  invoke void %20(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %22 = icmp ne ptr %_6.val.i.i.i.i.i, null
  call void @llvm.assume(i1 %22)
  %23 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %24 = load i64, ptr %23, align 8, !range !8, !invariant.load !3
  %25 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %26 = load i64, ptr %25, align 8, !range !9, !invariant.load !3
  %27 = add i64 %26, -1
  %28 = icmp sgt i64 %27, -1
  call void @llvm.assume(i1 %28)
  %29 = icmp eq i64 %24, 0
  br i1 %29, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %24, i64 noundef range(i64 1, -9223372036854775807) %26) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  %31 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %32 = load i64, ptr %31, align 8, !range !8, !invariant.load !3
  %33 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %34 = load i64, ptr %33, align 8, !range !9, !invariant.load !3
  %35 = add i64 %34, -1
  %36 = icmp sgt i64 %35, -1
  call void @llvm.assume(i1 %36)
  %37 = icmp eq i64 %32, 0
  br i1 %37, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %32, i64 noundef range(i64 1, -9223372036854775807) %34) #17
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %17, i64 noundef 24, i64 noundef 8) #17
  br label %bb90

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %17, i64 noundef 24, i64 noundef 8) #17
  br label %bb16

bb13:                                             ; preds = %bb11
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_19)
  store ptr %_2.val.i.i, ptr %_19, align 8
  %38 = getelementptr inbounds nuw i8, ptr %_19, i64 8
  store i64 %_2.val1.i.i, ptr %38, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %_19, ptr %args, align 8
  %_22.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_22.sroa.4.0..sroa_idx, align 8
  %39 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %err, ptr %39, align 8
  %_23.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_23.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_ec0fb45a03827c6e6fbcf8024afc16d6, ptr noundef nonnull %args)
          to label %bb14 unwind label %cleanup7

bb16:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i, %bb15
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err)
  br label %bb76

bb14:                                             ; preds = %bb13
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_19)
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %unreachable unwind label %cleanup7

unreachable:                                      ; preds = %bb63, %bb14
  unreachable

terminate:                                        ; preds = %cleanup7, %cleanup14, %bb71
  %40 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

bb17:                                             ; preds = %bb76
  call void @llvm.experimental.noalias.scope.decl(metadata !60)
  %41 = load i64, ptr %_27, align 8, !range !41, !alias.scope !60, !noalias !63, !noundef !3
  %.not.i89 = icmp eq i64 %41, -9223372036854775808
  br i1 %.not.i89, label %bb18, label %bb2.i

bb2.i:                                            ; preds = %bb17
  %x.sroa.7.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_27, i64 8
  %x.sroa.7.0.copyload.i = load ptr, ptr %x.sroa.7.0.self.sroa_idx.i, align 8, !alias.scope !60, !noalias !63
  %x.sroa.9.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_27, i64 16
  %x.sroa.9.0.copyload.i = load i64, ptr %x.sroa.9.0.self.sroa_idx.i, align 8, !alias.scope !60, !noalias !63
  %_3.i.not.i = icmp eq i64 %x.sroa.9.0.copyload.i, 0
  br i1 %_3.i.not.i, label %bb4.i, label %bb18

bb4.i:                                            ; preds = %bb2.i
  %42 = icmp eq i64 %41, 0
  br i1 %42, label %bb18, label %bb2.i.i.i4.i.i.i5.i

bb2.i.i.i4.i.i.i5.i:                              ; preds = %bb4.i
  %43 = icmp ne ptr %x.sroa.7.0.copyload.i, null
  call void @llvm.assume(i1 %43)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload.i, i64 noundef %41, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !65
  br label %bb18

bb18:                                             ; preds = %bb17, %bb2.i.i.i4.i.i.i5.i, %bb4.i, %bb2.i
  %rustc_wrapper.sroa.9.0 = phi i64 [ undef, %bb17 ], [ undef, %bb2.i.i.i4.i.i.i5.i ], [ undef, %bb4.i ], [ %x.sroa.9.0.copyload.i, %bb2.i ]
  %rustc_wrapper.sroa.7.0 = phi ptr [ undef, %bb17 ], [ undef, %bb2.i.i.i4.i.i.i5.i ], [ undef, %bb4.i ], [ %x.sroa.7.0.copyload.i, %bb2.i ]
  %rustc_wrapper.sroa.0.0 = phi i64 [ -9223372036854775808, %bb17 ], [ -9223372036854775808, %bb2.i.i.i4.i.i.i5.i ], [ -9223372036854775808, %bb4.i ], [ %41, %bb2.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_27)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_29)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_29, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_fbe0d85396ee55e48aae2aa2891c1dc3, i64 noundef 23)
          to label %bb19 unwind label %bb88

bb19:                                             ; preds = %bb18
  call void @llvm.experimental.noalias.scope.decl(metadata !66)
  %44 = load i64, ptr %_29, align 8, !range !41, !alias.scope !66, !noalias !69, !noundef !3
  %.not.i93 = icmp eq i64 %44, -9223372036854775808
  br i1 %.not.i93, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i, label %bb2.i94

bb2.i94:                                          ; preds = %bb19
  %x.sroa.7.0.self.sroa_idx.i95 = getelementptr inbounds nuw i8, ptr %_29, i64 8
  %x.sroa.7.0.copyload.i96 = load ptr, ptr %x.sroa.7.0.self.sroa_idx.i95, align 8, !alias.scope !66, !noalias !69
  %x.sroa.9.0.self.sroa_idx.i97 = getelementptr inbounds nuw i8, ptr %_29, i64 16
  %x.sroa.9.0.copyload.i98 = load i64, ptr %x.sroa.9.0.self.sroa_idx.i97, align 8, !alias.scope !66, !noalias !69
  %_3.i.not.i99 = icmp eq i64 %x.sroa.9.0.copyload.i98, 0
  br i1 %_3.i.not.i99, label %bb4.i103, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i

bb4.i103:                                         ; preds = %bb2.i94
  %45 = icmp eq i64 %44, 0
  br i1 %45, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i, label %bb2.i.i.i4.i.i.i5.i104

bb2.i.i.i4.i.i.i5.i104:                           ; preds = %bb4.i103
  %46 = icmp ne ptr %x.sroa.7.0.copyload.i96, null
  call void @llvm.assume(i1 %46)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload.i96, i64 noundef %44, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !71
  br label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i: ; preds = %bb19, %bb2.i.i.i4.i.i.i5.i104, %bb4.i103, %bb2.i94
  %.val3.i.i.i = phi ptr [ undef, %bb19 ], [ undef, %bb2.i.i.i4.i.i.i5.i104 ], [ undef, %bb4.i103 ], [ %x.sroa.7.0.copyload.i96, %bb2.i94 ]
  %47 = phi i64 [ -9223372036854775808, %bb19 ], [ -9223372036854775808, %bb2.i.i.i4.i.i.i5.i104 ], [ -9223372036854775808, %bb4.i103 ], [ %44, %bb2.i94 ]
  %x.sroa.11.0.copyload7.i = phi i64 [ undef, %bb19 ], [ undef, %bb2.i.i.i4.i.i.i5.i104 ], [ undef, %bb4.i103 ], [ %x.sroa.9.0.copyload.i98, %bb2.i94 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_29)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %rustc1)
  call void @llvm.experimental.noalias.scope.decl(metadata !72)
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  %48 = getelementptr inbounds nuw i8, ptr %rustc1, i64 24
  %_31.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 32
  store ptr %rustc_wrapper.sroa.7.0, ptr %_31.sroa.4.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_31.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 40
  store i64 %rustc_wrapper.sroa.9.0, ptr %_31.sroa.5.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_31.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 48
  store i64 %47, ptr %_31.sroa.6.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_31.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 56
  store ptr %.val3.i.i.i, ptr %_31.sroa.7.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_31.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 64
  store i64 %x.sroa.11.0.copyload7.i, ptr %_31.sroa.8.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  store i64 %3, ptr %rustc1, align 8, !alias.scope !79, !noalias !72
  %_34.sroa.4.0.rustc1.sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 8
  store ptr %rustc.sroa.5.0.copyload210, ptr %_34.sroa.4.0.rustc1.sroa_idx, align 8, !alias.scope !79, !noalias !72
  %_34.sroa.5.0.rustc1.sroa_idx = getelementptr inbounds nuw i8, ptr %rustc1, i64 16
  store i64 %rustc.sroa.6.0.copyload211, ptr %_34.sroa.5.0.rustc1.sroa_idx, align 8, !alias.scope !79, !noalias !72
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd)
  call void @llvm.experimental.noalias.scope.decl(metadata !80)
  call void @llvm.experimental.noalias.scope.decl(metadata !83)
  %.not3.i.i.i.i = icmp eq i64 %rustc_wrapper.sroa.0.0, -9223372036854775808
  %spec.store.select.i.i.i.i = select i1 %.not3.i.i.i.i, i64 -9223372036854775807, i64 -9223372036854775808
  store i64 %spec.store.select.i.i.i.i, ptr %48, align 8, !alias.scope !86, !noalias !93
  br i1 %.not3.i.i.i.i, label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i, label %bb97

_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i: ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i
  store i64 -9223372036854775808, ptr %_31.sroa.6.0..sroa_idx, align 8, !alias.scope !96, !noalias !103
  %.not3.i = icmp eq i64 %47, -9223372036854775808
  br i1 %.not3.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i, label %bb97

bb91.thread258:                                   ; preds = %bb81
  %49 = landingpad { ptr, i32 }
          cleanup
  br label %bb73

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i: ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i
  store i64 -9223372036854775806, ptr %48, align 8, !alias.scope !105, !noalias !106
  store i64 -9223372036854775808, ptr %rustc1, align 8, !alias.scope !107, !noalias !114
  br label %bb97

bb97:                                             ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i
  %_38.sroa.0.0._38.sroa.0.0. = phi i64 [ %3, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.0.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %47, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i ]
  %_38.sroa.7.1 = phi ptr [ %rustc.sroa.5.0.copyload210, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.7.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %.val3.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i ]
  %_38.sroa.8.1 = phi i64 [ %rustc.sroa.6.0.copyload211, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsb2sIUxn0KTS_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.9.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %x.sroa.11.0.copyload7.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i ]
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i113), !noalias !116
  %50 = icmp ne ptr %_38.sroa.7.1, null
  call void @llvm.assume(i1 %50)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i113, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_38.sroa.7.1, i64 noundef %_38.sroa.8.1)
          to label %bb2.i115 unwind label %cleanup.i, !noalias !116

cleanup.i:                                        ; preds = %bb97
  %51 = landingpad { ptr, i32 }
          cleanup
  %52 = icmp eq i64 %_38.sroa.0.0._38.sroa.0.0., 0
  br i1 %52, label %bb86, label %bb2.i.i.i4.i.i.i.i114

bb2.i.i.i4.i.i.i.i114:                            ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_38.sroa.7.1, i64 noundef %_38.sroa.0.0._38.sroa.0.0., i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !116
  br label %bb86

bb2.i115:                                         ; preds = %bb97
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd, ptr noundef nonnull align 8 dereferenceable(200) %_2.i113, i64 200, i1 false), !noalias !120
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i113), !noalias !116
  %53 = icmp eq i64 %_38.sroa.0.0._38.sroa.0.0., 0
  br i1 %53, label %bb23, label %bb2.i.i.i4.i.i.i6.i

bb2.i.i.i4.i.i.i6.i:                              ; preds = %bb2.i115
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_38.sroa.7.1, i64 noundef %_38.sroa.0.0._38.sroa.0.0., i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !116
  br label %bb23

bb23:                                             ; preds = %bb2.i.i.i4.i.i.i6.i, %bb2.i115
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter.i), !noalias !121
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %iter.i, ptr noundef nonnull align 8 dereferenceable(72) %rustc1, i64 72, i1 false)
  %_3.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 24
  %_3.i.promoted.i = load i64, ptr %_3.i.i, align 8, !alias.scope !125, !noalias !130
  %x.sroa.7.0.opt.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 32
  %x.sroa.7.0.copyload7.i.i.i.i.i.i = load ptr, ptr %x.sroa.7.0.opt.sroa_idx.i.i.i.i.i.i, align 8, !noalias !121
  %x.sroa.8.0.opt.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 40
  %x.sroa.8.0.copyload8.i.i.i.i.i.i = load i64, ptr %x.sroa.8.0.opt.sroa_idx.i.i.i.i.i.i, align 8, !noalias !121
  %_57.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 48
  %x.sroa.9.0._57.i.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 56
  %x.sroa.9.0.copyload6.i.i.i = load ptr, ptr %x.sroa.9.0._57.i.i.sroa_idx.i.i.i, align 8, !noalias !121
  %x.sroa.11.0._57.i.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 64
  %x.sroa.11.0.copyload7.i.i.i = load i64, ptr %x.sroa.11.0._57.i.i.sroa_idx.i.i.i, align 8, !noalias !121
  %_5.sroa.8.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 8
  %_5.sroa.8.0.copyload12.i = load ptr, ptr %_5.sroa.8.0.iter.sroa_idx.i, align 8, !noalias !121
  %_5.sroa.9.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 16
  %_5.sroa.9.0.copyload13.i = load i64, ptr %_5.sroa.9.0.iter.sroa_idx.i, align 8, !noalias !121
  br label %bb2.i117

bb2.i117:                                         ; preds = %bb9.i, %bb23
  %54 = phi i64 [ %_3.i.promoted.i, %bb23 ], [ %65, %bb9.i ]
  %55 = phi i64 [ %_3.i.promoted.i, %bb23 ], [ %66, %bb9.i ]
  %_5.sroa.9.0.i = phi i64 [ undef, %bb23 ], [ %_5.sroa.9.220.i, %bb9.i ]
  %_5.sroa.8.0.i = phi ptr [ undef, %bb23 ], [ %_5.sroa.8.221.i, %bb9.i ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !133)
  call void @llvm.experimental.noalias.scope.decl(metadata !134)
  call void @llvm.experimental.noalias.scope.decl(metadata !135)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  %.not.i.i.i = icmp eq i64 %55, -9223372036854775806
  br i1 %.not.i.i.i, label %bb2.i.i.i, label %bb11.i.i.i

bb11.i.i.i:                                       ; preds = %bb2.i117
  call void @llvm.experimental.noalias.scope.decl(metadata !136)
  call void @llvm.experimental.noalias.scope.decl(metadata !139)
  %.not.i.i.i.i.i.i = icmp eq i64 %55, -9223372036854775807
  br i1 %.not.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i: ; preds = %bb11.i.i.i
  %.not3.i.i.i.i.i.i = icmp eq i64 %55, -9223372036854775808
  %spec.store.select.i.i.i.i.i.i = select i1 %.not3.i.i.i.i.i.i, i64 -9223372036854775807, i64 -9223372036854775808
  store i64 %spec.store.select.i.i.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !142, !noalias !147
  call void @llvm.experimental.noalias.scope.decl(metadata !149)
  br i1 %.not3.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %bb3.thread.i

bb2.i.i.i.i.i.i:                                  ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i, %bb11.i.i.i
  %56 = phi i64 [ -9223372036854775807, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ], [ %54, %bb11.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !152)
  %57 = load i64, ptr %_57.i.i.i.i.i, align 8, !range !18, !alias.scope !155, !noalias !158, !noundef !3
  %.not.i.i.i.i.i.i.i119 = icmp eq i64 %57, -9223372036854775807
  br i1 %.not.i.i.i.i.i.i.i119, label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i.i
  store i64 %57, ptr %x.sroa.0.i.i.i, align 8, !alias.scope !160, !noalias !164
  br label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i

_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i: ; preds = %bb5.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i
  %x.sroa.11.0.i.i.i = phi i64 [ undef, %bb2.i.i.i.i.i.i ], [ %x.sroa.11.0.copyload7.i.i.i, %bb5.i.i.i.i.i.i.i ]
  %x.sroa.9.0.i.i.i = phi ptr [ undef, %bb2.i.i.i.i.i.i ], [ %x.sroa.9.0.copyload6.i.i.i, %bb5.i.i.i.i.i.i.i ]
  %_1.sink.i.i.i.i.i.i.i = phi ptr [ %x.sroa.0.i.i.i, %bb2.i.i.i.i.i.i ], [ %_57.i.i.i.i.i, %bb5.i.i.i.i.i.i.i ]
  store i64 -9223372036854775808, ptr %_1.sink.i.i.i.i.i.i.i, align 8, !alias.scope !165, !noalias !166
  %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i = load i64, ptr %x.sroa.0.i.i.i, align 8, !noalias !167
  %.not3.i.i.i = icmp eq i64 %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i, -9223372036854775808
  br i1 %.not3.i.i.i, label %bb4.i.i.i.i.i, label %bb3.thread.i

bb4.i.i.i.i.i:                                    ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !168)
  call void @llvm.experimental.noalias.scope.decl(metadata !171)
  %.val2.i.i.i.i.i = load i64, ptr %_57.i.i.i.i.i, align 8, !range !18, !alias.scope !174, !noalias !130, !noundef !3
  switch i64 %.val2.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i.i120
    i64 -9223372036854775808, label %bb4.i.i.i120
    i64 0, label %bb4.i.i.i120
  ]

bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i:              ; preds = %bb4.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.9.0.copyload6.i.i.i, i64 noundef %.val2.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !175
  br label %bb4.i.i.i120

bb4.i.i.i120:                                     ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i, %bb4.i.i.i.i.i, %bb4.i.i.i.i.i, %bb4.i.i.i.i.i
  store i64 -9223372036854775806, ptr %_3.i.i, align 8, !alias.scope !125, !noalias !130
  br label %bb2.i.i.i

bb3.thread.i:                                     ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i
  %58 = phi i64 [ %56, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ -9223372036854775808, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ]
  %59 = phi i64 [ -9223372036854775807, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ -9223372036854775808, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.7.0.i.i = phi i64 [ %x.sroa.11.0.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %x.sroa.8.0.copyload8.i.i.i.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.6.0.i.i = phi ptr [ %x.sroa.9.0.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %x.sroa.7.0.copyload7.i.i.i.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.0.0.i.i = phi i64 [ %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build.exit.i.i.i ], [ %55, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !176)
  call void @llvm.experimental.noalias.scope.decl(metadata !179)
  call void @llvm.experimental.noalias.scope.decl(metadata !181)
  store i64 %_2.sroa.0.0.i.i, ptr %_5.sroa.0.i, align 8, !alias.scope !183, !noalias !184
  br label %bb7.i

bb2.i.i.i:                                        ; preds = %bb4.i.i.i120, %bb2.i117
  %60 = phi i64 [ -9223372036854775806, %bb4.i.i.i120 ], [ %54, %bb2.i117 ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !185)
  %61 = load i64, ptr %iter.i, align 8, !range !18, !alias.scope !188, !noalias !190, !noundef !3
  %.not.i.i.i.i121 = icmp eq i64 %61, -9223372036854775807
  br i1 %.not.i.i.i.i121, label %bb3.i122, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb2.i.i.i
  store i64 %61, ptr %_5.sroa.0.i, align 8, !alias.scope !191, !noalias !195
  br label %bb3.i122

bb12.i:                                           ; preds = %bb2.i.i.i4.i.i.i.i118, %cleanup1.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::option::IntoIter<std::ffi::os_str::OsString>>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %iter.i) #18, !noalias !196
  br label %bb71

bb3.i122:                                         ; preds = %bb5.i.i.i.i, %bb2.i.i.i
  %_5.sroa.9.1.i = phi i64 [ %_5.sroa.9.0.i, %bb2.i.i.i ], [ %_5.sroa.9.0.copyload13.i, %bb5.i.i.i.i ]
  %_5.sroa.8.1.i = phi ptr [ %_5.sroa.8.0.i, %bb2.i.i.i ], [ %_5.sroa.8.0.copyload12.i, %bb5.i.i.i.i ]
  %_1.sink.i.i.i.i = phi ptr [ %_5.sroa.0.i, %bb2.i.i.i ], [ %iter.i, %bb5.i.i.i.i ]
  store i64 -9223372036854775808, ptr %_1.sink.i.i.i.i, align 8, !alias.scope !197, !noalias !195
  %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i = load i64, ptr %_5.sroa.0.i, align 8, !noalias !121
  %.not.i123 = icmp eq i64 %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i, -9223372036854775808
  br i1 %.not.i123, label %bb6.i124, label %bb7.i

bb6.i124:                                         ; preds = %bb3.i122
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !198)
  call void @llvm.experimental.noalias.scope.decl(metadata !201)
  %62 = icmp eq i64 %60, -9223372036854775806
  br i1 %62, label %bb4.i.i, label %bb2.i.i8.i

bb2.i.i8.i:                                       ; preds = %bb6.i124
  call void @llvm.experimental.noalias.scope.decl(metadata !204)
  switch i64 %60, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i.i.i
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb2.i.i8.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload7.i.i.i.i.i.i, i64 noundef %60, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !207
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i8.i, %bb2.i.i8.i, %bb2.i.i8.i
  %.val2.i.i.i.i = load i64, ptr %_57.i.i.i.i.i, align 8, !range !18, !alias.scope !208, !noalias !121, !noundef !3
  switch i64 %.val2.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i
    i64 -9223372036854775808, label %bb4.i.i
    i64 0, label %bb4.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i:                ; preds = %bb4.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.9.0.copyload6.i.i.i, i64 noundef %.val2.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !207
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i, %bb4.i.i.i.i, %bb4.i.i.i.i, %bb4.i.i.i.i, %bb6.i124
  %_1.val2.i.i = load i64, ptr %iter.i, align 8, !range !18, !alias.scope !198, !noalias !121, !noundef !3
  switch i64 %_1.val2.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i4.i.i [
    i64 -9223372036854775807, label %bb24
    i64 -9223372036854775808, label %bb24
    i64 0, label %bb24
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i4.i.i:                  ; preds = %bb4.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.0.copyload12.i, i64 noundef %_1.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !209
  br label %bb24

cleanup1.i:                                       ; preds = %bb7.i
  %63 = landingpad { ptr, i32 }
          cleanup
  %64 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.19.i, 0
  br i1 %64, label %bb12.i, label %bb2.i.i.i4.i.i.i.i118

bb2.i.i.i4.i.i.i.i118:                            ; preds = %cleanup1.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.221.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !196
  br label %bb12.i

bb7.i:                                            ; preds = %bb3.i122, %bb3.thread.i
  %65 = phi i64 [ %58, %bb3.thread.i ], [ %60, %bb3.i122 ]
  %66 = phi i64 [ %59, %bb3.thread.i ], [ -9223372036854775806, %bb3.i122 ]
  %_5.sroa.8.221.i = phi ptr [ %_2.sroa.6.0.i.i, %bb3.thread.i ], [ %_5.sroa.8.1.i, %bb3.i122 ]
  %_5.sroa.9.220.i = phi i64 [ %_2.sroa.7.0.i.i, %bb3.thread.i ], [ %_5.sroa.9.1.i, %bb3.i122 ]
  %_5.sroa.0.0._5.sroa.0.0.19.i = phi i64 [ %_2.sroa.0.0.i.i, %bb3.thread.i ], [ %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i, %bb3.i122 ]
  %67 = icmp ne ptr %_5.sroa.8.221.i, null
  call void @llvm.assume(i1 %67)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.8.221.i, i64 noundef %_5.sroa.9.220.i)
          to label %bb8.i unwind label %cleanup1.i, !noalias !196

bb8.i:                                            ; preds = %bb7.i
  %68 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.19.i, 0
  br i1 %68, label %bb9.i, label %bb2.i.i.i4.i.i.i9.i

bb2.i.i.i4.i.i.i9.i:                              ; preds = %bb8.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.221.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !196
  br label %bb9.i

bb9.i:                                            ; preds = %bb2.i.i.i4.i.i.i9.i, %bb8.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  br label %bb2.i117

bb71:                                             ; preds = %cleanup12, %bb2.i.i.i4.i.i167, %bb2.i.i.i4.i.i.i165, %bb84, %bb2.i.i.i4.i.i.i.i159, %cleanup.i157, %cleanup.i144, %bb2.i.i.i4.i.i.i.i.i, %bb12.i, %bb1.i.i.i.i.i.i, %bb1.i.i.i.i.i191, %cleanup10, %cleanup14
  %_97.sroa.0.4.off0 = phi i1 [ false, %cleanup14 ], [ true, %bb12.i ], [ false, %bb1.i.i.i.i.i.i ], [ %_97.sroa.0.5.off0, %cleanup10 ], [ false, %bb1.i.i.i.i.i191 ], [ false, %bb2.i.i.i4.i.i.i.i.i ], [ false, %cleanup.i144 ], [ false, %cleanup.i157 ], [ false, %bb2.i.i.i4.i.i.i.i159 ], [ false, %bb84 ], [ false, %bb2.i.i.i4.i.i.i165 ], [ false, %bb2.i.i.i4.i.i167 ], [ false, %cleanup12 ]
  %.pn41 = phi { ptr, i32 } [ %122, %cleanup14 ], [ %63, %bb12.i ], [ %113, %bb1.i.i.i.i.i.i ], [ %69, %cleanup10 ], [ %136, %bb1.i.i.i.i.i191 ], [ %70, %bb2.i.i.i4.i.i.i.i.i ], [ %70, %cleanup.i144 ], [ %75, %cleanup.i157 ], [ %75, %bb2.i.i.i4.i.i.i.i159 ], [ %78, %bb84 ], [ %78, %bb2.i.i.i4.i.i.i165 ], [ %86, %bb2.i.i.i4.i.i167 ], [ %86, %cleanup12 ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd) #18
          to label %bb91 unwind label %terminate

cleanup10:                                        ; preds = %bb57, %bb77, %bb37, %bb35, %bb34, %bb33, %bb32, %bb31, %bb30, %bb29, %bb28, %bb27, %bb25, %bb78
  %_97.sroa.0.5.off0 = phi i1 [ false, %bb78 ], [ true, %bb25 ], [ true, %bb27 ], [ true, %bb28 ], [ true, %bb29 ], [ true, %bb30 ], [ true, %bb31 ], [ true, %bb32 ], [ true, %bb33 ], [ true, %bb34 ], [ true, %bb35 ], [ false, %bb37 ], [ false, %bb77 ], [ false, %bb57 ]
  %69 = landingpad { ptr, i32 }
          cleanup
  br label %bb71

bb24:                                             ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i4.i.i, %bb4.i.i, %bb4.i.i, %bb4.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter.i), !noalias !121
  br i1 %rustc_bootstrap, label %bb27, label %bb25

bb25:                                             ; preds = %bb24
  %_4.i = getelementptr inbounds nuw i8, ptr %cmd, i64 96
; invoke <std::sys::process::env::CommandEnv>::remove
  invoke void @_RNvMs_NtNtNtCs5sEH5CPMdak_3std3sys7process3envNtB4_10CommandEnv6remove(ptr noalias noundef nonnull align 8 dereferenceable(32) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d563101362ed4a06747b9210d55c4c5b, i64 noundef 15)
          to label %bb27 unwind label %cleanup10

bb27:                                             ; preds = %bb25, %bb24
; invoke <std::sys::process::unix::common::Command>::stderr
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command6stderr(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, i32 noundef 1, i32 undef)
          to label %bb28 unwind label %cleanup10

bb28:                                             ; preds = %bb27
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_74b6a34a01881648f3efb1322e7c9ba4, i64 noundef 24)
          to label %bb29 unwind label %cleanup10

bb29:                                             ; preds = %bb28
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_31c405a4038c01b5a14020c6d50bb4ce, i64 noundef 14)
          to label %bb30 unwind label %cleanup10

bb30:                                             ; preds = %bb29
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1e60bf274ec782b0083f842c06658d23, i64 noundef 19)
          to label %bb31 unwind label %cleanup10

bb31:                                             ; preds = %bb30
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a6e356e753364954471dcbf409cc4c4e, i64 noundef 16)
          to label %bb32 unwind label %cleanup10

bb32:                                             ; preds = %bb31
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9930910b9e2bf161f6d41704390848d2, i64 noundef 17)
          to label %bb33 unwind label %cleanup10

bb33:                                             ; preds = %bb32
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4d01ae01a4b8e52c5a54208511747587, i64 noundef 24)
          to label %bb34 unwind label %cleanup10

bb34:                                             ; preds = %bb33
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_8bbf703e0ecc0326ac386a57604275da, i64 noundef 9)
          to label %bb35 unwind label %cleanup10

bb35:                                             ; preds = %bb34
  %_2.val.i.i141 = load ptr, ptr %13, align 8, !alias.scope !210, !noalias !213, !nonnull !3, !noundef !3
  %_2.val1.i.i142 = load i64, ptr %14, align 8, !alias.scope !210, !noalias !213, !noundef !3
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i141, i64 noundef %_2.val1.i.i142)
          to label %bb36 unwind label %cleanup10

bb36:                                             ; preds = %bb35
  %_56.sroa.0.0.copyload = load i64, ptr %probefile, align 8
  %_56.sroa.5.0.probefile.sroa_idx = getelementptr inbounds nuw i8, ptr %probefile, i64 8
  %_56.sroa.5.0.copyload = load ptr, ptr %_56.sroa.5.0.probefile.sroa_idx, align 8, !nonnull !3, !noundef !3
  %_56.sroa.6.0.probefile.sroa_idx = getelementptr inbounds nuw i8, ptr %probefile, i64 16
  %_56.sroa.6.0.copyload = load i64, ptr %_56.sroa.6.0.probefile.sroa_idx, align 8
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_56.sroa.5.0.copyload, i64 noundef %_56.sroa.6.0.copyload)
          to label %bb2.i146 unwind label %cleanup.i144, !noalias !215

cleanup.i144:                                     ; preds = %bb36
  %70 = landingpad { ptr, i32 }
          cleanup
  %71 = icmp eq i64 %_56.sroa.0.0.copyload, 0
  br i1 %71, label %bb71, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i144
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_56.sroa.5.0.copyload, i64 noundef %_56.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !215
  br label %bb71

bb2.i146:                                         ; preds = %bb36
  %72 = icmp eq i64 %_56.sroa.0.0.copyload, 0
  br i1 %72, label %bb37, label %bb2.i.i.i4.i.i.i.i6.i

bb2.i.i.i4.i.i.i.i6.i:                            ; preds = %bb2.i146
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_56.sroa.5.0.copyload, i64 noundef %_56.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !215
  br label %bb37

bb37:                                             ; preds = %bb2.i.i.i4.i.i.i.i6.i, %bb2.i146
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_57)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_57, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dcbc225a8ec7dbfaaef714ff8a7176fb, i64 noundef 6)
          to label %bb38 unwind label %cleanup10

bb38:                                             ; preds = %bb37
  %73 = load i64, ptr %_57, align 8, !range !41, !noundef !3
  %.not29 = icmp eq i64 %73, -9223372036854775808
  br i1 %.not29, label %bb77, label %bb39

bb39:                                             ; preds = %bb38
  %target.sroa.5.0._57.sroa_idx = getelementptr inbounds nuw i8, ptr %_57, i64 8
  %target.sroa.5.0.copyload = load ptr, ptr %target.sroa.5.0._57.sroa_idx, align 8
  %target.sroa.6.0._57.sroa_idx = getelementptr inbounds nuw i8, ptr %_57, i64 16
  %target.sroa.6.0.copyload = load i64, ptr %target.sroa.6.0._57.sroa_idx, align 8
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c20974c698c079af35c03642327d3f4f, i64 noundef 8)
          to label %bb40 unwind label %bb84

bb77:                                             ; preds = %bb2.i161, %bb2.i.i.i4.i.i.i6.i163, %bb38
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_57)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_64)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_64, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
          to label %bb42 unwind label %cleanup10

bb40:                                             ; preds = %bb39
  %74 = icmp ne ptr %target.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %74)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %target.sroa.5.0.copyload, i64 noundef %target.sroa.6.0.copyload)
          to label %bb2.i161 unwind label %cleanup.i157, !noalias !218

cleanup.i157:                                     ; preds = %bb40
  %75 = landingpad { ptr, i32 }
          cleanup
  %76 = icmp eq i64 %73, 0
  br i1 %76, label %bb71, label %bb2.i.i.i4.i.i.i.i159

bb2.i.i.i4.i.i.i.i159:                            ; preds = %cleanup.i157
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %73, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !218
  br label %bb71

bb2.i161:                                         ; preds = %bb40
  %77 = icmp eq i64 %73, 0
  br i1 %77, label %bb77, label %bb2.i.i.i4.i.i.i6.i163

bb2.i.i.i4.i.i.i6.i163:                           ; preds = %bb2.i161
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %73, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !218
  br label %bb77

bb84:                                             ; preds = %bb39
  %78 = landingpad { ptr, i32 }
          cleanup
  %79 = icmp eq i64 %73, 0
  br i1 %79, label %bb71, label %bb2.i.i.i4.i.i.i165

bb2.i.i.i4.i.i.i165:                              ; preds = %bb84
  %80 = icmp ne ptr %target.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %80)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %73, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb71

bb42:                                             ; preds = %bb77
  %_65 = load i64, ptr %_64, align 8, !range !221, !noundef !3
  %81 = trunc nuw i64 %_65 to i1
  br i1 %81, label %bb3.i172, label %bb43

bb43:                                             ; preds = %bb42
  %82 = getelementptr inbounds nuw i8, ptr %_64, i64 8
  %rustflags.sroa.0.0.copyload = load i64, ptr %82, align 8
  %rustflags.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_64, i64 16
  %rustflags.sroa.5.0.copyload = load ptr, ptr %rustflags.sroa.5.0..sroa_idx, align 8
  %rustflags.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_64, i64 24
  %rustflags.sroa.8.0.copyload = load i64, ptr %rustflags.sroa.8.0..sroa_idx, align 8
  %_157 = icmp sgt i64 %rustflags.sroa.8.0.copyload, -1
  call void @llvm.assume(i1 %_157)
  %83 = icmp eq i64 %rustflags.sroa.8.0.copyload, 0
  br i1 %83, label %bb51, label %bb98

bb51:                                             ; preds = %bb43, %bb49
  %84 = icmp eq i64 %rustflags.sroa.0.0.copyload, 0
  br i1 %84, label %bb78, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb51
  %85 = icmp ne ptr %rustflags.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %85)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb78

cleanup12:                                        ; preds = %bb48, %bb46
  %86 = landingpad { ptr, i32 }
          cleanup
  %87 = icmp eq i64 %rustflags.sroa.0.0.copyload, 0
  br i1 %87, label %bb71, label %bb2.i.i.i4.i.i167

bb2.i.i.i4.i.i167:                                ; preds = %cleanup12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb71

bb98:                                             ; preds = %bb43
  %88 = icmp ne ptr %rustflags.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %88)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter)
  store i64 0, ptr %iter, align 8
  %_67.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %rustflags.sroa.8.0.copyload, ptr %_67.sroa.2.0.iter.sroa_idx, align 8
  %_67.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %rustflags.sroa.5.0.copyload, ptr %_67.sroa.3.0.iter.sroa_idx, align 8
  %_67.sroa.3.sroa.2.0._67.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store i64 %rustflags.sroa.8.0.copyload, ptr %_67.sroa.3.sroa.2.0._67.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_67.sroa.3.sroa.3.0._67.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 0, ptr %_67.sroa.3.sroa.3.0._67.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_67.sroa.3.sroa.4.0._67.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  store i64 %rustflags.sroa.8.0.copyload, ptr %_67.sroa.3.sroa.4.0._67.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_67.sroa.3.sroa.5.0._67.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store <2 x i32> splat (i32 31), ptr %_67.sroa.3.sroa.5.0._67.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_67.sroa.3.sroa.7.0._67.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 56
  store i8 1, ptr %_67.sroa.3.sroa.7.0._67.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_67.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i8 1, ptr %_67.sroa.4.0.iter.sroa_idx, align 8
  %_67.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 65
  store i8 0, ptr %_67.sroa.5.0.iter.sroa_idx, align 1
  br label %bb46

bb46:                                             ; preds = %bb48, %bb98
; invoke <core::str::iter::SplitInternal<char>>::next
  %89 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %iter)
          to label %bb99 unwind label %cleanup12

bb99:                                             ; preds = %bb46
  %90 = extractvalue { ptr, i64 } %89, 0
  %.not30 = icmp eq ptr %90, null
  br i1 %.not30, label %bb49, label %bb48

bb48:                                             ; preds = %bb99
  %91 = extractvalue { ptr, i64 } %89, 1
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %90, i64 noundef %91)
          to label %bb46 unwind label %cleanup12

bb49:                                             ; preds = %bb99
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter)
  br label %bb51

bb3.i172:                                         ; preds = %bb42
  call void @llvm.experimental.noalias.scope.decl(metadata !222)
  %92 = getelementptr inbounds nuw i8, ptr %_64, i64 8
  %.val.i = load i64, ptr %92, align 8, !alias.scope !222
  switch i64 %.val.i, label %bb1.sink.split.i [
    i64 -9223372036854775808, label %bb78
    i64 0, label %bb78
  ]

bb1.sink.split.i:                                 ; preds = %bb3.i172
  %93 = getelementptr inbounds nuw i8, ptr %_64, i64 16
  %.val3.i = load ptr, ptr %93, align 8, !alias.scope !222, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i, i64 noundef %.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !222
  br label %bb78

bb78:                                             ; preds = %bb51, %bb2.i.i.i4.i.i, %bb1.sink.split.i, %bb3.i172, %bb3.i172
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_64)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_75)
; invoke <std::process::Command>::status
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6status(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_75, ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd)
          to label %bb53 unwind label %cleanup10

bb53:                                             ; preds = %bb78
  %94 = load i32, ptr %_75, align 8, !range !225, !noundef !3
  %95 = trunc nuw i32 %94 to i1
  %96 = getelementptr inbounds nuw i8, ptr %_75, i64 4
  %status = load i32, ptr %96, align 4
  %.not32 = icmp eq i32 %status, 0
  %not. = xor i1 %95, true
  %success.sroa.0.1.off0 = select i1 %not., i1 %.not32, i1 false
  %97 = getelementptr inbounds nuw i8, ptr %_75, i64 8
  %_75.val71 = load ptr, ptr %97, align 8
  %98 = icmp eq i32 %94, 0
  br i1 %98, label %bb57, label %bb2.i174

bb2.i174:                                         ; preds = %bb53
  %99 = icmp ne ptr %_75.val71, null
  call void @llvm.assume(i1 %99)
  %bits.i.i.i.i.i = ptrtoint ptr %_75.val71 to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %bb57, !prof !28

bb2.i2.i.i.i.i:                                   ; preds = %bb2.i174
  %100 = getelementptr i8, ptr %_75.val71, i64 -1
  %101 = icmp ne ptr %100, null
  call void @llvm.assume(i1 %101)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %100, align 8
  %102 = getelementptr i8, ptr %_75.val71, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %102, align 8, !nonnull !3, !align !7, !noundef !3
  %103 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %103, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %104 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %104)
  invoke void %103(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %105 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %105)
  %106 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %107 = load i64, ptr %106, align 8, !range !8, !invariant.load !3
  %108 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %109 = load i64, ptr %108, align 8, !range !9, !invariant.load !3
  %110 = add i64 %109, -1
  %111 = icmp sgt i64 %110, -1
  call void @llvm.assume(i1 %111)
  %112 = icmp eq i64 %107, 0
  br i1 %112, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %107, i64 noundef range(i64 1, -9223372036854775807) %109) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %113 = landingpad { ptr, i32 }
          cleanup
  %114 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %115 = load i64, ptr %114, align 8, !range !8, !invariant.load !3
  %116 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %117 = load i64, ptr %116, align 8, !range !9, !invariant.load !3
  %118 = add i64 %117, -1
  %119 = icmp sgt i64 %118, -1
  call void @llvm.assume(i1 %119)
  %120 = icmp eq i64 %115, 0
  br i1 %120, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %115, i64 noundef range(i64 1, -9223372036854775807) %117) #17
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %100, i64 noundef 24, i64 noundef 8) #17
  br label %bb71

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %100, i64 noundef 24, i64 noundef 8) #17
  br label %bb57

bb57:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i, %bb2.i174, %bb53
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_75)
; invoke std::sys::fs::remove_dir_all
  %_0.i180 = invoke noundef ptr @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs14remove_dir_all(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i141, i64 noundef %_2.val1.i.i142)
          to label %bb58 unwind label %cleanup10

bb58:                                             ; preds = %bb57
  %.not34 = icmp eq ptr %_0.i180, null
  br i1 %.not34, label %bb81, label %bb59

bb59:                                             ; preds = %bb58
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err2)
  store ptr %_0.i180, ptr %err2, align 8
; call <std::io::error::Error>::kind
  %_82 = call fastcc noundef i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr nonnull %_0.i180)
  %121 = icmp eq i8 %_82, 0
  br i1 %121, label %bb61, label %bb62

bb81:                                             ; preds = %bb64, %bb58
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd)
          to label %bb65 unwind label %bb91.thread258

cleanup14:                                        ; preds = %bb63, %bb62
  %122 = landingpad { ptr, i32 }
          cleanup
  %err2.val64 = load ptr, ptr %err2, align 8, !nonnull !3, !noundef !3
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsb2sIUxn0KTS_18build_script_build(ptr nonnull %err2.val64) #18
          to label %bb71 unwind label %terminate

bb61:                                             ; preds = %bb59
  %bits.i.i.i.i181 = ptrtoint ptr %_0.i180 to i64
  %_5.i.i.i.i182 = and i64 %bits.i.i.i.i181, 3
  %switch.i.i.i183 = icmp eq i64 %_5.i.i.i.i182, 1
  br i1 %switch.i.i.i183, label %bb2.i2.i.i.i184, label %bb64, !prof !28

bb2.i2.i.i.i184:                                  ; preds = %bb61
  %123 = getelementptr i8, ptr %_0.i180, i64 -1
  %124 = icmp ne ptr %123, null
  call void @llvm.assume(i1 %124)
  %_6.val.i.i.i.i.i185 = load ptr, ptr %123, align 8
  %125 = getelementptr i8, ptr %_0.i180, i64 7
  %_6.val1.i.i.i.i.i186 = load ptr, ptr %125, align 8, !nonnull !3, !align !7, !noundef !3
  %126 = load ptr, ptr %_6.val1.i.i.i.i.i186, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i187 = icmp eq ptr %126, null
  br i1 %.not.i.i.i.i.i.i.i187, label %bb3.i.i.i.i.i.i.i192, label %is_not_null.i.i.i.i.i.i.i188

is_not_null.i.i.i.i.i.i.i188:                     ; preds = %bb2.i2.i.i.i184
  %127 = icmp ne ptr %_6.val.i.i.i.i.i185, null
  call void @llvm.assume(i1 %127)
  invoke void %126(ptr noundef nonnull %_6.val.i.i.i.i.i185)
          to label %bb3.i.i.i.i.i.i.i192 unwind label %cleanup.i.i.i.i.i.i.i189

bb3.i.i.i.i.i.i.i192:                             ; preds = %is_not_null.i.i.i.i.i.i.i188, %bb2.i2.i.i.i184
  %128 = icmp ne ptr %_6.val.i.i.i.i.i185, null
  call void @llvm.assume(i1 %128)
  %129 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i186, i64 8
  %130 = load i64, ptr %129, align 8, !range !8, !invariant.load !3
  %131 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i186, i64 16
  %132 = load i64, ptr %131, align 8, !range !9, !invariant.load !3
  %133 = add i64 %132, -1
  %134 = icmp sgt i64 %133, -1
  call void @llvm.assume(i1 %134)
  %135 = icmp eq i64 %130, 0
  br i1 %135, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i194, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i193

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i193: ; preds = %bb3.i.i.i.i.i.i.i192
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i185, i64 noundef %130, i64 noundef range(i64 1, -9223372036854775807) %132) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i194

cleanup.i.i.i.i.i.i.i189:                         ; preds = %is_not_null.i.i.i.i.i.i.i188
  %136 = landingpad { ptr, i32 }
          cleanup
  %137 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i186, i64 8
  %138 = load i64, ptr %137, align 8, !range !8, !invariant.load !3
  %139 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i186, i64 16
  %140 = load i64, ptr %139, align 8, !range !9, !invariant.load !3
  %141 = add i64 %140, -1
  %142 = icmp sgt i64 %141, -1
  call void @llvm.assume(i1 %142)
  %143 = icmp eq i64 %138, 0
  br i1 %143, label %bb1.i.i.i.i.i191, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i190

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i190: ; preds = %cleanup.i.i.i.i.i.i.i189
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i185, i64 noundef %138, i64 noundef range(i64 1, -9223372036854775807) %140) #17
  br label %bb1.i.i.i.i.i191

bb1.i.i.i.i.i191:                                 ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i190, %cleanup.i.i.i.i.i.i.i189
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %123, i64 noundef 24, i64 noundef 8) #17
  br label %bb71

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i194: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i193, %bb3.i.i.i.i.i.i.i192
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %123, i64 noundef 24, i64 noundef 8) #17
  br label %bb64

bb62:                                             ; preds = %bb59
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_87)
  store ptr %_2.val.i.i141, ptr %_87, align 8
  %144 = getelementptr inbounds nuw i8, ptr %_87, i64 8
  store i64 %_2.val1.i.i142, ptr %144, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args3)
  store ptr %_87, ptr %args3, align 8
  %_90.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args3, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_90.sroa.4.0..sroa_idx, align 8
  %145 = getelementptr inbounds nuw i8, ptr %args3, i64 16
  store ptr %err2, ptr %145, align 8
  %_91.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args3, i64 24
  store ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_91.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_62e6ec0e1c3bfea4ae2f14deaee8dee9, ptr noundef nonnull %args3)
          to label %bb63 unwind label %cleanup14

bb64:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i194, %bb61
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err2)
  br label %bb81

bb63:                                             ; preds = %bb62
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args3)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_87)
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %unreachable unwind label %cleanup14

bb65:                                             ; preds = %bb81
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %rustc1)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %probefile)
  %out_subdir.val62 = load i64, ptr %out_subdir, align 8
  %146 = icmp eq i64 %out_subdir.val62, 0
  br i1 %146, label %bb66, label %bb2.i.i.i4.i.i.i.i199

bb2.i.i.i4.i.i.i.i199:                            ; preds = %bb65
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val.i.i141, i64 noundef %out_subdir.val62, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb66

bb66:                                             ; preds = %bb2.i.i.i4.i.i.i.i199, %bb65
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %out_subdir)
  %147 = icmp eq i64 %5, 0
  br i1 %147, label %bb68, label %bb2.i.i.i4.i.i.i201

bb2.i.i.i4.i.i.i201:                              ; preds = %bb66
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_dir.sroa.5.0.copyload, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb68

bb86:                                             ; preds = %bb2.i.i.i4.i.i.i.i114, %cleanup.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::option::IntoIter<std::ffi::os_str::OsString>>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %rustc1) #18
  br label %bb90

bb88:                                             ; preds = %bb18
  %148 = landingpad { ptr, i32 }
          cleanup
  switch i64 %rustc_wrapper.sroa.0.0, label %bb2.i.i.i4.i.i.i.i203 [
    i64 -9223372036854775808, label %bb90
    i64 0, label %bb90
  ]

bb2.i.i.i4.i.i.i.i203:                            ; preds = %bb88
  %149 = icmp ne ptr %rustc_wrapper.sroa.7.0, null
  call void @llvm.assume(i1 %149)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_wrapper.sroa.7.0, i64 noundef %rustc_wrapper.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb90

bb90:                                             ; preds = %bb86, %cleanup6, %bb1.i.i.i.i.i, %cleanup7, %bb2.i.i.i4.i.i.i.i203, %bb88, %bb88, %bb91
  %.pn46228 = phi { ptr, i32 } [ %.pn41, %bb91 ], [ %148, %bb88 ], [ %148, %bb88 ], [ %148, %bb2.i.i.i4.i.i.i.i203 ], [ %16, %cleanup7 ], [ %15, %cleanup6 ], [ %30, %bb1.i.i.i.i.i ], [ %51, %bb86 ]
  %_98.sroa.0.5.off0227 = phi i1 [ false, %bb91 ], [ true, %bb88 ], [ true, %bb88 ], [ true, %bb2.i.i.i4.i.i.i.i203 ], [ true, %cleanup7 ], [ true, %cleanup6 ], [ true, %bb1.i.i.i.i.i ], [ false, %bb86 ]
  %probefile.val = load i64, ptr %probefile, align 8
  %150 = icmp eq i64 %probefile.val, 0
  br i1 %150, label %bb73, label %bb2.i.i.i4.i.i.i.i205

bb2.i.i.i4.i.i.i.i205:                            ; preds = %bb90
  %151 = getelementptr inbounds nuw i8, ptr %probefile, i64 8
  %probefile.val61 = load ptr, ptr %151, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %probefile.val61, i64 noundef %probefile.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb73

bb75:                                             ; preds = %bb2.i.i.i4.i.i.i207, %bb92, %bb93
  %.pn46.pn.pn.pn222 = phi { ptr, i32 } [ %.pn46.pn.pn, %bb93 ], [ %.pn46.pn.pn.pn223, %bb92 ], [ %.pn46.pn.pn.pn223, %bb2.i.i.i4.i.i.i207 ]
  resume { ptr, i32 } %.pn46.pn.pn.pn222

bb92:                                             ; preds = %bb93.thread, %bb93
  %.pn46.pn.pn.pn223 = phi { ptr, i32 } [ %6, %bb93.thread ], [ %.pn46.pn.pn, %bb93 ]
  %152 = icmp eq i64 %3, 0
  br i1 %152, label %bb75, label %bb2.i.i.i4.i.i.i207

bb2.i.i.i4.i.i.i207:                              ; preds = %bb92
  %153 = icmp ne ptr %rustc.sroa.5.0.copyload210, null
  call void @llvm.assume(i1 %153)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.5.0.copyload210, i64 noundef %3, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb75
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCsb2sIUxn0KTS_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i10.i = alloca [200 x i8], align 8
  %_7.i.i = alloca [16 x i8], align 8
  %_2.i.i = alloca [24 x i8], align 8
  %key.i.i = alloca [16 x i8], align 8
  %pieces.i = alloca [72 x i8], align 8
  %_12.i = alloca [24 x i8], align 8
  %_8.i = alloca [200 x i8], align 8
  %_5.i = alloca [56 x i8], align 8
  %output.i = alloca [56 x i8], align 8
  %_6 = alloca [24 x i8], align 8
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a5ada7cbbef10488e9785c51c0789d49, ptr noundef nonnull inttoptr (i64 77 to ptr))
; call build_script_build::compile_probe
  %_5 = tail call fastcc noundef zeroext i1 @_RNvCsb2sIUxn0KTS_18build_script_build13compile_probe(i1 noundef zeroext false)
  br i1 %_5, label %bb14, label %bb4

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d563101362ed4a06747b9210d55c4c5b, i64 noundef 15)
  %0 = load i64, ptr %_6, align 8, !range !41, !noundef !3
  %.not3 = icmp eq i64 %0, -9223372036854775808
  br i1 %.not3, label %bb17.thread, label %bb6

bb17.thread:                                      ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  br label %bb18

bb6:                                              ; preds = %bb4
  %rustc_bootstrap.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %rustc_bootstrap.sroa.5.0.copyload = load ptr, ptr %rustc_bootstrap.sroa.5.0._6.sroa_idx, align 8
  %rustc_bootstrap.sroa.8.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %rustc_bootstrap.sroa.8.0.copyload = load i64, ptr %rustc_bootstrap.sroa.8.0._6.sroa_idx, align 8
; invoke build_script_build::compile_probe
  %_9 = invoke fastcc noundef zeroext i1 @_RNvCsb2sIUxn0KTS_18build_script_build13compile_probe(i1 noundef zeroext true)
          to label %bb7 unwind label %cleanup

cleanup:                                          ; preds = %bb6
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = icmp eq i64 %0, 0
  br i1 %2, label %common.resume, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup
  %3 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %3)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %common.resume

bb7:                                              ; preds = %bb6
  br i1 %_9, label %bb10, label %bb9

bb9:                                              ; preds = %bb7
  %4 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %4)
  %_3.not.i = icmp eq i64 %rustc_bootstrap.sroa.8.0.copyload, 1
  br i1 %_3.not.i, label %bb10.thread, label %bb10.thread.thread

bb10:                                             ; preds = %bb7
  %5 = icmp eq i64 %0, 0
  br i1 %5, label %bb18.critedge, label %bb2.i.i.i4.i.i.i10

bb10.thread:                                      ; preds = %bb9
  %lhsc = load i8, ptr %rustc_bootstrap.sroa.5.0.copyload, align 1
  %.not = icmp eq i8 %lhsc, 49
  %6 = icmp eq i64 %0, 0
  br i1 %6, label %bb13, label %bb17

bb10.thread.thread:                               ; preds = %bb9
  %7 = icmp eq i64 %0, 0
  br i1 %7, label %bb13.thread, label %bb17.thread58

bb17.thread58:                                    ; preds = %bb10.thread.thread
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  br label %bb18

bb13.thread:                                      ; preds = %bb10.thread.thread
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  br label %bb18

bb2.i.i.i4.i.i.i10:                               ; preds = %bb10
  %8 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %8)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb18.critedge

common.resume:                                    ; preds = %cleanup.i.i, %bb2.i.i.i4.i.i.i.i.i, %cleanup.body.i, %cleanup1.i, %cleanup, %bb2.i.i.i4.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %1, %bb2.i.i.i4.i.i.i ], [ %1, %cleanup ], [ %11, %bb2.i.i.i4.i.i.i.i.i ], [ %11, %cleanup.i.i ], [ %39, %cleanup1.i ], [ %eh.lpad-body.i, %cleanup.body.i ]
  resume { ptr, i32 } %common.resume.op

bb13:                                             ; preds = %bb10.thread
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  br i1 %.not, label %bb20, label %bb18

bb17:                                             ; preds = %bb10.thread
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  br i1 %.not, label %bb20, label %bb18

bb14:                                             ; preds = %start
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a0f0604af2246bca35707903288f4f53, ptr noundef nonnull inttoptr (i64 61 to ptr))
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_e181ada66eb53f56ba6935f91cca5e48, ptr noundef nonnull inttoptr (i64 89 to ptr))
  br label %bb20

bb20:                                             ; preds = %bb14, %bb13, %bb18, %bb17
  %error_generic_member_access.sroa.0.2.off01724 = phi i1 [ %error_generic_member_access.sroa.0.2.off01725, %bb18 ], [ false, %bb17 ], [ true, %bb14 ], [ false, %bb13 ]
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i.i)
  store ptr @alloc_806c1ac911172019779ceab530bc1f0e, ptr %key.i.i, align 8, !noalias !226
  %9 = getelementptr inbounds nuw i8, ptr %key.i.i, i64 8
  store i64 5, ptr %9, align 8, !noalias !226
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !226
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5), !noalias !230
  %10 = load i64, ptr %_2.i.i, align 8, !range !41, !noalias !226, !noundef !3
  %.not.i.i = icmp eq i64 %10, -9223372036854775808
  br i1 %.not.i.i, label %bb3.i.i, label %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit.i

bb3.i.i:                                          ; preds = %bb20
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i.i), !noalias !226
  store ptr %key.i.i, ptr %_7.i.i, align 8, !noalias !226
  %_8.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_7.i.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsb2sIUxn0KTS_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i.i, align 8, !noalias !226
; call std::io::stdio::_eprint
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i.i), !noalias !230
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i.i), !noalias !226
; call std::process::exit
  call void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21, !noalias !230
  unreachable

_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit.i: ; preds = %bb20
  %rustc.sroa.3.0._2.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %rustc.sroa.3.0.copyload.i = load ptr, ptr %rustc.sroa.3.0._2.i.sroa_idx.i, align 8, !noalias !231, !nonnull !3, !noundef !3
  %rustc.sroa.4.0._2.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %rustc.sroa.4.0.copyload.i = load i64, ptr %rustc.sroa.4.0._2.i.sroa_idx.i, align 8, !noalias !231
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i), !noalias !226
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_5.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_8.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i10.i), !noalias !232
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i10.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustc.sroa.3.0.copyload.i, i64 noundef %rustc.sroa.4.0.copyload.i)
          to label %bb2.i.i unwind label %cleanup.i.i, !noalias !232

cleanup.i.i:                                      ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit.i
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = icmp eq i64 %10, 0
  br i1 %12, label %common.resume, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !232
  br label %common.resume

bb2.i.i:                                          ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var.exit.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noundef nonnull align 8 dereferenceable(200) %_2.i10.i, i64 200, i1 false), !noalias !236
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i10.i), !noalias !232
  %13 = icmp eq i64 %10, 0
  br i1 %13, label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build.exit.i, label %bb2.i.i.i4.i.i.i6.i.i

bb2.i.i.i4.i.i.i6.i.i:                            ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !232
  br label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build.exit.i

_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build.exit.i: ; preds = %bb2.i.i.i4.i.i.i6.i.i, %bb2.i.i
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a887f9858119cc7413062dc002c4d9ab, i64 noundef 9)
          to label %bb3.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb3.i, %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build.exit.i
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %14, %cleanup.i ], [ %31, %bb1.i.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i) #18
          to label %common.resume unwind label %terminate.i

bb3.i:                                            ; preds = %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build.exit.i
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i)
          to label %bb4.i unwind label %cleanup.i

bb4.i:                                            ; preds = %bb3.i
  %15 = load i64, ptr %_5.i, align 8, !range !41, !noundef !3
  %16 = icmp eq i64 %15, -9223372036854775808
  br i1 %16, label %bb3.i13.i, label %bb25.i

bb3.i13.i:                                        ; preds = %bb4.i
  call void @llvm.experimental.noalias.scope.decl(metadata !237)
  %17 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %.val.i.i = load ptr, ptr %17, align 8, !alias.scope !237, !nonnull !3, !noundef !3
  %bits.i.i.i.i.i.i = ptrtoint ptr %.val.i.i to i64
  %_5.i.i.i.i.i.i = and i64 %bits.i.i.i.i.i.i, 3
  %switch.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i.i, label %bb2.i2.i.i.i.i.i, label %bb24.i, !prof !28

bb2.i2.i.i.i.i.i:                                 ; preds = %bb3.i13.i
  %18 = getelementptr i8, ptr %.val.i.i, i64 -1
  %19 = icmp ne ptr %18, null
  call void @llvm.assume(i1 %19)
  %_6.val.i.i.i.i.i.i.i = load ptr, ptr %18, align 8, !noalias !237
  %20 = getelementptr i8, ptr %.val.i.i, i64 7
  %_6.val1.i.i.i.i.i.i.i = load ptr, ptr %20, align 8, !noalias !237, !nonnull !3, !align !7, !noundef !3
  %21 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i, align 8, !invariant.load !3, !noalias !237
  %.not.i.i.i.i.i.i.i.i.i = icmp eq ptr %21, null
  br i1 %.not.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i2.i.i.i.i.i
  %22 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %22)
  invoke void %21(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i, !noalias !237

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i
  %23 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %23)
  %24 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %25 = load i64, ptr %24, align 8, !range !8, !invariant.load !3, !noalias !237
  %26 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %27 = load i64, ptr %26, align 8, !range !9, !invariant.load !3, !noalias !237
  %28 = add i64 %27, -1
  %29 = icmp sgt i64 %28, -1
  call void @llvm.assume(i1 %29)
  %30 = icmp eq i64 %25, 0
  br i1 %30, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %25, i64 noundef range(i64 1, -9223372036854775807) %27) #17, !noalias !237
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i:                        ; preds = %is_not_null.i.i.i.i.i.i.i.i.i
  %31 = landingpad { ptr, i32 }
          cleanup
  %32 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %33 = load i64, ptr %32, align 8, !range !8, !invariant.load !3, !noalias !237
  %34 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %35 = load i64, ptr %34, align 8, !range !9, !invariant.load !3, !noalias !237
  %36 = add i64 %35, -1
  %37 = icmp sgt i64 %36, -1
  call void @llvm.assume(i1 %37)
  %38 = icmp eq i64 %33, 0
  br i1 %38, label %bb1.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %33, i64 noundef range(i64 1, -9223372036854775807) %35) #17, !noalias !237
  br label %bb1.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %18, i64 noundef 24, i64 noundef 8) #17, !noalias !237
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %18, i64 noundef 24, i64 noundef 8) #17, !noalias !237
  br label %bb24.i

bb25.i:                                           ; preds = %bb4.i
  %_24.sroa.4.0._5.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %val.sroa.4.0.output.sroa_idx.i = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val.sroa.4.0.output.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_24.sroa.4.0._5.sroa_idx.i, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
  store i64 %15, ptr %output.i, align 8
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
          to label %bb6.i unwind label %cleanup1.i

bb24.i:                                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsb2sIUxn0KTS_18build_script_build.exit.i.i.i.i.i.i, %bb3.i13.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
; call core::ptr::drop_in_place::<std::process::Command>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_8.i)
  br label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread

cleanup1.i:                                       ; preds = %bb9.i, %bb28.i, %bb6.i, %bb25.i
  %39 = landingpad { ptr, i32 }
          cleanup
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i) #18
  br label %common.resume

bb6.i:                                            ; preds = %bb25.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_8.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_12.i)
  %_31.i = load ptr, ptr %val.sroa.4.0.output.sroa_idx.i, align 8, !nonnull !3, !noundef !3
  %40 = getelementptr inbounds nuw i8, ptr %output.i, i64 16
  %_30.i = load i64, ptr %40, align 8, !noundef !3
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_12.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_31.i, i64 noundef %_30.i)
          to label %bb7.i unwind label %cleanup1.i

bb7.i:                                            ; preds = %bb6.i
  %_32.i = load i64, ptr %_12.i, align 8, !range !221, !noundef !3
  %41 = trunc nuw i64 %_32.i to i1
  br i1 %41, label %bb26.i, label %bb28.i

bb26.i:                                           ; preds = %bb7.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12.i)
  br label %bb12.i

bb28.i:                                           ; preds = %bb7.i
  %42 = getelementptr inbounds nuw i8, ptr %_12.i, i64 8
  %_33.0.i = load ptr, ptr %42, align 8, !nonnull !3, !align !25, !noundef !3
  %43 = getelementptr inbounds nuw i8, ptr %_12.i, i64 16
  %_33.1.i = load i64, ptr %43, align 8, !noundef !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12.i)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %pieces.i)
  store i64 0, ptr %pieces.i, align 8
  %_35.sroa.4.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 8
  store i64 %_33.1.i, ptr %_35.sroa.4.0.pieces.sroa_idx.i, align 8
  %_35.sroa.5.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 16
  store ptr %_33.0.i, ptr %_35.sroa.5.0.pieces.sroa_idx.i, align 8
  %_35.sroa.5.sroa.4.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 24
  store i64 %_33.1.i, ptr %_35.sroa.5.sroa.4.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.5.sroa.5.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 32
  store i64 0, ptr %_35.sroa.5.sroa.5.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.5.sroa.6.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 40
  store i64 %_33.1.i, ptr %_35.sroa.5.sroa.6.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.5.sroa.7.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 48
  store <2 x i32> splat (i32 46), ptr %_35.sroa.5.sroa.7.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.5.sroa.9.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 56
  store i8 1, ptr %_35.sroa.5.sroa.9.0._35.sroa.5.0.pieces.sroa_idx.sroa_idx.i, align 8
  %_35.sroa.6.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 64
  store i8 1, ptr %_35.sroa.6.0.pieces.sroa_idx.i, align 8
  %_35.sroa.7.0.pieces.sroa_idx.i = getelementptr inbounds nuw i8, ptr %pieces.i, i64 65
  store i8 0, ptr %_35.sroa.7.0.pieces.sroa_idx.i, align 1
; invoke <core::str::iter::SplitInternal<char>>::next
  %44 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb29.i unwind label %cleanup1.i

bb29.i:                                           ; preds = %bb28.i
  %45 = extractvalue { ptr, i64 } %44, 0
  %.not7.i = icmp ne ptr %45, null
  %46 = extractvalue { ptr, i64 } %44, 1
  %_3.not.i.i = icmp eq i64 %46, 7
  %or.cond52.i = select i1 %.not7.i, i1 %_3.not.i.i, i1 false
  br i1 %or.cond52.i, label %bb32.i, label %bb8.i

bb8.i:                                            ; preds = %bb32.i, %bb29.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  br label %bb12.i

bb32.i:                                           ; preds = %bb29.i
  %47 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %45, ptr noundef nonnull dereferenceable(7) @alloc_ca36d7e792bb4bbd1a68749f90007ce8, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !240
  %48 = icmp eq i32 %47, 0
  br i1 %48, label %bb9.i, label %bb8.i

bb9.i:                                            ; preds = %bb32.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %49 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb33.i unwind label %cleanup1.i

bb33.i:                                           ; preds = %bb9.i
  %50 = extractvalue { ptr, i64 } %49, 0
  %.not8.i = icmp eq ptr %50, null
  br i1 %.not8.i, label %bb34.i, label %bb35.i

bb35.i:                                           ; preds = %bb33.i
  %51 = extractvalue { ptr, i64 } %49, 1
  switch i64 %51, label %bb9thread-pre-split.i.i [
    i64 0, label %bb36.i
    i64 1, label %bb7.i.i
  ]

bb7.i.i:                                          ; preds = %bb35.i
  %52 = load i8, ptr %50, align 1, !alias.scope !244, !noundef !3
  switch i8 %52, label %bb9.i.i [
    i8 43, label %bb36.i
    i8 45, label %bb36.i
  ]

bb9thread-pre-split.i.i:                          ; preds = %bb35.i
  %.pr.i.i = load i8, ptr %50, align 1, !alias.scope !244
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb9thread-pre-split.i.i, %bb7.i.i
  %53 = phi i8 [ %.pr.i.i, %bb9thread-pre-split.i.i ], [ %52, %bb7.i.i ]
  %cond.i.i = icmp eq i8 %53, 43
  %rest.1.i.i = sext i1 %cond.i.i to i64
  %src.sroa.15.0.i.i = add nsw i64 %51, %rest.1.i.i
  %src.sroa.0.0.idx.i.i = zext i1 %cond.i.i to i64
  %src.sroa.0.0.i.i = getelementptr inbounds nuw i8, ptr %50, i64 %src.sroa.0.0.idx.i.i
  %_10.i.i = icmp samesign ult i64 %src.sroa.15.0.i.i, 9
  br i1 %_10.i.i, label %bb15.preheader.i.i, label %bb22.i.i

bb15.preheader.i.i:                               ; preds = %bb9.i.i
  %_13.not56.i.i = icmp eq i64 %src.sroa.15.0.i.i, 0
  br i1 %_13.not56.i.i, label %bb36.i, label %bb16.i.i

bb22.i.i:                                         ; preds = %bb9.i.i, %bb33.i.i
  %result.sroa.0.0.i.i = phi i32 [ %_60.0.i.i, %bb33.i.i ], [ 0, %bb9.i.i ]
  %src.sroa.15.1.i.i = phi i64 [ %rest.12.i.i, %bb33.i.i ], [ %src.sroa.15.0.i.i, %bb9.i.i ]
  %src.sroa.0.1.i.i = phi ptr [ %rest.01.i.i, %bb33.i.i ], [ %src.sroa.0.0.i.i, %bb9.i.i ]
  %_28.not.i.not.i = icmp eq i64 %src.sroa.15.1.i.i, 0
  br i1 %_28.not.i.not.i, label %bb36.i, label %bb23.i.i

bb23.i.i:                                         ; preds = %bb22.i.i
  %54 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i.i, i32 10)
  %_57.1.i.i = extractvalue { i32, i1 } %54, 1
  br i1 %_57.1.i.i, label %bb36.i, label %bb33.i.i, !prof !247

bb33.i.i:                                         ; preds = %bb23.i.i
  %_57.0.i.i = extractvalue { i32, i1 } %54, 0
  %rest.12.i.i = add nsw i64 %src.sroa.15.1.i.i, -1
  %rest.01.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i, i64 1
  %55 = load i8, ptr %src.sroa.0.1.i.i, align 1, !alias.scope !244, !noundef !3
  %56 = zext i8 %55 to i32
  %57 = add nsw i32 %56, -48
  %_14.i.i.i = icmp ugt i32 %57, 9
  %_60.0.i.i = add i32 %57, %_57.0.i.i
  %_60.1.i.i = icmp ult i32 %_60.0.i.i, %_57.0.i.i
  %or.cond.i = select i1 %_14.i.i.i, i1 true, i1 %_60.1.i.i
  br i1 %or.cond.i, label %bb36.i, label %bb22.i.i, !prof !248

bb16.i.i:                                         ; preds = %bb15.preheader.i.i, %bb20.i.i
  %src.sroa.0.259.i.i = phi ptr [ %rest.04.i.i, %bb20.i.i ], [ %src.sroa.0.0.i.i, %bb15.preheader.i.i ]
  %src.sroa.15.258.i.i = phi i64 [ %rest.15.i.i, %bb20.i.i ], [ %src.sroa.15.0.i.i, %bb15.preheader.i.i ]
  %result.sroa.0.257.i.i = phi i32 [ %60, %bb20.i.i ], [ 0, %bb15.preheader.i.i ]
  %_19.i.i = load i8, ptr %src.sroa.0.259.i.i, align 1, !alias.scope !244, !noundef !3
  %_18.i.i = zext i8 %_19.i.i to i32
  %58 = add nsw i32 %_18.i.i, -48
  %_14.i47.i.i = icmp ult i32 %58, 10
  br i1 %_14.i47.i.i, label %bb20.i.i, label %bb36.i

bb20.i.i:                                         ; preds = %bb16.i.i
  %59 = mul i32 %result.sroa.0.257.i.i, 10
  %rest.15.i.i = add nsw i64 %src.sroa.15.258.i.i, -1
  %rest.04.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i, i64 1
  %60 = add i32 %58, %59
  %_13.not.i.i = icmp eq i64 %rest.15.i.i, 0
  br i1 %_13.not.i.i, label %bb36.i, label %bb16.i.i

bb34.i:                                           ; preds = %bb33.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !249)
  %61 = icmp eq i64 %15, 0
  br i1 %61, label %bb4.i.i, label %bb2.i.i.i4.i.i.i12

bb2.i.i.i4.i.i.i12:                               ; preds = %bb34.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !249
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i12, %bb34.i
  %62 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %62, align 8, !alias.scope !249
  %63 = icmp eq i64 %.val2.i.i, 0
  br i1 %63, label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i.i
  %64 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %64, align 8, !alias.scope !249, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !249
  br label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread

bb36.i:                                           ; preds = %bb33.i.i, %bb23.i.i, %bb22.i.i, %bb20.i.i, %bb16.i.i, %bb15.preheader.i.i, %bb7.i.i, %bb7.i.i, %bb35.i
  %not._0.sroa.0.0.i.i = phi i1 [ true, %bb15.preheader.i.i ], [ false, %bb7.i.i ], [ false, %bb7.i.i ], [ false, %bb35.i ], [ %_14.i47.i.i, %bb16.i.i ], [ %_14.i47.i.i, %bb20.i.i ], [ %_28.not.i.not.i, %bb22.i.i ], [ %_28.not.i.not.i, %bb23.i.i ], [ %_28.not.i.not.i, %bb33.i.i ]
  %65 = phi i32 [ 0, %bb15.preheader.i.i ], [ undef, %bb7.i.i ], [ undef, %bb7.i.i ], [ undef, %bb35.i ], [ %60, %bb20.i.i ], [ undef, %bb16.i.i ], [ undef, %bb33.i.i ], [ undef, %bb23.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !252)
  %66 = icmp eq i64 %15, 0
  br i1 %66, label %bb4.i18.i, label %bb2.i.i.i4.i.i16.i

bb2.i.i.i4.i.i16.i:                               ; preds = %bb36.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !252
  br label %bb4.i18.i

bb4.i18.i:                                        ; preds = %bb2.i.i.i4.i.i16.i, %bb36.i
  %67 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i19.i = load i64, ptr %67, align 8, !alias.scope !252
  %68 = icmp eq i64 %.val2.i19.i, 0
  br i1 %68, label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit, label %bb2.i.i.i4.i7.i20.i

bb2.i.i.i4.i7.i20.i:                              ; preds = %bb4.i18.i
  %69 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i21.i = load ptr, ptr %69, align 8, !alias.scope !252, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i21.i, i64 noundef %.val2.i19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !252
  br label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit

bb12.i:                                           ; preds = %bb8.i, %bb26.i
  call void @llvm.experimental.noalias.scope.decl(metadata !255)
  %70 = icmp eq i64 %15, 0
  br i1 %70, label %bb4.i26.i, label %bb2.i.i.i4.i.i24.i

bb2.i.i.i4.i.i24.i:                               ; preds = %bb12.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !255
  br label %bb4.i26.i

bb4.i26.i:                                        ; preds = %bb2.i.i.i4.i.i24.i, %bb12.i
  %71 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i27.i = load i64, ptr %71, align 8, !alias.scope !255
  %72 = icmp eq i64 %.val2.i27.i, 0
  br i1 %72, label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i28.i

bb2.i.i.i4.i7.i28.i:                              ; preds = %bb4.i26.i
  %73 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i29.i = load ptr, ptr %73, align 8, !alias.scope !255, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i29.i, i64 noundef %.val2.i27.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !255
  br label %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread

terminate.i:                                      ; preds = %cleanup.body.i
  %74 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread: ; preds = %bb2.i.i.i4.i7.i28.i, %bb4.i26.i, %bb24.i, %bb4.i.i, %bb2.i.i.i4.i7.i.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br label %bb57

_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit: ; preds = %bb4.i18.i, %bb2.i.i.i4.i7.i20.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br i1 %not._0.sroa.0.0.i.i, label %bb24, label %bb57

bb18.critedge:                                    ; preds = %bb10, %bb2.i.i.i4.i.i.i10
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a0f0604af2246bca35707903288f4f53, ptr noundef nonnull inttoptr (i64 61 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_e181ada66eb53f56ba6935f91cca5e48, ptr noundef nonnull inttoptr (i64 89 to ptr))
  br label %bb18

bb18:                                             ; preds = %bb17.thread58, %bb13.thread, %bb18.critedge, %bb13, %bb17.thread, %bb17
  %error_generic_member_access.sroa.0.2.off01725 = phi i1 [ false, %bb17.thread ], [ false, %bb17 ], [ false, %bb13 ], [ true, %bb18.critedge ], [ false, %bb13.thread ], [ false, %bb17.thread58 ]
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c4fe0d46c3935d35a63bc8de9de91c71, ptr noundef nonnull inttoptr (i64 87 to ptr))
  br label %bb20

bb24:                                             ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit
  %_21 = icmp ugt i32 %65, 79
  br i1 %_21, label %bb43.thread33, label %bb36

bb57:                                             ; preds = %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit.thread, %_RNvCsb2sIUxn0KTS_18build_script_build19rustc_minor_version.exit, %bb53, %bb54
  ret void

bb36:                                             ; preds = %bb24
  %_42 = icmp samesign ult i32 %65, 51
  br i1 %_42, label %bb39.thread, label %bb39

bb43.thread33:                                    ; preds = %bb24
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_7d35a291c18dd0521a5e644c84ccaf0c, ptr noundef nonnull inttoptr (i64 93 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_72df481b03b7eb46c2809fd337b26da7, ptr noundef nonnull inttoptr (i64 101 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_5dfc04774f16d076cbbf5aa4aa0b112a, ptr noundef nonnull inttoptr (i64 113 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_be9f510b6961f26854e9160336556929, ptr noundef nonnull inttoptr (i64 97 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a5780797e07fb9d8b2064b159c0780ee, ptr noundef nonnull inttoptr (i64 109 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_fcc25653342892fcd6eef156552226a0, ptr noundef nonnull inttoptr (i64 117 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_fed985032aec30b656cbc8da62e5d8b4, ptr noundef nonnull inttoptr (i64 99 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_1ac038b884c8e490cbaf26d8798c71a3, ptr noundef nonnull inttoptr (i64 131 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_3e2d38849520ab73d6bdac75533cb117, ptr noundef nonnull inttoptr (i64 111 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_311235f6f62f0542f667345a272b98e8, ptr noundef nonnull inttoptr (i64 83 to ptr))
  br label %bb46

bb39:                                             ; preds = %bb36
  %_45 = icmp eq i32 %65, 51
  br i1 %_45, label %bb43.thread, label %bb43

bb39.thread:                                      ; preds = %bb36
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_5699aa2caa76aa64718f60b150ec0ee6, ptr noundef nonnull inttoptr (i64 77 to ptr))
  br label %bb43.thread

bb43:                                             ; preds = %bb39
  %_50 = icmp samesign ult i32 %65, 56
  br i1 %_50, label %bb50.thread, label %bb46

bb43.thread:                                      ; preds = %bb39, %bb39.thread
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_7351d367884d97b1604bb68c0a75d428, ptr noundef nonnull inttoptr (i64 95 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_286391b658cb8dcdee605b2b79832083, ptr noundef nonnull inttoptr (i64 109 to ptr))
  br label %bb50.thread

bb46:                                             ; preds = %bb43.thread33, %bb43
  %_53 = icmp ult i32 %65, 65
  %or.cond.not = or i1 %error_generic_member_access.sroa.0.2.off01724, %_53
  br i1 %or.cond.not, label %bb50, label %bb48

bb50.thread:                                      ; preds = %bb43.thread, %bb43
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_27cc1f87a94a5f6e40c716f850700584, ptr noundef nonnull inttoptr (i64 87 to ptr))
  br label %bb53.thread

bb50:                                             ; preds = %bb48, %bb46
  %_56 = icmp ult i32 %65, 81
  br i1 %_56, label %bb53.thread, label %bb53

bb48:                                             ; preds = %bb46
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_a0f0604af2246bca35707903288f4f53, ptr noundef nonnull inttoptr (i64 61 to ptr))
  br label %bb50

bb53:                                             ; preds = %bb50
  %_59 = icmp ult i32 %65, 85
  br i1 %_59, label %bb54, label %bb57

bb53.thread:                                      ; preds = %bb50, %bb50.thread
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_6df1e55893960c0368c61ff3e9d5b943, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br label %bb54

bb54:                                             ; preds = %bb53.thread, %bb53
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_afd5ca28658b6cdb1db5eabbd0000b2d, ptr noundef nonnull inttoptr (i64 91 to ptr))
  br label %bb57
}

; <std::io::error::Error>::kind
; Function Attrs: inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read, inaccessiblemem: write) uwtable
define internal fastcc noundef range(i8 0, 42) i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr %self.0.val) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %self.0.val, null
  tail call void @llvm.assume(i1 %0)
  %bits.i = ptrtoint ptr %self.0.val to i64
  %_5.i = and i64 %bits.i, 3
  switch i64 %_5.i, label %default.unreachable [
    i64 2, label %bb5
    i64 3, label %bb4.i
    i64 0, label %bb2
    i64 1, label %bb4
  ], !prof !258

default.unreachable:                              ; preds = %start
  unreachable

bb4.i:                                            ; preds = %start
  %_10.i = lshr i64 %bits.i, 32
  %switch.idx.cast = trunc i64 %_10.i to i8
  br label %bb6

bb5:                                              ; preds = %start
  %_7.i = lshr i64 %bits.i, 32
  %code.i = trunc nuw i64 %_7.i to i32
  switch i32 %code.i, label %bb43.i [
    i32 7, label %bb6
    i32 48, label %bb37.i
    i32 49, label %bb36.i
    i32 16, label %bb35.i
    i32 53, label %bb34.i
    i32 61, label %bb33.i
    i32 54, label %bb32.i
    i32 11, label %bb31.i
    i32 69, label %bb30.i
    i32 17, label %bb29.i
    i32 27, label %bb28.i
    i32 65, label %bb27.i
    i32 4, label %bb26.i
    i32 22, label %bb25.i
    i32 21, label %bb24.i
    i32 62, label %bb23.i
    i32 2, label %bb22.i
    i32 12, label %bb21.i
    i32 28, label %bb20.i
    i32 78, label %bb19.i
    i32 31, label %bb18.i
    i32 63, label %bb17.i
    i32 50, label %bb16.i
    i32 51, label %bb15.i
    i32 57, label %bb14.i
    i32 20, label %bb13.i
    i32 66, label %bb12.i
    i32 32, label %bb11.i
    i32 30, label %bb10.i
    i32 29, label %bb9.i
    i32 70, label %bb8.i
    i32 60, label %bb7.i
    i32 26, label %bb6.i
    i32 18, label %bb5.i3
    i32 36, label %bb4.i2
    i32 102, label %bb19.i
    i32 13, label %bb2.i1
    i32 1, label %bb2.i1
    i32 35, label %bb42.i
  ]

bb37.i:                                           ; preds = %bb5
  br label %bb6

bb36.i:                                           ; preds = %bb5
  br label %bb6

bb35.i:                                           ; preds = %bb5
  br label %bb6

bb34.i:                                           ; preds = %bb5
  br label %bb6

bb33.i:                                           ; preds = %bb5
  br label %bb6

bb32.i:                                           ; preds = %bb5
  br label %bb6

bb31.i:                                           ; preds = %bb5
  br label %bb6

bb30.i:                                           ; preds = %bb5
  br label %bb6

bb29.i:                                           ; preds = %bb5
  br label %bb6

bb28.i:                                           ; preds = %bb5
  br label %bb6

bb27.i:                                           ; preds = %bb5
  br label %bb6

bb26.i:                                           ; preds = %bb5
  br label %bb6

bb25.i:                                           ; preds = %bb5
  br label %bb6

bb24.i:                                           ; preds = %bb5
  br label %bb6

bb23.i:                                           ; preds = %bb5
  br label %bb6

bb22.i:                                           ; preds = %bb5
  br label %bb6

bb21.i:                                           ; preds = %bb5
  br label %bb6

bb20.i:                                           ; preds = %bb5
  br label %bb6

bb19.i:                                           ; preds = %bb5, %bb5
  br label %bb6

bb18.i:                                           ; preds = %bb5
  br label %bb6

bb17.i:                                           ; preds = %bb5
  br label %bb6

bb16.i:                                           ; preds = %bb5
  br label %bb6

bb15.i:                                           ; preds = %bb5
  br label %bb6

bb14.i:                                           ; preds = %bb5
  br label %bb6

bb13.i:                                           ; preds = %bb5
  br label %bb6

bb12.i:                                           ; preds = %bb5
  br label %bb6

bb11.i:                                           ; preds = %bb5
  br label %bb6

bb10.i:                                           ; preds = %bb5
  br label %bb6

bb9.i:                                            ; preds = %bb5
  br label %bb6

bb8.i:                                            ; preds = %bb5
  br label %bb6

bb7.i:                                            ; preds = %bb5
  br label %bb6

bb6.i:                                            ; preds = %bb5
  br label %bb6

bb5.i3:                                           ; preds = %bb5
  br label %bb6

bb4.i2:                                           ; preds = %bb5
  br label %bb6

bb2.i1:                                           ; preds = %bb5, %bb5
  br label %bb6

bb43.i:                                           ; preds = %bb5
  br label %bb6

bb42.i:                                           ; preds = %bb5
  br label %bb6

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 16
  %2 = load i8, ptr %1, align 8, !range !259, !noundef !3
  br label %bb6

bb4:                                              ; preds = %start
  %3 = getelementptr i8, ptr %self.0.val, i64 -1
  %4 = icmp ne ptr %3, null
  tail call void @llvm.assume(i1 %4)
  %5 = getelementptr i8, ptr %self.0.val, i64 15
  %6 = load i8, ptr %5, align 8, !range !259, !noundef !3
  br label %bb6

bb6:                                              ; preds = %bb4.i, %bb42.i, %bb43.i, %bb2.i1, %bb4.i2, %bb5.i3, %bb6.i, %bb7.i, %bb8.i, %bb9.i, %bb10.i, %bb11.i, %bb12.i, %bb13.i, %bb14.i, %bb15.i, %bb16.i, %bb17.i, %bb18.i, %bb19.i, %bb20.i, %bb21.i, %bb22.i, %bb23.i, %bb24.i, %bb25.i, %bb26.i, %bb27.i, %bb28.i, %bb29.i, %bb30.i, %bb31.i, %bb32.i, %bb33.i, %bb34.i, %bb35.i, %bb36.i, %bb37.i, %bb5, %bb4, %bb2
  %kind.sroa.0.0 = phi i8 [ %2, %bb2 ], [ %6, %bb4 ], [ 41, %bb43.i ], [ 8, %bb37.i ], [ 9, %bb36.i ], [ 28, %bb35.i ], [ 6, %bb34.i ], [ 2, %bb33.i ], [ 3, %bb32.i ], [ 30, %bb31.i ], [ 26, %bb30.i ], [ 12, %bb29.i ], [ 27, %bb28.i ], [ 4, %bb27.i ], [ 35, %bb26.i ], [ 20, %bb25.i ], [ 15, %bb24.i ], [ 18, %bb23.i ], [ 0, %bb22.i ], [ 38, %bb21.i ], [ 24, %bb20.i ], [ 36, %bb19.i ], [ 32, %bb18.i ], [ 33, %bb17.i ], [ 10, %bb16.i ], [ 5, %bb15.i ], [ 7, %bb14.i ], [ 14, %bb13.i ], [ 16, %bb12.i ], [ 11, %bb11.i ], [ 17, %bb10.i ], [ 25, %bb9.i ], [ 19, %bb8.i ], [ 22, %bb7.i ], [ 29, %bb6.i ], [ 31, %bb5.i3 ], [ 39, %bb4.i2 ], [ 1, %bb2.i1 ], [ 13, %bb42.i ], [ 34, %bb5 ], [ %switch.idx.cast, %bb4.i ]
  ret i8 %kind.sroa.0.0
}

; <core::str::iter::SplitInternal<char>>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !260, !noundef !3
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !3, !align !25, !noundef !3
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1 = load i64, ptr %2, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !261)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i = load i64, ptr %4, align 8, !alias.scope !261, !noalias !264, !noundef !3
  %_38.not.i = icmp ugt i64 %index2.i, %_4.val1
  %.promoted.i = load i64, ptr %3, align 8, !alias.scope !261, !noalias !264
  %_4325.i = icmp ult i64 %index2.i, %.promoted.i
  %or.cond26.i = or i1 %_38.not.i, %_4325.i
  br i1 %or.cond26.i, label %bb1.i, label %bb12.lr.ph.i

bb12.lr.ph.i:                                     ; preds = %bb2
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_48.i = load i8, ptr %5, align 8, !alias.scope !261, !noalias !264, !noundef !3
  %_12.i = zext i8 %_48.i to i64
  %6 = getelementptr i8, ptr %_10.i, i64 %_12.i
  %_49.i = getelementptr i8, ptr %6, i64 -1
  %_65.i = icmp ult i8 %_48.i, 5
  %last_byte.us.pre.i = load i8, ptr %_49.i, align 1, !alias.scope !261, !noalias !264
  br i1 %_65.i, label %bb12.us.i, label %bb12.i, !prof !266

bb12.us.i:                                        ; preds = %bb12.lr.ph.i, %bb9.us.i
  %7 = phi i64 [ %16, %bb9.us.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.us.i = sub nuw i64 %index2.i, %7
  %_46.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %7
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i), !noalias !267
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %10, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %9, align 1, !alias.scope !268, !noalias !267, !noundef !3
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
  store i64 %16, ptr %3, align 8, !alias.scope !261, !noalias !264
  %_17.not.us.i = icmp ult i64 %16, %_12.i
  %_54.not.us.i = icmp ugt i64 %16, %_4.val1
  %or.cond.i = or i1 %_17.not.us.i, %_54.not.us.i
  br i1 %or.cond.i, label %bb9.us.i, label %bb19.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %found_char.us.i = sub nuw i64 %16, %_12.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %found_char.us.i
  %17 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us.i, ptr nonnull readonly align 1 %_10.i, i64 range(i64 0, -9223372036854775808) %_12.i), !alias.scope !271, !noalias !264
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
  %20 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i), !noalias !267
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
  %_9.i.i = load i8, ptr %23, align 1, !alias.scope !268, !noalias !267, !noundef !3
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
  store i64 %28, ptr %3, align 8, !alias.scope !261, !noalias !264
  %_17.not.i = icmp ult i64 %28, %_12.i
  %_54.not.i = icmp ugt i64 %28, %_4.val1
  %or.cond70.i = or i1 %_17.not.i, %_54.not.i
  br i1 %or.cond70.i, label %bb9.i, label %bb25.i

bb10.i:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  store i64 %index2.i, ptr %3, align 8, !alias.scope !261, !noalias !264
  br label %bb1.i

bb9.i:                                            ; preds = %bb4.i
  %_43.i = icmp ult i64 %index2.i, %28
  br i1 %_43.i, label %bb1.i, label %bb12.i

bb25.i:                                           ; preds = %bb4.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #22, !noalias !267
  unreachable

bb7:                                              ; preds = %bb19.us.i
  %i = load i64, ptr %self, align 8, !noundef !3
  %new_len = sub nuw i64 %found_char.us.i, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %16, ptr %self, align 8
  br label %bb9

bb1.i:                                            ; preds = %bb9.i, %bb9.us.i, %bb2, %bb10.i
  store i8 1, ptr %0, align 1, !alias.scope !275
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %30 = load i8, ptr %29, align 8, !range !260, !alias.scope !275, !noundef !3
  %_3.i = trunc nuw i8 %30 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !275
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !275
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
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCsb2sIUxn0KTS_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !3
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !278)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !281)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !284, !noalias !285
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !284, !noalias !285
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !284, !noalias !285
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !284, !noalias !285
  store i64 0, ptr %self, align 8, !alias.scope !284, !noalias !285
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit

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
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !287, !nonnull !3, !noundef !3
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !288, !noundef !3
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #17, !noalias !293
  %11 = load ptr, ptr %10, align 8, !noalias !288, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #17, !noalias !293
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !294)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !297)
  %_3.i.i = load i64, ptr %self, align 8, !range !221, !alias.scope !300, !noalias !301, !noundef !3
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !300, !noalias !301, !noundef !3
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !303, !noalias !306
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !303, !noalias !306
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !300, !noalias !301, !noundef !3
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !300, !noalias !301, !nonnull !3, !noundef !3
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !300, !noalias !301
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !308, !nonnull !3, !noundef !3
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !309)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !310, !noundef !3
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !317, !noundef !3
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
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !322, !nonnull !3, !noundef !3
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !317
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #17, !noalias !326
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !310, !noundef !3
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #17, !noalias !326
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #22
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !327

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #21, !noalias !328
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !303, !noalias !306
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !303, !noalias !306
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !303, !noalias !306
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build.exit
  ret void
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCsb2sIUxn0KTS_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !3, !align !25, !noundef !3
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !3
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #5

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #5

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #7

; <std::path::Path>::_join
; Function Attrs: uwtable
declare void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::fs::DirBuilder>::_create
; Function Attrs: uwtable
declare noundef ptr @_RNvMsF_NtCs5sEH5CPMdak_3std2fsNtB5_10DirBuilder7__create(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #9

; <std::sys::process::env::CommandEnv>::remove
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtCs5sEH5CPMdak_3std3sys7process3envNtB4_10CommandEnv6remove(ptr noalias noundef align 8 dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::arg
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef align 8 dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::new
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr dead_on_unwind noalias noundef writable sret([200 x i8]) align 8 captures(none) dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::stderr
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command6stderr(ptr noalias noundef align 8 dereferenceable(200), i32 noundef range(i32 0, 5), i32) unnamed_addr #0

; std::sys::fs::remove_dir_all
; Function Attrs: uwtable
declare noundef ptr @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs14remove_dir_all(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

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

; std::io::stdio::_eprint
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; std::process::exit
; Function Attrs: noreturn uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef) unnamed_addr #10

; <std::path::Display as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <std::io::error::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <std::process::Command>::status
; Function Attrs: uwtable
declare void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6status(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(200)) unnamed_addr #0

; <std::process::Command>::output
; Function Attrs: uwtable
declare void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr dead_on_unwind noalias noundef writable sret([56 x i8]) align 8 captures(none) dereferenceable(56), ptr noalias noundef align 8 dereferenceable(200)) unnamed_addr #0

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #11

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #12

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #13

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #14

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #8

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #15 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCsb2sIUxn0KTS_18build_script_build4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #16

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #10 = { noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #12 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #13 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #14 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #17 = { nounwind }
attributes #18 = { cold }
attributes #19 = { cold noreturn nounwind }
attributes #20 = { noinline }
attributes #21 = { noreturn }
attributes #22 = { noinline noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECsb2sIUxn0KTS_18build_script_build: %_1.0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECsb2sIUxn0KTS_18build_script_build"}
!7 = !{i64 8}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!12 = distinct !{!12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build"}
!13 = !{i64 0, i64 -9223372036854775805}
!14 = !{!15}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build"}
!17 = !{!15, !11}
!18 = !{i64 0, i64 -9223372036854775806}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECsb2sIUxn0KTS_18build_script_build: %_1"}
!21 = distinct !{!21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECsb2sIUxn0KTS_18build_script_build"}
!22 = !{!23, !20}
!23 = distinct !{!23, !24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECsb2sIUxn0KTS_18build_script_build: %_1"}
!24 = distinct !{!24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECsb2sIUxn0KTS_18build_script_build"}
!25 = !{i64 1}
!26 = !{i64 4}
!27 = !{i32 0, i32 6}
!28 = !{!"branch_weights", i32 2000, i32 6001}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!31 = distinct !{!31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build"}
!32 = !{!33}
!33 = distinct !{!33, !34, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build: %self"}
!34 = distinct !{!34, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build"}
!35 = !{!33, !30}
!36 = !{!37, !39, !33, !30}
!37 = distinct !{!37, !38, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build: %self"}
!38 = distinct !{!38, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCsb2sIUxn0KTS_18build_script_build"}
!39 = distinct !{!39, !40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!40 = distinct !{!40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECsb2sIUxn0KTS_18build_script_build"}
!41 = !{i64 0, i64 -9223372036854775807}
!42 = !{i64 15093661837926264}
!43 = !{!44}
!44 = distinct !{!44, !45, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Csb2sIUxn0KTS_18build_script_build: %_1"}
!45 = distinct !{!45, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Csb2sIUxn0KTS_18build_script_build"}
!46 = !{!47, !49}
!47 = distinct !{!47, !48, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: %_0"}
!48 = distinct !{!48, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var"}
!49 = distinct !{!49, !48, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: argument 1"}
!50 = !{!47}
!51 = !{!49}
!52 = !{!53, !55}
!53 = distinct !{!53, !54, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: %_0"}
!54 = distinct !{!54, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var"}
!55 = distinct !{!55, !54, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: argument 1"}
!56 = !{!55}
!57 = !{!58}
!58 = distinct !{!58, !59, !"_RINvNtCs5sEH5CPMdak_3std2fs10create_dirRNtNtB4_4path7PathBufECsb2sIUxn0KTS_18build_script_build: argument 0"}
!59 = distinct !{!59, !"_RINvNtCs5sEH5CPMdak_3std2fs10create_dirRNtNtB4_4path7PathBufECsb2sIUxn0KTS_18build_script_build"}
!60 = !{!61}
!61 = distinct !{!61, !62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probe0EB1C_: %self"}
!62 = distinct !{!62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probe0EB1C_"}
!63 = !{!64}
!64 = distinct !{!64, !62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probe0EB1C_: %_0"}
!65 = !{!64, !61}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probes_0EB1C_: %self"}
!68 = distinct !{!68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probes_0EB1C_"}
!69 = !{!70}
!70 = distinct !{!70, !68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCsb2sIUxn0KTS_18build_script_build13compile_probes_0EB1C_: %_0"}
!71 = !{!70, !67}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECsb2sIUxn0KTS_18build_script_build: %self"}
!74 = distinct !{!74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECsb2sIUxn0KTS_18build_script_build"}
!75 = !{!76}
!76 = distinct !{!76, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECsb2sIUxn0KTS_18build_script_build: %other"}
!77 = !{!78, !73}
!78 = distinct !{!78, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECsb2sIUxn0KTS_18build_script_build: %_0"}
!79 = !{!78, !76}
!80 = !{!81}
!81 = distinct !{!81, !82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build: %_0"}
!82 = distinct !{!82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build"}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %_0"}
!85 = distinct !{!85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build"}
!86 = !{!87, !89, !90, !91}
!87 = distinct !{!87, !88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %opt"}
!88 = distinct !{!88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build"}
!89 = distinct !{!89, !85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %self"}
!90 = distinct !{!90, !82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build: argument 1"}
!91 = distinct !{!91, !92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %opt"}
!92 = distinct !{!92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build"}
!93 = !{!94, !84, !81, !95}
!94 = distinct !{!94, !88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %_0"}
!95 = distinct !{!95, !92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %_0"}
!96 = !{!97, !99, !100, !102, !84, !89, !81, !90}
!97 = distinct !{!97, !98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_0"}
!98 = distinct !{!98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build"}
!99 = distinct !{!99, !98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_1"}
!100 = distinct !{!100, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %x"}
!101 = distinct !{!101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build"}
!102 = distinct !{!102, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %f"}
!103 = !{!104, !95}
!104 = distinct !{!104, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %self"}
!105 = !{!91}
!106 = !{!95}
!107 = !{!108, !110, !111, !113}
!108 = distinct !{!108, !109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_0"}
!109 = distinct !{!109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build"}
!110 = distinct !{!110, !109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_1"}
!111 = distinct !{!111, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %x"}
!112 = distinct !{!112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build"}
!113 = distinct !{!113, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %f"}
!114 = !{!115}
!115 = distinct !{!115, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %self"}
!116 = !{!117, !119}
!117 = distinct !{!117, !118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build: %_0"}
!118 = distinct !{!118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build"}
!119 = distinct !{!119, !118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build: %program"}
!120 = !{!119}
!121 = !{!122, !124}
!122 = distinct !{!122, !123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECsb2sIUxn0KTS_18build_script_build: %self"}
!123 = distinct !{!123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECsb2sIUxn0KTS_18build_script_build"}
!124 = distinct !{!124, !123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECsb2sIUxn0KTS_18build_script_build: %args"}
!125 = !{!126, !128}
!126 = distinct !{!126, !127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %opt"}
!127 = distinct !{!127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build"}
!128 = distinct !{!128, !129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %self"}
!129 = distinct !{!129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build"}
!130 = !{!131, !132, !122, !124}
!131 = distinct !{!131, !127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %_0"}
!132 = distinct !{!132, !129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %_0"}
!133 = !{!132}
!134 = !{!128}
!135 = !{!126}
!136 = !{!137}
!137 = distinct !{!137, !138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build: %_0"}
!138 = distinct !{!138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build"}
!139 = !{!140}
!140 = distinct !{!140, !141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %_0"}
!141 = distinct !{!141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build"}
!142 = !{!143, !145, !146, !126, !128}
!143 = distinct !{!143, !144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %opt"}
!144 = distinct !{!144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build"}
!145 = distinct !{!145, !141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %self"}
!146 = distinct !{!146, !138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCsb2sIUxn0KTS_18build_script_build: argument 1"}
!147 = !{!148, !140, !137, !131, !132, !122, !124}
!148 = distinct !{!148, !144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECsb2sIUxn0KTS_18build_script_build: %_0"}
!149 = !{!150}
!150 = distinct !{!150, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %x"}
!151 = distinct !{!151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build"}
!152 = !{!153}
!153 = distinct !{!153, !154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_0"}
!154 = distinct !{!154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build"}
!155 = !{!156, !157, !145, !146, !126, !128}
!156 = distinct !{!156, !154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_1"}
!157 = distinct !{!157, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %f"}
!158 = !{!153, !150, !159, !140, !137, !131, !132, !122, !124}
!159 = distinct !{!159, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %self"}
!160 = !{!161, !163, !153, !156, !150, !157, !140, !145, !137, !146}
!161 = distinct !{!161, !162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %_0"}
!162 = distinct !{!162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build"}
!163 = distinct !{!163, !162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %self"}
!164 = !{!159, !131, !126, !132, !128, !122, !124}
!165 = !{!153, !156, !150, !157, !140, !145, !137, !146}
!166 = !{!159, !131, !132, !122, !124}
!167 = !{!131, !126, !132, !128, !122, !124}
!168 = !{!169}
!169 = distinct !{!169, !170, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!170 = distinct !{!170, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build"}
!171 = !{!172}
!172 = distinct !{!172, !173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build: %_1"}
!173 = distinct !{!173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build"}
!174 = !{!172, !169, !126, !128}
!175 = !{!172, !169, !131, !126, !132, !128, !124}
!176 = !{!177}
!177 = distinct !{!177, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %x"}
!178 = distinct !{!178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build"}
!179 = !{!180}
!180 = distinct !{!180, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %self"}
!181 = !{!182}
!182 = distinct !{!182, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECsb2sIUxn0KTS_18build_script_build: %f"}
!183 = !{!177, !180, !132}
!184 = !{!182, !128, !122, !124}
!185 = !{!186}
!186 = distinct !{!186, !187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_0"}
!187 = distinct !{!187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build"}
!188 = !{!189, !182, !128}
!189 = distinct !{!189, !187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Csb2sIUxn0KTS_18build_script_build: %_1"}
!190 = !{!186, !177, !180, !132, !122, !124}
!191 = !{!192, !194, !186, !189, !177, !182, !132, !128}
!192 = distinct !{!192, !193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %_0"}
!193 = distinct !{!193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build"}
!194 = distinct !{!194, !193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCsb2sIUxn0KTS_18build_script_build: %self"}
!195 = !{!180, !122, !124}
!196 = !{!124}
!197 = !{!186, !189, !177, !182, !132, !128}
!198 = !{!199}
!199 = distinct !{!199, !200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!200 = distinct !{!200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECsb2sIUxn0KTS_18build_script_build"}
!201 = !{!202}
!202 = distinct !{!202, !203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build: %_1"}
!203 = distinct !{!203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECsb2sIUxn0KTS_18build_script_build"}
!204 = !{!205}
!205 = distinct !{!205, !206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build: %_1"}
!206 = distinct !{!206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECsb2sIUxn0KTS_18build_script_build"}
!207 = !{!205, !202, !199, !124}
!208 = !{!205, !202, !199}
!209 = !{!199, !124}
!210 = !{!211}
!211 = distinct !{!211, !212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECsb2sIUxn0KTS_18build_script_build: argument 1"}
!212 = distinct !{!212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECsb2sIUxn0KTS_18build_script_build"}
!213 = !{!214}
!214 = distinct !{!214, !212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECsb2sIUxn0KTS_18build_script_build: %self"}
!215 = !{!216}
!216 = distinct !{!216, !217, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtB8_4path7PathBufECsb2sIUxn0KTS_18build_script_build: %arg"}
!217 = distinct !{!217, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtB8_4path7PathBufECsb2sIUxn0KTS_18build_script_build"}
!218 = !{!219}
!219 = distinct !{!219, !220, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build: %arg"}
!220 = distinct !{!220, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build"}
!221 = !{i64 0, i64 2}
!222 = !{!223}
!223 = distinct !{!223, !224, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsb2sIUxn0KTS_18build_script_build: %_1"}
!224 = distinct !{!224, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECsb2sIUxn0KTS_18build_script_build"}
!225 = !{i32 0, i32 2}
!226 = !{!227, !229}
!227 = distinct !{!227, !228, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: %_0"}
!228 = distinct !{!228, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var"}
!229 = distinct !{!229, !228, !"_RNvCsb2sIUxn0KTS_18build_script_build13cargo_env_var: argument 1"}
!230 = !{!227}
!231 = !{!229}
!232 = !{!233, !235}
!233 = distinct !{!233, !234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build: %_0"}
!234 = distinct !{!234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build"}
!235 = distinct !{!235, !234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECsb2sIUxn0KTS_18build_script_build: %program"}
!236 = !{!235}
!237 = !{!238}
!238 = distinct !{!238, !239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECsb2sIUxn0KTS_18build_script_build: %_1"}
!239 = distinct !{!239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECsb2sIUxn0KTS_18build_script_build"}
!240 = !{!241, !243}
!241 = distinct !{!241, !242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build: %self.0"}
!242 = distinct !{!242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build"}
!243 = distinct !{!243, !242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build: %other.0"}
!244 = !{!245}
!245 = distinct !{!245, !246, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!246 = distinct !{!246, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!247 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!248 = !{!"branch_weights", i32 2002, i32 2000}
!249 = !{!250}
!250 = distinct !{!250, !251, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build: %_1"}
!251 = distinct !{!251, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build"}
!252 = !{!253}
!253 = distinct !{!253, !254, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build: %_1"}
!254 = distinct !{!254, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build"}
!255 = !{!256}
!256 = distinct !{!256, !257, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build: %_1"}
!257 = distinct !{!257, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECsb2sIUxn0KTS_18build_script_build"}
!258 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000, i32 2000}
!259 = !{i8 0, i8 42}
!260 = !{i8 0, i8 2}
!261 = !{!262}
!262 = distinct !{!262, !263, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!263 = distinct !{!263, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!264 = !{!265}
!265 = distinct !{!265, !263, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!266 = !{!"branch_weights", i32 4000000, i32 4001}
!267 = !{!265, !262}
!268 = !{!269}
!269 = distinct !{!269, !270, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!270 = distinct !{!270, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!271 = !{!272, !274}
!272 = distinct !{!272, !273, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build: %self.0"}
!273 = distinct !{!273, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build"}
!274 = distinct !{!274, !273, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsb2sIUxn0KTS_18build_script_build: %other.0"}
!275 = !{!276}
!276 = distinct !{!276, !277, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCsb2sIUxn0KTS_18build_script_build: %self"}
!277 = distinct !{!277, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCsb2sIUxn0KTS_18build_script_build"}
!278 = !{!279}
!279 = distinct !{!279, !280, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %self"}
!280 = distinct !{!280, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build"}
!281 = !{!282}
!282 = distinct !{!282, !283, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCsb2sIUxn0KTS_18build_script_build: %self"}
!283 = distinct !{!283, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCsb2sIUxn0KTS_18build_script_build"}
!284 = !{!282, !279}
!285 = !{!286}
!286 = distinct !{!286, !283, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCsb2sIUxn0KTS_18build_script_build: %_0"}
!287 = !{!286, !282, !279}
!288 = !{!289, !291, !279}
!289 = distinct !{!289, !290, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCsb2sIUxn0KTS_18build_script_build: %_0"}
!290 = distinct !{!290, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCsb2sIUxn0KTS_18build_script_build"}
!291 = distinct !{!291, !292, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %ret"}
!292 = distinct !{!292, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build"}
!293 = !{!291, !279}
!294 = !{!295}
!295 = distinct !{!295, !296, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %self"}
!296 = distinct !{!296, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build"}
!297 = !{!298}
!298 = distinct !{!298, !299, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCsb2sIUxn0KTS_18build_script_build: %self"}
!299 = distinct !{!299, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCsb2sIUxn0KTS_18build_script_build"}
!300 = !{!298, !295}
!301 = !{!302}
!302 = distinct !{!302, !296, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %_0"}
!303 = !{!304, !295}
!304 = distinct !{!304, !305, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECsb2sIUxn0KTS_18build_script_build: %v"}
!305 = distinct !{!305, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECsb2sIUxn0KTS_18build_script_build"}
!306 = !{!307, !302}
!307 = distinct !{!307, !305, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECsb2sIUxn0KTS_18build_script_build: %ret"}
!308 = !{!298, !302, !295}
!309 = !{!304}
!310 = !{!311, !313, !314, !316, !307, !304, !302, !295}
!311 = distinct !{!311, !312, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %_0"}
!312 = distinct !{!312, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build"}
!313 = distinct !{!313, !312, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %self"}
!314 = distinct !{!314, !315, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Csb2sIUxn0KTS_18build_script_build: %val"}
!315 = distinct !{!315, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Csb2sIUxn0KTS_18build_script_build"}
!316 = distinct !{!316, !315, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Csb2sIUxn0KTS_18build_script_build: %leaf_edge"}
!317 = !{!318, !320, !311, !313, !314, !316, !307, !304, !302, !295}
!318 = distinct !{!318, !319, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCsb2sIUxn0KTS_18build_script_build: %_0"}
!319 = distinct !{!319, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCsb2sIUxn0KTS_18build_script_build"}
!320 = distinct !{!320, !321, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build: %ret"}
!321 = distinct !{!321, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECsb2sIUxn0KTS_18build_script_build"}
!322 = !{!323, !325, !311, !313, !314, !316, !307, !304, !302, !295}
!323 = distinct !{!323, !324, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCsb2sIUxn0KTS_18build_script_build: %_0"}
!324 = distinct !{!324, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCsb2sIUxn0KTS_18build_script_build"}
!325 = distinct !{!325, !324, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCsb2sIUxn0KTS_18build_script_build: %self"}
!326 = !{!320, !311, !313, !314, !316, !307, !304, !302, !295}
!327 = !{!307, !304, !302, !295}
!328 = !{!302, !295}
