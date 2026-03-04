; ModuleID = 'once_cell.9e810019c0a8cafc-cgu.0'
source_filename = "once_cell.9e810019c0a8cafc-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_edb3ed16ed07237eac14eb16826c52e0 = private unnamed_addr constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 8
@alloc_e5b8217fb5c008ced29863ec33a8a0e5 = private unnamed_addr constant [104 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/once_cell-1.21.3/src/imp_std.rs\00", align 1
@alloc_1031c80e13800c8982f52f15d71b7e7a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e5b8217fb5c008ced29863ec33a8a0e5, [16 x i8] c"g\00\00\00\00\00\00\00\A1\00\00\006\00\00\00" }>, align 8
@alloc_674d3c86a5d5842a72b6b220eb1e60f8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_e5b8217fb5c008ced29863ec33a8a0e5, [16 x i8] c"g\00\00\00\00\00\00\00\9B\00\00\00\09\00\00\00" }>, align 8

; once_cell::imp::initialize_or_wait
; Function Attrs: noinline uwtable
define void @_RNvNtCsdBIbacKQwMq_9once_cell3imp18initialize_or_wait(ptr noundef nonnull align 8 %queue, ptr noundef align 1 %0, ptr readonly captures(address_is_null) %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %node.i = alloca [24 x i8], align 8
  %guard = alloca [16 x i8], align 8
  %2 = load atomic ptr, ptr %queue acquire, align 8
  %.not = icmp eq ptr %0, null
  %3 = getelementptr inbounds nuw i8, ptr %node.i, i64 16
  %4 = getelementptr inbounds nuw i8, ptr %node.i, i64 8
  br label %bb1.outer

bb11:                                             ; preds = %bb1.outer, %bb8
  ret void

bb2:                                              ; preds = %bb1.outer
  br i1 %.not, label %bb3, label %bb4

bb3:                                              ; preds = %bb1.outer, %bb2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %node.i)
; call std::thread::current::current
  %_915.i = call noundef nonnull ptr @_RNvNtNtCs5sEH5CPMdak_3std6thread7current7current()
  %_2717.i = sub nsw i64 0, %curr_state
  %5 = getelementptr i8, ptr %curr_queue.sroa.0.0.ph, i64 %_2717.i
  store ptr %_915.i, ptr %node.i, align 8
  store i8 0, ptr %3, align 8
  store ptr %5, ptr %4, align 8
  %6 = getelementptr i8, ptr %node.i, i64 %curr_state
  %7 = cmpxchg ptr %queue, ptr %curr_queue.sroa.0.0.ph, ptr %6 release monotonic, align 8
  %_8.sroa.18.0.in.i18.i = extractvalue { ptr, i1 } %7, 1
  br i1 %_8.sroa.18.0.in.i18.i, label %bb19.i.preheader, label %bb3.i

bb19.i.preheader:                                 ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i, %bb3
  br label %bb19.i

cleanup.i:                                        ; preds = %bb8.i
  %8 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !2)
  call void @llvm.experimental.noalias.scope.decl(metadata !5)
  call void @llvm.experimental.noalias.scope.decl(metadata !8)
  call void @llvm.experimental.noalias.scope.decl(metadata !11)
  %9 = load ptr, ptr %node.i, align 8, !alias.scope !14, !noundef !15
  %10 = icmp eq ptr %9, null
  br i1 %10, label %common.resume, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %cleanup.i
  %11 = atomicrmw sub ptr %9, i64 1 release, align 8, !noalias !16
  %12 = icmp eq i64 %11, 1
  br i1 %12, label %bb2.i.i.i.i.i.i.i.i.i, label %common.resume

bb2.i.i.i.i.i.i.i.i.i:                            ; preds = %bb2.i.i.i.i.i
  fence acquire
; invoke <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef nonnull align 8 dereferenceable(24) %node.i) #9
          to label %common.resume unwind label %terminate.i

bb3.i:                                            ; preds = %bb3, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i
  %.pn.i = phi { ptr, i1 } [ %18, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i ], [ %7, %bb3 ]
  %_8.sroa.0.0.i20.i = extractvalue { ptr, i1 } %.pn.i, 0
  %_20.i = ptrtoint ptr %_8.sroa.0.0.i20.i to i64
  %_19.i = and i64 %_20.i, 3
  %_18.not.i = icmp eq i64 %_19.i, %curr_state
  br i1 %_18.not.i, label %bb5.i, label %bb4.i

bb5.i:                                            ; preds = %bb3.i
  call void @llvm.experimental.noalias.scope.decl(metadata !25)
  call void @llvm.experimental.noalias.scope.decl(metadata !28)
  call void @llvm.experimental.noalias.scope.decl(metadata !31)
  call void @llvm.experimental.noalias.scope.decl(metadata !34)
  %13 = load ptr, ptr %node.i, align 8, !alias.scope !37, !noundef !15
  %14 = icmp eq ptr %13, null
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i, label %bb2.i.i.i.i5.i

bb2.i.i.i.i5.i:                                   ; preds = %bb5.i
  %15 = atomicrmw sub ptr %13, i64 1 release, align 8, !noalias !38
  %16 = icmp eq i64 %15, 1
  br i1 %16, label %bb2.i.i.i.i.i.i.i.i6.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i

bb2.i.i.i.i.i.i.i.i6.i:                           ; preds = %bb2.i.i.i.i5.i
  fence acquire
; call <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef nonnull align 8 dereferenceable(24) %node.i) #9
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_.exit7.i: ; preds = %bb2.i.i.i.i.i.i.i.i6.i, %bb2.i.i.i.i5.i, %bb5.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %node.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %node.i)
; call std::thread::current::current
  %_9.i = call noundef nonnull ptr @_RNvNtNtCs5sEH5CPMdak_3std6thread7current7current()
  %17 = getelementptr i8, ptr %_8.sroa.0.0.i20.i, i64 %_2717.i
  store ptr %_9.i, ptr %node.i, align 8
  store i8 0, ptr %3, align 8
  store ptr %17, ptr %4, align 8
  %18 = cmpxchg ptr %queue, ptr %_8.sroa.0.0.i20.i, ptr %6 release monotonic, align 8
  %_8.sroa.18.0.in.i.i = extractvalue { ptr, i1 } %18, 1
  br i1 %_8.sroa.18.0.in.i.i, label %bb19.i.preheader, label %bb3.i

bb4.i:                                            ; preds = %bb3.i
  call void @llvm.experimental.noalias.scope.decl(metadata !47)
  call void @llvm.experimental.noalias.scope.decl(metadata !50)
  call void @llvm.experimental.noalias.scope.decl(metadata !53)
  call void @llvm.experimental.noalias.scope.decl(metadata !56)
  %19 = load ptr, ptr %node.i, align 8, !alias.scope !59, !noundef !15
  %20 = icmp eq ptr %19, null
  br i1 %20, label %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit, label %bb2.i.i.i.i8.i

bb2.i.i.i.i8.i:                                   ; preds = %bb4.i
  %21 = atomicrmw sub ptr %19, i64 1 release, align 8, !noalias !60
  %22 = icmp eq i64 %21, 1
  br i1 %22, label %bb13.sink.split.i, label %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit

bb13.sink.split.i:                                ; preds = %bb2.i.i.i.i11.i, %bb2.i.i.i.i8.i
  fence acquire
; call <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef nonnull align 8 dereferenceable(24) %node.i) #9
  br label %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit

bb19.i:                                           ; preds = %bb19.i.preheader, %bb8.i
  %23 = load atomic i8, ptr %3 acquire, align 8
  %24 = icmp eq i8 %23, 0
  br i1 %24, label %bb8.i, label %bb7.i

bb8.i:                                            ; preds = %bb19.i
; invoke std::thread::functions::park
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std6thread9functions4park()
          to label %bb19.i unwind label %cleanup.i

bb7.i:                                            ; preds = %bb19.i
  call void @llvm.experimental.noalias.scope.decl(metadata !69)
  call void @llvm.experimental.noalias.scope.decl(metadata !72)
  call void @llvm.experimental.noalias.scope.decl(metadata !75)
  call void @llvm.experimental.noalias.scope.decl(metadata !78)
  %25 = load ptr, ptr %node.i, align 8, !alias.scope !81, !noundef !15
  %26 = icmp eq ptr %25, null
  br i1 %26, label %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit, label %bb2.i.i.i.i11.i

bb2.i.i.i.i11.i:                                  ; preds = %bb7.i
  %27 = atomicrmw sub ptr %25, i64 1 release, align 8, !noalias !82
  %28 = icmp eq i64 %27, 1
  br i1 %28, label %bb13.sink.split.i, label %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit

terminate.i:                                      ; preds = %bb2.i.i.i.i.i.i.i.i.i
  %29 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #10
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i, %bb2.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %8, %bb2.i.i.i.i.i.i.i.i.i ], [ %8, %bb2.i.i.i.i.i ], [ %8, %cleanup.i ], [ %37, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit:     ; preds = %bb4.i, %bb2.i.i.i.i8.i, %bb13.sink.split.i, %bb7.i, %bb2.i.i.i.i11.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %node.i)
  %30 = load atomic ptr, ptr %queue acquire, align 8
  br label %bb1.outer.backedge

bb1.outer.backedge:                               ; preds = %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit, %bb4
  %curr_queue.sroa.0.0.ph.be = phi ptr [ %_8.sroa.0.0.i, %bb4 ], [ %30, %_RNvNtCsdBIbacKQwMq_9once_cell3imp4wait.exit ]
  br label %bb1.outer

bb1.outer:                                        ; preds = %bb1.outer.backedge, %start
  %curr_queue.sroa.0.0.ph = phi ptr [ %2, %start ], [ %curr_queue.sroa.0.0.ph.be, %bb1.outer.backedge ]
  %_4 = ptrtoint ptr %curr_queue.sroa.0.0.ph to i64
  %curr_state = and i64 %_4, 3
  switch i64 %curr_state, label %default.unreachable [
    i64 2, label %bb11
    i64 0, label %bb2
    i64 1, label %bb3
    i64 3, label %bb1.us
  ]

default.unreachable:                              ; preds = %bb1.outer
  unreachable

bb1.us:                                           ; preds = %bb1.outer, %bb1.us
  br label %bb1.us

bb4:                                              ; preds = %bb2
  %31 = getelementptr i8, ptr %curr_queue.sroa.0.0.ph, i64 1
  %32 = cmpxchg ptr %queue, ptr %curr_queue.sroa.0.0.ph, ptr %31 acquire acquire, align 8
  %_8.sroa.18.0.in.i = extractvalue { ptr, i1 } %32, 1
  %_8.sroa.0.0.i = extractvalue { ptr, i1 } %32, 0
  br i1 %_8.sroa.18.0.in.i, label %bb6, label %bb1.outer.backedge

bb6:                                              ; preds = %bb4
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %guard)
  store ptr %queue, ptr %guard, align 8
  %33 = getelementptr inbounds nuw i8, ptr %guard, i64 8
  store ptr null, ptr %33, align 8
  %34 = icmp ne ptr %1, null
  call void @llvm.assume(i1 %34)
  %35 = getelementptr inbounds nuw i8, ptr %1, i64 32
  %36 = load ptr, ptr %35, align 8, !invariant.load !15, !nonnull !15
  %_14 = invoke noundef zeroext i1 %36(ptr noundef nonnull align 1 %0)
          to label %bb17 unwind label %cleanup

cleanup:                                          ; preds = %bb6
  %37 = landingpad { ptr, i32 }
          cleanup
; invoke <once_cell::imp::Guard as core::ops::drop::Drop>::drop
  invoke void @_RNvXs3_NtCsdBIbacKQwMq_9once_cell3impNtB5_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %guard)
          to label %common.resume unwind label %terminate

bb17:                                             ; preds = %bb6
  br i1 %_14, label %bb7, label %bb8

bb8:                                              ; preds = %bb7, %bb17
; call <once_cell::imp::Guard as core::ops::drop::Drop>::drop
  call void @_RNvXs3_NtCsdBIbacKQwMq_9once_cell3impNtB5_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readonly align 8 dereferenceable(16) %guard)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %guard)
  br label %bb11

bb7:                                              ; preds = %bb17
  store ptr inttoptr (i64 2 to ptr), ptr %33, align 8
  br label %bb8

terminate:                                        ; preds = %cleanup
  %38 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #10
  unreachable
}

; <once_cell::imp::Guard as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXs3_NtCsdBIbacKQwMq_9once_cell3impNtB5_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %thread = alloca [8 x i8], align 8
  %state = alloca [8 x i8], align 8
  %_16 = load ptr, ptr %self, align 8, !nonnull !15, !align !91, !noundef !15
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3 = load ptr, ptr %0, align 8, !noundef !15
  %1 = atomicrmw xchg ptr %_16, ptr %_3 acq_rel, align 8
  %_5 = ptrtoint ptr %1 to i64
  %2 = and i64 %_5, 3
  store i64 %2, ptr %state, align 8
  %3 = icmp eq i64 %2, 1
  br i1 %3, label %bb1, label %bb2, !prof !92

bb1:                                              ; preds = %start
  %4 = getelementptr i8, ptr %1, i64 -1
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb4, label %bb5

bb2:                                              ; preds = %start
; call core::panicking::assert_failed::<usize, usize>
  call void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %state, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @alloc_edb3ed16ed07237eac14eb16826c52e0, ptr noundef null, ptr undef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_674d3c86a5d5842a72b6b220eb1e60f8) #11
  unreachable

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7, %bb1
  ret void

bb5:                                              ; preds = %bb1, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7
  %waiter.sroa.0.08 = phi ptr [ %next, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7 ], [ %4, %bb1 ]
  %6 = getelementptr inbounds nuw i8, ptr %waiter.sroa.0.08, i64 8
  %next = load ptr, ptr %6, align 8, !noundef !15
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %thread)
  %7 = load ptr, ptr %waiter.sroa.0.08, align 8, !noundef !15
  store ptr null, ptr %waiter.sroa.0.08, align 8
  %.not = icmp eq ptr %7, null
  br i1 %.not, label %bb12, label %bb14, !prof !93

bb12:                                             ; preds = %bb5
; call core::option::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1031c80e13800c8982f52f15d71b7e7a) #11
  unreachable

cleanup:                                          ; preds = %bb14
  %8 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !94)
  call void @llvm.experimental.noalias.scope.decl(metadata !97)
  call void @llvm.experimental.noalias.scope.decl(metadata !100)
  call void @llvm.experimental.noalias.scope.decl(metadata !103)
  %_10.i.i.i.i = load ptr, ptr %thread, align 8, !alias.scope !106, !nonnull !15, !noundef !15
  %9 = atomicrmw sub ptr %_10.i.i.i.i, i64 1 release, align 8, !noalias !106
  %10 = icmp eq i64 %9, 1
  br i1 %10, label %bb2.i.i.i.i, label %bb8

bb2.i.i.i.i:                                      ; preds = %cleanup
  fence acquire
; invoke <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef nonnull align 8 dereferenceable(8) %thread) #9
          to label %bb8 unwind label %terminate

bb14:                                             ; preds = %bb5
  store ptr %7, ptr %thread, align 8
  %_14 = getelementptr inbounds nuw i8, ptr %waiter.sroa.0.08, i64 16
  store atomic i8 1, ptr %_14 release, align 1
  %_38 = getelementptr inbounds nuw i8, ptr %7, i64 40
; invoke <std::sys::sync::thread_parking::darwin::Parker>::unpark
  invoke void @_RNvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync14thread_parking6darwinNtB5_6Parker6unpark(ptr noundef nonnull align 8 %_38)
          to label %bb15 unwind label %cleanup

bb15:                                             ; preds = %bb14
  call void @llvm.experimental.noalias.scope.decl(metadata !107)
  call void @llvm.experimental.noalias.scope.decl(metadata !110)
  call void @llvm.experimental.noalias.scope.decl(metadata !113)
  call void @llvm.experimental.noalias.scope.decl(metadata !116)
  %_10.i.i.i.i5 = load ptr, ptr %thread, align 8, !alias.scope !119, !nonnull !15, !noundef !15
  %11 = atomicrmw sub ptr %_10.i.i.i.i5, i64 1 release, align 8, !noalias !119
  %12 = icmp eq i64 %11, 1
  br i1 %12, label %bb2.i.i.i.i6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7

bb2.i.i.i.i6:                                     ; preds = %bb15
  fence acquire
; call <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef nonnull align 8 dereferenceable(8) %thread) #9
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell.exit7: ; preds = %bb15, %bb2.i.i.i.i6
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %thread)
  %13 = icmp eq ptr %next, null
  br i1 %13, label %bb4, label %bb5

terminate:                                        ; preds = %bb2.i.i.i.i
  %14 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #10
  unreachable

bb8:                                              ; preds = %cleanup, %bb2.i.i.i.i
  resume { ptr, i32 } %8
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #3

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #3

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #4

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #5

; std::thread::current::current
; Function Attrs: uwtable
declare noundef nonnull ptr @_RNvNtNtCs5sEH5CPMdak_3std6thread7current7current() unnamed_addr #1

; std::thread::functions::park
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std6thread9functions4park() unnamed_addr #1

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #6

; <std::sys::sync::thread_parking::darwin::Parker>::unpark
; Function Attrs: uwtable
declare void @_RNvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync14thread_parking6darwinNtB5_6Parker6unpark(ptr noundef nonnull align 8) unnamed_addr #1

; core::panicking::assert_failed::<usize, usize>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef range(i8 0, 3), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noundef, ptr, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #7

; <alloc::sync::Arc<std::thread::thread::Inner, std::alloc::System>>::drop_slow
; Function Attrs: noinline uwtable
declare void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemE9drop_slowBM_(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #8

attributes #0 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #3 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #9 = { noinline }
attributes #10 = { cold noreturn nounwind }
attributes #11 = { noinline noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_: %_1"}
!4 = distinct !{!4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_"}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!7 = distinct !{!7, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!10 = distinct !{!10, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell: %_1"}
!13 = distinct !{!13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell"}
!14 = !{!12, !9, !6, !3}
!15 = !{}
!16 = !{!17, !19, !21, !23, !12, !9, !6, !3}
!17 = distinct !{!17, !18, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!18 = distinct !{!18, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!19 = distinct !{!19, !20, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!20 = distinct !{!20, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!21 = distinct !{!21, !22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!22 = distinct !{!22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!23 = distinct !{!23, !24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!24 = distinct !{!24, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_: %_1"}
!27 = distinct !{!27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_"}
!28 = !{!29}
!29 = distinct !{!29, !30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!30 = distinct !{!30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!31 = !{!32}
!32 = distinct !{!32, !33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!33 = distinct !{!33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell: %_1"}
!36 = distinct !{!36, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell"}
!37 = !{!35, !32, !29, !26}
!38 = !{!39, !41, !43, !45, !35, !32, !29, !26}
!39 = distinct !{!39, !40, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!40 = distinct !{!40, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!41 = distinct !{!41, !42, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!42 = distinct !{!42, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!43 = distinct !{!43, !44, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!44 = distinct !{!44, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!45 = distinct !{!45, !46, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!46 = distinct !{!46, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!47 = !{!48}
!48 = distinct !{!48, !49, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_: %_1"}
!49 = distinct !{!49, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_"}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!52 = distinct !{!52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!53 = !{!54}
!54 = distinct !{!54, !55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!55 = distinct !{!55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!56 = !{!57}
!57 = distinct !{!57, !58, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell: %_1"}
!58 = distinct !{!58, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell"}
!59 = !{!57, !54, !51, !48}
!60 = !{!61, !63, !65, !67, !57, !54, !51, !48}
!61 = distinct !{!61, !62, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!62 = distinct !{!62, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!63 = distinct !{!63, !64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!64 = distinct !{!64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!65 = distinct !{!65, !66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!66 = distinct !{!66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!67 = distinct !{!67, !68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!68 = distinct !{!68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!69 = !{!70}
!70 = distinct !{!70, !71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_: %_1"}
!71 = distinct !{!71, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdBIbacKQwMq_9once_cell3imp6WaiterEBK_"}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!74 = distinct !{!74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell4CellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!75 = !{!76}
!76 = distinct !{!76, !77, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell: %_1"}
!77 = distinct !{!77, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEEECsdBIbacKQwMq_9once_cell"}
!78 = !{!79}
!79 = distinct !{!79, !80, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell: %_1"}
!80 = distinct !{!80, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadEECsdBIbacKQwMq_9once_cell"}
!81 = !{!79, !76, !73, !70}
!82 = !{!83, !85, !87, !89, !79, !76, !73, !70}
!83 = distinct !{!83, !84, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!84 = distinct !{!84, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!85 = distinct !{!85, !86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!86 = distinct !{!86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!87 = distinct !{!87, !88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!88 = distinct !{!88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!89 = distinct !{!89, !90, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!90 = distinct !{!90, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!91 = !{i64 8}
!92 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!93 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!94 = !{!95}
!95 = distinct !{!95, !96, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!96 = distinct !{!96, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!97 = !{!98}
!98 = distinct !{!98, !99, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!99 = distinct !{!99, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!100 = !{!101}
!101 = distinct !{!101, !102, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!102 = distinct !{!102, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!103 = !{!104}
!104 = distinct !{!104, !105, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!105 = distinct !{!105, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!106 = !{!104, !101, !98, !95}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell: %_1"}
!109 = distinct !{!109, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std6thread6thread6ThreadECsdBIbacKQwMq_9once_cell"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell: %_1"}
!112 = distinct !{!112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1A_5alloc6SystemEEECsdBIbacKQwMq_9once_cell"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell: %_1"}
!115 = distinct !{!115, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtB1k_5alloc6SystemEECsdBIbacKQwMq_9once_cell"}
!116 = !{!117}
!117 = distinct !{!117, !118, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell: %self"}
!118 = distinct !{!118, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs5sEH5CPMdak_3std6thread6thread5InnerNtNtBM_5alloc6SystemENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCsdBIbacKQwMq_9once_cell"}
!119 = !{!117, !114, !111, !108}
