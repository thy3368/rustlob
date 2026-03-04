; ModuleID = 'build_script_build.49d4127095fcfa17-cgu.0'
source_filename = "build_script_build.49d4127095fcfa17-cgu.0"
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
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs6kZbxSpzgup_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs6kZbxSpzgup_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs6kZbxSpzgup_18build_script_build }>, align 8
@alloc_93816f04728d387347072ad30618ff9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_69009fdc319497586282719e739ab5f8, [16 x i8] c"\87\00\00\00\00\00\00\00X\02\00\000\00\00\00" }>, align 8
@alloc_193ab55f01318f0887536940a400dd6a = private unnamed_addr constant [71 x i8] c"\16Environment variable $\C0- is not set during execution of build script\0A\00", align 1
@alloc_806c1ac911172019779ceab530bc1f0e = private unnamed_addr constant [5 x i8] c"RUSTC", align 1
@alloc_a887f9858119cc7413062dc002c4d9ab = private unnamed_addr constant [9 x i8] c"--version", align 1
@alloc_ca36d7e792bb4bbd1a68749f90007ce8 = private unnamed_addr constant [7 x i8] c"rustc 1", align 1
@alloc_742f06589122110502429e832b81e8bd = private unnamed_addr constant [32 x i8] c"cargo:rerun-if-changed=build.rs\0A", align 1
@alloc_3dee171236971d7a153245e0048b0a70 = private unnamed_addr constant [38 x i8] c"cargo:rustc-check-cfg=cfg(exhaustive)\0A", align 1
@alloc_b1881eb1ec95323fce0d23ff6a1c5d14 = private unnamed_addr constant [56 x i8] c"cargo:rustc-check-cfg=cfg(zmij_no_select_unpredictable)\0A", align 1
@alloc_705a2a19dbcd66b78bc70bde770ef776 = private unnamed_addr constant [45 x i8] c"cargo:rustc-cfg=zmij_no_select_unpredictable\0A", align 1
@alloc_7fe94be2e120ffbd80c490b1b3c481ee = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/str/pattern.rs\00", align 1
@alloc_e52d3af24e8037dfb4f35693fba7d9f6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7fe94be2e120ffbd80c490b1b3c481ee, [16 x i8] c"w\00\00\00\00\00\00\00\CD\01\00\007\00\00\00" }>, align 8

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECs6kZbxSpzgup_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
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
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs6kZbxSpzgup_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !4)
  %_78.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_78.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i
  %_3.sroa.0.09.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i ], [ 0, %start ]
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
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #16, !noalias !4
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i

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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #16, !noalias !4
  br label %bb4.i.i.preheader

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i
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
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build(ptr %_4.val.i.i, ptr nonnull %_4.val6.i.i) #17
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !4

terminate.i.i:                                    ; preds = %bb3.i.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #18, !noalias !4
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %25 = icmp eq i64 %_1.val2, 0
  br i1 %25, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #16
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !8, !noundef !3
  %26 = icmp eq i64 %_1.val4, 0
  br i1 %26, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs6kZbxSpzgup_18build_script_build.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val4, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #16
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs6kZbxSpzgup_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3j_4SyncEL_EEECs6kZbxSpzgup_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %14
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2W_4SyncEL_EECs6kZbxSpzgup_18build_script_build(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
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
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs6kZbxSpzgup_18build_script_build.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #16
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs6kZbxSpzgup_18build_script_build.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDINtNtNtCsjMrxcFdYDNN_4core3ops8function5FnMutuEp6OutputINtNtBP_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtBP_6marker4SendNtB2E_4SyncEL_ENtNtBN_4drop4Drop4dropCs6kZbxSpzgup_18build_script_build.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #16
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<std::process::Output>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %bb4, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %1, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #16
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i, %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val2 = load i64, ptr %2, align 8
  %3 = icmp eq i64 %.val2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs6kZbxSpzgup_18build_script_build.exit8, label %bb2.i.i.i4.i7

bb2.i.i.i4.i7:                                    ; preds = %bb4
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val3 = load ptr, ptr %4, align 8, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #16
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs6kZbxSpzgup_18build_script_build.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs6kZbxSpzgup_18build_script_build.exit8: ; preds = %bb4, %bb2.i.i.i4.i7
  ret void
}

; core::ptr::drop_in_place::<std::process::Command>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs6kZbxSpzgup_18build_script_build(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !10, !nonnull !3, !noundef !3
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 136
  %.val24.i = load i64, ptr %1, align 8, !alias.scope !10
  store i8 0, ptr %.val.i, align 1, !noalias !10
  %2 = icmp eq i64 %.val24.i, 0
  br i1 %2, label %bb20.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %.val24.i, i64 noundef 1) #16
  br label %bb20.i

bb20.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i, %start
; invoke <std::sys::process::unix::common::cstring_array::CStringArray as core::ops::drop::Drop>::drop
  invoke void @_RNvXs3_NtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_arrayNtB5_12CStringArrayNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(200) %_1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb20.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %_1.val.i.i = load i64, ptr %_1, align 8, !alias.scope !13
  %4 = icmp eq i64 %_1.val.i.i, 0
  br i1 %4, label %bb10.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i = load ptr, ptr %5, align 8, !alias.scope !13, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i.i.i = shl nuw i64 %_1.val.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #16
  br label %bb10.i

bb4.i.i:                                          ; preds = %bb20.i
  %_1.val2.i.i = load i64, ptr %_1, align 8, !alias.scope !13
  %6 = icmp eq i64 %_1.val2.i.i, 0
  br i1 %6, label %bb19.i, label %bb2.i.i.i4.i4.i.i

bb2.i.i.i4.i4.i.i:                                ; preds = %bb4.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val3.i.i = load ptr, ptr %7, align 8, !alias.scope !13, !nonnull !3, !noundef !3
  %alloc_size.i.i.i.i5.i5.i.i = shl nuw i64 %_1.val2.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val3.i.i, i64 noundef %alloc_size.i.i.i.i5.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #16
  br label %bb19.i

bb10.i:                                           ; preds = %bb2.i.i.i4.i.i.i, %cleanup.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %8) #17
          to label %bb9.i unwind label %terminate.i

bb19.i:                                           ; preds = %bb2.i.i.i4.i4.i.i, %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 96
; invoke core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(32) %9)
          to label %bb18.i unwind label %cleanup2.i

bb9.i:                                            ; preds = %cleanup2.i, %bb10.i
  %.pn10.i = phi { ptr, i32 } [ %14, %cleanup2.i ], [ %3, %bb10.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val27.i = load ptr, ptr %10, align 8, !alias.scope !10, !align !16, !noundef !3
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val28.i = load i64, ptr %11, align 8, !alias.scope !10
  %12 = icmp eq ptr %.val27.i, null
  br i1 %12, label %bb8.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb9.i
  store i8 0, ptr %.val27.i, align 1
  %13 = icmp eq i64 %.val28.i, 0
  br i1 %13, label %bb8.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val27.i, i64 noundef %.val28.i, i64 noundef 1) #16
  br label %bb8.i

cleanup2.i:                                       ; preds = %bb19.i
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %bb9.i

bb18.i:                                           ; preds = %bb19.i
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 144
  %.val31.i = load ptr, ptr %15, align 8, !alias.scope !10, !align !16, !noundef !3
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 152
  %.val32.i = load i64, ptr %16, align 8, !alias.scope !10
  %17 = icmp eq ptr %.val31.i, null
  br i1 %17, label %bb17.i, label %bb2.i50.i

bb2.i50.i:                                        ; preds = %bb18.i
  store i8 0, ptr %.val31.i, align 1
  %18 = icmp eq i64 %.val32.i, 0
  br i1 %18, label %bb17.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i: ; preds = %bb2.i50.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val31.i, i64 noundef %.val32.i, i64 noundef 1) #16
  br label %bb17.i

bb8.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i.i, %bb2.i.i, %bb9.i
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val25.i = load ptr, ptr %19, align 8, !alias.scope !10, !align !16, !noundef !3
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val26.i = load i64, ptr %20, align 8, !alias.scope !10
  %21 = icmp eq ptr %.val25.i, null
  br i1 %21, label %bb7.i, label %bb2.i54.i

bb2.i54.i:                                        ; preds = %bb8.i
  store i8 0, ptr %.val25.i, align 1
  %22 = icmp eq i64 %.val26.i, 0
  br i1 %22, label %bb7.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i: ; preds = %bb2.i54.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val25.i, i64 noundef %.val26.i, i64 noundef 1) #16
  br label %bb7.i

bb17.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i51.i, %bb2.i50.i, %bb18.i
  %23 = getelementptr inbounds nuw i8, ptr %_1, i64 160
  %.val29.i = load ptr, ptr %23, align 8, !alias.scope !10, !align !16, !noundef !3
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 168
  %.val30.i = load i64, ptr %24, align 8, !alias.scope !10
  %25 = icmp eq ptr %.val29.i, null
  br i1 %25, label %bb16.i, label %bb2.i58.i

bb2.i58.i:                                        ; preds = %bb17.i
  store i8 0, ptr %.val29.i, align 1
  %26 = icmp eq i64 %.val30.i, 0
  br i1 %26, label %bb16.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i: ; preds = %bb2.i58.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val29.i, i64 noundef %.val30.i, i64 noundef 1) #16
  br label %bb16.i

bb7.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i55.i, %bb2.i54.i, %bb8.i
  %27 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %27) #17
          to label %bb6.i unwind label %terminate.i

bb16.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i5.i.i59.i, %bb2.i58.i, %bb17.i
  %28 = getelementptr inbounds nuw i8, ptr %_1, i64 24
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<alloc::boxed::Box<dyn core::ops::function::FnMut<(), Output = core::result::Result<(), std::io::error::Error>> + core::marker::Send + core::marker::Sync>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtBL_5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB3c_4SyncEL_EEECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(24) %28)
          to label %bb15.i unwind label %cleanup5.i

bb6.i:                                            ; preds = %cleanup5.i, %bb7.i
  %.pn16.i = phi { ptr, i32 } [ %34, %cleanup5.i ], [ %.pn10.i, %bb7.i ]
  %29 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val33.i = load ptr, ptr %29, align 8, !alias.scope !10, !align !17, !noundef !3
  %30 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val34.i = load i64, ptr %30, align 8, !alias.scope !10
  %31 = icmp eq ptr %.val33.i, null
  %32 = icmp eq i64 %.val34.i, 0
  %or.cond.i.i = select i1 %31, i1 true, i1 %32
  br i1 %or.cond.i.i, label %bb5.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb6.i
  %33 = shl nuw nsw i64 %.val34.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val33.i, i64 noundef %33, i64 noundef 4) #16
  br label %bb5.i

cleanup5.i:                                       ; preds = %bb16.i
  %34 = landingpad { ptr, i32 }
          cleanup
  br label %bb6.i

bb15.i:                                           ; preds = %bb16.i
  %35 = getelementptr inbounds nuw i8, ptr %_1, i64 176
  %.val35.i = load ptr, ptr %35, align 8, !alias.scope !10, !align !17, !noundef !3
  %36 = getelementptr inbounds nuw i8, ptr %_1, i64 184
  %.val36.i = load i64, ptr %36, align 8, !alias.scope !10
  %37 = icmp eq ptr %.val35.i, null
  %38 = icmp eq i64 %.val36.i, 0
  %or.cond.i63.i = select i1 %37, i1 true, i1 %38
  br i1 %or.cond.i63.i, label %bb14.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i: ; preds = %bb15.i
  %39 = shl nuw nsw i64 %.val36.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val35.i, i64 noundef %39, i64 noundef 4) #16
  br label %bb14.i

bb5.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb6.i
  %40 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val41.i = load i32, ptr %40, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i.i = icmp eq i32 %.val41.i, 3
  br i1 %cond.i.i, label %bb2.i.i.i, label %bb4.i

bb2.i.i.i:                                        ; preds = %bb5.i
  %41 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val42.i = load i32, ptr %41, align 4, !alias.scope !10
  %_5.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val42.i) #16
  br label %bb4.i

bb14.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i64.i, %bb15.i
  %42 = getelementptr inbounds nuw i8, ptr %_1, i64 72
  %.val47.i = load i32, ptr %42, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i68.i = icmp eq i32 %.val47.i, 3
  br i1 %cond.i68.i, label %bb2.i.i70.i, label %bb13.i

bb2.i.i70.i:                                      ; preds = %bb14.i
  %43 = getelementptr inbounds nuw i8, ptr %_1, i64 76
  %.val48.i = load i32, ptr %43, align 4, !alias.scope !10
  %_5.i.i.i.i.i71.i = tail call noundef i32 @close(i32 noundef %.val48.i) #16
  br label %bb13.i

bb4.i:                                            ; preds = %bb2.i.i.i, %bb5.i
  %44 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val39.i = load i32, ptr %44, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i73.i = icmp eq i32 %.val39.i, 3
  br i1 %cond.i73.i, label %bb2.i.i75.i, label %bb3.i

bb2.i.i75.i:                                      ; preds = %bb4.i
  %45 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val40.i = load i32, ptr %45, align 4, !alias.scope !10
  %_5.i.i.i.i.i76.i = tail call noundef i32 @close(i32 noundef %.val40.i) #16
  br label %bb3.i

bb13.i:                                           ; preds = %bb2.i.i70.i, %bb14.i
  %46 = getelementptr inbounds nuw i8, ptr %_1, i64 80
  %.val45.i = load i32, ptr %46, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i78.i = icmp eq i32 %.val45.i, 3
  br i1 %cond.i78.i, label %bb2.i.i80.i, label %bb12.i

bb2.i.i80.i:                                      ; preds = %bb13.i
  %47 = getelementptr inbounds nuw i8, ptr %_1, i64 84
  %.val46.i = load i32, ptr %47, align 4, !alias.scope !10
  %_5.i.i.i.i.i81.i = tail call noundef i32 @close(i32 noundef %.val46.i) #16
  br label %bb12.i

bb3.i:                                            ; preds = %bb2.i.i75.i, %bb4.i
  %48 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val37.i = load i32, ptr %48, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i83.i = icmp eq i32 %.val37.i, 3
  br i1 %cond.i83.i, label %bb2.i.i85.i, label %bb1.i

bb2.i.i85.i:                                      ; preds = %bb3.i
  %49 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val38.i = load i32, ptr %49, align 4, !alias.scope !10
  %_5.i.i.i.i.i86.i = tail call noundef i32 @close(i32 noundef %.val38.i) #16
  br label %bb1.i

bb12.i:                                           ; preds = %bb2.i.i80.i, %bb13.i
  %50 = getelementptr inbounds nuw i8, ptr %_1, i64 88
  %.val43.i = load i32, ptr %50, align 8, !range !18, !alias.scope !10, !noundef !3
  %cond.i88.i = icmp eq i32 %.val43.i, 3
  br i1 %cond.i88.i, label %bb2.i.i90.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs6kZbxSpzgup_18build_script_build.exit

bb2.i.i90.i:                                      ; preds = %bb12.i
  %51 = getelementptr inbounds nuw i8, ptr %_1, i64 92
  %.val44.i = load i32, ptr %51, align 4, !alias.scope !10
  %_5.i.i.i.i.i91.i = tail call noundef i32 @close(i32 noundef %.val44.i) #16
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs6kZbxSpzgup_18build_script_build.exit

terminate.i:                                      ; preds = %bb7.i, %bb10.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #18
  unreachable

bb1.i:                                            ; preds = %bb2.i.i85.i, %bb3.i
  resume { ptr, i32 } %.pn16.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs6kZbxSpzgup_18build_script_build.exit: ; preds = %bb12.i, %bb2.i.i90.i
  ret void
}

; core::ptr::drop_in_place::<std::sys::process::env::CommandEnv>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys7process3env10CommandEnvECs6kZbxSpzgup_18build_script_build(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i = alloca [24 x i8], align 8
  %_x.i.i = alloca [72 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !19)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !22)
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_x.i.i), !noalias !25
  %self1.sroa.0.0.copyload.i.i = load ptr, ptr %_1, align 8, !alias.scope !25
  %.not.i.i = icmp eq ptr %self1.sroa.0.0.copyload.i.i, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self1.sroa.5.0.copyload.i.i = load i64, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !25
  %self1.sroa.4.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self1.sroa.4.0.copyload.i.i = load i64, ptr %self1.sroa.4.0.self.sroa_idx.i.i, align 8, !alias.scope !25
  %full_range.sroa.4.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 8
  store ptr null, ptr %full_range.sroa.4.0._x.sroa_idx.i.i, align 8, !noalias !25
  %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 16
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.4.sroa.4.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !25
  %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 24
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.4.sroa.5.0.full_range.sroa.4.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !25
  %full_range.sroa.6.0._x.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 40
  store ptr null, ptr %full_range.sroa.6.0._x.sroa_idx.i.i, align 8, !noalias !25
  %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 48
  store ptr %self1.sroa.0.0.copyload.i.i, ptr %full_range.sroa.6.sroa.4.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !25
  %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 56
  store i64 %self1.sroa.4.0.copyload.i.i, ptr %full_range.sroa.6.sroa.5.0.full_range.sroa.6.0._x.sroa_idx.sroa_idx.i.i, align 8, !noalias !25
  br label %bb3.i.i

bb3.i.i:                                          ; preds = %bb1.i.i, %start
  %.sink9.i.i = phi i64 [ 1, %bb1.i.i ], [ 0, %start ]
  %self1.sroa.5.0.copyload.sink.i.i = phi i64 [ %self1.sroa.5.0.copyload.i.i, %bb1.i.i ], [ 0, %start ]
  store i64 %.sink9.i.i, ptr %_x.i.i, align 8, !noalias !25
  %0 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 32
  store i64 %.sink9.i.i, ptr %0, align 8, !noalias !25
  %1 = getelementptr inbounds nuw i8, ptr %_x.i.i, i64 64
  store i64 %self1.sroa.5.0.copyload.sink.i.i, ptr %1, align 8, !noalias !25
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !26
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !25
  %2 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !26, !noundef !3
  %.not3.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build.exit, label %bb3.lr.ph.i.i.i.i

bb3.lr.ph.i.i.i.i:                                ; preds = %bb3.i.i
  %kv.sroa.22.0._2.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i, i64 16
  br label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb4.i.i.i.i, %bb3.lr.ph.i.i.i.i
  %3 = phi ptr [ %2, %bb3.lr.ph.i.i.i.i ], [ %7, %bb4.i.i.i.i ]
  %kv.sroa.22.0.copyload.i.i.i.i = load i64, ptr %kv.sroa.22.0._2.sroa_idx.i.i.i.i, align 8, !noalias !26
  %_5.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 8
  %key.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<std::ffi::os_str::OsString>", ptr %_5.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %_9.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 272
  %_17.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<core::option::Option<std::ffi::os_str::OsString>>", ptr %_9.i.i.i.i.i, i64 %kv.sroa.22.0.copyload.i.i.i.i
  %key.val.i.i.i.i.i = load i64, ptr %key.i.i.i.i.i, align 8, !noalias !26
  %4 = icmp eq i64 %key.val.i.i.i.i.i, 0
  br i1 %4, label %bb8.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb3.i.i.i.i
  %5 = getelementptr i8, ptr %key.i.i.i.i.i, i64 8
  %key.val1.i.i.i.i.i = load ptr, ptr %5, align 8, !noalias !26, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %key.val1.i.i.i.i.i, i64 noundef %key.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !26
  br label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i
  %self1.val.i.i.i.i.i.i.i = load i64, ptr %_17.i.i.i.i.i, align 8, !range !31, !noalias !26, !noundef !3
  switch i64 %self1.val.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i
    i64 0, label %bb4.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb8.i.i.i.i.i
  %6 = getelementptr i8, ptr %_17.i.i.i.i.i, i64 8
  %self1.val2.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !26, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self1.val2.i.i.i.i.i.i.i, i64 noundef %self1.val.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !26
  br label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i, %bb8.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !26
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !26
; call <alloc::collections::btree::map::IntoIter<std::ffi::os_str::OsString, core::option::Option<std::ffi::os_str::OsString>>>::dying_next
  call fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_2.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(72) %_x.i.i), !noalias !25
  %7 = load ptr, ptr %_2.i.i.i.i, align 8, !noalias !26, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %7, null
  br i1 %.not.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build.exit, label %bb3.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build.exit: ; preds = %bb4.i.i.i.i, %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i.i.i), !noalias !26
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_x.i.i), !noalias !25
  ret void
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs6kZbxSpzgup_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #2 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #16, !srcloc !32
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs6kZbxSpzgup_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #3 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs6kZbxSpzgup_18build_script_build(ptr noundef nonnull %_4) #19
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCs6kZbxSpzgup_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !3, !noundef !3
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECs6kZbxSpzgup_18build_script_build(ptr noundef nonnull readonly %0) #19, !noalias !33
  ret i32 0
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCs6kZbxSpzgup_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
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
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_742f06589122110502429e832b81e8bd, ptr noundef nonnull inttoptr (i64 65 to ptr))
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %key.i.i)
  store ptr @alloc_806c1ac911172019779ceab530bc1f0e, ptr %key.i.i, align 8, !noalias !36
  %0 = getelementptr inbounds nuw i8, ptr %key.i.i, i64 8
  store i64 5, ptr %0, align 8, !noalias !36
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !36
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_806c1ac911172019779ceab530bc1f0e, i64 noundef 5), !noalias !36
  %1 = load i64, ptr %_2.i.i, align 8, !range !31, !noalias !36, !noundef !3
  %.not.i.i = icmp eq i64 %1, -9223372036854775808
  br i1 %.not.i.i, label %bb3.i.i, label %_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var.exit.i

bb3.i.i:                                          ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_7.i.i), !noalias !36
  store ptr %key.i.i, ptr %_7.i.i, align 8, !noalias !36
  %_8.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_7.i.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6kZbxSpzgup_18build_script_build, ptr %_8.sroa.4.0..sroa_idx.i.i, align 8, !noalias !36
; call std::io::stdio::_eprint
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio7__eprint(ptr noundef nonnull @alloc_193ab55f01318f0887536940a400dd6a, ptr noundef nonnull %_7.i.i), !noalias !36
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_7.i.i), !noalias !36
; call std::process::exit
  call void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef 1) #20, !noalias !36
  unreachable

_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var.exit.i: ; preds = %start
  %rustc.sroa.3.0._2.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  %rustc.sroa.3.0.copyload.i = load ptr, ptr %rustc.sroa.3.0._2.i.sroa_idx.i, align 8, !nonnull !3, !noundef !3
  %rustc.sroa.4.0._2.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %rustc.sroa.4.0.copyload.i = load i64, ptr %rustc.sroa.4.0._2.i.sroa_idx.i, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i), !noalias !36
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %key.i.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %output.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_5.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_8.i)
  call void @llvm.lifetime.start.p0(i64 200, ptr nonnull %_2.i10.i), !noalias !39
; invoke <std::sys::process::unix::common::Command>::new
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr noalias noundef nonnull sret([200 x i8]) align 8 captures(none) dereferenceable(200) %_2.i10.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %rustc.sroa.3.0.copyload.i, i64 noundef %rustc.sroa.4.0.copyload.i)
          to label %bb2.i.i unwind label %cleanup.i.i, !noalias !39

cleanup.i.i:                                      ; preds = %_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var.exit.i
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = icmp eq i64 %1, 0
  br i1 %3, label %common.resume.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !39
  br label %common.resume.i

bb2.i.i:                                          ; preds = %_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var.exit.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noundef nonnull align 8 dereferenceable(200) %_2.i10.i, i64 200, i1 false), !noalias !43
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_2.i10.i), !noalias !39
  %4 = icmp eq i64 %1, 0
  br i1 %4, label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build.exit.i, label %bb2.i.i.i4.i.i.i6.i.i

bb2.i.i.i4.i.i.i6.i.i:                            ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rustc.sroa.3.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !39
  br label %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build.exit.i

common.resume.i:                                  ; preds = %cleanup1.i, %cleanup.body.i, %bb2.i.i.i4.i.i.i.i.i, %cleanup.i.i
  %common.resume.op.i = phi { ptr, i32 } [ %2, %bb2.i.i.i4.i.i.i.i.i ], [ %2, %cleanup.i.i ], [ %30, %cleanup1.i ], [ %eh.lpad-body.i, %cleanup.body.i ]
  resume { ptr, i32 } %common.resume.op.i

_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build.exit.i: ; preds = %bb2.i.i.i4.i.i.i6.i.i, %bb2.i.i
; invoke <std::sys::process::unix::common::Command>::arg
  invoke void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a887f9858119cc7413062dc002c4d9ab, i64 noundef 9)
          to label %bb3.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb3.i, %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build.exit.i
  %5 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %5, %cleanup.i ], [ %22, %bb1.i.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i) #17
          to label %common.resume.i unwind label %terminate.i

bb3.i:                                            ; preds = %_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build.exit.i
; invoke <std::process::Command>::output
  invoke void @_RNvMsk_NtCs5sEH5CPMdak_3std7processNtB5_7Command6output(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(200) %_8.i)
          to label %bb4.i unwind label %cleanup.i

bb4.i:                                            ; preds = %bb3.i
  %6 = load i64, ptr %_5.i, align 8, !range !31, !noundef !3
  %7 = icmp eq i64 %6, -9223372036854775808
  br i1 %7, label %bb3.i13.i, label %bb25.i

bb3.i13.i:                                        ; preds = %bb4.i
  call void @llvm.experimental.noalias.scope.decl(metadata !44)
  %8 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %.val.i.i = load ptr, ptr %8, align 8, !alias.scope !44, !nonnull !3, !noundef !3
  %bits.i.i.i.i.i.i = ptrtoint ptr %.val.i.i to i64
  %_5.i.i.i.i.i.i = and i64 %bits.i.i.i.i.i.i, 3
  %switch.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i.i, label %bb2.i2.i.i.i.i.i, label %bb24.i, !prof !47

bb2.i2.i.i.i.i.i:                                 ; preds = %bb3.i13.i
  %9 = getelementptr i8, ptr %.val.i.i, i64 -1
  %10 = icmp ne ptr %9, null
  call void @llvm.assume(i1 %10)
  %_6.val.i.i.i.i.i.i.i = load ptr, ptr %9, align 8, !noalias !44
  %11 = getelementptr i8, ptr %.val.i.i, i64 7
  %_6.val1.i.i.i.i.i.i.i = load ptr, ptr %11, align 8, !noalias !44, !nonnull !3, !align !7, !noundef !3
  %12 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i, align 8, !invariant.load !3, !noalias !44
  %.not.i.i.i.i.i.i.i.i.i = icmp eq ptr %12, null
  br i1 %.not.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i2.i.i.i.i.i
  %13 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %13)
  invoke void %12(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i, !noalias !44

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i
  %14 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %14)
  %15 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !8, !invariant.load !3, !noalias !44
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !9, !invariant.load !3, !noalias !44
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs6kZbxSpzgup_18build_script_build.exit.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #16, !noalias !44
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs6kZbxSpzgup_18build_script_build.exit.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i:                        ; preds = %is_not_null.i.i.i.i.i.i.i.i.i
  %22 = landingpad { ptr, i32 }
          cleanup
  %23 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %24 = load i64, ptr %23, align 8, !range !8, !invariant.load !3, !noalias !44
  %25 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %26 = load i64, ptr %25, align 8, !range !9, !invariant.load !3, !noalias !44
  %27 = add i64 %26, -1
  %28 = icmp sgt i64 %27, -1
  call void @llvm.assume(i1 %28)
  %29 = icmp eq i64 %24, 0
  br i1 %29, label %bb1.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %24, i64 noundef range(i64 1, -9223372036854775807) %26) #16, !noalias !44
  br label %bb1.i.i.i.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #16, !noalias !44
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs6kZbxSpzgup_18build_script_build.exit.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #16, !noalias !44
  br label %bb24.i

bb25.i:                                           ; preds = %bb4.i
  %_24.sroa.4.0._5.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %val.sroa.4.0.output.sroa_idx.i = getelementptr inbounds nuw i8, ptr %output.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %val.sroa.4.0.output.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_24.sroa.4.0._5.sroa_idx.i, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
  store i64 %6, ptr %output.i, align 8
; invoke core::ptr::drop_in_place::<std::process::Command>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
          to label %bb6.i unwind label %cleanup1.i

bb24.i:                                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECs6kZbxSpzgup_18build_script_build.exit.i.i.i.i.i.i, %bb3.i13.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_5.i)
; call core::ptr::drop_in_place::<std::process::Command>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process7CommandECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(200) %_8.i)
  call void @llvm.lifetime.end.p0(i64 200, ptr nonnull %_8.i)
  br label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread

cleanup1.i:                                       ; preds = %bb9.i, %bb28.i, %bb6.i, %bb25.i
  %30 = landingpad { ptr, i32 }
          cleanup
; call core::ptr::drop_in_place::<std::process::Output>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(56) %output.i) #17
  br label %common.resume.i

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
  %_32.i = load i64, ptr %_12.i, align 8, !range !48, !noundef !3
  %32 = trunc nuw i64 %_32.i to i1
  br i1 %32, label %bb26.i, label %bb28.i

bb26.i:                                           ; preds = %bb7.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_12.i)
  br label %bb12.i

bb28.i:                                           ; preds = %bb7.i
  %33 = getelementptr inbounds nuw i8, ptr %_12.i, i64 8
  %_33.0.i = load ptr, ptr %33, align 8, !nonnull !3, !align !16, !noundef !3
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
  %35 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
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
  %38 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %36, ptr noundef nonnull dereferenceable(7) @alloc_ca36d7e792bb4bbd1a68749f90007ce8, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !49
  %39 = icmp eq i32 %38, 0
  br i1 %39, label %bb9.i, label %bb8.i

bb9.i:                                            ; preds = %bb32.i
; invoke <core::str::iter::SplitInternal<char>>::next
  %40 = invoke fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs6kZbxSpzgup_18build_script_build(ptr noalias noundef align 8 dereferenceable(72) %pieces.i)
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
  %43 = load i8, ptr %41, align 1, !alias.scope !53, !noundef !3
  switch i8 %43, label %bb9.i.i [
    i8 43, label %bb36.i
    i8 45, label %bb36.i
  ]

bb9thread-pre-split.i.i:                          ; preds = %bb35.i
  %.pr.i.i = load i8, ptr %41, align 1, !alias.scope !53
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
  br i1 %_57.1.i.i, label %bb36.i, label %bb33.i.i, !prof !56

bb33.i.i:                                         ; preds = %bb23.i.i
  %_57.0.i.i = extractvalue { i32, i1 } %45, 0
  %rest.12.i.i = add nsw i64 %src.sroa.15.1.i.i, -1
  %rest.01.i.i = getelementptr inbounds nuw i8, ptr %src.sroa.0.1.i.i, i64 1
  %46 = load i8, ptr %src.sroa.0.1.i.i, align 1, !alias.scope !53, !noundef !3
  %47 = zext i8 %46 to i32
  %48 = add nsw i32 %47, -48
  %_14.i.i.i = icmp ugt i32 %48, 9
  %_60.0.i.i = add i32 %48, %_57.0.i.i
  %_60.1.i.i = icmp ult i32 %_60.0.i.i, %_57.0.i.i
  %or.cond.i = select i1 %_14.i.i.i, i1 true, i1 %_60.1.i.i
  br i1 %or.cond.i, label %bb36.i, label %bb22.i.i, !prof !57

bb16.i.i:                                         ; preds = %bb15.preheader.i.i, %bb20.i.i
  %src.sroa.0.259.i.i = phi ptr [ %rest.04.i.i, %bb20.i.i ], [ %src.sroa.0.0.i.i, %bb15.preheader.i.i ]
  %src.sroa.15.258.i.i = phi i64 [ %rest.15.i.i, %bb20.i.i ], [ %src.sroa.15.0.i.i, %bb15.preheader.i.i ]
  %result.sroa.0.257.i.i = phi i32 [ %51, %bb20.i.i ], [ 0, %bb15.preheader.i.i ]
  %_19.i.i = load i8, ptr %src.sroa.0.259.i.i, align 1, !alias.scope !53, !noundef !3
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
  call void @llvm.experimental.noalias.scope.decl(metadata !58)
  %52 = icmp eq i64 %6, 0
  br i1 %52, label %bb4.i.i, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %bb34.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !58
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i, %bb34.i
  %53 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i.i = load i64, ptr %53, align 8, !alias.scope !58
  %54 = icmp eq i64 %.val2.i.i, 0
  br i1 %54, label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i.i

bb2.i.i.i4.i7.i.i:                                ; preds = %bb4.i.i
  %55 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i.i = load ptr, ptr %55, align 8, !alias.scope !58, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i, i64 noundef %.val2.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !58
  br label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread

bb36.i:                                           ; preds = %bb33.i.i, %bb23.i.i, %bb22.i.i, %bb20.i.i, %bb16.i.i, %bb15.preheader.i.i, %bb7.i.i, %bb7.i.i, %bb35.i
  %not._0.sroa.0.0.i.i = phi i32 [ 0, %bb15.preheader.i.i ], [ -1, %bb7.i.i ], [ -1, %bb7.i.i ], [ -1, %bb35.i ], [ %51, %bb20.i.i ], [ -1, %bb16.i.i ], [ %result.sroa.0.0.i.i, %bb22.i.i ], [ -1, %bb23.i.i ], [ -1, %bb33.i.i ]
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %pieces.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !61)
  %56 = icmp eq i64 %6, 0
  br i1 %56, label %bb4.i18.i, label %bb2.i.i.i4.i.i16.i

bb2.i.i.i4.i.i16.i:                               ; preds = %bb36.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !61
  br label %bb4.i18.i

bb4.i18.i:                                        ; preds = %bb2.i.i.i4.i.i16.i, %bb36.i
  %57 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i19.i = load i64, ptr %57, align 8, !alias.scope !61
  %58 = icmp eq i64 %.val2.i19.i, 0
  br i1 %58, label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit, label %bb2.i.i.i4.i7.i20.i

bb2.i.i.i4.i7.i20.i:                              ; preds = %bb4.i18.i
  %59 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i21.i = load ptr, ptr %59, align 8, !alias.scope !61, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i21.i, i64 noundef %.val2.i19.i, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !61
  br label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit

bb12.i:                                           ; preds = %bb8.i, %bb26.i
  call void @llvm.experimental.noalias.scope.decl(metadata !64)
  %60 = icmp eq i64 %6, 0
  br i1 %60, label %bb4.i26.i, label %bb2.i.i.i4.i.i24.i

bb2.i.i.i4.i.i24.i:                               ; preds = %bb12.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_31.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !64
  br label %bb4.i26.i

bb4.i26.i:                                        ; preds = %bb2.i.i.i4.i.i24.i, %bb12.i
  %61 = getelementptr inbounds nuw i8, ptr %output.i, i64 24
  %.val2.i27.i = load i64, ptr %61, align 8, !alias.scope !64
  %62 = icmp eq i64 %.val2.i27.i, 0
  br i1 %62, label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread, label %bb2.i.i.i4.i7.i28.i

bb2.i.i.i4.i7.i28.i:                              ; preds = %bb4.i26.i
  %63 = getelementptr inbounds nuw i8, ptr %output.i, i64 32
  %.val3.i29.i = load ptr, ptr %63, align 8, !alias.scope !64, !nonnull !3, !noundef !3
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i29.i, i64 noundef %.val2.i27.i, i64 noundef range(i64 1, -9223372036854775807) 1) #16, !noalias !64
  br label %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread

terminate.i:                                      ; preds = %cleanup.body.i
  %64 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #18
  unreachable

_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread: ; preds = %bb2.i.i.i4.i7.i28.i, %bb4.i26.i, %bb24.i, %bb4.i.i, %bb2.i.i.i4.i7.i.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  br label %bb7

_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit: ; preds = %bb4.i18.i, %bb2.i.i.i4.i7.i20.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %output.i)
  %_5 = icmp ugt i32 %not._0.sroa.0.0.i.i, 79
  br i1 %_5, label %bb7, label %bb8

bb7:                                              ; preds = %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit, %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread
  %rustc.sroa.0.08 = phi i32 [ -1, %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit.thread ], [ %not._0.sroa.0.0.i.i, %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit ]
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_3dee171236971d7a153245e0048b0a70, ptr noundef nonnull inttoptr (i64 77 to ptr))
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_b1881eb1ec95323fce0d23ff6a1c5d14, ptr noundef nonnull inttoptr (i64 113 to ptr))
  %_11 = icmp ult i32 %rustc.sroa.0.08, 88
  br i1 %_11, label %bb8, label %bb11

bb8:                                              ; preds = %_RNvCs6kZbxSpzgup_18build_script_build19rustc_minor_version.exit, %bb7
; call std::io::stdio::_print
  call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_705a2a19dbcd66b78bc70bde770ef776, ptr noundef nonnull inttoptr (i64 91 to ptr))
  br label %bb11

bb11:                                             ; preds = %bb7, %bb8
  ret void
}

; <core::str::iter::SplitInternal<char>>::next
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs6kZbxSpzgup_18build_script_build(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 65
  %1 = load i8, ptr %0, align 1, !range !67, !noundef !3
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb9, label %bb2

bb2:                                              ; preds = %start
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.val = load ptr, ptr %_4, align 8, !nonnull !3, !align !16, !noundef !3
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.val1 = load i64, ptr %2, align 8, !noundef !3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !68)
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %index2.i = load i64, ptr %4, align 8, !alias.scope !68, !noalias !71, !noundef !3
  %_38.not.i = icmp ugt i64 %index2.i, %_4.val1
  %.promoted.i = load i64, ptr %3, align 8, !alias.scope !68, !noalias !71
  %_4325.i = icmp ult i64 %index2.i, %.promoted.i
  %or.cond26.i = or i1 %_38.not.i, %_4325.i
  br i1 %or.cond26.i, label %bb1.i, label %bb12.lr.ph.i

bb12.lr.ph.i:                                     ; preds = %bb2
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 48
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_48.i = load i8, ptr %5, align 8, !alias.scope !68, !noalias !71, !noundef !3
  %_12.i = zext i8 %_48.i to i64
  %6 = getelementptr i8, ptr %_10.i, i64 %_12.i
  %_49.i = getelementptr i8, ptr %6, i64 -1
  %_65.i = icmp ult i8 %_48.i, 5
  %last_byte.us.pre.i = load i8, ptr %_49.i, align 1, !alias.scope !68, !noalias !71
  br i1 %_65.i, label %bb12.us.i, label %bb12.i, !prof !73

bb12.us.i:                                        ; preds = %bb12.lr.ph.i, %bb9.us.i
  %7 = phi i64 [ %16, %bb9.us.i ], [ %.promoted.i, %bb12.lr.ph.i ]
  %new_len.us.i = sub nuw i64 %index2.i, %7
  %_46.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %7
  %_3.i.us.i = icmp samesign ult i64 %new_len.us.i, 16
  br i1 %_3.i.us.i, label %bb5.preheader.i.us.i, label %bb2.i.us.i

bb2.i.us.i:                                       ; preds = %bb12.us.i
; call core::slice::memchr::memchr_aligned
  %8 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i), !noalias !74
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i

bb5.preheader.i.us.i:                             ; preds = %bb12.us.i
  %_64.not.i.us.i = icmp eq i64 %new_len.us.i, 0
  br i1 %_64.not.i.us.i, label %bb4.i.us.i, label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb5.preheader.i.us.i, %bb9.i.us.i
  %i.sroa.0.05.i.us.i = phi i64 [ %10, %bb9.i.us.i ], [ 0, %bb5.preheader.i.us.i ]
  %9 = getelementptr inbounds nuw i8, ptr %_46.us.i, i64 %i.sroa.0.05.i.us.i
  %_9.i.us.i = load i8, ptr %9, align 1, !alias.scope !75, !noalias !74, !noundef !3
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
  store i64 %16, ptr %3, align 8, !alias.scope !68, !noalias !71
  %_17.not.us.i = icmp ult i64 %16, %_12.i
  %_54.not.us.i = icmp ugt i64 %16, %_4.val1
  %or.cond.i = or i1 %_17.not.us.i, %_54.not.us.i
  br i1 %or.cond.i, label %bb9.us.i, label %bb19.us.i

bb19.us.i:                                        ; preds = %bb4.us.i
  %found_char.us.i = sub nuw i64 %16, %_12.i
  %_62.us.i = getelementptr inbounds nuw i8, ptr %_4.val, i64 %found_char.us.i
  %17 = tail call i32 @memcmp(ptr nonnull readonly align 1 %_62.us.i, ptr nonnull readonly align 1 %_10.i, i64 range(i64 0, -9223372036854775808) %_12.i), !alias.scope !78, !noalias !71
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
  %20 = tail call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef %last_byte.us.pre.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.i), !noalias !74
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
  %_9.i.i = load i8, ptr %23, align 1, !alias.scope !75, !noalias !74, !noundef !3
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
  store i64 %28, ptr %3, align 8, !alias.scope !68, !noalias !71
  %_17.not.i = icmp ult i64 %28, %_12.i
  %_54.not.i = icmp ugt i64 %28, %_4.val1
  %or.cond70.i = or i1 %_17.not.i, %_54.not.i
  br i1 %or.cond70.i, label %bb9.i, label %bb25.i

bb10.i:                                           ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.i, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i
  store i64 %index2.i, ptr %3, align 8, !alias.scope !68, !noalias !71
  br label %bb1.i

bb9.i:                                            ; preds = %bb4.i
  %_43.i = icmp ult i64 %index2.i, %28
  br i1 %_43.i, label %bb1.i, label %bb12.i

bb25.i:                                           ; preds = %bb4.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_12.i, i64 noundef 4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e52d3af24e8037dfb4f35693fba7d9f6) #21, !noalias !74
  unreachable

bb7:                                              ; preds = %bb19.us.i
  %i = load i64, ptr %self, align 8, !noundef !3
  %new_len = sub nuw i64 %found_char.us.i, %i
  %data = getelementptr inbounds nuw i8, ptr %_4.val, i64 %i
  store i64 %16, ptr %self, align 8
  br label %bb9

bb1.i:                                            ; preds = %bb9.i, %bb9.us.i, %bb2, %bb10.i
  store i8 1, ptr %0, align 1, !alias.scope !82
  %29 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %30 = load i8, ptr %29, align 8, !range !67, !alias.scope !82, !noundef !3
  %_3.i = trunc nuw i8 %30 to i1
  %i.pre.i = load i64, ptr %self, align 8, !alias.scope !82
  %.phi.trans.insert.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %i1.pre.i = load i64, ptr %.phi.trans.insert.i, align 8, !alias.scope !82
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
define internal fastcc void @_RNvMsz_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EE10dying_nextCs6kZbxSpzgup_18build_script_build(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 captures(none) dereferenceable(72) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_2 = load i64, ptr %0, align 8, !noundef !3
  %1 = icmp eq i64 %_2, 0
  br i1 %1, label %bb1, label %bb4

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !85)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !88)
  %self1.sroa.0.0.copyload.i.i = load i64, ptr %self, align 8, !alias.scope !91, !noalias !92
  %self1.sroa.5.0.self.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self1.sroa.5.sroa.0.0.copyload.i.i = load ptr, ptr %self1.sroa.5.0.self.sroa_idx.i.i, align 8, !alias.scope !91, !noalias !92
  %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self1.sroa.5.sroa.5.0.copyload.i.i = load ptr, ptr %self1.sroa.5.sroa.5.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !91, !noalias !92
  %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.sroa.5.sroa.6.0.copyload.i.i = load i64, ptr %self1.sroa.5.sroa.6.0.self1.sroa.5.0.self.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !91, !noalias !92
  store i64 0, ptr %self, align 8, !alias.scope !91, !noalias !92
  %2 = trunc nuw i64 %self1.sroa.0.0.copyload.i.i to i1
  br i1 %2, label %bb7.i.i, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit

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
  %5 = load ptr, ptr %_19.i.i, align 8, !noalias !94, !nonnull !3, !noundef !3
  %6 = add i64 %root.sroa.0.010.i.i, -1
  %7 = icmp eq i64 %6, 0
  br i1 %7, label %bb2.i, label %bb10.i.i

bb2.i:                                            ; preds = %bb10.i.i, %bb3.i.i, %bb7.i.i
  %_3.sroa.8.0.ph.i = phi ptr [ null, %bb3.i.i ], [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb7.i.i ], [ null, %bb10.i.i ]
  %_3.sroa.0.0.ph.i = phi ptr [ %self1.sroa.5.sroa.5.0.copyload.i.i, %bb3.i.i ], [ %self1.sroa.5.sroa.0.0.copyload.i.i, %bb7.i.i ], [ %5, %bb10.i.i ]
  %8 = ptrtoint ptr %_3.sroa.8.0.ph.i to i64
  %9 = load ptr, ptr %_3.sroa.0.0.ph.i, align 8, !noalias !95, !noundef !3
  %.not.i.i4.i.i = icmp eq ptr %9, null
  br i1 %.not.i.i4.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb2.i, %bb4.i.i
  %10 = phi ptr [ %11, %bb4.i.i ], [ %9, %bb2.i ]
  %edge.sroa.0.06.i.i = phi ptr [ %10, %bb4.i.i ], [ %_3.sroa.0.0.ph.i, %bb2.i ]
  %edge.sroa.3.05.i.i = phi i64 [ %_18.i.i.i.i, %bb4.i.i ], [ %8, %bb2.i ]
  %_18.i.i.i.i = add i64 %edge.sroa.3.05.i.i, 1
  %_10.not.i.i.i = icmp eq i64 %edge.sroa.3.05.i.i, 0
  %..i.i.i = select i1 %_10.not.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.06.i.i, i64 noundef %..i.i.i, i64 noundef 8) #16, !noalias !100
  %11 = load ptr, ptr %10, align 8, !noalias !95, !noundef !3
  %.not.i.i.i.i = icmp eq ptr %11, null
  br i1 %.not.i.i.i.i, label %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit.i, label %bb4.i.i

_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit.i: ; preds = %bb4.i.i, %bb2.i
  %edge.sroa.3.0.lcssa.i.i = phi i64 [ %8, %bb2.i ], [ %_18.i.i.i.i, %bb4.i.i ]
  %edge.sroa.0.0.lcssa.i.i = phi ptr [ %_3.sroa.0.0.ph.i, %bb2.i ], [ %10, %bb4.i.i ]
  %_10.not.i2.i.i = icmp eq i64 %edge.sroa.3.0.lcssa.i.i, 0
  %..i3.i.i = select i1 %_10.not.i2.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.0.lcssa.i.i, i64 noundef %..i3.i.i, i64 noundef 8) #16, !noalias !100
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit: ; preds = %bb1, %_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit.i
  store ptr null, ptr %_0, align 8
  br label %bb7

bb4:                                              ; preds = %start
  %12 = add i64 %_2, -1
  store i64 %12, ptr %0, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !101)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !104)
  %_3.i.i = load i64, ptr %self, align 8, !range !48, !alias.scope !107, !noalias !108, !noundef !3
  %13 = trunc nuw i64 %_3.i.i to i1
  br i1 %13, label %bb1.i.i, label %bb6.i

bb1.i.i:                                          ; preds = %bb4
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load ptr, ptr %14, align 8, !alias.scope !107, !noalias !108, !noundef !3
  %.not.i.i1 = icmp eq ptr %15, null
  %16 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %.not.i.i1, label %bb2.i.i, label %bb1.i.i.bb7.i_crit_edge

bb1.i.i.bb7.i_crit_edge:                          ; preds = %bb1.i.i
  %value.sroa.2.0.copyload.i.i.pre = load i64, ptr %16, align 8, !alias.scope !110, !noalias !113
  %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 24
  %value.sroa.3.0.copyload.i.i.pre = load i64, ptr %value.sroa.3.0.v.sroa_idx.i.i.phi.trans.insert, align 8, !alias.scope !110, !noalias !113
  br label %bb7.i

bb2.i.i:                                          ; preds = %bb1.i.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %18 = load i64, ptr %17, align 8, !alias.scope !107, !noalias !108, !noundef !3
  %self2.sroa.0.07.i.i = load ptr, ptr %16, align 8, !alias.scope !107, !noalias !108, !nonnull !3, !noundef !3
  %19 = icmp eq i64 %18, 0
  br i1 %19, label %bb11.i.i, label %bb12.i.i

bb11.i.i:                                         ; preds = %bb12.i.i, %bb2.i.i
  %self2.sroa.0.0.lcssa.i.i = phi ptr [ %self2.sroa.0.07.i.i, %bb2.i.i ], [ %self2.sroa.0.0.i.i, %bb12.i.i ]
  store i64 1, ptr %self, align 8, !alias.scope !107, !noalias !108
  br label %bb7.i

bb12.i.i:                                         ; preds = %bb2.i.i, %bb12.i.i
  %self2.sroa.0.09.i.i = phi ptr [ %self2.sroa.0.0.i.i, %bb12.i.i ], [ %self2.sroa.0.07.i.i, %bb2.i.i ]
  %self1.sroa.0.08.i.i = phi i64 [ %20, %bb12.i.i ], [ %18, %bb2.i.i ]
  %_19.i.i2 = getelementptr inbounds nuw i8, ptr %self2.sroa.0.09.i.i, i64 544
  %20 = add i64 %self1.sroa.0.08.i.i, -1
  %self2.sroa.0.0.i.i = load ptr, ptr %_19.i.i2, align 8, !noalias !115, !nonnull !3, !noundef !3
  %21 = icmp eq i64 %20, 0
  br i1 %21, label %bb11.i.i, label %bb12.i.i

bb7.i:                                            ; preds = %bb1.i.i.bb7.i_crit_edge, %bb11.i.i
  %value.sroa.3.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.3.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.2.0.copyload.i.i = phi i64 [ 0, %bb11.i.i ], [ %value.sroa.2.0.copyload.i.i.pre, %bb1.i.i.bb7.i_crit_edge ]
  %value.sroa.0.0.copyload.i.i = phi ptr [ %self2.sroa.0.0.lcssa.i.i, %bb11.i.i ], [ %15, %bb1.i.i.bb7.i_crit_edge ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !116)
  %value.sroa.2.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %value.sroa.3.0.v.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %22 = getelementptr inbounds nuw i8, ptr %value.sroa.0.0.copyload.i.i, i64 538
  %_2219.i.i.i.i = load i16, ptr %22, align 2, !noalias !117, !noundef !3
  %_1820.i.i.i.i = zext i16 %_2219.i.i.i.i to i64
  %_1621.i.i.i.i = icmp ult i64 %value.sroa.3.0.copyload.i.i, %_1820.i.i.i.i
  br i1 %_1621.i.i.i.i, label %bb12.i.i.i.i, label %bb13.i.i.i.i

bb13.i.i.i.i:                                     ; preds = %bb7.i, %bb7.i.i.i.i
  %edge.sroa.0.023.i.i.i.i = phi ptr [ %23, %bb7.i.i.i.i ], [ %value.sroa.0.0.copyload.i.i, %bb7.i ]
  %edge.sroa.5.022.i.i.i.i = phi i64 [ %_18.i.i.i.i.i.i, %bb7.i.i.i.i ], [ %value.sroa.2.0.copyload.i.i, %bb7.i ]
  %23 = load ptr, ptr %edge.sroa.0.023.i.i.i.i, align 8, !noalias !124, !noundef !3
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
  br label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit

bb3.i.i.i.i.i:                                    ; preds = %bb12.i.i.i.i
  %25 = getelementptr i8, ptr %edge.sroa.0.0.lcssa.i.i.i.i, i64 552
  %self9.i.i.i.i.i = getelementptr ptr, ptr %25, i64 %edge.sroa.8.0.lcssa.i.i.i.i
  br label %bb6.i.i.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i, %bb3.i.i.i.i.i
  %node.sroa.0.0.in.i.i.i.i.i = phi ptr [ %self9.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_29.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.in.i.i.i.i.i = phi i64 [ %edge.sroa.5.0.lcssa.i.i.i.i, %bb3.i.i.i.i.i ], [ %self1.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %self1.sroa.0.0.i.i.i.i.i = add i64 %self1.sroa.0.0.in.i.i.i.i.i, -1
  %node.sroa.0.0.i.i.i.i.i = load ptr, ptr %node.sroa.0.0.in.i.i.i.i.i, align 8, !noalias !129, !nonnull !3, !noundef !3
  %26 = icmp eq i64 %self1.sroa.0.0.i.i.i.i.i, 0
  %_29.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %node.sroa.0.0.i.i.i.i.i, i64 544
  br i1 %26, label %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit, label %bb6.i.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb13.i.i.i.i
  %_18.i.i.i.i.i.i = add i64 %edge.sroa.5.022.i.i.i.i, 1
  %27 = getelementptr inbounds nuw i8, ptr %edge.sroa.0.023.i.i.i.i, i64 536
  %28 = load i16, ptr %27, align 8, !noalias !124
  %_10.not.i.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i.i.i.i.i = select i1 %_10.not.i.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i.i.i.i.i, i64 noundef 8) #16, !noalias !133
  %29 = getelementptr inbounds nuw i8, ptr %23, i64 538
  %_22.i.i.i.i = load i16, ptr %29, align 2, !noalias !117, !noundef !3
  %_16.i.i.i.i = icmp ult i16 %28, %_22.i.i.i.i
  br i1 %_16.i.i.i.i, label %bb12.loopexit.i.i.i.i, label %bb13.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i
  %_10.not.i14.i.i.i.i = icmp eq i64 %edge.sroa.5.022.i.i.i.i, 0
  %..i15.i.i.i.i = select i1 %_10.not.i14.i.i.i.i, i64 544, i64 640
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %edge.sroa.0.023.i.i.i.i, i64 noundef %..i15.i.i.i.i, i64 noundef 8) #16, !noalias !133
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_93816f04728d387347072ad30618ff9c) #21
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !134

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.trap()
  unreachable

bb6.i:                                            ; preds = %bb4
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1df1e5171bffdf21494df69d159bd444) #20, !noalias !135
  unreachable

_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit: ; preds = %bb6.i.i.i.i.i, %bb2.i.i.i.i.i
  %self.sroa.7.0.ph.i.i.i = phi i64 [ %_11.i.i.i.i.i, %bb2.i.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %self.sroa.0.0.ph.i.i.i = phi ptr [ %edge.sroa.0.0.lcssa.i.i.i.i, %bb2.i.i.i.i.i ], [ %node.sroa.0.0.i.i.i.i.i, %bb6.i.i.i.i.i ]
  store ptr %self.sroa.0.0.ph.i.i.i, ptr %14, align 8, !alias.scope !110, !noalias !113
  store i64 0, ptr %value.sroa.2.0.v.sroa_idx.i.i, align 8, !alias.scope !110, !noalias !113
  store i64 %self.sroa.7.0.ph.i.i.i, ptr %value.sroa.3.0.v.sroa_idx.i.i, align 8, !alias.scope !110, !noalias !113
  store ptr %edge.sroa.0.0.lcssa.i.i.i.i, ptr %_0, align 8
  %_7.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %edge.sroa.5.0.lcssa.i.i.i.i, ptr %_7.sroa.4.0._0.sroa_idx, align 8
  %_7.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %edge.sroa.8.0.lcssa.i.i.i.i, ptr %_7.sroa.5.0._0.sroa_idx, align 8
  br label %bb7

bb7:                                              ; preds = %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit, %_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build.exit
  ret void
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6kZbxSpzgup_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !3, !align !16, !noundef !3
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !3
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #4

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #5

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #4

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #7

; <std::sys::process::unix::common::Command>::arg
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3arg(ptr noalias noundef align 8 dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <std::sys::process::unix::common::Command>::new
; Function Attrs: uwtable
declare void @_RNvMs_NtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6commonNtB4_7Command3new(ptr dead_on_unwind noalias noundef writable sret([200 x i8]) align 8 captures(none) dereferenceable(200), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #8

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

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
declare void @_RNvNtCs5sEH5CPMdak_3std7process4exit(i32 noundef) unnamed_addr #9

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
declare { i32, i1 } @llvm.umul.with.overflow.i32(i32, i32) #10

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #11

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #12

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #13

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #6

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #14 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCs6kZbxSpzgup_18build_script_build4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #15

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #8 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #11 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #12 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #13 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #16 = { nounwind }
attributes #17 = { cold }
attributes #18 = { cold noreturn nounwind }
attributes #19 = { noinline }
attributes #20 = { noreturn }
attributes #21 = { noinline noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs6kZbxSpzgup_18build_script_build: %_1.0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDINtNtNtB4_3ops8function5FnMutuEp6OutputINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorENtNtB4_6marker4SendNtB2X_4SyncEL_EECs6kZbxSpzgup_18build_script_build"}
!7 = !{i64 8}
!8 = !{i64 0, i64 -9223372036854775808}
!9 = !{i64 1, i64 0}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs6kZbxSpzgup_18build_script_build: %_1"}
!12 = distinct !{!12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common7CommandECs6kZbxSpzgup_18build_script_build"}
!13 = !{!14, !11}
!14 = distinct !{!14, !15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs6kZbxSpzgup_18build_script_build: %_1"}
!15 = distinct !{!15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys7process4unix6common13cstring_array12CStringArrayECs6kZbxSpzgup_18build_script_build"}
!16 = !{i64 1}
!17 = !{i64 4}
!18 = !{i32 0, i32 6}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build: %_1"}
!21 = distinct !{!21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build"}
!22 = !{!23}
!23 = distinct !{!23, !24, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs6kZbxSpzgup_18build_script_build: %self"}
!24 = distinct !{!24, !"_RNvXNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB2_8BTreeMapNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB14_EENtNtNtB1R_3ops4drop4Drop4dropCs6kZbxSpzgup_18build_script_build"}
!25 = !{!23, !20}
!26 = !{!27, !29, !23, !20}
!27 = distinct !{!27, !28, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs6kZbxSpzgup_18build_script_build: %self"}
!28 = distinct !{!28, !"_RNvXsy_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mapINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB17_EENtNtNtB1U_3ops4drop4Drop4dropCs6kZbxSpzgup_18build_script_build"}
!29 = distinct !{!29, !30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build: %_1"}
!30 = distinct !{!30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3map8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtB4_6option6OptionB1F_EEECs6kZbxSpzgup_18build_script_build"}
!31 = !{i64 0, i64 -9223372036854775807}
!32 = !{i64 18640003450157273}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs6kZbxSpzgup_18build_script_build: %_1"}
!35 = distinct !{!35, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0Cs6kZbxSpzgup_18build_script_build"}
!36 = !{!37}
!37 = distinct !{!37, !38, !"_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var: %_0"}
!38 = distinct !{!38, !"_RNvCs6kZbxSpzgup_18build_script_build13cargo_env_var"}
!39 = !{!40, !42}
!40 = distinct !{!40, !41, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build: %_0"}
!41 = distinct !{!41, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build"}
!42 = distinct !{!42, !41, !"_RINvMsk_NtCs5sEH5CPMdak_3std7processNtB6_7Command3newNtNtNtB8_3ffi6os_str8OsStringECs6kZbxSpzgup_18build_script_build: %program"}
!43 = !{!42}
!44 = !{!45}
!45 = distinct !{!45, !46, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECs6kZbxSpzgup_18build_script_build: %_1"}
!46 = distinct !{!46, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std7process6OutputNtNtNtB16_2io5error5ErrorEECs6kZbxSpzgup_18build_script_build"}
!47 = !{!"branch_weights", i32 2000, i32 6001}
!48 = !{i64 0, i64 2}
!49 = !{!50, !52}
!50 = distinct !{!50, !51, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build: %self.0"}
!51 = distinct !{!51, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build"}
!52 = distinct !{!52, !51, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build: %other.0"}
!53 = !{!54}
!54 = distinct !{!54, !55, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix: argument 0"}
!55 = distinct !{!55, !"_RNvMsB_NtCsjMrxcFdYDNN_4core3numm16from_ascii_radix"}
!56 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!57 = !{!"branch_weights", i32 2002, i32 2000}
!58 = !{!59}
!59 = distinct !{!59, !60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build: %_1"}
!60 = distinct !{!60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build"}
!61 = !{!62}
!62 = distinct !{!62, !63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build: %_1"}
!63 = distinct !{!63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build"}
!64 = !{!65}
!65 = distinct !{!65, !66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build: %_1"}
!66 = distinct !{!66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std7process6OutputECs6kZbxSpzgup_18build_script_build"}
!67 = !{i8 0, i8 2}
!68 = !{!69}
!69 = distinct !{!69, !70, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!70 = distinct !{!70, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!71 = !{!72}
!72 = distinct !{!72, !70, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!73 = !{!"branch_weights", i32 4000000, i32 4001}
!74 = !{!72, !69}
!75 = !{!76}
!76 = distinct !{!76, !77, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!77 = distinct !{!77, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!78 = !{!79, !81}
!79 = distinct !{!79, !80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build: %self.0"}
!80 = distinct !{!80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build"}
!81 = distinct !{!81, !80, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6kZbxSpzgup_18build_script_build: %other.0"}
!82 = !{!83}
!83 = distinct !{!83, !84, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs6kZbxSpzgup_18build_script_build: %self"}
!84 = distinct !{!84, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE7get_endCs6kZbxSpzgup_18build_script_build"}
!85 = !{!86}
!86 = distinct !{!86, !87, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %self"}
!87 = distinct !{!87, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE16deallocating_endNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build"}
!88 = !{!89}
!89 = distinct !{!89, !90, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs6kZbxSpzgup_18build_script_build: %self"}
!90 = distinct !{!90, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs6kZbxSpzgup_18build_script_build"}
!91 = !{!89, !86}
!92 = !{!93}
!93 = distinct !{!93, !90, !"_RNvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10take_frontCs6kZbxSpzgup_18build_script_build: %_0"}
!94 = !{!93, !89, !86}
!95 = !{!96, !98, !86}
!96 = distinct !{!96, !97, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs6kZbxSpzgup_18build_script_build: %_0"}
!97 = distinct !{!97, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs6kZbxSpzgup_18build_script_build"}
!98 = distinct !{!98, !99, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %ret"}
!99 = distinct !{!99, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build"}
!100 = !{!98, !86}
!101 = !{!102}
!102 = distinct !{!102, !103, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %self"}
!103 = distinct !{!103, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build"}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs6kZbxSpzgup_18build_script_build: %self"}
!106 = distinct !{!106, !"_RNvMsc_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB5_13LazyLeafRangeNtNtNtB7_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1J_EE10init_frontCs6kZbxSpzgup_18build_script_build"}
!107 = !{!105, !102}
!108 = !{!109}
!109 = distinct !{!109, !103, !"_RINvMsb_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtB6_13LazyLeafRangeNtNtNtB8_4node6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1K_EE27deallocating_next_uncheckedNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %_0"}
!110 = !{!111, !102}
!111 = distinct !{!111, !112, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs6kZbxSpzgup_18build_script_build: %v"}
!112 = distinct !{!112, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs6kZbxSpzgup_18build_script_build"}
!113 = !{!114, !109}
!114 = distinct !{!114, !112, !"_RINvNtNtNtCsdJPVW0sQgAG_5alloc11collections5btree3mem7replaceINtNtB4_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_4LeafENtB1y_4EdgeEIBY_IB1i_B1w_B1R_B2z_NtB1y_14LeafOrInternalENtB1y_2KVENCINvMsm_NtB4_8navigateBX_27deallocating_next_uncheckedNtNtB8_5alloc6GlobalE0ECs6kZbxSpzgup_18build_script_build: %ret"}
!115 = !{!105, !109, !102}
!116 = !{!111}
!117 = !{!118, !120, !121, !123, !114, !111, !109, !102}
!118 = distinct !{!118, !119, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %_0"}
!119 = distinct !{!119, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build"}
!120 = distinct !{!120, !119, !"_RINvMsj_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB8_4node6HandleINtB11_7NodeRefNtNtB11_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1S_ENtB1z_4LeafENtB1z_4EdgeE17deallocating_nextNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %self"}
!121 = distinct !{!121, !122, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs6kZbxSpzgup_18build_script_build: %val"}
!122 = distinct !{!122, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs6kZbxSpzgup_18build_script_build"}
!123 = distinct !{!123, !122, !"_RNCINvMsm_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtBa_4node6HandleINtB13_7NodeRefNtNtB13_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1U_ENtB1B_4LeafENtB1B_4EdgeE27deallocating_next_uncheckedNtNtBe_5alloc6GlobalE0Cs6kZbxSpzgup_18build_script_build: %leaf_edge"}
!124 = !{!125, !127, !118, !120, !121, !123, !114, !111, !109, !102}
!125 = distinct !{!125, !126, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs6kZbxSpzgup_18build_script_build: %_0"}
!126 = distinct !{!126, !"_RNvMse_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB5_7NodeRefNtNtB5_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1r_ENtB19_14LeafOrInternalE6ascendCs6kZbxSpzgup_18build_script_build"}
!127 = distinct !{!127, !128, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build: %ret"}
!128 = distinct !{!128, !"_RINvMsh_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree4nodeINtB6_7NodeRefNtNtB6_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1s_ENtB1a_14LeafOrInternalE21deallocate_and_ascendNtNtBc_5alloc6GlobalECs6kZbxSpzgup_18build_script_build"}
!129 = !{!130, !132, !118, !120, !121, !123, !114, !111, !109, !102}
!130 = distinct !{!130, !131, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs6kZbxSpzgup_18build_script_build: %_0"}
!131 = distinct !{!131, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs6kZbxSpzgup_18build_script_build"}
!132 = distinct !{!132, !131, !"_RNvMsp_NtNtNtCsdJPVW0sQgAG_5alloc11collections5btree8navigateINtNtB7_4node6HandleINtB10_7NodeRefNtNtB10_6marker5DyingNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringINtNtCsjMrxcFdYDNN_4core6option6OptionB1R_ENtB1y_14LeafOrInternalENtB1y_2KVE14next_leaf_edgeCs6kZbxSpzgup_18build_script_build: %self"}
!133 = !{!127, !118, !120, !121, !123, !114, !111, !109, !102}
!134 = !{!114, !111, !109, !102}
!135 = !{!109, !102}
