; ModuleID = 'build_script_build.585db57ecf29d0bf-cgu.0'
source_filename = "build_script_build.585db57ecf29d0bf-cgu.0"
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
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs7AmXS38G9s1_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs7AmXS38G9s1_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs7AmXS38G9s1_18build_script_build }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_193ab55f01318f0887536940a400dd6a = private unnamed_addr constant [71 x i8] c"\16Environment variable $\C0- is not set during execution of build script\0A\00", align 1
@alloc_71c72c6df710458e1c89ad45a5b4690d = private unnamed_addr constant [41 x i8] c"!cargo:rerun-if-changed=src/probe/\C0\04.rs\0A\00", align 1
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_ebcdb5f66b6f511cde89ece546cbdd6d = private unnamed_addr constant [7 x i8] c"OUT_DIR", align 1
@alloc_6b9c4d547a268aa1e418a0468c721e03 = private unnamed_addr constant [5 x i8] c"probe", align 1
@alloc_ec4612a7974a19b5164a1467c0223b7b = private unnamed_addr constant [3 x i8] c"src", align 1
@alloc_749443a32d6b0b2c0d8853cb47cc8f5e = private unnamed_addr constant [2 x i8] c"rs", align 1
@alloc_ec0fb45a03827c6e6fbcf8024afc16d6 = private unnamed_addr constant [26 x i8] c"\11Failed to create \C0\02: \C0\01\0A\00", align 1
@alloc_f36ce88bd5d4a921175f5521f484b675 = private unnamed_addr constant [13 x i8] c"RUSTC_WRAPPER", align 1
@alloc_fbe0d85396ee55e48aae2aa2891c1dc3 = private unnamed_addr constant [23 x i8] c"RUSTC_WORKSPACE_WRAPPER", align 1
@alloc_d563101362ed4a06747b9210d55c4c5b = private unnamed_addr constant [15 x i8] c"RUSTC_BOOTSTRAP", align 1
@alloc_d77d4c061985de9d519aa4991a74a299 = private unnamed_addr constant [28 x i8] c"--cfg=procmacro2_build_probe", align 1
@alloc_5b36b722e9f9b16c88a11c962ad61741 = private unnamed_addr constant [14 x i8] c"--edition=2021", align 1
@alloc_29adbd50892bfa5241d9339bfe91f188 = private unnamed_addr constant [24 x i8] c"--crate-name=proc_macro2", align 1
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
@alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26 = private unnamed_addr constant [11 x i8] c"RUSTC_STAGE", align 1
@alloc_3f5ffa2c8843a87c929224eb38f9612c = private unnamed_addr constant [35 x i8] c"cargo:rustc-check-cfg=cfg(fuzzing)\0A", align 1
@alloc_10e1a7b545f0c4f1d59070b08c5db106 = private unnamed_addr constant [43 x i8] c"cargo:rustc-check-cfg=cfg(no_is_available)\0A", align 1
@alloc_34d41541334731d13af8ae8372fb9cba = private unnamed_addr constant [53 x i8] c"cargo:rustc-check-cfg=cfg(no_literal_byte_character)\0A", align 1
@alloc_031bcea5e65ed731845949db9e0ab153 = private unnamed_addr constant [47 x i8] c"cargo:rustc-check-cfg=cfg(no_literal_c_string)\0A", align 1
@alloc_31141e1830579c4679b90d7c535c23eb = private unnamed_addr constant [42 x i8] c"cargo:rustc-check-cfg=cfg(no_source_text)\0A", align 1
@alloc_700cd7122563efc652176cd3deabdf94 = private unnamed_addr constant [43 x i8] c"cargo:rustc-check-cfg=cfg(proc_macro_span)\0A", align 1
@alloc_8b4555c431240d0c3945d5158c0cca26 = private unnamed_addr constant [48 x i8] c"cargo:rustc-check-cfg=cfg(proc_macro_span_file)\0A", align 1
@alloc_d4d6a4c8a8ec3eac6d741e3b835b1484 = private unnamed_addr constant [52 x i8] c"cargo:rustc-check-cfg=cfg(proc_macro_span_location)\0A", align 1
@alloc_d0e119f2301624664874e5e9731914f8 = private unnamed_addr constant [48 x i8] c"cargo:rustc-check-cfg=cfg(procmacro2_backtrace)\0A", align 1
@alloc_6820b661dd9cc52746977e525dd81c84 = private unnamed_addr constant [50 x i8] c"cargo:rustc-check-cfg=cfg(procmacro2_build_probe)\0A", align 1
@alloc_f64296080ab33db35230262cec8954d5 = private unnamed_addr constant [54 x i8] c"cargo:rustc-check-cfg=cfg(procmacro2_nightly_testing)\0A", align 1
@alloc_b7f5f0853ea12bb6490a947873f2b916 = private unnamed_addr constant [52 x i8] c"cargo:rustc-check-cfg=cfg(procmacro2_semver_exempt)\0A", align 1
@alloc_0e4bcf46f47e70282bb1af6e3f94cfe4 = private unnamed_addr constant [44 x i8] c"cargo:rustc-check-cfg=cfg(randomize_layout)\0A", align 1
@alloc_306832e19bb2d9d2c4f3f53ed7f1b42e = private unnamed_addr constant [42 x i8] c"cargo:rustc-check-cfg=cfg(span_locations)\0A", align 1
@alloc_7a9a2e1a912491d88aa3cacea2f24883 = private unnamed_addr constant [42 x i8] c"cargo:rustc-check-cfg=cfg(super_unstable)\0A", align 1
@alloc_26b0ac36ec7a3963a878d23341283334 = private unnamed_addr constant [43 x i8] c"cargo:rustc-check-cfg=cfg(wrap_proc_macro)\0A", align 1
@alloc_db1a0c96a1ddd2b29c00e1bd77fa5a95 = private unnamed_addr constant [32 x i8] c"cargo:rustc-cfg=no_is_available\0A", align 1
@alloc_c75a26d14bafba6eaea018555e51fe08 = private unnamed_addr constant [31 x i8] c"cargo:rustc-cfg=no_source_text\0A", align 1
@alloc_feb3714f38a5fb28df83bc12bd963cd2 = private unnamed_addr constant [42 x i8] c"cargo:rustc-cfg=no_literal_byte_character\0A", align 1
@alloc_2db9ba60cb7f510177111c1d84e78c0f = private unnamed_addr constant [36 x i8] c"cargo:rustc-cfg=no_literal_c_string\0A", align 1
@alloc_1c86b039f881a10c46f4717c27a32d0e = private unnamed_addr constant [15 x i8] c"proc_macro_span", align 1
@alloc_36f02c749ad09a7e3496c0b77a2eed14 = private unnamed_addr constant [32 x i8] c"cargo:rustc-cfg=wrap_proc_macro\0A", align 1
@alloc_91482d60220a3de858c1d085837d0ff7 = private unnamed_addr constant [32 x i8] c"cargo:rustc-cfg=proc_macro_span\0A", align 1
@alloc_bd43b6b16475694c16f8c5bd38c53a67 = private unnamed_addr constant [24 x i8] c"proc_macro_span_location", align 1
@alloc_99296b26f853c0e00fd05857e3136e29 = private unnamed_addr constant [41 x i8] c"cargo:rustc-cfg=proc_macro_span_location\0A", align 1
@alloc_ddcfd028fe4f27013cd9e73d07c52a9e = private unnamed_addr constant [20 x i8] c"proc_macro_span_file", align 1
@alloc_34ae0dc4ffa16faaed57f3cbab572bbd = private unnamed_addr constant [37 x i8] c"cargo:rustc-cfg=proc_macro_span_file\0A", align 1
@alloc_c4fe0d46c3935d35a63bc8de9de91c71 = private unnamed_addr constant [43 x i8] c"cargo:rerun-if-env-changed=RUSTC_BOOTSTRAP\0A", align 1
@alloc_7fe94be2e120ffbd80c490b1b3c481ee = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/pattern.rs\00", align 1
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECs7AmXS38G9s1_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
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
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i ], [ 0, %start ]
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
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #17, !noalias !4
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i

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

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #18
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

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs7AmXS38G9s1_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs7AmXS38G9s1_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs7AmXS38G9s1_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs7AmXS38G9s1_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs7AmXS38G9s1_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #17
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs7AmXS38G9s1_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs7AmXS38G9s1_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
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
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
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
    i64 -9223372036854775807, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECs7AmXS38G9s1_18build_script_build.exit5
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECs7AmXS38G9s1_18build_script_build.exit5
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECs7AmXS38G9s1_18build_script_build.exit5
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i4:                      ; preds = %bb4
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3 = load ptr, ptr %6, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3, i64 noundef %_1.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECs7AmXS38G9s1_18build_script_build.exit5

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter7sources4once4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEEECs7AmXS38G9s1_18build_script_build.exit5: ; preds = %bb4, %bb4, %bb4, %bb2.i.i.i4.i.i.i.i.i.i.i.i4
  ret void
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
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
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7AmXS38G9s1_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7AmXS38G9s1_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7AmXS38G9s1_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #18
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #18
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
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
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs7AmXS38G9s1_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !19
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs7AmXS38G9s1_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs7AmXS38G9s1_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::io::error::Error>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs7AmXS38G9s1_18build_script_build(ptr %_1.0.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %bits.i.i.i = ptrtoint ptr %_1.0.val to i64
  %_5.i.i.i = and i64 %bits.i.i.i, 3
  %switch.i.i = icmp eq i64 %_5.i.i.i, 1
  br i1 %switch.i.i, label %bb2.i2.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs7AmXS38G9s1_18build_script_build.exit, !prof !28

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
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i

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

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef 24, i64 noundef 8) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs7AmXS38G9s1_18build_script_build.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECs7AmXS38G9s1_18build_script_build.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !35
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !36, !noundef !3
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

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
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !35
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !36, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !36
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !35
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7AmXS38G9s1_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #17, !srcloc !42
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs7AmXS38G9s1_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #3 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7AmXS38G9s1_18build_script_build(ptr noundef nonnull %_4) #20
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs7AmXS38G9s1_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs7AmXS38G9s1_18build_script_build(ptr noundef nonnull readonly %0) #20, !noalias !43
  ret i32 0
}

; build_script_build::do_compile_probe
; Function Attrs: uwtable
define internal fastcc noundef zeroext i1 @_RNvCs7AmXS38G9s1_18build_script_build16do_compile_probe(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef range(i64 15, 25) %1, i1 noundef zeroext %rustc_bootstrap) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %x.sroa.0.i.i.i = alloca i64, align 8
  %_5.sroa.0.i = alloca i64, align 8
  %iter.i = alloca [72 x i8], align 8
  %_2.i132 = alloca [200 x i8], align 8
  %_3.i = alloca [4 x i8], align 2
  %_7.i81 = alloca [16 x i8], align 8
  %_2.i82 = alloca [24 x i8], align 8
  %key.i83 = alloca [16 x i8], align 8
  %_7.i = alloca [16 x i8], align 8
  %_2.i = alloca [24 x i8], align 8
  %key.i = alloca [16 x i8], align 8
  %args4 = alloca [32 x i8], align 8
  %_97 = alloca [16 x i8], align 8
  %err3 = alloca [8 x i8], align 8
  %_85 = alloca [16 x i8], align 8
  %iter = alloca [72 x i8], align 8
  %_74 = alloca [32 x i8], align 8
  %_67 = alloca [24 x i8], align 8
  %cmd = alloca [200 x i8], align 8
  %rustc2 = alloca [72 x i8], align 8
  %_39 = alloca [24 x i8], align 8
  %_37 = alloca [24 x i8], align 8
  %args1 = alloca [32 x i8], align 8
  %_29 = alloca [16 x i8], align 8
  %err = alloca [8 x i8], align 8
  %_17 = alloca [24 x i8], align 8
  %_15 = alloca [24 x i8], align 8
  %probefile = alloca [24 x i8], align 8
  %out_subdir = alloca [24 x i8], align 8
  %args = alloca [16 x i8], align 8
  %feature = alloca [16 x i8], align 8
  store ptr %0, ptr %feature, align 8
  %2 = getelementptr inbounds nuw i8, ptr %feature, i64 8
  store i64 %1, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %feature, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs7AmXS38G9s1_18build_script_build, ptr %_7.sroa.4.0..sroa_idx, align 8
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_71c72c6df710458e1c89ad45a5b4690d, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i)
  store ptr @alloc_806c1ac911172019779ceab530bc1f0e, ptr %key.i, align 8, !noalias !46
  %3 = getelementptr inbounds nuw i8, ptr %key.i, i64 8
  store i64 5, ptr %3, align 8, !noalias !46
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i), !noalias !46
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5), !noalias !50
  %4 = load i64, ptr %_2.i, align 8, !range !41, !noalias !46, !noundef !3
  %.not.i = icmp eq i64 %4, -9223372036854775808
  br i1 %.not.i, label %bb3.i, label %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit

bb3.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i), !noalias !46
  store ptr %key.i, ptr %_7.i, align 8, !noalias !46
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_7.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs7AmXS38G9s1_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i, align 8, !noalias !46
; call std::io::stdio::_eprint
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i), !noalias !50
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i), !noalias !46
; call std::process::exit
  call void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21, !noalias !50
  unreachable

_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit: ; preds = %start
  %rustc.sroa.5.0._2.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i, i64 8
  %rustc.sroa.5.0.copyload227 = load ptr, ptr %rustc.sroa.5.0._2.i.sroa_idx, align 8, !noalias !51
  %rustc.sroa.6.0._2.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i, i64 16
  %rustc.sroa.6.0.copyload228 = load i64, ptr %rustc.sroa.6.0._2.i.sroa_idx, align 8, !noalias !51
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !46
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i83)
  store ptr @alloc_ebcdb5f66b6f511cde89ece546cbdd6d, ptr %key.i83, align 8, !noalias !52
  %5 = getelementptr inbounds nuw i8, ptr %key.i83, i64 8
  store i64 7, ptr %5, align 8, !noalias !52
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i82), !noalias !52
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i82, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ebcdb5f66b6f511cde89ece546cbdd6d, i64 noundef 7)
          to label %.noexc unwind label %bb95.thread

.noexc:                                           ; preds = %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit
  %6 = load i64, ptr %_2.i82, align 8, !range !41, !noalias !52, !noundef !3
  %.not.i84 = icmp eq i64 %6, -9223372036854775808
  br i1 %.not.i84, label %bb3.i85, label %bb3

bb3.i85:                                          ; preds = %.noexc
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i81), !noalias !52
  store ptr %key.i83, ptr %_7.i81, align 8, !noalias !52
  %_8.sroa.4.0..sroa_idx.i86 = getelementptr inbounds nuw i8, ptr %_7.i81, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs7AmXS38G9s1_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i86, align 8, !noalias !52
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i81)
          to label %.noexc87 unwind label %bb95.thread

.noexc87:                                         ; preds = %bb3.i85
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i81), !noalias !52
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %.noexc88 unwind label %bb95.thread

.noexc88:                                         ; preds = %.noexc87
  unreachable

bb95:                                             ; preds = %bb2.i.i.i4.i.i.i, %bb76
  br i1 %_108.sroa.0.2.off0, label %bb94, label %bb77

bb95.thread:                                      ; preds = %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit, %bb3.i85, %.noexc87
  %7 = landingpad { ptr, i32 }
          cleanup
  br label %bb94

bb3:                                              ; preds = %.noexc
  %out_dir.sroa.5.0._2.i82.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i82, i64 8
  %out_dir.sroa.5.0.copyload = load ptr, ptr %out_dir.sroa.5.0._2.i82.sroa_idx, align 8, !noalias !56, !nonnull !3, !noundef !3
  %out_dir.sroa.8.0._2.i82.sroa_idx = getelementptr inbounds nuw i8, ptr %_2.i82, i64 16
  %out_dir.sroa.8.0.copyload = load i64, ptr %out_dir.sroa.8.0._2.i82.sroa_idx, align 8, !noalias !56
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i82), !noalias !52
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i83)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %out_subdir)
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %out_subdir, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %out_dir.sroa.5.0.copyload, i64 noundef %out_dir.sroa.8.0.copyload, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6b9c4d547a268aa1e418a0468c721e03, i64 noundef 5)
          to label %bb4 unwind label %cleanup5

bb76:                                             ; preds = %bb2.i.i.i4.i.i.i.i, %bb75, %cleanup5
  %_108.sroa.0.2.off0 = phi i1 [ true, %cleanup5 ], [ %_108.sroa.0.4.off0, %bb75 ], [ %_108.sroa.0.4.off0, %bb2.i.i.i4.i.i.i.i ]
  %.pn50.pn.pn = phi { ptr, i32 } [ %9, %cleanup5 ], [ %.pn50.pn, %bb75 ], [ %.pn50.pn, %bb2.i.i.i4.i.i.i.i ]
  %8 = icmp eq i64 %6, 0
  br i1 %8, label %bb95, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb76
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_dir.sroa.5.0.copyload, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb95

cleanup5:                                         ; preds = %bb3
  %9 = landingpad { ptr, i32 }
          cleanup
  br label %bb76

bb4:                                              ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %probefile)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_15)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_17)
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_17, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ec4612a7974a19b5164a1467c0223b7b, i64 noundef 3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6b9c4d547a268aa1e418a0468c721e03, i64 noundef 5)
          to label %bb5 unwind label %cleanup6

bb75:                                             ; preds = %bb93.thread275, %bb2.i.i.i4.i.i.i.i222, %bb92, %bb2.i.i.i4.i.i.i.i95, %bb74, %bb93, %cleanup6
  %_108.sroa.0.4.off0 = phi i1 [ false, %bb93 ], [ true, %cleanup6 ], [ true, %bb74 ], [ true, %bb2.i.i.i4.i.i.i.i95 ], [ %_108.sroa.0.5.off0244, %bb92 ], [ %_108.sroa.0.5.off0244, %bb2.i.i.i4.i.i.i.i222 ], [ false, %bb93.thread275 ]
  %.pn50.pn = phi { ptr, i32 } [ %.pn45, %bb93 ], [ %12, %cleanup6 ], [ %.pn, %bb74 ], [ %.pn, %bb2.i.i.i4.i.i.i.i95 ], [ %.pn50245, %bb92 ], [ %.pn50245, %bb2.i.i.i4.i.i.i.i222 ], [ %62, %bb93.thread275 ]
  %out_subdir.val = load i64, ptr %out_subdir, align 8
  %10 = icmp eq i64 %out_subdir.val, 0
  br i1 %10, label %bb76, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb75
  %11 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 8
  %out_subdir.val63 = load ptr, ptr %11, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_subdir.val63, i64 noundef %out_subdir.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb76

cleanup6:                                         ; preds = %bb4
  %12 = landingpad { ptr, i32 }
          cleanup
  br label %bb75

bb5:                                              ; preds = %bb4
  %13 = getelementptr inbounds nuw i8, ptr %_17, i64 8
  %_142 = load ptr, ptr %13, align 8, !nonnull !3, !noundef !3
  %14 = getelementptr inbounds nuw i8, ptr %_17, i64 16
  %_141 = load i64, ptr %14, align 8, !noundef !3
  %15 = load ptr, ptr %feature, align 8, !nonnull !3, !align !25, !noundef !3
  %16 = load i64, ptr %2, align 8, !noundef !3
; invoke <std::path::Path>::_join
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path5__join(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_15, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_142, i64 noundef %_141, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %15, i64 noundef %16)
          to label %bb6 unwind label %cleanup7

bb74:                                             ; preds = %bb2.i.i.i4.i.i.i.i98, %cleanup8, %cleanup7
  %.pn = phi { ptr, i32 } [ %18, %cleanup7 ], [ %21, %cleanup8 ], [ %21, %bb2.i.i.i4.i.i.i.i98 ]
  %_17.val = load i64, ptr %_17, align 8
  %17 = icmp eq i64 %_17.val, 0
  br i1 %17, label %bb75, label %bb2.i.i.i4.i.i.i.i95

bb2.i.i.i4.i.i.i.i95:                             ; preds = %bb74
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_142, i64 noundef %_17.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb75

cleanup7:                                         ; preds = %bb5
  %18 = landingpad { ptr, i32 }
          cleanup
  br label %bb74

bb6:                                              ; preds = %bb5
  %19 = getelementptr inbounds nuw i8, ptr %_15, i64 8
  %_152 = load ptr, ptr %19, align 8, !nonnull !3, !noundef !3
  %20 = getelementptr inbounds nuw i8, ptr %_15, i64 16
  %_151 = load i64, ptr %20, align 8, !noundef !3
; invoke <std::path::Path>::_with_extension
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path15__with_extension(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %probefile, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_152, i64 noundef %_151, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_749443a32d6b0b2c0d8853cb47cc8f5e, i64 noundef 2)
          to label %bb7 unwind label %cleanup8

cleanup8:                                         ; preds = %bb6
  %21 = landingpad { ptr, i32 }
          cleanup
  %_15.val = load i64, ptr %_15, align 8
  %22 = icmp eq i64 %_15.val, 0
  br i1 %22, label %bb74, label %bb2.i.i.i4.i.i.i.i98

bb2.i.i.i4.i.i.i.i98:                             ; preds = %cleanup8
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_152, i64 noundef %_15.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb74

bb7:                                              ; preds = %bb6
  %_15.val73 = load i64, ptr %_15, align 8
  %23 = icmp eq i64 %_15.val73, 0
  br i1 %23, label %bb8, label %bb2.i.i.i4.i.i.i.i100

bb2.i.i.i4.i.i.i.i100:                            ; preds = %bb7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_152, i64 noundef %_15.val73, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb8

bb8:                                              ; preds = %bb2.i.i.i4.i.i.i.i100, %bb7
  %_17.val71 = load i64, ptr %_17, align 8
  %24 = icmp eq i64 %_17.val71, 0
  br i1 %24, label %bb9, label %bb2.i.i.i4.i.i.i.i104

bb2.i.i.i4.i.i.i.i104:                            ; preds = %bb8
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_142, i64 noundef %_17.val71, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb9

bb93:                                             ; preds = %bb70
  br i1 %_107.sroa.0.4.off0, label %bb92, label %bb75

cleanup10:                                        ; preds = %bb78, %bb9
  %25 = landingpad { ptr, i32 }
          cleanup
  br label %bb92

bb9:                                              ; preds = %bb2.i.i.i4.i.i.i.i104, %bb8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_17)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_15)
  call void @llvm.experimental.noalias.scope.decl(metadata !57)
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_3.i), !noalias !57
  store i16 511, ptr %_3.i, align 2, !noalias !57
  %26 = getelementptr inbounds nuw i8, ptr %_3.i, i64 2
  store i8 0, ptr %26, align 2, !noalias !57
  %27 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 8
  %_2.val.i.i = load ptr, ptr %27, align 8, !alias.scope !57, !nonnull !3, !noundef !3
  %28 = getelementptr inbounds nuw i8, ptr %out_subdir, i64 16
  %_2.val1.i.i = load i64, ptr %28, align 8, !alias.scope !57, !noundef !3
; invoke <std::fs::DirBuilder>::_create
  %_0.i.i106 = invoke noundef ptr @_RNvMsF_NtCs5sEH5CPMdak_3std2fsNtB5_10DirBuilder7__create(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(4) %_3.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i, i64 noundef %_2.val1.i.i)
          to label %bb10 unwind label %cleanup10

bb10:                                             ; preds = %bb9
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_3.i), !noalias !57
  %.not = icmp eq ptr %_0.i.i106, null
  br i1 %.not, label %bb78, label %bb11

bb11:                                             ; preds = %bb10
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err)
  store ptr %_0.i.i106, ptr %err, align 8
; call <std::io::error::Error>::kind
  %_24 = call fastcc noundef i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr nonnull %_0.i.i106)
  %_155.not = icmp eq i8 %_24, 12
  br i1 %_155.not, label %bb15, label %bb13

bb78:                                             ; preds = %bb16, %bb10
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_37)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_37, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f36ce88bd5d4a921175f5521f484b675, i64 noundef 13)
          to label %bb17 unwind label %cleanup10

cleanup11:                                        ; preds = %bb14, %bb13
  %29 = landingpad { ptr, i32 }
          cleanup
  %err.val = load ptr, ptr %err, align 8, !nonnull !3, !noundef !3
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs7AmXS38G9s1_18build_script_build(ptr nonnull %err.val) #18
          to label %bb92 unwind label %terminate

bb15:                                             ; preds = %bb11
  %bits.i.i.i.i = ptrtoint ptr %_0.i.i106 to i64
  %_5.i.i.i.i = and i64 %bits.i.i.i.i, 3
  %switch.i.i.i = icmp eq i64 %_5.i.i.i.i, 1
  br i1 %switch.i.i.i, label %bb2.i2.i.i.i, label %bb16, !prof !28

bb2.i2.i.i.i:                                     ; preds = %bb15
  %30 = getelementptr i8, ptr %_0.i.i106, i64 -1
  %31 = icmp ne ptr %30, null
  call void @llvm.assume(i1 %31)
  %_6.val.i.i.i.i.i = load ptr, ptr %30, align 8
  %32 = getelementptr i8, ptr %_0.i.i106, i64 7
  %_6.val1.i.i.i.i.i = load ptr, ptr %32, align 8, !nonnull !3, !align !7, !noundef !3
  %33 = load ptr, ptr %_6.val1.i.i.i.i.i, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i = icmp eq ptr %33, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i:                        ; preds = %bb2.i2.i.i.i
  %34 = icmp ne ptr %_6.val.i.i.i.i.i, null
  call void @llvm.assume(i1 %34)
  invoke void %33(ptr noundef nonnull %_6.val.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %is_not_null.i.i.i.i.i.i.i, %bb2.i2.i.i.i
  %35 = icmp ne ptr %_6.val.i.i.i.i.i, null
  call void @llvm.assume(i1 %35)
  %36 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %37 = load i64, ptr %36, align 8, !range !8, !invariant.load !3
  %38 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %39 = load i64, ptr %38, align 8, !range !9, !invariant.load !3
  %40 = add i64 %39, -1
  %41 = icmp sgt i64 %40, -1
  call void @llvm.assume(i1 %41)
  %42 = icmp eq i64 %37, 0
  br i1 %42, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %37, i64 noundef range(i64 1, -9223372036854775807) %39) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i

cleanup.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i
  %43 = landingpad { ptr, i32 }
          cleanup
  %44 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 8
  %45 = load i64, ptr %44, align 8, !range !8, !invariant.load !3
  %46 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i, i64 16
  %47 = load i64, ptr %46, align 8, !range !9, !invariant.load !3
  %48 = add i64 %47, -1
  %49 = icmp sgt i64 %48, -1
  call void @llvm.assume(i1 %49)
  %50 = icmp eq i64 %45, 0
  br i1 %50, label %bb1.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i, i64 noundef %45, i64 noundef range(i64 1, -9223372036854775807) %47) #17
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %30, i64 noundef 24, i64 noundef 8) #17
  br label %bb92

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %30, i64 noundef 24, i64 noundef 8) #17
  br label %bb16

bb13:                                             ; preds = %bb11
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_29)
  store ptr %_2.val.i.i, ptr %_29, align 8
  %51 = getelementptr inbounds nuw i8, ptr %_29, i64 8
  store i64 %_2.val1.i.i, ptr %51, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args1)
  store ptr %_29, ptr %args1, align 8
  %_32.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_32.sroa.4.0..sroa_idx, align 8
  %52 = getelementptr inbounds nuw i8, ptr %args1, i64 16
  store ptr %err, ptr %52, align 8
  %_33.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 24
  store ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_33.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_ec0fb45a03827c6e6fbcf8024afc16d6, ptr noundef nonnull %args1)
          to label %bb14 unwind label %cleanup11

bb16:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i, %bb15
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err)
  br label %bb78

bb14:                                             ; preds = %bb13
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_29)
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %unreachable unwind label %cleanup11

unreachable:                                      ; preds = %bb63, %bb14
  unreachable

terminate:                                        ; preds = %cleanup11, %cleanup18, %bb70
  %53 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

bb17:                                             ; preds = %bb78
  call void @llvm.experimental.noalias.scope.decl(metadata !60)
  %54 = load i64, ptr %_37, align 8, !range !41, !alias.scope !60, !noalias !63, !noundef !3
  %.not.i108 = icmp eq i64 %54, -9223372036854775808
  br i1 %.not.i108, label %bb18, label %bb2.i

bb2.i:                                            ; preds = %bb17
  %x.sroa.7.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_37, i64 8
  %x.sroa.7.0.copyload.i = load ptr, ptr %x.sroa.7.0.self.sroa_idx.i, align 8, !alias.scope !60, !noalias !63
  %x.sroa.9.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_37, i64 16
  %x.sroa.9.0.copyload.i = load i64, ptr %x.sroa.9.0.self.sroa_idx.i, align 8, !alias.scope !60, !noalias !63
  %_3.i.not.i = icmp eq i64 %x.sroa.9.0.copyload.i, 0
  br i1 %_3.i.not.i, label %bb4.i, label %bb18

bb4.i:                                            ; preds = %bb2.i
  %55 = icmp eq i64 %54, 0
  br i1 %55, label %bb18, label %bb2.i.i.i4.i.i.i5.i

bb2.i.i.i4.i.i.i5.i:                              ; preds = %bb4.i
  %56 = icmp ne ptr %x.sroa.7.0.copyload.i, null
  call void @llvm.assume(i1 %56)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload.i, i64 noundef %54, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !65
  br label %bb18

bb18:                                             ; preds = %bb17, %bb2.i.i.i4.i.i.i5.i, %bb4.i, %bb2.i
  %rustc_wrapper.sroa.9.0 = phi i64 [ undef, %bb17 ], [ undef, %bb2.i.i.i4.i.i.i5.i ], [ undef, %bb4.i ], [ %x.sroa.9.0.copyload.i, %bb2.i ]
  %rustc_wrapper.sroa.7.0 = phi ptr [ undef, %bb17 ], [ undef, %bb2.i.i.i4.i.i.i5.i ], [ undef, %bb4.i ], [ %x.sroa.7.0.copyload.i, %bb2.i ]
  %rustc_wrapper.sroa.0.0 = phi i64 [ -9223372036854775808, %bb17 ], [ -9223372036854775808, %bb2.i.i.i4.i.i.i5.i ], [ -9223372036854775808, %bb4.i ], [ %54, %bb2.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_37)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_39)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_39, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_fbe0d85396ee55e48aae2aa2891c1dc3, i64 noundef 23)
          to label %bb19 unwind label %bb90

bb19:                                             ; preds = %bb18
  call void @llvm.experimental.noalias.scope.decl(metadata !66)
  %57 = load i64, ptr %_39, align 8, !range !41, !alias.scope !66, !noalias !69, !noundef !3
  %.not.i112 = icmp eq i64 %57, -9223372036854775808
  br i1 %.not.i112, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i, label %bb2.i113

bb2.i113:                                         ; preds = %bb19
  %x.sroa.7.0.self.sroa_idx.i114 = getelementptr inbounds nuw i8, ptr %_39, i64 8
  %x.sroa.7.0.copyload.i115 = load ptr, ptr %x.sroa.7.0.self.sroa_idx.i114, align 8, !alias.scope !66, !noalias !69
  %x.sroa.9.0.self.sroa_idx.i116 = getelementptr inbounds nuw i8, ptr %_39, i64 16
  %x.sroa.9.0.copyload.i117 = load i64, ptr %x.sroa.9.0.self.sroa_idx.i116, align 8, !alias.scope !66, !noalias !69
  %_3.i.not.i118 = icmp eq i64 %x.sroa.9.0.copyload.i117, 0
  br i1 %_3.i.not.i118, label %bb4.i122, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i

bb4.i122:                                         ; preds = %bb2.i113
  %58 = icmp eq i64 %57, 0
  br i1 %58, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i, label %bb2.i.i.i4.i.i.i5.i123

bb2.i.i.i4.i.i.i5.i123:                           ; preds = %bb4.i122
  %59 = icmp ne ptr %x.sroa.7.0.copyload.i115, null
  call void @llvm.assume(i1 %59)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload.i115, i64 noundef %57, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !71
  br label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i: ; preds = %bb19, %bb2.i.i.i4.i.i.i5.i123, %bb4.i122, %bb2.i113
  %.val3.i.i.i = phi ptr [ undef, %bb19 ], [ undef, %bb2.i.i.i4.i.i.i5.i123 ], [ undef, %bb4.i122 ], [ %x.sroa.7.0.copyload.i115, %bb2.i113 ]
  %60 = phi i64 [ -9223372036854775808, %bb19 ], [ -9223372036854775808, %bb2.i.i.i4.i.i.i5.i123 ], [ -9223372036854775808, %bb4.i122 ], [ %57, %bb2.i113 ]
  %x.sroa.11.0.copyload7.i = phi i64 [ undef, %bb19 ], [ undef, %bb2.i.i.i4.i.i.i5.i123 ], [ undef, %bb4.i122 ], [ %x.sroa.9.0.copyload.i117, %bb2.i113 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_39)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %rustc2)
  call void @llvm.experimental.noalias.scope.decl(metadata !72)
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  %61 = getelementptr inbounds nuw i8, ptr %rustc2, i64 24
  %_41.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 32
  store ptr %rustc_wrapper.sroa.7.0, ptr %_41.sroa.4.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_41.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 40
  store i64 %rustc_wrapper.sroa.9.0, ptr %_41.sroa.5.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_41.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 48
  store i64 %60, ptr %_41.sroa.6.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_41.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 56
  store ptr %.val3.i.i.i, ptr %_41.sroa.7.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  %_41.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 64
  store i64 %x.sroa.11.0.copyload7.i, ptr %_41.sroa.8.0..sroa_idx, align 8, !alias.scope !77, !noalias !75
  store i64 %4, ptr %rustc2, align 8, !alias.scope !79, !noalias !72
  %_44.sroa.4.0.rustc2.sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 8
  store ptr %rustc.sroa.5.0.copyload227, ptr %_44.sroa.4.0.rustc2.sroa_idx, align 8, !alias.scope !79, !noalias !72
  %_44.sroa.5.0.rustc2.sroa_idx = getelementptr inbounds nuw i8, ptr %rustc2, i64 16
  store i64 %rustc.sroa.6.0.copyload228, ptr %_44.sroa.5.0.rustc2.sroa_idx, align 8, !alias.scope !79, !noalias !72
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %cmd)
  call void @llvm.experimental.noalias.scope.decl(metadata !80)
  call void @llvm.experimental.noalias.scope.decl(metadata !83)
  %.not3.i.i.i.i = icmp eq i64 %rustc_wrapper.sroa.0.0, -9223372036854775808
  %spec.store.select.i.i.i.i = select i1 %.not3.i.i.i.i, i64 -9223372036854775807, i64 -9223372036854775808
  store i64 %spec.store.select.i.i.i.i, ptr %61, align 8, !alias.scope !86, !noalias !93
  br i1 %.not3.i.i.i.i, label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i, label %bb99

_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i: ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i
  store i64 -9223372036854775808, ptr %_41.sroa.6.0..sroa_idx, align 8, !alias.scope !96, !noalias !103
  %.not3.i = icmp eq i64 %60, -9223372036854775808
  br i1 %.not3.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i, label %bb99

bb93.thread275:                                   ; preds = %bb83
  %62 = landingpad { ptr, i32 }
          cleanup
  br label %bb75

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i: ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i
  store i64 -9223372036854775806, ptr %61, align 8, !alias.scope !105, !noalias !106
  store i64 -9223372036854775808, ptr %rustc2, align 8, !alias.scope !107, !noalias !114
  br label %bb99

bb99:                                             ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i
  %_48.sroa.0.0._48.sroa.0.0. = phi i64 [ %4, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.0.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %60, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i ]
  %_48.sroa.7.1 = phi ptr [ %rustc.sroa.5.0.copyload227, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.7.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %.val3.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i ]
  %_48.sroa.8.1 = phi i64 [ %rustc.sroa.6.0.copyload228, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i ], [ %rustc_wrapper.sroa.9.0, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %x.sroa.11.0.copyload7.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i ]
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i132), !noalias !116
  %63 = icmp ne ptr %_48.sroa.7.1, null
  call void @llvm.assume(i1 %63)
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i132, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_48.sroa.7.1, i64 noundef %_48.sroa.8.1)
          to label %bb2.i134 unwind label %cleanup.i, !noalias !116

cleanup.i:                                        ; preds = %bb99
  %64 = landingpad { ptr, i32 }
          cleanup
  %65 = icmp eq i64 %_48.sroa.0.0._48.sroa.0.0., 0
  br i1 %65, label %bb88, label %bb2.i.i.i4.i.i.i.i133

bb2.i.i.i4.i.i.i.i133:                            ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_48.sroa.7.1, i64 noundef %_48.sroa.0.0._48.sroa.0.0., i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !116
  br label %bb88

bb2.i134:                                         ; preds = %bb99
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %cmd, ptr noundef nonnull align 8 dereferenceable(200) %_2.i132, i64 200, i1 false), !noalias !120
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i132), !noalias !116
  %66 = icmp eq i64 %_48.sroa.0.0._48.sroa.0.0., 0
  br i1 %66, label %bb23, label %bb2.i.i.i4.i.i.i6.i

bb2.i.i.i4.i.i.i6.i:                              ; preds = %bb2.i134
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_48.sroa.7.1, i64 noundef %_48.sroa.0.0._48.sroa.0.0., i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !116
  br label %bb23

bb23:                                             ; preds = %bb2.i.i.i4.i.i.i6.i, %bb2.i134
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter.i), !noalias !121
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %iter.i, ptr noundef nonnull align 8 dereferenceable(72) %rustc2, i64 72, i1 false)
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
  br label %bb2.i136

bb2.i136:                                         ; preds = %bb9.i, %bb23
  %67 = phi i64 [ %_3.i.promoted.i, %bb23 ], [ %78, %bb9.i ]
  %68 = phi i64 [ %_3.i.promoted.i, %bb23 ], [ %79, %bb9.i ]
  %_5.sroa.9.0.i = phi i64 [ undef, %bb23 ], [ %_5.sroa.9.220.i, %bb9.i ]
  %_5.sroa.8.0.i = phi ptr [ undef, %bb23 ], [ %_5.sroa.8.221.i, %bb9.i ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !133)
  call void @llvm.experimental.noalias.scope.decl(metadata !134)
  call void @llvm.experimental.noalias.scope.decl(metadata !135)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  %.not.i.i.i = icmp eq i64 %68, -9223372036854775806
  br i1 %.not.i.i.i, label %bb2.i.i.i, label %bb11.i.i.i

bb11.i.i.i:                                       ; preds = %bb2.i136
  call void @llvm.experimental.noalias.scope.decl(metadata !136)
  call void @llvm.experimental.noalias.scope.decl(metadata !139)
  %.not.i.i.i.i.i.i = icmp eq i64 %68, -9223372036854775807
  br i1 %.not.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i: ; preds = %bb11.i.i.i
  %.not3.i.i.i.i.i.i = icmp eq i64 %68, -9223372036854775808
  %spec.store.select.i.i.i.i.i.i = select i1 %.not3.i.i.i.i.i.i, i64 -9223372036854775807, i64 -9223372036854775808
  store i64 %spec.store.select.i.i.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !142, !noalias !147
  call void @llvm.experimental.noalias.scope.decl(metadata !149)
  br i1 %.not3.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %bb3.thread.i

bb2.i.i.i.i.i.i:                                  ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i, %bb11.i.i.i
  %69 = phi i64 [ -9223372036854775807, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ], [ %67, %bb11.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !152)
  %70 = load i64, ptr %_57.i.i.i.i.i, align 8, !range !18, !alias.scope !155, !noalias !158, !noundef !3
  %.not.i.i.i.i.i.i.i138 = icmp eq i64 %70, -9223372036854775807
  br i1 %.not.i.i.i.i.i.i.i138, label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i.i
  store i64 %70, ptr %x.sroa.0.i.i.i, align 8, !alias.scope !160, !noalias !164
  br label %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i

_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i: ; preds = %bb5.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i
  %x.sroa.11.0.i.i.i = phi i64 [ undef, %bb2.i.i.i.i.i.i ], [ %x.sroa.11.0.copyload7.i.i.i, %bb5.i.i.i.i.i.i.i ]
  %x.sroa.9.0.i.i.i = phi ptr [ undef, %bb2.i.i.i.i.i.i ], [ %x.sroa.9.0.copyload6.i.i.i, %bb5.i.i.i.i.i.i.i ]
  %_1.sink.i.i.i.i.i.i.i = phi ptr [ %x.sroa.0.i.i.i, %bb2.i.i.i.i.i.i ], [ %_57.i.i.i.i.i, %bb5.i.i.i.i.i.i.i ]
  store i64 -9223372036854775808, ptr %_1.sink.i.i.i.i.i.i.i, align 8, !alias.scope !165, !noalias !166
  %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i = load i64, ptr %x.sroa.0.i.i.i, align 8, !noalias !167
  %.not3.i.i.i = icmp eq i64 %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i, -9223372036854775808
  br i1 %.not3.i.i.i, label %bb4.i.i.i.i.i, label %bb3.thread.i

bb4.i.i.i.i.i:                                    ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !168)
  call void @llvm.experimental.noalias.scope.decl(metadata !171)
  %.val2.i.i.i.i.i = load i64, ptr %_57.i.i.i.i.i, align 8, !range !18, !alias.scope !174, !noalias !130, !noundef !3
  switch i64 %.val2.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i.i139
    i64 -9223372036854775808, label %bb4.i.i.i139
    i64 0, label %bb4.i.i.i139
  ]

bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i:              ; preds = %bb4.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.9.0.copyload6.i.i.i, i64 noundef %.val2.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !175
  br label %bb4.i.i.i139

bb4.i.i.i139:                                     ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i.i, %bb4.i.i.i.i.i, %bb4.i.i.i.i.i, %bb4.i.i.i.i.i
  store i64 -9223372036854775806, ptr %_3.i.i, align 8, !alias.scope !125, !noalias !130
  br label %bb2.i.i.i

bb3.thread.i:                                     ; preds = %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i
  %71 = phi i64 [ %69, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ -9223372036854775808, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ]
  %72 = phi i64 [ -9223372036854775807, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ -9223372036854775808, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.7.0.i.i = phi i64 [ %x.sroa.11.0.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %x.sroa.8.0.copyload8.i.i.i.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.6.0.i.i = phi ptr [ %x.sroa.9.0.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %x.sroa.7.0.copyload7.i.i.i.i.i.i, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ]
  %_2.sroa.0.0.i.i = phi i64 [ %x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.i.0.x.sroa.0.i.0.x.sroa.0.i.0.x.sroa.0.0.x.sroa.0.0.x.sroa.0.0..pr.i.i.i, %_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build.exit.i.i.i ], [ %68, %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !176)
  call void @llvm.experimental.noalias.scope.decl(metadata !179)
  call void @llvm.experimental.noalias.scope.decl(metadata !181)
  store i64 %_2.sroa.0.0.i.i, ptr %_5.sroa.0.i, align 8, !alias.scope !183, !noalias !184
  br label %bb7.i

bb2.i.i.i:                                        ; preds = %bb4.i.i.i139, %bb2.i136
  %73 = phi i64 [ -9223372036854775806, %bb4.i.i.i139 ], [ %67, %bb2.i136 ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %x.sroa.0.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !185)
  %74 = load i64, ptr %iter.i, align 8, !range !18, !alias.scope !188, !noalias !190, !noundef !3
  %.not.i.i.i.i140 = icmp eq i64 %74, -9223372036854775807
  br i1 %.not.i.i.i.i140, label %bb3.i141, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb2.i.i.i
  store i64 %74, ptr %_5.sroa.0.i, align 8, !alias.scope !191, !noalias !195
  br label %bb3.i141

bb12.i:                                           ; preds = %bb2.i.i.i4.i.i.i.i137, %cleanup1.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::option::IntoIter<std::ffi::os_str::OsString>>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %iter.i) #18, !noalias !196
  br label %bb70

bb3.i141:                                         ; preds = %bb5.i.i.i.i, %bb2.i.i.i
  %_5.sroa.9.1.i = phi i64 [ %_5.sroa.9.0.i, %bb2.i.i.i ], [ %_5.sroa.9.0.copyload13.i, %bb5.i.i.i.i ]
  %_5.sroa.8.1.i = phi ptr [ %_5.sroa.8.0.i, %bb2.i.i.i ], [ %_5.sroa.8.0.copyload12.i, %bb5.i.i.i.i ]
  %_1.sink.i.i.i.i = phi ptr [ %_5.sroa.0.i, %bb2.i.i.i ], [ %iter.i, %bb5.i.i.i.i ]
  store i64 -9223372036854775808, ptr %_1.sink.i.i.i.i, align 8, !alias.scope !197, !noalias !195
  %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i = load i64, ptr %_5.sroa.0.i, align 8, !noalias !121
  %.not.i142 = icmp eq i64 %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i, -9223372036854775808
  br i1 %.not.i142, label %bb6.i143, label %bb7.i

bb6.i143:                                         ; preds = %bb3.i141
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !198)
  call void @llvm.experimental.noalias.scope.decl(metadata !201)
  %75 = icmp eq i64 %73, -9223372036854775806
  br i1 %75, label %bb4.i.i, label %bb2.i.i8.i

bb2.i.i8.i:                                       ; preds = %bb6.i143
  call void @llvm.experimental.noalias.scope.decl(metadata !204)
  switch i64 %73, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775807, label %bb4.i.i.i.i
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb2.i.i8.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %x.sroa.7.0.copyload7.i.i.i.i.i.i, i64 noundef %73, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !207
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

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i7.i.i.i.i, %bb4.i.i.i.i, %bb4.i.i.i.i, %bb4.i.i.i.i, %bb6.i143
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
  %76 = landingpad { ptr, i32 }
          cleanup
  %77 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.19.i, 0
  br i1 %77, label %bb12.i, label %bb2.i.i.i4.i.i.i.i137

bb2.i.i.i4.i.i.i.i137:                            ; preds = %cleanup1.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.221.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !196
  br label %bb12.i

bb7.i:                                            ; preds = %bb3.i141, %bb3.thread.i
  %78 = phi i64 [ %71, %bb3.thread.i ], [ %73, %bb3.i141 ]
  %79 = phi i64 [ %72, %bb3.thread.i ], [ -9223372036854775806, %bb3.i141 ]
  %_5.sroa.8.221.i = phi ptr [ %_2.sroa.6.0.i.i, %bb3.thread.i ], [ %_5.sroa.8.1.i, %bb3.i141 ]
  %_5.sroa.9.220.i = phi i64 [ %_2.sroa.7.0.i.i, %bb3.thread.i ], [ %_5.sroa.9.1.i, %bb3.i141 ]
  %_5.sroa.0.0._5.sroa.0.0.19.i = phi i64 [ %_2.sroa.0.0.i.i, %bb3.thread.i ], [ %_5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.i.0._5.sroa.0.0._5.sroa.0.0._5.sroa.0.0..pr.i, %bb3.i141 ]
  %80 = icmp ne ptr %_5.sroa.8.221.i, null
  call void @llvm.assume(i1 %80)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_5.sroa.8.221.i, i64 noundef %_5.sroa.9.220.i)
          to label %bb8.i unwind label %cleanup1.i, !noalias !196

bb8.i:                                            ; preds = %bb7.i
  %81 = icmp eq i64 %_5.sroa.0.0._5.sroa.0.0.19.i, 0
  br i1 %81, label %bb9.i, label %bb2.i.i.i4.i.i.i9.i

bb2.i.i.i4.i.i.i9.i:                              ; preds = %bb8.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.sroa.8.221.i, i64 noundef %_5.sroa.0.0._5.sroa.0.0.19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !196
  br label %bb9.i

bb9.i:                                            ; preds = %bb2.i.i.i4.i.i.i9.i, %bb8.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_5.sroa.0.i)
  br label %bb2.i136

bb70:                                             ; preds = %cleanup16, %bb2.i.i.i4.i.i185, %bb2.i.i.i4.i.i.i183, %bb86, %bb2.i.i.i4.i.i.i.i177, %cleanup.i175, %cleanup.i163, %bb2.i.i.i4.i.i.i.i.i, %bb12.i, %bb1.i.i.i.i.i.i, %bb1.i.i.i.i.i209, %cleanup14, %cleanup18
  %_107.sroa.0.4.off0 = phi i1 [ false, %cleanup18 ], [ true, %bb12.i ], [ false, %bb1.i.i.i.i.i.i ], [ %_107.sroa.0.5.off0, %cleanup14 ], [ false, %bb1.i.i.i.i.i209 ], [ false, %bb2.i.i.i4.i.i.i.i.i ], [ false, %cleanup.i163 ], [ false, %cleanup.i175 ], [ false, %bb2.i.i.i4.i.i.i.i177 ], [ false, %bb86 ], [ false, %bb2.i.i.i4.i.i.i183 ], [ false, %bb2.i.i.i4.i.i185 ], [ false, %cleanup16 ]
  %.pn45 = phi { ptr, i32 } [ %134, %cleanup18 ], [ %76, %bb12.i ], [ %125, %bb1.i.i.i.i.i.i ], [ %82, %cleanup14 ], [ %148, %bb1.i.i.i.i.i209 ], [ %83, %bb2.i.i.i4.i.i.i.i.i ], [ %83, %cleanup.i163 ], [ %88, %cleanup.i175 ], [ %88, %bb2.i.i.i4.i.i.i.i177 ], [ %91, %bb86 ], [ %91, %bb2.i.i.i4.i.i.i183 ], [ %99, %bb2.i.i.i4.i.i185 ], [ %99, %cleanup16 ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd) #18
          to label %bb93 unwind label %terminate

cleanup14:                                        ; preds = %bb57, %bb79, %bb37, %bb35, %bb34, %bb33, %bb32, %bb31, %bb30, %bb29, %bb28, %bb27, %bb25, %bb80
  %_107.sroa.0.5.off0 = phi i1 [ false, %bb80 ], [ true, %bb25 ], [ true, %bb27 ], [ true, %bb28 ], [ true, %bb29 ], [ true, %bb30 ], [ true, %bb31 ], [ true, %bb32 ], [ true, %bb33 ], [ true, %bb34 ], [ true, %bb35 ], [ false, %bb37 ], [ false, %bb79 ], [ false, %bb57 ]
  %82 = landingpad { ptr, i32 }
          cleanup
  br label %bb70

bb24:                                             ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i4.i.i, %bb4.i.i, %bb4.i.i, %bb4.i.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter.i), !noalias !121
  br i1 %rustc_bootstrap, label %bb27, label %bb25

bb25:                                             ; preds = %bb24
  %_4.i = getelementptr inbounds nuw i8, ptr %cmd, i64 96
; invoke <std::sys::process::env::CommandEnv>::remove
  invoke void @_RNvMs_NtNtNtCs5sEH5CPMdak_3std3sys7process3envNtB4_10CommandEnv6remove(ptr noalias noundef nonnull align 8 dereferenceable(32) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d563101362ed4a06747b9210d55c4c5b, i64 noundef 15)
          to label %bb27 unwind label %cleanup14

bb27:                                             ; preds = %bb25, %bb24
; invoke <std::sys::process::unix::common::Command>::stderr
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command6stderr(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, i32 noundef 1, i32 undef)
          to label %bb28 unwind label %cleanup14

bb28:                                             ; preds = %bb27
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d77d4c061985de9d519aa4991a74a299, i64 noundef 28)
          to label %bb29 unwind label %cleanup14

bb29:                                             ; preds = %bb28
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5b36b722e9f9b16c88a11c962ad61741, i64 noundef 14)
          to label %bb30 unwind label %cleanup14

bb30:                                             ; preds = %bb29
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_29adbd50892bfa5241d9339bfe91f188, i64 noundef 24)
          to label %bb31 unwind label %cleanup14

bb31:                                             ; preds = %bb30
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a6e356e753364954471dcbf409cc4c4e, i64 noundef 16)
          to label %bb32 unwind label %cleanup14

bb32:                                             ; preds = %bb31
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9930910b9e2bf161f6d41704390848d2, i64 noundef 17)
          to label %bb33 unwind label %cleanup14

bb33:                                             ; preds = %bb32
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4d01ae01a4b8e52c5a54208511747587, i64 noundef 24)
          to label %bb34 unwind label %cleanup14

bb34:                                             ; preds = %bb33
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_8bbf703e0ecc0326ac386a57604275da, i64 noundef 9)
          to label %bb35 unwind label %cleanup14

bb35:                                             ; preds = %bb34
  %_2.val.i.i160 = load ptr, ptr %27, align 8, !alias.scope !210, !noalias !213, !nonnull !3, !noundef !3
  %_2.val1.i.i161 = load i64, ptr %28, align 8, !alias.scope !210, !noalias !213, !noundef !3
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i160, i64 noundef %_2.val1.i.i161)
          to label %bb36 unwind label %cleanup14

bb36:                                             ; preds = %bb35
  %_66.sroa.0.0.copyload = load i64, ptr %probefile, align 8
  %_66.sroa.5.0.probefile.sroa_idx = getelementptr inbounds nuw i8, ptr %probefile, i64 8
  %_66.sroa.5.0.copyload = load ptr, ptr %_66.sroa.5.0.probefile.sroa_idx, align 8, !nonnull !3, !noundef !3
  %_66.sroa.6.0.probefile.sroa_idx = getelementptr inbounds nuw i8, ptr %probefile, i64 16
  %_66.sroa.6.0.copyload = load i64, ptr %_66.sroa.6.0.probefile.sroa_idx, align 8
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_66.sroa.5.0.copyload, i64 noundef %_66.sroa.6.0.copyload)
          to label %bb2.i165 unwind label %cleanup.i163, !noalias !215

cleanup.i163:                                     ; preds = %bb36
  %83 = landingpad { ptr, i32 }
          cleanup
  %84 = icmp eq i64 %_66.sroa.0.0.copyload, 0
  br i1 %84, label %bb70, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i163
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_66.sroa.5.0.copyload, i64 noundef %_66.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !215
  br label %bb70

bb2.i165:                                         ; preds = %bb36
  %85 = icmp eq i64 %_66.sroa.0.0.copyload, 0
  br i1 %85, label %bb37, label %bb2.i.i.i4.i.i.i.i6.i

bb2.i.i.i4.i.i.i.i6.i:                            ; preds = %bb2.i165
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_66.sroa.5.0.copyload, i64 noundef %_66.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !215
  br label %bb37

bb37:                                             ; preds = %bb2.i.i.i4.i.i.i.i6.i, %bb2.i165
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_67)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_67, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dcbc225a8ec7dbfaaef714ff8a7176fb, i64 noundef 6)
          to label %bb38 unwind label %cleanup14

bb38:                                             ; preds = %bb37
  %86 = load i64, ptr %_67, align 8, !range !41, !noundef !3
  %.not33 = icmp eq i64 %86, -9223372036854775808
  br i1 %.not33, label %bb79, label %bb39

bb39:                                             ; preds = %bb38
  %target.sroa.5.0._67.sroa_idx = getelementptr inbounds nuw i8, ptr %_67, i64 8
  %target.sroa.5.0.copyload = load ptr, ptr %target.sroa.5.0._67.sroa_idx, align 8
  %target.sroa.6.0._67.sroa_idx = getelementptr inbounds nuw i8, ptr %_67, i64 16
  %target.sroa.6.0.copyload = load i64, ptr %target.sroa.6.0._67.sroa_idx, align 8
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c20974c698c079af35c03642327d3f4f, i64 noundef 8)
          to label %bb40 unwind label %bb86

bb79:                                             ; preds = %bb2.i179, %bb2.i.i.i4.i.i.i6.i181, %bb38
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_67)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_74)
; invoke std::env::_var
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env4__var(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_74, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_07f3eec4949a8d39db630a4a477c65b3, i64 noundef 23)
          to label %bb42 unwind label %cleanup14

bb40:                                             ; preds = %bb39
  %87 = icmp ne ptr %target.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %87)
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %target.sroa.5.0.copyload, i64 noundef %target.sroa.6.0.copyload)
          to label %bb2.i179 unwind label %cleanup.i175, !noalias !218

cleanup.i175:                                     ; preds = %bb40
  %88 = landingpad { ptr, i32 }
          cleanup
  %89 = icmp eq i64 %86, 0
  br i1 %89, label %bb70, label %bb2.i.i.i4.i.i.i.i177

bb2.i.i.i4.i.i.i.i177:                            ; preds = %cleanup.i175
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %86, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !218
  br label %bb70

bb2.i179:                                         ; preds = %bb40
  %90 = icmp eq i64 %86, 0
  br i1 %90, label %bb79, label %bb2.i.i.i4.i.i.i6.i181

bb2.i.i.i4.i.i.i6.i181:                           ; preds = %bb2.i179
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %86, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !218
  br label %bb79

bb86:                                             ; preds = %bb39
  %91 = landingpad { ptr, i32 }
          cleanup
  %92 = icmp eq i64 %86, 0
  br i1 %92, label %bb70, label %bb2.i.i.i4.i.i.i183

bb2.i.i.i4.i.i.i183:                              ; preds = %bb86
  %93 = icmp ne ptr %target.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %93)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target.sroa.5.0.copyload, i64 noundef %86, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb70

bb42:                                             ; preds = %bb79
  %_75 = load i64, ptr %_74, align 8, !range !221, !noundef !3
  %94 = trunc nuw i64 %_75 to i1
  br i1 %94, label %bb3.i190, label %bb43

bb43:                                             ; preds = %bb42
  %95 = getelementptr inbounds nuw i8, ptr %_74, i64 8
  %rustflags.sroa.0.0.copyload = load i64, ptr %95, align 8
  %rustflags.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_74, i64 16
  %rustflags.sroa.5.0.copyload = load ptr, ptr %rustflags.sroa.5.0..sroa_idx, align 8
  %rustflags.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_74, i64 24
  %rustflags.sroa.8.0.copyload = load i64, ptr %rustflags.sroa.8.0..sroa_idx, align 8
  %_194 = icmp sgt i64 %rustflags.sroa.8.0.copyload, -1
  call void @llvm.assume(i1 %_194)
  %96 = icmp eq i64 %rustflags.sroa.8.0.copyload, 0
  br i1 %96, label %bb51, label %bb100

bb51:                                             ; preds = %bb43, %bb49
  %97 = icmp eq i64 %rustflags.sroa.0.0.copyload, 0
  br i1 %97, label %bb80, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb51
  %98 = icmp ne ptr %rustflags.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %98)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb80

cleanup16:                                        ; preds = %bb48, %bb46
  %99 = landingpad { ptr, i32 }
          cleanup
  %100 = icmp eq i64 %rustflags.sroa.0.0.copyload, 0
  br i1 %100, label %bb70, label %bb2.i.i.i4.i.i185

bb2.i.i.i4.i.i185:                                ; preds = %cleanup16
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustflags.sroa.5.0.copyload, i64 noundef %rustflags.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb70

bb100:                                            ; preds = %bb43
  %101 = icmp ne ptr %rustflags.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %101)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %iter)
  store i64 0, ptr %iter, align 8
  %_77.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %rustflags.sroa.8.0.copyload, ptr %_77.sroa.2.0.iter.sroa_idx, align 8
  %_77.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %rustflags.sroa.5.0.copyload, ptr %_77.sroa.3.0.iter.sroa_idx, align 8
  %_77.sroa.3.sroa.2.0._77.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store i64 %rustflags.sroa.8.0.copyload, ptr %_77.sroa.3.sroa.2.0._77.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_77.sroa.3.sroa.3.0._77.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 0, ptr %_77.sroa.3.sroa.3.0._77.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_77.sroa.3.sroa.4.0._77.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  store i64 %rustflags.sroa.8.0.copyload, ptr %_77.sroa.3.sroa.4.0._77.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_77.sroa.3.sroa.5.0._77.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 48
  store <2 x i32> splat (i32 31), ptr %_77.sroa.3.sroa.5.0._77.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_77.sroa.3.sroa.7.0._77.sroa.3.0.iter.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 56
  store i8 1, ptr %_77.sroa.3.sroa.7.0._77.sroa.3.0.iter.sroa_idx.sroa_idx, align 8
  %_77.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 64
  store i8 1, ptr %_77.sroa.4.0.iter.sroa_idx, align 8
  %_77.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 65
  store i8 0, ptr %_77.sroa.5.0.iter.sroa_idx, align 1
  br label %bb46

bb46:                                             ; preds = %bb48, %bb100
; invoke <core::str::iter::SplitInternal<char>>::next
  %102 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %iter)
          to label %bb101 unwind label %cleanup16

bb101:                                            ; preds = %bb46
  %103 = extractvalue { ptr, i64 } %102, 0
  %.not34 = icmp eq ptr %103, null
  br i1 %.not34, label %bb49, label %bb48

bb48:                                             ; preds = %bb101
  %104 = extractvalue { ptr, i64 } %102, 1
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %103, i64 noundef %104)
          to label %bb46 unwind label %cleanup16

bb49:                                             ; preds = %bb101
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %iter)
  br label %bb51

bb3.i190:                                         ; preds = %bb42
  call void @llvm.experimental.noalias.scope.decl(metadata !222)
  %105 = getelementptr inbounds nuw i8, ptr %_74, i64 8
  %.val.i = load i64, ptr %105, align 8, !alias.scope !222
  switch i64 %.val.i, label %bb1.sink.split.i [
    i64 -9223372036854775808, label %bb80
    i64 0, label %bb80
  ]

bb1.sink.split.i:                                 ; preds = %bb3.i190
  %106 = getelementptr inbounds nuw i8, ptr %_74, i64 16
  %.val3.i = load ptr, ptr %106, align 8, !alias.scope !222, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i, i64 noundef %.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !222
  br label %bb80

bb80:                                             ; preds = %bb51, %bb2.i.i.i4.i.i, %bb1.sink.split.i, %bb3.i190, %bb3.i190
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_74)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_85)
; invoke <std::process::Command>::status
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6status(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_85, ptr noalias noundef nonnull align 8 dereferenceable(200) %cmd)
          to label %bb53 unwind label %cleanup14

bb53:                                             ; preds = %bb80
  %107 = load i32, ptr %_85, align 8, !range !225, !noundef !3
  %108 = getelementptr inbounds nuw i8, ptr %_85, i64 4
  %status = load i32, ptr %108, align 4
  %109 = getelementptr inbounds nuw i8, ptr %_85, i64 8
  %_85.val80 = load ptr, ptr %109, align 8
  %110 = icmp eq i32 %107, 0
  br i1 %110, label %bb57, label %bb2.i192

bb2.i192:                                         ; preds = %bb53
  %111 = icmp ne ptr %_85.val80, null
  call void @llvm.assume(i1 %111)
  %bits.i.i.i.i.i = ptrtoint ptr %_85.val80 to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %bb57, !prof !28

bb2.i2.i.i.i.i:                                   ; preds = %bb2.i192
  %112 = getelementptr i8, ptr %_85.val80, i64 -1
  %113 = icmp ne ptr %112, null
  call void @llvm.assume(i1 %113)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %112, align 8
  %114 = getelementptr i8, ptr %_85.val80, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %114, align 8, !nonnull !3, !align !7, !noundef !3
  %115 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %115, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %116 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %116)
  invoke void %115(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %117 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %117)
  %118 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %119 = load i64, ptr %118, align 8, !range !8, !invariant.load !3
  %120 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %121 = load i64, ptr %120, align 8, !range !9, !invariant.load !3
  %122 = add i64 %121, -1
  %123 = icmp sgt i64 %122, -1
  call void @llvm.assume(i1 %123)
  %124 = icmp eq i64 %119, 0
  br i1 %124, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %119, i64 noundef range(i64 1, -9223372036854775807) %121) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %125 = landingpad { ptr, i32 }
          cleanup
  %126 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %127 = load i64, ptr %126, align 8, !range !8, !invariant.load !3
  %128 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %129 = load i64, ptr %128, align 8, !range !9, !invariant.load !3
  %130 = add i64 %129, -1
  %131 = icmp sgt i64 %130, -1
  call void @llvm.assume(i1 %131)
  %132 = icmp eq i64 %127, 0
  br i1 %132, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %127, i64 noundef range(i64 1, -9223372036854775807) %129) #17
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %112, i64 noundef 24, i64 noundef 8) #17
  br label %bb70

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %112, i64 noundef 24, i64 noundef 8) #17
  br label %bb57

bb57:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i, %bb2.i192, %bb53
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_85)
; invoke std::sys::fs::remove_dir_all
  %_0.i198 = invoke noundef ptr @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs14remove_dir_all(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i160, i64 noundef %_2.val1.i.i161)
          to label %bb58 unwind label %cleanup14

bb58:                                             ; preds = %bb57
  %.not38 = icmp eq ptr %_0.i198, null
  br i1 %.not38, label %bb83, label %bb59

bb59:                                             ; preds = %bb58
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err3)
  store ptr %_0.i198, ptr %err3, align 8
; call <std::io::error::Error>::kind
  %_92 = call fastcc noundef i8 @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4kind(ptr nonnull %_0.i198)
  %133 = icmp eq i8 %_92, 0
  br i1 %133, label %bb61, label %bb62

bb83:                                             ; preds = %bb64, %bb58
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %cmd)
          to label %bb65 unwind label %bb93.thread275

cleanup18:                                        ; preds = %bb63, %bb62
  %134 = landingpad { ptr, i32 }
          cleanup
  %err3.val = load ptr, ptr %err3, align 8, !nonnull !3, !noundef !3
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECs7AmXS38G9s1_18build_script_build(ptr nonnull %err3.val) #18
          to label %bb70 unwind label %terminate

bb61:                                             ; preds = %bb59
  %bits.i.i.i.i199 = ptrtoint ptr %_0.i198 to i64
  %_5.i.i.i.i200 = and i64 %bits.i.i.i.i199, 3
  %switch.i.i.i201 = icmp eq i64 %_5.i.i.i.i200, 1
  br i1 %switch.i.i.i201, label %bb2.i2.i.i.i202, label %bb64, !prof !28

bb2.i2.i.i.i202:                                  ; preds = %bb61
  %135 = getelementptr i8, ptr %_0.i198, i64 -1
  %136 = icmp ne ptr %135, null
  call void @llvm.assume(i1 %136)
  %_6.val.i.i.i.i.i203 = load ptr, ptr %135, align 8
  %137 = getelementptr i8, ptr %_0.i198, i64 7
  %_6.val1.i.i.i.i.i204 = load ptr, ptr %137, align 8, !nonnull !3, !align !7, !noundef !3
  %138 = load ptr, ptr %_6.val1.i.i.i.i.i204, align 8, !invariant.load !3
  %.not.i.i.i.i.i.i.i205 = icmp eq ptr %138, null
  br i1 %.not.i.i.i.i.i.i.i205, label %bb3.i.i.i.i.i.i.i210, label %is_not_null.i.i.i.i.i.i.i206

is_not_null.i.i.i.i.i.i.i206:                     ; preds = %bb2.i2.i.i.i202
  %139 = icmp ne ptr %_6.val.i.i.i.i.i203, null
  call void @llvm.assume(i1 %139)
  invoke void %138(ptr noundef nonnull %_6.val.i.i.i.i.i203)
          to label %bb3.i.i.i.i.i.i.i210 unwind label %cleanup.i.i.i.i.i.i.i207

bb3.i.i.i.i.i.i.i210:                             ; preds = %is_not_null.i.i.i.i.i.i.i206, %bb2.i2.i.i.i202
  %140 = icmp ne ptr %_6.val.i.i.i.i.i203, null
  call void @llvm.assume(i1 %140)
  %141 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i204, i64 8
  %142 = load i64, ptr %141, align 8, !range !8, !invariant.load !3
  %143 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i204, i64 16
  %144 = load i64, ptr %143, align 8, !range !9, !invariant.load !3
  %145 = add i64 %144, -1
  %146 = icmp sgt i64 %145, -1
  call void @llvm.assume(i1 %146)
  %147 = icmp eq i64 %142, 0
  br i1 %147, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i212, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i211

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i211: ; preds = %bb3.i.i.i.i.i.i.i210
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i203, i64 noundef %142, i64 noundef range(i64 1, -9223372036854775807) %144) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i212

cleanup.i.i.i.i.i.i.i207:                         ; preds = %is_not_null.i.i.i.i.i.i.i206
  %148 = landingpad { ptr, i32 }
          cleanup
  %149 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i204, i64 8
  %150 = load i64, ptr %149, align 8, !range !8, !invariant.load !3
  %151 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i204, i64 16
  %152 = load i64, ptr %151, align 8, !range !9, !invariant.load !3
  %153 = add i64 %152, -1
  %154 = icmp sgt i64 %153, -1
  call void @llvm.assume(i1 %154)
  %155 = icmp eq i64 %150, 0
  br i1 %155, label %bb1.i.i.i.i.i209, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i208

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i208: ; preds = %cleanup.i.i.i.i.i.i.i207
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i203, i64 noundef %150, i64 noundef range(i64 1, -9223372036854775807) %152) #17
  br label %bb1.i.i.i.i.i209

bb1.i.i.i.i.i209:                                 ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i208, %cleanup.i.i.i.i.i.i.i207
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %135, i64 noundef 24, i64 noundef 8) #17
  br label %bb70

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i212: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i211, %bb3.i.i.i.i.i.i.i210
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %135, i64 noundef 24, i64 noundef 8) #17
  br label %bb64

bb62:                                             ; preds = %bb59
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_97)
  store ptr %_2.val.i.i160, ptr %_97, align 8
  %156 = getelementptr inbounds nuw i8, ptr %_97, i64 8
  store i64 %_2.val1.i.i161, ptr %156, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args4)
  store ptr %_97, ptr %args4, align 8
  %_100.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args4, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_100.sroa.4.0..sroa_idx, align 8
  %157 = getelementptr inbounds nuw i8, ptr %args4, i64 16
  store ptr %err3, ptr %157, align 8
  %_101.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args4, i64 24
  store ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_101.sroa.4.0..sroa_idx, align 8
; invoke std::io::stdio::_eprint
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_62e6ec0e1c3bfea4ae2f14deaee8dee9, ptr noundef nonnull %args4)
          to label %bb63 unwind label %cleanup18

bb64:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i212, %bb61
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err3)
  br label %bb83

bb63:                                             ; preds = %bb62
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_97)
; invoke std::process::exit
  invoke void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21
          to label %unreachable unwind label %cleanup18

bb65:                                             ; preds = %bb83
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %cmd)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %rustc2)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %probefile)
  %out_subdir.val69 = load i64, ptr %out_subdir, align 8
  %158 = icmp eq i64 %out_subdir.val69, 0
  br i1 %158, label %bb66, label %bb2.i.i.i4.i.i.i.i217

bb2.i.i.i4.i.i.i.i217:                            ; preds = %bb65
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val.i.i160, i64 noundef %out_subdir.val69, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb66

bb66:                                             ; preds = %bb2.i.i.i4.i.i.i.i217, %bb65
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %out_subdir)
  %159 = icmp eq i64 %6, 0
  br i1 %159, label %bb67, label %bb2.i.i.i4.i.i.i219

bb2.i.i.i4.i.i.i219:                              ; preds = %bb66
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %out_dir.sroa.5.0.copyload, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb67

bb67:                                             ; preds = %bb2.i.i.i4.i.i.i219, %bb66
  %160 = trunc nuw i32 %107 to i1
  %.not37 = icmp eq i32 %status, 0
  %not. = xor i1 %160, true
  %success.sroa.0.0.off0 = select i1 %not., i1 %.not37, i1 false
  ret i1 %success.sroa.0.0.off0

bb88:                                             ; preds = %bb2.i.i.i4.i.i.i.i133, %cleanup.i
; call core::ptr::drop_in_place::<core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::option::IntoIter<std::ffi::os_str::OsString>, core::option::IntoIter<std::ffi::os_str::OsString>>, core::iter::sources::once::Once<std::ffi::os_str::OsString>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %rustc2) #18
  br label %bb92

bb90:                                             ; preds = %bb18
  %161 = landingpad { ptr, i32 }
          cleanup
  switch i64 %rustc_wrapper.sroa.0.0, label %bb2.i.i.i4.i.i.i.i221 [
    i64 -9223372036854775808, label %bb92
    i64 0, label %bb92
  ]

bb2.i.i.i4.i.i.i.i221:                            ; preds = %bb90
  %162 = icmp ne ptr %rustc_wrapper.sroa.7.0, null
  call void @llvm.assume(i1 %162)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_wrapper.sroa.7.0, i64 noundef %rustc_wrapper.sroa.0.0, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb92

bb92:                                             ; preds = %bb88, %cleanup10, %bb1.i.i.i.i.i, %cleanup11, %bb2.i.i.i4.i.i.i.i221, %bb90, %bb90, %bb93
  %.pn50245 = phi { ptr, i32 } [ %.pn45, %bb93 ], [ %161, %bb90 ], [ %161, %bb90 ], [ %161, %bb2.i.i.i4.i.i.i.i221 ], [ %29, %cleanup11 ], [ %25, %cleanup10 ], [ %43, %bb1.i.i.i.i.i ], [ %64, %bb88 ]
  %_108.sroa.0.5.off0244 = phi i1 [ false, %bb93 ], [ true, %bb90 ], [ true, %bb90 ], [ true, %bb2.i.i.i4.i.i.i.i221 ], [ true, %cleanup11 ], [ true, %cleanup10 ], [ true, %bb1.i.i.i.i.i ], [ false, %bb88 ]
  %probefile.val = load i64, ptr %probefile, align 8
  %163 = icmp eq i64 %probefile.val, 0
  br i1 %163, label %bb75, label %bb2.i.i.i4.i.i.i.i222

bb2.i.i.i4.i.i.i.i222:                            ; preds = %bb92
  %164 = getelementptr inbounds nuw i8, ptr %probefile, i64 8
  %probefile.val66 = load ptr, ptr %164, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %probefile.val66, i64 noundef %probefile.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb75

bb77:                                             ; preds = %bb2.i.i.i4.i.i.i224, %bb94, %bb95
  %.pn50.pn.pn.pn239 = phi { ptr, i32 } [ %.pn50.pn.pn, %bb95 ], [ %.pn50.pn.pn.pn240, %bb94 ], [ %.pn50.pn.pn.pn240, %bb2.i.i.i4.i.i.i224 ]
  resume { ptr, i32 } %.pn50.pn.pn.pn239

bb94:                                             ; preds = %bb95.thread, %bb95
  %.pn50.pn.pn.pn240 = phi { ptr, i32 } [ %7, %bb95.thread ], [ %.pn50.pn.pn, %bb95 ]
  %165 = icmp eq i64 %4, 0
  br i1 %165, label %bb77, label %bb2.i.i.i4.i.i.i224

bb2.i.i.i4.i.i.i224:                              ; preds = %bb94
  %166 = icmp ne ptr %rustc.sroa.5.0.copyload227, null
  call void @llvm.assume(i1 %166)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.5.0.copyload227, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb77
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCs7AmXS38G9s1_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i36 = alloca [24 x i8], align 8
  %_2.i = alloca [24 x i8], align 8
  %_4.i17 = alloca [24 x i8], align 8
  %_4.i = alloca [24 x i8], align 8
  %_2.i10.i = alloca [200 x i8], align 8
  %_7.i.i = alloca [16 x i8], align 8
  %_2.i.i = alloca [24 x i8], align 8
  %key.i.i = alloca [16 x i8], align 8
  %pieces.i = alloca [72 x i8], align 8
  %_12.i = alloca [24 x i8], align 8
  %_8.i = alloca [200 x i8], align 8
  %_5.i = alloca [56 x i8], align 8
  %output.i = alloca [56 x i8], align 8
  %_54 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i.i)
  store ptr @alloc_806c1ac911172019779ceab530bc1f0e, ptr %key.i.i, align 8, !noalias !226
  %0 = getelementptr inbounds nuw i8, ptr %key.i.i, i64 8
  store i64 5, ptr %0, align 8, !noalias !226
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !226
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5), !noalias !230
  %1 = load i64, ptr %_2.i.i, align 8, !range !41, !noalias !226, !noundef !3
  %.not.i.i = icmp eq i64 %1, -9223372036854775808
  br i1 %.not.i.i, label %bb3.i.i, label %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit.i

bb3.i.i:                                          ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i.i), !noalias !226
  store ptr %key.i.i, ptr %_7.i.i, align 8, !noalias !226
  %_8.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_7.i.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs7AmXS38G9s1_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i.i, align 8, !noalias !226
; call std::io::stdio::_eprint
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i.i), !noalias !230
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i.i), !noalias !226
; call std::process::exit
  call void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #21, !noalias !230
  unreachable

_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit.i: ; preds = %start
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

cleanup.i.i:                                      ; preds = %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit.i
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %1, 0
  br i1 %3, label %common.resume, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !232
  br label %common.resume

bb2.i.i:                                          ; preds = %_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var.exit.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noundef nonnull align 8 dereferenceable(200) %_2.i10.i, i64 200, i1 false), !noalias !236
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i10.i), !noalias !232
  %4 = icmp eq i64 %1, 0
  br i1 %4, label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build.exit.i, label %bb2.i.i.i4.i.i.i6.i.i

bb2.i.i.i4.i.i.i6.i.i:                            ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !232
  br label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build.exit.i

common.resume:                                    ; preds = %cleanup, %bb2.i.i.i4.i.i.i26, %cleanup.i.i, %bb2.i.i.i4.i.i.i.i.i, %cleanup.body.i, %cleanup1.i
  %common.resume.op = phi { ptr, i32 } [ %2, %bb2.i.i.i4.i.i.i.i.i ], [ %2, %cleanup.i.i ], [ %30, %cleanup1.i ], [ %eh.lpad-body.i, %cleanup.body.i ], [ %73, %bb2.i.i.i4.i.i.i26 ], [ %73, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build.exit.i: ; preds = %bb2.i.i.i4.i.i.i6.i.i, %bb2.i.i
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a887f9858119cc7413062dc002c4d9ab, i64 noundef 9)
          to label %bb3.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb3.i, %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build.exit.i
  %5 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %5, %cleanup.i ], [ %22, %bb1.i.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i) #18
          to label %common.resume unwind label %terminate.i

bb3.i:                                            ; preds = %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build.exit.i
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i)
          to label %bb4.i unwind label %cleanup.i

bb4.i:                                            ; preds = %bb3.i
  %6 = load i64, ptr %_5.i, align 8, !range !41, !noundef !3
  %7 = icmp eq i64 %6, -9223372036854775808
  br i1 %7, label %bb3.i13.i, label %bb25.i

bb3.i13.i:                                        ; preds = %bb4.i
  call void @llvm.experimental.noalias.scope.decl(metadata !237)
  %8 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %.val.i.i = load ptr, ptr %8, align 8, !alias.scope !237, !nonnull !3, !noundef !3
  %bits.i.i.i.i.i.i = ptrtoint ptr %.val.i.i to i64
  %_5.i.i.i.i.i.i = and i64 %bits.i.i.i.i.i.i, 3
  %switch.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i.i, label %bb2.i2.i.i.i.i.i, label %bb24.i, !prof !28

bb2.i2.i.i.i.i.i:                                 ; preds = %bb3.i13.i
  %9 = getelementptr i8, ptr %.val.i.i, i64 -1
  %10 = icmp ne ptr %9, null
  call void @llvm.assume(i1 %10)
  %_6.val.i.i.i.i.i.i.i = load ptr, ptr %9, align 8, !noalias !237
  %11 = getelementptr i8, ptr %.val.i.i, i64 7
  %_6.val1.i.i.i.i.i.i.i = load ptr, ptr %11, align 8, !noalias !237, !nonnull !3, !align !7, !noundef !3
  %12 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i, align 8, !invariant.load !3, !noalias !237
  %.not.i.i.i.i.i.i.i.i.i = icmp eq ptr %12, null
  br i1 %.not.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i2.i.i.i.i.i
  %13 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %13)
  invoke void %12(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i, !noalias !237

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i
  %14 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %14)
  %15 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !8, !invariant.load !3, !noalias !237
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !9, !invariant.load !3, !noalias !237
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #17, !noalias !237
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i:                        ; preds = %is_not_null.i.i.i.i.i.i.i.i.i
  %22 = landingpad { ptr, i32 }
          cleanup
  %23 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %24 = load i64, ptr %23, align 8, !range !8, !invariant.load !3, !noalias !237
  %25 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %26 = load i64, ptr %25, align 8, !range !9, !invariant.load !3, !noalias !237
  %27 = add i64 %26, -1
  %28 = icmp sgt i64 %27, -1
  call void @llvm.assume(i1 %28)
  %29 = icmp eq i64 %24, 0
  br i1 %29, label %bb1.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %24, i64 noundef range(i64 1, -9223372036854775807) %26) #17, !noalias !237
  br label %bb1.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #17, !noalias !237
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #17, !noalias !237
  br label %bb24.i

bb25.i:                                           ; preds = %bb4.i
  %_24.sroa.4.0._5.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %val.sroa.4.0.output.sroa_idx.i = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val.sroa.4.0.output.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_24.sroa.4.0._5.sroa_idx.i, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
  store i64 %6, ptr %output.i, align 8
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
          to label %bb6.i unwind label %cleanup1.i

bb24.i:                                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs7AmXS38G9s1_18build_script_build.exit.i.i.i.i.i.i, %bb3.i13.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
; call core::ptr::drop_in_place::<std::process::Command>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_8.i)
  br label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread

cleanup1.i:                                       ; preds = %bb9.i, %bb28.i, %bb6.i, %bb25.i
  %30 = landingpad { ptr, i32 }
          cleanup
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i) #18
  br label %common.resume

bb6.i:                                            ; preds = %bb25.i
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_8.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_12.i)
  %_31.i = load ptr, ptr %val.sroa.4.0.output.sroa_idx.i, align 8, !nonnull !3, !noundef !3
  %31 = getelementptr inbounds nuw i8, ptr %output.i, i64 16
  %_30.i = load i64, ptr %31, align 8, !noundef !3
; invoke core::str::converts::from_utf8
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_12.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_31.i, i64 noundef %_30.i)
          to label %bb7.i unwind label %cleanup1.i

bb7.i:                                            ; preds = %bb6.i
  %_32.i = load i64, ptr %_12.i, align 8, !range !221, !noundef !3
  %32 = trunc nuw i64 %_32.i to i1
  br i1 %32, label %bb26.i, label %bb28.i

bb26.i:                                           ; preds = %bb7.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12.i)
  br label %bb12.i

bb28.i:                                           ; preds = %bb7.i
  %33 = getelementptr inbounds nuw i8, ptr %_12.i, i64 8
  %_33.0.i = load ptr, ptr %33, align 8, !nonnull !3, !align !25, !noundef !3
  %34 = getelementptr inbounds nuw i8, ptr %_12.i, i64 16
  %_33.1.i = load i64, ptr %34, align 8, !noundef !3
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
  %35 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb29.i unwind label %cleanup1.i

bb29.i:                                           ; preds = %bb28.i
  %36 = extractvalue { ptr, i64 } %35, 0
  %.not7.i = icmp ne ptr %36, null
  %37 = extractvalue { ptr, i64 } %35, 1
  %_3.not.i.i = icmp eq i64 %37, 7
  %or.cond52.i = select i1 %.not7.i, i1 %_3.not.i.i, i1 false
  br i1 %or.cond52.i, label %bb32.i, label %bb8.i

bb8.i:                                            ; preds = %bb32.i, %bb29.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  br label %bb12.i

bb32.i:                                           ; preds = %bb29.i
  %38 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %36, ptr noundef nonnull dereferenceable(7) @alloc_ca36d7e792bb4bbd1a68749f90007ce8, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !240
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %bb9.i, label %bb8.i

bb9.i:                                            ; preds = %bb32.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %40 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
          to label %bb33.i unwind label %cleanup1.i

bb33.i:                                           ; preds = %bb9.i
  %41 = extractvalue { ptr, i64 } %40, 0
  %.not8.i = icmp eq ptr %41, null
  br i1 %.not8.i, label %bb34.i, label %bb35.i

bb35.i:                                           ; preds = %bb33.i
  %42 = extractvalue { ptr, i64 } %40, 1
  switch i64 %42, label %bb9thread-pre-split.i.i [
    i64 0, label %bb36.i
    i64 1, label %bb7.i.i
  ]

bb7.i.i:                                          ; preds = %bb35.i
  %43 = load i8, ptr %41, align 1, !alias.scope !244, !noundef !3
  switch i8 %43, label %bb9.i.i [
    i8 43, label %bb36.i
    i8 45, label %bb36.i
  ]

bb9thread-pre-split.i.i:                          ; preds = %bb35.i
  %.pr.i.i = load i8, ptr %41, align 1, !alias.scope !244
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb9thread-pre-split.i.i, %bb7.i.i
  %44 = phi i8 [ %.pr.i.i, %bb9thread-pre-split.i.i ], [ %43, %bb7.i.i ]
  %cond.i.i = icmp eq i8 %44, 43
  %rest.1.i.i = sext i1 %cond.i.i to i64
  %src.sroa.15.0.i.i = add nsw i64 %42, %rest.1.i.i
  %src.sroa.0.0.idx.i.i = zext i1 %cond.i.i to i64
  %src.sroa.0.0.i.i = getelementptr inbounds nuw i8, ptr %41, i64 %src.sroa.0.0.idx.i.i
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
  %45 = call { i32, i1 } @llvm.umul.with.overflow.i32(i32 %result.sroa.0.0.i.i, i32 10)
  %_57.1.i.i = extractvalue { i32, i1 } %45, 1
  br i1 %_57.1.i.i, label %bb36.i, label %bb33.i.i, !prof !247

bb33.i.i:                                         ; preds = %bb23.i.i
  %_57.0.i.i = extractvalue { i32, i1 } %45, 0
  %rest.12.i.i = add nsw i64 %src.sroa.15.1.i.i, -1
  %rest.01.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i, i64 1
  %46 = load i8, ptr %src.sroa.0.1.i.i, align 1, !alias.scope !244, !noundef !3
  %47 = zext i8 %46 to i32
  %48 = add nsw i32 %47, -48
  %_14.i.i.i = icmp ugt i32 %48, 9
  %_60.0.i.i = add i32 %48, %_57.0.i.i
  %_60.1.i.i = icmp ult i32 %_60.0.i.i, %_57.0.i.i
  %or.cond.i = select i1 %_14.i.i.i, i1 true, i1 %_60.1.i.i
  br i1 %or.cond.i, label %bb36.i, label %bb22.i.i, !prof !248

bb16.i.i:                                         ; preds = %bb15.preheader.i.i, %bb20.i.i
  %src.sroa.0.259.i.i = phi ptr [ %rest.04.i.i, %bb20.i.i ], [ %src.sroa.0.0.i.i, %bb15.preheader.i.i ]
  %src.sroa.15.258.i.i = phi i64 [ %rest.15.i.i, %bb20.i.i ], [ %src.sroa.15.0.i.i, %bb15.preheader.i.i ]
  %result.sroa.0.257.i.i = phi i32 [ %51, %bb20.i.i ], [ 0, %bb15.preheader.i.i ]
  %_19.i.i = load i8, ptr %src.sroa.0.259.i.i, align 1, !alias.scope !244, !noundef !3
  %_18.i.i = zext i8 %_19.i.i to i32
  %49 = add nsw i32 %_18.i.i, -48
  %_14.i47.i.i = icmp ugt i32 %49, 9
  br i1 %_14.i47.i.i, label %bb36.i, label %bb20.i.i

bb20.i.i:                                         ; preds = %bb16.i.i
  %50 = mul i32 %result.sroa.0.257.i.i, 10
  %rest.15.i.i = add nsw i64 %src.sroa.15.258.i.i, -1
  %rest.04.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.259.i.i, i64 1
  %51 = add i32 %49, %50
  %_13.not.i.i = icmp eq i64 %rest.15.i.i, 0
  br i1 %_13.not.i.i, label %bb36.i, label %bb16.i.i

bb34.i:                                           ; preds = %bb33.i
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !249)
  %52 = icmp eq i64 %6, 0
  br i1 %52, label %bb4.i.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb34.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !249
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i, %bb34.i
  %53 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %53, align 8, !alias.scope !249
  %54 = icmp eq i64 %.val2.i.i, 0
  br i1 %54, label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i.i
  %55 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %55, align 8, !alias.scope !249, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !249
  br label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread

bb36.i:                                           ; preds = %bb33.i.i, %bb23.i.i, %bb22.i.i, %bb20.i.i, %bb16.i.i, %bb15.preheader.i.i, %bb7.i.i, %bb7.i.i, %bb35.i
  %not._0.sroa.0.0.i.i = phi i32 [ 0, %bb15.preheader.i.i ], [ -1, %bb7.i.i ], [ -1, %bb7.i.i ], [ -1, %bb35.i ], [ %51, %bb20.i.i ], [ -1, %bb16.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ], [ -1, %bb23.i.i ], [ -1, %bb33.i.i ]
  %56 = phi i32 [ 0, %bb15.preheader.i.i ], [ undef, %bb7.i.i ], [ undef, %bb7.i.i ], [ undef, %bb35.i ], [ %51, %bb20.i.i ], [ undef, %bb16.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ], [ undef, %bb23.i.i ], [ undef, %bb33.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !252)
  %57 = icmp eq i64 %6, 0
  br i1 %57, label %bb4.i18.i, label %bb2.i.i.i4.i.i16.i

bb2.i.i.i4.i.i16.i:                               ; preds = %bb36.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !252
  br label %bb4.i18.i

bb4.i18.i:                                        ; preds = %bb2.i.i.i4.i.i16.i, %bb36.i
  %58 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i19.i = load i64, ptr %58, align 8, !alias.scope !252
  %59 = icmp eq i64 %.val2.i19.i, 0
  br i1 %59, label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit, label %bb2.i.i.i4.i7.i20.i

bb2.i.i.i4.i7.i20.i:                              ; preds = %bb4.i18.i
  %60 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i21.i = load ptr, ptr %60, align 8, !alias.scope !252, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i21.i, i64 noundef %.val2.i19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !252
  br label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit

bb12.i:                                           ; preds = %bb8.i, %bb26.i
  call void @llvm.experimental.noalias.scope.decl(metadata !255)
  %61 = icmp eq i64 %6, 0
  br i1 %61, label %bb4.i26.i, label %bb2.i.i.i4.i.i24.i

bb2.i.i.i4.i.i24.i:                               ; preds = %bb12.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !255
  br label %bb4.i26.i

bb4.i26.i:                                        ; preds = %bb2.i.i.i4.i.i24.i, %bb12.i
  %62 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i27.i = load i64, ptr %62, align 8, !alias.scope !255
  %63 = icmp eq i64 %.val2.i27.i, 0
  br i1 %63, label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i28.i

bb2.i.i.i4.i7.i28.i:                              ; preds = %bb4.i26.i
  %64 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i29.i = load ptr, ptr %64, align 8, !alias.scope !255, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i29.i, i64 noundef %.val2.i27.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !255
  br label %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread

terminate.i:                                      ; preds = %cleanup.body.i
  %65 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #19
  unreachable

_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread: ; preds = %bb2.i.i.i4.i7.i28.i, %bb4.i26.i, %bb24.i, %bb4.i.i, %bb2.i.i.i4.i7.i.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br label %bb28.thread64

_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit: ; preds = %bb4.i18.i, %bb2.i.i.i4.i7.i20.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  %_3 = icmp ugt i32 %not._0.sroa.0.0.i.i, 79
  br i1 %_3, label %bb28.thread64, label %bb20

bb28.thread64:                                    ; preds = %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit, %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread
  %rustc.sroa.0.049 = phi i32 [ -1, %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit.thread ], [ %not._0.sroa.0.0.i.i, %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit ]
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_3f5ffa2c8843a87c929224eb38f9612c, ptr noundef nonnull inttoptr (i64 71 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_10e1a7b545f0c4f1d59070b08c5db106, ptr noundef nonnull inttoptr (i64 87 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34d41541334731d13af8ae8372fb9cba, ptr noundef nonnull inttoptr (i64 107 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_031bcea5e65ed731845949db9e0ab153, ptr noundef nonnull inttoptr (i64 95 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_31141e1830579c4679b90d7c535c23eb, ptr noundef nonnull inttoptr (i64 85 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_700cd7122563efc652176cd3deabdf94, ptr noundef nonnull inttoptr (i64 87 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8b4555c431240d0c3945d5158c0cca26, ptr noundef nonnull inttoptr (i64 97 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_d4d6a4c8a8ec3eac6d741e3b835b1484, ptr noundef nonnull inttoptr (i64 105 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_d0e119f2301624664874e5e9731914f8, ptr noundef nonnull inttoptr (i64 97 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_6820b661dd9cc52746977e525dd81c84, ptr noundef nonnull inttoptr (i64 101 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_f64296080ab33db35230262cec8954d5, ptr noundef nonnull inttoptr (i64 109 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_b7f5f0853ea12bb6490a947873f2b916, ptr noundef nonnull inttoptr (i64 105 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_0e4bcf46f47e70282bb1af6e3f94cfe4, ptr noundef nonnull inttoptr (i64 89 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_306832e19bb2d9d2c4f3f53ed7f1b42e, ptr noundef nonnull inttoptr (i64 85 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_7a9a2e1a912491d88aa3cacea2f24883, ptr noundef nonnull inttoptr (i64 85 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_26b0ac36ec7a3963a878d23341283334, ptr noundef nonnull inttoptr (i64 87 to ptr))
  br label %bb33

bb20:                                             ; preds = %_RNvCs7AmXS38G9s1_18build_script_build19rustc_minor_version.exit
  %_37 = icmp ult i32 %56, 57
  br i1 %_37, label %bb24.thread, label %bb24

bb24.thread:                                      ; preds = %bb20
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_db1a0c96a1ddd2b29c00e1bd77fa5a95, ptr noundef nonnull inttoptr (i64 65 to ptr))
  br label %bb28.thread

bb24:                                             ; preds = %bb20
  %_41 = icmp ult i32 %56, 66
  br i1 %_41, label %bb28.thread, label %bb28

bb28.thread:                                      ; preds = %bb24, %bb24.thread
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c75a26d14bafba6eaea018555e51fe08, ptr noundef nonnull inttoptr (i64 63 to ptr))
  br label %bb29

bb28:                                             ; preds = %bb24
  %_45 = icmp ult i32 %56, 79
  br i1 %_45, label %bb29, label %bb33

bb29:                                             ; preds = %bb28.thread, %bb28
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_feb3714f38a5fb28df83bc12bd963cd2, ptr noundef nonnull inttoptr (i64 85 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_2db9ba60cb7f510177111c1d84e78c0f, ptr noundef nonnull inttoptr (i64 73 to ptr))
  br label %bb33

bb33:                                             ; preds = %bb28.thread64, %bb28, %bb29
  %rustc.sroa.0.048525562 = phi i32 [ 79, %bb28 ], [ %56, %bb29 ], [ %rustc.sroa.0.049, %bb28.thread64 ]
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11)
  %66 = load i64, ptr %_4.i, align 8, !range !41, !noundef !3
  switch i64 %66, label %bb2.i.i.i4.i.i.i.i.i16 [
    i64 -9223372036854775808, label %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit
    i64 0, label %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit.thread
  ]

bb2.i.i.i4.i.i.i.i.i16:                           ; preds = %bb33
  %67 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_4.val3.i = load ptr, ptr %67, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.val3.i, i64 noundef %66, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit.thread

_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit.thread: ; preds = %bb33, %bb2.i.i.i4.i.i.i.i.i16
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i)
  br label %bb36

_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit: ; preds = %bb33
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i)
; call build_script_build::do_compile_probe
  %68 = call fastcc noundef zeroext i1 @_RNvCs7AmXS38G9s1_18build_script_build16do_compile_probe(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1c86b039f881a10c46f4717c27a32d0e, i64 noundef 15, i1 noundef zeroext false)
  br i1 %68, label %bb45.thread, label %bb36

bb45.thread:                                      ; preds = %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_36f02c749ad09a7e3496c0b77a2eed14, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_91482d60220a3de858c1d085837d0ff7, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_99296b26f853c0e00fd05857e3136e29, ptr noundef nonnull inttoptr (i64 83 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34ae0dc4ffa16faaed57f3cbab572bbd, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br label %bb66

bb36:                                             ; preds = %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit.thread, %_RNvCs7AmXS38G9s1_18build_script_build22compile_probe_unstable.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_54)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_54, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d563101362ed4a06747b9210d55c4c5b, i64 noundef 15)
  %69 = load i64, ptr %_54, align 8, !range !41, !noundef !3
  %.not = icmp eq i64 %69, -9223372036854775808
  br i1 %.not, label %bb50.sink.split, label %bb38

bb38:                                             ; preds = %bb36
  %rustc_bootstrap.sroa.5.0._54.sroa_idx = getelementptr inbounds nuw i8, ptr %_54, i64 8
  %rustc_bootstrap.sroa.5.0.copyload = load ptr, ptr %rustc_bootstrap.sroa.5.0._54.sroa_idx, align 8
  %rustc_bootstrap.sroa.8.0._54.sroa_idx = getelementptr inbounds nuw i8, ptr %_54, i64 16
  %rustc_bootstrap.sroa.8.0.copyload = load i64, ptr %rustc_bootstrap.sroa.8.0._54.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i17)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_4.i17, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11)
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb38
  %70 = load i64, ptr %_4.i17, align 8, !range !41, !noundef !3
  switch i64 %70, label %bb2.i.i.i4.i.i.i.i.i22 [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i21
    i64 0, label %bb39.thread
  ]

bb2.i.i.i4.i.i.i.i.i22:                           ; preds = %.noexc
  %71 = getelementptr inbounds nuw i8, ptr %_4.i17, i64 8
  %_4.val3.i23 = load ptr, ptr %71, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.val3.i23, i64 noundef %70, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %bb39.thread

bb39.thread:                                      ; preds = %.noexc, %bb2.i.i.i4.i.i.i.i.i22
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i17)
  br label %bb41

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i21: ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i17)
; invoke build_script_build::do_compile_probe
  %72 = invoke fastcc noundef zeroext i1 @_RNvCs7AmXS38G9s1_18build_script_build16do_compile_probe(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1c86b039f881a10c46f4717c27a32d0e, i64 noundef 15, i1 noundef zeroext true)
          to label %bb39 unwind label %cleanup

cleanup:                                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i21, %bb38
  %73 = landingpad { ptr, i32 }
          cleanup
  %74 = icmp eq i64 %69, 0
  br i1 %74, label %common.resume, label %bb2.i.i.i4.i.i.i26

bb2.i.i.i4.i.i.i26:                               ; preds = %cleanup
  %75 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %75)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %69, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %common.resume

bb39:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs7AmXS38G9s1_18build_script_build.exit5.i21
  br i1 %72, label %bb42.thread, label %bb41

bb41:                                             ; preds = %bb39.thread, %bb39
  %76 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %76)
  %_3.not.i = icmp eq i64 %rustc_bootstrap.sroa.8.0.copyload, 1
  br i1 %_3.not.i, label %bb2.i, label %bb42

bb2.i:                                            ; preds = %bb41
  %lhsc = load i8, ptr %rustc_bootstrap.sroa.5.0.copyload, align 1
  %77 = icmp ne i8 %lhsc, 49
  br label %bb42

bb42:                                             ; preds = %bb41, %bb2.i
  %consider_rustc_bootstrap.sroa.0.0.off0 = phi i1 [ %77, %bb2.i ], [ true, %bb41 ]
  %78 = icmp eq i64 %69, 0
  br i1 %78, label %bb50.sink.split, label %bb2.i.i.i4.i.i.i29

bb42.thread:                                      ; preds = %bb39
  %79 = icmp eq i64 %69, 0
  br i1 %79, label %bb45.thread111, label %bb2.i.i.i4.i.i.i29

bb45.thread111:                                   ; preds = %bb42.thread
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_54)
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_36f02c749ad09a7e3496c0b77a2eed14, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_91482d60220a3de858c1d085837d0ff7, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_99296b26f853c0e00fd05857e3136e29, ptr noundef nonnull inttoptr (i64 83 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34ae0dc4ffa16faaed57f3cbab572bbd, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br label %bb64

bb2.i.i.i4.i.i.i29:                               ; preds = %bb42.thread, %bb42
  %consider_rustc_bootstrap.sroa.0.0.off0108 = phi i1 [ true, %bb42.thread ], [ %consider_rustc_bootstrap.sroa.0.0.off0, %bb42 ]
  %proc_macro_span.sroa.0.0.off0106 = phi i1 [ true, %bb42.thread ], [ false, %bb42 ]
  %80 = icmp ne ptr %rustc_bootstrap.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %80)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc_bootstrap.sroa.5.0.copyload, i64 noundef %69, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_54)
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_36f02c749ad09a7e3496c0b77a2eed14, ptr noundef nonnull inttoptr (i64 65 to ptr))
  br i1 %proc_macro_span.sroa.0.0.off0106, label %bb56, label %bb50

bb50.sink.split:                                  ; preds = %bb42, %bb36
  %consider_rustc_bootstrap.sroa.0.1.off081.ph = phi i1 [ true, %bb36 ], [ %consider_rustc_bootstrap.sroa.0.0.off0, %bb42 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_54)
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_36f02c749ad09a7e3496c0b77a2eed14, ptr noundef nonnull inttoptr (i64 65 to ptr))
  br label %bb50

bb50:                                             ; preds = %bb50.sink.split, %bb2.i.i.i4.i.i.i29
  %consider_rustc_bootstrap.sroa.0.1.off081 = phi i1 [ %consider_rustc_bootstrap.sroa.0.0.off0108, %bb2.i.i.i4.i.i.i29 ], [ %consider_rustc_bootstrap.sroa.0.1.off081.ph, %bb50.sink.split ]
  %_65 = icmp ugt i32 %rustc.sroa.0.048525562, 87
  br i1 %_65, label %bb51, label %bb63

bb51:                                             ; preds = %bb50
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i), !noalias !258
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11), !noalias !258
  %81 = load i64, ptr %_2.i, align 8, !range !41, !noalias !258, !noundef !3
  switch i64 %81, label %bb2.i.i.i4.i.i.i.i.i35 [
    i64 -9223372036854775808, label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit
    i64 0, label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit.thread
  ]

bb2.i.i.i4.i.i.i.i.i35:                           ; preds = %bb51
  %82 = getelementptr inbounds nuw i8, ptr %_2.i, i64 8
  %_2.val3.i = load ptr, ptr %82, align 8, !noalias !258, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val3.i, i64 noundef %81, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !258
  br label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit.thread

_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit.thread: ; preds = %bb51, %bb2.i.i.i4.i.i.i.i.i35
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !258
  br label %bb57.thread

_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit: ; preds = %bb51
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i), !noalias !258
; call build_script_build::do_compile_probe
  %83 = call fastcc noundef zeroext i1 @_RNvCs7AmXS38G9s1_18build_script_build16do_compile_probe(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_bd43b6b16475694c16f8c5bd38c53a67, i64 noundef 24, i1 noundef zeroext true)
  br i1 %83, label %bb57.thread, label %bb58

bb57.thread:                                      ; preds = %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit.thread, %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_99296b26f853c0e00fd05857e3136e29, ptr noundef nonnull inttoptr (i64 83 to ptr))
  br label %bb58

bb56:                                             ; preds = %bb2.i.i.i4.i.i.i29
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_91482d60220a3de858c1d085837d0ff7, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_99296b26f853c0e00fd05857e3136e29, ptr noundef nonnull inttoptr (i64 83 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34ae0dc4ffa16faaed57f3cbab572bbd, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br i1 %consider_rustc_bootstrap.sroa.0.0.off0108, label %bb64, label %bb66

bb60:                                             ; preds = %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34ae0dc4ffa16faaed57f3cbab572bbd, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br i1 %consider_rustc_bootstrap.sroa.0.1.off081, label %bb64, label %bb66

bb58:                                             ; preds = %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit, %bb57.thread
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i36), !noalias !261
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i36, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_dd8815cdae13b8c8aeb9b9be3f3d7a26, i64 noundef 11), !noalias !261
  %84 = load i64, ptr %_2.i36, align 8, !range !41, !noalias !261, !noundef !3
  switch i64 %84, label %bb2.i.i.i4.i.i.i.i.i41 [
    i64 -9223372036854775808, label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43
    i64 0, label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43.thread
  ]

bb2.i.i.i4.i.i.i.i.i41:                           ; preds = %bb58
  %85 = getelementptr inbounds nuw i8, ptr %_2.i36, i64 8
  %_2.val3.i42 = load ptr, ptr %85, align 8, !noalias !261, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val3.i42, i64 noundef %84, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !261
  br label %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43.thread

_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43.thread: ; preds = %bb58, %bb2.i.i.i4.i.i.i.i.i41
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i36), !noalias !261
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_34ae0dc4ffa16faaed57f3cbab572bbd, ptr noundef nonnull inttoptr (i64 75 to ptr))
  br i1 %consider_rustc_bootstrap.sroa.0.1.off081, label %bb64, label %bb66

_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43: ; preds = %bb58
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i36), !noalias !261
; call build_script_build::do_compile_probe
  %86 = call fastcc noundef zeroext i1 @_RNvCs7AmXS38G9s1_18build_script_build16do_compile_probe(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ddcfd028fe4f27013cd9e73d07c52a9e, i64 noundef 20, i1 noundef zeroext true)
  br i1 %86, label %bb60, label %bb63

bb63:                                             ; preds = %bb50, %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43
  br i1 %consider_rustc_bootstrap.sroa.0.1.off081, label %bb64, label %bb66

bb66:                                             ; preds = %bb45.thread, %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43.thread, %bb56, %bb60, %bb64, %bb63
  ret void

bb64:                                             ; preds = %bb45.thread111, %_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable.exit43.thread, %bb56, %bb60, %bb63
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_c4fe0d46c3935d35a63bc8de9de91c71, ptr noundef nonnull inttoptr (i64 87 to ptr))
  br label %bb66
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
  ], !prof !264

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
  %2 = load i8, ptr %1, align 8, !range !265, !noundef !3
  br label %bb6

bb4:                                              ; preds = %start
  %3 = getelementptr i8, ptr %self.0.val, i64 -1
  %4 = icmp ne ptr %3, null
  tail call void @llvm.assume(i1 %4)
  %5 = getelementptr i8, ptr %self.0.val, i64 15
  %6 = load i8, ptr %5, align 8, !range !265, !noundef !3
  br label %bb6

bb6:                                              ; preds = %bb4.i, %bb42.i, %bb43.i, %bb2.i1, %bb4.i2, %bb5.i3, %bb6.i, %bb7.i, %bb8.i, %bb9.i, %bb10.i, %bb11.i, %bb12.i, %bb13.i, %bb14.i, %bb15.i, %bb16.i, %bb17.i, %bb18.i, %bb19.i, %bb20.i, %bb21.i, %bb22.i, %bb23.i, %bb24.i, %bb25.i, %bb26.i, %bb27.i, %bb28.i, %bb29.i, %bb30.i, %bb31.i, %bb32.i, %bb33.i, %bb34.i, %bb35.i, %bb36.i, %bb37.i, %bb5, %bb4, %bb2
  %kind.sroa.0.0 = phi i8 [ %2, %bb2 ], [ %6, %bb4 ], [ 41, %bb43.i ], [ 8, %bb37.i ], [ 9, %bb36.i ], [ 28, %bb35.i ], [ 6, %bb34.i ], [ 2, %bb33.i ], [ 3, %bb32.i ], [ 30, %bb31.i ], [ 26, %bb30.i ], [ 12, %bb29.i ], [ 27, %bb28.i ], [ 4, %bb27.i ], [ 35, %bb26.i ], [ 20, %bb25.i ], [ 15, %bb24.i ], [ 18, %bb23.i ], [ 0, %bb22.i ], [ 38, %bb21.i ], [ 24, %bb20.i ], [ 36, %bb19.i ], [ 32, %bb18.i ], [ 33, %bb17.i ], [ 10, %bb16.i ], [ 5, %bb15.i ], [ 7, %bb14.i ], [ 14, %bb13.i ], [ 16, %bb12.i ], [ 11, %bb11.i ], [ 17, %bb10.i ], [ 25, %bb9.i ], [ 19, %bb8.i ], [ 22, %bb7.i ], [ 29, %bb6.i ], [ 31, %bb5.i3 ], [ 39, %bb4.i2 ], [ 1, %bb2.i1 ], [ 13, %bb42.i ], [ 34, %bb5 ], [ %switch.idx.cast, %bb4.i ]
  ret i8 %kind.sroa.0.0
}

; <core::str::iter::SplitInternal<char>>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7AmXS38G9s1_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !266, !noundef !3
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !3, !align !25, !noundef !3
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1 = load i64, ptr %2, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !267)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i = load i64, ptr %4, align 8, !alias.scope !267, !noalias !270, !noundef !3
  %_38.not.i = icmp ugt i64 %index2.i, %_4.val1
  %.promoted.i = load i64, ptr %3, align 8, !alias.scope !267, !noalias !270
  %_4325.i = icmp ult i64 %index2.i, %.promoted.i
  %or.cond26.i = or i1 %_38.not.i, %_4325.i
  br i1 %or.cond26.i, label %bb1.i, label %bb12.lr.ph.i

bb12.lr.ph.i:                                     ; preds = %bb2
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_48.i = load i8, ptr %5, align 8, !alias.scope !267, !noalias !270, !noundef !3
  %_12.i = zext i8 %_48.i to i64
  %6 = getelementptr i8, ptr %_10.i, i64 %_12.i
  %_49.i = getelementptr i8, ptr %6, i64 -1
  %_65.i = icmp ult i8 %_48.i, 5
  %last_byte.us.pre.i = load i8, ptr %_49.i, align 1, !alias.scope !267, !noalias !270
  br i1 %_65.i, label %bb12.us.i, label %bb12.i, !prof !272

bb12.us.i:                                        ; preds = %bb12.lr.ph.i, %bb9.us.i
  %7 = phi i64 [ %16, %bb9.us.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.us.i = sub nuw i64 %index2.i, %7
  %_46.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %7
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i), !noalias !273
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %10, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %9, align 1, !alias.scope !274, !noalias !273, !noundef !3
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
  store i64 %16, ptr %3, align 8, !alias.scope !267, !noalias !270
  %_17.not.us.i = icmp ult i64 %16, %_12.i
  %_54.not.us.i = icmp ugt i64 %16, %_4.val1
  %or.cond.i = or i1 %_17.not.us.i, %_54.not.us.i
  br i1 %or.cond.i, label %bb9.us.i, label %bb19.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %found_char.us.i = sub nuw i64 %16, %_12.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %found_char.us.i
  %17 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us.i, ptr nonnull readonly align 1 %_10.i, i64 range(i64 0, -9223372036854775808) %_12.i), !alias.scope !277, !noalias !270
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
  %20 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i), !noalias !273
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
  %_9.i.i = load i8, ptr %23, align 1, !alias.scope !274, !noalias !273, !noundef !3
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
  store i64 %28, ptr %3, align 8, !alias.scope !267, !noalias !270
  %_17.not.i = icmp ult i64 %28, %_12.i
  %_54.not.i = icmp ugt i64 %28, %_4.val1
  %or.cond70.i = or i1 %_17.not.i, %_54.not.i
  br i1 %or.cond70.i, label %bb9.i, label %bb25.i

bb10.i:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  store i64 %index2.i, ptr %3, align 8, !alias.scope !267, !noalias !270
  br label %bb1.i

bb9.i:                                            ; preds = %bb4.i
  %_43.i = icmp ult i64 %index2.i, %28
  br i1 %_43.i, label %bb1.i, label %bb12.i

bb25.i:                                           ; preds = %bb4.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #22, !noalias !273
  unreachable

bb7:                                              ; preds = %bb19.us.i
  %i = load i64, ptr %self, align 8, !noundef !3
  %new_len = sub nuw i64 %found_char.us.i, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %16, ptr %self, align 8
  br label %bb9

bb1.i:                                            ; preds = %bb9.i, %bb9.us.i, %bb2, %bb10.i
  store i8 1, ptr %0, align 1, !alias.scope !281
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %30 = load i8, ptr %29, align 8, !range !266, !alias.scope !281, !noundef !3
  %_3.i = trunc nuw i8 %30 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !281
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !281
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
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs7AmXS38G9s1_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !3
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !284)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !287)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !290, !noalias !291
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !290, !noalias !291
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !290, !noalias !291
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !290, !noalias !291
  store i64 0, ptr %self, align 8, !alias.scope !290, !noalias !291
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit

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
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !293, !nonnull !3, !noundef !3
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !294, !noundef !3
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #17, !noalias !299
  %11 = load ptr, ptr %10, align 8, !noalias !294, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #17, !noalias !299
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !300)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !303)
  %_3.i.i = load i64, ptr %self, align 8, !range !221, !alias.scope !306, !noalias !307, !noundef !3
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !306, !noalias !307, !noundef !3
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !309, !noalias !312
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !309, !noalias !312
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !306, !noalias !307, !noundef !3
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !306, !noalias !307, !nonnull !3, !noundef !3
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !306, !noalias !307
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !314, !nonnull !3, !noundef !3
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !315)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !316, !noundef !3
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !323, !noundef !3
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
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !328, !nonnull !3, !noundef !3
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !323
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #17, !noalias !332
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !316, !noundef !3
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #17, !noalias !332
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #22
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !333

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #21, !noalias !334
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !309, !noalias !312
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !309, !noalias !312
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !309, !noalias !312
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build.exit
  ret void
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs7AmXS38G9s1_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
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

; <std::path::Path>::_with_extension
; Function Attrs: uwtable
declare void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path15__with_extension(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

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

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

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
  store ptr @_RNvCs7AmXS38G9s1_18build_script_build4main, ptr %_7.i, align 8
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
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs7AmXS38G9s1_18build_script_build: %_1.0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs7AmXS38G9s1_18build_script_build"}
!7 = !{i64 8}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!12 = distinct !{!12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build"}
!13 = !{i64 0, i64 -9223372036854775805}
!14 = !{!15}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build"}
!17 = !{!15, !11}
!18 = !{i64 0, i64 -9223372036854775806}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs7AmXS38G9s1_18build_script_build: %_1"}
!21 = distinct !{!21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs7AmXS38G9s1_18build_script_build"}
!22 = !{!23, !20}
!23 = distinct !{!23, !24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs7AmXS38G9s1_18build_script_build: %_1"}
!24 = distinct !{!24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs7AmXS38G9s1_18build_script_build"}
!25 = !{i64 1}
!26 = !{i64 4}
!27 = !{i32 0, i32 6}
!28 = !{!"branch_weights", i32 2000, i32 6001}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!31 = distinct !{!31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build"}
!32 = !{!33}
!33 = distinct !{!33, !34, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs7AmXS38G9s1_18build_script_build: %self"}
!34 = distinct !{!34, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs7AmXS38G9s1_18build_script_build"}
!35 = !{!33, !30}
!36 = !{!37, !39, !33, !30}
!37 = distinct !{!37, !38, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs7AmXS38G9s1_18build_script_build: %self"}
!38 = distinct !{!38, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs7AmXS38G9s1_18build_script_build"}
!39 = distinct !{!39, !40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!40 = distinct !{!40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs7AmXS38G9s1_18build_script_build"}
!41 = !{i64 0, i64 -9223372036854775807}
!42 = !{i64 18809646068454179}
!43 = !{!44}
!44 = distinct !{!44, !45, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs7AmXS38G9s1_18build_script_build: %_1"}
!45 = distinct !{!45, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs7AmXS38G9s1_18build_script_build"}
!46 = !{!47, !49}
!47 = distinct !{!47, !48, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: %_0"}
!48 = distinct !{!48, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var"}
!49 = distinct !{!49, !48, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: argument 1"}
!50 = !{!47}
!51 = !{!49}
!52 = !{!53, !55}
!53 = distinct !{!53, !54, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: %_0"}
!54 = distinct !{!54, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var"}
!55 = distinct !{!55, !54, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: argument 1"}
!56 = !{!55}
!57 = !{!58}
!58 = distinct !{!58, !59, !"_RINvNtCs5sEH5CPMdak_3std2fs10create_dirRNtNtB4_4path7PathBufECs7AmXS38G9s1_18build_script_build: argument 0"}
!59 = distinct !{!59, !"_RINvNtCs5sEH5CPMdak_3std2fs10create_dirRNtNtB4_4path7PathBufECs7AmXS38G9s1_18build_script_build"}
!60 = !{!61}
!61 = distinct !{!61, !62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probe0EB1C_: %self"}
!62 = distinct !{!62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probe0EB1C_"}
!63 = !{!64}
!64 = distinct !{!64, !62, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probe0EB1C_: %_0"}
!65 = !{!64, !61}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probes_0EB1C_: %self"}
!68 = distinct !{!68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probes_0EB1C_"}
!69 = !{!70}
!70 = distinct !{!70, !68, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE6filterNCNvCs7AmXS38G9s1_18build_script_build16do_compile_probes_0EB1C_: %_0"}
!71 = !{!70, !67}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECs7AmXS38G9s1_18build_script_build: %self"}
!74 = distinct !{!74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECs7AmXS38G9s1_18build_script_build"}
!75 = !{!76}
!76 = distinct !{!76, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECs7AmXS38G9s1_18build_script_build: %other"}
!77 = !{!78, !73}
!78 = distinct !{!78, !74, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBV_ENtNtNtBa_6traits8iterator8Iterator5chainINtNtNtBa_7sources4once4OnceB1j_EECs7AmXS38G9s1_18build_script_build: %_0"}
!79 = !{!78, !76}
!80 = !{!81}
!81 = distinct !{!81, !82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build: %_0"}
!82 = distinct !{!82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build"}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %_0"}
!85 = distinct !{!85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build"}
!86 = !{!87, !89, !90, !91}
!87 = distinct !{!87, !88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %opt"}
!88 = distinct !{!88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build"}
!89 = distinct !{!89, !85, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %self"}
!90 = distinct !{!90, !82, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build: argument 1"}
!91 = distinct !{!91, !92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %opt"}
!92 = distinct !{!92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build"}
!93 = !{!94, !84, !81, !95}
!94 = distinct !{!94, !88, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %_0"}
!95 = distinct !{!95, !92, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %_0"}
!96 = !{!97, !99, !100, !102, !84, !89, !81, !90}
!97 = distinct !{!97, !98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_0"}
!98 = distinct !{!98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build"}
!99 = distinct !{!99, !98, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_1"}
!100 = distinct !{!100, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %x"}
!101 = distinct !{!101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build"}
!102 = distinct !{!102, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %f"}
!103 = !{!104, !95}
!104 = distinct !{!104, !101, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %self"}
!105 = !{!91}
!106 = !{!95}
!107 = !{!108, !110, !111, !113}
!108 = distinct !{!108, !109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_0"}
!109 = distinct !{!109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build"}
!110 = distinct !{!110, !109, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_1"}
!111 = distinct !{!111, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %x"}
!112 = distinct !{!112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build"}
!113 = distinct !{!113, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %f"}
!114 = !{!115}
!115 = distinct !{!115, !112, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %self"}
!116 = !{!117, !119}
!117 = distinct !{!117, !118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build: %_0"}
!118 = distinct !{!118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build"}
!119 = distinct !{!119, !118, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build: %program"}
!120 = !{!119}
!121 = !{!122, !124}
!122 = distinct !{!122, !123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECs7AmXS38G9s1_18build_script_build: %self"}
!123 = distinct !{!123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECs7AmXS38G9s1_18build_script_build"}
!124 = distinct !{!124, !123, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command4argsINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainIBR_INtNtBZ_6option8IntoIterNtNtNtB8_3ffi6os_str8OsStringEB1M_EINtNtNtBX_7sources4once4OnceB2a_EEB2a_ECs7AmXS38G9s1_18build_script_build: %args"}
!125 = !{!126, !128}
!126 = distinct !{!126, !127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %opt"}
!127 = distinct !{!127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build"}
!128 = distinct !{!128, !129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %self"}
!129 = distinct !{!129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build"}
!130 = !{!131, !132, !122, !124}
!131 = distinct !{!131, !127, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtB2_5ChainINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1g_EB1E_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %_0"}
!132 = distinct !{!132, !129, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainIBO_INtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB13_EINtNtNtB8_7sources4once4OnceB1r_EENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %_0"}
!133 = !{!132}
!134 = !{!128}
!135 = !{!126}
!136 = !{!137}
!137 = distinct !{!137, !138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build: %_0"}
!138 = distinct !{!138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build"}
!139 = !{!140}
!140 = distinct !{!140, !141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %_0"}
!141 = distinct !{!141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build"}
!142 = !{!143, !145, !146, !126, !128}
!143 = distinct !{!143, !144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %opt"}
!144 = distinct !{!144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build"}
!145 = distinct !{!145, !141, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtBa_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBZ_ENtNtNtB8_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %self"}
!146 = distinct !{!146, !138, !"_RNvYNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtBe_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEBX_ENtNtNtBc_6traits8iterator8Iterator4nextINtNtNtBe_3ops8function6FnOnceTQB5_EE9call_onceCs7AmXS38G9s1_18build_script_build: argument 1"}
!147 = !{!148, !140, !137, !131, !132, !122, !124}
!148 = distinct !{!148, !144, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain17and_then_or_clearINtNtB8_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1s_NvYB14_NtNtNtB6_6traits8iterator8Iterator4nextECs7AmXS38G9s1_18build_script_build: %_0"}
!149 = !{!150}
!150 = distinct !{!150, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %x"}
!151 = distinct !{!151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build"}
!152 = !{!153}
!153 = distinct !{!153, !154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_0"}
!154 = distinct !{!154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build"}
!155 = !{!156, !157, !145, !146, !126, !128}
!156 = distinct !{!156, !154, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainINtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB11_ENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_1"}
!157 = distinct !{!157, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %f"}
!158 = !{!153, !150, !159, !140, !137, !131, !132, !122, !124}
!159 = distinct !{!159, !151, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainINtB3_8IntoIterBI_EB2m_ENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %self"}
!160 = !{!161, !163, !153, !156, !150, !157, !140, !145, !137, !146}
!161 = distinct !{!161, !162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %_0"}
!162 = distinct !{!162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build"}
!163 = distinct !{!163, !162, !"_RNvXsy_NtCsjMrxcFdYDNN_4core6optionINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtB7_4iter6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %self"}
!164 = !{!159, !131, !126, !132, !128, !122, !124}
!165 = !{!153, !156, !150, !157, !140, !145, !137, !146}
!166 = !{!159, !131, !132, !122, !124}
!167 = !{!131, !126, !132, !128, !122, !124}
!168 = !{!169}
!169 = distinct !{!169, !170, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!170 = distinct !{!170, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build"}
!171 = !{!172}
!172 = distinct !{!172, !173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build: %_1"}
!173 = distinct !{!173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build"}
!174 = !{!172, !169, !126, !128}
!175 = !{!172, !169, !131, !126, !132, !128, !124}
!176 = !{!177}
!177 = distinct !{!177, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %x"}
!178 = distinct !{!178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build"}
!179 = !{!180}
!180 = distinct !{!180, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %self"}
!181 = !{!182}
!182 = distinct !{!182, !178, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE7or_elseNCNvXs_NtNtNtB5_4iter8adapters5chainINtB1G_5ChainIB2a_INtB3_8IntoIterBI_EB2r_EINtNtNtB1K_7sources4once4OnceBI_EENtNtNtB1K_6traits8iterator8Iterator4next0ECs7AmXS38G9s1_18build_script_build: %f"}
!183 = !{!177, !180, !132}
!184 = !{!182, !128, !122, !124}
!185 = !{!186}
!186 = distinct !{!186, !187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_0"}
!187 = distinct !{!187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build"}
!188 = !{!189, !182, !128}
!189 = distinct !{!189, !187, !"_RNCNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB6_5ChainIBQ_INtNtBc_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB15_EINtNtNtBa_7sources4once4OnceB1t_EENtNtNtBa_6traits8iterator8Iterator4next0Cs7AmXS38G9s1_18build_script_build: %_1"}
!190 = !{!186, !177, !180, !132, !122, !124}
!191 = !{!192, !194, !186, !189, !177, !182, !132, !128}
!192 = distinct !{!192, !193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %_0"}
!193 = distinct !{!193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build"}
!194 = distinct !{!194, !193, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtB6_6traits8iterator8Iterator4nextCs7AmXS38G9s1_18build_script_build: %self"}
!195 = !{!180, !122, !124}
!196 = !{!124}
!197 = !{!186, !189, !177, !182, !132, !128}
!198 = !{!199}
!199 = distinct !{!199, !200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!200 = distinct !{!200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainIBH_INtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1m_EINtNtNtBN_7sources4once4OnceB1K_EEECs7AmXS38G9s1_18build_script_build"}
!201 = !{!202}
!202 = distinct !{!202, !203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build: %_1"}
!203 = distinct !{!203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters5chain5ChainINtBJ_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1E_EEECs7AmXS38G9s1_18build_script_build"}
!204 = !{!205}
!205 = distinct !{!205, !206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build: %_1"}
!206 = distinct !{!206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters5chain5ChainINtNtB4_6option8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEB1i_EECs7AmXS38G9s1_18build_script_build"}
!207 = !{!205, !202, !199, !124}
!208 = !{!205, !202, !199}
!209 = !{!199, !124}
!210 = !{!211}
!211 = distinct !{!211, !212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECs7AmXS38G9s1_18build_script_build: argument 1"}
!212 = distinct !{!212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECs7AmXS38G9s1_18build_script_build"}
!213 = !{!214}
!214 = distinct !{!214, !212, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argRNtNtB8_4path7PathBufECs7AmXS38G9s1_18build_script_build: %self"}
!215 = !{!216}
!216 = distinct !{!216, !217, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtB8_4path7PathBufECs7AmXS38G9s1_18build_script_build: %arg"}
!217 = distinct !{!217, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtB8_4path7PathBufECs7AmXS38G9s1_18build_script_build"}
!218 = !{!219}
!219 = distinct !{!219, !220, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build: %arg"}
!220 = distinct !{!220, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3argNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build"}
!221 = !{i64 0, i64 2}
!222 = !{!223}
!223 = distinct !{!223, !224, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs7AmXS38G9s1_18build_script_build: %_1"}
!224 = distinct !{!224, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCs5sEH5CPMdak_3std3env8VarErrorEECs7AmXS38G9s1_18build_script_build"}
!225 = !{i32 0, i32 2}
!226 = !{!227, !229}
!227 = distinct !{!227, !228, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: %_0"}
!228 = distinct !{!228, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var"}
!229 = distinct !{!229, !228, !"_RNvCs7AmXS38G9s1_18build_script_build13cargo_env_var: argument 1"}
!230 = !{!227}
!231 = !{!229}
!232 = !{!233, !235}
!233 = distinct !{!233, !234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build: %_0"}
!234 = distinct !{!234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build"}
!235 = distinct !{!235, !234, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs7AmXS38G9s1_18build_script_build: %program"}
!236 = !{!235}
!237 = !{!238}
!238 = distinct !{!238, !239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECs7AmXS38G9s1_18build_script_build: %_1"}
!239 = distinct !{!239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECs7AmXS38G9s1_18build_script_build"}
!240 = !{!241, !243}
!241 = distinct !{!241, !242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build: %self.0"}
!242 = distinct !{!242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build"}
!243 = distinct !{!243, !242, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build: %other.0"}
!244 = !{!245}
!245 = distinct !{!245, !246, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!246 = distinct !{!246, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!247 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!248 = !{!"branch_weights", i32 2002, i32 2000}
!249 = !{!250}
!250 = distinct !{!250, !251, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build: %_1"}
!251 = distinct !{!251, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build"}
!252 = !{!253}
!253 = distinct !{!253, !254, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build: %_1"}
!254 = distinct !{!254, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build"}
!255 = !{!256}
!256 = distinct !{!256, !257, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build: %_1"}
!257 = distinct !{!257, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs7AmXS38G9s1_18build_script_build"}
!258 = !{!259}
!259 = distinct !{!259, !260, !"_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable: %feature.0"}
!260 = distinct !{!260, !"_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable"}
!261 = !{!262}
!262 = distinct !{!262, !263, !"_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable: %feature.0"}
!263 = distinct !{!263, !"_RNvCs7AmXS38G9s1_18build_script_build20compile_probe_stable"}
!264 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000, i32 2000}
!265 = !{i8 0, i8 42}
!266 = !{i8 0, i8 2}
!267 = !{!268}
!268 = distinct !{!268, !269, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!269 = distinct !{!269, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!270 = !{!271}
!271 = distinct !{!271, !269, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!272 = !{!"branch_weights", i32 4000000, i32 4001}
!273 = !{!271, !268}
!274 = !{!275}
!275 = distinct !{!275, !276, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!276 = distinct !{!276, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!277 = !{!278, !280}
!278 = distinct !{!278, !279, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build: %self.0"}
!279 = distinct !{!279, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build"}
!280 = distinct !{!280, !279, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7AmXS38G9s1_18build_script_build: %other.0"}
!281 = !{!282}
!282 = distinct !{!282, !283, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs7AmXS38G9s1_18build_script_build: %self"}
!283 = distinct !{!283, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs7AmXS38G9s1_18build_script_build"}
!284 = !{!285}
!285 = distinct !{!285, !286, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %self"}
!286 = distinct !{!286, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build"}
!287 = !{!288}
!288 = distinct !{!288, !289, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs7AmXS38G9s1_18build_script_build: %self"}
!289 = distinct !{!289, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs7AmXS38G9s1_18build_script_build"}
!290 = !{!288, !285}
!291 = !{!292}
!292 = distinct !{!292, !289, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs7AmXS38G9s1_18build_script_build: %_0"}
!293 = !{!292, !288, !285}
!294 = !{!295, !297, !285}
!295 = distinct !{!295, !296, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs7AmXS38G9s1_18build_script_build: %_0"}
!296 = distinct !{!296, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs7AmXS38G9s1_18build_script_build"}
!297 = distinct !{!297, !298, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %ret"}
!298 = distinct !{!298, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build"}
!299 = !{!297, !285}
!300 = !{!301}
!301 = distinct !{!301, !302, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %self"}
!302 = distinct !{!302, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build"}
!303 = !{!304}
!304 = distinct !{!304, !305, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs7AmXS38G9s1_18build_script_build: %self"}
!305 = distinct !{!305, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs7AmXS38G9s1_18build_script_build"}
!306 = !{!304, !301}
!307 = !{!308}
!308 = distinct !{!308, !302, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %_0"}
!309 = !{!310, !301}
!310 = distinct !{!310, !311, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs7AmXS38G9s1_18build_script_build: %v"}
!311 = distinct !{!311, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs7AmXS38G9s1_18build_script_build"}
!312 = !{!313, !308}
!313 = distinct !{!313, !311, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs7AmXS38G9s1_18build_script_build: %ret"}
!314 = !{!304, !308, !301}
!315 = !{!310}
!316 = !{!317, !319, !320, !322, !313, !310, !308, !301}
!317 = distinct !{!317, !318, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %_0"}
!318 = distinct !{!318, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build"}
!319 = distinct !{!319, !318, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %self"}
!320 = distinct !{!320, !321, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs7AmXS38G9s1_18build_script_build: %val"}
!321 = distinct !{!321, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs7AmXS38G9s1_18build_script_build"}
!322 = distinct !{!322, !321, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs7AmXS38G9s1_18build_script_build: %leaf_edge"}
!323 = !{!324, !326, !317, !319, !320, !322, !313, !310, !308, !301}
!324 = distinct !{!324, !325, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs7AmXS38G9s1_18build_script_build: %_0"}
!325 = distinct !{!325, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs7AmXS38G9s1_18build_script_build"}
!326 = distinct !{!326, !327, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build: %ret"}
!327 = distinct !{!327, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs7AmXS38G9s1_18build_script_build"}
!328 = !{!329, !331, !317, !319, !320, !322, !313, !310, !308, !301}
!329 = distinct !{!329, !330, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs7AmXS38G9s1_18build_script_build: %_0"}
!330 = distinct !{!330, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs7AmXS38G9s1_18build_script_build"}
!331 = distinct !{!331, !330, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs7AmXS38G9s1_18build_script_build: %self"}
!332 = !{!326, !317, !319, !320, !322, !313, !310, !308, !301}
!333 = !{!313, !310, !308, !301}
!334 = !{!308, !301}
