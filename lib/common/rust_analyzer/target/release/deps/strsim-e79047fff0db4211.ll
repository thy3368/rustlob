; ModuleID = 'strsim.4eaa3c055b8b769f-cgu.0'
source_filename = "strsim.4eaa3c055b8b769f-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<(u64, u64)>, !>" = type { %"core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>>", i8, [7 x i8] }
%"core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>>" = type { %"core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>" }
%"core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>" = type { [2 x i64] }
%"GrowingHashmapMapElemChar<RowId>" = type { i64, i32, [1 x i32] }

@alloc_9e0e4f78394e4fcc2308aabb1453004e = private unnamed_addr constant [97 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/strsim-0.11.1/src/lib.rs\00", align 1
@alloc_b796bdecba9627647adb3e704eedc700 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00n\00\00\00+\00\00\00" }>, align 8
@alloc_b0edf788d54645fb4e0f80eae3b38cde = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00}\00\00\007\00\00\00" }>, align 8
@alloc_51d3fde95428bd5e448f6069d0d69356 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00~\00\00\00\11\00\00\00" }>, align 8
@alloc_33490829e982a20313b24b098088ff01 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\FC\00\00\00\1F\00\00\00" }>, align 8
@alloc_979475be75b63577188022943ab16aa0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\9A\02\00\00\06\00\00\00" }>, align 8
@alloc_62d10971df7d5a4785f495974ef28733 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00y\02\00\00\1E\00\00\00" }>, align 8
@alloc_862551a6b5209c91aedf6fe87f92b332 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00~\02\00\00\1A\00\00\00" }>, align 8
@alloc_8da7b938e9f69bada7a79d2c6e57c2b6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\80\02\00\00\18\00\00\00" }>, align 8
@alloc_6bf673dbe204d5f5608d7daead0841b4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\8C\02\00\00'\00\00\00" }>, align 8
@alloc_6e277f70d633006168d39b714ca85e89 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\85\02\00\00\13\00\00\00" }>, align 8
@alloc_b944b20bc9fbafa729d24f8cbad2dd57 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\94\02\00\00\1A\00\00\00" }>, align 8
@alloc_a48736f782ce01888cde2ce41f6badc9 = private unnamed_addr constant [8 x i8] c"\FF\FF\FF\FF\FF\FF\FF\FF", align 8
@anon.5b7b4b03f51fa7c2fe1eb56c4b7bdd51.0 = private unnamed_addr constant <{ ptr, [24 x i8] }> <{ ptr @alloc_a48736f782ce01888cde2ce41f6badc9, [24 x i8] zeroinitializer }>, align 8
@_RNvNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBa_11RandomState3new4KEYS0s_023___RUST_STD_INTERNAL_VAL = external thread_local local_unnamed_addr global %"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<(u64, u64)>, !>"
@alloc_713c70d72f94083925b2dbcd729a89c1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\D4\01\00\00:\00\00\00" }>, align 8
@alloc_f07197f8880f02704fbb192991fd92e5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00P\01\00\00\13\00\00\00" }>, align 8
@alloc_5008b3d584a72b0077539f66d172f9b5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\008\01\00\00\17\00\00\00" }>, align 8
@alloc_2bae18778776445d604cc9d7d59ee8c2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00>\01\00\00#\00\00\00" }>, align 8
@alloc_6944f81dd681dfdbe5866d7fa21924c3 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00<\01\00\00\1B\00\00\00" }>, align 8
@alloc_1a619d5774b4c6a4ea0553f4f8c35387 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00B\01\00\00V\00\00\00" }>, align 8
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@alloc_a649d603cf56162b8a3f165589c1de6f = private unnamed_addr constant [39 x i8] c"callers have to ensure map is allocated", align 1
@alloc_1c02a211ff8957e465458a51c31a81de = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00)\02\00\00R\00\00\00" }>, align 8
@alloc_42d8339c69e16a1e3f5e0ef029dbe447 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\04\02\00\00\0E\00\00\00" }>, align 8
@alloc_d619813ccfd16ab2fd70bc85a627a0cb = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\06\02\00\00\0F\00\00\00" }>, align 8
@alloc_0a640bf0e45dbde7c4f1706a4879d5ca = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\0E\02\00\00\13\00\00\00" }>, align 8
@alloc_96408fa515ac8b1e77c68a6bc3e8f856 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\E0\01\00\00:\00\00\00" }>, align 8
@alloc_caa5cd3ec5b4a333d6ac178e6b813739 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9e0e4f78394e4fcc2308aabb1453004e, [16 x i8] c"`\00\00\00\00\00\00\00\F1\01\00\00:\00\00\00" }>, align 8
@_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP = external local_unnamed_addr global [256 x i8]
@alloc_f5d606a991b48f2a72fc31ffd42f8a04 = private unnamed_addr constant [35 x i8] c"Differing length arguments provided", align 1
@alloc_0c812808379efded5a4fb82d2790b556 = private unnamed_addr constant [2 x i8] c"\C0\00", align 1
@alloc_db51a71a1b6b25b4224d4dc5277f93e7 = private unnamed_addr constant [256 x i8] c"\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\01\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\03\04\04\04\04\04\00\00\00\00\00\00\00\00\00\00\00", align 1

; strsim::generic_jaro::<strsim::StringWrapper, strsim::StringWrapper, char, char>
; Function Attrs: uwtable
define internal fastcc noundef double @_RINvCs6KJnav5oeQt_6strsim12generic_jaroNtB2_13StringWrapperBB_ccEB2_(ptr captures(address, read_provenance) %a.0.val, i64 %a.8.val, ptr captures(address, read_provenance) %b.0.val, i64 %b.8.val) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %a.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_8.i = getelementptr inbounds nuw i8, ptr %a.0.val, i64 %a.8.val
  %_6.i = icmp ult i64 %a.8.val, 32
  br i1 %_6.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %start
; call core::str::count::do_count_chars
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0.val, i64 noundef %a.8.val)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

bb2.i:                                            ; preds = %start
; call core::str::count::char_count_general_case
  %2 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0.val, i64 noundef %a.8.val)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit: ; preds = %bb3.i, %bb2.i
  %_0.sroa.0.0.i = phi i64 [ %2, %bb2.i ], [ %1, %bb3.i ]
  %3 = icmp ne ptr %b.0.val, null
  tail call void @llvm.assume(i1 %3)
  %_8.i50 = getelementptr inbounds nuw i8, ptr %b.0.val, i64 %b.8.val
  %_6.i51 = icmp ult i64 %b.8.val, 32
  br i1 %_6.i51, label %bb2.i54, label %bb3.i52

bb3.i52:                                          ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::do_count_chars
  %4 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0.val, i64 noundef %b.8.val)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit55

bb2.i54:                                          ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::char_count_general_case
  %5 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0.val, i64 noundef %b.8.val)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit55

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit55: ; preds = %bb3.i52, %bb2.i54
  %_0.sroa.0.0.i53 = phi i64 [ %5, %bb2.i54 ], [ %4, %bb3.i52 ]
  %6 = icmp eq i64 %_0.sroa.0.0.i, 0
  %7 = icmp eq i64 %_0.sroa.0.0.i53, 0
  br i1 %6, label %bb5, label %bb7

bb5:                                              ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit55
  br i1 %7, label %bb68, label %bb8

bb7:                                              ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit55
  br i1 %7, label %bb8, label %bb9

bb8:                                              ; preds = %bb7, %bb5
  br label %bb68

bb9:                                              ; preds = %bb7
  %_0.sroa.0.0.i56 = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i53, i64 %_0.sroa.0.0.i)
  %8 = lshr i64 %_0.sroa.0.0.i56, 1
  %9 = tail call i64 @llvm.usub.sat.i64(i64 %8, i64 1)
  %_11 = add i64 %_0.sroa.0.0.i53, %_0.sroa.0.0.i
  %_23.i.i.i = icmp eq i64 %_11, 0
  br i1 %_23.i.i.i, label %_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim.exit, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb9
  %or.cond.not.i = icmp sgt i64 %_11, 0
  br i1 %or.cond.not.i, label %bb3.i4.i, label %bb14.i, !prof !2

bb3.i4.i:                                         ; preds = %bb6.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !3
; call __rustc::__rust_alloc_zeroed
  %10 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed(i64 noundef range(i64 1, 0) %_11, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !3
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb14.i, label %bb10.i.i

bb10.i.i:                                         ; preds = %bb3.i4.i
  %12 = ptrtoint ptr %10 to i64
  br label %_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

bb14.i:                                           ; preds = %bb3.i4.i, %bb6.i.i.i
  %_16.sroa.4.0.ph.i = phi i64 [ 1, %bb3.i4.i ], [ 0, %bb6.i.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_16.sroa.4.0.ph.i, i64 %_11) #24, !noalias !8
  unreachable

_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim.exit: ; preds = %bb9, %bb10.i.i
  %_16.sroa.10.0.i = phi i64 [ %12, %bb10.i.i ], [ 1, %bb9 ]
  %13 = inttoptr i64 %_16.sroa.10.0.i to ptr
  %_6.not.i = icmp ugt i64 %_0.sroa.0.0.i, %_11
  br i1 %_6.not.i, label %bb3.i58, label %bb13, !prof !9

bb3.i58:                                          ; preds = %_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim.exit
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b796bdecba9627647adb3e704eedc700) #25
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb3.i58
  unreachable

bb68:                                             ; preds = %bb2.i.i.i4.i103, %bb5, %bb8
  %_0.sroa.0.0 = phi double [ 0.000000e+00, %bb8 ], [ 1.000000e+00, %bb5 ], [ %_0.sroa.0.1, %bb2.i.i.i4.i103 ]
  ret double %_0.sroa.0.0

bb77:                                             ; preds = %cleanup10, %cleanup
  %.pn = phi { ptr, i32 } [ %38, %cleanup10 ], [ %14, %cleanup ]
  br i1 %_23.i.i.i, label %bb78, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %bb77
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %13, i64 noundef %_11, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb78

cleanup:                                          ; preds = %bb3.i58
  %14 = landingpad { ptr, i32 }
          cleanup
  br label %bb77

bb13:                                             ; preds = %_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim.exit
  %data.i.i = getelementptr inbounds nuw i8, ptr %13, i64 %_0.sroa.0.0.i
  %_6.i.i.not.i.i73 = icmp samesign eq i64 %a.8.val, 0
  br i1 %_6.i.i.not.i.i73, label %bb2.i.i.i4.i103, label %bb14.i.i.i.lr.ph

bb14.i.i.i.lr.ph:                                 ; preds = %bb13
  %_31 = add nuw nsw i64 %9, 1
  br label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb14.i.i.i.lr.ph, %bb81
  %matches.sroa.0.076 = phi i64 [ 0, %bb14.i.i.i.lr.ph ], [ %matches.sroa.0.1, %bb81 ]
  %iter.sroa.10.075 = phi i64 [ 0, %bb14.i.i.i.lr.ph ], [ %_8.0.i15, %bb81 ]
  %iter.sroa.0.074 = phi ptr [ %a.0.val, %bb14.i.i.i.lr.ph ], [ %iter.sroa.0.113, %bb81 ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074, i64 1
  %x.i.i.i = load i8, ptr %iter.sroa.0.074, align 1, !noalias !10, !noundef !17
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_8.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !10, !noundef !17
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %15 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i61, label %bb23

bb3.i.i.i:                                        ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb23

bb6.i.i.i61:                                      ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_8.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !10, !noundef !17
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %16 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, label %bb23

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %bb6.i.i.i61
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_8.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !10, !noundef !17
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %17 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  %.not.i = icmp eq i32 %17, 1114112
  br i1 %.not.i, label %bb18, label %bb15

bb15:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.074, i64 4
  br label %bb23

bb18:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, %bb81
  %matches.sroa.0.0.lcssa = phi i64 [ %matches.sroa.0.076, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ], [ %matches.sroa.0.1, %bb81 ]
  %18 = icmp eq i64 %matches.sroa.0.0.lcssa, 0
  br i1 %18, label %bb2.i.i.i4.i103, label %bb9.i.preheader

bb9.i.preheader:                                  ; preds = %bb18
  %_100 = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %_0.sroa.0.0.i53
  br label %bb9.i

bb9.i:                                            ; preds = %bb9.i.preheader, %bb58
  %transpositions.sroa.0.183 = phi i64 [ %transpositions.sroa.0.3, %bb58 ], [ 0, %bb9.i.preheader ]
  %iter2.sroa.7.082 = phi ptr [ %iter2.sroa.7.123, %bb58 ], [ %a.0.val, %bb9.i.preheader ]
  %iter2.sroa.0.081 = phi ptr [ %_16.i.i, %bb58 ], [ %13, %bb9.i.preheader ]
  %b_iter.sroa.7.080 = phi ptr [ %b_iter.sroa.7.3, %bb58 ], [ %b.0.val, %bb9.i.preheader ]
  %b_iter.sroa.0.079 = phi ptr [ %b_iter.sroa.0.3, %bb58 ], [ %data.i.i, %bb9.i.preheader ]
  %_16.i.i = getelementptr inbounds nuw i8, ptr %iter2.sroa.0.081, i64 1
  %_6.i.i.not.i.i66 = icmp eq ptr %iter2.sroa.7.082, %_8.i
  br i1 %_6.i.i.not.i.i66, label %bb64, label %bb14.i.i.i67

bb14.i.i.i67:                                     ; preds = %bb9.i
  %_16.i.i.i.i68 = getelementptr inbounds nuw i8, ptr %iter2.sroa.7.082, i64 1
  %x.i.i.i69 = load i8, ptr %iter2.sroa.7.082, align 1, !noalias !18, !noundef !17
  %_6.i.i.i70 = icmp sgt i8 %x.i.i.i69, -1
  br i1 %_6.i.i.i70, label %bb3.i.i.i101, label %bb4.i.i.i71

bb4.i.i.i71:                                      ; preds = %bb14.i.i.i67
  %_30.i.i.i72 = and i8 %x.i.i.i69, 31
  %init.i.i.i73 = zext nneg i8 %_30.i.i.i72 to i32
  %_6.i10.i.i.i74 = icmp ne ptr %_16.i.i.i.i68, %_8.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i74)
  %_16.i12.i.i.i75 = getelementptr inbounds nuw i8, ptr %iter2.sroa.7.082, i64 2
  %y.i.i.i76 = load i8, ptr %_16.i.i.i.i68, align 1, !noalias !18, !noundef !17
  %_33.i.i.i77 = shl nuw nsw i32 %init.i.i.i73, 6
  %_35.i.i.i78 = and i8 %y.i.i.i76, 63
  %_34.i.i.i79 = zext nneg i8 %_35.i.i.i78 to i32
  %19 = or disjoint i32 %_33.i.i.i77, %_34.i.i.i79
  %_13.i.i.i80 = icmp samesign ugt i8 %x.i.i.i69, -33
  br i1 %_13.i.i.i80, label %bb6.i.i.i82, label %bb44

bb3.i.i.i101:                                     ; preds = %bb14.i.i.i67
  %_7.i.i.i102 = zext nneg i8 %x.i.i.i69 to i32
  br label %bb44

bb6.i.i.i82:                                      ; preds = %bb4.i.i.i71
  %_6.i17.i.i.i83 = icmp ne ptr %_16.i12.i.i.i75, %_8.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i83)
  %_16.i19.i.i.i84 = getelementptr inbounds nuw i8, ptr %iter2.sroa.7.082, i64 3
  %z.i.i.i85 = load i8, ptr %_16.i12.i.i.i75, align 1, !noalias !18, !noundef !17
  %_38.i.i.i86 = shl nuw nsw i32 %_34.i.i.i79, 6
  %_40.i.i.i87 = and i8 %z.i.i.i85, 63
  %_39.i.i.i88 = zext nneg i8 %_40.i.i.i87 to i32
  %y_z.i.i.i89 = or disjoint i32 %_38.i.i.i86, %_39.i.i.i88
  %_20.i.i.i90 = shl nuw nsw i32 %init.i.i.i73, 12
  %20 = or disjoint i32 %y_z.i.i.i89, %_20.i.i.i90
  %_21.i.i.i91 = icmp samesign ugt i8 %x.i.i.i69, -17
  br i1 %_21.i.i.i91, label %bb89, label %bb44

bb89:                                             ; preds = %bb6.i.i.i82
  %_6.i24.i.i.i92 = icmp ne ptr %_16.i19.i.i.i84, %_8.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i92)
  %_16.i26.i.i.i93 = getelementptr inbounds nuw i8, ptr %iter2.sroa.7.082, i64 4
  %w.i.i.i94 = load i8, ptr %_16.i19.i.i.i84, align 1, !noalias !18, !noundef !17
  %_26.i.i.i95 = shl nuw nsw i32 %init.i.i.i73, 18
  %_25.i.i.i96 = and i32 %_26.i.i.i95, 1835008
  %_43.i.i.i97 = shl nuw nsw i32 %y_z.i.i.i89, 6
  %_45.i.i.i98 = and i8 %w.i.i.i94, 63
  %_44.i.i.i99 = zext nneg i8 %_45.i.i.i98 to i32
  %_27.i.i.i100 = or disjoint i32 %_43.i.i.i97, %_44.i.i.i99
  %21 = or disjoint i32 %_27.i.i.i100, %_25.i.i.i96
  %.not33 = icmp eq i32 %21, 1114112
  br i1 %.not33, label %bb64, label %bb44

bb44:                                             ; preds = %bb6.i.i.i82, %bb3.i.i.i101, %bb4.i.i.i71, %bb89
  %_0.sroa.2.0.i8125 = phi i32 [ %21, %bb89 ], [ %20, %bb6.i.i.i82 ], [ %_7.i.i.i102, %bb3.i.i.i101 ], [ %19, %bb4.i.i.i71 ]
  %iter2.sroa.7.123 = phi ptr [ %_16.i26.i.i.i93, %bb89 ], [ %_16.i19.i.i.i84, %bb6.i.i.i82 ], [ %_16.i.i.i.i68, %bb3.i.i.i101 ], [ %_16.i12.i.i.i75, %bb4.i.i.i71 ]
  %22 = icmp ne ptr %iter2.sroa.0.081, null
  tail call void @llvm.assume(i1 %22)
  %23 = load i8, ptr %iter2.sroa.0.081, align 1, !range !25, !noundef !17
  %_63 = trunc nuw i8 %23 to i1
  br i1 %_63, label %bb46.outer, label %bb58

bb64:                                             ; preds = %bb9.i, %bb58, %bb89
  %transpositions.sroa.0.1.lcssa.ph = phi i64 [ %transpositions.sroa.0.183, %bb9.i ], [ %transpositions.sroa.0.3, %bb58 ], [ %transpositions.sroa.0.183, %bb89 ]
  %24 = lshr i64 %transpositions.sroa.0.1.lcssa.ph, 1
  %_77 = uitofp i64 %matches.sroa.0.0.lcssa to double
  %_79 = uitofp i64 %_0.sroa.0.0.i to double
  %_76 = fdiv double %_77, %_79
  %_83 = uitofp i64 %_0.sroa.0.0.i53 to double
  %_80 = fdiv double %_77, %_83
  %_75 = fadd double %_76, %_80
  %_86 = sub i64 %matches.sroa.0.0.lcssa, %24
  %_85 = uitofp i64 %_86 to double
  %_84 = fdiv double %_85, %_77
  %_74 = fadd double %_75, %_84
  %25 = fdiv double %_74, 3.000000e+00
  br label %bb2.i.i.i4.i103

bb2.i.i.i4.i103:                                  ; preds = %bb13, %bb64, %bb18
  %_0.sroa.0.1 = phi double [ %25, %bb64 ], [ 0.000000e+00, %bb18 ], [ 0.000000e+00, %bb13 ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %13, i64 noundef %_11, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb68

bb46:                                             ; preds = %bb46.outer, %bb46
  br label %bb46, !llvm.loop !26

bb9.i108:                                         ; preds = %bb46.outer
  %_16.i.i109 = getelementptr inbounds nuw i8, ptr %b_iter.sroa.0.1.ph, i64 1
  %26 = icmp ne ptr %b_iter.sroa.7.1.ph, null
  tail call void @llvm.assume(i1 %26)
  %_6.i.i.not.i.i113 = icmp eq ptr %b_iter.sroa.7.1.ph, %_8.i50
  br i1 %_6.i.i.not.i.i113, label %bb46.outer.backedge, label %bb14.i.i.i114

bb14.i.i.i114:                                    ; preds = %bb9.i108
  %_16.i.i.i.i115 = getelementptr inbounds nuw i8, ptr %b_iter.sroa.7.1.ph, i64 1
  %x.i.i.i116 = load i8, ptr %b_iter.sroa.7.1.ph, align 1, !noalias !28, !noundef !17
  %_6.i.i.i117 = icmp sgt i8 %x.i.i.i116, -1
  br i1 %_6.i.i.i117, label %bb3.i.i.i150, label %bb4.i.i.i118

bb4.i.i.i118:                                     ; preds = %bb14.i.i.i114
  %_30.i.i.i119 = and i8 %x.i.i.i116, 31
  %init.i.i.i120 = zext nneg i8 %_30.i.i.i119 to i32
  %_6.i10.i.i.i121 = icmp ne ptr %_16.i.i.i.i115, %_8.i50
  tail call void @llvm.assume(i1 %_6.i10.i.i.i121)
  %_16.i12.i.i.i122 = getelementptr inbounds nuw i8, ptr %b_iter.sroa.7.1.ph, i64 2
  %y.i.i.i123 = load i8, ptr %_16.i.i.i.i115, align 1, !noalias !28, !noundef !17
  %_33.i.i.i124 = shl nuw nsw i32 %init.i.i.i120, 6
  %_35.i.i.i125 = and i8 %y.i.i.i123, 63
  %_34.i.i.i126 = zext nneg i8 %_35.i.i.i125 to i32
  %27 = or disjoint i32 %_33.i.i.i124, %_34.i.i.i126
  %_13.i.i.i127 = icmp samesign ugt i8 %x.i.i.i116, -33
  br i1 %_13.i.i.i127, label %bb6.i.i.i130, label %bb47

bb3.i.i.i150:                                     ; preds = %bb14.i.i.i114
  %_7.i.i.i151 = zext nneg i8 %x.i.i.i116 to i32
  br label %bb47

bb6.i.i.i130:                                     ; preds = %bb4.i.i.i118
  %_6.i17.i.i.i131 = icmp ne ptr %_16.i12.i.i.i122, %_8.i50
  tail call void @llvm.assume(i1 %_6.i17.i.i.i131)
  %_16.i19.i.i.i132 = getelementptr inbounds nuw i8, ptr %b_iter.sroa.7.1.ph, i64 3
  %z.i.i.i133 = load i8, ptr %_16.i12.i.i.i122, align 1, !noalias !28, !noundef !17
  %_38.i.i.i134 = shl nuw nsw i32 %_34.i.i.i126, 6
  %_40.i.i.i135 = and i8 %z.i.i.i133, 63
  %_39.i.i.i136 = zext nneg i8 %_40.i.i.i135 to i32
  %y_z.i.i.i137 = or disjoint i32 %_38.i.i.i134, %_39.i.i.i136
  %_20.i.i.i138 = shl nuw nsw i32 %init.i.i.i120, 12
  %28 = or disjoint i32 %y_z.i.i.i137, %_20.i.i.i138
  %_21.i.i.i139 = icmp samesign ugt i8 %x.i.i.i116, -17
  br i1 %_21.i.i.i139, label %bb90, label %bb47

bb90:                                             ; preds = %bb6.i.i.i130
  %_6.i24.i.i.i141 = icmp ne ptr %_16.i19.i.i.i132, %_8.i50
  tail call void @llvm.assume(i1 %_6.i24.i.i.i141)
  %_16.i26.i.i.i142 = getelementptr inbounds nuw i8, ptr %b_iter.sroa.7.1.ph, i64 4
  %w.i.i.i143 = load i8, ptr %_16.i19.i.i.i132, align 1, !noalias !28, !noundef !17
  %_26.i.i.i144 = shl nuw nsw i32 %init.i.i.i120, 18
  %_25.i.i.i145 = and i32 %_26.i.i.i144, 1835008
  %_43.i.i.i146 = shl nuw nsw i32 %y_z.i.i.i137, 6
  %_45.i.i.i147 = and i8 %w.i.i.i143, 63
  %_44.i.i.i148 = zext nneg i8 %_45.i.i.i147 to i32
  %_27.i.i.i149 = or disjoint i32 %_43.i.i.i146, %_44.i.i.i148
  %29 = or disjoint i32 %_27.i.i.i149, %_25.i.i.i145
  %.not34 = icmp eq i32 %29, 1114112
  br i1 %.not34, label %bb46.outer.backedge, label %bb47

bb46.outer.backedge:                              ; preds = %bb90, %bb47, %bb9.i108
  %b_iter.sroa.7.1.ph.be = phi ptr [ %_8.i50, %bb9.i108 ], [ %b_iter.sroa.7.239, %bb47 ], [ %_16.i26.i.i.i142, %bb90 ]
  br label %bb46.outer

bb46.outer:                                       ; preds = %bb44, %bb46.outer.backedge
  %b_iter.sroa.0.1.ph = phi ptr [ %_16.i.i109, %bb46.outer.backedge ], [ %b_iter.sroa.0.079, %bb44 ]
  %b_iter.sroa.7.1.ph = phi ptr [ %b_iter.sroa.7.1.ph.be, %bb46.outer.backedge ], [ %b_iter.sroa.7.080, %bb44 ]
  %30 = icmp ne ptr %b_iter.sroa.0.1.ph, null
  tail call void @llvm.assume(i1 %30)
  %_6.i.i107.peel = icmp eq ptr %b_iter.sroa.0.1.ph, %_100
  br i1 %_6.i.i107.peel, label %bb46, label %bb9.i108

bb47:                                             ; preds = %bb6.i.i.i130, %bb3.i.i.i150, %bb4.i.i.i118, %bb90
  %_0.sroa.2.0.i12941 = phi i32 [ %29, %bb90 ], [ %28, %bb6.i.i.i130 ], [ %_7.i.i.i151, %bb3.i.i.i150 ], [ %27, %bb4.i.i.i118 ]
  %b_iter.sroa.7.239 = phi ptr [ %_16.i26.i.i.i142, %bb90 ], [ %_16.i19.i.i.i132, %bb6.i.i.i130 ], [ %_16.i.i.i.i115, %bb3.i.i.i150 ], [ %_16.i12.i.i.i122, %bb4.i.i.i118 ]
  %31 = load i8, ptr %b_iter.sroa.0.1.ph, align 1, !range !25, !noundef !17
  %_69 = trunc nuw i8 %31 to i1
  br i1 %_69, label %bb49, label %bb46.outer.backedge

bb49:                                             ; preds = %bb47
  %_0.i.not = icmp ne i32 %_0.sroa.2.0.i8125, %_0.sroa.2.0.i12941
  %32 = zext i1 %_0.i.not to i64
  %spec.select = add i64 %transpositions.sroa.0.183, %32
  br label %bb58

bb58:                                             ; preds = %bb44, %bb49
  %b_iter.sroa.0.3 = phi ptr [ %_16.i.i109, %bb49 ], [ %b_iter.sroa.0.079, %bb44 ]
  %b_iter.sroa.7.3 = phi ptr [ %b_iter.sroa.7.239, %bb49 ], [ %b_iter.sroa.7.080, %bb44 ]
  %transpositions.sroa.0.3 = phi i64 [ %spec.select, %bb49 ], [ %transpositions.sroa.0.183, %bb44 ]
  %_6.i.i = icmp eq ptr %_16.i.i, %data.i.i
  br i1 %_6.i.i, label %bb64, label %bb9.i

bb23:                                             ; preds = %bb3.i.i.i, %bb6.i.i.i61, %bb4.i.i.i, %bb15
  %spec.select.i5.i14 = phi i32 [ %17, %bb15 ], [ %15, %bb4.i.i.i ], [ %16, %bb6.i.i.i61 ], [ %_7.i.i.i, %bb3.i.i.i ]
  %iter.sroa.0.113 = phi ptr [ %_16.i26.i.i.i, %bb15 ], [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i61 ], [ %_16.i.i.i.i, %bb3.i.i.i ]
  %_8.0.i15 = add i64 %iter.sroa.10.075, 1
  %min_bound.sroa.0.0 = tail call i64 @llvm.usub.sat.i64(i64 %iter.sroa.10.075, i64 %9)
  %_30 = add i64 %_31, %iter.sroa.10.075
  %33 = icmp eq i64 %_30, 0
  br i1 %33, label %bb81, label %bb86.preheader

bb86.preheader:                                   ; preds = %bb23
  %_0.sroa.0.0.i62 = tail call noundef i64 @llvm.umin.i64(i64 %_30, i64 %_0.sroa.0.0.i53)
  br label %bb86

bb86:                                             ; preds = %bb86.preheader, %bb35
  %iter1.sroa.13.072 = phi i64 [ %34, %bb35 ], [ %_0.sroa.0.0.i62, %bb86.preheader ]
  %iter1.sroa.10.071 = phi i64 [ %_8.0.i17463, %bb35 ], [ 0, %bb86.preheader ]
  %iter1.sroa.0.070 = phi ptr [ %iter1.sroa.0.161, %bb35 ], [ %b.0.val, %bb86.preheader ]
  %34 = add i64 %iter1.sroa.13.072, -1
  %_6.i.i.not.i.i156 = icmp eq ptr %iter1.sroa.0.070, %_8.i50
  br i1 %_6.i.i.not.i.i156, label %bb81, label %bb14.i.i.i157

bb14.i.i.i157:                                    ; preds = %bb86
  %_16.i.i.i.i158 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.070, i64 1
  %x.i.i.i159 = load i8, ptr %iter1.sroa.0.070, align 1, !noalias !35, !noundef !17
  %_6.i.i.i160 = icmp sgt i8 %x.i.i.i159, -1
  br i1 %_6.i.i.i160, label %bb3.i.i.i198, label %bb4.i.i.i161

bb4.i.i.i161:                                     ; preds = %bb14.i.i.i157
  %_30.i.i.i162 = and i8 %x.i.i.i159, 31
  %init.i.i.i163 = zext nneg i8 %_30.i.i.i162 to i32
  %_6.i10.i.i.i164 = icmp ne ptr %_16.i.i.i.i158, %_8.i50
  tail call void @llvm.assume(i1 %_6.i10.i.i.i164)
  %_16.i12.i.i.i165 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.070, i64 2
  %y.i.i.i166 = load i8, ptr %_16.i.i.i.i158, align 1, !noalias !35, !noundef !17
  %_33.i.i.i167 = shl nuw nsw i32 %init.i.i.i163, 6
  %_35.i.i.i168 = and i8 %y.i.i.i166, 63
  %_34.i.i.i169 = zext nneg i8 %_35.i.i.i168 to i32
  %35 = or disjoint i32 %_33.i.i.i167, %_34.i.i.i169
  %_13.i.i.i170 = icmp samesign ugt i8 %x.i.i.i159, -33
  br i1 %_13.i.i.i170, label %bb6.i.i.i177, label %bb25

bb3.i.i.i198:                                     ; preds = %bb14.i.i.i157
  %_7.i.i.i199 = zext nneg i8 %x.i.i.i159 to i32
  br label %bb25

bb6.i.i.i177:                                     ; preds = %bb4.i.i.i161
  %_6.i17.i.i.i178 = icmp ne ptr %_16.i12.i.i.i165, %_8.i50
  tail call void @llvm.assume(i1 %_6.i17.i.i.i178)
  %_16.i19.i.i.i179 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.070, i64 3
  %z.i.i.i180 = load i8, ptr %_16.i12.i.i.i165, align 1, !noalias !35, !noundef !17
  %_38.i.i.i181 = shl nuw nsw i32 %_34.i.i.i169, 6
  %_40.i.i.i182 = and i8 %z.i.i.i180, 63
  %_39.i.i.i183 = zext nneg i8 %_40.i.i.i182 to i32
  %y_z.i.i.i184 = or disjoint i32 %_38.i.i.i181, %_39.i.i.i183
  %_20.i.i.i185 = shl nuw nsw i32 %init.i.i.i163, 12
  %36 = or disjoint i32 %y_z.i.i.i184, %_20.i.i.i185
  %_21.i.i.i186 = icmp samesign ugt i8 %x.i.i.i159, -17
  br i1 %_21.i.i.i186, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i187, label %bb25

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i187: ; preds = %bb6.i.i.i177
  %_6.i24.i.i.i188 = icmp ne ptr %_16.i19.i.i.i179, %_8.i50
  tail call void @llvm.assume(i1 %_6.i24.i.i.i188)
  %w.i.i.i190 = load i8, ptr %_16.i19.i.i.i179, align 1, !noalias !35, !noundef !17
  %_26.i.i.i191 = shl nuw nsw i32 %init.i.i.i163, 18
  %_25.i.i.i192 = and i32 %_26.i.i.i191, 1835008
  %_43.i.i.i193 = shl nuw nsw i32 %y_z.i.i.i184, 6
  %_45.i.i.i194 = and i8 %w.i.i.i190, 63
  %_44.i.i.i195 = zext nneg i8 %_45.i.i.i194 to i32
  %_27.i.i.i196 = or disjoint i32 %_43.i.i.i193, %_44.i.i.i195
  %37 = or disjoint i32 %_27.i.i.i196, %_25.i.i.i192
  %.not.i197 = icmp eq i32 %37, 1114112
  br i1 %.not.i197, label %bb81, label %bb85

bb85:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i187
  %_16.i26.i.i.i189 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.070, i64 4
  br label %bb25

bb25:                                             ; preds = %bb3.i.i.i198, %bb6.i.i.i177, %bb4.i.i.i161, %bb85
  %spec.select.i5.i17262 = phi i32 [ %37, %bb85 ], [ %35, %bb4.i.i.i161 ], [ %36, %bb6.i.i.i177 ], [ %_7.i.i.i199, %bb3.i.i.i198 ]
  %iter1.sroa.0.161 = phi ptr [ %_16.i26.i.i.i189, %bb85 ], [ %_16.i12.i.i.i165, %bb4.i.i.i161 ], [ %_16.i19.i.i.i179, %bb6.i.i.i177 ], [ %_16.i.i.i.i158, %bb3.i.i.i198 ]
  %_8.0.i17463 = add i64 %iter1.sroa.10.071, 1
  %_41.not = icmp ule i64 %min_bound.sroa.0.0, %iter1.sroa.10.071
  %_0.i201 = icmp eq i32 %spec.select.i5.i14, %spec.select.i5.i17262
  %or.cond = select i1 %_41.not, i1 %_0.i201, i1 false
  br i1 %or.cond, label %bb28, label %bb35

bb81:                                             ; preds = %bb35, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i187, %bb86, %bb23, %bb31
  %matches.sroa.0.1 = phi i64 [ %43, %bb31 ], [ %matches.sroa.0.076, %bb23 ], [ %matches.sroa.0.076, %bb86 ], [ %matches.sroa.0.076, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i187 ], [ %matches.sroa.0.076, %bb35 ]
  %_6.i.i.not.i.i = icmp eq ptr %iter.sroa.0.113, %_8.i
  br i1 %_6.i.i.not.i.i, label %bb18, label %bb14.i.i.i

cleanup10:                                        ; preds = %panic11.invoke
  %38 = landingpad { ptr, i32 }
          cleanup
  br label %bb77

bb28:                                             ; preds = %bb25
  %_48 = icmp ult i64 %iter1.sroa.10.071, %_0.sroa.0.0.i53
  br i1 %_48, label %bb29, label %panic11.invoke

bb29:                                             ; preds = %bb28
  %39 = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %iter1.sroa.10.071
  %40 = load i8, ptr %39, align 1, !range !25, !noundef !17
  %_46 = trunc nuw i8 %40 to i1
  br i1 %_46, label %bb35, label %bb30

bb30:                                             ; preds = %bb29
  %_50 = icmp ult i64 %iter.sroa.10.075, %_0.sroa.0.0.i
  br i1 %_50, label %bb31, label %panic11.invoke

bb31:                                             ; preds = %bb30
  %41 = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %iter1.sroa.10.071
  %42 = getelementptr inbounds nuw i8, ptr %13, i64 %iter.sroa.10.075
  store i8 1, ptr %42, align 1
  store i8 1, ptr %41, align 1
  %43 = add i64 %matches.sroa.0.076, 1
  br label %bb81

panic11.invoke:                                   ; preds = %bb30, %bb28
  %44 = phi i64 [ %iter1.sroa.10.071, %bb28 ], [ %iter.sroa.10.075, %bb30 ]
  %45 = phi i64 [ %_0.sroa.0.0.i53, %bb28 ], [ %_0.sroa.0.0.i, %bb30 ]
  %46 = phi ptr [ @alloc_b0edf788d54645fb4e0f80eae3b38cde, %bb28 ], [ @alloc_51d3fde95428bd5e448f6069d0d69356, %bb30 ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %44, i64 noundef %45, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %46) #24
          to label %panic11.cont unwind label %cleanup10

panic11.cont:                                     ; preds = %panic11.invoke
  unreachable

bb35:                                             ; preds = %bb29, %bb25
  %47 = icmp eq i64 %34, 0
  br i1 %47, label %bb81, label %bb86

bb78:                                             ; preds = %bb2.i.i.i4.i, %bb77
  resume { ptr, i32 } %.pn
}

; strsim::damerau_levenshtein_impl::<core::str::iter::Chars, core::str::iter::Chars>
; Function Attrs: uwtable
define internal fastcc noundef i64 @_RINvCs6KJnav5oeQt_6strsim24damerau_levenshtein_implNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBN_EB2_(ptr noundef nonnull readonly captures(address) %s1.0, ptr noundef nonnull readnone captures(address) %s1.1, i64 noundef %len1, ptr noundef nonnull readonly captures(address) %0, ptr noundef nonnull readnone captures(address) %1, i64 noundef %len2) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %r1 = alloca [24 x i8], align 8
  %fr = alloca [24 x i8], align 8
  %last_row_id = alloca [2088 x i8], align 8
  %_0.sroa.0.0.i = tail call noundef i64 @llvm.umax.i64(i64 %len2, i64 %len1)
  %max_val = add i64 %_0.sroa.0.0.i, 1
  call void @llvm.lifetime.start.p0(i64 2088, ptr nonnull %last_row_id)
  %2 = getelementptr inbounds nuw i8, ptr %last_row_id, i64 40
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(2048) %2, i8 -1, i64 2048, i1 false), !alias.scope !42
  store i64 -9223372036854775808, ptr %last_row_id, align 8, !alias.scope !42
  %_1.sroa.62.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %last_row_id, i64 24
  %_1.sroa.7.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %last_row_id, i64 28
  store <2 x i32> zeroinitializer, ptr %_1.sroa.62.0._0.sroa_idx.i, align 8, !alias.scope !42
  %_1.sroa.8.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %last_row_id, i64 32
  store i32 -1, ptr %_1.sroa.8.0._0.sroa_idx.i, align 8, !alias.scope !42
  %size = add i64 %len2, 2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %fr)
; invoke <isize as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
  invoke fastcc void @_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemiNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(24) %fr, i64 noundef %max_val, i64 noundef %size)
          to label %bb38 unwind label %cleanup2

bb32:                                             ; preds = %bb2.i.i.i4.i, %bb31, %cleanup2
  %.pn.pn.pn.pn = phi { ptr, i32 } [ %4, %cleanup2 ], [ %.pn.pn.pn, %bb31 ], [ %.pn.pn.pn, %bb2.i.i.i4.i ]
  %last_row_id.val42 = load i64, ptr %last_row_id, align 8, !range !45, !noundef !17
  switch i64 %last_row_id.val42, label %bb2.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %bb34
    i64 0, label %bb34
  ]

bb2.i.i.i4.i.i.i.i:                               ; preds = %bb32
  %3 = getelementptr inbounds nuw i8, ptr %last_row_id, i64 8
  %last_row_id.val43 = load ptr, ptr %3, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i.i.i.i = shl nuw i64 %last_row_id.val42, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %last_row_id.val43, i64 noundef %alloc_size.i.i.i.i5.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb34

cleanup2:                                         ; preds = %start
  %4 = landingpad { ptr, i32 }
          cleanup
  br label %bb32

bb38:                                             ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %r1)
; invoke <isize as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
  invoke fastcc void @_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemiNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(24) %r1, i64 noundef %max_val, i64 noundef %size)
          to label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtNtBa_3ops5range5RangeiEBZ_ENtNtNtB8_6traits8iterator8Iterator9size_hintCs6KJnav5oeQt_6strsim.exit.i unwind label %cleanup3

bb31:                                             ; preds = %bb2.i.i.i4.i45, %bb30, %cleanup3
  %.pn.pn.pn = phi { ptr, i32 } [ %7, %cleanup3 ], [ %.pn.pn, %bb30 ], [ %.pn.pn, %bb2.i.i.i4.i45 ]
  %fr.val = load i64, ptr %fr, align 8
  %5 = icmp eq i64 %fr.val, 0
  br i1 %5, label %bb32, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %bb31
  %6 = getelementptr inbounds nuw i8, ptr %fr, i64 8
  %fr.val32 = load ptr, ptr %6, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i = shl nuw i64 %fr.val, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %fr.val32, i64 noundef %alloc_size.i.i.i.i5.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb32

cleanup3:                                         ; preds = %bb38
  %7 = landingpad { ptr, i32 }
          cleanup
  br label %bb31

bb30:                                             ; preds = %bb2.i.i.i4.i49, %bb29, %cleanup4
  %.pn.pn = phi { ptr, i32 } [ %10, %cleanup4 ], [ %.pn, %bb29 ], [ %.pn, %bb2.i.i.i4.i49 ]
  %r1.val = load i64, ptr %r1, align 8
  %8 = icmp eq i64 %r1.val, 0
  br i1 %8, label %bb31, label %bb2.i.i.i4.i45

bb2.i.i.i4.i45:                                   ; preds = %bb30
  %9 = getelementptr inbounds nuw i8, ptr %r1, i64 8
  %r1.val33 = load ptr, ptr %9, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i46 = shl nuw i64 %r1.val, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %r1.val33, i64 noundef %alloc_size.i.i.i.i5.i46, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb31

cleanup4:                                         ; preds = %bb3.i5.i
  %10 = landingpad { ptr, i32 }
          cleanup
  br label %bb30

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtNtBa_3ops5range5RangeiEBZ_ENtNtNtB8_6traits8iterator8Iterator9size_hintCs6KJnav5oeQt_6strsim.exit.i: ; preds = %bb38
  %_15 = add i64 %_0.sroa.0.0.i, 2
  %_18 = add i64 %len2, 1
  %_0.i.i9.i.i = icmp slt i64 %max_val, %_15
  %spec.select.i11.i.i = zext i1 %_0.i.i9.i.i to i64
  %spec.select.i14.i.i = tail call i64 @llvm.smax.i64(i64 %_18, i64 0)
  %_22.0.i.i = add nuw i64 %spec.select.i14.i.i, %spec.select.i11.i.i
  %_23.i.i.i.i = icmp eq i64 %_22.0.i.i, 0
  br i1 %_23.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtNtBa_3ops5range5RangeiEBZ_ENtNtNtB8_6traits8iterator8Iterator9size_hintCs6KJnav5oeQt_6strsim.exit.i
  %_24.i.i.i.i = add i64 %_22.0.i.i, -1
  %_27.0.i.i.i.i = shl i64 %_24.i.i.i.i, 3
  %_27.1.i.i.i.i = icmp samesign ugt i64 %_24.i.i.i.i, 2305843009213693951
  %_40.i.i.i.i = icmp ugt i64 %_27.0.i.i.i.i, 9223372036854775799
  %or.cond.i = or i1 %_27.1.i.i.i.i, %_40.i.i.i.i
  br i1 %or.cond.i, label %bb3.i5.i, label %bb3.i.i.i, !prof !46

bb3.i.i.i:                                        ; preds = %bb6.i.i.i.i
  %new_size2.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i, 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !47
; call __rustc::__rust_alloc
  %11 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !47
  %12 = icmp eq ptr %11, null
  br i1 %12, label %bb3.i5.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i

bb3.i5.i:                                         ; preds = %bb3.i.i.i, %bb6.i.i.i.i
  %_4.sroa.4.0.ph.i.i = phi i64 [ 8, %bb3.i.i.i ], [ 0, %bb6.i.i.i.i ]
  %_4.sroa.10.0.ph.i.i = phi i64 [ %new_size2.i.i.i.i, %bb3.i.i.i ], [ undef, %bb6.i.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i, i64 %_4.sroa.10.0.ph.i.i) #24
          to label %.noexc unwind label %cleanup4

.noexc:                                           ; preds = %bb3.i5.i
  unreachable

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i: ; preds = %bb3.i.i.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtNtBa_3ops5range5RangeiEBZ_ENtNtNtB8_6traits8iterator8Iterator9size_hintCs6KJnav5oeQt_6strsim.exit.i
  %_4.sroa.10.0.i.i = phi ptr [ inttoptr (i64 8 to ptr), %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB4_5ChainINtNtNtBa_3ops5range5RangeiEBZ_ENtNtNtB8_6traits8iterator8Iterator9size_hintCs6KJnav5oeQt_6strsim.exit.i ], [ %11, %bb3.i.i.i ]
  %13 = ptrtoint ptr %_4.sroa.10.0.i.i to i64
  br i1 %_0.i.i9.i.i, label %bb3.i.i.i.i.i.i.preheader, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i.i.preheader:                        ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i
  store i64 %max_val, ptr %_4.sroa.10.0.i.i, align 8, !noalias !53
  br label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb3.i.i.i.i.i.i.preheader, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i
  %_4.sroa.5.0.i.i.i.i = phi i64 [ 0, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i ], [ 1, %bb3.i.i.i.i.i.i.preheader ]
  %_0.i.i.i11.i.i.i.i.i.i = icmp ult i64 %len2, 9223372036854775807
  br i1 %_0.i.i.i11.i.i.i.i.i.i, label %bb3.i9.i.i.i.i.i.preheader, label %bb3

bb3.i9.i.i.i.i.i.preheader:                       ; preds = %bb3.i.i.i.i.i
  %14 = add nuw i64 %len2, 1
  %min.iters.check = icmp ult i64 %len2, 7
  br i1 %min.iters.check, label %bb3.i9.i.i.i.i.i.preheader886, label %vector.ph

vector.ph:                                        ; preds = %bb3.i9.i.i.i.i.i.preheader
  %n.vec = and i64 %14, -8
  %15 = or disjoint i64 %_4.sroa.5.0.i.i.i.i, %n.vec
  %16 = getelementptr inbounds nuw i64, ptr %_4.sroa.10.0.i.i, i64 %_4.sroa.5.0.i.i.i.i
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %vec.ind = phi <2 x i64> [ <i64 0, i64 1>, %vector.ph ], [ %vec.ind.next, %vector.body ]
  %step.add = add <2 x i64> %vec.ind, splat (i64 2)
  %step.add.2 = add <2 x i64> %vec.ind, splat (i64 4)
  %step.add.3 = add <2 x i64> %vec.ind, splat (i64 6)
  %17 = getelementptr inbounds nuw i64, ptr %16, i64 %index
  %18 = getelementptr inbounds nuw i8, ptr %17, i64 16
  %19 = getelementptr inbounds nuw i8, ptr %17, i64 32
  %20 = getelementptr inbounds nuw i8, ptr %17, i64 48
  store <2 x i64> %vec.ind, ptr %17, align 8, !noalias !72
  store <2 x i64> %step.add, ptr %18, align 8, !noalias !72
  store <2 x i64> %step.add.2, ptr %19, align 8, !noalias !72
  store <2 x i64> %step.add.3, ptr %20, align 8, !noalias !72
  %index.next = add nuw i64 %index, 8
  %vec.ind.next = add <2 x i64> %vec.ind, splat (i64 8)
  %21 = icmp eq i64 %index.next, %n.vec
  br i1 %21, label %middle.block, label %vector.body, !llvm.loop !79

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %14, %n.vec
  br i1 %cmp.n, label %bb3, label %bb3.i9.i.i.i.i.i.preheader886

bb3.i9.i.i.i.i.i.preheader886:                    ; preds = %bb3.i9.i.i.i.i.i.preheader, %middle.block
  %.ph887 = phi i64 [ %_4.sroa.5.0.i.i.i.i, %bb3.i9.i.i.i.i.i.preheader ], [ %15, %middle.block ]
  %self.sroa.0.012.i.i.i.i.i.i.ph = phi i64 [ 0, %bb3.i9.i.i.i.i.i.preheader ], [ %n.vec, %middle.block ]
  br label %bb3.i9.i.i.i.i.i

bb3.i9.i.i.i.i.i:                                 ; preds = %bb3.i9.i.i.i.i.i.preheader886, %bb3.i9.i.i.i.i.i
  %22 = phi i64 [ %24, %bb3.i9.i.i.i.i.i ], [ %.ph887, %bb3.i9.i.i.i.i.i.preheader886 ]
  %self.sroa.0.012.i.i.i.i.i.i = phi i64 [ %23, %bb3.i9.i.i.i.i.i ], [ %self.sroa.0.012.i.i.i.i.i.i.ph, %bb3.i9.i.i.i.i.i.preheader886 ]
  %23 = add nuw nsw i64 %self.sroa.0.012.i.i.i.i.i.i, 1
  %_3.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %_4.sroa.10.0.i.i, i64 %22
  store i64 %self.sroa.0.012.i.i.i.i.i.i, ptr %_3.i.i.i.i.i.i.i.i, align 8, !noalias !72
  %24 = add i64 %22, 1
  %exitcond.not.i10.i.i.i.i.i = icmp eq i64 %self.sroa.0.012.i.i.i.i.i.i, %len2
  br i1 %exitcond.not.i10.i.i.i.i.i, label %bb3, label %bb3.i9.i.i.i.i.i, !llvm.loop !82

bb29:                                             ; preds = %cleanup6.loopexit, %cleanup6.loopexit.split-lp, %cleanup.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i, %cleanup9, %cleanup5
  %r.sroa.7.0 = phi i64 [ %r.sroa.7.1.lcssa, %cleanup5 ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %cleanup9 ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %cleanup.i.i.i ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %cleanup6.loopexit ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %cleanup6.loopexit.split-lp ]
  %r.sroa.0.0 = phi i64 [ %r.sroa.0.1.lcssa, %cleanup5 ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %cleanup9 ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %cleanup.i.i.i ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %cleanup6.loopexit ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %cleanup6.loopexit.split-lp ]
  %.pn = phi { ptr, i32 } [ %28, %cleanup5 ], [ %53, %cleanup9 ], [ %lpad.phi.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i ], [ %lpad.phi.i.i.i, %cleanup.i.i.i ], [ %lpad.loopexit, %cleanup6.loopexit ], [ %lpad.loopexit.split-lp, %cleanup6.loopexit.split-lp ]
  %25 = icmp eq i64 %r.sroa.0.0, 0
  br i1 %25, label %bb30, label %bb2.i.i.i4.i49

bb2.i.i.i4.i49:                                   ; preds = %bb29
  %26 = inttoptr i64 %r.sroa.7.0 to ptr
  %alloc_size.i.i.i.i5.i50 = shl nuw i64 %r.sroa.0.0, 3
  %27 = icmp ne i64 %r.sroa.7.0, 0
  tail call void @llvm.assume(i1 %27)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %26, i64 noundef %alloc_size.i.i.i.i5.i50, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb30

cleanup5:                                         ; preds = %panic
  %28 = landingpad { ptr, i32 }
          cleanup
  br label %bb29

bb3:                                              ; preds = %bb3.i9.i.i.i.i.i, %middle.block, %bb3.i.i.i.i.i
  %storemerge.i.i.i.i = phi i64 [ %_4.sroa.5.0.i.i.i.i, %bb3.i.i.i.i.i ], [ %15, %middle.block ], [ %24, %bb3.i9.i.i.i.i.i ]
  %_6.i.i.not.i.i386 = icmp eq ptr %s1.0, %s1.1
  br i1 %_6.i.i.not.i.i386, label %bb42, label %bb14.i.i.i.lr.ph

bb14.i.i.i.lr.ph:                                 ; preds = %bb3
  %_13.i.i.1.i.i = getelementptr inbounds nuw i8, ptr %r1, i64 8
  %_13.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %r1, i64 16
  %_6.i.i.not.i.i70380 = icmp eq ptr %0, %1
  %29 = getelementptr inbounds nuw i8, ptr %last_row_id, i64 8
  %30 = getelementptr inbounds nuw i8, ptr %last_row_id, i64 16
  %31 = getelementptr inbounds nuw i8, ptr %fr, i64 16
  %32 = getelementptr inbounds nuw i8, ptr %fr, i64 8
  br label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb14.i.i.i.lr.ph, %bb20
  %r.sroa.0.1391 = phi i64 [ %_22.0.i.i, %bb14.i.i.i.lr.ph ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %bb20 ]
  %r.sroa.7.1390 = phi i64 [ %13, %bb14.i.i.i.lr.ph ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %bb20 ]
  %r.sroa.15.0389 = phi i64 [ %storemerge.i.i.i.i, %bb14.i.i.i.lr.ph ], [ %_4.sroa.0.0.copyload.i.i.i.2.i.i, %bb20 ]
  %iter.sroa.10.0388 = phi i64 [ 0, %bb14.i.i.i.lr.ph ], [ %_8.0.i178, %bb20 ]
  %iter.sroa.0.0387 = phi ptr [ %s1.0, %bb14.i.i.i.lr.ph ], [ %iter.sroa.0.1176, %bb20 ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0387, i64 1
  %x.i.i.i = load i8, ptr %iter.sroa.0.0387, align 1, !noalias !83, !noundef !17
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i55, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %s1.1
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0387, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !83, !noundef !17
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %33 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb43

bb3.i.i.i55:                                      ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb43

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %s1.1
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0387, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !83, !noundef !17
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %34 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, label %bb43

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %s1.1
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !83, !noundef !17
  %_26.i.i.i53 = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i53, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i54 = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %35 = or disjoint i32 %_27.i.i.i54, %_25.i.i.i
  %.not.i = icmp eq i32 %35, 1114112
  br i1 %.not.i, label %bb42, label %bb41

cleanup6.loopexit:                                ; preds = %bb16.i.i, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %bb29

cleanup6.loopexit.split-lp:                       ; preds = %panic.i.i.invoke, %bb3.i.i.i.i.i123.invoke, %panic7
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb29

bb41:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0387, i64 4
  br label %bb43

bb43:                                             ; preds = %bb3.i.i.i55, %bb6.i.i.i, %bb4.i.i.i, %bb41
  %spec.select.i5.i177 = phi i32 [ %35, %bb41 ], [ %33, %bb4.i.i.i ], [ %34, %bb6.i.i.i ], [ %_7.i.i.i, %bb3.i.i.i55 ]
  %iter.sroa.0.1176 = phi ptr [ %_16.i26.i.i.i, %bb41 ], [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i.i.i.i, %bb3.i.i.i55 ]
  %_8.0.i178 = add i64 %iter.sroa.10.0388, 1
  %_4.sroa.0.0.copyload.i.i.i.i.i = load i64, ptr %r1, align 8, !alias.scope !90, !noalias !93
  store i64 %r.sroa.0.1391, ptr %r1, align 8, !alias.scope !90, !noalias !93
  %_4.sroa.0.0.copyload.i.i.i.1.i.i = load i64, ptr %_13.i.i.1.i.i, align 8, !alias.scope !95, !noalias !97
  store i64 %r.sroa.7.1390, ptr %_13.i.i.1.i.i, align 8, !alias.scope !95, !noalias !97
  %_4.sroa.0.0.copyload.i.i.i.2.i.i = load i64, ptr %_13.i.i.2.i.i, align 8, !alias.scope !99, !noalias !101
  store i64 %r.sroa.15.0389, ptr %_13.i.i.2.i.i, align 8, !alias.scope !99, !noalias !101
  %_95 = icmp ugt i64 %_4.sroa.0.0.copyload.i.i.i.2.i.i, 1
  %36 = inttoptr i64 %r.sroa.7.1390 to ptr
  br i1 %_95, label %bb7, label %panic7

bb42:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, %bb20, %bb3
  %r.sroa.15.0.lcssa = phi i64 [ %storemerge.i.i.i.i, %bb3 ], [ %_4.sroa.0.0.copyload.i.i.i.2.i.i, %bb20 ], [ %r.sroa.15.0389, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %r.sroa.7.1.lcssa = phi i64 [ %13, %bb3 ], [ %_4.sroa.0.0.copyload.i.i.i.1.i.i, %bb20 ], [ %r.sroa.7.1390, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %r.sroa.0.1.lcssa = phi i64 [ %_22.0.i.i, %bb3 ], [ %_4.sroa.0.0.copyload.i.i.i.i.i, %bb20 ], [ %r.sroa.0.1391, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %_153 = icmp ult i64 %_18, %r.sroa.15.0.lcssa
  br i1 %_153, label %bb62, label %panic

bb62:                                             ; preds = %bb42
  %37 = inttoptr i64 %r.sroa.7.1.lcssa to ptr
  %_81 = getelementptr inbounds nuw i64, ptr %37, i64 %_18
  %_80 = load i64, ptr %_81, align 8, !noundef !17
  %38 = icmp eq i64 %r.sroa.0.1.lcssa, 0
  br i1 %38, label %bb22, label %bb2.i.i.i4.i56

bb2.i.i.i4.i56:                                   ; preds = %bb62
  %alloc_size.i.i.i.i5.i57 = shl nuw i64 %r.sroa.0.1.lcssa, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %37, i64 noundef %alloc_size.i.i.i.i5.i57, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb22

panic:                                            ; preds = %bb42
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_18, i64 noundef %r.sroa.15.0.lcssa, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_979475be75b63577188022943ab16aa0) #24
          to label %unreachable unwind label %cleanup5

unreachable:                                      ; preds = %panic7, %panic
  unreachable

bb22:                                             ; preds = %bb2.i.i.i4.i56, %bb62
  %r1.val37 = load i64, ptr %r1, align 8
  %39 = icmp eq i64 %r1.val37, 0
  br i1 %39, label %bb23, label %bb2.i.i.i4.i59

bb2.i.i.i4.i59:                                   ; preds = %bb22
  %40 = getelementptr inbounds nuw i8, ptr %r1, i64 8
  %r1.val38 = load ptr, ptr %40, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i60 = shl nuw i64 %r1.val37, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %r1.val38, i64 noundef %alloc_size.i.i.i.i5.i60, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb23

bb23:                                             ; preds = %bb2.i.i.i4.i59, %bb22
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %r1)
  %fr.val35 = load i64, ptr %fr, align 8
  %41 = icmp eq i64 %fr.val35, 0
  br i1 %41, label %bb24, label %bb2.i.i.i4.i62

bb2.i.i.i4.i62:                                   ; preds = %bb23
  %42 = getelementptr inbounds nuw i8, ptr %fr, i64 8
  %fr.val36 = load ptr, ptr %42, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i63 = shl nuw i64 %fr.val35, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %fr.val36, i64 noundef %alloc_size.i.i.i.i5.i63, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb24

bb24:                                             ; preds = %bb2.i.i.i4.i62, %bb23
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %fr)
  %last_row_id.val = load i64, ptr %last_row_id, align 8, !range !45, !noundef !17
  switch i64 %last_row_id.val, label %bb2.i.i.i4.i.i.i.i65 [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtCs6KJnav5oeQt_6strsim24HybridGrowingHashmapCharNtBJ_5RowIdEEBJ_.exit67
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtCs6KJnav5oeQt_6strsim24HybridGrowingHashmapCharNtBJ_5RowIdEEBJ_.exit67
  ]

bb2.i.i.i4.i.i.i.i65:                             ; preds = %bb24
  %43 = getelementptr inbounds nuw i8, ptr %last_row_id, i64 8
  %last_row_id.val41 = load ptr, ptr %43, align 8, !nonnull !17, !noundef !17
  %alloc_size.i.i.i.i5.i.i.i.i66 = shl nuw i64 %last_row_id.val, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %last_row_id.val41, i64 noundef %alloc_size.i.i.i.i5.i.i.i.i66, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtCs6KJnav5oeQt_6strsim24HybridGrowingHashmapCharNtBJ_5RowIdEEBJ_.exit67

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtCs6KJnav5oeQt_6strsim24HybridGrowingHashmapCharNtBJ_5RowIdEEBJ_.exit67: ; preds = %bb24, %bb24, %bb2.i.i.i4.i.i.i.i65
  call void @llvm.lifetime.end.p0(i64 2088, ptr nonnull %last_row_id)
  ret i64 %_80

panic7:                                           ; preds = %bb43
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef 1, i64 noundef %_4.sroa.0.0.copyload.i.i.i.2.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_62d10971df7d5a4785f495974ef28733) #24
          to label %unreachable unwind label %cleanup6.loopexit.split-lp

bb7:                                              ; preds = %bb43
  %44 = inttoptr i64 %_4.sroa.0.0.copyload.i.i.i.1.i.i to ptr
  %_28 = getelementptr inbounds nuw i8, ptr %44, i64 8
  %45 = load i64, ptr %_28, align 8, !noundef !17
  store i64 %_8.0.i178, ptr %_28, align 8
  br i1 %_6.i.i.not.i.i70380, label %bb48, label %bb14.i.i.i71.lr.ph

bb14.i.i.i71.lr.ph:                               ; preds = %bb7
  %46 = load i64, ptr %last_row_id, align 8, !range !45
  %.not.i135 = icmp eq i64 %46, -9223372036854775808
  %.val.i.i = load ptr, ptr %29, align 8
  %.val1.i.i = load i64, ptr %30, align 8
  %_5.i = load i32, ptr %_1.sroa.8.0._0.sroa_idx.i, align 8
  %_4.i146 = sext i32 %_5.i to i64
  %_135 = load i64, ptr %31, align 8
  %_136 = load ptr, ptr %32, align 8, !nonnull !17
  %47 = tail call i64 @llvm.usub.sat.i64(i64 %r.sroa.15.0389, i64 1)
  %48 = tail call i64 @llvm.usub.sat.i64(i64 %r.sroa.15.0389, i64 2)
  %49 = add i64 %_4.sroa.0.0.copyload.i.i.i.2.i.i, -2
  br label %bb14.i.i.i71

bb14.i.i.i71:                                     ; preds = %bb14.i.i.i71.lr.ph, %bb61
  %last_col_id.sroa.0.0385 = phi i64 [ -1, %bb14.i.i.i71.lr.ph ], [ %last_col_id.sroa.0.1, %bb61 ]
  %t.sroa.0.0384 = phi i64 [ %max_val, %bb14.i.i.i71.lr.ph ], [ %t.sroa.0.1, %bb61 ]
  %last_i2l1.sroa.0.0383 = phi i64 [ %45, %bb14.i.i.i71.lr.ph ], [ %109, %bb61 ]
  %iter1.sroa.10.0382 = phi i64 [ 0, %bb14.i.i.i71.lr.ph ], [ %_8.0.i88192, %bb61 ]
  %iter1.sroa.0.0381 = phi ptr [ %0, %bb14.i.i.i71.lr.ph ], [ %iter1.sroa.0.1190, %bb61 ]
  %_16.i.i.i.i72 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0381, i64 1
  %x.i.i.i73 = load i8, ptr %iter1.sroa.0.0381, align 1, !noalias !103, !noundef !17
  %_6.i.i.i74 = icmp sgt i8 %x.i.i.i73, -1
  br i1 %_6.i.i.i74, label %bb3.i.i.i112, label %bb4.i.i.i75

bb4.i.i.i75:                                      ; preds = %bb14.i.i.i71
  %_30.i.i.i76 = and i8 %x.i.i.i73, 31
  %init.i.i.i77 = zext nneg i8 %_30.i.i.i76 to i32
  %_6.i10.i.i.i78 = icmp ne ptr %_16.i.i.i.i72, %1
  tail call void @llvm.assume(i1 %_6.i10.i.i.i78)
  %_16.i12.i.i.i79 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0381, i64 2
  %y.i.i.i80 = load i8, ptr %_16.i.i.i.i72, align 1, !noalias !103, !noundef !17
  %_33.i.i.i81 = shl nuw nsw i32 %init.i.i.i77, 6
  %_35.i.i.i82 = and i8 %y.i.i.i80, 63
  %_34.i.i.i83 = zext nneg i8 %_35.i.i.i82 to i32
  %50 = or disjoint i32 %_33.i.i.i81, %_34.i.i.i83
  %_13.i.i.i84 = icmp samesign ugt i8 %x.i.i.i73, -33
  br i1 %_13.i.i.i84, label %bb6.i.i.i91, label %bb49

bb3.i.i.i112:                                     ; preds = %bb14.i.i.i71
  %_7.i.i.i113 = zext nneg i8 %x.i.i.i73 to i32
  br label %bb49

bb6.i.i.i91:                                      ; preds = %bb4.i.i.i75
  %_6.i17.i.i.i92 = icmp ne ptr %_16.i12.i.i.i79, %1
  tail call void @llvm.assume(i1 %_6.i17.i.i.i92)
  %_16.i19.i.i.i93 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0381, i64 3
  %z.i.i.i94 = load i8, ptr %_16.i12.i.i.i79, align 1, !noalias !103, !noundef !17
  %_38.i.i.i95 = shl nuw nsw i32 %_34.i.i.i83, 6
  %_40.i.i.i96 = and i8 %z.i.i.i94, 63
  %_39.i.i.i97 = zext nneg i8 %_40.i.i.i96 to i32
  %y_z.i.i.i98 = or disjoint i32 %_38.i.i.i95, %_39.i.i.i97
  %_20.i.i.i99 = shl nuw nsw i32 %init.i.i.i77, 12
  %51 = or disjoint i32 %y_z.i.i.i98, %_20.i.i.i99
  %_21.i.i.i100 = icmp samesign ugt i8 %x.i.i.i73, -17
  br i1 %_21.i.i.i100, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i101, label %bb49

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i101: ; preds = %bb6.i.i.i91
  %_6.i24.i.i.i102 = icmp ne ptr %_16.i19.i.i.i93, %1
  tail call void @llvm.assume(i1 %_6.i24.i.i.i102)
  %w.i.i.i104 = load i8, ptr %_16.i19.i.i.i93, align 1, !noalias !103, !noundef !17
  %_26.i.i.i105 = shl nuw nsw i32 %init.i.i.i77, 18
  %_25.i.i.i106 = and i32 %_26.i.i.i105, 1835008
  %_43.i.i.i107 = shl nuw nsw i32 %y_z.i.i.i98, 6
  %_45.i.i.i108 = and i8 %w.i.i.i104, 63
  %_44.i.i.i109 = zext nneg i8 %_45.i.i.i108 to i32
  %_27.i.i.i110 = or disjoint i32 %_43.i.i.i107, %_44.i.i.i109
  %52 = or disjoint i32 %_27.i.i.i110, %_25.i.i.i106
  %.not.i111 = icmp eq i32 %52, 1114112
  br i1 %.not.i111, label %bb48, label %bb47

cleanup9:                                         ; preds = %panic17.invoke, %panic2.i.invoke
  %53 = landingpad { ptr, i32 }
          cleanup
  br label %bb29

bb47:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i101
  %_16.i26.i.i.i103 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0381, i64 4
  br label %bb49

bb49:                                             ; preds = %bb3.i.i.i112, %bb6.i.i.i91, %bb4.i.i.i75, %bb47
  %spec.select.i5.i86191 = phi i32 [ %52, %bb47 ], [ %50, %bb4.i.i.i75 ], [ %51, %bb6.i.i.i91 ], [ %_7.i.i.i113, %bb3.i.i.i112 ]
  %iter1.sroa.0.1190 = phi ptr [ %_16.i26.i.i.i103, %bb47 ], [ %_16.i12.i.i.i79, %bb4.i.i.i75 ], [ %_16.i19.i.i.i93, %bb6.i.i.i91 ], [ %_16.i.i.i.i72, %bb3.i.i.i112 ]
  %_8.0.i88192 = add nuw i64 %iter1.sroa.10.0382, 1
  %exitcond.not = icmp eq i64 %iter1.sroa.10.0382, %47
  br i1 %exitcond.not, label %panic17.invoke, label %bb51

bb48:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i101, %bb61, %bb7
  tail call void @llvm.experimental.noalias.scope.decl(metadata !110)
  %_4.i = icmp samesign ult i32 %spec.select.i5.i177, 256
  br i1 %_4.i, label %bb1.i, label %bb4.i

bb4.i:                                            ; preds = %bb48
  tail call void @llvm.experimental.noalias.scope.decl(metadata !113)
  %54 = load i64, ptr %last_row_id, align 8, !range !45, !alias.scope !116, !noundef !17
  %.not.i.i = icmp eq i64 %54, -9223372036854775808
  br i1 %.not.i.i, label %bb1.i.i, label %start.bb16_crit_edge.i.i

start.bb16_crit_edge.i.i:                         ; preds = %bb4.i
  %_36.pre.i.i = load i64, ptr %30, align 8, !alias.scope !116
  br label %bb16.i.i

bb1.i.i:                                          ; preds = %bb4.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !117)
  store i32 7, ptr %_1.sroa.8.0._0.sroa_idx.i, align 8, !alias.scope !120
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !121
; call __rustc::__rust_alloc
  %55 = tail call noundef align 8 dereferenceable_or_null(128) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 128, i64 noundef range(i64 1, 9) 8) #23, !noalias !121
  %56 = icmp eq ptr %55, null
  br i1 %56, label %bb3.i.i.i.i.i123.invoke, label %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_.exit.i.i

bb3.i.i.i.i.i123.invoke:                          ; preds = %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i120, %bb1.i.i
  %57 = phi i64 [ 8, %bb1.i.i ], [ 8, %bb3.i.i.i.i.i.i120 ], [ 0, %bb6.i.i.i.i.i.i.i ]
  %58 = phi i64 [ 128, %bb1.i.i ], [ %new_size2.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i120 ], [ undef, %bb6.i.i.i.i.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %57, i64 %58) #24
          to label %bb3.i.i.i.i.i123.cont unwind label %cleanup6.loopexit.split-lp

bb3.i.i.i.i.i123.cont:                            ; preds = %bb3.i.i.i.i.i123.invoke
  unreachable

_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_.exit.i.i: ; preds = %bb1.i.i
  store i64 -1, ptr %55, align 8, !noalias !126
  %59 = getelementptr inbounds nuw i8, ptr %55, i64 8
  store i32 0, ptr %59, align 8, !noalias !126
  %_15.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 16
  store i64 -1, ptr %_15.i.i.i.i.i, align 8, !noalias !126
  %60 = getelementptr inbounds nuw i8, ptr %55, i64 24
  store i32 0, ptr %60, align 8, !noalias !126
  %_15.i.i.1.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 32
  store i64 -1, ptr %_15.i.i.1.i.i.i, align 8, !noalias !126
  %61 = getelementptr inbounds nuw i8, ptr %55, i64 40
  store i32 0, ptr %61, align 8, !noalias !126
  %_15.i.i.2.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 48
  store i64 -1, ptr %_15.i.i.2.i.i.i, align 8, !noalias !126
  %62 = getelementptr inbounds nuw i8, ptr %55, i64 56
  store i32 0, ptr %62, align 8, !noalias !126
  %_15.i.i.3.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 64
  store i64 -1, ptr %_15.i.i.3.i.i.i, align 8, !noalias !126
  %63 = getelementptr inbounds nuw i8, ptr %55, i64 72
  store i32 0, ptr %63, align 8, !noalias !126
  %_15.i.i.4.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 80
  store i64 -1, ptr %_15.i.i.4.i.i.i, align 8, !noalias !126
  %64 = getelementptr inbounds nuw i8, ptr %55, i64 88
  store i32 0, ptr %64, align 8, !noalias !126
  %_15.i.i.5.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 96
  store i64 -1, ptr %_15.i.i.5.i.i.i, align 8, !noalias !126
  %65 = getelementptr inbounds nuw i8, ptr %55, i64 104
  store i32 0, ptr %65, align 8, !noalias !126
  %_15.i.i.6.i.i.i = getelementptr inbounds nuw i8, ptr %55, i64 112
  store i64 -1, ptr %_15.i.i.6.i.i.i, align 8, !noalias !126
  %66 = getelementptr inbounds nuw i8, ptr %55, i64 120
  store i32 0, ptr %66, align 8, !noalias !126
  store i64 8, ptr %last_row_id, align 8, !alias.scope !120
  store ptr %55, ptr %29, align 8, !alias.scope !120
  store i64 8, ptr %30, align 8, !alias.scope !120
  br label %bb16.i.i

bb16.i.i:                                         ; preds = %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_.exit.i.i, %start.bb16_crit_edge.i.i
  %_36.i.i = phi i64 [ 8, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_.exit.i.i ], [ %_36.pre.i.i, %start.bb16_crit_edge.i.i ]
  %67 = phi i64 [ 8, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_.exit.i.i ], [ %54, %start.bb16_crit_edge.i.i ]
; invoke <strsim::GrowingHashmapChar<strsim::RowId>>::lookup
  %68 = invoke fastcc noundef i64 @_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2088) %last_row_id, i32 noundef range(i32 256, 1114112) %spec.select.i5.i177)
          to label %.noexc125 unwind label %cleanup6.loopexit

.noexc125:                                        ; preds = %bb16.i.i
  %_38.i.i = icmp ult i64 %68, %_36.i.i
  br i1 %_38.i.i, label %bb17.i.i, label %panic.i.i.invoke

bb17.i.i:                                         ; preds = %.noexc125
  %_37.i.i = load ptr, ptr %29, align 8, !alias.scope !116, !nonnull !17, !noundef !17
  %_8.i.i = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_37.i.i, i64 %68
  %_8.val.i.i = load i64, ptr %_8.i.i, align 8, !noalias !116, !noundef !17
  %_0.i.i.i = icmp eq i64 %_8.val.i.i, -1
  br i1 %_0.i.i.i, label %bb6.i.i117, label %bb19.i.i

panic.i.i.invoke:                                 ; preds = %bb19.i.i, %.noexc125
  %69 = phi i64 [ %68, %.noexc125 ], [ %i.sroa.0.0.i.i, %bb19.i.i ]
  %70 = phi i64 [ %_36.i.i, %.noexc125 ], [ %_45.i.i, %bb19.i.i ]
  %71 = phi ptr [ @alloc_96408fa515ac8b1e77c68a6bc3e8f856, %.noexc125 ], [ @alloc_caa5cd3ec5b4a333d6ac178e6b813739, %bb19.i.i ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %69, i64 noundef %70, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %71) #25
          to label %panic.i.i.cont unwind label %cleanup6.loopexit.split-lp

panic.i.i.cont:                                   ; preds = %panic.i.i.invoke
  unreachable

bb6.i.i117:                                       ; preds = %bb17.i.i
  %72 = load i32, ptr %_1.sroa.7.0._0.sroa_idx.i, align 4, !alias.scope !116, !noundef !17
  %73 = add i32 %72, 1
  store i32 %73, ptr %_1.sroa.7.0._0.sroa_idx.i, align 4, !alias.scope !116
  %_15.i.i = mul i32 %73, 3
  %_19.i.i = load i32, ptr %_1.sroa.8.0._0.sroa_idx.i, align 8, !alias.scope !116, !noundef !17
  %_18.i.i = shl i32 %_19.i.i, 1
  %_17.i.i = add i32 %_18.i.i, 2
  %_14.not.i.i = icmp slt i32 %_15.i.i, %_17.i.i
  %.pre.i.i = load i32, ptr %_1.sroa.62.0._0.sroa_idx.i, align 8, !alias.scope !116
  br i1 %_14.not.i.i, label %bb11.i.i, label %bb7.i.i118

bb7.i.i118:                                       ; preds = %bb6.i.i117
  %_22.i.i = shl i32 %.pre.i.i, 1
  %_21.i.i = add i32 %_22.i.i, 2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !129)
  %74 = add i32 %_19.i.i, 1
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb1.i.i.i, %bb7.i.i118
  %new_size.sroa.0.0.i.i.i = phi i32 [ %74, %bb7.i.i118 ], [ %75, %bb1.i.i.i ]
  %_5.not.i.i.i = icmp sgt i32 %new_size.sroa.0.0.i.i.i, %_21.i.i
  %75 = shl i32 %new_size.sroa.0.0.i.i.i, 1
  br i1 %_5.not.i.i.i, label %bb21.i.i.i, label %bb1.i.i.i

bb21.i.i.i:                                       ; preds = %bb1.i.i.i
  store i32 %.pre.i.i, ptr %_1.sroa.7.0._0.sroa_idx.i, align 4, !alias.scope !132
  %76 = add nsw i32 %new_size.sroa.0.0.i.i.i, -1
  store i32 %76, ptr %_1.sroa.8.0._0.sroa_idx.i, align 8, !alias.scope !132
  %_13.i.i.i119 = sext i32 %new_size.sroa.0.0.i.i.i to i64
  %_23.i.i.i.i.i.i.i = icmp eq i32 %new_size.sroa.0.0.i.i.i, 0
  br i1 %_23.i.i.i.i.i.i.i, label %_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i, label %bb6.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb21.i.i.i
  %_24.i.i.i.i.i.i.i = add nsw i64 %_13.i.i.i119, -1
  %_27.0.i.i.i.i.i.i.i = shl nsw i64 %_24.i.i.i.i.i.i.i, 4
  %_27.1.i.i.i.i.i.i.i = icmp ugt i64 %_24.i.i.i.i.i.i.i, 1152921504606846975
  %_32.i.i.i.i.i.i.i = icmp ugt i64 %_27.0.i.i.i.i.i.i.i, 9223372036854775800
  %or.cond.i.i.i.i.i.i.i = or i1 %_27.1.i.i.i.i.i.i.i, %_32.i.i.i.i.i.i.i
  br i1 %or.cond.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i123.invoke, label %bb3.i.i.i.i.i.i120, !prof !133

bb3.i.i.i.i.i.i120:                               ; preds = %bb6.i.i.i.i.i.i.i
  %new_size2.i.i.i.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i.i.i.i, 16
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !134
; call __rustc::__rust_alloc
  %77 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !134
  %78 = icmp eq ptr %77, null
  br i1 %78, label %bb3.i.i.i.i.i123.invoke, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i120
  %_2418.i.not.i.i.i.i = icmp eq i32 %new_size.sroa.0.0.i.i.i, 1
  br i1 %_2418.i.not.i.i.i.i, label %bb4.i.i.i.i.i, label %bb3.i4.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb3.i4.i.i.i.i, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i
  %ptr.sroa.0.0.lcssa.i9.i.i.i.i = phi ptr [ %77, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i ], [ %_15.i.i.i7.i.i, %bb3.i4.i.i.i.i ]
  store i64 -1, ptr %ptr.sroa.0.0.lcssa.i9.i.i.i.i, align 8, !noalias !139
  %79 = getelementptr inbounds nuw i8, ptr %ptr.sroa.0.0.lcssa.i9.i.i.i.i, i64 8
  store i32 0, ptr %79, align 8, !noalias !139
  br label %_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i

bb3.i4.i.i.i.i:                                   ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i, %bb3.i4.i.i.i.i
  %ptr.sroa.0.021.i.i.i.i.i = phi ptr [ %_15.i.i.i7.i.i, %bb3.i4.i.i.i.i ], [ %77, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i ]
  %iter.sroa.0.020.i.i.i.i.i = phi i64 [ %_28.i.i.i.i.i, %bb3.i4.i.i.i.i ], [ 1, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBH_5RowIdEE7reserveBH_.exit.i.i.i.i.i ]
  %_28.i.i.i.i.i = add nuw i64 %iter.sroa.0.020.i.i.i.i.i, 1
  store i64 -1, ptr %ptr.sroa.0.021.i.i.i.i.i, align 8, !noalias !139
  %80 = getelementptr inbounds nuw i8, ptr %ptr.sroa.0.021.i.i.i.i.i, i64 8
  store i32 0, ptr %80, align 8, !noalias !139
  %_15.i.i.i7.i.i = getelementptr inbounds nuw i8, ptr %ptr.sroa.0.021.i.i.i.i.i, i64 16
  %exitcond.not.i.i.i.i.i = icmp eq i64 %_28.i.i.i.i.i, %_13.i.i.i119
  br i1 %exitcond.not.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb3.i4.i.i.i.i

_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i: ; preds = %bb4.i.i.i.i.i, %bb21.i.i.i
  %_4.sroa.10.0.i14.i.i.i.i = phi ptr [ %77, %bb4.i.i.i.i.i ], [ inttoptr (i64 8 to ptr), %bb21.i.i.i ]
  store i64 %_13.i.i.i119, ptr %last_row_id, align 8, !alias.scope !132
  store ptr %_4.sroa.10.0.i14.i.i.i.i, ptr %29, align 8, !alias.scope !132
  store i64 %_13.i.i.i119, ptr %30, align 8, !alias.scope !132
  %_48.i.i.i = icmp ult i64 %_36.i.i, 576460752303423488
  tail call void @llvm.assume(i1 %_48.i.i.i)
  %_41.idx.i.i.i = shl nuw nsw i64 %_36.i.i, 4
  %_41.i.i.i = getelementptr inbounds nuw i8, ptr %_37.i.i, i64 %_41.idx.i.i.i
  br label %bb9.i.i.i

cleanup.loopexit.i.i.i:                           ; preds = %bb10.i.i.i122
  %lpad.loopexit.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i.i

cleanup.loopexit.split-lp.i.i.i:                  ; preds = %panic.i.i.i
  %lpad.loopexit.split-lp.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i.i

cleanup.i.i.i:                                    ; preds = %cleanup.loopexit.split-lp.i.i.i, %cleanup.loopexit.i.i.i
  %lpad.phi.i.i.i = phi { ptr, i32 } [ %lpad.loopexit.i.i.i, %cleanup.loopexit.i.i.i ], [ %lpad.loopexit.split-lp.i.i.i, %cleanup.loopexit.split-lp.i.i.i ]
  %81 = icmp eq i64 %67, 0
  br i1 %81, label %bb29, label %bb2.i.i.i.i.i4.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i:                         ; preds = %cleanup.i.i.i
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i = shl nuw i64 %67, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_37.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !142
  br label %bb29

bb16.i.i.i:                                       ; preds = %bb33.i.i.i, %bb15.i.i.i
  %82 = icmp eq i64 %67, 0
  br i1 %82, label %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i, label %bb2.i.i.i.i.i4.i.i10.i.i.i

bb2.i.i.i.i.i4.i.i10.i.i.i:                       ; preds = %bb16.i.i.i
  %alloc_size.i.i.i.i.i.i6.i.i11.i.i.i = shl nuw i64 %67, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_37.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i11.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !145
  br label %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i

bb9.i.i.i:                                        ; preds = %bb15.i.i.i, %_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i
  %iter.sroa.5.022.i.i.i = phi ptr [ %_23.i.i.i.i121, %bb15.i.i.i ], [ %_37.i.i, %_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i ]
  %83 = phi i32 [ %85, %bb15.i.i.i ], [ %.pre.i.i, %_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_.exit.i.i.i ]
  %_23.i.i.i.i121 = getelementptr inbounds nuw i8, ptr %iter.sroa.5.022.i.i.i, i64 16
  %_17.0.i.i.i.i = load i64, ptr %iter.sroa.5.022.i.i.i, align 8, !noalias !148, !noundef !17
  %84 = getelementptr inbounds nuw i8, ptr %iter.sroa.5.022.i.i.i, i64 8
  %_17.1.i.i.i.i = load i32, ptr %84, align 8, !noalias !148, !noundef !17
  %_0.i.i.not.i.i.i = icmp eq i64 %_17.0.i.i.i.i, -1
  br i1 %_0.i.i.not.i.i.i, label %bb15.i.i.i, label %bb10.i.i.i122

bb10.i.i.i122:                                    ; preds = %bb9.i.i.i
; invoke <strsim::GrowingHashmapChar<strsim::RowId>>::lookup
  %j.i.i.i = invoke fastcc noundef i64 @_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2088) %last_row_id, i32 noundef %_17.1.i.i.i.i)
          to label %bb32.i.i.i unwind label %cleanup.loopexit.i.i.i

bb15.i.i.i:                                       ; preds = %bb33.i.i.i, %bb9.i.i.i
  %85 = phi i32 [ %83, %bb9.i.i.i ], [ %87, %bb33.i.i.i ]
  %_9.i.i.i.i = icmp eq ptr %_23.i.i.i.i121, %_41.i.i.i
  br i1 %_9.i.i.i.i, label %bb16.i.i.i, label %bb9.i.i.i

bb32.i.i.i:                                       ; preds = %bb10.i.i.i122
  %_57.i.i.i = icmp ult i64 %j.i.i.i, %_13.i.i.i119
  br i1 %_57.i.i.i, label %bb33.i.i.i, label %panic.i.i.i

unreachable.i.i.i:                                ; preds = %panic.i.i.i
  unreachable

bb33.i.i.i:                                       ; preds = %bb32.i.i.i
  %new_elem.i.i.i = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_4.sroa.10.0.i14.i.i.i.i, i64 %j.i.i.i
  %86 = getelementptr inbounds nuw i8, ptr %new_elem.i.i.i, i64 8
  store i32 %_17.1.i.i.i.i, ptr %86, align 8, !noalias !132
  store i64 %_17.0.i.i.i.i, ptr %new_elem.i.i.i, align 8, !noalias !132
  %87 = add i32 %83, -1
  store i32 %87, ptr %_1.sroa.62.0._0.sroa_idx.i, align 8, !alias.scope !132
  %88 = icmp eq i32 %87, 0
  br i1 %88, label %bb16.i.i.i, label %bb15.i.i.i

panic.i.i.i:                                      ; preds = %bb32.i.i.i
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %j.i.i.i, i64 noundef %_13.i.i.i119, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1c02a211ff8957e465458a51c31a81de) #24
          to label %unreachable.i.i.i unwind label %cleanup.loopexit.split-lp.i.i.i, !noalias !132

_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i: ; preds = %bb2.i.i.i.i.i4.i.i10.i.i.i, %bb16.i.i.i
  store i32 %.pre.i.i, ptr %_1.sroa.62.0._0.sroa_idx.i, align 8, !alias.scope !132
; invoke <strsim::GrowingHashmapChar<strsim::RowId>>::lookup
  %89 = invoke fastcc noundef i64 @_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(2088) %last_row_id, i32 noundef range(i32 256, 1114112) %spec.select.i5.i177)
          to label %bb11.i.i unwind label %cleanup6.loopexit

bb11.i.i:                                         ; preds = %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i, %bb6.i.i117
  %_4614.i.i = phi ptr [ %_37.i.i, %bb6.i.i117 ], [ %_4.sroa.10.0.i14.i.i.i.i, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i ]
  %_4512.i.i = phi i64 [ %_36.i.i, %bb6.i.i117 ], [ %_13.i.i.i119, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i ]
  %i.sroa.0.1.i.i = phi i64 [ %68, %bb6.i.i117 ], [ %89, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_.exit.i.i ]
  %90 = add i32 %.pre.i.i, 1
  store i32 %90, ptr %_1.sroa.62.0._0.sroa_idx.i, align 8, !alias.scope !116
  br label %bb19.i.i

bb19.i.i:                                         ; preds = %bb11.i.i, %bb17.i.i
  %_46.i.i = phi ptr [ %_4614.i.i, %bb11.i.i ], [ %_37.i.i, %bb17.i.i ]
  %_45.i.i = phi i64 [ %_4512.i.i, %bb11.i.i ], [ %_36.i.i, %bb17.i.i ]
  %i.sroa.0.0.i.i = phi i64 [ %i.sroa.0.1.i.i, %bb11.i.i ], [ %68, %bb17.i.i ]
  %_47.i.i = icmp ult i64 %i.sroa.0.0.i.i, %_45.i.i
  br i1 %_47.i.i, label %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_.exit.i, label %panic.i.i.invoke

_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_.exit.i: ; preds = %bb19.i.i
  %elem.i.i = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_46.i.i, i64 %i.sroa.0.0.i.i
  %91 = getelementptr inbounds nuw i8, ptr %elem.i.i, i64 8
  store i32 %spec.select.i5.i177, ptr %91, align 8, !noalias !116
  br label %bb20

bb1.i:                                            ; preds = %bb48
  %_11.i = zext nneg i32 %spec.select.i5.i177 to i64
  %92 = getelementptr inbounds nuw i64, ptr %2, i64 %_11.i
  br label %bb20

bb20:                                             ; preds = %bb1.i, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_.exit.i
  %_0.sroa.0.0.i116 = phi ptr [ %92, %bb1.i ], [ %elem.i.i, %_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_.exit.i ]
  store i64 %_8.0.i178, ptr %_0.sroa.0.0.i116, align 8
  %_6.i.i.not.i.i = icmp eq ptr %iter.sroa.0.1176, %s1.1
  br i1 %_6.i.i.not.i.i, label %bb42, label %bb14.i.i.i

bb51:                                             ; preds = %bb49
  %_50 = add nuw i64 %iter1.sroa.10.0382, 2
  %exitcond562.not = icmp eq i64 %iter1.sroa.10.0382, %48
  br i1 %exitcond562.not, label %panic17.invoke, label %bb54

bb54:                                             ; preds = %bb51
  %_41 = getelementptr inbounds nuw i64, ptr %36, i64 %_8.0.i88192
  %_40 = load i64, ptr %_41, align 8, !noundef !17
  %_43 = icmp ne i32 %spec.select.i5.i177, %spec.select.i5.i86191
  %_42 = zext i1 %_43 to i64
  %diag = add i64 %_40, %_42
  %_46 = getelementptr inbounds nuw i64, ptr %44, i64 %_8.0.i88192
  %_45 = load i64, ptr %_46, align 8, !noundef !17
  %left = add i64 %_45, 1
  %_49 = getelementptr inbounds nuw i64, ptr %36, i64 %_50
  %_48 = load i64, ptr %_49, align 8, !noundef !17
  %up = add i64 %_48, 1
  %_0.sroa.0.0.i131 = tail call noundef i64 @llvm.smin.i64(i64 %up, i64 %left)
  %_0.sroa.0.0.i132 = tail call noundef i64 @llvm.smin.i64(i64 %_0.sroa.0.0.i131, i64 %diag)
  %_53 = icmp eq i32 %spec.select.i5.i177, %spec.select.i5.i86191
  br i1 %_53, label %bb55, label %bb10

bb10:                                             ; preds = %bb54
  %_4.i133 = icmp samesign ult i32 %spec.select.i5.i86191, 256
  br i1 %_4.i133, label %bb1.i141, label %bb4.i134

bb4.i134:                                         ; preds = %bb10
  br i1 %.not.i135, label %bb11, label %bb19.i

bb19.i:                                           ; preds = %bb4.i134
  %_3.i = zext nneg i32 %spec.select.i5.i86191 to i64
  %93 = and i64 %_4.i146, %_3.i
  %_43.i = icmp ult i64 %93, %.val1.i.i
  br i1 %_43.i, label %bb20.i, label %panic2.i.invoke

bb20.i:                                           ; preds = %bb19.i
  %_10.i = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %.val.i.i, i64 %93
  %_10.val.i = load i64, ptr %_10.i, align 8, !noalias !152, !noundef !17
  %_0.i.i = icmp eq i64 %_10.val.i, -1
  br i1 %_0.i.i, label %_4.i.i.i.noexc, label %bb21.i

bb21.i:                                           ; preds = %bb20.i
  %94 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %.val.i.i, i64 %93, i32 1
  %_15.i = load i32, ptr %94, align 8, !noalias !152, !noundef !17
  %_14.i = icmp eq i32 %_15.i, %spec.select.i5.i86191
  br i1 %_14.i, label %_4.i.i.i.noexc, label %bb8.preheader.i

bb8.preheader.i:                                  ; preds = %bb21.i
  %_2114.i = mul nuw nsw i64 %93, 5
  %_2016.i = add nuw nsw i64 %_3.i, 1
  %_1917.i = add nuw nsw i64 %_2016.i, %_2114.i
  %95 = and i64 %_1917.i, %_4.i146
  %_5318.i = icmp ult i64 %95, %.val1.i.i
  br i1 %_5318.i, label %bb22.i, label %panic2.i.invoke

bb8.i148:                                         ; preds = %bb23.i
  %96 = lshr i32 %perturb.sroa.0.019.i, 5
  %_21.i = mul i64 %98, 5
  %narrow.i = add nuw nsw i32 %96, 1
  %_20.i = zext nneg i32 %narrow.i to i64
  %_19.i = add i64 %_21.i, %_20.i
  %97 = and i64 %_19.i, %_4.i146
  %_53.i = icmp ult i64 %97, %.val1.i.i
  br i1 %_53.i, label %bb22.i, label %panic2.i.invoke

bb22.i:                                           ; preds = %bb8.preheader.i, %bb8.i148
  %98 = phi i64 [ %97, %bb8.i148 ], [ %95, %bb8.preheader.i ]
  %perturb.sroa.0.019.i = phi i32 [ %96, %bb8.i148 ], [ %spec.select.i5.i86191, %bb8.preheader.i ]
  %_28.i = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %.val.i.i, i64 %98
  %_28.val.i = load i64, ptr %_28.i, align 8, !noalias !152, !noundef !17
  %_0.i12.i = icmp eq i64 %_28.val.i, -1
  br i1 %_0.i12.i, label %_4.i.i.i.noexc, label %bb23.i

panic2.i.invoke:                                  ; preds = %_4.i.i.i.noexc, %bb8.preheader.i, %bb19.i, %bb8.i148
  %99 = phi i64 [ %97, %bb8.i148 ], [ %i.sroa.0.1.i, %_4.i.i.i.noexc ], [ %95, %bb8.preheader.i ], [ %93, %bb19.i ]
  %100 = phi ptr [ @alloc_0a640bf0e45dbde7c4f1706a4879d5ca, %bb8.i148 ], [ @alloc_713c70d72f94083925b2dbcd729a89c1, %_4.i.i.i.noexc ], [ @alloc_0a640bf0e45dbde7c4f1706a4879d5ca, %bb8.preheader.i ], [ @alloc_d619813ccfd16ab2fd70bc85a627a0cb, %bb19.i ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %99, i64 noundef %.val1.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %100) #25
          to label %panic2.i.cont unwind label %cleanup9

panic2.i.cont:                                    ; preds = %panic2.i.invoke
  unreachable

bb23.i:                                           ; preds = %bb22.i
  %101 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %.val.i.i, i64 %98, i32 1
  %_33.i = load i32, ptr %101, align 8, !noalias !152, !noundef !17
  %_32.i = icmp eq i32 %_33.i, %spec.select.i5.i86191
  br i1 %_32.i, label %_4.i.i.i.noexc, label %bb8.i148

_4.i.i.i.noexc:                                   ; preds = %bb23.i, %bb22.i, %bb21.i, %bb20.i
  %_0.i.i.i139 = phi i64 [ %_10.val.i, %bb21.i ], [ -1, %bb20.i ], [ %_28.val.i, %bb23.i ], [ -1, %bb22.i ]
  %i.sroa.0.1.i = phi i64 [ %93, %bb21.i ], [ %93, %bb20.i ], [ %98, %bb22.i ], [ %98, %bb23.i ]
  %_13.i.i.i137 = icmp ult i64 %i.sroa.0.1.i, %.val1.i.i
  br i1 %_13.i.i.i137, label %bb11, label %panic2.i.invoke

bb1.i141:                                         ; preds = %bb10
  %_12.i = zext nneg i32 %spec.select.i5.i86191 to i64
  %102 = getelementptr inbounds nuw i64, ptr %2, i64 %_12.i
  %103 = load i64, ptr %102, align 8, !alias.scope !155, !noundef !17
  br label %bb11

bb11:                                             ; preds = %_4.i.i.i.noexc, %bb1.i141, %bb4.i134
  %_0.sroa.0.0.i140 = phi i64 [ %103, %bb1.i141 ], [ -1, %bb4.i134 ], [ %_0.i.i.i139, %_4.i.i.i.noexc ]
  %_62 = sub i64 %_8.0.i88192, %last_col_id.sroa.0.0385
  %104 = icmp eq i64 %_62, 1
  br i1 %104, label %bb12, label %bb13

bb12:                                             ; preds = %bb11
  %_137 = icmp ult i64 %_50, %_135
  br i1 %_137, label %bb57, label %panic17.invoke

bb13:                                             ; preds = %bb11
  %_70 = sub i64 %_8.0.i178, %_0.sroa.0.0.i140
  %105 = icmp eq i64 %_70, 1
  br i1 %105, label %bb14, label %bb18

bb57:                                             ; preds = %bb12
  %_66 = getelementptr inbounds nuw i64, ptr %_136, i64 %_50
  %_65 = load i64, ptr %_66, align 8, !noundef !17
  %_67 = sub i64 %_8.0.i178, %_0.sroa.0.0.i140
  %transpose = add i64 %_67, %_65
  %_0.sroa.0.0.i144 = tail call noundef i64 @llvm.smin.i64(i64 %transpose, i64 %_0.sroa.0.0.i132)
  br label %bb18

bb14:                                             ; preds = %bb13
  %transpose14 = add i64 %_62, %t.sroa.0.0384
  %_0.sroa.0.0.i145 = tail call noundef i64 @llvm.smin.i64(i64 %transpose14, i64 %_0.sroa.0.0.i132)
  br label %bb18

bb18:                                             ; preds = %bb14, %bb57, %bb13, %bb56
  %temp.sroa.0.0 = phi i64 [ %_0.sroa.0.0.i132, %bb56 ], [ %_0.sroa.0.0.i132, %bb13 ], [ %_0.sroa.0.0.i144, %bb57 ], [ %_0.sroa.0.0.i145, %bb14 ]
  %t.sroa.0.1 = phi i64 [ %last_i2l1.sroa.0.0383, %bb56 ], [ %t.sroa.0.0384, %bb13 ], [ %t.sroa.0.0384, %bb57 ], [ %t.sroa.0.0384, %bb14 ]
  %last_col_id.sroa.0.1 = phi i64 [ %_8.0.i88192, %bb56 ], [ %last_col_id.sroa.0.0385, %bb13 ], [ %last_col_id.sroa.0.0385, %bb57 ], [ %last_col_id.sroa.0.0385, %bb14 ]
  %exitcond563.not = icmp eq i64 %iter1.sroa.10.0382, %49
  br i1 %exitcond563.not, label %panic17.invoke, label %bb61

bb55:                                             ; preds = %bb54
  %_132 = icmp ult i64 %_50, %_135
  br i1 %_132, label %bb56, label %panic17.invoke

bb56:                                             ; preds = %bb55
  %_55 = getelementptr inbounds nuw i64, ptr %36, i64 %iter1.sroa.10.0382
  %_54 = load i64, ptr %_55, align 8, !noundef !17
  %_57 = getelementptr inbounds nuw i64, ptr %_136, i64 %_50
  store i64 %_54, ptr %_57, align 8
  br label %bb18

panic17.invoke:                                   ; preds = %bb18, %bb55, %bb12, %bb51, %bb49
  %106 = phi i64 [ %_8.0.i88192, %bb49 ], [ %_50, %bb51 ], [ %_50, %bb12 ], [ %_50, %bb55 ], [ %_4.sroa.0.0.copyload.i.i.i.2.i.i, %bb18 ]
  %107 = phi i64 [ %r.sroa.15.0389, %bb49 ], [ %r.sroa.15.0389, %bb51 ], [ %_135, %bb12 ], [ %_135, %bb55 ], [ %_4.sroa.0.0.copyload.i.i.i.2.i.i, %bb18 ]
  %108 = phi ptr [ @alloc_862551a6b5209c91aedf6fe87f92b332, %bb49 ], [ @alloc_8da7b938e9f69bada7a79d2c6e57c2b6, %bb51 ], [ @alloc_6bf673dbe204d5f5608d7daead0841b4, %bb12 ], [ @alloc_6e277f70d633006168d39b714ca85e89, %bb55 ], [ @alloc_b944b20bc9fbafa729d24f8cbad2dd57, %bb18 ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %106, i64 noundef %107, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %108) #24
          to label %panic17.cont unwind label %cleanup9

panic17.cont:                                     ; preds = %panic17.invoke
  unreachable

bb61:                                             ; preds = %bb18
  %_76 = getelementptr inbounds nuw i64, ptr %44, i64 %_50
  %109 = load i64, ptr %_76, align 8, !noundef !17
  store i64 %temp.sroa.0.0, ptr %_76, align 8
  %_6.i.i.not.i.i70 = icmp eq ptr %iter1.sroa.0.1190, %1
  br i1 %_6.i.i.not.i.i70, label %bb48, label %bb14.i.i.i71

bb34:                                             ; preds = %bb2.i.i.i4.i.i.i.i, %bb32, %bb32
  resume { ptr, i32 } %.pn.pn.pn.pn
}

; <hashbrown::raw::RawTable<((char, char), usize)>>::reserve_rehash::<hashbrown::map::make_hasher<(char, char), usize, std::hash::random::RandomState>::{closure#0}>
; Function Attrs: cold noinline uwtable
define { i64, i64 } @_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, i64 noundef %additional, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %0, i1 noundef zeroext %fallibility) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !158)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self1.i = load i64, ptr %1, align 8, !alias.scope !158, !noalias !161, !noundef !17
  %_21.0.i = add i64 %self1.i, %additional
  %_21.1.i = icmp ult i64 %_21.0.i, %self1.i
  br i1 %_21.1.i, label %bb9.i, label %bb11.i, !prof !9

bb11.i:                                           ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %3 = load i64, ptr %2, align 8, !alias.scope !158, !noalias !161, !noundef !17
  %_24.i = icmp ult i64 %3, 8
  %_26.i = add i64 %3, 1
  %_255.i = lshr i64 %_26.i, 3
  %4 = mul nuw i64 %_255.i, 7
  %full_capacity.sroa.0.0.i = select i1 %_24.i, i64 %3, i64 %4
  %_146.i = lshr i64 %full_capacity.sroa.0.0.i, 1
  %_13.not.i = icmp ugt i64 %_21.0.i, %_146.i
  br i1 %_13.not.i, label %bb4.i, label %bb2.i

bb9.i:                                            ; preds = %start
; call <hashbrown::raw::Fallibility>::capacity_overflow
  %5 = tail call { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility17capacity_overflow(i1 noundef zeroext %fallibility), !noalias !163
  %_11.0.i = extractvalue { i64, i64 } %5, 0
  %_11.1.i = extractvalue { i64, i64 } %5, 1
  br label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

bb4.i:                                            ; preds = %bb11.i
  %_19.i = add nuw i64 %full_capacity.sroa.0.0.i, 1
  %_0.sroa.0.0.i5 = tail call noundef i64 @llvm.umax.i64(i64 %_19.i, i64 %_21.0.i)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !164)
  %_6.i.i = icmp ult i64 %_0.sroa.0.0.i5, 15
  br i1 %_6.i.i, label %bb10.thread.i, label %bb13.i.i7

bb13.i.i7:                                        ; preds = %bb4.i
  %_30.1.i.i = icmp ugt i64 %_0.sroa.0.0.i5, 2305843009213693951
  br i1 %_30.1.i.i, label %bb9.i9, label %bb10.i, !prof !9

bb10.thread.i:                                    ; preds = %bb4.i
  %_13.i.i = icmp samesign ult i64 %_0.sroa.0.0.i5, 4
  %6 = and i64 %_0.sroa.0.0.i5, 8
  %..i.i = add nuw nsw i64 %6, 8
  %buckets.sroa.0.0.i.i = select i1 %_13.i.i, i64 4, i64 %..i.i
  br label %bb11.i.i.i

bb10.i:                                           ; preds = %bb13.i.i7
  %_30.0.i.i = shl nuw i64 %_0.sroa.0.0.i5, 3
  %adjusted_cap.i.i = udiv i64 %_30.0.i.i, 7
  %p.i.i = add nsw i64 %adjusted_cap.i.i, -1
  %7 = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %p.i.i, i1 true)
  %8 = lshr i64 -1, %7
  %9 = add nuw nsw i64 %8, 1
  %_25.1.i.i.i = icmp samesign ugt i64 %8, 1152921504606846974
  br i1 %_25.1.i.i.i, label %bb3.i.i, label %bb11.i.i.i, !prof !167

bb11.i.i.i:                                       ; preds = %bb10.i, %bb10.thread.i
  %_0.sroa.4.0.i.ph8.i = phi i64 [ %buckets.sroa.0.0.i.i, %bb10.thread.i ], [ %9, %bb10.i ]
  %_25.0.i.i.i = shl nuw i64 %_0.sroa.4.0.i.ph8.i, 4
  %rhs5.i.i.i = add nuw nsw i64 %_0.sroa.4.0.i.ph8.i, 8
  %_37.0.i.i.i = add i64 %rhs5.i.i.i, %_25.0.i.i.i
  %_37.1.i.i.i = icmp ult i64 %_37.0.i.i.i, %_25.0.i.i.i
  %_19.i.i.i = icmp ugt i64 %_37.0.i.i.i, 9223372036854775800
  %or.cond.i.i = or i1 %_37.1.i.i.i, %_19.i.i.i
  br i1 %or.cond.i.i, label %bb3.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i, !prof !133

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i: ; preds = %bb11.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !168
; call __rustc::__rust_alloc
  %10 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %_37.0.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !168
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb14.i.i8, label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

bb3.i.i:                                          ; preds = %bb11.i.i.i, %bb10.i
; call <hashbrown::raw::Fallibility>::capacity_overflow
  %12 = tail call { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility17capacity_overflow(i1 noundef zeroext %fallibility), !noalias !168
  br label %bb11.i.i

bb14.i.i8:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i
; call <hashbrown::raw::Fallibility>::alloc_err
  %13 = tail call { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility9alloc_err(i1 noundef zeroext %fallibility, i64 noundef 8, i64 noundef %_37.0.i.i.i), !noalias !168
  br label %bb11.i.i

bb9.i9:                                           ; preds = %bb13.i.i7
; call <hashbrown::raw::Fallibility>::capacity_overflow
  %14 = tail call { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility17capacity_overflow(i1 noundef zeroext %fallibility), !noalias !173
  br label %bb11.i.i

_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i
  %_26.i.i = add nsw i64 %_0.sroa.4.0.i.ph8.i, -1
  %_43.i.i = icmp samesign ult i64 %_26.i.i, 8
  %_447.i.i = lshr i64 %_0.sroa.4.0.i.ph8.i, 3
  %15 = mul nuw nsw i64 %_447.i.i, 7
  %bucket_mask.sroa.0.0.i.i = select i1 %_43.i.i, i64 %_26.i.i, i64 %15
  %ptr.i.i = getelementptr inbounds nuw i8, ptr %10, i64 %_25.0.i.i.i
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(1) %ptr.i.i, i8 -1, i64 %rhs5.i.i.i, i1 false), !noalias !173
  %16 = ptrtoint ptr %ptr.i.i to i64
  %invariant.gep = getelementptr i8, ptr %ptr.i.i, i64 8
  %17 = icmp eq i64 %self1.i, 0
  br i1 %17, label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit.bb4.i.i_crit_edge, label %bb1.i.preheader.lr.ph

_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit.bb4.i.i_crit_edge: ; preds = %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit
  %_3.sroa.0.0.copyload.i.i.i.i.i.pre = load i64, ptr %self, align 8, !alias.scope !174, !noalias !177
  %_3.sroa.0.0.copyload.i.i.i.i.i.pre.ptr = inttoptr i64 %_3.sroa.0.0.copyload.i.i.i.i.i.pre to ptr
  br label %bb4.i.i

bb1.i.preheader.lr.ph:                            ; preds = %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit
  %_57.i.i = load ptr, ptr %self, align 8, !alias.scope !179, !noalias !180, !nonnull !17, !noundef !17
  %tmp.sroa.0.0.copyload.i.i = load <8 x i8>, ptr %_57.i.i, align 1, !noalias !182
  %18 = icmp sgt <8 x i8> %tmp.sroa.0.0.copyload.i.i, splat (i8 -1)
  %19 = sext <8 x i1> %18 to <8 x i8>
  %20 = bitcast <8 x i8> %19 to i64
  %_64.i.i = and i64 %20, -9187201950435737472
  %hash_builder.val.i.i = load i64, ptr %0, align 8, !noalias !187, !noundef !17
  %21 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %hash_builder.val1.i.i = load i64, ptr %21, align 8, !noalias !187, !noundef !17
  %22 = xor i64 %hash_builder.val.i.i, 7816392313619706465
  %23 = xor i64 %hash_builder.val1.i.i, 7237128888997146477
  %24 = xor i64 %hash_builder.val.i.i, 8317987319222330741
  %_2.i.i.i.i.i.i.i.i = add i64 %23, %24
  %25 = tail call noundef i64 @llvm.fshl.i64(i64 %23, i64 %23, i64 13)
  %26 = xor i64 %25, %_2.i.i.i.i.i.i.i.i
  %27 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i.i, i64 32)
  %invariant.op46 = add i64 %22, %26
  %28 = tail call noundef i64 @llvm.fshl.i64(i64 %26, i64 %26, i64 17)
  %invariant.op68 = xor i64 %hash_builder.val1.i.i, 8387220255154660723
  br label %bb1.i.preheader

bb11.i.i:                                         ; preds = %bb3.i.i, %bb14.i.i8, %bb9.i9
  %.pn = phi { i64, i64 } [ %14, %bb9.i9 ], [ %13, %bb14.i.i8 ], [ %12, %bb3.i.i ]
  %self3.i.i.sroa.7.033 = extractvalue { i64, i64 } %.pn, 0
  %self3.i.i.sroa.12.034 = extractvalue { i64, i64 } %.pn, 1
  br label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

bb1.i.preheader:                                  ; preds = %bb1.i.preheader.lr.ph, %bb18.i.i
  %iter.i.i.sroa.0.045 = phi ptr [ %_57.i.i, %bb1.i.preheader.lr.ph ], [ %iter.i.i.sroa.0.1.lcssa, %bb18.i.i ]
  %iter.i.i.sroa.5.044 = phi i64 [ %_64.i.i, %bb1.i.preheader.lr.ph ], [ %_32.i, %bb18.i.i ]
  %iter.i.i.sroa.9.043 = phi i64 [ 0, %bb1.i.preheader.lr.ph ], [ %iter.i.i.sroa.9.1.lcssa, %bb18.i.i ]
  %iter.i.i.sroa.13.042 = phi i64 [ %self1.i, %bb1.i.preheader.lr.ph ], [ %40, %bb18.i.i ]
  %.not.i37 = icmp eq i64 %iter.i.i.sroa.5.044, 0
  br i1 %.not.i37, label %self6.i.noexc, label %bb5.i.i

self6.i.noexc:                                    ; preds = %bb1.i.preheader, %self6.i.noexc
  %iter.i.i.sroa.0.139 = phi ptr [ %ptr.i, %self6.i.noexc ], [ %iter.i.i.sroa.0.045, %bb1.i.preheader ]
  %iter.i.i.sroa.9.138 = phi i64 [ %32, %self6.i.noexc ], [ %iter.i.i.sroa.9.043, %bb1.i.preheader ]
  %ptr.i = getelementptr inbounds nuw i8, ptr %iter.i.i.sroa.0.139, i64 8
  %tmp.sroa.0.0.copyload.i.i11 = load <8 x i8>, ptr %ptr.i, align 1, !noalias !191
  %29 = icmp sgt <8 x i8> %tmp.sroa.0.0.copyload.i.i11, splat (i8 -1)
  %30 = sext <8 x i1> %29 to <8 x i8>
  %31 = bitcast <8 x i8> %30 to i64
  %_43.i = and i64 %31, -9187201950435737472
  %32 = add i64 %iter.i.i.sroa.9.138, 8
  %.not.i = icmp eq i64 %_43.i, 0
  br i1 %.not.i, label %self6.i.noexc, label %bb5.i.i

bb4.i.i:                                          ; preds = %bb18.i.i, %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit.bb4.i.i_crit_edge
  %_3.sroa.0.0.copyload.i.i.i.i.i.ptr = phi ptr [ %_3.sroa.0.0.copyload.i.i.i.i.i.pre.ptr, %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit.bb4.i.i_crit_edge ], [ %_57.i.i, %bb18.i.i ]
  %33 = sub i64 %bucket_mask.sroa.0.0.i.i, %self1.i
  store i64 %16, ptr %self, align 8, !alias.scope !174, !noalias !177
  store i64 %_26.i.i, ptr %2, align 8, !alias.scope !196, !noalias !198
  %_11.i.i.2.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  store i64 %33, ptr %_11.i.i.2.i.i, align 8, !alias.scope !200, !noalias !202
  %34 = icmp eq i64 %3, 0
  br i1 %34, label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb4.i.i
  %_9.i.i.i = shl i64 %3, 4
  %35 = add i64 %_9.i.i.i, 16
  %_32.0.i.i.i.i = add i64 %_9.i.i.i, 23
  %_32.1.i.i.i.i = icmp uge i64 %_32.0.i.i.i.i, %35
  tail call void @llvm.assume(i1 %_32.1.i.i.i.i)
  %ctrl_offset.i.i.i.i = and i64 %_32.0.i.i.i.i, -16
  %rhs5.i.i.i.i = add i64 %3, 9
  %_37.0.i.i.i.i = add i64 %rhs5.i.i.i.i, %ctrl_offset.i.i.i.i
  %_37.1.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i, %ctrl_offset.i.i.i.i
  %_19.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i, 9223372036854775801
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i)
  %36 = icmp ne ptr %_3.sroa.0.0.copyload.i.i.i.i.i.ptr, null
  tail call void @llvm.assume(i1 %36)
  %37 = icmp eq i64 %_37.0.i.i.i.i, 0
  br i1 %37, label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit, label %bb1.i1.i.i.i

bb1.i1.i.i.i:                                     ; preds = %bb2.i.i.i
  %_17.i.i.i = sub nsw i64 0, %ctrl_offset.i.i.i.i
  %ptr.i.i.i = getelementptr inbounds i8, ptr %_3.sroa.0.0.copyload.i.i.i.i.i.ptr, i64 %_17.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i, i64 noundef %_37.0.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !204
  br label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

bb5.i.i:                                          ; preds = %self6.i.noexc, %bb1.i.preheader
  %iter.i.i.sroa.9.1.lcssa = phi i64 [ %iter.i.i.sroa.9.043, %bb1.i.preheader ], [ %32, %self6.i.noexc ]
  %iter.i.i.sroa.5.1.lcssa = phi i64 [ %iter.i.i.sroa.5.044, %bb1.i.preheader ], [ %_43.i, %self6.i.noexc ]
  %iter.i.i.sroa.0.1.lcssa = phi ptr [ %iter.i.i.sroa.0.045, %bb1.i.preheader ], [ %ptr.i, %self6.i.noexc ]
  %38 = add i64 %iter.i.i.sroa.5.1.lcssa, -1
  %39 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %iter.i.i.sroa.5.1.lcssa, i1 true)
  %_228.i = lshr i64 %39, 3
  %_32.i = and i64 %38, %iter.i.i.sroa.5.1.lcssa
  %_5.i = add i64 %_228.i, %iter.i.i.sroa.9.1.lcssa
  %40 = add i64 %iter.i.i.sroa.13.042, -1
  %_18.i = sub nsw i64 0, %_5.i
  %41 = getelementptr inbounds { { i32, i32 }, i64 }, ptr %_57.i.i, i64 %_18.i
  %42 = getelementptr inbounds i8, ptr %41, i64 -16
  %.val.i = load i32, ptr %42, align 4, !range !209, !alias.scope !210, !noalias !213, !noundef !17
  %43 = getelementptr i8, ptr %41, i64 -12
  %.val1.i = load i32, ptr %43, align 4, !range !209, !alias.scope !210, !noalias !213, !noundef !17
  %44 = zext nneg i32 %.val1.i to i64
  %.pre.i.i77.i.i.i = zext nneg i32 %.val.i to i64
  %_7.i.i.i.i.i.i.i = shl nuw nsw i64 %44, 32
  %45 = or disjoint i64 %_7.i.i.i.i.i.i.i, %.pre.i.i77.i.i.i
  %.reass57.reass = xor i64 %45, %invariant.op68
  %_5.i33.i.i.i.i.i.i.i = add i64 %.reass57.reass, %22
  %46 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass57.reass, i64 %.reass57.reass, i64 16)
  %47 = xor i64 %_5.i33.i.i.i.i.i.i.i, %46
  %_16.i34.i.i.i.i.i.i.i.reass = add i64 %.reass57.reass, %invariant.op46
  %_19.i35.i.i.i.i.i.i.i = add i64 %47, %27
  %48 = xor i64 %_16.i34.i.i.i.i.i.i.i.reass, %28
  %49 = tail call noundef i64 @llvm.fshl.i64(i64 %47, i64 %47, i64 21)
  %50 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i34.i.i.i.i.i.i.i.reass, i64 %_16.i34.i.i.i.i.i.i.i.reass, i64 32)
  %51 = xor i64 %_19.i35.i.i.i.i.i.i.i, %45
  %52 = xor i64 %49, %_19.i35.i.i.i.i.i.i.i
  %53 = xor i64 %52, 576460752303423488
  %_2.i.i.i.i.i.i = add i64 %51, %48
  %_5.i.i.i.i.i.i = add i64 %53, %50
  %54 = tail call noundef i64 @llvm.fshl.i64(i64 %48, i64 %48, i64 13)
  %55 = xor i64 %_2.i.i.i.i.i.i, %54
  %56 = tail call noundef i64 @llvm.fshl.i64(i64 %53, i64 %53, i64 16)
  %57 = xor i64 %56, %_5.i.i.i.i.i.i
  %58 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i, i64 32)
  %_16.i.i.i.i.i.i = add i64 %_5.i.i.i.i.i.i, %55
  %_19.i.i.i.i.i.i = add i64 %57, %58
  %59 = tail call noundef i64 @llvm.fshl.i64(i64 %55, i64 %55, i64 17)
  %60 = xor i64 %_16.i.i.i.i.i.i, %59
  %61 = tail call noundef i64 @llvm.fshl.i64(i64 %57, i64 %57, i64 21)
  %62 = xor i64 %61, %_19.i.i.i.i.i.i
  %63 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i, i64 %_16.i.i.i.i.i.i, i64 32)
  %64 = xor i64 %_19.i.i.i.i.i.i, 576460752303423488
  %65 = xor i64 %63, 255
  %_2.i3.i.i.i.i.i = add i64 %64, %60
  %_5.i6.i.i.i.i.i = add i64 %62, %65
  %66 = tail call noundef i64 @llvm.fshl.i64(i64 %60, i64 %60, i64 13)
  %67 = xor i64 %_2.i3.i.i.i.i.i, %66
  %68 = tail call noundef i64 @llvm.fshl.i64(i64 %62, i64 %62, i64 16)
  %69 = xor i64 %68, %_5.i6.i.i.i.i.i
  %70 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i = add i64 %67, %_5.i6.i.i.i.i.i
  %_19.i8.i.i.i.i.i = add i64 %69, %70
  %71 = tail call noundef i64 @llvm.fshl.i64(i64 %67, i64 %67, i64 17)
  %72 = xor i64 %_16.i7.i.i.i.i.i, %71
  %73 = tail call noundef i64 @llvm.fshl.i64(i64 %69, i64 %69, i64 21)
  %74 = xor i64 %73, %_19.i8.i.i.i.i.i
  %75 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i = add i64 %72, %_19.i8.i.i.i.i.i
  %_33.i.i.i.i.i.i = add i64 %74, %75
  %76 = tail call noundef i64 @llvm.fshl.i64(i64 %72, i64 %72, i64 13)
  %77 = xor i64 %76, %_30.i.i.i.i.i.i
  %78 = tail call noundef i64 @llvm.fshl.i64(i64 %74, i64 %74, i64 16)
  %79 = xor i64 %78, %_33.i.i.i.i.i.i
  %80 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i = add i64 %77, %_33.i.i.i.i.i.i
  %_47.i.i.i.i.i.i = add i64 %79, %80
  %81 = tail call noundef i64 @llvm.fshl.i64(i64 %77, i64 %77, i64 17)
  %82 = xor i64 %81, %_44.i.i.i.i.i.i
  %83 = tail call noundef i64 @llvm.fshl.i64(i64 %79, i64 %79, i64 21)
  %84 = xor i64 %83, %_47.i.i.i.i.i.i
  %85 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i = add i64 %82, %_47.i.i.i.i.i.i
  %_61.i.i.i.i.i.i = add i64 %84, %85
  %86 = tail call noundef i64 @llvm.fshl.i64(i64 %82, i64 %82, i64 13)
  %87 = xor i64 %86, %_58.i.i.i.i.i.i
  %88 = tail call noundef i64 @llvm.fshl.i64(i64 %84, i64 %84, i64 16)
  %89 = xor i64 %88, %_61.i.i.i.i.i.i
  %_72.i.i.i.i.i.i = add i64 %87, %_61.i.i.i.i.i.i
  %90 = tail call noundef i64 @llvm.fshl.i64(i64 %87, i64 %87, i64 17)
  %91 = tail call noundef i64 @llvm.fshl.i64(i64 %89, i64 %89, i64 21)
  %92 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i, i64 32)
  %93 = xor i64 %91, %90
  %94 = xor i64 %93, %92
  %_0.i.i.i.i.i = xor i64 %94, %_72.i.i.i.i.i.i
  %probe_seq.sroa.0.01.i = and i64 %_0.i.i.i.i.i, %_26.i.i
  %_182.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %probe_seq.sroa.0.01.i
  %tmp.sroa.0.0.copyload.i.i3.i = load <8 x i8>, ptr %_182.i, align 1, !noalias !216
  %.lobit.i.i.i4.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i3.i, splat (i8 7)
  %95 = bitcast <8 x i8> %.lobit.i.i.i4.i to i64
  %.not.i.not5.i = icmp eq i64 %95, 0
  br i1 %.not.i.not5.i, label %bb6.i, label %bb9.i13, !prof !221

bb6.i:                                            ; preds = %bb5.i.i, %bb6.i
  %probe_seq.sroa.0.06.i = phi i64 [ %probe_seq.sroa.0.0.i, %bb6.i ], [ %probe_seq.sroa.0.01.i, %bb5.i.i ]
  %96 = phi i64 [ %97, %bb6.i ], [ 0, %bb5.i.i ]
  %97 = add i64 %96, 8
  %98 = add i64 %97, %probe_seq.sroa.0.06.i
  %probe_seq.sroa.0.0.i = and i64 %98, %_26.i.i
  %_18.i15 = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %probe_seq.sroa.0.0.i
  %tmp.sroa.0.0.copyload.i.i.i = load <8 x i8>, ptr %_18.i15, align 1, !noalias !216
  %.lobit.i.i.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i, splat (i8 7)
  %99 = bitcast <8 x i8> %.lobit.i.i.i.i to i64
  %.not.i.not.i = icmp eq i64 %99, 0
  br i1 %.not.i.not.i, label %bb6.i, label %bb9.i13, !prof !222

bb9.i13:                                          ; preds = %bb6.i, %bb5.i.i
  %probe_seq.sroa.0.0.lcssa.i = phi i64 [ %probe_seq.sroa.0.01.i, %bb5.i.i ], [ %probe_seq.sroa.0.0.i, %bb6.i ]
  %.lcssa.i = phi i64 [ %95, %bb5.i.i ], [ %99, %bb6.i ]
  %100 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %.lcssa.i, i1 true)
  %_187.i.i = lshr i64 %100, 3
  %_10.i.i = add nuw nsw i64 %_187.i.i, %probe_seq.sroa.0.0.lcssa.i
  %_9.i.i = and i64 %_10.i.i, %_26.i.i
  %_10.i5.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %_9.i.i
  %_14.i.i = load i8, ptr %_10.i5.i, align 1, !noundef !17
  %101 = icmp sgt i8 %_14.i.i, -1
  br i1 %101, label %bb3.i.i14, label %bb18.i.i, !prof !9

bb3.i.i14:                                        ; preds = %bb9.i13
  %tmp.sroa.0.0.copyload.i.i.i.i = load <8 x i8>, ptr %ptr.i.i, align 8, !noalias !223
  %.lobit.i.i.i6.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i, splat (i8 7)
  %102 = bitcast <8 x i8> %.lobit.i.i.i6.i to i64
  %103 = icmp ne i64 %102, 0
  tail call void @llvm.assume(i1 %103)
  %104 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %102, i1 true)
  %_266.i.i = lshr i64 %104, 3
  br label %bb18.i.i

bb18.i.i:                                         ; preds = %bb3.i.i14, %bb9.i13
  %index.sroa.0.0.i.i = phi i64 [ %_266.i.i, %bb3.i.i14 ], [ %_9.i.i, %bb9.i13 ]
  %_72.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i, i64 %index.sroa.0.0.i.i
  %_76.i.i = lshr i64 %_0.i.i.i.i.i, 57
  %_77.i.i = trunc nuw nsw i64 %_76.i.i to i8
  %_81.i.i = add nsw i64 %index.sroa.0.0.i.i, -8
  %_80.i.i = and i64 %_81.i.i, %_26.i.i
  store i8 %_77.i.i, ptr %_72.i.i, align 1
  %gep = getelementptr i8, ptr %invariant.gep, i64 %_80.i.i
  store i8 %_77.i.i, ptr %gep, align 1
  %_93.i.i = shl i64 %_5.i, 4
  %_95.i.i = sub nuw nsw i64 -16, %_93.i.i
  %_23.i.i = getelementptr inbounds i8, ptr %_57.i.i, i64 %_95.i.i
  %_99.i.i = shl i64 %index.sroa.0.0.i.i, 4
  %_101.i.i = sub nuw nsw i64 -16, %_99.i.i
  %dst.i.i = getelementptr inbounds i8, ptr %ptr.i.i, i64 %_101.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %dst.i.i, ptr noundef nonnull align 1 dereferenceable(16) %_23.i.i, i64 16, i1 false)
  %105 = icmp eq i64 %40, 0
  br i1 %105, label %bb4.i.i, label %bb1.i.preheader

bb2.i:                                            ; preds = %bb11.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !228)
  %self.val.i = load ptr, ptr %self, align 8, !alias.scope !228
  %_27.not4.i.i = icmp eq i64 %_26.i, 0
  br i1 %_27.not4.i.i, label %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread11.i, label %bb6.lr.ph.i.i

_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread11.i: ; preds = %bb2.i
  %106 = icmp ne ptr %self.val.i, null
  tail call void @llvm.assume(i1 %106)
  br label %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit

bb6.lr.ph.i.i:                                    ; preds = %bb2.i
  %r1.i.i.i.i = and i64 %_26.i, 7
  %_19.not.i.i.i.i = icmp ne i64 %r1.i.i.i.i, 0
  %107 = zext i1 %_19.not.i.i.i.i to i64
  %yield_count.sroa.0.0.i.i.i.i = add nuw nsw i64 %_255.i, %107
  %108 = icmp ne ptr %self.val.i, null
  tail call void @llvm.assume(i1 %108)
  br label %bb6.i.i

bb7.i.i:                                          ; preds = %bb6.i.i
  %b.i.i = icmp ult i64 %_26.i, 8
  br i1 %b.i.i, label %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.i, label %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread.i, !prof !231

bb6.i.i:                                          ; preds = %bb6.i.i, %bb6.lr.ph.i.i
  %iter.sroa.0.06.i.i = phi i64 [ 0, %bb6.lr.ph.i.i ], [ %_29.i.i, %bb6.i.i ]
  %iter.sroa.3.05.i.i = phi i64 [ %yield_count.sroa.0.0.i.i.i.i, %bb6.lr.ph.i.i ], [ %109, %bb6.i.i ]
  %_29.i.i = add i64 %iter.sroa.0.06.i.i, 8
  %109 = add i64 %iter.sroa.3.05.i.i, -1
  %_34.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %iter.sroa.0.06.i.i
  %tmp.sroa.0.0.copyload.i.i.i.i16 = load <8 x i8>, ptr %_34.i.i, align 1, !noalias !232
  %.lobit.i.i.i.i17 = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i16, splat (i8 7)
  %110 = or <8 x i8> %.lobit.i.i.i.i17, splat (i8 -128)
  store <8 x i8> %110, ptr %_34.i.i, align 1, !noalias !237
  %_27.not.i.i = icmp eq i64 %109, 0
  br i1 %_27.not.i.i, label %bb7.i.i, label %bb6.i.i

_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread.i: ; preds = %bb7.i.i
  %_59.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %_26.i
  %111 = load i64, ptr %self.val.i, align 1, !noalias !228
  store i64 %111, ptr %_59.i.i, align 1, !noalias !228
  br label %bb14.lr.ph.i

_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.i: ; preds = %bb7.i.i
  %_52.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 8
  tail call void @llvm.memmove.p0.p0.i64(ptr nonnull align 1 %_52.i.i, ptr nonnull align 1 %self.val.i, i64 %_26.i, i1 false), !noalias !228
  br label %bb14.lr.ph.i

bb14.lr.ph.i:                                     ; preds = %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.i, %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread.i
  %invariant.gep.i = getelementptr i8, ptr %self.val.i, i64 8
  %hash_builder.val.i.i.i = load i64, ptr %0, align 8
  %112 = getelementptr inbounds nuw i8, ptr %0, i64 8
  %hash_builder.val1.i.i.i = load i64, ptr %112, align 8
  %113 = xor i64 %hash_builder.val.i.i.i, 7816392313619706465
  %114 = xor i64 %hash_builder.val1.i.i.i, 7237128888997146477
  %115 = xor i64 %hash_builder.val.i.i.i, 8317987319222330741
  %_2.i.i.i.i.i.i.i.i.i = add i64 %114, %115
  %116 = tail call i64 @llvm.fshl.i64(i64 %114, i64 %114, i64 13)
  %117 = xor i64 %116, %_2.i.i.i.i.i.i.i.i.i
  %118 = tail call i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i.i.i, i64 32)
  %invariant.op = add i64 %113, %117
  %119 = tail call i64 @llvm.fshl.i64(i64 %117, i64 %117, i64 17)
  %invariant.op67 = xor i64 %hash_builder.val1.i.i.i, 8387220255154660723
  br label %bb14.i

bb14.i:                                           ; preds = %bb11.i18, %bb14.lr.ph.i
  %iter.sroa.0.05.i = phi i64 [ 0, %bb14.lr.ph.i ], [ %_63.i, %bb11.i18 ]
  %_63.i = add nuw i64 %iter.sroa.0.05.i, 1
  %_66.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %iter.sroa.0.05.i
  %_70.i = load i8, ptr %_66.i, align 1, !noalias !228, !noundef !17
  %_69.not.i = icmp eq i8 %_70.i, -128
  br i1 %_69.not.i, label %bb5.i19, label %bb11.i18

bb5.i19:                                          ; preds = %bb14.i
  %_63.neg.i = xor i64 %iter.sroa.0.05.i, -1
  %_72.neg.i = shl i64 %_63.neg.i, 4
  %i_p.i = getelementptr inbounds i8, ptr %self.val.i, i64 %_72.neg.i
  %_18.i.i = sub nsw i64 0, %iter.sroa.0.05.i
  %120 = getelementptr inbounds { { i32, i32 }, i64 }, ptr %self.val.i, i64 %_18.i.i
  %121 = getelementptr inbounds i8, ptr %120, i64 -16
  %122 = getelementptr i8, ptr %120, i64 -12
  %_11.i.i.1.i = getelementptr inbounds nuw i8, ptr %i_p.i, i64 8
  br label %bb7.i

bb7.i:                                            ; preds = %bb3.i.i26.preheader.i, %bb5.i19
  %.val.i.i = load i32, ptr %121, align 4, !range !209, !alias.scope !242, !noalias !245, !noundef !17
  %.val1.i.i = load i32, ptr %122, align 4, !range !209, !alias.scope !242, !noalias !245, !noundef !17
  %123 = zext nneg i32 %.val1.i.i to i64
  %.pre.i.i77.i.i.i.i = zext nneg i32 %.val.i.i to i64
  %_7.i.i.i.i.i.i.i.i = shl nuw nsw i64 %123, 32
  %124 = or disjoint i64 %_7.i.i.i.i.i.i.i.i, %.pre.i.i77.i.i.i.i
  %.reass.reass = xor i64 %124, %invariant.op67
  %_5.i33.i.i.i.i.i.i.i.i = add i64 %.reass.reass, %113
  %125 = tail call noundef i64 @llvm.fshl.i64(i64 %.reass.reass, i64 %.reass.reass, i64 16)
  %126 = xor i64 %_5.i33.i.i.i.i.i.i.i.i, %125
  %_16.i34.i.i.i.i.i.i.i.i.reass = add i64 %.reass.reass, %invariant.op
  %_19.i35.i.i.i.i.i.i.i.i = add i64 %126, %118
  %127 = xor i64 %_16.i34.i.i.i.i.i.i.i.i.reass, %119
  %128 = tail call noundef i64 @llvm.fshl.i64(i64 %126, i64 %126, i64 21)
  %129 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i34.i.i.i.i.i.i.i.i.reass, i64 %_16.i34.i.i.i.i.i.i.i.i.reass, i64 32)
  %130 = xor i64 %_19.i35.i.i.i.i.i.i.i.i, %124
  %131 = xor i64 %_19.i35.i.i.i.i.i.i.i.i, %128
  %132 = xor i64 %131, 576460752303423488
  %_2.i.i.i.i.i.i.i = add i64 %130, %127
  %_5.i.i.i.i.i.i.i = add i64 %132, %129
  %133 = tail call noundef i64 @llvm.fshl.i64(i64 %127, i64 %127, i64 13)
  %134 = xor i64 %_2.i.i.i.i.i.i.i, %133
  %135 = tail call noundef i64 @llvm.fshl.i64(i64 %132, i64 %132, i64 16)
  %136 = xor i64 %135, %_5.i.i.i.i.i.i.i
  %137 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i.i, i64 32)
  %_16.i.i.i.i.i.i.i = add i64 %_5.i.i.i.i.i.i.i, %134
  %_19.i.i.i.i.i.i.i = add i64 %136, %137
  %138 = tail call noundef i64 @llvm.fshl.i64(i64 %134, i64 %134, i64 17)
  %139 = xor i64 %_16.i.i.i.i.i.i.i, %138
  %140 = tail call noundef i64 @llvm.fshl.i64(i64 %136, i64 %136, i64 21)
  %141 = xor i64 %140, %_19.i.i.i.i.i.i.i
  %142 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i.i, i64 %_16.i.i.i.i.i.i.i, i64 32)
  %143 = xor i64 %_19.i.i.i.i.i.i.i, 576460752303423488
  %144 = xor i64 %142, 255
  %_2.i3.i.i.i.i.i.i = add i64 %143, %139
  %_5.i6.i.i.i.i.i.i = add i64 %141, %144
  %145 = tail call noundef i64 @llvm.fshl.i64(i64 %139, i64 %139, i64 13)
  %146 = xor i64 %_2.i3.i.i.i.i.i.i, %145
  %147 = tail call noundef i64 @llvm.fshl.i64(i64 %141, i64 %141, i64 16)
  %148 = xor i64 %147, %_5.i6.i.i.i.i.i.i
  %149 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i.i = add i64 %146, %_5.i6.i.i.i.i.i.i
  %_19.i8.i.i.i.i.i.i = add i64 %148, %149
  %150 = tail call noundef i64 @llvm.fshl.i64(i64 %146, i64 %146, i64 17)
  %151 = xor i64 %_16.i7.i.i.i.i.i.i, %150
  %152 = tail call noundef i64 @llvm.fshl.i64(i64 %148, i64 %148, i64 21)
  %153 = xor i64 %152, %_19.i8.i.i.i.i.i.i
  %154 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i.i = add i64 %151, %_19.i8.i.i.i.i.i.i
  %_33.i.i.i.i.i.i.i = add i64 %153, %154
  %155 = tail call noundef i64 @llvm.fshl.i64(i64 %151, i64 %151, i64 13)
  %156 = xor i64 %155, %_30.i.i.i.i.i.i.i
  %157 = tail call noundef i64 @llvm.fshl.i64(i64 %153, i64 %153, i64 16)
  %158 = xor i64 %157, %_33.i.i.i.i.i.i.i
  %159 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i.i = add i64 %156, %_33.i.i.i.i.i.i.i
  %_47.i.i.i.i.i.i.i = add i64 %158, %159
  %160 = tail call noundef i64 @llvm.fshl.i64(i64 %156, i64 %156, i64 17)
  %161 = xor i64 %160, %_44.i.i.i.i.i.i.i
  %162 = tail call noundef i64 @llvm.fshl.i64(i64 %158, i64 %158, i64 21)
  %163 = xor i64 %162, %_47.i.i.i.i.i.i.i
  %164 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i.i = add i64 %161, %_47.i.i.i.i.i.i.i
  %_61.i.i.i.i.i.i.i = add i64 %163, %164
  %165 = tail call noundef i64 @llvm.fshl.i64(i64 %161, i64 %161, i64 13)
  %166 = xor i64 %165, %_58.i.i.i.i.i.i.i
  %167 = tail call noundef i64 @llvm.fshl.i64(i64 %163, i64 %163, i64 16)
  %168 = xor i64 %167, %_61.i.i.i.i.i.i.i
  %_72.i.i.i.i.i.i.i = add i64 %166, %_61.i.i.i.i.i.i.i
  %169 = tail call noundef i64 @llvm.fshl.i64(i64 %166, i64 %166, i64 17)
  %170 = tail call noundef i64 @llvm.fshl.i64(i64 %168, i64 %168, i64 21)
  %171 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i.i, i64 32)
  %172 = xor i64 %170, %169
  %173 = xor i64 %172, %171
  %_0.i.i.i.i.i.i = xor i64 %173, %_72.i.i.i.i.i.i.i
  %probe_seq.sroa.0.01.i.i = and i64 %_0.i.i.i.i.i.i, %3
  %_182.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %probe_seq.sroa.0.01.i.i
  %tmp.sroa.0.0.copyload.i.i3.i.i = load <8 x i8>, ptr %_182.i.i, align 1, !noalias !251
  %.lobit.i.i.i4.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i3.i.i, splat (i8 7)
  %174 = bitcast <8 x i8> %.lobit.i.i.i4.i.i to i64
  %.not.i.not5.i.i = icmp eq i64 %174, 0
  br i1 %.not.i.not5.i.i, label %bb6.i23.i, label %bb9.i.i20, !prof !221

bb6.i23.i:                                        ; preds = %bb7.i, %bb6.i23.i
  %probe_seq.sroa.0.06.i.i = phi i64 [ %probe_seq.sroa.0.0.i.i, %bb6.i23.i ], [ %probe_seq.sroa.0.01.i.i, %bb7.i ]
  %175 = phi i64 [ %176, %bb6.i23.i ], [ 0, %bb7.i ]
  %176 = add i64 %175, 8
  %177 = add i64 %176, %probe_seq.sroa.0.06.i.i
  %probe_seq.sroa.0.0.i.i = and i64 %177, %3
  %_18.i24.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %probe_seq.sroa.0.0.i.i
  %tmp.sroa.0.0.copyload.i.i.i25.i = load <8 x i8>, ptr %_18.i24.i, align 1, !noalias !251
  %.lobit.i.i.i.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i25.i, splat (i8 7)
  %178 = bitcast <8 x i8> %.lobit.i.i.i.i.i to i64
  %.not.i.not.i.i = icmp eq i64 %178, 0
  br i1 %.not.i.not.i.i, label %bb6.i23.i, label %bb9.i.i20, !prof !222

bb9.i.i20:                                        ; preds = %bb6.i23.i, %bb7.i
  %probe_seq.sroa.0.0.lcssa.i.i = phi i64 [ %probe_seq.sroa.0.01.i.i, %bb7.i ], [ %probe_seq.sroa.0.0.i.i, %bb6.i23.i ]
  %.lcssa.i.i = phi i64 [ %174, %bb7.i ], [ %178, %bb6.i23.i ]
  %179 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %.lcssa.i.i, i1 true)
  %_187.i.i.i = lshr i64 %179, 3
  %_10.i.i.i = add i64 %_187.i.i.i, %probe_seq.sroa.0.0.lcssa.i.i
  %_9.i.i.i21 = and i64 %_10.i.i.i, %3
  %_10.i5.i.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %_9.i.i.i21
  %_14.i.i.i = load i8, ptr %_10.i5.i.i, align 1, !noalias !228, !noundef !17
  %180 = icmp sgt i8 %_14.i.i.i, -1
  br i1 %180, label %bb3.i.i.i, label %bb8.i, !prof !9

bb3.i.i.i:                                        ; preds = %bb9.i.i20
  %tmp.sroa.0.0.copyload.i.i.i.i.i = load <8 x i8>, ptr %self.val.i, align 1, !noalias !256
  %.lobit.i.i.i6.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i, splat (i8 7)
  %181 = bitcast <8 x i8> %.lobit.i.i.i6.i.i to i64
  %182 = icmp ne i64 %181, 0
  tail call void @llvm.assume(i1 %182)
  %183 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %181, i1 true)
  %_266.i.i.i = lshr i64 %183, 3
  br label %bb8.i

bb8.i:                                            ; preds = %bb3.i.i.i, %bb9.i.i20
  %index.sroa.0.0.i.i.i = phi i64 [ %_266.i.i.i, %bb3.i.i.i ], [ %_9.i.i.i21, %bb9.i.i20 ]
  %_83.i = sub i64 %iter.sroa.0.05.i, %probe_seq.sroa.0.01.i.i
  %_85.i = sub i64 %index.sroa.0.0.i.i.i, %probe_seq.sroa.0.01.i.i
  %_8220.i = xor i64 %_85.i, %_83.i
  %b.unshifted.i = and i64 %_8220.i, %3
  %b.i = icmp ult i64 %b.unshifted.i, 8
  br i1 %b.i, label %bb16.i, label %bb17.i, !prof !261

bb17.i:                                           ; preds = %bb8.i
  %_105.i = shl i64 %index.sroa.0.0.i.i.i, 4
  %_107.i = sub nuw nsw i64 -16, %_105.i
  %new_i_p.i = getelementptr inbounds i8, ptr %self.val.i, i64 %_107.i
  %_110.i = getelementptr inbounds nuw i8, ptr %self.val.i, i64 %index.sroa.0.0.i.i.i
  %prev_ctrl.i = load i8, ptr %_110.i, align 1, !noalias !228, !noundef !17
  %_114.i = lshr i64 %_0.i.i.i.i.i.i, 57
  %_115.i = trunc nuw nsw i64 %_114.i to i8
  %_119.i = add i64 %index.sroa.0.0.i.i.i, -8
  %_118.i = and i64 %_119.i, %3
  store i8 %_115.i, ptr %_110.i, align 1, !noalias !228
  %gep.i = getelementptr i8, ptr %invariant.gep.i, i64 %_118.i
  store i8 %_115.i, ptr %gep.i, align 1, !noalias !228
  %184 = icmp eq i8 %prev_ctrl.i, -1
  br i1 %184, label %bb9.i22, label %bb3.i.i26.preheader.i

bb3.i.i26.preheader.i:                            ; preds = %bb17.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !262)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !265)
  %_3.sroa.0.0.copyload.i.i.i.i = load i64, ptr %i_p.i, align 1, !alias.scope !262, !noalias !267
  %_4.sroa.0.0.copyload.i.i.i.i = load i64, ptr %new_i_p.i, align 1, !alias.scope !265, !noalias !268
  store i64 %_4.sroa.0.0.copyload.i.i.i.i, ptr %i_p.i, align 1, !alias.scope !262, !noalias !267
  store i64 %_3.sroa.0.0.copyload.i.i.i.i, ptr %new_i_p.i, align 1, !alias.scope !265, !noalias !268
  %_13.i.i.1.i = getelementptr inbounds nuw i8, ptr %new_i_p.i, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !269)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !271)
  %_3.sroa.0.0.copyload.i.i.i.1.i = load i64, ptr %_11.i.i.1.i, align 1, !alias.scope !269, !noalias !273
  %_4.sroa.0.0.copyload.i.i.i.1.i = load i64, ptr %_13.i.i.1.i, align 1, !alias.scope !271, !noalias !274
  store i64 %_4.sroa.0.0.copyload.i.i.i.1.i, ptr %_11.i.i.1.i, align 1, !alias.scope !269, !noalias !273
  store i64 %_3.sroa.0.0.copyload.i.i.i.1.i, ptr %_13.i.i.1.i, align 1, !alias.scope !271, !noalias !274
  br label %bb7.i

bb16.i:                                           ; preds = %bb8.i
  %_88.i = lshr i64 %_0.i.i.i.i.i.i, 57
  %_89.i = trunc nuw nsw i64 %_88.i to i8
  %_93.i = add i64 %iter.sroa.0.05.i, -8
  %_92.i = and i64 %_93.i, %3
  store i8 %_89.i, ptr %_66.i, align 1, !noalias !228
  %gep9.i = getelementptr i8, ptr %invariant.gep.i, i64 %_92.i
  store i8 %_89.i, ptr %gep9.i, align 1, !noalias !228
  br label %bb11.i18

bb9.i22:                                          ; preds = %bb17.i
  %_132.i = add i64 %iter.sroa.0.05.i, -8
  %_131.i = and i64 %_132.i, %3
  store i8 -1, ptr %_66.i, align 1, !noalias !228
  %gep7.i = getelementptr i8, ptr %invariant.gep.i, i64 %_131.i
  store i8 -1, ptr %gep7.i, align 1, !noalias !228
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(16) %new_i_p.i, ptr noundef nonnull align 1 dereferenceable(16) %i_p.i, i64 16, i1 false), !noalias !228
  br label %bb11.i18

bb11.i18:                                         ; preds = %bb9.i22, %bb16.i, %bb14.i
  %exitcond.not.i = icmp eq i64 %iter.sroa.0.05.i, %3
  br i1 %exitcond.not.i, label %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit, label %bb14.i

_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit: ; preds = %bb11.i18, %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner23prepare_rehash_in_place.exit.thread11.i
  %185 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %186 = sub i64 %full_capacity.sroa.0.0.i, %self1.i
  store i64 %186, ptr %185, align 8, !alias.scope !228
  br label %_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit

_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim.exit: ; preds = %bb1.i1.i.i.i, %bb2.i.i.i, %bb4.i.i, %bb11.i.i, %bb9.i, %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit
  %_0.sroa.4.0.i = phi i64 [ %_11.1.i, %bb9.i ], [ undef, %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit ], [ %self3.i.i.sroa.12.034, %bb11.i.i ], [ undef, %bb4.i.i ], [ undef, %bb2.i.i.i ], [ undef, %bb1.i1.i.i.i ]
  %_0.sroa.0.0.i = phi i64 [ %_11.0.i, %bb9.i ], [ -9223372036854775807, %_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place.exit ], [ %self3.i.i.sroa.7.033, %bb11.i.i ], [ -9223372036854775807, %bb4.i.i ], [ -9223372036854775807, %bb2.i.i.i ], [ -9223372036854775807, %bb1.i1.i.i.i ]
  %187 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i, 0
  %188 = insertvalue { i64, i64 } %187, i64 %_0.sroa.4.0.i, 1
  ret { i64, i64 } %188
}

; core::ptr::drop_in_place::<std::collections::hash::map::HashMap<(char, char), usize>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std11collections4hash3map7HashMapTccEjEECs6KJnav5oeQt_6strsim(ptr %_1.0.val, i64 %_1.8.val) unnamed_addr #2 {
start:
  %0 = icmp eq i64 %_1.8.val, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateEECs6KJnav5oeQt_6strsim.exit, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %start
  %_9.i.i.i.i = shl i64 %_1.8.val, 4
  %1 = add i64 %_9.i.i.i.i, 16
  %rhs5.i.i.i.i.i = add i64 %_1.8.val, 9
  %_37.0.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i, %1
  %_37.1.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i, %1
  %_19.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i, 9223372036854775801
  tail call void @llvm.assume(i1 %_37.1.i.i.i.i.i)
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i)
  %2 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %2)
  %3 = icmp eq i64 %_37.0.i.i.i.i.i, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateEECs6KJnav5oeQt_6strsim.exit, label %bb1.i2.i.i.i.i

bb1.i2.i.i.i.i:                                   ; preds = %bb2.i.i.i.i
  %_17.i.i.i.i = sub nuw nsw i64 -16, %_9.i.i.i.i
  %ptr.i.i.i.i = getelementptr inbounds i8, ptr %_1.0.val, i64 %_17.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateEECs6KJnav5oeQt_6strsim.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateEECs6KJnav5oeQt_6strsim.exit: ; preds = %start, %bb2.i.i.i.i, %bb1.i2.i.i.i.i
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !275)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !278, !alias.scope !275, !noundef !17
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 %v16.i)
  %0 = icmp eq i64 %elem_layout.1, 1
  %v1.sroa.0.0.i = select i1 %0, i64 8, i64 4
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 %v1.sroa.0.0.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !275
  %1 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !275
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1), !noalias !275
  %_35.i = load i64, ptr %self3.i, align 8, !range !279, !noalias !275, !noundef !17
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %3, align 8, !range !45, !noalias !275, !noundef !17
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !275
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !275
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #24
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %3, align 8, !noalias !275, !nonnull !17, !noundef !17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !275
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !275
  %5 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !275
  ret void
}

; <isize as alloc::vec::spec_from_elem::SpecFromElem>::from_elem::<alloc::alloc::Global>
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemiNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, i64 noundef %0, i64 noundef %n) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %_0.i = icmp eq i64 %0, 0
  %_23.i.i = icmp eq i64 %n, 0
  br i1 %_0.i, label %bb2, label %bb3

bb3:                                              ; preds = %start
  br i1 %_23.i.i, label %bb5, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb3
  %_24.i.i.i = add i64 %n, -1
  %_27.0.i.i.i = shl i64 %_24.i.i.i, 3
  %_27.1.i.i.i = icmp ugt i64 %_24.i.i.i, 2305843009213693951
  %_40.i.i.i = icmp ugt i64 %_27.0.i.i.i, 9223372036854775799
  %or.cond = or i1 %_27.1.i.i.i, %_40.i.i.i
  br i1 %or.cond, label %bb3.i, label %bb3.i.i, !prof !46

bb3.i.i:                                          ; preds = %bb6.i.i.i
  %new_size2.i.i.i = add nuw nsw i64 %_27.0.i.i.i, 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !280
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !280
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb3.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i

bb3.i:                                            ; preds = %bb3.i.i, %bb6.i.i.i
  %_4.sroa.4.0.ph.i = phi i64 [ 8, %bb3.i.i ], [ 0, %bb6.i.i.i ]
  %_4.sroa.10.0.ph.i = phi i64 [ %new_size2.i.i.i, %bb3.i.i ], [ undef, %bb6.i.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i, i64 %_4.sroa.10.0.ph.i) #24
  unreachable

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i: ; preds = %bb3.i.i
  %_2416.i.not = icmp eq i64 %n, 1
  br i1 %_2416.i.not, label %bb4.i, label %bb3.i4.preheader

bb3.i4.preheader:                                 ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i
  %min.iters.check = icmp ult i64 %n, 9
  br i1 %min.iters.check, label %bb3.i4.preheader23, label %vector.ph

vector.ph:                                        ; preds = %bb3.i4.preheader
  %n.vec = and i64 %_24.i.i.i, 2305843009213693944
  %3 = shl nuw i64 %n.vec, 3
  %4 = getelementptr i8, ptr %1, i64 %3
  %5 = or disjoint i64 %n.vec, 1
  %broadcast.splatinsert = insertelement <2 x i64> poison, i64 %0, i64 0
  %broadcast.splat = shufflevector <2 x i64> %broadcast.splatinsert, <2 x i64> poison, <2 x i32> zeroinitializer
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %offset.idx = shl i64 %index, 3
  %next.gep = getelementptr i8, ptr %1, i64 %offset.idx
  %6 = getelementptr i8, ptr %next.gep, i64 16
  %7 = getelementptr i8, ptr %next.gep, i64 32
  %8 = getelementptr i8, ptr %next.gep, i64 48
  store <2 x i64> %broadcast.splat, ptr %next.gep, align 8, !noalias !283
  store <2 x i64> %broadcast.splat, ptr %6, align 8, !noalias !283
  store <2 x i64> %broadcast.splat, ptr %7, align 8, !noalias !283
  store <2 x i64> %broadcast.splat, ptr %8, align 8, !noalias !283
  %index.next = add nuw i64 %index, 8
  %9 = icmp eq i64 %index.next, %n.vec
  br i1 %9, label %middle.block, label %vector.body, !llvm.loop !286

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %_24.i.i.i, %n.vec
  br i1 %cmp.n, label %bb4.i, label %bb3.i4.preheader23

bb3.i4.preheader23:                               ; preds = %bb3.i4.preheader, %middle.block
  %ptr.sroa.0.019.i.ph = phi ptr [ %1, %bb3.i4.preheader ], [ %4, %middle.block ]
  %iter.sroa.0.018.i.ph = phi i64 [ 1, %bb3.i4.preheader ], [ %5, %middle.block ]
  br label %bb3.i4

bb4.i:                                            ; preds = %bb3.i4, %middle.block, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i
  %ptr.sroa.0.0.lcssa26.i = phi ptr [ %1, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VeciE7reserveCs6KJnav5oeQt_6strsim.exit.i ], [ %4, %middle.block ], [ %_15.i, %bb3.i4 ]
  store i64 %0, ptr %ptr.sroa.0.0.lcssa26.i, align 8, !noalias !283
  br label %bb5

bb3.i4:                                           ; preds = %bb3.i4.preheader23, %bb3.i4
  %ptr.sroa.0.019.i = phi ptr [ %_15.i, %bb3.i4 ], [ %ptr.sroa.0.019.i.ph, %bb3.i4.preheader23 ]
  %iter.sroa.0.018.i = phi i64 [ %_28.i, %bb3.i4 ], [ %iter.sroa.0.018.i.ph, %bb3.i4.preheader23 ]
  %_28.i = add nuw i64 %iter.sroa.0.018.i, 1
  store i64 %0, ptr %ptr.sroa.0.019.i, align 8, !noalias !283
  %_15.i = getelementptr inbounds nuw i8, ptr %ptr.sroa.0.019.i, i64 8
  %exitcond.not.i = icmp eq i64 %_28.i, %n
  br i1 %exitcond.not.i, label %bb4.i, label %bb3.i4, !llvm.loop !287

bb2:                                              ; preds = %start
  br i1 %_23.i.i, label %bb15, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb2
  %_24.i.i = add i64 %n, -1
  %_27.0.i.i = shl i64 %_24.i.i, 3
  %_27.1.i.i = icmp ugt i64 %_24.i.i, 2305843009213693951
  %_40.i.i = icmp ugt i64 %_27.0.i.i, 9223372036854775799
  %or.cond14 = or i1 %_27.1.i.i, %_40.i.i
  br i1 %or.cond14, label %bb14, label %bb3.i5, !prof !46

bb3.i5:                                           ; preds = %bb6.i.i
  %new_size2.i.i = add nuw nsw i64 %_27.0.i.i, 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !288
; call __rustc::__rust_alloc_zeroed
  %10 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed(i64 noundef range(i64 1, 0) %new_size2.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !288
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb14, label %bb10.i

bb10.i:                                           ; preds = %bb3.i5
  %12 = ptrtoint ptr %10 to i64
  br label %bb15

bb5:                                              ; preds = %bb4.i, %bb3, %bb15
  %.sink = phi ptr [ %15, %bb15 ], [ %1, %bb4.i ], [ inttoptr (i64 8 to ptr), %bb3 ]
  %n.sink = phi i64 [ %n, %bb15 ], [ %n, %bb4.i ], [ 0, %bb3 ]
  store i64 %n, ptr %_0, align 8
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %.sink, ptr %13, align 8
  %14 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %n.sink, ptr %14, align 8
  ret void

bb14:                                             ; preds = %bb6.i.i, %bb3.i5
  %_16.sroa.4.0.ph = phi i64 [ 8, %bb3.i5 ], [ 0, %bb6.i.i ]
  %_16.sroa.10.0.ph = phi i64 [ %new_size2.i.i, %bb3.i5 ], [ undef, %bb6.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_16.sroa.4.0.ph, i64 %_16.sroa.10.0.ph) #24
  unreachable

bb15:                                             ; preds = %bb10.i, %bb2
  %_16.sroa.10.0 = phi i64 [ %12, %bb10.i ], [ 8, %bb2 ]
  %15 = inttoptr i64 %_16.sroa.10.0 to ptr
  br label %bb5
}

; strsim::levenshtein
; Function Attrs: uwtable
define noundef i64 @_RNvCs6KJnav5oeQt_6strsim11levenshtein(ptr noalias noundef nonnull readonly align 1 captures(address) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_8.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  %_6.i.i = icmp ult i64 %b.1, 32
  br i1 %_6.i.i, label %bb2.i.i, label %bb3.i.i

bb3.i.i:                                          ; preds = %start
; call core::str::count::do_count_chars
  %0 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i

bb2.i.i:                                          ; preds = %start
; call core::str::count::char_count_general_case
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i: ; preds = %bb2.i.i, %bb3.i.i
  %_0.sroa.0.0.i.i = phi i64 [ %1, %bb2.i.i ], [ %0, %bb3.i.i ]
  %_6.i = add i64 %_0.sroa.0.0.i.i, 1
  %spec.select.i.i.i = tail call i64 @llvm.usub.sat.i64(i64 %_6.i, i64 1)
  %_23.i.i.i.not.i.i = icmp ugt i64 %_6.i, 1
  br i1 %_23.i.i.i.not.i.i, label %bb6.i.i.i.i.i, label %bb4.i

bb6.i.i.i.i.i:                                    ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i
  %_24.i.i.i.i.i = add i64 %spec.select.i.i.i, -1
  %_27.0.i.i.i.i.i = shl i64 %_24.i.i.i.i.i, 3
  %_27.1.i.i.i.i.i = icmp ugt i64 %_24.i.i.i.i.i, 2305843009213693951
  %_40.i.i.i.i.i = icmp ugt i64 %_27.0.i.i.i.i.i, 9223372036854775799
  %or.cond.i.i = or i1 %_27.1.i.i.i.i.i, %_40.i.i.i.i.i
  br i1 %or.cond.i.i, label %bb3.i.i.i, label %bb3.i.i.i.i, !prof !46

bb3.i.i.i.i:                                      ; preds = %bb6.i.i.i.i.i
  %new_size2.i.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i.i, 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !291
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !291
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb3.i.i.i, label %bb3.i.i.i.i.i.i.preheader

bb3.i.i.i.i.i.i.preheader:                        ; preds = %bb3.i.i.i.i
  %min.iters.check = icmp ult i64 %_0.sroa.0.0.i.i, 8
  br i1 %min.iters.check, label %bb3.i.i.i.i.i.i.preheader14, label %vector.ph

vector.ph:                                        ; preds = %bb3.i.i.i.i.i.i.preheader
  %n.vec = and i64 %_0.sroa.0.0.i.i, -8
  %4 = or disjoint i64 %n.vec, 1
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %vec.ind = phi <2 x i64> [ <i64 1, i64 2>, %vector.ph ], [ %vec.ind.next, %vector.body ]
  %step.add = add <2 x i64> %vec.ind, splat (i64 2)
  %step.add.2 = add <2 x i64> %vec.ind, splat (i64 4)
  %step.add.3 = add <2 x i64> %vec.ind, splat (i64 6)
  %5 = getelementptr inbounds nuw i64, ptr %2, i64 %index
  %6 = getelementptr inbounds nuw i8, ptr %5, i64 16
  %7 = getelementptr inbounds nuw i8, ptr %5, i64 32
  %8 = getelementptr inbounds nuw i8, ptr %5, i64 48
  store <2 x i64> %vec.ind, ptr %5, align 8, !noalias !296
  store <2 x i64> %step.add, ptr %6, align 8, !noalias !296
  store <2 x i64> %step.add.2, ptr %7, align 8, !noalias !296
  store <2 x i64> %step.add.3, ptr %8, align 8, !noalias !296
  %index.next = add nuw i64 %index, 8
  %vec.ind.next = add <2 x i64> %vec.ind, splat (i64 8)
  %9 = icmp eq i64 %index.next, %n.vec
  br i1 %9, label %middle.block, label %vector.body, !llvm.loop !309

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %_0.sroa.0.0.i.i, %n.vec
  br i1 %cmp.n, label %bb4.i, label %bb3.i.i.i.i.i.i.preheader14

bb3.i.i.i.i.i.i.preheader14:                      ; preds = %bb3.i.i.i.i.i.i.preheader, %middle.block
  %.ph = phi i64 [ 0, %bb3.i.i.i.i.i.i.preheader ], [ %n.vec, %middle.block ]
  %self.sroa.0.012.i.i.i.i.i.i.ph = phi i64 [ 1, %bb3.i.i.i.i.i.i.preheader ], [ %4, %middle.block ]
  br label %bb3.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb3.i.i.i.i, %bb6.i.i.i.i.i
  %_4.sroa.4.0.ph.i.i.i = phi i64 [ 8, %bb3.i.i.i.i ], [ 0, %bb6.i.i.i.i.i ]
  %_4.sroa.10.0.ph.i.i.i = phi i64 [ %new_size2.i.i.i.i.i, %bb3.i.i.i.i ], [ undef, %bb6.i.i.i.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i.i, i64 %_4.sroa.10.0.ph.i.i.i) #24, !noalias !310
  unreachable

bb3.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i.i.preheader14, %bb3.i.i.i.i.i.i
  %10 = phi i64 [ %11, %bb3.i.i.i.i.i.i ], [ %.ph, %bb3.i.i.i.i.i.i.preheader14 ]
  %self.sroa.0.012.i.i.i.i.i.i = phi i64 [ %_0.i1.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ], [ %self.sroa.0.012.i.i.i.i.i.i.ph, %bb3.i.i.i.i.i.i.preheader14 ]
  %_0.i1.i.i.i.i.i.i.i.i = add nuw i64 %self.sroa.0.012.i.i.i.i.i.i, 1
  %_3.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %2, i64 %10
  store i64 %self.sroa.0.012.i.i.i.i.i.i, ptr %_3.i.i.i.i.i.i.i.i, align 8, !noalias !296
  %11 = add nuw i64 %10, 1
  %exitcond.not.i.i.i.i.i.i = icmp eq i64 %self.sroa.0.012.i.i.i.i.i.i, %_0.sroa.0.0.i.i
  br i1 %exitcond.not.i.i.i.i.i.i, label %bb4.i, label %bb3.i.i.i.i.i.i, !llvm.loop !311

bb2.i.i.i4.i.i:                                   ; preds = %cleanup5.i
  %alloc_size.i.i.i.i5.i.i = shl nuw i64 %spec.select.i.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i7.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb27.i

bb4.i:                                            ; preds = %bb3.i.i.i.i.i.i, %middle.block, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i
  %_4.sroa.10.0.i7.i.i = phi ptr [ inttoptr (i64 8 to ptr), %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i ], [ %2, %middle.block ], [ %2, %bb3.i.i.i.i.i.i ]
  %f.val4.i.i.i.i.i.i = phi i64 [ 0, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit.i ], [ %_0.sroa.0.0.i.i, %middle.block ], [ %_0.sroa.0.0.i.i, %bb3.i.i.i.i.i.i ]
  %_8.i26.i = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  %_6.i.i.not.i.i40.i = icmp samesign eq i64 %a.1, 0
  br i1 %_6.i.i.not.i.i40.i, label %bb9.i, label %bb14.i.i.i.lr.ph.i

bb14.i.i.i.lr.ph.i:                               ; preds = %bb4.i
  %_6.i.i.not.i.i3534.i = icmp eq i64 %b.1, 0
  br i1 %_6.i.i.not.i.i3534.i, label %bb14.i.i.i.us.i, label %bb14.i.i.i.i

bb14.i.i.i.us.i:                                  ; preds = %bb14.i.i.i.lr.ph.i, %bb11.us.i
  %b_len.sroa.0.043.us.i = phi i64 [ %_8.0.i16.us.i, %bb11.us.i ], [ %_0.sroa.0.0.i.i, %bb14.i.i.i.lr.ph.i ]
  %iter.sroa.10.042.us.i = phi i64 [ %_8.0.i16.us.i, %bb11.us.i ], [ 0, %bb14.i.i.i.lr.ph.i ]
  %iter.sroa.0.041.us.i = phi ptr [ %iter.sroa.0.114.us.i, %bb11.us.i ], [ %a.0, %bb14.i.i.i.lr.ph.i ]
  %_16.i.i.i.i.us.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us.i, i64 1
  %x.i.i.i.us.i = load i8, ptr %iter.sroa.0.041.us.i, align 1, !noalias !312, !noundef !17
  %_6.i.i.i.us.i = icmp sgt i8 %x.i.i.i.us.i, -1
  br i1 %_6.i.i.i.us.i, label %bb11.us.i, label %bb4.i.i.i.us.i

bb4.i.i.i.us.i:                                   ; preds = %bb14.i.i.i.us.i
  %init.i.i.i.us.i = zext i8 %x.i.i.i.us.i to i32
  %_6.i10.i.i.i.us.i = icmp ne ptr %_16.i.i.i.i.us.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.us.i)
  %_16.i12.i.i.i.us.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us.i, i64 2
  %_13.i.i.i.us.i = icmp samesign ugt i8 %x.i.i.i.us.i, -33
  br i1 %_13.i.i.i.us.i, label %bb6.i.i.i.us.i, label %bb11.us.i

bb6.i.i.i.us.i:                                   ; preds = %bb4.i.i.i.us.i
  %y.i.i.i.us.i = load i8, ptr %_16.i.i.i.i.us.i, align 1, !noalias !312, !noundef !17
  %_6.i17.i.i.i.us.i = icmp ne ptr %_16.i12.i.i.i.us.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.us.i)
  %_16.i19.i.i.i.us.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us.i, i64 3
  %_21.i.i.i.us.i = icmp samesign ugt i8 %x.i.i.i.us.i, -17
  br i1 %_21.i.i.i.us.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us.i, label %bb11.us.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us.i: ; preds = %bb6.i.i.i.us.i
  %z.i.i.i.us.i = load i8, ptr %_16.i12.i.i.i.us.i, align 1, !noalias !312, !noundef !17
  %_40.i.i.i.us.i = and i8 %z.i.i.i.us.i, 63
  %_39.i.i.i.us.i = zext nneg i8 %_40.i.i.i.us.i to i32
  %_35.i.i.i.us.i = and i8 %y.i.i.i.us.i, 63
  %_34.i.i.i.us.i = zext nneg i8 %_35.i.i.i.us.i to i32
  %_6.i24.i.i.i.us.i = icmp ne ptr %_16.i19.i.i.i.us.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.us.i)
  %w.i.i.i.us.i = load i8, ptr %_16.i19.i.i.i.us.i, align 1, !noalias !312, !noundef !17
  %_26.i.i.i.us.i = shl nuw nsw i32 %init.i.i.i.us.i, 18
  %_25.i.i.i.us.i = and i32 %_26.i.i.i.us.i, 1835008
  %12 = shl nuw nsw i32 %_34.i.i.i.us.i, 12
  %13 = shl nuw nsw i32 %_39.i.i.i.us.i, 6
  %_43.i.i.i.us.i = or disjoint i32 %13, %12
  %_45.i.i.i.us.i = and i8 %w.i.i.i.us.i, 63
  %_44.i.i.i.us.i = zext nneg i8 %_45.i.i.i.us.i to i32
  %_27.i.i.i.us.i = or disjoint i32 %_43.i.i.i.us.i, %_44.i.i.i.us.i
  %14 = or disjoint i32 %_27.i.i.i.us.i, %_25.i.i.i.us.i
  %.not.i.us.i = icmp eq i32 %14, 1114112
  br i1 %.not.i.us.i, label %bb9.i, label %bb6.us.i

bb6.us.i:                                         ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us.i
  %_16.i26.i.i.i.us.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.us.i, i64 4
  br label %bb11.us.i

bb11.us.i:                                        ; preds = %bb6.us.i, %bb6.i.i.i.us.i, %bb4.i.i.i.us.i, %bb14.i.i.i.us.i
  %iter.sroa.0.114.us.i = phi ptr [ %_16.i26.i.i.i.us.i, %bb6.us.i ], [ %_16.i12.i.i.i.us.i, %bb4.i.i.i.us.i ], [ %_16.i19.i.i.i.us.i, %bb6.i.i.i.us.i ], [ %_16.i.i.i.i.us.i, %bb14.i.i.i.us.i ]
  %_8.0.i16.us.i = add i64 %iter.sroa.10.042.us.i, 1
  %_6.i.i.not.i.i.us.i = icmp eq ptr %iter.sroa.0.114.us.i, %_8.i26.i
  br i1 %_6.i.i.not.i.i.us.i, label %bb9.i, label %bb14.i.i.i.us.i

bb14.i.i.i.i:                                     ; preds = %bb14.i.i.i.lr.ph.i, %bb15.i
  %b_len.sroa.0.043.i = phi i64 [ %b_len.sroa.0.1.lcssa.i, %bb15.i ], [ %_0.sroa.0.0.i.i, %bb14.i.i.i.lr.ph.i ]
  %iter.sroa.10.042.i = phi i64 [ %_8.0.i16.i, %bb15.i ], [ 0, %bb14.i.i.i.lr.ph.i ]
  %iter.sroa.0.041.i = phi ptr [ %iter.sroa.0.114.i, %bb15.i ], [ %a.0, %bb14.i.i.i.lr.ph.i ]
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.i, i64 1
  %x.i.i.i.i = load i8, ptr %iter.sroa.0.041.i, align 1, !noalias !312, !noundef !17
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i28.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.i, i64 2
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !312, !noundef !17
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %15 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i.i, label %bb11.i

bb3.i.i.i28.i:                                    ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb11.i

bb6.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.i, i64 3
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !312, !noundef !17
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %16 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i, label %bb11.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i: ; preds = %bb6.i.i.i.i
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_8.i26.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !312, !noundef !17
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %17 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  %.not.i.i = icmp eq i32 %17, 1114112
  br i1 %.not.i.i, label %bb9.i, label %bb6.i

bb6.i:                                            ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.041.i, i64 4
  br label %bb11.i

bb9.i:                                            ; preds = %bb15.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i, %bb11.us.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us.i, %bb4.i
  %b_len.sroa.0.0.lcssa.i = phi i64 [ %_0.sroa.0.0.i.i, %bb4.i ], [ %b_len.sroa.0.043.us.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us.i ], [ %_8.0.i16.us.i, %bb11.us.i ], [ %b_len.sroa.0.043.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i ], [ %b_len.sroa.0.1.lcssa.i, %bb15.i ]
  %18 = icmp ult i64 %_6.i, 2
  br i1 %18, label %_RINvCs6KJnav5oeQt_6strsim19generic_levenshteinNtB2_13StringWrapperBI_ccEB2_.exit, label %bb2.i.i.i4.i30.i

bb2.i.i.i4.i30.i:                                 ; preds = %bb9.i
  %alloc_size.i.i.i.i5.i31.i = shl nuw i64 %spec.select.i.i.i, 3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_4.sroa.10.0.i7.i.i, i64 noundef %alloc_size.i.i.i.i5.i31.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %_RINvCs6KJnav5oeQt_6strsim19generic_levenshteinNtB2_13StringWrapperBI_ccEB2_.exit

bb11.i:                                           ; preds = %bb6.i, %bb6.i.i.i.i, %bb3.i.i.i28.i, %bb4.i.i.i.i
  %spec.select.i5.i15.i = phi i32 [ %17, %bb6.i ], [ %15, %bb4.i.i.i.i ], [ %16, %bb6.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i28.i ]
  %iter.sroa.0.114.i = phi ptr [ %_16.i26.i.i.i.i, %bb6.i ], [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i.i ], [ %_16.i.i.i.i.i, %bb3.i.i.i28.i ]
  %_8.0.i16.i = add i64 %iter.sroa.10.042.i, 1
  br label %bb14.i.i.i36.i

bb14.i.i.i36.i:                                   ; preds = %bb32.i, %bb11.i
  %b_len.sroa.0.138.i = phi i64 [ %_8.0.i16.i, %bb11.i ], [ %_0.sroa.0.0.i81.i, %bb32.i ]
  %i.sroa.0.037.i = phi i64 [ %iter.sroa.10.042.i, %bb11.i ], [ %24, %bb32.i ]
  %iter1.sroa.10.036.i = phi i64 [ 0, %bb11.i ], [ %_8.0.i5330.i, %bb32.i ]
  %iter1.sroa.0.035.i = phi ptr [ %b.0, %bb11.i ], [ %iter1.sroa.0.128.i, %bb32.i ]
  %_16.i.i.i.i37.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.035.i, i64 1
  %x.i.i.i38.i = load i8, ptr %iter1.sroa.0.035.i, align 1, !noalias !319, !noundef !17
  %_6.i.i.i39.i = icmp sgt i8 %x.i.i.i38.i, -1
  br i1 %_6.i.i.i39.i, label %bb3.i.i.i77.i, label %bb4.i.i.i40.i

bb4.i.i.i40.i:                                    ; preds = %bb14.i.i.i36.i
  %_30.i.i.i41.i = and i8 %x.i.i.i38.i, 31
  %init.i.i.i42.i = zext nneg i8 %_30.i.i.i41.i to i32
  %_6.i10.i.i.i43.i = icmp ne ptr %_16.i.i.i.i37.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i43.i)
  %_16.i12.i.i.i44.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.035.i, i64 2
  %y.i.i.i45.i = load i8, ptr %_16.i.i.i.i37.i, align 1, !noalias !319, !noundef !17
  %_33.i.i.i46.i = shl nuw nsw i32 %init.i.i.i42.i, 6
  %_35.i.i.i47.i = and i8 %y.i.i.i45.i, 63
  %_34.i.i.i48.i = zext nneg i8 %_35.i.i.i47.i to i32
  %19 = or disjoint i32 %_33.i.i.i46.i, %_34.i.i.i48.i
  %_13.i.i.i49.i = icmp samesign ugt i8 %x.i.i.i38.i, -33
  br i1 %_13.i.i.i49.i, label %bb6.i.i.i56.i, label %bb16.i

bb3.i.i.i77.i:                                    ; preds = %bb14.i.i.i36.i
  %_7.i.i.i78.i = zext nneg i8 %x.i.i.i38.i to i32
  br label %bb16.i

bb6.i.i.i56.i:                                    ; preds = %bb4.i.i.i40.i
  %_6.i17.i.i.i57.i = icmp ne ptr %_16.i12.i.i.i44.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i57.i)
  %_16.i19.i.i.i58.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.035.i, i64 3
  %z.i.i.i59.i = load i8, ptr %_16.i12.i.i.i44.i, align 1, !noalias !319, !noundef !17
  %_38.i.i.i60.i = shl nuw nsw i32 %_34.i.i.i48.i, 6
  %_40.i.i.i61.i = and i8 %z.i.i.i59.i, 63
  %_39.i.i.i62.i = zext nneg i8 %_40.i.i.i61.i to i32
  %y_z.i.i.i63.i = or disjoint i32 %_38.i.i.i60.i, %_39.i.i.i62.i
  %_20.i.i.i64.i = shl nuw nsw i32 %init.i.i.i42.i, 12
  %20 = or disjoint i32 %y_z.i.i.i63.i, %_20.i.i.i64.i
  %_21.i.i.i65.i = icmp samesign ugt i8 %x.i.i.i38.i, -17
  br i1 %_21.i.i.i65.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i66.i, label %bb16.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i66.i: ; preds = %bb6.i.i.i56.i
  %_6.i24.i.i.i67.i = icmp ne ptr %_16.i19.i.i.i58.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i67.i)
  %w.i.i.i69.i = load i8, ptr %_16.i19.i.i.i58.i, align 1, !noalias !319, !noundef !17
  %_26.i.i.i70.i = shl nuw nsw i32 %init.i.i.i42.i, 18
  %_25.i.i.i71.i = and i32 %_26.i.i.i70.i, 1835008
  %_43.i.i.i72.i = shl nuw nsw i32 %y_z.i.i.i63.i, 6
  %_45.i.i.i73.i = and i8 %w.i.i.i69.i, 63
  %_44.i.i.i74.i = zext nneg i8 %_45.i.i.i73.i to i32
  %_27.i.i.i75.i = or disjoint i32 %_43.i.i.i72.i, %_44.i.i.i74.i
  %21 = or disjoint i32 %_27.i.i.i75.i, %_25.i.i.i71.i
  %.not.i76.i = icmp eq i32 %21, 1114112
  br i1 %.not.i76.i, label %bb15.i, label %bb13.i

bb13.i:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i66.i
  %_16.i26.i.i.i68.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.035.i, i64 4
  br label %bb16.i

bb15.i:                                           ; preds = %bb32.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i66.i
  %b_len.sroa.0.1.lcssa.i = phi i64 [ %_0.sroa.0.0.i81.i, %bb32.i ], [ %b_len.sroa.0.138.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i66.i ]
  %_6.i.i.not.i.i.i = icmp eq ptr %iter.sroa.0.114.i, %_8.i26.i
  br i1 %_6.i.i.not.i.i.i, label %bb9.i, label %bb14.i.i.i.i

cleanup5.i:                                       ; preds = %panic.i
  %22 = landingpad { ptr, i32 }
          cleanup
  %23 = icmp ult i64 %_6.i, 2
  br i1 %23, label %bb27.i, label %bb2.i.i.i4.i.i

bb16.i:                                           ; preds = %bb13.i, %bb6.i.i.i56.i, %bb3.i.i.i77.i, %bb4.i.i.i40.i
  %spec.select.i5.i5129.i = phi i32 [ %21, %bb13.i ], [ %19, %bb4.i.i.i40.i ], [ %20, %bb6.i.i.i56.i ], [ %_7.i.i.i78.i, %bb3.i.i.i77.i ]
  %iter1.sroa.0.128.i = phi ptr [ %_16.i26.i.i.i68.i, %bb13.i ], [ %_16.i12.i.i.i44.i, %bb4.i.i.i40.i ], [ %_16.i19.i.i.i58.i, %bb6.i.i.i56.i ], [ %_16.i.i.i.i37.i, %bb3.i.i.i77.i ]
  %exitcond.not.i = icmp eq i64 %iter1.sroa.10.036.i, %f.val4.i.i.i.i.i.i
  br i1 %exitcond.not.i, label %panic.i, label %bb32.i

panic.i:                                          ; preds = %bb16.i
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %f.val4.i.i.i.i.i.i, i64 noundef %f.val4.i.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33490829e982a20313b24b098088ff01) #24
          to label %unreachable.i unwind label %cleanup5.i

unreachable.i:                                    ; preds = %panic.i
  unreachable

bb32.i:                                           ; preds = %bb16.i
  %_0.i.i = icmp ne i32 %spec.select.i5.i15.i, %spec.select.i5.i5129.i
  %_8.0.i5330.i = add nuw i64 %iter1.sroa.10.036.i, 1
  %cost.i = zext i1 %_0.i.i to i64
  %distance_a.i = add i64 %i.sroa.0.037.i, %cost.i
  %_28.i = getelementptr inbounds nuw i64, ptr %_4.sroa.10.0.i7.i.i, i64 %iter1.sroa.10.036.i
  %24 = load i64, ptr %_28.i, align 8, !noundef !17
  %_31.i = add i64 %24, 1
  %_0.sroa.0.0.i80.i = tail call noundef i64 @llvm.umin.i64(i64 %_31.i, i64 %distance_a.i)
  %_29.i = add i64 %b_len.sroa.0.138.i, 1
  %_0.sroa.0.0.i81.i = tail call noundef i64 @llvm.umin.i64(i64 %_0.sroa.0.0.i80.i, i64 %_29.i)
  store i64 %_0.sroa.0.0.i81.i, ptr %_28.i, align 8
  %_6.i.i.not.i.i35.i = icmp eq ptr %iter1.sroa.0.128.i, %_8.i.i
  br i1 %_6.i.i.not.i.i35.i, label %bb15.i, label %bb14.i.i.i36.i

bb27.i:                                           ; preds = %cleanup5.i, %bb2.i.i.i4.i.i
  resume { ptr, i32 } %22

_RINvCs6KJnav5oeQt_6strsim19generic_levenshteinNtB2_13StringWrapperBI_ccEB2_.exit: ; preds = %bb9.i, %bb2.i.i.i4.i30.i
  ret i64 %b_len.sroa.0.0.lcssa.i
}

; strsim::jaro_winkler
; Function Attrs: uwtable
define noundef double @_RNvCs6KJnav5oeQt_6strsim12jaro_winkler(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
; call strsim::generic_jaro::<strsim::StringWrapper, strsim::StringWrapper, char, char>
  %sim.i = tail call fastcc noundef double @_RINvCs6KJnav5oeQt_6strsim12generic_jaroNtB2_13StringWrapperBB_ccEB2_(ptr nonnull %a.0, i64 %a.1, ptr nonnull %b.0, i64 %b.1)
  %_4.i = fcmp ogt double %sim.i, 0x3FE6666666666666
  br i1 %_4.i, label %bb1.i.i.i.lr.ph.i.i.i.i, label %_RINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2_13StringWrapperBJ_ccEB2_.exit

bb1.i.i.i.lr.ph.i.i.i.i:                          ; preds = %start
  %_8.i.i = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  %_8.i.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  %_6.i.i.not.i.i.i.i.i.i.i.i = icmp samesign eq i64 %a.1, 0
  br i1 %_6.i.i.not.i.i.i.i.i.i.i.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb1.i.i.i.lr.ph.i.i.i.i
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %a.0, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %a.0, align 1, !noalias !326, !noundef !17
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp samesign ne i64 %a.1, 1
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %a.0, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !326, !noundef !17
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %0 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  br label %bb9.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp samesign ne i64 %a.1, 2
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %a.0, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !noalias !326, !noundef !17
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %1 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i, label %bb9.i.i.i.i.i.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp samesign ne i64 %a.1, 3
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %a.0, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !noalias !326, !noundef !17
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %2 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  %.not.i.i.i.i.i.i = icmp eq i32 %2, 1114112
  br i1 %.not.i.i.i.i.i.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb9.i.i.i.i.i.i

bb9.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %_16.i26.i.i.i.i.i16.i.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.0.0.i10.i.i.i.i.i.i = phi i32 [ %2, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i ], [ %0, %bb4.i.i.i.i.i.i.i.i.i ], [ %1, %bb6.i.i.i.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i ]
  %_6.i.i.not.i.i.i.i.i.i.i = icmp samesign eq i64 %b.1, 0
  br i1 %_6.i.i.not.i.i.i.i.i.i.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i:                             ; preds = %bb9.i.i.i.i.i.i
  %_16.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 1
  %x.i.i.i.i.i.i.i.i = load i8, ptr %b.0, align 1, !noalias !344, !noundef !17
  %_6.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i = icmp samesign ne i64 %b.1, 1
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 2
  %y.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i, align 1, !noalias !344, !noundef !17
  %_33.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i to i32
  %3 = or disjoint i32 %_33.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i to i32
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i

bb6.i.i.i.i.i.i.i.i:                              ; preds = %bb4.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i = icmp samesign ne i64 %b.1, 2
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 3
  %z.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i, align 1, !noalias !344, !noundef !17
  %_38.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 12
  %4 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i = icmp samesign ne i64 %b.1, 3
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %b.0, i64 4
  %w.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i, align 1, !noalias !344, !noundef !17
  %_26.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i
  %5 = or disjoint i32 %_27.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i
  %.fr.i.i.i.i.i.i = freeze i32 %5
  %.not6.i.i.i.i.i.i = icmp eq i32 %.fr.i.i.i.i.i.i, 1114112
  br i1 %.not6.i.i.i.i.i.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i: ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i
  %_16.i26.i.i.i.i31.i.i.i.i = phi ptr [ %_16.i12.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.i.i.i.i.i = phi i32 [ %3, %bb4.i.i.i.i.i.i.i.i ], [ %4, %bb6.i.i.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i ], [ %.fr.i.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i ]
  %.not.i.i.i.i = icmp ne i32 %_0.sroa.0.0.i10.i.i.i.i.i.i, 1114112
  %_0.i.i.i.i.i.i.i = icmp eq i32 %_0.sroa.0.0.i10.i.i.i.i.i.i, %_0.sroa.4.0.i.i.i.i.i.i
  %or.cond.i.i = select i1 %.not.i.i.i.i, i1 %_0.i.i.i.i.i.i.i, i1 false
  br i1 %or.cond.i.i, label %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i

_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.i: ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i
  %_6.i.i.not.i.i.i.i.i.i.i.1.i = icmp eq ptr %_16.i26.i.i.i.i.i16.i.i.i.i, %_8.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.i.1.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.i.1.i

bb14.i.i.i.i.i.i.i.i.1.i:                         ; preds = %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.i
  %_16.i.i.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.i, align 1, !noalias !326, !noundef !17
  %_6.i.i.i.i.i.i.i.i.1.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.1.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.1.i, label %bb3.i.i.i.i.i.i.i.i.1.i, label %bb4.i.i.i.i.i.i.i.i.1.i

bb4.i.i.i.i.i.i.i.i.1.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.1.i
  %_30.i.i.i.i.i.i.i.i.1.i = and i8 %x.i.i.i.i.i.i.i.i.1.i, 31
  %init.i.i.i.i.i.i.i.i.1.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.1.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.1.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.1.i)
  %_16.i12.i.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.1.i, align 1, !noalias !326, !noundef !17
  %_33.i.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.1.i, 6
  %_35.i.i.i.i.i.i.i.i.1.i = and i8 %y.i.i.i.i.i.i.i.i.1.i, 63
  %_34.i.i.i.i.i.i.i.i.1.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.1.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.1.i, %_34.i.i.i.i.i.i.i.i.1.i
  %_13.i.i.i.i.i.i.i.i.1.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.1.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.1.i, label %bb6.i.i.i.i.i.i.i.i.1.i, label %bb9.i.i.i.i.i.1.i

bb6.i.i.i.i.i.i.i.i.1.i:                          ; preds = %bb4.i.i.i.i.i.i.i.i.1.i
  %_6.i17.i.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.1.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.1.i)
  %_16.i19.i.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.1.i, align 1, !noalias !326, !noundef !17
  %_38.i.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.1.i, 6
  %_40.i.i.i.i.i.i.i.i.1.i = and i8 %z.i.i.i.i.i.i.i.i.1.i, 63
  %_39.i.i.i.i.i.i.i.i.1.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.1.i to i32
  %y_z.i.i.i.i.i.i.i.i.1.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.1.i, %_39.i.i.i.i.i.i.i.i.1.i
  %_20.i.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.1.i, 12
  %7 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.1.i, %_20.i.i.i.i.i.i.i.i.1.i
  %_21.i.i.i.i.i.i.i.i.1.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.1.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.1.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i, label %bb9.i.i.i.i.i.1.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i: ; preds = %bb6.i.i.i.i.i.i.i.i.1.i
  %_6.i24.i.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.1.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.1.i)
  %_16.i26.i.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.1.i, align 1, !noalias !326, !noundef !17
  %_26.i.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.1.i, 18
  %_25.i.i.i.i.i.i.i.i.1.i = and i32 %_26.i.i.i.i.i.i.i.i.1.i, 1835008
  %_43.i.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.1.i, 6
  %_45.i.i.i.i.i.i.i.i.1.i = and i8 %w.i.i.i.i.i.i.i.i.1.i, 63
  %_44.i.i.i.i.i.i.i.i.1.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.1.i to i32
  %_27.i.i.i.i.i.i.i.i.1.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.1.i, %_44.i.i.i.i.i.i.i.i.1.i
  %8 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.1.i, %_25.i.i.i.i.i.i.i.i.1.i
  %.not.i.i.i.i.i.1.i = icmp eq i32 %8, 1114112
  br i1 %.not.i.i.i.i.i.1.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb9.i.i.i.i.i.1.i

bb3.i.i.i.i.i.i.i.i.1.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.1.i
  %_7.i.i.i.i.i.i.i.i.1.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.1.i to i32
  br label %bb9.i.i.i.i.i.1.i

bb9.i.i.i.i.i.1.i:                                ; preds = %bb3.i.i.i.i.i.i.i.i.1.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i, %bb6.i.i.i.i.i.i.i.i.1.i, %bb4.i.i.i.i.i.i.i.i.1.i
  %_16.i26.i.i.i.i.i16.i.i.i.1.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.1.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i ], [ %_16.i12.i.i.i.i.i.i.i.i.1.i, %bb4.i.i.i.i.i.i.i.i.1.i ], [ %_16.i19.i.i.i.i.i.i.i.i.1.i, %bb6.i.i.i.i.i.i.i.i.1.i ], [ %_16.i.i.i.i.i.i.i.i.i.1.i, %bb3.i.i.i.i.i.i.i.i.1.i ]
  %_0.sroa.0.0.i10.i.i.i.i.i.1.i = phi i32 [ %8, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.1.i ], [ %7, %bb6.i.i.i.i.i.i.i.i.1.i ], [ %_7.i.i.i.i.i.i.i.i.1.i, %bb3.i.i.i.i.i.i.i.i.1.i ]
  %_6.i.i.not.i.i.i.i.i.i.1.i = icmp eq ptr %_16.i26.i.i.i.i31.i.i.i.i, %_8.i.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.1.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.1.i

bb14.i.i.i.i.i.i.i.1.i:                           ; preds = %bb9.i.i.i.i.i.1.i
  %_16.i.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i26.i.i.i.i31.i.i.i.i, align 1, !noalias !344, !noundef !17
  %_6.i.i.i.i.i.i.i.1.i = icmp sgt i8 %x.i.i.i.i.i.i.i.1.i, -1
  br i1 %_6.i.i.i.i.i.i.i.1.i, label %bb3.i.i.i.i.i.i.i.1.i, label %bb4.i.i.i.i.i.i.i.1.i

bb4.i.i.i.i.i.i.i.1.i:                            ; preds = %bb14.i.i.i.i.i.i.i.1.i
  %_30.i.i.i.i.i.i.i.1.i = and i8 %x.i.i.i.i.i.i.i.1.i, 31
  %init.i.i.i.i.i.i.i.1.i = zext nneg i8 %_30.i.i.i.i.i.i.i.1.i to i32
  %_6.i10.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.1.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.1.i)
  %_16.i12.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.1.i, align 1, !noalias !344, !noundef !17
  %_33.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.1.i, 6
  %_35.i.i.i.i.i.i.i.1.i = and i8 %y.i.i.i.i.i.i.i.1.i, 63
  %_34.i.i.i.i.i.i.i.1.i = zext nneg i8 %_35.i.i.i.i.i.i.i.1.i to i32
  %9 = or disjoint i32 %_33.i.i.i.i.i.i.i.1.i, %_34.i.i.i.i.i.i.i.1.i
  %_13.i.i.i.i.i.i.i.1.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.1.i, -33
  br i1 %_13.i.i.i.i.i.i.i.1.i, label %bb6.i.i.i.i.i.i.i.1.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i

bb6.i.i.i.i.i.i.i.1.i:                            ; preds = %bb4.i.i.i.i.i.i.i.1.i
  %_6.i17.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.1.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.1.i)
  %_16.i19.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.1.i, align 1, !noalias !344, !noundef !17
  %_38.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.1.i, 6
  %_40.i.i.i.i.i.i.i.1.i = and i8 %z.i.i.i.i.i.i.i.1.i, 63
  %_39.i.i.i.i.i.i.i.1.i = zext nneg i8 %_40.i.i.i.i.i.i.i.1.i to i32
  %y_z.i.i.i.i.i.i.i.1.i = or disjoint i32 %_38.i.i.i.i.i.i.i.1.i, %_39.i.i.i.i.i.i.i.1.i
  %_20.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.1.i, 12
  %10 = or disjoint i32 %y_z.i.i.i.i.i.i.i.1.i, %_20.i.i.i.i.i.i.i.1.i
  %_21.i.i.i.i.i.i.i.1.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.1.i, -17
  br i1 %_21.i.i.i.i.i.i.i.1.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i: ; preds = %bb6.i.i.i.i.i.i.i.1.i
  %_6.i24.i.i.i.i.i.i.i.1.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.1.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.1.i)
  %_16.i26.i.i.i.i.i.i.i.1.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.1.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.1.i, align 1, !noalias !344, !noundef !17
  %_26.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.1.i, 18
  %_25.i.i.i.i.i.i.i.1.i = and i32 %_26.i.i.i.i.i.i.i.1.i, 1835008
  %_43.i.i.i.i.i.i.i.1.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.1.i, 6
  %_45.i.i.i.i.i.i.i.1.i = and i8 %w.i.i.i.i.i.i.i.1.i, 63
  %_44.i.i.i.i.i.i.i.1.i = zext nneg i8 %_45.i.i.i.i.i.i.i.1.i to i32
  %_27.i.i.i.i.i.i.i.1.i = or disjoint i32 %_43.i.i.i.i.i.i.i.1.i, %_44.i.i.i.i.i.i.i.1.i
  %11 = or disjoint i32 %_27.i.i.i.i.i.i.i.1.i, %_25.i.i.i.i.i.i.i.1.i
  %.fr.i.i.i.i.i.1.i = freeze i32 %11
  %.not6.i.i.i.i.i.1.i = icmp eq i32 %.fr.i.i.i.i.i.1.i, 1114112
  br i1 %.not6.i.i.i.i.i.1.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i

bb3.i.i.i.i.i.i.i.1.i:                            ; preds = %bb14.i.i.i.i.i.i.i.1.i
  %_7.i.i.i.i.i.i.i.1.i = zext nneg i8 %x.i.i.i.i.i.i.i.1.i to i32
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i: ; preds = %bb3.i.i.i.i.i.i.i.1.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i, %bb6.i.i.i.i.i.i.i.1.i, %bb4.i.i.i.i.i.i.i.1.i
  %_16.i26.i.i.i.i31.i.i.i.1.i = phi ptr [ %_16.i12.i.i.i.i.i.i.i.1.i, %bb4.i.i.i.i.i.i.i.1.i ], [ %_16.i19.i.i.i.i.i.i.i.1.i, %bb6.i.i.i.i.i.i.i.1.i ], [ %_16.i.i.i.i.i.i.i.i.1.i, %bb3.i.i.i.i.i.i.i.1.i ], [ %_16.i26.i.i.i.i.i.i.i.1.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i ]
  %_0.sroa.4.0.i.i.i.i.i.1.i = phi i32 [ %9, %bb4.i.i.i.i.i.i.i.1.i ], [ %10, %bb6.i.i.i.i.i.i.i.1.i ], [ %_7.i.i.i.i.i.i.i.1.i, %bb3.i.i.i.i.i.i.i.1.i ], [ %.fr.i.i.i.i.i.1.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i ]
  %.not.i.i.i.1.i = icmp ne i32 %_0.sroa.0.0.i10.i.i.i.i.i.1.i, 1114112
  %_0.i.i.i.i.i.i.1.i = icmp eq i32 %_0.sroa.0.0.i10.i.i.i.i.i.1.i, %_0.sroa.4.0.i.i.i.i.i.1.i
  %or.cond.i.1.i = select i1 %.not.i.i.i.1.i, i1 %_0.i.i.i.i.i.i.1.i, i1 false
  br i1 %or.cond.i.1.i, label %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.1.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i

_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.1.i: ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i
  %_6.i.i.not.i.i.i.i.i.i.i.2.i = icmp eq ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, %_8.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.i.2.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.i.2.i

bb14.i.i.i.i.i.i.i.i.2.i:                         ; preds = %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.1.i
  %_16.i.i.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, i64 1
  %x.i.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, align 1, !noalias !326, !noundef !17
  %_6.i.i.i.i.i.i.i.i.2.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.2.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.2.i, label %bb3.i.i.i.i.i.i.i.i.2.i, label %bb4.i.i.i.i.i.i.i.i.2.i

bb4.i.i.i.i.i.i.i.i.2.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.2.i
  %_30.i.i.i.i.i.i.i.i.2.i = and i8 %x.i.i.i.i.i.i.i.i.2.i, 31
  %init.i.i.i.i.i.i.i.i.2.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.2.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.2.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.2.i)
  %_16.i12.i.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, i64 2
  %y.i.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.2.i, align 1, !noalias !326, !noundef !17
  %_33.i.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.2.i, 6
  %_35.i.i.i.i.i.i.i.i.2.i = and i8 %y.i.i.i.i.i.i.i.i.2.i, 63
  %_34.i.i.i.i.i.i.i.i.2.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.2.i to i32
  %12 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.2.i, %_34.i.i.i.i.i.i.i.i.2.i
  %_13.i.i.i.i.i.i.i.i.2.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.2.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.2.i, label %bb6.i.i.i.i.i.i.i.i.2.i, label %bb9.i.i.i.i.i.2.i

bb6.i.i.i.i.i.i.i.i.2.i:                          ; preds = %bb4.i.i.i.i.i.i.i.i.2.i
  %_6.i17.i.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.2.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.2.i)
  %_16.i19.i.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, i64 3
  %z.i.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.2.i, align 1, !noalias !326, !noundef !17
  %_38.i.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.2.i, 6
  %_40.i.i.i.i.i.i.i.i.2.i = and i8 %z.i.i.i.i.i.i.i.i.2.i, 63
  %_39.i.i.i.i.i.i.i.i.2.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.2.i to i32
  %y_z.i.i.i.i.i.i.i.i.2.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.2.i, %_39.i.i.i.i.i.i.i.i.2.i
  %_20.i.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.2.i, 12
  %13 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.2.i, %_20.i.i.i.i.i.i.i.i.2.i
  %_21.i.i.i.i.i.i.i.i.2.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.2.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.2.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i, label %bb9.i.i.i.i.i.2.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i: ; preds = %bb6.i.i.i.i.i.i.i.i.2.i
  %_6.i24.i.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.2.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.2.i)
  %_16.i26.i.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.1.i, i64 4
  %w.i.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.2.i, align 1, !noalias !326, !noundef !17
  %_26.i.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.2.i, 18
  %_25.i.i.i.i.i.i.i.i.2.i = and i32 %_26.i.i.i.i.i.i.i.i.2.i, 1835008
  %_43.i.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.2.i, 6
  %_45.i.i.i.i.i.i.i.i.2.i = and i8 %w.i.i.i.i.i.i.i.i.2.i, 63
  %_44.i.i.i.i.i.i.i.i.2.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.2.i to i32
  %_27.i.i.i.i.i.i.i.i.2.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.2.i, %_44.i.i.i.i.i.i.i.i.2.i
  %14 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.2.i, %_25.i.i.i.i.i.i.i.i.2.i
  %.not.i.i.i.i.i.2.i = icmp eq i32 %14, 1114112
  br i1 %.not.i.i.i.i.i.2.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb9.i.i.i.i.i.2.i

bb3.i.i.i.i.i.i.i.i.2.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.2.i
  %_7.i.i.i.i.i.i.i.i.2.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.2.i to i32
  br label %bb9.i.i.i.i.i.2.i

bb9.i.i.i.i.i.2.i:                                ; preds = %bb3.i.i.i.i.i.i.i.i.2.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i, %bb6.i.i.i.i.i.i.i.i.2.i, %bb4.i.i.i.i.i.i.i.i.2.i
  %_16.i26.i.i.i.i.i16.i.i.i.2.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.2.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i ], [ %_16.i12.i.i.i.i.i.i.i.i.2.i, %bb4.i.i.i.i.i.i.i.i.2.i ], [ %_16.i19.i.i.i.i.i.i.i.i.2.i, %bb6.i.i.i.i.i.i.i.i.2.i ], [ %_16.i.i.i.i.i.i.i.i.i.2.i, %bb3.i.i.i.i.i.i.i.i.2.i ]
  %_0.sroa.0.0.i10.i.i.i.i.i.2.i = phi i32 [ %14, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.2.i ], [ %13, %bb6.i.i.i.i.i.i.i.i.2.i ], [ %_7.i.i.i.i.i.i.i.i.2.i, %bb3.i.i.i.i.i.i.i.i.2.i ]
  %_6.i.i.not.i.i.i.i.i.i.2.i = icmp eq ptr %_16.i26.i.i.i.i31.i.i.i.1.i, %_8.i.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.2.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.2.i

bb14.i.i.i.i.i.i.i.2.i:                           ; preds = %bb9.i.i.i.i.i.2.i
  %_16.i.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.1.i, i64 1
  %x.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i26.i.i.i.i31.i.i.i.1.i, align 1, !noalias !344, !noundef !17
  %_6.i.i.i.i.i.i.i.2.i = icmp sgt i8 %x.i.i.i.i.i.i.i.2.i, -1
  br i1 %_6.i.i.i.i.i.i.i.2.i, label %bb3.i.i.i.i.i.i.i.2.i, label %bb4.i.i.i.i.i.i.i.2.i

bb4.i.i.i.i.i.i.i.2.i:                            ; preds = %bb14.i.i.i.i.i.i.i.2.i
  %_30.i.i.i.i.i.i.i.2.i = and i8 %x.i.i.i.i.i.i.i.2.i, 31
  %init.i.i.i.i.i.i.i.2.i = zext nneg i8 %_30.i.i.i.i.i.i.i.2.i to i32
  %_6.i10.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.2.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.2.i)
  %_16.i12.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.1.i, i64 2
  %y.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.2.i, align 1, !noalias !344, !noundef !17
  %_33.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.2.i, 6
  %_35.i.i.i.i.i.i.i.2.i = and i8 %y.i.i.i.i.i.i.i.2.i, 63
  %_34.i.i.i.i.i.i.i.2.i = zext nneg i8 %_35.i.i.i.i.i.i.i.2.i to i32
  %15 = or disjoint i32 %_33.i.i.i.i.i.i.i.2.i, %_34.i.i.i.i.i.i.i.2.i
  %_13.i.i.i.i.i.i.i.2.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.2.i, -33
  br i1 %_13.i.i.i.i.i.i.i.2.i, label %bb6.i.i.i.i.i.i.i.2.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i

bb6.i.i.i.i.i.i.i.2.i:                            ; preds = %bb4.i.i.i.i.i.i.i.2.i
  %_6.i17.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.2.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.2.i)
  %_16.i19.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.1.i, i64 3
  %z.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.2.i, align 1, !noalias !344, !noundef !17
  %_38.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.2.i, 6
  %_40.i.i.i.i.i.i.i.2.i = and i8 %z.i.i.i.i.i.i.i.2.i, 63
  %_39.i.i.i.i.i.i.i.2.i = zext nneg i8 %_40.i.i.i.i.i.i.i.2.i to i32
  %y_z.i.i.i.i.i.i.i.2.i = or disjoint i32 %_38.i.i.i.i.i.i.i.2.i, %_39.i.i.i.i.i.i.i.2.i
  %_20.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.2.i, 12
  %16 = or disjoint i32 %y_z.i.i.i.i.i.i.i.2.i, %_20.i.i.i.i.i.i.i.2.i
  %_21.i.i.i.i.i.i.i.2.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.2.i, -17
  br i1 %_21.i.i.i.i.i.i.i.2.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i: ; preds = %bb6.i.i.i.i.i.i.i.2.i
  %_6.i24.i.i.i.i.i.i.i.2.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.2.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.2.i)
  %_16.i26.i.i.i.i.i.i.i.2.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.1.i, i64 4
  %w.i.i.i.i.i.i.i.2.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.2.i, align 1, !noalias !344, !noundef !17
  %_26.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.2.i, 18
  %_25.i.i.i.i.i.i.i.2.i = and i32 %_26.i.i.i.i.i.i.i.2.i, 1835008
  %_43.i.i.i.i.i.i.i.2.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.2.i, 6
  %_45.i.i.i.i.i.i.i.2.i = and i8 %w.i.i.i.i.i.i.i.2.i, 63
  %_44.i.i.i.i.i.i.i.2.i = zext nneg i8 %_45.i.i.i.i.i.i.i.2.i to i32
  %_27.i.i.i.i.i.i.i.2.i = or disjoint i32 %_43.i.i.i.i.i.i.i.2.i, %_44.i.i.i.i.i.i.i.2.i
  %17 = or disjoint i32 %_27.i.i.i.i.i.i.i.2.i, %_25.i.i.i.i.i.i.i.2.i
  %.fr.i.i.i.i.i.2.i = freeze i32 %17
  %.not6.i.i.i.i.i.2.i = icmp eq i32 %.fr.i.i.i.i.i.2.i, 1114112
  br i1 %.not6.i.i.i.i.i.2.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i

bb3.i.i.i.i.i.i.i.2.i:                            ; preds = %bb14.i.i.i.i.i.i.i.2.i
  %_7.i.i.i.i.i.i.i.2.i = zext nneg i8 %x.i.i.i.i.i.i.i.2.i to i32
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i: ; preds = %bb3.i.i.i.i.i.i.i.2.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i, %bb6.i.i.i.i.i.i.i.2.i, %bb4.i.i.i.i.i.i.i.2.i
  %_16.i26.i.i.i.i31.i.i.i.2.i = phi ptr [ %_16.i12.i.i.i.i.i.i.i.2.i, %bb4.i.i.i.i.i.i.i.2.i ], [ %_16.i19.i.i.i.i.i.i.i.2.i, %bb6.i.i.i.i.i.i.i.2.i ], [ %_16.i.i.i.i.i.i.i.i.2.i, %bb3.i.i.i.i.i.i.i.2.i ], [ %_16.i26.i.i.i.i.i.i.i.2.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i ]
  %_0.sroa.4.0.i.i.i.i.i.2.i = phi i32 [ %15, %bb4.i.i.i.i.i.i.i.2.i ], [ %16, %bb6.i.i.i.i.i.i.i.2.i ], [ %_7.i.i.i.i.i.i.i.2.i, %bb3.i.i.i.i.i.i.i.2.i ], [ %.fr.i.i.i.i.i.2.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i ]
  %.not.i.i.i.2.i = icmp ne i32 %_0.sroa.0.0.i10.i.i.i.i.i.2.i, 1114112
  %_0.i.i.i.i.i.i.2.i = icmp eq i32 %_0.sroa.0.0.i10.i.i.i.i.i.2.i, %_0.sroa.4.0.i.i.i.i.i.2.i
  %or.cond.i.2.i = select i1 %.not.i.i.i.2.i, i1 %_0.i.i.i.i.i.i.2.i, i1 false
  br i1 %or.cond.i.2.i, label %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.2.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i

_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.2.i: ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i
  %_6.i.i.not.i.i.i.i.i.i.i.3.i = icmp eq ptr %_16.i26.i.i.i.i.i16.i.i.i.2.i, %_8.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.i.3.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.i.3.i

bb14.i.i.i.i.i.i.i.i.3.i:                         ; preds = %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.2.i
  %x.i.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.2.i, align 1, !noalias !326, !noundef !17
  %_6.i.i.i.i.i.i.i.i.3.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.3.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.3.i, label %bb3.i.i.i.i.i.i.i.i.3.i, label %bb4.i.i.i.i.i.i.i.i.3.i

bb4.i.i.i.i.i.i.i.i.3.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.3.i
  %_16.i.i.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.2.i, i64 1
  %_30.i.i.i.i.i.i.i.i.3.i = and i8 %x.i.i.i.i.i.i.i.i.3.i, 31
  %init.i.i.i.i.i.i.i.i.3.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.3.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.3.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.3.i)
  %y.i.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.3.i, align 1, !noalias !326, !noundef !17
  %_33.i.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.3.i, 6
  %_35.i.i.i.i.i.i.i.i.3.i = and i8 %y.i.i.i.i.i.i.i.i.3.i, 63
  %_34.i.i.i.i.i.i.i.i.3.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.3.i to i32
  %18 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.3.i, %_34.i.i.i.i.i.i.i.i.3.i
  %_13.i.i.i.i.i.i.i.i.3.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.3.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.3.i, label %bb6.i.i.i.i.i.i.i.i.3.i, label %bb9.i.i.i.i.i.3.i

bb6.i.i.i.i.i.i.i.i.3.i:                          ; preds = %bb4.i.i.i.i.i.i.i.i.3.i
  %_16.i12.i.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.2.i, i64 2
  %_6.i17.i.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.3.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.3.i)
  %z.i.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.3.i, align 1, !noalias !326, !noundef !17
  %_38.i.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.3.i, 6
  %_40.i.i.i.i.i.i.i.i.3.i = and i8 %z.i.i.i.i.i.i.i.i.3.i, 63
  %_39.i.i.i.i.i.i.i.i.3.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.3.i to i32
  %y_z.i.i.i.i.i.i.i.i.3.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.3.i, %_39.i.i.i.i.i.i.i.i.3.i
  %_20.i.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.3.i, 12
  %19 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.3.i, %_20.i.i.i.i.i.i.i.i.3.i
  %_21.i.i.i.i.i.i.i.i.3.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.3.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.3.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i, label %bb9.i.i.i.i.i.3.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i: ; preds = %bb6.i.i.i.i.i.i.i.i.3.i
  %_16.i19.i.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i.i16.i.i.i.2.i, i64 3
  %_6.i24.i.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.3.i, %_8.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.3.i)
  %w.i.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.3.i, align 1, !noalias !326, !noundef !17
  %_26.i.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.3.i, 18
  %_25.i.i.i.i.i.i.i.i.3.i = and i32 %_26.i.i.i.i.i.i.i.i.3.i, 1835008
  %_43.i.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.3.i, 6
  %_45.i.i.i.i.i.i.i.i.3.i = and i8 %w.i.i.i.i.i.i.i.i.3.i, 63
  %_44.i.i.i.i.i.i.i.i.3.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.3.i to i32
  %_27.i.i.i.i.i.i.i.i.3.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.3.i, %_44.i.i.i.i.i.i.i.i.3.i
  %20 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.3.i, %_25.i.i.i.i.i.i.i.i.3.i
  %.not.i.i.i.i.i.3.i = icmp eq i32 %20, 1114112
  br i1 %.not.i.i.i.i.i.3.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb9.i.i.i.i.i.3.i

bb3.i.i.i.i.i.i.i.i.3.i:                          ; preds = %bb14.i.i.i.i.i.i.i.i.3.i
  %_7.i.i.i.i.i.i.i.i.3.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.3.i to i32
  br label %bb9.i.i.i.i.i.3.i

bb9.i.i.i.i.i.3.i:                                ; preds = %bb3.i.i.i.i.i.i.i.i.3.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i, %bb6.i.i.i.i.i.i.i.i.3.i, %bb4.i.i.i.i.i.i.i.i.3.i
  %_0.sroa.0.0.i10.i.i.i.i.i.3.i = phi i32 [ %20, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i ], [ %18, %bb4.i.i.i.i.i.i.i.i.3.i ], [ %19, %bb6.i.i.i.i.i.i.i.i.3.i ], [ %_7.i.i.i.i.i.i.i.i.3.i, %bb3.i.i.i.i.i.i.i.i.3.i ]
  %_6.i.i.not.i.i.i.i.i.i.3.i = icmp eq ptr %_16.i26.i.i.i.i31.i.i.i.2.i, %_8.i.i.i
  br i1 %_6.i.i.not.i.i.i.i.i.i.3.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %bb14.i.i.i.i.i.i.i.3.i

bb14.i.i.i.i.i.i.i.3.i:                           ; preds = %bb9.i.i.i.i.i.3.i
  %x.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i26.i.i.i.i31.i.i.i.2.i, align 1, !noalias !344, !noundef !17
  %_6.i.i.i.i.i.i.i.3.i = icmp sgt i8 %x.i.i.i.i.i.i.i.3.i, -1
  br i1 %_6.i.i.i.i.i.i.i.3.i, label %bb3.i.i.i.i.i.i.i.3.i, label %bb4.i.i.i.i.i.i.i.3.i

bb4.i.i.i.i.i.i.i.3.i:                            ; preds = %bb14.i.i.i.i.i.i.i.3.i
  %_16.i.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.2.i, i64 1
  %_30.i.i.i.i.i.i.i.3.i = and i8 %x.i.i.i.i.i.i.i.3.i, 31
  %init.i.i.i.i.i.i.i.3.i = zext nneg i8 %_30.i.i.i.i.i.i.i.3.i to i32
  %_6.i10.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.3.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.3.i)
  %y.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.3.i, align 1, !noalias !344, !noundef !17
  %_33.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.3.i, 6
  %_35.i.i.i.i.i.i.i.3.i = and i8 %y.i.i.i.i.i.i.i.3.i, 63
  %_34.i.i.i.i.i.i.i.3.i = zext nneg i8 %_35.i.i.i.i.i.i.i.3.i to i32
  %21 = or disjoint i32 %_33.i.i.i.i.i.i.i.3.i, %_34.i.i.i.i.i.i.i.3.i
  %_13.i.i.i.i.i.i.i.3.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.3.i, -33
  br i1 %_13.i.i.i.i.i.i.i.3.i, label %bb6.i.i.i.i.i.i.i.3.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i

bb6.i.i.i.i.i.i.i.3.i:                            ; preds = %bb4.i.i.i.i.i.i.i.3.i
  %_16.i12.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.2.i, i64 2
  %_6.i17.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.3.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.3.i)
  %z.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.3.i, align 1, !noalias !344, !noundef !17
  %_38.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.3.i, 6
  %_40.i.i.i.i.i.i.i.3.i = and i8 %z.i.i.i.i.i.i.i.3.i, 63
  %_39.i.i.i.i.i.i.i.3.i = zext nneg i8 %_40.i.i.i.i.i.i.i.3.i to i32
  %y_z.i.i.i.i.i.i.i.3.i = or disjoint i32 %_38.i.i.i.i.i.i.i.3.i, %_39.i.i.i.i.i.i.i.3.i
  %_20.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.3.i, 12
  %22 = or disjoint i32 %y_z.i.i.i.i.i.i.i.3.i, %_20.i.i.i.i.i.i.i.3.i
  %_21.i.i.i.i.i.i.i.3.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.3.i, -17
  br i1 %_21.i.i.i.i.i.i.i.3.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i: ; preds = %bb6.i.i.i.i.i.i.i.3.i
  %_16.i19.i.i.i.i.i.i.i.3.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i31.i.i.i.2.i, i64 3
  %_6.i24.i.i.i.i.i.i.i.3.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.3.i, %_8.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.3.i)
  %w.i.i.i.i.i.i.i.3.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.3.i, align 1, !noalias !344, !noundef !17
  %_26.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.3.i, 18
  %_25.i.i.i.i.i.i.i.3.i = and i32 %_26.i.i.i.i.i.i.i.3.i, 1835008
  %_43.i.i.i.i.i.i.i.3.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.3.i, 6
  %_45.i.i.i.i.i.i.i.3.i = and i8 %w.i.i.i.i.i.i.i.3.i, 63
  %_44.i.i.i.i.i.i.i.3.i = zext nneg i8 %_45.i.i.i.i.i.i.i.3.i to i32
  %_27.i.i.i.i.i.i.i.3.i = or disjoint i32 %_43.i.i.i.i.i.i.i.3.i, %_44.i.i.i.i.i.i.i.3.i
  %23 = or disjoint i32 %_27.i.i.i.i.i.i.i.3.i, %_25.i.i.i.i.i.i.i.3.i
  %.fr.i.i.i.i.i.3.i = freeze i32 %23
  %.not6.i.i.i.i.i.3.i = icmp eq i32 %.fr.i.i.i.i.i.3.i, 1114112
  br i1 %.not6.i.i.i.i.i.3.i, label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i, label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i

bb3.i.i.i.i.i.i.i.3.i:                            ; preds = %bb14.i.i.i.i.i.i.i.3.i
  %_7.i.i.i.i.i.i.i.3.i = zext nneg i8 %x.i.i.i.i.i.i.i.3.i to i32
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i: ; preds = %bb3.i.i.i.i.i.i.i.3.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i, %bb6.i.i.i.i.i.i.i.3.i, %bb4.i.i.i.i.i.i.i.3.i
  %_0.sroa.4.0.i.i.i.i.i.3.i = phi i32 [ %21, %bb4.i.i.i.i.i.i.i.3.i ], [ %22, %bb6.i.i.i.i.i.i.i.3.i ], [ %_7.i.i.i.i.i.i.i.3.i, %bb3.i.i.i.i.i.i.i.3.i ], [ %.fr.i.i.i.i.i.3.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i ]
  %.not.i.i.i.3.i = icmp ne i32 %_0.sroa.0.0.i10.i.i.i.i.i.3.i, 1114112
  %_0.i.i.i.i.i.i.3.i = icmp eq i32 %_0.sroa.0.0.i10.i.i.i.i.i.3.i, %_0.sroa.4.0.i.i.i.i.i.3.i
  %or.cond.i.3.i = select i1 %.not.i.i.i.3.i, i1 %_0.i.i.i.i.i.i.3.i, i1 false
  %spec.select.i = select i1 %or.cond.i.3.i, double 4.000000e-01, double 0x3FD3333333333334
  br label %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i

_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i: ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i, %bb9.i.i.i.i.i.3.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.2.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i, %bb9.i.i.i.i.i.2.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.1.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i, %bb9.i.i.i.i.i.1.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i, %bb9.i.i.i.i.i.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i, %bb1.i.i.i.lr.ph.i.i.i.i
  %v.sroa.0.1.i.i.i = phi double [ 0.000000e+00, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i ], [ 0.000000e+00, %bb9.i.i.i.i.i.i ], [ 0.000000e+00, %bb1.i.i.i.lr.ph.i.i.i.i ], [ 0.000000e+00, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i ], [ 0.000000e+00, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i ], [ 1.000000e-01, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.i ], [ 1.000000e-01, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.1.i ], [ 1.000000e-01, %bb9.i.i.i.i.i.1.i ], [ 1.000000e-01, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.1.i ], [ 1.000000e-01, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.1.i ], [ 2.000000e-01, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.1.i ], [ 2.000000e-01, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.2.i ], [ 2.000000e-01, %bb9.i.i.i.i.i.2.i ], [ 2.000000e-01, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.2.i ], [ 2.000000e-01, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.2.i ], [ 0x3FD3333333333334, %_RNCINvNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtBa_9TakeWhileppENtNtNtBe_6traits8iterator8Iterator8try_fold5checkTccEjINtNtNtBg_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2Y_13StringWrapperB3F_ccE0NCINvMs0_B2d_B2a_10wrap_mut_2jB25_NCNvYIB10_INtNtBc_3zip3ZipINtNtBc_4take4TakeNtNtNtBg_3str4iter5CharsEB5o_EB2T_EB1i_5count0E0E0B2Y_.exit.i.i.i.2.i ], [ 0x3FD3333333333334, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.3.i ], [ 0x3FD3333333333334, %bb9.i.i.i.i.i.3.i ], [ 0x3FD3333333333334, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.3.i ], [ %spec.select.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit.i.i.i.3.i ]
  %_13.i = fsub double 1.000000e+00, %sim.i
  %_10.i = fmul double %_13.i, %v.sroa.0.1.i.i.i
  %24 = fadd double %sim.i, %_10.i
  br label %_RINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2_13StringWrapperBJ_ccEB2_.exit

_RINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2_13StringWrapperBJ_ccEB2_.exit: ; preds = %start, %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i
  %_0.sroa.0.0.i = phi double [ %24, %_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_.exit.i ], [ %sim.i, %start ]
  ret double %_0.sroa.0.0.i
}

; strsim::osa_distance
; Function Attrs: uwtable
define noundef i64 @_RNvCs6KJnav5oeQt_6strsim12osa_distance(ptr noalias noundef nonnull readonly align 1 captures(address) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_65 = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  %_6.i = icmp ult i64 %b.1, 32
  br i1 %_6.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %start
; call core::str::count::do_count_chars
  %0 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

bb2.i:                                            ; preds = %start
; call core::str::count::char_count_general_case
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit: ; preds = %bb3.i, %bb2.i
  %_0.sroa.0.0.i = phi i64 [ %1, %bb2.i ], [ %0, %bb3.i ]
  %_7 = add i64 %_0.sroa.0.0.i, 1
  %_23.i.i.i.not.i.not = icmp eq i64 %_7, 0
  br i1 %_23.i.i.i.not.i.not, label %bb28, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
  %_27.0.i.i.i.i = shl i64 %_0.sroa.0.0.i, 3
  %_27.1.i.i.i.i = icmp ugt i64 %_0.sroa.0.0.i, 2305843009213693951
  %_40.i.i.i.i = icmp ugt i64 %_27.0.i.i.i.i, 9223372036854775799
  %or.cond.i = or i1 %_27.1.i.i.i.i, %_40.i.i.i.i
  br i1 %or.cond.i, label %bb3.i.i, label %bb3.i.i.i, !prof !46

bb3.i.i.i:                                        ; preds = %bb6.i.i.i.i
  %new_size2.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i, 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !349
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !349
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb3.i.i, label %bb3.i.i.i.i.i.preheader

bb3.i.i.i.i.i.preheader:                          ; preds = %bb3.i.i.i
  %min.iters.check = icmp ult i64 %_7, 8
  br i1 %min.iters.check, label %bb3.i.i.i.i.i.preheader531, label %vector.ph

vector.ph:                                        ; preds = %bb3.i.i.i.i.i.preheader
  %n.vec = and i64 %_7, 4611686018427387896
  br label %vector.body

vector.body:                                      ; preds = %vector.body, %vector.ph
  %index = phi i64 [ 0, %vector.ph ], [ %index.next, %vector.body ]
  %vec.ind = phi <2 x i64> [ <i64 0, i64 1>, %vector.ph ], [ %vec.ind.next, %vector.body ]
  %step.add = add <2 x i64> %vec.ind, splat (i64 2)
  %step.add.2 = add <2 x i64> %vec.ind, splat (i64 4)
  %step.add.3 = add <2 x i64> %vec.ind, splat (i64 6)
  %4 = getelementptr inbounds nuw i64, ptr %2, i64 %index
  %5 = getelementptr inbounds nuw i8, ptr %4, i64 16
  %6 = getelementptr inbounds nuw i8, ptr %4, i64 32
  %7 = getelementptr inbounds nuw i8, ptr %4, i64 48
  store <2 x i64> %vec.ind, ptr %4, align 8, !noalias !354
  store <2 x i64> %step.add, ptr %5, align 8, !noalias !354
  store <2 x i64> %step.add.2, ptr %6, align 8, !noalias !354
  store <2 x i64> %step.add.3, ptr %7, align 8, !noalias !354
  %index.next = add nuw i64 %index, 8
  %vec.ind.next = add <2 x i64> %vec.ind, splat (i64 8)
  %8 = icmp eq i64 %index.next, %n.vec
  br i1 %8, label %middle.block, label %vector.body, !llvm.loop !367

middle.block:                                     ; preds = %vector.body
  %cmp.n = icmp eq i64 %_7, %n.vec
  br i1 %cmp.n, label %bb6.i.i.i.i40, label %bb3.i.i.i.i.i.preheader531

bb3.i.i.i.i.i.preheader531:                       ; preds = %bb3.i.i.i.i.i.preheader, %middle.block
  %.ph532 = phi i64 [ 0, %bb3.i.i.i.i.i.preheader ], [ %n.vec, %middle.block ]
  br label %bb3.i.i.i.i.i

bb3.i.i:                                          ; preds = %bb3.i.i.i, %bb6.i.i.i.i
  %_4.sroa.4.0.ph.i.i = phi i64 [ 8, %bb3.i.i.i ], [ 0, %bb6.i.i.i.i ]
  %_4.sroa.10.0.ph.i.i = phi i64 [ %new_size2.i.i.i.i, %bb3.i.i.i ], [ undef, %bb6.i.i.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i, i64 %_4.sroa.10.0.ph.i.i) #24, !noalias !368
  unreachable

bb3.i.i.i.i.i:                                    ; preds = %bb3.i.i.i.i.i.preheader531, %bb3.i.i.i.i.i
  %9 = phi i64 [ %10, %bb3.i.i.i.i.i ], [ %.ph532, %bb3.i.i.i.i.i.preheader531 ]
  %10 = add nuw nsw i64 %9, 1
  %_3.i.i.i.i.i.i.i = getelementptr inbounds nuw i64, ptr %2, i64 %9
  store i64 %9, ptr %_3.i.i.i.i.i.i.i, align 8, !noalias !354
  %exitcond.not.i.i.i.i.i = icmp eq i64 %9, %_0.sroa.0.0.i
  br i1 %exitcond.not.i.i.i.i.i, label %bb6.i.i.i.i40, label %bb3.i.i.i.i.i, !llvm.loop !369

bb6.i.i.i.i40:                                    ; preds = %bb3.i.i.i.i.i, %middle.block
  %.lcssa484 = phi i64 [ %n.vec, %middle.block ], [ %10, %bb3.i.i.i.i.i ]
  %11 = ptrtoint ptr %2 to i64
  %_40.i.i.i.i43 = icmp samesign ugt i64 %_0.sroa.0.0.i, 1152921504606846974
  br i1 %_40.i.i.i.i43, label %bb3.i.i53, label %bb3.i.i.i45, !prof !46

bb3.i.i.i45:                                      ; preds = %bb6.i.i.i.i40
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !370
; call __rustc::__rust_alloc
  %12 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i, i64 noundef range(i64 1, 9) 8) #23, !noalias !370
  %13 = icmp eq ptr %12, null
  br i1 %13, label %bb3.i.i53, label %bb3.i.i.i.i.i48.preheader

bb3.i.i.i.i.i48.preheader:                        ; preds = %bb3.i.i.i45
  %min.iters.check486 = icmp ult i64 %_7, 8
  br i1 %min.iters.check486, label %bb3.i.i.i.i.i48.preheader528, label %vector.ph487

vector.ph487:                                     ; preds = %bb3.i.i.i.i.i48.preheader
  %n.vec489 = and i64 %_7, 2305843009213693944
  br label %vector.body490

vector.body490:                                   ; preds = %vector.body490, %vector.ph487
  %index491 = phi i64 [ 0, %vector.ph487 ], [ %index.next496, %vector.body490 ]
  %vec.ind492 = phi <2 x i64> [ <i64 0, i64 1>, %vector.ph487 ], [ %vec.ind.next497, %vector.body490 ]
  %step.add493 = add <2 x i64> %vec.ind492, splat (i64 2)
  %step.add.2494 = add <2 x i64> %vec.ind492, splat (i64 4)
  %step.add.3495 = add <2 x i64> %vec.ind492, splat (i64 6)
  %14 = getelementptr inbounds nuw i64, ptr %12, i64 %index491
  %15 = getelementptr inbounds nuw i8, ptr %14, i64 16
  %16 = getelementptr inbounds nuw i8, ptr %14, i64 32
  %17 = getelementptr inbounds nuw i8, ptr %14, i64 48
  store <2 x i64> %vec.ind492, ptr %14, align 8, !noalias !375
  store <2 x i64> %step.add493, ptr %15, align 8, !noalias !375
  store <2 x i64> %step.add.2494, ptr %16, align 8, !noalias !375
  store <2 x i64> %step.add.3495, ptr %17, align 8, !noalias !375
  %index.next496 = add nuw i64 %index491, 8
  %vec.ind.next497 = add <2 x i64> %vec.ind492, splat (i64 8)
  %18 = icmp eq i64 %index.next496, %n.vec489
  br i1 %18, label %middle.block498, label %vector.body490, !llvm.loop !388

middle.block498:                                  ; preds = %vector.body490
  %cmp.n499 = icmp eq i64 %_7, %n.vec489
  br i1 %cmp.n499, label %bb6.i.i.i, label %bb3.i.i.i.i.i48.preheader528

bb3.i.i.i.i.i48.preheader528:                     ; preds = %bb3.i.i.i.i.i48.preheader, %middle.block498
  %.ph529 = phi i64 [ 0, %bb3.i.i.i.i.i48.preheader ], [ %n.vec489, %middle.block498 ]
  br label %bb3.i.i.i.i.i48

bb3.i.i53:                                        ; preds = %bb3.i.i.i45, %bb6.i.i.i.i40
  %_4.sroa.4.0.ph.i.i54 = phi i64 [ 8, %bb3.i.i.i45 ], [ 0, %bb6.i.i.i.i40 ]
  %_4.sroa.10.0.ph.i.i55 = phi i64 [ %new_size2.i.i.i.i, %bb3.i.i.i45 ], [ undef, %bb6.i.i.i.i40 ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i54, i64 %_4.sroa.10.0.ph.i.i55) #24
          to label %.noexc unwind label %bb24.thread

.noexc:                                           ; preds = %bb3.i.i53
  unreachable

bb3.i.i.i.i.i48:                                  ; preds = %bb3.i.i.i.i.i48.preheader528, %bb3.i.i.i.i.i48
  %19 = phi i64 [ %20, %bb3.i.i.i.i.i48 ], [ %.ph529, %bb3.i.i.i.i.i48.preheader528 ]
  %20 = add nuw nsw i64 %19, 1
  %_3.i.i.i.i.i.i.i51 = getelementptr inbounds nuw i64, ptr %12, i64 %19
  store i64 %19, ptr %_3.i.i.i.i.i.i.i51, align 8, !noalias !375
  %exitcond.not.i.i.i.i.i52 = icmp eq i64 %19, %_0.sroa.0.0.i
  br i1 %exitcond.not.i.i.i.i.i52, label %bb6.i.i.i, label %bb3.i.i.i.i.i48, !llvm.loop !389

bb24:                                             ; preds = %bb23, %bb23.thread
  %21 = phi ptr [ %12, %bb23.thread ], [ %32, %bb23 ]
  %.pn182 = phi { ptr, i32 } [ %34, %bb23.thread ], [ %44, %bb23 ]
  %prev_two_distances.sroa.7.1178 = phi ptr [ %2, %bb23.thread ], [ %33, %bb23 ]
  %alloc_size.i.i.i.i5.i61 = shl nuw i64 %_7, 3
  %22 = icmp ne ptr %21, null
  tail call void @llvm.assume(i1 %22)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %21, i64 noundef %alloc_size.i.i.i.i5.i61, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %bb24, %bb24.thread
  %23 = phi ptr [ %2, %bb24.thread ], [ %prev_two_distances.sroa.7.1178, %bb24 ]
  %.pn.pn164 = phi { ptr, i32 } [ %25, %bb24.thread ], [ %.pn182, %bb24 ]
  %alloc_size.i.i.i.i5.i = shl nuw i64 %_7, 3
  %24 = icmp ne ptr %23, null
  tail call void @llvm.assume(i1 %24)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %23, i64 noundef %alloc_size.i.i.i.i5.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb25

bb24.thread:                                      ; preds = %bb3.i.i53
  %25 = landingpad { ptr, i32 }
          cleanup
  br label %bb2.i.i.i4.i

bb6.i.i.i:                                        ; preds = %bb3.i.i.i.i.i48, %middle.block498
  %.lcssa = phi i64 [ %n.vec489, %middle.block498 ], [ %20, %bb3.i.i.i.i.i48 ]
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23, !noalias !390
; call __rustc::__rust_alloc_zeroed
  %26 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed(i64 noundef range(i64 1, 0) %new_size2.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23, !noalias !390
  %27 = icmp eq ptr %26, null
  br i1 %27, label %bb14.i, label %bb10.i.i

bb10.i.i:                                         ; preds = %bb6.i.i.i
  %28 = ptrtoint ptr %12 to i64
  %29 = ptrtoint ptr %26 to i64
  br label %bb28

bb14.i:                                           ; preds = %bb6.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 8, i64 %new_size2.i.i.i.i) #24
          to label %.noexc59 unwind label %bb23.thread

.noexc59:                                         ; preds = %bb14.i
  unreachable

bb23:                                             ; preds = %cleanup3
  %30 = inttoptr i64 %curr_distances.sroa.7.0250 to ptr
  %alloc_size.i.i.i.i5.i68 = shl nuw i64 %_7, 3
  %31 = icmp ne i64 %curr_distances.sroa.7.0250, 0
  tail call void @llvm.assume(i1 %31)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %30, i64 noundef %alloc_size.i.i.i.i5.i68, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  %32 = inttoptr i64 %prev_distances.sroa.9.1239 to ptr
  %33 = inttoptr i64 %prev_two_distances.sroa.7.2228 to ptr
  br label %bb24

bb23.thread:                                      ; preds = %bb14.i
  %34 = landingpad { ptr, i32 }
          cleanup
  br label %bb24

bb28:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit, %bb10.i.i
  %35 = phi i64 [ %28, %bb10.i.i ], [ 8, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit ]
  %f.val4.i.i.i.i.i37171 = phi i64 [ %.lcssa, %bb10.i.i ], [ 0, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit ]
  %f.val4.i.i.i.i.i158170 = phi i64 [ %.lcssa484, %bb10.i.i ], [ 0, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit ]
  %36 = phi i64 [ %11, %bb10.i.i ], [ 8, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit ]
  %_16.sroa.10.0.i = phi i64 [ %29, %bb10.i.i ], [ 8, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit ]
  %_74 = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  %_6.i.i.not.i.i262 = icmp samesign eq i64 %a.1, 0
  br i1 %_6.i.i.not.i.i262, label %bb6, label %bb14.i.i.i.lr.ph

bb14.i.i.i.lr.ph:                                 ; preds = %bb28
  %_6.i.i.not.i.i81257 = icmp eq i64 %b.1, 0
  br i1 %_6.i.i.not.i.i81257, label %bb14.i.i.i.us, label %bb14.i.i.i.preheader

bb14.i.i.i.preheader:                             ; preds = %bb14.i.i.i.lr.ph
  %_16.i.i.i.i83.peel = getelementptr inbounds nuw i8, ptr %b.0, i64 1
  %_6.i10.i.i.i89.peel = icmp samesign ne i64 %b.1, 1
  %_16.i12.i.i.i90.peel = getelementptr inbounds nuw i8, ptr %b.0, i64 2
  %_6.i17.i.i.i103.peel = icmp samesign ne i64 %b.1, 2
  %_16.i19.i.i.i104.peel = getelementptr inbounds nuw i8, ptr %b.0, i64 3
  %_6.i24.i.i.i113.peel = icmp samesign ne i64 %b.1, 3
  %_16.i26.i.i.i114.peel = getelementptr inbounds nuw i8, ptr %b.0, i64 4
  br label %bb14.i.i.i

bb14.i.i.i.us:                                    ; preds = %bb14.i.i.i.lr.ph, %bb29.us
  %prev_two_distances.sroa.7.2270.us = phi i64 [ %prev_distances.sroa.9.1268.us, %bb29.us ], [ %36, %bb14.i.i.i.lr.ph ]
  %prev_two_distances.sroa.13.0269.us = phi i64 [ %prev_distances.sroa.18.0267.us, %bb29.us ], [ %f.val4.i.i.i.i.i158170, %bb14.i.i.i.lr.ph ]
  %prev_distances.sroa.9.1268.us = phi i64 [ %curr_distances.sroa.7.0266.us, %bb29.us ], [ %35, %bb14.i.i.i.lr.ph ]
  %prev_distances.sroa.18.0267.us = phi i64 [ %curr_distances.sroa.17.0265.us, %bb29.us ], [ %f.val4.i.i.i.i.i37171, %bb14.i.i.i.lr.ph ]
  %curr_distances.sroa.7.0266.us = phi i64 [ %prev_two_distances.sroa.7.2270.us, %bb29.us ], [ %_16.sroa.10.0.i, %bb14.i.i.i.lr.ph ]
  %curr_distances.sroa.17.0265.us = phi i64 [ %prev_two_distances.sroa.13.0269.us, %bb29.us ], [ %_7, %bb14.i.i.i.lr.ph ]
  %iter.sroa.0.0264.us = phi ptr [ %iter.sroa.0.1195.us, %bb29.us ], [ %a.0, %bb14.i.i.i.lr.ph ]
  %iter.sroa.10.0263.us = phi i64 [ %_8.0.i197.us, %bb29.us ], [ 0, %bb14.i.i.i.lr.ph ]
  %_16.i.i.i.i.us = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264.us, i64 1
  %x.i.i.i.us = load i8, ptr %iter.sroa.0.0264.us, align 1, !noalias !395, !noundef !17
  %_6.i.i.i.us = icmp sgt i8 %x.i.i.i.us, -1
  br i1 %_6.i.i.i.us, label %bb5.us, label %bb4.i.i.i.us

bb4.i.i.i.us:                                     ; preds = %bb14.i.i.i.us
  %init.i.i.i.us = zext i8 %x.i.i.i.us to i32
  %_6.i10.i.i.i.us = icmp ne ptr %_16.i.i.i.i.us, %_74
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.us)
  %_16.i12.i.i.i.us = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264.us, i64 2
  %_13.i.i.i.us = icmp samesign ugt i8 %x.i.i.i.us, -33
  br i1 %_13.i.i.i.us, label %bb6.i.i.i64.us, label %bb5.us

bb6.i.i.i64.us:                                   ; preds = %bb4.i.i.i.us
  %y.i.i.i.us = load i8, ptr %_16.i.i.i.i.us, align 1, !noalias !395, !noundef !17
  %_6.i17.i.i.i.us = icmp ne ptr %_16.i12.i.i.i.us, %_74
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.us)
  %_16.i19.i.i.i.us = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264.us, i64 3
  %_21.i.i.i.us = icmp samesign ugt i8 %x.i.i.i.us, -17
  br i1 %_21.i.i.i.us, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us, label %bb5.us

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us: ; preds = %bb6.i.i.i64.us
  %z.i.i.i.us = load i8, ptr %_16.i12.i.i.i.us, align 1, !noalias !395, !noundef !17
  %_40.i.i.i65.us = and i8 %z.i.i.i.us, 63
  %_39.i.i.i.us = zext nneg i8 %_40.i.i.i65.us to i32
  %_35.i.i.i.us = and i8 %y.i.i.i.us, 63
  %_34.i.i.i.us = zext nneg i8 %_35.i.i.i.us to i32
  %_6.i24.i.i.i.us = icmp ne ptr %_16.i19.i.i.i.us, %_74
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.us)
  %w.i.i.i.us = load i8, ptr %_16.i19.i.i.i.us, align 1, !noalias !395, !noundef !17
  %_26.i.i.i.us = shl nuw nsw i32 %init.i.i.i.us, 18
  %_25.i.i.i.us = and i32 %_26.i.i.i.us, 1835008
  %37 = shl nuw nsw i32 %_34.i.i.i.us, 12
  %38 = shl nuw nsw i32 %_39.i.i.i.us, 6
  %_43.i.i.i.us = or disjoint i32 %37, %38
  %_45.i.i.i.us = and i8 %w.i.i.i.us, 63
  %_44.i.i.i.us = zext nneg i8 %_45.i.i.i.us to i32
  %_27.i.i.i.us = or disjoint i32 %_43.i.i.i.us, %_44.i.i.i.us
  %39 = or disjoint i32 %_27.i.i.i.us, %_25.i.i.i.us
  %.not.i.us = icmp eq i32 %39, 1114112
  br i1 %.not.i.us, label %bb6, label %bb3.us

bb3.us:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us
  %_16.i26.i.i.i.us = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264.us, i64 4
  br label %bb5.us

bb5.us:                                           ; preds = %bb14.i.i.i.us, %bb3.us, %bb6.i.i.i64.us, %bb4.i.i.i.us
  %iter.sroa.0.1195.us = phi ptr [ %_16.i26.i.i.i.us, %bb3.us ], [ %_16.i12.i.i.i.us, %bb4.i.i.i.us ], [ %_16.i19.i.i.i.us, %bb6.i.i.i64.us ], [ %_16.i.i.i.i.us, %bb14.i.i.i.us ]
  %_83.not.us = icmp eq i64 %curr_distances.sroa.17.0265.us, 0
  br i1 %_83.not.us, label %panic4, label %bb29.us

bb29.us:                                          ; preds = %bb5.us
  %_8.0.i197.us = add i64 %iter.sroa.10.0263.us, 1
  %40 = inttoptr i64 %curr_distances.sroa.7.0266.us to ptr
  store i64 %_8.0.i197.us, ptr %40, align 8
  %_6.i.i.not.i.i.us = icmp eq ptr %iter.sroa.0.1195.us, %_74
  br i1 %_6.i.i.not.i.i.us, label %bb6, label %bb14.i.i.i.us

bb14.i.i.i:                                       ; preds = %bb14.i.i.i.preheader, %bb10
  %prev_a_char.sroa.0.0272 = phi i32 [ %spec.select.i5.i196, %bb10 ], [ 1114111, %bb14.i.i.i.preheader ]
  %prev_two_distances.sroa.7.2270 = phi i64 [ %prev_distances.sroa.9.1268, %bb10 ], [ %36, %bb14.i.i.i.preheader ]
  %prev_two_distances.sroa.13.0269 = phi i64 [ %prev_distances.sroa.18.0267, %bb10 ], [ %f.val4.i.i.i.i.i158170, %bb14.i.i.i.preheader ]
  %prev_distances.sroa.9.1268 = phi i64 [ %curr_distances.sroa.7.0266, %bb10 ], [ %35, %bb14.i.i.i.preheader ]
  %prev_distances.sroa.18.0267 = phi i64 [ %curr_distances.sroa.17.0265, %bb10 ], [ %f.val4.i.i.i.i.i37171, %bb14.i.i.i.preheader ]
  %curr_distances.sroa.7.0266 = phi i64 [ %prev_two_distances.sroa.7.2270, %bb10 ], [ %_16.sroa.10.0.i, %bb14.i.i.i.preheader ]
  %curr_distances.sroa.17.0265 = phi i64 [ %prev_two_distances.sroa.13.0269, %bb10 ], [ %_7, %bb14.i.i.i.preheader ]
  %iter.sroa.0.0264 = phi ptr [ %iter.sroa.0.1195, %bb10 ], [ %a.0, %bb14.i.i.i.preheader ]
  %iter.sroa.10.0263 = phi i64 [ %_8.0.i197, %bb10 ], [ 0, %bb14.i.i.i.preheader ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264, i64 1
  %x.i.i.i = load i8, ptr %iter.sroa.0.0264, align 1, !noalias !395, !noundef !17
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i66, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_74
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !395, !noundef !17
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %41 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i64, label %bb5

bb3.i.i.i66:                                      ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb5

bb6.i.i.i64:                                      ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_74
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !395, !noundef !17
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i65 = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i65 to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %42 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, label %bb5

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %bb6.i.i.i64
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_74
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !395, !noundef !17
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %43 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  %.not.i = icmp eq i32 %43, 1114112
  br i1 %.not.i, label %bb6, label %bb3

cleanup3:                                         ; preds = %panic10.invoke, %panic4, %panic
  %curr_distances.sroa.7.0250 = phi i64 [ %.us-phi284, %panic4 ], [ %curr_distances.sroa.7.0.lcssa, %panic ], [ %curr_distances.sroa.7.0266, %panic10.invoke ]
  %prev_distances.sroa.9.1239 = phi i64 [ %.us-phi285, %panic4 ], [ %prev_distances.sroa.9.1.lcssa, %panic ], [ %prev_distances.sroa.9.1268, %panic10.invoke ]
  %prev_two_distances.sroa.7.2228 = phi i64 [ %.us-phi286, %panic4 ], [ %prev_two_distances.sroa.7.2.lcssa, %panic ], [ %prev_two_distances.sroa.7.2270, %panic10.invoke ]
  %44 = landingpad { ptr, i32 }
          cleanup
  br i1 %_23.i.i.i.not.i.not, label %bb25, label %bb23

bb3:                                              ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.0264, i64 4
  br label %bb5

bb5:                                              ; preds = %bb3.i.i.i66, %bb6.i.i.i64, %bb4.i.i.i, %bb3
  %spec.select.i5.i196 = phi i32 [ %43, %bb3 ], [ %41, %bb4.i.i.i ], [ %42, %bb6.i.i.i64 ], [ %_7.i.i.i, %bb3.i.i.i66 ]
  %iter.sroa.0.1195 = phi ptr [ %_16.i26.i.i.i, %bb3 ], [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i64 ], [ %_16.i.i.i.i, %bb3.i.i.i66 ]
  %_8.0.i197 = add i64 %iter.sroa.10.0263, 1
  %_83.not = icmp eq i64 %curr_distances.sroa.17.0265, 0
  br i1 %_83.not, label %panic4, label %bb29

bb6:                                              ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i, %bb10, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us, %bb29.us, %bb28
  %curr_distances.sroa.7.0.lcssa = phi i64 [ %_16.sroa.10.0.i, %bb28 ], [ %prev_two_distances.sroa.7.2270.us, %bb29.us ], [ %curr_distances.sroa.7.0266.us, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us ], [ %prev_two_distances.sroa.7.2270, %bb10 ], [ %curr_distances.sroa.7.0266, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %prev_distances.sroa.18.0.lcssa = phi i64 [ %f.val4.i.i.i.i.i37171, %bb28 ], [ %curr_distances.sroa.17.0265.us, %bb29.us ], [ %prev_distances.sroa.18.0267.us, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us ], [ %curr_distances.sroa.17.0265, %bb10 ], [ %prev_distances.sroa.18.0267, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %prev_distances.sroa.9.1.lcssa = phi i64 [ %35, %bb28 ], [ %curr_distances.sroa.7.0266.us, %bb29.us ], [ %prev_distances.sroa.9.1268.us, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us ], [ %curr_distances.sroa.7.0266, %bb10 ], [ %prev_distances.sroa.9.1268, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %prev_two_distances.sroa.7.2.lcssa = phi i64 [ %36, %bb28 ], [ %prev_distances.sroa.9.1268.us, %bb29.us ], [ %prev_two_distances.sroa.7.2270.us, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.us ], [ %prev_distances.sroa.9.1268, %bb10 ], [ %prev_two_distances.sroa.7.2270, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %_88 = icmp ult i64 %_0.sroa.0.0.i, %prev_distances.sroa.18.0.lcssa
  br i1 %_88, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecjEECs6KJnav5oeQt_6strsim.exit78, label %panic

panic:                                            ; preds = %bb6
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_0.sroa.0.0.i, i64 noundef %prev_distances.sroa.18.0.lcssa, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f07197f8880f02704fbb192991fd92e5) #24
          to label %unreachable unwind label %cleanup3

unreachable:                                      ; preds = %panic4, %panic
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecjEECs6KJnav5oeQt_6strsim.exit78: ; preds = %bb6
  %45 = inttoptr i64 %prev_distances.sroa.9.1.lcssa to ptr
  %_59 = getelementptr inbounds nuw i64, ptr %45, i64 %_0.sroa.0.0.i
  %_0 = load i64, ptr %_59, align 8, !noundef !17
  %46 = inttoptr i64 %curr_distances.sroa.7.0.lcssa to ptr
  %alloc_size.i.i.i.i5.i71 = shl nuw i64 %_7, 3
  %47 = icmp ne i64 %curr_distances.sroa.7.0.lcssa, 0
  tail call void @llvm.assume(i1 %47)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %46, i64 noundef %alloc_size.i.i.i.i5.i71, i64 noundef range(i64 1, -9223372036854775807) 8) #23
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %45, i64 noundef %alloc_size.i.i.i.i5.i71, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  %48 = inttoptr i64 %prev_two_distances.sroa.7.2.lcssa to ptr
  %49 = icmp ne i64 %prev_two_distances.sroa.7.2.lcssa, 0
  tail call void @llvm.assume(i1 %49)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %48, i64 noundef %alloc_size.i.i.i.i5.i71, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  ret i64 %_0

bb29:                                             ; preds = %bb5
  %50 = inttoptr i64 %curr_distances.sroa.7.0266 to ptr
  store i64 %_8.0.i197, ptr %50, align 8
  %51 = inttoptr i64 %prev_distances.sroa.9.1268 to ptr
  %_44 = icmp ne i64 %iter.sroa.10.0263, 0
  %52 = inttoptr i64 %prev_two_distances.sroa.7.2270 to ptr
  %53 = tail call i64 @llvm.usub.sat.i64(i64 %prev_distances.sroa.18.0267, i64 1)
  %54 = add i64 %curr_distances.sroa.17.0265, -1
  %x.i.i.i84.peel = load i8, ptr %b.0, align 1, !noalias !402, !noundef !17
  %_6.i.i.i85.peel = icmp sgt i8 %x.i.i.i84.peel, -1
  br i1 %_6.i.i.i85.peel, label %bb3.i.i.i123.peel, label %bb4.i.i.i86.peel

bb4.i.i.i86.peel:                                 ; preds = %bb29
  %_30.i.i.i87.peel = and i8 %x.i.i.i84.peel, 31
  %init.i.i.i88.peel = zext nneg i8 %_30.i.i.i87.peel to i32
  tail call void @llvm.assume(i1 %_6.i10.i.i.i89.peel)
  %y.i.i.i91.peel = load i8, ptr %_16.i.i.i.i83.peel, align 1, !noalias !402, !noundef !17
  %_33.i.i.i92.peel = shl nuw nsw i32 %init.i.i.i88.peel, 6
  %_35.i.i.i93.peel = and i8 %y.i.i.i91.peel, 63
  %_34.i.i.i94.peel = zext nneg i8 %_35.i.i.i93.peel to i32
  %55 = or disjoint i32 %_33.i.i.i92.peel, %_34.i.i.i94.peel
  %_13.i.i.i95.peel = icmp samesign ugt i8 %x.i.i.i84.peel, -33
  br i1 %_13.i.i.i95.peel, label %bb6.i.i.i102.peel, label %bb32.peel

bb6.i.i.i102.peel:                                ; preds = %bb4.i.i.i86.peel
  tail call void @llvm.assume(i1 %_6.i17.i.i.i103.peel)
  %z.i.i.i105.peel = load i8, ptr %_16.i12.i.i.i90.peel, align 1, !noalias !402, !noundef !17
  %_38.i.i.i106.peel = shl nuw nsw i32 %_34.i.i.i94.peel, 6
  %_40.i.i.i107.peel = and i8 %z.i.i.i105.peel, 63
  %_39.i.i.i108.peel = zext nneg i8 %_40.i.i.i107.peel to i32
  %y_z.i.i.i109.peel = or disjoint i32 %_38.i.i.i106.peel, %_39.i.i.i108.peel
  %_20.i.i.i110.peel = shl nuw nsw i32 %init.i.i.i88.peel, 12
  %56 = or disjoint i32 %y_z.i.i.i109.peel, %_20.i.i.i110.peel
  %_21.i.i.i111.peel = icmp samesign ugt i8 %x.i.i.i84.peel, -17
  br i1 %_21.i.i.i111.peel, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel, label %bb32.peel

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel: ; preds = %bb6.i.i.i102.peel
  tail call void @llvm.assume(i1 %_6.i24.i.i.i113.peel)
  %w.i.i.i115.peel = load i8, ptr %_16.i19.i.i.i104.peel, align 1, !noalias !402, !noundef !17
  %_26.i.i.i116.peel = shl nuw nsw i32 %init.i.i.i88.peel, 18
  %_25.i.i.i117.peel = and i32 %_26.i.i.i116.peel, 1835008
  %_43.i.i.i118.peel = shl nuw nsw i32 %y_z.i.i.i109.peel, 6
  %_45.i.i.i119.peel = and i8 %w.i.i.i115.peel, 63
  %_44.i.i.i120.peel = zext nneg i8 %_45.i.i.i119.peel to i32
  %_27.i.i.i121.peel = or disjoint i32 %_43.i.i.i118.peel, %_44.i.i.i120.peel
  %57 = or disjoint i32 %_27.i.i.i121.peel, %_25.i.i.i117.peel
  %.not.i122.peel = icmp eq i32 %57, 1114112
  br i1 %.not.i122.peel, label %bb10, label %bb32.peel

bb3.i.i.i123.peel:                                ; preds = %bb29
  %_7.i.i.i124.peel = zext nneg i8 %x.i.i.i84.peel to i32
  br label %bb32.peel

bb32.peel:                                        ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel, %bb4.i.i.i86.peel, %bb6.i.i.i102.peel, %bb3.i.i.i123.peel
  %spec.select.i5.i97210.peel = phi i32 [ %55, %bb4.i.i.i86.peel ], [ %56, %bb6.i.i.i102.peel ], [ %_7.i.i.i124.peel, %bb3.i.i.i123.peel ], [ %57, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel ]
  %iter1.sroa.0.1209.peel = phi ptr [ %_16.i12.i.i.i90.peel, %bb4.i.i.i86.peel ], [ %_16.i19.i.i.i104.peel, %bb6.i.i.i102.peel ], [ %_16.i.i.i.i83.peel, %bb3.i.i.i123.peel ], [ %_16.i26.i.i.i114.peel, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel ]
  %exitcond.peel.not = icmp ult i64 %prev_distances.sroa.18.0267, 2
  br i1 %exitcond.peel.not, label %panic10.invoke, label %bb36.peel

bb36.peel:                                        ; preds = %bb32.peel
  %exitcond337.peel.not = icmp eq i64 %54, 0
  br i1 %exitcond337.peel.not, label %panic10.invoke, label %bb18.peel

bb18.peel:                                        ; preds = %bb36.peel
  %_41.peel = load i64, ptr %51, align 8, !noundef !17
  %_30.peel = icmp ne i32 %spec.select.i5.i196, %spec.select.i5.i97210.peel
  %cost.peel = zext i1 %_30.peel to i64
  %_40.peel = add i64 %_41.peel, %cost.peel
  %_38.peel = getelementptr inbounds nuw i8, ptr %51, i64 8
  %_37.peel = load i64, ptr %_38.peel, align 8, !noundef !17
  %_36.peel = add i64 %_37.peel, 1
  %_0.sroa.0.0.i136.peel = tail call noundef i64 @llvm.umin.i64(i64 %_40.peel, i64 %_36.peel)
  %_32.peel = add i64 %iter.sroa.10.0263, 2
  %_0.sroa.0.0.i137.peel = tail call noundef i64 @llvm.umin.i64(i64 %_0.sroa.0.0.i136.peel, i64 %_32.peel)
  %_43.peel = getelementptr inbounds nuw i8, ptr %50, i64 8
  store i64 %_0.sroa.0.0.i137.peel, ptr %_43.peel, align 8
  %_6.i.i.not.i.i81.peel = icmp eq ptr %iter1.sroa.0.1209.peel, %_65
  br i1 %_6.i.i.not.i.i81.peel, label %bb10, label %bb14.i.i.i82

panic4:                                           ; preds = %bb5, %bb5.us
  %.us-phi284 = phi i64 [ %curr_distances.sroa.7.0266.us, %bb5.us ], [ %curr_distances.sroa.7.0266, %bb5 ]
  %.us-phi285 = phi i64 [ %prev_distances.sroa.9.1268.us, %bb5.us ], [ %prev_distances.sroa.9.1268, %bb5 ]
  %.us-phi286 = phi i64 [ %prev_two_distances.sroa.7.2270.us, %bb5.us ], [ %prev_two_distances.sroa.7.2270, %bb5 ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef 0, i64 noundef 0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5008b3d584a72b0077539f66d172f9b5) #24
          to label %unreachable unwind label %cleanup3

bb14.i.i.i82:                                     ; preds = %bb18.peel, %bb18
  %_33 = phi i64 [ %_33364, %bb18 ], [ %_0.sroa.0.0.i137.peel, %bb18.peel ]
  %prev_b_char.sroa.0.1260 = phi i32 [ %spec.select.i5.i97210, %bb18 ], [ %spec.select.i5.i97210.peel, %bb18.peel ]
  %iter1.sroa.10.0259 = phi i64 [ %_8.0.i99211, %bb18 ], [ 1, %bb18.peel ]
  %iter1.sroa.0.0258 = phi ptr [ %iter1.sroa.0.1209, %bb18 ], [ %iter1.sroa.0.1209.peel, %bb18.peel ]
  %_16.i.i.i.i83 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0258, i64 1
  %x.i.i.i84 = load i8, ptr %iter1.sroa.0.0258, align 1, !noalias !402, !noundef !17
  %_6.i.i.i85 = icmp sgt i8 %x.i.i.i84, -1
  br i1 %_6.i.i.i85, label %bb3.i.i.i123, label %bb4.i.i.i86

bb4.i.i.i86:                                      ; preds = %bb14.i.i.i82
  %_30.i.i.i87 = and i8 %x.i.i.i84, 31
  %init.i.i.i88 = zext nneg i8 %_30.i.i.i87 to i32
  %_6.i10.i.i.i89 = icmp ne ptr %_16.i.i.i.i83, %_65
  tail call void @llvm.assume(i1 %_6.i10.i.i.i89)
  %_16.i12.i.i.i90 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0258, i64 2
  %y.i.i.i91 = load i8, ptr %_16.i.i.i.i83, align 1, !noalias !402, !noundef !17
  %_33.i.i.i92 = shl nuw nsw i32 %init.i.i.i88, 6
  %_35.i.i.i93 = and i8 %y.i.i.i91, 63
  %_34.i.i.i94 = zext nneg i8 %_35.i.i.i93 to i32
  %58 = or disjoint i32 %_33.i.i.i92, %_34.i.i.i94
  %_13.i.i.i95 = icmp samesign ugt i8 %x.i.i.i84, -33
  br i1 %_13.i.i.i95, label %bb6.i.i.i102, label %bb32

bb3.i.i.i123:                                     ; preds = %bb14.i.i.i82
  %_7.i.i.i124 = zext nneg i8 %x.i.i.i84 to i32
  br label %bb32

bb6.i.i.i102:                                     ; preds = %bb4.i.i.i86
  %_6.i17.i.i.i103 = icmp ne ptr %_16.i12.i.i.i90, %_65
  tail call void @llvm.assume(i1 %_6.i17.i.i.i103)
  %_16.i19.i.i.i104 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0258, i64 3
  %z.i.i.i105 = load i8, ptr %_16.i12.i.i.i90, align 1, !noalias !402, !noundef !17
  %_38.i.i.i106 = shl nuw nsw i32 %_34.i.i.i94, 6
  %_40.i.i.i107 = and i8 %z.i.i.i105, 63
  %_39.i.i.i108 = zext nneg i8 %_40.i.i.i107 to i32
  %y_z.i.i.i109 = or disjoint i32 %_38.i.i.i106, %_39.i.i.i108
  %_20.i.i.i110 = shl nuw nsw i32 %init.i.i.i88, 12
  %59 = or disjoint i32 %y_z.i.i.i109, %_20.i.i.i110
  %_21.i.i.i111 = icmp samesign ugt i8 %x.i.i.i84, -17
  br i1 %_21.i.i.i111, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112, label %bb32

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112: ; preds = %bb6.i.i.i102
  %_6.i24.i.i.i113 = icmp ne ptr %_16.i19.i.i.i104, %_65
  tail call void @llvm.assume(i1 %_6.i24.i.i.i113)
  %w.i.i.i115 = load i8, ptr %_16.i19.i.i.i104, align 1, !noalias !402, !noundef !17
  %_26.i.i.i116 = shl nuw nsw i32 %init.i.i.i88, 18
  %_25.i.i.i117 = and i32 %_26.i.i.i116, 1835008
  %_43.i.i.i118 = shl nuw nsw i32 %y_z.i.i.i109, 6
  %_45.i.i.i119 = and i8 %w.i.i.i115, 63
  %_44.i.i.i120 = zext nneg i8 %_45.i.i.i119 to i32
  %_27.i.i.i121 = or disjoint i32 %_43.i.i.i118, %_44.i.i.i120
  %60 = or disjoint i32 %_27.i.i.i121, %_25.i.i.i117
  %.not.i122 = icmp eq i32 %60, 1114112
  br i1 %.not.i122, label %bb10, label %bb8

bb8:                                              ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112
  %_16.i26.i.i.i114 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.0258, i64 4
  br label %bb32

bb10:                                             ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112, %bb18, %bb18.peel, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i112.peel
  %_6.i.i.not.i.i = icmp eq ptr %iter.sroa.0.1195, %_74
  br i1 %_6.i.i.not.i.i, label %bb6, label %bb14.i.i.i

bb32:                                             ; preds = %bb8, %bb4.i.i.i86, %bb6.i.i.i102, %bb3.i.i.i123
  %spec.select.i5.i97210 = phi i32 [ %60, %bb8 ], [ %58, %bb4.i.i.i86 ], [ %59, %bb6.i.i.i102 ], [ %_7.i.i.i124, %bb3.i.i.i123 ]
  %iter1.sroa.0.1209 = phi ptr [ %_16.i26.i.i.i114, %bb8 ], [ %_16.i12.i.i.i90, %bb4.i.i.i86 ], [ %_16.i19.i.i.i104, %bb6.i.i.i102 ], [ %_16.i.i.i.i83, %bb3.i.i.i123 ]
  %_8.0.i99211 = add nuw i64 %iter1.sroa.10.0259, 1
  %_30 = icmp ne i32 %spec.select.i5.i196, %spec.select.i5.i97210
  %exitcond.not = icmp eq i64 %iter1.sroa.10.0259, %53
  br i1 %exitcond.not, label %panic10.invoke, label %bb36

bb36:                                             ; preds = %bb32
  %cost = zext i1 %_30 to i64
  %_32 = add i64 %_33, 1
  %_38 = getelementptr inbounds nuw i64, ptr %51, i64 %_8.0.i99211
  %_37 = load i64, ptr %_38, align 8, !noundef !17
  %_36 = add i64 %_37, 1
  %_42 = getelementptr inbounds nuw i64, ptr %51, i64 %iter1.sroa.10.0259
  %_41 = load i64, ptr %_42, align 8, !noundef !17
  %_40 = add i64 %_41, %cost
  %_0.sroa.0.0.i136 = tail call noundef i64 @llvm.umin.i64(i64 %_40, i64 %_36)
  %_0.sroa.0.0.i137 = tail call noundef i64 @llvm.umin.i64(i64 %_0.sroa.0.0.i136, i64 %_32)
  %exitcond337.not = icmp eq i64 %iter1.sroa.10.0259, %54
  br i1 %exitcond337.not, label %panic10.invoke, label %bb37

bb37:                                             ; preds = %bb36
  %_43 = getelementptr inbounds nuw i64, ptr %50, i64 %_8.0.i99211
  store i64 %_0.sroa.0.0.i137, ptr %_43, align 8
  %brmerge.not20 = and i1 %_44, %_30
  %_46 = icmp eq i32 %spec.select.i5.i196, %prev_b_char.sroa.0.1260
  %or.cond12 = select i1 %brmerge.not20, i1 %_46, i1 false
  %_47 = icmp eq i32 %spec.select.i5.i97210, %prev_a_char.sroa.0.0272
  %or.cond13 = and i1 %_47, %or.cond12
  br i1 %or.cond13, label %bb38, label %bb18

bb18:                                             ; preds = %bb37, %bb41
  %_33364 = phi i64 [ %_0.sroa.0.0.i137, %bb37 ], [ %_0.sroa.0.0.i138, %bb41 ]
  %_6.i.i.not.i.i81 = icmp eq ptr %iter1.sroa.0.1209, %_65
  br i1 %_6.i.i.not.i.i81, label %bb10, label %bb14.i.i.i82, !llvm.loop !409

bb38:                                             ; preds = %bb37
  %_55 = add i64 %iter1.sroa.10.0259, -1
  %_128 = icmp ult i64 %_55, %prev_two_distances.sroa.13.0269
  br i1 %_128, label %bb41, label %panic10.invoke

panic10.invoke:                                   ; preds = %bb36.peel, %bb32.peel, %bb38, %bb36, %bb32
  %61 = phi i64 [ %_55, %bb38 ], [ %curr_distances.sroa.17.0265, %bb36 ], [ %_8.0.i99211, %bb32 ], [ %curr_distances.sroa.17.0265, %bb36.peel ], [ 1, %bb32.peel ]
  %62 = phi i64 [ %prev_two_distances.sroa.13.0269, %bb38 ], [ %curr_distances.sroa.17.0265, %bb36 ], [ %prev_distances.sroa.18.0267, %bb32 ], [ %curr_distances.sroa.17.0265, %bb36.peel ], [ %prev_distances.sroa.18.0267, %bb32.peel ]
  %63 = phi ptr [ @alloc_1a619d5774b4c6a4ea0553f4f8c35387, %bb38 ], [ @alloc_6944f81dd681dfdbe5866d7fa21924c3, %bb36 ], [ @alloc_2bae18778776445d604cc9d7d59ee8c2, %bb32 ], [ @alloc_6944f81dd681dfdbe5866d7fa21924c3, %bb36.peel ], [ @alloc_2bae18778776445d604cc9d7d59ee8c2, %bb32.peel ]
; invoke core::panicking::panic_bounds_check
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %61, i64 noundef %62, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %63) #24
          to label %panic10.cont unwind label %cleanup3

panic10.cont:                                     ; preds = %panic10.invoke
  unreachable

bb41:                                             ; preds = %bb38
  %_54 = getelementptr inbounds nuw i64, ptr %52, i64 %_55
  %_53 = load i64, ptr %_54, align 8, !noundef !17
  %_52 = add i64 %_53, 1
  %_0.sroa.0.0.i138 = tail call noundef i64 @llvm.umin.i64(i64 %_52, i64 %_0.sroa.0.0.i137)
  store i64 %_0.sroa.0.0.i138, ptr %_43, align 8
  br label %bb18

bb25:                                             ; preds = %cleanup3, %bb2.i.i.i4.i
  %.pn.pn165 = phi { ptr, i32 } [ %.pn.pn164, %bb2.i.i.i4.i ], [ %44, %cleanup3 ]
  resume { ptr, i32 } %.pn.pn165
}

; strsim::sorensen_dice
; Function Attrs: uwtable
define noundef double @_RNvCs6KJnav5oeQt_6strsim13sorensen_dice(ptr noalias noundef nonnull readonly align 1 captures(address) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %buf.i31 = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  %_92 = alloca [24 x i8], align 8
  %_80 = alloca [24 x i8], align 8
  %iter1 = alloca [56 x i8], align 8
  %iter = alloca [56 x i8], align 8
  %a_bigrams = alloca [48 x i8], align 8
  %_47 = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !410
  store i64 0, ptr %buf.i, align 8, !noalias !410
  %_5.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !410
  %_5.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !410
  tail call void @llvm.experimental.noalias.scope.decl(metadata !413)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !416)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !419)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !422)
  %_6.i.i.not.i13.i.i.i.i.i = icmp samesign eq i64 %a.1, 0
  br i1 %_6.i.i.not.i13.i.i.i.i.i, label %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %start, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i4.i = phi ptr [ %_20.i.i.i.i.i.i.i.i5.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ inttoptr (i64 1 to ptr), %start ]
  %len.i.i.i.i.i.i.i.i.i = phi i64 [ %len.i.i.i6.i.i.i.i.i2.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ 0, %start ]
  %self.sroa.0.014.i.i.i.i.i = phi ptr [ %self.sroa.0.17.i.i.i.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ %a.0, %start ]
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %self.sroa.0.014.i.i.i.i.i, align 1, !noalias !425, !noundef !17
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_47
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !noalias !425, !noundef !17
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %0 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb3.i.i.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_47
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !noalias !425, !noundef !17
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %1 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i, label %bb3.i.i.i.i.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i: ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_47
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !noalias !425, !noundef !17
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %2 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  %.not.i.i.i.i.i = icmp eq i32 %2, 1114112
  br i1 %.not.i.i.i.i.i, label %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i
  %spec.select.i8.i.i.i.i.i = phi i32 [ %2, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i ], [ %0, %bb4.i.i.i.i.i.i.i ], [ %1, %bb6.i.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %self.sroa.0.17.i.i.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  switch i32 %spec.select.i8.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i [
    i32 32, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 13, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 12, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 11, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 10, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 9, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
  ]

bb2.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i
  %_7.i.i2.i.i.i.i.i = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i, 128
  br i1 %_7.i.i2.i.i.i.i.i, label %bb2.i.i.i.i.i.i, label %bb6.i.i3.i.i.i.i.i

bb6.i.i3.i.i.i.i.i:                               ; preds = %bb2.i.i.i.i.i.i.i
  %_3.i.i.i.i.i.i.i.i = lshr i32 %spec.select.i8.i.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i [
    i32 0, label %bb6.i.i.i.i.i.i.i.i
    i32 22, label %bb4.i.i.i.i.i.i.i.i
    i32 32, label %bb7.i.i.i.i.i.i.i.i
    i32 48, label %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i
  ]

bb4.i.i.i.i.i.i.i.i:                              ; preds = %bb6.i.i3.i.i.i.i.i
  %3 = icmp eq i32 %spec.select.i8.i.i.i.i.i, 5760
  br i1 %3, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i:                              ; preds = %bb6.i.i3.i.i.i.i.i
  %4 = and i32 %spec.select.i8.i.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i.i = zext nneg i32 %4 to i64
  %5 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_8.i.i.i.i.i.i.i.i
  %_6.i.i.i.i.i.i.i.i = load i8, ptr %5, align 1, !noalias !430, !noundef !17
  %extract.t.i.i.i.i.i.i.i.i = trunc i8 %_6.i.i.i.i.i.i.i.i to i1
  br i1 %extract.t.i.i.i.i.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i:                              ; preds = %bb6.i.i3.i.i.i.i.i
  %6 = and i32 %spec.select.i8.i.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i.i = zext nneg i32 %6 to i64
  %7 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_14.i.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i.i = load i8, ptr %7, align 1, !noalias !430, !noundef !17
  %8 = and i8 %_12.i.i.i.i.i.i.i.i, 2
  %extract.t3.i.i.not.i.i.i.i.i.i = icmp eq i8 %8, 0
  br i1 %extract.t3.i.i.not.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i

_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i: ; preds = %bb6.i.i3.i.i.i.i.i
  %9 = icmp eq i32 %spec.select.i8.i.i.i.i.i, 12288
  br i1 %9, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !431)
  %_14.i.i.i.i.i.i.i.i.i = icmp sgt i64 %len.i.i.i.i.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_14.i.i.i.i.i.i.i.i.i)
  br label %bb2.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i, %bb6.i.i3.i.i.i.i.i
  %_14.i.i.i7.i.i.i.i.i.i = icmp sgt i64 %len.i.i.i.i.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_14.i.i.i7.i.i.i.i.i.i)
  %_17.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i, 2048
  br i1 %_17.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb3.i.i.i.i.i.i.i.i.i
  %_18.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i, 65536
  %..i.i.i.i.i.i.i.i.i = select i1 %_18.i.i.i.i.i.i.i.i.i, i64 3, i64 4
  br label %bb2.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i
  %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i = phi i64 [ 1, %bb2.i.i.i.i.i.i ], [ %..i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ], [ 2, %bb3.i.i.i.i.i.i.i.i.i ]
  %self2.i.i.i.i.i.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !278, !alias.scope !434, !noalias !410, !noundef !17
  %_9.i.i.i.i.i.i.i.i.i.i = sub nsw i64 %self2.i.i.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i = icmp ugt i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i, %_9.i.i.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i, !prof !9

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb2.i.i.i.i.i.i.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i.i.i.i.i, i64 noundef %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i, i64 noundef 1, i64 noundef 1)
          to label %.noexc.i unwind label %cleanup.i, !noalias !410

.noexc.i:                                         ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %count.pre.i.i.i.i.i.i.i.i.i = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !437, !noalias !410
  %_20.i.i.i.i.i.i.i.i.pre.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !437, !noalias !410
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i: ; preds = %.noexc.i, %bb2.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = phi ptr [ %_20.i.i.i.i.i.i.i.i4.i, %bb2.i.i.i.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i.pre.i, %.noexc.i ]
  %count.i.i.i.i.i.i.i.i.i = phi i64 [ %len.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i ], [ %count.pre.i.i.i.i.i.i.i.i.i, %.noexc.i ]
  %_21.i.i.i.i.i.i.i.i.i = icmp sgt i64 %count.i.i.i.i.i.i.i.i.i, -1
  tail call void @llvm.assume(i1 %_21.i.i.i.i.i.i.i.i.i)
  %_8.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_20.i.i.i.i.i.i.i.i.i, i64 %count.i.i.i.i.i.i.i.i.i
  br i1 %_7.i.i2.i.i.i.i.i, label %bb12.i.i.i.i.i.i.i.i.i.i, label %bb7.i.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i
  %_27.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i, 2048
  %10 = trunc i32 %spec.select.i8.i.i.i.i.i to i8
  %_5.i.i.i.i.i.i.i.i.i.i = and i8 %10, 63
  %last1.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_5.i.i.i.i.i.i.i.i.i.i, -128
  %_10.i.i.i.i.i.i.i.i.i.i = lshr i32 %spec.select.i8.i.i.i.i.i, 6
  %11 = trunc i32 %_10.i.i.i.i.i.i.i.i.i.i to i8
  %_8.i.i.i.i.i.i.i.i.i.i = and i8 %11, 63
  %last2.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_8.i.i.i.i.i.i.i.i.i.i, -128
  %_14.i.i.i.i.i.i.i.i.i.i = lshr i32 %spec.select.i8.i.i.i.i.i, 12
  %12 = trunc i32 %_14.i.i.i.i.i.i.i.i.i.i to i8
  %_12.i.i.i.i.i.i.i.i.i.i = and i8 %12, 63
  %last3.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_12.i.i.i.i.i.i.i.i.i.i, -128
  %_18.i.i.i.i.i.i.i.i.i.i = lshr i32 %spec.select.i8.i.i.i.i.i, 18
  %_16.i.i.i.i.i.i.i.i.i.i = trunc nuw nsw i32 %_18.i.i.i.i.i.i.i.i.i.i to i8
  %last4.i.i.i.i.i.i.i.i.i.i = or disjoint i8 %_16.i.i.i.i.i.i.i.i.i.i, -16
  br i1 %_27.i.i.i.i.i.i.i.i.i.i, label %bb1.i2.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb12.i.i.i.i.i.i.i.i.i.i:                         ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i
  %13 = trunc nuw nsw i32 %spec.select.i8.i.i.i.i.i to i8
  store i8 %13, ptr %_8.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i.i.i.i:                         ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %14 = or disjoint i8 %11, -64
  store i8 %14, ptr %_8.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_20.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last1.i.i.i.i.i.i.i.i.i.i, ptr %_20.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %_28.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i, 65536
  br i1 %_28.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
  %15 = or disjoint i8 %12, -32
  store i8 %15, ptr %_8.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_21.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last2.i.i.i.i.i.i.i.i.i.i, ptr %_21.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_22.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 2
  store i8 %last1.i.i.i.i.i.i.i.i.i.i, ptr %_22.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
  store i8 %last4.i.i.i.i.i.i.i.i.i.i, ptr %_8.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_23.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 1
  store i8 %last3.i.i.i.i.i.i.i.i.i.i, ptr %_23.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_24.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 2
  store i8 %last2.i.i.i.i.i.i.i.i.i.i, ptr %_24.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  %_25.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i, i64 3
  store i8 %last1.i.i.i.i.i.i.i.i.i.i, ptr %_25.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !438
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i, %bb1.i2.i.i.i.i.i.i.i.i.i, %bb12.i.i.i.i.i.i.i.i.i.i
  %new_len.i.i.i.i.i.i.i.i.i = add nuw i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i, %len.i.i.i.i.i.i.i.i.i
  store i64 %new_len.i.i.i.i.i.i.i.i.i, ptr %_5.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !437, !noalias !410
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i: ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i, %bb3.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i5.i = phi ptr [ %_20.i.i.i.i.i.i.i.i.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb7.i.i.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb6.i.i.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb4.i.i.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i, %bb3.i.i.i.i.i ]
  %len.i.i.i6.i.i.i.i.i2.i = phi i64 [ %new_len.i.i.i.i.i.i.i.i.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EE0E0B3A_.exit.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0B3_.exit.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i ]
  %_6.i.i.not.i.i.i.i.i.i = icmp eq ptr %self.sroa.0.17.i.i.i.i.i, %_47
  br i1 %_6.i.i.not.i.i.i.i.i.i, label %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit, label %bb14.i.i.i.i.i.i.i

cleanup.i:                                        ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  %buf.val.i = load i64, ptr %buf.i, align 8, !noalias !410
  %17 = icmp eq i64 %buf.val.i, 0
  br i1 %17, label %common.resume, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
  %buf.val1.i = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !410, !nonnull !17, !noundef !17
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %buf.val1.i, i64 noundef %buf.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !410
  br label %common.resume

common.resume:                                    ; preds = %bb27, %bb2.i.i.i4.i.i, %cleanup.i, %bb2.i.i.i4.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %16, %bb2.i.i.i4.i.i.i ], [ %16, %cleanup.i ], [ %.pn16, %bb2.i.i.i4.i.i ], [ %.pn16, %bb27 ]
  resume { ptr, i32 } %common.resume.op

_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit: ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i
  %a.sroa.12.0.copyload183 = phi i64 [ %len.i.i.i6.i.i.i.i.i2.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2L_6StringINtNtB1T_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i ]
  %a.sroa.0.0.copyload.pre = load i64, ptr %buf.i, align 8
  %a.sroa.7.0.copyload.pre = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i, align 8
  br label %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit

_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit: ; preds = %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit, %start
  %a.sroa.12.0.copyload = phi i64 [ %a.sroa.12.0.copyload183, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit ], [ 0, %start ]
  %a.sroa.7.0.copyload = phi ptr [ %a.sroa.7.0.copyload.pre, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit ], [ inttoptr (i64 1 to ptr), %start ]
  %a.sroa.0.0.copyload = phi i64 [ %a.sroa.0.0.copyload.pre, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit.loopexit ], [ 0, %start ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !410
  %_56 = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i31), !noalias !439
  store i64 0, ptr %buf.i31, align 8, !noalias !439
  %_5.sroa.4.0.buf.sroa_idx.i32 = getelementptr inbounds nuw i8, ptr %buf.i31, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_5.sroa.4.0.buf.sroa_idx.i32, align 8, !noalias !439
  %_5.sroa.5.0.buf.sroa_idx.i33 = getelementptr inbounds nuw i8, ptr %buf.i31, i64 16
  store i64 0, ptr %_5.sroa.5.0.buf.sroa_idx.i33, align 8, !noalias !439
  tail call void @llvm.experimental.noalias.scope.decl(metadata !442)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !445)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !448)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !451)
  %_6.i.i.not.i13.i.i.i.i.i34 = icmp samesign eq i64 %b.1, 0
  br i1 %_6.i.i.not.i13.i.i.i.i.i34, label %bb30, label %bb14.i.i.i.i.i.i.i35

bb14.i.i.i.i.i.i.i35:                             ; preds = %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i4.i36 = phi ptr [ %_20.i.i.i.i.i.i.i.i5.i55, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ inttoptr (i64 1 to ptr), %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  %len.i.i.i.i.i.i.i.i.i37 = phi i64 [ %len.i.i.i6.i.i.i.i.i2.i56, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ 0, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  %self.sroa.0.014.i.i.i.i.i38 = phi ptr [ %self.sroa.0.17.i.i.i.i.i54, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ], [ %b.0, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  %_16.i.i.i.i.i.i.i.i39 = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i38, i64 1
  %x.i.i.i.i.i.i.i40 = load i8, ptr %self.sroa.0.014.i.i.i.i.i38, align 1, !noalias !454, !noundef !17
  %_6.i.i.i.i.i.i.i41 = icmp sgt i8 %x.i.i.i.i.i.i.i40, -1
  br i1 %_6.i.i.i.i.i.i.i41, label %bb3.i.i.i.i.i.i.i145, label %bb4.i.i.i.i.i.i.i42

bb4.i.i.i.i.i.i.i42:                              ; preds = %bb14.i.i.i.i.i.i.i35
  %_30.i.i.i.i.i.i.i43 = and i8 %x.i.i.i.i.i.i.i40, 31
  %init.i.i.i.i.i.i.i44 = zext nneg i8 %_30.i.i.i.i.i.i.i43 to i32
  %_6.i10.i.i.i.i.i.i.i45 = icmp ne ptr %_16.i.i.i.i.i.i.i.i39, %_56
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i45)
  %_16.i12.i.i.i.i.i.i.i46 = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i38, i64 2
  %y.i.i.i.i.i.i.i47 = load i8, ptr %_16.i.i.i.i.i.i.i.i39, align 1, !noalias !454, !noundef !17
  %_33.i.i.i.i.i.i.i48 = shl nuw nsw i32 %init.i.i.i.i.i.i.i44, 6
  %_35.i.i.i.i.i.i.i49 = and i8 %y.i.i.i.i.i.i.i47, 63
  %_34.i.i.i.i.i.i.i50 = zext nneg i8 %_35.i.i.i.i.i.i.i49 to i32
  %18 = or disjoint i32 %_33.i.i.i.i.i.i.i48, %_34.i.i.i.i.i.i.i50
  %_13.i.i.i.i.i.i.i51 = icmp samesign ugt i8 %x.i.i.i.i.i.i.i40, -33
  br i1 %_13.i.i.i.i.i.i.i51, label %bb6.i.i.i.i.i.i.i124, label %bb3.i.i.i.i.i52

bb3.i.i.i.i.i.i.i145:                             ; preds = %bb14.i.i.i.i.i.i.i35
  %_7.i.i.i.i.i.i.i146 = zext nneg i8 %x.i.i.i.i.i.i.i40 to i32
  br label %bb3.i.i.i.i.i52

bb6.i.i.i.i.i.i.i124:                             ; preds = %bb4.i.i.i.i.i.i.i42
  %_6.i17.i.i.i.i.i.i.i125 = icmp ne ptr %_16.i12.i.i.i.i.i.i.i46, %_56
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i125)
  %_16.i19.i.i.i.i.i.i.i126 = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i38, i64 3
  %z.i.i.i.i.i.i.i127 = load i8, ptr %_16.i12.i.i.i.i.i.i.i46, align 1, !noalias !454, !noundef !17
  %_38.i.i.i.i.i.i.i128 = shl nuw nsw i32 %_34.i.i.i.i.i.i.i50, 6
  %_40.i.i.i.i.i.i.i129 = and i8 %z.i.i.i.i.i.i.i127, 63
  %_39.i.i.i.i.i.i.i130 = zext nneg i8 %_40.i.i.i.i.i.i.i129 to i32
  %y_z.i.i.i.i.i.i.i131 = or disjoint i32 %_38.i.i.i.i.i.i.i128, %_39.i.i.i.i.i.i.i130
  %_20.i.i.i.i.i.i.i132 = shl nuw nsw i32 %init.i.i.i.i.i.i.i44, 12
  %19 = or disjoint i32 %y_z.i.i.i.i.i.i.i131, %_20.i.i.i.i.i.i.i132
  %_21.i.i.i.i.i.i.i133 = icmp samesign ugt i8 %x.i.i.i.i.i.i.i40, -17
  br i1 %_21.i.i.i.i.i.i.i133, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134, label %bb3.i.i.i.i.i52

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134: ; preds = %bb6.i.i.i.i.i.i.i124
  %_6.i24.i.i.i.i.i.i.i135 = icmp ne ptr %_16.i19.i.i.i.i.i.i.i126, %_56
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i135)
  %_16.i26.i.i.i.i.i.i.i136 = getelementptr inbounds nuw i8, ptr %self.sroa.0.014.i.i.i.i.i38, i64 4
  %w.i.i.i.i.i.i.i137 = load i8, ptr %_16.i19.i.i.i.i.i.i.i126, align 1, !noalias !454, !noundef !17
  %_26.i.i.i.i.i.i.i138 = shl nuw nsw i32 %init.i.i.i.i.i.i.i44, 18
  %_25.i.i.i.i.i.i.i139 = and i32 %_26.i.i.i.i.i.i.i138, 1835008
  %_43.i.i.i.i.i.i.i140 = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i131, 6
  %_45.i.i.i.i.i.i.i141 = and i8 %w.i.i.i.i.i.i.i137, 63
  %_44.i.i.i.i.i.i.i142 = zext nneg i8 %_45.i.i.i.i.i.i.i141 to i32
  %_27.i.i.i.i.i.i.i143 = or disjoint i32 %_43.i.i.i.i.i.i.i140, %_44.i.i.i.i.i.i.i142
  %20 = or disjoint i32 %_27.i.i.i.i.i.i.i143, %_25.i.i.i.i.i.i.i139
  %.not.i.i.i.i.i144 = icmp eq i32 %20, 1114112
  br i1 %.not.i.i.i.i.i144, label %bb30.loopexit, label %bb3.i.i.i.i.i52

bb3.i.i.i.i.i52:                                  ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134, %bb6.i.i.i.i.i.i.i124, %bb3.i.i.i.i.i.i.i145, %bb4.i.i.i.i.i.i.i42
  %spec.select.i8.i.i.i.i.i53 = phi i32 [ %20, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134 ], [ %18, %bb4.i.i.i.i.i.i.i42 ], [ %19, %bb6.i.i.i.i.i.i.i124 ], [ %_7.i.i.i.i.i.i.i146, %bb3.i.i.i.i.i.i.i145 ]
  %self.sroa.0.17.i.i.i.i.i54 = phi ptr [ %_16.i26.i.i.i.i.i.i.i136, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134 ], [ %_16.i12.i.i.i.i.i.i.i46, %bb4.i.i.i.i.i.i.i42 ], [ %_16.i19.i.i.i.i.i.i.i126, %bb6.i.i.i.i.i.i.i124 ], [ %_16.i.i.i.i.i.i.i.i39, %bb3.i.i.i.i.i.i.i145 ]
  switch i32 %spec.select.i8.i.i.i.i.i53, label %bb2.i.i.i.i.i.i.i58 [
    i32 32, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 13, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 12, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 11, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 10, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
    i32 9, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
  ]

bb2.i.i.i.i.i.i.i58:                              ; preds = %bb3.i.i.i.i.i52
  %_7.i.i2.i.i.i.i.i59 = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i53, 128
  br i1 %_7.i.i2.i.i.i.i.i59, label %bb2.i.i.i.i.i.i122, label %bb6.i.i3.i.i.i.i.i60

bb6.i.i3.i.i.i.i.i60:                             ; preds = %bb2.i.i.i.i.i.i.i58
  %_3.i.i.i.i.i.i.i.i61 = lshr i32 %spec.select.i8.i.i.i.i.i53, 8
  switch i32 %_3.i.i.i.i.i.i.i.i61, label %bb3.i.i.i.i.i.i.i.i.i62 [
    i32 0, label %bb6.i.i.i.i.i.i.i.i118
    i32 22, label %bb4.i.i.i.i.i.i.i.i117
    i32 32, label %bb7.i.i.i.i.i.i.i.i113
    i32 48, label %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i
  ]

bb4.i.i.i.i.i.i.i.i117:                           ; preds = %bb6.i.i3.i.i.i.i.i60
  %21 = icmp eq i32 %spec.select.i8.i.i.i.i.i53, 5760
  br i1 %21, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i62

bb6.i.i.i.i.i.i.i.i118:                           ; preds = %bb6.i.i3.i.i.i.i.i60
  %22 = and i32 %spec.select.i8.i.i.i.i.i53, 255
  %_8.i.i.i.i.i.i.i.i119 = zext nneg i32 %22 to i64
  %23 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_8.i.i.i.i.i.i.i.i119
  %_6.i.i.i.i.i.i.i.i120 = load i8, ptr %23, align 1, !noalias !459, !noundef !17
  %extract.t.i.i.i.i.i.i.i.i121 = trunc i8 %_6.i.i.i.i.i.i.i.i120 to i1
  br i1 %extract.t.i.i.i.i.i.i.i.i121, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i62

bb7.i.i.i.i.i.i.i.i113:                           ; preds = %bb6.i.i3.i.i.i.i.i60
  %24 = and i32 %spec.select.i8.i.i.i.i.i53, 255
  %_14.i.i.i.i.i.i.i.i114 = zext nneg i32 %24 to i64
  %25 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_14.i.i.i.i.i.i.i.i114
  %_12.i.i.i.i.i.i.i.i115 = load i8, ptr %25, align 1, !noalias !459, !noundef !17
  %26 = and i8 %_12.i.i.i.i.i.i.i.i115, 2
  %extract.t3.i.i.not.i.i.i.i.i.i116 = icmp eq i8 %26, 0
  br i1 %extract.t3.i.i.not.i.i.i.i.i.i116, label %bb3.i.i.i.i.i.i.i.i.i62, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i

_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i: ; preds = %bb6.i.i3.i.i.i.i.i60
  %27 = icmp eq i32 %spec.select.i8.i.i.i.i.i53, 12288
  br i1 %27, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i62

bb2.i.i.i.i.i.i122:                               ; preds = %bb2.i.i.i.i.i.i.i58
  tail call void @llvm.experimental.noalias.scope.decl(metadata !460)
  %_14.i.i.i.i.i.i.i.i.i123 = icmp sgt i64 %len.i.i.i.i.i.i.i.i.i37, -1
  tail call void @llvm.assume(i1 %_14.i.i.i.i.i.i.i.i.i123)
  br label %bb2.i.i.i.i.i.i.i.i.i68

bb3.i.i.i.i.i.i.i.i.i62:                          ; preds = %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i113, %bb6.i.i.i.i.i.i.i.i118, %bb4.i.i.i.i.i.i.i.i117, %bb6.i.i3.i.i.i.i.i60
  %_14.i.i.i7.i.i.i.i.i.i63 = icmp sgt i64 %len.i.i.i.i.i.i.i.i.i37, -1
  tail call void @llvm.assume(i1 %_14.i.i.i7.i.i.i.i.i.i63)
  %_17.i.i.i.i.i.i.i.i.i64 = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i53, 2048
  br i1 %_17.i.i.i.i.i.i.i.i.i64, label %bb2.i.i.i.i.i.i.i.i.i68, label %bb4.i.i.i.i.i.i.i.i.i65

bb4.i.i.i.i.i.i.i.i.i65:                          ; preds = %bb3.i.i.i.i.i.i.i.i.i62
  %_18.i.i.i.i.i.i.i.i.i66 = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i53, 65536
  %..i.i.i.i.i.i.i.i.i67 = select i1 %_18.i.i.i.i.i.i.i.i.i66, i64 3, i64 4
  br label %bb2.i.i.i.i.i.i.i.i.i68

bb2.i.i.i.i.i.i.i.i.i68:                          ; preds = %bb4.i.i.i.i.i.i.i.i.i65, %bb3.i.i.i.i.i.i.i.i.i62, %bb2.i.i.i.i.i.i122
  %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i69 = phi i64 [ 1, %bb2.i.i.i.i.i.i122 ], [ %..i.i.i.i.i.i.i.i.i67, %bb4.i.i.i.i.i.i.i.i.i65 ], [ 2, %bb3.i.i.i.i.i.i.i.i.i62 ]
  %self2.i.i.i.i.i.i.i.i.i.i70 = load i64, ptr %buf.i31, align 8, !range !278, !alias.scope !463, !noalias !439, !noundef !17
  %_9.i.i.i.i.i.i.i.i.i.i71 = sub nsw i64 %self2.i.i.i.i.i.i.i.i.i.i70, %len.i.i.i.i.i.i.i.i.i37
  %_7.i.i.i.i.i.i.i.i.i.i72 = icmp ugt i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i69, %_9.i.i.i.i.i.i.i.i.i.i71
  br i1 %_7.i.i.i.i.i.i.i.i.i.i72, label %bb1.i.i.i.i.i.i.i.i.i.i104, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i73, !prof !9

bb1.i.i.i.i.i.i.i.i.i.i104:                       ; preds = %bb2.i.i.i.i.i.i.i.i.i68
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i31, i64 noundef %len.i.i.i.i.i.i.i.i.i37, i64 noundef %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i69, i64 noundef 1, i64 noundef 1)
          to label %.noexc.i110 unwind label %cleanup.i105, !noalias !439

.noexc.i110:                                      ; preds = %bb1.i.i.i.i.i.i.i.i.i.i104
  %count.pre.i.i.i.i.i.i.i.i.i111 = load i64, ptr %_5.sroa.5.0.buf.sroa_idx.i33, align 8, !alias.scope !466, !noalias !439
  %_20.i.i.i.i.i.i.i.i.pre.i112 = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i32, align 8, !alias.scope !466, !noalias !439
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i73

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i73: ; preds = %.noexc.i110, %bb2.i.i.i.i.i.i.i.i.i68
  %_20.i.i.i.i.i.i.i.i.i74 = phi ptr [ %_20.i.i.i.i.i.i.i.i4.i36, %bb2.i.i.i.i.i.i.i.i.i68 ], [ %_20.i.i.i.i.i.i.i.i.pre.i112, %.noexc.i110 ]
  %count.i.i.i.i.i.i.i.i.i75 = phi i64 [ %len.i.i.i.i.i.i.i.i.i37, %bb2.i.i.i.i.i.i.i.i.i68 ], [ %count.pre.i.i.i.i.i.i.i.i.i111, %.noexc.i110 ]
  %_21.i.i.i.i.i.i.i.i.i76 = icmp sgt i64 %count.i.i.i.i.i.i.i.i.i75, -1
  tail call void @llvm.assume(i1 %_21.i.i.i.i.i.i.i.i.i76)
  %_8.i.i.i.i.i.i.i.i.i77 = getelementptr inbounds nuw i8, ptr %_20.i.i.i.i.i.i.i.i.i74, i64 %count.i.i.i.i.i.i.i.i.i75
  br i1 %_7.i.i2.i.i.i.i.i59, label %bb12.i.i.i.i.i.i.i.i.i.i103, label %bb7.i.i.i.i.i.i.i.i.i.i78

bb7.i.i.i.i.i.i.i.i.i.i78:                        ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i73
  %_27.i.i.i.i.i.i.i.i.i.i79 = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i53, 2048
  %28 = trunc i32 %spec.select.i8.i.i.i.i.i53 to i8
  %_5.i.i.i.i.i.i.i.i.i.i80 = and i8 %28, 63
  %last1.i.i.i.i.i.i.i.i.i.i81 = or disjoint i8 %_5.i.i.i.i.i.i.i.i.i.i80, -128
  %_10.i.i.i.i.i.i.i.i.i.i82 = lshr i32 %spec.select.i8.i.i.i.i.i53, 6
  %29 = trunc i32 %_10.i.i.i.i.i.i.i.i.i.i82 to i8
  %_8.i.i.i.i.i.i.i.i.i.i83 = and i8 %29, 63
  %last2.i.i.i.i.i.i.i.i.i.i84 = or disjoint i8 %_8.i.i.i.i.i.i.i.i.i.i83, -128
  %_14.i.i.i.i.i.i.i.i.i.i85 = lshr i32 %spec.select.i8.i.i.i.i.i53, 12
  %30 = trunc i32 %_14.i.i.i.i.i.i.i.i.i.i85 to i8
  %_12.i.i.i.i.i.i.i.i.i.i86 = and i8 %30, 63
  %last3.i.i.i.i.i.i.i.i.i.i87 = or disjoint i8 %_12.i.i.i.i.i.i.i.i.i.i86, -128
  %_18.i.i.i.i.i.i.i.i.i.i88 = lshr i32 %spec.select.i8.i.i.i.i.i53, 18
  %_16.i.i.i.i.i.i.i.i.i.i89 = trunc nuw nsw i32 %_18.i.i.i.i.i.i.i.i.i.i88 to i8
  %last4.i.i.i.i.i.i.i.i.i.i90 = or disjoint i8 %_16.i.i.i.i.i.i.i.i.i.i89, -16
  br i1 %_27.i.i.i.i.i.i.i.i.i.i79, label %bb1.i2.i.i.i.i.i.i.i.i.i101, label %bb2.i.i.i.i.i.i.i.i.i.i91

bb12.i.i.i.i.i.i.i.i.i.i103:                      ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim.exit.i.i.i.i.i.i.i.i.i73
  %31 = trunc nuw nsw i32 %spec.select.i8.i.i.i.i.i53 to i8
  store i8 %31, ptr %_8.i.i.i.i.i.i.i.i.i77, align 1, !noalias !467
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i

bb1.i2.i.i.i.i.i.i.i.i.i101:                      ; preds = %bb7.i.i.i.i.i.i.i.i.i.i78
  %32 = or disjoint i8 %29, -64
  store i8 %32, ptr %_8.i.i.i.i.i.i.i.i.i77, align 1, !noalias !467
  %_20.i.i.i.i.i.i.i.i.i.i102 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 1
  store i8 %last1.i.i.i.i.i.i.i.i.i.i81, ptr %_20.i.i.i.i.i.i.i.i.i.i102, align 1, !noalias !467
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i91:                        ; preds = %bb7.i.i.i.i.i.i.i.i.i.i78
  %_28.i.i.i.i.i.i.i.i.i.i92 = icmp samesign ult i32 %spec.select.i8.i.i.i.i.i53, 65536
  br i1 %_28.i.i.i.i.i.i.i.i.i.i92, label %bb3.i.i.i.i.i.i.i.i.i.i98, label %bb4.i.i.i.i.i.i.i.i.i.i93

bb3.i.i.i.i.i.i.i.i.i.i98:                        ; preds = %bb2.i.i.i.i.i.i.i.i.i.i91
  %33 = or disjoint i8 %30, -32
  store i8 %33, ptr %_8.i.i.i.i.i.i.i.i.i77, align 1, !noalias !467
  %_21.i.i.i.i.i.i.i.i.i.i99 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 1
  store i8 %last2.i.i.i.i.i.i.i.i.i.i84, ptr %_21.i.i.i.i.i.i.i.i.i.i99, align 1, !noalias !467
  %_22.i.i.i.i.i.i.i.i.i.i100 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 2
  store i8 %last1.i.i.i.i.i.i.i.i.i.i81, ptr %_22.i.i.i.i.i.i.i.i.i.i100, align 1, !noalias !467
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i93:                        ; preds = %bb2.i.i.i.i.i.i.i.i.i.i91
  store i8 %last4.i.i.i.i.i.i.i.i.i.i90, ptr %_8.i.i.i.i.i.i.i.i.i77, align 1, !noalias !467
  %_23.i.i.i.i.i.i.i.i.i.i94 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 1
  store i8 %last3.i.i.i.i.i.i.i.i.i.i87, ptr %_23.i.i.i.i.i.i.i.i.i.i94, align 1, !noalias !467
  %_24.i.i.i.i.i.i.i.i.i.i95 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 2
  store i8 %last2.i.i.i.i.i.i.i.i.i.i84, ptr %_24.i.i.i.i.i.i.i.i.i.i95, align 1, !noalias !467
  %_25.i.i.i.i.i.i.i.i.i.i96 = getelementptr inbounds nuw i8, ptr %_8.i.i.i.i.i.i.i.i.i77, i64 3
  store i8 %last1.i.i.i.i.i.i.i.i.i.i81, ptr %_25.i.i.i.i.i.i.i.i.i.i96, align 1, !noalias !467
  br label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i93, %bb3.i.i.i.i.i.i.i.i.i.i98, %bb1.i2.i.i.i.i.i.i.i.i.i101, %bb12.i.i.i.i.i.i.i.i.i.i103
  %new_len.i.i.i.i.i.i.i.i.i97 = add nuw i64 %ch_len.sroa.0.0.i.i.i.i.i.i.i.i.i69, %len.i.i.i.i.i.i.i.i.i37
  store i64 %new_len.i.i.i.i.i.i.i.i.i97, ptr %_5.sroa.5.0.buf.sroa_idx.i33, align 8, !alias.scope !466, !noalias !439
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i: ; preds = %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i, %bb7.i.i.i.i.i.i.i.i113, %bb6.i.i.i.i.i.i.i.i118, %bb4.i.i.i.i.i.i.i.i117, %bb3.i.i.i.i.i52, %bb3.i.i.i.i.i52, %bb3.i.i.i.i.i52, %bb3.i.i.i.i.i52, %bb3.i.i.i.i.i52, %bb3.i.i.i.i.i52
  %_20.i.i.i.i.i.i.i.i5.i55 = phi ptr [ %_20.i.i.i.i.i.i.i.i.i74, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i36, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb7.i.i.i.i.i.i.i.i113 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb6.i.i.i.i.i.i.i.i118 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb4.i.i.i.i.i.i.i.i117 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ], [ %_20.i.i.i.i.i.i.i.i4.i36, %bb3.i.i.i.i.i52 ]
  %len.i.i.i6.i.i.i.i.i2.i56 = phi i64 [ %new_len.i.i.i.i.i.i.i.i.i97, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB1p_6StringINtNtBa_7collect6ExtendcE6extendINtNtNtBc_8adapters6filter6FilterNtNtNtBe_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EE0E0B3A_.exit.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i37, %_RNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0B3_.exit.i.i.i.i.i.i ], [ %len.i.i.i.i.i.i.i.i.i37, %bb7.i.i.i.i.i.i.i.i113 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb6.i.i.i.i.i.i.i.i118 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb4.i.i.i.i.i.i.i.i117 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ], [ %len.i.i.i.i.i.i.i.i.i37, %bb3.i.i.i.i.i52 ]
  %_6.i.i.not.i.i.i.i.i.i57 = icmp eq ptr %self.sroa.0.17.i.i.i.i.i54, %_56
  br i1 %_6.i.i.not.i.i.i.i.i.i57, label %bb30.loopexit, label %bb14.i.i.i.i.i.i.i35

cleanup.i105:                                     ; preds = %bb1.i.i.i.i.i.i.i.i.i.i104
  %34 = landingpad { ptr, i32 }
          cleanup
  %buf.val.i106 = load i64, ptr %buf.i31, align 8, !noalias !439
  %35 = icmp eq i64 %buf.val.i106, 0
  br i1 %35, label %bb27, label %bb2.i.i.i4.i.i.i107

bb2.i.i.i4.i.i.i107:                              ; preds = %cleanup.i105
  %buf.val1.i108 = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i32, align 8, !noalias !439, !nonnull !17, !noundef !17
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %buf.val1.i108, i64 noundef %buf.val.i106, i64 noundef range(i64 1, -9223372036854775807) 1) #23, !noalias !439
  br label %bb27

bb27:                                             ; preds = %bb2.i.i.i4.i.i147, %bb26, %bb2.i.i.i4.i.i.i107, %cleanup.i105
  %.pn16 = phi { ptr, i32 } [ %34, %bb2.i.i.i4.i.i.i107 ], [ %34, %cleanup.i105 ], [ %.pn, %bb26 ], [ %.pn, %bb2.i.i.i4.i.i147 ]
  %36 = icmp eq i64 %a.sroa.0.0.copyload, 0
  br i1 %36, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb27
  %37 = icmp ne ptr %a.sroa.7.0.copyload, null
  call void @llvm.assume(i1 %37)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %a.sroa.7.0.copyload, i64 noundef %a.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %common.resume

bb30.loopexit:                                    ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i
  %b.sroa.12.0.copyload187 = phi i64 [ %len.i.i.i.i.i.i.i.i.i37, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i134 ], [ %len.i.i.i6.i.i.i.i.i2.i56, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2N_6StringINtNtB1V_7collect6ExtendcE6extendINtB4_6FilterNtNtNtBa_3str4iter5CharsB13_EE0E0E0B17_.exit.i.i.i.i.i ]
  %b.sroa.0.0.copyload.pre = load i64, ptr %buf.i31, align 8
  %b.sroa.7.0.copyload.pre = load ptr, ptr %_5.sroa.4.0.buf.sroa_idx.i32, align 8
  br label %bb30

bb30:                                             ; preds = %bb30.loopexit, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit
  %b.sroa.12.0.copyload = phi i64 [ %b.sroa.12.0.copyload187, %bb30.loopexit ], [ 0, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  %b.sroa.7.0.copyload = phi ptr [ %b.sroa.7.0.copyload.pre, %bb30.loopexit ], [ inttoptr (i64 1 to ptr), %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  %b.sroa.0.0.copyload = phi i64 [ %b.sroa.0.0.copyload.pre, %bb30.loopexit ], [ 0, %_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_.exit ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i31), !noalias !439
  %38 = icmp ne ptr %a.sroa.7.0.copyload, null
  tail call void @llvm.assume(i1 %38)
  %_3.not.i = icmp eq i64 %a.sroa.12.0.copyload, %b.sroa.12.0.copyload
  br i1 %_3.not.i, label %bb31, label %bb2

bb26:                                             ; preds = %cleanup3, %cleanup2
  %.pn = phi { ptr, i32 } [ %lpad.phi, %cleanup3 ], [ %40, %cleanup2 ]
  %39 = icmp eq i64 %b.sroa.0.0.copyload, 0
  br i1 %39, label %bb27, label %bb2.i.i.i4.i.i147

bb2.i.i.i4.i.i147:                                ; preds = %bb26
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %b.sroa.7.0.copyload, i64 noundef %b.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb27

cleanup2:                                         ; preds = %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim.exit.i.i
  %40 = landingpad { ptr, i32 }
          cleanup
  br label %bb26

bb31:                                             ; preds = %bb30
  %41 = tail call i32 @memcmp(ptr nonnull readonly align 1 %a.sroa.7.0.copyload, ptr nonnull readonly align 1 %b.sroa.7.0.copyload, i64 range(i64 0, -9223372036854775808) %a.sroa.12.0.copyload), !alias.scope !468
  %42 = icmp eq i32 %41, 0
  br i1 %42, label %bb21, label %bb2

bb2:                                              ; preds = %bb30, %bb31
  %_70 = icmp sgt i64 %a.sroa.12.0.copyload, -1
  tail call void @llvm.assume(i1 %_70)
  %_10 = icmp samesign ult i64 %a.sroa.12.0.copyload, 2
  br i1 %_10, label %bb21, label %bb4

bb4:                                              ; preds = %bb2
  %_71 = icmp sgt i64 %b.sroa.12.0.copyload, -1
  tail call void @llvm.assume(i1 %_71)
  %_12 = icmp samesign ult i64 %b.sroa.12.0.copyload, 2
  br i1 %_12, label %bb21, label %bb7

bb7:                                              ; preds = %bb4
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %a_bigrams)
  %_3.i.i.i.i = tail call align 8 ptr @llvm.threadlocal.address.p0(ptr @_RNvNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBa_11RandomState3new4KEYS0s_023___RUST_STD_INTERNAL_VAL)
  %_12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 16
  %43 = load i8, ptr %_12.i.i.i.i.i, align 8, !range !25, !noalias !472, !noundef !17
  %_4.i.i.i.i.i = trunc nuw i8 %43 to i1
  br i1 %_4.i.i.i.i.i, label %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim.exit_crit_edge.i.i, label %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim.exit.i.i, !prof !261

start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim.exit_crit_edge.i.i: ; preds = %bb7
  %_9.i.pre.i.i = load i64, ptr %_3.i.i.i.i, align 8, !noalias !481
  %.phi.trans.insert.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 8
  %_10.i.pre.i.i = load i64, ptr %.phi.trans.insert.i.i, align 8, !noalias !481
  br label %bb8

_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim.exit.i.i: ; preds = %bb7
; invoke std::sys::random::hashmap_random_keys
  %44 = invoke { i64, i64 } @_RNvNtNtCs5sEH5CPMdak_3std3sys6random19hashmap_random_keys()
          to label %.noexc unwind label %cleanup2

.noexc:                                           ; preds = %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim.exit.i.i
  %45 = extractvalue { i64, i64 } %44, 0
  %46 = extractvalue { i64, i64 } %44, 1
  %47 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 8
  store i64 %46, ptr %47, align 8, !noalias !482
  store i8 1, ptr %_12.i.i.i.i.i, align 8, !noalias !482
  br label %bb8

cleanup3.loopexit:                                ; preds = %bb16
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup3

cleanup3.loopexit.split-lp:                       ; preds = %bb11
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup3

cleanup3:                                         ; preds = %cleanup3.loopexit.split-lp, %cleanup3.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup3.loopexit ], [ %lpad.loopexit.split-lp, %cleanup3.loopexit.split-lp ]
  %a_bigrams.val = load ptr, ptr %a_bigrams, align 8
  %48 = getelementptr inbounds nuw i8, ptr %a_bigrams, i64 8
  %a_bigrams.val28 = load i64, ptr %48, align 8, !noundef !17
; call core::ptr::drop_in_place::<std::collections::hash::map::HashMap<(char, char), usize>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std11collections4hash3map7HashMapTccEjEECs6KJnav5oeQt_6strsim(ptr %a_bigrams.val, i64 %a_bigrams.val28) #26
  br label %bb26

bb8:                                              ; preds = %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim.exit_crit_edge.i.i, %.noexc
  %_72.1.pre-phi = phi i64 [ %_10.i.pre.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim.exit_crit_edge.i.i ], [ %46, %.noexc ]
  %_9.i.i.i = phi i64 [ %_9.i.pre.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim.exit_crit_edge.i.i ], [ %45, %.noexc ]
  %_4.i.i.i = add i64 %_9.i.i.i, 1
  store i64 %_4.i.i.i, ptr %_3.i.i.i.i, align 8, !noalias !481
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %a_bigrams, ptr noundef nonnull align 8 dereferenceable(32) @anon.5b7b4b03f51fa7c2fe1eb56c4b7bdd51.0, i64 32, i1 false)
  %_73.sroa.4.0.a_bigrams.sroa_idx = getelementptr inbounds nuw i8, ptr %a_bigrams, i64 32
  store i64 %_9.i.i.i, ptr %_73.sroa.4.0.a_bigrams.sroa_idx, align 8
  %_73.sroa.5.0.a_bigrams.sroa_idx = getelementptr inbounds nuw i8, ptr %a_bigrams, i64 40
  store i64 %_72.1.pre-phi, ptr %_73.sroa.5.0.a_bigrams.sroa_idx, align 8
  %_10.i = getelementptr inbounds nuw i8, ptr %a.sroa.7.0.copyload, i64 %a.sroa.12.0.copyload
  %_15.sroa.6.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 40
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %iter)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_15.sroa.6.0.iter.sroa_idx, i8 0, i64 16, i1 false)
  store ptr %a.sroa.7.0.copyload, ptr %iter, align 8
  %_15.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store ptr %_10.i, ptr %_15.sroa.2.0.iter.sroa_idx, align 8
  %_15.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %a.sroa.7.0.copyload, ptr %_15.sroa.3.0.iter.sroa_idx, align 8
  %_15.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store ptr %_10.i, ptr %_15.sroa.4.0.iter.sroa_idx, align 8
  %_15.sroa.5.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 32
  store i64 1, ptr %_15.sroa.5.0.iter.sroa_idx, align 8
; call <core::iter::adapters::zip::Zip<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>> as core::iter::adapters::zip::ZipImpl<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>>>::next
  %49 = call fastcc { i32, i32 } @_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipNtNtNtBb_3str4iter5CharsINtNtB7_4skip4SkipBW_EEINtB5_7ZipImplBW_B1k_E4nextCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 dereferenceable(56) %iter)
  %50 = extractvalue { i32, i32 } %49, 0
  %.not178 = icmp eq i32 %50, 1114112
  br i1 %.not178, label %bb14, label %bb11.lr.ph

bb11.lr.ph:                                       ; preds = %bb8
  %_85.sroa.4.0._80.sroa_idx = getelementptr inbounds nuw i8, ptr %_80, i64 4
  %_85.sroa.5.0._80.sroa_idx = getelementptr inbounds nuw i8, ptr %_80, i64 8
  %_85.sroa.6.0._80.sroa_idx = getelementptr inbounds nuw i8, ptr %_80, i64 16
  br label %bb11

bb11:                                             ; preds = %bb11.lr.ph, %bb13
  %51 = phi i32 [ %50, %bb11.lr.ph ], [ %97, %bb13 ]
  %52 = phi { i32, i32 } [ %49, %bb11.lr.ph ], [ %96, %bb13 ]
  %53 = extractvalue { i32, i32 } %52, 1
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_80)
; invoke <hashbrown::map::HashMap<(char, char), usize, std::hash::random::RandomState>>::rustc_entry
  invoke fastcc void @_RNvMNtCsh9QrOU9e3Ke_9hashbrown11rustc_entryINtNtB4_3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entryCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_80, ptr noalias noundef align 8 dereferenceable(48) %a_bigrams, i32 noundef %51, i32 noundef %53)
          to label %bb34 unwind label %cleanup3.loopexit.split-lp

bb14:                                             ; preds = %bb13, %bb8
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %iter)
  %_10.i149 = getelementptr inbounds nuw i8, ptr %b.sroa.7.0.copyload, i64 %b.sroa.12.0.copyload
  %_25.sroa.6.0.iter1.sroa_idx = getelementptr inbounds nuw i8, ptr %iter1, i64 40
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %iter1)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_25.sroa.6.0.iter1.sroa_idx, i8 0, i64 16, i1 false)
  store ptr %b.sroa.7.0.copyload, ptr %iter1, align 8
  %_25.sroa.2.0.iter1.sroa_idx = getelementptr inbounds nuw i8, ptr %iter1, i64 8
  store ptr %_10.i149, ptr %_25.sroa.2.0.iter1.sroa_idx, align 8
  %_25.sroa.3.0.iter1.sroa_idx = getelementptr inbounds nuw i8, ptr %iter1, i64 16
  store ptr %b.sroa.7.0.copyload, ptr %_25.sroa.3.0.iter1.sroa_idx, align 8
  %_25.sroa.4.0.iter1.sroa_idx = getelementptr inbounds nuw i8, ptr %iter1, i64 24
  store ptr %_10.i149, ptr %_25.sroa.4.0.iter1.sroa_idx, align 8
  %_25.sroa.5.0.iter1.sroa_idx = getelementptr inbounds nuw i8, ptr %iter1, i64 32
  store i64 1, ptr %_25.sroa.5.0.iter1.sroa_idx, align 8
; call <core::iter::adapters::zip::Zip<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>> as core::iter::adapters::zip::ZipImpl<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>>>::next
  %54 = call fastcc { i32, i32 } @_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipNtNtNtBb_3str4iter5CharsINtNtB7_4skip4SkipBW_EEINtB5_7ZipImplBW_B1k_E4nextCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 dereferenceable(56) %iter1)
  %55 = extractvalue { i32, i32 } %54, 0
  %.not12179 = icmp eq i32 %55, 1114112
  br i1 %.not12179, label %bb17, label %bb16.lr.ph

bb16.lr.ph:                                       ; preds = %bb14
  %56 = getelementptr inbounds nuw i8, ptr %_92, i64 8
  br label %bb16

bb16:                                             ; preds = %bb16.lr.ph, %bb42
  %57 = phi i32 [ %55, %bb16.lr.ph ], [ %74, %bb42 ]
  %58 = phi { i32, i32 } [ %54, %bb16.lr.ph ], [ %73, %bb42 ]
  %intersection_size.sroa.0.0180 = phi i64 [ 0, %bb16.lr.ph ], [ %intersection_size.sroa.0.1, %bb42 ]
  %59 = extractvalue { i32, i32 } %58, 1
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_92)
; invoke <hashbrown::map::HashMap<(char, char), usize, std::hash::random::RandomState>>::rustc_entry
  invoke fastcc void @_RNvMNtCsh9QrOU9e3Ke_9hashbrown11rustc_entryINtNtB4_3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entryCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 captures(none) dereferenceable(24) %_92, ptr noalias noundef align 8 dereferenceable(48) %a_bigrams, i32 noundef %57, i32 noundef %59)
          to label %bb39 unwind label %cleanup3.loopexit

bb17.loopexit:                                    ; preds = %bb42
  %60 = shl i64 %intersection_size.sroa.0.1, 1
  %61 = uitofp i64 %60 to double
  br label %bb17

bb17:                                             ; preds = %bb17.loopexit, %bb14
  %intersection_size.sroa.0.0.lcssa = phi double [ 0.000000e+00, %bb14 ], [ %61, %bb17.loopexit ]
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %iter1)
  %_39 = add nsw i64 %a.sroa.12.0.copyload, -2
  %_38 = add i64 %_39, %b.sroa.12.0.copyload
  %_37 = uitofp i64 %_38 to double
  %62 = fdiv double %intersection_size.sroa.0.0.lcssa, %_37
  %a_bigrams.val29 = load ptr, ptr %a_bigrams, align 8
  %63 = getelementptr inbounds nuw i8, ptr %a_bigrams, i64 8
  %a_bigrams.val30 = load i64, ptr %63, align 8, !noundef !17
  %64 = icmp eq i64 %a_bigrams.val30, 0
  br i1 %64, label %bb18, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb17
  %_9.i.i.i.i.i = shl i64 %a_bigrams.val30, 4
  %65 = add i64 %_9.i.i.i.i.i, 16
  %rhs5.i.i.i.i.i.i = add i64 %a_bigrams.val30, 9
  %_37.0.i.i.i.i.i.i = add i64 %rhs5.i.i.i.i.i.i, %65
  %_37.1.i.i.i.i.i.i = icmp uge i64 %_37.0.i.i.i.i.i.i, %65
  %_19.i.i.i.i.i.i = icmp ult i64 %_37.0.i.i.i.i.i.i, 9223372036854775801
  call void @llvm.assume(i1 %_37.1.i.i.i.i.i.i)
  call void @llvm.assume(i1 %_19.i.i.i.i.i.i)
  %66 = icmp ne ptr %a_bigrams.val29, null
  call void @llvm.assume(i1 %66)
  %67 = icmp eq i64 %_37.0.i.i.i.i.i.i, 0
  br i1 %67, label %bb18, label %bb1.i2.i.i.i.i.i

bb1.i2.i.i.i.i.i:                                 ; preds = %bb2.i.i.i.i.i
  %_17.i.i.i.i.i = sub nuw nsw i64 -16, %_9.i.i.i.i.i
  %ptr.i.i.i.i.i = getelementptr inbounds i8, ptr %a_bigrams.val29, i64 %_17.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i, i64 noundef %_37.0.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #23
  br label %bb18

bb18:                                             ; preds = %bb1.i2.i.i.i.i.i, %bb2.i.i.i.i.i, %bb17
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %a_bigrams)
  %68 = icmp eq i64 %b.sroa.0.0.copyload, 0
  br i1 %68, label %bb19, label %bb2.i.i.i4.i.i152

bb2.i.i.i4.i.i152:                                ; preds = %bb18
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %b.sroa.7.0.copyload, i64 noundef %b.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb19

bb19:                                             ; preds = %bb2.i.i.i4.i.i152, %bb18
  %69 = icmp eq i64 %a.sroa.0.0.copyload, 0
  br i1 %69, label %bb24, label %bb2.i.i.i4.i.i154

bb2.i.i.i4.i.i154:                                ; preds = %bb19
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %a.sroa.7.0.copyload, i64 noundef %a.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb24

bb24:                                             ; preds = %bb2.i.i.i4.i.i160, %bb22, %bb2.i.i.i4.i.i154, %bb19
  %_0.sroa.0.0 = phi double [ %62, %bb19 ], [ %62, %bb2.i.i.i4.i.i154 ], [ %_0.sroa.0.1, %bb22 ], [ %_0.sroa.0.1, %bb2.i.i.i4.i.i160 ]
  ret double %_0.sroa.0.0

bb39:                                             ; preds = %bb16
  %70 = load i32, ptr %_92, align 8, !range !485, !noundef !17
  %.not13 = icmp eq i32 %70, 1114112
  br i1 %.not13, label %bb41, label %bb40

bb40:                                             ; preds = %bb39
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_92)
  br label %bb42

bb41:                                             ; preds = %bb39
  %_95.0 = load ptr, ptr %56, align 8, !nonnull !17, !noundef !17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_92)
  %_98 = getelementptr inbounds i8, ptr %_95.0, i64 -8
  %_102 = load i64, ptr %_98, align 8, !noundef !17
  %_101.not = icmp eq i64 %_102, 0
  br i1 %_101.not, label %bb42, label %bb43

bb43:                                             ; preds = %bb41
  %71 = add i64 %_102, -1
  store i64 %71, ptr %_98, align 8
  %72 = add i64 %intersection_size.sroa.0.0180, 1
  br label %bb42

bb42:                                             ; preds = %bb43, %bb41, %bb40
  %intersection_size.sroa.0.1 = phi i64 [ %intersection_size.sroa.0.0180, %bb40 ], [ %72, %bb43 ], [ %intersection_size.sroa.0.0180, %bb41 ]
; call <core::iter::adapters::zip::Zip<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>> as core::iter::adapters::zip::ZipImpl<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>>>::next
  %73 = call fastcc { i32, i32 } @_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipNtNtNtBb_3str4iter5CharsINtNtB7_4skip4SkipBW_EEINtB5_7ZipImplBW_B1k_E4nextCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 dereferenceable(56) %iter1)
  %74 = extractvalue { i32, i32 } %73, 0
  %.not12 = icmp eq i32 %74, 1114112
  br i1 %.not12, label %bb17.loopexit, label %bb16

bb34:                                             ; preds = %bb11
  %75 = load i32, ptr %_80, align 8, !range !485, !noundef !17
  %.not14 = icmp eq i32 %75, 1114112
  br i1 %.not14, label %bb3.i157, label %bb2.i156

bb2.i156:                                         ; preds = %bb34
  %_85.sroa.4.0.copyload = load i32, ptr %_85.sroa.4.0._80.sroa_idx, align 4
  %_85.sroa.5.0.copyload = load ptr, ptr %_85.sroa.5.0._80.sroa_idx, align 8
  %_85.sroa.6.0.copyload = load i64, ptr %_85.sroa.6.0._80.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_80)
  call void @llvm.experimental.noalias.scope.decl(metadata !486)
  %self.val.i.i = load ptr, ptr %_85.sroa.5.0.copyload, align 8, !alias.scope !486, !noalias !489, !nonnull !17, !noundef !17
  %76 = getelementptr inbounds nuw i8, ptr %_85.sroa.5.0.copyload, i64 8
  %self.val2.i.i = load i64, ptr %76, align 8, !alias.scope !486, !noalias !489, !noundef !17
  %probe_seq.sroa.0.01.i.i.i = and i64 %self.val2.i.i, %_85.sroa.6.0.copyload
  %_182.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %probe_seq.sroa.0.01.i.i.i
  %tmp.sroa.0.0.copyload.i.i3.i.i.i = load <8 x i8>, ptr %_182.i.i.i, align 1, !noalias !493
  %.lobit.i.i.i4.i.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i3.i.i.i, splat (i8 7)
  %77 = bitcast <8 x i8> %.lobit.i.i.i4.i.i.i to i64
  %.not.i.not5.i.i.i = icmp eq i64 %77, 0
  br i1 %.not.i.not5.i.i.i, label %bb6.i.i.i, label %bb9.i.i.i, !prof !221

bb6.i.i.i:                                        ; preds = %bb2.i156, %bb6.i.i.i
  %probe_seq.sroa.0.06.i.i.i = phi i64 [ %probe_seq.sroa.0.0.i.i.i, %bb6.i.i.i ], [ %probe_seq.sroa.0.01.i.i.i, %bb2.i156 ]
  %78 = phi i64 [ %79, %bb6.i.i.i ], [ 0, %bb2.i156 ]
  %79 = add i64 %78, 8
  %80 = add i64 %79, %probe_seq.sroa.0.06.i.i.i
  %probe_seq.sroa.0.0.i.i.i = and i64 %80, %self.val2.i.i
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %probe_seq.sroa.0.0.i.i.i
  %tmp.sroa.0.0.copyload.i.i.i.i.i = load <8 x i8>, ptr %_18.i.i.i, align 1, !noalias !493
  %.lobit.i.i.i.i.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i, splat (i8 7)
  %81 = bitcast <8 x i8> %.lobit.i.i.i.i.i.i to i64
  %.not.i.not.i.i.i = icmp eq i64 %81, 0
  br i1 %.not.i.not.i.i.i, label %bb6.i.i.i, label %bb9.i.i.i, !prof !222

bb9.i.i.i:                                        ; preds = %bb6.i.i.i, %bb2.i156
  %probe_seq.sroa.0.0.lcssa.i.i.i = phi i64 [ %probe_seq.sroa.0.01.i.i.i, %bb2.i156 ], [ %probe_seq.sroa.0.0.i.i.i, %bb6.i.i.i ]
  %.lcssa.i.i.i = phi i64 [ %77, %bb2.i156 ], [ %81, %bb6.i.i.i ]
  %82 = call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %.lcssa.i.i.i, i1 true)
  %_187.i.i.i.i = lshr i64 %82, 3
  %_10.i.i.i.i = add i64 %_187.i.i.i.i, %probe_seq.sroa.0.0.lcssa.i.i.i
  %_9.i.i.i.i = and i64 %_10.i.i.i.i, %self.val2.i.i
  %_10.i5.i.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %_9.i.i.i.i
  %_14.i.i.i.i = load i8, ptr %_10.i5.i.i.i, align 1, !noalias !498, !noundef !17
  %83 = icmp sgt i8 %_14.i.i.i.i, -1
  br i1 %83, label %bb3.i.i.i.i, label %_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim.exit.i, !prof !9

bb3.i.i.i.i:                                      ; preds = %bb9.i.i.i
  %tmp.sroa.0.0.copyload.i.i.i.i.i.i = load <8 x i8>, ptr %self.val.i.i, align 1, !noalias !499
  %.lobit.i.i.i6.i.i.i = ashr <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i.i, splat (i8 7)
  %84 = bitcast <8 x i8> %.lobit.i.i.i6.i.i.i to i64
  %85 = icmp ne i64 %84, 0
  call void @llvm.assume(i1 %85)
  %86 = call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %84, i1 true)
  %_266.i.i.i.i = lshr i64 %86, 3
  %_13.phi.trans.insert.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %_266.i.i.i.i
  %old_ctrl.pre.i.i = load i8, ptr %_13.phi.trans.insert.i.i, align 1, !noalias !498
  br label %_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim.exit.i

_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim.exit.i: ; preds = %bb3.i.i.i.i, %bb9.i.i.i
  %old_ctrl.i.i = phi i8 [ %old_ctrl.pre.i.i, %bb3.i.i.i.i ], [ %_14.i.i.i.i, %bb9.i.i.i ]
  %index.sroa.0.0.i.i.i.i = phi i64 [ %_266.i.i.i.i, %bb3.i.i.i.i ], [ %_9.i.i.i.i, %bb9.i.i.i ]
  %_13.i.i = getelementptr inbounds nuw i8, ptr %self.val.i.i, i64 %index.sroa.0.0.i.i.i.i
  %_17.i.i = lshr i64 %_85.sroa.6.0.copyload, 57
  %_18.i.i = trunc nuw nsw i64 %_17.i.i to i8
  %_22.i.i = add i64 %index.sroa.0.0.i.i.i.i, -8
  %_21.i.i = and i64 %_22.i.i, %self.val2.i.i
  store i8 %_18.i.i, ptr %_13.i.i, align 1, !noalias !498
  %87 = getelementptr i8, ptr %self.val.i.i, i64 %_21.i.i
  %_29.i.i = getelementptr i8, ptr %87, i64 8
  store i8 %_18.i.i, ptr %_29.i.i, align 1, !noalias !498
  %_42.i.i = sub nsw i64 0, %index.sroa.0.0.i.i.i.i
  %88 = getelementptr inbounds { { i32, i32 }, i64 }, ptr %self.val.i.i, i64 %_42.i.i
  %_45.i.i = and i8 %old_ctrl.i.i, 1
  %_6.i.i = zext nneg i8 %_45.i.i to i64
  %89 = getelementptr inbounds nuw i8, ptr %_85.sroa.5.0.copyload, i64 16
  %90 = getelementptr inbounds i8, ptr %88, i64 -16
  store i32 %75, ptr %90, align 8, !noalias !504
  %_7.sroa.4.0..sroa_idx.i = getelementptr inbounds i8, ptr %88, i64 -12
  store i32 %_85.sroa.4.0.copyload, ptr %_7.sroa.4.0..sroa_idx.i, align 4, !noalias !504
  %_7.sroa.5.0..sroa_idx.i = getelementptr inbounds i8, ptr %88, i64 -8
  store i64 0, ptr %_7.sroa.5.0..sroa_idx.i, align 8, !noalias !504
  %91 = load <2 x i64>, ptr %89, align 8, !alias.scope !486, !noalias !489
  %92 = insertelement <2 x i64> <i64 poison, i64 -1>, i64 %_6.i.i, i64 0
  %93 = sub <2 x i64> %91, %92
  store <2 x i64> %93, ptr %89, align 8, !alias.scope !486, !noalias !489
  br label %bb13

bb3.i157:                                         ; preds = %bb34
  %_83.0 = load ptr, ptr %_85.sroa.5.0._80.sroa_idx, align 8, !nonnull !17, !noundef !17
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_80)
  br label %bb13

bb13:                                             ; preds = %bb3.i157, %_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim.exit.i
  %bucket.pn.i = phi ptr [ %88, %_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim.exit.i ], [ %_83.0, %bb3.i157 ]
  %_0.sroa.0.0.i = getelementptr inbounds i8, ptr %bucket.pn.i, i64 -8
  %94 = load i64, ptr %_0.sroa.0.0.i, align 8, !noundef !17
  %95 = add i64 %94, 1
  store i64 %95, ptr %_0.sroa.0.0.i, align 8
; call <core::iter::adapters::zip::Zip<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>> as core::iter::adapters::zip::ZipImpl<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>>>::next
  %96 = call fastcc { i32, i32 } @_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipNtNtNtBb_3str4iter5CharsINtNtB7_4skip4SkipBW_EEINtB5_7ZipImplBW_B1k_E4nextCs6KJnav5oeQt_6strsim(ptr noalias noundef align 8 dereferenceable(56) %iter)
  %97 = extractvalue { i32, i32 } %96, 0
  %.not = icmp eq i32 %97, 1114112
  br i1 %.not, label %bb14, label %bb11

bb21:                                             ; preds = %bb2, %bb4, %bb31
  %_0.sroa.0.1 = phi double [ 1.000000e+00, %bb31 ], [ 0.000000e+00, %bb4 ], [ 0.000000e+00, %bb2 ]
  %98 = icmp eq i64 %b.sroa.0.0.copyload, 0
  br i1 %98, label %bb22, label %bb2.i.i.i4.i.i158

bb2.i.i.i4.i.i158:                                ; preds = %bb21
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %b.sroa.7.0.copyload, i64 noundef %b.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb22

bb22:                                             ; preds = %bb2.i.i.i4.i.i158, %bb21
  %99 = icmp eq i64 %a.sroa.0.0.copyload, 0
  br i1 %99, label %bb24, label %bb2.i.i.i4.i.i160

bb2.i.i.i4.i.i160:                                ; preds = %bb22
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %a.sroa.7.0.copyload, i64 noundef %a.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #23
  br label %bb24
}

; strsim::damerau_levenshtein
; Function Attrs: uwtable
define noundef i64 @_RNvCs6KJnav5oeQt_6strsim19damerau_levenshtein(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 {
start:
  %_6.i = icmp ult i64 %a.1, 32
  br i1 %_6.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %start
; call core::str::count::do_count_chars
  %0 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

bb2.i:                                            ; preds = %start
; call core::str::count::char_count_general_case
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit: ; preds = %bb3.i, %bb2.i
  %_0.sroa.0.0.i = phi i64 [ %1, %bb2.i ], [ %0, %bb3.i ]
  %_6.i1 = icmp ult i64 %b.1, 32
  br i1 %_6.i1, label %bb2.i4, label %bb3.i2

bb3.i2:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::do_count_chars
  %2 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

bb2.i4:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::char_count_general_case
  %3 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5: ; preds = %bb3.i2, %bb2.i4
  %_0.sroa.0.0.i3 = phi i64 [ %3, %bb2.i4 ], [ %2, %bb3.i2 ]
  %_30 = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  %_14 = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
; call strsim::damerau_levenshtein_impl::<core::str::iter::Chars, core::str::iter::Chars>
  %_0 = tail call fastcc noundef i64 @_RINvCs6KJnav5oeQt_6strsim24damerau_levenshtein_implNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBN_EB2_(ptr noundef nonnull %a.0, ptr noundef %_14, i64 noundef %_0.sroa.0.0.i, ptr noundef nonnull %b.0, ptr noundef %_30, i64 noundef %_0.sroa.0.0.i3)
  ret i64 %_0
}

; strsim::normalized_levenshtein
; Function Attrs: uwtable
define noundef double @_RNvCs6KJnav5oeQt_6strsim22normalized_levenshtein(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = or i64 %b.1, %a.1
  %or.cond = icmp eq i64 %0, 0
  br i1 %or.cond, label %bb9, label %bb4

bb4:                                              ; preds = %start
; call strsim::levenshtein
  %_5 = tail call noundef i64 @_RNvCs6KJnav5oeQt_6strsim11levenshtein(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  %_4 = uitofp i64 %_5 to double
  %_6.i = icmp ult i64 %a.1, 32
  br i1 %_6.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %bb4
; call core::str::count::do_count_chars
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

bb2.i:                                            ; preds = %bb4
; call core::str::count::char_count_general_case
  %2 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit: ; preds = %bb3.i, %bb2.i
  %_0.sroa.0.0.i = phi i64 [ %2, %bb2.i ], [ %1, %bb3.i ]
  %_6.i1 = icmp ult i64 %b.1, 32
  br i1 %_6.i1, label %bb2.i4, label %bb3.i2

bb3.i2:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::do_count_chars
  %3 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

bb2.i4:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::char_count_general_case
  %4 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5: ; preds = %bb3.i2, %bb2.i4
  %_0.sroa.0.0.i3 = phi i64 [ %4, %bb2.i4 ], [ %3, %bb3.i2 ]
  %_0.sroa.0.0.i6 = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i3, i64 %_0.sroa.0.0.i)
  %_6 = uitofp i64 %_0.sroa.0.0.i6 to double
  %_3 = fdiv double %_4, %_6
  %5 = fsub double 1.000000e+00, %_3
  br label %bb9

bb9:                                              ; preds = %start, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5
  %_0.sroa.0.0 = phi double [ %5, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5 ], [ 1.000000e+00, %start ]
  ret double %_0.sroa.0.0
}

; strsim::normalized_damerau_levenshtein
; Function Attrs: uwtable
define noundef double @_RNvCs6KJnav5oeQt_6strsim30normalized_damerau_levenshtein(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = or i64 %b.1, %a.1
  %or.cond = icmp eq i64 %0, 0
  br i1 %or.cond, label %bb8, label %bb4

bb4:                                              ; preds = %start
  %_21 = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  %_6.i = icmp ult i64 %a.1, 32
  br i1 %_6.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %bb4
; call core::str::count::do_count_chars
  %1 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

bb2.i:                                            ; preds = %bb4
; call core::str::count::char_count_general_case
  %2 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit: ; preds = %bb3.i, %bb2.i
  %_0.sroa.0.0.i = phi i64 [ %2, %bb2.i ], [ %1, %bb3.i ]
  %_30 = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  %_6.i1 = icmp ult i64 %b.1, 32
  br i1 %_6.i1, label %bb2.i4, label %bb3.i2

bb3.i2:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::do_count_chars
  %3 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

bb2.i4:                                           ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit
; call core::str::count::char_count_general_case
  %4 = tail call noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1)
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5: ; preds = %bb3.i2, %bb2.i4
  %_0.sroa.0.0.i3 = phi i64 [ %4, %bb2.i4 ], [ %3, %bb3.i2 ]
; call strsim::damerau_levenshtein_impl::<core::str::iter::Chars, core::str::iter::Chars>
  %dist = tail call fastcc noundef i64 @_RINvCs6KJnav5oeQt_6strsim24damerau_levenshtein_implNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBN_EB2_(ptr noundef nonnull %a.0, ptr noundef %_21, i64 noundef %_0.sroa.0.0.i, ptr noundef nonnull %b.0, ptr noundef %_30, i64 noundef %_0.sroa.0.0.i3)
  %_11 = uitofp i64 %dist to double
  %_0.sroa.0.0.i6 = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i3, i64 %_0.sroa.0.0.i)
  %_12 = uitofp i64 %_0.sroa.0.0.i6 to double
  %_10 = fdiv double %_11, %_12
  %5 = fsub double 1.000000e+00, %_10
  br label %bb8

bb8:                                              ; preds = %start, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5
  %_0.sroa.0.0 = phi double [ %5, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator5count.exit5 ], [ 1.000000e+00, %start ]
  ret double %_0.sroa.0.0
}

; strsim::jaro
; Function Attrs: uwtable
define noundef double @_RNvCs6KJnav5oeQt_6strsim4jaro(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %b.0, i64 noundef %b.1) unnamed_addr #0 {
start:
; call strsim::generic_jaro::<strsim::StringWrapper, strsim::StringWrapper, char, char>
  %_0 = tail call fastcc noundef double @_RINvCs6KJnav5oeQt_6strsim12generic_jaroNtB2_13StringWrapperBB_ccEB2_(ptr nonnull %a.0, i64 %a.1, ptr nonnull %b.0, i64 %b.1)
  ret double %_0
}

; strsim::hamming
; Function Attrs: nofree norecurse nosync nounwind memory(argmem: read, inaccessiblemem: write) uwtable
define { i64, i64 } @_RNvCs6KJnav5oeQt_6strsim7hamming(ptr noalias noundef nonnull readonly align 1 captures(address) %a.0, i64 noundef %a.1, ptr noalias noundef nonnull readonly align 1 captures(address) %b.0, i64 noundef %b.1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %_10 = getelementptr inbounds nuw i8, ptr %a.0, i64 %a.1
  %_19 = getelementptr inbounds nuw i8, ptr %b.0, i64 %b.1
  br label %bb3.i

bb3.i:                                            ; preds = %bb10.i, %start
  %itb.sroa.0.0.i = phi ptr [ %b.0, %start ], [ %itb.sroa.0.1.i, %bb10.i ]
  %ita.sroa.0.0.i = phi ptr [ %a.0, %start ], [ %ita.sroa.0.169.i, %bb10.i ]
  %count.sroa.0.0.i = phi i64 [ 0, %start ], [ %spec.select.i, %bb10.i ]
  %_6.i.i.not.i.i = icmp eq ptr %ita.sroa.0.0.i, %_10
  br i1 %_6.i.i.not.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb3.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %ita.sroa.0.0.i, i64 1
  %x.i.i.i = load i8, ptr %ita.sroa.0.0.i, align 1, !noalias !505, !noundef !17
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_10
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %ita.sroa.0.0.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !505, !noundef !17
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %0 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i

bb3.i.i.i:                                        ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_10
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %ita.sroa.0.0.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !505, !noundef !17
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %1 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i

bb8.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_10
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %ita.sroa.0.0.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !505, !noundef !17
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %2 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i: ; preds = %bb8.i.i.i, %bb6.i.i.i, %bb3.i.i.i, %bb4.i.i.i
  %ita.sroa.0.1.i = phi ptr [ %_16.i.i.i.i, %bb3.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i12.i.i.i, %bb4.i.i.i ]
  %spec.select.i.i = phi i32 [ %_7.i.i.i, %bb3.i.i.i ], [ %2, %bb8.i.i.i ], [ %1, %bb6.i.i.i ], [ %0, %bb4.i.i.i ]
  %_6.i.i.not.i18.i = icmp eq ptr %itb.sroa.0.0.i, %_19
  br i1 %_6.i.i.not.i18.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.thread.i, label %bb14.i.i19.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i: ; preds = %bb3.i
  %_6.i.i.not.i1868.i = icmp eq ptr %itb.sroa.0.0.i, %_19
  br i1 %_6.i.i.not.i1868.i, label %_RINvCs6KJnav5oeQt_6strsim15generic_hammingNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBE_ccEB2_.exit, label %bb14.i.i19.i

bb14.i.i19.i:                                     ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i
  %spec.select.i70.i = phi i32 [ 1114112, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i ], [ %spec.select.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %ita.sroa.0.169.i = phi ptr [ %_10, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i ], [ %ita.sroa.0.1.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i ]
  %_16.i.i.i20.i = getelementptr inbounds nuw i8, ptr %itb.sroa.0.0.i, i64 1
  %x.i.i21.i = load i8, ptr %itb.sroa.0.0.i, align 1, !noalias !510, !noundef !17
  %_6.i.i22.i = icmp sgt i8 %x.i.i21.i, -1
  br i1 %_6.i.i22.i, label %bb3.i.i54.i, label %bb4.i.i23.i

bb4.i.i23.i:                                      ; preds = %bb14.i.i19.i
  %_30.i.i24.i = and i8 %x.i.i21.i, 31
  %init.i.i25.i = zext nneg i8 %_30.i.i24.i to i32
  %_6.i10.i.i26.i = icmp ne ptr %_16.i.i.i20.i, %_19
  tail call void @llvm.assume(i1 %_6.i10.i.i26.i)
  %_16.i12.i.i27.i = getelementptr inbounds nuw i8, ptr %itb.sroa.0.0.i, i64 2
  %y.i.i28.i = load i8, ptr %_16.i.i.i20.i, align 1, !noalias !510, !noundef !17
  %_33.i.i29.i = shl nuw nsw i32 %init.i.i25.i, 6
  %_35.i.i30.i = and i8 %y.i.i28.i, 63
  %_34.i.i31.i = zext nneg i8 %_35.i.i30.i to i32
  %3 = or disjoint i32 %_33.i.i29.i, %_34.i.i31.i
  %_13.i.i32.i = icmp samesign ugt i8 %x.i.i21.i, -33
  br i1 %_13.i.i32.i, label %bb6.i.i34.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i

bb3.i.i54.i:                                      ; preds = %bb14.i.i19.i
  %_7.i.i55.i = zext nneg i8 %x.i.i21.i to i32
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i

bb6.i.i34.i:                                      ; preds = %bb4.i.i23.i
  %_6.i17.i.i35.i = icmp ne ptr %_16.i12.i.i27.i, %_19
  tail call void @llvm.assume(i1 %_6.i17.i.i35.i)
  %_16.i19.i.i36.i = getelementptr inbounds nuw i8, ptr %itb.sroa.0.0.i, i64 3
  %z.i.i37.i = load i8, ptr %_16.i12.i.i27.i, align 1, !noalias !510, !noundef !17
  %_38.i.i38.i = shl nuw nsw i32 %_34.i.i31.i, 6
  %_40.i.i39.i = and i8 %z.i.i37.i, 63
  %_39.i.i40.i = zext nneg i8 %_40.i.i39.i to i32
  %y_z.i.i41.i = or disjoint i32 %_38.i.i38.i, %_39.i.i40.i
  %_20.i.i42.i = shl nuw nsw i32 %init.i.i25.i, 12
  %4 = or disjoint i32 %y_z.i.i41.i, %_20.i.i42.i
  %_21.i.i43.i = icmp samesign ugt i8 %x.i.i21.i, -17
  br i1 %_21.i.i43.i, label %bb8.i.i44.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i

bb8.i.i44.i:                                      ; preds = %bb6.i.i34.i
  %_6.i24.i.i45.i = icmp ne ptr %_16.i19.i.i36.i, %_19
  tail call void @llvm.assume(i1 %_6.i24.i.i45.i)
  %_16.i26.i.i46.i = getelementptr inbounds nuw i8, ptr %itb.sroa.0.0.i, i64 4
  %w.i.i47.i = load i8, ptr %_16.i19.i.i36.i, align 1, !noalias !510, !noundef !17
  %_26.i.i48.i = shl nuw nsw i32 %init.i.i25.i, 18
  %_25.i.i49.i = and i32 %_26.i.i48.i, 1835008
  %_43.i.i50.i = shl nuw nsw i32 %y_z.i.i41.i, 6
  %_45.i.i51.i = and i8 %w.i.i47.i, 63
  %_44.i.i52.i = zext nneg i8 %_45.i.i51.i to i32
  %_27.i.i53.i = or disjoint i32 %_43.i.i50.i, %_44.i.i52.i
  %5 = or disjoint i32 %_27.i.i53.i, %_25.i.i49.i
  br label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i: ; preds = %bb8.i.i44.i, %bb6.i.i34.i, %bb3.i.i54.i, %bb4.i.i23.i
  %itb.sroa.0.1.i = phi ptr [ %_16.i.i.i20.i, %bb3.i.i54.i ], [ %_16.i26.i.i46.i, %bb8.i.i44.i ], [ %_16.i19.i.i36.i, %bb6.i.i34.i ], [ %_16.i12.i.i27.i, %bb4.i.i23.i ]
  %spec.select.i33.i = phi i32 [ %_7.i.i55.i, %bb3.i.i54.i ], [ %5, %bb8.i.i44.i ], [ %4, %bb6.i.i34.i ], [ %3, %bb4.i.i23.i ]
  %.not.i = icmp eq i32 %spec.select.i70.i, 1114112
  br i1 %.not.i, label %bb8.i, label %bb7.i

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.thread.i: ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.i
  %.not59.i = icmp ne i32 %spec.select.i.i, 1114112
  %spec.select = select i1 %.not59.i, i64 undef, i64 %count.sroa.0.0.i
  br label %_RINvCs6KJnav5oeQt_6strsim15generic_hammingNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBE_ccEB2_.exit

bb7.i:                                            ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i
  %.not12.i = icmp eq i32 %spec.select.i33.i, 1114112
  br i1 %.not12.i, label %_RINvCs6KJnav5oeQt_6strsim15generic_hammingNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBE_ccEB2_.exit, label %bb10.i

bb8.i:                                            ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.i
  %6 = icmp ne i32 %spec.select.i33.i, 1114112
  br label %_RINvCs6KJnav5oeQt_6strsim15generic_hammingNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBE_ccEB2_.exit

bb10.i:                                           ; preds = %bb7.i
  %_0.i.not.i = icmp ne i32 %spec.select.i70.i, %spec.select.i33.i
  %7 = zext i1 %_0.i.not.i to i64
  %spec.select.i = add i64 %count.sroa.0.0.i, %7
  br label %bb3.i

_RINvCs6KJnav5oeQt_6strsim15generic_hammingNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsBE_ccEB2_.exit: ; preds = %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i, %bb7.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.thread.i, %bb8.i
  %_0.sroa.3.0.i = phi i64 [ %count.sroa.0.0.i, %bb8.i ], [ %spec.select, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.thread.i ], [ %count.sroa.0.0.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i ], [ undef, %bb7.i ]
  %_0.sroa.0.0.i.shrunk = phi i1 [ %6, %bb8.i ], [ %.not59.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit56.thread.i ], [ false, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit.thread.i ], [ true, %bb7.i ]
  %_0.sroa.0.0.i = zext i1 %_0.sroa.0.0.i.shrunk to i64
  %8 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0.i, 0
  %9 = insertvalue { i64, i64 } %8, i64 %_0.sroa.3.0.i, 1
  ret { i64, i64 } %9
}

; <hashbrown::map::HashMap<(char, char), usize, std::hash::random::RandomState>>::rustc_entry
; Function Attrs: uwtable
define internal fastcc void @_RNvMNtCsh9QrOU9e3Ke_9hashbrown11rustc_entryINtNtB4_3map7HashMapTccEjNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entryCs6KJnav5oeQt_6strsim(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull align 8 dereferenceable(48) %self, i32 noundef range(i32 0, 1114112) %0, i32 noundef range(i32 0, 1114112) %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %hash_builder = getelementptr inbounds nuw i8, ptr %self, i64 32
  %hash_builder.val = load i64, ptr %hash_builder, align 8, !noundef !17
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %hash_builder.val2 = load i64, ptr %2, align 8, !noundef !17
  %3 = zext nneg i32 %1 to i64
  %.pre.i.i77.i = zext nneg i32 %0 to i64
  %4 = xor i64 %hash_builder.val, 7816392313619706465
  %5 = xor i64 %hash_builder.val2, 7237128888997146477
  %6 = xor i64 %hash_builder.val, 8317987319222330741
  %_7.i.i.i.i.i = shl nuw nsw i64 %3, 32
  %7 = or disjoint i64 %_7.i.i.i.i.i, %.pre.i.i77.i
  %8 = xor i64 %7, %hash_builder.val2
  %9 = xor i64 %8, 8387220255154660723
  %_2.i.i.i.i.i.i = add i64 %5, %6
  %_5.i33.i.i.i.i.i = add i64 %9, %4
  %10 = tail call noundef i64 @llvm.fshl.i64(i64 %5, i64 %5, i64 13)
  %11 = xor i64 %10, %_2.i.i.i.i.i.i
  %12 = tail call noundef i64 @llvm.fshl.i64(i64 %9, i64 %9, i64 16)
  %13 = xor i64 %_5.i33.i.i.i.i.i, %12
  %14 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i, i64 32)
  %_16.i34.i.i.i.i.i = add i64 %11, %_5.i33.i.i.i.i.i
  %_19.i35.i.i.i.i.i = add i64 %13, %14
  %15 = tail call noundef i64 @llvm.fshl.i64(i64 %11, i64 %11, i64 17)
  %16 = xor i64 %_16.i34.i.i.i.i.i, %15
  %17 = tail call noundef i64 @llvm.fshl.i64(i64 %13, i64 %13, i64 21)
  %18 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i34.i.i.i.i.i, i64 %_16.i34.i.i.i.i.i, i64 32)
  %19 = xor i64 %_19.i35.i.i.i.i.i, %7
  %20 = xor i64 %17, %_19.i35.i.i.i.i.i
  %21 = xor i64 %20, 576460752303423488
  %_2.i.i.i.i = add i64 %19, %16
  %_5.i.i.i.i = add i64 %21, %18
  %22 = tail call noundef i64 @llvm.fshl.i64(i64 %16, i64 %16, i64 13)
  %23 = xor i64 %_2.i.i.i.i, %22
  %24 = tail call noundef i64 @llvm.fshl.i64(i64 %21, i64 %21, i64 16)
  %25 = xor i64 %24, %_5.i.i.i.i
  %26 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i, i64 %_2.i.i.i.i, i64 32)
  %_16.i.i.i.i = add i64 %_5.i.i.i.i, %23
  %_19.i.i.i.i = add i64 %25, %26
  %27 = tail call noundef i64 @llvm.fshl.i64(i64 %23, i64 %23, i64 17)
  %28 = xor i64 %_16.i.i.i.i, %27
  %29 = tail call noundef i64 @llvm.fshl.i64(i64 %25, i64 %25, i64 21)
  %30 = xor i64 %29, %_19.i.i.i.i
  %31 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i, i64 %_16.i.i.i.i, i64 32)
  %32 = xor i64 %_19.i.i.i.i, 576460752303423488
  %33 = xor i64 %31, 255
  %_2.i3.i.i.i = add i64 %32, %28
  %_5.i6.i.i.i = add i64 %30, %33
  %34 = tail call noundef i64 @llvm.fshl.i64(i64 %28, i64 %28, i64 13)
  %35 = xor i64 %_2.i3.i.i.i, %34
  %36 = tail call noundef i64 @llvm.fshl.i64(i64 %30, i64 %30, i64 16)
  %37 = xor i64 %36, %_5.i6.i.i.i
  %38 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i, i64 %_2.i3.i.i.i, i64 32)
  %_16.i7.i.i.i = add i64 %35, %_5.i6.i.i.i
  %_19.i8.i.i.i = add i64 %37, %38
  %39 = tail call noundef i64 @llvm.fshl.i64(i64 %35, i64 %35, i64 17)
  %40 = xor i64 %_16.i7.i.i.i, %39
  %41 = tail call noundef i64 @llvm.fshl.i64(i64 %37, i64 %37, i64 21)
  %42 = xor i64 %41, %_19.i8.i.i.i
  %43 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i, i64 %_16.i7.i.i.i, i64 32)
  %_30.i.i.i.i = add i64 %40, %_19.i8.i.i.i
  %_33.i.i.i.i = add i64 %42, %43
  %44 = tail call noundef i64 @llvm.fshl.i64(i64 %40, i64 %40, i64 13)
  %45 = xor i64 %44, %_30.i.i.i.i
  %46 = tail call noundef i64 @llvm.fshl.i64(i64 %42, i64 %42, i64 16)
  %47 = xor i64 %46, %_33.i.i.i.i
  %48 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i, i64 %_30.i.i.i.i, i64 32)
  %_44.i.i.i.i = add i64 %45, %_33.i.i.i.i
  %_47.i.i.i.i = add i64 %47, %48
  %49 = tail call noundef i64 @llvm.fshl.i64(i64 %45, i64 %45, i64 17)
  %50 = xor i64 %49, %_44.i.i.i.i
  %51 = tail call noundef i64 @llvm.fshl.i64(i64 %47, i64 %47, i64 21)
  %52 = xor i64 %51, %_47.i.i.i.i
  %53 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i, i64 %_44.i.i.i.i, i64 32)
  %_58.i.i.i.i = add i64 %50, %_47.i.i.i.i
  %_61.i.i.i.i = add i64 %52, %53
  %54 = tail call noundef i64 @llvm.fshl.i64(i64 %50, i64 %50, i64 13)
  %55 = xor i64 %54, %_58.i.i.i.i
  %56 = tail call noundef i64 @llvm.fshl.i64(i64 %52, i64 %52, i64 16)
  %57 = xor i64 %56, %_61.i.i.i.i
  %_72.i.i.i.i = add i64 %55, %_61.i.i.i.i
  %58 = tail call noundef i64 @llvm.fshl.i64(i64 %55, i64 %55, i64 17)
  %59 = tail call noundef i64 @llvm.fshl.i64(i64 %57, i64 %57, i64 21)
  %60 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i, i64 %_72.i.i.i.i, i64 32)
  %61 = xor i64 %59, %58
  %62 = xor i64 %61, %60
  %_0.i.i.i = xor i64 %62, %_72.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !515)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !518)
  %_24.i.i = lshr i64 %_0.i.i.i, 57
  %_25.i.i = trunc nuw nsw i64 %_24.i.i to i8
  %63 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_29.i.i = load i64, ptr %63, align 8, !alias.scope !521, !noalias !522, !noundef !17
  %_32.i.i = load ptr, ptr %self, align 8, !alias.scope !521, !noalias !522, !nonnull !17, !noundef !17
  %64 = insertelement <1 x i8> poison, i8 %_25.i.i, i64 0
  %65 = shufflevector <1 x i8> %64, <1 x i8> poison, <8 x i32> zeroinitializer
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb16.i.i, %start
  %probe_seq.sroa.9.0.i.i = phi i64 [ 0, %start ], [ %77, %bb16.i.i ]
  %hash.pn.i = phi i64 [ %_0.i.i.i, %start ], [ %78, %bb16.i.i ]
  %probe_seq.sroa.0.0.i.i = and i64 %hash.pn.i, %_29.i.i
  %_30.i.i = getelementptr inbounds nuw i8, ptr %_32.i.i, i64 %probe_seq.sroa.0.0.i.i
  %tmp.sroa.0.0.copyload.i.i.i = load <8 x i8>, ptr %_30.i.i, align 1, !noalias !524
  %66 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i.i, %65
  %67 = sext <8 x i1> %66 to <8 x i8>
  %68 = bitcast <8 x i8> %67 to i64
  %_36.i.i = and i64 %68, -9187201950435737472
  %.not.i.not10.i = icmp eq i64 %_36.i.i, 0
  br i1 %.not.i.not10.i, label %bb9.i.i, label %bb8.i.i

bb8.i.i:                                          ; preds = %bb1.i.i, %bb13.i.i
  %iter.sroa.0.0.i11.i = phi i64 [ %_53.i.i, %bb13.i.i ], [ %_36.i.i, %bb1.i.i ]
  %69 = tail call range(i64 1, 65) i64 @llvm.cttz.i64(i64 %iter.sroa.0.0.i11.i, i1 true)
  %_4315.i.i = lshr i64 %69, 3
  %_15.i.i = add i64 %_4315.i.i, %probe_seq.sroa.0.0.i.i
  %index6.i.i = and i64 %_15.i.i, %_29.i.i
  %_18.i.i = sub nsw i64 0, %index6.i.i
  %70 = getelementptr inbounds { { i32, i32 }, i64 }, ptr %_32.i.i, i64 %_18.i.i
  %71 = getelementptr inbounds i8, ptr %70, i64 -16
  %.val.i.i = load i32, ptr %71, align 4, !range !209, !noalias !529, !noundef !17
  %72 = getelementptr i8, ptr %70, i64 -12
  %.val1.i.i = load i32, ptr %72, align 4, !noalias !529
  %_0.i.i.i.i.i = icmp eq i32 %.val.i.i, %0
  %_0.i1.i.i.i.i = icmp eq i32 %.val1.i.i, %1
  %spec.select.i.i.i.i = select i1 %_0.i.i.i.i.i, i1 %_0.i1.i.i.i.i, i1 false
  br i1 %spec.select.i.i.i.i, label %bb2, label %bb13.i.i, !prof !261

bb9.i.i:                                          ; preds = %bb13.i.i, %bb1.i.i
  %73 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i.i, splat (i8 -1)
  %74 = bitcast <8 x i1> %73 to i8
  %75 = icmp eq i8 %74, 0
  br i1 %75, label %bb16.i.i, label %bb3, !prof !9

bb13.i.i:                                         ; preds = %bb8.i.i
  %76 = add i64 %iter.sroa.0.0.i11.i, -1
  %_53.i.i = and i64 %76, %iter.sroa.0.0.i11.i
  %.not.i.not.i = icmp eq i64 %_53.i.i, 0
  br i1 %.not.i.not.i, label %bb9.i.i, label %bb8.i.i

bb16.i.i:                                         ; preds = %bb9.i.i
  %77 = add i64 %probe_seq.sroa.9.0.i.i, 8
  %78 = add i64 %probe_seq.sroa.0.0.i.i, %77
  br label %bb1.i.i

bb2:                                              ; preds = %bb8.i.i
  %79 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %70, ptr %79, align 8
  %80 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %self, ptr %80, align 8
  store i32 1114112, ptr %_0, align 8
  br label %bb4

bb3:                                              ; preds = %bb9.i.i
  %81 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_5.i = load i64, ptr %81, align 8, !alias.scope !532, !noalias !535, !noundef !17
  %b.i = icmp eq i64 %_5.i, 0
  br i1 %b.i, label %bb8.i, label %_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim.exit, !prof !9

bb8.i:                                            ; preds = %bb3
; call <hashbrown::raw::RawTable<((char, char), usize)>>::reserve_rehash::<hashbrown::map::make_hasher<(char, char), usize, std::hash::random::RandomState>::{closure#0}>
  %82 = tail call { i64, i64 } @_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE14reserve_rehashNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, i64 noundef 1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %hash_builder, i1 noundef zeroext true)
  %_8.0.i = extractvalue { i64, i64 } %82, 0
  %83 = icmp eq i64 %_8.0.i, -9223372036854775807
  tail call void @llvm.assume(i1 %83)
  br label %_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim.exit

_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim.exit: ; preds = %bb3, %bb8.i
  store i32 %0, ptr %_0, align 8
  %_14.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 4
  store i32 %1, ptr %_14.sroa.4.0._0.sroa_idx, align 4
  %_14.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %self, ptr %_14.sroa.5.0._0.sroa_idx, align 8
  %_14.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_0.i.i.i, ptr %_14.sroa.6.0._0.sroa_idx, align 8
  br label %bb4

bb4:                                              ; preds = %_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim.exit, %bb2
  ret void
}

; <strsim::GrowingHashmapChar<strsim::RowId>>::lookup
; Function Attrs: uwtable
define internal fastcc noundef range(i64 0, -1) i64 @_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %self, i32 noundef %key) unnamed_addr #0 {
start:
  %_3 = zext i32 %key to i64
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_5 = load i32, ptr %0, align 8, !noundef !17
  %_4 = sext i32 %_5 to i64
  %1 = and i64 %_4, %_3
  %2 = load i64, ptr %self, align 8, !range !45, !noundef !17
  %.not = icmp eq i64 %2, -9223372036854775808
  br i1 %.not, label %bb18, label %bb19, !prof !9

bb19:                                             ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_41 = load i64, ptr %3, align 8, !noundef !17
  %_43 = icmp ult i64 %1, %_41
  br i1 %_43, label %bb20, label %panic

bb18:                                             ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a649d603cf56162b8a3f165589c1de6f, i64 noundef 39, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_42d8339c69e16a1e3f5e0ef029dbe447) #25
  unreachable

bb20:                                             ; preds = %bb19
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_42 = load ptr, ptr %4, align 8, !nonnull !17, !noundef !17
  %_10 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_42, i64 %1
  %_10.val = load i64, ptr %_10, align 8, !noundef !17
  %_0.i = icmp eq i64 %_10.val, -1
  br i1 %_0.i, label %bb16, label %bb21

panic:                                            ; preds = %bb19
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %1, i64 noundef %_41, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d619813ccfd16ab2fd70bc85a627a0cb) #25
  unreachable

bb21:                                             ; preds = %bb20
  %5 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_42, i64 %1, i32 1
  %_15 = load i32, ptr %5, align 8, !noundef !17
  %_14 = icmp eq i32 %_15, %key
  br i1 %_14, label %bb16, label %bb8.preheader

bb8.preheader:                                    ; preds = %bb21
  %_2114 = mul nuw nsw i64 %1, 5
  %_2016 = add nuw nsw i64 %_3, 1
  %_1917 = add nuw nsw i64 %_2016, %_2114
  %6 = and i64 %_1917, %_4
  %_5318 = icmp ult i64 %6, %_41
  br i1 %_5318, label %bb22, label %panic2

bb8:                                              ; preds = %bb23
  %7 = lshr i32 %perturb.sroa.0.019, 5
  %_21 = mul i64 %9, 5
  %narrow = add nuw nsw i32 %7, 1
  %_20 = zext nneg i32 %narrow to i64
  %_19 = add i64 %_21, %_20
  %8 = and i64 %_19, %_4
  %_53 = icmp ult i64 %8, %_41
  br i1 %_53, label %bb22, label %panic2

bb22:                                             ; preds = %bb8.preheader, %bb8
  %9 = phi i64 [ %8, %bb8 ], [ %6, %bb8.preheader ]
  %perturb.sroa.0.019 = phi i32 [ %7, %bb8 ], [ %key, %bb8.preheader ]
  %_28 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_42, i64 %9
  %_28.val = load i64, ptr %_28, align 8, !noundef !17
  %_0.i12 = icmp eq i64 %_28.val, -1
  br i1 %_0.i12, label %bb16, label %bb23

panic2:                                           ; preds = %bb8, %bb8.preheader
  %.lcssa = phi i64 [ %6, %bb8.preheader ], [ %8, %bb8 ]
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %.lcssa, i64 noundef %_41, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0a640bf0e45dbde7c4f1706a4879d5ca) #25
  unreachable

bb23:                                             ; preds = %bb22
  %10 = getelementptr inbounds nuw %"GrowingHashmapMapElemChar<RowId>", ptr %_42, i64 %9, i32 1
  %_33 = load i32, ptr %10, align 8, !noundef !17
  %_32 = icmp eq i32 %_33, %key
  br i1 %_32, label %bb16, label %bb8

bb16:                                             ; preds = %bb22, %bb23, %bb20, %bb21
  %i.sroa.0.1 = phi i64 [ %1, %bb21 ], [ %1, %bb20 ], [ %9, %bb23 ], [ %9, %bb22 ]
  ret i64 %i.sroa.0.1
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs6KJnav5oeQt_6strsim(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1) unnamed_addr #6 {
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
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !133

bb11.i:                                           ; preds = %bb6.i
  %new_size2.i = add nuw i64 %_27.0.i, %elem_layout.1
  %_40.i = icmp ugt i64 %new_size2.i, %_33.i
  br i1 %_40.i, label %bb11, label %bb14

bb14:                                             ; preds = %bb11.i
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb1.i.i11, label %bb3.i.i

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

bb1.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #23
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #23
  br label %bb7

bb7:                                              ; preds = %bb1.i.i11, %bb3.i.i
  %_27.sroa.7.012 = phi i64 [ %_27.sroa.7.01321, %bb3.i.i ], [ %new_size2.i, %bb1.i.i11 ]
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3.i.i ], [ %4, %bb1.i.i11 ]
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

; <strsim::StrSimError as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXCs6KJnav5oeQt_6strsimNtB2_11StrSimErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %fmt) unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  %text = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %text)
  store ptr @alloc_f5d606a991b48f2a72fc31ffd42f8a04, ptr %text, align 8
  %0 = getelementptr inbounds nuw i8, ptr %text, i64 8
  store i64 35, ptr %0, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %text, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6KJnav5oeQt_6strsim, ptr %_7.sroa.4.0..sroa_idx, align 8
  %_20.0 = load ptr, ptr %fmt, align 8, !nonnull !17, !align !537, !noundef !17
  %1 = getelementptr inbounds nuw i8, ptr %fmt, i64 8
  %_20.1 = load ptr, ptr %1, align 8, !nonnull !17, !align !538, !noundef !17
; call core::fmt::write
  %2 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_20.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_20.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %text)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  ret i1 %2
}

; <core::iter::adapters::zip::Zip<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>> as core::iter::adapters::zip::ZipImpl<core::str::iter::Chars, core::iter::adapters::skip::Skip<core::str::iter::Chars>>>::next
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define internal fastcc { i32, i32 } @_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipNtNtNtBb_3str4iter5CharsINtNtB7_4skip4SkipBW_EEINtB5_7ZipImplBW_B1k_E4nextCs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(56) %self) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !539)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !542)
  %ptr.i.i.i = load ptr, ptr %self, align 8, !alias.scope !545, !nonnull !17, !noundef !17
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %end_or_len.i.i.i = load ptr, ptr %0, align 8, !alias.scope !545, !nonnull !17, !noundef !17
  %_6.i.i.not.i = icmp eq ptr %ptr.i.i.i, %end_or_len.i.i.i
  br i1 %_6.i.i.not.i, label %bb5, label %bb14.i.i

bb14.i.i:                                         ; preds = %start
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 1
  store ptr %_16.i.i.i, ptr %self, align 8, !alias.scope !545
  %x.i.i = load i8, ptr %ptr.i.i.i, align 1, !noalias !548, !noundef !17
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i)
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 2
  store ptr %_16.i12.i.i, ptr %self, align 8, !alias.scope !549
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !noalias !548, !noundef !17
  %_33.i.i = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %1 = or disjoint i32 %_33.i.i, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i, label %bb9

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb9

bb6.i.i:                                          ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i)
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 3
  store ptr %_16.i19.i.i, ptr %self, align 8, !alias.scope !552
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !noalias !548, !noundef !17
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %2 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit, label %bb9

_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit: ; preds = %bb6.i.i
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i)
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 4
  store ptr %_16.i26.i.i, ptr %self, align 8, !alias.scope !555
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !noalias !548, !noundef !17
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %3 = or disjoint i32 %_27.i.i, %_25.i.i
  %.not = icmp eq i32 %3, 1114112
  br i1 %.not, label %bb5, label %bb9

bb9:                                              ; preds = %bb4.i.i, %bb6.i.i, %bb3.i.i, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit
  %spec.select.i10 = phi i32 [ %3, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit ], [ %1, %bb4.i.i ], [ %2, %bb6.i.i ], [ %_7.i.i, %bb3.i.i ]
  %_9 = getelementptr inbounds nuw i8, ptr %self, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !558)
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %_3.i = load i64, ptr %4, align 8, !alias.scope !558, !noundef !17
  %b.not.i = icmp eq i64 %_3.i, 0
  br i1 %b.not.i, label %bb6.i, label %bb4.i, !prof !261

bb6.i:                                            ; preds = %bb9
  tail call void @llvm.experimental.noalias.scope.decl(metadata !561)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !564)
  %ptr.i.i.i.i = load ptr, ptr %_9, align 8, !alias.scope !567, !nonnull !17, !noundef !17
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %end_or_len.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !567, !nonnull !17, !noundef !17
  %_6.i.i.not.i.i = icmp eq ptr %ptr.i.i.i.i, %end_or_len.i.i.i.i
  br i1 %_6.i.i.not.i.i, label %bb5, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb6.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i.i, i64 1
  store ptr %_16.i.i.i.i, ptr %_9, align 8, !alias.scope !567
  %x.i.i.i = load i8, ptr %ptr.i.i.i.i, align 1, !noalias !570, !noundef !17
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %end_or_len.i.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i.i, i64 2
  store ptr %_16.i12.i.i.i, ptr %_9, align 8, !alias.scope !571
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !570, !noundef !17
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb5

bb3.i.i.i:                                        ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb5

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %end_or_len.i.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i.i, i64 3
  store ptr %_16.i19.i.i.i, ptr %_9, align 8, !alias.scope !574
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !570, !noundef !17
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %7 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i, label %bb5

bb8.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %end_or_len.i.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i.i, i64 4
  store ptr %_16.i26.i.i.i, ptr %_9, align 8, !alias.scope !577
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !570, !noundef !17
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %8 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  %9 = freeze i32 %8
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit

bb4.i:                                            ; preds = %bb9
  store i64 0, ptr %4, align 8, !alias.scope !558
; call <core::str::iter::Chars as core::iter::traits::iterator::Iterator>::nth
  %10 = tail call fastcc noundef i32 @_RNvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB8_4iter6traits8iterator8Iterator3nthCs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 dereferenceable(24) %_9, i64 noundef %_3.i) #27
  br label %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit

_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit: ; preds = %bb8.i.i.i, %bb4.i
  %_0.sroa.0.0.i = phi i32 [ %10, %bb4.i ], [ %9, %bb8.i.i.i ]
  %.not6 = icmp eq i32 %_0.sroa.0.0.i, 1114112
  %spec.select = select i1 %.not6, i32 1114112, i32 %spec.select.i10
  br label %bb5

bb5:                                              ; preds = %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit, %bb4.i.i.i, %bb6.i.i.i, %bb3.i.i.i, %bb6.i, %start, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit
  %_0.sroa.4.0 = phi i32 [ undef, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit ], [ undef, %start ], [ 1114112, %bb6.i ], [ %6, %bb4.i.i.i ], [ %7, %bb6.i.i.i ], [ %_7.i.i.i, %bb3.i.i.i ], [ %_0.sroa.0.0.i, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit ]
  %_0.sroa.0.0 = phi i32 [ 1114112, %_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next.exit ], [ 1114112, %start ], [ 1114112, %bb6.i ], [ %spec.select.i10, %bb4.i.i.i ], [ %spec.select.i10, %bb6.i.i.i ], [ %spec.select.i10, %bb3.i.i.i ], [ %spec.select, %_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim.exit ]
  %11 = insertvalue { i32, i32 } poison, i32 %_0.sroa.0.0, 0
  %12 = insertvalue { i32, i32 } %11, i32 %_0.sroa.4.0, 1
  ret { i32, i32 } %12
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6KJnav5oeQt_6strsim(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !17, !align !537, !noundef !17
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !17
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <core::str::iter::Chars as core::iter::traits::iterator::Iterator>::nth
; Function Attrs: inlinehint nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define internal fastcc noundef range(i32 0, 1114113) i32 @_RNvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB8_4iter6traits8iterator8Iterator3nthCs6KJnav5oeQt_6strsim(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %self, i64 noundef range(i64 1, 0) %n) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !580)
  %_3.i = icmp ugt i64 %n, 31
  %self1.i22.i = load ptr, ptr %self, align 8, !alias.scope !580
  br i1 %_3.i, label %bb1.i, label %bb23.lr.ph.i

bb1.i:                                            ; preds = %start
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_7.i23.i = load ptr, ptr %0, align 8, !alias.scope !583, !nonnull !17, !noundef !17
  %1 = ptrtoint ptr %_7.i23.i to i64
  %2 = ptrtoint ptr %self1.i22.i to i64
  %3 = sub nuw i64 %1, %2
  %_57.idx.i = and i64 %3, -32
  %_57.i = getelementptr inbounds nuw i8, ptr %self1.i22.i, i64 %_57.idx.i
  %_1039.i = icmp eq i64 %n, 32
  %_6540.i = icmp samesign eq i64 %_57.idx.i, 0
  %or.cond41.i = select i1 %_1039.i, i1 true, i1 %_6540.i
  br i1 %or.cond41.i, label %bb9.i, label %bb34.i

bb21.i:                                           ; preds = %bb15.i, %bb14.i
  %self.promoted50.i7 = phi ptr [ %_29.i31.i, %bb15.i ], [ %_29.i314749.i, %bb14.i ]
  %_39.not52.i = icmp eq i64 %remainder.sroa.0.1.lcssa.i, 0
  br i1 %_39.not52.i, label %bb21.i.bb5_crit_edge, label %bb23.lr.ph.i

bb21.i.bb5_crit_edge:                             ; preds = %bb21.i
  %end_or_len.i.i.i.pre = load ptr, ptr %0, align 8, !alias.scope !586
  br label %bb5

bb23.lr.ph.i:                                     ; preds = %start, %bb9.i, %bb21.i
  %self.promoted50.i = phi ptr [ %self.promoted50.i7, %bb21.i ], [ %_29.i.i, %bb9.i ], [ %self1.i22.i, %start ]
  %remainder.sroa.0.061.i = phi i64 [ %remainder.sroa.0.1.lcssa.i, %bb21.i ], [ %remainder.sroa.0.1.lcssa.i, %bb9.i ], [ %n, %start ]
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.i.i = load ptr, ptr %4, align 8, !alias.scope !593, !nonnull !17, !noundef !17
  %5 = ptrtoint ptr %_4.i.i to i64
  br label %bb23.i

bb9.i:                                            ; preds = %bb34.i, %bb1.i
  %bytes_skipped.sroa.0.0.lcssa.i = phi i64 [ 0, %bb1.i ], [ %8, %bb34.i ]
  %remainder.sroa.0.1.lcssa.i = phi i64 [ %n, %bb1.i ], [ %op.rdx, %bb34.i ]
  %_29.i.i = getelementptr inbounds nuw i8, ptr %self1.i22.i, i64 %bytes_skipped.sroa.0.0.lcssa.i
  store ptr %_29.i.i, ptr %self, align 8, !alias.scope !596
  %.not.i = icmp ule i64 %bytes_skipped.sroa.0.0.lcssa.i, %3
  tail call void @llvm.assume(i1 %.not.i)
  %_28.not48.i = icmp eq ptr %_7.i23.i, %_29.i.i
  br i1 %_28.not48.i, label %bb23.lr.ph.i, label %bb14.i

bb34.i:                                           ; preds = %bb1.i, %bb34.i
  %remainder.sroa.0.144.i = phi i64 [ %op.rdx, %bb34.i ], [ %n, %bb1.i ]
  %chunks.sroa.0.043.i = phi ptr [ %_74.i, %bb34.i ], [ %self1.i22.i, %bb1.i ]
  %bytes_skipped.sroa.0.042.i = phi i64 [ %8, %bb34.i ], [ 0, %bb1.i ]
  %6 = load <32 x i8>, ptr %chunks.sroa.0.043.i, align 1, !noalias !580
  %7 = icmp sgt <32 x i8> %6, splat (i8 -65)
  %_74.i = getelementptr inbounds nuw i8, ptr %chunks.sroa.0.043.i, i64 32
  %8 = add i64 %bytes_skipped.sroa.0.042.i, 32
  %9 = bitcast <32 x i1> %7 to i32
  %10 = tail call range(i32 0, 33) i32 @llvm.ctpop.i32(i32 %9)
  %11 = zext nneg i32 %10 to i64
  %op.rdx = sub i64 %remainder.sroa.0.144.i, %11
  %_10.i = icmp ult i64 %op.rdx, 33
  %_65.i = icmp eq ptr %_74.i, %_57.i
  %or.cond.i = select i1 %_10.i, i1 true, i1 %_65.i
  br i1 %or.cond.i, label %bb9.i, label %bb34.i

bb14.i:                                           ; preds = %bb9.i, %bb15.i
  %_29.i314749.i = phi ptr [ %_29.i31.i, %bb15.i ], [ %_29.i.i, %bb9.i ]
  %b.i = load i8, ptr %_29.i314749.i, align 1, !noalias !580, !noundef !17
  %_36.i = icmp slt i8 %b.i, -64
  br i1 %_36.i, label %bb15.i, label %bb21.i

bb15.i:                                           ; preds = %bb14.i
  %_29.i31.i = getelementptr inbounds nuw i8, ptr %_29.i314749.i, i64 1
  store ptr %_29.i31.i, ptr %self, align 8, !alias.scope !599
  %_28.not.i = icmp eq ptr %_7.i23.i, %_29.i31.i
  br i1 %_28.not.i, label %bb21.i, label %bb14.i

bb23.i:                                           ; preds = %bb46.i, %bb23.lr.ph.i
  %remainder.sroa.0.254.i = phi i64 [ %remainder.sroa.0.061.i, %bb23.lr.ph.i ], [ %14, %bb46.i ]
  %_29.i365153.i = phi ptr [ %self.promoted50.i, %bb23.lr.ph.i ], [ %_29.i36.i, %bb46.i ]
  %_41.not.i = icmp eq ptr %_4.i.i, %_29.i365153.i
  br i1 %_41.not.i, label %bb3, label %bb46.i

bb46.i:                                           ; preds = %bb23.i
  %12 = ptrtoint ptr %_29.i365153.i to i64
  %13 = sub nuw i64 %5, %12
  %14 = add i64 %remainder.sroa.0.254.i, -1
  %b7.i = load i8, ptr %_29.i365153.i, align 1, !noalias !580, !noundef !17
  %_96.i = zext i8 %b7.i to i64
  %15 = getelementptr inbounds nuw i8, ptr @alloc_db51a71a1b6b25b4224d4dc5277f93e7, i64 %_96.i
  %_94.i = load i8, ptr %15, align 1, !noalias !580, !noundef !17
  %slurp.i = zext i8 %_94.i to i64
  %_29.i36.i = getelementptr inbounds nuw i8, ptr %_29.i365153.i, i64 %slurp.i
  store ptr %_29.i36.i, ptr %self, align 8, !alias.scope !602
  %.not18.i = icmp uge i64 %13, %slurp.i
  tail call void @llvm.assume(i1 %.not18.i)
  %_39.not.i = icmp eq i64 %14, 0
  br i1 %_39.not.i, label %bb5, label %bb23.i

bb5:                                              ; preds = %bb46.i, %bb21.i.bb5_crit_edge
  %end_or_len.i.i.i = phi ptr [ %end_or_len.i.i.i.pre, %bb21.i.bb5_crit_edge ], [ %_4.i.i, %bb46.i ]
  %ptr.i.i.i = phi ptr [ %self.promoted50.i7, %bb21.i.bb5_crit_edge ], [ %_29.i36.i, %bb46.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !605)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !606)
  %_6.i.i.not.i = icmp eq ptr %ptr.i.i.i, %end_or_len.i.i.i
  br i1 %_6.i.i.not.i, label %bb3, label %bb14.i.i

bb14.i.i:                                         ; preds = %bb5
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 1
  store ptr %_16.i.i.i, ptr %self, align 8, !alias.scope !586
  %x.i.i = load i8, ptr %ptr.i.i.i, align 1, !noalias !607, !noundef !17
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i)
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 2
  store ptr %_16.i12.i.i, ptr %self, align 8, !alias.scope !608
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !noalias !607, !noundef !17
  %_33.i.i = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %16 = or disjoint i32 %_33.i.i, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i, label %bb3

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb3

bb6.i.i:                                          ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i)
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 3
  store ptr %_16.i19.i.i, ptr %self, align 8, !alias.scope !611
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !noalias !607, !noundef !17
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %17 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb3

bb8.i.i:                                          ; preds = %bb6.i.i
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %end_or_len.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i)
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %ptr.i.i.i, i64 4
  store ptr %_16.i26.i.i, ptr %self, align 8, !alias.scope !614
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !noalias !607, !noundef !17
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %18 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb3

bb3:                                              ; preds = %bb23.i, %bb8.i.i, %bb6.i.i, %bb3.i.i, %bb4.i.i, %bb5
  %_0.sroa.0.0 = phi i32 [ %_7.i.i, %bb3.i.i ], [ %18, %bb8.i.i ], [ %17, %bb6.i.i ], [ %16, %bb4.i.i ], [ 1114112, %bb5 ], [ 1114112, %bb23.i ]
  ret i32 %_0.sroa.0.0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.usub.sat.i64(i64, i64) #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #10

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #11

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #13

; <hashbrown::raw::Fallibility>::capacity_overflow
; Function Attrs: uwtable
declare { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility17capacity_overflow(i1 noundef zeroext) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #9

; <hashbrown::raw::Fallibility>::alloc_err
; Function Attrs: uwtable
declare { i64, i64 } @_RNvMNtCsh9QrOU9e3Ke_9hashbrown3rawNtB2_11Fallibility9alloc_err(i1 noundef zeroext, i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #14

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.fshl.i64(i64, i64, i64) #9

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #15

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare nonnull ptr @llvm.threadlocal.address.p0(ptr nonnull) #9

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #16

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #17

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #2

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #18

; __rustc::__rust_alloc_zeroed
; Function Attrs: nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed(i64 noundef, i64 allocalign noundef) unnamed_addr #19

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #9

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #13

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.cttz.i64(i64, i1 immarg) #9

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memmove.p0.p0.i64(ptr writeonly captures(none), ptr readonly captures(none), i64, i1 immarg) #11

; std::sys::random::hashmap_random_keys
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCs5sEH5CPMdak_3std3sys6random19hashmap_random_keys() unnamed_addr #0

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; core::str::count::do_count_chars
; Function Attrs: uwtable
declare noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count14do_count_chars(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::str::count::char_count_general_case
; Function Attrs: uwtable
declare noundef i64 @_RNvNtNtCsjMrxcFdYDNN_4core3str5count23char_count_general_case(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #20

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #21

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #22

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #22

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smin.i64(i64, i64) #22

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smax.i64(i64, i64) #22

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i32 @llvm.ctpop.i32(i32) #22

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { nofree norecurse nosync nounwind memory(argmem: read, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { inlinehint nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #10 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #11 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #12 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #15 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { nounwind allockind("alloc,zeroed,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #21 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #22 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #23 = { nounwind }
attributes #24 = { noreturn }
attributes #25 = { noinline noreturn }
attributes #26 = { cold }
attributes #27 = { inlinehint }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!"branch_weights", i32 2000, i32 6004}
!3 = !{!4, !6}
!4 = distinct !{!4, !5, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!5 = distinct !{!5, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!6 = distinct !{!6, !7, !"_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim: %_0"}
!7 = distinct !{!7, !"_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elembNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!8 = !{!6}
!9 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!10 = !{!11, !13, !15}
!11 = distinct !{!11, !12, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!12 = distinct !{!12, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!13 = distinct !{!13, !14, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!14 = distinct !{!14, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!15 = distinct !{!15, !16, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!16 = distinct !{!16, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!17 = !{}
!18 = !{!19, !21, !23}
!19 = distinct !{!19, !20, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!20 = distinct !{!20, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!21 = distinct !{!21, !22, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!22 = distinct !{!22, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!23 = distinct !{!23, !24, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterbENtNtNtBb_3str4iter5CharsEINtB5_7ZipImplBW_B1o_E4nextCs6KJnav5oeQt_6strsim: %self"}
!24 = distinct !{!24, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterbENtNtNtBb_3str4iter5CharsEINtB5_7ZipImplBW_B1o_E4nextCs6KJnav5oeQt_6strsim"}
!25 = !{i8 0, i8 2}
!26 = distinct !{!26, !27}
!27 = !{!"llvm.loop.peeled.count", i32 1}
!28 = !{!29, !31, !33}
!29 = distinct !{!29, !30, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!30 = distinct !{!30, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!31 = distinct !{!31, !32, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!32 = distinct !{!32, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!33 = distinct !{!33, !34, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterbENtNtNtBb_3str4iter5CharsEINtB5_7ZipImplBW_B1o_E4nextCs6KJnav5oeQt_6strsim: %self"}
!34 = distinct !{!34, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtNtBb_5slice4iter4IterbENtNtNtBb_3str4iter5CharsEINtB5_7ZipImplBW_B1o_E4nextCs6KJnav5oeQt_6strsim"}
!35 = !{!36, !38, !40}
!36 = distinct !{!36, !37, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!37 = distinct !{!37, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!38 = distinct !{!38, !39, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!39 = distinct !{!39, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!40 = distinct !{!40, !41, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!41 = distinct !{!41, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!42 = !{!43}
!43 = distinct !{!43, !44, !"_RNvXs5_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdENtNtCsjMrxcFdYDNN_4core7default7Default7defaultB5_: %_0"}
!44 = distinct !{!44, !"_RNvXs5_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdENtNtCsjMrxcFdYDNN_4core7default7Default7defaultB5_"}
!45 = !{i64 0, i64 -9223372036854775807}
!46 = !{!"branch_weights", i32 6004, i32 2000}
!47 = !{!48, !50, !52}
!48 = distinct !{!48, !49, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!49 = distinct !{!49, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!50 = distinct !{!50, !51, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VeciEINtB4_18SpecFromIterNestediINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1F_3ops5range5RangeiEB2o_EE9from_iterCs6KJnav5oeQt_6strsim: %_0"}
!51 = distinct !{!51, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VeciEINtB4_18SpecFromIterNestediINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1F_3ops5range5RangeiEB2o_EE9from_iterCs6KJnav5oeQt_6strsim"}
!52 = distinct !{!52, !51, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VeciEINtB4_18SpecFromIterNestediINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1F_3ops5range5RangeiEB2o_EE9from_iterCs6KJnav5oeQt_6strsim: %iterator"}
!53 = !{!54, !56, !58, !60, !62, !63, !65, !66, !68, !69, !71, !50, !52}
!54 = distinct !{!54, !55, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB19_3ops5range5RangeiEB1S_EE0Cs6KJnav5oeQt_6strsim: %_1"}
!55 = distinct !{!55, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB19_3ops5range5RangeiEB1S_EE0Cs6KJnav5oeQt_6strsim"}
!56 = distinct !{!56, !57, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VeciE14extend_trustedINtNtNtBc_8adapters5chain5ChainINtNtNtBe_3ops5range5RangeiEB2N_EE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!57 = distinct !{!57, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VeciE14extend_trustedINtNtNtBc_8adapters5chain5ChainINtNtNtBe_3ops5range5RangeiEB2N_EE0E0Cs6KJnav5oeQt_6strsim"}
!58 = distinct !{!58, !59, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangeiENtNtNtNtBa_4iter6traits8iterator8Iterator4folduQNCINvNvBL_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB25_3VeciE14extend_trustedINtNtNtBR_8adapters5chain5ChainB3_B3_EE0E0ECs6KJnav5oeQt_6strsim: argument 0"}
!59 = distinct !{!59, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangeiENtNtNtNtBa_4iter6traits8iterator8Iterator4folduQNCINvNvBL_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB25_3VeciE14extend_trustedINtNtNtBR_8adapters5chain5ChainB3_B3_EE0E0ECs6KJnav5oeQt_6strsim"}
!60 = distinct !{!60, !61, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_3ops5range5RangeiEB10_ENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1x_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2K_3VeciE14extend_trustedBO_E0E0ECs6KJnav5oeQt_6strsim: %self"}
!61 = distinct !{!61, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_3ops5range5RangeiEB10_ENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1x_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2K_3VeciE14extend_trustedBO_E0E0ECs6KJnav5oeQt_6strsim"}
!62 = distinct !{!62, !61, !"_RINvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chainINtB5_5ChainINtNtNtBb_3ops5range5RangeiEB10_ENtNtNtB9_6traits8iterator8Iterator4folduNCINvNvB1x_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2K_3VeciE14extend_trustedBO_E0E0ECs6KJnav5oeQt_6strsim: %f"}
!63 = distinct !{!63, !64, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtBc_3ops5range5RangeiEBV_ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2h_3VeciE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim: %self"}
!64 = distinct !{!64, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtBc_3ops5range5RangeiEBV_ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2h_3VeciE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim"}
!65 = distinct !{!65, !64, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtBc_3ops5range5RangeiEBV_ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2h_3VeciE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim: %f"}
!66 = distinct !{!66, !67, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB17_3ops5range5RangeiEB1Q_EECs6KJnav5oeQt_6strsim: %self"}
!67 = distinct !{!67, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB17_3ops5range5RangeiEB1Q_EECs6KJnav5oeQt_6strsim"}
!68 = distinct !{!68, !67, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB17_3ops5range5RangeiEB1Q_EECs6KJnav5oeQt_6strsim: %iterator"}
!69 = distinct !{!69, !70, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VeciEINtB4_10SpecExtendiINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1n_3ops5range5RangeiEB26_EE11spec_extendCs6KJnav5oeQt_6strsim: %self"}
!70 = distinct !{!70, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VeciEINtB4_10SpecExtendiINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1n_3ops5range5RangeiEB26_EE11spec_extendCs6KJnav5oeQt_6strsim"}
!71 = distinct !{!71, !70, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VeciEINtB4_10SpecExtendiINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB1n_3ops5range5RangeiEB26_EE11spec_extendCs6KJnav5oeQt_6strsim: %iterator"}
!72 = !{!73, !75, !77, !60, !62, !63, !65, !66, !68, !69, !71, !50, !52}
!73 = distinct !{!73, !74, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB19_3ops5range5RangeiEB1S_EE0Cs6KJnav5oeQt_6strsim: %_1"}
!74 = distinct !{!74, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VeciE14extend_trustedINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters5chain5ChainINtNtNtB19_3ops5range5RangeiEB1S_EE0Cs6KJnav5oeQt_6strsim"}
!75 = distinct !{!75, !76, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VeciE14extend_trustedINtNtNtBc_8adapters5chain5ChainINtNtNtBe_3ops5range5RangeiEB2N_EE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!76 = distinct !{!76, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VeciE14extend_trustedINtNtNtBc_8adapters5chain5ChainINtNtNtBe_3ops5range5RangeiEB2N_EE0E0Cs6KJnav5oeQt_6strsim"}
!77 = distinct !{!77, !78, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangeiENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VeciE14extend_trustedINtNtNtBR_8adapters5chain5ChainB3_B3_EE0E0ECs6KJnav5oeQt_6strsim: %f"}
!78 = distinct !{!78, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangeiENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calliNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VeciE14extend_trustedINtNtNtBR_8adapters5chain5ChainB3_B3_EE0E0ECs6KJnav5oeQt_6strsim"}
!79 = distinct !{!79, !80, !81}
!80 = !{!"llvm.loop.isvectorized", i32 1}
!81 = !{!"llvm.loop.unroll.runtime.disable"}
!82 = distinct !{!82, !81, !80}
!83 = !{!84, !86, !88}
!84 = distinct !{!84, !85, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!85 = distinct !{!85, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!86 = distinct !{!86, !87, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!87 = distinct !{!87, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!88 = distinct !{!88, !89, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!89 = distinct !{!89, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y"}
!92 = distinct !{!92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim"}
!93 = !{!94}
!94 = distinct !{!94, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x"}
!95 = !{!96}
!96 = distinct !{!96, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y:It1"}
!97 = !{!98}
!98 = distinct !{!98, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x:It1"}
!99 = !{!100}
!100 = distinct !{!100, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y:It2"}
!101 = !{!102}
!102 = distinct !{!102, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x:It2"}
!103 = !{!104, !106, !108}
!104 = distinct !{!104, !105, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!105 = distinct !{!105, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!106 = distinct !{!106, !107, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!107 = distinct !{!107, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!108 = distinct !{!108, !109, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!109 = distinct !{!109, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_RNvMs4_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdE7get_mutB5_: %self"}
!112 = distinct !{!112, !"_RNvMs4_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdE7get_mutB5_"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_: %self"}
!115 = distinct !{!115, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE7get_mutB5_"}
!116 = !{!114, !111}
!117 = !{!118}
!118 = distinct !{!118, !119, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_: %self"}
!119 = distinct !{!119, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE8allocateB5_"}
!120 = !{!118, !114, !111}
!121 = !{!122, !124, !118, !114, !111}
!122 = distinct !{!122, !123, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!123 = distinct !{!123, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!124 = distinct !{!124, !125, !"_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_: %_0"}
!125 = distinct !{!125, !"_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_"}
!126 = !{!127, !124, !118, !114, !111}
!127 = distinct !{!127, !128, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBI_5RowIdEE11extend_withBI_: %self"}
!128 = distinct !{!128, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBI_5RowIdEE11extend_withBI_"}
!129 = !{!130}
!130 = distinct !{!130, !131, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_: %self"}
!131 = distinct !{!131, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE4growB5_"}
!132 = !{!130, !114, !111}
!133 = !{!"branch_weights", i32 2002, i32 2000}
!134 = !{!135, !137, !130, !114, !111}
!135 = distinct !{!135, !136, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!136 = distinct !{!136, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!137 = distinct !{!137, !138, !"_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_: %_0"}
!138 = distinct !{!138, !"_RINvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBO_5RowIdENtB3_12SpecFromElem9from_elemNtNtB7_5alloc6GlobalEBO_"}
!139 = !{!140, !137, !130, !114, !111}
!140 = distinct !{!140, !141, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBI_5RowIdEE11extend_withBI_: %self"}
!141 = distinct !{!141, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBI_5RowIdEE11extend_withBI_"}
!142 = !{!143, !130, !114, !111}
!143 = distinct !{!143, !144, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBZ_: %self"}
!144 = distinct !{!144, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBZ_"}
!145 = !{!146, !130, !114, !111}
!146 = distinct !{!146, !147, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBZ_: %self"}
!147 = distinct !{!147, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBZ_"}
!148 = !{!149, !151, !130, !114, !111}
!149 = distinct !{!149, !150, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextBZ_: %_0"}
!150 = distinct !{!150, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextBZ_"}
!151 = distinct !{!151, !150, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtCs6KJnav5oeQt_6strsim25GrowingHashmapMapElemCharNtBZ_5RowIdEENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextBZ_: %self"}
!152 = !{!153}
!153 = distinct !{!153, !154, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_: %self"}
!154 = distinct !{!154, !"_RNvMs3_Cs6KJnav5oeQt_6strsimINtB5_18GrowingHashmapCharNtB5_5RowIdE6lookupB5_"}
!155 = !{!156}
!156 = distinct !{!156, !157, !"_RNvMs4_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdE3getB5_: %self"}
!157 = distinct !{!157, !"_RNvMs4_Cs6KJnav5oeQt_6strsimINtB5_24HybridGrowingHashmapCharNtB5_5RowIdE3getB5_"}
!158 = !{!159}
!159 = distinct !{!159, !160, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %self"}
!160 = distinct !{!160, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!161 = !{!162}
!162 = distinct !{!162, !160, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner20reserve_rehash_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %alloc"}
!163 = !{!159, !162}
!164 = !{!165}
!165 = distinct !{!165, !166, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner12resize_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %self"}
!166 = distinct !{!166, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner12resize_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!167 = !{!"branch_weights", !"expected", i32 2146946, i32 2145336702}
!168 = !{!169, !171}
!169 = distinct !{!169, !170, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner17new_uninitializedNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %_0"}
!170 = distinct !{!170, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner17new_uninitializedNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!171 = distinct !{!171, !172, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %_0"}
!172 = distinct !{!172, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner22fallible_with_capacityNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!173 = !{!171}
!174 = !{!175}
!175 = distinct !{!175, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x"}
!176 = distinct !{!176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim"}
!177 = !{!178}
!178 = distinct !{!178, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y"}
!179 = !{!165, !159}
!180 = !{!181, !162}
!181 = distinct !{!181, !166, !"_RINvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB6_13RawTableInner12resize_innerNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalECs6KJnav5oeQt_6strsim: %alloc"}
!182 = !{!183, !185, !165, !159}
!183 = distinct !{!183, !184, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!184 = distinct !{!184, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!185 = distinct !{!185, !186, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!186 = distinct !{!186, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!187 = !{!188, !190}
!188 = distinct !{!188, !189, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!189 = distinct !{!189, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim"}
!190 = distinct !{!190, !189, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim: %table"}
!191 = !{!192, !194}
!192 = distinct !{!192, !193, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!193 = distinct !{!193, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!194 = distinct !{!194, !195, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!195 = distinct !{!195, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!196 = !{!197}
!197 = distinct !{!197, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x:It1"}
!198 = !{!199}
!199 = distinct !{!199, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y:It1"}
!200 = !{!201}
!201 = distinct !{!201, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x:It2"}
!202 = !{!203}
!203 = distinct !{!203, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y:It2"}
!204 = !{!205, !207}
!205 = distinct !{!205, !206, !"_RNvXs1_NtCsh9QrOU9e3Ke_9hashbrown10scopeguardINtB5_10ScopeGuardNtNtB7_3raw13RawTableInnerNCINvMsa_B11_BZ_14prepare_resizeNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalE0ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6KJnav5oeQt_6strsim: %self"}
!206 = distinct !{!206, !"_RNvXs1_NtCsh9QrOU9e3Ke_9hashbrown10scopeguardINtB5_10ScopeGuardNtNtB7_3raw13RawTableInnerNCINvMsa_B11_BZ_14prepare_resizeNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalE0ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6KJnav5oeQt_6strsim"}
!207 = distinct !{!207, !208, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown10scopeguard10ScopeGuardNtNtBL_3raw13RawTableInnerNCINvMsa_B1z_B1x_14prepare_resizeNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalE0EECs6KJnav5oeQt_6strsim: %_1"}
!208 = distinct !{!208, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsh9QrOU9e3Ke_9hashbrown10scopeguard10ScopeGuardNtNtBL_3raw13RawTableInnerNCINvMsa_B1z_B1x_14prepare_resizeNtNtCsdJPVW0sQgAG_5alloc5alloc6GlobalE0EECs6KJnav5oeQt_6strsim"}
!209 = !{i32 0, i32 1114112}
!210 = !{!211}
!211 = distinct !{!211, !212, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRTccEECs6KJnav5oeQt_6strsim: argument 0"}
!212 = distinct !{!212, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRTccEECs6KJnav5oeQt_6strsim"}
!213 = !{!214, !188, !190}
!214 = distinct !{!214, !215, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRTccENtB8_4Hash4hashNtNtNtCs5sEH5CPMdak_3std4hash6random13DefaultHasherECs6KJnav5oeQt_6strsim: %state"}
!215 = distinct !{!215, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRTccENtB8_4Hash4hashNtNtNtCs5sEH5CPMdak_3std4hash6random13DefaultHasherECs6KJnav5oeQt_6strsim"}
!216 = !{!217, !219}
!217 = distinct !{!217, !218, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!218 = distinct !{!218, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!219 = distinct !{!219, !220, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!220 = distinct !{!220, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!221 = !{!"branch_weights", i32 1, i32 1999}
!222 = !{!"branch_weights", i32 0, i32 1}
!223 = !{!224, !226}
!224 = distinct !{!224, !225, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!225 = distinct !{!225, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!226 = distinct !{!226, !227, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!227 = distinct !{!227, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!228 = !{!229}
!229 = distinct !{!229, !230, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place: %self"}
!230 = distinct !{!230, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner15rehash_in_place"}
!231 = !{!"branch_weights", !"expected", i32 0, i32 -2147483648}
!232 = !{!233, !235, !229}
!233 = distinct !{!233, !234, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!234 = distinct !{!234, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!235 = distinct !{!235, !236, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!236 = distinct !{!236, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!237 = !{!238, !240, !229}
!238 = distinct !{!238, !239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr15write_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %src"}
!239 = distinct !{!239, !"_RINvNtCsjMrxcFdYDNN_4core3ptr15write_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!240 = distinct !{!240, !241, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vst1_u8: %a"}
!241 = distinct !{!241, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vst1_u8"}
!242 = !{!243}
!243 = distinct !{!243, !244, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRTccEECs6KJnav5oeQt_6strsim: argument 0"}
!244 = distinct !{!244, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRTccEECs6KJnav5oeQt_6strsim"}
!245 = !{!246, !248, !250, !229}
!246 = distinct !{!246, !247, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRTccENtB8_4Hash4hashNtNtNtCs5sEH5CPMdak_3std4hash6random13DefaultHasherECs6KJnav5oeQt_6strsim: %state"}
!247 = distinct !{!247, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRTccENtB8_4Hash4hashNtNtNtCs5sEH5CPMdak_3std4hash6random13DefaultHasherECs6KJnav5oeQt_6strsim"}
!248 = distinct !{!248, !249, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!249 = distinct !{!249, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim"}
!250 = distinct !{!250, !249, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE14reserve_rehashNCINvNtBa_3map11make_hasherBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0E0Cs6KJnav5oeQt_6strsim: %table"}
!251 = !{!252, !254, !229}
!252 = distinct !{!252, !253, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!253 = distinct !{!253, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!254 = distinct !{!254, !255, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!255 = distinct !{!255, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!256 = !{!257, !259, !229}
!257 = distinct !{!257, !258, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!258 = distinct !{!258, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!259 = distinct !{!259, !260, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!260 = distinct !{!260, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!261 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!262 = !{!263}
!263 = distinct !{!263, !264, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x"}
!264 = distinct !{!264, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim"}
!265 = !{!266}
!266 = distinct !{!266, !264, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y"}
!267 = !{!266, !229}
!268 = !{!263, !229}
!269 = !{!270}
!270 = distinct !{!270, !264, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %x:It1"}
!271 = !{!272}
!272 = distinct !{!272, !264, !"_RINvNtCsjMrxcFdYDNN_4core3ptr10swap_chunkKj8_ECs6KJnav5oeQt_6strsim: %y:It1"}
!273 = !{!272, !229}
!274 = !{!270, !229}
!275 = !{!276}
!276 = distinct !{!276, !277, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs6KJnav5oeQt_6strsim: %self"}
!277 = distinct !{!277, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs6KJnav5oeQt_6strsim"}
!278 = !{i64 0, i64 -9223372036854775808}
!279 = !{i64 0, i64 2}
!280 = !{!281}
!281 = distinct !{!281, !282, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!282 = distinct !{!282, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!283 = !{!284}
!284 = distinct !{!284, !285, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VeciE11extend_withCs6KJnav5oeQt_6strsim: %self"}
!285 = distinct !{!285, !"_RNvMs3_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VeciE11extend_withCs6KJnav5oeQt_6strsim"}
!286 = distinct !{!286, !80, !81}
!287 = distinct !{!287, !81, !80}
!288 = !{!289}
!289 = distinct !{!289, !290, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!290 = distinct !{!290, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!291 = !{!292, !294}
!292 = distinct !{!292, !293, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!293 = distinct !{!293, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!294 = distinct !{!294, !295, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim: %_0"}
!295 = distinct !{!295, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim"}
!296 = !{!297, !299, !301, !303, !305, !307, !294}
!297 = distinct !{!297, !298, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim: %_1"}
!298 = distinct !{!298, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim"}
!299 = distinct !{!299, !300, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!300 = distinct !{!300, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim"}
!301 = distinct !{!301, !302, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim: %f"}
!302 = distinct !{!302, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim"}
!303 = distinct !{!303, !304, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim: %f"}
!304 = distinct !{!304, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim"}
!305 = distinct !{!305, !306, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim: %self"}
!306 = distinct !{!306, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim"}
!307 = distinct !{!307, !308, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim: %self"}
!308 = distinct !{!308, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim"}
!309 = distinct !{!309, !80, !81}
!310 = !{!294}
!311 = distinct !{!311, !81, !80}
!312 = !{!313, !315, !317}
!313 = distinct !{!313, !314, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!314 = distinct !{!314, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!315 = distinct !{!315, !316, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!316 = distinct !{!316, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!317 = distinct !{!317, !318, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!318 = distinct !{!318, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!319 = !{!320, !322, !324}
!320 = distinct !{!320, !321, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!321 = distinct !{!321, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!322 = distinct !{!322, !323, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!323 = distinct !{!323, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!324 = distinct !{!324, !325, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!325 = distinct !{!325, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!326 = !{!327, !329, !331, !333, !335, !337, !339, !340, !342}
!327 = distinct !{!327, !328, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!328 = distinct !{!328, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!329 = distinct !{!329, !330, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!330 = distinct !{!330, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!331 = distinct !{!331, !332, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!332 = distinct !{!332, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4takeINtB4_4TakeNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!333 = distinct !{!333, !334, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtB7_4take4TakeNtNtNtBb_3str4iter5CharsEB1e_EINtB5_7ZipImplBW_B1e_E4nextCs6KJnav5oeQt_6strsim: %self"}
!334 = distinct !{!334, !"_RNvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB5_3ZipINtNtB7_4take4TakeNtNtNtBb_3str4iter5CharsEB1e_EINtB5_7ZipImplBW_B1e_E4nextCs6KJnav5oeQt_6strsim"}
!335 = distinct !{!335, !336, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!336 = distinct !{!336, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zipINtB4_3ZipINtNtB6_4take4TakeNtNtNtBa_3str4iter5CharsEB1d_ENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!337 = distinct !{!337, !338, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB19_ENtNtNtBa_6traits8iterator8Iterator8try_foldjNCINvNvXs0_NtB8_10take_whileINtB2w_9TakeWhileppEB1D_8try_fold5checkTccEjINtNtNtBc_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB4j_13StringWrapperB50_ccE0NCINvMs0_B3y_B3v_10wrap_mut_2jB3q_NCNvYIB2O_B3_B4e_EB1D_5count0E0E0INtNtB3A_12control_flow11ControlFlowB3v_jEEB4j_: %self"}
!338 = distinct !{!338, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB19_ENtNtNtBa_6traits8iterator8Iterator8try_foldjNCINvNvXs0_NtB8_10take_whileINtB2w_9TakeWhileppEB1D_8try_fold5checkTccEjINtNtNtBc_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB4j_13StringWrapperB50_ccE0NCINvMs0_B3y_B3v_10wrap_mut_2jB3q_NCNvYIB2O_B3_B4e_EB1D_5count0E0E0INtNtB3A_12control_flow11ControlFlowB3v_jEEB4j_"}
!339 = distinct !{!339, !338, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB19_ENtNtNtBa_6traits8iterator8Iterator8try_foldjNCINvNvXs0_NtB8_10take_whileINtB2w_9TakeWhileppEB1D_8try_fold5checkTccEjINtNtNtBc_3ops9try_trait17NeverShortCircuitjENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB4j_13StringWrapperB50_ccE0NCINvMs0_B3y_B3v_10wrap_mut_2jB3q_NCNvYIB2O_B3_B4e_EB1D_5count0E0E0INtNtB3A_12control_flow11ControlFlowB3v_jEEB4j_: argument 1"}
!340 = distinct !{!340, !341, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator8try_foldjNCINvMs0_NtNtBc_3ops9try_traitINtB4k_17NeverShortCircuitjE10wrap_mut_2jTccENCNvYBV_B3t_5count0E0B4F_EB2i_: %self"}
!341 = distinct !{!341, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator8try_foldjNCINvMs0_NtNtBc_3ops9try_traitINtB4k_17NeverShortCircuitjE10wrap_mut_2jTccENCNvYBV_B3t_5count0E0B4F_EB2i_"}
!342 = distinct !{!342, !343, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_: %self"}
!343 = distinct !{!343, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters10take_whileINtB6_9TakeWhileINtNtB8_3zip3ZipINtNtB8_4take4TakeNtNtNtBc_3str4iter5CharsEB1J_ENCINvCs6KJnav5oeQt_6strsim20generic_jaro_winklerNtB2i_13StringWrapperB2Z_ccE0ENtNtNtBa_6traits8iterator8Iterator4foldjNCNvYBV_B3t_5count0EB2i_"}
!344 = !{!345, !347, !333, !335, !337, !339, !340, !342}
!345 = distinct !{!345, !346, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!346 = distinct !{!346, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!347 = distinct !{!347, !348, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!348 = distinct !{!348, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!349 = !{!350, !352}
!350 = distinct !{!350, !351, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!351 = distinct !{!351, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!352 = distinct !{!352, !353, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim: %_0"}
!353 = distinct !{!353, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim"}
!354 = !{!355, !357, !359, !361, !363, !365, !352}
!355 = distinct !{!355, !356, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim: %_1"}
!356 = distinct !{!356, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim"}
!357 = distinct !{!357, !358, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!358 = distinct !{!358, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim"}
!359 = distinct !{!359, !360, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim: %f"}
!360 = distinct !{!360, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim"}
!361 = distinct !{!361, !362, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim: %f"}
!362 = distinct !{!362, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim"}
!363 = distinct !{!363, !364, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim: %self"}
!364 = distinct !{!364, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim"}
!365 = distinct !{!365, !366, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim: %self"}
!366 = distinct !{!366, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim"}
!367 = distinct !{!367, !80, !81}
!368 = !{!352}
!369 = distinct !{!369, !81, !80}
!370 = !{!371, !373}
!371 = distinct !{!371, !372, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!372 = distinct !{!372, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!373 = distinct !{!373, !374, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim: %_0"}
!374 = distinct !{!374, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecjEINtB4_18SpecFromIterNestedjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE9from_iterCs6KJnav5oeQt_6strsim"}
!375 = !{!376, !378, !380, !382, !384, !386, !373}
!376 = distinct !{!376, !377, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim: %_1"}
!377 = distinct !{!377, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE0Cs6KJnav5oeQt_6strsim"}
!378 = distinct !{!378, !379, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim: %_1"}
!379 = distinct !{!379, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VecjE14extend_trustedINtNtNtBe_3ops5range5RangejEE0E0Cs6KJnav5oeQt_6strsim"}
!380 = distinct !{!380, !381, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim: %f"}
!381 = distinct !{!381, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator4folduNCINvNvBL_8for_each4calljNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB24_3VecjE14extend_trustedB3_E0E0ECs6KJnav5oeQt_6strsim"}
!382 = distinct !{!382, !383, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim: %f"}
!383 = distinct !{!383, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1I_3VecjE14extend_trustedB3_E0ECs6KJnav5oeQt_6strsim"}
!384 = distinct !{!384, !385, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim: %self"}
!385 = distinct !{!385, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecjE14extend_trustedINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs6KJnav5oeQt_6strsim"}
!386 = distinct !{!386, !387, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim: %self"}
!387 = distinct !{!387, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VecjEINtB4_10SpecExtendjINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEE11spec_extendCs6KJnav5oeQt_6strsim"}
!388 = distinct !{!388, !80, !81}
!389 = distinct !{!389, !81, !80}
!390 = !{!391, !393}
!391 = distinct !{!391, !392, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim: %_0"}
!392 = distinct !{!392, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6KJnav5oeQt_6strsim"}
!393 = distinct !{!393, !394, !"_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemjNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim: %_0"}
!394 = distinct !{!394, !"_RINvXs_NtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_elemjNtB5_12SpecFromElem9from_elemNtNtB9_5alloc6GlobalECs6KJnav5oeQt_6strsim"}
!395 = !{!396, !398, !400}
!396 = distinct !{!396, !397, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!397 = distinct !{!397, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!398 = distinct !{!398, !399, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!399 = distinct !{!399, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!400 = distinct !{!400, !401, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!401 = distinct !{!401, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!402 = !{!403, !405, !407}
!403 = distinct !{!403, !404, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!404 = distinct !{!404, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!405 = distinct !{!405, !406, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!406 = distinct !{!406, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!407 = distinct !{!407, !408, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!408 = distinct !{!408, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!409 = distinct !{!409, !27}
!410 = !{!411}
!411 = distinct !{!411, !412, !"_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_: %_0"}
!412 = distinct !{!412, !"_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2W_"}
!413 = !{!414}
!414 = distinct !{!414, !415, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2M_: %self"}
!415 = distinct !{!415, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0EEB2M_"}
!416 = !{!417}
!417 = distinct !{!417, !418, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2R_6StringINtNtB25_7collect6ExtendcE6extendB3_E0EB1p_: %f"}
!418 = distinct !{!418, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2R_6StringINtNtB25_7collect6ExtendcE6extendB3_E0EB1p_"}
!419 = !{!420}
!420 = distinct !{!420, !421, !"_RINvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filterINtB6_6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB27_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3k_6StringINtNtB2b_7collect6ExtendcE6extendBQ_E0E0EB1v_: %fold"}
!421 = distinct !{!421, !"_RINvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filterINtB6_6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB27_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3k_6StringINtNtB2b_7collect6ExtendcE6extendBQ_E0E0EB1v_"}
!422 = !{!423}
!423 = distinct !{!423, !424, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits8iterator8Iterator4folduNCINvNtNtBN_8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvBH_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3m_6StringINtNtBL_7collect6ExtendcE6extendINtB1x_6FilterB3_B29_EE0E0E0EB2d_: argument 0"}
!424 = distinct !{!424, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits8iterator8Iterator4folduNCINvNtNtBN_8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dice0NCINvNvBH_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3m_6StringINtNtBL_7collect6ExtendcE6extendINtB1x_6FilterB3_B29_EE0E0E0EB2d_"}
!425 = !{!426, !428, !423, !420, !417, !414, !411}
!426 = distinct !{!426, !427, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!427 = distinct !{!427, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!428 = distinct !{!428, !429, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!429 = distinct !{!429, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!430 = !{!423, !420, !417, !414, !411}
!431 = !{!432}
!432 = distinct !{!432, !433, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!433 = distinct !{!433, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!434 = !{!435, !432, !423, !420, !417, !414}
!435 = distinct !{!435, !436, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim: %self"}
!436 = distinct !{!436, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim"}
!437 = !{!432, !423, !420, !417, !414}
!438 = !{!432, !423, !420, !417, !414, !411}
!439 = !{!440}
!440 = distinct !{!440, !441, !"_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EEB2W_: %_0"}
!441 = distinct !{!441, !"_RINvXs5_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12FromIteratorcE9from_iterINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EEB2W_"}
!442 = !{!443}
!443 = distinct !{!443, !444, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EEB2M_: %self"}
!444 = distinct !{!444, !"_RINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB6_6StringINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendcE6extendINtNtNtBS_8adapters6filter6FilterNtNtNtBU_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0EEB2M_"}
!445 = !{!446}
!446 = distinct !{!446, !447, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2T_6StringINtNtB27_7collect6ExtendcE6extendB3_E0EB1p_: %f"}
!447 = distinct !{!447, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filter6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB2T_6StringINtNtB27_7collect6ExtendcE6extendB3_E0EB1p_"}
!448 = !{!449}
!449 = distinct !{!449, !450, !"_RINvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filterINtB6_6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB29_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3m_6StringINtNtB2d_7collect6ExtendcE6extendBQ_E0E0EB1v_: %fold"}
!450 = distinct !{!450, !"_RINvXs1_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters6filterINtB6_6FilterNtNtNtBc_3str4iter5CharsNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB29_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3m_6StringINtNtB2d_7collect6ExtendcE6extendBQ_E0E0EB1v_"}
!451 = !{!452}
!452 = distinct !{!452, !453, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits8iterator8Iterator4folduNCINvNtNtBN_8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvBH_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3o_6StringINtNtBL_7collect6ExtendcE6extendINtB1x_6FilterB3_B29_EE0E0E0EB2d_: argument 0"}
!453 = distinct !{!453, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5CharsNtNtNtNtB9_4iter6traits8iterator8Iterator4folduNCINvNtNtBN_8adapters6filter11filter_foldcuNCNvCs6KJnav5oeQt_6strsim13sorensen_dices_0NCINvNvBH_8for_each4callcNCINvXsd_NtCsdJPVW0sQgAG_5alloc6stringNtB3o_6StringINtNtBL_7collect6ExtendcE6extendINtB1x_6FilterB3_B29_EE0E0E0EB2d_"}
!454 = !{!455, !457, !452, !449, !446, !443, !440}
!455 = distinct !{!455, !456, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!456 = distinct !{!456, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!457 = distinct !{!457, !458, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!458 = distinct !{!458, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!459 = !{!452, !449, !446, !443, !440}
!460 = !{!461}
!461 = distinct !{!461, !462, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!462 = distinct !{!462, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!463 = !{!464, !461, !452, !449, !446, !443}
!464 = distinct !{!464, !465, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim: %self"}
!465 = distinct !{!465, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs6KJnav5oeQt_6strsim"}
!466 = !{!461, !452, !449, !446, !443}
!467 = !{!461, !452, !449, !446, !443, !440}
!468 = !{!469, !471}
!469 = distinct !{!469, !470, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6KJnav5oeQt_6strsim: %self.0"}
!470 = distinct !{!470, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6KJnav5oeQt_6strsim"}
!471 = distinct !{!471, !470, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6KJnav5oeQt_6strsim: %other.0"}
!472 = !{!473, !475, !477, !479}
!473 = distinct !{!473, !474, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim: %i"}
!474 = distinct !{!474, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim"}
!475 = distinct !{!475, !476, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs6KJnav5oeQt_6strsim: %__rust_std_internal_init"}
!476 = distinct !{!476, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs6KJnav5oeQt_6strsim"}
!477 = distinct !{!477, !478, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim: argument 0"}
!478 = distinct !{!478, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs6KJnav5oeQt_6strsim"}
!479 = distinct !{!479, !480, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs6KJnav5oeQt_6strsim: %_0"}
!480 = distinct !{!480, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs6KJnav5oeQt_6strsim"}
!481 = !{!479}
!482 = !{!483, !479}
!483 = distinct !{!483, !484, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim: argument 0"}
!484 = distinct !{!484, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs6KJnav5oeQt_6strsim"}
!485 = !{i32 0, i32 1114113}
!486 = !{!487}
!487 = distinct !{!487, !488, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim: %self"}
!488 = distinct !{!488, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim"}
!489 = !{!490, !491}
!490 = distinct !{!490, !488, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTTccEjEE14insert_no_growCs6KJnav5oeQt_6strsim: %value"}
!491 = distinct !{!491, !492, !"_RNvMs1a_NtNtNtCs5sEH5CPMdak_3std11collections4hash3mapINtB6_5EntryTccEjE9or_insertCs6KJnav5oeQt_6strsim: %self"}
!492 = distinct !{!492, !"_RNvMs1a_NtNtNtCs5sEH5CPMdak_3std11collections4hash3mapINtB6_5EntryTccEjE9or_insertCs6KJnav5oeQt_6strsim"}
!493 = !{!494, !496, !487, !490, !491}
!494 = distinct !{!494, !495, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!495 = distinct !{!495, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!496 = distinct !{!496, !497, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!497 = distinct !{!497, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!498 = !{!487, !490, !491}
!499 = !{!500, !502, !487, !490, !491}
!500 = distinct !{!500, !501, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!501 = distinct !{!501, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!502 = distinct !{!502, !503, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!503 = distinct !{!503, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!504 = !{!487, !491}
!505 = !{!506, !508}
!506 = distinct !{!506, !507, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!507 = distinct !{!507, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!508 = distinct !{!508, !509, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!509 = distinct !{!509, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!510 = !{!511, !513}
!511 = distinct !{!511, !512, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!512 = distinct !{!512, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!513 = distinct !{!513, !514, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!514 = distinct !{!514, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!515 = !{!516}
!516 = distinct !{!516, !517, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE4findNCNvMNtB8_11rustc_entryINtNtB8_3map7HashMapBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entry0ECs6KJnav5oeQt_6strsim: %self"}
!517 = distinct !{!517, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE4findNCNvMNtB8_11rustc_entryINtNtB8_3map7HashMapBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entry0ECs6KJnav5oeQt_6strsim"}
!518 = !{!519}
!519 = distinct !{!519, !520, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner10find_inner: %self"}
!520 = distinct !{!520, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner10find_inner"}
!521 = !{!519, !516}
!522 = !{!523}
!523 = distinct !{!523, !517, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE4findNCNvMNtB8_11rustc_entryINtNtB8_3map7HashMapBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entry0ECs6KJnav5oeQt_6strsim: argument 1"}
!524 = !{!525, !527, !519, !516, !523}
!525 = distinct !{!525, !526, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim: %_0"}
!526 = distinct !{!526, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs6KJnav5oeQt_6strsim"}
!527 = distinct !{!527, !528, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!528 = distinct !{!528, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!529 = !{!530, !519, !516, !523}
!530 = distinct !{!530, !531, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE4findNCNvMNtBa_11rustc_entryINtNtBa_3map7HashMapBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entry0E0Cs6KJnav5oeQt_6strsim: %_1"}
!531 = distinct !{!531, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTTccEjEE4findNCNvMNtBa_11rustc_entryINtNtBa_3map7HashMapBS_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE11rustc_entry0E0Cs6KJnav5oeQt_6strsim"}
!532 = !{!533}
!533 = distinct !{!533, !534, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim: %self"}
!534 = distinct !{!534, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim"}
!535 = !{!536}
!536 = distinct !{!536, !534, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTTccEjEE7reserveNCINvNtB8_3map11make_hasherBQ_jNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateE0ECs6KJnav5oeQt_6strsim: %hasher"}
!537 = !{i64 1}
!538 = !{i64 8}
!539 = !{!540}
!540 = distinct !{!540, !541, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!541 = distinct !{!541, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!542 = !{!543}
!543 = distinct !{!543, !544, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!544 = distinct !{!544, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!545 = !{!546, !543, !540}
!546 = distinct !{!546, !547, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!547 = distinct !{!547, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!548 = !{!543, !540}
!549 = !{!550, !543, !540}
!550 = distinct !{!550, !551, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!551 = distinct !{!551, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!552 = !{!553, !543, !540}
!553 = distinct !{!553, !554, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!554 = distinct !{!554, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!555 = !{!556, !543, !540}
!556 = distinct !{!556, !557, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!557 = distinct !{!557, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!558 = !{!559}
!559 = distinct !{!559, !560, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!560 = distinct !{!560, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4skipINtB4_4SkipNtNtNtBa_3str4iter5CharsENtNtNtB8_6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!561 = !{!562}
!562 = distinct !{!562, !563, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!563 = distinct !{!563, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!564 = !{!565}
!565 = distinct !{!565, !566, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!566 = distinct !{!566, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!567 = !{!568, !565, !562, !559}
!568 = distinct !{!568, !569, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!569 = distinct !{!569, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!570 = !{!565, !562, !559}
!571 = !{!572, !565, !562, !559}
!572 = distinct !{!572, !573, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!573 = distinct !{!573, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!574 = !{!575, !565, !562, !559}
!575 = distinct !{!575, !576, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!576 = distinct !{!576, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!577 = !{!578, !565, !562, !559}
!578 = distinct !{!578, !579, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!579 = distinct !{!579, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!580 = !{!581}
!581 = distinct !{!581, !582, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator10advance_by: %self"}
!582 = distinct !{!582, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator10advance_by"}
!583 = !{!584, !581}
!584 = distinct !{!584, !585, !"_RNvMs2C_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhE10make_sliceCs6KJnav5oeQt_6strsim: %self"}
!585 = distinct !{!585, !"_RNvMs2C_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhE10make_sliceCs6KJnav5oeQt_6strsim"}
!586 = !{!587, !589, !591}
!587 = distinct !{!587, !588, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!588 = distinct !{!588, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!589 = distinct !{!589, !590, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim: %bytes"}
!590 = distinct !{!590, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs6KJnav5oeQt_6strsim"}
!591 = distinct !{!591, !592, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!592 = distinct !{!592, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!593 = !{!594, !581}
!594 = distinct !{!594, !595, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs6KJnav5oeQt_6strsim: %self"}
!595 = distinct !{!595, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs6KJnav5oeQt_6strsim"}
!596 = !{!597, !581}
!597 = distinct !{!597, !598, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim: %self"}
!598 = distinct !{!598, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim"}
!599 = !{!600, !581}
!600 = distinct !{!600, !601, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim: %self"}
!601 = distinct !{!601, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim"}
!602 = !{!603, !581}
!603 = distinct !{!603, !604, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim: %self"}
!604 = distinct !{!604, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator10advance_byCs6KJnav5oeQt_6strsim"}
!605 = !{!591}
!606 = !{!589}
!607 = !{!589, !591}
!608 = !{!609, !589, !591}
!609 = distinct !{!609, !610, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!610 = distinct !{!610, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!611 = !{!612, !589, !591}
!612 = distinct !{!612, !613, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!613 = distinct !{!613, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
!614 = !{!615, !589, !591}
!615 = distinct !{!615, !616, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim: %self"}
!616 = distinct !{!616, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs6KJnav5oeQt_6strsim"}
