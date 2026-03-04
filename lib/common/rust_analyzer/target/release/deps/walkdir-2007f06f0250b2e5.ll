; ModuleID = 'walkdir.79bb5ea2e88512ab-cgu.0'
source_filename = "walkdir.79bb5ea2e88512ab-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"core::result::Result<dent::DirEntry, error::Error>" = type { i64, [6 x i64] }
%Ancestor = type { %"std::path::PathBuf" }
%"std::path::PathBuf" = type { %"std::ffi::os_str::OsString" }
%"std::ffi::os_str::OsString" = type { %"std::sys::os_str::bytes::Buf" }
%"std::sys::os_str::bytes::Buf" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}
%"dent::DirEntry" = type { %"std::path::PathBuf", i64, i64, i16, i8, [5 x i8] }
%DirList = type { i64, [7 x i64] }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNSNvYNCNvMs_CsarYwbBXrH4d_7walkdirNtBb_7WalkDir17sort_by_file_name0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtNtBb_4dent8DirEntryB1P_EE9call_once6vtableBb_, ptr @_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_ }>, align 8
@alloc_74d19fd2f6ebacd6902753249f054837 = private unnamed_addr constant [136 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/core/src/slice/sort/stable/quicksort.rs\00", align 1
@alloc_63b5b202754b5efb770c3fd338a9a466 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_74d19fd2f6ebacd6902753249f054837, [16 x i8] c"\87\00\00\00\00\00\00\00J\00\00\00\1F\00\00\00" }>, align 8
@alloc_36ce49ab0499b14b6f7dec8f3551378e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_74d19fd2f6ebacd6902753249f054837, [16 x i8] c"\87\00\00\00\00\00\00\00D\00\00\00\17\00\00\00" }>, align 8
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@alloc_9c940d26a66a2a68adaa819ca564fd28 = private unnamed_addr constant [97 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/walkdir-2.5.0/src/lib.rs\00", align 1
@alloc_47e9a95a7e6743c630b8dc44fd8334eb = private unnamed_addr constant [51 x i8] c"BUG: called is_same_file_system without root device", align 1
@alloc_5d4660e25ce5d46712141b88085cffca = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\E5\03\00\00\0E\00\00\00" }>, align 8
@alloc_a7455143ebb32770d194a02d72a823aa = private unnamed_addr constant [32 x i8] c"BUG: cannot pop from empty stack", align 1
@alloc_81e4be2212f19f75ac10f97b281265e5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\B7\03\00\00\1F\00\00\00" }>, align 8
@alloc_369d4fca9889d20ea478b2535f2f67dd = private unnamed_addr constant [33 x i8] c"BUG: list/path stacks out of sync", align 1
@alloc_f5c1e5e8c2c061740d120bfbf0725b0f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\B9\03\00\00#\00\00\00" }>, align 8
@alloc_fe0796cedbf57eec1a1715554c92530a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\8A\03\00\00\1C\00\00\00" }>, align 8
@alloc_3e766849636974ea221c0ef7c96d5e97 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\B1\03\00\00D\00\00\00" }>, align 8
@alloc_28fa3a3487cb94fdc7ce5cf2fd7f5672 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\88\03\00\00C\00\00\00" }>, align 8
@alloc_f7dab50b19f9d07673ad709515025da0 = private unnamed_addr constant [9 x i8] c"Some(...)", align 1
@alloc_37d2e53432a03a1f90b3e7253015eaf9 = private unnamed_addr constant [4 x i8] c"None", align 1
@alloc_d714d9dfd28cf6a9ad0e82cf8b1372ea = private unnamed_addr constant [14 x i8] c"WalkDirOptions", align 1
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsf_NtCsjMrxcFdYDNN_4core3fmtbNtB5_5Debug3fmt }>, align 8
@alloc_308a25eec56a6cc6390bb5730f0fa3f5 = private unnamed_addr constant [12 x i8] c"follow_links", align 1
@alloc_d71d874d004130da7ff6b3af219e7179 = private unnamed_addr constant [16 x i8] c"follow_root_link", align 1
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt }>, align 8
@alloc_320b930fa94d45d4ab31770bc0e0cda7 = private unnamed_addr constant [8 x i8] c"max_open", align 1
@alloc_c105ca25c81a06e2a92ad52809bc2a8a = private unnamed_addr constant [9 x i8] c"min_depth", align 1
@alloc_7c939dbf1615c4691bd3f4cfbc046ac7 = private unnamed_addr constant [9 x i8] c"max_depth", align 1
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir }>, align 8
@alloc_2d58c8bbe4563c561819bb5666a21d2a = private unnamed_addr constant [6 x i8] c"sorter", align 1
@alloc_e2a0bf94f60c5c26117b4eff9977d604 = private unnamed_addr constant [14 x i8] c"contents_first", align 1
@alloc_eeb3957c73ae0b537beb090154f28b19 = private unnamed_addr constant [16 x i8] c"same_file_system", align 1
@alloc_15295448e71ad0a49794d2a0b60d405b = private unnamed_addr constant [14 x i8] c"\09DirEntry(\C0\01)\00", align 1
@alloc_6c264a22652d2d2029a1b24c0820ee41 = private unnamed_addr constant [33 x i8] c"\1AIO error for operation on \C0\02: \C0\00", align 1
@alloc_884522af716a825d46454297f5f5d715 = private unnamed_addr constant [52 x i8] c"\18File system loop found: \C0\17 points to an ancestor \C0\00", align 1
@alloc_65a24c48507a3f07655bc752e59c1139 = private unnamed_addr constant [30 x i8] c"BUG: stack should be non-empty", align 1
@alloc_4f06d9fc34a8ce4ebe7bb54bb63ea865 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9c940d26a66a2a68adaa819ca564fd28, [16 x i8] c"`\00\00\00\00\00\00\00\CB\02\00\00\12\00\00\00" }>, align 8
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerNtB6_5Debug3fmtBA_ }>, align 8
@alloc_99ac8a81a24cac863217ce4a5cbfabea = private unnamed_addr constant [5 x i8] c"Error", align 1
@alloc_1496eca9c646eb8a70ec7074a69455e2 = private unnamed_addr constant [5 x i8] c"depth", align 1
@alloc_6c342f467cee9eb46aaa013cf1ccd49c = private unnamed_addr constant [5 x i8] c"inner", align 1
@vtable.5 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufENtNtB7_3fmt5Debug3fmtCsarYwbBXrH4d_7walkdir }>, align 8
@vtable.6 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir }>, align 8
@alloc_24a9d1d9c182d85dcff0523bfa2532d4 = private unnamed_addr constant [2 x i8] c"Io", align 1
@alloc_1713fdbdd59e3f6dd78509f861b8bb36 = private unnamed_addr constant [4 x i8] c"path", align 1
@alloc_753aceb6ee79a10c747ccb31227ed98f = private unnamed_addr constant [3 x i8] c"err", align 1
@vtable.7 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsG_NtCs5sEH5CPMdak_3std4pathNtB5_7PathBufNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@vtable.8 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCs5sEH5CPMdak_3std4path7PathBufNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir }>, align 8
@alloc_cbe560349b6a680eee426b88f9a47a11 = private unnamed_addr constant [4 x i8] c"Loop", align 1
@alloc_5b1139eb08b255c691d8263efc399d94 = private unnamed_addr constant [8 x i8] c"ancestor", align 1
@alloc_16ae073f6b5125d23ce16ce8c0c96634 = private unnamed_addr constant [5 x i8] c"child", align 1
@alloc_9535bf4c204f3eb9b19ec2c83e446e52 = private unnamed_addr constant [4 x i8] c"Some", align 1
@alloc_04d7ce44d7c86a9a02b346ab945bf155 = private unnamed_addr constant [40 x i8] c"description() is deprecated; use Display", align 1
@alloc_48bada85afbcc0a7dd8eb6b62a4956f4 = private unnamed_addr constant [22 x i8] c"file system loop found", align 1
@alloc_042f92cb4f8839bdce1a95ebac842941 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir, [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt }>, align 8
@vtable.9 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir, [16 x i8] c"\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt, ptr @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr @alloc_042f92cb4f8839bdce1a95ebac842941, ptr @_RNvXs8_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source, ptr @_RNvYNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idCsarYwbBXrH4d_7walkdir, ptr @_RNvYNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error11descriptionCsarYwbBXrH4d_7walkdir, ptr @_RNvXs8_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error5cause, ptr @_RNvYNtNtCsarYwbBXrH4d_7walkdir5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7provideB6_ }>, align 8
@alloc_83aa21f520eb34c82bb7b61f637df61f = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_, [16 x i8] c"8\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs0_NtCsarYwbBXrH4d_7walkdir5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt }>, align 8
@vtable.a = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_, [16 x i8] c"8\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs2_NtCsarYwbBXrH4d_7walkdir5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt, ptr @_RNvXs0_NtCsarYwbBXrH4d_7walkdir5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr @alloc_83aa21f520eb34c82bb7b61f637df61f, ptr @_RNvXs_NtCsarYwbBXrH4d_7walkdir5errorNtB4_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error5cause, ptr @_RNvYNtNtCsarYwbBXrH4d_7walkdir5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idB6_, ptr @_RNvXs_NtCsarYwbBXrH4d_7walkdir5errorNtB4_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error11description, ptr @_RNvXs_NtCsarYwbBXrH4d_7walkdir5errorNtB4_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error5cause, ptr @_RNvYNtNtCsarYwbBXrH4d_7walkdir5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7provideB6_ }>, align 8
@anon.e110384ad14f08951530940520241fc9.0 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 3100052035237948756 to ptr), ptr inttoptr (i64 -7966408041620907540 to ptr) }>, align 8
@anon.e110384ad14f08951530940520241fc9.1 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 -199690759461531251 to ptr), ptr inttoptr (i64 -16825625559297924 to ptr) }>, align 8

; <std::io::error::Error>::new::<walkdir::error::Error>
; Function Attrs: noinline uwtable
define noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBU_(i8 noundef range(i8 0, 42) %kind, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(56) %error) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !2
; call __rustc::__rust_alloc
  %0 = tail call noundef align 8 dereferenceable_or_null(56) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 56, i64 noundef range(i64 1, -9223372036854775807) 8) #26, !noalias !2
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb2.i.i.i, label %_RNvXs1_NtCsjMrxcFdYDNN_4core7convertNtNtCsarYwbBXrH4d_7walkdir5error5ErrorINtB5_4IntoINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB7_5error5ErrorNtNtB7_6marker4SendNtB2g_4SyncEL_EE4intoBC_.exit, !prof !9

bb2.i.i.i:                                        ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 56) #27
          to label %.noexc.i.i unwind label %cleanup.i.i.i, !noalias !10

.noexc.i.i:                                       ; preds = %bb2.i.i.i
  unreachable

cleanup.i.i.i:                                    ; preds = %bb2.i.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<walkdir::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(56) %error) #28
          to label %bb3.i.i.i unwind label %terminate.i.i.i

terminate.i.i.i:                                  ; preds = %cleanup.i.i.i
  %3 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !10
  unreachable

bb3.i.i.i:                                        ; preds = %cleanup.i.i.i
  resume { ptr, i32 } %2

_RNvXs1_NtCsjMrxcFdYDNN_4core7convertNtNtCsarYwbBXrH4d_7walkdir5error5ErrorINtB5_4IntoINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB7_5error5ErrorNtNtB7_6marker4SendNtB2g_4SyncEL_EE4intoBC_.exit: ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %0, ptr noundef nonnull readonly align 8 dereferenceable(56) %error, i64 56, i1 false)
; call <std::io::error::Error>::_new
  %_0 = tail call noundef nonnull ptr @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4__new(i8 noundef %kind, ptr noundef nonnull align 1 %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(80) @vtable.a)
  ret ptr %_0
}

; core::ptr::drop_in_place::<core::option::Option<std::path::PathBuf>>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !11, !noundef !12
  %1 = icmp eq i64 %0, -9223372036854775808
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb2.i.i.i4.i.i.i.i, %bb2, %start
  ret void

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13)
  %2 = icmp eq i64 %0, 0
  br i1 %2, label %bb1, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb2
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i = load ptr, ptr %3, align 8, !alias.scope !13, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !13
  br label %bb1
}

; core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !16, !noundef !12
  %1 = icmp eq i64 %0, -9223372036854775807
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %.val = load i64, ptr %2, align 8, !alias.scope !17
  %3 = icmp eq i64 %.val, 0
  br i1 %3, label %bb1, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb2
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val1 = load ptr, ptr %4, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !20
  br label %bb1

bb3:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26)
  %5 = icmp eq i64 %0, -9223372036854775808
  br i1 %5, label %bb5.i.i, label %bb8.i.i

bb5.i.i:                                          ; preds = %bb3
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29)
  %7 = load i64, ptr %6, align 8, !range !11, !alias.scope !32, !noundef !12
  %8 = icmp eq i64 %7, -9223372036854775808
  br i1 %8, label %bb4.i.i, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb5.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33)
  %9 = icmp eq i64 %7, 0
  br i1 %9, label %bb4.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i.i.i.i = load ptr, ptr %10, align 8, !alias.scope !36, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i.i, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !36
  br label %bb4.i.i

bb8.i.i:                                          ; preds = %bb3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37)
  %11 = icmp eq i64 %0, 0
  br i1 %11, label %bb7.i.i, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %bb8.i.i
  %12 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i.i = load ptr, ptr %12, align 8, !alias.scope !40, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !40
  br label %bb7.i.i

bb4.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i, %bb2.i.i.i, %bb5.i.i
  %13 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; call core::ptr::drop_in_place::<std::io::error::Error>
  tail call void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %13)
  br label %bb1

bb7.i.i:                                          ; preds = %bb2.i.i.i4.i.i.i.i.i.i, %bb8.i.i
  %14 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !41)
  %_1.val.i7.i.i = load i64, ptr %14, align 8, !alias.scope !44
  %15 = icmp eq i64 %_1.val.i7.i.i, 0
  br i1 %15, label %bb1, label %bb2.i.i.i4.i.i.i.i8.i.i

bb2.i.i.i4.i.i.i.i8.i.i:                          ; preds = %bb7.i.i
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %_1.val1.i9.i.i = load ptr, ptr %16, align 8, !alias.scope !44, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i9.i.i, i64 noundef %_1.val.i7.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !44
  br label %bb1

bb1:                                              ; preds = %bb2.i.i.i4.i.i.i.i8.i.i, %bb7.i.i, %bb4.i.i, %bb2.i.i.i4.i.i.i.i.i, %bb2
  ret void
}

; core::ptr::drop_in_place::<alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !12, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !12
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb5.i.i, %start
  %_3.sroa.0.0.i.i = phi i64 [ 0, %start ], [ %2, %bb5.i.i ]
  %_7.i.i = icmp eq i64 %_3.sroa.0.0.i.i, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb6.i.i
  %_6.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_1.val, i64 %_3.sroa.0.0.i.i
  %2 = add i64 %_3.sroa.0.0.i.i, 1
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef readonly align 8 dereferenceable(56) %_6.i.i)
          to label %bb6.i.i unwind label %cleanup.i.i

bb4.i.i:                                          ; preds = %bb3.i.i, %cleanup.i.i
  %_3.sroa.0.1.i.i = phi i64 [ %2, %cleanup.i.i ], [ %4, %bb3.i.i ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %_1.val1
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

cleanup.i.i:                                      ; preds = %bb5.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_1.val, i64 %_3.sroa.0.1.i.i
  %4 = add i64 %_3.sroa.0.1.i.i, 1
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef readonly align 8 dereferenceable(56) %_4.i.i) #28
          to label %bb4.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %bb3.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !45
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !48, !noundef !12
  %6 = icmp eq i64 %_1.val2, 0
  br i1 %6, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = mul nuw i64 %_1.val2, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #26
  br label %bb1

bb4:                                              ; preds = %bb6.i.i
  %_1.val4 = load i64, ptr %_1, align 8, !range !48, !noundef !12
  %7 = icmp eq i64 %_1.val4, 0
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1K_5error5ErrorEEEB1K_.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = mul nuw i64 %_1.val4, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #26
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1K_5error5ErrorEEEB1K_.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1K_5error5ErrorEEEB1K_.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %3
}

; core::ptr::drop_in_place::<walkdir::DirList>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !49, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775805
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !50)
  %1 = icmp eq i64 %0, -9223372036854775806
  br i1 %1, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %bb2
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !53)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !56)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !59)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !62)
  %_10.i.i.i.i.i = load ptr, ptr %2, align 8, !alias.scope !65, !nonnull !12, !noundef !12
  %3 = atomicrmw sub ptr %_10.i.i.i.i.i, i64 1 release, align 8, !noalias !65
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb2.i.i.i.i.i, label %bb1

bb2.i.i.i.i.i:                                    ; preds = %bb2.i
  fence acquire
; call <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  tail call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(16) %2) #30
  br label %bb1

bb3.i:                                            ; preds = %bb2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !66)
  %5 = icmp eq i64 %0, -9223372036854775807
  br i1 %5, label %bb1, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb3.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !69)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !72)
  %6 = icmp eq i64 %0, -9223372036854775808
  br i1 %6, label %bb5.i.i.i.i, label %bb8.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb2.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !75)
  %8 = load i64, ptr %7, align 8, !range !11, !alias.scope !78, !noundef !12
  %9 = icmp eq i64 %8, -9223372036854775808
  br i1 %9, label %bb4.i.i.i.i, label %bb2.i.i.i.i1.i

bb2.i.i.i.i1.i:                                   ; preds = %bb5.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  %10 = icmp eq i64 %8, 0
  br i1 %10, label %bb4.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i:                     ; preds = %bb2.i.i.i.i1.i
  %11 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i.i.i.i.i.i = load ptr, ptr %11, align 8, !alias.scope !82, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i.i.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !82
  br label %bb4.i.i.i.i

bb8.i.i.i.i:                                      ; preds = %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !83)
  %12 = icmp eq i64 %0, 0
  br i1 %12, label %bb7.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb8.i.i.i.i
  %13 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i.i.i.i = load ptr, ptr %13, align 8, !alias.scope !86, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i.i.i, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !86
  br label %bb7.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i1.i, %bb5.i.i.i.i
  %14 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; call core::ptr::drop_in_place::<std::io::error::Error>
  tail call void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %14)
  br label %bb1

bb7.i.i.i.i:                                      ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb8.i.i.i.i
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !87)
  %_1.val.i7.i.i.i.i = load i64, ptr %15, align 8, !alias.scope !90
  %16 = icmp eq i64 %_1.val.i7.i.i.i.i, 0
  br i1 %16, label %bb1, label %bb2.i.i.i4.i.i.i.i8.i.i.i.i

bb2.i.i.i4.i.i.i.i8.i.i.i.i:                      ; preds = %bb7.i.i.i.i
  %17 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %_1.val1.i9.i.i.i.i = load ptr, ptr %17, align 8, !alias.scope !90, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i9.i.i.i.i, i64 noundef %_1.val.i7.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !90
  br label %bb1

bb3:                                              ; preds = %start
  %18 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !91)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !94)
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self.val.i.i = load ptr, ptr %19, align 8, !alias.scope !97, !nonnull !12, !noundef !12
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %self.val1.i.i = load ptr, ptr %20, align 8, !alias.scope !97, !nonnull !12, !noundef !12
  %21 = ptrtoint ptr %self.val1.i.i to i64
  %22 = ptrtoint ptr %self.val.i.i to i64
  %23 = sub nuw i64 %21, %22
  %24 = udiv exact i64 %23, 56
  br label %bb6.i.i.i

cleanup.body.i.i:                                 ; preds = %bb4.i.i.i
  %25 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %capacity1.i.i.i.i = load i64, ptr %25, align 8, !alias.scope !97, !noundef !12
  %26 = icmp eq i64 %capacity1.i.i.i.i, 0
  br i1 %26, label %bb5.i.i, label %bb2.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %cleanup.body.i.i
  %ptr.i.i.i.i = load ptr, ptr %18, align 8, !alias.scope !97, !nonnull !12, !noundef !12
  %alloc_size.i.i.i.i.i.i.i.i = mul nuw i64 %capacity1.i.i.i.i, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #26, !noalias !97
  br label %bb5.i.i

bb6.i.i.i:                                        ; preds = %bb5.i.i.i, %bb3
  %_3.sroa.0.0.i.i.i = phi i64 [ 0, %bb3 ], [ %27, %bb5.i.i.i ]
  %_7.i.i.i = icmp eq i64 %_3.sroa.0.0.i.i.i, %24
  br i1 %_7.i.i.i, label %bb2.i.i1, label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %self.val.i.i, i64 %_3.sroa.0.0.i.i.i
  %27 = add nuw nsw i64 %_3.sroa.0.0.i.i.i, 1
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef readonly align 8 dereferenceable(56) %_6.i.i.i)
          to label %bb6.i.i.i unwind label %cleanup.i.i.i, !noalias !97

bb4.i.i.i:                                        ; preds = %bb3.i.i.i, %cleanup.i.i.i
  %_3.sroa.0.1.i.i.i = phi i64 [ %27, %cleanup.i.i.i ], [ %29, %bb3.i.i.i ]
  %_5.i.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i, %24
  br i1 %_5.i.i.i, label %cleanup.body.i.i, label %bb3.i.i.i

cleanup.i.i.i:                                    ; preds = %bb5.i.i.i
  %28 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i.i

bb3.i.i.i:                                        ; preds = %bb4.i.i.i
  %_4.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %self.val.i.i, i64 %_3.sroa.0.1.i.i.i
  %29 = add i64 %_3.sroa.0.1.i.i.i, 1
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef readonly align 8 dereferenceable(56) %_4.i.i.i) #28
          to label %bb4.i.i.i unwind label %terminate.i.i.i, !noalias !97

terminate.i.i.i:                                  ; preds = %bb3.i.i.i
  %30 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !98
  unreachable

bb2.i.i1:                                         ; preds = %bb6.i.i.i
  %31 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %capacity1.i.i3.i.i = load i64, ptr %31, align 8, !alias.scope !97, !noundef !12
  %32 = icmp eq i64 %capacity1.i.i3.i.i, 0
  br i1 %32, label %bb1, label %bb2.i.i.i.i.i4.i.i

bb2.i.i.i.i.i4.i.i:                               ; preds = %bb2.i.i1
  %ptr.i.i5.i.i = load ptr, ptr %18, align 8, !alias.scope !97, !nonnull !12, !noundef !12
  %alloc_size.i.i.i.i.i.i6.i.i = mul nuw i64 %capacity1.i.i3.i.i, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #26, !noalias !97
  br label %bb1

bb5.i.i:                                          ; preds = %bb2.i.i.i.i.i.i.i, %cleanup.body.i.i
  resume { ptr, i32 } %28

bb1:                                              ; preds = %bb2.i.i.i.i.i4.i.i, %bb2.i.i1, %bb2.i.i.i4.i.i.i.i8.i.i.i.i, %bb7.i.i.i.i, %bb4.i.i.i.i, %bb3.i, %bb2.i.i.i.i.i, %bb2.i
  ret void
}

; core::ptr::drop_in_place::<walkdir::WalkDir>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7WalkDirEBI_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val = load ptr, ptr %0, align 8, !align !101, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %.val1 = load ptr, ptr %1, align 8
  %2 = icmp eq ptr %.val, null
  br i1 %2, label %bb4, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
  %3 = icmp ne ptr %.val1, null
  tail call void @llvm.assume(i1 %3)
  %4 = load ptr, ptr %.val1, align 8, !invariant.load !12
  %.not.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i, label %bb3.i.i.i, label %is_not_null.i.i.i

is_not_null.i.i.i:                                ; preds = %bb2.i.i
  invoke void %4(ptr noundef nonnull %.val)
          to label %bb3.i.i.i unwind label %cleanup.i.i.i

bb3.i.i.i:                                        ; preds = %is_not_null.i.i.i, %bb2.i.i
  %5 = getelementptr inbounds nuw i8, ptr %.val1, i64 8
  %6 = load i64, ptr %5, align 8, !range !48, !invariant.load !12
  %7 = getelementptr inbounds nuw i8, ptr %.val1, i64 16
  %8 = load i64, ptr %7, align 8, !range !102, !invariant.load !12
  %9 = add i64 %8, -1
  %10 = icmp sgt i64 %9, -1
  tail call void @llvm.assume(i1 %10)
  %11 = icmp eq i64 %6, 0
  br i1 %11, label %bb4, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) %8) #26
  br label %bb4

cleanup.i.i.i:                                    ; preds = %is_not_null.i.i.i
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = getelementptr inbounds nuw i8, ptr %.val1, i64 8
  %14 = load i64, ptr %13, align 8, !range !48, !invariant.load !12
  %15 = getelementptr inbounds nuw i8, ptr %.val1, i64 16
  %16 = load i64, ptr %15, align 8, !range !102, !invariant.load !12
  %17 = add i64 %16, -1
  %18 = icmp sgt i64 %17, -1
  tail call void @llvm.assume(i1 %18)
  %19 = icmp eq i64 %14, 0
  br i1 %19, label %cleanup.body, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i: ; preds = %cleanup.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val, i64 noundef %14, i64 noundef range(i64 1, -9223372036854775807) %16) #26
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !103)
  %_1.val.i = load i64, ptr %_1, align 8, !alias.scope !103
  %20 = icmp eq i64 %_1.val.i, 0
  br i1 %20, label %bb1, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup.body
  %21 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i = load ptr, ptr %21, align 8, !alias.scope !103, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !103
  br label %bb1

bb4:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i, %bb3.i.i.i, %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !106)
  %_1.val.i2 = load i64, ptr %_1, align 8, !alias.scope !106
  %22 = icmp eq i64 %_1.val.i2, 0
  br i1 %22, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit5, label %bb2.i.i.i4.i.i.i.i3

bb2.i.i.i4.i.i.i.i3:                              ; preds = %bb4
  %23 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i4 = load ptr, ptr %23, align 8, !alias.scope !106, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i4, i64 noundef %_1.val.i2, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !106
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit5

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit5: ; preds = %bb4, %bb2.i.i.i4.i.i.i.i3
  ret void

bb1:                                              ; preds = %bb2.i.i.i4.i.i.i.i, %cleanup.body
  resume { ptr, i32 } %12
}

; core::ptr::drop_in_place::<same_file::Handle>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(24) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
; invoke <same_file::unix::Handle as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(24) %_1)
          to label %bb4.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val.i = load i32, ptr %1, align 8, !alias.scope !109, !noundef !12
  %2 = icmp eq i32 %.val.i, -1
  br i1 %2, label %bb1.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %cleanup.i
  %_5.i.i.i.i.i.i.i = tail call noundef i32 @close(i32 noundef %.val.i) #26
  br label %bb1.i

bb4.i:                                            ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val1.i = load i32, ptr %3, align 8, !alias.scope !109, !noundef !12
  %4 = icmp eq i32 %.val1.i, -1
  br i1 %4, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir.exit, label %bb2.i2.i

bb2.i2.i:                                         ; preds = %bb4.i
  %_5.i.i.i.i.i.i3.i = tail call noundef i32 @close(i32 noundef %.val1.i) #26
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir.exit

bb1.i:                                            ; preds = %bb2.i.i, %cleanup.i
  resume { ptr, i32 } %0

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir.exit: ; preds = %bb4.i, %bb2.i2.i
  ret void
}

; core::ptr::drop_in_place::<std::path::PathBuf>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsarYwbBXrH4d_7walkdir.exit, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %1, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #26
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsarYwbBXrH4d_7walkdir.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsarYwbBXrH4d_7walkdir.exit: ; preds = %start, %bb2.i.i.i4.i.i.i
  ret void
}

; core::ptr::drop_in_place::<walkdir::error::Error>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(56) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !112)
  %0 = load i64, ptr %_1, align 8, !range !11, !alias.scope !112, !noundef !12
  %1 = icmp eq i64 %0, -9223372036854775808
  br i1 %1, label %bb5.i, label %bb8.i

bb5.i:                                            ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !115)
  %3 = load i64, ptr %2, align 8, !range !11, !alias.scope !118, !noundef !12
  %4 = icmp eq i64 %3, -9223372036854775808
  br i1 %4, label %bb4.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb5.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !119)
  %5 = icmp eq i64 %3, 0
  br i1 %5, label %bb4.i, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %bb2.i.i
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i.i.i = load ptr, ptr %6, align 8, !alias.scope !122, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i, i64 noundef %3, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !122
  br label %bb4.i

bb8.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !123)
  %7 = icmp eq i64 %0, 0
  br i1 %7, label %bb7.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb8.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i.i = load ptr, ptr %8, align 8, !alias.scope !126, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !126
  br label %bb7.i

bb4.i:                                            ; preds = %bb2.i.i.i4.i.i.i.i.i.i, %bb2.i.i, %bb5.i
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; call core::ptr::drop_in_place::<std::io::error::Error>
  tail call void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %9)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_.exit

bb7.i:                                            ; preds = %bb2.i.i.i4.i.i.i.i.i, %bb8.i
  %10 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !127)
  %_1.val.i7.i = load i64, ptr %10, align 8, !alias.scope !130
  %11 = icmp eq i64 %_1.val.i7.i, 0
  br i1 %11, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_.exit, label %bb2.i.i.i4.i.i.i.i8.i

bb2.i.i.i4.i.i.i.i8.i:                            ; preds = %bb7.i
  %12 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %_1.val1.i9.i = load ptr, ptr %12, align 8, !alias.scope !130, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i9.i, i64 noundef %_1.val.i7.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !130
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_.exit: ; preds = %bb4.i, %bb7.i, %bb2.i.i.i4.i.i.i.i8.i
  ret void
}

; core::ptr::drop_in_place::<std::io::error::Error>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !12, !noundef !12
  %bits.i.i.i = ptrtoint ptr %_1.val to i64
  %_5.i.i.i = and i64 %bits.i.i.i, 3
  %switch.i.i = icmp eq i64 %_5.i.i.i, 1
  br i1 %switch.i.i, label %bb2.i2.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsarYwbBXrH4d_7walkdir.exit, !prof !131

bb2.i2.i.i:                                       ; preds = %start
  %0 = getelementptr i8, ptr %_1.val, i64 -1
  %1 = icmp ne ptr %0, null
  tail call void @llvm.assume(i1 %1)
  %_6.val.i.i.i.i = load ptr, ptr %0, align 8
  %2 = getelementptr i8, ptr %_1.val, i64 7
  %_6.val1.i.i.i.i = load ptr, ptr %2, align 8, !nonnull !12, !align !132, !noundef !12
  %3 = load ptr, ptr %_6.val1.i.i.i.i, align 8, !invariant.load !12
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
  %7 = load i64, ptr %6, align 8, !range !48, !invariant.load !12
  %8 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %9 = load i64, ptr %8, align 8, !range !102, !invariant.load !12
  %10 = add i64 %9, -1
  %11 = icmp sgt i64 %10, -1
  tail call void @llvm.assume(i1 %11)
  %12 = icmp eq i64 %7, 0
  br i1 %12, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsarYwbBXrH4d_7walkdir.exit.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) %9) #26
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsarYwbBXrH4d_7walkdir.exit.i.i.i

cleanup.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i
  %13 = landingpad { ptr, i32 }
          cleanup
  %14 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 8
  %15 = load i64, ptr %14, align 8, !range !48, !invariant.load !12
  %16 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i, i64 16
  %17 = load i64, ptr %16, align 8, !range !102, !invariant.load !12
  %18 = add i64 %17, -1
  %19 = icmp sgt i64 %18, -1
  tail call void @llvm.assume(i1 %19)
  %20 = icmp eq i64 %15, 0
  br i1 %20, label %bb1.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i, i64 noundef %15, i64 noundef range(i64 1, -9223372036854775807) %17) #26
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 24, i64 noundef 8) #26
  resume { ptr, i32 } %13

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsarYwbBXrH4d_7walkdir.exit.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 24, i64 noundef 8) #26
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsarYwbBXrH4d_7walkdir.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std2io5error14repr_bitpacked4ReprECsarYwbBXrH4d_7walkdir.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsarYwbBXrH4d_7walkdir.exit.i.i.i
  ret void
}

; core::slice::sort::stable::driftsort_main::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}, alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
; Function Attrs: noinline uwtable
define void @_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable14driftsort_mainINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1p_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBZ_7sort_byNCNvMs3_B1p_NtB1p_8IntoIter4pushs_0E0INtNtB2s_3vec3VecBZ_EEB1p_(ptr noalias noundef nonnull align 8 %v.0, i64 noundef range(i64 0, 164703072086692426) %v.1, ptr noalias noundef align 8 dereferenceable(8) %is_less) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %heap_buf = alloca [24 x i8], align 8
  %stack_buf = alloca [4096 x i8], align 8
  %_105 = lshr i64 %v.1, 1
  %v1 = sub nsw i64 %v.1, %_105
  %_0.sroa.0.0.i = tail call noundef i64 @llvm.umin.i64(i64 %v.1, i64 142857)
  %_0.sroa.0.0.i7 = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i, i64 %v1)
  %_0.sroa.0.0.i8 = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i7, i64 48)
  call void @llvm.lifetime.start.p0(i64 4096, ptr nonnull %stack_buf)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %heap_buf)
  %_15 = icmp ugt i64 %_0.sroa.0.0.i7, 73
  br i1 %_15, label %bb4, label %bb8

bb4:                                              ; preds = %start
  %_24.i.i.i.i = add i64 %_0.sroa.0.0.i8, -1
  %or.cond.i = icmp ugt i64 %_24.i.i.i.i, 164703072086692424
  br i1 %or.cond.i, label %bb3.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i, !prof !133

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i: ; preds = %bb4
  %_27.0.i.i.i.i = mul nuw nsw i64 %_24.i.i.i.i, 56
  %new_size2.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i, 56
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !134
; call __rustc::__rust_alloc
  %0 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i, i64 noundef range(i64 1, 9) 8) #26, !noalias !134
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb3.i.i, label %bb6

bb3.i.i:                                          ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i, %bb4
  %_4.sroa.4.0.ph.i.i = phi i64 [ 8, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ 0, %bb4 ]
  %_4.sroa.10.0.ph.i.i = phi i64 [ %new_size2.i.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i ], [ undef, %bb4 ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i, i64 %_4.sroa.10.0.ph.i.i) #27
  unreachable

cleanup:                                          ; preds = %bb8
  %2 = landingpad { ptr, i32 }
          cleanup
  br i1 %_15, label %bb13, label %common.resume

bb6:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i
  store i64 %_0.sroa.0.0.i8, ptr %heap_buf, align 8
  %_18.sroa.4.0.heap_buf.sroa_idx = getelementptr inbounds nuw i8, ptr %heap_buf, i64 8
  store ptr %0, ptr %_18.sroa.4.0.heap_buf.sroa_idx, align 8
  %_18.sroa.5.0.heap_buf.sroa_idx = getelementptr inbounds nuw i8, ptr %heap_buf, i64 16
  store i64 0, ptr %_18.sroa.5.0.heap_buf.sroa_idx, align 8
  br label %bb8

bb8:                                              ; preds = %bb6, %start
  %_10.i10 = phi ptr [ undef, %start ], [ %0, %bb6 ]
  %stack_scratch.sroa.4.0 = phi i64 [ 73, %start ], [ %_0.sroa.0.0.i8, %bb6 ]
  %stack_buf.pn = phi ptr [ %stack_buf, %start ], [ %0, %bb6 ]
  %eager_sort = icmp samesign ult i64 %v.1, 65
; invoke core::slice::sort::stable::drift::sort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  invoke fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift4sortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBW_7sort_byNCNvMs3_B1m_NtB1m_8IntoIter4pushs_0E0EB1m_(ptr noalias noundef nonnull align 8 %v.0, i64 noundef %v.1, ptr noalias noundef nonnull align 8 %stack_buf.pn, i64 noundef %stack_scratch.sroa.4.0, i1 noundef zeroext %eager_sort, ptr noalias noundef align 8 dereferenceable(8) %is_less)
          to label %bb9 unwind label %cleanup

bb9:                                              ; preds = %bb8
  br i1 %_15, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_.exit, label %bb10

bb10:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_.exit, %bb9
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %heap_buf)
  call void @llvm.lifetime.end.p0(i64 4096, ptr nonnull %stack_buf)
  ret void

common.resume:                                    ; preds = %cleanup, %bb13
  resume { ptr, i32 } %2

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_.exit: ; preds = %bb9
  %alloc_size.i.i.i.i7.i = mul nuw i64 %_0.sroa.0.0.i8, 56
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_10.i10, i64 noundef %alloc_size.i.i.i.i7.i, i64 noundef range(i64 1, -9223372036854775807) 8) #26, !noalias !139
  br label %bb10

bb13:                                             ; preds = %cleanup
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_(ptr noalias noundef align 8 dereferenceable(24) %heap_buf) #28
          to label %common.resume unwind label %terminate

terminate:                                        ; preds = %bb13
  %3 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29
  unreachable
}

; core::slice::sort::shared::pivot::median3_rec::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
; Function Attrs: uwtable
define internal fastcc noundef nonnull ptr @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot11median3_recINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_(ptr noundef nonnull %0, ptr noundef nonnull %1, ptr noundef nonnull %2, i64 noundef range(i64 0, 20587884010836554) %n, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) unnamed_addr #2 {
start:
  %_6 = icmp samesign ugt i64 %n, 7
  br i1 %_6, label %bb1, label %bb6

bb1:                                              ; preds = %start
  %n84 = lshr i64 %n, 3
  %count = shl nuw nsw i64 %n84, 2
  %_10 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %0, i64 %count
  %count1 = mul nuw nsw i64 %n84, 7
  %_13 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %0, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  %3 = tail call fastcc noundef ptr @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot11median3_recINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_(ptr noundef %0, ptr noundef %_10, ptr noundef %_13, i64 noundef %n84, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  %_16 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %1, i64 %count
  %_18 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %1, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  %4 = tail call fastcc noundef ptr @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot11median3_recINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_(ptr noundef %1, ptr noundef %_16, ptr noundef %_18, i64 noundef %n84, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  %_20 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %2, i64 %count
  %_22 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %2, i64 %count1
; call core::slice::sort::shared::pivot::median3_rec::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  %5 = tail call fastcc noundef ptr @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot11median3_recINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_(ptr noundef %2, ptr noundef %_20, ptr noundef %_22, i64 noundef %n84, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  br label %bb6

bb6:                                              ; preds = %start, %bb1
  %c.sroa.0.0 = phi ptr [ %5, %bb1 ], [ %2, %start ]
  %b.sroa.0.0 = phi ptr [ %4, %bb1 ], [ %1, %start ]
  %a.sroa.0.0 = phi ptr [ %3, %bb1 ], [ %0, %start ]
  %is_less.val6 = load ptr, ptr %is_less, align 8, !nonnull !12, !align !132, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !142)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !145)
  %_7.val.i = load ptr, ptr %is_less.val6, align 8, !noalias !147
  tail call void @llvm.experimental.noalias.scope.decl(metadata !148)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !151)
  %6 = load i64, ptr %a.sroa.0.0, align 8, !range !16, !alias.scope !153, !noalias !154, !noundef !12
  %.not.i.i = icmp ne i64 %6, -9223372036854775807
  %7 = load i64, ptr %b.sroa.0.0, align 8, !range !16, !alias.scope !154, !noalias !153, !noundef !12
  %.not3.i.i = icmp eq i64 %7, -9223372036854775807
  %.not3.i.not.i = xor i1 %.not3.i.i, true
  %brmerge.i = or i1 %.not.i.i, %.not3.i.not.i
  %.not3.i.mux.i = and i1 %.not.i.i, %.not3.i.i
  br i1 %brmerge.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb6
  %a1.i.i = getelementptr inbounds nuw i8, ptr %a.sroa.0.0, i64 8
  %b2.i.i = getelementptr inbounds nuw i8, ptr %b.sroa.0.0, i64 8
  %8 = icmp ne ptr %_7.val.i, null
  tail call void @llvm.assume(i1 %8)
  %_11.i.i = load ptr, ptr %_7.val.i, align 8, !noalias !155, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i = load ptr, ptr %_11.i.i, align 8, !noalias !155, !nonnull !12, !noundef !12
  %9 = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_14.1.i.i = load ptr, ptr %9, align 8, !noalias !155, !nonnull !12, !align !132, !noundef !12
  %10 = getelementptr inbounds nuw i8, ptr %_14.1.i.i, i64 32
  %11 = load ptr, ptr %10, align 8, !invariant.load !12, !noalias !155, !nonnull !12
  %12 = tail call noundef i8 %11(ptr noundef nonnull align 1 %_14.0.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i) #31
  %13 = icmp eq i8 %12, -1
  %is_less.val5.pre = load ptr, ptr %is_less, align 8
  %_7.val.i7.pre = load ptr, ptr %is_less.val5.pre, align 8, !noalias !156
  %.pre = load i64, ptr %a.sroa.0.0, align 8, !range !16, !alias.scope !160, !noalias !163
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit: ; preds = %bb6, %bb7.i.i
  %14 = phi i64 [ %.pre, %bb7.i.i ], [ %6, %bb6 ]
  %_7.val.i7 = phi ptr [ %_7.val.i7.pre, %bb7.i.i ], [ %_7.val.i, %bb6 ]
  %_0.sroa.0.0.i.i = phi i1 [ %13, %bb7.i.i ], [ %.not3.i.mux.i, %bb6 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !165)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !166)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !167)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !168)
  %.not.i.i8 = icmp ne i64 %14, -9223372036854775807
  %15 = load i64, ptr %c.sroa.0.0, align 8, !range !16, !alias.scope !163, !noalias !160, !noundef !12
  %.not3.i.i9 = icmp eq i64 %15, -9223372036854775807
  %.not3.i.not.i10 = xor i1 %.not3.i.i9, true
  %brmerge.i11 = or i1 %.not.i.i8, %.not3.i.not.i10
  %.not3.i.mux.i12 = and i1 %.not.i.i8, %.not3.i.i9
  br i1 %brmerge.i11, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20, label %bb7.i.i13

bb7.i.i13:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit
  %a1.i.i14 = getelementptr inbounds nuw i8, ptr %a.sroa.0.0, i64 8
  %b2.i.i15 = getelementptr inbounds nuw i8, ptr %c.sroa.0.0, i64 8
  %16 = icmp ne ptr %_7.val.i7, null
  tail call void @llvm.assume(i1 %16)
  %_11.i.i16 = load ptr, ptr %_7.val.i7, align 8, !noalias !169, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i17 = load ptr, ptr %_11.i.i16, align 8, !noalias !169, !nonnull !12, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %_11.i.i16, i64 8
  %_14.1.i.i18 = load ptr, ptr %17, align 8, !noalias !169, !nonnull !12, !align !132, !noundef !12
  %18 = getelementptr inbounds nuw i8, ptr %_14.1.i.i18, i64 32
  %19 = load ptr, ptr %18, align 8, !invariant.load !12, !noalias !169, !nonnull !12
  %20 = tail call noundef i8 %19(ptr noundef nonnull align 1 %_14.0.i.i17, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i14, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i15) #31
  %21 = icmp eq i8 %20, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, %bb7.i.i13
  %_0.sroa.0.0.i.i19 = phi i1 [ %21, %bb7.i.i13 ], [ %.not3.i.mux.i12, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ]
  %22 = xor i1 %_0.sroa.0.0.i.i, %_0.sroa.0.0.i.i19
  br i1 %22, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot7median3INtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1p_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBZ_7sort_byNCNvMs3_B1p_NtB1p_8IntoIter4pushs_0E0EB1p_.exit, label %bb3.i

bb3.i:                                            ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20
  %is_less.val = load ptr, ptr %is_less, align 8, !nonnull !12, !align !132, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !170)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !173)
  %_7.val.i21 = load ptr, ptr %is_less.val, align 8, !noalias !175
  tail call void @llvm.experimental.noalias.scope.decl(metadata !176)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !179)
  %23 = load i64, ptr %b.sroa.0.0, align 8, !range !16, !alias.scope !181, !noalias !182, !noundef !12
  %.not.i.i22 = icmp ne i64 %23, -9223372036854775807
  %24 = load i64, ptr %c.sroa.0.0, align 8, !range !16, !alias.scope !182, !noalias !181, !noundef !12
  %.not3.i.i23 = icmp eq i64 %24, -9223372036854775807
  %.not3.i.not.i24 = xor i1 %.not3.i.i23, true
  %brmerge.i25 = or i1 %.not.i.i22, %.not3.i.not.i24
  %.not3.i.mux.i26 = and i1 %.not.i.i22, %.not3.i.i23
  br i1 %brmerge.i25, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34, label %bb7.i.i27

bb7.i.i27:                                        ; preds = %bb3.i
  %a1.i.i28 = getelementptr inbounds nuw i8, ptr %b.sroa.0.0, i64 8
  %b2.i.i29 = getelementptr inbounds nuw i8, ptr %c.sroa.0.0, i64 8
  %25 = icmp ne ptr %_7.val.i21, null
  tail call void @llvm.assume(i1 %25)
  %_11.i.i30 = load ptr, ptr %_7.val.i21, align 8, !noalias !183, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i31 = load ptr, ptr %_11.i.i30, align 8, !noalias !183, !nonnull !12, !noundef !12
  %26 = getelementptr inbounds nuw i8, ptr %_11.i.i30, i64 8
  %_14.1.i.i32 = load ptr, ptr %26, align 8, !noalias !183, !nonnull !12, !align !132, !noundef !12
  %27 = getelementptr inbounds nuw i8, ptr %_14.1.i.i32, i64 32
  %28 = load ptr, ptr %27, align 8, !invariant.load !12, !noalias !183, !nonnull !12
  %29 = tail call noundef i8 %28(ptr noundef nonnull align 1 %_14.0.i.i31, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i28, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i29) #31
  %30 = icmp eq i8 %29, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34: ; preds = %bb3.i, %bb7.i.i27
  %_0.sroa.0.0.i.i33 = phi i1 [ %30, %bb7.i.i27 ], [ %.not3.i.mux.i26, %bb3.i ]
  %_12.i = xor i1 %_0.sroa.0.0.i.i, %_0.sroa.0.0.i.i33
  %c.b.i = select i1 %_12.i, ptr %c.sroa.0.0, ptr %b.sroa.0.0
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot7median3INtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1p_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBZ_7sort_byNCNvMs3_B1p_NtB1p_8IntoIter4pushs_0E0EB1p_.exit

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot7median3INtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1p_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBZ_7sort_byNCNvMs3_B1p_NtB1p_8IntoIter4pushs_0E0EB1p_.exit: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34
  %_0.sroa.0.0.i = phi ptr [ %a.sroa.0.0, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20 ], [ %c.b.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34 ]
  ret ptr %_0.sroa.0.0.i
}

; core::slice::sort::shared::smallsort::sort4_stable::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort12sort4_stableINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1z_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB19_7sort_byNCNvMs3_B1z_NtB1z_8IntoIter4pushs_0E0EB1z_(ptr noundef nonnull captures(address, read_provenance) %v_base, ptr noundef nonnull writeonly captures(none) initializes((0, 224)) %dst, ptr readonly captures(address_is_null) %is_less.0.val) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_7 = getelementptr inbounds nuw i8, ptr %v_base, i64 56
  %0 = icmp ne ptr %is_less.0.val, null
  tail call void @llvm.assume(i1 %0)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !184)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !187)
  %_7.val.i = load ptr, ptr %is_less.0.val, align 8, !noalias !189
  tail call void @llvm.experimental.noalias.scope.decl(metadata !190)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !193)
  %1 = load i64, ptr %_7, align 8, !range !16, !alias.scope !195, !noalias !196, !noundef !12
  %.not.i.i = icmp ne i64 %1, -9223372036854775807
  %2 = load i64, ptr %v_base, align 8, !range !16, !alias.scope !196, !noalias !195, !noundef !12
  %.not3.i.i = icmp eq i64 %2, -9223372036854775807
  %.not3.i.not.i = xor i1 %.not3.i.i, true
  %brmerge.i = or i1 %.not.i.i, %.not3.i.not.i
  %.not3.i.mux.i = and i1 %.not.i.i, %.not3.i.i
  br i1 %brmerge.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, label %bb7.i.i

bb7.i.i:                                          ; preds = %start
  %a1.i.i = getelementptr inbounds nuw i8, ptr %v_base, i64 64
  %b2.i.i = getelementptr inbounds nuw i8, ptr %v_base, i64 8
  %3 = icmp ne ptr %_7.val.i, null
  tail call void @llvm.assume(i1 %3)
  %_11.i.i = load ptr, ptr %_7.val.i, align 8, !noalias !197, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i = load ptr, ptr %_11.i.i, align 8, !noalias !197, !nonnull !12, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_14.1.i.i = load ptr, ptr %4, align 8, !noalias !197, !nonnull !12, !align !132, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %_14.1.i.i, i64 32
  %6 = load ptr, ptr %5, align 8, !invariant.load !12, !noalias !197, !nonnull !12
  %7 = tail call noundef i8 %6(ptr noundef nonnull align 1 %_14.0.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i) #31
  %8 = icmp eq i8 %7, -1
  %_7.val.i11.pre = load ptr, ptr %is_less.0.val, align 8, !noalias !198
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit: ; preds = %start, %bb7.i.i
  %_7.val.i11 = phi ptr [ %_7.val.i11.pre, %bb7.i.i ], [ %_7.val.i, %start ]
  %_0.sroa.0.0.i.i = phi i1 [ %8, %bb7.i.i ], [ %.not3.i.mux.i, %start ]
  %_12 = getelementptr inbounds nuw i8, ptr %v_base, i64 168
  %_14 = getelementptr inbounds nuw i8, ptr %v_base, i64 112
  tail call void @llvm.experimental.noalias.scope.decl(metadata !202)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !203)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !204)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !207)
  %9 = load i64, ptr %_12, align 8, !range !16, !alias.scope !209, !noalias !210, !noundef !12
  %.not.i.i12 = icmp ne i64 %9, -9223372036854775807
  %10 = load i64, ptr %_14, align 8, !range !16, !alias.scope !210, !noalias !209, !noundef !12
  %.not3.i.i13 = icmp eq i64 %10, -9223372036854775807
  %.not3.i.not.i14 = xor i1 %.not3.i.i13, true
  %brmerge.i15 = or i1 %.not.i.i12, %.not3.i.not.i14
  %.not3.i.mux.i16 = and i1 %.not.i.i12, %.not3.i.i13
  br i1 %brmerge.i15, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24, label %bb7.i.i17

bb7.i.i17:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit
  %a1.i.i18 = getelementptr inbounds nuw i8, ptr %v_base, i64 176
  %b2.i.i19 = getelementptr inbounds nuw i8, ptr %v_base, i64 120
  %11 = icmp ne ptr %_7.val.i11, null
  tail call void @llvm.assume(i1 %11)
  %_11.i.i20 = load ptr, ptr %_7.val.i11, align 8, !noalias !211, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i21 = load ptr, ptr %_11.i.i20, align 8, !noalias !211, !nonnull !12, !noundef !12
  %12 = getelementptr inbounds nuw i8, ptr %_11.i.i20, i64 8
  %_14.1.i.i22 = load ptr, ptr %12, align 8, !noalias !211, !nonnull !12, !align !132, !noundef !12
  %13 = getelementptr inbounds nuw i8, ptr %_14.1.i.i22, i64 32
  %14 = load ptr, ptr %13, align 8, !invariant.load !12, !noalias !211, !nonnull !12
  %15 = tail call noundef i8 %14(ptr noundef nonnull align 1 %_14.0.i.i21, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i18, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i19) #31
  %16 = icmp eq i8 %15, -1
  %_7.val.i25.pre = load ptr, ptr %is_less.0.val, align 8, !noalias !212
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, %bb7.i.i17
  %_7.val.i25 = phi ptr [ %_7.val.i25.pre, %bb7.i.i17 ], [ %_7.val.i11, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ]
  %_0.sroa.0.0.i.i23 = phi i1 [ %16, %bb7.i.i17 ], [ %.not3.i.mux.i16, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ]
  %count = zext i1 %_0.sroa.0.0.i.i to i64
  %a = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v_base, i64 %count
  %_19 = xor i1 %_0.sroa.0.0.i.i, true
  %count1 = zext i1 %_19 to i64
  %b = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v_base, i64 %count1
  %count2 = select i1 %_0.sroa.0.0.i.i23, i64 3, i64 2
  %c = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v_base, i64 %count2
  %count3 = select i1 %_0.sroa.0.0.i.i23, i64 2, i64 3
  %d = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v_base, i64 %count3
  tail call void @llvm.experimental.noalias.scope.decl(metadata !216)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !217)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !218)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !221)
  %17 = load i64, ptr %c, align 8, !range !16, !alias.scope !223, !noalias !224, !noundef !12
  %.not.i.i26 = icmp ne i64 %17, -9223372036854775807
  %18 = load i64, ptr %a, align 8, !range !16, !alias.scope !224, !noalias !223, !noundef !12
  %.not3.i.i27 = icmp eq i64 %18, -9223372036854775807
  %.not3.i.not.i28 = xor i1 %.not3.i.i27, true
  %brmerge.i29 = or i1 %.not.i.i26, %.not3.i.not.i28
  %.not3.i.mux.i30 = and i1 %.not.i.i26, %.not3.i.i27
  br i1 %brmerge.i29, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38, label %bb7.i.i31

bb7.i.i31:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24
  %a1.i.i32 = getelementptr inbounds nuw i8, ptr %c, i64 8
  %b2.i.i33 = getelementptr inbounds nuw i8, ptr %a, i64 8
  %19 = icmp ne ptr %_7.val.i25, null
  tail call void @llvm.assume(i1 %19)
  %_11.i.i34 = load ptr, ptr %_7.val.i25, align 8, !noalias !225, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i35 = load ptr, ptr %_11.i.i34, align 8, !noalias !225, !nonnull !12, !noundef !12
  %20 = getelementptr inbounds nuw i8, ptr %_11.i.i34, i64 8
  %_14.1.i.i36 = load ptr, ptr %20, align 8, !noalias !225, !nonnull !12, !align !132, !noundef !12
  %21 = getelementptr inbounds nuw i8, ptr %_14.1.i.i36, i64 32
  %22 = load ptr, ptr %21, align 8, !invariant.load !12, !noalias !225, !nonnull !12
  %23 = tail call noundef i8 %22(ptr noundef nonnull align 1 %_14.0.i.i35, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i32, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i33) #31
  %24 = icmp eq i8 %23, -1
  %_7.val.i39.pre = load ptr, ptr %is_less.0.val, align 8, !noalias !226
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24, %bb7.i.i31
  %_7.val.i39 = phi ptr [ %_7.val.i39.pre, %bb7.i.i31 ], [ %_7.val.i25, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24 ]
  %_0.sroa.0.0.i.i37 = phi i1 [ %24, %bb7.i.i31 ], [ %.not3.i.mux.i30, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit24 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !230)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !231)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !232)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !235)
  %25 = load i64, ptr %d, align 8, !range !16, !alias.scope !237, !noalias !238, !noundef !12
  %.not.i.i40 = icmp ne i64 %25, -9223372036854775807
  %26 = load i64, ptr %b, align 8, !range !16, !alias.scope !238, !noalias !237, !noundef !12
  %.not3.i.i41 = icmp eq i64 %26, -9223372036854775807
  %.not3.i.not.i42 = xor i1 %.not3.i.i41, true
  %brmerge.i43 = or i1 %.not.i.i40, %.not3.i.not.i42
  %.not3.i.mux.i44 = and i1 %.not.i.i40, %.not3.i.i41
  br i1 %brmerge.i43, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52, label %bb7.i.i45

bb7.i.i45:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38
  %a1.i.i46 = getelementptr inbounds nuw i8, ptr %d, i64 8
  %b2.i.i47 = getelementptr inbounds nuw i8, ptr %b, i64 8
  %27 = icmp ne ptr %_7.val.i39, null
  tail call void @llvm.assume(i1 %27)
  %_11.i.i48 = load ptr, ptr %_7.val.i39, align 8, !noalias !239, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i49 = load ptr, ptr %_11.i.i48, align 8, !noalias !239, !nonnull !12, !noundef !12
  %28 = getelementptr inbounds nuw i8, ptr %_11.i.i48, i64 8
  %_14.1.i.i50 = load ptr, ptr %28, align 8, !noalias !239, !nonnull !12, !align !132, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %_14.1.i.i50, i64 32
  %30 = load ptr, ptr %29, align 8, !invariant.load !12, !noalias !239, !nonnull !12
  %31 = tail call noundef i8 %30(ptr noundef nonnull align 1 %_14.0.i.i49, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i46, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i47) #31
  %32 = icmp eq i8 %31, -1
  %_7.val.i53.pre = load ptr, ptr %is_less.0.val, align 8, !noalias !240
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38, %bb7.i.i45
  %_7.val.i53 = phi ptr [ %_7.val.i53.pre, %bb7.i.i45 ], [ %_7.val.i39, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38 ]
  %_0.sroa.0.0.i.i51 = phi i1 [ %32, %bb7.i.i45 ], [ %.not3.i.mux.i44, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit38 ]
  %33 = select i1 %_0.sroa.0.0.i.i51, ptr %c, ptr %b, !unpredictable !12
  %34 = select i1 %_0.sroa.0.0.i.i37, ptr %a, ptr %33, !unpredictable !12
  %35 = select i1 %_0.sroa.0.0.i.i37, ptr %b, ptr %c, !unpredictable !12
  %36 = select i1 %_0.sroa.0.0.i.i51, ptr %d, ptr %35, !unpredictable !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !244)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !245)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !246)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !249)
  %37 = load i64, ptr %36, align 8, !range !16, !alias.scope !251, !noalias !252, !noundef !12
  %.not.i.i54 = icmp ne i64 %37, -9223372036854775807
  %38 = load i64, ptr %34, align 8, !range !16, !alias.scope !252, !noalias !251, !noundef !12
  %.not3.i.i55 = icmp eq i64 %38, -9223372036854775807
  %.not3.i.not.i56 = xor i1 %.not3.i.i55, true
  %brmerge.i57 = or i1 %.not.i.i54, %.not3.i.not.i56
  %.not3.i.mux.i58 = and i1 %.not.i.i54, %.not3.i.i55
  br i1 %brmerge.i57, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit66, label %bb7.i.i59

bb7.i.i59:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52
  %a1.i.i60 = getelementptr inbounds nuw i8, ptr %36, i64 8
  %b2.i.i61 = getelementptr inbounds nuw i8, ptr %34, i64 8
  %39 = icmp ne ptr %_7.val.i53, null
  tail call void @llvm.assume(i1 %39)
  %_11.i.i62 = load ptr, ptr %_7.val.i53, align 8, !noalias !253, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i63 = load ptr, ptr %_11.i.i62, align 8, !noalias !253, !nonnull !12, !noundef !12
  %40 = getelementptr inbounds nuw i8, ptr %_11.i.i62, i64 8
  %_14.1.i.i64 = load ptr, ptr %40, align 8, !noalias !253, !nonnull !12, !align !132, !noundef !12
  %41 = getelementptr inbounds nuw i8, ptr %_14.1.i.i64, i64 32
  %42 = load ptr, ptr %41, align 8, !invariant.load !12, !noalias !253, !nonnull !12
  %43 = tail call noundef i8 %42(ptr noundef nonnull align 1 %_14.0.i.i63, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i60, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i61) #31
  %44 = icmp eq i8 %43, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit66

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit66: ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52, %bb7.i.i59
  %_0.sroa.0.0.i.i65 = phi i1 [ %44, %bb7.i.i59 ], [ %.not3.i.mux.i58, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit52 ]
  %45 = select i1 %_0.sroa.0.0.i.i51, ptr %b, ptr %d, !unpredictable !12
  %46 = select i1 %_0.sroa.0.0.i.i37, ptr %c, ptr %a, !unpredictable !12
  %47 = select i1 %_0.sroa.0.0.i.i65, ptr %36, ptr %34, !unpredictable !12
  %48 = select i1 %_0.sroa.0.0.i.i65, ptr %34, ptr %36, !unpredictable !12
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst, ptr noundef nonnull align 8 dereferenceable(56) %46, i64 56, i1 false)
  %dst4 = getelementptr inbounds nuw i8, ptr %dst, i64 56
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst4, ptr noundef nonnull align 8 dereferenceable(56) %47, i64 56, i1 false)
  %dst5 = getelementptr inbounds nuw i8, ptr %dst, i64 112
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst5, ptr noundef nonnull align 8 dereferenceable(56) %48, i64 56, i1 false)
  %dst6 = getelementptr inbounds nuw i8, ptr %dst, i64 168
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst6, ptr noundef nonnull align 8 dereferenceable(56) %45, i64 56, i1 false)
  ret void
}

; core::slice::sort::shared::smallsort::insertion_sort_shift_left::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1M_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1m_7sort_byNCNvMs3_B1M_NtB1M_8IntoIter4pushs_0E0EB1M_(ptr noalias noundef nonnull align 8 captures(address, read_provenance) %v.0, i64 noundef range(i64 2, 21) %v.1, ptr readonly captures(address_is_null) %is_less.0.val) unnamed_addr #2 personality ptr @rust_eh_personality {
bb5.lr.ph:
  %tmp.i = alloca [56 x i8], align 8
  %v_end.idx = mul nuw nsw i64 %v.1, 56
  %v_end = getelementptr inbounds nuw i8, ptr %v.0, i64 %v_end.idx
  %tail.sroa.0.02 = getelementptr inbounds nuw i8, ptr %v.0, i64 56
  %0 = icmp ne ptr %is_less.0.val, null
  tail call void @llvm.assume(i1 %0)
  %a1.i.i9.i = getelementptr inbounds nuw i8, ptr %tmp.i, i64 8
  br label %bb5

bb7:                                              ; preds = %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit
  ret void

bb5:                                              ; preds = %bb5.lr.ph, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit
  %tail.sroa.0.05 = phi ptr [ %tail.sroa.0.02, %bb5.lr.ph ], [ %tail.sroa.0.0, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit ]
  %v.0.pn4 = phi ptr [ %v.0, %bb5.lr.ph ], [ %tail.sroa.0.05, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit ]
  call void @llvm.experimental.noalias.scope.decl(metadata !254)
  call void @llvm.experimental.noalias.scope.decl(metadata !257)
  %_7.val.i.i = load ptr, ptr %is_less.0.val, align 8, !noalias !259
  call void @llvm.experimental.noalias.scope.decl(metadata !260)
  call void @llvm.experimental.noalias.scope.decl(metadata !263)
  %1 = load i64, ptr %tail.sroa.0.05, align 8, !range !16, !alias.scope !265, !noalias !266, !noundef !12
  %.not.i.i.i = icmp ne i64 %1, -9223372036854775807
  %2 = load i64, ptr %v.0.pn4, align 8, !range !16, !alias.scope !266, !noalias !265, !noundef !12
  %.not3.i.i.i = icmp eq i64 %2, -9223372036854775807
  %.not3.i.not.i.i = xor i1 %.not3.i.i.i, true
  %brmerge.i.i = or i1 %.not.i.i.i, %.not3.i.not.i.i
  br i1 %brmerge.i.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i, label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb5
  %a1.i.i.i = getelementptr inbounds nuw i8, ptr %v.0.pn4, i64 64
  %b2.i.i.i = getelementptr inbounds nuw i8, ptr %v.0.pn4, i64 8
  %3 = icmp ne ptr %_7.val.i.i, null
  call void @llvm.assume(i1 %3)
  %_11.i.i.i = load ptr, ptr %_7.val.i.i, align 8, !noalias !267, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i = load ptr, ptr %_11.i.i.i, align 8, !noalias !267, !nonnull !12, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %_11.i.i.i, i64 8
  %_14.1.i.i.i = load ptr, ptr %4, align 8, !noalias !267, !nonnull !12, !align !132, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i, i64 32
  %6 = load ptr, ptr %5, align 8, !invariant.load !12, !noalias !267, !nonnull !12
  %7 = call noundef i8 %6(ptr noundef nonnull align 1 %_14.0.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i) #31
  %8 = icmp eq i8 %7, -1
  br i1 %8, label %bb2.i, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i: ; preds = %bb5
  %.not3.i.mux.i.i = and i1 %.not.i.i.i, %.not3.i.i.i
  br i1 %.not3.i.mux.i.i, label %bb2.i, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit

bb2.i:                                            ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i, %bb7.i.i.i
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %tmp.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %tmp.i, ptr noundef nonnull align 8 dereferenceable(56) %tail.sroa.0.05, i64 56, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %tail.sroa.0.05, ptr noundef nonnull align 8 dereferenceable(56) %v.0.pn4, i64 56, i1 false)
  %_183.i = icmp eq ptr %v.0.pn4, %v.0
  br i1 %_183.i, label %bb10.i, label %bb6.i

bb6.i:                                            ; preds = %bb2.i, %bb4.backedge.i
  %sift.sroa.0.04.i = phi ptr [ %9, %bb4.backedge.i ], [ %v.0.pn4, %bb2.i ]
  %9 = getelementptr inbounds i8, ptr %sift.sroa.0.04.i, i64 -56
  call void @llvm.experimental.noalias.scope.decl(metadata !268)
  call void @llvm.experimental.noalias.scope.decl(metadata !271)
  %_7.val.i2.i = load ptr, ptr %is_less.0.val, align 8, !noalias !273
  call void @llvm.experimental.noalias.scope.decl(metadata !274)
  call void @llvm.experimental.noalias.scope.decl(metadata !277)
  %10 = load i64, ptr %tmp.i, align 8, !range !16, !alias.scope !279, !noalias !280, !noundef !12
  %.not.i.i3.i = icmp ne i64 %10, -9223372036854775807
  %11 = load i64, ptr %9, align 8, !range !16, !alias.scope !280, !noalias !279, !noundef !12
  %.not3.i.i4.i = icmp eq i64 %11, -9223372036854775807
  %.not3.i.not.i5.i = xor i1 %.not3.i.i4.i, true
  %brmerge.i6.i = or i1 %.not.i.i3.i, %.not3.i.not.i5.i
  br i1 %brmerge.i6.i, label %bb7.i, label %bb7.i.i8.i

bb7.i.i8.i:                                       ; preds = %bb6.i
  %b2.i.i10.i = getelementptr inbounds i8, ptr %sift.sroa.0.04.i, i64 -48
  %12 = icmp ne ptr %_7.val.i2.i, null
  call void @llvm.assume(i1 %12)
  %_11.i.i11.i = load ptr, ptr %_7.val.i2.i, align 8, !noalias !281, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i12.i = load ptr, ptr %_11.i.i11.i, align 8, !noalias !281, !nonnull !12, !noundef !12
  %13 = getelementptr inbounds nuw i8, ptr %_11.i.i11.i, i64 8
  %_14.1.i.i13.i = load ptr, ptr %13, align 8, !noalias !281, !nonnull !12, !align !132, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %_14.1.i.i13.i, i64 32
  %15 = load ptr, ptr %14, align 8, !invariant.load !12, !noalias !281, !nonnull !12
  %16 = invoke noundef i8 %15(ptr noundef nonnull align 1 %_14.0.i.i12.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i9.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i10.i) #31
          to label %.noexc.i unwind label %bb14.i

.noexc.i:                                         ; preds = %bb7.i.i8.i
  %17 = icmp eq i8 %16, -1
  br i1 %17, label %bb4.backedge.i, label %bb10.i

bb7.i:                                            ; preds = %bb6.i
  %.not3.i.mux.i7.i = and i1 %.not.i.i3.i, %.not3.i.i4.i
  br i1 %.not3.i.mux.i7.i, label %bb4.backedge.i, label %bb10.i

bb4.backedge.i:                                   ; preds = %bb7.i, %.noexc.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.04.i, ptr noundef nonnull align 8 dereferenceable(56) %9, i64 56, i1 false)
  %_18.i = icmp eq ptr %9, %v.0
  br i1 %_18.i, label %bb10.i, label %bb6.i

bb10.i:                                           ; preds = %bb4.backedge.i, %bb7.i, %.noexc.i, %bb2.i
  %sift.sroa.0.0.lcssa.i = phi ptr [ %v.0, %bb2.i ], [ %v.0, %bb4.backedge.i ], [ %sift.sroa.0.04.i, %bb7.i ], [ %sift.sroa.0.04.i, %.noexc.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.0.lcssa.i, ptr noundef nonnull align 8 dereferenceable(56) %tmp.i, i64 56, i1 false), !noalias !282
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %tmp.i)
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit

bb14.i:                                           ; preds = %bb7.i.i8.i
  %18 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.04.i, ptr noundef nonnull align 8 dereferenceable(56) %tmp.i, i64 56, i1 false), !noalias !287
  resume { ptr, i32 } %18

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort11insert_tailINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1y_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB18_7sort_byNCNvMs3_B1y_NtB1y_8IntoIter4pushs_0E0EB1y_.exit: ; preds = %bb7.i.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i, %bb10.i
  %tail.sroa.0.0 = getelementptr inbounds nuw i8, ptr %tail.sroa.0.05, i64 56
  %_10.not = icmp eq ptr %tail.sroa.0.0, %v_end
  br i1 %_10.not, label %bb7, label %bb5
}

; core::slice::sort::stable::drift::sort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift4sortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBW_7sort_byNCNvMs3_B1m_NtB1m_8IntoIter4pushs_0E0EB1m_(ptr noalias noundef nonnull align 8 %v.0, i64 noundef range(i64 0, 164703072086692426) %v.1, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i1 noundef zeroext %eager_sort, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %desired_depth_storage = alloca [66 x i8], align 1
  %run_storage = alloca [528 x i8], align 8
  %_6 = icmp samesign ult i64 %v.1, 2
  br i1 %_6, label %bb23, label %bb2

bb2:                                              ; preds = %start
  %d = udiv i64 4611686018427387904, %v.1
  %0 = mul i64 %d, %v.1
  %r.decomposed = sub i64 4611686018427387904, %0
  %_72.not = icmp ne i64 %r.decomposed, 0
  %1 = zext i1 %_72.not to i64
  %scale_factor.sroa.0.0 = add nuw nsw i64 %d, %1
  %_9 = icmp samesign ult i64 %v.1, 4097
  br i1 %_9, label %bb3, label %bb4

bb4:                                              ; preds = %bb2
; call core::slice::sort::stable::drift::sqrt_approx
  %2 = tail call noundef i64 @_RNvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift11sqrt_approx(i64 noundef %v.1)
  br label %bb5

bb3:                                              ; preds = %bb2
  %_1134 = lshr i64 %v.1, 1
  %v1 = sub nsw i64 %v.1, %_1134
  %_0.sroa.0.0.i35 = tail call noundef i64 @llvm.umin.i64(i64 %v1, i64 64)
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %min_good_run_len.sroa.0.0 = phi i64 [ %_0.sroa.0.0.i35, %bb3 ], [ %2, %bb4 ]
  call void @llvm.lifetime.start.p0(i64 528, ptr nonnull %run_storage)
  call void @llvm.lifetime.start.p0(i64 66, ptr nonnull %desired_depth_storage)
  br label %bb6

bb6:                                              ; preds = %bb19, %bb5
  %prev_run.sroa.0.0 = phi i64 [ 1, %bb5 ], [ %next_run.sroa.0.0, %bb19 ]
  %scan_idx.sroa.0.0 = phi i64 [ 0, %bb5 ], [ %75, %bb19 ]
  %stack_len.sroa.0.0 = phi i64 [ 0, %bb5 ], [ %74, %bb19 ]
  %_22 = icmp ult i64 %scan_idx.sroa.0.0, %v.1
  br i1 %_22, label %bb30, label %bb10

bb10:                                             ; preds = %bb6, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit
  %desired_depth.sroa.0.0 = phi i8 [ %38, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit ], [ 0, %bb6 ]
  %next_run.sroa.0.0 = phi i64 [ %_0.sroa.0.0.i38, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit ], [ 1, %bb6 ]
  %_37121 = icmp ugt i64 %stack_len.sroa.0.0, 1
  br i1 %_37121, label %bb12.lr.ph, label %bb17

bb12.lr.ph:                                       ; preds = %bb10
  %v_end.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.0, i64 %scan_idx.sroa.0.0
  br label %bb12

bb30:                                             ; preds = %bb6
  %new_len = sub nuw nsw i64 %v.1, %scan_idx.sroa.0.0
  %_81 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.0, i64 %scan_idx.sroa.0.0
  tail call void @llvm.experimental.noalias.scope.decl(metadata !292)
  %_7.not.i = icmp ult i64 %new_len, %min_good_run_len.sroa.0.0
  br i1 %_7.not.i, label %bb7.i39, label %bb1.i

bb7.i39:                                          ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, %bb30
  br i1 %eager_sort, label %bb16.i43, label %bb11.i40

bb1.i:                                            ; preds = %bb30
  %_4.i.i = icmp samesign ult i64 %new_len, 2
  br i1 %_4.i.i, label %bb5.i36, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb1.i
  %_28.i.i = getelementptr inbounds nuw i8, ptr %_81, i64 56
  %is_less.val2.i = load ptr, ptr %is_less, align 8, !alias.scope !292, !noalias !295, !nonnull !12, !align !132, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !298)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !301)
  %_7.val.i76 = load ptr, ptr %is_less.val2.i, align 8, !noalias !303
  tail call void @llvm.experimental.noalias.scope.decl(metadata !304), !noalias !307
  tail call void @llvm.experimental.noalias.scope.decl(metadata !308), !noalias !307
  %3 = load i64, ptr %_28.i.i, align 8, !range !16, !alias.scope !310, !noalias !311, !noundef !12
  %.not.i.i77 = icmp ne i64 %3, -9223372036854775807
  %4 = load i64, ptr %_81, align 8, !range !16, !alias.scope !312, !noalias !313, !noundef !12
  %.not3.i.i78 = icmp eq i64 %4, -9223372036854775807
  %.not3.i.not.i79 = xor i1 %.not3.i.i78, true
  %brmerge.i80 = or i1 %.not.i.i77, %.not3.i.not.i79
  br i1 %brmerge.i80, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit89, label %bb7.i.i82

bb7.i.i82:                                        ; preds = %bb2.i.i
  %a1.i.i83 = getelementptr inbounds nuw i8, ptr %_81, i64 64
  %b2.i.i84 = getelementptr inbounds nuw i8, ptr %_81, i64 8
  %5 = icmp ne ptr %_7.val.i76, null
  tail call void @llvm.assume(i1 %5), !noalias !307
  %_11.i.i85 = load ptr, ptr %_7.val.i76, align 8, !noalias !314, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i86 = load ptr, ptr %_11.i.i85, align 8, !noalias !314, !nonnull !12, !noundef !12
  %6 = getelementptr inbounds nuw i8, ptr %_11.i.i85, i64 8
  %_14.1.i.i87 = load ptr, ptr %6, align 8, !noalias !314, !nonnull !12, !align !132, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %_14.1.i.i87, i64 32
  %8 = load ptr, ptr %7, align 8, !invariant.load !12, !noalias !314, !nonnull !12
  %9 = tail call noundef i8 %8(ptr noundef nonnull align 1 %_14.0.i.i86, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i83, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i84) #31, !noalias !307
  %10 = icmp eq i8 %9, -1
  br i1 %10, label %bb4.i.i.preheader, label %bb11.i.i.preheader

bb4.i.i.preheader:                                ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit89, %bb7.i.i82
  %_10.i.i116.not = icmp eq i64 %new_len, 2
  br i1 %_10.i.i116.not, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb5.i.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit89: ; preds = %bb2.i.i
  %.not3.i.mux.i81 = and i1 %.not.i.i77, %.not3.i.i78
  br i1 %.not3.i.mux.i81, label %bb4.i.i.preheader, label %bb11.i.i.preheader

bb11.i.i.preheader:                               ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit89, %bb7.i.i82
  %_19.i.i112.not = icmp eq i64 %new_len, 2
  br i1 %_19.i.i112.not, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb12.i.i

bb12.i.i:                                         ; preds = %bb11.i.i.preheader, %bb15.i.i
  %run_len.sroa.0.0.i.i113 = phi i64 [ %19, %bb15.i.i ], [ 2, %bb11.i.i.preheader ]
  %_40.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %run_len.sroa.0.0.i.i113
  %index1.i.i = add nsw i64 %run_len.sroa.0.0.i.i113, -1
  %_45.i.i = icmp ult i64 %index1.i.i, %new_len
  tail call void @llvm.assume(i1 %_45.i.i)
  %_43.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %index1.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !315)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !318)
  %_7.val.i62 = load ptr, ptr %is_less.val2.i, align 8, !noalias !320
  tail call void @llvm.experimental.noalias.scope.decl(metadata !321), !noalias !307
  tail call void @llvm.experimental.noalias.scope.decl(metadata !324), !noalias !307
  %11 = load i64, ptr %_40.i.i, align 8, !range !16, !alias.scope !326, !noalias !327, !noundef !12
  %.not.i.i63 = icmp ne i64 %11, -9223372036854775807
  %12 = load i64, ptr %_43.i.i, align 8, !range !16, !alias.scope !328, !noalias !329, !noundef !12
  %.not3.i.i64 = icmp eq i64 %12, -9223372036854775807
  %.not3.i.not.i65 = xor i1 %.not3.i.i64, true
  %brmerge.i66 = or i1 %.not.i.i63, %.not3.i.not.i65
  br i1 %brmerge.i66, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75, label %bb7.i.i68

bb7.i.i68:                                        ; preds = %bb12.i.i
  %a1.i.i69 = getelementptr inbounds nuw i8, ptr %_40.i.i, i64 8
  %b2.i.i70 = getelementptr inbounds nuw i8, ptr %_43.i.i, i64 8
  %13 = icmp ne ptr %_7.val.i62, null
  tail call void @llvm.assume(i1 %13), !noalias !307
  %_11.i.i71 = load ptr, ptr %_7.val.i62, align 8, !noalias !330, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i72 = load ptr, ptr %_11.i.i71, align 8, !noalias !330, !nonnull !12, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %_11.i.i71, i64 8
  %_14.1.i.i73 = load ptr, ptr %14, align 8, !noalias !330, !nonnull !12, !align !132, !noundef !12
  %15 = getelementptr inbounds nuw i8, ptr %_14.1.i.i73, i64 32
  %16 = load ptr, ptr %15, align 8, !invariant.load !12, !noalias !330, !nonnull !12
  %17 = tail call noundef i8 %16(ptr noundef nonnull align 1 %_14.0.i.i72, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i69, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i70) #31, !noalias !307
  %18 = icmp eq i8 %17, -1
  br i1 %18, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb15.i.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75: ; preds = %bb12.i.i
  %.not3.i.mux.i67 = and i1 %.not.i.i63, %.not3.i.i64
  br i1 %.not3.i.mux.i67, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb15.i.i

bb15.i.i:                                         ; preds = %bb7.i.i68, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75
  %19 = add nuw i64 %run_len.sroa.0.0.i.i113, 1
  %exitcond.not = icmp eq i64 %19, %new_len
  br i1 %exitcond.not, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb12.i.i

bb5.i.i:                                          ; preds = %bb4.i.i.preheader, %bb7.i.i
  %run_len.sroa.0.1.i.i117 = phi i64 [ %28, %bb7.i.i ], [ 2, %bb4.i.i.preheader ]
  %_34.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %run_len.sroa.0.1.i.i117
  %index3.i.i = add nsw i64 %run_len.sroa.0.1.i.i117, -1
  %_39.i.i = icmp ult i64 %index3.i.i, %new_len
  tail call void @llvm.assume(i1 %_39.i.i)
  %_37.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %index3.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !331)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !334)
  %_7.val.i = load ptr, ptr %is_less.val2.i, align 8, !noalias !336
  tail call void @llvm.experimental.noalias.scope.decl(metadata !337), !noalias !307
  tail call void @llvm.experimental.noalias.scope.decl(metadata !340), !noalias !307
  %20 = load i64, ptr %_34.i.i, align 8, !range !16, !alias.scope !342, !noalias !343, !noundef !12
  %.not.i.i = icmp ne i64 %20, -9223372036854775807
  %21 = load i64, ptr %_37.i.i, align 8, !range !16, !alias.scope !344, !noalias !345, !noundef !12
  %.not3.i.i = icmp eq i64 %21, -9223372036854775807
  %.not3.i.not.i = xor i1 %.not3.i.i, true
  %brmerge.i = or i1 %.not.i.i, %.not3.i.not.i
  br i1 %brmerge.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, label %bb7.i.i60

bb7.i.i60:                                        ; preds = %bb5.i.i
  %a1.i.i = getelementptr inbounds nuw i8, ptr %_34.i.i, i64 8
  %b2.i.i = getelementptr inbounds nuw i8, ptr %_37.i.i, i64 8
  %22 = icmp ne ptr %_7.val.i, null
  tail call void @llvm.assume(i1 %22), !noalias !307
  %_11.i.i = load ptr, ptr %_7.val.i, align 8, !noalias !346, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i = load ptr, ptr %_11.i.i, align 8, !noalias !346, !nonnull !12, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_14.1.i.i = load ptr, ptr %23, align 8, !noalias !346, !nonnull !12, !align !132, !noundef !12
  %24 = getelementptr inbounds nuw i8, ptr %_14.1.i.i, i64 32
  %25 = load ptr, ptr %24, align 8, !invariant.load !12, !noalias !346, !nonnull !12
  %26 = tail call noundef i8 %25(ptr noundef nonnull align 1 %_14.0.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i) #31, !noalias !307
  %27 = icmp eq i8 %26, -1
  br i1 %27, label %bb7.i.i, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit: ; preds = %bb5.i.i
  %.not3.i.mux.i = and i1 %.not.i.i, %.not3.i.i
  br i1 %.not3.i.mux.i, label %bb7.i.i, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i

bb7.i.i:                                          ; preds = %bb7.i.i60, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit
  %28 = add nuw i64 %run_len.sroa.0.1.i.i117, 1
  %exitcond137.not = icmp eq i64 %28, %new_len
  br i1 %exitcond137.not, label %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i, label %bb5.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i: ; preds = %bb15.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75, %bb7.i.i68, %bb7.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, %bb7.i.i60, %bb11.i.i.preheader, %bb4.i.i.preheader
  %_0.sroa.3.0.off0.i.i = phi i1 [ true, %bb4.i.i.preheader ], [ false, %bb11.i.i.preheader ], [ true, %bb7.i.i60 ], [ true, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ], [ true, %bb7.i.i ], [ false, %bb7.i.i68 ], [ false, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75 ], [ false, %bb15.i.i ]
  %_0.sroa.0.0.i.i = phi i64 [ 2, %bb4.i.i.preheader ], [ 2, %bb11.i.i.preheader ], [ %new_len, %bb7.i.i ], [ %run_len.sroa.0.1.i.i117, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ], [ %run_len.sroa.0.1.i.i117, %bb7.i.i60 ], [ %new_len, %bb15.i.i ], [ %run_len.sroa.0.0.i.i113, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit75 ], [ %run_len.sroa.0.0.i.i113, %bb7.i.i68 ]
  %_12.i = icmp ule i64 %_0.sroa.0.0.i.i, %new_len
  tail call void @llvm.assume(i1 %_12.i)
  %_13.not.i = icmp ult i64 %_0.sroa.0.0.i.i, %min_good_run_len.sroa.0.0
  br i1 %_13.not.i, label %bb7.i39, label %bb3.i

bb3.i:                                            ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared17find_existing_runINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1s_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB12_7sort_byNCNvMs3_B1s_NtB1s_8IntoIter4pushs_0E0EB1s_.exit.i
  br i1 %_0.sroa.3.0.off0.i.i, label %bb14.i, label %bb5.i36

bb11.i40:                                         ; preds = %bb7.i39
  %_0.sroa.0.0.i59 = tail call noundef i64 @llvm.umin.i64(i64 range(i64 0, 164703072086692426) %new_len, i64 %min_good_run_len.sroa.0.0)
  %_36.i41 = shl nuw nsw i64 %_0.sroa.0.0.i59, 1
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit

bb16.i43:                                         ; preds = %bb7.i39
  %_0.sroa.0.0.i58 = tail call noundef i64 @llvm.umin.i64(i64 range(i64 0, 164703072086692426) %new_len, i64 32)
; call core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  tail call void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %_81, i64 noundef %_0.sroa.0.0.i58, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i32 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) null, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) #30
  %_35.i44 = shl nuw nsw i64 %_0.sroa.0.0.i58, 1
  %_34.i45 = or disjoint i64 %_35.i44, 1
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit

bb5.i36:                                          ; preds = %bb6.i.i, %bb1.i, %bb14.i, %bb3.i
  %_0.sroa.0.0.i.i97100 = phi i64 [ %_0.sroa.0.0.i.i, %bb3.i ], [ %_0.sroa.0.0.i.i, %bb14.i ], [ %new_len, %bb1.i ], [ %_0.sroa.0.0.i.i, %bb6.i.i ]
  %_27.i37 = shl nuw nsw i64 %_0.sroa.0.0.i.i97100, 1
  %_26.i = or disjoint i64 %_27.i37, 1
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit

bb14.i:                                           ; preds = %bb3.i
  %half_len1.i = lshr i64 %_0.sroa.0.0.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !347), !noalias !307
  tail call void @llvm.experimental.noalias.scope.decl(metadata !350), !noalias !307
  %_917.not.i.i = icmp samesign ult i64 %_0.sroa.0.0.i.i, 2
  br i1 %_917.not.i.i, label %bb5.i36, label %bb5.preheader.i.i

bb5.preheader.i.i:                                ; preds = %bb14.i
  %end.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %_0.sroa.0.0.i.i
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb6.i.i, %bb5.preheader.i.i
  %i.sroa.0.018.i.i = phi i64 [ %36, %bb6.i.i ], [ 0, %bb5.preheader.i.i ]
  %29 = xor i64 %i.sroa.0.018.i.i, -1
  %x.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_81, i64 %i.sroa.0.018.i.i
  %y.i.i = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %end.i, i64 %29
  %30 = load <2 x i64>, ptr %x.i.i, align 8, !alias.scope !352, !noalias !358
  %31 = load <2 x i64>, ptr %y.i.i, align 8, !alias.scope !359, !noalias !362
  store <2 x i64> %31, ptr %x.i.i, align 8, !alias.scope !352, !noalias !358
  store <2 x i64> %30, ptr %y.i.i, align 8, !alias.scope !359, !noalias !362
  %_11.2.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %x.i.i, i64 16
  %_13.2.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %y.i.i, i64 16
  %32 = load <2 x i64>, ptr %_11.2.i.i.i.i.i.i, align 8, !alias.scope !363, !noalias !358
  %33 = load <2 x i64>, ptr %_13.2.i.i.i.i.i.i, align 8, !alias.scope !366, !noalias !362
  store <2 x i64> %33, ptr %_11.2.i.i.i.i.i.i, align 8, !alias.scope !363, !noalias !358
  store <2 x i64> %32, ptr %_13.2.i.i.i.i.i.i, align 8, !alias.scope !366, !noalias !362
  %_11.4.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %x.i.i, i64 32
  %_13.4.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %y.i.i, i64 32
  %34 = load <2 x i64>, ptr %_11.4.i.i.i.i.i.i, align 8, !alias.scope !369, !noalias !358
  %35 = load <2 x i64>, ptr %_13.4.i.i.i.i.i.i, align 8, !alias.scope !372, !noalias !362
  store <2 x i64> %35, ptr %_11.4.i.i.i.i.i.i, align 8, !alias.scope !369, !noalias !358
  store <2 x i64> %34, ptr %_13.4.i.i.i.i.i.i, align 8, !alias.scope !372, !noalias !362
  %_11.6.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %x.i.i, i64 48
  %_13.6.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %y.i.i, i64 48
  tail call void @llvm.experimental.noalias.scope.decl(metadata !375), !noalias !307
  tail call void @llvm.experimental.noalias.scope.decl(metadata !377), !noalias !307
  %_3.sroa.0.0.copyload.i.6.i.i.i.i.i.i = load i64, ptr %_11.6.i.i.i.i.i.i, align 8, !alias.scope !379, !noalias !380
  %_4.sroa.0.0.copyload.i.6.i.i.i.i.i.i = load i64, ptr %_13.6.i.i.i.i.i.i, align 8, !alias.scope !381, !noalias !382
  store i64 %_4.sroa.0.0.copyload.i.6.i.i.i.i.i.i, ptr %_11.6.i.i.i.i.i.i, align 8, !alias.scope !379, !noalias !380
  store i64 %_3.sroa.0.0.copyload.i.6.i.i.i.i.i.i, ptr %_13.6.i.i.i.i.i.i, align 8, !alias.scope !381, !noalias !382
  %36 = add nuw nsw i64 %i.sroa.0.018.i.i, 1
  %exitcond.not.i.i = icmp eq i64 %36, %half_len1.i
  br i1 %exitcond.not.i.i, label %bb5.i36, label %bb6.i.i

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_.exit: ; preds = %bb11.i40, %bb16.i43, %bb5.i36
  %_0.sroa.0.0.i38 = phi i64 [ %_26.i, %bb5.i36 ], [ %_34.i45, %bb16.i43 ], [ %_36.i41, %bb11.i40 ]
  %_31 = lshr i64 %prev_run.sroa.0.0, 1
  %_35 = lshr i64 %_0.sroa.0.0.i38, 1
  %factor = shl i64 %scan_idx.sroa.0.0, 1
  %x = sub i64 %factor, %_31
  %y = add i64 %_35, %factor
  %_89 = mul i64 %x, %scale_factor.sroa.0.0
  %_90 = mul i64 %y, %scale_factor.sroa.0.0
  %self3 = xor i64 %_90, %_89
  %37 = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %self3, i1 false)
  %38 = trunc nuw nsw i64 %37 to i8
  br label %bb10

bb12:                                             ; preds = %bb12.lr.ph, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit
  %stack_len.sroa.0.1123 = phi i64 [ %stack_len.sroa.0.0, %bb12.lr.ph ], [ %count, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit ]
  %prev_run.sroa.0.1122 = phi i64 [ %prev_run.sroa.0.0, %bb12.lr.ph ], [ %_0.sroa.0.0.i, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit ]
  %count = add i64 %stack_len.sroa.0.1123, -1
  %_41 = getelementptr inbounds nuw i8, ptr %desired_depth_storage, i64 %count
  %_40 = load i8, ptr %_41, align 1, !noundef !12
  %_39.not = icmp ult i8 %_40, %desired_depth.sroa.0.0
  br i1 %_39.not, label %bb17, label %bb13

bb17:                                             ; preds = %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit, %bb12, %bb10
  %prev_run.sroa.0.1.lcssa = phi i64 [ %prev_run.sroa.0.0, %bb10 ], [ %prev_run.sroa.0.1122, %bb12 ], [ %_0.sroa.0.0.i, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit ]
  %stack_len.sroa.0.1.lcssa = phi i64 [ %stack_len.sroa.0.0, %bb10 ], [ %stack_len.sroa.0.1123, %bb12 ], [ 1, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit ]
  %_59 = getelementptr inbounds nuw i64, ptr %run_storage, i64 %stack_len.sroa.0.1.lcssa
  store i64 %prev_run.sroa.0.1.lcssa, ptr %_59, align 8
  %_61 = getelementptr inbounds nuw i8, ptr %desired_depth_storage, i64 %stack_len.sroa.0.1.lcssa
  store i8 %desired_depth.sroa.0.0, ptr %_61, align 1
  br i1 %_22, label %bb19, label %bb18

bb13:                                             ; preds = %bb12
  %_46 = getelementptr inbounds nuw i64, ptr %run_storage, i64 %count
  %left9 = load i64, ptr %_46, align 8, !noundef !12
  %_50 = lshr i64 %left9, 1
  %_51 = lshr i64 %prev_run.sroa.0.1122, 1
  %merged_len = add nuw i64 %_50, %_51
  %merge_start_idx = sub i64 %scan_idx.sroa.0.0, %merged_len
  %_96 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.0, i64 %merge_start_idx
  %can_fit_in_scratch.i = icmp samesign ugt i64 %merged_len, %scratch.1
  %_18.i = and i64 %prev_run.sroa.0.1122, 1
  %.not4.i = icmp eq i64 %_18.i, 0
  %39 = or i64 %left9, %prev_run.sroa.0.1122
  %40 = and i64 %39, 1
  %41 = icmp ne i64 %40, 0
  %or.cond2.i = or i1 %can_fit_in_scratch.i, %41
  br i1 %or.cond2.i, label %bb5.i, label %bb13.i

bb5.i:                                            ; preds = %bb13
  %_17.i = and i64 %left9, 1
  %.not.i = icmp eq i64 %_17.i, 0
  br i1 %.not.i, label %bb15.i, label %bb8.i

bb13.i:                                           ; preds = %bb13
  %_36.i = shl nuw nsw i64 %merged_len, 1
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit

bb8.i:                                            ; preds = %bb15.i, %bb5.i
  br i1 %.not4.i, label %bb20.i, label %bb11.i

bb15.i:                                           ; preds = %bb5.i
  %self.i = or i64 %_50, 1
  %42 = tail call range(i64 6, 64) i64 @llvm.ctlz.i64(i64 %self.i, i1 true)
  %43 = trunc nuw nsw i64 %42 to i32
  %log.i = shl nuw nsw i32 %43, 1
  %limit.i = xor i32 %log.i, 126
; call core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  tail call void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %_96, i64 noundef range(i64 0, 164703072086692426) %_50, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i32 noundef %limit.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) null, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) #30
  br label %bb8.i

bb11.i:                                           ; preds = %bb20.i, %bb8.i
  %is_less.val = load ptr, ptr %is_less, align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !383)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !386)
  %44 = icmp ult i64 %left9, 2
  %_6.i = icmp ult i64 %prev_run.sroa.0.1122, 2
  %or.cond.i = select i1 %44, i1 true, i1 %_6.i
  br i1 %or.cond.i, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit, label %bb2.i

bb2.i:                                            ; preds = %bb11.i
  %_0.sroa.0.0.i.i46 = tail call i64 @llvm.umin.i64(i64 %_51, i64 range(i64 0, -9223372036854775808) %_50)
  %_7.i = icmp samesign ult i64 %scratch.1, %_0.sroa.0.0.i.i46
  br i1 %_7.i, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit, label %bb5.i47

bb5.i47:                                          ; preds = %bb2.i
  %v_mid.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_96, i64 %_50
  %left_is_shorter.not.i = icmp samesign ugt i64 %_50, %_51
  %spec.select.i = select i1 %left_is_shorter.not.i, ptr %v_mid.i, ptr %_96
  %45 = mul nuw nsw i64 %_0.sroa.0.0.i.i46, 56
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %scratch.0, ptr nonnull align 8 %spec.select.i, i64 %45, i1 false), !alias.scope !388
  %_21.i = getelementptr inbounds nuw i8, ptr %scratch.0, i64 %45
  %46 = icmp ne ptr %is_less.val, null
  tail call void @llvm.assume(i1 %46)
  br i1 %left_is_shorter.not.i, label %bb1.i.i, label %bb3.i.i

bb1.i.i:                                          ; preds = %bb5.i47, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i
  %merge_state.sroa.13.0.i = phi ptr [ %_18.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %v_mid.i, %bb5.i47 ]
  %merge_state.sroa.7.0.i = phi ptr [ %_21.i.i51, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %_21.i, %bb5.i47 ]
  %out.sroa.0.0.i.i = phi ptr [ %49, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %v_end.i, %bb5.i47 ]
  %47 = getelementptr inbounds i8, ptr %merge_state.sroa.13.0.i, i64 -56
  %48 = getelementptr inbounds i8, ptr %merge_state.sroa.7.0.i, i64 -56
  %49 = getelementptr inbounds i8, ptr %out.sroa.0.0.i.i, i64 -56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !389)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !392)
  %_7.val.i.i.i = load ptr, ptr %is_less.val, align 8, !noalias !394
  tail call void @llvm.experimental.noalias.scope.decl(metadata !397)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !400)
  %50 = load i64, ptr %48, align 8, !range !16, !alias.scope !402, !noalias !403, !noundef !12
  %.not.i.i.i.i = icmp ne i64 %50, -9223372036854775807
  %51 = load i64, ptr %47, align 8, !range !16, !alias.scope !404, !noalias !405, !noundef !12
  %.not3.i.i.i.i = icmp eq i64 %51, -9223372036854775807
  %.not3.i.not.i.i.i = xor i1 %.not3.i.i.i.i, true
  %brmerge.i.i.i = or i1 %.not.i.i.i.i, %.not3.i.not.i.i.i
  %.not3.i.mux.i.i.i = and i1 %.not.i.i.i.i, %.not3.i.i.i.i
  br i1 %brmerge.i.i.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i, label %bb7.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb1.i.i
  %a1.i.i.i.i = getelementptr inbounds i8, ptr %merge_state.sroa.7.0.i, i64 -48
  %b2.i.i.i.i = getelementptr inbounds i8, ptr %merge_state.sroa.13.0.i, i64 -48
  %52 = icmp ne ptr %_7.val.i.i.i, null
  tail call void @llvm.assume(i1 %52)
  %_11.i.i.i.i = load ptr, ptr %_7.val.i.i.i, align 8, !noalias !406, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i.i = load ptr, ptr %_11.i.i.i.i, align 8, !noalias !406, !nonnull !12, !noundef !12
  %53 = getelementptr inbounds nuw i8, ptr %_11.i.i.i.i, i64 8
  %_14.1.i.i.i.i = load ptr, ptr %53, align 8, !noalias !406, !nonnull !12, !align !132, !noundef !12
  %54 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i.i, i64 32
  %55 = load ptr, ptr %54, align 8, !invariant.load !12, !noalias !406, !nonnull !12
  %56 = invoke noundef i8 %55(ptr noundef nonnull align 1 %_14.0.i.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i.i) #31
          to label %.noexc.i unwind label %bb20.loopexit.i

.noexc.i:                                         ; preds = %bb7.i.i.i.i
  %57 = icmp eq i8 %56, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i: ; preds = %.noexc.i, %bb1.i.i
  %_0.sroa.0.0.i.i.i.i = phi i1 [ %57, %.noexc.i ], [ %.not3.i.mux.i.i.i, %bb1.i.i ]
  %..i.i = select i1 %_0.sroa.0.0.i.i.i.i, ptr %47, ptr %48
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %49, ptr noundef nonnull align 8 dereferenceable(56) %..i.i, i64 56, i1 false), !alias.scope !388, !noalias !407
  %_20.i.i = xor i1 %_0.sroa.0.0.i.i.i.i, true
  %count.i.i = zext i1 %_20.i.i to i64
  %_18.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %47, i64 %count.i.i
  %count4.i.i = zext i1 %_0.sroa.0.0.i.i.i.i to i64
  %_21.i.i51 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %48, i64 %count4.i.i
  %_23.i.i = icmp eq ptr %_18.i.i, %_96
  %_26.i.i = icmp eq ptr %_21.i.i51, %scratch.0
  %or.cond.i.i = select i1 %_23.i.i, i1 true, i1 %_26.i.i
  br i1 %or.cond.i.i, label %bb16.i50, label %bb1.i.i

bb3.i.i:                                          ; preds = %bb5.i47, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i
  %merge_state.sroa.13.1.i = phi ptr [ %_25.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ], [ %_96, %bb5.i47 ]
  %merge_state.sroa.0.0.i = phi ptr [ %_20.i28.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ], [ %scratch.0, %bb5.i47 ]
  %right.sroa.0.06.i.i = phi ptr [ %_23.i29.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ], [ %v_mid.i, %bb5.i47 ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !408)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !411)
  %_7.val.i.i13.i = load ptr, ptr %is_less.val, align 8, !noalias !413
  tail call void @llvm.experimental.noalias.scope.decl(metadata !416)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !419)
  %58 = load i64, ptr %right.sroa.0.06.i.i, align 8, !range !16, !alias.scope !421, !noalias !422, !noundef !12
  %.not.i.i.i14.i = icmp ne i64 %58, -9223372036854775807
  %59 = load i64, ptr %merge_state.sroa.0.0.i, align 8, !range !16, !alias.scope !423, !noalias !424, !noundef !12
  %.not3.i.i.i15.i = icmp eq i64 %59, -9223372036854775807
  %.not3.i.not.i.i16.i = xor i1 %.not3.i.i.i15.i, true
  %brmerge.i.i17.i = or i1 %.not.i.i.i14.i, %.not3.i.not.i.i16.i
  %.not3.i.mux.i.i18.i = and i1 %.not.i.i.i14.i, %.not3.i.i.i15.i
  br i1 %brmerge.i.i17.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i, label %bb7.i.i.i19.i

bb7.i.i.i19.i:                                    ; preds = %bb3.i.i
  %a1.i.i.i20.i = getelementptr inbounds nuw i8, ptr %right.sroa.0.06.i.i, i64 8
  %b2.i.i.i21.i = getelementptr inbounds nuw i8, ptr %merge_state.sroa.0.0.i, i64 8
  %60 = icmp ne ptr %_7.val.i.i13.i, null
  tail call void @llvm.assume(i1 %60)
  %_11.i.i.i22.i = load ptr, ptr %_7.val.i.i13.i, align 8, !noalias !425, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i23.i = load ptr, ptr %_11.i.i.i22.i, align 8, !noalias !425, !nonnull !12, !noundef !12
  %61 = getelementptr inbounds nuw i8, ptr %_11.i.i.i22.i, i64 8
  %_14.1.i.i.i24.i = load ptr, ptr %61, align 8, !noalias !425, !nonnull !12, !align !132, !noundef !12
  %62 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i24.i, i64 32
  %63 = load ptr, ptr %62, align 8, !invariant.load !12, !noalias !425, !nonnull !12
  %64 = invoke noundef i8 %63(ptr noundef nonnull align 1 %_14.0.i.i.i23.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i20.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i21.i) #31
          to label %.noexc31.i unwind label %bb20.loopexit.split-lp.i

.noexc31.i:                                       ; preds = %bb7.i.i.i19.i
  %65 = icmp eq i8 %64, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i: ; preds = %.noexc31.i, %bb3.i.i
  %_0.sroa.0.0.i.i.i26.i = phi i1 [ %65, %.noexc31.i ], [ %.not3.i.mux.i.i18.i, %bb3.i.i ]
  %consume_left.i.i = xor i1 %_0.sroa.0.0.i.i.i26.i, true
  %src.sroa.0.0.i.i = select i1 %_0.sroa.0.0.i.i.i26.i, ptr %right.sroa.0.06.i.i, ptr %merge_state.sroa.0.0.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %merge_state.sroa.13.1.i, ptr noundef nonnull align 8 dereferenceable(56) %src.sroa.0.0.i.i, i64 56, i1 false), !alias.scope !388, !noalias !426
  %count.i27.i = zext i1 %consume_left.i.i to i64
  %_20.i28.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %merge_state.sroa.0.0.i, i64 %count.i27.i
  %count2.i.i = zext i1 %_0.sroa.0.0.i.i.i26.i to i64
  %_23.i29.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %right.sroa.0.06.i.i, i64 %count2.i.i
  %_25.i.i = getelementptr inbounds nuw i8, ptr %merge_state.sroa.13.1.i, i64 56
  %_7.i.i = icmp ne ptr %_20.i28.i, %_21.i
  %_10.i.i49 = icmp ne ptr %_23.i29.i, %v_end.i
  %or.cond.i30.i = select i1 %_7.i.i, i1 %_10.i.i49, i1 false
  br i1 %or.cond.i30.i, label %bb3.i.i, label %bb16.i50

bb16.i50:                                         ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i
  %merge_state.sroa.13.4.i = phi ptr [ %_18.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %_25.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ]
  %merge_state.sroa.7.2.i = phi ptr [ %_21.i.i51, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %_21.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ]
  %merge_state.sroa.0.3.i = phi ptr [ %scratch.0, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ], [ %_20.i28.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i25.i ]
  %66 = ptrtoint ptr %merge_state.sroa.7.2.i to i64
  %67 = ptrtoint ptr %merge_state.sroa.0.3.i to i64
  %68 = sub nuw i64 %66, %67
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %merge_state.sroa.13.4.i, ptr align 8 %merge_state.sroa.0.3.i, i64 %68, i1 false), !alias.scope !388, !noalias !427
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit

bb20.loopexit.i:                                  ; preds = %bb7.i.i.i.i
  %lpad.loopexit.i = landingpad { ptr, i32 }
          cleanup
  br label %bb20.i48

bb20.loopexit.split-lp.i:                         ; preds = %bb7.i.i.i19.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %bb20.i48

bb20.i48:                                         ; preds = %bb20.loopexit.split-lp.i, %bb20.loopexit.i
  %merge_state.sroa.13.3.i = phi ptr [ %merge_state.sroa.13.0.i, %bb20.loopexit.i ], [ %merge_state.sroa.13.1.i, %bb20.loopexit.split-lp.i ]
  %merge_state.sroa.7.1.i = phi ptr [ %merge_state.sroa.7.0.i, %bb20.loopexit.i ], [ %_21.i, %bb20.loopexit.split-lp.i ]
  %merge_state.sroa.0.2.i = phi ptr [ %scratch.0, %bb20.loopexit.i ], [ %merge_state.sroa.0.0.i, %bb20.loopexit.split-lp.i ]
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit.i, %bb20.loopexit.i ], [ %lpad.loopexit.split-lp.i, %bb20.loopexit.split-lp.i ]
  %69 = ptrtoint ptr %merge_state.sroa.7.1.i to i64
  %70 = ptrtoint ptr %merge_state.sroa.0.2.i to i64
  %71 = sub nuw i64 %69, %70
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %merge_state.sroa.13.3.i, ptr nonnull align 8 %merge_state.sroa.0.2.i, i64 %71, i1 false), !alias.scope !388, !noalias !432
  resume { ptr, i32 } %lpad.phi.i

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit: ; preds = %bb11.i, %bb2.i, %bb16.i50
  %_35.i = shl nuw nsw i64 %merged_len, 1
  %_34.i = or disjoint i64 %_35.i, 1
  br label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit

bb20.i:                                           ; preds = %bb8.i
  %_33.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_96, i64 %_50
  %self.i52 = or i64 %_51, 1
  %72 = tail call range(i64 6, 64) i64 @llvm.ctlz.i64(i64 %self.i52, i1 true)
  %73 = trunc nuw nsw i64 %72 to i32
  %log.i53 = shl nuw nsw i32 %73, 1
  %limit.i54 = xor i32 %log.i53, 126
; call core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  tail call void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %_33.i, i64 noundef range(i64 0, 164703072086692426) %_51, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i32 noundef %limit.i54, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) null, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) #30
  br label %bb11.i

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift13logical_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1w_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB16_7sort_byNCNvMs3_B1w_NtB1w_8IntoIter4pushs_0E0EB1w_.exit: ; preds = %bb13.i, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit
  %_0.sroa.0.0.i = phi i64 [ %_34.i, %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_.exit ], [ %_36.i, %bb13.i ]
  %_37 = icmp ugt i64 %count, 1
  br i1 %_37, label %bb12, label %bb17

bb19:                                             ; preds = %bb17
  %74 = add i64 %stack_len.sroa.0.1.lcssa, 1
  %_65 = lshr i64 %next_run.sroa.0.0, 1
  %75 = add nuw i64 %_65, %scan_idx.sroa.0.0
  br label %bb6

bb18:                                             ; preds = %bb17
  %_97 = and i64 %prev_run.sroa.0.1.lcssa, 1
  %.not = icmp eq i64 %_97, 0
  br i1 %.not, label %bb21, label %bb22

bb21:                                             ; preds = %bb18
  %self.i55 = or i64 %v.1, 1
  %76 = tail call range(i64 6, 64) i64 @llvm.ctlz.i64(i64 %self.i55, i1 true)
  %77 = trunc nuw nsw i64 %76 to i32
  %log.i56 = shl nuw nsw i32 %77, 1
  %limit.i57 = xor i32 %log.i56, 126
; call core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  tail call void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %v.0, i64 noundef range(i64 0, 164703072086692426) %v.1, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i32 noundef %limit.i57, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) null, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) #30
  br label %bb22

bb22:                                             ; preds = %bb18, %bb21
  call void @llvm.lifetime.end.p0(i64 66, ptr nonnull %desired_depth_storage)
  call void @llvm.lifetime.end.p0(i64 528, ptr nonnull %run_storage)
  br label %bb23

bb23:                                             ; preds = %start, %bb22
  ret void
}

; core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
; Function Attrs: noinline uwtable
define void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %0, i64 noundef range(i64 0, 164703072086692426) %1, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef range(i64 0, 164703072086692426) %scratch.1, i32 noundef %2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) %3, ptr noalias noundef align 8 dereferenceable(8) %is_less) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %tmp.i.i = alloca [56 x i8], align 8
  %pivot_copy = alloca [56 x i8], align 8
  %_8128134 = icmp samesign ult i64 %1, 33
  br i1 %_8128134, label %bb3, label %bb5.lr.ph

bb5.lr.ph:                                        ; preds = %start, %bb28
  %v.sroa.0.0.ph138 = phi ptr [ %_54, %bb28 ], [ %0, %start ]
  %v.sroa.16.0.ph137 = phi i64 [ %_63.i70, %bb28 ], [ %1, %start ]
  %limit.sroa.0.0.ph136 = phi i32 [ %70, %bb28 ], [ %2, %start ]
  %left_ancestor_pivot.sroa.0.0.ph135 = phi ptr [ null, %bb28 ], [ %3, %start ]
  %a1.i.i.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.0.ph138, i64 8
  %4 = ptrtoint ptr %v.sroa.0.0.ph138 to i64
  %.not = icmp eq ptr %left_ancestor_pivot.sroa.0.0.ph135, null
  %a1.i.i = getelementptr inbounds nuw i8, ptr %left_ancestor_pivot.sroa.0.0.ph135, i64 8
  br label %bb5

bb5:                                              ; preds = %bb5.lr.ph, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit
  %v.sroa.16.0130 = phi i64 [ %v.sroa.16.0.ph137, %bb5.lr.ph ], [ %state.sroa.11.1.lcssa.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit ]
  %limit.sroa.0.0129 = phi i32 [ %limit.sroa.0.0.ph136, %bb5.lr.ph ], [ %70, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit ]
  %5 = icmp eq i32 %limit.sroa.0.0129, 0
  br i1 %5, label %bb6, label %bb7

bb3:                                              ; preds = %bb28, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit, %start
  %v.sroa.0.0.ph.lcssa127 = phi ptr [ %0, %start ], [ %v.sroa.0.0.ph138, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit ], [ %_54, %bb28 ]
  %v.sroa.16.0.lcssa = phi i64 [ %1, %start ], [ %state.sroa.11.1.lcssa.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit ], [ %_63.i70, %bb28 ]
  %is_less.val23 = load ptr, ptr %is_less, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !437)
  call void @llvm.experimental.noalias.scope.decl(metadata !440)
  %_5.i = icmp samesign ult i64 %v.sroa.16.0.lcssa, 2
  br i1 %_5.i, label %bb22, label %bb2.i

bb2.i:                                            ; preds = %bb3
  %_8.i = add nuw nsw i64 %v.sroa.16.0.lcssa, 16
  %_6.i = icmp samesign ult i64 %scratch.1, %_8.i
  br i1 %_6.i, label %bb3.i, label %bb4.i

bb4.i:                                            ; preds = %bb2.i
  %len_div_29.i = lshr i64 %v.sroa.16.0.lcssa, 1
  %_22.i = icmp samesign ugt i64 %v.sroa.16.0.lcssa, 7
  br i1 %_22.i, label %bb10.i, label %bb33.i

bb3.i:                                            ; preds = %bb2.i
  call void @llvm.trap()
  unreachable

bb10.i:                                           ; preds = %bb4.i
  %6 = icmp ne ptr %is_less.val23, null
  call void @llvm.assume(i1 %6)
; call core::slice::sort::shared::smallsort::sort4_stable::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  call fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort12sort4_stableINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1z_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB19_7sort_byNCNvMs3_B1z_NtB1z_8IntoIter4pushs_0E0EB1z_(ptr noundef nonnull align 8 %v.sroa.0.0.ph.lcssa127, ptr noundef nonnull align 8 %scratch.0, ptr nonnull readonly %is_less.val23)
  %_27.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph.lcssa127, i64 %len_div_29.i
  %_28.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %len_div_29.i
; call core::slice::sort::shared::smallsort::sort4_stable::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  call fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort12sort4_stableINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1z_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB19_7sort_byNCNvMs3_B1z_NtB1z_8IntoIter4pushs_0E0EB1z_(ptr noundef %_27.i, ptr noundef %_28.i, ptr nonnull readonly %is_less.val23)
  br label %bb15.i

bb33.i:                                           ; preds = %bb4.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %scratch.0, ptr noundef nonnull align 8 dereferenceable(56) %v.sroa.0.0.ph.lcssa127, i64 56, i1 false), !alias.scope !442
  %_31.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph.lcssa127, i64 %len_div_29.i
  %dst.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %len_div_29.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i, ptr noundef nonnull align 8 dereferenceable(56) %_31.i, i64 56, i1 false), !alias.scope !442
  br label %bb15.i

bb15.i:                                           ; preds = %bb33.i, %bb10.i
  %presorted_len.sroa.0.0.i = phi i64 [ 4, %bb10.i ], [ 1, %bb33.i ]
  %7 = sub nsw i64 %v.sroa.16.0.lcssa, %len_div_29.i
  %8 = icmp ne ptr %is_less.val23, null
  %a1.i.i9.i.i = getelementptr inbounds nuw i8, ptr %tmp.i.i, i64 8
  %_746.i = icmp samesign ult i64 %presorted_len.sroa.0.0.i, %len_div_29.i
  br i1 %_746.i, label %bb38.lr.ph.i, label %bb16.loopexit.i

bb16.loopexit.i:                                  ; preds = %bb24.i, %bb15.i
  %src.1.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph.lcssa127, i64 %len_div_29.i
  %dst5.1.i = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %len_div_29.i
  %_746.1.i = icmp ult i64 %presorted_len.sroa.0.0.i, %7
  br i1 %_746.1.i, label %bb38.lr.ph.1.i, label %bb16.loopexit.1.i

bb38.lr.ph.1.i:                                   ; preds = %bb16.loopexit.i
  call void @llvm.assume(i1 %8)
  br label %bb38.1.i

bb38.1.i:                                         ; preds = %bb24.1.i, %bb38.lr.ph.1.i
  %iter1.sroa.0.07.1.i = phi i64 [ %presorted_len.sroa.0.0.i, %bb38.lr.ph.1.i ], [ %_78.1.i, %bb24.1.i ]
  %_49.1.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %src.1.i, i64 %iter1.sroa.0.07.1.i
  %dst6.idx.1.i = mul nuw nsw i64 %iter1.sroa.0.07.1.i, 56
  %dst6.1.i = getelementptr inbounds nuw i8, ptr %dst5.1.i, i64 %dst6.idx.1.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst6.1.i, ptr noundef nonnull align 8 dereferenceable(56) %_49.1.i, i64 56, i1 false), !alias.scope !442
  %9 = getelementptr inbounds i8, ptr %dst6.1.i, i64 -56
  call void @llvm.experimental.noalias.scope.decl(metadata !443)
  call void @llvm.experimental.noalias.scope.decl(metadata !446)
  %_7.val.i.i17.1.i = load ptr, ptr %is_less.val23, align 8, !noalias !448
  call void @llvm.experimental.noalias.scope.decl(metadata !449)
  call void @llvm.experimental.noalias.scope.decl(metadata !452)
  %10 = load i64, ptr %dst6.1.i, align 8, !range !16, !alias.scope !454, !noalias !455, !noundef !12
  %.not.i.i.i18.1.i = icmp ne i64 %10, -9223372036854775807
  %11 = load i64, ptr %9, align 8, !range !16, !alias.scope !456, !noalias !457, !noundef !12
  %.not3.i.i.i19.1.i = icmp eq i64 %11, -9223372036854775807
  %.not3.i.not.i.i20.1.i = xor i1 %.not3.i.i.i19.1.i, true
  %brmerge.i.i21.1.i = or i1 %.not.i.i.i18.1.i, %.not3.i.not.i.i20.1.i
  br i1 %brmerge.i.i21.1.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.1.i, label %bb7.i.i.i22.1.i

bb7.i.i.i22.1.i:                                  ; preds = %bb38.1.i
  %a1.i.i.i23.1.i = getelementptr inbounds nuw i8, ptr %dst6.1.i, i64 8
  %b2.i.i.i24.1.i = getelementptr inbounds i8, ptr %dst6.1.i, i64 -48
  %12 = icmp ne ptr %_7.val.i.i17.1.i, null
  call void @llvm.assume(i1 %12)
  %_11.i.i.i25.1.i = load ptr, ptr %_7.val.i.i17.1.i, align 8, !noalias !458, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i26.1.i = load ptr, ptr %_11.i.i.i25.1.i, align 8, !noalias !458, !nonnull !12, !noundef !12
  %13 = getelementptr inbounds nuw i8, ptr %_11.i.i.i25.1.i, i64 8
  %_14.1.i.i.i27.1.i = load ptr, ptr %13, align 8, !noalias !458, !nonnull !12, !align !132, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i27.1.i, i64 32
  %15 = load ptr, ptr %14, align 8, !invariant.load !12, !noalias !458, !nonnull !12
  %16 = call noundef i8 %15(ptr noundef nonnull align 1 %_14.0.i.i.i26.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i23.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i24.1.i) #31
  %17 = icmp eq i8 %16, -1
  br i1 %17, label %bb2.i.1.i, label %bb24.1.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.1.i: ; preds = %bb38.1.i
  %.not3.i.mux.i.i29.1.i = and i1 %.not.i.i.i18.1.i, %.not3.i.i.i19.1.i
  br i1 %.not3.i.mux.i.i29.1.i, label %bb2.i.1.i, label %bb24.1.i

bb2.i.1.i:                                        ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.1.i, %bb7.i.i.i22.1.i
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %tmp.i.i), !noalias !442
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %tmp.i.i, ptr noundef nonnull align 8 dereferenceable(56) %dst6.1.i, i64 56, i1 false), !noalias !437
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst6.1.i, ptr noundef nonnull align 8 dereferenceable(56) %9, i64 56, i1 false), !alias.scope !440, !noalias !437
  %_183.i.1.i = icmp eq i64 %iter1.sroa.0.07.1.i, 1
  br i1 %_183.i.1.i, label %bb10.i.1.i, label %bb6.i.1.i

bb6.i.1.i:                                        ; preds = %bb2.i.1.i, %bb4.backedge.i.1.i
  %sift.sroa.0.04.i.1.i = phi ptr [ %18, %bb4.backedge.i.1.i ], [ %9, %bb2.i.1.i ]
  %18 = getelementptr inbounds i8, ptr %sift.sroa.0.04.i.1.i, i64 -56
  call void @llvm.experimental.noalias.scope.decl(metadata !459)
  call void @llvm.experimental.noalias.scope.decl(metadata !462)
  %_7.val.i2.i.1.i = load ptr, ptr %is_less.val23, align 8, !noalias !464
  call void @llvm.experimental.noalias.scope.decl(metadata !465)
  call void @llvm.experimental.noalias.scope.decl(metadata !468)
  %19 = load i64, ptr %tmp.i.i, align 8, !range !16, !alias.scope !470, !noalias !471, !noundef !12
  %.not.i.i3.i.1.i = icmp ne i64 %19, -9223372036854775807
  %20 = load i64, ptr %18, align 8, !range !16, !alias.scope !472, !noalias !473, !noundef !12
  %.not3.i.i4.i.1.i = icmp eq i64 %20, -9223372036854775807
  %.not3.i.not.i5.i.1.i = xor i1 %.not3.i.i4.i.1.i, true
  %brmerge.i6.i.1.i = or i1 %.not.i.i3.i.1.i, %.not3.i.not.i5.i.1.i
  br i1 %brmerge.i6.i.1.i, label %bb7.i.1.i, label %bb7.i.i8.i.1.i

bb7.i.i8.i.1.i:                                   ; preds = %bb6.i.1.i
  %b2.i.i10.i.1.i = getelementptr inbounds i8, ptr %sift.sroa.0.04.i.1.i, i64 -48
  %21 = icmp ne ptr %_7.val.i2.i.1.i, null
  call void @llvm.assume(i1 %21)
  %_11.i.i11.i.1.i = load ptr, ptr %_7.val.i2.i.1.i, align 8, !noalias !474, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i12.i.1.i = load ptr, ptr %_11.i.i11.i.1.i, align 8, !noalias !474, !nonnull !12, !noundef !12
  %22 = getelementptr inbounds nuw i8, ptr %_11.i.i11.i.1.i, i64 8
  %_14.1.i.i13.i.1.i = load ptr, ptr %22, align 8, !noalias !474, !nonnull !12, !align !132, !noundef !12
  %23 = getelementptr inbounds nuw i8, ptr %_14.1.i.i13.i.1.i, i64 32
  %24 = load ptr, ptr %23, align 8, !invariant.load !12, !noalias !474, !nonnull !12
  %25 = invoke noundef i8 %24(ptr noundef nonnull align 1 %_14.0.i.i12.i.1.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i9.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i10.i.1.i) #31
          to label %.noexc.i.1.i unwind label %bb14.i.loopexit.split-lp.i

.noexc.i.1.i:                                     ; preds = %bb7.i.i8.i.1.i
  %26 = icmp eq i8 %25, -1
  br i1 %26, label %bb4.backedge.i.1.i, label %bb10.i.1.i

bb7.i.1.i:                                        ; preds = %bb6.i.1.i
  %.not3.i.mux.i7.i.1.i = and i1 %.not.i.i3.i.1.i, %.not3.i.i4.i.1.i
  br i1 %.not3.i.mux.i7.i.1.i, label %bb4.backedge.i.1.i, label %bb10.i.1.i

bb4.backedge.i.1.i:                               ; preds = %bb7.i.1.i, %.noexc.i.1.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.04.i.1.i, ptr noundef nonnull align 8 dereferenceable(56) %18, i64 56, i1 false), !alias.scope !440, !noalias !437
  %_18.i.1.i = icmp eq ptr %18, %dst5.1.i
  br i1 %_18.i.1.i, label %bb10.i.1.i, label %bb6.i.1.i

bb10.i.1.i:                                       ; preds = %bb4.backedge.i.1.i, %bb7.i.1.i, %.noexc.i.1.i, %bb2.i.1.i
  %sift.sroa.0.0.lcssa.i.1.i = phi ptr [ %9, %bb2.i.1.i ], [ %sift.sroa.0.04.i.1.i, %.noexc.i.1.i ], [ %sift.sroa.0.04.i.1.i, %bb7.i.1.i ], [ %dst5.1.i, %bb4.backedge.i.1.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.0.lcssa.i.1.i, ptr noundef nonnull align 8 dereferenceable(56) %tmp.i.i, i64 56, i1 false), !noalias !475
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %tmp.i.i), !noalias !442
  br label %bb24.1.i

bb24.1.i:                                         ; preds = %bb10.i.1.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.1.i, %bb7.i.i.i22.1.i
  %_78.1.i = add i64 %iter1.sroa.0.07.1.i, 1
  %exitcond.1.not.i = icmp eq i64 %_78.1.i, %7
  br i1 %exitcond.1.not.i, label %bb16.loopexit.1.i, label %bb38.1.i

bb16.loopexit.1.i:                                ; preds = %bb24.1.i, %bb16.loopexit.i
  call void @llvm.experimental.noalias.scope.decl(metadata !480)
  %27 = getelementptr i8, ptr %dst5.1.i, i64 -56
  %count1.i.i = add nsw i64 %v.sroa.16.0.lcssa, -1
  %28 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %count1.i.i
  %29 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph.lcssa127, i64 %count1.i.i
  call void @llvm.assume(i1 %8)
  br label %bb15.i.i

bb38.lr.ph.i:                                     ; preds = %bb15.i
  call void @llvm.assume(i1 %8)
  br label %bb38.i

bb16.i.i:                                         ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i
  %30 = getelementptr i8, ptr %49, i64 56
  %31 = getelementptr i8, ptr %48, i64 56
  %_43.i.i = and i64 %v.sroa.16.0.lcssa, 1
  %_22.i.i = icmp eq i64 %_43.i.i, 0
  br i1 %_22.i.i, label %bb9.i.i, label %bb5.i.i

bb15.i.i:                                         ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i, %bb16.loopexit.1.i
  %dst.sroa.0.08.i.i = phi ptr [ %v.sroa.0.0.ph.lcssa127, %bb16.loopexit.1.i ], [ %_16.i.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %iter.sroa.0.07.i.i = phi i64 [ 0, %bb16.loopexit.1.i ], [ %_39.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %left.sroa.0.06.i.i = phi ptr [ %scratch.0, %bb16.loopexit.1.i ], [ %_14.i.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %right.sroa.0.05.i.i = phi ptr [ %dst5.1.i, %bb16.loopexit.1.i ], [ %_12.i.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %left_rev.sroa.0.04.i.i = phi ptr [ %27, %bb16.loopexit.1.i ], [ %49, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %right_rev.sroa.0.03.i.i = phi ptr [ %28, %bb16.loopexit.1.i ], [ %48, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %dst_rev.sroa.0.02.i.i = phi ptr [ %29, %bb16.loopexit.1.i ], [ %50, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i ]
  %_39.i.i = add nuw nsw i64 %iter.sroa.0.07.i.i, 1
  call void @llvm.experimental.noalias.scope.decl(metadata !483)
  call void @llvm.experimental.noalias.scope.decl(metadata !486)
  %_7.val.i.i.i = load ptr, ptr %is_less.val23, align 8, !noalias !488
  call void @llvm.experimental.noalias.scope.decl(metadata !489)
  call void @llvm.experimental.noalias.scope.decl(metadata !492)
  %32 = load i64, ptr %right.sroa.0.05.i.i, align 8, !range !16, !alias.scope !494, !noalias !495, !noundef !12
  %.not.i.i.i.i = icmp ne i64 %32, -9223372036854775807
  %33 = load i64, ptr %left.sroa.0.06.i.i, align 8, !range !16, !alias.scope !496, !noalias !497, !noundef !12
  %.not3.i.i.i.i = icmp eq i64 %33, -9223372036854775807
  %.not3.i.not.i.i.i = xor i1 %.not3.i.i.i.i, true
  %brmerge.i.i.i = or i1 %.not.i.i.i.i, %.not3.i.not.i.i.i
  %.not3.i.mux.i.i.i = and i1 %.not.i.i.i.i, %.not3.i.i.i.i
  br i1 %brmerge.i.i.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i, label %bb7.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb15.i.i
  %a1.i.i.i.i = getelementptr inbounds nuw i8, ptr %right.sroa.0.05.i.i, i64 8
  %b2.i.i.i.i = getelementptr inbounds nuw i8, ptr %left.sroa.0.06.i.i, i64 8
  %34 = icmp ne ptr %_7.val.i.i.i, null
  call void @llvm.assume(i1 %34)
  %_11.i.i.i.i = load ptr, ptr %_7.val.i.i.i, align 8, !noalias !498, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i.i = load ptr, ptr %_11.i.i.i.i, align 8, !noalias !498, !nonnull !12, !noundef !12
  %35 = getelementptr inbounds nuw i8, ptr %_11.i.i.i.i, i64 8
  %_14.1.i.i.i.i = load ptr, ptr %35, align 8, !noalias !498, !nonnull !12, !align !132, !noundef !12
  %36 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i.i, i64 32
  %37 = load ptr, ptr %36, align 8, !invariant.load !12, !noalias !498, !nonnull !12
  %38 = invoke noundef i8 %37(ptr noundef nonnull align 1 %_14.0.i.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i.i) #31
          to label %.noexc.i unwind label %cleanup2.loopexit.i

.noexc.i:                                         ; preds = %bb7.i.i.i.i
  %39 = icmp eq i8 %38, -1
  %_7.val.i20.i.pre.i = load ptr, ptr %is_less.val23, align 8, !noalias !499
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i: ; preds = %.noexc.i, %bb15.i.i
  %_7.val.i20.i.i = phi ptr [ %_7.val.i20.i.pre.i, %.noexc.i ], [ %_7.val.i.i.i, %bb15.i.i ]
  %_0.sroa.0.0.i.i.i.i = phi i1 [ %39, %.noexc.i ], [ %.not3.i.mux.i.i.i, %bb15.i.i ]
  %..i17.i.i = select i1 %_0.sroa.0.0.i.i.i.i, ptr %right.sroa.0.05.i.i, ptr %left.sroa.0.06.i.i
  %is_l.i18.i.i = xor i1 %_0.sroa.0.0.i.i.i.i, true
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.sroa.0.08.i.i, ptr noundef nonnull align 8 dereferenceable(56) %..i17.i.i, i64 56, i1 false), !alias.scope !442, !noalias !503
  %count.i.i.i = zext i1 %_0.sroa.0.0.i.i.i.i to i64
  %_12.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %right.sroa.0.05.i.i, i64 %count.i.i.i
  %count2.i.i.i = zext i1 %is_l.i18.i.i to i64
  %_14.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %left.sroa.0.06.i.i, i64 %count2.i.i.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %dst.sroa.0.08.i.i, i64 56
  call void @llvm.experimental.noalias.scope.decl(metadata !507)
  call void @llvm.experimental.noalias.scope.decl(metadata !508)
  call void @llvm.experimental.noalias.scope.decl(metadata !509)
  call void @llvm.experimental.noalias.scope.decl(metadata !512)
  %40 = load i64, ptr %right_rev.sroa.0.03.i.i, align 8, !range !16, !alias.scope !514, !noalias !515, !noundef !12
  %.not.i.i21.i.i = icmp ne i64 %40, -9223372036854775807
  %41 = load i64, ptr %left_rev.sroa.0.04.i.i, align 8, !range !16, !alias.scope !516, !noalias !517, !noundef !12
  %.not3.i.i22.i.i = icmp eq i64 %41, -9223372036854775807
  %.not3.i.not.i23.i.i = xor i1 %.not3.i.i22.i.i, true
  %brmerge.i24.i.i = or i1 %.not.i.i21.i.i, %.not3.i.not.i23.i.i
  %.not3.i.mux.i25.i.i = and i1 %.not.i.i21.i.i, %.not3.i.i22.i.i
  br i1 %brmerge.i24.i.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i, label %bb7.i.i26.i.i

bb7.i.i26.i.i:                                    ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i
  %a1.i.i27.i.i = getelementptr inbounds nuw i8, ptr %right_rev.sroa.0.03.i.i, i64 8
  %b2.i.i28.i.i = getelementptr inbounds nuw i8, ptr %left_rev.sroa.0.04.i.i, i64 8
  %42 = icmp ne ptr %_7.val.i20.i.i, null
  call void @llvm.assume(i1 %42)
  %_11.i.i29.i.i = load ptr, ptr %_7.val.i20.i.i, align 8, !noalias !518, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i30.i.i = load ptr, ptr %_11.i.i29.i.i, align 8, !noalias !518, !nonnull !12, !noundef !12
  %43 = getelementptr inbounds nuw i8, ptr %_11.i.i29.i.i, i64 8
  %_14.1.i.i31.i.i = load ptr, ptr %43, align 8, !noalias !518, !nonnull !12, !align !132, !noundef !12
  %44 = getelementptr inbounds nuw i8, ptr %_14.1.i.i31.i.i, i64 32
  %45 = load ptr, ptr %44, align 8, !invariant.load !12, !noalias !518, !nonnull !12
  %46 = invoke noundef i8 %45(ptr noundef nonnull align 1 %_14.0.i.i30.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i27.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i28.i.i) #31
          to label %.noexc14.i unwind label %cleanup2.loopexit.i

.noexc14.i:                                       ; preds = %bb7.i.i26.i.i
  %47 = icmp eq i8 %46, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit33.i.i: ; preds = %.noexc14.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i
  %_0.sroa.0.0.i.i32.i.i = phi i1 [ %47, %.noexc14.i ], [ %.not3.i.mux.i25.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i.i ]
  %..i.i.i = select i1 %_0.sroa.0.0.i.i32.i.i, ptr %left_rev.sroa.0.04.i.i, ptr %right_rev.sroa.0.03.i.i
  %is_l.i.i.i = xor i1 %_0.sroa.0.0.i.i32.i.i, true
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst_rev.sroa.0.02.i.i, ptr noundef nonnull align 8 dereferenceable(56) %..i.i.i, i64 56, i1 false), !alias.scope !442, !noalias !519
  %count.neg.i.i.i = sext i1 %is_l.i.i.i to i64
  %48 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %right_rev.sroa.0.03.i.i, i64 %count.neg.i.i.i
  %count3.neg.i.i.i = sext i1 %_0.sroa.0.0.i.i32.i.i to i64
  %49 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %left_rev.sroa.0.04.i.i, i64 %count3.neg.i.i.i
  %50 = getelementptr inbounds i8, ptr %dst_rev.sroa.0.02.i.i, i64 -56
  %exitcond.not.i.i = icmp eq i64 %_39.i.i, %len_div_29.i
  br i1 %exitcond.not.i.i, label %bb16.i.i, label %bb15.i.i

bb5.i.i:                                          ; preds = %bb16.i.i
  %left_nonempty.i.i = icmp ult ptr %_14.i.i.i, %30
  %left.sroa.0.0.right.sroa.0.0.i.i = select i1 %left_nonempty.i.i, ptr %_14.i.i.i, ptr %_12.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_16.i.i.i, ptr noundef nonnull align 8 dereferenceable(56) %left.sroa.0.0.right.sroa.0.0.i.i, i64 56, i1 false), !alias.scope !442
  %count2.i.i = zext i1 %left_nonempty.i.i to i64
  %_26.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_14.i.i.i, i64 %count2.i.i
  %_30.i.i = xor i1 %left_nonempty.i.i, true
  %count3.i.i = zext i1 %_30.i.i to i64
  %_28.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_12.i.i.i, i64 %count3.i.i
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb5.i.i, %bb16.i.i
  %right.sroa.0.1.i.i = phi ptr [ %_12.i.i.i, %bb16.i.i ], [ %_28.i.i, %bb5.i.i ]
  %left.sroa.0.1.i.i = phi ptr [ %_14.i.i.i, %bb16.i.i ], [ %_26.i.i, %bb5.i.i ]
  %_31.i.i = icmp ne ptr %left.sroa.0.1.i.i, %30
  %_32.i.i = icmp ne ptr %right.sroa.0.1.i.i, %31
  %or.cond.i.i = select i1 %_31.i.i, i1 true, i1 %_32.i.i
  br i1 %or.cond.i.i, label %bb13.i.i, label %bb22, !prof !523

bb13.i.i:                                         ; preds = %bb9.i.i
; invoke core::slice::sort::shared::smallsort::panic_on_ord_violation
  invoke void @_RNvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort22panic_on_ord_violation() #32
          to label %.noexc15.i unwind label %cleanup2.loopexit.split-lp.i

.noexc15.i:                                       ; preds = %bb13.i.i
  unreachable

cleanup2.loopexit.i:                              ; preds = %bb7.i.i26.i.i, %bb7.i.i.i.i
  %lpad.loopexit.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2.i

cleanup2.loopexit.split-lp.i:                     ; preds = %bb13.i.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2.i

cleanup2.i:                                       ; preds = %cleanup2.loopexit.split-lp.i, %cleanup2.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit.i, %cleanup2.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup2.loopexit.split-lp.i ]
  %51 = mul nuw nsw i64 %v.sroa.16.0.lcssa, 56
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %v.sroa.0.0.ph.lcssa127, ptr nonnull align 8 %scratch.0, i64 %51, i1 false), !alias.scope !442, !noalias !524
  br label %bb29.i

bb29.i:                                           ; preds = %bb14.i.i, %cleanup2.i
  %.pn.i = phi { ptr, i32 } [ %lpad.phi.i, %cleanup2.i ], [ %lpad.phi13.i, %bb14.i.i ]
  resume { ptr, i32 } %.pn.i

bb38.i:                                           ; preds = %bb24.i, %bb38.lr.ph.i
  %iter1.sroa.0.07.i = phi i64 [ %presorted_len.sroa.0.0.i, %bb38.lr.ph.i ], [ %_78.i, %bb24.i ]
  %_49.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph.lcssa127, i64 %iter1.sroa.0.07.i
  %dst6.idx.i = mul nuw nsw i64 %iter1.sroa.0.07.i, 56
  %dst6.i = getelementptr inbounds nuw i8, ptr %scratch.0, i64 %dst6.idx.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst6.i, ptr noundef nonnull align 8 dereferenceable(56) %_49.i, i64 56, i1 false), !alias.scope !442
  %52 = getelementptr inbounds i8, ptr %dst6.i, i64 -56
  call void @llvm.experimental.noalias.scope.decl(metadata !529)
  call void @llvm.experimental.noalias.scope.decl(metadata !531)
  %_7.val.i.i17.i = load ptr, ptr %is_less.val23, align 8, !noalias !533
  call void @llvm.experimental.noalias.scope.decl(metadata !534)
  call void @llvm.experimental.noalias.scope.decl(metadata !536)
  %53 = load i64, ptr %dst6.i, align 8, !range !16, !alias.scope !538, !noalias !539, !noundef !12
  %.not.i.i.i18.i = icmp ne i64 %53, -9223372036854775807
  %54 = load i64, ptr %52, align 8, !range !16, !alias.scope !540, !noalias !541, !noundef !12
  %.not3.i.i.i19.i = icmp eq i64 %54, -9223372036854775807
  %.not3.i.not.i.i20.i = xor i1 %.not3.i.i.i19.i, true
  %brmerge.i.i21.i = or i1 %.not.i.i.i18.i, %.not3.i.not.i.i20.i
  br i1 %brmerge.i.i21.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.i, label %bb7.i.i.i22.i

bb7.i.i.i22.i:                                    ; preds = %bb38.i
  %a1.i.i.i23.i = getelementptr inbounds nuw i8, ptr %dst6.i, i64 8
  %b2.i.i.i24.i = getelementptr inbounds i8, ptr %dst6.i, i64 -48
  %55 = icmp ne ptr %_7.val.i.i17.i, null
  call void @llvm.assume(i1 %55)
  %_11.i.i.i25.i = load ptr, ptr %_7.val.i.i17.i, align 8, !noalias !542, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i26.i = load ptr, ptr %_11.i.i.i25.i, align 8, !noalias !542, !nonnull !12, !noundef !12
  %56 = getelementptr inbounds nuw i8, ptr %_11.i.i.i25.i, i64 8
  %_14.1.i.i.i27.i = load ptr, ptr %56, align 8, !noalias !542, !nonnull !12, !align !132, !noundef !12
  %57 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i27.i, i64 32
  %58 = load ptr, ptr %57, align 8, !invariant.load !12, !noalias !542, !nonnull !12
  %59 = call noundef i8 %58(ptr noundef nonnull align 1 %_14.0.i.i.i26.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i23.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i24.i) #31
  %60 = icmp eq i8 %59, -1
  br i1 %60, label %bb2.i.i, label %bb24.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.i: ; preds = %bb38.i
  %.not3.i.mux.i.i29.i = and i1 %.not.i.i.i18.i, %.not3.i.i.i19.i
  br i1 %.not3.i.mux.i.i29.i, label %bb2.i.i, label %bb24.i

bb2.i.i:                                          ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.i, %bb7.i.i.i22.i
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %tmp.i.i), !noalias !442
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %tmp.i.i, ptr noundef nonnull align 8 dereferenceable(56) %dst6.i, i64 56, i1 false), !noalias !437
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst6.i, ptr noundef nonnull align 8 dereferenceable(56) %52, i64 56, i1 false), !alias.scope !440, !noalias !437
  %_183.i.i = icmp eq i64 %iter1.sroa.0.07.i, 1
  br i1 %_183.i.i, label %bb10.i.i, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb2.i.i, %bb4.backedge.i.i
  %sift.sroa.0.04.i.i = phi ptr [ %61, %bb4.backedge.i.i ], [ %52, %bb2.i.i ]
  %61 = getelementptr inbounds i8, ptr %sift.sroa.0.04.i.i, i64 -56
  call void @llvm.experimental.noalias.scope.decl(metadata !543)
  call void @llvm.experimental.noalias.scope.decl(metadata !545)
  %_7.val.i2.i.i = load ptr, ptr %is_less.val23, align 8, !noalias !547
  call void @llvm.experimental.noalias.scope.decl(metadata !548)
  call void @llvm.experimental.noalias.scope.decl(metadata !550)
  %62 = load i64, ptr %tmp.i.i, align 8, !range !16, !alias.scope !552, !noalias !553, !noundef !12
  %.not.i.i3.i.i = icmp ne i64 %62, -9223372036854775807
  %63 = load i64, ptr %61, align 8, !range !16, !alias.scope !554, !noalias !555, !noundef !12
  %.not3.i.i4.i.i = icmp eq i64 %63, -9223372036854775807
  %.not3.i.not.i5.i.i = xor i1 %.not3.i.i4.i.i, true
  %brmerge.i6.i.i = or i1 %.not.i.i3.i.i, %.not3.i.not.i5.i.i
  br i1 %brmerge.i6.i.i, label %bb7.i.i, label %bb7.i.i8.i.i

bb7.i.i8.i.i:                                     ; preds = %bb6.i.i
  %b2.i.i10.i.i = getelementptr inbounds i8, ptr %sift.sroa.0.04.i.i, i64 -48
  %64 = icmp ne ptr %_7.val.i2.i.i, null
  call void @llvm.assume(i1 %64)
  %_11.i.i11.i.i = load ptr, ptr %_7.val.i2.i.i, align 8, !noalias !556, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i12.i.i = load ptr, ptr %_11.i.i11.i.i, align 8, !noalias !556, !nonnull !12, !noundef !12
  %65 = getelementptr inbounds nuw i8, ptr %_11.i.i11.i.i, i64 8
  %_14.1.i.i13.i.i = load ptr, ptr %65, align 8, !noalias !556, !nonnull !12, !align !132, !noundef !12
  %66 = getelementptr inbounds nuw i8, ptr %_14.1.i.i13.i.i, i64 32
  %67 = load ptr, ptr %66, align 8, !invariant.load !12, !noalias !556, !nonnull !12
  %68 = invoke noundef i8 %67(ptr noundef nonnull align 1 %_14.0.i.i12.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i9.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i10.i.i) #31
          to label %.noexc.i.i unwind label %bb14.i.loopexit.i

.noexc.i.i:                                       ; preds = %bb7.i.i8.i.i
  %69 = icmp eq i8 %68, -1
  br i1 %69, label %bb4.backedge.i.i, label %bb10.i.i

bb7.i.i:                                          ; preds = %bb6.i.i
  %.not3.i.mux.i7.i.i = and i1 %.not.i.i3.i.i, %.not3.i.i4.i.i
  br i1 %.not3.i.mux.i7.i.i, label %bb4.backedge.i.i, label %bb10.i.i

bb4.backedge.i.i:                                 ; preds = %bb7.i.i, %.noexc.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.04.i.i, ptr noundef nonnull align 8 dereferenceable(56) %61, i64 56, i1 false), !alias.scope !440, !noalias !437
  %_18.i.i = icmp eq ptr %61, %scratch.0
  br i1 %_18.i.i, label %bb10.i.i, label %bb6.i.i

bb10.i.i:                                         ; preds = %bb4.backedge.i.i, %bb7.i.i, %.noexc.i.i, %bb2.i.i
  %sift.sroa.0.0.lcssa.i.i = phi ptr [ %52, %bb2.i.i ], [ %sift.sroa.0.04.i.i, %.noexc.i.i ], [ %sift.sroa.0.04.i.i, %bb7.i.i ], [ %scratch.0, %bb4.backedge.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.0.lcssa.i.i, ptr noundef nonnull align 8 dereferenceable(56) %tmp.i.i, i64 56, i1 false), !noalias !475
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %tmp.i.i), !noalias !442
  br label %bb24.i

bb14.i.loopexit.i:                                ; preds = %bb7.i.i8.i.i
  %lpad.loopexit11.i = landingpad { ptr, i32 }
          cleanup
  br label %bb14.i.i

bb14.i.loopexit.split-lp.i:                       ; preds = %bb7.i.i8.i.1.i
  %lpad.loopexit.split-lp12.i = landingpad { ptr, i32 }
          cleanup
  br label %bb14.i.i

bb14.i.i:                                         ; preds = %bb14.i.loopexit.split-lp.i, %bb14.i.loopexit.i
  %sift.sroa.0.04.i.lcssa.i = phi ptr [ %sift.sroa.0.04.i.i, %bb14.i.loopexit.i ], [ %sift.sroa.0.04.i.1.i, %bb14.i.loopexit.split-lp.i ]
  %lpad.phi13.i = phi { ptr, i32 } [ %lpad.loopexit11.i, %bb14.i.loopexit.i ], [ %lpad.loopexit.split-lp12.i, %bb14.i.loopexit.split-lp.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %sift.sroa.0.04.i.lcssa.i, ptr noundef nonnull align 8 dereferenceable(56) %tmp.i.i, i64 56, i1 false), !noalias !557
  br label %bb29.i

bb24.i:                                           ; preds = %bb10.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i28.i, %bb7.i.i.i22.i
  %_78.i = add i64 %iter1.sroa.0.07.i, 1
  %exitcond.not.i = icmp eq i64 %_78.i, %len_div_29.i
  br i1 %exitcond.not.i, label %bb16.loopexit.i, label %bb38.i

bb6:                                              ; preds = %bb5
; call core::slice::sort::stable::drift::sort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  call fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift4sortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBW_7sort_byNCNvMs3_B1m_NtB1m_8IntoIter4pushs_0E0EB1m_(ptr noalias noundef nonnull align 8 %v.sroa.0.0.ph138, i64 noundef %v.sroa.16.0130, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef %scratch.1, i1 noundef zeroext true, ptr noalias noundef align 8 dereferenceable(8) %is_less)
  br label %bb22

bb7:                                              ; preds = %bb5
  %70 = add i32 %limit.sroa.0.0129, -1
  call void @llvm.experimental.noalias.scope.decl(metadata !562)
  call void @llvm.experimental.noalias.scope.decl(metadata !565)
  %len_div_84.i = lshr i64 %v.sroa.16.0130, 3
  %b.idx.i = mul nuw nsw i64 %len_div_84.i, 224
  %b.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.0.ph138, i64 %b.idx.i
  %c.idx.i = mul nuw nsw i64 %len_div_84.i, 392
  %c.i = getelementptr inbounds nuw i8, ptr %v.sroa.0.0.ph138, i64 %c.idx.i
  %_12.i = icmp samesign ult i64 %v.sroa.16.0130, 64
  br i1 %_12.i, label %bb3.i25, label %bb5.i

bb5.i:                                            ; preds = %bb7
; call core::slice::sort::shared::pivot::median3_rec::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  %self.i = call fastcc noundef ptr @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot11median3_recINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_(ptr noundef nonnull readonly align 8 %v.sroa.0.0.ph138, ptr noundef readonly %b.i, ptr noundef readonly %c.i, i64 noundef %len_div_84.i, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less)
  br label %bb10

bb3.i25:                                          ; preds = %bb7
  %is_less.val6.i = load ptr, ptr %is_less, align 8, !alias.scope !565, !noalias !562, !nonnull !12, !align !132, !noundef !12
  call void @llvm.experimental.noalias.scope.decl(metadata !567)
  call void @llvm.experimental.noalias.scope.decl(metadata !570)
  %_7.val.i.i = load ptr, ptr %is_less.val6.i, align 8, !noalias !572
  call void @llvm.experimental.noalias.scope.decl(metadata !573)
  call void @llvm.experimental.noalias.scope.decl(metadata !576)
  %71 = load i64, ptr %v.sroa.0.0.ph138, align 8, !range !16, !alias.scope !578, !noalias !579, !noundef !12
  %.not.i.i.i = icmp ne i64 %71, -9223372036854775807
  %72 = load i64, ptr %b.i, align 8, !range !16, !alias.scope !580, !noalias !581, !noundef !12
  %.not3.i.i.i = icmp eq i64 %72, -9223372036854775807
  %.not3.i.not.i.i = xor i1 %.not3.i.i.i, true
  %brmerge.i.i = or i1 %.not.i.i.i, %.not3.i.not.i.i
  %.not3.i.mux.i.i = and i1 %.not.i.i.i, %.not3.i.i.i
  br i1 %brmerge.i.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i, label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb3.i25
  %b2.i.i.i = getelementptr inbounds nuw i8, ptr %b.i, i64 8
  %73 = icmp ne ptr %_7.val.i.i, null
  call void @llvm.assume(i1 %73)
  %_11.i.i.i = load ptr, ptr %_7.val.i.i, align 8, !noalias !582, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i = load ptr, ptr %_11.i.i.i, align 8, !noalias !582, !nonnull !12, !noundef !12
  %74 = getelementptr inbounds nuw i8, ptr %_11.i.i.i, i64 8
  %_14.1.i.i.i = load ptr, ptr %74, align 8, !noalias !582, !nonnull !12, !align !132, !noundef !12
  %75 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i, i64 32
  %76 = load ptr, ptr %75, align 8, !invariant.load !12, !noalias !582, !nonnull !12
  %77 = call noundef i8 %76(ptr noundef nonnull align 1 %_14.0.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i) #31, !noalias !565
  %78 = icmp eq i8 %77, -1
  %_7.val.i7.pre.i = load ptr, ptr %is_less.val6.i, align 8, !noalias !583
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i: ; preds = %bb7.i.i.i, %bb3.i25
  %_7.val.i7.i = phi ptr [ %_7.val.i7.pre.i, %bb7.i.i.i ], [ %_7.val.i.i, %bb3.i25 ]
  %_0.sroa.0.0.i.i.i = phi i1 [ %78, %bb7.i.i.i ], [ %.not3.i.mux.i.i, %bb3.i25 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !587)
  call void @llvm.experimental.noalias.scope.decl(metadata !588)
  %79 = load i64, ptr %c.i, align 8, !range !16, !alias.scope !591, !noalias !592, !noundef !12
  %.not3.i.i9.i = icmp eq i64 %79, -9223372036854775807
  %.not3.i.not.i10.i = xor i1 %.not3.i.i9.i, true
  %brmerge.i11.i = or i1 %.not.i.i.i, %.not3.i.not.i10.i
  %.not3.i.mux.i12.i = and i1 %.not.i.i.i, %.not3.i.i9.i
  br i1 %brmerge.i11.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i, label %bb7.i.i13.i

bb7.i.i13.i:                                      ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i
  %b2.i.i15.i = getelementptr inbounds nuw i8, ptr %c.i, i64 8
  %80 = icmp ne ptr %_7.val.i7.i, null
  call void @llvm.assume(i1 %80)
  %_11.i.i16.i = load ptr, ptr %_7.val.i7.i, align 8, !noalias !594, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i17.i = load ptr, ptr %_11.i.i16.i, align 8, !noalias !594, !nonnull !12, !noundef !12
  %81 = getelementptr inbounds nuw i8, ptr %_11.i.i16.i, i64 8
  %_14.1.i.i18.i = load ptr, ptr %81, align 8, !noalias !594, !nonnull !12, !align !132, !noundef !12
  %82 = getelementptr inbounds nuw i8, ptr %_14.1.i.i18.i, i64 32
  %83 = load ptr, ptr %82, align 8, !invariant.load !12, !noalias !594, !nonnull !12
  %84 = call noundef i8 %83(ptr noundef nonnull align 1 %_14.0.i.i17.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i15.i) #31, !noalias !565
  %85 = icmp eq i8 %84, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i: ; preds = %bb7.i.i13.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i
  %_0.sroa.0.0.i.i19.i = phi i1 [ %85, %bb7.i.i13.i ], [ %.not3.i.mux.i12.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i ]
  %86 = xor i1 %_0.sroa.0.0.i.i.i, %_0.sroa.0.0.i.i19.i
  br i1 %86, label %bb10, label %bb3.i.i

bb3.i.i:                                          ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i
  %.not.i.i22.i = icmp ne i64 %72, -9223372036854775807
  %brmerge.i25.i = or i1 %.not.i.i22.i, %.not3.i.not.i10.i
  %.not3.i.mux.i26.i = and i1 %.not.i.i22.i, %.not3.i.i9.i
  br i1 %brmerge.i25.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34.i, label %bb7.i.i27.i

bb7.i.i27.i:                                      ; preds = %bb3.i.i
  %_7.val.i21.i = load ptr, ptr %is_less.val6.i, align 8, !noalias !595, !nonnull !12, !noundef !12
  %a1.i.i28.i = getelementptr inbounds nuw i8, ptr %b.i, i64 8
  %b2.i.i29.i = getelementptr inbounds nuw i8, ptr %c.i, i64 8
  %_11.i.i30.i = load ptr, ptr %_7.val.i21.i, align 8, !noalias !599, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i31.i = load ptr, ptr %_11.i.i30.i, align 8, !noalias !599, !nonnull !12, !noundef !12
  %87 = getelementptr inbounds nuw i8, ptr %_11.i.i30.i, i64 8
  %_14.1.i.i32.i = load ptr, ptr %87, align 8, !noalias !599, !nonnull !12, !align !132, !noundef !12
  %88 = getelementptr inbounds nuw i8, ptr %_14.1.i.i32.i, i64 32
  %89 = load ptr, ptr %88, align 8, !invariant.load !12, !noalias !599, !nonnull !12
  %90 = call noundef i8 %89(ptr noundef nonnull align 1 %_14.0.i.i31.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i28.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i29.i) #31, !noalias !565
  %91 = icmp eq i8 %90, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34.i

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34.i: ; preds = %bb7.i.i27.i, %bb3.i.i
  %_0.sroa.0.0.i.i33.i = phi i1 [ %91, %bb7.i.i27.i ], [ %.not3.i.mux.i26.i, %bb3.i.i ]
  %_12.i.i = xor i1 %_0.sroa.0.0.i.i.i, %_0.sroa.0.0.i.i33.i
  %c.b.i.i = select i1 %_12.i.i, ptr %c.i, ptr %b.i
  br label %bb10

bb22:                                             ; preds = %bb3.thread, %bb9.i.i, %bb3, %bb6
  ret void

bb10:                                             ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i, %bb5.i
  %_0.sroa.0.0.i.sink.i = phi ptr [ %self.i, %bb5.i ], [ %v.sroa.0.0.ph138, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit20.i ], [ %c.b.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit34.i ]
  %92 = ptrtoint ptr %_0.sroa.0.0.i.sink.i to i64
  %93 = sub nuw i64 %92, %4
  %index.sroa.0.0.i = udiv exact i64 %93, 56
  %cond.i = icmp samesign ult i64 %index.sroa.0.0.i, %v.sroa.16.0130
  call void @llvm.assume(i1 %cond.i)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %pivot_copy)
  %src = getelementptr inbounds nuw i8, ptr %v.sroa.0.0.ph138, i64 %93
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %pivot_copy, ptr noundef nonnull align 8 dereferenceable(56) %src, i64 56, i1 false)
  %is_less.val24.pre174 = load ptr, ptr %is_less, align 8
  br i1 %.not, label %bb14, label %bb12

bb12:                                             ; preds = %bb10
  call void @llvm.experimental.noalias.scope.decl(metadata !603)
  call void @llvm.experimental.noalias.scope.decl(metadata !606)
  %_7.val.i = load ptr, ptr %is_less.val24.pre174, align 8, !noalias !608
  call void @llvm.experimental.noalias.scope.decl(metadata !609)
  call void @llvm.experimental.noalias.scope.decl(metadata !612)
  %94 = load i64, ptr %left_ancestor_pivot.sroa.0.0.ph135, align 8, !range !16, !alias.scope !614, !noalias !615, !noundef !12
  %.not.i.i = icmp ne i64 %94, -9223372036854775807
  %95 = load i64, ptr %src, align 8, !range !16, !alias.scope !615, !noalias !614, !noundef !12
  %.not3.i.i = icmp eq i64 %95, -9223372036854775807
  %.not3.i.not.i = xor i1 %.not3.i.i, true
  %brmerge.i = or i1 %.not.i.i, %.not3.i.not.i
  br i1 %brmerge.i, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, label %bb7.i.i26

bb7.i.i26:                                        ; preds = %bb12
  %b2.i.i = getelementptr inbounds nuw i8, ptr %src, i64 8
  %96 = icmp ne ptr %_7.val.i, null
  call void @llvm.assume(i1 %96)
  %_11.i.i = load ptr, ptr %_7.val.i, align 8, !noalias !616, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i = load ptr, ptr %_11.i.i, align 8, !noalias !616, !nonnull !12, !noundef !12
  %97 = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_14.1.i.i = load ptr, ptr %97, align 8, !noalias !616, !nonnull !12, !align !132, !noundef !12
  %98 = getelementptr inbounds nuw i8, ptr %_14.1.i.i, i64 32
  %99 = load ptr, ptr %98, align 8, !invariant.load !12, !noalias !616, !nonnull !12
  %100 = call noundef i8 %99(ptr noundef nonnull align 1 %_14.0.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i) #31
  %101 = icmp eq i8 %100, -1
  br i1 %101, label %bb7.i.i26.bb14_crit_edge, label %bb17

bb7.i.i26.bb14_crit_edge:                         ; preds = %bb7.i.i26
  %is_less.val24.pre = load ptr, ptr %is_less, align 8
  br label %bb14

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit: ; preds = %bb12
  %.not3.i.mux.i = and i1 %.not.i.i, %.not3.i.i
  br i1 %.not3.i.mux.i, label %bb14, label %bb17

bb14:                                             ; preds = %bb7.i.i26.bb14_crit_edge, %bb10, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit
  %is_less.val24 = phi ptr [ %is_less.val24.pre, %bb7.i.i26.bb14_crit_edge ], [ %is_less.val24.pre174, %bb10 ], [ %is_less.val24.pre174, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit ]
  call void @llvm.experimental.noalias.scope.decl(metadata !617)
  call void @llvm.experimental.noalias.scope.decl(metadata !620)
  %_8.i27.not = icmp samesign ult i64 %scratch.1, %v.sroa.16.0130
  br i1 %_8.i27.not, label %bb28.i, label %bb30.i, !prof !523

bb30.i:                                           ; preds = %bb14
  %_83.i = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %v.sroa.16.0130
  %102 = icmp ne ptr %is_less.val24, null
  %b2.i.i.i28 = getelementptr inbounds nuw i8, ptr %src, i64 8
  br label %bb3.i29

bb28.i:                                           ; preds = %bb14
  call void @llvm.trap()
  unreachable

bb3.i29:                                          ; preds = %bb23.i, %bb30.i
  %state.sroa.11.0.i = phi i64 [ 0, %bb30.i ], [ %state.sroa.11.1.lcssa.i, %bb23.i ]
  %state.sroa.5.0.i = phi ptr [ %v.sroa.0.0.ph138, %bb30.i ], [ %_9.i13.i, %bb23.i ]
  %state.sroa.19.0.i = phi ptr [ %_83.i, %bb30.i ], [ %113, %bb23.i ]
  %pivot_pos.sroa.0.0.i = phi i64 [ %index.sroa.0.0.i, %bb30.i ], [ %v.sroa.16.0130, %bb23.i ]
  %loop_end.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %pivot_pos.sroa.0.0.i
  %_476.i = icmp ult ptr %state.sroa.5.0.i, %loop_end.i
  br i1 %_476.i, label %bb18.lr.ph.i, label %bb21.i

bb18.lr.ph.i:                                     ; preds = %bb3.i29
  call void @llvm.assume(i1 %102)
  br label %bb18.i

bb21.i:                                           ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43, %bb3.i29
  %state.sroa.11.1.lcssa.i = phi i64 [ %state.sroa.11.0.i, %bb3.i29 ], [ %112, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  %state.sroa.5.1.lcssa.i = phi ptr [ %state.sroa.5.0.i, %bb3.i29 ], [ %_9.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  %state.sroa.19.1.lcssa.i = phi ptr [ %state.sroa.19.0.i, %bb3.i29 ], [ %111, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  %_55.i = icmp eq i64 %pivot_pos.sroa.0.0.i, %v.sroa.16.0130
  br i1 %_55.i, label %bb22.i, label %bb23.i

bb18.i:                                           ; preds = %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43, %bb18.lr.ph.i
  %state.sroa.19.19.i = phi ptr [ %state.sroa.19.0.i, %bb18.lr.ph.i ], [ %111, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  %state.sroa.5.18.i = phi ptr [ %state.sroa.5.0.i, %bb18.lr.ph.i ], [ %_9.i.i, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  %state.sroa.11.17.i = phi i64 [ %state.sroa.11.0.i, %bb18.lr.ph.i ], [ %112, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !622)
  call void @llvm.experimental.noalias.scope.decl(metadata !625)
  %_7.val.i.i32 = load ptr, ptr %is_less.val24, align 8, !noalias !627
  call void @llvm.experimental.noalias.scope.decl(metadata !628)
  call void @llvm.experimental.noalias.scope.decl(metadata !631)
  %103 = load i64, ptr %state.sroa.5.18.i, align 8, !range !16, !alias.scope !633, !noalias !634, !noundef !12
  %.not.i.i.i33 = icmp ne i64 %103, -9223372036854775807
  %104 = load i64, ptr %src, align 8, !range !16, !alias.scope !635, !noalias !636, !noundef !12
  %.not3.i.i.i34 = icmp eq i64 %104, -9223372036854775807
  %.not3.i.not.i.i35 = xor i1 %.not3.i.i.i34, true
  %brmerge.i.i36 = or i1 %.not.i.i.i33, %.not3.i.not.i.i35
  %.not3.i.mux.i.i37 = and i1 %.not.i.i.i33, %.not3.i.i.i34
  br i1 %brmerge.i.i36, label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43, label %bb7.i.i.i38

bb7.i.i.i38:                                      ; preds = %bb18.i
  %a1.i.i.i39 = getelementptr inbounds nuw i8, ptr %state.sroa.5.18.i, i64 8
  %105 = icmp ne ptr %_7.val.i.i32, null
  call void @llvm.assume(i1 %105)
  %_11.i.i.i40 = load ptr, ptr %_7.val.i.i32, align 8, !noalias !637, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i41 = load ptr, ptr %_11.i.i.i40, align 8, !noalias !637, !nonnull !12, !noundef !12
  %106 = getelementptr inbounds nuw i8, ptr %_11.i.i.i40, i64 8
  %_14.1.i.i.i42 = load ptr, ptr %106, align 8, !noalias !637, !nonnull !12, !align !132, !noundef !12
  %107 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i42, i64 32
  %108 = load ptr, ptr %107, align 8, !invariant.load !12, !noalias !637, !nonnull !12
  %109 = call noundef i8 %108(ptr noundef nonnull align 1 %_14.0.i.i.i41, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i39, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i28) #31, !noalias !620
  %110 = icmp eq i8 %109, -1
  br label %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43

_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit.i43: ; preds = %bb7.i.i.i38, %bb18.i
  %_0.sroa.0.0.i.i.i44 = phi i1 [ %110, %bb7.i.i.i38 ], [ %.not3.i.mux.i.i37, %bb18.i ]
  %111 = getelementptr inbounds i8, ptr %state.sroa.19.19.i, i64 -56
  %dst_base.sroa.0.0.i.i = select i1 %_0.sroa.0.0.i.i.i44, ptr %scratch.0, ptr %111
  %dst.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %dst_base.sroa.0.0.i.i, i64 %state.sroa.11.17.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i.i, ptr noundef nonnull align 8 dereferenceable(56) %state.sroa.5.18.i, i64 56, i1 false), !alias.scope !638, !noalias !639
  %_8.i.i = zext i1 %_0.sroa.0.0.i.i.i44 to i64
  %112 = add i64 %state.sroa.11.17.i, %_8.i.i
  %_9.i.i = getelementptr inbounds nuw i8, ptr %state.sroa.5.18.i, i64 56
  %_47.i = icmp ult ptr %_9.i.i, %loop_end.i
  br i1 %_47.i, label %bb18.i, label %bb21.i

bb23.i:                                           ; preds = %bb21.i
  %113 = getelementptr inbounds i8, ptr %state.sroa.19.1.lcssa.i, i64 -56
  %dst.i11.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %113, i64 %state.sroa.11.1.lcssa.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i11.i, ptr noundef nonnull align 8 dereferenceable(56) %state.sroa.5.1.lcssa.i, i64 56, i1 false), !alias.scope !638, !noalias !642
  %_9.i13.i = getelementptr inbounds nuw i8, ptr %state.sroa.5.1.lcssa.i, i64 56
  br label %bb3.i29

bb22.i:                                           ; preds = %bb21.i
  %114 = mul i64 %state.sroa.11.1.lcssa.i, 56
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %v.sroa.0.0.ph138, ptr nonnull align 8 %scratch.0, i64 %114, i1 false), !alias.scope !638
  %_63.i = sub i64 %v.sroa.16.0130, %state.sroa.11.1.lcssa.i
  %_9212.not.i = icmp eq i64 %v.sroa.16.0130, %state.sroa.11.1.lcssa.i
  br i1 %_9212.not.i, label %bb16, label %bb39.lr.ph.i

bb39.lr.ph.i:                                     ; preds = %bb22.i
  %115 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %state.sroa.11.1.lcssa.i
  br label %bb39.i

bb39.i:                                           ; preds = %bb39.i, %bb39.lr.ph.i
  %iter.sroa.0.013.i = phi i64 [ 0, %bb39.lr.ph.i ], [ %_96.i, %bb39.i ]
  %_96.i = add nuw i64 %iter.sroa.0.013.i, 1
  %116 = xor i64 %iter.sroa.0.013.i, -1
  %_69.i = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %_83.i, i64 %116
  %dst.i30 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %115, i64 %iter.sroa.0.013.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i30, ptr noundef nonnull align 8 dereferenceable(56) %_69.i, i64 56, i1 false), !alias.scope !638
  %exitcond.not.i31 = icmp eq i64 %_96.i, %_63.i
  br i1 %exitcond.not.i31, label %bb16, label %bb39.i

bb16:                                             ; preds = %bb39.i, %bb22.i
  %117 = icmp eq i64 %state.sroa.11.1.lcssa.i, 0
  br i1 %117, label %bb17, label %bb19

bb19:                                             ; preds = %bb16
  %_6.not.i = icmp ugt i64 %state.sroa.11.1.lcssa.i, %v.sroa.16.0130
  br i1 %_6.not.i, label %bb3.i45, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit, !prof !645

bb3.i45:                                          ; preds = %bb19
; call core::panicking::panic_fmt
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_63b5b202754b5efb770c3fd338a9a466) #32, !noalias !646
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_.exit: ; preds = %bb19
  %data.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %state.sroa.11.1.lcssa.i
; call core::slice::sort::stable::quicksort::quicksort::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  call void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_(ptr noalias noundef nonnull align 8 %data.i.i, i64 noundef %_63.i, ptr noalias noundef nonnull align 8 %scratch.0, i64 noundef %scratch.1, i32 noundef %70, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable_or_null(56) %pivot_copy, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less) #30
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %pivot_copy)
  %_8 = icmp ult i64 %state.sroa.11.1.lcssa.i, 33
  br i1 %_8, label %bb3, label %bb5

bb17:                                             ; preds = %bb7.i.i26, %_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_.exit, %bb16
  call void @llvm.experimental.noalias.scope.decl(metadata !650)
  call void @llvm.experimental.noalias.scope.decl(metadata !653)
  %_8.i46.not = icmp samesign ult i64 %scratch.1, %v.sroa.16.0130
  br i1 %_8.i46.not, label %bb28.i49, label %bb30.i50, !prof !523

bb30.i50:                                         ; preds = %bb17
  %_83.i52 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %v.sroa.16.0130
  %a1.i.i.i.i53 = getelementptr inbounds nuw i8, ptr %src, i64 8
  br label %bb3.i54

bb28.i49:                                         ; preds = %bb17
  call void @llvm.trap()
  unreachable

bb3.i54:                                          ; preds = %bb23.i66, %bb30.i50
  %state.sroa.11.0.i55 = phi i64 [ 0, %bb30.i50 ], [ %129, %bb23.i66 ]
  %state.sroa.5.0.i56 = phi ptr [ %v.sroa.0.0.ph138, %bb30.i50 ], [ %_9.i13.i68, %bb23.i66 ]
  %state.sroa.19.0.i57 = phi ptr [ %_83.i52, %bb30.i50 ], [ %128, %bb23.i66 ]
  %pivot_pos.sroa.0.0.i58 = phi i64 [ %index.sroa.0.0.i, %bb30.i50 ], [ %v.sroa.16.0130, %bb23.i66 ]
  %loop_end.i59 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %pivot_pos.sroa.0.0.i58
  %_476.i60 = icmp ult ptr %state.sroa.5.0.i56, %loop_end.i59
  br i1 %_476.i60, label %bb18.i80, label %bb21.i61

bb21.i61:                                         ; preds = %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i, %bb3.i54
  %state.sroa.11.1.lcssa.i62 = phi i64 [ %state.sroa.11.0.i55, %bb3.i54 ], [ %127, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ]
  %state.sroa.5.1.lcssa.i63 = phi ptr [ %state.sroa.5.0.i56, %bb3.i54 ], [ %_9.i.i99, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ]
  %state.sroa.19.1.lcssa.i64 = phi ptr [ %state.sroa.19.0.i57, %bb3.i54 ], [ %126, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ]
  %_55.i65 = icmp eq i64 %pivot_pos.sroa.0.0.i58, %v.sroa.16.0130
  br i1 %_55.i65, label %bb22.i69, label %bb23.i66

bb18.i80:                                         ; preds = %bb3.i54, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i
  %state.sroa.19.19.i81 = phi ptr [ %126, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ], [ %state.sroa.19.0.i57, %bb3.i54 ]
  %state.sroa.5.18.i82 = phi ptr [ %_9.i.i99, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ], [ %state.sroa.5.0.i56, %bb3.i54 ]
  %state.sroa.11.17.i83 = phi i64 [ %127, %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i ], [ %state.sroa.11.0.i55, %bb3.i54 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !655)
  call void @llvm.experimental.noalias.scope.decl(metadata !658)
  %_6.val.i.i = load ptr, ptr %is_less, align 8, !noalias !660, !nonnull !12, !align !132, !noundef !12
  call void @llvm.experimental.noalias.scope.decl(metadata !661)
  call void @llvm.experimental.noalias.scope.decl(metadata !664)
  %_7.val.i.i.i84 = load ptr, ptr %_6.val.i.i, align 8, !noalias !666
  call void @llvm.experimental.noalias.scope.decl(metadata !667)
  call void @llvm.experimental.noalias.scope.decl(metadata !670)
  %118 = load i64, ptr %src, align 8, !range !16, !alias.scope !672, !noalias !673, !noundef !12
  %.not.i.i.i.i85 = icmp ne i64 %118, -9223372036854775807
  %119 = load i64, ptr %state.sroa.5.18.i82, align 8, !range !16, !alias.scope !674, !noalias !675, !noundef !12
  %.not3.i.i.i.i86 = icmp eq i64 %119, -9223372036854775807
  %.not3.i.not.i.i.i87 = xor i1 %.not3.i.i.i.i86, true
  %brmerge.i.i.i88 = or i1 %.not.i.i.i.i85, %.not3.i.not.i.i.i87
  %.not3.i.mux.i.i.i89 = and i1 %.not.i.i.i.i85, %.not3.i.i.i.i86
  br i1 %brmerge.i.i.i88, label %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i, label %bb7.i.i.i.i90

bb7.i.i.i.i90:                                    ; preds = %bb18.i80
  %b2.i.i.i.i91 = getelementptr inbounds nuw i8, ptr %state.sroa.5.18.i82, i64 8
  %120 = icmp ne ptr %_7.val.i.i.i84, null
  call void @llvm.assume(i1 %120)
  %_11.i.i.i.i92 = load ptr, ptr %_7.val.i.i.i84, align 8, !noalias !676, !nonnull !12, !align !132, !noundef !12
  %_14.0.i.i.i.i93 = load ptr, ptr %_11.i.i.i.i92, align 8, !noalias !676, !nonnull !12, !noundef !12
  %121 = getelementptr inbounds nuw i8, ptr %_11.i.i.i.i92, i64 8
  %_14.1.i.i.i.i94 = load ptr, ptr %121, align 8, !noalias !676, !nonnull !12, !align !132, !noundef !12
  %122 = getelementptr inbounds nuw i8, ptr %_14.1.i.i.i.i94, i64 32
  %123 = load ptr, ptr %122, align 8, !invariant.load !12, !noalias !676, !nonnull !12
  %124 = call noundef i8 %123(ptr noundef nonnull align 1 %_14.0.i.i.i.i93, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %a1.i.i.i.i53, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %b2.i.i.i.i91) #31, !noalias !653
  %125 = icmp eq i8 %124, -1
  br label %_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i

_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_.exit.i: ; preds = %bb7.i.i.i.i90, %bb18.i80
  %_0.sroa.0.0.i.i.i.i95 = phi i1 [ %125, %bb7.i.i.i.i90 ], [ %.not3.i.mux.i.i.i89, %bb18.i80 ]
  %_0.i.i = xor i1 %_0.sroa.0.0.i.i.i.i95, true
  %126 = getelementptr inbounds i8, ptr %state.sroa.19.19.i81, i64 -56
  %dst_base.sroa.0.0.i.i96 = select i1 %_0.sroa.0.0.i.i.i.i95, ptr %126, ptr %scratch.0
  %dst.i.i97 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %dst_base.sroa.0.0.i.i96, i64 %state.sroa.11.17.i83
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i.i97, ptr noundef nonnull align 8 dereferenceable(56) %state.sroa.5.18.i82, i64 56, i1 false), !alias.scope !677, !noalias !678
  %_8.i.i98 = zext i1 %_0.i.i to i64
  %127 = add i64 %state.sroa.11.17.i83, %_8.i.i98
  %_9.i.i99 = getelementptr inbounds nuw i8, ptr %state.sroa.5.18.i82, i64 56
  %_47.i100 = icmp ult ptr %_9.i.i99, %loop_end.i59
  br i1 %_47.i100, label %bb18.i80, label %bb21.i61

bb23.i66:                                         ; preds = %bb21.i61
  %128 = getelementptr inbounds i8, ptr %state.sroa.19.1.lcssa.i64, i64 -56
  %dst.i11.i67 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %scratch.0, i64 %state.sroa.11.1.lcssa.i62
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i11.i67, ptr noundef nonnull align 8 dereferenceable(56) %state.sroa.5.1.lcssa.i63, i64 56, i1 false), !alias.scope !677, !noalias !681
  %129 = add i64 %state.sroa.11.1.lcssa.i62, 1
  %_9.i13.i68 = getelementptr inbounds nuw i8, ptr %state.sroa.5.1.lcssa.i63, i64 56
  br label %bb3.i54

bb22.i69:                                         ; preds = %bb21.i61
  %130 = mul i64 %state.sroa.11.1.lcssa.i62, 56
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %v.sroa.0.0.ph138, ptr nonnull align 8 %scratch.0, i64 %130, i1 false), !alias.scope !677
  %_63.i70 = sub i64 %v.sroa.16.0130, %state.sroa.11.1.lcssa.i62
  %_9212.not.i71 = icmp eq i64 %v.sroa.16.0130, %state.sroa.11.1.lcssa.i62
  br i1 %_9212.not.i71, label %bb3.thread, label %bb39.lr.ph.i72

bb39.lr.ph.i72:                                   ; preds = %bb22.i69
  %131 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %state.sroa.11.1.lcssa.i62
  br label %bb39.i73

bb39.i73:                                         ; preds = %bb39.i73, %bb39.lr.ph.i72
  %iter.sroa.0.013.i74 = phi i64 [ 0, %bb39.lr.ph.i72 ], [ %_96.i75, %bb39.i73 ]
  %_96.i75 = add nuw i64 %iter.sroa.0.013.i74, 1
  %132 = xor i64 %iter.sroa.0.013.i74, -1
  %_69.i76 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %_83.i52, i64 %132
  %dst.i77 = getelementptr %"core::result::Result<dent::DirEntry, error::Error>", ptr %131, i64 %iter.sroa.0.013.i74
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i77, ptr noundef nonnull align 8 dereferenceable(56) %_69.i76, i64 56, i1 false), !alias.scope !677
  %exitcond.not.i78 = icmp eq i64 %_96.i75, %_63.i70
  br i1 %exitcond.not.i78, label %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_.exit, label %bb39.i73

_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_.exit: ; preds = %bb39.i73
  %_47 = icmp ugt i64 %state.sroa.11.1.lcssa.i62, %v.sroa.16.0130
  br i1 %_47, label %bb27, label %bb28, !prof !645

bb3.thread:                                       ; preds = %bb22.i69
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %pivot_copy)
  br label %bb22

bb28:                                             ; preds = %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_.exit
  %_54 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %v.sroa.0.0.ph138, i64 %state.sroa.11.1.lcssa.i62
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %pivot_copy)
  %_8128 = icmp ult i64 %_63.i70, 33
  br i1 %_8128, label %bb3, label %bb5.lr.ph

bb27:                                             ; preds = %_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_.exit
; call core::slice::index::slice_index_fail
  call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %state.sroa.11.1.lcssa.i62, i64 noundef %v.sroa.16.0130, i64 noundef %v.sroa.16.0130, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_36ce49ab0499b14b6f7dec8f3551378e) #32
  unreachable
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 1, 0) %additional) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !684)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !48, !alias.scope !684, !noundef !12
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !684
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !684
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsarYwbBXrH4d_7walkdir(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef 56)
  %_35.i = load i64, ptr %self3.i, align 8, !range !687, !noalias !684, !noundef !12
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !11, !noalias !684, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !684
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !684
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #27
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !684, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !684
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !684
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !684
  ret void
}

; <walkdir::WalkDir>::sort_by_file_name::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef range(i8 -1, 2) i8 @_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_(ptr noalias nonnull readnone align 1 captures(none) %_1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %a, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %b) unnamed_addr #4 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !688)
  %0 = getelementptr inbounds nuw i8, ptr %a, i64 8
  %_11.i = load ptr, ptr %0, align 8, !alias.scope !688, !nonnull !12, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %a, i64 16
  %_10.i = load i64, ptr %1, align 8, !alias.scope !688, !noundef !12
; call <std::path::Path>::file_name
  %2 = tail call { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_11.i, i64 noundef %_10.i), !noalias !688
  %3 = extractvalue { ptr, i64 } %2, 0
  %.not.i = icmp eq ptr %3, null
  %_4.0 = select i1 %.not.i, ptr %_11.i, ptr %3
  %4 = extractvalue { ptr, i64 } %2, 1
  %_4.1 = select i1 %.not.i, i64 %_10.i, i64 %4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !691)
  %5 = getelementptr inbounds nuw i8, ptr %b, i64 8
  %_11.i4 = load ptr, ptr %5, align 8, !alias.scope !691, !nonnull !12, !noundef !12
  %6 = getelementptr inbounds nuw i8, ptr %b, i64 16
  %_10.i5 = load i64, ptr %6, align 8, !alias.scope !691, !noundef !12
; call <std::path::Path>::file_name
  %7 = tail call { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_11.i4, i64 noundef %_10.i5), !noalias !691
  %8 = extractvalue { ptr, i64 } %7, 0
  %.not.i6 = icmp eq ptr %8, null
  %_5.0 = select i1 %.not.i6, ptr %_11.i4, ptr %8
  %9 = extractvalue { ptr, i64 } %7, 1
  %_5.1 = select i1 %.not.i6, i64 %_10.i5, i64 %9
  %spec.store.select = tail call i64 @llvm.umin.i64(i64 %_4.1, i64 %_5.1)
  %10 = tail call i32 @memcmp(ptr nonnull %_4.0, ptr nonnull %_5.0, i64 %spec.store.select)
  %11 = sext i32 %10 to i64
  %12 = icmp eq i32 %10, 0
  %_8 = sub i64 %_4.1, %_5.1
  %spec.select = select i1 %12, i64 %_8, i64 %11
  %_0 = tail call i8 @llvm.scmp.i8.i64(i64 %spec.select, i64 0)
  ret i8 %_0
}

; <<walkdir::WalkDir>::sort_by_file_name::{closure#0} as core::ops::function::FnOnce<(&walkdir::dent::DirEntry, &walkdir::dent::DirEntry)>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef range(i8 -1, 2) i8 @_RNSNvYNCNvMs_CsarYwbBXrH4d_7walkdirNtBb_7WalkDir17sort_by_file_name0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtNtBb_4dent8DirEntryB1P_EE9call_once6vtableBb_(ptr readnone captures(none) %_1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %1) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %.val = load ptr, ptr %2, align 8, !alias.scope !694, !noalias !699, !nonnull !12, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %.val1 = load i64, ptr %3, align 8, !alias.scope !694, !noalias !699, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %.val2 = load ptr, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %1, i64 16
  %.val3 = load i64, ptr %5, align 8
; call <std::path::Path>::file_name
  %6 = tail call { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %.val, i64 noundef %.val1), !noalias !701
  %7 = extractvalue { ptr, i64 } %6, 0
  %.not.i.i.i = icmp eq ptr %7, null
  %_4.0.i.i = select i1 %.not.i.i.i, ptr %.val, ptr %7
  %8 = extractvalue { ptr, i64 } %6, 1
  %_4.1.i.i = select i1 %.not.i.i.i, i64 %.val1, i64 %8
  %9 = icmp ne ptr %.val2, null
  tail call void @llvm.assume(i1 %9)
; call <std::path::Path>::file_name
  %10 = tail call { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %.val2, i64 noundef %.val3), !noalias !707
  %11 = extractvalue { ptr, i64 } %10, 0
  %.not.i6.i.i = icmp eq ptr %11, null
  %_5.0.i.i = select i1 %.not.i6.i.i, ptr %.val2, ptr %11
  %12 = extractvalue { ptr, i64 } %10, 1
  %_5.1.i.i = select i1 %.not.i6.i.i, i64 %.val3, i64 %12
  %spec.store.select.i.i = tail call i64 @llvm.umin.i64(i64 %_4.1.i.i, i64 %_5.1.i.i)
  %13 = tail call i32 @memcmp(ptr nonnull %_4.0.i.i, ptr nonnull %_5.0.i.i, i64 %spec.store.select.i.i), !noalias !710
  %14 = sext i32 %13 to i64
  %15 = icmp eq i32 %13, 0
  %_8.i.i = sub i64 %_4.1.i.i, %_5.1.i.i
  %spec.select.i.i = select i1 %15, i64 %_8.i.i, i64 %14
  %_0.i.i = tail call noundef range(i8 -1, 2) i8 @llvm.scmp.i8.i64(i64 %spec.select.i.i, i64 0)
  ret i8 %_0.i.i
}

; <walkdir::dent::DirEntry>::from_entry
; Function Attrs: uwtable
define void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry10from_entry(ptr dead_on_unwind noalias noundef writable writeonly sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_0, i64 noundef %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(1056) %1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_4.i = alloca [24 x i8], align 8
  %err.i = alloca [8 x i8], align 8
  %_5 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5)
; call <std::fs::DirEntry>::file_type
  call void @_RNvMsC_NtCs5sEH5CPMdak_3std2fsNtB5_8DirEntry9file_type(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_5, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %1)
  %2 = load i16, ptr %_5, align 8, !range !711, !noundef !12
  %3 = trunc nuw i16 %2 to i1
  br i1 %3, label %bb5, label %bb6

bb5:                                              ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_16 = load ptr, ptr %4, align 8, !nonnull !12, !noundef !12
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i)
  store ptr %_16, ptr %err.i, align 8, !noalias !712
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i), !noalias !712
; invoke <std::fs::DirEntry>::path
  invoke void @_RNvMsC_NtCs5sEH5CPMdak_3std2fsNtB5_8DirEntry4path(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_4.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %1)
          to label %_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry10from_entry0B6_.exit unwind label %cleanup.i, !noalias !712

cleanup.i:                                        ; preds = %bb5
  %5 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i) #28
          to label %bb2.i unwind label %terminate.i, !noalias !712

terminate.i:                                      ; preds = %cleanup.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !712
  unreachable

bb2.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %5

_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry10from_entry0B6_.exit: ; preds = %bb5
  %_17.sroa.4.8.copyload = load i16, ptr %_4.i, align 8
  %_17.sroa.6.8._4.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i, i64 2
  %_22.sroa.3.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 10
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 2 dereferenceable(22) %_22.sroa.3.0._0.sroa_idx, ptr noundef nonnull align 2 dereferenceable(22) %_17.sroa.6.8._4.i.sroa_idx, i64 22, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i), !noalias !712
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  store i64 -9223372036854775808, ptr %_0, align 8
  %_22.sroa.2.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i16 %_17.sroa.4.8.copyload, ptr %_22.sroa.2.0._0.sroa_idx, align 8
  %_22.sroa.3.sroa.2.0._22.sroa.3.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_16, ptr %_22.sroa.3.sroa.2.0._22.sroa.3.0._0.sroa_idx.sroa_idx, align 8
  %_22.sroa.3.sroa.4.0._22.sroa.3.0._0.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %0, ptr %_22.sroa.3.sroa.4.0._22.sroa.3.0._0.sroa_idx.sroa_idx, align 8
  br label %bb4

bb6:                                              ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %_5, i64 2
  %_15 = load i16, ptr %7, align 2, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 8
; call <std::fs::DirEntry>::path
  tail call void @_RNvMsC_NtCs5sEH5CPMdak_3std2fsNtB5_8DirEntry4path(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %8, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %1)
  %9 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %_13 = load i64, ptr %9, align 8, !noundef !12
  %_11.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 %0, ptr %_11.sroa.4.0..sroa_idx, align 8
  %_11.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %_13, ptr %_11.sroa.5.0..sroa_idx, align 8
  %_11.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i16 %_15, ptr %_11.sroa.6.0..sroa_idx, align 8
  %_11.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 50
  store i8 0, ptr %_11.sroa.7.0..sroa_idx, align 2
  store i64 -9223372036854775807, ptr %_0, align 8
  br label %bb4

bb4:                                              ; preds = %_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry10from_entry0B6_.exit, %bb6
  ret void
}

; <walkdir::dent::DirEntry>::metadata
; Function Attrs: uwtable
define void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry8metadata(ptr dead_on_unwind noalias noundef writable writeonly sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %self) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_6.i.i = alloca [24 x i8], align 8
  %err.i.i = alloca [8 x i8], align 8
  %self.i1.i = alloca [152 x i8], align 8
  %self.i.i = alloca [152 x i8], align 8
  %_2.sroa.11.i = alloca [136 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !715)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !718)
  call void @llvm.lifetime.start.p0(i64 136, ptr nonnull %_2.sroa.11.i)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 42
  %1 = load i8, ptr %0, align 2, !range !720, !alias.scope !718, !noalias !715, !noundef !12
  %_3.i = trunc nuw i8 %1 to i1
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  br i1 %_3.i, label %bb1.i, label %bb3.i

bb3.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !721)
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i.i), !noalias !724
  %_2.val.i.i.i = load ptr, ptr %2, align 8, !alias.scope !726, !noalias !727, !nonnull !12, !noundef !12
  %_2.val1.i.i.i = load i64, ptr %3, align 8, !alias.scope !726, !noalias !727, !noundef !12
; call std::sys::fs::symlink_metadata
  call void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs16symlink_metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i.i, i64 noundef %_2.val1.i.i.i), !noalias !724
  %_5.i.i = load i64, ptr %self.i.i, align 8, !range !687, !noalias !724, !noundef !12
  %4 = trunc nuw i64 %_5.i.i to i1
  %5 = getelementptr inbounds nuw i8, ptr %self.i.i, i64 8
  %e.i.i = load ptr, ptr %5, align 8, !noalias !728
  br i1 %4, label %bb5.thread.i, label %bb5.i

bb5.thread.i:                                     ; preds = %bb3.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i.i), !noalias !724
  br label %bb8.i

bb1.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !729)
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i1.i), !noalias !732
  %_2.val.i.i2.i = load ptr, ptr %2, align 8, !alias.scope !734, !noalias !735, !nonnull !12, !noundef !12
  %_2.val1.i.i3.i = load i64, ptr %3, align 8, !alias.scope !734, !noalias !735, !noundef !12
; call std::sys::fs::metadata
  call void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i1.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i2.i, i64 noundef %_2.val1.i.i3.i), !noalias !732
  %_5.i4.i = load i64, ptr %self.i1.i, align 8, !range !687, !noalias !732, !noundef !12
  %6 = trunc nuw i64 %_5.i4.i to i1
  %7 = getelementptr inbounds nuw i8, ptr %self.i1.i, i64 8
  %e.i8.i = load ptr, ptr %7, align 8, !noalias !736
  br i1 %6, label %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i, label %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i

_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i: ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i1.i), !noalias !732
  br label %bb8.i

_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i: ; preds = %bb1.i
  %_2.sroa.11.8..sroa_idx11.i = getelementptr inbounds nuw i8, ptr %self.i1.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(136) %_2.sroa.11.i, ptr noundef nonnull align 8 dereferenceable(136) %_2.sroa.11.8..sroa_idx11.i, i64 136, i1 false), !noalias !737
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i1.i), !noalias !732
  br label %bb9.i

bb5.i:                                            ; preds = %bb3.i
  %_2.sroa.11.8..sroa_idx.i = getelementptr inbounds nuw i8, ptr %self.i.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(136) %_2.sroa.11.i, ptr noundef nonnull align 8 dereferenceable(136) %_2.sroa.11.8..sroa_idx.i, i64 136, i1 false), !noalias !737
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i.i), !noalias !724
  br label %bb9.i

bb8.i:                                            ; preds = %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i, %bb5.thread.i
  %_2.sroa.5.017.i = phi ptr [ %e.i.i, %bb5.thread.i ], [ %e.i8.i, %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i ]
  %_15.i16.i = phi ptr [ %_2.val.i.i.i, %bb5.thread.i ], [ %_2.val.i.i2.i, %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i ]
  %_14.i15.i = phi i64 [ %_2.val1.i.i.i, %bb5.thread.i ], [ %_2.val1.i.i3.i, %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !738)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i.i), !noalias !737
  store ptr %_2.sroa.5.017.i, ptr %err.i.i, align 8, !noalias !741
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_3.i.i = load i64, ptr %8, align 8, !alias.scope !743, !noalias !744, !noundef !12
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i.i), !noalias !741
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_15.i16.i, i64 noundef %_14.i15.i)
          to label %_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry.exit.i unwind label %cleanup.i.i, !noalias !741

cleanup.i.i:                                      ; preds = %bb8.i
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i.i) #28
          to label %bb3.i.i unwind label %terminate.i.i, !noalias !741

terminate.i.i:                                    ; preds = %cleanup.i.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !741
  unreachable

bb3.i.i:                                          ; preds = %cleanup.i.i
  resume { ptr, i32 } %9

_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry.exit.i: ; preds = %bb8.i
  %_9.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_9.sroa.4.0..sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(24) %_6.i.i, i64 24, i1 false), !noalias !718
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i.i), !noalias !741
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i.i), !noalias !737
  %11 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 -9223372036854775808, ptr %11, align 8, !alias.scope !715, !noalias !718
  %_9.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store ptr %_2.sroa.5.017.i, ptr %_9.sroa.5.0..sroa_idx.i, align 8, !alias.scope !715, !noalias !718
  %_9.sroa.612.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 56
  store i64 %_3.i.i, ptr %_9.sroa.612.0..sroa_idx.i, align 8, !alias.scope !715, !noalias !718
  br label %_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal.exit

bb9.i:                                            ; preds = %bb5.i, %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i
  %_2.sroa.5.018.i = phi ptr [ %e.i8.i, %_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i ], [ %e.i.i, %bb5.i ]
  %12 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_2.sroa.5.018.i, ptr %12, align 8, !alias.scope !715, !noalias !718
  %_7.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(136) %_7.sroa.4.0..sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(136) %_2.sroa.11.i, i64 136, i1 false), !noalias !718
  br label %_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal.exit

_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal.exit: ; preds = %_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry.exit.i, %bb9.i
  %storemerge.i = phi i64 [ 0, %bb9.i ], [ 1, %_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry.exit.i ]
  store i64 %storemerge.i, ptr %_0, align 8, !alias.scope !715, !noalias !718
  call void @llvm.lifetime.end.p0(i64 136, ptr nonnull %_2.sroa.11.i)
  ret void
}

; <walkdir::dent::DirEntry>::file_name
; Function Attrs: uwtable
define { ptr, i64 } @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name(ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %self) unnamed_addr #2 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_11 = load ptr, ptr %0, align 8, !nonnull !12, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_10 = load i64, ptr %1, align 8, !noundef !12
; call <std::path::Path>::file_name
  %2 = tail call { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_11, i64 noundef %_10)
  %3 = extractvalue { ptr, i64 } %2, 0
  %.not = icmp eq ptr %3, null
  %4 = insertvalue { ptr, i64 } poison, ptr %_11, 0
  %5 = insertvalue { ptr, i64 } %4, i64 %_10, 1
  %.merged = select i1 %.not, { ptr, i64 } %5, { ptr, i64 } %2
  ret { ptr, i64 } %.merged
}

; <walkdir::dent::DirEntry>::from_path
; Function Attrs: uwtable
define internal fastcc void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9from_path(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(56) %_0, i64 noundef %0, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %pb, i1 noundef zeroext %follow) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %err.i130 = alloca [8 x i8], align 8
  %err.i = alloca [8 x i8], align 8
  %self.i117 = alloca [152 x i8], align 8
  %self.i = alloca [152 x i8], align 8
  %1 = getelementptr inbounds nuw i8, ptr %pb, i64 8
  %2 = getelementptr inbounds nuw i8, ptr %pb, i64 16
  br i1 %follow, label %bb1, label %bb4

bb4:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !745)
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i), !noalias !748
  %_2.val.i.i = load ptr, ptr %1, align 8, !alias.scope !745, !noalias !750, !nonnull !12, !noundef !12
  %_2.val1.i.i = load i64, ptr %2, align 8, !alias.scope !745, !noalias !750, !noundef !12
; invoke std::sys::fs::symlink_metadata
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs16symlink_metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i, i64 noundef %_2.val1.i.i)
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb4
  %_5.i = load i64, ptr %self.i, align 8, !range !687, !noalias !748, !noundef !12
  %3 = trunc nuw i64 %_5.i to i1
  %4 = getelementptr inbounds nuw i8, ptr %self.i, i64 8
  %e.i = load ptr, ptr %4, align 8, !noalias !745
  br i1 %3, label %bb14, label %bb15

bb1:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !751)
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i117), !noalias !754
  %_2.val.i.i118 = load ptr, ptr %1, align 8, !alias.scope !751, !noalias !756, !nonnull !12, !noundef !12
  %_2.val1.i.i119 = load i64, ptr %2, align 8, !alias.scope !751, !noalias !756, !noundef !12
; invoke std::sys::fs::metadata
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i117, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.val.i.i118, i64 noundef %_2.val1.i.i119)
          to label %.noexc125 unwind label %cleanup

.noexc125:                                        ; preds = %bb1
  %_5.i120 = load i64, ptr %self.i117, align 8, !range !687, !noalias !754, !noundef !12
  %5 = trunc nuw i64 %_5.i120 to i1
  %6 = getelementptr inbounds nuw i8, ptr %self.i117, i64 8
  %e.i124 = load ptr, ptr %6, align 8, !noalias !751
  br i1 %5, label %bb11, label %bb12

cleanup:                                          ; preds = %bb1, %bb4
  %_1.val1.i155 = phi ptr [ %_2.val.i.i118, %bb1 ], [ %_2.val.i.i, %bb4 ]
  %7 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.i139, %cleanup.i, %cleanup
  %_1.val1.i = phi ptr [ %_1.val1.i155, %cleanup ], [ %_2.val.i.i, %cleanup.i ], [ %_2.val.i.i118, %cleanup.i139 ]
  %eh.lpad-body = phi { ptr, i32 } [ %7, %cleanup ], [ %11, %cleanup.i ], [ %19, %cleanup.i139 ]
  call void @llvm.experimental.noalias.scope.decl(metadata !757)
  %_1.val.i = load i64, ptr %pb, align 8, !alias.scope !757
  %8 = icmp eq i64 %_1.val.i, 0
  br i1 %8, label %bb9, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %cleanup.body
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !757
  br label %bb9

bb14:                                             ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i), !noalias !748
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i)
  store ptr %e.i, ptr %err.i, align 8, !noalias !760
  %_23.i.i.i.i.i.i = icmp eq i64 %_2.val1.i.i, 0
  br i1 %_23.i.i.i.i.i.i, label %bb16, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i: ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !763
; call __rustc::__rust_alloc
  %9 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %_2.val1.i.i, i64 noundef range(i64 1, 9) 1) #26, !noalias !763
  %10 = icmp eq ptr %9, null
  br i1 %10, label %bb3.i.i.i.i, label %bb16

bb3.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %_2.val1.i.i) #27
          to label %.noexc.i unwind label %cleanup.i, !noalias !760

.noexc.i:                                         ; preds = %bb3.i.i.i.i
  unreachable

cleanup.i:                                        ; preds = %bb3.i.i.i.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i) #28
          to label %cleanup.body unwind label %terminate.i, !noalias !760

terminate.i:                                      ; preds = %cleanup.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !760
  unreachable

bb15:                                             ; preds = %.noexc
  %_14.sroa.9.8..sroa_idx = getelementptr inbounds nuw i8, ptr %self.i, i64 16
  %_14.sroa.9.8.copyload = load i64, ptr %_14.sroa.9.8..sroa_idx, align 8, !noalias !745
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i), !noalias !748
  br label %bb6

bb6:                                              ; preds = %bb12, %bb15
  %md2.sroa.0.0.in.in.in = phi ptr [ %e.i124, %bb12 ], [ %e.i, %bb15 ]
  %md4.sroa.0.0 = phi i64 [ %_6.sroa.9.8.copyload, %bb12 ], [ %_14.sroa.9.8.copyload, %bb15 ]
  %md2.sroa.0.0.in.in = ptrtoint ptr %md2.sroa.0.0.in.in.in to i64
  %md2.sroa.0.0.in = lshr i64 %md2.sroa.0.0.in.in, 32
  %md2.sroa.0.0 = trunc i64 %md2.sroa.0.0.in to i16
  %13 = zext i1 %follow to i8
  %14 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %14, ptr noundef nonnull align 8 dereferenceable(24) %pb, i64 24, i1 false)
  %_20.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 %0, ptr %_20.sroa.4.0..sroa_idx, align 8
  %_20.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %md4.sroa.0.0, ptr %_20.sroa.5.0..sroa_idx, align 8
  %_20.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i16 %md2.sroa.0.0, ptr %_20.sroa.6.0..sroa_idx, align 8
  %_20.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 50
  store i8 %13, ptr %_20.sroa.7.0..sroa_idx, align 2
  store i64 -9223372036854775807, ptr %_0, align 8
  br label %bb8

bb16:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i, %bb14
  %_4.sroa.10.0.i.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb14 ], [ %9, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i, ptr nonnull readonly align 1 %_2.val.i.i, i64 range(i64 0, -9223372036854775808) %_2.val1.i.i, i1 false), !noalias !771
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i)
  br label %bb7

bb7:                                              ; preds = %bb13, %bb16
  %_4.sroa.10.0.i.i.i.i133.sink = phi ptr [ %_4.sroa.10.0.i.i.i.i133, %bb13 ], [ %_4.sroa.10.0.i.i.i.i, %bb16 ]
  %_2.val1.i.i119.sink158 = phi i64 [ %_2.val1.i.i119, %bb13 ], [ %_2.val1.i.i, %bb16 ]
  %.sink.in = phi ptr [ %e.i124, %bb13 ], [ %e.i, %bb16 ]
  %_1.val1.i128 = phi ptr [ %_2.val.i.i118, %bb13 ], [ %_2.val.i.i, %bb16 ]
  %.sink = ptrtoint ptr %.sink.in to i64
  %15 = ptrtoint ptr %_4.sroa.10.0.i.i.i.i133.sink to i64
  %_26.sroa.8.16.extract.trunc = trunc i64 %15 to i32
  %_26.sroa.8.20.extract.shift = lshr i64 %15, 32
  %_26.sroa.8.20.extract.trunc = trunc nuw i64 %_26.sroa.8.20.extract.shift to i32
  %_26.sroa.11.24.extract.trunc = trunc i64 %_2.val1.i.i119.sink158 to i32
  %_26.sroa.11.28.extract.shift = lshr i64 %_2.val1.i.i119.sink158, 32
  %_26.sroa.11.28.extract.trunc = trunc nuw i64 %_26.sroa.11.28.extract.shift to i32
  store i32 0, ptr %_0, align 8
  %_31.sroa.2.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 4
  store i16 0, ptr %_31.sroa.2.0._0.sroa_idx, align 4
  %_31.sroa.3.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 6
  store i16 -32768, ptr %_31.sroa.3.0._0.sroa_idx, align 2
  %_31.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_2.val1.i.i119.sink158, ptr %_31.sroa.4.0._0.sroa_idx, align 8
  %_31.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 %_26.sroa.8.16.extract.trunc, ptr %_31.sroa.5.0._0.sroa_idx, align 8
  %_31.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i32 %_26.sroa.8.20.extract.trunc, ptr %_31.sroa.6.0._0.sroa_idx, align 4
  %_31.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i32 %_26.sroa.11.24.extract.trunc, ptr %_31.sroa.7.0._0.sroa_idx, align 8
  %_31.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 28
  store i32 %_26.sroa.11.28.extract.trunc, ptr %_31.sroa.8.0._0.sroa_idx, align 4
  %_31.sroa.9.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 %.sink, ptr %_31.sroa.9.0._0.sroa_idx, align 8
  %_31.sroa.11.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %0, ptr %_31.sroa.11.0._0.sroa_idx, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !772)
  %_1.val.i126 = load i64, ptr %pb, align 8, !alias.scope !772
  %16 = icmp eq i64 %_1.val.i126, 0
  br i1 %16, label %bb8, label %bb2.i.i.i4.i.i.i.i127

bb2.i.i.i4.i.i.i.i127:                            ; preds = %bb7
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i128, i64 noundef %_1.val.i126, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !772
  br label %bb8

bb11:                                             ; preds = %.noexc125
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i117), !noalias !754
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i130)
  store ptr %e.i124, ptr %err.i130, align 8, !noalias !775
  %_23.i.i.i.i.i.i131 = icmp eq i64 %_2.val1.i.i119, 0
  br i1 %_23.i.i.i.i.i.i131, label %bb13, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i132

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i132: ; preds = %bb11
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !778
; call __rustc::__rust_alloc
  %17 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %_2.val1.i.i119, i64 noundef range(i64 1, 9) 1) #26, !noalias !778
  %18 = icmp eq ptr %17, null
  br i1 %18, label %bb3.i.i.i.i138, label %bb13

bb3.i.i.i.i138:                                   ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i132
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %_2.val1.i.i119) #27
          to label %.noexc.i142 unwind label %cleanup.i139, !noalias !775

.noexc.i142:                                      ; preds = %bb3.i.i.i.i138
  unreachable

cleanup.i139:                                     ; preds = %bb3.i.i.i.i138
  %19 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i130) #28
          to label %cleanup.body unwind label %terminate.i140, !noalias !775

terminate.i140:                                   ; preds = %cleanup.i139
  %20 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !775
  unreachable

bb12:                                             ; preds = %.noexc125
  %_6.sroa.9.8..sroa_idx = getelementptr inbounds nuw i8, ptr %self.i117, i64 16
  %_6.sroa.9.8.copyload = load i64, ptr %_6.sroa.9.8..sroa_idx, align 8, !noalias !751
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i117), !noalias !754
  br label %bb6

bb8:                                              ; preds = %bb2.i.i.i4.i.i.i.i127, %bb7, %bb6
  ret void

bb13:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i132, %bb11
  %_4.sroa.10.0.i.i.i.i133 = phi ptr [ inttoptr (i64 1 to ptr), %bb11 ], [ %17, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i132 ]
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i133, ptr nonnull readonly align 1 %_2.val.i.i118, i64 range(i64 0, -9223372036854775808) %_2.val1.i.i119, i1 false), !noalias !786
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i130)
  br label %bb7

bb9:                                              ; preds = %bb2.i.i.i4.i.i.i.i, %cleanup.body
  resume { ptr, i32 } %eh.lpad-body
}

; <walkdir::error::Error>::into_io_error
; Function Attrs: nounwind uwtable
define noundef ptr @_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error13into_io_error(ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(56) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %err = load ptr, ptr %1, align 8, !nonnull !12
  br i1 %.not, label %bb5, label %bb8.i

bb8.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !787)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !790)
  %2 = icmp eq i64 %0, 0
  br i1 %2, label %bb7.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb8.i
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_1.val1.i.i = load ptr, ptr %3, align 8, !alias.scope !793, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !793
  br label %bb7.i

bb7.i:                                            ; preds = %bb2.i.i.i4.i.i.i.i.i, %bb8.i
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !794)
  %_1.val.i7.i = load i64, ptr %4, align 8, !alias.scope !797
  %5 = icmp eq i64 %_1.val.i7.i, 0
  br i1 %5, label %bb4, label %bb2.i.i.i4.i.i.i.i8.i

bb2.i.i.i4.i.i.i.i8.i:                            ; preds = %bb7.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %err, i64 noundef %_1.val.i7.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !797
  br label %bb4

bb5:                                              ; preds = %start
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !798)
  %7 = load i64, ptr %6, align 8, !range !11, !alias.scope !798, !noundef !12
  %8 = icmp eq i64 %7, -9223372036854775808
  br i1 %8, label %bb4, label %bb2.i

bb2.i:                                            ; preds = %bb5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !801)
  %9 = icmp eq i64 %7, 0
  br i1 %9, label %bb4, label %bb2.i.i.i4.i.i.i.i.i2

bb2.i.i.i4.i.i.i.i.i2:                            ; preds = %bb2.i
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_1.val1.i.i3 = load ptr, ptr %10, align 8, !alias.scope !804, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i3, i64 noundef %7, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !804
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i.i.i.i.i2, %bb2.i, %bb5, %bb2.i.i.i4.i.i.i.i8.i, %bb7.i
  %_0.sroa.0.0 = phi ptr [ %err, %bb2.i.i.i4.i.i.i.i.i2 ], [ %err, %bb2.i ], [ %err, %bb5 ], [ null, %bb2.i.i.i4.i.i.i.i8.i ], [ null, %bb7.i ]
  ret ptr %_0.sroa.0.0
}

; <walkdir::IntoIter>::handle_entry
; Function Attrs: uwtable
define internal fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter12handle_entry(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(56) %_0, ptr noalias noundef nonnull align 8 dereferenceable(176) %self, ptr dead_on_return noalias noundef nonnull align 8 captures(none) dereferenceable(48) %dent) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_6.i6.i = alloca [24 x i8], align 8
  %err.i.i = alloca [8 x i8], align 8
  %self.i.i.i = alloca [152 x i8], align 8
  %_4.i73 = alloca [24 x i8], align 8
  %err.i = alloca [8 x i8], align 8
  %self.i = alloca [152 x i8], align 8
  %_6.i.i.i = alloca [24 x i8], align 8
  %_5.i30.i.i = alloca [24 x i8], align 8
  %_5.i.i.i.i = alloca [12 x i8], align 4
  %_3.i.i.i.i = alloca [16 x i8], align 8
  %_10.i26.i.i = alloca [24 x i8], align 8
  %_4.i.i.i = alloca [24 x i8], align 8
  %_5.i.i.i = alloca [12 x i8], align 4
  %_3.i.i.i = alloca [16 x i8], align 8
  %_26.i.i = alloca [24 x i8], align 8
  %hchild.i.i = alloca [24 x i8], align 8
  %_6.i = alloca [24 x i8], align 8
  %_4.i = alloca [56 x i8], align 8
  %_40 = alloca [56 x i8], align 8
  %_26 = alloca [56 x i8], align 8
  %_21 = alloca [56 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %1 = load i8, ptr %0, align 8, !range !720, !noundef !12
  %_3 = trunc nuw i8 %1 to i1
  %2 = getelementptr inbounds nuw i8, ptr %dent, i64 40
  %_76 = load i16, ptr %2, align 8
  %_52 = and i16 %_76, -4096
  %3 = icmp eq i16 %_52, -24576
  %or.cond209 = select i1 %_3, i1 %3, i1 false
  br i1 %or.cond209, label %bb2, label %bb8

bb8:                                              ; preds = %start, %bb6
  %_77 = phi i16 [ %56, %bb6 ], [ %_76, %start ]
  %_53 = and i16 %_77, -4096
  %4 = icmp eq i16 %_53, 16384
  br i1 %4, label %bb12, label %bb27

bb2:                                              ; preds = %start
  %_6.sroa.0.0.copyload = load i64, ptr %dent, align 8
  %_6.sroa.7.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %_6.sroa.7.0.copyload = load ptr, ptr %_6.sroa.7.0.dent.sroa_idx, align 8, !nonnull !12, !noundef !12
  %_6.sroa.9.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 16
  %_6.sroa.9.0.copyload = load i64, ptr %_6.sroa.9.0.dent.sroa_idx, align 8
  %_6.sroa.11.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !805)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_4.i), !noalias !808
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %_5.i = load i64, ptr %5, align 8, !alias.scope !805, !noalias !811, !noundef !12
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i), !noalias !808
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_6.sroa.7.0.copyload, i64 noundef %_6.sroa.9.0.copyload)
          to label %bb1.i unwind label %cleanup.i, !noalias !808

bb12.i:                                           ; preds = %cleanup.i.i35.i.i, %cleanup2.body.i.i, %cleanup.i.i.i.i, %common.resume.sink.split.i.i, %cleanup.i
  %_6.sroa.0.1 = phi i64 [ %_3.sroa.6.i.sroa.0.0.copyload130, %cleanup.i.i.i.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %common.resume.sink.split.i.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %cleanup.i.i35.i.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %cleanup2.body.i.i ], [ %_6.sroa.0.0, %cleanup.i ]
  %dent.val6.i = phi ptr [ %12, %cleanup.i.i.i.i ], [ %12, %common.resume.sink.split.i.i ], [ %12, %cleanup.i.i35.i.i ], [ %12, %cleanup2.body.i.i ], [ %dent.val660.i, %cleanup.i ]
  %.pn.i = phi { ptr, i32 } [ %35, %cleanup.i.i.i.i ], [ %common.resume.op.ph.i.i, %common.resume.sink.split.i.i ], [ %47, %cleanup.i.i35.i.i ], [ %eh.lpad-body.i.i, %cleanup2.body.i.i ], [ %7, %cleanup.i ]
  %6 = icmp eq i64 %_6.sroa.0.1, 0
  br i1 %6, label %bb51, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %bb12.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %dent.val6.i, i64 noundef %_6.sroa.0.1, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !812
  br label %bb51

cleanup.i:                                        ; preds = %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i, %bb6.i, %bb1.i, %bb2
  %_6.sroa.0.0 = phi i64 [ %_3.sroa.6.i.sroa.0.0.copyload130, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %bb6.i ], [ %_6.sroa.0.0.copyload, %bb1.i ], [ %_6.sroa.0.0.copyload, %bb2 ]
  %dent.val660.i = phi ptr [ %12, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i ], [ %12, %bb6.i ], [ %_6.sroa.7.0.copyload, %bb1.i ], [ %_6.sroa.7.0.copyload, %bb2 ]
  %7 = landingpad { ptr, i32 }
          cleanup
  br label %bb12.i

bb1.i:                                            ; preds = %bb2
; invoke <walkdir::dent::DirEntry>::from_path
  invoke fastcc void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9from_path(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_4.i, i64 noundef %_5.i, ptr noalias noundef align 8 captures(address) dereferenceable(24) %_6.i, i1 noundef zeroext true)
          to label %bb2.i unwind label %cleanup.i, !noalias !808

bb2.i:                                            ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i), !noalias !808
  %8 = load i64, ptr %_4.i, align 8, !range !16, !noalias !808, !noundef !12
  %.not.i = icmp eq i64 %8, -9223372036854775807
  %9 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.sroa.6.i.sroa.0.0.copyload130 = load i64, ptr %9, align 8, !noalias !808
  %_3.sroa.6.i.sroa.7.0..sroa_idx131 = getelementptr inbounds nuw i8, ptr %_4.i, i64 16
  %_3.sroa.6.i.sroa.7.0.copyload132 = load i64, ptr %_3.sroa.6.i.sroa.7.0..sroa_idx131, align 8, !noalias !808
  %_3.sroa.6.i.sroa.10.0..sroa_idx133 = getelementptr inbounds nuw i8, ptr %_4.i, i64 24
  %_3.sroa.6.i.sroa.10.0.copyload134 = load i64, ptr %_3.sroa.6.i.sroa.10.0..sroa_idx133, align 8, !noalias !808
  %_3.sroa.6.i.sroa.13.0..sroa_idx135 = getelementptr inbounds nuw i8, ptr %_4.i, i64 32
  %_3.sroa.6.i.sroa.13.0.copyload136 = load ptr, ptr %_3.sroa.6.i.sroa.13.0..sroa_idx135, align 8, !noalias !808
  %_3.sroa.6.i.sroa.16.0..sroa_idx137 = getelementptr inbounds nuw i8, ptr %_4.i, i64 40
  %_3.sroa.6.i.sroa.16.0.copyload138 = load i64, ptr %_3.sroa.6.i.sroa.16.0..sroa_idx137, align 8, !noalias !808
  %_3.sroa.6.i.sroa.18.0..sroa_idx139 = getelementptr inbounds nuw i8, ptr %_4.i, i64 48
  %_3.sroa.6.i.sroa.18.0.copyload140 = load i64, ptr %_3.sroa.6.i.sroa.18.0..sroa_idx139, align 8, !noalias !808
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_4.i), !noalias !808
  br i1 %.not.i, label %bb15.i, label %bb14.i

bb14.i:                                           ; preds = %bb2.i
  %10 = inttoptr i64 %_3.sroa.6.i.sroa.0.0.copyload130 to ptr
  br label %bb10.i

bb15.i:                                           ; preds = %bb2.i
  %11 = icmp eq i64 %_6.sroa.0.0.copyload, 0
  br i1 %11, label %bb4.i, label %bb2.i.i.i4.i.i.i.i.i11.i

bb2.i.i.i4.i.i.i.i.i11.i:                         ; preds = %bb15.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.sroa.7.0.copyload, i64 noundef %_6.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !815
  br label %bb4.i

bb4.i:                                            ; preds = %bb2.i.i.i4.i.i.i.i.i11.i, %bb15.i
  %12 = inttoptr i64 %_3.sroa.6.i.sroa.7.0.copyload132 to ptr
  %_28.i175 = and i64 %_3.sroa.6.i.sroa.18.0.copyload140, 61440
  %13 = icmp eq i64 %_28.i175, 16384
  br i1 %13, label %bb6.i, label %bb6

bb6.i:                                            ; preds = %bb4.i
  %14 = icmp ne i64 %_3.sroa.6.i.sroa.7.0.copyload132, 0
  tail call void @llvm.assume(i1 %14)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !818)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %hchild.i.i), !noalias !821
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_26.i.i), !noalias !821
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3.i.i.i), !noalias !824
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_5.i.i.i), !noalias !824
  store i32 0, ptr %_5.i.i.i, align 4, !noalias !824
  %_8.sroa.4.0._5.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 4
  store i16 438, ptr %_8.sroa.4.0._5.sroa_idx.i.i.i, align 4, !noalias !824
  %_8.sroa.5.0._5.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i, i64 6
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 2 dereferenceable(6) %_8.sroa.5.0._5.sroa_idx.i.i.i, i8 0, i64 6, i1 false), !noalias !824
  store i8 1, ptr %_8.sroa.5.0._5.sroa_idx.i.i.i, align 2, !noalias !824
; invoke <std::fs::OpenOptions>::_open
  invoke void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_3.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %_5.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %12, i64 noundef %_3.sroa.6.i.sroa.10.0.copyload134)
          to label %.noexc.i unwind label %cleanup.i, !noalias !808

.noexc.i:                                         ; preds = %bb6.i
  %15 = load i32, ptr %_3.i.i.i, align 8, !range !827, !noalias !824, !noundef !12
  %16 = trunc nuw i32 %15 to i1
  br i1 %16, label %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.thread.i.i, label %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i

_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.thread.i.i: ; preds = %.noexc.i
  %17 = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 8
  %_11.i.i.i = load ptr, ptr %17, align 8, !noalias !824, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i.i.i), !noalias !824
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_5.i.i.i), !noalias !824
  br label %bb7.thread28.i

_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i: ; preds = %.noexc.i
  %18 = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 4
  %_10.i.i.i = load i32, ptr %18, align 4, !range !828, !noalias !824, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i.i.i), !noalias !824
; invoke <same_file::unix::Handle>::from_file
  invoke void @_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_26.i.i, i32 noundef %_10.i.i.i)
          to label %.noexc13.i unwind label %cleanup.i, !noalias !808

.noexc13.i:                                       ; preds = %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.i.i
  %.phi.trans.insert.i.i = getelementptr inbounds nuw i8, ptr %_26.i.i, i64 20
  %.pre.i.i = load i8, ptr %.phi.trans.insert.i.i, align 4, !range !829, !noalias !821
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_5.i.i.i), !noalias !824
  %19 = icmp eq i8 %.pre.i.i, 2
  %_30.i.pre.i = load ptr, ptr %_26.i.i, align 8, !noalias !821
  br i1 %19, label %bb7.thread28.i, label %bb17.i.i

common.resume.sink.split.i.i:                     ; preds = %cleanup.i.i35.i.i, %cleanup.i.i.i.i
  %.val.i.i36.sink.i.i = phi i32 [ %.val.i.i.i.i, %cleanup.i.i.i.i ], [ %.val.i.i36.i.i, %cleanup.i.i35.i.i ]
  %common.resume.op.ph.i.i = phi { ptr, i32 } [ %35, %cleanup.i.i.i.i ], [ %47, %cleanup.i.i35.i.i ]
  %_5.i.i.i.i.i.i.i.i38.i.i = call noundef i32 @close(i32 noundef %.val.i.i36.sink.i.i) #26, !noalias !830
  br label %bb12.i

bb7.thread28.i:                                   ; preds = %.noexc13.i, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.thread.i.i
  %_30.i.i = phi ptr [ %_11.i.i.i, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir.exit.thread.i.i ], [ %_30.i.pre.i, %.noexc13.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_26.i.i), !noalias !821
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %hchild.i.i), !noalias !821
  br label %bb10.i

bb17.i.i:                                         ; preds = %.noexc13.i
  %_28.sroa.5.0._26.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_26.i.i, i64 8
  %val.sroa.4.0.hchild.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %val.sroa.4.0.hchild.sroa_idx.i.i, ptr noundef nonnull align 8 dereferenceable(12) %_28.sroa.5.0._26.sroa_idx.i.i, i64 12, i1 false), !noalias !821
  %_28.sroa.7.0._26.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_26.i.i, i64 21
  %val.sroa.6.0.hchild.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 21
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %val.sroa.6.0.hchild.sroa_idx.i.i, ptr noundef nonnull align 1 dereferenceable(3) %_28.sroa.7.0._26.sroa_idx.i.i, i64 3, i1 false), !noalias !821
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_26.i.i), !noalias !821
  store ptr %_30.i.pre.i, ptr %hchild.i.i, align 8, !noalias !821
  %val.sroa.5.0.hchild.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 20
  store i8 %.pre.i.i, ptr %val.sroa.5.0.hchild.sroa_idx.i.i, align 4, !noalias !821
  %20 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_43.i.i = load ptr, ptr %20, align 8, !alias.scope !831, !noalias !832, !nonnull !12, !noundef !12
  %21 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_42.i.i = load i64, ptr %21, align 8, !alias.scope !831, !noalias !832, !noundef !12
  %_47.i.i = getelementptr inbounds nuw %Ancestor, ptr %_43.i.i, i64 %_42.i.i
  %_8.sroa.4.0._5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i, i64 4
  %_8.sroa.5.0._5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i, i64 6
  %22 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 4
  %.phi.trans.insert.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i26.i.i, i64 20
  %_12.sroa.5.0._10.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i26.i.i, i64 8
  %val.sroa.4.0._4.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 8
  %_12.sroa.7.0._10.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i26.i.i, i64 21
  %val.sroa.6.0._4.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 21
  %val.sroa.5.0._4.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 20
  %23 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 16
  br label %bb2.i.i

bb2.i.i:                                          ; preds = %bb21.i.i, %bb17.i.i
  %iter.sroa.5.0.i.i = phi ptr [ %_47.i.i, %bb17.i.i ], [ %_62.i.i, %bb21.i.i ]
  %_51.i.i = icmp eq ptr %_43.i.i, %iter.sroa.5.0.i.i
  br i1 %_51.i.i, label %bb18.i.i, label %bb19.i.i

bb19.i.i:                                         ; preds = %bb2.i.i
  %_62.i.i = getelementptr inbounds i8, ptr %iter.sroa.5.0.i.i, i64 -24
  %24 = getelementptr i8, ptr %iter.sroa.5.0.i.i, i64 -16
  %_62.val.i.i = load ptr, ptr %24, align 8, !alias.scope !833, !noalias !836, !nonnull !12, !noundef !12
  %25 = getelementptr i8, ptr %iter.sroa.5.0.i.i, i64 -8
  %_62.val24.i.i = load i64, ptr %25, align 8, !alias.scope !833, !noalias !836, !noundef !12
  %hchild.val.i.i = load i64, ptr %hchild.i.i, align 8, !noalias !821
  %hchild.val25.i.i = load i64, ptr %val.sroa.4.0.hchild.sroa_idx.i.i, align 8, !noalias !821
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i.i.i), !noalias !839
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_10.i26.i.i), !noalias !839
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3.i.i.i.i), !noalias !842
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_5.i.i.i.i), !noalias !842
  store i32 0, ptr %_5.i.i.i.i, align 4, !noalias !842
  store i16 438, ptr %_8.sroa.4.0._5.sroa_idx.i.i.i.i, align 4, !noalias !842
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 2 dereferenceable(6) %_8.sroa.5.0._5.sroa_idx.i.i.i.i, i8 0, i64 6, i1 false), !noalias !842
  store i8 1, ptr %_8.sroa.5.0._5.sroa_idx.i.i.i.i, align 2, !noalias !842
; invoke <std::fs::OpenOptions>::_open
  invoke void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(none) dereferenceable(16) %_3.i.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(12) %_5.i.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_62.val.i.i, i64 noundef %_62.val24.i.i)
          to label %.noexc.i.i unwind label %cleanup2.loopexit.i.i, !noalias !830

.noexc.i.i:                                       ; preds = %bb19.i.i
  %26 = load i32, ptr %_3.i.i.i.i, align 8, !range !827, !noalias !842, !noundef !12
  %27 = trunc nuw i32 %26 to i1
  br i1 %27, label %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i.i.i, label %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i.i.i

_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i.i.i: ; preds = %.noexc.i.i
  %28 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 8
  %_11.i.i.i.i = load ptr, ptr %28, align 8, !noalias !842, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i.i.i.i), !noalias !842
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_5.i.i.i.i), !noalias !842
  br label %bb20.i.i

_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i.i.i: ; preds = %.noexc.i.i
  %_10.i.i.i.i = load i32, ptr %22, align 4, !range !828, !noalias !842, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3.i.i.i.i), !noalias !842
; invoke <same_file::unix::Handle>::from_file
  invoke void @_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_10.i26.i.i, i32 noundef %_10.i.i.i.i)
          to label %.noexc29.i.i unwind label %cleanup2.loopexit.i.i, !noalias !830

.noexc29.i.i:                                     ; preds = %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i.i.i
  %.pre.i.i.i = load i8, ptr %.phi.trans.insert.i.i.i, align 4, !range !829, !noalias !839
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_5.i.i.i.i), !noalias !842
  %29 = icmp eq i8 %.pre.i.i.i, 2
  %_14.i.pre.i.i = load ptr, ptr %_10.i26.i.i, align 8, !noalias !839
  br i1 %29, label %bb20.i.i, label %bb6.i27.i.i

bb6.i27.i.i:                                      ; preds = %.noexc29.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %val.sroa.4.0._4.sroa_idx.i.i.i, ptr noundef nonnull align 8 dereferenceable(12) %_12.sroa.5.0._10.sroa_idx.i.i.i, i64 12, i1 false), !noalias !839
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %val.sroa.6.0._4.sroa_idx.i.i.i, ptr noundef nonnull align 1 dereferenceable(3) %_12.sroa.7.0._10.sroa_idx.i.i.i, i64 3, i1 false), !noalias !839
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_10.i26.i.i), !noalias !839
  store ptr %_14.i.pre.i.i, ptr %_4.i.i.i, align 8, !noalias !839
  store i8 %.pre.i.i.i, ptr %val.sroa.5.0._4.sroa_idx.i.i.i, align 4, !noalias !839
  %30 = ptrtoint ptr %_14.i.pre.i.i to i64
  %_22.i.i.i = icmp eq i64 %hchild.val.i.i, %30
  %_21.i.i.i = load i64, ptr %val.sroa.4.0._4.sroa_idx.i.i.i, align 8, !noalias !839
  %31 = icmp eq i64 %hchild.val25.i.i, %_21.i.i.i
  %_3.sroa.0.0.off0.i.i.i = select i1 %_22.i.i.i, i1 %31, i1 false
; invoke <same_file::unix::Handle as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(24) %_4.i.i.i)
          to label %bb4.i.i.i.i.i unwind label %cleanup.i.i.i.i.i, !noalias !845

cleanup.i.i.i.i.i:                                ; preds = %bb6.i27.i.i
  %32 = landingpad { ptr, i32 }
          cleanup
  %.val.i.i.i.i.i = load i32, ptr %23, align 8, !alias.scope !846, !noalias !839, !noundef !12
  %33 = icmp eq i32 %.val.i.i.i.i.i, -1
  br i1 %33, label %cleanup2.body.i.i, label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %cleanup.i.i.i.i.i
  %_5.i.i.i.i.i.i.i.i.i.i.i = call noundef i32 @close(i32 noundef %.val.i.i.i.i.i) #26, !noalias !845
  br label %cleanup2.body.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb6.i27.i.i
  %.val1.i.i.i.i.i = load i32, ptr %23, align 8, !alias.scope !846, !noalias !839, !noundef !12
  %34 = icmp eq i32 %.val1.i.i.i.i.i, -1
  br i1 %34, label %bb21.i.i, label %bb2.i2.i.i.i.i.i

bb2.i2.i.i.i.i.i:                                 ; preds = %bb4.i.i.i.i.i
  %_5.i.i.i.i.i.i3.i.i.i.i.i = call noundef i32 @close(i32 noundef %.val1.i.i.i.i.i) #26, !noalias !845
  br label %bb21.i.i

bb18.i.i:                                         ; preds = %bb2.i.i
; invoke <same_file::unix::Handle as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(24) %hchild.i.i)
          to label %bb4.i.i.i.i unwind label %cleanup.i.i.i.i, !noalias !830

cleanup.i.i.i.i:                                  ; preds = %bb18.i.i
  %35 = landingpad { ptr, i32 }
          cleanup
  %36 = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 16
  %.val.i.i.i.i = load i32, ptr %36, align 8, !alias.scope !851, !noalias !821, !noundef !12
  %37 = icmp eq i32 %.val.i.i.i.i, -1
  br i1 %37, label %bb12.i, label %common.resume.sink.split.i.i

bb4.i.i.i.i:                                      ; preds = %bb18.i.i
  %38 = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 16
  %.val1.i.i.i.i = load i32, ptr %38, align 8, !alias.scope !851, !noalias !821, !noundef !12
  %39 = icmp eq i32 %.val1.i.i.i.i, -1
  br i1 %39, label %bb7.thread.i, label %bb2.i2.i.i.i.i

bb2.i2.i.i.i.i:                                   ; preds = %bb4.i.i.i.i
  %_5.i.i.i.i.i.i3.i.i.i.i = call noundef i32 @close(i32 noundef %.val1.i.i.i.i) #26, !noalias !830
  br label %bb7.thread.i

cleanup2.loopexit.i.i:                            ; preds = %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.i.i.i, %bb19.i.i
  %lpad.loopexit.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2.body.i.i

cleanup2.loopexit.split-lp.i.i:                   ; preds = %bb5.i.i
  %lpad.loopexit.split-lp.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup2.body.i.i

cleanup2.body.i.i:                                ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i, %cleanup.i.i.i, %cleanup2.loopexit.split-lp.i.i, %cleanup2.loopexit.i.i, %bb2.i.i.i.i.i.i, %cleanup.i.i.i.i.i
  %eh.lpad-body.i.i = phi { ptr, i32 } [ %32, %bb2.i.i.i.i.i.i ], [ %32, %cleanup.i.i.i.i.i ], [ %42, %bb2.i.i.i4.i.i.i.i.i.i.i ], [ %42, %cleanup.i.i.i ], [ %lpad.loopexit.i.i, %cleanup2.loopexit.i.i ], [ %lpad.loopexit.split-lp.i.i, %cleanup2.loopexit.split-lp.i.i ]
; invoke core::ptr::drop_in_place::<same_file::Handle>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir(ptr noalias noundef align 8 dereferenceable(24) %hchild.i.i) #28
          to label %bb12.i unwind label %terminate.i.i, !noalias !830

bb20.i.i:                                         ; preds = %.noexc29.i.i, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i.i.i
  %_14.i.i.i = phi ptr [ %_11.i.i.i.i, %_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir.exit.thread.i.i.i ], [ %_14.i.pre.i.i, %.noexc29.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_10.i26.i.i), !noalias !839
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i.i.i), !noalias !839
  br label %bb9.i.i

bb21.i.i:                                         ; preds = %bb2.i2.i.i.i.i.i, %bb4.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i.i.i), !noalias !839
  br i1 %_3.sroa.0.0.off0.i.i.i, label %bb5.i.i, label %bb2.i.i

bb5.i.i:                                          ; preds = %bb21.i.i
  %40 = getelementptr i8, ptr %iter.sroa.5.0.i.i, i64 -16
  %41 = getelementptr i8, ptr %iter.sroa.5.0.i.i, i64 -8
  %_83.i.i = load ptr, ptr %40, align 8, !noalias !830, !nonnull !12, !noundef !12
  %_82.i.i = load i64, ptr %41, align 8, !noalias !830, !noundef !12
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i30.i.i), !noalias !856
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i30.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_83.i.i, i64 noundef %_82.i.i)
          to label %.noexc32.i.i unwind label %cleanup2.loopexit.split-lp.i.i, !noalias !830

.noexc32.i.i:                                     ; preds = %bb5.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i.i.i), !noalias !856
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %12, i64 noundef %_3.sroa.6.i.sroa.10.0.copyload134)
          to label %bb6.i.i unwind label %cleanup.i.i.i, !noalias !861

cleanup.i.i.i:                                    ; preds = %.noexc32.i.i
  %42 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !862)
  %_1.val.i.i.i.i = load i64, ptr %_5.i30.i.i, align 8, !alias.scope !862, !noalias !856
  %43 = icmp eq i64 %_1.val.i.i.i.i, 0
  br i1 %43, label %cleanup2.body.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i:                         ; preds = %cleanup.i.i.i
  %44 = getelementptr inbounds nuw i8, ptr %_5.i30.i.i, i64 8
  %_1.val1.i.i.i.i = load ptr, ptr %44, align 8, !alias.scope !862, !noalias !856, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i.i, i64 noundef %_1.val.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !865
  br label %cleanup2.body.i.i

bb6.i.i:                                          ; preds = %.noexc32.i.i
  %_4.i31.i.sroa.0.0.copyload.i = load i64, ptr %_5.i30.i.i, align 8, !noalias !821
  %_4.i31.i.sroa.4.0._5.i30.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i30.i.i, i64 8
  %_4.i31.i.sroa.4.0.copyload.i = load ptr, ptr %_4.i31.i.sroa.4.0._5.i30.i.sroa_idx.i, align 8, !noalias !821
  %_4.i31.i.sroa.5.0._5.i30.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_5.i30.i.i, i64 16
  %45 = load i64, ptr %_4.i31.i.sroa.5.0._5.i30.i.sroa_idx.i, align 8, !noalias !821
  %46 = load i64, ptr %_6.i.i.i, align 8, !noalias !821
  %_4.i31.i.sroa.7.24._6.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 8
  %_4.i31.i.sroa.7.24.copyload.i = load ptr, ptr %_4.i31.i.sroa.7.24._6.i.i.sroa_idx.i, align 8, !noalias !821
  %_4.i31.i.sroa.8.24._6.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i.i.i, i64 16
  %_4.i31.i.sroa.8.24.copyload.i = load i64, ptr %_4.i31.i.sroa.8.24._6.i.i.sroa_idx.i, align 8, !noalias !821
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i.i.i), !noalias !856
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i30.i.i), !noalias !856
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb6.i.i, %bb20.i.i
  %_11.sroa.12.sroa.0.1.i = phi i64 [ undef, %bb20.i.i ], [ %45, %bb6.i.i ]
  %_11.sroa.12.sroa.5.1.i = phi i64 [ undef, %bb20.i.i ], [ %46, %bb6.i.i ]
  %_11.sroa.9.0.i = phi ptr [ inttoptr (i64 -9223372036854775808 to ptr), %bb20.i.i ], [ %_4.i31.i.sroa.4.0.copyload.i, %bb6.i.i ]
  %_11.sroa.14.0.i = phi i64 [ undef, %bb20.i.i ], [ %_4.i31.i.sroa.8.24.copyload.i, %bb6.i.i ]
  %_11.sroa.1218.0.i = phi ptr [ %_14.i.i.i, %bb20.i.i ], [ %_4.i31.i.sroa.7.24.copyload.i, %bb6.i.i ]
  %_11.sroa.0.0.i = phi i64 [ -9223372036854775808, %bb20.i.i ], [ %_4.i31.i.sroa.0.0.copyload.i, %bb6.i.i ]
; invoke <same_file::unix::Handle as core::ops::drop::Drop>::drop
  invoke void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(24) %hchild.i.i)
          to label %bb4.i.i40.i.i unwind label %cleanup.i.i35.i.i, !noalias !830

cleanup.i.i35.i.i:                                ; preds = %bb9.i.i
  %47 = landingpad { ptr, i32 }
          cleanup
  %48 = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 16
  %.val.i.i36.i.i = load i32, ptr %48, align 8, !alias.scope !866, !noalias !821, !noundef !12
  %49 = icmp eq i32 %.val.i.i36.i.i, -1
  br i1 %49, label %bb12.i, label %common.resume.sink.split.i.i

bb4.i.i40.i.i:                                    ; preds = %bb9.i.i
  %50 = getelementptr inbounds nuw i8, ptr %hchild.i.i, i64 16
  %.val1.i.i41.i.i = load i32, ptr %50, align 8, !alias.scope !866, !noalias !821, !noundef !12
  %51 = icmp eq i32 %.val1.i.i41.i.i, -1
  br i1 %51, label %bb7.i, label %bb2.i2.i.i42.i.i

bb2.i2.i.i42.i.i:                                 ; preds = %bb4.i.i40.i.i
  %_5.i.i.i.i.i.i3.i.i43.i.i = call noundef i32 @close(i32 noundef %.val1.i.i41.i.i) #26, !noalias !830
  br label %bb7.i

terminate.i.i:                                    ; preds = %cleanup2.body.i.i
  %52 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !830
  unreachable

bb7.thread.i:                                     ; preds = %bb2.i2.i.i.i.i, %bb4.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %hchild.i.i), !noalias !821
  br label %bb6

bb7.i:                                            ; preds = %bb2.i2.i.i42.i.i, %bb4.i.i40.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %hchild.i.i), !noalias !821
  %.not5.i = icmp eq i64 %_11.sroa.0.0.i, -9223372036854775807
  br i1 %.not5.i, label %bb6, label %bb10.i

bb10.i:                                           ; preds = %bb7.thread28.i, %bb7.i, %bb14.i
  %_6.sroa.0.2 = phi i64 [ %_6.sroa.0.0.copyload, %bb14.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %bb7.i ], [ %_3.sroa.6.i.sroa.0.0.copyload130, %bb7.thread28.i ]
  %_4.sroa.13.0 = phi i64 [ %_3.sroa.6.i.sroa.18.0.copyload140, %bb14.i ], [ %_5.i, %bb7.i ], [ %_5.i, %bb7.thread28.i ]
  %_4.sroa.12.0 = phi i64 [ %_3.sroa.6.i.sroa.16.0.copyload138, %bb14.i ], [ %_11.sroa.14.0.i, %bb7.i ], [ undef, %bb7.thread28.i ]
  %_4.sroa.11.0 = phi ptr [ %_3.sroa.6.i.sroa.13.0.copyload136, %bb14.i ], [ %_11.sroa.1218.0.i, %bb7.i ], [ %_30.i.i, %bb7.thread28.i ]
  %_4.sroa.10.0 = phi i64 [ %_3.sroa.6.i.sroa.10.0.copyload134, %bb14.i ], [ %_11.sroa.12.sroa.5.1.i, %bb7.i ], [ undef, %bb7.thread28.i ]
  %_4.sroa.9.0 = phi i64 [ %_3.sroa.6.i.sroa.7.0.copyload132, %bb14.i ], [ %_11.sroa.12.sroa.0.1.i, %bb7.i ], [ undef, %bb7.thread28.i ]
  %_4.sroa.5.0 = phi ptr [ %10, %bb14.i ], [ %_11.sroa.9.0.i, %bb7.i ], [ inttoptr (i64 -9223372036854775808 to ptr), %bb7.thread28.i ]
  %_4.sroa.0.0 = phi i64 [ %8, %bb14.i ], [ %_11.sroa.0.0.i, %bb7.i ], [ -9223372036854775808, %bb7.thread28.i ]
  %dent.val8.i = phi ptr [ %_6.sroa.7.0.copyload, %bb14.i ], [ %12, %bb7.i ], [ %12, %bb7.thread28.i ]
  %53 = icmp eq i64 %_6.sroa.0.2, 0
  br i1 %53, label %bb5, label %bb2.i.i.i4.i.i.i.i.i14.i

bb2.i.i.i4.i.i.i.i.i14.i:                         ; preds = %bb10.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %dent.val8.i, i64 noundef %_6.sroa.0.2, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !871
  br label %bb5

cleanup:                                          ; preds = %bb7.i81, %bb14, %bb30, %bb52, %bb22, %bb32
  %54 = landingpad { ptr, i32 }
          cleanup
  br label %bb54

bb5:                                              ; preds = %bb10.i, %bb2.i.i.i4.i.i.i.i.i14.i
  store i64 %_4.sroa.0.0, ptr %_0, align 8
  %_4.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_4.sroa.5.0, ptr %_4.sroa.5.0._0.sroa_idx, align 8
  %_4.sroa.9.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_4.sroa.9.0, ptr %_4.sroa.9.0._0.sroa_idx, align 8
  %_4.sroa.10.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %_4.sroa.10.0, ptr %_4.sroa.10.0._0.sroa_idx, align 8
  %_4.sroa.11.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_4.sroa.11.0, ptr %_4.sroa.11.0._0.sroa_idx, align 8
  %_4.sroa.12.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %_4.sroa.12.0, ptr %_4.sroa.12.0._0.sroa_idx, align 8
  %_4.sroa.13.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %_4.sroa.13.0, ptr %_4.sroa.13.0._0.sroa_idx, align 8
  br label %bb50

bb6:                                              ; preds = %bb7.i, %bb7.thread.i, %bb4.i
  %55 = inttoptr i64 %_3.sroa.6.i.sroa.0.0.copyload130 to ptr
  store ptr %55, ptr %dent, align 8
  store i64 %_3.sroa.6.i.sroa.7.0.copyload132, ptr %_6.sroa.7.0.dent.sroa_idx, align 8
  store i64 %_3.sroa.6.i.sroa.10.0.copyload134, ptr %_6.sroa.9.0.dent.sroa_idx, align 8
  store ptr %_3.sroa.6.i.sroa.13.0.copyload136, ptr %_6.sroa.11.0.dent.sroa_idx, align 8
  %v.sroa.5.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 32
  store i64 %_3.sroa.6.i.sroa.16.0.copyload138, ptr %v.sroa.5.0.dent.sroa_idx, align 8
  store i64 %_3.sroa.6.i.sroa.18.0.copyload140, ptr %2, align 8
  %56 = trunc i64 %_3.sroa.6.i.sroa.18.0.copyload140 to i16
  br label %bb8

bb50:                                             ; preds = %bb2.i.i.i4.i.i.i.i.i101, %bb45, %bb2.i.i.i4.i.i.i.i.i, %bb49, %bb46, %bb59, %bb5
  ret void

bb27:                                             ; preds = %bb8
  %57 = icmp eq i16 %_53, -24576
  %58 = getelementptr inbounds nuw i8, ptr %dent, i64 24
  %_31 = load i64, ptr %58, align 8, !noundef !12
  %59 = icmp eq i64 %_31, 0
  %60 = getelementptr inbounds nuw i8, ptr %self, i64 153
  %61 = load i8, ptr %60, align 1, !range !720
  %_32 = trunc nuw i8 %61 to i1
  %62 = and i1 %57, %59
  %or.cond = select i1 %62, i1 %_32, i1 false
  br i1 %or.cond, label %bb30, label %bb44

bb12:                                             ; preds = %bb8
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 155
  %64 = load i8, ptr %63, align 1, !range !720, !noundef !12
  %_12 = trunc nuw i8 %64 to i1
  %65 = getelementptr inbounds nuw i8, ptr %dent, i64 24
  %_14 = load i64, ptr %65, align 8
  %_13 = icmp ne i64 %_14, 0
  %or.cond33 = select i1 %_12, i1 %_13, i1 false
  br i1 %or.cond33, label %bb14, label %bb22

bb30:                                             ; preds = %bb27
  %66 = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %_63 = load ptr, ptr %66, align 8, !nonnull !12, !noundef !12
  %67 = getelementptr inbounds nuw i8, ptr %dent, i64 16
  %_62 = load i64, ptr %67, align 8, !noundef !12
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i), !noalias !874
; invoke std::sys::fs::metadata
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_63, i64 noundef %_62)
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb30
  %_5.i70 = load i64, ptr %self.i, align 8, !range !687, !noalias !874, !noundef !12
  %68 = trunc nuw i64 %_5.i70 to i1
  %69 = getelementptr inbounds nuw i8, ptr %self.i, i64 8
  %e.i = load ptr, ptr %69, align 8, !noalias !878
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i), !noalias !874
  br i1 %68, label %bb56, label %bb57

bb56:                                             ; preds = %.noexc
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i)
  store ptr %e.i, ptr %err.i, align 8, !noalias !879
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i73), !noalias !879
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4.i73, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_63, i64 noundef %_62)
          to label %bb58 unwind label %cleanup.i74, !noalias !879

cleanup.i74:                                      ; preds = %bb56
  %70 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i) #28
          to label %bb54 unwind label %terminate.i, !noalias !879

terminate.i:                                      ; preds = %cleanup.i74
  %71 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !879
  unreachable

bb57:                                             ; preds = %.noexc
  %72 = ptrtoint ptr %e.i to i64
  %73 = and i64 %72, 263882790666240
  %74 = icmp eq i64 %73, 70368744177664
  br i1 %74, label %bb32, label %bb44

bb32:                                             ; preds = %bb57
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_40)
; invoke <walkdir::IntoIter>::push
  invoke fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter4push(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_40, ptr noalias noundef align 8 dereferenceable(176) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) %dent)
          to label %bb33 unwind label %cleanup

bb33:                                             ; preds = %bb32
  %75 = load i64, ptr %_40, align 8, !range !16, !noundef !12
  %.not60 = icmp eq i64 %75, -9223372036854775807
  br i1 %.not60, label %bb35, label %bb34

bb34:                                             ; preds = %bb33
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %_40, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_40)
  br label %bb49

bb35:                                             ; preds = %bb33
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_40)
  br label %bb44

bb58:                                             ; preds = %bb56
  %_68.sroa.6.8.copyload = load i64, ptr %_4.i73, align 8, !noalias !883
  %_68.sroa.8.8._4.i73.sroa_idx = getelementptr inbounds nuw i8, ptr %_4.i73, i64 8
  %76 = ptrtoint ptr %e.i to i64
  %_39.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 4
  %_39.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 6
  %_39.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  %_39.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  %77 = load <4 x i32>, ptr %_68.sroa.8.8._4.i73.sroa_idx, align 8, !noalias !883
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i73), !noalias !879
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i)
  store i32 0, ptr %_0, align 8
  store i16 0, ptr %_39.sroa.4.0._0.sroa_idx, align 4
  store i16 -32768, ptr %_39.sroa.5.0._0.sroa_idx, align 2
  store i64 %_68.sroa.6.8.copyload, ptr %_39.sroa.6.0._0.sroa_idx, align 8
  store <4 x i32> %77, ptr %_39.sroa.7.0._0.sroa_idx, align 8
  %_39.sroa.11.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 %76, ptr %_39.sroa.11.0._0.sroa_idx, align 8
  %_39.sroa.13.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 0, ptr %_39.sroa.13.0._0.sroa_idx, align 8
  br label %bb49

bb49:                                             ; preds = %bb24, %bb19, %bb16, %bb34, %bb58
  %dent.val65 = load i64, ptr %dent, align 8, !alias.scope !17
  %78 = icmp eq i64 %dent.val65, 0
  br i1 %78, label %bb50, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb49
  %79 = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %dent.val66 = load ptr, ptr %79, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %dent.val66, i64 noundef %dent.val65, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !884
  br label %bb50

bb41:                                             ; preds = %bb17, %bb25, %bb20
  %80 = getelementptr inbounds nuw i8, ptr %self, i64 154
  %81 = load i8, ptr %80, align 2, !range !720
  %_45 = trunc nuw i8 %81 to i1
  br i1 %_45, label %bb43, label %bb44

bb22:                                             ; preds = %bb12
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_26)
; invoke <walkdir::IntoIter>::push
  invoke fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter4push(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_26, ptr noalias noundef align 8 dereferenceable(176) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) %dent)
          to label %bb23 unwind label %cleanup

bb14:                                             ; preds = %bb12
  %self.val = load i64, ptr %self, align 8
  %82 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val69 = load i64, ptr %82, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !887)
  %83 = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %_18.i = load ptr, ptr %83, align 8, !alias.scope !887, !noalias !890, !nonnull !12, !noundef !12
  %84 = getelementptr inbounds nuw i8, ptr %dent, i64 16
  %_17.i = load i64, ptr %84, align 8, !alias.scope !887, !noalias !890, !noundef !12
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i.i.i), !noalias !892
; invoke std::sys::fs::metadata
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_18.i, i64 noundef %_17.i)
          to label %.noexc86 unwind label %cleanup

.noexc86:                                         ; preds = %bb14
  %_5.i.i.i80 = load i64, ptr %self.i.i.i, align 8, !range !687, !noalias !892, !noundef !12
  %85 = trunc nuw i64 %_5.i.i.i80 to i1
  %86 = getelementptr inbounds nuw i8, ptr %self.i.i.i, i64 8
  %e.i.i.i = load ptr, ptr %86, align 8, !noalias !898
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i.i.i), !noalias !892
  %87 = ptrtoint ptr %e.i.i.i to i64
  %sext.i.i = shl i64 %87, 32
  %_6.i.i = ashr exact i64 %sext.i.i, 32
  br i1 %85, label %bb4.i84, label %bb5.i

bb4.i84:                                          ; preds = %.noexc86
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i.i), !noalias !899
  store ptr %e.i.i.i, ptr %err.i.i, align 8, !noalias !900
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6.i6.i), !noalias !900
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6.i6.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_18.i, i64 noundef %_17.i)
          to label %bb16 unwind label %cleanup.i.i, !noalias !900

cleanup.i.i:                                      ; preds = %bb4.i84
  %88 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i.i) #28
          to label %bb54 unwind label %terminate.i.i85, !noalias !900

terminate.i.i85:                                  ; preds = %cleanup.i.i
  %89 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !900
  unreachable

bb5.i:                                            ; preds = %.noexc86
  %90 = trunc nuw i64 %self.val to i1
  br i1 %90, label %bb17, label %bb7.i81, !prof !904

bb7.i81:                                          ; preds = %bb5.i
; invoke core::option::expect_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_47e9a95a7e6743c630b8dc44fd8334eb, i64 noundef 51, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5d4660e25ce5d46712141b88085cffca) #32
          to label %.noexc90 unwind label %cleanup

.noexc90:                                         ; preds = %bb7.i81
  unreachable

bb23:                                             ; preds = %bb22
  %91 = load i64, ptr %_26, align 8, !range !16, !noundef !12
  %.not61 = icmp eq i64 %91, -9223372036854775807
  br i1 %.not61, label %bb25, label %bb24

bb24:                                             ; preds = %bb23
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %_26, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_26)
  br label %bb49

bb25:                                             ; preds = %bb23
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_26)
  br label %bb41

bb16:                                             ; preds = %bb4.i84
  %_23.sroa.4.8.copyload.i = load i64, ptr %_6.i6.i, align 8, !noalias !905
  %_23.sroa.6.8._6.i6.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_6.i6.i, i64 8
  %_20.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_20.sroa.5.0._0.sroa_idx, ptr noundef nonnull align 8 dereferenceable(16) %_23.sroa.6.8._6.i6.sroa_idx.i, i64 16, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6.i6.i), !noalias !900
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i.i), !noalias !899
  store i64 -9223372036854775808, ptr %_0, align 8
  %_20.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_23.sroa.4.8.copyload.i, ptr %_20.sroa.4.0._0.sroa_idx, align 8
  %_20.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %e.i.i.i, ptr %_20.sroa.6.0._0.sroa_idx, align 8
  %_20.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %_14, ptr %_20.sroa.8.0._0.sroa_idx, align 8
  br label %bb49

bb17:                                             ; preds = %bb5.i
  %_29.i83 = icmp eq i64 %self.val69, %_6.i.i
  br i1 %_29.i83, label %bb52, label %bb41

bb52:                                             ; preds = %bb17
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_21)
; invoke <walkdir::IntoIter>::push
  invoke fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter4push(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_21, ptr noalias noundef align 8 dereferenceable(176) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) %dent)
          to label %bb18 unwind label %cleanup

bb18:                                             ; preds = %bb52
  %92 = load i64, ptr %_21, align 8, !range !16, !noundef !12
  %.not63 = icmp eq i64 %92, -9223372036854775807
  br i1 %.not63, label %bb20, label %bb19

bb19:                                             ; preds = %bb18
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %_21, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_21)
  br label %bb49

bb20:                                             ; preds = %bb18
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_21)
  br label %bb41

bb44:                                             ; preds = %bb27, %bb57, %bb35, %bb41
  %93 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %_73 = load i64, ptr %93, align 8, !noundef !12
  %94 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %_74 = load i64, ptr %94, align 8, !noundef !12
  %_72 = icmp ult i64 %_73, %_74
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %_75 = load i64, ptr %95, align 8
  %_48 = icmp ugt i64 %_73, %_75
  %or.cond35 = select i1 %_72, i1 true, i1 %_48
  br i1 %or.cond35, label %bb45, label %bb46

bb43:                                             ; preds = %bb41
  %_46 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %_47.sroa.0.0.copyload = load i64, ptr %dent, align 8
  %_47.sroa.5.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %_47.sroa.5.0.copyload = load ptr, ptr %_47.sroa.5.0.dent.sroa_idx, align 8
  %_47.sroa.6.0.dent.sroa_idx = getelementptr inbounds nuw i8, ptr %dent, i64 16
  call void @llvm.experimental.noalias.scope.decl(metadata !906)
  %96 = getelementptr inbounds nuw i8, ptr %self, i64 80
  %len.i = load i64, ptr %96, align 8, !alias.scope !906, !noalias !909, !noundef !12
  %self1.i = load i64, ptr %_46, align 8, !range !48, !alias.scope !906, !noalias !909, !noundef !12
  %_4.i91 = icmp eq i64 %len.i, %self1.i
  br i1 %_4.i91, label %bb1.i94, label %bb59

bb1.i94:                                          ; preds = %bb43
; invoke <alloc::raw_vec::RawVec<walkdir::dent::DirEntry>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryE8grow_oneBQ_(ptr noalias noundef nonnull align 8 dereferenceable(24) %_46)
          to label %bb59 unwind label %cleanup.i95, !noalias !909

cleanup.i95:                                      ; preds = %bb1.i94
  %97 = landingpad { ptr, i32 }
          cleanup
  %98 = icmp eq i64 %_47.sroa.0.0.copyload, 0
  br i1 %98, label %bb51, label %bb2.i.i.i4.i.i.i.i.i.i96

bb2.i.i.i4.i.i.i.i.i.i96:                         ; preds = %cleanup.i95
  %99 = icmp ne ptr %_47.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %99)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_47.sroa.5.0.copyload, i64 noundef %_47.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !911
  br label %bb51

bb46:                                             ; preds = %bb44
  %_49.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_49.sroa.4.0._0.sroa_idx, ptr noundef nonnull align 8 dereferenceable(48) %dent, i64 48, i1 false)
  store i64 -9223372036854775807, ptr %_0, align 8
  br label %bb50

bb45:                                             ; preds = %bb44
  store i64 -9223372036854775806, ptr %_0, align 8
  %dent.val67 = load i64, ptr %dent, align 8, !alias.scope !17
  %100 = icmp eq i64 %dent.val67, 0
  br i1 %100, label %bb50, label %bb2.i.i.i4.i.i.i.i.i101

bb2.i.i.i4.i.i.i.i.i101:                          ; preds = %bb45
  %101 = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %dent.val68 = load ptr, ptr %101, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %dent.val68, i64 noundef %dent.val67, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !914
  br label %bb50

bb59:                                             ; preds = %bb1.i94, %bb43
  %102 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %_14.i93 = load ptr, ptr %102, align 8, !alias.scope !906, !noalias !909, !nonnull !12, !noundef !12
  %end.i = getelementptr inbounds nuw %"dent::DirEntry", ptr %_14.i93, i64 %len.i
  store i64 %_47.sroa.0.0.copyload, ptr %end.i, align 8, !noalias !906
  %_47.sroa.5.0.end.i.sroa_idx = getelementptr inbounds nuw i8, ptr %end.i, i64 8
  store ptr %_47.sroa.5.0.copyload, ptr %_47.sroa.5.0.end.i.sroa_idx, align 8, !noalias !906
  %_47.sroa.6.0.end.i.sroa_idx = getelementptr inbounds nuw i8, ptr %end.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_47.sroa.6.0.end.i.sroa_idx, ptr noundef nonnull align 8 dereferenceable(32) %_47.sroa.6.0.dent.sroa_idx, i64 32, i1 false)
  %103 = add i64 %len.i, 1
  store i64 %103, ptr %96, align 8, !alias.scope !906, !noalias !909
  store i64 -9223372036854775806, ptr %_0, align 8
  br label %bb50

bb51:                                             ; preds = %bb2.i.i.i4.i.i.i.i.i103, %bb54, %cleanup.i95, %bb2.i.i.i4.i.i.i.i.i.i96, %bb2.i.i.i4.i.i.i.i.i.i, %bb12.i
  %eh.lpad-body145 = phi { ptr, i32 } [ %.pn.i, %bb2.i.i.i4.i.i.i.i.i.i ], [ %.pn.i, %bb12.i ], [ %97, %bb2.i.i.i4.i.i.i.i.i.i96 ], [ %97, %cleanup.i95 ], [ %eh.lpad-body.ph, %bb54 ], [ %eh.lpad-body.ph, %bb2.i.i.i4.i.i.i.i.i103 ]
  resume { ptr, i32 } %eh.lpad-body145

bb54:                                             ; preds = %cleanup, %cleanup.i74, %cleanup.i.i
  %eh.lpad-body.ph = phi { ptr, i32 } [ %54, %cleanup ], [ %70, %cleanup.i74 ], [ %88, %cleanup.i.i ]
  %dent.val = load i64, ptr %dent, align 8, !alias.scope !17
  %104 = icmp eq i64 %dent.val, 0
  br i1 %104, label %bb51, label %bb2.i.i.i4.i.i.i.i.i103

bb2.i.i.i4.i.i.i.i.i103:                          ; preds = %bb54
  %105 = getelementptr inbounds nuw i8, ptr %dent, i64 8
  %dent.val64 = load ptr, ptr %105, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %dent.val64, i64 noundef %dent.val, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !917
  br label %bb51
}

; <walkdir::IntoIter>::skip_current_dir
; Function Attrs: uwtable
define void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16skip_current_dir(ptr noalias noundef align 8 captures(none) dereferenceable(176) %self) unnamed_addr #2 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_3 = load i64, ptr %0, align 8, !noundef !12
  %_4 = icmp ult i64 %_3, 144115188075855872
  tail call void @llvm.assume(i1 %_4)
  %1 = icmp eq i64 %_3, 0
  br i1 %1, label %bb3, label %bb2

bb2:                                              ; preds = %start
; call <walkdir::IntoIter>::pop
  tail call fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter3pop(ptr noalias noundef align 8 dereferenceable(176) %self)
  br label %bb3

bb3:                                              ; preds = %start, %bb2
  ret void
}

; <walkdir::IntoIter>::pop
; Function Attrs: uwtable
define internal fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter3pop(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(176) %self) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_2 = alloca [64 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_2)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_10 = load i64, ptr %0, align 8, !noundef !12
  %1 = icmp eq i64 %_10, 0
  br i1 %1, label %bb5, label %bb6, !prof !645

bb5:                                              ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a7455143ebb32770d194a02d72a823aa, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_81e4be2212f19f75ac10f97b281265e5) #32
  unreachable

bb6:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %3 = add nsw i64 %_10, -1
  store i64 %3, ptr %0, align 8
  %_18 = load i64, ptr %2, align 8, !range !48, !noundef !12
  %_11 = icmp samesign ult i64 %3, %_18
  tail call void @llvm.assume(i1 %_11)
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_19 = load ptr, ptr %4, align 8, !nonnull !12, !noundef !12
  %_20 = icmp ult i64 %_10, 144115188075855873
  tail call void @llvm.assume(i1 %_20)
  %_15 = getelementptr inbounds nuw %DirList, ptr %_19, i64 %3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_2, ptr noundef nonnull align 8 dereferenceable(64) %_15, i64 64, i1 false)
; call core::ptr::drop_in_place::<walkdir::DirList>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef align 8 dereferenceable(64) %_2)
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %_2)
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 152
  %6 = load i8, ptr %5, align 8, !range !720, !noundef !12
  %_4 = trunc nuw i8 %6 to i1
  br i1 %_4, label %bb2, label %bb4

bb4:                                              ; preds = %bb2.i.i.i4.i.i.i.i.i, %bb8, %bb6
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 160
  %_8 = load i64, ptr %7, align 8, !noundef !12
  %_0.sroa.0.0.i = call noundef i64 @llvm.umin.i64(i64 range(i64 -164703072086692423, 164703072086692426) %3, i64 %_8)
  store i64 %_0.sroa.0.0.i, ptr %7, align 8
  ret void

bb2:                                              ; preds = %bb6
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 56
  %_22 = load i64, ptr %8, align 8, !noundef !12
  %9 = icmp eq i64 %_22, 0
  br i1 %9, label %bb7, label %bb8, !prof !645

bb7:                                              ; preds = %bb2
; call core::option::expect_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_369d4fca9889d20ea478b2535f2f67dd, i64 noundef 33, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f5c1e5e8c2c061740d120bfbf0725b0f) #32
  unreachable

bb8:                                              ; preds = %bb2
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %11 = add nsw i64 %_22, -1
  store i64 %11, ptr %8, align 8
  %_30 = load i64, ptr %10, align 8, !range !48, !noundef !12
  %_23 = icmp samesign ult i64 %11, %_30
  call void @llvm.assume(i1 %_23)
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %_31 = load ptr, ptr %12, align 8, !nonnull !12, !noundef !12
  %_32 = icmp ult i64 %_22, 384307168202282327
  call void @llvm.assume(i1 %_32)
  %_27 = getelementptr inbounds nuw %Ancestor, ptr %_31, i64 %11
  %_26.sroa.0.0.copyload = load i64, ptr %_27, align 8
  %13 = icmp eq i64 %_26.sroa.0.0.copyload, 0
  br i1 %13, label %bb4, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb8
  %_26.sroa.4.0._27.sroa_idx = getelementptr inbounds nuw i8, ptr %_27, i64 8
  %_26.sroa.4.0.copyload = load ptr, ptr %_26.sroa.4.0._27.sroa_idx, align 8, !nonnull !12, !noundef !12
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_26.sroa.4.0.copyload, i64 noundef %_26.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !920
  br label %bb4
}

; <walkdir::IntoIter>::push
; Function Attrs: uwtable
define internal fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter4push(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(56) %_0, ptr noalias noundef nonnull align 8 dereferenceable(176) %0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %is_less.i.i = alloca [8 x i8], align 8
  %compare.i = alloca [8 x i8], align 8
  %r1.i.i.i.i = alloca [1056 x i8], align 8
  %_19.i.i.i.i16 = alloca [56 x i8], align 16
  %_10.i.i.i.i = alloca [1064 x i8], align 8
  %element.i.i.i = alloca [56 x i8], align 8
  %_3.sroa.11.i.i.i = alloca [48 x i8], align 8
  %r1.i.i = alloca [1056 x i8], align 8
  %_19.i.i = alloca [56 x i8], align 16
  %_10.i.i = alloca [1064 x i8], align 8
  %_19.i = alloca [64 x i8], align 8
  %element.i = alloca [56 x i8], align 8
  %_3.sroa.11.i = alloca [48 x i8], align 8
  %vector.i = alloca [24 x i8], align 8
  %_5.i = alloca [24 x i8], align 8
  %err.i = alloca [8 x i8], align 8
  %_3.i.i.i.i = alloca [56 x i8], align 8
  %_3.i.i = alloca [56 x i8], align 8
  %vector.i.i = alloca [24 x i8], align 8
  %self.i = alloca [16 x i8], align 8
  %_38 = alloca [64 x i8], align 8
  %_33 = alloca [24 x i8], align 8
  %_23 = alloca [64 x i8], align 8
  %entries = alloca [24 x i8], align 8
  %cmp = alloca [8 x i8], align 8
  %list = alloca [64 x i8], align 8
  %rd.sroa.5.sroa.0 = alloca [15 x i8], align 1
  %2 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %3 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %_5 = load i64, ptr %3, align 8, !noundef !12
  %_45 = icmp ult i64 %_5, 144115188075855872
  tail call void @llvm.assume(i1 %_45)
  %4 = getelementptr inbounds nuw i8, ptr %0, i64 160
  %_6 = load i64, ptr %4, align 8, !noundef !12
  %_46 = icmp ult i64 %_5, %_6
  br i1 %_46, label %bb21, label %bb22, !prof !645

bb22:                                             ; preds = %start
  %_47 = sub nuw nsw i64 %_5, %_6
  %5 = getelementptr inbounds nuw i8, ptr %0, i64 112
  %6 = getelementptr inbounds nuw i8, ptr %0, i64 128
  %_8 = load i64, ptr %6, align 8, !noundef !12
  %_7 = icmp eq i64 %_47, %_8
  br i1 %_7, label %bb1, label %bb3

bb21:                                             ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_28fa3a3487cb94fdc7ce5cf2fd7f5672) #32
  unreachable

bb1:                                              ; preds = %bb22
  %_54 = icmp samesign ult i64 %_6, %_5
  br i1 %_54, label %bb23, label %panic

bb3:                                              ; preds = %bb2.i, %bb23, %bb22
  %7 = getelementptr inbounds nuw i8, ptr %1, i64 8
  %_61 = load ptr, ptr %7, align 8, !nonnull !12, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %1, i64 16
  %_60 = load i64, ptr %8, align 8, !noundef !12
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %self.i), !noalias !923
; call std::sys::fs::read_dir
  call void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8read_dir(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %self.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_61, i64 noundef %_60), !noalias !927
  %9 = getelementptr inbounds nuw i8, ptr %self.i, i64 8
  %10 = load i8, ptr %9, align 8, !range !829, !noalias !923, !noundef !12
  %t.0.sink.i = load ptr, ptr %self.i, align 8, !noalias !923, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %self.i), !noalias !923
  %11 = icmp eq i8 %10, 2
  br i1 %11, label %bb25, label %bb24

bb23:                                             ; preds = %bb1
  %12 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %_53 = load ptr, ptr %12, align 8, !nonnull !12, !noundef !12
  %_10 = getelementptr inbounds nuw %DirList, ptr %_53, i64 %_6
  tail call void @llvm.experimental.noalias.scope.decl(metadata !928)
  %13 = load i64, ptr %_10, align 8, !range !49, !alias.scope !928, !noundef !12
  %14 = icmp eq i64 %13, -9223372036854775805
  br i1 %14, label %bb3, label %bb1.i

bb1.i:                                            ; preds = %bb23
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_3.i.i), !noalias !928
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i), !noalias !931
; call <&mut walkdir::DirList as core::iter::traits::iterator::Iterator>::next
  call fastcc void @_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtCsarYwbBXrH4d_7walkdir7DirListNtB5_8Iterator4nextBS_(ptr noalias noundef align 8 captures(address) dereferenceable(56) %_3.i.i, ptr nonnull align 8 dereferenceable(64) %_10), !noalias !935
  %15 = load i64, ptr %_3.i.i, align 8, !range !936, !noalias !931, !noundef !12
  %.not.i.i = icmp eq i64 %15, -9223372036854775806
  br i1 %.not.i.i, label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_.exit.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i

cleanup2.i.i:                                     ; preds = %bb3.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef align 8 dereferenceable(56) %_3.i.i) #28
          to label %common.resume unwind label %terminate.i.i, !noalias !935

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i: ; preds = %bb1.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !937
; call __rustc::__rust_alloc
  %17 = tail call noundef align 8 dereferenceable_or_null(224) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 224, i64 noundef range(i64 1, 9) 8) #26, !noalias !937
  %18 = icmp eq ptr %17, null
  br i1 %18, label %bb3.i.i.i, label %bb15.i.i

bb3.i.i.i:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 8, i64 224) #27
          to label %.noexc.i.i unwind label %cleanup2.i.i, !noalias !935

.noexc.i.i:                                       ; preds = %bb3.i.i.i
  unreachable

bb15.i.i:                                         ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %17, ptr noundef nonnull align 8 dereferenceable(56) %_3.i.i, i64 56, i1 false), !noalias !935
  store i64 4, ptr %vector.i.i, align 8, !noalias !931
  %vector1.sroa.4.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 8
  store ptr %17, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !noalias !931
  %vector1.sroa.6.0.vector.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !noalias !931
  tail call void @llvm.experimental.noalias.scope.decl(metadata !940)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !943)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_3.i.i.i.i), !noalias !946
; invoke <&mut walkdir::DirList as core::iter::traits::iterator::Iterator>::next
  invoke fastcc void @_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtCsarYwbBXrH4d_7walkdir7DirListNtB5_8Iterator4nextBS_(ptr noalias noundef align 8 captures(address) dereferenceable(56) %_3.i.i.i.i, ptr nonnull align 8 dereferenceable(64) %_10)
          to label %.noexc6.i.i unwind label %cleanup3.loopexit.split-lp.i.i, !noalias !935

.noexc6.i.i:                                      ; preds = %bb15.i.i
  %19 = load i64, ptr %_3.i.i.i.i, align 8, !range !936, !noalias !948, !noundef !12
  %.not5.i.i.i.i = icmp eq i64 %19, -9223372036854775806
  br i1 %.not5.i.i.i.i, label %bb5.i.i, label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %.noexc6.i.i, %.noexc7.i.i
  %_23.i.i8.i.i = phi ptr [ %_23.i.i.i.i, %.noexc7.i.i ], [ %17, %.noexc6.i.i ]
  %len.i.i.i.i = phi i64 [ %new_len.i.i.i.i, %.noexc7.i.i ], [ 1, %.noexc6.i.i ]
  %_19.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i, 164703072086692426
  tail call void @llvm.assume(i1 %_19.i.i.i.i)
  %self1.i.i.i.i = load i64, ptr %vector.i.i, align 8, !range !48, !alias.scope !950, !noalias !951, !noundef !12
  %_8.i.i.i.i = icmp eq i64 %len.i.i.i.i, %self1.i.i.i.i
  br i1 %_8.i.i.i.i, label %bb1.i.i.i.i.i, label %bb8.i.i.i.i

bb8.i.i.i.i:                                      ; preds = %bb1.i.i.i.bb8.i.i_crit_edge.i.i, %bb3.i.i.i.i
  %_23.i.i.i.i = phi ptr [ %_23.i.i.pre.i.i, %bb1.i.i.i.bb8.i.i_crit_edge.i.i ], [ %_23.i.i8.i.i, %bb3.i.i.i.i ]
  %dst.i.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_23.i.i.i.i, i64 %len.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(56) %_3.i.i.i.i, i64 56, i1 false), !noalias !952
  %new_len.i.i.i.i = add nuw nsw i64 %len.i.i.i.i, 1
  store i64 %new_len.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i, align 8, !alias.scope !950, !noalias !951
; invoke <&mut walkdir::DirList as core::iter::traits::iterator::Iterator>::next
  invoke fastcc void @_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtCsarYwbBXrH4d_7walkdir7DirListNtB5_8Iterator4nextBS_(ptr noalias noundef align 8 captures(address) dereferenceable(56) %_3.i.i.i.i, ptr nonnull align 8 dereferenceable(64) %_10)
          to label %.noexc7.i.i unwind label %cleanup3.loopexit.i.i, !noalias !935

.noexc7.i.i:                                      ; preds = %bb8.i.i.i.i
  %20 = load i64, ptr %_3.i.i.i.i, align 8, !range !936, !noalias !948, !noundef !12
  %.not.i.i.i.i = icmp eq i64 %20, -9223372036854775806
  br i1 %.not.i.i.i.i, label %bb5.i.loopexit.i, label %bb3.i.i.i.i

cleanup2.i.i.i.i:                                 ; preds = %bb1.i.i.i.i.i
  %21 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef align 8 dereferenceable(56) %_3.i.i.i.i) #28
          to label %cleanup3.body.i.i unwind label %terminate.i.i.i.i, !noalias !952

bb1.i.i.i.i.i:                                    ; preds = %bb3.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i, i64 noundef %len.i.i.i.i, i64 noundef range(i64 1, 0) 1)
          to label %bb1.i.i.i.bb8.i.i_crit_edge.i.i unwind label %cleanup2.i.i.i.i

bb1.i.i.i.bb8.i.i_crit_edge.i.i:                  ; preds = %bb1.i.i.i.i.i
  %_23.i.i.pre.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i, align 8, !alias.scope !950, !noalias !951
  br label %bb8.i.i.i.i

terminate.i.i.i.i:                                ; preds = %cleanup2.i.i.i.i
  %22 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !952
  unreachable

cleanup3.loopexit.i.i:                            ; preds = %bb8.i.i.i.i
  %lpad.loopexit.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup3.body.i.i

cleanup3.loopexit.split-lp.i.i:                   ; preds = %bb15.i.i
  %lpad.loopexit.split-lp.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup3.body.i.i

cleanup3.body.i.i:                                ; preds = %cleanup3.loopexit.split-lp.i.i, %cleanup3.loopexit.i.i, %cleanup2.i.i.i.i
  %eh.lpad-body.i.i = phi { ptr, i32 } [ %21, %cleanup2.i.i.i.i ], [ %lpad.loopexit.i.i, %cleanup3.loopexit.i.i ], [ %lpad.loopexit.split-lp.i.i, %cleanup3.loopexit.split-lp.i.i ]
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_(ptr noalias noundef align 8 dereferenceable(24) %vector.i.i) #28
          to label %common.resume unwind label %terminate.i.i, !noalias !935

bb5.i.loopexit.i:                                 ; preds = %.noexc7.i.i
  %_5.sroa.0.0.copyload.pre.i = load i64, ptr %vector.i.i, align 8, !noalias !953
  br label %bb5.i.i

bb5.i.i:                                          ; preds = %bb5.i.loopexit.i, %.noexc6.i.i
  %_5.sroa.5.0.copyload.i = phi i64 [ %new_len.i.i.i.i, %bb5.i.loopexit.i ], [ 1, %.noexc6.i.i ]
  %_5.sroa.3.0.copyload.i = phi ptr [ %_23.i.i.i.i, %bb5.i.loopexit.i ], [ %17, %.noexc6.i.i ]
  %_5.sroa.0.0.copyload.i = phi i64 [ %_5.sroa.0.0.copyload.pre.i, %bb5.i.loopexit.i ], [ 4, %.noexc6.i.i ]
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_3.i.i.i.i), !noalias !946
  br label %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_.exit.i

terminate.i.i:                                    ; preds = %cleanup3.body.i.i, %cleanup2.i.i
  %23 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !935
  unreachable

common.resume:                                    ; preds = %bb19, %cleanup1, %cleanup.i34, %bb10.i, %cleanup3.body.i, %cleanup.i13, %cleanup2.i.i, %cleanup3.body.i.i, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %24, %cleanup.i ], [ %eh.lpad-body.i.i, %cleanup3.body.i.i ], [ %16, %cleanup2.i.i ], [ %26, %cleanup.i13 ], [ %eh.lpad-body5177, %bb19 ], [ %73, %cleanup1 ], [ %75, %cleanup.i34 ], [ %eh.lpad-body19.i, %cleanup3.body.i ], [ %.pn.ph.i, %bb10.i ]
  resume { ptr, i32 } %common.resume.op

_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_.exit.i: ; preds = %bb5.i.i, %bb1.i
  %_5.sroa.5.0.i = phi i64 [ %_5.sroa.5.0.copyload.i, %bb5.i.i ], [ 0, %bb1.i ]
  %_5.sroa.3.0.i = phi ptr [ %_5.sroa.3.0.copyload.i, %bb5.i.i ], [ inttoptr (i64 8 to ptr), %bb1.i ]
  %_5.sroa.0.0.i = phi i64 [ %_5.sroa.0.0.copyload.i, %bb5.i.i ], [ 0, %bb1.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i), !noalias !931
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_3.i.i), !noalias !928
  %_14.i = icmp samesign ult i64 %_5.sroa.5.0.i, 164703072086692426
  tail call void @llvm.assume(i1 %_14.i)
  %_9.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_5.sroa.3.0.i, i64 %_5.sroa.5.0.i
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_10)
          to label %bb2.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_.exit.i
  %24 = landingpad { ptr, i32 }
          cleanup
  store i64 -9223372036854775805, ptr %_10, align 8, !alias.scope !928
  %_3.sroa.5.0.self.sroa_idx2.i = getelementptr inbounds nuw i8, ptr %_10, i64 8
  store ptr %_5.sroa.3.0.i, ptr %_3.sroa.5.0.self.sroa_idx2.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.5.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 16
  store ptr %_5.sroa.3.0.i, ptr %_3.sroa.5.sroa.5.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.6.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 24
  store i64 %_5.sroa.0.0.i, ptr %_3.sroa.5.sroa.6.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.7.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 32
  store ptr %_9.i, ptr %_3.sroa.5.sroa.7.0._3.sroa.5.0.self.sroa_idx2.sroa_idx.i, align 8, !alias.scope !928
  br label %common.resume

bb2.i:                                            ; preds = %_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_.exit.i
  store i64 -9223372036854775805, ptr %_10, align 8, !alias.scope !928
  %_3.sroa.5.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 8
  store ptr %_5.sroa.3.0.i, ptr %_3.sroa.5.0.self.sroa_idx.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.5.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 16
  store ptr %_5.sroa.3.0.i, ptr %_3.sroa.5.sroa.5.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.6.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 24
  store i64 %_5.sroa.0.0.i, ptr %_3.sroa.5.sroa.6.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i, align 8, !alias.scope !928
  %_3.sroa.5.sroa.7.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10, i64 32
  store ptr %_9.i, ptr %_3.sroa.5.sroa.7.0._3.sroa.5.0.self.sroa_idx.sroa_idx.i, align 8, !alias.scope !928
  br label %bb3

panic:                                            ; preds = %bb1
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_6, i64 noundef %_5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fe0796cedbf57eec1a1715554c92530a) #32
  unreachable

bb25:                                             ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err.i)
  store ptr %t.0.sink.i, ptr %err.i, align 8, !noalias !954
  %25 = getelementptr inbounds nuw i8, ptr %0, i64 168
  %_4.i = load i64, ptr %25, align 8, !noalias !954, !noundef !12
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !954
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_61, i64 noundef %_60)
          to label %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit unwind label %cleanup.i13, !noalias !954

cleanup.i13:                                      ; preds = %bb25
  %26 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %err.i) #28
          to label %common.resume unwind label %terminate.i, !noalias !954

terminate.i:                                      ; preds = %cleanup.i13
  %27 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !954
  unreachable

_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit: ; preds = %bb25
  %_66.sroa.4.8.copyload = load ptr, ptr %_5.i, align 8
  %_66.sroa.6.8._5.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %_66.sroa.6.8.copyload = load i8, ptr %_66.sroa.6.8._5.i.sroa_idx, align 8
  %_66.sroa.7.8._5.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_5.i, i64 9
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(15) %rd.sroa.5.sroa.0, ptr noundef nonnull align 1 dereferenceable(15) %_66.sroa.7.8._5.i.sroa_idx, i64 15, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !954
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err.i)
  br label %bb24

bb24:                                             ; preds = %bb3, %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit
  %rd.sroa.5.sroa.4.0 = phi i64 [ %_4.i, %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit ], [ undef, %bb3 ]
  %rd.sroa.4.0 = phi i8 [ %_66.sroa.6.8.copyload, %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit ], [ %10, %bb3 ]
  %rd.sroa.3.0 = phi ptr [ %_66.sroa.4.8.copyload, %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit ], [ %t.0.sink.i, %bb3 ]
  %rd.sroa.0.0 = phi i64 [ -9223372036854775808, %_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_.exit ], [ -9223372036854775806, %bb3 ]
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %list)
  %28 = getelementptr inbounds nuw i8, ptr %0, i64 168
  %_19 = load i64, ptr %28, align 8, !noundef !12
  %29 = getelementptr inbounds nuw i8, ptr %list, i64 56
  store i64 %_19, ptr %29, align 8
  store i64 %rd.sroa.0.0, ptr %list, align 8
  %rd.sroa.3.0.list.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 8
  store ptr %rd.sroa.3.0, ptr %rd.sroa.3.0.list.sroa_idx, align 8
  %rd.sroa.4.0.list.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 16
  store i8 %rd.sroa.4.0, ptr %rd.sroa.4.0.list.sroa_idx, align 8
  %rd.sroa.5.0.list.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 17
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(15) %rd.sroa.5.0.list.sroa_idx, ptr noundef nonnull align 1 dereferenceable(15) %rd.sroa.5.sroa.0, i64 15, i1 false)
  %rd.sroa.5.sroa.2.0.rd.sroa.5.0.list.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 32
  store ptr %t.0.sink.i, ptr %rd.sroa.5.sroa.2.0.rd.sroa.5.0.list.sroa_idx.sroa_idx, align 8
  %rd.sroa.5.sroa.4.0.rd.sroa.5.0.list.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 48
  store i64 %rd.sroa.5.sroa.4.0, ptr %rd.sroa.5.sroa.4.0.rd.sroa.5.0.list.sroa_idx.sroa_idx, align 8
  %30 = load ptr, ptr %5, align 8, !align !101, !noundef !12
  %.not = icmp eq ptr %30, null
  br i1 %.not, label %bb7, label %bb5

bb5:                                              ; preds = %bb24
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %cmp)
  store ptr %5, ptr %cmp, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %entries)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_23, ptr noundef nonnull align 8 dereferenceable(64) %list, i64 64, i1 false)
  call void @llvm.experimental.noalias.scope.decl(metadata !957)
  call void @llvm.experimental.noalias.scope.decl(metadata !960)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i), !noalias !962
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.sroa.11.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !963)
  %31 = load i64, ptr %_23, align 8, !range !49, !alias.scope !966, !noalias !967, !noundef !12
  %32 = icmp eq i64 %31, -9223372036854775805
  br i1 %32, label %bb3.i.i, label %bb2.i.i

bb3.i.i:                                          ; preds = %bb5
  call void @llvm.experimental.noalias.scope.decl(metadata !969)
  %_14.i.i = getelementptr inbounds nuw i8, ptr %_23, i64 32
  %_12.i.i = load ptr, ptr %_14.i.i, align 8, !alias.scope !972, !noalias !973, !nonnull !12, !noundef !12
  %33 = getelementptr inbounds nuw i8, ptr %_23, i64 16
  %_21.i.i = load ptr, ptr %33, align 8, !alias.scope !972, !noalias !973, !nonnull !12, !noundef !12
  %_9.i.i = icmp eq ptr %_21.i.i, %_12.i.i
  br i1 %_9.i.i, label %bb28.thread, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb3.i.i
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i, i64 56
  store ptr %_23.i.i, ptr %33, align 8, !alias.scope !972, !noalias !973
  %_3.sroa.0.0.copyload22.i = load i64, ptr %_21.i.i, align 8, !noalias !975
  %_3.sroa.11.0._21.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_21.i.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i, ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.0._21.i.sroa_idx.i, i64 48, i1 false), !noalias !975
  br label %bb1.i19

bb2.i.i:                                          ; preds = %bb5
  %34 = getelementptr inbounds nuw i8, ptr %_23, i64 56
  %35 = load i64, ptr %34, align 8, !alias.scope !966, !noalias !967, !noundef !12
  %.not.i.i17 = icmp eq i64 %31, -9223372036854775806
  br i1 %.not.i.i17, label %bb4.i.i, label %bb5.i.i18

bb5.i.i18:                                        ; preds = %bb2.i.i
  store i64 -9223372036854775807, ptr %_23, align 8, !alias.scope !966, !noalias !967
  %.not2.i.i = icmp eq i64 %31, -9223372036854775807
  br i1 %.not2.i.i, label %bb28.thread, label %bb1.i19.thread61

bb4.i.i:                                          ; preds = %bb2.i.i
  %rd.i.i = getelementptr inbounds nuw i8, ptr %_23, i64 8
  call void @llvm.lifetime.start.p0(i64 1064, ptr nonnull %_10.i.i), !noalias !976
; invoke <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXsB_NtCs5sEH5CPMdak_3std2fsNtB5_7ReadDirNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([1064 x i8]) align 8 captures(none) dereferenceable(1064) %_10.i.i, ptr noalias noundef nonnull align 8 dereferenceable(16) %rd.i.i)
          to label %.noexc6.i unwind label %cleanup.i26, !noalias !957

.noexc6.i:                                        ; preds = %bb4.i.i
  %_17.i.i = load i64, ptr %_10.i.i, align 8, !range !687, !noalias !976, !noundef !12
  %36 = trunc nuw i64 %_17.i.i to i1
  br i1 %36, label %bb14.i.i, label %bb12.i.i

bb14.i.i:                                         ; preds = %.noexc6.i
  %37 = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 8
  %_18.i.sroa.0.0.copyload.i = load ptr, ptr %37, align 8, !noalias !976
  %_18.i.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 16
  %_18.i.sroa.4.0.copyload.i = load ptr, ptr %_18.i.sroa.4.0..sroa_idx.i, align 8, !noalias !976
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_19.i.i), !noalias !976
  call void @llvm.experimental.noalias.scope.decl(metadata !977)
  %38 = icmp eq ptr %_18.i.sroa.0.0.copyload.i, null
  %_10.i10.i = add i64 %35, 1
  br i1 %38, label %bb2.i14.i, label %bb3.i11.i

bb2.i14.i:                                        ; preds = %bb14.i.i
  %39 = icmp ne ptr %_18.i.sroa.4.0.copyload.i, null
  call void @llvm.assume(i1 %39)
  store <2 x i64> splat (i64 -9223372036854775808), ptr %_19.i.i, align 16, !alias.scope !977, !noalias !980
  %_9.sroa.0.sroa.5.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 32
  store ptr %_18.i.sroa.4.0.copyload.i, ptr %_9.sroa.0.sroa.5.0._0.sroa_idx.i.i, align 16, !alias.scope !977, !noalias !980
  %_9.sroa.4.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 48
  store i64 %_10.i10.i, ptr %_9.sroa.4.0._0.sroa_idx.i.i, align 16, !alias.scope !977, !noalias !980
  br label %.noexc7.i

bb3.i11.i:                                        ; preds = %bb14.i.i
  %_18.i.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 24
  call void @llvm.lifetime.start.p0(i64 1056, ptr nonnull %r1.i.i), !noalias !982
  store ptr %_18.i.sroa.0.0.copyload.i, ptr %r1.i.i, align 8, !noalias !983
  %_20.i.sroa.5.0.r1.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %r1.i.i, i64 8
  store ptr %_18.i.sroa.4.0.copyload.i, ptr %_20.i.sroa.5.0.r1.i.sroa_idx.i, align 8, !noalias !983
  %_20.i.sroa.6.0.r1.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %r1.i.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(1040) %_20.i.sroa.6.0.r1.i.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(1040) %_18.i.sroa.5.0..sroa_idx.i, i64 1040, i1 false), !noalias !962
; invoke <walkdir::dent::DirEntry>::from_entry
  invoke void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry10from_entry(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_19.i.i, i64 noundef %_10.i10.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %r1.i.i)
          to label %bb4.i12.i unwind label %cleanup.i.i, !noalias !984

cleanup.i.i:                                      ; preds = %bb3.i11.i
  %40 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !985)
  call void @llvm.experimental.noalias.scope.decl(metadata !988)
  call void @llvm.experimental.noalias.scope.decl(metadata !991)
  call void @llvm.experimental.noalias.scope.decl(metadata !994)
  %_10.i.i.i.i.i.i = load ptr, ptr %r1.i.i, align 8, !alias.scope !997, !noalias !982, !nonnull !12, !noundef !12
  %41 = atomicrmw sub ptr %_10.i.i.i.i.i.i, i64 1 release, align 8, !noalias !998
  %42 = icmp eq i64 %41, 1
  br i1 %42, label %bb2.i.i.i.i.i.i, label %bb10.i

bb2.i.i.i.i.i.i:                                  ; preds = %cleanup.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i.i) #30
          to label %bb10.i unwind label %terminate.i.i27, !noalias !999

bb4.i12.i:                                        ; preds = %bb3.i11.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1000)
  call void @llvm.experimental.noalias.scope.decl(metadata !1003)
  call void @llvm.experimental.noalias.scope.decl(metadata !1006)
  call void @llvm.experimental.noalias.scope.decl(metadata !1009)
  %_10.i.i.i.i2.i.i = load ptr, ptr %r1.i.i, align 8, !alias.scope !1012, !noalias !982, !nonnull !12, !noundef !12
  %43 = atomicrmw sub ptr %_10.i.i.i.i2.i.i, i64 1 release, align 8, !noalias !1013
  %44 = icmp eq i64 %43, 1
  br i1 %44, label %bb2.i.i.i.i3.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i

bb2.i.i.i.i3.i.i:                                 ; preds = %bb4.i12.i
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i.i) #30
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i unwind label %cleanup.i26, !noalias !957

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i: ; preds = %bb2.i.i.i.i3.i.i, %bb4.i12.i
  call void @llvm.lifetime.end.p0(i64 1056, ptr nonnull %r1.i.i), !noalias !982
  %_3.sroa.0.0.copyload21.pre.i = load i64, ptr %_19.i.i, align 16, !noalias !1014
  br label %.noexc7.i

terminate.i.i27:                                  ; preds = %bb2.i.i.i.i.i.i
  %45 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !999
  unreachable

.noexc7.i:                                        ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i, %bb2.i14.i
  %_3.sroa.0.0.copyload21.i = phi i64 [ %_3.sroa.0.0.copyload21.pre.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i ], [ -9223372036854775808, %bb2.i14.i ]
  %_3.sroa.11.0._19.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i, ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.0._19.i.sroa_idx.i, i64 48, i1 false), !noalias !1014
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_19.i.i), !noalias !976
  br label %bb12.i.i

bb12.i.i:                                         ; preds = %.noexc7.i, %.noexc6.i
  %_3.sroa.0.2.i = phi i64 [ %_3.sroa.0.0.copyload21.i, %.noexc7.i ], [ -9223372036854775806, %.noexc6.i ]
  call void @llvm.lifetime.end.p0(i64 1064, ptr nonnull %_10.i.i), !noalias !976
  br label %bb1.i19

bb1.i19.thread61:                                 ; preds = %bb5.i.i18
  %46 = getelementptr inbounds nuw i8, ptr %list, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i, ptr noundef nonnull align 8 dereferenceable(48) %46, i64 48, i1 false)
  br label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i

cleanup.i26:                                      ; preds = %bb2.i.i.i.i3.i.i, %bb4.i.i
  %47 = landingpad { ptr, i32 }
          cleanup
  br label %bb10.i

bb1.i19:                                          ; preds = %bb12.i.i, %bb6.i.i
  %_3.sroa.0.3.i = phi i64 [ %_3.sroa.0.0.copyload22.i, %bb6.i.i ], [ %_3.sroa.0.2.i, %bb12.i.i ]
  %.not.i = icmp eq i64 %_3.sroa.0.3.i, -9223372036854775806
  br i1 %.not.i, label %bb28.thread, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i

bb28.thread:                                      ; preds = %bb1.i19, %bb3.i.i, %bb5.i.i18
  store i64 0, ptr %entries, align 8, !alias.scope !957, !noalias !960
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.sroa.11.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i), !noalias !962
; call core::ptr::drop_in_place::<walkdir::DirList>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_23)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %compare.i)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %is_less.i.i), !noalias !1015
  br label %bb6

cleanup2.i:                                       ; preds = %bb3.i17.i
  %48 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef align 8 dereferenceable(56) %element.i) #28
          to label %bb10.i unwind label %terminate.i22, !noalias !957

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i: ; preds = %bb1.i19.thread61, %bb1.i19
  %_3.sroa.0.3.i64 = phi i64 [ %31, %bb1.i19.thread61 ], [ %_3.sroa.0.3.i, %bb1.i19 ]
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %element.i), !noalias !962
  store i64 %_3.sroa.0.3.i64, ptr %element.i, align 8, !noalias !962
  %_3.sroa.11.0.element.sroa_idx.i = getelementptr inbounds nuw i8, ptr %element.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.0.element.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i, i64 48, i1 false), !noalias !962
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !1019
; call __rustc::__rust_alloc
  %49 = call noundef align 8 dereferenceable_or_null(224) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 224, i64 noundef range(i64 1, 9) 8) #26, !noalias !1019
  %50 = icmp eq ptr %49, null
  br i1 %50, label %bb3.i17.i, label %bb15.i

bb3.i17.i:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 8, i64 224) #27
          to label %.noexc18.i unwind label %cleanup2.i, !noalias !957

.noexc18.i:                                       ; preds = %bb3.i17.i
  unreachable

bb15.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %49, ptr noundef nonnull align 8 dereferenceable(56) %element.i, i64 56, i1 false), !noalias !957
  store i64 4, ptr %vector.i, align 8, !noalias !962
  %vector1.sroa.4.0.vector.sroa_idx.i = getelementptr inbounds nuw i8, ptr %vector.i, i64 8
  store ptr %49, ptr %vector1.sroa.4.0.vector.sroa_idx.i, align 8, !noalias !962
  %vector1.sroa.6.0.vector.sroa_idx.i = getelementptr inbounds nuw i8, ptr %vector.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i, align 8, !noalias !962
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %element.i), !noalias !962
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.sroa.11.i)
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_19.i), !noalias !962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_19.i, ptr noundef nonnull align 8 dereferenceable(64) %_23, i64 64, i1 false), !noalias !957
  call void @llvm.experimental.noalias.scope.decl(metadata !1022)
  call void @llvm.experimental.noalias.scope.decl(metadata !1025)
  call void @llvm.experimental.noalias.scope.decl(metadata !1027)
  call void @llvm.experimental.noalias.scope.decl(metadata !1030)
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %element.i.i.i), !noalias !1032
  %51 = getelementptr inbounds nuw i8, ptr %_19.i, i64 56
  %_8.sroa.5.0.self.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i, i64 8
  %52 = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i, i64 8
  %_18.i.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i, i64 16
  %_18.i.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i.i, i64 24
  %_20.i.sroa.5.0.r1.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %r1.i.i.i.i, i64 8
  %_20.i.sroa.6.0.r1.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %r1.i.i.i.i, i64 16
  %_9.sroa.0.sroa.4.0._0.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i16, i64 8
  %_9.sroa.0.sroa.5.0._0.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i16, i64 32
  %_9.sroa.4.0._0.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i.i16, i64 48
  %_14.i.i.i.i = getelementptr inbounds nuw i8, ptr %_19.i, i64 32
  %53 = getelementptr inbounds nuw i8, ptr %_19.i, i64 16
  %_3.sroa.11.0.element.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %element.i.i.i, i64 8
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb8.i.i.i, %bb15.i
  %len.i.i33.i = phi i64 [ %new_len.i.i.i, %bb8.i.i.i ], [ 1, %bb15.i ]
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3.sroa.11.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !1033)
  %54 = load i64, ptr %_19.i, align 8, !range !49, !alias.scope !1036, !noalias !1037, !noundef !12
  %55 = icmp eq i64 %54, -9223372036854775805
  br i1 %55, label %bb3.i.i.i.i24, label %bb2.i.i.i.i

bb3.i.i.i.i24:                                    ; preds = %bb1.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1039)
  %_12.i.i.i.i = load ptr, ptr %_14.i.i.i.i, align 8, !alias.scope !1042, !noalias !1043, !nonnull !12, !noundef !12
  %_21.i.i.i.i = load ptr, ptr %53, align 8, !alias.scope !1042, !noalias !1043, !nonnull !12, !noundef !12
  %_9.i.i.i.i = icmp eq ptr %_21.i.i.i.i, %_12.i.i.i.i
  br i1 %_9.i.i.i.i, label %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb3.i.i.i.i24
  %_23.i.i.i.i25 = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i, i64 56
  store ptr %_23.i.i.i.i25, ptr %53, align 8, !alias.scope !1042, !noalias !1043
  %_3.sroa.0.0.copyload19.i.i.i = load i64, ptr %_21.i.i.i.i, align 8, !noalias !1045
  %_3.sroa.11.0._21.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i.i.i, ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.0._21.i.sroa_idx.i.i.i, i64 48, i1 false), !noalias !1045
  br label %bb2.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb1.i.i.i
  %56 = load i64, ptr %51, align 8, !alias.scope !1036, !noalias !1037, !noundef !12
  %.not.i.i.i.i20 = icmp eq i64 %54, -9223372036854775806
  br i1 %.not.i.i.i.i20, label %bb4.i.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb2.i.i.i.i
  store i64 -9223372036854775807, ptr %_19.i, align 8, !alias.scope !1036, !noalias !1037
  %.not2.i.i.i.i = icmp eq i64 %54, -9223372036854775807
  br i1 %.not2.i.i.i.i, label %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i, label %bb2.i.thread3.i.i

bb4.i.i.i.i:                                      ; preds = %bb2.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 1064, ptr nonnull %_10.i.i.i.i), !noalias !1046
; invoke <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXsB_NtCs5sEH5CPMdak_3std2fsNtB5_7ReadDirNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([1064 x i8]) align 8 captures(none) dereferenceable(1064) %_10.i.i.i.i, ptr noalias noundef nonnull align 8 dereferenceable(16) %_8.sroa.5.0.self.sroa_idx.i.i.i.i)
          to label %.noexc5.i.i.i unwind label %cleanup.i.i.i, !noalias !1047

.noexc5.i.i.i:                                    ; preds = %bb4.i.i.i.i
  %_17.i.i.i.i = load i64, ptr %_10.i.i.i.i, align 8, !range !687, !noalias !1046, !noundef !12
  %57 = trunc nuw i64 %_17.i.i.i.i to i1
  br i1 %57, label %bb14.i.i.i.i, label %bb12.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %.noexc5.i.i.i
  %_18.i.sroa.0.0.copyload.i.i.i = load ptr, ptr %52, align 8, !noalias !1046
  %_18.i.sroa.4.0.copyload.i.i.i = load ptr, ptr %_18.i.sroa.4.0..sroa_idx.i.i.i, align 8, !noalias !1046
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_19.i.i.i.i16), !noalias !1046
  call void @llvm.experimental.noalias.scope.decl(metadata !1048)
  %58 = icmp eq ptr %_18.i.sroa.0.0.copyload.i.i.i, null
  %_10.i9.i.i.i = add i64 %56, 1
  br i1 %58, label %bb2.i13.i.i.i, label %bb3.i10.i.i.i

bb2.i13.i.i.i:                                    ; preds = %bb14.i.i.i.i
  %59 = icmp ne ptr %_18.i.sroa.4.0.copyload.i.i.i, null
  call void @llvm.assume(i1 %59)
  store <2 x i64> splat (i64 -9223372036854775808), ptr %_19.i.i.i.i16, align 16, !alias.scope !1048, !noalias !1051
  store ptr %_18.i.sroa.4.0.copyload.i.i.i, ptr %_9.sroa.0.sroa.5.0._0.sroa_idx.i.i.i.i, align 16, !alias.scope !1048, !noalias !1051
  store i64 %_10.i9.i.i.i, ptr %_9.sroa.4.0._0.sroa_idx.i.i.i.i, align 16, !alias.scope !1048, !noalias !1051
  br label %.noexc6.i.i.i

bb3.i10.i.i.i:                                    ; preds = %bb14.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 1056, ptr nonnull %r1.i.i.i.i), !noalias !1053
  store ptr %_18.i.sroa.0.0.copyload.i.i.i, ptr %r1.i.i.i.i, align 8, !noalias !1054
  store ptr %_18.i.sroa.4.0.copyload.i.i.i, ptr %_20.i.sroa.5.0.r1.i.sroa_idx.i.i.i, align 8, !noalias !1054
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(1040) %_20.i.sroa.6.0.r1.i.sroa_idx.i.i.i, ptr noundef nonnull align 8 dereferenceable(1040) %_18.i.sroa.5.0..sroa_idx.i.i.i, i64 1040, i1 false), !noalias !1055
; invoke <walkdir::dent::DirEntry>::from_entry
  invoke void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry10from_entry(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_19.i.i.i.i16, i64 noundef %_10.i9.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %r1.i.i.i.i)
          to label %bb4.i11.i.i.i unwind label %cleanup.i.i.i.i, !noalias !1056

cleanup.i.i.i.i:                                  ; preds = %bb3.i10.i.i.i
  %60 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !1057)
  call void @llvm.experimental.noalias.scope.decl(metadata !1060)
  call void @llvm.experimental.noalias.scope.decl(metadata !1063)
  call void @llvm.experimental.noalias.scope.decl(metadata !1066)
  %_10.i.i.i.i.i.i.i.i = load ptr, ptr %r1.i.i.i.i, align 8, !alias.scope !1069, !noalias !1053, !nonnull !12, !noundef !12
  %61 = atomicrmw sub ptr %_10.i.i.i.i.i.i.i.i, i64 1 release, align 8, !noalias !1070
  %62 = icmp eq i64 %61, 1
  br i1 %62, label %bb2.i.i.i.i.i.i.i.i, label %bb11.i.i.i

bb2.i.i.i.i.i.i.i.i:                              ; preds = %cleanup.i.i.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i.i.i.i) #30
          to label %bb11.i.i.i unwind label %terminate.i.i.i.i23, !noalias !1071

bb4.i11.i.i.i:                                    ; preds = %bb3.i10.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1072)
  call void @llvm.experimental.noalias.scope.decl(metadata !1075)
  call void @llvm.experimental.noalias.scope.decl(metadata !1078)
  call void @llvm.experimental.noalias.scope.decl(metadata !1081)
  %_10.i.i.i.i2.i.i.i.i = load ptr, ptr %r1.i.i.i.i, align 8, !alias.scope !1084, !noalias !1053, !nonnull !12, !noundef !12
  %63 = atomicrmw sub ptr %_10.i.i.i.i2.i.i.i.i, i64 1 release, align 8, !noalias !1085
  %64 = icmp eq i64 %63, 1
  br i1 %64, label %bb2.i.i.i.i3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i.i.i

bb2.i.i.i.i3.i.i.i.i:                             ; preds = %bb4.i11.i.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i.i.i.i) #30
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i.i.i unwind label %cleanup.i.i.i, !noalias !1047

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i.i.i: ; preds = %bb2.i.i.i.i3.i.i.i.i, %bb4.i11.i.i.i
  call void @llvm.lifetime.end.p0(i64 1056, ptr nonnull %r1.i.i.i.i), !noalias !1053
  %_3.sroa.0.0.copyload18.pre.i.i.i = load i64, ptr %_19.i.i.i.i16, align 16, !noalias !1086
  br label %.noexc6.i.i.i

terminate.i.i.i.i23:                              ; preds = %bb2.i.i.i.i.i.i.i.i
  %65 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1071
  unreachable

.noexc6.i.i.i:                                    ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i.i.i, %bb2.i13.i.i.i
  %_3.sroa.0.0.copyload18.i.i.i = phi i64 [ %_3.sroa.0.0.copyload18.pre.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i.i.i.i ], [ -9223372036854775808, %bb2.i13.i.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i.i.i, ptr noundef nonnull align 8 dereferenceable(48) %_9.sroa.0.sroa.4.0._0.sroa_idx.i.i.i.i, i64 48, i1 false), !noalias !1086
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_19.i.i.i.i16), !noalias !1046
  br label %bb12.i.i.i.i

bb12.i.i.i.i:                                     ; preds = %.noexc6.i.i.i, %.noexc5.i.i.i
  %_3.sroa.0.2.i.i.i = phi i64 [ %_3.sroa.0.0.copyload18.i.i.i, %.noexc6.i.i.i ], [ -9223372036854775806, %.noexc5.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 1064, ptr nonnull %_10.i.i.i.i), !noalias !1046
  br label %bb2.i.i.i

bb2.i.thread3.i.i:                                ; preds = %bb5.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i.i.i, ptr noundef nonnull align 8 dereferenceable(48) %_8.sroa.5.0.self.sroa_idx.i.i.i.i, i64 48, i1 false), !noalias !1087
  br label %bb3.i.i.i21

bb11.i.i.i:                                       ; preds = %cleanup2.i.i.i, %cleanup.i.i.i, %bb2.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i
  %.pn.i.i.i = phi { ptr, i32 } [ %67, %cleanup2.i.i.i ], [ %66, %cleanup.i.i.i ], [ %60, %bb2.i.i.i.i.i.i.i.i ], [ %60, %cleanup.i.i.i.i ]
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_19.i) #28
          to label %cleanup3.body.i unwind label %terminate.i.i.i, !noalias !1047

cleanup.i.i.i:                                    ; preds = %bb2.i.i.i.i3.i.i.i.i, %bb4.i.i.i.i
  %66 = landingpad { ptr, i32 }
          cleanup
  br label %bb11.i.i.i

bb2.i.i.i:                                        ; preds = %bb12.i.i.i.i, %bb6.i.i.i.i
  %_3.sroa.0.3.i.i.i = phi i64 [ %_3.sroa.0.0.copyload19.i.i.i, %bb6.i.i.i.i ], [ %_3.sroa.0.2.i.i.i, %bb12.i.i.i.i ]
  %.not.i.i.i = icmp eq i64 %_3.sroa.0.3.i.i.i, -9223372036854775806
  br i1 %.not.i.i.i, label %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i, label %bb2.i.i.bb3.i.i_crit_edge.i

bb2.i.i.bb3.i.i_crit_edge.i:                      ; preds = %bb2.i.i.i
  %len.i.i.pre.i = load i64, ptr %vector1.sroa.6.0.vector.sroa_idx.i, align 8, !alias.scope !1088, !noalias !1089
  br label %bb3.i.i.i21

bb3.i.i.i21:                                      ; preds = %bb2.i.i.bb3.i.i_crit_edge.i, %bb2.i.thread3.i.i
  %len.i.i.i = phi i64 [ %len.i.i33.i, %bb2.i.thread3.i.i ], [ %len.i.i.pre.i, %bb2.i.i.bb3.i.i_crit_edge.i ]
  %_3.sroa.0.3.i6.i.i = phi i64 [ %54, %bb2.i.thread3.i.i ], [ %_3.sroa.0.3.i.i.i, %bb2.i.i.bb3.i.i_crit_edge.i ]
  store i64 %_3.sroa.0.3.i6.i.i, ptr %element.i.i.i, align 8, !noalias !1055
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.0.element.sroa_idx.i.i.i, ptr noundef nonnull align 8 dereferenceable(48) %_3.sroa.11.i.i.i, i64 48, i1 false), !noalias !1055
  %_19.i.i.i = icmp ult i64 %len.i.i.i, 164703072086692426
  call void @llvm.assume(i1 %_19.i.i.i)
  %self1.i.i.i = load i64, ptr %vector.i, align 8, !range !48, !alias.scope !1088, !noalias !1089, !noundef !12
  %_8.i.i.i = icmp eq i64 %len.i.i.i, %self1.i.i.i
  br i1 %_8.i.i.i, label %bb1.i.i.i.i, label %bb8.i.i.i

bb8.i.i.i:                                        ; preds = %bb1.i.i.i.i, %bb3.i.i.i21
  %_23.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i, align 8, !alias.scope !1088, !noalias !1089, !nonnull !12, !noundef !12
  %dst.i.i.i = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_23.i.i.i, i64 %len.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %dst.i.i.i, ptr noundef nonnull align 8 dereferenceable(56) %element.i.i.i, i64 56, i1 false), !noalias !1047
  %new_len.i.i.i = add nuw nsw i64 %len.i.i.i, 1
  store i64 %new_len.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i, align 8, !alias.scope !1088, !noalias !1089
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.sroa.11.i.i.i)
  br label %bb1.i.i.i

cleanup2.i.i.i:                                   ; preds = %bb1.i.i.i.i
  %67 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB16_5error5ErrorEEB16_(ptr noalias noundef align 8 dereferenceable(56) %element.i.i.i) #28
          to label %bb11.i.i.i unwind label %terminate.i.i.i, !noalias !1047

bb1.i.i.i.i:                                      ; preds = %bb3.i.i.i21
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i, i64 noundef %len.i.i.i, i64 noundef range(i64 1, 0) 1)
          to label %bb8.i.i.i unwind label %cleanup2.i.i.i

terminate.i.i.i:                                  ; preds = %cleanup2.i.i.i, %bb11.i.i.i
  %68 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1047
  unreachable

_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i: ; preds = %bb2.i.i.i, %bb5.i.i.i.i, %bb3.i.i.i.i24
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3.sroa.11.i.i.i)
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_19.i)
          to label %bb28 unwind label %cleanup3.i, !noalias !957

cleanup3.i:                                       ; preds = %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i
  %69 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup3.body.i

cleanup3.body.i:                                  ; preds = %cleanup3.i, %bb11.i.i.i
  %eh.lpad-body19.i = phi { ptr, i32 } [ %69, %cleanup3.i ], [ %.pn.i.i.i, %bb11.i.i.i ]
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_(ptr noalias noundef align 8 dereferenceable(24) %vector.i) #28
          to label %common.resume unwind label %terminate.i22, !noalias !957

terminate.i22:                                    ; preds = %bb10.i, %cleanup3.body.i, %cleanup2.i
  %70 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !957
  unreachable

bb10.i:                                           ; preds = %cleanup2.i, %cleanup.i26, %bb2.i.i.i.i.i.i, %cleanup.i.i
  %.pn.ph.i = phi { ptr, i32 } [ %48, %cleanup2.i ], [ %47, %cleanup.i26 ], [ %40, %bb2.i.i.i.i.i.i ], [ %40, %cleanup.i.i ]
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_23) #28
          to label %common.resume unwind label %terminate.i22, !noalias !957

bb7:                                              ; preds = %bb6, %bb24
  %71 = getelementptr inbounds nuw i8, ptr %0, i64 152
  %72 = load i8, ptr %71, align 8, !range !720, !noundef !12
  %_30 = trunc nuw i8 %72 to i1
  br i1 %_30, label %bb8, label %bb11

bb20:                                             ; preds = %bb8
  %lpad.thr_comm.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb19

bb28:                                             ; preds = %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_.exit.i.i
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %element.i.i.i), !noalias !1032
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %_19.i), !noalias !962
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %entries, ptr noundef nonnull align 8 dereferenceable(24) %vector.i, i64 24, i1 false), !noalias !960
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i), !noalias !962
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %entries, i64 8
  %_71.pre = load ptr, ptr %.phi.trans.insert, align 8
  %.phi.trans.insert87 = getelementptr inbounds nuw i8, ptr %entries, i64 16
  %_70.pre = load i64, ptr %.phi.trans.insert87, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %compare.i)
  store ptr %cmp, ptr %compare.i, align 8, !noalias !1015
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %is_less.i.i), !noalias !1015
  store ptr %compare.i, ptr %is_less.i.i, align 8, !noalias !1090
  %b.i.i = icmp samesign ult i64 %_70.pre, 2
  br i1 %b.i.i, label %bb6, label %bb7.i.i, !prof !1094

bb7.i.i:                                          ; preds = %bb28
  %b1.i.i = icmp samesign ult i64 %_70.pre, 21
  br i1 %b1.i.i, label %bb9.i.i28, label %bb10.i.i, !prof !904

bb10.i.i:                                         ; preds = %bb7.i.i
; invoke core::slice::sort::stable::driftsort_main::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}, alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
  invoke void @_RINvNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable14driftsort_mainINtNtB8_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1p_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBZ_7sort_byNCNvMs3_B1p_NtB1p_8IntoIter4pushs_0E0INtNtB2s_3vec3VecBZ_EEB1p_(ptr noalias noundef nonnull align 8 %_71.pre, i64 noundef range(i64 0, 164703072086692426) %_70.pre, ptr noalias noundef nonnull align 8 dereferenceable(8) %is_less.i.i)
          to label %bb6 unwind label %cleanup1

bb9.i.i28:                                        ; preds = %bb7.i.i
; invoke core::slice::sort::shared::smallsort::insertion_sort_shift_left::<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>, <[core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>]>::sort_by<<walkdir::IntoIter>::push::{closure#1}>::{closure#0}>
  invoke fastcc void @_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort25insertion_sort_shift_leftINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1M_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1m_7sort_byNCNvMs3_B1M_NtB1M_8IntoIter4pushs_0E0EB1M_(ptr noalias noundef nonnull align 8 %_71.pre, i64 noundef range(i64 0, 164703072086692426) %_70.pre, ptr nonnull align 8 dereferenceable(8) %compare.i)
          to label %bb6 unwind label %cleanup1

cleanup1:                                         ; preds = %bb9.i.i28, %bb10.i.i
  %73 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<core::result::Result<walkdir::dent::DirEntry, walkdir::error::Error>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_(ptr noalias noundef align 8 dereferenceable(24) %entries) #28
          to label %common.resume unwind label %terminate

bb6:                                              ; preds = %bb9.i.i28, %bb28.thread, %bb28, %bb10.i.i
  %_7193 = phi ptr [ inttoptr (i64 8 to ptr), %bb28.thread ], [ %_71.pre, %bb28 ], [ %_71.pre, %bb10.i.i ], [ %_71.pre, %bb9.i.i28 ]
  %_7092 = phi i64 [ 0, %bb28.thread ], [ %_70.pre, %bb28 ], [ %_70.pre, %bb10.i.i ], [ %_70.pre, %bb9.i.i28 ]
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %is_less.i.i), !noalias !1015
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %compare.i)
  %_98 = load i64, ptr %entries, align 8, !range !48, !noundef !12
  %_78 = icmp ult i64 %_7092, 164703072086692426
  call void @llvm.assume(i1 %_78)
  %_75 = getelementptr inbounds nuw %"core::result::Result<dent::DirEntry, error::Error>", ptr %_7193, i64 %_7092
  store i64 -9223372036854775805, ptr %list, align 8
  store ptr %_7193, ptr %rd.sroa.3.0.list.sroa_idx, align 8
  store ptr %_7193, ptr %rd.sroa.4.0.list.sroa_idx, align 8
  %_28.sroa.4.sroa.5.0._28.sroa.4.0.list.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %list, i64 24
  store i64 %_98, ptr %_28.sroa.4.sroa.5.0._28.sroa.4.0.list.sroa_idx.sroa_idx, align 8
  store ptr %_75, ptr %rd.sroa.5.sroa.2.0.rd.sroa.5.0.list.sroa_idx.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %entries)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %cmp)
  br label %bb7

terminate:                                        ; preds = %bb19, %cleanup1
  %74 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29
  unreachable

bb11:                                             ; preds = %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_.exit, %bb7
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %_38)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_38, ptr noundef nonnull align 8 dereferenceable(64) %list, i64 64, i1 false)
  call void @llvm.experimental.noalias.scope.decl(metadata !1095)
  %len.i = load i64, ptr %3, align 8, !alias.scope !1095, !noalias !1098, !noundef !12
  %self1.i = load i64, ptr %2, align 8, !range !48, !alias.scope !1095, !noalias !1098, !noundef !12
  %_4.i31 = icmp eq i64 %len.i, %self1.i
  br i1 %_4.i31, label %bb1.i33, label %bb32

bb1.i33:                                          ; preds = %bb11
; invoke <alloc::raw_vec::RawVec<walkdir::DirList>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsarYwbBXrH4d_7walkdir7DirListE8grow_oneBO_(ptr noalias noundef nonnull align 8 dereferenceable(24) %2)
          to label %bb32 unwind label %cleanup.i34, !noalias !1098

cleanup.i34:                                      ; preds = %bb1.i33
  %75 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef nonnull align 8 dereferenceable(64) %_38) #28
          to label %common.resume unwind label %terminate.i35, !noalias !1095

terminate.i35:                                    ; preds = %cleanup.i34
  %76 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1095
  unreachable

bb8:                                              ; preds = %bb7
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_33)
; invoke <std::path::Path>::to_path_buf
  invoke void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_33, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_61, i64 noundef %_60)
          to label %bb9 unwind label %bb20

bb9:                                              ; preds = %bb8
  %77 = load i64, ptr %_33, align 8, !range !11, !noundef !12
  %78 = icmp eq i64 %77, -9223372036854775808
  %79 = getelementptr inbounds nuw i8, ptr %_33, i64 8
  %_81 = load ptr, ptr %79, align 8
  br i1 %78, label %bb29, label %bb30

bb29:                                             ; preds = %bb9
  %_83 = load i64, ptr %28, align 8, !noundef !12
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_33)
  store <2 x i64> splat (i64 -9223372036854775808), ptr %_0, align 8
  %_90.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store ptr %_81, ptr %_90.sroa.4.0._0.sroa_idx, align 8
  %_90.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %_83, ptr %_90.sroa.6.0._0.sroa_idx, align 8
; call core::ptr::drop_in_place::<walkdir::DirList>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef align 8 dereferenceable(64) %list)
  br label %bb16

bb30:                                             ; preds = %bb9
  %_80.sroa.5.sroa.5.0._80.sroa.5.0._33.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_33, i64 16
  %_80.sroa.5.sroa.5.0.copyload = load i64, ptr %_80.sroa.5.sroa.5.0._80.sroa.5.0._33.sroa_idx.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_33)
  %_36 = getelementptr inbounds nuw i8, ptr %0, i64 40
  call void @llvm.experimental.noalias.scope.decl(metadata !1100)
  %80 = getelementptr inbounds nuw i8, ptr %0, i64 56
  %len.i41 = load i64, ptr %80, align 8, !alias.scope !1100, !noalias !1103, !noundef !12
  %self1.i42 = load i64, ptr %_36, align 8, !range !48, !alias.scope !1100, !noalias !1103, !noundef !12
  %_4.i43 = icmp eq i64 %len.i41, %self1.i42
  br i1 %_4.i43, label %bb1.i46, label %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_.exit

bb1.i46:                                          ; preds = %bb30
; invoke <alloc::raw_vec::RawVec<walkdir::Ancestor>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsarYwbBXrH4d_7walkdir8AncestorE8grow_oneBO_(ptr noalias noundef nonnull align 8 dereferenceable(24) %_36)
          to label %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_.exit unwind label %cleanup.i47, !noalias !1103

cleanup.i47:                                      ; preds = %bb1.i46
  %81 = landingpad { ptr, i32 }
          cleanup
  %82 = icmp eq i64 %77, 0
  br i1 %82, label %bb19, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %cleanup.i47
  %83 = icmp ne ptr %_81, null
  call void @llvm.assume(i1 %83)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_81, i64 noundef %77, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !1105
  br label %bb19

_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_.exit: ; preds = %bb30, %bb1.i46
  %84 = getelementptr inbounds nuw i8, ptr %0, i64 48
  %_14.i44 = load ptr, ptr %84, align 8, !alias.scope !1100, !noalias !1103, !nonnull !12, !noundef !12
  %end.i45 = getelementptr inbounds nuw %Ancestor, ptr %_14.i44, i64 %len.i41
  store i64 %77, ptr %end.i45, align 8, !noalias !1100
  %ancestor.sroa.3.0.end.i45.sroa_idx = getelementptr inbounds nuw i8, ptr %end.i45, i64 8
  store ptr %_81, ptr %ancestor.sroa.3.0.end.i45.sroa_idx, align 8, !noalias !1100
  %ancestor.sroa.5.0.end.i45.sroa_idx = getelementptr inbounds nuw i8, ptr %end.i45, i64 16
  store i64 %_80.sroa.5.sroa.5.0.copyload, ptr %ancestor.sroa.5.0.end.i45.sroa_idx, align 8, !noalias !1100
  %85 = add i64 %len.i41, 1
  store i64 %85, ptr %80, align 8, !alias.scope !1100, !noalias !1103
  br label %bb11

bb32:                                             ; preds = %bb1.i33, %bb11
  %86 = getelementptr inbounds nuw i8, ptr %0, i64 24
  %_14.i32 = load ptr, ptr %86, align 8, !alias.scope !1095, !noalias !1098, !nonnull !12, !noundef !12
  %end.i = getelementptr inbounds nuw %DirList, ptr %_14.i32, i64 %len.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %end.i, ptr noundef nonnull align 8 dereferenceable(64) %_38, i64 64, i1 false), !noalias !1095
  %87 = add i64 %len.i, 1
  store i64 %87, ptr %3, align 8, !alias.scope !1095, !noalias !1098
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %_38)
  %_40 = load i64, ptr %6, align 8, !noundef !12
  %_39 = icmp eq i64 %_47, %_40
  br i1 %_39, label %bb12, label %bb14

bb12:                                             ; preds = %bb32
  %_43 = load i64, ptr %4, align 8, !noundef !12
  %_93.1 = icmp eq i64 %_43, -1
  br i1 %_93.1, label %bb33, label %bb35, !prof !645

bb14:                                             ; preds = %bb32, %bb35
  store i64 -9223372036854775807, ptr %_0, align 8
  br label %bb16

bb35:                                             ; preds = %bb12
  %_93.0 = add nuw i64 %_43, 1
  store i64 %_93.0, ptr %4, align 8
  br label %bb14

bb33:                                             ; preds = %bb12
; call core::option::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3e766849636974ea221c0ef7c96d5e97) #27
  unreachable

bb16:                                             ; preds = %bb29, %bb14
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %list)
  ret void

bb19:                                             ; preds = %bb2.i.i.i4.i.i.i.i.i.i, %cleanup.i47, %bb20
  %eh.lpad-body5177 = phi { ptr, i32 } [ %lpad.thr_comm.split-lp, %bb20 ], [ %81, %cleanup.i47 ], [ %81, %bb2.i.i.i4.i.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<walkdir::DirList>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7DirListEBI_(ptr noalias noundef align 8 dereferenceable(64) %list) #28
          to label %common.resume unwind label %terminate
}

; <alloc::raw_vec::RawVec<walkdir::DirList>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsarYwbBXrH4d_7walkdir7DirListE8grow_oneBO_(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !48, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1108)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !1108
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !1108
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsarYwbBXrH4d_7walkdir(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 64)
  %_35.i = load i64, ptr %self3.i, align 8, !range !687, !noalias !1108, !noundef !12
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !11, !noalias !1108, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !1108
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1108
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #27
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !1108, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1108
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !1108
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !1108
  ret void
}

; <alloc::raw_vec::RawVec<walkdir::Ancestor>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsarYwbBXrH4d_7walkdir8AncestorE8grow_oneBO_(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !48, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1111)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !1111
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !1111
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsarYwbBXrH4d_7walkdir(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 24)
  %_35.i = load i64, ptr %self3.i, align 8, !range !687, !noalias !1111, !noundef !12
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !11, !noalias !1111, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !1111
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1111
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #27
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !1111, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1111
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !1111
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !1111
  ret void
}

; <alloc::raw_vec::RawVec<walkdir::dent::DirEntry>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryE8grow_oneBQ_(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !48, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1114)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !1114
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !1114
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsarYwbBXrH4d_7walkdir(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 48)
  %_35.i = load i64, ptr %self3.i, align 8, !range !687, !noalias !1114, !noundef !12
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !11, !noalias !1114, !noundef !12
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !1114
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1114
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #27
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !1114, !nonnull !12, !noundef !12
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !1114
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !1114
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !1114
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsarYwbBXrH4d_7walkdir(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 24, 65) %elem_layout.1) unnamed_addr #6 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %_17.i = add nuw nsw i64 %elem_layout.1, 7
  %new_size.i = and i64 %_17.i, 248
  %_24.i = add i64 %cap, -1
  %0 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %new_size.i, i64 %_24.i)
  %_27.0.i = extractvalue { i64, i1 } %0, 0
  %_27.1.i = extractvalue { i64, i1 } %0, 1
  %_32.i = icmp ugt i64 %_27.0.i, 9223372036854775800
  %or.cond.i = or i1 %_27.1.i, %_32.i
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !1117

bb11.i:                                           ; preds = %bb6.i
  %new_size2.i = add nuw i64 %_27.0.i, %elem_layout.1
  %_40.i = icmp ugt i64 %new_size2.i, 9223372036854775800
  br i1 %_40.i, label %bb11, label %bb14

bb14:                                             ; preds = %bb11.i
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb4.i.i11, label %bb3.i.i

bb14.thread:                                      ; preds = %start
  %2 = icmp eq i64 %self.0.val, 0
  br i1 %2, label %bb9, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb14.thread, %bb14
  %_27.sroa.7.01321 = phi i64 [ %new_size2.i, %bb14 ], [ 0, %bb14.thread ]
  %3 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %3)
  %alloc_size.i23 = mul nuw i64 %elem_layout.1, %self.0.val
  %cond.i.i = icmp uge i64 %_27.sroa.7.01321, %alloc_size.i23
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) 8, i64 noundef %_27.sroa.7.01321) #26
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26
; call __rustc::__rust_alloc
  %4 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) 8) #26
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %bb3.i.i
  %_27.sroa.7.012 = phi i64 [ %_27.sroa.7.01321, %bb3.i.i ], [ %new_size2.i, %bb4.i.i11 ]
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3.i.i ], [ %4, %bb4.i.i11 ]
  %5 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %5, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 8, ptr %6, align 8
  br label %bb11

bb9:                                              ; preds = %bb14.thread, %bb7
  %raw_ptr.i.i.pn31 = phi ptr [ %raw_ptr.i.i.pn, %bb7 ], [ inttoptr (i64 8 to ptr), %bb14.thread ]
  %_27.sroa.7.01230 = phi i64 [ %_27.sroa.7.012, %bb7 ], [ 0, %bb14.thread ]
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

; <walkdir::WalkDir>::sort_by_file_name
; Function Attrs: uwtable
define void @_RNvMs_CsarYwbBXrH4d_7walkdirNtB4_7WalkDir17sort_by_file_name(ptr dead_on_unwind noalias noundef writable writeonly sret([72 x i8]) align 8 captures(none) dereferenceable(72) %_0, ptr dead_on_return noalias noundef align 8 captures(none) dereferenceable(72) %self) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1118)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1121)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !1121, !noalias !1118, !align !101, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %.val4.i = load ptr, ptr %1, align 8, !alias.scope !1121, !noalias !1118
  %2 = icmp eq ptr %.val.i, null
  br i1 %2, label %_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_.exit, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
  %3 = icmp ne ptr %.val4.i, null
  tail call void @llvm.assume(i1 %3)
  %4 = load ptr, ptr %.val4.i, align 8, !invariant.load !12, !noalias !1123
  %.not.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i, label %bb3.i.i.i, label %is_not_null.i.i.i

is_not_null.i.i.i:                                ; preds = %bb2.i.i
  invoke void %4(ptr noundef nonnull %.val.i)
          to label %bb3.i.i.i unwind label %cleanup.i.i.i, !noalias !1123

bb3.i.i.i:                                        ; preds = %is_not_null.i.i.i, %bb2.i.i
  %5 = getelementptr inbounds nuw i8, ptr %.val4.i, i64 8
  %6 = load i64, ptr %5, align 8, !range !48, !invariant.load !12, !noalias !1123
  %7 = getelementptr inbounds nuw i8, ptr %.val4.i, i64 16
  %8 = load i64, ptr %7, align 8, !range !102, !invariant.load !12, !noalias !1123
  %9 = add i64 %8, -1
  %10 = icmp sgt i64 %9, -1
  tail call void @llvm.assume(i1 %10)
  %11 = icmp eq i64 %6, 0
  br i1 %11, label %_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i: ; preds = %bb3.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %6, i64 noundef range(i64 1, -9223372036854775807) %8) #26, !noalias !1123
  br label %_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_.exit

cleanup.i.i.i:                                    ; preds = %is_not_null.i.i.i
  %12 = landingpad { ptr, i32 }
          cleanup
  %13 = getelementptr inbounds nuw i8, ptr %.val4.i, i64 8
  %14 = load i64, ptr %13, align 8, !range !48, !invariant.load !12, !noalias !1123
  %15 = getelementptr inbounds nuw i8, ptr %.val4.i, i64 16
  %16 = load i64, ptr %15, align 8, !range !102, !invariant.load !12, !noalias !1123
  %17 = add i64 %16, -1
  %18 = icmp sgt i64 %17, -1
  tail call void @llvm.assume(i1 %18)
  %19 = icmp eq i64 %14, 0
  br i1 %19, label %cleanup1.body.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i: ; preds = %cleanup.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val.i, i64 noundef %14, i64 noundef range(i64 1, -9223372036854775807) %16) #26, !noalias !1123
  br label %cleanup1.body.i

cleanup1.body.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i, %cleanup.i.i.i
  store ptr inttoptr (i64 1 to ptr), ptr %0, align 8, !alias.scope !1121, !noalias !1118
  store ptr @vtable.0, ptr %1, align 8, !alias.scope !1121, !noalias !1118
; invoke core::ptr::drop_in_place::<walkdir::WalkDir>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsarYwbBXrH4d_7walkdir7WalkDirEBI_(ptr noalias noundef nonnull align 8 dereferenceable(72) %self) #28
          to label %bb5.i unwind label %terminate.i, !noalias !1118

terminate.i:                                      ; preds = %cleanup1.body.i
  %20 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1123
  unreachable

bb5.i:                                            ; preds = %cleanup1.body.i
  resume { ptr, i32 } %12

_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_.exit: ; preds = %start, %bb3.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i
  store ptr inttoptr (i64 1 to ptr), ptr %0, align 8, !alias.scope !1121, !noalias !1118
  store ptr @vtable.0, ptr %1, align 8, !alias.scope !1121, !noalias !1118
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %_0, ptr noundef nonnull align 8 dereferenceable(72) %self, i64 72, i1 false), !alias.scope !1123
  ret void
}

; <walkdir::WalkDirOptions as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXCsarYwbBXrH4d_7walkdirNtB2_14WalkDirOptionsNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %_13 = alloca [16 x i8], align 8
  %sorter_str = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %sorter_str)
  %0 = load ptr, ptr %self, align 8, !align !101, !noundef !12
  %.not = icmp eq ptr %0, null
  %spec.select = select i1 %.not, ptr @alloc_37d2e53432a03a1f90b3e7253015eaf9, ptr @alloc_f7dab50b19f9d07673ad709515025da0
  %spec.select1 = select i1 %.not, i64 4, i64 9
  store ptr %spec.select, ptr %sorter_str, align 8
  %1 = getelementptr inbounds nuw i8, ptr %sorter_str, i64 8
  store i64 %spec.select1, ptr %1, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_13)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_13, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d714d9dfd28cf6a9ad0e82cf8b1372ea, i64 noundef 14)
  %_15 = getelementptr inbounds nuw i8, ptr %self, i64 40
; call <core::fmt::builders::DebugStruct>::field
  %_11 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_308a25eec56a6cc6390bb5730f0fa3f5, i64 noundef 12, ptr noundef nonnull align 1 %_15, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1)
  %_17 = getelementptr inbounds nuw i8, ptr %self, i64 41
; call <core::fmt::builders::DebugStruct>::field
  %_10 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_11, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d71d874d004130da7ff6b3af219e7179, i64 noundef 16, ptr noundef nonnull align 1 %_17, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1)
  %_19 = getelementptr inbounds nuw i8, ptr %self, i64 16
; call <core::fmt::builders::DebugStruct>::field
  %_9 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_10, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_320b930fa94d45d4ab31770bc0e0cda7, i64 noundef 8, ptr noundef nonnull align 1 %_19, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2)
  %_21 = getelementptr inbounds nuw i8, ptr %self, i64 24
; call <core::fmt::builders::DebugStruct>::field
  %_8 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c105ca25c81a06e2a92ad52809bc2a8a, i64 noundef 9, ptr noundef nonnull align 1 %_21, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2)
  %_23 = getelementptr inbounds nuw i8, ptr %self, i64 32
; call <core::fmt::builders::DebugStruct>::field
  %_7 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_8, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_7c939dbf1615c4691bd3f4cfbc046ac7, i64 noundef 9, ptr noundef nonnull align 1 %_23, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2)
; call <core::fmt::builders::DebugStruct>::field
  %_6 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_2d58c8bbe4563c561819bb5666a21d2a, i64 noundef 6, ptr noundef nonnull align 1 %sorter_str, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3)
  %_27 = getelementptr inbounds nuw i8, ptr %self, i64 42
; call <core::fmt::builders::DebugStruct>::field
  %_5 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e2a0bf94f60c5c26117b4eff9977d604, i64 noundef 14, ptr noundef nonnull align 1 %_27, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1)
  %_29 = getelementptr inbounds nuw i8, ptr %self, i64 43
; call <core::fmt::builders::DebugStruct>::field
  %_4 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_eeb3957c73ae0b537beb090154f28b19, i64 noundef 16, ptr noundef nonnull align 1 %_29, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1)
; call <core::fmt::builders::DebugStruct>::finish
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %sorter_str)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_13)
  ret i1 %_0
}

; <walkdir::dent::DirEntry as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs0_NtCsarYwbBXrH4d_7walkdir4dentNtB5_8DirEntryNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #2 {
start:
  %args = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsG_NtCs5sEH5CPMdak_3std4pathNtB5_7PathBufNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt, ptr %_6.sroa.4.0..sroa_idx, align 8
  %_19.0 = load ptr, ptr %f, align 8, !nonnull !12, !align !101, !noundef !12
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_19.1 = load ptr, ptr %0, align 8, !nonnull !12, !align !132, !noundef !12
; call core::fmt::write
  %1 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_19.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_19.1, ptr noundef nonnull @alloc_15295448e71ad0a49794d2a0b60d405b, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  ret i1 %1
}

; <walkdir::error::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs0_NtCsarYwbBXrH4d_7walkdir5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(56) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %args1 = alloca [32 x i8], align 8
  %_22 = alloca [16 x i8], align 8
  %_20 = alloca [16 x i8], align 8
  %args = alloca [32 x i8], align 8
  %_10 = alloca [16 x i8], align 8
  %err = alloca [8 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb2, label %bb3

bb3:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_20)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_34 = load ptr, ptr %1, align 8, !nonnull !12, !noundef !12
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %_33 = load i64, ptr %2, align 8, !noundef !12
  store ptr %_34, ptr %_20, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_20, i64 8
  store i64 %_33, ptr %3, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_22)
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_86 = load ptr, ptr %4, align 8, !nonnull !12, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_85 = load i64, ptr %5, align 8, !noundef !12
  store ptr %_86, ptr %_22, align 8
  %6 = getelementptr inbounds nuw i8, ptr %_22, i64 8
  store i64 %_85, ptr %6, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args1)
  store ptr %_20, ptr %args1, align 8
  %_24.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_24.sroa.4.0..sroa_idx, align 8
  %7 = getelementptr inbounds nuw i8, ptr %args1, i64 16
  store ptr %_22, ptr %7, align 8
  %_25.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 24
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_25.sroa.4.0..sroa_idx, align 8
  %_108.0 = load ptr, ptr %f, align 8, !nonnull !12, !align !101, !noundef !12
  %8 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_108.1 = load ptr, ptr %8, align 8, !nonnull !12, !align !132, !noundef !12
; call core::fmt::write
  %9 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_108.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_108.1, ptr noundef nonnull @alloc_884522af716a825d46454297f5f5d715, ptr noundef nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_22)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_20)
  br label %bb6

bb2:                                              ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %11 = load i64, ptr %10, align 8, !range !11, !noundef !12
  %.not15 = icmp eq i64 %11, -9223372036854775808
  br i1 %.not15, label %bb5, label %bb4

bb4:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %err)
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 32
  store ptr %12, ptr %err, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_10)
  %13 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_44 = load ptr, ptr %13, align 8, !nonnull !12, !noundef !12
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_43 = load i64, ptr %14, align 8, !noundef !12
  store ptr %_44, ptr %_10, align 8
  %15 = getelementptr inbounds nuw i8, ptr %_10, i64 8
  store i64 %_43, ptr %15, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %_10, ptr %args, align 8
  %_13.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx, align 8
  %16 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %err, ptr %16, align 8
  %_14.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtB6_7Display3fmtCsarYwbBXrH4d_7walkdir, ptr %_14.sroa.4.0..sroa_idx, align 8
  %_66.0 = load ptr, ptr %f, align 8, !nonnull !12, !align !101, !noundef !12
  %17 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_66.1 = load ptr, ptr %17, align 8, !nonnull !12, !align !132, !noundef !12
; call core::fmt::write
  %18 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_66.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_66.1, ptr noundef nonnull @alloc_6c264a22652d2d2029a1b24c0820ee41, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_10)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %err)
  br label %bb6

bb5:                                              ; preds = %bb2
  %err2 = getelementptr inbounds nuw i8, ptr %self, i64 32
; call <std::io::error::Error as core::fmt::Display>::fmt
  %19 = tail call noundef zeroext i1 @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %err2, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb6:                                              ; preds = %bb3, %bb4, %bb5
  %_0.sroa.0.0.in = phi i1 [ %9, %bb3 ], [ %18, %bb4 ], [ %19, %bb5 ]
  ret i1 %_0.sroa.0.0.in
}

; <&mut walkdir::DirList as core::iter::traits::iterator::Iterator>::next
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtCsarYwbBXrH4d_7walkdir7DirListNtB5_8Iterator4nextBS_(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(56) %_0, ptr %self.0.val) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %r1.i = alloca [1056 x i8], align 8
  %_19.i = alloca [56 x i8], align 16
  %_10.i = alloca [1064 x i8], align 8
  %0 = icmp ne ptr %self.0.val, null
  tail call void @llvm.assume(i1 %0)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1124)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1127)
  %1 = load i64, ptr %self.0.val, align 8, !range !49, !alias.scope !1127, !noalias !1124, !noundef !12
  %2 = icmp eq i64 %1, -9223372036854775805
  br i1 %2, label %bb3.i, label %bb2.i

bb3.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1129)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1132)
  %_14.i = getelementptr inbounds nuw i8, ptr %self.0.val, i64 32
  %_12.i = load ptr, ptr %_14.i, align 8, !alias.scope !1132, !noalias !1129, !nonnull !12, !noundef !12
  %3 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 16
  %_21.i = load ptr, ptr %3, align 8, !alias.scope !1132, !noalias !1129, !nonnull !12, !noundef !12
  %_9.i = icmp eq ptr %_21.i, %_12.i
  br i1 %_9.i, label %bb5.i2, label %bb6.i

bb6.i:                                            ; preds = %bb3.i
  %_23.i = getelementptr inbounds nuw i8, ptr %_21.i, i64 56
  store ptr %_23.i, ptr %3, align 8, !alias.scope !1132, !noalias !1129
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %_21.i, i64 56, i1 false), !noalias !1132
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

bb5.i2:                                           ; preds = %bb3.i
  store i64 -9223372036854775806, ptr %_0, align 8, !alias.scope !1129, !noalias !1132
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

bb2.i:                                            ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 56
  %5 = load i64, ptr %4, align 8, !alias.scope !1127, !noalias !1124, !noundef !12
  %.not.i = icmp eq i64 %1, -9223372036854775806
  br i1 %.not.i, label %bb4.i, label %bb5.i

bb5.i:                                            ; preds = %bb2.i
  store i64 -9223372036854775807, ptr %self.0.val, align 8, !alias.scope !1127, !noalias !1124
  %.not2.i = icmp eq i64 %1, -9223372036854775807
  br i1 %.not2.i, label %bb9.i, label %bb11.i

bb4.i:                                            ; preds = %bb2.i
  %rd.i = getelementptr inbounds nuw i8, ptr %self.0.val, i64 8
  call void @llvm.lifetime.start.p0(i64 1064, ptr nonnull %_10.i), !noalias !1134
; call <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
  call void @_RNvXsB_NtCs5sEH5CPMdak_3std2fsNtB5_7ReadDirNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([1064 x i8]) align 8 captures(none) dereferenceable(1064) %_10.i, ptr noalias noundef nonnull align 8 dereferenceable(16) %rd.i), !noalias !1124
  %_17.i = load i64, ptr %_10.i, align 8, !range !687, !noalias !1134, !noundef !12
  %6 = trunc nuw i64 %_17.i to i1
  br i1 %6, label %bb14.i, label %bb13.i

bb14.i:                                           ; preds = %bb4.i
  %7 = getelementptr inbounds nuw i8, ptr %_10.i, i64 8
  %_18.i.sroa.0.0.copyload = load ptr, ptr %7, align 8, !noalias !1134
  %_18.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10.i, i64 16
  %_18.i.sroa.4.0.copyload = load ptr, ptr %_18.i.sroa.4.0..sroa_idx, align 8, !noalias !1134
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_19.i), !noalias !1134
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1135)
  %8 = icmp eq ptr %_18.i.sroa.0.0.copyload, null
  %_10.i3 = add i64 %5, 1
  br i1 %8, label %bb2.i7, label %bb3.i4

bb2.i7:                                           ; preds = %bb14.i
  %9 = icmp ne ptr %_18.i.sroa.4.0.copyload, null
  tail call void @llvm.assume(i1 %9)
  store <2 x i64> splat (i64 -9223372036854775808), ptr %_19.i, align 16, !alias.scope !1135, !noalias !1138
  %_9.sroa.0.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i, i64 32
  store ptr %_18.i.sroa.4.0.copyload, ptr %_9.sroa.0.sroa.5.0._0.sroa_idx.i, align 16, !alias.scope !1135, !noalias !1138
  %_9.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i, i64 48
  store i64 %_10.i3, ptr %_9.sroa.4.0._0.sroa_idx.i, align 16, !alias.scope !1135, !noalias !1138
  br label %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit

bb3.i4:                                           ; preds = %bb14.i
  %_18.i.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10.i, i64 24
  call void @llvm.lifetime.start.p0(i64 1056, ptr nonnull %r1.i), !noalias !1140
  store ptr %_18.i.sroa.0.0.copyload, ptr %r1.i, align 8, !noalias !1135
  %_20.i.sroa.5.0.r1.i.sroa_idx = getelementptr inbounds nuw i8, ptr %r1.i, i64 8
  store ptr %_18.i.sroa.4.0.copyload, ptr %_20.i.sroa.5.0.r1.i.sroa_idx, align 8, !noalias !1135
  %_20.i.sroa.6.0.r1.i.sroa_idx = getelementptr inbounds nuw i8, ptr %r1.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(1040) %_20.i.sroa.6.0.r1.i.sroa_idx, ptr noundef nonnull align 8 dereferenceable(1040) %_18.i.sroa.5.0..sroa_idx, i64 1040, i1 false)
; invoke <walkdir::dent::DirEntry>::from_entry
  invoke void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry10from_entry(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_19.i, i64 noundef %_10.i3, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %r1.i)
          to label %bb4.i5 unwind label %cleanup.i, !noalias !1138

cleanup.i:                                        ; preds = %bb3.i4
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = atomicrmw sub ptr %_18.i.sroa.0.0.copyload, i64 1 release, align 8, !noalias !1141
  %12 = icmp eq i64 %11, 1
  br i1 %12, label %bb2.i.i.i.i.i, label %bb8.i

bb2.i.i.i.i.i:                                    ; preds = %cleanup.i
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i) #30
          to label %bb8.i unwind label %terminate.i, !noalias !1140

bb4.i5:                                           ; preds = %bb3.i4
  %13 = atomicrmw sub ptr %_18.i.sroa.0.0.copyload, i64 1 release, align 8, !noalias !1150
  %14 = icmp eq i64 %13, 1
  br i1 %14, label %bb2.i.i.i.i3.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i

bb2.i.i.i.i3.i:                                   ; preds = %bb4.i5
  fence acquire
; call <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i) #30, !noalias !1140
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i: ; preds = %bb2.i.i.i.i3.i, %bb4.i5
  call void @llvm.lifetime.end.p0(i64 1056, ptr nonnull %r1.i), !noalias !1140
  br label %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit

terminate.i:                                      ; preds = %bb2.i.i.i.i.i
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1140
  unreachable

bb8.i:                                            ; preds = %bb2.i.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %10

_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit: ; preds = %bb2.i7, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %_19.i, i64 56, i1 false), !noalias !1127
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_19.i), !noalias !1134
  br label %bb12.i

bb13.i:                                           ; preds = %bb4.i
  store i64 -9223372036854775806, ptr %_0, align 8, !alias.scope !1124, !noalias !1127
  br label %bb12.i

bb12.i:                                           ; preds = %bb13.i, %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit
  call void @llvm.lifetime.end.p0(i64 1064, ptr nonnull %_10.i), !noalias !1134
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

bb11.i:                                           ; preds = %bb5.i
  %_8.sroa.5.0.self.sroa_idx.i = getelementptr inbounds nuw i8, ptr %self.0.val, i64 8
  %_16.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 8
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_16.sroa.4.0._0.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %_8.sroa.5.0.self.sroa_idx.i, i64 48, i1 false)
  br label %bb9.i

bb9.i:                                            ; preds = %bb5.i, %bb11.i
  %.sink = phi i64 [ %1, %bb11.i ], [ -9223372036854775806, %bb5.i ]
  store i64 %.sink, ptr %_0, align 8, !alias.scope !1124, !noalias !1127
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit: ; preds = %bb12.i, %bb9.i, %bb5.i2, %bb6.i
  ret void
}

; <std::io::error::Error as core::convert::From<walkdir::error::Error>>::from
; Function Attrs: uwtable
define noundef nonnull ptr @_RNvXs1_NtCsarYwbBXrH4d_7walkdir5errorNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorINtNtCsjMrxcFdYDNN_4core7convert4FromNtB5_5ErrorE4from(ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(56) %walk_err) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %walk_err, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb3, label %bb4

bb3:                                              ; preds = %start
  %err = getelementptr inbounds nuw i8, ptr %walk_err, i64 32
  %err.val = load ptr, ptr %err, align 8, !nonnull !12, !noundef !12
  %bits.i.i = ptrtoint ptr %err.val to i64
  %_5.i.i = and i64 %bits.i.i, 3
  switch i64 %_5.i.i, label %default.unreachable [
    i64 2, label %bb5.i
    i64 3, label %bb4.i.i
    i64 0, label %bb2.i
    i64 1, label %bb4.i
  ], !prof !1159

default.unreachable:                              ; preds = %bb3
  unreachable

bb4.i.i:                                          ; preds = %bb3
  %_10.i.i = lshr i64 %bits.i.i, 32
  %switch.idx.cast = trunc i64 %_10.i.i to i8
  br label %bb4

bb5.i:                                            ; preds = %bb3
  %_7.i.i = lshr i64 %bits.i.i, 32
  %code.i.i = trunc nuw i64 %_7.i.i to i32
  switch i32 %code.i.i, label %bb43.i.i [
    i32 7, label %bb4
    i32 48, label %bb37.i.i
    i32 49, label %bb36.i.i
    i32 16, label %bb35.i.i
    i32 53, label %bb34.i.i
    i32 61, label %bb33.i.i
    i32 54, label %bb32.i.i
    i32 11, label %bb31.i.i
    i32 69, label %bb30.i.i
    i32 17, label %bb29.i.i
    i32 27, label %bb28.i.i
    i32 65, label %bb27.i.i
    i32 4, label %bb26.i.i
    i32 22, label %bb25.i.i
    i32 21, label %bb24.i.i
    i32 62, label %bb23.i.i
    i32 2, label %bb22.i.i
    i32 12, label %bb21.i.i
    i32 28, label %bb20.i.i
    i32 78, label %bb19.i.i
    i32 31, label %bb18.i.i
    i32 63, label %bb17.i.i
    i32 50, label %bb16.i.i
    i32 51, label %bb15.i.i
    i32 57, label %bb14.i.i
    i32 20, label %bb13.i.i
    i32 66, label %bb12.i.i
    i32 32, label %bb11.i.i
    i32 30, label %bb10.i.i
    i32 29, label %bb9.i.i
    i32 70, label %bb8.i.i
    i32 60, label %bb7.i.i
    i32 26, label %bb6.i.i
    i32 18, label %bb5.i3.i
    i32 36, label %bb4.i2.i
    i32 102, label %bb19.i.i
    i32 13, label %bb2.i1.i
    i32 1, label %bb2.i1.i
    i32 35, label %bb42.i.i
  ]

bb37.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb36.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb35.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb34.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb33.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb32.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb31.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb30.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb29.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb28.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb27.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb26.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb25.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb24.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb23.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb22.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb21.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb20.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb19.i.i:                                         ; preds = %bb5.i, %bb5.i
  br label %bb4

bb18.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb17.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb16.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb15.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb14.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb13.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb12.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb11.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb10.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb9.i.i:                                          ; preds = %bb5.i
  br label %bb4

bb8.i.i:                                          ; preds = %bb5.i
  br label %bb4

bb7.i.i:                                          ; preds = %bb5.i
  br label %bb4

bb6.i.i:                                          ; preds = %bb5.i
  br label %bb4

bb5.i3.i:                                         ; preds = %bb5.i
  br label %bb4

bb4.i2.i:                                         ; preds = %bb5.i
  br label %bb4

bb2.i1.i:                                         ; preds = %bb5.i, %bb5.i
  br label %bb4

bb43.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb42.i.i:                                         ; preds = %bb5.i
  br label %bb4

bb2.i:                                            ; preds = %bb3
  %1 = getelementptr inbounds nuw i8, ptr %err.val, i64 16
  %2 = load i8, ptr %1, align 8, !range !1160, !noundef !12
  br label %bb4

bb4.i:                                            ; preds = %bb3
  %3 = getelementptr i8, ptr %err.val, i64 -1
  %4 = icmp ne ptr %3, null
  tail call void @llvm.assume(i1 %4)
  %5 = getelementptr i8, ptr %err.val, i64 15
  %6 = load i8, ptr %5, align 8, !range !1160, !noundef !12
  br label %bb4

bb4:                                              ; preds = %bb4.i.i, %bb4.i, %bb2.i, %bb42.i.i, %bb43.i.i, %bb2.i1.i, %bb4.i2.i, %bb5.i3.i, %bb6.i.i, %bb7.i.i, %bb8.i.i, %bb9.i.i, %bb10.i.i, %bb11.i.i, %bb12.i.i, %bb13.i.i, %bb14.i.i, %bb15.i.i, %bb16.i.i, %bb17.i.i, %bb18.i.i, %bb19.i.i, %bb20.i.i, %bb21.i.i, %bb22.i.i, %bb23.i.i, %bb24.i.i, %bb25.i.i, %bb26.i.i, %bb27.i.i, %bb28.i.i, %bb29.i.i, %bb30.i.i, %bb31.i.i, %bb32.i.i, %bb33.i.i, %bb34.i.i, %bb35.i.i, %bb36.i.i, %bb37.i.i, %bb5.i, %start
  %kind.sroa.0.0 = phi i8 [ 40, %start ], [ %2, %bb2.i ], [ %6, %bb4.i ], [ 41, %bb43.i.i ], [ 8, %bb37.i.i ], [ 9, %bb36.i.i ], [ 28, %bb35.i.i ], [ 6, %bb34.i.i ], [ 2, %bb33.i.i ], [ 3, %bb32.i.i ], [ 30, %bb31.i.i ], [ 26, %bb30.i.i ], [ 12, %bb29.i.i ], [ 27, %bb28.i.i ], [ 4, %bb27.i.i ], [ 35, %bb26.i.i ], [ 20, %bb25.i.i ], [ 15, %bb24.i.i ], [ 18, %bb23.i.i ], [ 0, %bb22.i.i ], [ 38, %bb21.i.i ], [ 24, %bb20.i.i ], [ 36, %bb19.i.i ], [ 32, %bb18.i.i ], [ 33, %bb17.i.i ], [ 10, %bb16.i.i ], [ 5, %bb15.i.i ], [ 7, %bb14.i.i ], [ 14, %bb13.i.i ], [ 16, %bb12.i.i ], [ 11, %bb11.i.i ], [ 17, %bb10.i.i ], [ 25, %bb9.i.i ], [ 19, %bb8.i.i ], [ 22, %bb7.i.i ], [ 29, %bb6.i.i ], [ 31, %bb5.i3.i ], [ 39, %bb4.i2.i ], [ 1, %bb2.i1.i ], [ 13, %bb42.i.i ], [ 34, %bb5.i ], [ %switch.idx.cast, %bb4.i.i ]
; call <std::io::error::Error>::new::<walkdir::error::Error>
  %_0 = tail call noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBU_(i8 noundef %kind.sroa.0.0, ptr noalias noundef nonnull align 8 captures(address) dereferenceable(56) %walk_err)
  ret ptr %_0
}

; <&std::path::PathBuf as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCs5sEH5CPMdak_3std4path7PathBufNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !12, !align !132, !noundef !12
; call <std::path::PathBuf as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsG_NtCs5sEH5CPMdak_3std4pathNtB5_7PathBufNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <&walkdir::error::ErrorInner as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerNtB6_5Debug3fmtBA_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %__self_11.i = alloca [8 x i8], align 8
  %__self_1.i = alloca [8 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !nonnull !12, !align !132, !noundef !12
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1161)
  %0 = load i64, ptr %_3, align 8, !range !11, !alias.scope !1161, !noalias !1164, !noundef !12
  %.not.i = icmp eq i64 %0, -9223372036854775808
  br i1 %.not.i, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_11.i), !noalias !1166
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 24
  store ptr %1, ptr %__self_11.i, align 8, !noalias !1166
; call <core::fmt::Formatter>::debug_struct_field2_finish
  %2 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cbe560349b6a680eee426b88f9a47a11, i64 noundef 4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5b1139eb08b255c691d8263efc399d94, i64 noundef 8, ptr noundef nonnull readonly align 8 dereferenceable(48) %_3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_16ae073f6b5125d23ce16ce8c0c96634, i64 noundef 5, ptr noundef nonnull align 1 %__self_11.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.8)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_11.i), !noalias !1166
  br label %_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt.exit

bb3.i:                                            ; preds = %start
  %__self_0.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_1.i), !noalias !1166
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 32
  store ptr %3, ptr %__self_1.i, align 8, !noalias !1166
; call <core::fmt::Formatter>::debug_struct_field2_finish
  %4 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_24a9d1d9c182d85dcff0523bfa2532d4, i64 noundef 2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1713fdbdd59e3f6dd78509f861b8bb36, i64 noundef 4, ptr noundef nonnull readonly align 1 %__self_0.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_753aceb6ee79a10c747ccb31227ed98f, i64 noundef 3, ptr noundef nonnull align 1 %__self_1.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_1.i), !noalias !1166
  br label %_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt.exit

_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt.exit: ; preds = %bb2.i, %bb3.i
  %_0.sroa.0.0.in.i = phi i1 [ %2, %bb2.i ], [ %4, %bb3.i ]
  ret i1 %_0.sroa.0.0.in.i
}

; <&std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !12, !align !132, !noundef !12
; call <std::io::error::Error as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <&str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !12, !align !101, !noundef !12
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !12
; call <str as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <&std::io::error::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtB6_7Display3fmtCsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #2 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !12, !align !132, !noundef !12
; call <std::io::error::Error as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <walkdir::IntoIter as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
define void @_RNvXs2_CsarYwbBXrH4d_7walkdirNtB5_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable sret([56 x i8]) align 8 captures(none) dereferenceable(56) %next, ptr noalias noundef align 8 dereferenceable(176) %self) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %r1.i = alloca [1056 x i8], align 8
  %e.i = alloca [8 x i8], align 8
  %self.i.i = alloca [152 x i8], align 8
  %_19.i = alloca [56 x i8], align 16
  %_10.i = alloca [1064 x i8], align 8
  %dent5 = alloca [48 x i8], align 8
  %dent = alloca [48 x i8], align 8
  %_16 = alloca [24 x i8], align 8
  %_15 = alloca [56 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 88
  %_2.sroa.0.0.copyload = load i64, ptr %0, align 8
  %_2.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 96
  %_2.sroa.6.sroa.0.0.copyload = load ptr, ptr %_2.sroa.6.0..sroa_idx, align 8
  %_2.sroa.6.sroa.5.0._2.sroa.6.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 104
  %_2.sroa.6.sroa.5.0.copyload = load i64, ptr %_2.sroa.6.sroa.5.0._2.sroa.6.0..sroa_idx.sroa_idx, align 8
  store i64 -9223372036854775808, ptr %0, align 8
  %.not = icmp eq i64 %_2.sroa.0.0.copyload, -9223372036854775808
  br i1 %.not, label %bb37, label %bb1

bb1:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 155
  %2 = load i8, ptr %1, align 1, !range !720, !noundef !12
  %_5 = trunc nuw i8 %2 to i1
  br i1 %_5, label %bb2, label %bb5

bb37:                                             ; preds = %bb8, %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_5492 = load i64, ptr %3, align 8, !noundef !12
  %_5593 = icmp ult i64 %_5492, 144115188075855872
  call void @llvm.assume(i1 %_5593)
  %4 = icmp eq i64 %_5492, 0
  br i1 %4, label %bb13, label %bb14.lr.ph

bb14.lr.ph:                                       ; preds = %bb37
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 168
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 154
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 80
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %9 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 144
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_16.sroa.4.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %next, i64 8
  %13 = getelementptr inbounds nuw i8, ptr %_10.i, i64 8
  %_18.i.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10.i, i64 16
  %_18.i.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_10.i, i64 24
  %_20.i.sroa.5.0.r1.i.sroa_idx = getelementptr inbounds nuw i8, ptr %r1.i, i64 8
  %_20.i.sroa.6.0.r1.i.sroa_idx = getelementptr inbounds nuw i8, ptr %r1.i, i64 16
  %_9.sroa.0.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i, i64 32
  %_9.sroa.4.0._0.sroa_idx.i73 = getelementptr inbounds nuw i8, ptr %_19.i, i64 48
  br label %bb14

bb5:                                              ; preds = %bb42, %bb1
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_16)
  store i64 %_2.sroa.0.0.copyload, ptr %_16, align 8
  %start1.sroa.7.0._16.sroa_idx = getelementptr inbounds nuw i8, ptr %_16, i64 8
  store ptr %_2.sroa.6.sroa.0.0.copyload, ptr %start1.sroa.7.0._16.sroa_idx, align 8
  %start1.sroa.12.0._16.sroa_idx = getelementptr inbounds nuw i8, ptr %_16, i64 16
  store i64 %_2.sroa.6.sroa.5.0.copyload, ptr %start1.sroa.12.0._16.sroa_idx, align 8
; call <walkdir::dent::DirEntry>::from_path
  call fastcc void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9from_path(ptr noalias noundef align 8 captures(none) dereferenceable(56) %_15, i64 noundef 0, ptr noalias noundef align 8 captures(address) dereferenceable(24) %_16, i1 noundef zeroext false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_16)
  %14 = load i64, ptr %_15, align 8, !range !16, !noundef !12
  %.not11 = icmp eq i64 %14, -9223372036854775807
  br i1 %.not11, label %bb8, label %bb7

bb2:                                              ; preds = %bb1
  %15 = icmp ne ptr %_2.sroa.6.sroa.0.0.copyload, null
  tail call void @llvm.assume(i1 %15)
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %self.i.i), !noalias !1167
; invoke std::sys::fs::metadata
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(address) dereferenceable(152) %self.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.sroa.6.sroa.0.0.copyload, i64 noundef %_2.sroa.6.sroa.5.0.copyload)
          to label %bb3 unwind label %cleanup.body

cleanup.body:                                     ; preds = %bb2
  %lpad.thr_comm.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb39

bb3:                                              ; preds = %bb2
  %_5.i.i = load i64, ptr %self.i.i, align 8, !range !687, !noalias !1167, !noundef !12
  %16 = trunc nuw i64 %_5.i.i to i1
  %17 = getelementptr inbounds nuw i8, ptr %self.i.i, i64 8
  %e.i.i = load ptr, ptr %17, align 8, !noalias !1173
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %self.i.i), !noalias !1167
  br i1 %16, label %bb41, label %bb42

bb41:                                             ; preds = %bb3
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %e.i)
  store ptr %e.i.i, ptr %e.i, align 8, !noalias !1174
  %_23.i.i.i.i.i.i = icmp eq i64 %_2.sroa.6.sroa.5.0.copyload, 0
  br i1 %_23.i.i.i.i.i.i, label %bb43, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i: ; preds = %bb41
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !1177
; call __rustc::__rust_alloc
  %18 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %_2.sroa.6.sroa.5.0.copyload, i64 noundef range(i64 1, 9) 1) #26, !noalias !1177
  %19 = icmp eq ptr %18, null
  br i1 %19, label %bb3.i.i.i.i, label %bb43

bb3.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %_2.sroa.6.sroa.5.0.copyload) #27
          to label %.noexc.i unwind label %cleanup.i, !noalias !1174

.noexc.i:                                         ; preds = %bb3.i.i.i.i
  unreachable

cleanup.i:                                        ; preds = %bb3.i.i.i.i
  %20 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::io::error::Error>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorECsarYwbBXrH4d_7walkdir(ptr noalias noundef nonnull align 8 dereferenceable(8) %e.i) #28
          to label %bb39 unwind label %terminate.i, !noalias !1174

terminate.i:                                      ; preds = %cleanup.i
  %21 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1174
  unreachable

bb42:                                             ; preds = %bb3
  %22 = ptrtoint ptr %e.i.i to i64
  %sext.i = shl i64 %22, 32
  %_6.i = ashr exact i64 %sext.i, 32
  store i64 1, ptr %self, align 8
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 8
  store i64 %_6.i, ptr %23, align 8
  br label %bb5

bb7:                                              ; preds = %bb5
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %next, ptr noundef nonnull align 8 dereferenceable(56) %_15, i64 56, i1 false)
  br label %bb35

bb8:                                              ; preds = %bb5
  %24 = getelementptr inbounds nuw i8, ptr %_15, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %dent, ptr noundef nonnull align 8 dereferenceable(48) %24, i64 48, i1 false)
; call <walkdir::IntoIter>::handle_entry
  call fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter12handle_entry(ptr noalias noundef align 8 captures(none) dereferenceable(56) %next, ptr noalias noundef align 8 dereferenceable(176) %self, ptr noalias noundef align 8 captures(address) dereferenceable(48) %dent)
  %25 = load i64, ptr %next, align 8, !range !936, !noundef !12
  %.not12 = icmp eq i64 %25, -9223372036854775806
  br i1 %.not12, label %bb37, label %bb35

bb43:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i, %bb41
  %_4.sroa.10.0.i.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb41 ], [ %18, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i.i, ptr nonnull readonly align 1 %_2.sroa.6.sroa.0.0.copyload, i64 range(i64 0, -9223372036854775808) %_2.sroa.6.sroa.5.0.copyload, i1 false), !noalias !1185
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %e.i)
  store i64 -9223372036854775808, ptr %next, align 8
  %_14.sroa.4.0.next.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 8
  store i64 %_2.sroa.6.sroa.5.0.copyload, ptr %_14.sroa.4.0.next.sroa_idx, align 8
  %_14.sroa.5.0.next.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 16
  store ptr %_4.sroa.10.0.i.i.i.i, ptr %_14.sroa.5.0.next.sroa_idx, align 8
  %_14.sroa.5.sroa.4.0._14.sroa.5.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 24
  store i64 %_2.sroa.6.sroa.5.0.copyload, ptr %_14.sroa.5.sroa.4.0._14.sroa.5.0.next.sroa_idx.sroa_idx, align 8
  %_14.sroa.5.sroa.5.0._14.sroa.5.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 32
  store ptr %e.i.i, ptr %_14.sroa.5.sroa.5.0._14.sroa.5.0.next.sroa_idx.sroa_idx, align 8
  %_14.sroa.5.sroa.7.0._14.sroa.5.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 48
  store i64 0, ptr %_14.sroa.5.sroa.7.0._14.sroa.5.0.next.sroa_idx.sroa_idx, align 8
  %26 = icmp eq i64 %_2.sroa.0.0.copyload, 0
  br i1 %26, label %bb35, label %bb2.i.i.i4.i.i.i.i

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb43
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.sroa.6.sroa.0.0.copyload, i64 noundef %_2.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !1186
  br label %bb35

bb35:                                             ; preds = %bb23, %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit, %bb2.i.i.i4.i.i.i.i, %bb43, %bb7, %bb8, %bb16, %bb32, %bb30
  ret void

common.resume:                                    ; preds = %cleanup.i67, %bb2.i.i.i.i.i, %bb39, %bb2.i.i.i4.i.i.i.i21
  %common.resume.op = phi { ptr, i32 } [ %eh.lpad-body81, %bb39 ], [ %eh.lpad-body81, %bb2.i.i.i4.i.i.i.i21 ], [ %51, %bb2.i.i.i.i.i ], [ %51, %cleanup.i67 ]
  resume { ptr, i32 } %common.resume.op

bb39:                                             ; preds = %cleanup.i, %cleanup.body
  %eh.lpad-body81 = phi { ptr, i32 } [ %lpad.thr_comm.split-lp, %cleanup.body ], [ %20, %cleanup.i ]
  %27 = icmp eq i64 %_2.sroa.0.0.copyload, 0
  br i1 %27, label %common.resume, label %bb2.i.i.i4.i.i.i.i21

bb2.i.i.i4.i.i.i.i21:                             ; preds = %bb39
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.sroa.6.sroa.0.0.copyload, i64 noundef %_2.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !1189
  br label %common.resume

bb13:                                             ; preds = %bb12.backedge, %bb37
  %28 = getelementptr inbounds nuw i8, ptr %self, i64 154
  %29 = load i8, ptr %28, align 2, !range !720, !noundef !12
  %_41 = trunc nuw i8 %29 to i1
  br i1 %_41, label %bb1.i35, label %bb32

bb14:                                             ; preds = %bb14.lr.ph, %bb12.backedge
  %_5494 = phi i64 [ %_5492, %bb14.lr.ph ], [ %_54, %bb12.backedge ]
  store i64 %_5494, ptr %5, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !1192)
  %30 = load i8, ptr %6, align 2, !range !720, !alias.scope !1192, !noalias !1195, !noundef !12
  %_2.i = trunc nuw i8 %30 to i1
  br i1 %_2.i, label %bb1.i25, label %bb17

bb1.i25:                                          ; preds = %bb14
  %_5.i = load i64, ptr %7, align 8, !alias.scope !1192, !noalias !1195, !noundef !12
  %_10.i26 = icmp ult i64 %_5.i, 192153584101141163
  call void @llvm.assume(i1 %_10.i26)
  %_3.i = icmp samesign ult i64 %_5494, %_5.i
  br i1 %_3.i, label %bb11.i27, label %bb17

bb11.i27:                                         ; preds = %bb1.i25
  %31 = add nsw i64 %_5.i, -1
  store i64 %31, ptr %7, align 8, !alias.scope !1192, !noalias !1195
  %_19.i28 = load i64, ptr %8, align 8, !range !48, !alias.scope !1192, !noalias !1195, !noundef !12
  %_12.i = icmp samesign ult i64 %31, %_19.i28
  call void @llvm.assume(i1 %_12.i)
  %_20.i29 = load ptr, ptr %9, align 8, !alias.scope !1192, !noalias !1195, !nonnull !12, !noundef !12
  %_16.i = getelementptr inbounds nuw %"dent::DirEntry", ptr %_20.i29, i64 %31
  %_15.sroa.0.0.copyload.i = load i64, ptr %_16.i, align 8, !noalias !1197
  %_15.sroa.4.0._16.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_16.i, i64 8
  %_15.sroa.4.0.copyload.i = load ptr, ptr %_15.sroa.4.0._16.sroa_idx.i, align 8, !noalias !1197
  %_25.i = load i64, ptr %10, align 8, !alias.scope !1192, !noalias !1195, !noundef !12
  %_23.i = icmp ult i64 %_5494, %_25.i
  %_26.i = load i64, ptr %11, align 8, !alias.scope !1192, !noalias !1195
  %_8.i = icmp ugt i64 %_5494, %_26.i
  %or.cond.i = select i1 %_23.i, i1 true, i1 %_8.i
  br i1 %or.cond.i, label %bb3.i31, label %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit

bb3.i31:                                          ; preds = %bb11.i27
  %32 = icmp eq i64 %_15.sroa.0.0.copyload.i, 0
  br i1 %32, label %bb17, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %bb3.i31
  %33 = icmp ne ptr %_15.sroa.4.0.copyload.i, null
  call void @llvm.assume(i1 %33)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_15.sroa.4.0.copyload.i, i64 noundef %_15.sroa.0.0.copyload.i, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !1198
  %_29.pre = load i64, ptr %5, align 8
  br label %bb17

_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit: ; preds = %bb11.i27
  %.not13 = icmp eq i64 %_15.sroa.0.0.copyload.i, -9223372036854775808
  br i1 %.not13, label %bb17, label %bb16

bb32:                                             ; preds = %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit60, %bb1.i35, %bb2.i.i.i4.i.i.i.i.i.i58, %bb3.i57, %bb13
  store i64 -9223372036854775806, ptr %next, align 8
  br label %bb35

bb1.i35:                                          ; preds = %bb13
  %34 = getelementptr inbounds nuw i8, ptr %self, i64 168
  store i64 0, ptr %34, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !1201)
  %35 = getelementptr inbounds nuw i8, ptr %self, i64 80
  %_5.i37 = load i64, ptr %35, align 8, !alias.scope !1201, !noalias !1204, !noundef !12
  %_10.i38 = icmp ult i64 %_5.i37, 192153584101141163
  call void @llvm.assume(i1 %_10.i38)
  %_3.i39.not = icmp eq i64 %_5.i37, 0
  br i1 %_3.i39.not, label %bb32, label %bb11.i40

bb11.i40:                                         ; preds = %bb1.i35
  %36 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %37 = add nsw i64 %_5.i37, -1
  store i64 %37, ptr %35, align 8, !alias.scope !1201, !noalias !1204
  %_19.i41 = load i64, ptr %36, align 8, !range !48, !alias.scope !1201, !noalias !1204, !noundef !12
  %_12.i42 = icmp samesign ult i64 %37, %_19.i41
  call void @llvm.assume(i1 %_12.i42)
  %38 = getelementptr inbounds nuw i8, ptr %self, i64 72
  %_20.i43 = load ptr, ptr %38, align 8, !alias.scope !1201, !noalias !1204, !nonnull !12, !noundef !12
  %_16.i44 = getelementptr inbounds nuw %"dent::DirEntry", ptr %_20.i43, i64 %37
  %_15.sroa.0.0.copyload.i45 = load i64, ptr %_16.i44, align 8, !noalias !1206
  %_15.sroa.4.0._16.sroa_idx.i46 = getelementptr inbounds nuw i8, ptr %_16.i44, i64 8
  %_15.sroa.4.0.copyload.i47 = load ptr, ptr %_15.sroa.4.0._16.sroa_idx.i46, align 8, !noalias !1206
  %39 = getelementptr inbounds nuw i8, ptr %self, i64 136
  %_25.i48 = load i64, ptr %39, align 8, !alias.scope !1201, !noalias !1204, !noundef !12
  %_23.i49.not = icmp eq i64 %_25.i48, 0
  br i1 %_23.i49.not, label %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit60, label %bb3.i57

bb3.i57:                                          ; preds = %bb11.i40
  %40 = icmp eq i64 %_15.sroa.0.0.copyload.i45, 0
  br i1 %40, label %bb32, label %bb2.i.i.i4.i.i.i.i.i.i58

bb2.i.i.i4.i.i.i.i.i.i58:                         ; preds = %bb3.i57
  %41 = icmp ne ptr %_15.sroa.4.0.copyload.i47, null
  call void @llvm.assume(i1 %41)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_15.sroa.4.0.copyload.i47, i64 noundef %_15.sroa.0.0.copyload.i45, i64 noundef range(i64 1, -9223372036854775807) 1) #26, !noalias !1207
  br label %bb32

_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit60: ; preds = %bb11.i40
  %.not17 = icmp eq i64 %_15.sroa.0.0.copyload.i45, -9223372036854775808
  br i1 %.not17, label %bb32, label %bb30

bb30:                                             ; preds = %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit60
  %_15.sroa.5.0._16.sroa_idx.i54 = getelementptr inbounds nuw i8, ptr %_16.i44, i64 16
  %_46.sroa.4.sroa.5.0._46.sroa.4.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 24
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_46.sroa.4.sroa.5.0._46.sroa.4.0.next.sroa_idx.sroa_idx, ptr noundef nonnull align 8 dereferenceable(32) %_15.sroa.5.0._16.sroa_idx.i54, i64 32, i1 false)
  store i64 -9223372036854775807, ptr %next, align 8
  %_46.sroa.4.0.next.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 8
  store i64 %_15.sroa.0.0.copyload.i45, ptr %_46.sroa.4.0.next.sroa_idx, align 8
  %_46.sroa.4.sroa.4.0._46.sroa.4.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 16
  store ptr %_15.sroa.4.0.copyload.i47, ptr %_46.sroa.4.sroa.4.0._46.sroa.4.0.next.sroa_idx.sroa_idx, align 8
  br label %bb35

bb16:                                             ; preds = %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit
  %_15.sroa.5.0._16.sroa_idx.i.le = getelementptr inbounds nuw i8, ptr %_16.i, i64 16
  %_27.sroa.4.sroa.5.0._27.sroa.4.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 24
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_27.sroa.4.sroa.5.0._27.sroa.4.0.next.sroa_idx.sroa_idx, ptr noundef nonnull align 8 dereferenceable(32) %_15.sroa.5.0._16.sroa_idx.i.le, i64 32, i1 false)
  store i64 -9223372036854775807, ptr %next, align 8
  store i64 %_15.sroa.0.0.copyload.i, ptr %_16.sroa.4.0._0.sroa_idx.i, align 8
  %_27.sroa.4.sroa.4.0._27.sroa.4.0.next.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %next, i64 16
  store ptr %_15.sroa.4.0.copyload.i, ptr %_27.sroa.4.sroa.4.0._27.sroa.4.0.next.sroa_idx.sroa_idx, align 8
  br label %bb35

bb17:                                             ; preds = %bb3.i31, %bb2.i.i.i4.i.i.i.i.i.i, %bb14, %bb1.i25, %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit
  %_29 = phi i64 [ %_5494, %bb3.i31 ], [ %_29.pre, %bb2.i.i.i4.i.i.i.i.i.i ], [ %_5494, %bb14 ], [ %_5494, %bb1.i25 ], [ %_5494, %_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir.exit ]
  %_30 = load i64, ptr %11, align 8, !noundef !12
  %_28 = icmp ugt i64 %_29, %_30
  br i1 %_28, label %bb12.backedge.sink.split, label %bb20

bb20:                                             ; preds = %bb17
  %_59 = load i64, ptr %3, align 8, !noundef !12
  %_61.not = icmp eq i64 %_59, 0
  br i1 %_61.not, label %bb45, label %bb44, !prof !645

bb45:                                             ; preds = %bb20
; call core::option::expect_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_65a24c48507a3f07655bc752e59c1139, i64 noundef 30, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4f06d9fc34a8ce4ebe7bb54bb63ea865) #32
  unreachable

bb44:                                             ; preds = %bb20
  %_60 = load ptr, ptr %12, align 8, !nonnull !12, !noundef !12
  %42 = getelementptr %DirList, ptr %_60, i64 %_59
  %_62 = getelementptr i8, ptr %42, i64 -64
  call void @llvm.experimental.noalias.scope.decl(metadata !1210)
  call void @llvm.experimental.noalias.scope.decl(metadata !1213)
  %43 = load i64, ptr %_62, align 8, !range !49, !alias.scope !1213, !noalias !1210, !noundef !12
  %44 = icmp eq i64 %43, -9223372036854775805
  br i1 %44, label %bb3.i, label %bb2.i

bb3.i:                                            ; preds = %bb44
  call void @llvm.experimental.noalias.scope.decl(metadata !1215)
  call void @llvm.experimental.noalias.scope.decl(metadata !1218)
  %_14.i = getelementptr i8, ptr %42, i64 -32
  %_12.i61 = load ptr, ptr %_14.i, align 8, !alias.scope !1218, !noalias !1215, !nonnull !12, !noundef !12
  %45 = getelementptr i8, ptr %42, i64 -48
  %_21.i = load ptr, ptr %45, align 8, !alias.scope !1218, !noalias !1215, !nonnull !12, !noundef !12
  %_9.i = icmp eq ptr %_21.i, %_12.i61
  br i1 %_9.i, label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.thread, label %bb6.i

bb6.i:                                            ; preds = %bb3.i
  %_23.i62 = getelementptr inbounds nuw i8, ptr %_21.i, i64 56
  store ptr %_23.i62, ptr %45, align 8, !alias.scope !1218, !noalias !1215
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %next, ptr noundef nonnull align 8 dereferenceable(56) %_21.i, i64 56, i1 false), !noalias !1218
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exitthread-pre-split

_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.thread: ; preds = %bb3.i
  store i64 -9223372036854775806, ptr %next, align 8, !alias.scope !1215, !noalias !1218
  br label %bb12.backedge.sink.split

bb2.i:                                            ; preds = %bb44
  %46 = getelementptr i8, ptr %42, i64 -8
  %47 = load i64, ptr %46, align 8, !alias.scope !1213, !noalias !1210, !noundef !12
  %.not.i = icmp eq i64 %43, -9223372036854775806
  %rd.i = getelementptr i8, ptr %42, i64 -56
  br i1 %.not.i, label %bb4.i, label %bb5.i

bb5.i:                                            ; preds = %bb2.i
  store i64 -9223372036854775807, ptr %_62, align 8, !alias.scope !1213, !noalias !1210
  %.not2.i = icmp eq i64 %43, -9223372036854775807
  br i1 %.not2.i, label %bb9.i, label %bb11.i

bb4.i:                                            ; preds = %bb2.i
  call void @llvm.lifetime.start.p0(i64 1064, ptr nonnull %_10.i), !noalias !1220
; call <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
  call void @_RNvXsB_NtCs5sEH5CPMdak_3std2fsNtB5_7ReadDirNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([1064 x i8]) align 8 captures(none) dereferenceable(1064) %_10.i, ptr noalias noundef nonnull align 8 dereferenceable(16) %rd.i), !noalias !1210
  %_17.i = load i64, ptr %_10.i, align 8, !range !687, !noalias !1220, !noundef !12
  %48 = trunc nuw i64 %_17.i to i1
  br i1 %48, label %bb14.i, label %bb13.i

bb14.i:                                           ; preds = %bb4.i
  %_18.i.sroa.0.0.copyload = load ptr, ptr %13, align 8, !noalias !1220
  %_18.i.sroa.4.0.copyload = load ptr, ptr %_18.i.sroa.4.0..sroa_idx, align 8, !noalias !1220
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_19.i), !noalias !1220
  call void @llvm.experimental.noalias.scope.decl(metadata !1221)
  %49 = icmp eq ptr %_18.i.sroa.0.0.copyload, null
  %_10.i65 = add i64 %47, 1
  br i1 %49, label %bb2.i72, label %bb3.i66

bb2.i72:                                          ; preds = %bb14.i
  %50 = icmp ne ptr %_18.i.sroa.4.0.copyload, null
  call void @llvm.assume(i1 %50)
  store <2 x i64> splat (i64 -9223372036854775808), ptr %_19.i, align 16, !alias.scope !1221, !noalias !1224
  store ptr %_18.i.sroa.4.0.copyload, ptr %_9.sroa.0.sroa.5.0._0.sroa_idx.i, align 16, !alias.scope !1221, !noalias !1224
  store i64 %_10.i65, ptr %_9.sroa.4.0._0.sroa_idx.i73, align 16, !alias.scope !1221, !noalias !1224
  br label %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit

bb3.i66:                                          ; preds = %bb14.i
  call void @llvm.lifetime.start.p0(i64 1056, ptr nonnull %r1.i), !noalias !1226
  store ptr %_18.i.sroa.0.0.copyload, ptr %r1.i, align 8, !noalias !1221
  store ptr %_18.i.sroa.4.0.copyload, ptr %_20.i.sroa.5.0.r1.i.sroa_idx, align 8, !noalias !1221
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(1040) %_20.i.sroa.6.0.r1.i.sroa_idx, ptr noundef nonnull align 8 dereferenceable(1040) %_18.i.sroa.5.0..sroa_idx, i64 1040, i1 false)
; invoke <walkdir::dent::DirEntry>::from_entry
  invoke void @_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry10from_entry(ptr noalias noundef nonnull sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_19.i, i64 noundef %_10.i65, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(1056) %r1.i)
          to label %bb4.i70 unwind label %cleanup.i67, !noalias !1224

cleanup.i67:                                      ; preds = %bb3.i66
  %51 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !1227)
  call void @llvm.experimental.noalias.scope.decl(metadata !1230)
  call void @llvm.experimental.noalias.scope.decl(metadata !1233)
  call void @llvm.experimental.noalias.scope.decl(metadata !1236)
  %_10.i.i.i.i.i = load ptr, ptr %r1.i, align 8, !alias.scope !1239, !noalias !1226, !nonnull !12, !noundef !12
  %52 = atomicrmw sub ptr %_10.i.i.i.i.i, i64 1 release, align 8, !noalias !1240
  %53 = icmp eq i64 %52, 1
  br i1 %53, label %bb2.i.i.i.i.i, label %common.resume

bb2.i.i.i.i.i:                                    ; preds = %cleanup.i67
  fence acquire
; invoke <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i) #30
          to label %common.resume unwind label %terminate.i69, !noalias !1226

bb4.i70:                                          ; preds = %bb3.i66
  call void @llvm.experimental.noalias.scope.decl(metadata !1241)
  call void @llvm.experimental.noalias.scope.decl(metadata !1244)
  call void @llvm.experimental.noalias.scope.decl(metadata !1247)
  call void @llvm.experimental.noalias.scope.decl(metadata !1250)
  %_10.i.i.i.i2.i = load ptr, ptr %r1.i, align 8, !alias.scope !1253, !noalias !1226, !nonnull !12, !noundef !12
  %54 = atomicrmw sub ptr %_10.i.i.i.i2.i, i64 1 release, align 8, !noalias !1254
  %55 = icmp eq i64 %54, 1
  br i1 %55, label %bb2.i.i.i.i3.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i

bb2.i.i.i.i3.i:                                   ; preds = %bb4.i70
  fence acquire
; call <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef nonnull align 8 dereferenceable(1056) %r1.i) #30, !noalias !1226
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i: ; preds = %bb2.i.i.i.i3.i, %bb4.i70
  call void @llvm.lifetime.end.p0(i64 1056, ptr nonnull %r1.i), !noalias !1226
  br label %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit

terminate.i69:                                    ; preds = %bb2.i.i.i.i.i
  %56 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #29, !noalias !1226
  unreachable

_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit: ; preds = %bb2.i72, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir.exit4.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %next, ptr noundef nonnull align 16 dereferenceable(56) %_19.i, i64 56, i1 false), !noalias !1213
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_19.i), !noalias !1220
  br label %bb12.i

bb13.i:                                           ; preds = %bb4.i
  store i64 -9223372036854775806, ptr %next, align 8, !alias.scope !1210, !noalias !1213
  br label %bb12.i

bb12.i:                                           ; preds = %bb13.i, %_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_.exit
  call void @llvm.lifetime.end.p0(i64 1064, ptr nonnull %_10.i), !noalias !1220
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exitthread-pre-split

bb11.i:                                           ; preds = %bb5.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_16.sroa.4.0._0.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(48) %rd.i, i64 48, i1 false)
  br label %bb9.i

bb9.i:                                            ; preds = %bb5.i, %bb11.i
  %.sink = phi i64 [ %43, %bb11.i ], [ -9223372036854775806, %bb5.i ]
  store i64 %.sink, ptr %next, align 8, !alias.scope !1210, !noalias !1213
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exitthread-pre-split: ; preds = %bb6.i, %bb12.i
  %.pr.pr = load i64, ptr %next, align 8
  br label %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit

_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit: ; preds = %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exitthread-pre-split, %bb9.i
  %.pr = phi i64 [ %.pr.pr, %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exitthread-pre-split ], [ %.sink, %bb9.i ]
  switch i64 %.pr, label %bb35 [
    i64 -9223372036854775806, label %bb12.backedge.sink.split
    i64 -9223372036854775807, label %bb23
  ]

bb12.backedge.sink.split:                         ; preds = %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit, %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.thread, %bb17
; call <walkdir::IntoIter>::pop
  call fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter3pop(ptr noalias noundef align 8 dereferenceable(176) %self)
  br label %bb12.backedge

bb12.backedge:                                    ; preds = %bb12.backedge.sink.split, %bb23
  %_54 = load i64, ptr %3, align 8, !noundef !12
  %_55 = icmp ult i64 %_54, 144115188075855872
  call void @llvm.assume(i1 %_55)
  %57 = icmp eq i64 %_54, 0
  br i1 %57, label %bb13, label %bb14

bb23:                                             ; preds = %_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %dent5, ptr noundef nonnull align 8 dereferenceable(48) %_16.sroa.4.0._0.sroa_idx.i, i64 48, i1 false)
; call <walkdir::IntoIter>::handle_entry
  call fastcc void @_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter12handle_entry(ptr noalias noundef align 8 captures(none) dereferenceable(56) %next, ptr noalias noundef align 8 dereferenceable(176) %self, ptr noalias noundef align 8 captures(address) dereferenceable(48) %dent5)
  %58 = load i64, ptr %next, align 8, !range !936, !noundef !12
  %.not16 = icmp eq i64 %58, -9223372036854775806
  br i1 %.not16, label %bb12.backedge, label %bb35
}

; <walkdir::error::Error as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXs2_NtCsarYwbBXrH4d_7walkdir5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(56) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %_7 = alloca [8 x i8], align 8
  %_4 = getelementptr inbounds nuw i8, ptr %self, i64 48
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %self, ptr %_7, align 8
; call <core::fmt::Formatter>::debug_struct_field2_finish
  %_0 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_99ac8a81a24cac863217ce4a5cbfabea, i64 noundef 5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1496eca9c646eb8a70ec7074a69455e2, i64 noundef 5, ptr noundef nonnull align 1 %_4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6c342f467cee9eb46aaa013cf1ccd49c, i64 noundef 5, ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i1 %_0
}

; <core::option::Option<std::path::PathBuf> as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufENtNtB7_3fmt5Debug3fmtCsarYwbBXrH4d_7walkdir(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %__self_0 = alloca [8 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_0)
  store ptr %self, ptr %__self_0, align 8
; call <core::fmt::Formatter>::debug_tuple_field1_finish
  %1 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9535bf4c204f3eb9b19ec2c83e446e52, i64 noundef 4, ptr noundef nonnull align 1 %__self_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.8)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_0)
  br label %bb5

bb3:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %2 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_37d2e53432a03a1f90b3e7253015eaf9, i64 noundef 4)
  br label %bb5

bb5:                                              ; preds = %bb2, %bb3
  %_0.sroa.0.0.in = phi i1 [ %1, %bb2 ], [ %2, %bb3 ]
  ret i1 %_0.sroa.0.0.in
}

; <usize as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !12
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_5 = and i32 %_4, 67108864
  %2 = icmp eq i32 %_5, 0
  br i1 %2, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call <usize as core::fmt::LowerHex>::fmt
  %3 = tail call noundef zeroext i1 @_RNvXs6_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb4:                                              ; preds = %bb2
; call <usize as core::fmt::Display>::fmt
  %4 = tail call noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb3:                                              ; preds = %bb2
; call <usize as core::fmt::UpperHex>::fmt
  %5 = tail call noundef zeroext i1 @_RNvXs8_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb3, %bb1
  %_0.sroa.0.0.in = phi i1 [ %4, %bb4 ], [ %5, %bb3 ], [ %3, %bb1 ]
  ret i1 %_0.sroa.0.0.in
}

; <walkdir::dent::DirEntry as core::clone::Clone>::clone
; Function Attrs: uwtable
define void @_RNvXs_NtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntryNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr dead_on_unwind noalias noundef writable writeonly sret([48 x i8]) align 8 captures(none) dereferenceable(48) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(48) %self) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val = load ptr, ptr %0, align 8, !nonnull !12, !noundef !12
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1 = load i64, ptr %1, align 8, !noundef !12
  %_23.i.i.i.i.i = icmp eq i64 %self.val1, 0
  br i1 %_23.i.i.i.i.i, label %_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i: ; preds = %start
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #26, !noalias !1255
; call __rustc::__rust_alloc
  %2 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %self.val1, i64 noundef range(i64 1, 9) 1) #26, !noalias !1255
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb3.i.i.i, label %_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir.exit

bb3.i.i.i:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %self.val1) #27, !noalias !1263
  unreachable

_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir.exit: ; preds = %start, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
  %_4.sroa.10.0.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %start ], [ %2, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i ]
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i, ptr nonnull readonly align 1 %self.val, i64 range(i64 0, -9223372036854775808) %self.val1, i1 false), !noalias !1264
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %_3 = load i16, ptr %4, align 8, !noundef !12
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 42
  %6 = load i8, ptr %5, align 2, !range !720, !noundef !12
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 24
  store i64 %self.val1, ptr %_0, align 8
  %_2.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_4.sroa.10.0.i.i.i, ptr %_2.sroa.4.0._0.sroa_idx, align 8
  %_2.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %self.val1, ptr %_2.sroa.5.0._0.sroa_idx, align 8
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i16 %_3, ptr %8, align 8
  %9 = getelementptr inbounds nuw i8, ptr %_0, i64 42
  store i8 %6, ptr %9, align 2
  %10 = getelementptr inbounds nuw i8, ptr %_0, i64 24
  %11 = load <2 x i64>, ptr %7, align 8
  store <2 x i64> %11, ptr %10, align 8
  ret void
}

; <walkdir::error::Error as core::error::Error>::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal { ptr, i64 } @_RNvXs_NtCsarYwbBXrH4d_7walkdir5errorNtB4_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error11description(ptr noalias noundef readonly align 8 captures(none) dereferenceable(56) %self) unnamed_addr #7 {
start:
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  %. = select i1 %.not, i64 40, i64 22
  %alloc_04d7ce44d7c86a9a02b346ab945bf155.alloc_48bada85afbcc0a7dd8eb6b62a4956f4 = select i1 %.not, ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, ptr @alloc_48bada85afbcc0a7dd8eb6b62a4956f4
  %1 = insertvalue { ptr, i64 } poison, ptr %alloc_04d7ce44d7c86a9a02b346ab945bf155.alloc_48bada85afbcc0a7dd8eb6b62a4956f4, 0
  %2 = insertvalue { ptr, i64 } %1, i64 %., 1
  ret { ptr, i64 } %2
}

; <walkdir::error::Error as core::error::Error>::cause
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal { ptr, ptr } @_RNvXs_NtCsarYwbBXrH4d_7walkdir5errorNtB4_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error5cause(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(56) %self) unnamed_addr #7 {
start:
  %0 = load i64, ptr %self, align 8, !range !11, !noundef !12
  %.not = icmp eq i64 %0, -9223372036854775808
  %_3 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_0.sroa.0.0 = select i1 %.not, ptr %_3, ptr null
  %1 = insertvalue { ptr, ptr } poison, ptr %_0.sroa.0.0, 0
  %2 = insertvalue { ptr, ptr } %1, ptr @vtable.9, 1
  ret { ptr, ptr } %2
}

; <bool as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsf_NtCsjMrxcFdYDNN_4core3fmtbNtB5_5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
; call <bool as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <walkdir::error::Error as core::error::Error>::provide
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @_RNvYNtNtCsarYwbBXrH4d_7walkdir5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7provideB6_(ptr noalias readonly align 8 captures(none) %self, ptr nonnull readnone align 8 captures(none) %request.0, ptr noalias readonly align 8 captures(none) %request.1) unnamed_addr #8 {
start:
  ret void
}

; <walkdir::error::Error as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYNtNtCsarYwbBXrH4d_7walkdir5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idB6_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #9 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.e110384ad14f08951530940520241fc9.0, i64 16, i1 false)
  ret void
}

; <std::io::error::Error as core::error::Error>::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, i64 } @_RNvYNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error11descriptionCsarYwbBXrH4d_7walkdir(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #8 {
start:
  ret { ptr, i64 } { ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, i64 40 }
}

; <std::io::error::Error as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idCsarYwbBXrH4d_7walkdir(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #9 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.e110384ad14f08951530940520241fc9.1, i64 16, i1 false)
  ret void
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #10

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #10

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #11

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #12

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #13

; <same_file::unix::Handle>::from_file
; Function Attrs: uwtable
declare void @_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i32 noundef range(i32 0, -1)) unnamed_addr #2

; <std::io::error::Error>::_new
; Function Attrs: uwtable
declare noundef nonnull ptr @_RNvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5Error4__new(i8 noundef range(i8 0, 42), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(80)) unnamed_addr #2

; <std::fs::OpenOptions>::_open
; Function Attrs: uwtable
declare void @_RNvMsl_NtCs5sEH5CPMdak_3std2fsNtB5_11OpenOptions5__open(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(none) dereferenceable(16), ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(12), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; std::sys::fs::symlink_metadata
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs16symlink_metadata(ptr dead_on_unwind noalias noundef writable sret([152 x i8]) align 8 captures(address) dereferenceable(152), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; std::sys::fs::metadata
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8metadata(ptr dead_on_unwind noalias noundef writable sret([152 x i8]) align 8 captures(address) dereferenceable(152), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; std::sys::fs::read_dir
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std3sys2fs8read_dir(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; <same_file::unix::Handle as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #14

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #15

; core::slice::sort::shared::smallsort::panic_on_ord_violation
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort22panic_on_ord_violation() unnamed_addr #16

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #16

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #17

; core::slice::sort::stable::drift::sqrt_approx
; Function Attrs: uwtable
declare noundef i64 @_RNvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift11sqrt_approx(i64 noundef) unnamed_addr #2

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #18

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #19

; <std::fs::DirEntry>::path
; Function Attrs: uwtable
declare void @_RNvMsC_NtCs5sEH5CPMdak_3std2fsNtB5_8DirEntry4path(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(1056)) unnamed_addr #2

; <std::path::Path>::to_path_buf
; Function Attrs: uwtable
declare void @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path11to_path_buf(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #20

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare range(i8 -1, 2) i8 @llvm.scmp.i8.i64(i64, i64) #17

; <std::fs::DirEntry>::file_type
; Function Attrs: uwtable
declare void @_RNvMsC_NtCs5sEH5CPMdak_3std2fsNtB5_8DirEntry9file_type(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(none) dereferenceable(16), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(1056)) unnamed_addr #2

; <std::path::Path>::file_name
; Function Attrs: uwtable
declare { ptr, i64 } @_RNvMs16_NtCs5sEH5CPMdak_3std4pathNtB6_4Path9file_name(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #21

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #22

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #23

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #16

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #17

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #16

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #16

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #19

; <core::fmt::Formatter>::debug_struct
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; <core::fmt::builders::DebugStruct>::field
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef align 8 dereferenceable(16), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #2

; <core::fmt::builders::DebugStruct>::finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #2

; <std::path::PathBuf as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsG_NtCs5sEH5CPMdak_3std4pathNtB5_7PathBufNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #2

; <std::io::error::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs7_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <std::path::Display as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs1b_NtCs5sEH5CPMdak_3std4pathNtB6_7DisplayNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <std::io::error::Error as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtCs5sEH5CPMdak_3std2io5errorNtB2_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <core::fmt::Formatter>::debug_struct_field2_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #2

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; <std::fs::ReadDir as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
declare void @_RNvXsB_NtCs5sEH5CPMdak_3std2fsNtB5_7ReadDirNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable sret([1064 x i8]) align 8 captures(none) dereferenceable(1064), ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #2

; <alloc::sync::Arc<std::sys::fs::unix::InnerReadDir>>::drop_slow
; Function Attrs: noinline uwtable
declare void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirE9drop_slowBO_(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #0

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; <core::fmt::Formatter>::debug_tuple_field1_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #2

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <usize as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <usize as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs6_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; <std::io::error::Error as core::error::Error>::source
; Function Attrs: uwtable
declare { ptr, ptr } @_RNvXs8_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8)) unnamed_addr #2

; <std::io::error::Error as core::error::Error>::cause
; Function Attrs: uwtable
declare { ptr, ptr } @_RNvXs8_NtNtCs5sEH5CPMdak_3std2io5errorNtB5_5ErrorNtNtCsjMrxcFdYDNN_4core5error5Error5cause(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8)) unnamed_addr #2

; <bool as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #24

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #25

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #24

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #11 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #13 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #15 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #16 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #18 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #21 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #22 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #23 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #24 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #25 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #26 = { nounwind }
attributes #27 = { noreturn }
attributes #28 = { cold }
attributes #29 = { cold noreturn nounwind }
attributes #30 = { noinline }
attributes #31 = { inlinehint }
attributes #32 = { noinline noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3, !5, !7}
!3 = distinct !{!3, !4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCsarYwbBXrH4d_7walkdir5error5ErrorE3newBI_: %x"}
!4 = distinct !{!4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCsarYwbBXrH4d_7walkdir5error5ErrorE3newBI_"}
!5 = distinct !{!5, !6, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc5boxed7convertINtB7_3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtBW_6marker4SendNtB1t_4SyncEL_EINtNtBW_7convert4FromNtNtCsarYwbBXrH4d_7walkdir5error5ErrorE4fromB2o_: %err"}
!6 = distinct !{!6, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc5boxed7convertINtB7_3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtBW_6marker4SendNtB1t_4SyncEL_EINtNtBW_7convert4FromNtNtCsarYwbBXrH4d_7walkdir5error5ErrorE4fromB2o_"}
!7 = distinct !{!7, !8, !"_RNvXs1_NtCsjMrxcFdYDNN_4core7convertNtNtCsarYwbBXrH4d_7walkdir5error5ErrorINtB5_4IntoINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB7_5error5ErrorNtNtB7_6marker4SendNtB2g_4SyncEL_EE4intoBC_: %self"}
!8 = distinct !{!8, !"_RNvXs1_NtCsjMrxcFdYDNN_4core7convertNtNtCsarYwbBXrH4d_7walkdir5error5ErrorINtB5_4IntoINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB7_5error5ErrorNtNtB7_6marker4SendNtB2g_4SyncEL_EE4intoBC_"}
!9 = !{!"branch_weights", !"expected", i32 1717128, i32 2145766520}
!10 = !{!5, !7}
!11 = !{i64 0, i64 -9223372036854775807}
!12 = !{}
!13 = !{!14}
!14 = distinct !{!14, !15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!15 = distinct !{!15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!16 = !{i64 0, i64 -9223372036854775806}
!17 = !{!18}
!18 = distinct !{!18, !19, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!19 = distinct !{!19, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!20 = !{!21}
!21 = distinct !{!21, !22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!22 = distinct !{!22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!23 = !{!24}
!24 = distinct !{!24, !25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_: %_1"}
!25 = distinct !{!25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_"}
!26 = !{!27}
!27 = distinct !{!27, !28, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_: %_1"}
!28 = distinct !{!28, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_"}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir: %_1"}
!31 = distinct !{!31, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir"}
!32 = !{!30, !27, !24}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!35 = distinct !{!35, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!36 = !{!34, !30, !27, !24}
!37 = !{!38}
!38 = distinct !{!38, !39, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!39 = distinct !{!39, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!40 = !{!38, !27, !24}
!41 = !{!42}
!42 = distinct !{!42, !43, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!43 = distinct !{!43, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!44 = !{!42, !27, !24}
!45 = !{!46}
!46 = distinct !{!46, !47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB17_5error5ErrorEEB17_: %_1.0"}
!47 = distinct !{!47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB17_5error5ErrorEEB17_"}
!48 = !{i64 0, i64 -9223372036854775808}
!49 = !{i64 0, i64 -9223372036854775804}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std2fs7ReadDirINtNtB4_6option6OptionNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEEEB1Z_: %_1"}
!52 = distinct !{!52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultNtNtCs5sEH5CPMdak_3std2fs7ReadDirINtNtB4_6option6OptionNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEEEB1Z_"}
!53 = !{!54}
!54 = distinct !{!54, !55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs7ReadDirECsarYwbBXrH4d_7walkdir: %_1"}
!55 = distinct !{!55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs7ReadDirECsarYwbBXrH4d_7walkdir"}
!56 = !{!57}
!57 = distinct !{!57, !58, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix7ReadDirECsarYwbBXrH4d_7walkdir: %_1"}
!58 = distinct !{!58, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix7ReadDirECsarYwbBXrH4d_7walkdir"}
!59 = !{!60}
!60 = distinct !{!60, !61, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!61 = distinct !{!61, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!62 = !{!63}
!63 = distinct !{!63, !64, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!64 = distinct !{!64, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!65 = !{!63, !60, !57, !54, !51}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEEB16_: %_1"}
!68 = distinct !{!68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEEB16_"}
!69 = !{!70}
!70 = distinct !{!70, !71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_: %_1"}
!71 = distinct !{!71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error5ErrorEBK_"}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_: %_1"}
!74 = distinct !{!74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_"}
!75 = !{!76}
!76 = distinct !{!76, !77, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir: %_1"}
!77 = distinct !{!77, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir"}
!78 = !{!76, !73, !70, !67, !51}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!81 = distinct !{!81, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!82 = !{!80, !76, !73, !70, !67, !51}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!85 = distinct !{!85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!86 = !{!84, !73, !70, !67, !51}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!89 = distinct !{!89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!90 = !{!88, !73, !70, !67, !51}
!91 = !{!92}
!92 = distinct !{!92, !93, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_: %_1"}
!93 = distinct !{!93, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_"}
!94 = !{!95}
!95 = distinct !{!95, !96, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtB11_3ops4drop4Drop4dropB1C_: %self"}
!96 = distinct !{!96, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtB11_3ops4drop4Drop4dropB1C_"}
!97 = !{!95, !92}
!98 = !{!99, !95, !92}
!99 = distinct !{!99, !100, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB17_5error5ErrorEEB17_: %_1.0"}
!100 = distinct !{!100, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB17_5error5ErrorEEB17_"}
!101 = !{i64 1}
!102 = !{i64 1, i64 0}
!103 = !{!104}
!104 = distinct !{!104, !105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!105 = distinct !{!105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!106 = !{!107}
!107 = distinct !{!107, !108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!108 = distinct !{!108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!109 = !{!110}
!110 = distinct !{!110, !111, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!111 = distinct !{!111, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir"}
!112 = !{!113}
!113 = distinct !{!113, !114, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_: %_1"}
!114 = distinct !{!114, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_"}
!115 = !{!116}
!116 = distinct !{!116, !117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir: %_1"}
!117 = distinct !{!117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir"}
!118 = !{!116, !113}
!119 = !{!120}
!120 = distinct !{!120, !121, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!121 = distinct !{!121, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!122 = !{!120, !116, !113}
!123 = !{!124}
!124 = distinct !{!124, !125, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!125 = distinct !{!125, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!126 = !{!124, !113}
!127 = !{!128}
!128 = distinct !{!128, !129, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!129 = distinct !{!129, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!130 = !{!128, !113}
!131 = !{!"branch_weights", i32 2000, i32 6001}
!132 = !{i64 8}
!133 = !{!"branch_weights", i32 6004, i32 2000}
!134 = !{!135, !137}
!135 = distinct !{!135, !136, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!136 = distinct !{!136, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!137 = distinct !{!137, !138, !"_RNvXs8_NtCsdJPVW0sQgAG_5alloc5sliceINtNtB7_3vec3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorEEINtNtNtNtBS_5slice4sort6stable8BufGuardBN_E13with_capacityB1t_: %_0"}
!138 = distinct !{!138, !"_RNvXs8_NtCsdJPVW0sQgAG_5alloc5sliceINtNtB7_3vec3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorEEINtNtNtNtBS_5slice4sort6stable8BufGuardBN_E13with_capacityB1t_"}
!139 = !{!140}
!140 = distinct !{!140, !141, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_: %_1"}
!141 = distinct !{!141, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorEEEB1D_"}
!142 = !{!143}
!143 = distinct !{!143, !144, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!144 = distinct !{!144, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!145 = !{!146}
!146 = distinct !{!146, !144, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!147 = !{!143, !146}
!148 = !{!149}
!149 = distinct !{!149, !150, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!150 = distinct !{!150, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!151 = !{!152}
!152 = distinct !{!152, !150, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!153 = !{!149, !143}
!154 = !{!152, !146}
!155 = !{!149, !152, !143, !146}
!156 = !{!157, !159}
!157 = distinct !{!157, !158, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!158 = distinct !{!158, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!159 = distinct !{!159, !158, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!160 = !{!161, !157}
!161 = distinct !{!161, !162, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!162 = distinct !{!162, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!163 = !{!164, !159}
!164 = distinct !{!164, !162, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!165 = !{!157}
!166 = !{!159}
!167 = !{!161}
!168 = !{!164}
!169 = !{!161, !164, !157, !159}
!170 = !{!171}
!171 = distinct !{!171, !172, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!172 = distinct !{!172, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!173 = !{!174}
!174 = distinct !{!174, !172, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!175 = !{!171, !174}
!176 = !{!177}
!177 = distinct !{!177, !178, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!178 = distinct !{!178, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!179 = !{!180}
!180 = distinct !{!180, !178, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!181 = !{!177, !171}
!182 = !{!180, !174}
!183 = !{!177, !180, !171, !174}
!184 = !{!185}
!185 = distinct !{!185, !186, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!186 = distinct !{!186, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!187 = !{!188}
!188 = distinct !{!188, !186, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!189 = !{!185, !188}
!190 = !{!191}
!191 = distinct !{!191, !192, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!192 = distinct !{!192, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!193 = !{!194}
!194 = distinct !{!194, !192, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!195 = !{!191, !185}
!196 = !{!194, !188}
!197 = !{!191, !194, !185, !188}
!198 = !{!199, !201}
!199 = distinct !{!199, !200, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!200 = distinct !{!200, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!201 = distinct !{!201, !200, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!202 = !{!199}
!203 = !{!201}
!204 = !{!205}
!205 = distinct !{!205, !206, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!206 = distinct !{!206, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!207 = !{!208}
!208 = distinct !{!208, !206, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!209 = !{!205, !199}
!210 = !{!208, !201}
!211 = !{!205, !208, !199, !201}
!212 = !{!213, !215}
!213 = distinct !{!213, !214, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!214 = distinct !{!214, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!215 = distinct !{!215, !214, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!216 = !{!213}
!217 = !{!215}
!218 = !{!219}
!219 = distinct !{!219, !220, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!220 = distinct !{!220, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!221 = !{!222}
!222 = distinct !{!222, !220, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!223 = !{!219, !213}
!224 = !{!222, !215}
!225 = !{!219, !222, !213, !215}
!226 = !{!227, !229}
!227 = distinct !{!227, !228, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!228 = distinct !{!228, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!229 = distinct !{!229, !228, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!230 = !{!227}
!231 = !{!229}
!232 = !{!233}
!233 = distinct !{!233, !234, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!234 = distinct !{!234, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!235 = !{!236}
!236 = distinct !{!236, !234, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!237 = !{!233, !227}
!238 = !{!236, !229}
!239 = !{!233, !236, !227, !229}
!240 = !{!241, !243}
!241 = distinct !{!241, !242, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!242 = distinct !{!242, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!243 = distinct !{!243, !242, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!244 = !{!241}
!245 = !{!243}
!246 = !{!247}
!247 = distinct !{!247, !248, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!248 = distinct !{!248, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!249 = !{!250}
!250 = distinct !{!250, !248, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!251 = !{!247, !241}
!252 = !{!250, !243}
!253 = !{!247, !250, !241, !243}
!254 = !{!255}
!255 = distinct !{!255, !256, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!256 = distinct !{!256, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!257 = !{!258}
!258 = distinct !{!258, !256, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!259 = !{!255, !258}
!260 = !{!261}
!261 = distinct !{!261, !262, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!262 = distinct !{!262, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!263 = !{!264}
!264 = distinct !{!264, !262, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!265 = !{!261, !255}
!266 = !{!264, !258}
!267 = !{!261, !264, !255, !258}
!268 = !{!269}
!269 = distinct !{!269, !270, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!270 = distinct !{!270, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!271 = !{!272}
!272 = distinct !{!272, !270, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!273 = !{!269, !272}
!274 = !{!275}
!275 = distinct !{!275, !276, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!276 = distinct !{!276, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!277 = !{!278}
!278 = distinct !{!278, !276, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!279 = !{!275, !269}
!280 = !{!278, !272}
!281 = !{!275, !278, !269, !272}
!282 = !{!283, !285}
!283 = distinct !{!283, !284, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_: %self"}
!284 = distinct !{!284, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_"}
!285 = distinct !{!285, !286, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_: %_1"}
!286 = distinct !{!286, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_"}
!287 = !{!288, !290}
!288 = distinct !{!288, !289, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_: %self"}
!289 = distinct !{!289, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_"}
!290 = distinct !{!290, !291, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_: %_1"}
!291 = distinct !{!291, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_"}
!292 = !{!293}
!293 = distinct !{!293, !294, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_: %is_less"}
!294 = distinct !{!294, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_"}
!295 = !{!296, !297}
!296 = distinct !{!296, !294, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_: %v.0"}
!297 = distinct !{!297, !294, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5drift10create_runINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1t_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB13_7sort_byNCNvMs3_B1t_NtB1t_8IntoIter4pushs_0E0EB1t_: %scratch.0"}
!298 = !{!299}
!299 = distinct !{!299, !300, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!300 = distinct !{!300, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!301 = !{!302}
!302 = distinct !{!302, !300, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!303 = !{!299, !302, !297, !293}
!304 = !{!305}
!305 = distinct !{!305, !306, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!306 = distinct !{!306, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!307 = !{!297, !293}
!308 = !{!309}
!309 = distinct !{!309, !306, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!310 = !{!305, !299}
!311 = !{!309, !302, !297, !293}
!312 = !{!309, !302}
!313 = !{!305, !299, !297, !293}
!314 = !{!305, !309, !299, !302, !297, !293}
!315 = !{!316}
!316 = distinct !{!316, !317, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!317 = distinct !{!317, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!318 = !{!319}
!319 = distinct !{!319, !317, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!320 = !{!316, !319, !297, !293}
!321 = !{!322}
!322 = distinct !{!322, !323, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!323 = distinct !{!323, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!324 = !{!325}
!325 = distinct !{!325, !323, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!326 = !{!322, !316}
!327 = !{!325, !319, !297, !293}
!328 = !{!325, !319}
!329 = !{!322, !316, !297, !293}
!330 = !{!322, !325, !316, !319, !297, !293}
!331 = !{!332}
!332 = distinct !{!332, !333, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!333 = distinct !{!333, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!334 = !{!335}
!335 = distinct !{!335, !333, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!336 = !{!332, !335, !297, !293}
!337 = !{!338}
!338 = distinct !{!338, !339, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!339 = distinct !{!339, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!340 = !{!341}
!341 = distinct !{!341, !339, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!342 = !{!338, !332}
!343 = !{!341, !335, !297, !293}
!344 = !{!341, !335}
!345 = !{!338, !332, !297, !293}
!346 = !{!338, !341, !332, !335, !297, !293}
!347 = !{!348}
!348 = distinct !{!348, !349, !"_RINvNvMNtCsjMrxcFdYDNN_4core5sliceSp7reverse7revswapINtNtB7_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorEEB1e_: %a.0"}
!349 = distinct !{!349, !"_RINvNvMNtCsjMrxcFdYDNN_4core5sliceSp7reverse7revswapINtNtB7_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorEEB1e_"}
!350 = !{!351}
!351 = distinct !{!351, !349, !"_RINvNvMNtCsjMrxcFdYDNN_4core5sliceSp7reverse7revswapINtNtB7_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorEEB1e_: %b.0"}
!352 = !{!353, !348, !355, !357}
!353 = distinct !{!353, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It1"}
!354 = distinct !{!354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir"}
!355 = distinct !{!355, !356, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE7reverseBU_: %self.0"}
!356 = distinct !{!356, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE7reverseBU_"}
!357 = distinct !{!357, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x"}
!358 = !{!351, !297, !293}
!359 = !{!360, !351, !355, !361}
!360 = distinct !{!360, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It1"}
!361 = distinct !{!361, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y"}
!362 = !{!348, !297, !293}
!363 = !{!364, !348, !355, !365}
!364 = distinct !{!364, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It3"}
!365 = distinct !{!365, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It2"}
!366 = !{!367, !351, !355, !368}
!367 = distinct !{!367, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It3"}
!368 = distinct !{!368, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It2"}
!369 = !{!370, !348, !355, !371}
!370 = distinct !{!370, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It5"}
!371 = distinct !{!371, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It4"}
!372 = !{!373, !351, !355, !374}
!373 = distinct !{!373, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It5"}
!374 = distinct !{!374, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It4"}
!375 = !{!376}
!376 = distinct !{!376, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %x:It6"}
!377 = !{!378}
!378 = distinct !{!378, !354, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECsarYwbBXrH4d_7walkdir: %y:It6"}
!379 = !{!376, !348, !355}
!380 = !{!378, !351, !297, !293}
!381 = !{!378, !351, !355}
!382 = !{!376, !348, !297, !293}
!383 = !{!384}
!384 = distinct !{!384, !385, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_: %v.0"}
!385 = distinct !{!385, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_"}
!386 = !{!387}
!387 = distinct !{!387, !385, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5merge5mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSBX_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_: %scratch.0"}
!388 = !{!384, !387}
!389 = !{!390}
!390 = distinct !{!390, !391, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!391 = distinct !{!391, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!392 = !{!393}
!393 = distinct !{!393, !391, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!394 = !{!390, !393, !395, !384, !387}
!395 = distinct !{!395, !396, !"_RINvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB3_10MergeStateINtNtBb_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1A_5error5ErrorEE10merge_downNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1a_7sort_byNCNvMs3_B1A_NtB1A_8IntoIter4pushs_0E0EB1A_: %self"}
!396 = distinct !{!396, !"_RINvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB3_10MergeStateINtNtBb_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1A_5error5ErrorEE10merge_downNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1a_7sort_byNCNvMs3_B1A_NtB1A_8IntoIter4pushs_0E0EB1A_"}
!397 = !{!398}
!398 = distinct !{!398, !399, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!399 = distinct !{!399, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!400 = !{!401}
!401 = distinct !{!401, !399, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!402 = !{!398, !390, !387}
!403 = !{!401, !393, !395, !384}
!404 = !{!401, !393, !384}
!405 = !{!398, !390, !395, !387}
!406 = !{!398, !401, !390, !393, !395}
!407 = !{!395}
!408 = !{!409}
!409 = distinct !{!409, !410, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!410 = distinct !{!410, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!411 = !{!412}
!412 = distinct !{!412, !410, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!413 = !{!409, !412, !414, !384, !387}
!414 = distinct !{!414, !415, !"_RINvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB3_10MergeStateINtNtBb_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1A_5error5ErrorEE8merge_upNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1a_7sort_byNCNvMs3_B1A_NtB1A_8IntoIter4pushs_0E0EB1A_: %self"}
!415 = distinct !{!415, !"_RINvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB3_10MergeStateINtNtBb_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1A_5error5ErrorEE8merge_upNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1a_7sort_byNCNvMs3_B1A_NtB1A_8IntoIter4pushs_0E0EB1A_"}
!416 = !{!417}
!417 = distinct !{!417, !418, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!418 = distinct !{!418, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!419 = !{!420}
!420 = distinct !{!420, !418, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!421 = !{!417, !409, !384}
!422 = !{!420, !412, !414, !387}
!423 = !{!420, !412, !387}
!424 = !{!417, !409, !414, !384}
!425 = !{!417, !420, !409, !412, !414}
!426 = !{!414}
!427 = !{!428, !430}
!428 = distinct !{!428, !429, !"_RNvXs_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB4_10MergeStateINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1B_5error5ErrorEENtNtNtBc_3ops4drop4Drop4dropB1B_: %self"}
!429 = distinct !{!429, !"_RNvXs_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB4_10MergeStateINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1B_5error5ErrorEENtNtNtBc_3ops4drop4Drop4dropB1B_"}
!430 = distinct !{!430, !431, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6stable5merge10MergeStateINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_: %_1"}
!431 = distinct !{!431, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6stable5merge10MergeStateINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_"}
!432 = !{!433, !435}
!433 = distinct !{!433, !434, !"_RNvXs_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB4_10MergeStateINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1B_5error5ErrorEENtNtNtBc_3ops4drop4Drop4dropB1B_: %self"}
!434 = distinct !{!434, !"_RNvXs_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable5mergeINtB4_10MergeStateINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1B_5error5ErrorEENtNtNtBc_3ops4drop4Drop4dropB1B_"}
!435 = distinct !{!435, !436, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6stable5merge10MergeStateINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_: %_1"}
!436 = distinct !{!436, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6stable5merge10MergeStateINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1U_5error5ErrorEEEB1U_"}
!437 = !{!438}
!438 = distinct !{!438, !439, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort31small_sort_general_with_scratchINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1S_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1s_7sort_byNCNvMs3_B1S_NtB1S_8IntoIter4pushs_0E0EB1S_: %v.0"}
!439 = distinct !{!439, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort31small_sort_general_with_scratchINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1S_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1s_7sort_byNCNvMs3_B1S_NtB1S_8IntoIter4pushs_0E0EB1S_"}
!440 = !{!441}
!441 = distinct !{!441, !439, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort31small_sort_general_with_scratchINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1S_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1s_7sort_byNCNvMs3_B1S_NtB1S_8IntoIter4pushs_0E0EB1S_: %scratch.0"}
!442 = !{!438, !441}
!443 = !{!444}
!444 = distinct !{!444, !445, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a:It1"}
!445 = distinct !{!445, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!446 = !{!447}
!447 = distinct !{!447, !445, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b:It1"}
!448 = !{!444, !447, !438, !441}
!449 = !{!450}
!450 = distinct !{!450, !451, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a:It1"}
!451 = distinct !{!451, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!452 = !{!453}
!453 = distinct !{!453, !451, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b:It1"}
!454 = !{!450, !444, !441}
!455 = !{!453, !447, !438}
!456 = !{!453, !447, !441}
!457 = !{!450, !444, !438}
!458 = !{!450, !453, !444, !447}
!459 = !{!460}
!460 = distinct !{!460, !461, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a:It1"}
!461 = distinct !{!461, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!462 = !{!463}
!463 = distinct !{!463, !461, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b:It1"}
!464 = !{!460, !463, !438, !441}
!465 = !{!466}
!466 = distinct !{!466, !467, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a:It1"}
!467 = distinct !{!467, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!468 = !{!469}
!469 = distinct !{!469, !467, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b:It1"}
!470 = !{!466, !460}
!471 = !{!469, !463, !438, !441}
!472 = !{!469, !463, !441}
!473 = !{!466, !460, !438}
!474 = !{!466, !469, !460, !463}
!475 = !{!476, !478, !438}
!476 = distinct !{!476, !477, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_: %self"}
!477 = distinct !{!477, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_"}
!478 = distinct !{!478, !479, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_: %_1"}
!479 = distinct !{!479, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_"}
!480 = !{!481}
!481 = distinct !{!481, !482, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort19bidirectional_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1g_7sort_byNCNvMs3_B1G_NtB1G_8IntoIter4pushs_0E0EB1G_: %v.0"}
!482 = distinct !{!482, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort19bidirectional_mergeINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1g_7sort_byNCNvMs3_B1G_NtB1G_8IntoIter4pushs_0E0EB1G_"}
!483 = !{!484}
!484 = distinct !{!484, !485, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!485 = distinct !{!485, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!486 = !{!487}
!487 = distinct !{!487, !485, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!488 = !{!484, !487, !481, !438, !441}
!489 = !{!490}
!490 = distinct !{!490, !491, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!491 = distinct !{!491, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!492 = !{!493}
!493 = distinct !{!493, !491, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!494 = !{!490, !484, !481, !441}
!495 = !{!493, !487, !438}
!496 = !{!493, !487, !481, !441}
!497 = !{!490, !484, !438}
!498 = !{!490, !493, !484, !487}
!499 = !{!500, !502, !481, !438, !441}
!500 = distinct !{!500, !501, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!501 = distinct !{!501, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!502 = distinct !{!502, !501, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!503 = !{!504, !506}
!504 = distinct !{!504, !505, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort8merge_upINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_: %_0"}
!505 = distinct !{!505, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort8merge_upINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_"}
!506 = distinct !{!506, !505, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort8merge_upINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1u_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB14_7sort_byNCNvMs3_B1u_NtB1u_8IntoIter4pushs_0E0EB1u_: %is_less"}
!507 = !{!500}
!508 = !{!502}
!509 = !{!510}
!510 = distinct !{!510, !511, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!511 = distinct !{!511, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!512 = !{!513}
!513 = distinct !{!513, !511, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!514 = !{!510, !500, !481, !441}
!515 = !{!513, !502, !438}
!516 = !{!513, !502, !481, !441}
!517 = !{!510, !500, !438}
!518 = !{!510, !513, !500, !502}
!519 = !{!520, !522}
!520 = distinct !{!520, !521, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort10merge_downINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0EB1x_: %_0"}
!521 = distinct !{!521, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort10merge_downINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0EB1x_"}
!522 = distinct !{!522, !521, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsort10merge_downINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0EB1x_: %is_less"}
!523 = !{!"branch_weights", i32 4001, i32 4000000}
!524 = !{!525, !527}
!525 = distinct !{!525, !526, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_: %self"}
!526 = distinct !{!526, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_"}
!527 = distinct !{!527, !528, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_: %_1"}
!528 = distinct !{!528, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_"}
!529 = !{!530}
!530 = distinct !{!530, !445, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!531 = !{!532}
!532 = distinct !{!532, !445, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!533 = !{!530, !532, !438, !441}
!534 = !{!535}
!535 = distinct !{!535, !451, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!536 = !{!537}
!537 = distinct !{!537, !451, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!538 = !{!535, !530, !441}
!539 = !{!537, !532, !438}
!540 = !{!537, !532, !441}
!541 = !{!535, !530, !438}
!542 = !{!535, !537, !530, !532}
!543 = !{!544}
!544 = distinct !{!544, !461, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!545 = !{!546}
!546 = distinct !{!546, !461, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!547 = !{!544, !546, !438, !441}
!548 = !{!549}
!549 = distinct !{!549, !467, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!550 = !{!551}
!551 = distinct !{!551, !467, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!552 = !{!549, !544}
!553 = !{!551, !546, !438, !441}
!554 = !{!551, !546, !441}
!555 = !{!549, !544, !438}
!556 = !{!549, !551, !544, !546}
!557 = !{!558, !560, !438}
!558 = distinct !{!558, !559, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_: %self"}
!559 = distinct !{!559, !"_RNvXs5_NtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared9smallsortINtB5_10CopyOnDropINtNtBd_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1G_5error5ErrorEENtNtNtBd_3ops4drop4Drop4dropB1G_"}
!560 = distinct !{!560, !561, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_: %_1"}
!561 = distinct !{!561, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtNtB4_5slice4sort6shared9smallsort10CopyOnDropINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1Y_5error5ErrorEEEB1Y_"}
!562 = !{!563}
!563 = distinct !{!563, !564, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot12choose_pivotINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_: %v.0"}
!564 = distinct !{!564, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot12choose_pivotINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_"}
!565 = !{!566}
!566 = distinct !{!566, !564, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6shared5pivot12choose_pivotINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1v_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB15_7sort_byNCNvMs3_B1v_NtB1v_8IntoIter4pushs_0E0EB1v_: %is_less"}
!567 = !{!568}
!568 = distinct !{!568, !569, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!569 = distinct !{!569, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!570 = !{!571}
!571 = distinct !{!571, !569, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!572 = !{!568, !571, !563, !566}
!573 = !{!574}
!574 = distinct !{!574, !575, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!575 = distinct !{!575, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!576 = !{!577}
!577 = distinct !{!577, !575, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!578 = !{!574, !568, !563}
!579 = !{!577, !571, !566}
!580 = !{!577, !571, !563}
!581 = !{!574, !568, !566}
!582 = !{!574, !577, !568, !571, !563, !566}
!583 = !{!584, !586, !566}
!584 = distinct !{!584, !585, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!585 = distinct !{!585, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!586 = distinct !{!586, !585, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!587 = !{!586}
!588 = !{!589}
!589 = distinct !{!589, !590, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!590 = distinct !{!590, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!591 = !{!589, !586, !563}
!592 = !{!593, !584, !566}
!593 = distinct !{!593, !590, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!594 = !{!593, !589, !584, !586, !566}
!595 = !{!596, !598, !566}
!596 = distinct !{!596, !597, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!597 = distinct !{!597, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!598 = distinct !{!598, !597, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!599 = !{!600, !602, !596, !598, !566}
!600 = distinct !{!600, !601, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!601 = distinct !{!601, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!602 = distinct !{!602, !601, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!603 = !{!604}
!604 = distinct !{!604, !605, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!605 = distinct !{!605, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!606 = !{!607}
!607 = distinct !{!607, !605, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!608 = !{!604, !607}
!609 = !{!610}
!610 = distinct !{!610, !611, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!611 = distinct !{!611, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!612 = !{!613}
!613 = distinct !{!613, !611, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!614 = !{!610, !604}
!615 = !{!613, !607}
!616 = !{!610, !613, !604, !607}
!617 = !{!618}
!618 = distinct !{!618, !619, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0EB1D_: %v.0"}
!619 = distinct !{!619, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0EB1D_"}
!620 = !{!621}
!621 = distinct !{!621, !619, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0EB1D_: %scratch.0"}
!622 = !{!623}
!623 = distinct !{!623, !624, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!624 = distinct !{!624, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!625 = !{!626}
!626 = distinct !{!626, !624, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!627 = !{!623, !626, !618, !621}
!628 = !{!629}
!629 = distinct !{!629, !630, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!630 = distinct !{!630, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!631 = !{!632}
!632 = distinct !{!632, !630, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!633 = !{!629, !623, !618}
!634 = !{!632, !626, !621}
!635 = !{!632, !626, !618}
!636 = !{!629, !623, !621}
!637 = !{!629, !632, !623, !626, !621}
!638 = !{!618, !621}
!639 = !{!640}
!640 = distinct !{!640, !641, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_: %self"}
!641 = distinct !{!641, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_"}
!642 = !{!643}
!643 = distinct !{!643, !644, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_: %self"}
!644 = distinct !{!644, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_"}
!645 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!646 = !{!647, !649}
!647 = distinct !{!647, !648, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_: %pair"}
!648 = distinct !{!648, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_"}
!649 = distinct !{!649, !648, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSINtNtB4_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtBU_5error5ErrorE12split_at_mutBU_: %self.0"}
!650 = !{!651}
!651 = distinct !{!651, !652, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_: %v.0"}
!652 = distinct !{!652, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_"}
!653 = !{!654}
!654 = distinct !{!654, !652, !"_RINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort16stable_partitionINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1D_5error5ErrorENCINvB2_9quicksortB1d_NCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB1d_7sort_byNCNvMs3_B1D_NtB1D_8IntoIter4pushs_0E0E0EB1D_: %scratch.0"}
!655 = !{!656}
!656 = distinct !{!656, !657, !"_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_: %a"}
!657 = distinct !{!657, !"_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_"}
!658 = !{!659}
!659 = distinct !{!659, !657, !"_RNCINvNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksort9quicksortINtNtBc_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorENCINvMNtCsdJPVW0sQgAG_5alloc5sliceSB17_7sort_byNCNvMs3_B1x_NtB1x_8IntoIter4pushs_0E0E0B1x_: %b"}
!660 = !{!656, !659, !651, !654}
!661 = !{!662}
!662 = distinct !{!662, !663, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %a"}
!663 = distinct !{!663, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_"}
!664 = !{!665}
!665 = distinct !{!665, !663, !"_RNCINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1e_5error5ErrorE7sort_byNCNvMs3_B1e_NtB1e_8IntoIter4pushs_0E0B1e_: %b"}
!666 = !{!662, !665, !656, !659, !654}
!667 = !{!668}
!668 = distinct !{!668, !669, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %a"}
!669 = distinct !{!669, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_"}
!670 = !{!671}
!671 = distinct !{!671, !669, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4pushs_0B7_: %b"}
!672 = !{!668, !662, !659, !651}
!673 = !{!671, !665, !656, !654}
!674 = !{!671, !665, !656, !651}
!675 = !{!668, !662, !659, !654}
!676 = !{!668, !671, !662, !665, !656, !659, !654}
!677 = !{!651, !654}
!678 = !{!679}
!679 = distinct !{!679, !680, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_: %self"}
!680 = distinct !{!680, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_"}
!681 = !{!682}
!682 = distinct !{!682, !683, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_: %self"}
!683 = distinct !{!683, !"_RNvMNtNtNtNtCsjMrxcFdYDNN_4core5slice4sort6stable9quicksortINtB2_14PartitionStateINtNtBa_6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEE13partition_oneB1H_"}
!684 = !{!685}
!685 = distinct !{!685, !686, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir: %self"}
!686 = distinct !{!686, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir"}
!687 = !{i64 0, i64 2}
!688 = !{!689}
!689 = distinct !{!689, !690, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name: %self"}
!690 = distinct !{!690, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name"}
!691 = !{!692}
!692 = distinct !{!692, !693, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name: %self"}
!693 = distinct !{!693, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name"}
!694 = !{!695, !697}
!695 = distinct !{!695, !696, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name: %self"}
!696 = distinct !{!696, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name"}
!697 = distinct !{!697, !698, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_: %a"}
!698 = distinct !{!698, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_"}
!699 = !{!700}
!700 = distinct !{!700, !698, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_: %b"}
!701 = !{!702, !704, !706}
!702 = distinct !{!702, !703, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name: %self"}
!703 = distinct !{!703, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name"}
!704 = distinct !{!704, !705, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_: %a"}
!705 = distinct !{!705, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_"}
!706 = distinct !{!706, !705, !"_RNCNvMs_CsarYwbBXrH4d_7walkdirNtB6_7WalkDir17sort_by_file_name0B6_: %b"}
!707 = !{!708, !704, !706}
!708 = distinct !{!708, !709, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name: %self"}
!709 = distinct !{!709, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry9file_name"}
!710 = !{!704, !706}
!711 = !{i16 0, i16 2}
!712 = !{!713}
!713 = distinct !{!713, !714, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry10from_entry0B6_: %_0"}
!714 = distinct !{!714, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry10from_entry0B6_"}
!715 = !{!716}
!716 = distinct !{!716, !717, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal: %_0"}
!717 = distinct !{!717, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal"}
!718 = !{!719}
!719 = distinct !{!719, !717, !"_RNvMNtCsarYwbBXrH4d_7walkdir4dentNtB2_8DirEntry17metadata_internal: %self"}
!720 = !{i8 0, i8 2}
!721 = !{!722}
!722 = distinct !{!722, !723, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: argument 1"}
!723 = distinct !{!723, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir"}
!724 = !{!725, !722, !716, !719}
!725 = distinct !{!725, !723, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!726 = !{!722, !719}
!727 = !{!725, !716}
!728 = !{!722, !716, !719}
!729 = !{!730}
!730 = distinct !{!730, !731, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: argument 1"}
!731 = distinct !{!731, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir"}
!732 = !{!733, !730, !716, !719}
!733 = distinct !{!733, !731, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!734 = !{!730, !719}
!735 = !{!733, !716}
!736 = !{!730, !716, !719}
!737 = !{!716, !719}
!738 = !{!739}
!739 = distinct !{!739, !740, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry: %dent"}
!740 = distinct !{!740, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry"}
!741 = !{!742, !739, !716, !719}
!742 = distinct !{!742, !740, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry: %_0"}
!743 = !{!739, !719}
!744 = !{!742, !716}
!745 = !{!746}
!746 = distinct !{!746, !747, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: argument 1"}
!747 = distinct !{!747, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir"}
!748 = !{!749, !746}
!749 = distinct !{!749, !747, !"_RINvNtCs5sEH5CPMdak_3std2fs16symlink_metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!750 = !{!749}
!751 = !{!752}
!752 = distinct !{!752, !753, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: argument 1"}
!753 = distinct !{!753, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir"}
!754 = !{!755, !752}
!755 = distinct !{!755, !753, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!756 = !{!755}
!757 = !{!758}
!758 = distinct !{!758, !759, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!759 = distinct !{!759, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!760 = !{!761}
!761 = distinct !{!761, !762, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry9from_paths_0B6_: %_0"}
!762 = distinct !{!762, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry9from_paths_0B6_"}
!763 = !{!764, !766, !768, !769, !761}
!764 = distinct !{!764, !765, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!765 = distinct !{!765, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!766 = distinct !{!766, !767, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %v"}
!767 = distinct !{!767, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir"}
!768 = distinct !{!768, !767, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %s.0"}
!769 = distinct !{!769, !770, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir: %_0"}
!770 = distinct !{!770, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir"}
!771 = !{!766, !769, !761}
!772 = !{!773}
!773 = distinct !{!773, !774, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!774 = distinct !{!774, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!775 = !{!776}
!776 = distinct !{!776, !777, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry9from_path0B6_: %_0"}
!777 = distinct !{!777, !"_RNCNvMNtCsarYwbBXrH4d_7walkdir4dentNtB4_8DirEntry9from_path0B6_"}
!778 = !{!779, !781, !783, !784, !776}
!779 = distinct !{!779, !780, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!780 = distinct !{!780, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!781 = distinct !{!781, !782, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %v"}
!782 = distinct !{!782, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir"}
!783 = distinct !{!783, !782, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %s.0"}
!784 = distinct !{!784, !785, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir: %_0"}
!785 = distinct !{!785, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir"}
!786 = !{!781, !784, !776}
!787 = !{!788}
!788 = distinct !{!788, !789, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_: %_1"}
!789 = distinct !{!789, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsarYwbBXrH4d_7walkdir5error10ErrorInnerEBK_"}
!790 = !{!791}
!791 = distinct !{!791, !792, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!792 = distinct !{!792, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!793 = !{!791, !788}
!794 = !{!795}
!795 = distinct !{!795, !796, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!796 = distinct !{!796, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!797 = !{!795, !788}
!798 = !{!799}
!799 = distinct !{!799, !800, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir: %_1"}
!800 = distinct !{!800, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std4path7PathBufEECsarYwbBXrH4d_7walkdir"}
!801 = !{!802}
!802 = distinct !{!802, !803, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!803 = distinct !{!803, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!804 = !{!802, !799}
!805 = !{!806}
!806 = distinct !{!806, !807, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter6follow: %self"}
!807 = distinct !{!807, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter6follow"}
!808 = !{!809, !806, !810}
!809 = distinct !{!809, !807, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter6follow: %_0"}
!810 = distinct !{!810, !807, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter6follow: %dent"}
!811 = !{!809, !810}
!812 = !{!813, !809, !806, !810}
!813 = distinct !{!813, !814, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!814 = distinct !{!814, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!815 = !{!816, !809, !806, !810}
!816 = distinct !{!816, !817, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!817 = distinct !{!817, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!818 = !{!819}
!819 = distinct !{!819, !820, !"_RINvMs3_CsarYwbBXrH4d_7walkdirNtB6_8IntoIter10check_loopRNtNtCs5sEH5CPMdak_3std4path4PathEB6_: %self"}
!820 = distinct !{!820, !"_RINvMs3_CsarYwbBXrH4d_7walkdirNtB6_8IntoIter10check_loopRNtNtCs5sEH5CPMdak_3std4path4PathEB6_"}
!821 = !{!822, !819, !823, !809, !806, !810}
!822 = distinct !{!822, !820, !"_RINvMs3_CsarYwbBXrH4d_7walkdirNtB6_8IntoIter10check_loopRNtNtCs5sEH5CPMdak_3std4path4PathEB6_: %_0"}
!823 = distinct !{!823, !820, !"_RINvMs3_CsarYwbBXrH4d_7walkdirNtB6_8IntoIter10check_loopRNtNtCs5sEH5CPMdak_3std4path4PathEB6_: argument 2"}
!824 = !{!825, !822, !819, !823, !809, !806, !810}
!825 = distinct !{!825, !826, !"_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir: %_0"}
!826 = distinct !{!826, !"_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRRNtNtCs5sEH5CPMdak_3std4path4PathECsarYwbBXrH4d_7walkdir"}
!827 = !{i32 0, i32 2}
!828 = !{i32 0, i32 -1}
!829 = !{i8 0, i8 3}
!830 = !{!822, !819, !809, !806, !810}
!831 = !{!819, !806}
!832 = !{!822, !823, !809, !810}
!833 = !{!834}
!834 = distinct !{!834, !835, !"_RINvMsl_NtCs5sEH5CPMdak_3std2fsNtB6_11OpenOptions4openRNtNtB8_4path7PathBufECsarYwbBXrH4d_7walkdir: argument 2"}
!835 = distinct !{!835, !"_RINvMsl_NtCs5sEH5CPMdak_3std2fsNtB6_11OpenOptions4openRNtNtB8_4path7PathBufECsarYwbBXrH4d_7walkdir"}
!836 = !{!837, !838, !822, !819, !809, !806, !810}
!837 = distinct !{!837, !835, !"_RINvMsl_NtCs5sEH5CPMdak_3std2fsNtB6_11OpenOptions4openRNtNtB8_4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!838 = distinct !{!838, !835, !"_RINvMsl_NtCs5sEH5CPMdak_3std2fsNtB6_11OpenOptions4openRNtNtB8_4path7PathBufECsarYwbBXrH4d_7walkdir: %self"}
!839 = !{!840, !822, !819, !823, !809, !806, !810}
!840 = distinct !{!840, !841, !"_RNvMs1_CsarYwbBXrH4d_7walkdirNtB5_8Ancestor7is_same: %_0"}
!841 = distinct !{!841, !"_RNvMs1_CsarYwbBXrH4d_7walkdirNtB5_8Ancestor7is_same"}
!842 = !{!843, !840, !822, !819, !823, !809, !806, !810}
!843 = distinct !{!843, !844, !"_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_0"}
!844 = distinct !{!844, !"_RINvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB6_6Handle9from_pathRNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!845 = !{!840, !822, !819, !809, !806, !810}
!846 = !{!847, !849}
!847 = distinct !{!847, !848, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!848 = distinct !{!848, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir"}
!849 = distinct !{!849, !850, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!850 = distinct !{!850, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir"}
!851 = !{!852, !854}
!852 = distinct !{!852, !853, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!853 = distinct !{!853, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir"}
!854 = distinct !{!854, !855, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!855 = distinct !{!855, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir"}
!856 = !{!857, !859, !860, !822, !819, !823, !809, !806, !810}
!857 = distinct !{!857, !858, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error9from_loop: %_0"}
!858 = distinct !{!858, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error9from_loop"}
!859 = distinct !{!859, !858, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error9from_loop: %ancestor.0"}
!860 = distinct !{!860, !858, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error9from_loop: %child.0"}
!861 = !{!857, !822, !819, !809, !806, !810}
!862 = !{!863}
!863 = distinct !{!863, !864, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!864 = distinct !{!864, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!865 = !{!863, !857, !822, !819, !809, !806, !810}
!866 = !{!867, !869}
!867 = distinct !{!867, !868, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!868 = distinct !{!868, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleECsarYwbBXrH4d_7walkdir"}
!869 = distinct !{!869, !870, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir: %_1"}
!870 = distinct !{!870, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleECsarYwbBXrH4d_7walkdir"}
!871 = !{!872, !809, !806, !810}
!872 = distinct !{!872, !873, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!873 = distinct !{!873, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!874 = !{!875, !877}
!875 = distinct !{!875, !876, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: %_0"}
!876 = distinct !{!876, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir"}
!877 = distinct !{!877, !876, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: argument 1"}
!878 = !{!877}
!879 = !{!880, !882}
!880 = distinct !{!880, !881, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter12handle_entry0B7_: %_0"}
!881 = distinct !{!881, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter12handle_entry0B7_"}
!882 = distinct !{!882, !881, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter12handle_entry0B7_: %_1"}
!883 = !{!882}
!884 = !{!885}
!885 = distinct !{!885, !886, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!886 = distinct !{!886, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!887 = !{!888}
!888 = distinct !{!888, !889, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter19is_same_file_system: %dent"}
!889 = distinct !{!889, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter19is_same_file_system"}
!890 = !{!891}
!891 = distinct !{!891, !889, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter19is_same_file_system: %_0"}
!892 = !{!893, !895, !896, !891, !888}
!893 = distinct !{!893, !894, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: %_0"}
!894 = distinct !{!894, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir"}
!895 = distinct !{!895, !894, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: argument 1"}
!896 = distinct !{!896, !897, !"_RINvNtCsarYwbBXrH4d_7walkdir4util10device_numRNtNtCs5sEH5CPMdak_3std4path4PathEB4_: argument 0"}
!897 = distinct !{!897, !"_RINvNtCsarYwbBXrH4d_7walkdir4util10device_numRNtNtCs5sEH5CPMdak_3std4path4PathEB4_"}
!898 = !{!895, !896, !891, !888}
!899 = !{!891, !888}
!900 = !{!901, !903, !891, !888}
!901 = distinct !{!901, !902, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry: %_0"}
!902 = distinct !{!902, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry"}
!903 = distinct !{!903, !902, !"_RNvMNtCsarYwbBXrH4d_7walkdir5errorNtB2_5Error10from_entry: %dent"}
!904 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!905 = !{!903, !891, !888}
!906 = !{!907}
!907 = distinct !{!907, !908, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryE8push_mutBJ_: %self"}
!908 = distinct !{!908, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryE8push_mutBJ_"}
!909 = !{!910}
!910 = distinct !{!910, !908, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryE8push_mutBJ_: %value"}
!911 = !{!912, !907, !910}
!912 = distinct !{!912, !913, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!913 = distinct !{!913, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!914 = !{!915}
!915 = distinct !{!915, !916, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!916 = distinct !{!916, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!917 = !{!918}
!918 = distinct !{!918, !919, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!919 = distinct !{!919, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!920 = !{!921}
!921 = distinct !{!921, !922, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!922 = distinct !{!922, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!923 = !{!924, !926}
!924 = distinct !{!924, !925, !"_RINvNtCs5sEH5CPMdak_3std2fs8read_dirRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: %_0"}
!925 = distinct !{!925, !"_RINvNtCs5sEH5CPMdak_3std2fs8read_dirRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir"}
!926 = distinct !{!926, !925, !"_RINvNtCs5sEH5CPMdak_3std2fs8read_dirRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: argument 1"}
!927 = !{!924}
!928 = !{!929}
!929 = distinct !{!929, !930, !"_RNvMs5_CsarYwbBXrH4d_7walkdirNtB5_7DirList5close: %self"}
!930 = distinct !{!930, !"_RNvMs5_CsarYwbBXrH4d_7walkdirNtB5_7DirList5close"}
!931 = !{!932, !934, !929}
!932 = distinct !{!932, !933, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_: %_0"}
!933 = distinct !{!933, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_"}
!934 = distinct !{!934, !933, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_QNtB1H_7DirListE9from_iterB1H_: argument 1"}
!935 = !{!932}
!936 = !{i64 0, i64 -9223372036854775805}
!937 = !{!938, !932}
!938 = distinct !{!938, !939, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!939 = distinct !{!939, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!940 = !{!941}
!941 = distinct !{!941, !942, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_QNtB1x_7DirListE11spec_extendB1x_: %self"}
!942 = distinct !{!942, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_QNtB1x_7DirListE11spec_extendB1x_"}
!943 = !{!944}
!944 = distinct !{!944, !945, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredQNtB1m_7DirListEB1m_: %self"}
!945 = distinct !{!945, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredQNtB1m_7DirListEB1m_"}
!946 = !{!941, !947, !932, !934, !929}
!947 = distinct !{!947, !942, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_QNtB1x_7DirListE11spec_extendB1x_: %iter"}
!948 = !{!944, !949, !941, !947, !932, !934, !929}
!949 = distinct !{!949, !945, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredQNtB1m_7DirListEB1m_: argument 1"}
!950 = !{!944, !941}
!951 = !{!949, !947, !932, !934, !929}
!952 = !{!944, !941, !932}
!953 = !{!934, !929}
!954 = !{!955}
!955 = distinct !{!955, !956, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_: %_0"}
!956 = distinct !{!956, !"_RNCNvMs3_CsarYwbBXrH4d_7walkdirNtB7_8IntoIter4push0B7_"}
!957 = !{!958}
!958 = distinct !{!958, !959, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_NtB1H_7DirListE9from_iterB1H_: %_0"}
!959 = distinct !{!959, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_NtB1H_7DirListE9from_iterB1H_"}
!960 = !{!961}
!961 = distinct !{!961, !959, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1H_5error5ErrorEEINtB2_18SpecFromIterNestedB11_NtB1H_7DirListE9from_iterB1H_: %iterator"}
!962 = !{!958, !961}
!963 = !{!964}
!964 = distinct !{!964, !965, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!965 = distinct !{!965, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!966 = !{!964, !961}
!967 = !{!968, !958}
!968 = distinct !{!968, !965, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!969 = !{!970}
!970 = distinct !{!970, !971, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %self"}
!971 = distinct !{!971, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_"}
!972 = !{!970, !961}
!973 = !{!974, !958}
!974 = distinct !{!974, !971, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %_0"}
!975 = !{!970, !958}
!976 = !{!968, !964, !958, !961}
!977 = !{!978}
!978 = distinct !{!978, !979, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %_0"}
!979 = distinct !{!979, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_"}
!980 = !{!981, !958, !961}
!981 = distinct !{!981, !979, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %r"}
!982 = !{!978, !981, !958, !961}
!983 = !{!978, !958, !961}
!984 = !{!981, !958}
!985 = !{!986}
!986 = distinct !{!986, !987, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!987 = distinct !{!987, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!988 = !{!989}
!989 = distinct !{!989, !990, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!990 = distinct !{!990, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!991 = !{!992}
!992 = distinct !{!992, !993, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!993 = distinct !{!993, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!994 = !{!995}
!995 = distinct !{!995, !996, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!996 = distinct !{!996, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!997 = !{!995, !992, !989, !986}
!998 = !{!995, !992, !989, !986, !978, !981, !958}
!999 = !{!978, !981, !958}
!1000 = !{!1001}
!1001 = distinct !{!1001, !1002, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1002 = distinct !{!1002, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1003 = !{!1004}
!1004 = distinct !{!1004, !1005, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1005 = distinct !{!1005, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1006 = !{!1007}
!1007 = distinct !{!1007, !1008, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1008 = distinct !{!1008, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1009 = !{!1010}
!1010 = distinct !{!1010, !1011, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1011 = distinct !{!1011, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1012 = !{!1010, !1007, !1004, !1001}
!1013 = !{!1010, !1007, !1004, !1001, !978, !981, !958}
!1014 = !{!964, !958, !961}
!1015 = !{!1016, !1018}
!1016 = distinct !{!1016, !1017, !"_RINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1c_5error5ErrorE7sort_byNCNvMs3_B1c_NtB1c_8IntoIter4pushs_0EB1c_: %self.0"}
!1017 = distinct !{!1017, !"_RINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1c_5error5ErrorE7sort_byNCNvMs3_B1c_NtB1c_8IntoIter4pushs_0EB1c_"}
!1018 = distinct !{!1018, !1017, !"_RINvMNtCsdJPVW0sQgAG_5alloc5sliceSINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1c_5error5ErrorE7sort_byNCNvMs3_B1c_NtB1c_8IntoIter4pushs_0EB1c_: argument 1"}
!1019 = !{!1020, !958}
!1020 = distinct !{!1020, !1021, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!1021 = distinct !{!1021, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!1022 = !{!1023}
!1023 = distinct !{!1023, !1024, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_NtB1x_7DirListE11spec_extendB1x_: %self"}
!1024 = distinct !{!1024, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_NtB1x_7DirListE11spec_extendB1x_"}
!1025 = !{!1026}
!1026 = distinct !{!1026, !1024, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1x_5error5ErrorEEINtB2_10SpecExtendBR_NtB1x_7DirListE11spec_extendB1x_: %iter"}
!1027 = !{!1028}
!1028 = distinct !{!1028, !1029, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_: %self"}
!1029 = distinct !{!1029, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_"}
!1030 = !{!1031}
!1031 = distinct !{!1031, !1029, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1m_5error5ErrorEE16extend_desugaredNtB1m_7DirListEB1m_: %iterator"}
!1032 = !{!1023, !1026, !958, !961}
!1033 = !{!1034}
!1034 = distinct !{!1034, !1035, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!1035 = distinct !{!1035, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!1036 = !{!1034, !1031, !1026}
!1037 = !{!1038, !1028, !1023, !958, !961}
!1038 = distinct !{!1038, !1035, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!1039 = !{!1040}
!1040 = distinct !{!1040, !1041, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %self"}
!1041 = distinct !{!1041, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_"}
!1042 = !{!1040, !1031, !1026}
!1043 = !{!1044, !1028, !1023, !958, !961}
!1044 = distinct !{!1044, !1041, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %_0"}
!1045 = !{!1040, !1028, !1023, !958}
!1046 = !{!1038, !1034, !1028, !1031, !1023, !1026, !958, !961}
!1047 = !{!1028, !1023, !958}
!1048 = !{!1049}
!1049 = distinct !{!1049, !1050, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %_0"}
!1050 = distinct !{!1050, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_"}
!1051 = !{!1052, !1028, !1031, !1023, !1026, !958, !961}
!1052 = distinct !{!1052, !1050, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %r"}
!1053 = !{!1049, !1052, !1028, !1031, !1023, !1026, !958, !961}
!1054 = !{!1049, !1028, !1031, !1023, !1026, !958, !961}
!1055 = !{!1028, !1031, !1023, !1026, !958, !961}
!1056 = !{!1052, !1028, !1023, !958}
!1057 = !{!1058}
!1058 = distinct !{!1058, !1059, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1059 = distinct !{!1059, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1060 = !{!1061}
!1061 = distinct !{!1061, !1062, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1062 = distinct !{!1062, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1063 = !{!1064}
!1064 = distinct !{!1064, !1065, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1065 = distinct !{!1065, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1066 = !{!1067}
!1067 = distinct !{!1067, !1068, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1068 = distinct !{!1068, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1069 = !{!1067, !1064, !1061, !1058}
!1070 = !{!1067, !1064, !1061, !1058, !1049, !1052, !1028, !1023, !958}
!1071 = !{!1049, !1052, !1028, !1023, !958}
!1072 = !{!1073}
!1073 = distinct !{!1073, !1074, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1074 = distinct !{!1074, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1075 = !{!1076}
!1076 = distinct !{!1076, !1077, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1077 = distinct !{!1077, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1078 = !{!1079}
!1079 = distinct !{!1079, !1080, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1080 = distinct !{!1080, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1081 = !{!1082}
!1082 = distinct !{!1082, !1083, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1083 = distinct !{!1083, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1084 = !{!1082, !1079, !1076, !1073}
!1085 = !{!1082, !1079, !1076, !1073, !1049, !1052, !1028, !1023, !958}
!1086 = !{!1034, !1028, !1031, !1023, !1026, !958, !961}
!1087 = !{!1028, !1023, !958, !961}
!1088 = !{!1028, !1023}
!1089 = !{!1031, !1026, !958, !961}
!1090 = !{!1091, !1093, !1016, !1018}
!1091 = distinct !{!1091, !1092, !"_RINvNtCsdJPVW0sQgAG_5alloc5slice11stable_sortINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMB2_SBH_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_: %v.0"}
!1092 = distinct !{!1092, !"_RINvNtCsdJPVW0sQgAG_5alloc5slice11stable_sortINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMB2_SBH_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_"}
!1093 = distinct !{!1093, !1092, !"_RINvNtCsdJPVW0sQgAG_5alloc5slice11stable_sortINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1n_5error5ErrorENCINvMB2_SBH_7sort_byNCNvMs3_B1n_NtB1n_8IntoIter4pushs_0E0EB1n_: argument 1"}
!1094 = !{!"branch_weights", !"expected", i32 2144841883, i32 2641765}
!1095 = !{!1096}
!1096 = distinct !{!1096, !1097, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir7DirListE8push_mutBH_: %self"}
!1097 = distinct !{!1097, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir7DirListE8push_mutBH_"}
!1098 = !{!1099}
!1099 = distinct !{!1099, !1097, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir7DirListE8push_mutBH_: %value"}
!1100 = !{!1101}
!1101 = distinct !{!1101, !1102, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_: %self"}
!1102 = distinct !{!1102, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_"}
!1103 = !{!1104}
!1104 = distinct !{!1104, !1102, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsarYwbBXrH4d_7walkdir8AncestorE8push_mutBH_: %value"}
!1105 = !{!1106, !1101, !1104}
!1106 = distinct !{!1106, !1107, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!1107 = distinct !{!1107, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!1108 = !{!1109}
!1109 = distinct !{!1109, !1110, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir: %self"}
!1110 = distinct !{!1110, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir"}
!1111 = !{!1112}
!1112 = distinct !{!1112, !1113, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir: %self"}
!1113 = distinct !{!1113, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir"}
!1114 = !{!1115}
!1115 = distinct !{!1115, !1116, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir: %self"}
!1116 = distinct !{!1116, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsarYwbBXrH4d_7walkdir"}
!1117 = !{!"branch_weights", i32 2002, i32 2000}
!1118 = !{!1119}
!1119 = distinct !{!1119, !1120, !"_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_: %_0"}
!1120 = distinct !{!1120, !"_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_"}
!1121 = !{!1122}
!1122 = distinct !{!1122, !1120, !"_RINvMs_CsarYwbBXrH4d_7walkdirNtB5_7WalkDir7sort_byNCNvB2_17sort_by_file_name0EB5_: %self"}
!1123 = !{!1119, !1122}
!1124 = !{!1125}
!1125 = distinct !{!1125, !1126, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!1126 = distinct !{!1126, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!1127 = !{!1128}
!1128 = distinct !{!1128, !1126, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!1129 = !{!1130}
!1130 = distinct !{!1130, !1131, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %_0"}
!1131 = distinct !{!1131, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_"}
!1132 = !{!1133}
!1133 = distinct !{!1133, !1131, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %self"}
!1134 = !{!1125, !1128}
!1135 = !{!1136}
!1136 = distinct !{!1136, !1137, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %_0"}
!1137 = distinct !{!1137, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_"}
!1138 = !{!1139}
!1139 = distinct !{!1139, !1137, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %r"}
!1140 = !{!1136, !1139}
!1141 = !{!1142, !1144, !1146, !1148, !1136, !1139}
!1142 = distinct !{!1142, !1143, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1143 = distinct !{!1143, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1144 = distinct !{!1144, !1145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1145 = distinct !{!1145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1146 = distinct !{!1146, !1147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1147 = distinct !{!1147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1148 = distinct !{!1148, !1149, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1149 = distinct !{!1149, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1150 = !{!1151, !1153, !1155, !1157, !1136, !1139}
!1151 = distinct !{!1151, !1152, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1152 = distinct !{!1152, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1153 = distinct !{!1153, !1154, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1154 = distinct !{!1154, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1155 = distinct !{!1155, !1156, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1156 = distinct !{!1156, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1157 = distinct !{!1157, !1158, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1158 = distinct !{!1158, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1159 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000, i32 2000}
!1160 = !{i8 0, i8 42}
!1161 = !{!1162}
!1162 = distinct !{!1162, !1163, !"_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt: %self"}
!1163 = distinct !{!1163, !"_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt"}
!1164 = !{!1165}
!1165 = distinct !{!1165, !1163, !"_RNvXs3_NtCsarYwbBXrH4d_7walkdir5errorNtB5_10ErrorInnerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt: %f"}
!1166 = !{!1162, !1165}
!1167 = !{!1168, !1170, !1171}
!1168 = distinct !{!1168, !1169, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: %_0"}
!1169 = distinct !{!1169, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir"}
!1170 = distinct !{!1170, !1169, !"_RINvNtCs5sEH5CPMdak_3std2fs8metadataRNtNtB4_4path4PathECsarYwbBXrH4d_7walkdir: argument 1"}
!1171 = distinct !{!1171, !1172, !"_RINvNtCsarYwbBXrH4d_7walkdir4util10device_numRNtNtCs5sEH5CPMdak_3std4path7PathBufEB4_: argument 0"}
!1172 = distinct !{!1172, !"_RINvNtCsarYwbBXrH4d_7walkdir4util10device_numRNtNtCs5sEH5CPMdak_3std4path7PathBufEB4_"}
!1173 = !{!1170, !1171}
!1174 = !{!1175}
!1175 = distinct !{!1175, !1176, !"_RNCNvXs2_CsarYwbBXrH4d_7walkdirNtB7_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %_0"}
!1176 = distinct !{!1176, !"_RNCNvXs2_CsarYwbBXrH4d_7walkdirNtB7_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_"}
!1177 = !{!1178, !1180, !1182, !1183, !1175}
!1178 = distinct !{!1178, !1179, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!1179 = distinct !{!1179, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!1180 = distinct !{!1180, !1181, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %v"}
!1181 = distinct !{!1181, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir"}
!1182 = distinct !{!1182, !1181, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %s.0"}
!1183 = distinct !{!1183, !1184, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir: %_0"}
!1184 = distinct !{!1184, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir"}
!1185 = !{!1180, !1183, !1175}
!1186 = !{!1187}
!1187 = distinct !{!1187, !1188, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!1188 = distinct !{!1188, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!1189 = !{!1190}
!1190 = distinct !{!1190, !1191, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!1191 = distinct !{!1191, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!1192 = !{!1193}
!1193 = distinct !{!1193, !1194, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir: %self"}
!1194 = distinct !{!1194, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir"}
!1195 = !{!1196}
!1196 = distinct !{!1196, !1194, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir: %_0"}
!1197 = !{!1196, !1193}
!1198 = !{!1199, !1196, !1193}
!1199 = distinct !{!1199, !1200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!1200 = distinct !{!1200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!1201 = !{!1202}
!1202 = distinct !{!1202, !1203, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir: %self"}
!1203 = distinct !{!1203, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir"}
!1204 = !{!1205}
!1205 = distinct !{!1205, !1203, !"_RNvMs3_CsarYwbBXrH4d_7walkdirNtB5_8IntoIter16get_deferred_dir: %_0"}
!1206 = !{!1205, !1202}
!1207 = !{!1208, !1205, !1202}
!1208 = distinct !{!1208, !1209, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir: %_1"}
!1209 = distinct !{!1209, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std4path7PathBufECsarYwbBXrH4d_7walkdir"}
!1210 = !{!1211}
!1211 = distinct !{!1211, !1212, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!1212 = distinct !{!1212, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!1213 = !{!1214}
!1214 = distinct !{!1214, !1212, !"_RNvXs6_CsarYwbBXrH4d_7walkdirNtB5_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!1215 = !{!1216}
!1216 = distinct !{!1216, !1217, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %_0"}
!1217 = distinct !{!1217, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_"}
!1218 = !{!1219}
!1219 = distinct !{!1219, !1217, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsjMrxcFdYDNN_4core6result6ResultNtNtCsarYwbBXrH4d_7walkdir4dent8DirEntryNtNtB1C_5error5ErrorEENtNtNtNtB11_4iter6traits8iterator8Iterator4nextB1C_: %self"}
!1220 = !{!1211, !1214}
!1221 = !{!1222}
!1222 = distinct !{!1222, !1223, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %_0"}
!1223 = distinct !{!1223, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_"}
!1224 = !{!1225}
!1225 = distinct !{!1225, !1223, !"_RNCNvXs6_CsarYwbBXrH4d_7walkdirNtB7_7DirListNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next0B7_: %r"}
!1226 = !{!1222, !1225}
!1227 = !{!1228}
!1228 = distinct !{!1228, !1229, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1229 = distinct !{!1229, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1230 = !{!1231}
!1231 = distinct !{!1231, !1232, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1232 = distinct !{!1232, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1233 = !{!1234}
!1234 = distinct !{!1234, !1235, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1235 = distinct !{!1235, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1236 = !{!1237}
!1237 = distinct !{!1237, !1238, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1238 = distinct !{!1238, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1239 = !{!1237, !1234, !1231, !1228}
!1240 = !{!1237, !1234, !1231, !1228, !1222, !1225}
!1241 = !{!1242}
!1242 = distinct !{!1242, !1243, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1243 = distinct !{!1243, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std2fs8DirEntryECsarYwbBXrH4d_7walkdir"}
!1244 = !{!1245}
!1245 = distinct !{!1245, !1246, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir: %_1"}
!1246 = distinct !{!1246, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix8DirEntryECsarYwbBXrH4d_7walkdir"}
!1247 = !{!1248}
!1248 = distinct !{!1248, !1249, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir: %_1"}
!1249 = distinct !{!1249, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirEECsarYwbBXrH4d_7walkdir"}
!1250 = !{!1251}
!1251 = distinct !{!1251, !1252, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir: %self"}
!1252 = distinct !{!1252, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtNtCs5sEH5CPMdak_3std3sys2fs4unix12InnerReadDirENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsarYwbBXrH4d_7walkdir"}
!1253 = !{!1251, !1248, !1245, !1242}
!1254 = !{!1251, !1248, !1245, !1242, !1222, !1225}
!1255 = !{!1256, !1258, !1260, !1261}
!1256 = distinct !{!1256, !1257, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir: %_0"}
!1257 = distinct !{!1257, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCsarYwbBXrH4d_7walkdir"}
!1258 = distinct !{!1258, !1259, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %v"}
!1259 = distinct !{!1259, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir"}
!1260 = distinct !{!1260, !1259, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECsarYwbBXrH4d_7walkdir: %s.0"}
!1261 = distinct !{!1261, !1262, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir: %_0"}
!1262 = distinct !{!1262, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCsarYwbBXrH4d_7walkdir"}
!1263 = !{!1258, !1260, !1261}
!1264 = !{!1258, !1261}
