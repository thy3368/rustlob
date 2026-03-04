; ModuleID = 'anstyle_parse.48e8ae6dbed00af-cgu.0'
source_filename = "anstyle_parse.48e8ae6dbed00af-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_5043689c4ea8ad870b7940bac900b587 = private unnamed_addr constant [106 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-parse-0.2.7/src/params.rs\00", align 1
@alloc_17566ec506236c08945a72c124ffd1b2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5043689c4ea8ad870b7940bac900b587, [16 x i8] c"i\00\00\00\00\00\00\00l\00\00\00\1D\00\00\00" }>, align 8
@alloc_1447189394eca183925b24f3331ea4c7 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5043689c4ea8ad870b7940bac900b587, [16 x i8] c"i\00\00\00\00\00\00\00m\00\00\00(\00\00\00" }>, align 8
@alloc_036636151bab3754536335311c8d3ab8 = private unnamed_addr constant [1 x i8] c"[", align 1
@alloc_7a6887ef0f93938f57a4bb958cf80311 = private unnamed_addr constant [1 x i8] c"]", align 1
@alloc_67ff1e09660ce038a33d9b00e9fc869e = private unnamed_addr constant [1 x i8] c";", align 1
@alloc_3b8c95b91c663a5a9387bddd44e7b465 = private unnamed_addr constant [1 x i8] c":", align 1
@alloc_a956815dd852e1d78e37a53a853df475 = private unnamed_addr constant [84 x i8] c"internal error: entered unreachable code: multi-byte UTF8 characters are unsupported", align 1
@alloc_b44513a0b66af0fb6f9c0ed252baba65 = private unnamed_addr constant [103 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anstyle-parse-0.2.7/src/lib.rs\00", align 1
@alloc_4f3f6d8ef579fdebf1ff0939b9a2fbf8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b44513a0b66af0fb6f9c0ed252baba65, [16 x i8] c"f\00\00\00\00\00\00\00W\01\00\00\09\00\00\00" }>, align 8

; <anstyle_parse::Utf8Parser as anstyle_parse::CharAccumulator>::add
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable
define noundef range(i32 0, 1114113) i32 @_RNvXs0_CsofRcISb9et_13anstyle_parseNtB5_10Utf8ParserNtB5_15CharAccumulator3add(ptr noalias noundef align 4 captures(none) dereferenceable(8) %self, i8 noundef %byte) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 4
  %_7.i = load i8, ptr %0, align 4, !range !2, !alias.scope !3, !noundef !6
  switch i8 %_7.i, label %default.unreachable [
    i8 0, label %bb9.i.i
    i8 1, label %bb4.i.i
    i8 2, label %bb3.i.i
    i8 3, label %bb2.i.i
    i8 4, label %bb8.i.i
    i8 5, label %bb7.i.i
    i8 6, label %bb6.i.i
    i8 7, label %bb5.i.i
  ]

default.unreachable:                              ; preds = %start
  unreachable

bb9.i.i:                                          ; preds = %start
  %_13.i.i = icmp sgt i8 %byte, -1
  br i1 %_13.i.i, label %bb8.i7.i, label %bb12.i.i

bb4.i.i:                                          ; preds = %start
  %or.cond5.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond5.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %bb9.i8.i

bb3.i.i:                                          ; preds = %start
  %or.cond6.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond6.i.i, label %bb6.i5.i, label %bb9.i8.i

bb2.i.i:                                          ; preds = %start
  %or.cond7.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond7.i.i, label %bb7.i6.i, label %bb9.i8.i

bb8.i.i:                                          ; preds = %start
  %1 = and i8 %byte, -32
  %or.cond8.i.i = icmp eq i8 %1, -96
  br i1 %or.cond8.i.i, label %bb6.i5.i, label %bb9.i8.i

bb7.i.i:                                          ; preds = %start
  %or.cond9.i.i = icmp slt i8 %byte, -96
  br i1 %or.cond9.i.i, label %bb6.i5.i, label %bb9.i8.i

bb6.i.i:                                          ; preds = %start
  %2 = add i8 %byte, 112
  %or.cond10.i.i = icmp ult i8 %2, 48
  br i1 %or.cond10.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %bb9.i8.i

bb5.i.i:                                          ; preds = %start
  %or.cond11.i.i = icmp slt i8 %byte, -112
  br i1 %or.cond11.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %bb9.i8.i

bb12.i.i:                                         ; preds = %bb9.i.i
  %3 = add nsw i8 %byte, 62
  %or.cond1.i.i = icmp ult i8 %3, 30
  br i1 %or.cond1.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i, label %bb14.i.i

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i: ; preds = %bb12.i.i
  %_20.i.i = and i8 %byte, 31
  %_19.i.i = zext nneg i8 %_20.i.i to i32
  %_18.i.i = shl nuw nsw i32 %_19.i.i, 6
  %4 = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %5 = or i32 %4, %_18.i.i
  store i32 %5, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

bb14.i.i:                                         ; preds = %bb12.i.i
  switch i8 %byte, label %bb15.i.i [
    i8 -32, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i
    i8 -19, label %bb28.i.i
    i8 -16, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i
    i8 -12, label %bb26.i.i
  ]

bb15.i.i:                                         ; preds = %bb14.i.i
  %6 = add nsw i8 %byte, 31
  %or.cond2.i.i = icmp ult i8 %6, 12
  %7 = and i8 %byte, -2
  %or.cond3.i.i = icmp eq i8 %7, -18
  %or.cond.i.i = or i1 %or.cond2.i.i, %or.cond3.i.i
  br i1 %or.cond.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i, label %bb19.i.i

bb28.i.i:                                         ; preds = %bb14.i.i
  br label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i

bb26.i.i:                                         ; preds = %bb14.i.i
  br label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i

bb19.i.i:                                         ; preds = %bb15.i.i
  %8 = add nsw i8 %byte, 15
  %or.cond4.i.i = icmp ult i8 %8, 3
  br i1 %or.cond4.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i, label %bb9.i8.i

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i: ; preds = %bb5.i.i, %bb6.i.i, %bb4.i.i
  %_23.i.i = and i8 %byte, 63
  %_22.i.i = zext nneg i8 %_23.i.i to i32
  %_21.i.i = shl nuw nsw i32 %_22.i.i, 12
  %9 = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %10 = or i32 %9, %_21.i.i
  store i32 %10, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i: ; preds = %bb28.i.i, %bb15.i.i, %bb14.i.i
  %_0.sroa.0.0.i.ph16.i = phi i8 [ 2, %bb15.i.i ], [ 4, %bb14.i.i ], [ 5, %bb28.i.i ]
  %_26.i.i = and i8 %byte, 15
  %_25.i.i = zext nneg i8 %_26.i.i to i32
  %_24.i.i = shl nuw nsw i32 %_25.i.i, 12
  %11 = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %12 = or i32 %11, %_24.i.i
  store i32 %12, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i: ; preds = %bb19.i.i, %bb26.i.i, %bb14.i.i
  %_0.sroa.0.0.i.ph22.i = phi i8 [ 7, %bb26.i.i ], [ 6, %bb14.i.i ], [ 1, %bb19.i.i ]
  %_29.i.i = and i8 %byte, 7
  %_28.i.i = zext nneg i8 %_29.i.i to i32
  %_27.i.i = shl nuw nsw i32 %_28.i.i, 18
  %13 = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %14 = or i32 %13, %_27.i.i
  store i32 %14, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

bb9.i8.i:                                         ; preds = %bb3.i.i, %bb8.i.i, %bb7.i.i, %bb19.i.i, %bb5.i.i, %bb6.i.i, %bb2.i.i, %bb4.i.i
  store i32 0, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

bb8.i7.i:                                         ; preds = %bb9.i.i
  %_8.i.i = zext nneg i8 %byte to i32
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

bb7.i6.i:                                         ; preds = %bb2.i.i
  %_10.i.i = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %_12.i.i = and i8 %byte, 63
  %_11.i.i = zext nneg i8 %_12.i.i to i32
  %point.i.i = or i32 %_10.i.i, %_11.i.i
  %15 = icmp ult i32 %_10.i.i, 1114112
  tail call void @llvm.assume(i1 %15)
  store i32 0, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

bb6.i5.i:                                         ; preds = %bb7.i.i, %bb8.i.i, %bb3.i.i
  %_17.i.i = and i8 %byte, 63
  %_16.i.i = zext nneg i8 %_17.i.i to i32
  %_15.i.i = shl nuw nsw i32 %_16.i.i, 6
  %16 = load i32, ptr %self, align 4, !alias.scope !7, !noundef !6
  %17 = or i32 %16, %_15.i.i
  store i32 %17, ptr %self, align 4, !alias.scope !7
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit

_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_.exit: ; preds = %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i, %bb9.i8.i, %bb8.i7.i, %bb7.i6.i, %bb6.i5.i
  %c.sroa.0.0 = phi i32 [ %_8.i.i, %bb8.i7.i ], [ 1114112, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i ], [ 1114112, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i ], [ 1114112, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i ], [ 65533, %bb9.i8.i ], [ 1114112, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i ], [ 1114112, %bb6.i5.i ], [ %point.i.i, %bb7.i6.i ]
  %_0.sroa.0.0.i7.i = phi i8 [ 0, %bb8.i7.i ], [ 3, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i ], [ %_0.sroa.0.0.i.ph16.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i ], [ %_0.sroa.0.0.i.ph22.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i ], [ 0, %bb9.i8.i ], [ 2, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i ], [ 3, %bb6.i5.i ], [ 0, %bb7.i6.i ]
  store i8 %_0.sroa.0.0.i7.i, ptr %0, align 4, !alias.scope !3
  ret i32 %c.sroa.0.0
}

; <anstyle_parse::params::ParamsIter as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
define { ptr, i64 } @_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3 = load i64, ptr %0, align 8, !noundef !6
  %_14 = load ptr, ptr %self, align 8, !nonnull !6, !align !10, !noundef !6
  %1 = getelementptr inbounds nuw i8, ptr %_14, i64 96
  %_4 = load i64, ptr %1, align 8, !noundef !6
  %_2.not = icmp ult i64 %_3, %_4
  br i1 %_2.not, label %bb2, label %bb4

bb2:                                              ; preds = %start
  %_7 = icmp ult i64 %_3, 32
  br i1 %_7, label %bb3, label %panic

bb3:                                              ; preds = %bb2
  %2 = getelementptr inbounds nuw i8, ptr %_14, i64 64
  %3 = getelementptr inbounds nuw i8, ptr %2, i64 %_3
  %num_subparams = load i8, ptr %3, align 1, !noundef !6
  %_13 = zext i8 %num_subparams to i64
  %_11 = add nuw nsw i64 %_3, %_13
  %_18 = icmp samesign ult i64 %_11, 33
  br i1 %_18, label %bb5, label %bb6, !prof !11

panic:                                            ; preds = %bb2
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %_3, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_17566ec506236c08945a72c124ffd1b2) #7
  unreachable

bb6:                                              ; preds = %bb3
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %_3, i64 noundef %_11, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1447189394eca183925b24f3331ea4c7) #7
  unreachable

bb5:                                              ; preds = %bb3
  %_25 = getelementptr inbounds nuw i16, ptr %_14, i64 %_3
  store i64 %_11, ptr %0, align 8
  br label %bb4

bb4:                                              ; preds = %start, %bb5
  %_0.sroa.3.0 = phi i64 [ %_13, %bb5 ], [ undef, %start ]
  %_0.sroa.0.0 = phi ptr [ %_25, %bb5 ], [ null, %start ]
  %4 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0, 0
  %5 = insertvalue { ptr, i64 } %4, i64 %_0.sroa.3.0, 1
  ret { ptr, i64 } %5
}

; <anstyle_parse::params::Params as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_6ParamsNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(112) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_38.0 = load ptr, ptr %f, align 8, !nonnull !6, !align !12, !noundef !6
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_38.1 = load ptr, ptr %0, align 8, !nonnull !6, !align !10, !noundef !6
  %1 = getelementptr inbounds nuw i8, ptr %_38.1, i64 24
  %2 = load ptr, ptr %1, align 8, !invariant.load !6, !nonnull !6
  %3 = tail call noundef zeroext i1 %2(ptr noundef nonnull align 1 %_38.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_036636151bab3754536335311c8d3ab8, i64 noundef 1) #8
  br i1 %3, label %bb17, label %bb2.preheader

bb2.preheader:                                    ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 96
  %_4.i.i = load i64, ptr %4, align 8, !noalias !13, !noundef !6
  %_2.not.i.i33.not = icmp eq i64 %_4.i.i, 0
  br i1 %_2.not.i.i33.not, label %bb5, label %bb2.i.i.lr.ph

bb2.i.i.lr.ph:                                    ; preds = %bb2.preheader
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 64
  %6 = getelementptr inbounds nuw i8, ptr %f, i64 16
  br label %bb2.i.i

bb2.loopexit:                                     ; preds = %bb8.backedge, %bb8.backedge.peel, %bb7
  %_2.not.i.i = icmp ult i64 %_11.i.i, %_4.i.i
  br i1 %_2.not.i.i, label %bb2.i.i, label %bb5.loopexit

bb2.i.i:                                          ; preds = %bb2.i.i.lr.ph, %bb2.loopexit
  %iter.sroa.5.035 = phi i64 [ 0, %bb2.i.i.lr.ph ], [ %_11.i.i, %bb2.loopexit ]
  %iter.sroa.8.034 = phi i64 [ 0, %bb2.i.i.lr.ph ], [ %_8.0.i, %bb2.loopexit ]
  %_7.i.i = icmp samesign ult i64 %iter.sroa.5.035, 32
  br i1 %_7.i.i, label %bb3.i.i, label %panic.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %7 = getelementptr inbounds nuw i8, ptr %5, i64 %iter.sroa.5.035
  %num_subparams.i.i = load i8, ptr %7, align 1, !noalias !13, !noundef !6
  %_13.i.i = zext i8 %num_subparams.i.i to i64
  %_11.i.i = add nuw nsw i64 %iter.sroa.5.035, %_13.i.i
  %_18.i.i = icmp samesign ult i64 %_11.i.i, 33
  br i1 %_18.i.i, label %bb4, label %bb6.i.i, !prof !11

panic.i.i:                                        ; preds = %bb2.i.i
; call core::panicking::panic_bounds_check
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef %iter.sroa.5.035, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_17566ec506236c08945a72c124ffd1b2) #7, !noalias !13
  unreachable

bb6.i.i:                                          ; preds = %bb3.i.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %iter.sroa.5.035, i64 noundef %_11.i.i, i64 noundef 32, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1447189394eca183925b24f3331ea4c7) #7, !noalias !13
  unreachable

bb4:                                              ; preds = %bb3.i.i
  %_25.i.i = getelementptr inbounds nuw i16, ptr %self, i64 %iter.sroa.5.035
  %_8.0.i = add i64 %iter.sroa.8.034, 1
  %8 = icmp eq i64 %iter.sroa.8.034, 0
  br i1 %8, label %bb7, label %bb58

bb5.loopexit:                                     ; preds = %bb2.loopexit
  %_118.0.pre = load ptr, ptr %f, align 8
  %_118.1.pre = load ptr, ptr %0, align 8
  %.phi.trans.insert = getelementptr inbounds nuw i8, ptr %_118.1.pre, i64 24
  %.pre = load ptr, ptr %.phi.trans.insert, align 8, !invariant.load !6
  br label %bb5

bb5:                                              ; preds = %bb5.loopexit, %bb2.preheader
  %9 = phi ptr [ %.pre, %bb5.loopexit ], [ %2, %bb2.preheader ]
  %_118.0 = phi ptr [ %_118.0.pre, %bb5.loopexit ], [ %_38.0, %bb2.preheader ]
  %10 = tail call noundef zeroext i1 %9(ptr noundef nonnull align 1 %_118.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_7a6887ef0f93938f57a4bb958cf80311, i64 noundef 1) #8
  br label %bb17

bb17:                                             ; preds = %bb58, %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit.peel, %bb3.i.peel, %bb4.i.peel, %bb3.i, %bb4.i, %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit, %bb56, %start, %bb5
  %_0.sroa.0.0.off0 = phi i1 [ %10, %bb5 ], [ true, %start ], [ true, %bb56 ], [ true, %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit ], [ true, %bb4.i ], [ true, %bb3.i ], [ true, %bb4.i.peel ], [ true, %bb3.i.peel ], [ true, %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit.peel ], [ true, %bb58 ]
  ret i1 %_0.sroa.0.0.off0

bb7:                                              ; preds = %bb58, %bb4
  %_87.idx = shl nuw nsw i64 %_13.i.i, 1
  %_87 = getelementptr inbounds nuw i8, ptr %_25.i.i, i64 %_87.idx
  %_6.i.i30 = icmp eq i8 %num_subparams.i.i, 0
  br i1 %_6.i.i30, label %bb2.loopexit, label %bb13.peel

bb13.peel:                                        ; preds = %bb7
  %_16.i.i.peel = getelementptr inbounds nuw i8, ptr %_25.i.i, i64 2
  %_4.i.peel.pre = load i32, ptr %6, align 8, !alias.scope !19, !noalias !22
  %_3.i.peel = and i32 %_4.i.peel.pre, 33554432
  %11 = icmp eq i32 %_3.i.peel, 0
  br i1 %11, label %bb2.i.peel, label %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit.peel

_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit.peel: ; preds = %bb13.peel
; call <u16 as core::fmt::LowerHex>::fmt
  %12 = tail call noundef zeroext i1 @_RNvXsm_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %_25.i.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %12, label %bb17, label %bb8.backedge.peel

bb2.i.peel:                                       ; preds = %bb13.peel
  %_5.i.peel = and i32 %_4.i.peel.pre, 67108864
  %13 = icmp eq i32 %_5.i.peel, 0
  br i1 %13, label %bb4.i.peel, label %bb3.i.peel

bb3.i.peel:                                       ; preds = %bb2.i.peel
; call <u16 as core::fmt::UpperHex>::fmt
  %14 = tail call noundef zeroext i1 @_RNvXso_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %_25.i.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %14, label %bb17, label %bb8.backedge.peel

bb4.i.peel:                                       ; preds = %bb2.i.peel
; call <u16 as core::fmt::Display>::fmt
  %15 = tail call noundef zeroext i1 @_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3imptNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %_25.i.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %15, label %bb17, label %bb8.backedge.peel

bb8.backedge.peel:                                ; preds = %bb4.i.peel, %bb3.i.peel, %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit.peel
  %_6.i.i.peel = icmp eq i8 %num_subparams.i.i, 1
  br i1 %_6.i.i.peel, label %bb2.loopexit, label %bb56

bb58:                                             ; preds = %bb4
  %_71.0 = load ptr, ptr %f, align 8, !nonnull !6, !align !12, !noundef !6
  %_71.1 = load ptr, ptr %0, align 8, !nonnull !6, !align !10, !noundef !6
  %16 = getelementptr inbounds nuw i8, ptr %_71.1, i64 24
  %17 = load ptr, ptr %16, align 8, !invariant.load !6, !nonnull !6
  %18 = tail call noundef zeroext i1 %17(ptr noundef nonnull align 1 %_71.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_67ff1e09660ce038a33d9b00e9fc869e, i64 noundef 1) #8
  br i1 %18, label %bb17, label %bb7

bb13:                                             ; preds = %bb56
  %_4.i = load i32, ptr %6, align 8, !alias.scope !19, !noalias !22, !noundef !6
  %_3.i = and i32 %_4.i, 33554432
  %19 = icmp eq i32 %_3.i, 0
  br i1 %19, label %bb2.i, label %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit

bb2.i:                                            ; preds = %bb13
  %_5.i = and i32 %_4.i, 67108864
  %20 = icmp eq i32 %_5.i, 0
  br i1 %20, label %bb4.i, label %bb3.i

bb4.i:                                            ; preds = %bb2.i
; call <u16 as core::fmt::Display>::fmt
  %21 = tail call noundef zeroext i1 @_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3imptNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %iter1.sroa.0.031, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %21, label %bb17, label %bb8.backedge

bb3.i:                                            ; preds = %bb2.i
; call <u16 as core::fmt::UpperHex>::fmt
  %22 = tail call noundef zeroext i1 @_RNvXso_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %iter1.sroa.0.031, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %22, label %bb17, label %bb8.backedge

_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit: ; preds = %bb13
; call <u16 as core::fmt::LowerHex>::fmt
  %23 = tail call noundef zeroext i1 @_RNvXsm_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %iter1.sroa.0.031, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %23, label %bb17, label %bb8.backedge

bb8.backedge:                                     ; preds = %_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt.exit, %bb4.i, %bb3.i
  %_6.i.i = icmp eq ptr %_16.i.i, %_87
  br i1 %_6.i.i, label %bb2.loopexit, label %bb56, !llvm.loop !24

bb56:                                             ; preds = %bb8.backedge.peel, %bb8.backedge
  %iter1.sroa.0.031 = phi ptr [ %_16.i.i, %bb8.backedge ], [ %_16.i.i.peel, %bb8.backedge.peel ]
  %_16.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.031, i64 2
  %_102.0 = load ptr, ptr %f, align 8, !nonnull !6, !align !12, !noundef !6
  %_102.1 = load ptr, ptr %0, align 8, !nonnull !6, !align !10, !noundef !6
  %24 = getelementptr inbounds nuw i8, ptr %_102.1, i64 24
  %25 = load ptr, ptr %24, align 8, !invariant.load !6, !nonnull !6
  %26 = tail call noundef zeroext i1 %25(ptr noundef nonnull align 1 %_102.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3b8c95b91c663a5a9387bddd44e7b465, i64 noundef 1) #8
  br i1 %26, label %bb17, label %bb13
}

; <anstyle_parse::AsciiParser as anstyle_parse::CharAccumulator>::add
; Function Attrs: cold noreturn uwtable
define noundef range(i32 0, 1114113) i32 @_RNvXs_CsofRcISb9et_13anstyle_parseNtB4_11AsciiParserNtB4_15CharAccumulator3add(ptr noalias noundef nonnull readnone align 1 captures(none) %self, i8 noundef %_byte) unnamed_addr #2 {
start:
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_a956815dd852e1d78e37a53a853df475, ptr noundef nonnull inttoptr (i64 169 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4f3f6d8ef579fdebf1ff0939b9a2fbf8) #7
  unreachable
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #3

; core::panicking::panic_bounds_check
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking18panic_bounds_check(i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #4

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #5

; <u16 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3imptNtB9_7Display3fmt(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(2), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <u16 as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXso_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(2), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <u16 as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsm_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(2), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #5

declare i32 @rust_eh_personality(...) unnamed_addr #6

attributes #0 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { cold noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #4 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { "target-cpu"="apple-m1" }
attributes #7 = { noinline noreturn }
attributes #8 = { inlinehint }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{i8 0, i8 8}
!3 = !{!4}
!4 = distinct !{!4, !5, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_: %self"}
!5 = distinct !{!5, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBN_"}
!6 = !{}
!7 = !{!8, !4}
!8 = distinct !{!8, !9, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser14perform_actionNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBV_: %self"}
!9 = distinct !{!9, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser14perform_actionNtCsofRcISb9et_13anstyle_parse14VtUtf8ReceiverEBV_"}
!10 = !{i64 8}
!11 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!12 = !{i64 1}
!13 = !{!14, !16, !18}
!14 = distinct !{!14, !15, !"_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!15 = distinct !{!15, !"_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!16 = distinct !{!16, !17, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtCsofRcISb9et_13anstyle_parse6params10ParamsIterENtNtNtB8_6traits8iterator8Iterator4nextB1b_: %_0"}
!17 = distinct !{!17, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtCsofRcISb9et_13anstyle_parse6params10ParamsIterENtNtNtB8_6traits8iterator8Iterator4nextB1b_"}
!18 = distinct !{!18, !17, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtNtCsofRcISb9et_13anstyle_parse6params10ParamsIterENtNtNtB8_6traits8iterator8Iterator4nextB1b_: %self"}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt: %f"}
!21 = distinct !{!21, !"_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt"}
!22 = !{!23}
!23 = distinct !{!23, !21, !"_RNvXsV_NtNtCsjMrxcFdYDNN_4core3fmt3numtNtB7_5Debug3fmt: %self"}
!24 = distinct !{!24, !25}
!25 = !{!"llvm.loop.peeled.count", i32 1}
