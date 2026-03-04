; ModuleID = 'colorchoice.2324c1e8c78babf4-cgu.0'
source_filename = "colorchoice.2324c1e8c78babf4-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@_RNvCs314fcNNe6oa_11colorchoice4USER.0 = internal unnamed_addr global i64 0, align 8
@alloc_1125b2aae17333bd2866806796b44645 = private unnamed_addr constant [38 x i8] c"Only `ColorChoice` values can be `set`", align 1
@alloc_93a3560b7fe3b784b87743f41d1e6457 = private unnamed_addr constant [101 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/colorchoice-1.0.4/src/lib.rs\00", align 1
@alloc_91adbfeb86335231219a2a155043bfec = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_93a3560b7fe3b784b87743f41d1e6457, [16 x i8] c"d\00\00\00\00\00\00\00:\00\00\00!\00\00\00" }>, align 8

; <colorchoice::ColorChoice>::write_global
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(readwrite, argmem: none, inaccessiblemem: none) uwtable
define void @_RNvMCs314fcNNe6oa_11colorchoiceNtB2_11ColorChoice12write_global(i8 noundef range(i8 0, 4) %self) unnamed_addr #0 {
start:
  %_4.i = zext nneg i8 %self to i64
  store atomic i64 %_4.i, ptr @_RNvCs314fcNNe6oa_11colorchoice4USER.0 seq_cst, align 8
  ret void
}

; <colorchoice::ColorChoice>::global
; Function Attrs: uwtable
define noundef range(i8 0, 4) i8 @_RNvMCs314fcNNe6oa_11colorchoiceNtB2_11ColorChoice6global() unnamed_addr #1 {
start:
  %0 = load atomic i64, ptr @_RNvCs314fcNNe6oa_11colorchoice4USER.0 seq_cst, align 8
  %1 = icmp ult i64 %0, 4
  br i1 %1, label %switch.lookup, label %bb2.i

bb2.i:                                            ; preds = %start
; call core::option::expect_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1125b2aae17333bd2866806796b44645, i64 noundef 38, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_91adbfeb86335231219a2a155043bfec) #3
  unreachable

switch.lookup:                                    ; preds = %start
  %switch.idx.cast = trunc nuw i64 %0 to i8
  ret i8 %switch.idx.cast
}

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #2

attributes #0 = { mustprogress nofree norecurse nounwind willreturn memory(readwrite, argmem: none, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { noinline noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
