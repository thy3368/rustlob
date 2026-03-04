; ModuleID = 'hashbrown.1ace5466b3dba021-cgu.0'
source_filename = "hashbrown.1ace5466b3dba021-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_70fa19237e466d7d1e23911705586cf6 = private unnamed_addr constant [28 x i8] c"Hash table capacity overflow", align 1
@alloc_1897607591628032f2256e272f99b5a1 = private unnamed_addr constant [104 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/hashbrown-0.16.0/src/raw/mod.rs\00", align 1
@alloc_7c89f28573094c81c1326326fa107f6f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_1897607591628032f2256e272f99b5a1, [16 x i8] c"g\00\00\00\00\00\00\00%\00\00\00(\00\00\00" }>, align 8
@alloc_3c6431c5fc85ca277eb9f2e0ebb30f52 = private unnamed_addr constant [4 x i8] c"full", align 1
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt }>, align 8
@alloc_4d692e00cdd6193df669076a38c2cf3f = private unnamed_addr constant [7 x i8] c"DELETED", align 1
@alloc_5ecff085f33eeeacaf38ec6a4e5e2caf = private unnamed_addr constant [5 x i8] c"EMPTY", align 1

; <hashbrown::raw::Fallibility>::capacity_overflow
; Function Attrs: uwtable
define { i64, i64 } @_RNvMNtCs2iGD339VwsL_9hashbrown3rawNtB2_11Fallibility17capacity_overflow(i1 noundef zeroext %self) unnamed_addr #0 {
start:
  br i1 %self, label %bb2, label %bb3, !prof !2

bb2:                                              ; preds = %start
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_70fa19237e466d7d1e23911705586cf6, ptr noundef nonnull inttoptr (i64 57 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_7c89f28573094c81c1326326fa107f6f) #8
  unreachable

bb3:                                              ; preds = %start
  ret { i64, i64 } { i64 0, i64 undef }
}

; <hashbrown::raw::Fallibility>::alloc_err
; Function Attrs: uwtable
define { i64, i64 } @_RNvMNtCs2iGD339VwsL_9hashbrown3rawNtB2_11Fallibility9alloc_err(i1 noundef zeroext %self, i64 noundef range(i64 1, -9223372036854775807) %layout.0, i64 noundef %layout.1) unnamed_addr #0 {
start:
  br i1 %self, label %bb2, label %bb3, !prof !2

bb2:                                              ; preds = %start
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef %layout.0, i64 noundef %layout.1) #9
  unreachable

bb3:                                              ; preds = %start
  %0 = insertvalue { i64, i64 } poison, i64 %layout.0, 0
  %1 = insertvalue { i64, i64 } %0, i64 %layout.1, 1
  ret { i64, i64 } %1
}

; <hashbrown::raw::RawIterHashInner>::new
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(read, argmem: readwrite, inaccessiblemem: none) uwtable
define void @_RNvMsP_NtCs2iGD339VwsL_9hashbrown3rawNtB5_16RawIterHashInner3new(ptr dead_on_unwind noalias noundef writable writeonly sret([56 x i8]) align 8 captures(none) dereferenceable(56) initializes((0, 49)) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %table, i64 noundef %hash) unnamed_addr #1 {
start:
  %_10 = lshr i64 %hash, 57
  %_11 = trunc nuw nsw i64 %_10 to i8
  %0 = getelementptr inbounds nuw i8, ptr %table, i64 8
  %_15 = load i64, ptr %0, align 8, !noundef !3
  %_13 = and i64 %_15, %hash
  %_18 = load ptr, ptr %table, align 8, !nonnull !3, !noundef !3
  %_16 = getelementptr inbounds nuw i8, ptr %_18, i64 %_13
  %tmp.sroa.0.0.copyload.i.i = load <8 x i8>, ptr %_16, align 1, !noalias !4
  %1 = insertelement <1 x i8> poison, i8 %_11, i64 0
  %2 = shufflevector <1 x i8> %1, <1 x i8> poison, <8 x i32> zeroinitializer
  %3 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i, %2
  %4 = sext <8 x i1> %3 to <8 x i8>
  %5 = bitcast <8 x i8> %4 to i64
  %_22 = and i64 %5, -9187201950435737472
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_15, ptr %6, align 8
  store ptr %_18, ptr %_0, align 8
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i8 %_11, ptr %7, align 8
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_13, ptr %8, align 8
  %9 = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 0, ptr %9, align 8
  %10 = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store <8 x i8> %tmp.sroa.0.0.copyload.i.i, ptr %10, align 8
  %11 = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store i64 %_22, ptr %11, align 8
  ret void
}

; <hashbrown::raw::RawIterHashInner as core::iter::traits::iterator::Iterator>::next
; Function Attrs: nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: none) uwtable
define { i64, i64 } @_RNvXsR_NtCs2iGD339VwsL_9hashbrown3rawNtB5_16RawIterHashInnerNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self) unnamed_addr #2 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %.promoted = load i64, ptr %0, align 8
  %.not10 = icmp eq i64 %.promoted, 0
  br i1 %.not10, label %bb6.lr.ph, label %start.bb5_crit_edge

start.bb5_crit_edge:                              ; preds = %start
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_5.pre = load i64, ptr %.phi.trans.insert, align 8
  %.phi.trans.insert17 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_6.pre = load i64, ptr %.phi.trans.insert17, align 8
  br label %bb5

bb6.lr.ph:                                        ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_9 = load i64, ptr %2, align 8
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_14 = load ptr, ptr %self, align 8, !nonnull !3
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %6 = load <8 x i8>, ptr %5, align 8
  %7 = shufflevector <8 x i8> %6, <8 x i8> poison, <8 x i32> zeroinitializer
  %.promoted12 = load <8 x i8>, ptr %1, align 8
  %.promoted14 = load i64, ptr %4, align 8
  %.promoted15 = load i64, ptr %3, align 8
  br label %bb6

bb5:                                              ; preds = %bb10, %start.bb5_crit_edge
  %_6 = phi i64 [ %_6.pre, %start.bb5_crit_edge ], [ %_9, %bb10 ]
  %_5 = phi i64 [ %_5.pre, %start.bb5_crit_edge ], [ %19, %bb10 ]
  %_31.lcssa = phi i64 [ %.promoted, %start.bb5_crit_edge ], [ %_39, %bb10 ]
  %8 = add i64 %_31.lcssa, -1
  %9 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %_31.lcssa, i1 true)
  %_253 = lshr i64 %9, 3
  %_29 = and i64 %8, %_31.lcssa
  store i64 %_29, ptr %0, align 8
  %_4 = add i64 %_5, %_253
  %index = and i64 %_4, %_6
  br label %bb3

bb6:                                              ; preds = %bb6.lr.ph, %bb10
  %10 = phi i64 [ %.promoted15, %bb6.lr.ph ], [ %19, %bb10 ]
  %11 = phi i64 [ %.promoted14, %bb6.lr.ph ], [ %17, %bb10 ]
  %tmp.sroa.0.0.copyload.i.i13 = phi <8 x i8> [ %.promoted12, %bb6.lr.ph ], [ %tmp.sroa.0.0.copyload.i.i, %bb10 ]
  %12 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i13, splat (i8 -1)
  %13 = bitcast <8 x i1> %12 to i8
  %14 = icmp eq i8 %13, 0
  br i1 %14, label %bb10, label %bb3, !prof !2

bb3:                                              ; preds = %bb6, %bb5
  %_0.sroa.3.0 = phi i64 [ %index, %bb5 ], [ undef, %bb6 ]
  %_0.sroa.0.0 = phi i64 [ 1, %bb5 ], [ 0, %bb6 ]
  %15 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0, 0
  %16 = insertvalue { i64, i64 } %15, i64 %_0.sroa.3.0, 1
  ret { i64, i64 } %16

bb10:                                             ; preds = %bb6
  %17 = add i64 %11, 8
  store i64 %17, ptr %4, align 8
  %18 = add i64 %10, %17
  %19 = and i64 %18, %_9
  store i64 %19, ptr %3, align 8
  %_12 = getelementptr inbounds nuw i8, ptr %_14, i64 %19
  %tmp.sroa.0.0.copyload.i.i = load <8 x i8>, ptr %_12, align 1, !noalias !9
  store <8 x i8> %tmp.sroa.0.0.copyload.i.i, ptr %1, align 8
  %20 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i, %7
  %21 = sext <8 x i1> %20 to <8 x i8>
  %22 = bitcast <8 x i8> %21 to i64
  %_39 = and i64 %22, -9187201950435737472
  store i64 %_39, ptr %0, align 8
  %.not = icmp eq i64 %_39, 0
  br i1 %.not, label %bb6, label %bb5
}

; <u8 as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !3
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_5 = and i32 %_4, 67108864
  %2 = icmp eq i32 %_5, 0
  br i1 %2, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call <u8 as core::fmt::LowerHex>::fmt
  %3 = tail call noundef zeroext i1 @_RNvXse_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb4:                                              ; preds = %bb2
; call <u8 as core::fmt::Display>::fmt
  %4 = tail call noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb3:                                              ; preds = %bb2
; call <u8 as core::fmt::UpperHex>::fmt
  %5 = tail call noundef zeroext i1 @_RNvXsg_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb3, %bb1
  %_0.sroa.0.0.in = phi i1 [ %4, %bb4 ], [ %5, %bb3 ], [ %3, %bb1 ]
  ret i1 %_0.sroa.0.0.in
}

; <hashbrown::control::tag::Tag as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs_NtNtCs2iGD339VwsL_9hashbrown7control3tagNtB4_3TagNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_8 = alloca [1 x i8], align 1
  %_5 = alloca [24 x i8], align 8
  %_12 = load i8, ptr %self, align 1, !noundef !3
  %0 = icmp sgt i8 %_12, -1
  br i1 %0, label %bb5, label %bb1

bb5:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call <core::fmt::Formatter>::debug_tuple
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter11debug_tuple(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_5, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3c6431c5fc85ca277eb9f2e0ebb30f52, i64 noundef 4)
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %_8)
  store i8 %_12, ptr %_8, align 1
; call <core::fmt::builders::DebugTuple>::field
  %_3 = call noundef nonnull align 8 ptr @_RNvMs2_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_10DebugTuple5field(ptr noalias noundef nonnull align 8 dereferenceable(24) %_5, ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0)
; call <core::fmt::builders::DebugTuple>::finish
  %1 = call noundef zeroext i1 @_RNvMs2_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_10DebugTuple6finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %_3)
  call void @llvm.lifetime.end.p0(i64 1, ptr nonnull %_8)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  br label %bb9

bb1:                                              ; preds = %start
  %_11 = and i8 %_12, 1
  %2 = icmp eq i8 %_11, 0
  br i1 %2, label %bb3, label %bb2

bb9:                                              ; preds = %bb3, %bb2, %bb5
  %_0.sroa.0.0.in = phi i1 [ %1, %bb5 ], [ %3, %bb3 ], [ %4, %bb2 ]
  ret i1 %_0.sroa.0.0.in

bb3:                                              ; preds = %bb1
; call <core::fmt::Formatter>::pad
  %3 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4d692e00cdd6193df669076a38c2cf3f, i64 noundef 7)
  br label %bb9

bb2:                                              ; preds = %bb1
; call <core::fmt::Formatter>::pad
  %4 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5ecff085f33eeeacaf38ec6a4e5e2caf, i64 noundef 5)
  br label %bb9
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #4

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #4

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #5

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.cttz.i64(i64, i1 immarg) #7

; <u8 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <u8 as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsg_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <u8 as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXse_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <core::fmt::Formatter>::debug_tuple
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter11debug_tuple(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::fmt::builders::DebugTuple>::field
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs2_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_10DebugTuple5field(ptr noalias noundef align 8 dereferenceable(24), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; <core::fmt::builders::DebugTuple>::finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs2_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_10DebugTuple6finish(ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <core::fmt::Formatter>::pad
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { mustprogress nofree norecurse nosync nounwind willreturn memory(read, argmem: readwrite, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #5 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #8 = { noinline noreturn }
attributes #9 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!3 = !{}
!4 = !{!5, !7}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs2iGD339VwsL_9hashbrown: %_0"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs2iGD339VwsL_9hashbrown"}
!7 = distinct !{!7, !8, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!8 = distinct !{!8, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!9 = !{!10, !12}
!10 = distinct !{!10, !11, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs2iGD339VwsL_9hashbrown: %_0"}
!11 = distinct !{!11, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs2iGD339VwsL_9hashbrown"}
!12 = distinct !{!12, !13, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!13 = distinct !{!13, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
