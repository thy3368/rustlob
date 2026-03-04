; ModuleID = 'indexmap.b8fc80d1c2920b3d-cgu.0'
source_filename = "indexmap.b8fc80d1c2920b3d-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_236b52f5c81143245b05b9fdb0db30b7 = private unnamed_addr constant [64 x i8] c" because the computed capacity exceeded the collection's maximum", align 1
@alloc_8743fc3ff085a87cd6d6fd0bc23a3ce4 = private unnamed_addr constant [47 x i8] c" because the memory allocator returned an error", align 1
@alloc_1c48aaa25881cebb537398237e87da2c = private unnamed_addr constant [24 x i8] c"memory allocation failed", align 1
@alloc_86e0a546768ba43498e57d2b08934e8c = private unnamed_addr constant [25 x i8] c"an index is out of bounds", align 1
@alloc_bc6df772543dbc3d2dfcfab4736ff0df = private unnamed_addr constant [30 x i8] c"there were overlapping indices", align 1

; <indexmap::TryReserveError as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_CsfSG22zIyQn9_8indexmapNtB5_15TryReserveErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = load i64, ptr %self, align 8, !range !2, !noundef !3
  switch i64 %_4, label %default.unreachable1 [
    i64 0, label %bb4
    i64 1, label %bb5
    i64 2, label %bb2
  ]

default.unreachable1:                             ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %e = getelementptr inbounds nuw i8, ptr %self, i64 8
; call <alloc::collections::TryReserveError as core::fmt::Display>::fmt
  %0 = tail call noundef zeroext i1 @_RNvXs2_NtCsdJPVW0sQgAG_5alloc11collectionsNtB5_15TryReserveErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %e, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb9

bb2:                                              ; preds = %start
  br label %bb5

bb5:                                              ; preds = %start, %bb2
  %reason.sroa.3.0 = phi i64 [ 47, %bb2 ], [ 64, %start ]
  %reason.sroa.0.0 = phi ptr [ @alloc_8743fc3ff085a87cd6d6fd0bc23a3ce4, %bb2 ], [ @alloc_236b52f5c81143245b05b9fdb0db30b7, %start ]
; call <core::fmt::Formatter>::write_str
  %_6 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1c48aaa25881cebb537398237e87da2c, i64 noundef 24)
  br i1 %_6, label %bb9, label %bb11

bb11:                                             ; preds = %bb5
; call <core::fmt::Formatter>::write_str
  %1 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %reason.sroa.0.0, i64 noundef %reason.sroa.3.0)
  br label %bb9

bb9:                                              ; preds = %bb5, %bb4, %bb11
  %_0.sroa.0.0.shrunk = phi i1 [ %0, %bb4 ], [ %1, %bb11 ], [ true, %bb5 ]
  ret i1 %_0.sroa.0.0.shrunk
}

; <indexmap::GetDisjointMutError as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs4_CsfSG22zIyQn9_8indexmapNtB5_19GetDisjointMutErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %0 = load i8, ptr %self, align 1, !range !4, !noundef !3
  %1 = trunc nuw i8 %0 to i1
  %. = select i1 %1, i64 30, i64 25
  %alloc_bc6df772543dbc3d2dfcfab4736ff0df.alloc_86e0a546768ba43498e57d2b08934e8c = select i1 %1, ptr @alloc_bc6df772543dbc3d2dfcfab4736ff0df, ptr @alloc_86e0a546768ba43498e57d2b08934e8c
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %alloc_bc6df772543dbc3d2dfcfab4736ff0df.alloc_86e0a546768ba43498e57d2b08934e8c, i64 noundef %., ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <alloc::collections::TryReserveError as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs2_NtCsdJPVW0sQgAG_5alloc11collectionsNtB5_15TryReserveErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{i64 0, i64 3}
!3 = !{}
!4 = !{i8 0, i8 2}
