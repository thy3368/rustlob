; ModuleID = 'same_file.87397fa462e337be-cgu.0'
source_filename = "same_file.87397fa462e337be-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_00adcc2b8aa61b89390045b9d076a3b1 = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/same-file-1.0.6/src/unix.rs\00", align 1
@alloc_0f7161f579711c7aaea9b6e147ec0dce = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_00adcc2b8aa61b89390045b9d076a3b1, [16 x i8] c"c\00\00\00\00\00\00\00f\00\00\00#\00\00\00" }>, align 8
@alloc_9509eddc4d6e81dd8213f049dc964f69 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_00adcc2b8aa61b89390045b9d076a3b1, [16 x i8] c"c\00\00\00\00\00\00\00`\00\00\00#\00\00\00" }>, align 8
@alloc_be3e2f23fe6e4a8d751011e036ab50a2 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_00adcc2b8aa61b89390045b9d076a3b1, [16 x i8] c"c\00\00\00\00\00\00\00\17\00\00\00\1E\00\00\00" }>, align 8
@alloc_23ed025b64d92bca40f80c5fa21b6757 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_00adcc2b8aa61b89390045b9d076a3b1, [16 x i8] c"c\00\00\00\00\00\00\00(\00\00\00%\00\00\00" }>, align 8
@alloc_0c3d257f7286f001df016d69d79f10ac = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_00adcc2b8aa61b89390045b9d076a3b1, [16 x i8] c"c\00\00\00\00\00\00\000\00\00\00\1C\00\00\00" }>, align 8

; core::ptr::drop_in_place::<same_file::Handle>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 20
  %1 = load i8, ptr %0, align 4, !range !5, !alias.scope !6, !noundef !9
  %_2.i.i = trunc nuw i8 %1 to i1
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_3.i.i = load i32, ptr %2, align 8, !alias.scope !2
  br i1 %_2.i.i, label %bb1.i.i, label %bb4.i

bb1.i.i:                                          ; preds = %start
  store i32 -1, ptr %2, align 8, !alias.scope !6
  %.not.i.i = icmp eq i32 %_3.i.i, -1
  br i1 %.not.i.i, label %bb4.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_.exit, !prof !10

bb4.i.i:                                          ; preds = %bb1.i.i
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_be3e2f23fe6e4a8d751011e036ab50a2) #6, !noalias !2
  unreachable

bb4.i:                                            ; preds = %start
  %3 = icmp eq i32 %_3.i.i, -1
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_.exit, label %bb2.i3.i

bb2.i3.i:                                         ; preds = %bb4.i
  %_5.i.i.i.i.i.i4.i = tail call noundef i32 @close(i32 noundef %_3.i.i) #7, !noalias !2
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_.exit: ; preds = %bb1.i.i, %bb4.i, %bb2.i3.i
  ret void
}

; <same_file::Handle>::as_file_mut
; Function Attrs: uwtable
define noundef nonnull align 4 ptr @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle11as_file_mut(ptr noalias noundef readonly align 8 captures(ret: address, provenance) dereferenceable(24) %self) unnamed_addr #0 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !11)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load i32, ptr %0, align 8, !alias.scope !11, !noundef !9
  %.not.i = icmp eq i32 %1, -1
  br i1 %.not.i, label %bb2.i, label %_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle11as_file_mut.exit, !prof !10

bb2.i:                                            ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0f7161f579711c7aaea9b6e147ec0dce) #6, !noalias !11
  unreachable

_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle11as_file_mut.exit: ; preds = %start
  ret ptr %0
}

; <same_file::Handle>::stdin
; Function Attrs: uwtable
define void @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle5stdin(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i.i.i = alloca [152 x i8], align 8
  %file.i.i.i = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !14
  store i32 0, ptr %file.i.i.i, align 4, !noalias !19
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !19
; invoke <std::fs::File>::metadata
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_3.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %file.i.i.i)
          to label %bb1.i.i.i unwind label %bb4.i.i.i, !noalias !19

bb1.i.i.i:                                        ; preds = %start
  %_10.i.i.i = load i64, ptr %_3.i.i.i, align 8, !range !22, !noalias !19, !noundef !9
  %0 = trunc nuw i64 %_10.i.i.i to i1
  %1 = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 8
  %_12.i.i.i = load ptr, ptr %1, align 8, !noalias !19
  %2 = ptrtoint ptr %_12.i.i.i to i64
  br i1 %0, label %bb4, label %bb5

bb4.i.i.i:                                        ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 0) #7, !noalias !19
  resume { ptr, i32 } %3

bb4:                                              ; preds = %bb1.i.i.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !19
  %_5.i.i.i.i.i24.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 0) #7, !noalias !19
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !14
  br label %bb2

bb5:                                              ; preds = %bb1.i.i.i
  %_11.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 16
  %_11.sroa.5.0.copyload.i.i.i = load i64, ptr %_11.sroa.5.0..sroa_idx.i.i.i, align 8, !noalias !19
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !19
  %sext.i.i.i = shl i64 %2, 32
  %_9.i.i.i = ashr exact i64 %sext.i.i.i, 32
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !14
  %_4.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_11.sroa.5.0.copyload.i.i.i, ptr %_4.sroa.4.0._0.sroa_idx, align 8
  %_4.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 0, ptr %_4.sroa.5.0._0.sroa_idx, align 8
  br label %bb2

bb2:                                              ; preds = %bb4, %bb5
  %_9.i.i.i.sink = phi i64 [ %2, %bb4 ], [ %_9.i.i.i, %bb5 ]
  %.sink = phi i8 [ 2, %bb4 ], [ 1, %bb5 ]
  store i64 %_9.i.i.i.sink, ptr %_0, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i8 %.sink, ptr %4, align 4
  ret void
}

; <same_file::Handle>::stderr
; Function Attrs: uwtable
define void @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle6stderr(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i.i.i = alloca [152 x i8], align 8
  %file.i.i.i = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !23
  store i32 2, ptr %file.i.i.i, align 4, !noalias !28
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !28
; invoke <std::fs::File>::metadata
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_3.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %file.i.i.i)
          to label %bb1.i.i.i unwind label %bb4.i.i.i, !noalias !28

bb1.i.i.i:                                        ; preds = %start
  %_10.i.i.i = load i64, ptr %_3.i.i.i, align 8, !range !22, !noalias !28, !noundef !9
  %0 = trunc nuw i64 %_10.i.i.i to i1
  %1 = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 8
  %_12.i.i.i = load ptr, ptr %1, align 8, !noalias !28
  %2 = ptrtoint ptr %_12.i.i.i to i64
  br i1 %0, label %bb4, label %bb5

bb4.i.i.i:                                        ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 2) #7, !noalias !28
  resume { ptr, i32 } %3

bb4:                                              ; preds = %bb1.i.i.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !28
  %_5.i.i.i.i.i24.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 2) #7, !noalias !28
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !23
  br label %bb2

bb5:                                              ; preds = %bb1.i.i.i
  %_11.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 16
  %_11.sroa.5.0.copyload.i.i.i = load i64, ptr %_11.sroa.5.0..sroa_idx.i.i.i, align 8, !noalias !28
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !28
  %sext.i.i.i = shl i64 %2, 32
  %_9.i.i.i = ashr exact i64 %sext.i.i.i, 32
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !23
  %_4.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_11.sroa.5.0.copyload.i.i.i, ptr %_4.sroa.4.0._0.sroa_idx, align 8
  %_4.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 2, ptr %_4.sroa.5.0._0.sroa_idx, align 8
  br label %bb2

bb2:                                              ; preds = %bb4, %bb5
  %_9.i.i.i.sink = phi i64 [ %2, %bb4 ], [ %_9.i.i.i, %bb5 ]
  %.sink = phi i8 [ 2, %bb4 ], [ 1, %bb5 ]
  store i64 %_9.i.i.i.sink, ptr %_0, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i8 %.sink, ptr %4, align 4
  ret void
}

; <same_file::Handle>::stdout
; Function Attrs: uwtable
define void @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle6stdout(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i.i.i = alloca [152 x i8], align 8
  %file.i.i.i = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !31
  store i32 1, ptr %file.i.i.i, align 4, !noalias !36
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !36
; invoke <std::fs::File>::metadata
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_3.i.i.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %file.i.i.i)
          to label %bb1.i.i.i unwind label %bb4.i.i.i, !noalias !36

bb1.i.i.i:                                        ; preds = %start
  %_10.i.i.i = load i64, ptr %_3.i.i.i, align 8, !range !22, !noalias !36, !noundef !9
  %0 = trunc nuw i64 %_10.i.i.i to i1
  %1 = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 8
  %_12.i.i.i = load ptr, ptr %1, align 8, !noalias !36
  %2 = ptrtoint ptr %_12.i.i.i to i64
  br i1 %0, label %bb4, label %bb5

bb4.i.i.i:                                        ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 1) #7, !noalias !36
  resume { ptr, i32 } %3

bb4:                                              ; preds = %bb1.i.i.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !36
  %_5.i.i.i.i.i24.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) 1) #7, !noalias !36
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !31
  br label %bb2

bb5:                                              ; preds = %bb1.i.i.i
  %_11.sroa.5.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i, i64 16
  %_11.sroa.5.0.copyload.i.i.i = load i64, ptr %_11.sroa.5.0..sroa_idx.i.i.i, align 8, !noalias !36
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i.i.i), !noalias !36
  %sext.i.i.i = shl i64 %2, 32
  %_9.i.i.i = ashr exact i64 %sext.i.i.i, 32
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i.i.i), !noalias !31
  %_4.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_11.sroa.5.0.copyload.i.i.i, ptr %_4.sroa.4.0._0.sroa_idx, align 8
  %_4.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 1, ptr %_4.sroa.5.0._0.sroa_idx, align 8
  br label %bb2

bb2:                                              ; preds = %bb4, %bb5
  %_9.i.i.i.sink = phi i64 [ %2, %bb4 ], [ %_9.i.i.i, %bb5 ]
  %.sink = phi i8 [ 2, %bb4 ], [ 1, %bb5 ]
  store i64 %_9.i.i.i.sink, ptr %_0, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i8 %.sink, ptr %4, align 4
  ret void
}

; <same_file::Handle>::as_file
; Function Attrs: uwtable
define noundef nonnull align 4 ptr @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle7as_file(ptr noalias noundef readonly align 8 captures(ret: address, read_provenance) dereferenceable(24) %self) unnamed_addr #0 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !39)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load i32, ptr %0, align 8, !alias.scope !39, !noundef !9
  %.not.i = icmp eq i32 %1, -1
  br i1 %.not.i, label %bb2.i, label %_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle7as_file.exit, !prof !10

bb2.i:                                            ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9509eddc4d6e81dd8213f049dc964f69) #6, !noalias !39
  unreachable

_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle7as_file.exit: ; preds = %start
  ret ptr %0
}

; <same_file::Handle>::from_file
; Function Attrs: uwtable
define void @_RNvMCsbBNrbdBR1qA_9same_fileNtB2_6Handle9from_file(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i32 noundef range(i32 0, -1) %file) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [152 x i8], align 8
  %file.i = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %file.i)
  store i32 %file, ptr %file.i, align 4, !noalias !42
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %_3.i), !noalias !42
; invoke <std::fs::File>::metadata
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_3.i, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %file.i)
          to label %bb1.i unwind label %bb4.i, !noalias !42

bb1.i:                                            ; preds = %start
  %_10.i = load i64, ptr %_3.i, align 8, !range !22, !noalias !42, !noundef !9
  %0 = trunc nuw i64 %_10.i to i1
  %1 = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %_12.i = load ptr, ptr %1, align 8, !noalias !42
  %2 = ptrtoint ptr %_12.i to i64
  br i1 %0, label %bb4, label %bb5

bb4.i:                                            ; preds = %start
  %3 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i.i = call noundef i32 @close(i32 noundef range(i32 0, -1) %file) #7, !noalias !42
  resume { ptr, i32 } %3

bb4:                                              ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i), !noalias !42
  %_5.i.i.i.i.i24.i = call noundef i32 @close(i32 noundef range(i32 0, -1) %file) #7, !noalias !42
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i)
  br label %bb2

bb5:                                              ; preds = %bb1.i
  %_11.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  %_11.sroa.5.0.copyload.i = load i64, ptr %_11.sroa.5.0..sroa_idx.i, align 8, !noalias !42
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3.i), !noalias !42
  %sext.i = shl i64 %2, 32
  %_9.i = ashr exact i64 %sext.i, 32
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %file.i)
  %_5.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_11.sroa.5.0.copyload.i, ptr %_5.sroa.4.0._0.sroa_idx, align 8
  %_5.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 %file, ptr %_5.sroa.5.0._0.sroa_idx, align 8
  br label %bb2

bb2:                                              ; preds = %bb4, %bb5
  %_9.i.sink = phi i64 [ %2, %bb4 ], [ %_9.i, %bb5 ]
  %.sink = phi i8 [ 2, %bb4 ], [ 0, %bb5 ]
  store i64 %_9.i.sink, ptr %_0, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i8 %.sink, ptr %4, align 4
  ret void
}

; <same_file::unix::Handle>::from_file
; Function Attrs: uwtable
define void @_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, i32 noundef range(i32 0, -1) %0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3 = alloca [152 x i8], align 8
  %file = alloca [4 x i8], align 4
  store i32 %0, ptr %file, align 4
  call void @llvm.lifetime.start.p0(i64 152, ptr nonnull %_3)
; invoke <std::fs::File>::metadata
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr noalias noundef nonnull sret([152 x i8]) align 8 captures(none) dereferenceable(152) %_3, ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %file)
          to label %bb1 unwind label %bb4

bb1:                                              ; preds = %start
  %_10 = load i64, ptr %_3, align 8, !range !22, !noundef !9
  %1 = trunc nuw i64 %_10 to i1
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_12 = load ptr, ptr %2, align 8
  br i1 %1, label %bb6, label %bb7

bb6:                                              ; preds = %bb1
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3)
  %3 = ptrtoint ptr %_12 to i64
  %_5.i.i.i.i.i24 = call noundef i32 @close(i32 noundef %0) #7
  br label %bb3

bb7:                                              ; preds = %bb1
  %_11.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_11.sroa.5.0.copyload = load i64, ptr %_11.sroa.5.0..sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 152, ptr nonnull %_3)
  %4 = ptrtoint ptr %_12 to i64
  %sext = shl i64 %4, 32
  %_9 = ashr exact i64 %sext, 32
  %_6.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_11.sroa.5.0.copyload, ptr %_6.sroa.4.0._0.sroa_idx, align 8
  %_6.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 %0, ptr %_6.sroa.5.0._0.sroa_idx, align 8
  br label %bb3

bb3:                                              ; preds = %bb6, %bb7
  %_9.sink = phi i64 [ %3, %bb6 ], [ %_9, %bb7 ]
  %.sink = phi i8 [ 2, %bb6 ], [ 0, %bb7 ]
  store i64 %_9.sink, ptr %_0, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 20
  store i8 %.sink, ptr %5, align 4
  ret void

bb4:                                              ; preds = %start
  %6 = landingpad { ptr, i32 }
          cleanup
  %_5.i.i.i.i.i = call noundef i32 @close(i32 noundef %0) #7
  resume { ptr, i32 } %6
}

; <same_file::unix::Handle as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 20
  %1 = load i8, ptr %0, align 4, !range !5, !noundef !9
  %_2 = trunc nuw i8 %1 to i1
  br i1 %_2, label %bb1, label %bb2

bb2:                                              ; preds = %bb1, %start
  ret void

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_3 = load i32, ptr %2, align 8, !noundef !9
  store i32 -1, ptr %2, align 8
  %.not = icmp eq i32 %_3, -1
  br i1 %.not, label %bb4, label %bb2, !prof !10

bb4:                                              ; preds = %bb1
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_be3e2f23fe6e4a8d751011e036ab50a2) #6
  unreachable
}

; <same_file::Handle as std::os::fd::raw::AsRawFd>::as_raw_fd
; Function Attrs: uwtable
define noundef range(i32 0, -1) i32 @_RNvXs1_NtCsbBNrbdBR1qA_9same_file4unixNtB7_6HandleNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw7AsRawFd9as_raw_fd(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load i32, ptr %0, align 8, !noundef !9
  %.not = icmp eq i32 %1, -1
  br i1 %.not, label %bb2, label %bb3, !prof !10

bb3:                                              ; preds = %start
  ret i32 %1

bb2:                                              ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23ed025b64d92bca40f80c5fa21b6757) #6
  unreachable
}

; <same_file::Handle as std::os::fd::raw::IntoRawFd>::into_raw_fd
; Function Attrs: uwtable
define noundef range(i32 0, -1) i32 @_RNvXs2_NtCsbBNrbdBR1qA_9same_file4unixNtB7_6HandleNtNtNtNtCs5sEH5CPMdak_3std2os2fd3raw9IntoRawFd11into_raw_fd(ptr dead_on_return noalias noundef align 8 captures(none) dereferenceable(24) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load i32, ptr %0, align 8, !noundef !9
  store i32 -1, ptr %0, align 8
  %.not = icmp eq i32 %1, -1
  br i1 %.not, label %bb5, label %bb6, !prof !10

bb6:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !45)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !48)
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 20
  %3 = load i8, ptr %2, align 4, !range !5, !alias.scope !51, !noundef !9
  %_2.i.i.i = trunc nuw i8 %3 to i1
  br i1 %_2.i.i.i, label %bb4.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_.exit

bb4.i.i.i:                                        ; preds = %bb6
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_be3e2f23fe6e4a8d751011e036ab50a2) #6, !noalias !54
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_.exit: ; preds = %bb6
  ret i32 %1

bb5:                                              ; preds = %start
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_0c3d257f7286f001df016d69d79f10ac) #8
          to label %unreachable unwind label %cleanup

cleanup:                                          ; preds = %bb5
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<same_file::Handle>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_(ptr noalias noundef align 8 dereferenceable(24) %self) #9
          to label %bb3 unwind label %terminate

unreachable:                                      ; preds = %bb5
  unreachable

terminate:                                        ; preds = %cleanup
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #10
  unreachable

bb3:                                              ; preds = %cleanup
  resume { ptr, i32 } %4
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #2

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #3

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #2

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #4

; <std::fs::File>::metadata
; Function Attrs: uwtable
declare void @_RNvMs2_NtCs5sEH5CPMdak_3std2fsNtB5_4File8metadata(ptr dead_on_unwind noalias noundef writable sret([152 x i8]) align 8 captures(none) dereferenceable(152), ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4)) unnamed_addr #0

; Function Attrs: nounwind uwtable
declare noundef i32 @close(i32 noundef) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #5

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #6 = { noinline noreturn }
attributes #7 = { nounwind }
attributes #8 = { noreturn }
attributes #9 = { cold }
attributes #10 = { cold noreturn nounwind }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_: %_1"}
!4 = distinct !{!4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_"}
!5 = !{i8 0, i8 2}
!6 = !{!7, !3}
!7 = distinct !{!7, !8, !"_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!8 = distinct !{!8, !"_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!9 = !{}
!10 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle11as_file_mut: %self"}
!13 = distinct !{!13, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle11as_file_mut"}
!14 = !{!15, !17}
!15 = distinct !{!15, !16, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std: %_0"}
!16 = distinct !{!16, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std"}
!17 = distinct !{!17, !18, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle5stdin: %_0"}
!18 = distinct !{!18, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle5stdin"}
!19 = !{!20, !15, !17}
!20 = distinct !{!20, !21, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file: %_0"}
!21 = distinct !{!21, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file"}
!22 = !{i64 0, i64 2}
!23 = !{!24, !26}
!24 = distinct !{!24, !25, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std: %_0"}
!25 = distinct !{!25, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std"}
!26 = distinct !{!26, !27, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle6stderr: %_0"}
!27 = distinct !{!27, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle6stderr"}
!28 = !{!29, !24, !26}
!29 = distinct !{!29, !30, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file: %_0"}
!30 = distinct !{!30, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file"}
!31 = !{!32, !34}
!32 = distinct !{!32, !33, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std: %_0"}
!33 = distinct !{!33, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle8from_std"}
!34 = distinct !{!34, !35, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle6stdout: %_0"}
!35 = distinct !{!35, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle6stdout"}
!36 = !{!37, !32, !34}
!37 = distinct !{!37, !38, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file: %_0"}
!38 = distinct !{!38, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file"}
!39 = !{!40}
!40 = distinct !{!40, !41, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle7as_file: %self"}
!41 = distinct !{!41, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle7as_file"}
!42 = !{!43}
!43 = distinct !{!43, !44, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file: %_0"}
!44 = distinct !{!44, !"_RNvMs4_NtCsbBNrbdBR1qA_9same_file4unixNtB5_6Handle9from_file"}
!45 = !{!46}
!46 = distinct !{!46, !47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_: %_1"}
!47 = distinct !{!47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbBNrbdBR1qA_9same_file6HandleEBI_"}
!48 = !{!49}
!49 = distinct !{!49, !50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_: %_1"}
!50 = distinct !{!50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbBNrbdBR1qA_9same_file4unix6HandleEBK_"}
!51 = !{!52, !49, !46}
!52 = distinct !{!52, !53, !"_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!53 = distinct !{!53, !"_RNvXNtCsbBNrbdBR1qA_9same_file4unixNtB2_6HandleNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!54 = !{!49, !46}
