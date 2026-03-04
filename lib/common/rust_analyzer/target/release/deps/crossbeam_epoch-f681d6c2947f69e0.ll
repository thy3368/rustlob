; ModuleID = 'crossbeam_epoch.d0f7058e82b829c1-cgu.0'
source_filename = "crossbeam_epoch.d0f7058e82b829c1-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"deferred::Deferred" = type { ptr, %"core::mem::maybe_uninit::MaybeUninit<[usize; 3]>", %"core::marker::PhantomData<*mut ()>" }
%"core::mem::maybe_uninit::MaybeUninit<[usize; 3]>" = type { [3 x i64] }
%"core::marker::PhantomData<*mut ()>" = type {}

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBd_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1c_8OnceLockNtNtB1g_9collector9CollectorE10initializeNvMs1_B2i_B2g_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBd_9OnceStateEE9call_once6vtableB1g_, ptr @_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB17_8OnceLockNtNtB1b_9collector9CollectorE10initializeNvMs1_B2d_B2b_3newE0E0B1b_ }>, align 8
@alloc_63b5c1d216d612d5711b62223b36349f = private unnamed_addr constant [117 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-epoch-0.9.18/src/sync/once_lock.rs\00", align 1
@alloc_051d8689b267677c3f8147a749eccac6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_63b5c1d216d612d5711b62223b36349f, [16 x i8] c"t\00\00\00\00\00\00\00B\00\00\00\13\00\00\00" }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB6_5Debug3fmtBA_ }>, align 8
@alloc_2d35da0edd4cc6d13fa936157458b298 = private unnamed_addr constant [17 x i8] c"unaligned pointer", align 1
@alloc_4203124163e1b846e2d64e13ddf5f47a = private unnamed_addr constant [109 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-epoch-0.9.18/src/atomic.rs\00", align 1
@alloc_31ec4324eab67a881201d09fca59e0c9 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4203124163e1b846e2d64e13ddf5f47a, [16 x i8] c"l\00\00\00\00\00\00\00q\00\00\00\05\00\00\00" }>, align 8
@alloc_8abc8245dd622154cb39537f390f45d3 = private unnamed_addr constant [117 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sync/once.rs\00", align 1
@alloc_3129176114b12752e73d3989241bb5f0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_8abc8245dd622154cb39537f390f45d3, [16 x i8] c"t\00\00\00\00\00\00\00\9F\00\00\002\00\00\00" }>, align 8
@anon.e80f155be67e243e8d65baf1d259b60c.0 = private unnamed_addr constant <{ ptr, [24 x i8] }> <{ ptr @_RNvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB6_8Deferred5NO_OP10no_op_call, [24 x i8] undef }>, align 8
@alloc_ca9bafd89d420505367d1f114ba497ce = private unnamed_addr constant [111 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-epoch-0.9.18/src/internal.rs\00", align 1
@alloc_6ce925522e0221e39b5bde413077875f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ca9bafd89d420505367d1f114ba497ce, [16 x i8] c"n\00\00\00\00\00\00\00\81\01\00\009\00\00\00" }>, align 8
@_RNvNCNKNvNtCshWjyrkxAz4b_15crossbeam_epoch7default6HANDLE0023___RUST_STD_INTERNAL_VAL = thread_local local_unnamed_addr global <{ [8 x i8], [1 x i8], [7 x i8] }> <{ [8 x i8] undef, [1 x i8] zeroinitializer, [7 x i8] undef }>, align 8
@_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch5guard11unprotected11UNPROTECTED = constant [8 x i8] zeroinitializer, align 8
@_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR = internal global <{ [8 x i8], [8 x i8] }> <{ [8 x i8] c"\03\00\00\00\00\00\00\00", [8 x i8] undef }>, align 8
@alloc_c97741f92dd5ccff591c27dba386f700 = private unnamed_addr constant [15 x i8] c"Deferred { .. }", align 1
@alloc_5cead5022b6c1daff98c0518c274325d = private unnamed_addr constant [12 x i8] c"Guard { .. }", align 1
@alloc_6ded0471392c4b1917b1d9a4afadd291 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ca9bafd89d420505367d1f114ba497ce, [16 x i8] c"n\00\00\00\00\00\00\00w\00\00\00,\00\00\00" }>, align 8
@alloc_edb3ed16ed07237eac14eb16826c52e0 = private unnamed_addr constant [8 x i8] c"\01\00\00\00\00\00\00\00", align 8
@alloc_56bbe7868899b1046e5b399216aa5fd6 = private unnamed_addr constant [112 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-epoch-0.9.18/src/sync/list.rs\00", align 1
@alloc_8ec3bc029f16021168b4faa1ad83b4c0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_56bbe7868899b1046e5b399216aa5fd6, [16 x i8] c"o\00\00\00\00\00\00\00\E2\00\00\00\11\00\00\00" }>, align 8
@alloc_e22a7101984065c8df67d185dd44ede6 = private unnamed_addr constant [3 x i8] c"Bag", align 1
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB6_5Debug3fmtBB_ }>, align 8
@alloc_31fb308a800d703546e3033c8c1834ad = private unnamed_addr constant [9 x i8] c"deferreds", align 1
@alloc_bd8f6f3aa8c59c7dea37b6db299761bd = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_ca9bafd89d420505367d1f114ba497ce, [16 x i8] c"n\00\00\00\00\00\00\00\83\00\00\001\00\00\00" }>, align 8
@alloc_b7376c537a345be44440e786f26d0dce = private unnamed_addr constant [16 x i8] c"Collector { .. }", align 1
@alloc_39ad2ebca0184067d2550a7ac4b13b7f = private unnamed_addr constant [18 x i8] c"LocalHandle { .. }", align 1

@_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector = unnamed_addr alias ptr (), ptr @_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default17default_collector

; <crossbeam_epoch::guard::Guard>::defer_unchecked::<<crossbeam_epoch::guard::Guard>::defer_destroy<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>::{closure#0}, crossbeam_epoch::atomic::Owned<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB3_5Guard15defer_uncheckedNCINvB2_13defer_destroyINtNtNtB5_4sync5queue4NodeNtNtB5_8internal9SealedBagEE0INtNtB5_6atomic5OwnedB1v_EEB5_(ptr captures(address_is_null) %self.0.val, i64 noundef %f) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_13.i.i = alloca [2048 x i8], align 8
  %bag1.i.i = alloca [2056 x i8], align 8
  %0 = icmp eq ptr %self.0.val, null
  br i1 %0, label %bb8, label %bb9

bb8:                                              ; preds = %start
  %raw.i.i = and i64 %f, -8
  %_3.i.i.i = inttoptr i64 %raw.i.i to ptr
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.i.i.i, i64 noundef 2072, i64 noundef 8) #18
  br label %bb4

bb9:                                              ; preds = %start
  %_9.i = getelementptr inbounds nuw i8, ptr %self.0.val, i64 16
  %1 = getelementptr inbounds nuw i8, ptr %self.0.val, i64 2064
  %_114.i = load i64, ptr %1, align 8, !noalias !2, !noundef !5
  %_105.i = icmp ult i64 %_114.i, 64
  br i1 %_105.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit, label %bb5.lr.ph.i

bb5.lr.ph.i:                                      ; preds = %bb9
  %_15.i = getelementptr inbounds nuw i8, ptr %self.0.val, i64 8
  br label %bb5.i

bb5.i:                                            ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i, %bb5.lr.ph.i
  %_17.i = load ptr, ptr %_15.i, align 8, !noalias !2, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !6)
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_13.i.i), !noalias !2
  call void @llvm.lifetime.start.p0(i64 2056, ptr nonnull %bag1.i.i)
  br label %repeat_loop_body.i.i

repeat_loop_body.i.i:                             ; preds = %repeat_loop_body.i.i, %bb5.i
  %2 = phi i64 [ 0, %bb5.i ], [ %4, %repeat_loop_body.i.i ]
  %3 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_13.i.i, i64 %2
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %3, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !9
  %4 = add nuw nsw i64 %2, 1
  %exitcond.not.i.i = icmp eq i64 %4, 64
  br i1 %exitcond.not.i.i, label %bb6.i.i, label %repeat_loop_body.i.i

bb6.i.i:                                          ; preds = %repeat_loop_body.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i, ptr noundef nonnull align 16 dereferenceable(2056) %_9.i, i64 2056, i1 false), !noalias !2
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2056) %_9.i, ptr noundef nonnull align 8 dereferenceable(2048) %_13.i.i, i64 2048, i1 false), !noalias !2
  store i64 0, ptr %1, align 8, !alias.scope !6, !noalias !2
  fence seq_cst
  %_16.i.i = getelementptr inbounds nuw i8, ptr %_17.i, i64 384
  %5 = load atomic i64, ptr %_16.i.i monotonic, align 8, !noalias !9
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !10
; call __rustc::__rust_alloc
  %6 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !10
  %7 = icmp eq ptr %6, null
  br i1 %7, label %bb2.i.i.i.i, label %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i, !prof !15

bb2.i.i.i.i:                                      ; preds = %bb6.i.i
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19, !noalias !9
  unreachable

_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i: ; preds = %bb6.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %6, ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i, i64 2056, i1 false), !noalias !9
  %_10.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %6, i64 2056
  store i64 %5, ptr %_10.sroa.4.0..sroa_idx.i.i, align 8, !noalias !9
  %_4.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %6, i64 2064
  store i64 0, ptr %_4.sroa.4.0..sroa_idx.i.i.i, align 8, !noalias !16
  %new.i.i.i = ptrtoint ptr %6 to i64
  %_20.i.i.i = getelementptr inbounds nuw i8, ptr %_17.i, i64 256
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb1.i.i.i.backedge, %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i
  %8 = load atomic i64, ptr %_20.i.i.i acquire, align 8, !noalias !16
  %raw.i.i.i.i = and i64 %8, -8
  %_2.i.i.i.i.i = inttoptr i64 %raw.i.i.i.i to ptr
  %_18.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i.i, i64 2064
  %9 = load atomic i64, ptr %_18.i.i.i.i acquire, align 8, !noalias !17
  %.not.i.i.i.i = icmp ult i64 %9, 8
  br i1 %.not.i.i.i.i, label %bb5.i.i.i.i, label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb1.i.i.i
  %10 = cmpxchg ptr %_20.i.i.i, i64 %8, i64 %9 release monotonic, align 8, !noalias !20
  br label %bb1.i.i.i.backedge

bb5.i.i.i.i:                                      ; preds = %bb1.i.i.i
  %11 = cmpxchg ptr %_18.i.i.i.i, i64 0, i64 %new.i.i.i release monotonic, align 8, !noalias !23
  %_8.sroa.18.0.in.i.i9.i.i.i = extractvalue { i64, i1 } %11, 1
  br i1 %_8.sroa.18.0.in.i.i9.i.i.i, label %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i, label %bb1.i.i.i.backedge

bb1.i.i.i.backedge:                               ; preds = %bb5.i.i.i.i, %bb3.i.i.i.i
  br label %bb1.i.i.i

_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i: ; preds = %bb5.i.i.i.i
  %12 = cmpxchg ptr %_20.i.i.i, i64 %8, i64 %new.i.i.i release monotonic, align 8, !noalias !26
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_13.i.i), !noalias !2
  call void @llvm.lifetime.end.p0(i64 2056, ptr nonnull %bag1.i.i)
  %_11.i = load i64, ptr %1, align 8, !noalias !2, !noundef !5
  %_10.i = icmp ult i64 %_11.i, 64
  br i1 %_10.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit, label %bb5.i

_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit: ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i, %bb9
  %_11.lcssa.i = phi i64 [ %_114.i, %bb9 ], [ %_11.i, %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i ]
  %13 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_9.i, i64 %_11.lcssa.i
  store ptr @_RINvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB7_8Deferred3new4callNCINvMNtB9_5guardNtB1g_5Guard15defer_uncheckedNCINvB1f_13defer_destroyINtNtNtB9_4sync5queue4NodeNtNtB9_8internal9SealedBagEE0INtNtB9_6atomic5OwnedB2i_EE0EB9_, ptr %13, align 16
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %13, i64 8
  store i64 %f, ptr %_7.sroa.4.0..sroa_idx, align 8
  %14 = load i64, ptr %1, align 8, !noalias !2, !noundef !5
  %15 = add i64 %14, 1
  store i64 %15, ptr %1, align 8, !noalias !2
  br label %bb4

bb4:                                              ; preds = %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit, %bb8
  ret void
}

; <crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB6_8OnceLockNtNtBa_9collector9CollectorE10initializeNvMs1_B1b_B19_3newEBa_() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_11.i = alloca [8 x i8], align 8
  %f1.i = alloca [8 x i8], align 8
  %slot = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %slot)
  store ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR, i64 8), ptr %slot, align 8
  %0 = load atomic ptr, ptr @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR acquire, align 8, !noalias !29
  %_3.i = icmp eq ptr %0, null
  br i1 %_3.i, label %_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB15_8OnceLockNtNtB19_9collector9CollectorE10initializeNvMs1_B2b_B29_3newE0EB19_.exit, label %bb2.i, !prof !32

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %f1.i), !noalias !29
  store ptr %slot, ptr %f1.i, align 8, !noalias !29
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_11.i), !noalias !29
  store ptr %f1.i, ptr %_11.i, align 8, !noalias !29
; call <std::sys::sync::once::queue::Once>::call
  call void @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync4once5queueNtB2_4Once4call(ptr noundef nonnull align 8 @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR, i1 noundef zeroext false, ptr noundef nonnull align 1 %_11.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(40) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_051d8689b267677c3f8147a749eccac6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_11.i), !noalias !29
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %f1.i), !noalias !29
  br label %_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB15_8OnceLockNtNtB19_9collector9CollectorE10initializeNvMs1_B2b_B29_3newE0EB19_.exit

_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB15_8OnceLockNtNtB19_9collector9CollectorE10initializeNvMs1_B2b_B29_3newE0EB19_.exit: ; preds = %start, %bb2.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %slot)
  ret void
}

; core::ptr::drop_in_place::<crossbeam_epoch::sync::list::List<crossbeam_epoch::internal::Local>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4list4ListNtNtBN_8internal5LocalEEBN_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #0 {
start:
  %_10.i = alloca [8 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !33)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_10.i)
  %0 = load atomic i64, ptr %_1 monotonic, align 8, !alias.scope !33
  %raw.i6.i = and i64 %0, -8
  %.not8.i = icmp eq i64 %raw.i6.i, 0
  br i1 %.not8.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb3.i

bb3.i:                                            ; preds = %start, %bb4.i
  %_2.i.i9.in.i = phi i64 [ %raw.i.i, %bb4.i ], [ %raw.i6.i, %start ]
  %_2.i.i9.i = inttoptr i64 %_2.i.i9.in.i to ptr
  %1 = load atomic i64, ptr %_2.i.i9.i monotonic, align 8, !noalias !33
  %2 = and i64 %1, 7
  store i64 %2, ptr %_10.i, align 8, !noalias !33
  %3 = icmp eq i64 %2, 1
  br i1 %3, label %bb4.i, label %bb5.i, !prof !32

bb4.i:                                            ; preds = %bb3.i
; call <crossbeam_epoch::internal::Local as crossbeam_epoch::sync::list::IsElement<crossbeam_epoch::internal::Local>>::finalize
  tail call void @_RNvXs7_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5LocalINtNtNtB7_4sync4list9IsElementBL_E8finalize(ptr noundef nonnull align 8 %_2.i.i9.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch5guard11unprotected11UNPROTECTED), !noalias !33
  %raw.i.i = and i64 %1, -8
  %.not.i = icmp eq i64 %raw.i.i, 0
  br i1 %.not.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb3.i

bb5.i:                                            ; preds = %bb3.i
; call core::panicking::assert_failed::<usize, usize>
  call void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_10.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @alloc_edb3ed16ed07237eac14eb16826c52e0, ptr noundef null, ptr undef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8ec3bc029f16021168b4faa1ad83b4c0) #20, !noalias !33
  unreachable

_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit: ; preds = %bb4.i, %start
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_10.i)
  ret void
}

; core::ptr::drop_in_place::<crossbeam_epoch::sync::queue::Queue<crossbeam_epoch::internal::SealedBag>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue5QueueNtNtBN_8internal9SealedBagEEBN_(ptr noalias noundef nonnull align 128 captures(none) dereferenceable(256) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_9.i.i.i.i.i = alloca [32 x i8], align 8
  %_3.i = alloca [2064 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !36)
  call void @llvm.lifetime.start.p0(i64 2064, ptr nonnull %_3.i), !noalias !36
  %0 = load atomic i64, ptr %_1 acquire, align 128, !alias.scope !36, !noalias !39
  %raw.i5158.i = and i64 %0, -8
  %_2.i.i5259.i = inttoptr i64 %raw.i5158.i to ptr
  %_23.i5360.i = getelementptr inbounds nuw i8, ptr %_2.i.i5259.i, i64 2064
  %1 = load atomic i64, ptr %_23.i5360.i acquire, align 8, !noalias !42
  %raw.i35461.i = and i64 %1, -8
  %.not.i5562.i = icmp eq i64 %raw.i35461.i, 0
  br i1 %.not.i5562.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb5.i.lr.ph.lr.ph.i

bb5.i.lr.ph.lr.ph.i:                              ; preds = %start
  %_18.i.i = getelementptr inbounds nuw i8, ptr %_1, i64 128
  %_7.sroa.9.8._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %2 = getelementptr inbounds nuw i8, ptr %_3.i, i64 2048
  %_35.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_9.i.i.i.i.i, i64 8
  br label %bb5.i.lr.ph.i

bb5.i.lr.ph.i:                                    ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i, %bb5.i.lr.ph.lr.ph.i
  %raw.i35464.i = phi i64 [ %raw.i35461.i, %bb5.i.lr.ph.lr.ph.i ], [ %raw.i354.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i ]
  %3 = phi i64 [ %1, %bb5.i.lr.ph.lr.ph.i ], [ %12, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i ]
  %_2.i.i5263.i = phi ptr [ %_2.i.i5259.i, %bb5.i.lr.ph.lr.ph.i ], [ %_2.i.i52.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i ]
  %4 = phi i64 [ %0, %bb5.i.lr.ph.lr.ph.i ], [ %11, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i ]
  %5 = cmpxchg ptr %_1, i64 %4, i64 %3 release monotonic, align 8, !alias.scope !36, !noalias !43
  %_8.sroa.18.0.in.i.i.i7 = extractvalue { i64, i1 } %5, 1
  br i1 %_8.sroa.18.0.in.i.i.i7, label %bb11.i.i, label %bb9.i

bb5.i.i:                                          ; preds = %bb9.i
  %6 = cmpxchg ptr %_1, i64 %9, i64 %10 release monotonic, align 8, !alias.scope !36, !noalias !43
  %_8.sroa.18.0.in.i.i.i = extractvalue { i64, i1 } %6, 1
  br i1 %_8.sroa.18.0.in.i.i.i, label %bb11.i.i, label %bb9.i

bb11.i.i:                                         ; preds = %bb5.i.i, %bb5.i.lr.ph.i
  %raw.i357.i.lcssa = phi i64 [ %raw.i35464.i, %bb5.i.lr.ph.i ], [ %raw.i3.i, %bb5.i.i ]
  %.lcssa4 = phi i64 [ %3, %bb5.i.lr.ph.i ], [ %10, %bb5.i.i ]
  %_2.i.i56.i.lcssa = phi ptr [ %_2.i.i5263.i, %bb5.i.lr.ph.i ], [ %_2.i.i.i, %bb5.i.i ]
  %.lcssa = phi i64 [ %4, %bb5.i.lr.ph.i ], [ %9, %bb5.i.i ]
  %_2.i.i4.le.i = inttoptr i64 %raw.i357.i.lcssa to ptr
  %7 = load atomic i64, ptr %_18.i.i monotonic, align 128, !alias.scope !36, !noalias !46
  %_4.i.i = icmp eq i64 %.lcssa, %7
  br i1 %_4.i.i, label %bb1.i.i, label %bb8.i.i

bb1.i.i:                                          ; preds = %bb11.i.i
  %8 = cmpxchg ptr %_18.i.i, i64 %.lcssa, i64 %.lcssa4 release monotonic, align 8, !alias.scope !36, !noalias !50
  br label %bb8.i.i

bb8.i.i:                                          ; preds = %bb1.i.i, %bb11.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.i.i56.i.lcssa, i64 noundef 2072, i64 noundef 8) #18, !noalias !53
  %_26.i.sroa.0.0.copyload.i = load ptr, ptr %_2.i.i4.le.i, align 8, !noalias !54
  %_26.i.sroa.4.0._14.i.sroa.10.32._15.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i4.le.i, i64 8
  store ptr %_26.i.sroa.0.0.copyload.i, ptr %_3.i, align 8, !noalias !36
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %_7.sroa.9.8._3.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(2056) %_26.i.sroa.4.0._14.i.sroa.10.32._15.i.sroa_idx.i, i64 2056, i1 false), !noalias !36
  %.not.i = icmp eq ptr %_26.i.sroa.0.0.copyload.i, null
  br i1 %.not.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb2.i.i

bb9.i:                                            ; preds = %bb5.i.lr.ph.i, %bb5.i.i
  %9 = load atomic i64, ptr %_1 acquire, align 128, !alias.scope !36, !noalias !39
  %raw.i.i = and i64 %9, -8
  %_2.i.i.i = inttoptr i64 %raw.i.i to ptr
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i, i64 2064
  %10 = load atomic i64, ptr %_23.i.i acquire, align 8, !noalias !42
  %raw.i3.i = and i64 %10, -8
  %.not.i.i = icmp eq i64 %raw.i3.i, 0
  br i1 %.not.i.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb5.i.i

bb2.i.i:                                          ; preds = %bb8.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !55)
  call void @llvm.experimental.noalias.scope.decl(metadata !58)
  call void @llvm.experimental.noalias.scope.decl(metadata !61)
  call void @llvm.experimental.noalias.scope.decl(metadata !64)
  %_5.i.i.i.i.i = load i64, ptr %2, align 8, !alias.scope !67, !noalias !36, !noundef !5
  %_11.i.i.i.i.i = icmp ult i64 %_5.i.i.i.i.i, 65
  br i1 %_11.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb3.i.i.i.i.i, !prof !68

bb3.i.i.i.i.i:                                    ; preds = %bb2.i.i
; call core::slice::index::slice_index_fail
  call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5.i.i.i.i.i, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20, !noalias !69
  unreachable

bb2.i.i.i.i.i:                                    ; preds = %bb2.i.i
  %_18.idx.i.i.i.i.i = shl nuw nsw i64 %_5.i.i.i.i.i, 5
  %_18.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 %_18.idx.i.i.i.i.i
  %_241.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 0
  br i1 %_241.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i, label %bb7.i.i.i.i.i

bb7.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i.i, %bb7.i.i.i.i.i
  %iter.sroa.0.02.i.i.i.i.i = phi ptr [ %_30.i.i.i.i.i, %bb7.i.i.i.i.i ], [ %_3.i, %bb2.i.i.i.i.i ]
  %_30.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02.i.i.i.i.i, i64 32
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i.i.i.i), !noalias !69
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i, i64 32, i1 false), !noalias !36
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !36
  %_32.i.i.i.i.i = load ptr, ptr %_9.i.i.i.i.i, align 8, !noalias !69, !nonnull !5, !noundef !5
  call void %_32.i.i.i.i.i(ptr noundef nonnull %_35.i.i.i.i.i), !noalias !69
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i.i.i.i), !noalias !69
  %_24.i.i.i.i.i = icmp eq ptr %_30.i.i.i.i.i, %_18.i.i.i.i.i
  br i1 %_24.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i, label %bb7.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i: ; preds = %bb7.i.i.i.i.i, %bb2.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 2064, ptr nonnull %_3.i), !noalias !36
  call void @llvm.lifetime.start.p0(i64 2064, ptr nonnull %_3.i), !noalias !36
  %11 = load atomic i64, ptr %_1 acquire, align 128, !alias.scope !36, !noalias !39
  %raw.i51.i = and i64 %11, -8
  %_2.i.i52.i = inttoptr i64 %raw.i51.i to ptr
  %_23.i53.i = getelementptr inbounds nuw i8, ptr %_2.i.i52.i, i64 2064
  %12 = load atomic i64, ptr %_23.i53.i acquire, align 8, !noalias !42
  %raw.i354.i = and i64 %12, -8
  %.not.i55.i = icmp eq i64 %raw.i354.i, 0
  br i1 %.not.i55.i, label %_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit, label %bb5.i.lr.ph.i

_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_.exit: ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_.exit.i, %bb8.i.i, %bb9.i, %start
  call void @llvm.lifetime.end.p0(i64 2064, ptr nonnull %_3.i), !noalias !36
  %13 = load atomic i64, ptr %_1 monotonic, align 128, !alias.scope !36
  %raw.i.i.i = and i64 %13, -8
  %_3.i.i.i.i = inttoptr i64 %raw.i.i.i to ptr
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.i.i.i.i, i64 noundef 2072, i64 noundef 8) #18, !noalias !36
  ret void
}

; core::ptr::drop_in_place::<crossbeam_epoch::internal::Local>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_(ptr noalias noundef nonnull align 128 captures(address) dereferenceable(2304) %_1) unnamed_addr #0 {
start:
  %_9.i.i.i.i = alloca [32 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !70)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !73)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !76)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 2064
  %_5.i.i.i.i = load i64, ptr %1, align 16, !alias.scope !82, !noundef !5
  %_11.i.i.i.i = icmp ult i64 %_5.i.i.i.i, 65
  br i1 %_11.i.i.i.i, label %bb2.i.i.i.i, label %bb3.i.i.i.i, !prof !68

bb3.i.i.i.i:                                      ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5.i.i.i.i, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20, !noalias !82
  unreachable

bb2.i.i.i.i:                                      ; preds = %start
  %_18.idx.i.i.i.i = shl nuw nsw i64 %_5.i.i.i.i, 5
  %_18.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 %_18.idx.i.i.i.i
  %_241.i.i.i.i = icmp eq i64 %_5.i.i.i.i, 0
  br i1 %_241.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_.exit, label %bb7.lr.ph.i.i.i.i

bb7.lr.ph.i.i.i.i:                                ; preds = %bb2.i.i.i.i
  %_35.i.i.i.i = getelementptr inbounds nuw i8, ptr %_9.i.i.i.i, i64 8
  br label %bb7.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb7.i.i.i.i, %bb7.lr.ph.i.i.i.i
  %iter.sroa.0.02.i.i.i.i = phi ptr [ %0, %bb7.lr.ph.i.i.i.i ], [ %_30.i.i.i.i, %bb7.i.i.i.i ]
  %_30.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02.i.i.i.i, i64 32
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i.i.i), !noalias !82
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %_32.i.i.i.i = load ptr, ptr %_9.i.i.i.i, align 8, !noalias !82, !nonnull !5, !noundef !5
  call void %_32.i.i.i.i(ptr noundef nonnull %_35.i.i.i.i), !noalias !82
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i.i.i), !noalias !82
  %_24.i.i.i.i = icmp eq ptr %_30.i.i.i.i, %_18.i.i.i.i
  br i1 %_24.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_.exit, label %bb7.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_.exit: ; preds = %bb7.i.i.i.i, %bb2.i.i.i.i
  ret void
}

; core::ptr::drop_in_place::<crossbeam_epoch::internal::Global>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEBK_(ptr noalias noundef nonnull align 128 captures(none) dereferenceable(512) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_10.i.i = alloca [8 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 384
  tail call void @llvm.experimental.noalias.scope.decl(metadata !83)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !86)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_10.i.i), !noalias !83
  %1 = load atomic i64, ptr %0 monotonic, align 128, !alias.scope !89
  %raw.i6.i.i = and i64 %1, -8
  %.not8.i.i = icmp eq i64 %raw.i6.i.i, 0
  br i1 %.not8.i.i, label %bb4, label %bb3.i.i

bb3.i.i:                                          ; preds = %start, %.noexc
  %_2.i.i9.in.i.i = phi i64 [ %raw.i.i.i, %.noexc ], [ %raw.i6.i.i, %start ]
  %_2.i.i9.i.i = inttoptr i64 %_2.i.i9.in.i.i to ptr
  %2 = load atomic i64, ptr %_2.i.i9.i.i monotonic, align 8, !noalias !89
  %3 = and i64 %2, 7
  store i64 %3, ptr %_10.i.i, align 8, !noalias !89
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb4.i.i, label %bb5.i.i, !prof !32

bb4.i.i:                                          ; preds = %bb3.i.i
; invoke <crossbeam_epoch::internal::Local as crossbeam_epoch::sync::list::IsElement<crossbeam_epoch::internal::Local>>::finalize
  invoke void @_RNvXs7_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5LocalINtNtNtB7_4sync4list9IsElementBL_E8finalize(ptr noundef nonnull align 8 %_2.i.i9.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch5guard11unprotected11UNPROTECTED)
          to label %.noexc unwind label %cleanup.loopexit

.noexc:                                           ; preds = %bb4.i.i
  %raw.i.i.i = and i64 %2, -8
  %.not.i.i = icmp eq i64 %raw.i.i.i, 0
  br i1 %.not.i.i, label %bb4, label %bb3.i.i

bb5.i.i:                                          ; preds = %bb3.i.i
; invoke core::panicking::assert_failed::<usize, usize>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_10.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @alloc_edb3ed16ed07237eac14eb16826c52e0, ptr noundef null, ptr undef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8ec3bc029f16021168b4faa1ad83b4c0) #20
          to label %.noexc1 unwind label %cleanup.loopexit.split-lp

.noexc1:                                          ; preds = %bb5.i.i
  unreachable

cleanup.loopexit:                                 ; preds = %bb4.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup.loopexit.split-lp:                        ; preds = %bb5.i.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup

cleanup:                                          ; preds = %cleanup.loopexit.split-lp, %cleanup.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.split-lp ]
; invoke core::ptr::drop_in_place::<crossbeam_epoch::sync::queue::Queue<crossbeam_epoch::internal::SealedBag>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue5QueueNtNtBN_8internal9SealedBagEEBN_(ptr noalias noundef align 128 dereferenceable(256) %_1) #21
          to label %bb1 unwind label %terminate

bb4:                                              ; preds = %.noexc, %start
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_10.i.i), !noalias !83
; call core::ptr::drop_in_place::<crossbeam_epoch::sync::queue::Queue<crossbeam_epoch::internal::SealedBag>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue5QueueNtNtBN_8internal9SealedBagEEBN_(ptr noalias noundef align 128 dereferenceable(256) %_1)
  ret void

terminate:                                        ; preds = %cleanup
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

bb1:                                              ; preds = %cleanup
  resume { ptr, i32 } %lpad.phi
}

; <crossbeam_epoch::deferred::Deferred>::new::call::<<crossbeam_epoch::guard::Guard>::defer_unchecked<<crossbeam_epoch::guard::Guard>::defer_destroy<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>::{closure#0}, crossbeam_epoch::atomic::Owned<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>>::{closure#0}>
; Function Attrs: nounwind uwtable
define internal void @_RINvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB7_8Deferred3new4callNCINvMNtB9_5guardNtB1g_5Guard15defer_uncheckedNCINvB1f_13defer_destroyINtNtNtB9_4sync5queue4NodeNtNtB9_8internal9SealedBagEE0INtNtB9_6atomic5OwnedB2i_EE0EB9_(ptr noundef readonly captures(none) %raw) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %f = load i64, ptr %raw, align 8, !noundef !5
  %raw.i.i.i = and i64 %f, -8
  %_3.i.i.i.i = inttoptr i64 %raw.i.i.i to ptr
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.i.i.i.i, i64 noundef 2072, i64 noundef 8) #18
  ret void
}

; <crossbeam_epoch::deferred::Deferred>::new::call::<<crossbeam_epoch::guard::Guard>::defer_unchecked<<crossbeam_epoch::guard::Guard>::defer_destroy<crossbeam_epoch::internal::Local>::{closure#0}, crossbeam_epoch::atomic::Owned<crossbeam_epoch::internal::Local>>::{closure#0}>
; Function Attrs: uwtable
define internal void @_RINvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB7_8Deferred3new4callNCINvMNtB9_5guardNtB1g_5Guard15defer_uncheckedNCINvB1f_13defer_destroyNtNtB9_8internal5LocalE0INtNtB9_6atomic5OwnedB2i_EE0EB9_(ptr noundef readonly captures(none) %raw) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_9.i.i.i.i.i.i.i.i.i.i = alloca [32 x i8], align 8
  %f = load i64, ptr %raw, align 8, !noundef !5
  %raw.i.i.i = and i64 %f, -128
  %_3.i.i.i.i = inttoptr i64 %raw.i.i.i to ptr
  tail call void @llvm.experimental.noalias.scope.decl(metadata !90)
  %0 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !93)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !96)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !99)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !102)
  %1 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i, i64 2064
  %_5.i.i.i.i.i.i.i.i.i.i = load i64, ptr %1, align 16, !alias.scope !105, !noundef !5
  %_11.i.i.i.i.i.i.i.i.i.i = icmp ult i64 %_5.i.i.i.i.i.i.i.i.i.i, 65
  br i1 %_11.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i, !prof !68

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %start
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5.i.i.i.i.i.i.i.i.i.i, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20
          to label %.noexc.i.i.i.i.i unwind label %bb1.loopexit.split-lp.i.i.i.i.i

.noexc.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  unreachable

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %start
  %_18.idx.i.i.i.i.i.i.i.i.i.i = shl nuw nsw i64 %_5.i.i.i.i.i.i.i.i.i.i, 5
  %_18.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 %_18.idx.i.i.i.i.i.i.i.i.i.i
  %_241.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_241.i.i.i.i.i.i.i.i.i.i, label %_RNCINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB5_5Guard15defer_uncheckedNCINvB4_13defer_destroyNtNtB7_8internal5LocalE0INtNtB7_6atomic5OwnedB1x_EE0B7_.exit, label %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i

bb7.lr.ph.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
  %_35.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_9.i.i.i.i.i.i.i.i.i.i, i64 8
  br label %bb7.i.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i.i.i:                          ; preds = %.noexc2.i.i.i.i.i, %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i = phi ptr [ %0, %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i ], [ %_30.i.i.i.i.i.i.i.i.i.i, %.noexc2.i.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i.i.i.i.i.i.i.i.i), !noalias !105
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %_32.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %_9.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !105, !nonnull !5, !noundef !5
  invoke void %_32.i.i.i.i.i.i.i.i.i.i(ptr noundef nonnull %_35.i.i.i.i.i.i.i.i.i.i)
          to label %.noexc2.i.i.i.i.i unwind label %bb1.loopexit.i.i.i.i.i

.noexc2.i.i.i.i.i:                                ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, i64 32
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i.i.i.i.i.i.i.i.i), !noalias !105
  %_24.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %_30.i.i.i.i.i.i.i.i.i.i, %_18.i.i.i.i.i.i.i.i.i.i
  br i1 %_24.i.i.i.i.i.i.i.i.i.i, label %_RNCINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB5_5Guard15defer_uncheckedNCINvB4_13defer_destroyNtNtB7_8internal5LocalE0INtNtB7_6atomic5OwnedB1x_EE0B7_.exit, label %bb7.i.i.i.i.i.i.i.i.i.i

bb1.loopexit.i.i.i.i.i:                           ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.i.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb1.i.i.i.i.i

bb1.loopexit.split-lp.i.i.i.i.i:                  ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.split-lp.i.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb1.loopexit.split-lp.i.i.i.i.i, %bb1.loopexit.i.i.i.i.i
  %lpad.phi.i.i.i.i.i = phi { ptr, i32 } [ %lpad.loopexit.i.i.i.i.i, %bb1.loopexit.i.i.i.i.i ], [ %lpad.loopexit.split-lp.i.i.i.i.i, %bb1.loopexit.split-lp.i.i.i.i.i ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.i.i.i.i, i64 noundef 2304, i64 noundef 128) #18
  resume { ptr, i32 } %lpad.phi.i.i.i.i.i

_RNCINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB5_5Guard15defer_uncheckedNCINvB4_13defer_destroyNtNtB7_8internal5LocalE0INtNtB7_6atomic5OwnedB1x_EE0B7_.exit: ; preds = %.noexc2.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.i.i.i.i, i64 noundef 2304, i64 noundef 128) #18
  ret void
}

; <std::sync::once::Once>::call_once::<<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize<<crossbeam_epoch::collector::Collector>::new>::{closure#0}>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal void @_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB17_8OnceLockNtNtB1b_9collector9CollectorE10initializeNvMs1_B2d_B2b_3newE0E0B1b_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1, ptr nonnull readnone align 8 captures(none) %_2) unnamed_addr #3 {
start:
  %self1 = load ptr, ptr %_1, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = load ptr, ptr %self1, align 8, !align !106, !noundef !5
  store ptr null, ptr %self1, align 8
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb3, label %bb4, !prof !15

bb4:                                              ; preds = %start
  %.val = load ptr, ptr %0, align 8
; call <crossbeam_epoch::collector::Collector as core::default::Default>::default
  %_0.i.i.i = tail call noalias noundef nonnull ptr @_RNvXs0_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9CollectorNtNtCsjMrxcFdYDNN_4core7default7Default7default()
  store ptr %_0.i.i.i, ptr %.val, align 8
  ret void

bb3:                                              ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3129176114b12752e73d3989241bb5f0) #20
  unreachable
}

; <<std::sync::once::Once>::call_once<<crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize<<crossbeam_epoch::collector::Collector>::new>::{closure#0}>::{closure#0} as core::ops::function::FnOnce<(&std::sync::once::OnceState,)>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal void @_RNSNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBd_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1c_8OnceLockNtNtB1g_9collector9CollectorE10initializeNvMs1_B2i_B2g_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBd_9OnceStateEE9call_once6vtableB1g_(ptr noundef readonly captures(none) %_1, ptr nonnull readnone align 8 captures(none) %0) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = load ptr, ptr %_1, align 8, !nonnull !5, !align !106, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !107)
  %2 = load ptr, ptr %1, align 8, !alias.scope !107, !noalias !110, !align !106, !noundef !5
  store ptr null, ptr %1, align 8, !alias.scope !107, !noalias !110
  %.not.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i, label %bb3.i.i, label %_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1a_8OnceLockNtNtB1e_9collector9CollectorE10initializeNvMs1_B2g_B2e_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_.exit, !prof !15

bb3.i.i:                                          ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3129176114b12752e73d3989241bb5f0) #20, !noalias !113
  unreachable

_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1a_8OnceLockNtNtB1e_9collector9CollectorE10initializeNvMs1_B2g_B2e_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_.exit: ; preds = %start
  %.val.i.i = load ptr, ptr %2, align 8, !noalias !113
; call <crossbeam_epoch::collector::Collector as core::default::Default>::default
  %_0.i.i.i.i.i = tail call noalias noundef nonnull ptr @_RNvXs0_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9CollectorNtNtCsjMrxcFdYDNN_4core7default7Default7default(), !noalias !113
  store ptr %_0.i.i.i.i.i, ptr %.val.i.i, align 8, !noalias !113
  ret void
}

; <crossbeam_epoch::guard::Guard>::flush
; Function Attrs: uwtable
define void @_RNvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB2_5Guard5flush(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_13.i.i = alloca [2048 x i8], align 8
  %bag1.i.i = alloca [2056 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !noundef !5
  %0 = icmp eq ptr %_3, null
  br i1 %0, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_3, i64 2064
  %_10.i = load i64, ptr %1, align 8, !noalias !114, !noundef !5
  %2 = icmp eq i64 %_10.i, 0
  br i1 %2, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush.exit, label %bb2.i

bb2.i:                                            ; preds = %bb3
  %_12.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_14.i = load ptr, ptr %_12.i, align 8, !noalias !114, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !117)
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_13.i.i), !noalias !114
  call void @llvm.lifetime.start.p0(i64 2056, ptr nonnull %bag1.i.i)
  br label %repeat_loop_body.i.i

repeat_loop_body.i.i:                             ; preds = %repeat_loop_body.i.i, %bb2.i
  %3 = phi i64 [ 0, %bb2.i ], [ %5, %repeat_loop_body.i.i ]
  %4 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_13.i.i, i64 %3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %4, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !120
  %5 = add nuw nsw i64 %3, 1
  %exitcond.not.i.i = icmp eq i64 %5, 64
  br i1 %exitcond.not.i.i, label %bb6.i.i, label %repeat_loop_body.i.i

bb6.i.i:                                          ; preds = %repeat_loop_body.i.i
  %_9.i = getelementptr inbounds nuw i8, ptr %_3, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i, ptr noundef nonnull align 16 dereferenceable(2056) %_9.i, i64 2056, i1 false), !noalias !114
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2056) %_9.i, ptr noundef nonnull align 8 dereferenceable(2048) %_13.i.i, i64 2048, i1 false), !noalias !114
  store i64 0, ptr %1, align 8, !alias.scope !117, !noalias !114
  fence seq_cst
  %_16.i.i = getelementptr inbounds nuw i8, ptr %_14.i, i64 384
  %6 = load atomic i64, ptr %_16.i.i monotonic, align 8, !noalias !120
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !121
; call __rustc::__rust_alloc
  %7 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !121
  %8 = icmp eq ptr %7, null
  br i1 %8, label %bb2.i.i.i.i, label %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i, !prof !15

bb2.i.i.i.i:                                      ; preds = %bb6.i.i
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19, !noalias !120
  unreachable

_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i: ; preds = %bb6.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %7, ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i, i64 2056, i1 false), !noalias !120
  %_10.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %7, i64 2056
  store i64 %6, ptr %_10.sroa.4.0..sroa_idx.i.i, align 8, !noalias !120
  %_4.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %7, i64 2064
  store i64 0, ptr %_4.sroa.4.0..sroa_idx.i.i.i, align 8, !noalias !126
  %new.i.i.i = ptrtoint ptr %7 to i64
  %_20.i.i.i = getelementptr inbounds nuw i8, ptr %_14.i, i64 256
  br label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb1.i.i.i.backedge, %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i
  %9 = load atomic i64, ptr %_20.i.i.i acquire, align 8, !noalias !126
  %raw.i.i.i.i = and i64 %9, -8
  %_2.i.i.i.i.i = inttoptr i64 %raw.i.i.i.i to ptr
  %_18.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i.i, i64 2064
  %10 = load atomic i64, ptr %_18.i.i.i.i acquire, align 8, !noalias !127
  %.not.i.i.i.i = icmp ult i64 %10, 8
  br i1 %.not.i.i.i.i, label %bb5.i.i.i.i, label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb1.i.i.i
  %11 = cmpxchg ptr %_20.i.i.i, i64 %9, i64 %10 release monotonic, align 8, !noalias !130
  br label %bb1.i.i.i.backedge

bb5.i.i.i.i:                                      ; preds = %bb1.i.i.i
  %12 = cmpxchg ptr %_18.i.i.i.i, i64 0, i64 %new.i.i.i release monotonic, align 8, !noalias !133
  %_8.sroa.18.0.in.i.i9.i.i.i = extractvalue { i64, i1 } %12, 1
  br i1 %_8.sroa.18.0.in.i.i9.i.i.i, label %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i, label %bb1.i.i.i.backedge

bb1.i.i.i.backedge:                               ; preds = %bb5.i.i.i.i, %bb3.i.i.i.i
  br label %bb1.i.i.i

_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i: ; preds = %bb5.i.i.i.i
  %13 = cmpxchg ptr %_20.i.i.i, i64 %9, i64 %new.i.i.i release monotonic, align 8, !noalias !136
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_13.i.i), !noalias !114
  call void @llvm.lifetime.end.p0(i64 2056, ptr nonnull %bag1.i.i)
  br label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush.exit

_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush.exit: ; preds = %bb3, %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i
  %_16.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_18.i = load ptr, ptr %_16.i, align 8, !noalias !114, !nonnull !5, !noundef !5
  %_7.i = getelementptr inbounds nuw i8, ptr %_18.i, i64 128
; call <crossbeam_epoch::internal::Global>::collect
  tail call void @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global7collect(ptr noundef nonnull align 128 %_7.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self)
  br label %bb1

bb1:                                              ; preds = %start, %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush.exit
  ret void
}

; <crossbeam_epoch::guard::Guard>::repin
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(readwrite, inaccessiblemem: none) uwtable
define void @_RNvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB2_5Guard5repin(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #4 {
start:
  %_3 = load ptr, ptr %self, align 8, !noundef !5
  %0 = icmp eq ptr %_3, null
  br i1 %0, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %_5.i = getelementptr inbounds nuw i8, ptr %_3, i64 2072
  %guard_count.i = load i64, ptr %_5.i, align 8, !noundef !5
  %1 = icmp eq i64 %guard_count.i, 1
  br i1 %1, label %bb1.i, label %bb1

bb1.i:                                            ; preds = %bb3
  %_8.i = getelementptr inbounds nuw i8, ptr %_3, i64 2176
  %2 = load atomic i64, ptr %_8.i monotonic, align 8
  %_10.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_12.i = load ptr, ptr %_10.i, align 8, !nonnull !5, !noundef !5
  %_15.i = getelementptr inbounds nuw i8, ptr %_12.i, i64 384
  %3 = load atomic i64, ptr %_15.i monotonic, align 8
  %global_epoch.i = or i64 %3, 1
  %_17.not.i = icmp eq i64 %2, %global_epoch.i
  br i1 %_17.not.i, label %bb1, label %bb2.i

bb2.i:                                            ; preds = %bb1.i
  store atomic i64 %global_epoch.i, ptr %_8.i release, align 8
  br label %bb1

bb1:                                              ; preds = %bb2.i, %bb1.i, %bb3, %start
  ret void
}

; <crossbeam_epoch::collector::Collector>::new
; Function Attrs: uwtable
define noalias noundef nonnull ptr @_RNvMs1_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9Collector3new() unnamed_addr #0 {
start:
; call <crossbeam_epoch::collector::Collector as core::default::Default>::default
  %_0 = tail call noundef nonnull ptr @_RNvXs0_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9CollectorNtNtCsjMrxcFdYDNN_4core7default7Default7default()
  ret ptr %_0
}

; <crossbeam_epoch::collector::Collector>::register
; Function Attrs: uwtable
define noundef ptr @_RNvMs1_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9Collector8register(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_29.i = alloca [2048 x i8], align 8
  %_3.i = alloca [2304 x i8], align 128
  %self.val = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_29.i)
  call void @llvm.lifetime.start.p0(i64 2304, ptr nonnull %_3.i)
  %0 = atomicrmw add ptr %self.val, i64 1 monotonic, align 8
  %_20.i = icmp slt i64 %0, 0
  br i1 %_20.i, label %bb2.i, label %repeat_loop_body.i

bb2.i:                                            ; preds = %start
  tail call void @llvm.trap()
  unreachable

repeat_loop_body.i:                               ; preds = %start, %repeat_loop_body.i
  %1 = phi i64 [ %3, %repeat_loop_body.i ], [ 0, %start ]
  %2 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_29.i, i64 %1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %2, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %3 = add nuw nsw i64 %1, 1
  %exitcond.not.i = icmp eq i64 %3, 64
  br i1 %exitcond.not.i, label %repeat_loop_next.i, label %repeat_loop_body.i

repeat_loop_next.i:                               ; preds = %repeat_loop_body.i
  store i64 0, ptr %_3.i, align 128
  %4 = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  store ptr %self.val, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2048) %5, ptr noundef nonnull align 8 dereferenceable(2048) %_29.i, i64 2048, i1 false)
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 2064
  %6 = getelementptr inbounds nuw i8, ptr %_3.i, i64 2080
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 16 dereferenceable(16) %_8.sroa.4.0..sroa_idx.i, i8 0, i64 16, i1 false)
  store <2 x i64> <i64 1, i64 0>, ptr %6, align 32
  %7 = getelementptr inbounds nuw i8, ptr %_3.i, i64 2176
  store i64 0, ptr %7, align 128
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !139
; call __rustc::__rust_alloc
  %8 = tail call noundef align 128 dereferenceable_or_null(2304) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2304, i64 noundef range(i64 8, 129) 128) #18, !noalias !139
  %9 = icmp eq ptr %8, null
  br i1 %9, label %bb2.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_.exit.i, !prof !15

bb2.i.i:                                          ; preds = %repeat_loop_next.i
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 128, i64 noundef 2304) #19
          to label %.noexc.i unwind label %cleanup.i.i

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb2.i.i
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<crossbeam_epoch::internal::Local>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_(ptr noalias noundef nonnull align 128 dereferenceable(2304) %_3.i) #21
          to label %bb3.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

bb3.i.i:                                          ; preds = %cleanup.i.i
  resume { ptr, i32 } %10

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_.exit.i: ; preds = %repeat_loop_next.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 128 dereferenceable(2304) %8, ptr noundef nonnull align 128 dereferenceable(2304) %_3.i, i64 2304, i1 false)
  %_32.i = ptrtoint ptr %8 to i64
  call void @llvm.lifetime.end.p0(i64 2304, ptr nonnull %_3.i)
  %_13.i = getelementptr inbounds nuw i8, ptr %self.val, i64 512
  %12 = load atomic i64, ptr %_13.i monotonic, align 8
  store atomic i64 %12, ptr %8 monotonic, align 128
  %13 = cmpxchg weak ptr %_13.i, i64 %12, i64 %_32.i release monotonic, align 8, !noalias !142
  %14 = extractvalue { i64, i1 } %13, 1
  br i1 %14, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8register.exit, label %bb6.i.i

bb6.i.i:                                          ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_.exit.i, %bb6.i.i
  %15 = phi { i64, i1 } [ %17, %bb6.i.i ], [ %13, %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_.exit.i ]
  %16 = extractvalue { i64, i1 } %15, 0
  store atomic i64 %16, ptr %8 monotonic, align 128
  %17 = cmpxchg weak ptr %_13.i, i64 %16, i64 %_32.i release monotonic, align 8, !noalias !142
  %18 = extractvalue { i64, i1 } %17, 1
  br i1 %18, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8register.exit, label %bb6.i.i

_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8register.exit: ; preds = %bb6.i.i, %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_.exit.i
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_29.i)
  ret ptr %8
}

; <crossbeam_epoch::internal::Global>::try_advance
; Function Attrs: cold uwtable
define internal fastcc noundef i64 @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global11try_advance(ptr noundef nonnull align 128 captures(none) %self, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(8) %guard) unnamed_addr #1 {
start:
  %_16 = getelementptr inbounds nuw i8, ptr %self, i64 256
  %0 = load atomic i64, ptr %_16 monotonic, align 128
  fence seq_cst
  %_17 = getelementptr inbounds nuw i8, ptr %self, i64 384
  %1 = load atomic i64, ptr %_17 acquire, align 128
  br label %bb2

bb2:                                              ; preds = %bb7, %start
  %iter.sroa.5.0 = phi ptr [ %_17, %start ], [ %_2.i.i24.i, %bb7 ]
  %iter.sroa.11.0 = phi i64 [ %1, %start ], [ %2, %bb7 ]
  %raw.i21.i = and i64 %iter.sroa.11.0, -8
  %.not23.i = icmp eq i64 %raw.i21.i, 0
  br i1 %.not23.i, label %bb6, label %bb3.i

bb3.i:                                            ; preds = %bb2, %bb13.i
  %iter.sroa.11.1 = phi i64 [ %succ.sroa.0.0.i, %bb13.i ], [ %iter.sroa.11.0, %bb2 ]
  %_2.i.i24.in.i = phi i64 [ %succ.sroa.0.0.i, %bb13.i ], [ %raw.i21.i, %bb2 ]
  %_2.i.i24.i = inttoptr i64 %_2.i.i24.in.i to ptr
  %2 = load atomic i64, ptr %_2.i.i24.i acquire, align 8, !noalias !145
  %_7.i = and i64 %2, 7
  %3 = icmp eq i64 %_7.i, 1
  br i1 %3, label %bb4.i, label %bb7

bb4.i:                                            ; preds = %bb3.i
  %_32.i = and i64 %2, -8
  %4 = cmpxchg ptr %iter.sroa.5.0, i64 %iter.sroa.11.1, i64 %_32.i acquire acquire, align 8, !noalias !148
  %_8.sroa.18.0.in.i.i.i = extractvalue { i64, i1 } %4, 1
  br i1 %_8.sroa.18.0.in.i.i.i, label %bb8.i, label %bb7.i

bb7.i:                                            ; preds = %bb4.i
  %_8.sroa.0.0.i.i.i = extractvalue { i64, i1 } %4, 0
  br label %bb11.i

bb8.i:                                            ; preds = %bb4.i
  %raw.i9.i = and i64 %iter.sroa.11.1, -8
  %_2.i.i10.i = inttoptr i64 %raw.i9.i to ptr
; call <crossbeam_epoch::internal::Local as crossbeam_epoch::sync::list::IsElement<crossbeam_epoch::internal::Local>>::finalize
  tail call void @_RNvXs7_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5LocalINtNtNtB7_4sync4list9IsElementBL_E8finalize(ptr noundef nonnull align 8 %_2.i.i10.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %guard), !noalias !145
  br label %bb11.i

bb11.i:                                           ; preds = %bb8.i, %bb7.i
  %succ.sroa.0.0.i = phi i64 [ %_8.sroa.0.0.i.i.i, %bb7.i ], [ %_32.i, %bb8.i ]
  %_15.i = and i64 %succ.sroa.0.0.i, 7
  %5 = icmp eq i64 %_15.i, 0
  br i1 %5, label %bb13.i, label %bb5.thread

bb13.i:                                           ; preds = %bb11.i
  %.not.i = icmp eq i64 %succ.sroa.0.0.i, 0
  br i1 %.not.i, label %bb6, label %bb3.i

bb5.thread:                                       ; preds = %bb11.i
  %6 = load atomic i64, ptr %_17 acquire, align 128, !noalias !145
  br label %bb15

bb6:                                              ; preds = %bb2, %bb13.i
  fence acquire
  %_29 = add i64 %0, 2
  store atomic i64 %_29, ptr %_16 release, align 128
  br label %bb15

bb15:                                             ; preds = %bb7, %bb5.thread, %bb6
  %global_epoch.sroa.0.0 = phi i64 [ %_29, %bb6 ], [ %0, %bb5.thread ], [ %0, %bb7 ]
  ret i64 %global_epoch.sroa.0.0

bb7:                                              ; preds = %bb3.i
  %_25 = getelementptr inbounds nuw i8, ptr %_2.i.i24.i, i64 2176
  %7 = load atomic i64, ptr %_25 monotonic, align 8
  %_26 = and i64 %7, 1
  %.not = icmp eq i64 %_26, 0
  %_27 = and i64 %7, -2
  %_28.not = icmp eq i64 %_27, %0
  %or.cond = or i1 %.not, %_28.not
  br i1 %or.cond, label %bb2, label %bb15
}

; <crossbeam_epoch::internal::Global>::collect
; Function Attrs: cold uwtable
define void @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global7collect(ptr noundef nonnull align 128 captures(none) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %guard) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_9.i.i.i = alloca [32 x i8], align 8
  %_12 = alloca [2064 x i8], align 8
; call <crossbeam_epoch::internal::Global>::try_advance
  %0 = tail call fastcc noundef i64 @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global11try_advance(ptr noundef nonnull align 128 %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %guard)
  %_18.i9.i = getelementptr inbounds nuw i8, ptr %self, i64 128
  %_14.val.i.i = load ptr, ptr %guard, align 8
  %sealed_bag.sroa.4.0._12.sroa_idx = getelementptr inbounds nuw i8, ptr %_12, i64 8
  %1 = getelementptr inbounds nuw i8, ptr %_12, i64 2048
  %_35.i.i.i = getelementptr inbounds nuw i8, ptr %_9.i.i.i, i64 8
  br label %bb8

bb8:                                              ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit
  %iter.sroa.0.09 = phi i64 [ 0, %start ], [ %_14, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit ]
  %2 = load atomic i64, ptr %self acquire, align 128, !noalias !151
  %raw.i28.i = and i64 %2, -8
  %_2.i.i29.i = inttoptr i64 %raw.i28.i to ptr
  %_30.i30.i = getelementptr inbounds nuw i8, ptr %_2.i.i29.i, i64 2064
  %3 = load atomic i64, ptr %_30.i30.i acquire, align 8, !noalias !151
  %raw.i531.i = and i64 %3, -8
  %.not.i33.i = icmp eq i64 %raw.i531.i, 0
  br i1 %.not.i33.i, label %bb7, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb8, %bb4.i
  %_2.i.i634.in.i = phi i64 [ %raw.i5.i, %bb4.i ], [ %raw.i531.i, %bb8 ]
  %4 = phi i64 [ %11, %bb4.i ], [ %3, %bb8 ]
  %5 = phi i64 [ %10, %bb4.i ], [ %2, %bb8 ]
  %_2.i.i634.i = inttoptr i64 %_2.i.i634.in.i to ptr
  %6 = getelementptr i8, ptr %_2.i.i634.i, i64 2056
  %.val.i = load i64, ptr %6, align 8, !noalias !159, !noundef !5
  %_7.i.i.i.i = and i64 %.val.i, -2
  %_6.i.i.i.i = sub i64 %0, %_7.i.i.i.i
  %_0.i.i.i.i = icmp sgt i64 %_6.i.i.i.i, 3
  br i1 %_0.i.i.i.i, label %bb7.i.i, label %bb7

bb7.i.i:                                          ; preds = %bb5.i.i
  %7 = cmpxchg ptr %self, i64 %5, i64 %4 release monotonic, align 8, !noalias !160
  %_8.sroa.18.0.in.i.i.i = extractvalue { i64, i1 } %7, 1
  br i1 %_8.sroa.18.0.in.i.i.i, label %bb17.i.i, label %bb4.i

bb17.i.i:                                         ; preds = %bb7.i.i
  %8 = load atomic i64, ptr %_18.i9.i monotonic, align 128, !noalias !163
  %_4.i.i = icmp eq i64 %5, %8
  br i1 %_4.i.i, label %bb1.i.i, label %_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_.exit

bb1.i.i:                                          ; preds = %bb17.i.i
  %9 = cmpxchg ptr %_18.i9.i, i64 %5, i64 %4 release monotonic, align 8, !noalias !167
  br label %_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_.exit

bb4.i:                                            ; preds = %bb7.i.i
  %10 = load atomic i64, ptr %self acquire, align 128, !noalias !151
  %raw.i.i = and i64 %10, -8
  %_2.i.i.i = inttoptr i64 %raw.i.i to ptr
  %_30.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i, i64 2064
  %11 = load atomic i64, ptr %_30.i.i acquire, align 8, !noalias !151
  %raw.i5.i = and i64 %11, -8
  %.not.i.i = icmp eq i64 %raw.i5.i, 0
  br i1 %.not.i.i, label %bb7, label %bb5.i.i

_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_.exit: ; preds = %bb17.i.i, %bb1.i.i
; call <crossbeam_epoch::guard::Guard>::defer_unchecked::<<crossbeam_epoch::guard::Guard>::defer_destroy<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>::{closure#0}, crossbeam_epoch::atomic::Owned<crossbeam_epoch::sync::queue::Node<crossbeam_epoch::internal::SealedBag>>>
  call fastcc void @_RINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB3_5Guard15defer_uncheckedNCINvB2_13defer_destroyINtNtNtB5_4sync5queue4NodeNtNtB5_8internal9SealedBagEE0INtNtB5_6atomic5OwnedB1v_EEB5_(ptr %_14.val.i.i, i64 noundef %5), !noalias !163
  %_34.i.sroa.0.0.copyload.i = load ptr, ptr %_2.i.i634.i, align 8, !noalias !170
  %.not = icmp eq ptr %_34.i.sroa.0.0.copyload.i, null
  br i1 %.not, label %bb7, label %bb5

bb7:                                              ; preds = %bb8, %_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit, %bb4.i, %bb5.i.i
  ret void

bb5:                                              ; preds = %_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_.exit
  %_34.i.sroa.4.0._20.i.sroa.10.32._15.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_2.i.i634.i, i64 8
  %_14 = add nuw nsw i64 %iter.sroa.0.09, 1
  call void @llvm.lifetime.start.p0(i64 2064, ptr nonnull %_12)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %sealed_bag.sroa.4.0._12.sroa_idx, ptr noundef nonnull align 8 dereferenceable(2056) %_34.i.sroa.4.0._20.i.sroa.10.32._15.i.sroa_idx.i, i64 2056, i1 false)
  store ptr %_34.i.sroa.0.0.copyload.i, ptr %_12, align 8
  call void @llvm.experimental.noalias.scope.decl(metadata !171)
  call void @llvm.experimental.noalias.scope.decl(metadata !174)
  call void @llvm.experimental.noalias.scope.decl(metadata !177)
  %_5.i.i.i = load i64, ptr %1, align 8, !alias.scope !180, !noundef !5
  %_11.i.i.i = icmp ult i64 %_5.i.i.i, 65
  br i1 %_11.i.i.i, label %bb2.i.i.i, label %bb3.i.i.i, !prof !68

bb3.i.i.i:                                        ; preds = %bb5
; call core::slice::index::slice_index_fail
  call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5.i.i.i, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20, !noalias !180
  unreachable

bb2.i.i.i:                                        ; preds = %bb5
  %_18.idx.i.i.i = shl nuw nsw i64 %_5.i.i.i, 5
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %_12, i64 %_18.idx.i.i.i
  %_241.i.i.i = icmp eq i64 %_5.i.i.i, 0
  br i1 %_241.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit, label %bb7.i.i.i

bb7.i.i.i:                                        ; preds = %bb2.i.i.i, %bb7.i.i.i
  %iter.sroa.0.02.i.i.i = phi ptr [ %_30.i.i.i, %bb7.i.i.i ], [ %_12, %bb2.i.i.i ]
  %_30.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02.i.i.i, i64 32
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i.i), !noalias !180
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %_32.i.i.i = load ptr, ptr %_9.i.i.i, align 8, !noalias !180, !nonnull !5, !noundef !5
  call void %_32.i.i.i(ptr noundef nonnull %_35.i.i.i), !noalias !180
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i.i), !noalias !180
  %_24.i.i.i = icmp eq ptr %_30.i.i.i, %_18.i.i.i
  br i1 %_24.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit, label %bb7.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_.exit: ; preds = %bb7.i.i.i, %bb2.i.i.i
  call void @llvm.lifetime.end.p0(i64 2064, ptr nonnull %_12)
  %exitcond.not = icmp eq i64 %_14, 8
  br i1 %exitcond.not, label %bb7, label %bb8
}

; <crossbeam_epoch::internal::Local>::pin
; Function Attrs: inlinehint uwtable
define internal fastcc noundef nonnull ptr @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin(ptr noundef nonnull returned align 128 %self) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %guard = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %guard)
  store ptr %self, ptr %guard, align 8
  %_15 = getelementptr inbounds nuw i8, ptr %self, i64 2072
  %guard_count = load i64, ptr %_15, align 8, !noundef !5
  %_17.1 = icmp eq i64 %guard_count, -1
  br i1 %_17.1, label %bb9, label %bb11, !prof !15

bb11:                                             ; preds = %start
  %_17.0 = add nuw i64 %guard_count, 1
  store i64 %_17.0, ptr %_15, align 8
  %0 = icmp eq i64 %guard_count, 0
  br i1 %0, label %bb2, label %bb6

bb9:                                              ; preds = %start
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ce925522e0221e39b5bde413077875f) #19
          to label %unreachable unwind label %cleanup

bb6:                                              ; preds = %bb2, %bb3, %bb11
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %guard)
  ret ptr %self

cleanup:                                          ; preds = %bb9, %bb3
  %1 = landingpad { ptr, i32 }
          cleanup
; invoke <crossbeam_epoch::guard::Guard as core::ops::drop::Drop>::drop
  invoke fastcc void @_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %guard) #23
          to label %bb8 unwind label %terminate

bb2:                                              ; preds = %bb11
  %_24 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_26 = load ptr, ptr %_24, align 8, !nonnull !5, !noundef !5
  %_29 = getelementptr inbounds nuw i8, ptr %_26, i64 384
  %2 = load atomic i64, ptr %_29 monotonic, align 8
  %new_epoch = or i64 %2, 1
  %_33 = getelementptr inbounds nuw i8, ptr %self, i64 2176
  store atomic i64 %new_epoch, ptr %_33 monotonic, align 128
  fence seq_cst
  %_35 = getelementptr inbounds nuw i8, ptr %self, i64 2088
  %count = load i64, ptr %_35, align 8, !noundef !5
  %_36 = add i64 %count, 1
  store i64 %_36, ptr %_35, align 8
  %_10 = and i64 %count, 127
  %3 = icmp eq i64 %_10, 0
  br i1 %3, label %bb3, label %bb6, !prof !15

bb3:                                              ; preds = %bb2
  %_42 = load ptr, ptr %_24, align 8, !nonnull !5, !noundef !5
  %_12 = getelementptr inbounds nuw i8, ptr %_42, i64 128
; invoke <crossbeam_epoch::internal::Global>::collect
  invoke void @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global7collect(ptr noundef nonnull align 128 %_12, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %guard)
          to label %bb6 unwind label %cleanup

unreachable:                                      ; preds = %bb9
  unreachable

terminate:                                        ; preds = %cleanup
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

bb8:                                              ; preds = %cleanup
  resume { ptr, i32 } %1
}

; <crossbeam_epoch::internal::Local>::defer
; Function Attrs: uwtable
define void @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer(ptr noundef nonnull align 128 captures(none) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32) %deferred, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %guard) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_13.i = alloca [2048 x i8], align 8
  %bag1.i = alloca [2056 x i8], align 8
  %_9 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2064
  %_114 = load i64, ptr %0, align 16, !noundef !5
  %_105 = icmp ult i64 %_114, 64
  br i1 %_105, label %bb4, label %bb5.lr.ph

bb5.lr.ph:                                        ; preds = %start
  %_15 = getelementptr inbounds nuw i8, ptr %self, i64 8
  br label %bb5

bb5:                                              ; preds = %bb5.lr.ph, %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit
  %_17 = load ptr, ptr %_15, align 8, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !181)
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_13.i)
  call void @llvm.lifetime.start.p0(i64 2056, ptr nonnull %bag1.i)
  br label %repeat_loop_body.i

repeat_loop_body.i:                               ; preds = %repeat_loop_body.i, %bb5
  %1 = phi i64 [ 0, %bb5 ], [ %3, %repeat_loop_body.i ]
  %2 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_13.i, i64 %1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %2, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !181
  %3 = add nuw nsw i64 %1, 1
  %exitcond.not.i = icmp eq i64 %3, 64
  br i1 %exitcond.not.i, label %bb6.i, label %repeat_loop_body.i

bb6.i:                                            ; preds = %repeat_loop_body.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i, ptr noundef nonnull align 16 dereferenceable(2056) %_9, i64 2056, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2056) %_9, ptr noundef nonnull align 8 dereferenceable(2048) %_13.i, i64 2048, i1 false)
  store i64 0, ptr %0, align 16, !alias.scope !181
  fence seq_cst
  %_16.i = getelementptr inbounds nuw i8, ptr %_17, i64 384
  %4 = load atomic i64, ptr %_16.i monotonic, align 8, !noalias !181
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !184
; call __rustc::__rust_alloc
  %5 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !184
  %6 = icmp eq ptr %5, null
  br i1 %6, label %bb2.i.i.i, label %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i, !prof !15

bb2.i.i.i:                                        ; preds = %bb6.i
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19, !noalias !181
  unreachable

_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i: ; preds = %bb6.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %5, ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i, i64 2056, i1 false), !noalias !181
  %_10.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %5, i64 2056
  store i64 %4, ptr %_10.sroa.4.0..sroa_idx.i, align 8, !noalias !181
  %_4.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %5, i64 2064
  store i64 0, ptr %_4.sroa.4.0..sroa_idx.i.i, align 8, !noalias !189
  %new.i.i = ptrtoint ptr %5 to i64
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_17, i64 256
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb1.i.i.backedge, %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i
  %7 = load atomic i64, ptr %_20.i.i acquire, align 8, !noalias !189
  %raw.i.i.i = and i64 %7, -8
  %_2.i.i.i.i = inttoptr i64 %raw.i.i.i to ptr
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i, i64 2064
  %8 = load atomic i64, ptr %_18.i.i.i acquire, align 8, !noalias !190
  %.not.i.i.i = icmp ult i64 %8, 8
  br i1 %.not.i.i.i, label %bb5.i.i.i, label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %bb1.i.i
  %9 = cmpxchg ptr %_20.i.i, i64 %7, i64 %8 release monotonic, align 8, !noalias !193
  br label %bb1.i.i.backedge

bb5.i.i.i:                                        ; preds = %bb1.i.i
  %10 = cmpxchg ptr %_18.i.i.i, i64 0, i64 %new.i.i release monotonic, align 8, !noalias !196
  %_8.sroa.18.0.in.i.i9.i.i = extractvalue { i64, i1 } %10, 1
  br i1 %_8.sroa.18.0.in.i.i9.i.i, label %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit, label %bb1.i.i.backedge

bb1.i.i.backedge:                                 ; preds = %bb5.i.i.i, %bb3.i.i.i
  br label %bb1.i.i

_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit: ; preds = %bb5.i.i.i
  %11 = cmpxchg ptr %_20.i.i, i64 %7, i64 %new.i.i release monotonic, align 8, !noalias !199
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_13.i)
  call void @llvm.lifetime.end.p0(i64 2056, ptr nonnull %bag1.i)
  %_11 = load i64, ptr %0, align 16, !noundef !5
  %_10 = icmp ult i64 %_11, 64
  br i1 %_10, label %bb4, label %bb5

bb4:                                              ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit, %start
  %_11.lcssa = phi i64 [ %_114, %start ], [ %_11, %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit ]
  %12 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_9, i64 %_11.lcssa
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(32) %12, ptr noundef nonnull align 8 dereferenceable(32) %deferred, i64 32, i1 false)
  %13 = load i64, ptr %0, align 16, !noundef !5
  %14 = add i64 %13, 1
  store i64 %14, ptr %0, align 16
  ret void
}

; <crossbeam_epoch::internal::Local>::finalize
; Function Attrs: cold uwtable
define void @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8finalize(ptr noundef nonnull align 128 initializes((2080, 2088)) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_13.i = alloca [2048 x i8], align 8
  %bag1.i = alloca [2056 x i8], align 8
  %_14 = alloca [8 x i8], align 8
  %_9 = alloca [8 x i8], align 8
  %_16 = getelementptr inbounds nuw i8, ptr %self, i64 2080
  store i64 1, ptr %_16, align 32
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_9)
; call <crossbeam_epoch::internal::Local>::pin
  %0 = tail call fastcc noundef ptr @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin(ptr noundef nonnull align 128 %self) #23
  store ptr %self, ptr %_9, align 8
  %_18 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20 = load ptr, ptr %_18, align 8, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !202)
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_13.i)
  call void @llvm.lifetime.start.p0(i64 2056, ptr nonnull %bag1.i)
  br label %repeat_loop_body.i

repeat_loop_body.i:                               ; preds = %repeat_loop_body.i, %start
  %1 = phi i64 [ 0, %start ], [ %3, %repeat_loop_body.i ]
  %2 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_13.i, i64 %1
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %2, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !202
  %3 = add nuw nsw i64 %1, 1
  %exitcond.not.i = icmp eq i64 %3, 64
  br i1 %exitcond.not.i, label %bb6.i, label %repeat_loop_body.i

bb6.i:                                            ; preds = %repeat_loop_body.i
  %_22 = getelementptr inbounds nuw i8, ptr %self, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i, ptr noundef nonnull align 16 dereferenceable(2056) %_22, i64 2056, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2056) %_22, ptr noundef nonnull align 8 dereferenceable(2048) %_13.i, i64 2048, i1 false)
  %_5.sroa.4.0.bag.sroa_idx.i = getelementptr inbounds nuw i8, ptr %self, i64 2064
  store i64 0, ptr %_5.sroa.4.0.bag.sroa_idx.i, align 16, !alias.scope !202
  fence seq_cst
  %_16.i = getelementptr inbounds nuw i8, ptr %_20, i64 384
  %4 = load atomic i64, ptr %_16.i monotonic, align 8, !noalias !202
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !205
; call __rustc::__rust_alloc
  %5 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !205
  %6 = icmp eq ptr %5, null
  br i1 %6, label %bb2.i.i.i, label %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i, !prof !15

bb2.i.i.i:                                        ; preds = %bb6.i
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb2.i.i.i
  unreachable

_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i: ; preds = %bb6.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %5, ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i, i64 2056, i1 false), !noalias !202
  %_10.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %5, i64 2056
  store i64 %4, ptr %_10.sroa.4.0..sroa_idx.i, align 8, !noalias !202
  %_4.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %5, i64 2064
  store i64 0, ptr %_4.sroa.4.0..sroa_idx.i.i, align 8, !noalias !210
  %new.i.i = ptrtoint ptr %5 to i64
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_20, i64 256
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb1.i.i.backedge, %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i
  %7 = load atomic i64, ptr %_20.i.i acquire, align 8, !noalias !210
  %raw.i.i.i = and i64 %7, -8
  %_2.i.i.i.i = inttoptr i64 %raw.i.i.i to ptr
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i, i64 2064
  %8 = load atomic i64, ptr %_18.i.i.i acquire, align 8, !noalias !211
  %.not.i.i.i = icmp ult i64 %8, 8
  br i1 %.not.i.i.i, label %bb5.i.i.i, label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %bb1.i.i
  %9 = cmpxchg ptr %_20.i.i, i64 %7, i64 %8 release monotonic, align 8, !noalias !214
  br label %bb1.i.i.backedge

bb5.i.i.i:                                        ; preds = %bb1.i.i
  %10 = cmpxchg ptr %_18.i.i.i, i64 0, i64 %new.i.i release monotonic, align 8, !noalias !217
  %_8.sroa.18.0.in.i.i9.i.i = extractvalue { i64, i1 } %10, 1
  br i1 %_8.sroa.18.0.in.i.i9.i.i, label %bb2, label %bb1.i.i.backedge

bb1.i.i.backedge:                                 ; preds = %bb5.i.i.i, %bb3.i.i.i
  br label %bb1.i.i

cleanup:                                          ; preds = %bb2.i.i.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke <crossbeam_epoch::guard::Guard as core::ops::drop::Drop>::drop
  invoke fastcc void @_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %_9) #23
          to label %bb5 unwind label %terminate

bb2:                                              ; preds = %bb5.i.i.i
  %12 = cmpxchg ptr %_20.i.i, i64 %7, i64 %new.i.i release monotonic, align 8, !noalias !220
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_13.i)
  call void @llvm.lifetime.end.p0(i64 2056, ptr nonnull %bag1.i)
  %_7.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2072
  %guard_count.i.i = load i64, ptr %_7.i.i, align 8, !noalias !223, !noundef !5
  %_3.i.i = add i64 %guard_count.i.i, -1
  store i64 %_3.i.i, ptr %_7.i.i, align 8, !noalias !223
  %13 = icmp eq i64 %guard_count.i.i, 1
  br i1 %13, label %bb1.i.i5, label %_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit

bb1.i.i5:                                         ; preds = %bb2
  %_12.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2176
  store atomic i64 0, ptr %_12.i.i release, align 128, !noalias !223
  %_14.i.i = getelementptr inbounds nuw i8, ptr %self, i64 2080
  %_4.i.i = load i64, ptr %_14.i.i, align 32, !noalias !223, !noundef !5
  %14 = icmp eq i64 %_4.i.i, 0
  br i1 %14, label %bb2.i.i, label %_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit, !prof !15

bb2.i.i:                                          ; preds = %bb1.i.i5
; call <crossbeam_epoch::internal::Local>::finalize
  tail call void @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8finalize(ptr noundef nonnull align 128 %self), !noalias !223
  br label %_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit

_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit: ; preds = %bb2, %bb1.i.i5, %bb2.i.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_9)
  store i64 0, ptr %_16, align 32
  %collector = load ptr, ptr %_18, align 8, !nonnull !5, !noundef !5
  %15 = atomicrmw or ptr %self, i64 1 release, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_14)
  store ptr %collector, ptr %_14, align 8
  %16 = atomicrmw sub ptr %collector, i64 1 release, align 8, !noalias !226
  %17 = icmp eq i64 %16, 1
  br i1 %17, label %bb2.i.i.i4, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch9collector9CollectorEBK_.exit

bb2.i.i.i4:                                       ; preds = %_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit
  fence acquire
; call <alloc::sync::Arc<crossbeam_epoch::internal::Global>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalE9drop_slowBK_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_14) #24
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch9collector9CollectorEBK_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch9collector9CollectorEBK_.exit: ; preds = %_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop.exit, %bb2.i.i.i4
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_14)
  ret void

terminate:                                        ; preds = %cleanup
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

bb5:                                              ; preds = %cleanup
  resume { ptr, i32 } %11
}

; <alloc::sync::Arc<crossbeam_epoch::internal::Global>>::drop_slow
; Function Attrs: noinline uwtable
define void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalE9drop_slowBK_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %_6 = getelementptr inbounds nuw i8, ptr %_3, i64 128
; invoke core::ptr::drop_in_place::<crossbeam_epoch::internal::Global>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEBK_(ptr noalias noundef align 128 dereferenceable(512) %_6)
          to label %bb1 unwind label %cleanup

cleanup:                                          ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  %_16.i.i = icmp eq ptr %_3, inttoptr (i64 -1 to ptr)
  br i1 %_16.i.i, label %bb4, label %bb8.i.i

bb8.i.i:                                          ; preds = %cleanup
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %1 = atomicrmw sub ptr %_20.i.i, i64 1 release, align 8
  %2 = icmp eq i64 %1, 1
  br i1 %2, label %bb1.i.i, label %bb4

bb1.i.i:                                          ; preds = %bb8.i.i
  fence acquire
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 640, i64 noundef 128) #18
  br label %bb4

bb1:                                              ; preds = %start
  %_16.i.i3 = icmp eq ptr %_3, inttoptr (i64 -1 to ptr)
  br i1 %_16.i.i3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalRNtNtBL_5alloc6GlobalEEB1j_.exit7, label %bb8.i.i4

bb8.i.i4:                                         ; preds = %bb1
  %_20.i.i5 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %3 = atomicrmw sub ptr %_20.i.i5, i64 1 release, align 8
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb1.i.i6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalRNtNtBL_5alloc6GlobalEEB1j_.exit7

bb1.i.i6:                                         ; preds = %bb8.i.i4
  fence acquire
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 640, i64 noundef 128) #18
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalRNtNtBL_5alloc6GlobalEEB1j_.exit7

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalRNtNtBL_5alloc6GlobalEEB1j_.exit7: ; preds = %bb1, %bb8.i.i4, %bb1.i.i6
  ret void

bb4:                                              ; preds = %bb1.i.i, %bb8.i.i, %cleanup
  resume { ptr, i32 } %0
}

; crossbeam_epoch::default::default_collector
; Function Attrs: uwtable
define noundef nonnull align 8 ptr @_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default17default_collector() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load atomic ptr, ptr @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR acquire, align 8
  %_3.i.i = icmp eq ptr %0, null
  br i1 %_3.i.i, label %_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector.exit, label %bb2.i.i, !prof !32

bb2.i.i:                                          ; preds = %start
; call <crossbeam_epoch::sync::once_lock::OnceLock<crossbeam_epoch::collector::Collector>>::initialize::<<crossbeam_epoch::collector::Collector>::new>
  tail call fastcc void @_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB6_8OnceLockNtNtBa_9collector9CollectorE10initializeNvMs1_B1b_B19_3newEBa_()
  br label %_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector.exit

_RNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector.exit: ; preds = %start, %bb2.i.i
  ret ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch7default9collector9COLLECTOR, i64 8)
}

; <crossbeam_epoch::deferred::Deferred>::NO_OP::no_op_call
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @_RNvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB6_8Deferred5NO_OP10no_op_call(ptr readnone captures(none) %_raw) unnamed_addr #6 {
start:
  ret void
}

; <crossbeam_epoch::deferred::Deferred as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXNtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB2_8DeferredNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c97741f92dd5ccff591c27dba386f700, i64 noundef 15)
  ret i1 %_0
}

; <<crossbeam_epoch::guard::Guard>::repin_after::ScopeGuard as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXNvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB5_5Guard11repin_afterNtB2_10ScopeGuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %guard.i = alloca [8 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !noundef !5
  %0 = icmp eq ptr %_3, null
  br i1 %0, label %bb2, label %bb4

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %guard.i)
  store ptr %_3, ptr %guard.i, align 8
  %_15.i = getelementptr inbounds nuw i8, ptr %_3, i64 2072
  %guard_count.i = load i64, ptr %_15.i, align 8, !noundef !5
  %_17.1.i = icmp eq i64 %guard_count.i, -1
  br i1 %_17.1.i, label %bb9.i, label %bb11.i, !prof !15

bb11.i:                                           ; preds = %bb4
  %_17.0.i = add nuw i64 %guard_count.i, 1
  store i64 %_17.0.i, ptr %_15.i, align 8
  %1 = icmp eq i64 %guard_count.i, 0
  br i1 %1, label %bb2.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit

bb9.i:                                            ; preds = %bb4
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ce925522e0221e39b5bde413077875f) #19
          to label %unreachable.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb3.i, %bb9.i
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke <crossbeam_epoch::guard::Guard as core::ops::drop::Drop>::drop
  invoke fastcc void @_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %guard.i) #25
          to label %bb8.i unwind label %terminate.i

bb2.i:                                            ; preds = %bb11.i
  %_24.i = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %_26.i = load ptr, ptr %_24.i, align 8, !nonnull !5, !noundef !5
  %_29.i = getelementptr inbounds nuw i8, ptr %_26.i, i64 384
  %3 = load atomic i64, ptr %_29.i monotonic, align 8
  %new_epoch.i = or i64 %3, 1
  %_33.i = getelementptr inbounds nuw i8, ptr %_3, i64 2176
  store atomic i64 %new_epoch.i, ptr %_33.i monotonic, align 8
  fence seq_cst
  %_35.i = getelementptr inbounds nuw i8, ptr %_3, i64 2088
  %count.i = load i64, ptr %_35.i, align 8, !noundef !5
  %_36.i = add i64 %count.i, 1
  store i64 %_36.i, ptr %_35.i, align 8
  %_10.i = and i64 %count.i, 127
  %4 = icmp eq i64 %_10.i, 0
  br i1 %4, label %bb3.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit, !prof !15

bb3.i:                                            ; preds = %bb2.i
  %_42.i = load ptr, ptr %_24.i, align 8, !nonnull !5, !noundef !5
  %_12.i = getelementptr inbounds nuw i8, ptr %_42.i, i64 128
; invoke <crossbeam_epoch::internal::Global>::collect
  invoke void @_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global7collect(ptr noundef nonnull align 128 %_12.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %guard.i)
          to label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit unwind label %cleanup.i

unreachable.i:                                    ; preds = %bb9.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

bb8.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %2

_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit: ; preds = %bb11.i, %bb2.i, %bb3.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %guard.i)
  %_8 = load i64, ptr %_15.i, align 8, !noundef !5
  %_15 = getelementptr inbounds nuw i8, ptr %_3, i64 2080
  %_9 = load i64, ptr %_15, align 8, !noundef !5
  %_10 = add i64 %_9, -1
  store i64 %_10, ptr %_15, align 8
  %6 = icmp eq i64 %_8, 0
  %7 = icmp eq i64 %_9, 1
  %or.cond = and i1 %6, %7
  br i1 %or.cond, label %bb7, label %bb2, !prof !233

bb2:                                              ; preds = %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit, %bb7, %start
  ret void

bb7:                                              ; preds = %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local3pin.exit
; call <crossbeam_epoch::internal::Local>::finalize
  tail call void @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8finalize(ptr noundef nonnull align 128 %_3)
  br label %bb2
}

; <crossbeam_epoch::guard::Guard as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs0_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB5_5GuardNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5cead5022b6c1daff98c0518c274325d, i64 noundef 12)
  ret i1 %_0
}

; <crossbeam_epoch::collector::Collector as core::default::Default>::default
; Function Attrs: uwtable
define noalias noundef nonnull ptr @_RNvXs0_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9CollectorNtNtCsjMrxcFdYDNN_4core7default7Default7default() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %q.i.i = alloca [256 x i8], align 128
  %_1.i = alloca [8 x i8], align 8
  %_4 = alloca [640 x i8], align 128
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_1.i), !noalias !234
  store i64 0, ptr %_1.i, align 8, !noalias !234
  call void @llvm.lifetime.start.p0(i64 256, ptr nonnull %q.i.i), !noalias !237
  store i64 0, ptr %q.i.i, align 128, !noalias !237
  %0 = getelementptr inbounds nuw i8, ptr %q.i.i, i64 128
  store i64 0, ptr %0, align 128, !noalias !237
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !237
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !237
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i.i.i, label %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new.exit, !prof !15

bb2.i.i.i:                                        ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !237

.noexc.i.i:                                       ; preds = %bb2.i.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb2.i.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<crossbeam_epoch::sync::queue::Queue<crossbeam_epoch::internal::SealedBag>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue5QueueNtNtBN_8internal9SealedBagEEBN_(ptr noalias noundef align 128 dereferenceable(256) %q.i.i) #21
          to label %cleanup.body.i unwind label %terminate.i.i, !noalias !237

terminate.i.i:                                    ; preds = %cleanup.i.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22, !noalias !237
  unreachable

cleanup.body.i:                                   ; preds = %cleanup.i.i
; invoke core::ptr::drop_in_place::<crossbeam_epoch::sync::list::List<crossbeam_epoch::internal::Local>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4list4ListNtNtBN_8internal5LocalEEBN_(ptr noalias noundef align 8 dereferenceable(8) %_1.i) #21
          to label %common.resume unwind label %terminate.i2, !noalias !234

terminate.i2:                                     ; preds = %cleanup.body.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22, !noalias !234
  unreachable

common.resume:                                    ; preds = %cleanup.i, %cleanup.body.i
  %common.resume.op = phi { ptr, i32 } [ %3, %cleanup.body.i ], [ %9, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op

_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new.exit: ; preds = %start
  %_5.sroa.3.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %1, i64 2064
  store i64 0, ptr %_5.sroa.3.0..sroa_idx.i.i, align 8, !noalias !237
  %_9.i.i = ptrtoint ptr %1 to i64
  store atomic i64 %_9.i.i, ptr %q.i.i monotonic, align 128, !noalias !237
  store atomic i64 %_9.i.i, ptr %0 monotonic, align 128, !noalias !237
  %6 = getelementptr inbounds nuw i8, ptr %_4, i64 128
  call void @llvm.lifetime.start.p0(i64 640, ptr nonnull %_4)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 128 dereferenceable(256) %6, ptr noundef nonnull align 128 dereferenceable(256) %q.i.i, i64 256, i1 false)
  call void @llvm.lifetime.end.p0(i64 256, ptr nonnull %q.i.i), !noalias !237
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_1.i), !noalias !234
  store <2 x i64> splat (i64 1), ptr %_4, align 128
  %_2.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_4, i64 384
  store i64 0, ptr %_2.sroa.4.0..sroa_idx, align 128
  %_2.sroa.56.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_4, i64 512
  store i64 0, ptr %_2.sroa.56.0..sroa_idx, align 128
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !240
; call __rustc::__rust_alloc
  %7 = tail call noundef align 128 dereferenceable_or_null(640) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 640, i64 noundef range(i64 8, 129) 128) #18, !noalias !240
  %8 = icmp eq ptr %7, null
  br i1 %8, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEE3newB14_.exit, !prof !15

bb2.i:                                            ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new.exit
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 128, i64 noundef 640) #19
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<crossbeam_epoch::internal::Global>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEBK_(ptr noalias noundef align 128 dereferenceable(512) %6)
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #22
  unreachable

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEE3newB14_.exit: ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new.exit
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 128 dereferenceable(640) %7, ptr noundef nonnull align 128 dereferenceable(640) %_4, i64 640, i1 false)
  call void @llvm.lifetime.end.p0(i64 640, ptr nonnull %_4)
  ret ptr %7
}

; <crossbeam_epoch::internal::Bag as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 captures(address) dereferenceable(2056) %self) unnamed_addr #0 {
start:
  %_9 = alloca [32 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %_5 = load i64, ptr %0, align 8, !noundef !5
  %_11 = icmp ult i64 %_5, 65
  br i1 %_11, label %bb2, label %bb3, !prof !68

bb3:                                              ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20
  unreachable

bb2:                                              ; preds = %start
  %_18.idx = shl nuw nsw i64 %_5, 5
  %_18 = getelementptr inbounds nuw i8, ptr %self, i64 %_18.idx
  %_241 = icmp eq i64 %_5, 0
  br i1 %_241, label %bb6, label %bb7.lr.ph

bb7.lr.ph:                                        ; preds = %bb2
  %_35 = getelementptr inbounds nuw i8, ptr %_9, i64 8
  br label %bb7

bb7:                                              ; preds = %bb7.lr.ph, %bb7
  %iter.sroa.0.02 = phi ptr [ %self, %bb7.lr.ph ], [ %_30, %bb7 ]
  %_30 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02, i64 32
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %_32 = load ptr, ptr %_9, align 8, !nonnull !5, !noundef !5
  call void %_32(ptr noundef nonnull %_35)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9)
  %_24 = icmp eq ptr %_30, %_18
  br i1 %_24, label %bb6, label %bb7

bb6:                                              ; preds = %bb7, %bb2
  ret void
}

; <&crossbeam_epoch::deferred::Deferred as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB6_5Debug3fmtBA_(ptr noalias readonly align 8 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::pad
  %_0.i = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c97741f92dd5ccff591c27dba386f700, i64 noundef 15)
  ret i1 %_0.i
}

; <&[crossbeam_epoch::deferred::Deferred] as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB6_5Debug3fmtBB_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %entry.i.i = alloca [8 x i8], align 8
  %_5.i = alloca [16 x i8], align 8
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !5
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5.i), !noalias !243
; call <core::fmt::Formatter>::debug_list
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter10debug_list(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f), !noalias !247
  %_10.idx.i = shl nuw nsw i64 %_3.1, 5
  %_10.i = getelementptr inbounds nuw i8, ptr %_3.0, i64 %_10.idx.i
  %_6.i7.i.i = icmp eq i64 %_3.1, 0
  br i1 %_6.i7.i.i, label %_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_.exit, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %bb5.i.i
  %iter.sroa.0.08.i.i = phi ptr [ %_16.i.i.i, %bb5.i.i ], [ %_3.0, %start ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.08.i.i, i64 32
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %entry.i.i), !noalias !248
  store ptr %iter.sroa.0.08.i.i, ptr %entry.i.i, align 8, !noalias !248
; call <core::fmt::builders::DebugList>::entry
  %_9.i.i = call noundef nonnull align 8 ptr @_RNvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_9DebugList5entry(ptr noalias noundef nonnull align 8 dereferenceable(16) %_5.i, ptr noundef nonnull align 1 %entry.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %entry.i.i), !noalias !248
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_10.i
  br i1 %_6.i.i.i, label %_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_.exit, label %bb5.i.i

_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_.exit: ; preds = %bb5.i.i, %start
; call <core::fmt::builders::DebugList>::finish
  %_0.i = call noundef zeroext i1 @_RNvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_9DebugList6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_5.i)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5.i), !noalias !243
  ret i1 %_0.i
}

; <crossbeam_epoch::internal::Bag as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(2056) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_8 = alloca [16 x i8], align 8
  %_5 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_5, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e22a7101984065c8df67d185dd44ede6, i64 noundef 3)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_8)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 2048
  %_11 = load i64, ptr %0, align 8, !noundef !5
  %_13 = icmp ult i64 %_11, 65
  br i1 %_13, label %bb4, label %bb5, !prof !68

bb5:                                              ; preds = %start
; call core::slice::index::slice_index_fail
  call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_11, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bd8f6f3aa8c59c7dea37b6db299761bd) #20
  unreachable

bb4:                                              ; preds = %start
  store ptr %self, ptr %_8, align 8
  %1 = getelementptr inbounds nuw i8, ptr %_8, i64 8
  store i64 %_11, ptr %1, align 8
; call <core::fmt::builders::DebugStruct>::field
  %_3 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_31fb308a800d703546e3033c8c1834ad, i64 noundef 9, ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2)
; call <core::fmt::builders::DebugStruct>::finish
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_3)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_8)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  ret i1 %_0
}

; <crossbeam_epoch::collector::Collector as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs3_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_9CollectorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b7376c537a345be44440e786f26d0dce, i64 noundef 16)
  ret i1 %_0
}

; <crossbeam_epoch::internal::Local as crossbeam_epoch::sync::list::IsElement<crossbeam_epoch::internal::Local>>::finalize
; Function Attrs: uwtable
define void @_RNvXs7_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5LocalINtNtNtB7_4sync4list9IsElementBL_E8finalize(ptr noundef nonnull align 8 %entry, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %guard) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_13.i.i.i = alloca [2048 x i8], align 8
  %bag1.i.i.i = alloca [2056 x i8], align 8
  %_9.i.i.i.i.i.i.i.i.i.i = alloca [32 x i8], align 8
  %_3.i = alloca [8 x i8], align 8
  %_7 = ptrtoint ptr %entry to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_3.i)
  %0 = and i64 %_7, 120
  store i64 %0, ptr %_3.i, align 8
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %_RINvNtCshWjyrkxAz4b_15crossbeam_epoch6atomic14ensure_alignedNtNtB4_8internal5LocalEB4_.exit, label %bb2.i, !prof !32

bb2.i:                                            ; preds = %start
; call core::panicking::assert_failed::<usize, usize>
  call void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %_3.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @_RNvNvNtCshWjyrkxAz4b_15crossbeam_epoch5guard11unprotected11UNPROTECTED, ptr noundef nonnull @alloc_2d35da0edd4cc6d13fa936157458b298, ptr nonnull inttoptr (i64 35 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_31ec4324eab67a881201d09fca59e0c9) #20
  unreachable

_RINvNtCshWjyrkxAz4b_15crossbeam_epoch6atomic14ensure_alignedNtNtB4_8internal5LocalEB4_.exit: ; preds = %start
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_3.i)
  %guard.val = load ptr, ptr %guard, align 8, !noundef !5
  %2 = icmp eq ptr %guard.val, null
  br i1 %2, label %bb8.i, label %bb9.i

bb8.i:                                            ; preds = %_RINvNtCshWjyrkxAz4b_15crossbeam_epoch6atomic14ensure_alignedNtNtB4_8internal5LocalEB4_.exit
  tail call void @llvm.experimental.noalias.scope.decl(metadata !251)
  %3 = getelementptr inbounds nuw i8, ptr %entry, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !254)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !257)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !260)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !263)
  %4 = getelementptr inbounds nuw i8, ptr %entry, i64 2064
  %_5.i.i.i.i.i.i.i.i.i.i = load i64, ptr %4, align 8, !alias.scope !266, !noundef !5
  %_11.i.i.i.i.i.i.i.i.i.i = icmp ult i64 %_5.i.i.i.i.i.i.i.i.i.i, 65
  br i1 %_11.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i, !prof !68

bb3.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb8.i
; invoke core::slice::index::slice_index_fail
  invoke void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_5.i.i.i.i.i.i.i.i.i.i, i64 noundef 64, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6ded0471392c4b1917b1d9a4afadd291) #20
          to label %.noexc.i.i.i.i.i unwind label %bb1.loopexit.split-lp.i.i.i.i.i

.noexc.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  unreachable

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb8.i
  %_18.idx.i.i.i.i.i.i.i.i.i.i = shl nuw nsw i64 %_5.i.i.i.i.i.i.i.i.i.i, 5
  %_18.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %3, i64 %_18.idx.i.i.i.i.i.i.i.i.i.i
  %_241.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %_241.i.i.i.i.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCshWjyrkxAz4b_15crossbeam_epoch6atomic5OwnedNtNtBL_8internal5LocalEEBL_.exit.i, label %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i

bb7.lr.ph.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
  %_35.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_9.i.i.i.i.i.i.i.i.i.i, i64 8
  br label %bb7.i.i.i.i.i.i.i.i.i.i

bb7.i.i.i.i.i.i.i.i.i.i:                          ; preds = %.noexc2.i.i.i.i.i, %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i = phi ptr [ %3, %bb7.lr.ph.i.i.i.i.i.i.i.i.i.i ], [ %_30.i.i.i.i.i.i.i.i.i.i, %.noexc2.i.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i.i.i.i.i.i.i.i.i), !noalias !266
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false)
  %_32.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %_9.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !266, !nonnull !5, !noundef !5
  invoke void %_32.i.i.i.i.i.i.i.i.i.i(ptr noundef nonnull %_35.i.i.i.i.i.i.i.i.i.i)
          to label %.noexc2.i.i.i.i.i unwind label %bb1.loopexit.i.i.i.i.i

.noexc2.i.i.i.i.i:                                ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.02.i.i.i.i.i.i.i.i.i.i, i64 32
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i.i.i.i.i.i.i.i.i), !noalias !266
  %_24.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %_30.i.i.i.i.i.i.i.i.i.i, %_18.i.i.i.i.i.i.i.i.i.i
  br i1 %_24.i.i.i.i.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCshWjyrkxAz4b_15crossbeam_epoch6atomic5OwnedNtNtBL_8internal5LocalEEBL_.exit.i, label %bb7.i.i.i.i.i.i.i.i.i.i

bb1.loopexit.i.i.i.i.i:                           ; preds = %bb7.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.i.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb1.i.i.i.i.i

bb1.loopexit.split-lp.i.i.i.i.i:                  ; preds = %bb3.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit.split-lp.i.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb1.loopexit.split-lp.i.i.i.i.i, %bb1.loopexit.i.i.i.i.i
  %lpad.phi.i.i.i.i.i = phi { ptr, i32 } [ %lpad.loopexit.i.i.i.i.i, %bb1.loopexit.i.i.i.i.i ], [ %lpad.loopexit.split-lp.i.i.i.i.i, %bb1.loopexit.split-lp.i.i.i.i.i ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %entry, i64 noundef 2304, i64 noundef 128) #18
  resume { ptr, i32 } %lpad.phi.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCshWjyrkxAz4b_15crossbeam_epoch6atomic5OwnedNtNtBL_8internal5LocalEEBL_.exit.i: ; preds = %.noexc2.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %entry, i64 noundef 2304, i64 noundef 128) #18
  br label %_RINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB3_5Guard15defer_uncheckedNCINvB2_13defer_destroyNtNtB5_8internal5LocalE0INtNtB5_6atomic5OwnedB1v_EEB5_.exit

bb9.i:                                            ; preds = %_RINvNtCshWjyrkxAz4b_15crossbeam_epoch6atomic14ensure_alignedNtNtB4_8internal5LocalEB4_.exit
  %_9.i.i = getelementptr inbounds nuw i8, ptr %guard.val, i64 16
  %5 = getelementptr inbounds nuw i8, ptr %guard.val, i64 2064
  %_114.i.i = load i64, ptr %5, align 8, !noalias !267, !noundef !5
  %_105.i.i = icmp ult i64 %_114.i.i, 64
  br i1 %_105.i.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit.i, label %bb5.lr.ph.i.i

bb5.lr.ph.i.i:                                    ; preds = %bb9.i
  %_15.i.i = getelementptr inbounds nuw i8, ptr %guard.val, i64 8
  br label %bb5.i.i

bb5.i.i:                                          ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i.i, %bb5.lr.ph.i.i
  %_17.i.i = load ptr, ptr %_15.i.i, align 8, !noalias !267, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !270)
  call void @llvm.lifetime.start.p0(i64 2048, ptr nonnull %_13.i.i.i), !noalias !267
  call void @llvm.lifetime.start.p0(i64 2056, ptr nonnull %bag1.i.i.i)
  br label %repeat_loop_body.i.i.i

repeat_loop_body.i.i.i:                           ; preds = %repeat_loop_body.i.i.i, %bb5.i.i
  %6 = phi i64 [ 0, %bb5.i.i ], [ %8, %repeat_loop_body.i.i.i ]
  %7 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_13.i.i.i, i64 %6
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %7, ptr noundef nonnull align 8 dereferenceable(32) @anon.e80f155be67e243e8d65baf1d259b60c.0, i64 32, i1 false), !noalias !273
  %8 = add nuw nsw i64 %6, 1
  %exitcond.not.i.i.i = icmp eq i64 %8, 64
  br i1 %exitcond.not.i.i.i, label %bb6.i.i.i, label %repeat_loop_body.i.i.i

bb6.i.i.i:                                        ; preds = %repeat_loop_body.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i.i, ptr noundef nonnull align 16 dereferenceable(2056) %_9.i.i, i64 2056, i1 false), !noalias !267
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 16 dereferenceable(2056) %_9.i.i, ptr noundef nonnull align 8 dereferenceable(2048) %_13.i.i.i, i64 2048, i1 false), !noalias !267
  store i64 0, ptr %5, align 8, !alias.scope !270, !noalias !267
  fence seq_cst
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %_17.i.i, i64 384
  %9 = load atomic i64, ptr %_16.i.i.i monotonic, align 8, !noalias !273
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #18, !noalias !274
; call __rustc::__rust_alloc
  %10 = tail call noundef align 8 dereferenceable_or_null(2072) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 640, 2305) 2072, i64 noundef range(i64 8, 129) 8) #18, !noalias !274
  %11 = icmp eq ptr %10, null
  br i1 %11, label %bb2.i.i.i.i.i, label %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i.i, !prof !15

bb2.i.i.i.i.i:                                    ; preds = %bb6.i.i.i
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 2072) #19, !noalias !273
  unreachable

_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i.i: ; preds = %bb6.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(2056) %10, ptr noundef nonnull align 8 dereferenceable(2056) %bag1.i.i.i, i64 2056, i1 false), !noalias !273
  %_10.sroa.4.0..sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %10, i64 2056
  store i64 %9, ptr %_10.sroa.4.0..sroa_idx.i.i.i, align 8, !noalias !273
  %_4.sroa.4.0..sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %10, i64 2064
  store i64 0, ptr %_4.sroa.4.0..sroa_idx.i.i.i.i, align 8, !noalias !279
  %new.i.i.i.i = ptrtoint ptr %10 to i64
  %_20.i.i.i.i = getelementptr inbounds nuw i8, ptr %_17.i.i, i64 256
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb1.i.i.i.i.backedge, %_RNvNtCsdJPVW0sQgAG_5alloc5alloc15exchange_malloc.exit.i.i.i.i
  %12 = load atomic i64, ptr %_20.i.i.i.i acquire, align 8, !noalias !279
  %raw.i.i.i.i.i = and i64 %12, -8
  %_2.i.i.i.i.i.i = inttoptr i64 %raw.i.i.i.i.i to ptr
  %_18.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i.i.i, i64 2064
  %13 = load atomic i64, ptr %_18.i.i.i.i.i acquire, align 8, !noalias !280
  %.not.i.i.i.i.i = icmp ult i64 %13, 8
  br i1 %.not.i.i.i.i.i, label %bb5.i.i.i.i.i, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i
  %14 = cmpxchg ptr %_20.i.i.i.i, i64 %12, i64 %13 release monotonic, align 8, !noalias !283
  br label %bb1.i.i.i.i.backedge

bb5.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i
  %15 = cmpxchg ptr %_18.i.i.i.i.i, i64 0, i64 %new.i.i.i.i release monotonic, align 8, !noalias !286
  %_8.sroa.18.0.in.i.i9.i.i.i.i = extractvalue { i64, i1 } %15, 1
  br i1 %_8.sroa.18.0.in.i.i9.i.i.i.i, label %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i.i, label %bb1.i.i.i.i.backedge

bb1.i.i.i.i.backedge:                             ; preds = %bb5.i.i.i.i.i, %bb3.i.i.i.i.i
  br label %bb1.i.i.i.i

_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i.i: ; preds = %bb5.i.i.i.i.i
  %16 = cmpxchg ptr %_20.i.i.i.i, i64 %12, i64 %new.i.i.i.i release monotonic, align 8, !noalias !289
  call void @llvm.lifetime.end.p0(i64 2048, ptr nonnull %_13.i.i.i), !noalias !267
  call void @llvm.lifetime.end.p0(i64 2056, ptr nonnull %bag1.i.i.i)
  %_11.i.i = load i64, ptr %5, align 8, !noalias !267, !noundef !5
  %_10.i.i = icmp ult i64 %_11.i.i, 64
  br i1 %_10.i.i, label %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit.i, label %bb5.i.i

_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit.i: ; preds = %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i.i, %bb9.i
  %_11.lcssa.i.i = phi i64 [ %_114.i.i, %bb9.i ], [ %_11.i.i, %_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag.exit.i.i ]
  %17 = getelementptr inbounds nuw %"deferred::Deferred", ptr %_9.i.i, i64 %_11.lcssa.i.i
  store ptr @_RINvNvMs_NtCshWjyrkxAz4b_15crossbeam_epoch8deferredNtB7_8Deferred3new4callNCINvMNtB9_5guardNtB1g_5Guard15defer_uncheckedNCINvB1f_13defer_destroyNtNtB9_8internal5LocalE0INtNtB9_6atomic5OwnedB2i_EE0EB9_, ptr %17, align 16
  %_7.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %17, i64 8
  store i64 %_7, ptr %_7.sroa.4.0..sroa_idx.i, align 8
  %18 = load i64, ptr %5, align 8, !noalias !267, !noundef !5
  %19 = add i64 %18, 1
  store i64 %19, ptr %5, align 8, !noalias !267
  br label %_RINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB3_5Guard15defer_uncheckedNCINvB2_13defer_destroyNtNtB5_8internal5LocalE0INtNtB5_6atomic5OwnedB1v_EEB5_.exit

_RINvMNtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB3_5Guard15defer_uncheckedNCINvB2_13defer_destroyNtNtB5_8internal5LocalE0INtNtB5_6atomic5OwnedB1v_EEB5_.exit: ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCshWjyrkxAz4b_15crossbeam_epoch6atomic5OwnedNtNtBL_8internal5LocalEEBL_.exit.i, %_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer.exit.i
  ret void
}

; <crossbeam_epoch::collector::LocalHandle as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8_NtCshWjyrkxAz4b_15crossbeam_epoch9collectorNtB5_11LocalHandleNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_39ad2ebca0184067d2550a7ac4b13b7f, i64 noundef 18)
  ret i1 %_0
}

; <crossbeam_epoch::guard::Guard as core::ops::drop::Drop>::drop
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #3 {
start:
  %_3 = load ptr, ptr %self, align 8, !noundef !5
  %0 = icmp eq ptr %_3, null
  br i1 %0, label %bb1, label %bb3

bb3:                                              ; preds = %start
  %_7.i = getelementptr inbounds nuw i8, ptr %_3, i64 2072
  %guard_count.i = load i64, ptr %_7.i, align 8, !noundef !5
  %_3.i = add i64 %guard_count.i, -1
  store i64 %_3.i, ptr %_7.i, align 8
  %1 = icmp eq i64 %guard_count.i, 1
  br i1 %1, label %bb1.i, label %bb1

bb1.i:                                            ; preds = %bb3
  %_12.i = getelementptr inbounds nuw i8, ptr %_3, i64 2176
  store atomic i64 0, ptr %_12.i release, align 8
  %_14.i = getelementptr inbounds nuw i8, ptr %_3, i64 2080
  %_4.i = load i64, ptr %_14.i, align 8, !noundef !5
  %2 = icmp eq i64 %_4.i, 0
  br i1 %2, label %bb2.i, label %bb1, !prof !15

bb2.i:                                            ; preds = %bb1.i
; call <crossbeam_epoch::internal::Local>::finalize
  tail call void @_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local8finalize(ptr noundef nonnull align 128 %_3)
  br label %bb1

bb1:                                              ; preds = %bb2.i, %bb1.i, %bb3, %start
  ret void
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #7

; <std::sys::sync::once::queue::Once>::call
; Function Attrs: cold uwtable
declare void @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync4once5queueNtB2_4Once4call(ptr noundef nonnull align 8, i1 noundef zeroext, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(40), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #8

; <core::fmt::builders::DebugList>::entry
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_9DebugList5entry(ptr noalias noundef align 8 dereferenceable(16), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; core::panicking::assert_failed::<usize, usize>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef range(i8 0, 3), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noundef, ptr, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #10

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #11

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #2

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #13

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #14

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #15

; <core::fmt::Formatter>::pad
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #12

; <core::fmt::Formatter>::debug_struct
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::fmt::builders::DebugStruct>::field
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef align 8 dereferenceable(16), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #0

; <core::fmt::builders::DebugStruct>::finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #0

; <core::fmt::Formatter>::debug_list
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter10debug_list(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <core::fmt::builders::DebugList>::finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_9DebugList6finish(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #16

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #17

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nofree norecurse nounwind willreturn memory(readwrite, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #9 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #15 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #17 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #18 = { nounwind }
attributes #19 = { noreturn }
attributes #20 = { noinline noreturn }
attributes #21 = { cold }
attributes #22 = { cold noreturn nounwind }
attributes #23 = { inlinehint }
attributes #24 = { noinline }
attributes #25 = { inlinehint "function-inline-cost-multiplier"="2" }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer: %deferred"}
!4 = distinct !{!4, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer"}
!5 = !{}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag: %bag"}
!8 = distinct !{!8, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag"}
!9 = !{!7, !3}
!10 = !{!11, !13, !7, !3}
!11 = distinct !{!11, !12, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_: %x"}
!12 = distinct !{!12, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_"}
!13 = distinct !{!13, !14, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_: %t"}
!14 = distinct !{!14, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_"}
!15 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!16 = !{!13, !7, !3}
!17 = !{!18, !13, !7, !3}
!18 = distinct !{!18, !19, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_: %guard"}
!19 = distinct !{!19, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_"}
!20 = !{!21, !13, !7, !3}
!21 = distinct !{!21, !22, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!22 = distinct !{!22, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!23 = !{!24, !13, !7, !3}
!24 = distinct !{!24, !25, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!25 = distinct !{!25, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!26 = !{!27, !13, !7, !3}
!27 = distinct !{!27, !28, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!28 = distinct !{!28, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB15_8OnceLockNtNtB19_9collector9CollectorE10initializeNvMs1_B2b_B29_3newE0EB19_: %f"}
!31 = distinct !{!31, !"_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB15_8OnceLockNtNtB19_9collector9CollectorE10initializeNvMs1_B2b_B29_3newE0EB19_"}
!32 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_: %self"}
!35 = distinct !{!35, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_"}
!36 = !{!37}
!37 = distinct !{!37, !38, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_: %self"}
!38 = distinct !{!38, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_"}
!39 = !{!40}
!40 = distinct !{!40, !41, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE12pop_internalB9_: %_0"}
!41 = distinct !{!41, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE12pop_internalB9_"}
!42 = !{!40, !37}
!43 = !{!44}
!44 = distinct !{!44, !45, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!45 = distinct !{!45, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!46 = !{!47, !49, !40}
!47 = distinct !{!47, !48, !"_RNCNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB7_5QueueNtNtBb_8internal9SealedBagE12pop_internal0Bb_: %_0"}
!48 = distinct !{!48, !"_RNCNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB7_5QueueNtNtBb_8internal9SealedBagE12pop_internal0Bb_"}
!49 = distinct !{!49, !48, !"_RNCNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB7_5QueueNtNtBb_8internal9SealedBagE12pop_internal0Bb_: %_1"}
!50 = !{!51, !47, !49, !40}
!51 = distinct !{!51, !52, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!52 = distinct !{!52, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!53 = !{!47, !49, !40, !37}
!54 = !{!49, !40, !37}
!55 = !{!56}
!56 = distinct !{!56, !57, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_: %_1"}
!57 = distinct !{!57, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEEB16_"}
!58 = !{!59}
!59 = distinct !{!59, !60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_: %_1"}
!60 = distinct !{!60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_"}
!61 = !{!62}
!62 = distinct !{!62, !63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_: %_1"}
!63 = distinct !{!63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_"}
!64 = !{!65}
!65 = distinct !{!65, !66, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!66 = distinct !{!66, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!67 = !{!65, !62, !59, !56}
!68 = !{!"branch_weights", i32 4000000, i32 4001}
!69 = !{!65, !62, !59, !56, !37}
!70 = !{!71}
!71 = distinct !{!71, !72, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_: %_1"}
!72 = distinct !{!72, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_"}
!73 = !{!74}
!74 = distinct !{!74, !75, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_: %_1"}
!75 = distinct !{!75, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_"}
!76 = !{!77}
!77 = distinct !{!77, !78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_: %_1"}
!78 = distinct !{!78, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_"}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!81 = distinct !{!81, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!82 = !{!80, !77, !74, !71}
!83 = !{!84}
!84 = distinct !{!84, !85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4list4ListNtNtBN_8internal5LocalEEBN_: %_1"}
!85 = distinct !{!85, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4list4ListNtNtBN_8internal5LocalEEBN_"}
!86 = !{!87}
!87 = distinct !{!87, !88, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_: %self"}
!88 = distinct !{!88, !"_RNvXs1_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4ListNtNtB9_8internal5LocalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB9_"}
!89 = !{!87, !84}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_: %_1"}
!92 = distinct !{!92, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_"}
!93 = !{!94}
!94 = distinct !{!94, !95, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_: %_1"}
!95 = distinct !{!95, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_"}
!96 = !{!97}
!97 = distinct !{!97, !98, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_: %_1"}
!98 = distinct !{!98, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_"}
!99 = !{!100}
!100 = distinct !{!100, !101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_: %_1"}
!101 = distinct !{!101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_"}
!102 = !{!103}
!103 = distinct !{!103, !104, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!104 = distinct !{!104, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!105 = !{!103, !100, !97, !94, !91}
!106 = !{i64 8}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1a_8OnceLockNtNtB1e_9collector9CollectorE10initializeNvMs1_B2g_B2e_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_: argument 0"}
!109 = distinct !{!109, !"_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB1a_8OnceLockNtNtB1e_9collector9CollectorE10initializeNvMs1_B2g_B2e_3newE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_"}
!110 = !{!111}
!111 = distinct !{!111, !112, !"_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB17_8OnceLockNtNtB1b_9collector9CollectorE10initializeNvMs1_B2d_B2b_3newE0E0B1b_: %_1"}
!112 = distinct !{!112, !"_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync9once_lockINtB17_8OnceLockNtNtB1b_9collector9CollectorE10initializeNvMs1_B2d_B2b_3newE0E0B1b_"}
!113 = !{!111, !108}
!114 = !{!115}
!115 = distinct !{!115, !116, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush: %guard"}
!116 = distinct !{!116, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5flush"}
!117 = !{!118}
!118 = distinct !{!118, !119, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag: %bag"}
!119 = distinct !{!119, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag"}
!120 = !{!118, !115}
!121 = !{!122, !124, !118, !115}
!122 = distinct !{!122, !123, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_: %x"}
!123 = distinct !{!123, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_"}
!124 = distinct !{!124, !125, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_: %t"}
!125 = distinct !{!125, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_"}
!126 = !{!124, !118, !115}
!127 = !{!128, !124, !118, !115}
!128 = distinct !{!128, !129, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_: %guard"}
!129 = distinct !{!129, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_"}
!130 = !{!131, !124, !118, !115}
!131 = distinct !{!131, !132, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!132 = distinct !{!132, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!133 = !{!134, !124, !118, !115}
!134 = distinct !{!134, !135, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!135 = distinct !{!135, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!136 = !{!137, !124, !118, !115}
!137 = distinct !{!137, !138, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!138 = distinct !{!138, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!139 = !{!140}
!140 = distinct !{!140, !141, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_: %x"}
!141 = distinct !{!141, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalE3newBI_"}
!142 = !{!143}
!143 = distinct !{!143, !144, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicNtNtNtB8_4sync4list5EntryE21compare_exchange_weakINtB6_6SharedBX_EEB8_: %_0"}
!144 = distinct !{!144, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicNtNtNtB8_4sync4list5EntryE21compare_exchange_weakINtB6_6SharedBX_EEB8_"}
!145 = !{!146}
!146 = distinct !{!146, !147, !"_RNvXs2_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4IterNtNtB9_8internal5LocalBZ_ENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextB9_: %self"}
!147 = distinct !{!147, !"_RNvXs2_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync4listINtB5_4IterNtNtB9_8internal5LocalBZ_ENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextB9_"}
!148 = !{!149, !146}
!149 = distinct !{!149, !150, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicNtNtNtB8_4sync4list5EntryE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!150 = distinct !{!150, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicNtNtNtB8_4sync4list5EntryE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!151 = !{!152, !154, !155, !157, !158}
!152 = distinct !{!152, !153, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B14_NtB14_6Global7collect0EBa_: %_0"}
!153 = distinct !{!153, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B14_NtB14_6Global7collect0EBa_"}
!154 = distinct !{!154, !153, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B14_NtB14_6Global7collect0EBa_: %guard"}
!155 = distinct !{!155, !156, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_: %head"}
!156 = distinct !{!156, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_"}
!157 = distinct !{!157, !156, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_: argument 1"}
!158 = distinct !{!158, !156, !"_RINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB6_5QueueNtNtBa_8internal9SealedBagE10try_pop_ifRNCNvMs5_B14_NtB14_6Global7collect0EBa_: %guard"}
!159 = !{!155, !157, !158}
!160 = !{!161, !155, !157, !158}
!161 = distinct !{!161, !162, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!162 = distinct !{!162, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!163 = !{!164, !166, !152, !155, !157, !158}
!164 = distinct !{!164, !165, !"_RNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB8_5QueueNtNtBc_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B16_NtB16_6Global7collect0E0Bc_: %_0"}
!165 = distinct !{!165, !"_RNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB8_5QueueNtNtBc_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B16_NtB16_6Global7collect0E0Bc_"}
!166 = distinct !{!166, !165, !"_RNCINvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB8_5QueueNtNtBc_8internal9SealedBagE15pop_if_internalRRNCNvMs5_B16_NtB16_6Global7collect0E0Bc_: %_1"}
!167 = !{!168, !164, !166, !152, !155, !157, !158}
!168 = distinct !{!168, !169, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!169 = distinct !{!169, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!170 = !{!166, !152, !155, !157, !158}
!171 = !{!172}
!172 = distinct !{!172, !173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_: %_1"}
!173 = distinct !{!173, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal9SealedBagEBK_"}
!174 = !{!175}
!175 = distinct !{!175, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_: %_1"}
!176 = distinct !{!176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_"}
!177 = !{!178}
!178 = distinct !{!178, !179, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!179 = distinct !{!179, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!180 = !{!178, !175, !172}
!181 = !{!182}
!182 = distinct !{!182, !183, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag: %bag"}
!183 = distinct !{!183, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag"}
!184 = !{!185, !187, !182}
!185 = distinct !{!185, !186, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_: %x"}
!186 = distinct !{!186, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_"}
!187 = distinct !{!187, !188, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_: %t"}
!188 = distinct !{!188, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_"}
!189 = !{!187, !182}
!190 = !{!191, !187, !182}
!191 = distinct !{!191, !192, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_: %guard"}
!192 = distinct !{!192, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_"}
!193 = !{!194, !187, !182}
!194 = distinct !{!194, !195, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!195 = distinct !{!195, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!196 = !{!197, !187, !182}
!197 = distinct !{!197, !198, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!198 = distinct !{!198, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!199 = !{!200, !187, !182}
!200 = distinct !{!200, !201, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!201 = distinct !{!201, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!202 = !{!203}
!203 = distinct !{!203, !204, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag: %bag"}
!204 = distinct !{!204, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag"}
!205 = !{!206, !208, !203}
!206 = distinct !{!206, !207, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_: %x"}
!207 = distinct !{!207, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_"}
!208 = distinct !{!208, !209, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_: %t"}
!209 = distinct !{!209, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_"}
!210 = !{!208, !203}
!211 = !{!212, !208, !203}
!212 = distinct !{!212, !213, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_: %guard"}
!213 = distinct !{!213, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_"}
!214 = !{!215, !208, !203}
!215 = distinct !{!215, !216, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!216 = distinct !{!216, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!217 = !{!218, !208, !203}
!218 = distinct !{!218, !219, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!219 = distinct !{!219, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!220 = !{!221, !208, !203}
!221 = distinct !{!221, !222, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!222 = distinct !{!222, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!223 = !{!224}
!224 = distinct !{!224, !225, !"_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!225 = distinct !{!225, !"_RNvXs_NtCshWjyrkxAz4b_15crossbeam_epoch5guardNtB4_5GuardNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!226 = !{!227, !229, !231}
!227 = distinct !{!227, !228, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_: %self"}
!228 = distinct !{!228, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBK_"}
!229 = distinct !{!229, !230, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEEB1i_: %_1"}
!230 = distinct !{!230, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEEB1i_"}
!231 = distinct !{!231, !232, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch9collector9CollectorEBK_: %_1"}
!232 = distinct !{!232, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch9collector9CollectorEBK_"}
!233 = !{!"branch_weights", i32 1, i32 4001}
!234 = !{!235}
!235 = distinct !{!235, !236, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new: %_0"}
!236 = distinct !{!236, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global3new"}
!237 = !{!238, !235}
!238 = distinct !{!238, !239, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE3newB9_: %_0"}
!239 = distinct !{!239, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE3newB9_"}
!240 = !{!241}
!241 = distinct !{!241, !242, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEE3newB14_: %x"}
!242 = distinct !{!242, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal6GlobalEE3newB14_"}
!243 = !{!244, !246}
!244 = distinct !{!244, !245, !"_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_: %self.0"}
!245 = distinct !{!245, !"_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_"}
!246 = distinct !{!246, !245, !"_RNvXsr_NtCsjMrxcFdYDNN_4core3fmtSNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredNtB5_5Debug3fmtBz_: %f"}
!247 = !{!244}
!248 = !{!249, !244, !246}
!249 = distinct !{!249, !250, !"_RINvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredINtNtNtBa_5slice4iter4IterB14_EEB18_: %self"}
!250 = distinct !{!250, !"_RINvMs5_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB6_9DebugList7entriesRNtNtCshWjyrkxAz4b_15crossbeam_epoch8deferred8DeferredINtNtNtBa_5slice4iter4IterB14_EEB18_"}
!251 = !{!252}
!252 = distinct !{!252, !253, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_: %_1"}
!253 = distinct !{!253, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal5LocalEBK_"}
!254 = !{!255}
!255 = distinct !{!255, !256, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_: %_1"}
!256 = distinct !{!256, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCshWjyrkxAz4b_15crossbeam_epoch9primitive4cell10UnsafeCellNtNtBN_8internal3BagEEBN_"}
!257 = !{!258}
!258 = distinct !{!258, !259, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_: %_1"}
!259 = distinct !{!259, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_4cell10UnsafeCellNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEEB19_"}
!260 = !{!261}
!261 = distinct !{!261, !262, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_: %_1"}
!262 = distinct !{!262, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCshWjyrkxAz4b_15crossbeam_epoch8internal3BagEBK_"}
!263 = !{!264}
!264 = distinct !{!264, !265, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop: %self"}
!265 = distinct !{!265, !"_RNvXs1_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_3BagNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop"}
!266 = !{!264, !261, !258, !255, !252}
!267 = !{!268}
!268 = distinct !{!268, !269, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer: %deferred"}
!269 = distinct !{!269, !"_RNvMs6_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_5Local5defer"}
!270 = !{!271}
!271 = distinct !{!271, !272, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag: %bag"}
!272 = distinct !{!272, !"_RNvMs5_NtCshWjyrkxAz4b_15crossbeam_epoch8internalNtB5_6Global8push_bag"}
!273 = !{!271, !268}
!274 = !{!275, !277, !271, !268}
!275 = distinct !{!275, !276, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_: %x"}
!276 = distinct !{!276, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queue4NodeNtNtBL_8internal9SealedBagEE3newBL_"}
!277 = distinct !{!277, !278, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_: %t"}
!278 = distinct !{!278, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE4pushB9_"}
!279 = !{!277, !271, !268}
!280 = !{!281, !277, !271, !268}
!281 = distinct !{!281, !282, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_: %guard"}
!282 = distinct !{!282, !"_RNvMs0_NtNtCshWjyrkxAz4b_15crossbeam_epoch4sync5queueINtB5_5QueueNtNtB9_8internal9SealedBagE13push_internalB9_"}
!283 = !{!284, !277, !271, !268}
!284 = distinct !{!284, !285, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!285 = distinct !{!285, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!286 = !{!287, !277, !271, !268}
!287 = distinct !{!287, !288, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!288 = distinct !{!288, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
!289 = !{!290, !277, !271, !268}
!290 = distinct !{!290, !291, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_: %_0"}
!291 = distinct !{!291, !"_RINvMs7_NtCshWjyrkxAz4b_15crossbeam_epoch6atomicINtB6_6AtomicINtNtNtB8_4sync5queue4NodeNtNtB8_8internal9SealedBagEE16compare_exchangeINtB6_6SharedBX_EEB8_"}
