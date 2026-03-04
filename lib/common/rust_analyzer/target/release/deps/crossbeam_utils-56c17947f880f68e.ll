; ModuleID = 'crossbeam_utils.60de7661f99b2dbd-cgu.0'
source_filename = "crossbeam_utils.60de7661f99b2dbd-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<(u64, u64)>, !>" = type { %"core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>>", i8, [7 x i8] }
%"core::cell::UnsafeCell<core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>>" = type { %"core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>" }
%"core::mem::maybe_uninit::MaybeUninit<core::cell::Cell<(u64, u64)>>" = type { [2 x i64] }
%"core::sync::atomic::AtomicUsize" = type { i64 }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBd_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1c_8OnceLockINtNtNtBf_6poison5mutex5MutexNtNtB1e_12sharded_lock13ThreadIndicesEE10initializeNvNvB2L_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBd_9OnceStateEE9call_once6vtableB1g_, ptr @_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB17_8OnceLockINtNtNtBa_6poison5mutex5MutexNtNtB19_12sharded_lock13ThreadIndicesEE10initializeNvNvB2G_14thread_indices4initE0E0B1b_ }>, align 8
@alloc_6e953d184259704d269560a14bef7a12 = private unnamed_addr constant [117 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-utils-0.8.21/src/sync/once_lock.rs\00", align 1
@alloc_b0c1b9f0002e7b376ee1d8bf4e97d813 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_6e953d184259704d269560a14bef7a12, [16 x i8] c"t\00\00\00\00\00\00\00B\00\00\00\13\00\00\00" }>, align 8
@alloc_8abc8245dd622154cb39537f390f45d3 = private unnamed_addr constant [117 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sync/once.rs\00", align 1
@alloc_3129176114b12752e73d3989241bb5f0 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_8abc8245dd622154cb39537f390f45d3, [16 x i8] c"t\00\00\00\00\00\00\00\9F\00\00\002\00\00\00" }>, align 8
@_RNvNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBa_11RandomState3new4KEYS0s_023___RUST_STD_INTERNAL_VAL = external thread_local local_unnamed_addr global %"std::sys::thread_local::native::lazy::Storage<core::cell::Cell<(u64, u64)>, !>"
@anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.0 = private unnamed_addr constant [64 x i8] c"\A7\AB\AA2\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", align 8
@anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.1 = private unnamed_addr constant [48 x i8] c"\BB\B1\B0<\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", align 8
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1r_ }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1r_ }>, align 8
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1r_ }>, align 8
@vtable.5 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorTINtNtBJ_5mutex10MutexGuarduENtBL_17WaitTimeoutResultEEECs8jD91Rl7RDZ_15crossbeam_utils, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1r_ }>, align 8
@_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT = external local_unnamed_addr global %"core::sync::atomic::AtomicUsize"
@alloc_da75be5cf355a0c62ad51607063f583d = private unnamed_addr constant [54 x i8] c"attempted to use a condition variable with two mutexes", align 1
@alloc_a0f2a2e696707d28c1bad0bb984af047 = private unnamed_addr constant [132 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/sync/condvar/pthread.rs\00", align 1
@alloc_33c80814996188919076c1dd2d93215e = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_a0f2a2e696707d28c1bad0bb984af047, [16 x i8] c"\83\00\00\00\00\00\00\00'\00\00\00\12\00\00\00" }>, align 8
@alloc_c9bf24a4a19fbea0772327a81b845621 = private unnamed_addr constant [114 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-utils-0.8.21/src/sync/parker.rs\00", align 1
@alloc_5b7852eb24eb1fd922b857fc00d93d48 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00S\01\00\00&\00\00\00" }>, align 8
@alloc_5dd4f91c097d8856a8d5b8665ac69700 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00h\01\00\00+\00\00\00" }>, align 8
@alloc_4995b9999b13a44a131c11df8c199b04 = private unnamed_addr constant [36 x i8] c"!inconsistent park_timeout state: \C0\00", align 1
@alloc_6f84445f1996a99b81bfc2712928887a = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00t\01\00\00\22\00\00\00" }>, align 8
@alloc_dc35fa7e21dfb2345fd82d2a720ab282 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00o\01\00\00C\00\00\00" }>, align 8
@alloc_f85406d658e94b9c37cd2112f10307ee = private unnamed_addr constant [8 x i8] c"\02\00\00\00\00\00\00\00", align 8
@alloc_4956e422da54fdcef75db47b62671705 = private unnamed_addr constant [31 x i8] c"park state changed unexpectedly", align 1
@alloc_1ce448046d8ffdf2bb268a748fb5a871 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00_\01\00\00\11\00\00\00" }>, align 8
@alloc_2dc5e81b43b3aed87b3d95c30c679ec4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00b\01\00\00\17\00\00\00" }>, align 8
@alloc_80b7b76bc8174414d036e884de381c56 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00\9C\01\00\00\1F\00\00\00" }>, align 8
@alloc_4abcb6df1c22df8ca969c35c4c0d7ea2 = private unnamed_addr constant [28 x i8] c"inconsistent state in unpark", align 1
@alloc_b5f763a94cb0d078baa2ec601f473d62 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_c9bf24a4a19fbea0772327a81b845621, [16 x i8] c"q\00\00\00\00\00\00\00\91\01\00\00\12\00\00\00" }>, align 8
@alloc_7e43212db14c6a005fdaae70f5f46558 = private unnamed_addr constant [118 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-utils-0.8.21/src/sync/wait_group.rs\00", align 1
@alloc_9067e1d5a668f509be145c44a0a64f14 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00h\00\00\00%\00\00\00" }>, align 8
@alloc_af70a3b648b95ea60237fd7c7e73ca28 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00o\00\00\00,\00\00\00" }>, align 8
@alloc_f6524b7a65f7681dc4cb2656dc88f418 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00q\00\00\00,\00\00\00" }>, align 8
@_RNvNCNKNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock12REGISTRATION0023___RUST_STD_INTERNAL_VAL = thread_local local_unnamed_addr global <{ [16 x i8], [1 x i8], [7 x i8] }> <{ [16 x i8] undef, [1 x i8] zeroinitializer, [7 x i8] undef }>, align 8
@_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES = internal global <{ [96 x i8], [8 x i8] }> <{ [96 x i8] undef, [8 x i8] c"\03\00\00\00\00\00\00\00" }>, align 8
@alloc_a48736f782ce01888cde2ce41f6badc9 = private unnamed_addr constant [8 x i8] c"\FF\FF\FF\FF\FF\FF\FF\FF", align 8
@anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.2 = private unnamed_addr constant <{ ptr, [24 x i8] }> <{ ptr @alloc_a48736f782ce01888cde2ce41f6badc9, [24 x i8] zeroinitializer }>, align 8
@_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils6atomic11atomic_cell4lock5LOCKS = local_unnamed_addr global [8576 x i8] zeroinitializer, align 128
@alloc_779f4c4c227c35122e8522ff6a6f2abf = private unnamed_addr constant [8 x i8] c"<locked>", align 1
@alloc_467ef1e0da97d216b212e4a4e865bb86 = private unnamed_addr constant [12 x i8] c"Scope { .. }", align 1
@alloc_509d68a55f7b98fb47e78a4fe26f43b4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00x\00\00\001\00\00\00" }>, align 8
@alloc_1d58ea84bbabeff3efbb0146cc2da551 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00\83\00\00\001\00\00\00" }>, align 8
@alloc_bd08e1f6e55d704b221670e7013f731d = private unnamed_addr constant [13 x i8] c"Parker { .. }", align 1
@alloc_6e7eea196793eb89cef9a8d7b56ad060 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7e43212db14c6a005fdaae70f5f46558, [16 x i8] c"u\00\00\00\00\00\00\00\8E\00\00\007\00\00\00" }>, align 8
@alloc_040143ec29e8487e393481894bc3d0d2 = private unnamed_addr constant [9 x i8] c"WaitGroup", align 1
@vtable.6 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt }>, align 8
@alloc_cf49a750d6fbb7528728207623c220d8 = private unnamed_addr constant [5 x i8] c"count", align 1
@alloc_12266e6d1429d56a93188ba1015940af = private unnamed_addr constant [15 x i8] c"Unparker { .. }", align 1
@alloc_3ce6a1413887fb9dce1532808b933899 = private unnamed_addr constant [7 x i8] c"Backoff", align 1
@vtable.7 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00", ptr @_RNvXsu_NtCsjMrxcFdYDNN_4core3fmtINtNtB7_4cell4CellmENtB5_5Debug3fmtCs8jD91Rl7RDZ_15crossbeam_utils }>, align 8
@alloc_e8c8888d17bac1081896fdf7761479e9 = private unnamed_addr constant [4 x i8] c"step", align 1
@vtable.8 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsf_NtCsjMrxcFdYDNN_4core3fmtbNtB5_5Debug3fmt }>, align 8
@alloc_38120bd0eb09393889e2fcc8a0887b4a = private unnamed_addr constant [12 x i8] c"is_completed", align 1
@alloc_8e2410b80645266732854088d21653bc = private unnamed_addr constant [11 x i8] c"PoisonError", align 1
@alloc_4469d865571c05ec34aaa232b44160d5 = private unnamed_addr constant [120 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/crossbeam-utils-0.8.21/src/sync/sharded_lock.rs\00", align 1
@alloc_9734e5216d7378fcd643f75c93ad6161 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_4469d865571c05ec34aaa232b44160d5, [16 x i8] c"w\00\00\00\00\00\00\00d\02\00\003\00\00\00" }>, align 8
@alloc_30f876edaa9ab0a5b89f19bb4d7a0800 = private unnamed_addr constant [4 x i8] c"Cell", align 1
@vtable.9 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00\04\00\00\00\00\00\00\00", ptr @_RNvXsW_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_5Debug3fmt }>, align 8
@alloc_2fce15d1a77c62e67d5eacceaee24476 = private unnamed_addr constant [5 x i8] c"value", align 1

@_RNvMs4_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_8Unparker8from_raw = unnamed_addr alias ptr (ptr), ptr @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker8from_raw
@_RNvXs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB4_6ParkerNtNtCsjMrxcFdYDNN_4core7default7Default7default = unnamed_addr alias ptr (), ptr @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker3new
@_RNvXNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB2_9WaitGroupNtNtCsjMrxcFdYDNN_4core7default7Default7default = unnamed_addr alias ptr (), ptr @_RNvMs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB4_9WaitGroup3new

; <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
; Function Attrs: cold uwtable
define internal fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 captures(none) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %x.i = alloca [64 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 64, ptr nonnull %x.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %x.i, ptr noundef nonnull align 8 dereferenceable(64) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.0, i64 64, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #24, !noalias !2
; call __rustc::__rust_alloc
  %0 = tail call noundef align 8 dereferenceable_or_null(64) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 64, i64 noundef 8) #24, !noalias !2
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb2.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb2.i.i:                                          ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 64) #25
          to label %.noexc.i unwind label %cleanup.i.i

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb2.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 dereferenceable(64) %x.i)
          to label %common.resume unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %3 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

common.resume:                                    ; preds = %cleanup.i.i, %cleanup.i, %bb1.i
  %common.resume.op = phi { ptr, i32 } [ %9, %bb1.i ], [ %2, %cleanup.i.i ], [ %4, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %0, ptr noundef nonnull align 8 dereferenceable(64) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.0, i64 64, i1 false)
  call void @llvm.lifetime.end.p0(i64 64, ptr nonnull %x.i)
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::init
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4init(ptr noundef nonnull align 8 %0)
          to label %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit unwind label %cleanup.i

cleanup.i:                                        ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::pin::Pin<alloc::boxed::Box<std::sys::pal::unix::sync::mutex::Mutex>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils(ptr %0) #27
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %6 = cmpxchg ptr %self, ptr null, ptr %0 release acquire, align 8
  %7 = extractvalue { ptr, i1 } %6, 1
  %8 = extractvalue { ptr, i1 } %6, 0
  br i1 %7, label %bb5, label %bb3

bb3:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %0)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit unwind label %bb1.i

bb1.i:                                            ; preds = %bb3
  %9 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 64, i64 noundef 8) #24
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 64, i64 noundef 8) #24
  br label %bb5

bb5:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_0.sroa.0.0 = phi ptr [ %8, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit ], [ %0, %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5Mutex3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit ]
  %10 = icmp ne ptr %_0.sroa.0.0, null
  tail call void @llvm.assume(i1 %10)
  ret ptr %_0.sroa.0.0
}

; <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::condvar::Condvar>>::initialize::<<std::sys::sync::condvar::pthread::Condvar>::get::{closure#0}>
; Function Attrs: cold uwtable
define internal fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync7condvar7CondvarE10initializeNCNvMNtNtB5_7condvar7pthreadNtB1W_7Condvar3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 captures(none) %self) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %x.i = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %x.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %x.i, ptr noundef nonnull align 8 dereferenceable(48) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.1, i64 48, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #24, !noalias !6
; call __rustc::__rust_alloc
  %0 = tail call noundef align 8 dereferenceable_or_null(48) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 48, i64 noundef 8) #24, !noalias !6
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb2.i.i, label %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB4_7Condvar3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit, !prof !5

bb2.i.i:                                          ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 48) #25
          to label %.noexc.i unwind label %bb3.i.i

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

bb3.i.i:                                          ; preds = %bb2.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
  %r.i.i.i = call noundef i32 @pthread_cond_destroy(ptr noundef nonnull align 8 dereferenceable(48) %x.i) #24
  resume { ptr, i32 } %2

_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB4_7Condvar3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.1, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %x.i)
  %3 = cmpxchg ptr %self, ptr null, ptr %0 release acquire, align 8
  %4 = extractvalue { ptr, i1 } %3, 1
  br i1 %4, label %bb5, label %bb3

bb3:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB4_7Condvar3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit
  %5 = extractvalue { ptr, i1 } %3, 0
  %r.i.i.i1 = tail call noundef i32 @pthread_cond_destroy(ptr noundef nonnull align 8 %0) #24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %0, i64 noundef 48, i64 noundef 8) #24
  br label %bb5

bb5:                                              ; preds = %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB4_7Condvar3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit, %bb3
  %_0.sroa.0.0 = phi ptr [ %5, %bb3 ], [ %0, %_RNCNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB4_7Condvar3get0Cs8jD91Rl7RDZ_15crossbeam_utils.exit ]
  %6 = icmp ne ptr %_0.sroa.0.0, null
  tail call void @llvm.assume(i1 %6)
  ret ptr %_0.sroa.0.0
}

; <crossbeam_utils::sync::once_lock::OnceLock<std::sync::poison::mutex::Mutex<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>::initialize::<crossbeam_utils::sync::sharded_lock::thread_indices::init>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE10initializeNvNvB20_14thread_indices4initEBa_() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_11.i = alloca [8 x i8], align 8
  %f1.i = alloca [8 x i8], align 8
  %slot = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %slot)
  store ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, ptr %slot, align 8
  %0 = load atomic ptr, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 96) acquire, align 8, !noalias !9
  %_3.i = icmp eq ptr %0, null
  br i1 %_3.i, label %_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB15_8OnceLockINtNtNtB8_6poison5mutex5MutexNtNtB17_12sharded_lock13ThreadIndicesEE10initializeNvNvB2E_14thread_indices4initE0EB19_.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %f1.i), !noalias !9
  store ptr %slot, ptr %f1.i, align 8, !noalias !9
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_11.i), !noalias !9
  store ptr %f1.i, ptr %_11.i, align 8, !noalias !9
; call <std::sys::sync::once::queue::Once>::call
  call void @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync4once5queueNtB2_4Once4call(ptr noundef nonnull align 8 getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 96), i1 noundef zeroext false, ptr noundef nonnull align 1 %_11.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(40) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b0c1b9f0002e7b376ee1d8bf4e97d813)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_11.i), !noalias !9
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %f1.i), !noalias !9
  br label %_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB15_8OnceLockINtNtNtB8_6poison5mutex5MutexNtNtB17_12sharded_lock13ThreadIndicesEE10initializeNvNvB2E_14thread_indices4initE0EB19_.exit

_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB15_8OnceLockINtNtNtB8_6poison5mutex5MutexNtNtB17_12sharded_lock13ThreadIndicesEE10initializeNvNvB2E_14thread_indices4initE0EB19_.exit: ; preds = %start, %bb2.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %slot)
  ret void
}

; core::ptr::drop_in_place::<core::pin::Pin<alloc::boxed::Box<std::sys::pal::unix::sync::mutex::Mutex>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils(ptr nonnull %_1.0.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %_1.0.val)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef 64, i64 noundef 8) #24
  resume { ptr, i32 } %0

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef 64, i64 noundef 8) #24
  ret void
}

; core::ptr::drop_in_place::<alloc::sync::ArcInner<crossbeam_utils::sync::parker::Inner>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEEB1p_(ptr noalias noundef nonnull align 8 dereferenceable(56) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 16
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(40) %0)
          to label %bb4.i.i.i unwind label %cleanup.i.i.i

cleanup.i.i.i:                                    ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 dereferenceable(40) %0) #27
          to label %cleanup.body.i unwind label %terminate.i.i.i

bb4.i.i.i:                                        ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16)
  %ptr.i.i.i.i.i = load ptr, ptr %0, align 8, !alias.scope !19, !noundef !26
  store ptr null, ptr %0, align 8, !alias.scope !19
  %2 = icmp eq ptr %ptr.i.i.i.i.i, null
  br i1 %2, label %bb4.i, label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i.i.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i unwind label %bb1.i.i.i.i.i.i.i.i, !noalias !27

bb1.i.i.i.i.i.i.i.i:                              ; preds = %bb2.i.i.i.i.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i, i64 noundef 64, i64 noundef 8) #24, !noalias !27
  br label %cleanup.body.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i, i64 noundef 64, i64 noundef 8) #24, !noalias !27
  br label %bb4.i

terminate.i.i.i:                                  ; preds = %cleanup.i.i.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

cleanup.body.i:                                   ; preds = %bb1.i.i.i.i.i.i.i.i, %cleanup.i.i.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %3, %bb1.i.i.i.i.i.i.i.i ], [ %1, %cleanup.i.i.i ]
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; call core::ptr::drop_in_place::<std::sync::poison::condvar::Condvar>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef align 8 dereferenceable(16) %5) #27
  resume { ptr, i32 } %eh.lpad-body.i

bb4.i:                                            ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i, %bb4.i.i.i
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  tail call void @llvm.experimental.noalias.scope.decl(metadata !28)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !31)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !34)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !37)
  %ptr.i.i.i.i1.i = load ptr, ptr %6, align 8, !alias.scope !40, !noundef !26
  store ptr null, ptr %6, align 8, !alias.scope !40
  %7 = icmp eq ptr %ptr.i.i.i.i1.i, null
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEBM_.exit, label %bb2.i.i.i.i.i2.i

bb2.i.i.i.i.i2.i:                                 ; preds = %bb4.i
  %r.i.i.i.i.i.i.i.i.i.i = tail call noundef i32 @pthread_cond_destroy(ptr noundef nonnull align 8 %ptr.i.i.i.i1.i) #24, !noalias !41
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i1.i, i64 noundef 48, i64 noundef 8) #24, !noalias !41
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEBM_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEBM_.exit: ; preds = %bb4.i, %bb2.i.i.i.i.i2.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !26, !align !42, !noundef !26
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load i8, ptr %0, align 8, !range !43, !noundef !26
  %_3.i.i = getelementptr inbounds nuw i8, ptr %_1.val, i64 8
  %_3.i.i.i = trunc nuw i8 %_1.val1 to i1
  br i1 %_3.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i.i, 0
  br i1 %2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_3.i.i monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit: ; preds = %start, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %3 = load atomic ptr, ptr %_1.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::PoisonError<(std::sync::poison::mutex::MutexGuard<()>, std::sync::WaitTimeoutResult)>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorTINtNtBJ_5mutex10MutexGuarduENtBL_17WaitTimeoutResultEEECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 {
start:
  %_1.val = load ptr, ptr %_1, align 8, !nonnull !26, !align !42, !noundef !26
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load i8, ptr %0, align 8, !range !43, !noundef !26
  %_3.i.i.i = getelementptr inbounds nuw i8, ptr %_1.val, i64 8
  %_3.i.i.i.i = trunc nuw i8 %_1.val1 to i1
  br i1 %_3.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBO_17WaitTimeoutResultEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i.i.i, 0
  br i1 %2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBO_17WaitTimeoutResultEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i.i.i, !prof !12

bb6.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBO_17WaitTimeoutResultEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb6.i.i.i.i
  store atomic i8 1, ptr %_3.i.i.i monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBO_17WaitTimeoutResultEECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBO_17WaitTimeoutResultEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %start, %bb1.i.i.i.i, %bb6.i.i.i.i, %bb2.i.i.i.i
  %3 = load atomic ptr, ptr %_1.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !44)
  %ptr.i = load ptr, ptr %_1, align 8, !alias.scope !44, !noundef !26
  store ptr null, ptr %_1, align 8, !alias.scope !44
  %0 = icmp eq ptr %ptr.i, null
  br i1 %0, label %_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i

bb2.i.i:                                          ; preds = %start
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i unwind label %bb1.i.i.i.i, !noalias !44

bb1.i.i.i.i:                                      ; preds = %bb2.i.i
  %1 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i, i64 noundef 64, i64 noundef 8) #24, !noalias !44
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i: ; preds = %bb2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i, i64 noundef 64, i64 noundef 8) #24, !noalias !44
  br label %_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit

_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i
  ret void
}

; core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr captures(address_is_null) %_1.0.val, i8 %_1.8.val) unnamed_addr #1 {
start:
  %0 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %0)
  %_3.i = getelementptr inbounds nuw i8, ptr %_1.0.val, i64 8
  %_3.i.i = trunc nuw i8 %_1.8.val to i1
  br i1 %_3.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB19_.exit, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %1 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i = and i64 %1, 9223372036854775807
  %2 = icmp eq i64 %_7.i.i, 0
  br i1 %2, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB19_.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %bb1.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB19_.exit, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb6.i.i
  store atomic i8 1, ptr %_3.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB19_.exit

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropB19_.exit: ; preds = %start, %bb1.i.i, %bb6.i.i, %bb2.i.i
  %3 = load atomic ptr, ptr %_1.0.val monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %3)
  ret void
}

; core::ptr::drop_in_place::<crossbeam_utils::sync::wait_group::Inner>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEBM_(ptr noalias noundef nonnull align 8 dereferenceable(40) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !47)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !50)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !53)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !56)
  %ptr.i.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !59, !noundef !26
  store ptr null, ptr %_1, align 8, !alias.scope !59
  %0 = icmp eq ptr %ptr.i.i.i.i, null
  br i1 %0, label %bb4, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %start
  %r.i.i.i.i.i.i.i.i.i = tail call noundef i32 @pthread_cond_destroy(ptr noundef nonnull align 8 %ptr.i.i.i.i) #24, !noalias !59
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef 48, i64 noundef 8) #24, !noalias !59
  br label %bb4

bb4:                                              ; preds = %bb2.i.i.i.i.i, %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
; invoke <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(24) %1)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb4
  %2 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 dereferenceable(24) %1) #27
          to label %common.resume.i.i unwind label %terminate.i.i

bb4.i.i:                                          ; preds = %bb4
  tail call void @llvm.experimental.noalias.scope.decl(metadata !60)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !63)
  %ptr.i.i.i.i1 = load ptr, ptr %1, align 8, !alias.scope !66, !noundef !26
  store ptr null, ptr %1, align 8, !alias.scope !66
  %3 = icmp eq ptr %ptr.i.i.i.i1, null
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i.i.i2

bb2.i.i.i.i.i2:                                   ; preds = %bb4.i.i
; invoke <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
  invoke void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8 %ptr.i.i.i.i1)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i unwind label %bb1.i.i.i.i.i.i.i, !noalias !71

common.resume.i.i:                                ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i.i
  %common.resume.op.i.i = phi { ptr, i32 } [ %4, %bb1.i.i.i.i.i.i.i ], [ %2, %cleanup.i.i ]
  resume { ptr, i32 } %common.resume.op.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i2
  %4 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i1, i64 noundef 64, i64 noundef 8) #24, !noalias !71
  br label %common.resume.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i1, i64 noundef 64, i64 noundef 8) #24, !noalias !71
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexjEECs8jD91Rl7RDZ_15crossbeam_utils.exit

terminate.i.i:                                    ; preds = %cleanup.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexjEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb4.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_3pin3PinINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexEEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i
  ret void
}

; core::ptr::drop_in_place::<crossbeam_utils::sync::wait_group::WaitGroup>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; invoke <crossbeam_utils::sync::wait_group::WaitGroup as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %_1)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !72)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !75)
  %_10.i.i = load ptr, ptr %_1, align 8, !alias.scope !78, !nonnull !26, !noundef !26
  %1 = atomicrmw sub ptr %_10.i.i, i64 1 release, align 8, !noalias !78
  %2 = icmp eq i64 %1, 1
  br i1 %2, label %bb2.i.i, label %bb1

bb2.i.i:                                          ; preds = %cleanup
  fence acquire
; invoke <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_1) #28
          to label %bb1 unwind label %terminate

bb4:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !82)
  %_10.i.i1 = load ptr, ptr %_1, align 8, !alias.scope !85, !nonnull !26, !noundef !26
  %3 = atomicrmw sub ptr %_10.i.i1, i64 1 release, align 8, !noalias !85
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb2.i.i2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_.exit3

bb2.i.i2:                                         ; preds = %bb4
  fence acquire
; call <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  tail call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_1) #28
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_.exit3

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_.exit3: ; preds = %bb4, %bb2.i.i2
  ret void

terminate:                                        ; preds = %bb2.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

bb1:                                              ; preds = %cleanup, %bb2.i.i
  resume { ptr, i32 } %0
}

; core::ptr::drop_in_place::<std::sync::poison::condvar::Condvar>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !86)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !89)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !92)
  %ptr.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !95, !noundef !26
  store ptr null, ptr %_1, align 8, !alias.scope !95
  %0 = icmp eq ptr %ptr.i.i.i, null
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %start
  %r.i.i.i.i.i.i.i.i = tail call noundef i32 @pthread_cond_destroy(ptr noundef nonnull align 8 %ptr.i.i.i) #24, !noalias !95
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i, i64 noundef 48, i64 noundef 8) #24, !noalias !95
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %start, %bb2.i.i.i.i
  ret void
}

; <std::sync::once::Once>::call_once::<<crossbeam_utils::sync::once_lock::OnceLock<std::sync::poison::mutex::Mutex<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>::initialize<crossbeam_utils::sync::sharded_lock::thread_indices::init>::{closure#0}>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal void @_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB17_8OnceLockINtNtNtBa_6poison5mutex5MutexNtNtB19_12sharded_lock13ThreadIndicesEE10initializeNvNvB2G_14thread_indices4initE0E0B1b_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1, ptr nonnull readnone align 8 captures(none) %_2) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self1 = load ptr, ptr %_1, align 8, !nonnull !26, !align !42, !noundef !26
  %0 = load ptr, ptr %self1, align 8, !align !42, !noundef !26
  store ptr null, ptr %self1, align 8
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb3, label %bb4, !prof !5

bb4:                                              ; preds = %start
  %.val = load ptr, ptr %0, align 8
  %_3.i.i.i.i.i.i.i = tail call align 8 ptr @llvm.threadlocal.address.p0(ptr @_RNvNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBa_11RandomState3new4KEYS0s_023___RUST_STD_INTERNAL_VAL)
  %_12.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i, i64 16
  %1 = load i8, ptr %_12.i.i.i.i.i.i.i.i, align 8, !range !43, !noalias !96, !noundef !26
  %_4.i.i.i.i.i.i.i.i = trunc nuw i8 %1 to i1
  br i1 %_4.i.i.i.i.i.i.i.i, label %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i, label %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i, !prof !12

start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i: ; preds = %bb4
  %_9.i.pre.i.i.i.i.i = load i64, ptr %_3.i.i.i.i.i.i.i, align 8, !noalias !109
  %.phi.trans.insert.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i, i64 8
  %_10.i.pre.i.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i.i, align 8, !noalias !109
  br label %_RNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB8_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtBa_12sharded_lock13ThreadIndicesEE10initializeNvNvB22_14thread_indices4initE0Bc_.exit

_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i: ; preds = %bb4
; call std::sys::random::hashmap_random_keys
  %2 = tail call { i64, i64 } @_RNvNtNtCs5sEH5CPMdak_3std3sys6random19hashmap_random_keys(), !noalias !110
  %3 = extractvalue { i64, i64 } %2, 0
  %4 = extractvalue { i64, i64 } %2, 1
  %5 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i, i64 8
  store i64 %4, ptr %5, align 8, !noalias !110
  store i8 1, ptr %_12.i.i.i.i.i.i.i.i, align 8, !noalias !110
  br label %_RNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB8_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtBa_12sharded_lock13ThreadIndicesEE10initializeNvNvB22_14thread_indices4initE0Bc_.exit

_RNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB8_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtBa_12sharded_lock13ThreadIndicesEE10initializeNvNvB22_14thread_indices4initE0Bc_.exit: ; preds = %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i
  %_4.1.pre-phi.i.i.i = phi i64 [ %_10.i.pre.i.i.i.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i ], [ %4, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i ]
  %_9.i.i.i.i.i.i = phi i64 [ %_9.i.pre.i.i.i.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i ], [ %3, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i ]
  %_4.i.i.i.i.i.i = add i64 %_9.i.i.i.i.i.i, 1
  store i64 %_4.i.i.i.i.i.i, ptr %_3.i.i.i.i.i.i.i, align 8, !noalias !109
  store i64 0, ptr %.val, align 8
  %_5.sroa.4.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 8
  store i8 0, ptr %_5.sroa.4.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.6.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 16
  store i64 0, ptr %_5.sroa.6.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.7.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 24
  store ptr inttoptr (i64 8 to ptr), ptr %_5.sroa.7.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.8.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 32
  store i64 0, ptr %_5.sroa.8.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.9.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 40
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_5.sroa.9.0._1.0.val.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.2, i64 32, i1 false)
  %_5.sroa.10.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 72
  store i64 %_9.i.i.i.i.i.i, ptr %_5.sroa.10.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.11.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 80
  store i64 %_4.1.pre-phi.i.i.i, ptr %_5.sroa.11.0._1.0.val.sroa_idx.i, align 8
  %_5.sroa.12.0._1.0.val.sroa_idx.i = getelementptr inbounds nuw i8, ptr %.val, i64 88
  store i64 0, ptr %_5.sroa.12.0._1.0.val.sroa_idx.i, align 8
  ret void

bb3:                                              ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3129176114b12752e73d3989241bb5f0) #29
  unreachable
}

; <<std::sync::once::Once>::call_once<<crossbeam_utils::sync::once_lock::OnceLock<std::sync::poison::mutex::Mutex<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>::initialize<crossbeam_utils::sync::sharded_lock::thread_indices::init>::{closure#0}>::{closure#0} as core::ops::function::FnOnce<(&std::sync::once::OnceState,)>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal void @_RNSNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBd_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1c_8OnceLockINtNtNtBf_6poison5mutex5MutexNtNtB1e_12sharded_lock13ThreadIndicesEE10initializeNvNvB2L_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBd_9OnceStateEE9call_once6vtableB1g_(ptr noundef readonly captures(none) %_1, ptr nonnull readnone align 8 captures(none) %0) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %1 = load ptr, ptr %_1, align 8, !nonnull !26, !align !42, !noundef !26
  tail call void @llvm.experimental.noalias.scope.decl(metadata !113)
  %2 = load ptr, ptr %1, align 8, !alias.scope !113, !noalias !116, !align !42, !noundef !26
  store ptr null, ptr %1, align 8, !alias.scope !113, !noalias !116
  %.not.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i, label %bb3.i.i, label %bb4.i.i, !prof !5

bb4.i.i:                                          ; preds = %start
  %.val.i.i = load ptr, ptr %2, align 8, !noalias !119
  %_3.i.i.i.i.i.i.i.i.i = tail call align 8 ptr @llvm.threadlocal.address.p0(ptr @_RNvNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBa_11RandomState3new4KEYS0s_023___RUST_STD_INTERNAL_VAL)
  %_12.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i.i.i, i64 16
  %3 = load i8, ptr %_12.i.i.i.i.i.i.i.i.i.i, align 8, !range !43, !noalias !120, !noundef !26
  %_4.i.i.i.i.i.i.i.i.i.i = trunc nuw i8 %3 to i1
  br i1 %_4.i.i.i.i.i.i.i.i.i.i, label %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i.i.i, label %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i.i, !prof !12

start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i.i.i: ; preds = %bb4.i.i
  %_9.i.pre.i.i.i.i.i.i.i = load i64, ptr %_3.i.i.i.i.i.i.i.i.i, align 8, !noalias !133
  %.phi.trans.insert.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i.i.i, i64 8
  %_10.i.pre.i.i.i.i.i.i.i = load i64, ptr %.phi.trans.insert.i.i.i.i.i.i.i, align 8, !noalias !133
  br label %_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1a_8OnceLockINtNtNtBd_6poison5mutex5MutexNtNtB1c_12sharded_lock13ThreadIndicesEE10initializeNvNvB2J_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_.exit

_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i
; call std::sys::random::hashmap_random_keys
  %4 = tail call { i64, i64 } @_RNvNtNtCs5sEH5CPMdak_3std3sys6random19hashmap_random_keys(), !noalias !134
  %5 = extractvalue { i64, i64 } %4, 0
  %6 = extractvalue { i64, i64 } %4, 1
  %7 = getelementptr inbounds nuw i8, ptr %_3.i.i.i.i.i.i.i.i.i, i64 8
  store i64 %6, ptr %7, align 8, !noalias !134
  store i8 1, ptr %_12.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !134
  br label %_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1a_8OnceLockINtNtNtBd_6poison5mutex5MutexNtNtB1c_12sharded_lock13ThreadIndicesEE10initializeNvNvB2J_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_.exit

bb3.i.i:                                          ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3129176114b12752e73d3989241bb5f0) #29, !noalias !119
  unreachable

_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1a_8OnceLockINtNtNtBd_6poison5mutex5MutexNtNtB1c_12sharded_lock13ThreadIndicesEE10initializeNvNvB2J_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_.exit: ; preds = %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i.i.i, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i.i
  %_4.1.pre-phi.i.i.i.i.i = phi i64 [ %_10.i.pre.i.i.i.i.i.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i.i.i ], [ %6, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i.i ]
  %_9.i.i.i.i.i.i.i.i = phi i64 [ %_9.i.pre.i.i.i.i.i.i.i, %start._RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils.exit_crit_edge.i.i.i.i.i.i.i ], [ %5, %_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i.i.i.i.i.i ]
  %_4.i.i.i.i.i.i.i.i = add i64 %_9.i.i.i.i.i.i.i.i, 1
  store i64 %_4.i.i.i.i.i.i.i.i, ptr %_3.i.i.i.i.i.i.i.i.i, align 8, !noalias !133
  store i64 0, ptr %.val.i.i, align 8, !noalias !119
  %_5.sroa.4.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 8
  store i8 0, ptr %_5.sroa.4.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.6.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 16
  store i64 0, ptr %_5.sroa.6.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.7.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 24
  store ptr inttoptr (i64 8 to ptr), ptr %_5.sroa.7.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.8.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 32
  store i64 0, ptr %_5.sroa.8.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.9.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 40
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_5.sroa.9.0._1.0.val.sroa_idx.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) @anon.7c828d2dbbf9fbcbeaaa6e70d677eeed.2, i64 32, i1 false), !noalias !119
  %_5.sroa.10.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 72
  store i64 %_9.i.i.i.i.i.i.i.i, ptr %_5.sroa.10.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.11.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 80
  store i64 %_4.1.pre-phi.i.i.i.i.i, ptr %_5.sroa.11.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  %_5.sroa.12.0._1.0.val.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %.val.i.i, i64 88
  store i64 0, ptr %_5.sroa.12.0._1.0.val.sroa_idx.i.i.i, align 8, !noalias !119
  ret void
}

; <crossbeam_utils::sync::parker::Parker>::park_timeout
; Function Attrs: uwtable
define void @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker12park_timeout(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, i64 noundef %timeout.0, i32 noundef range(i32 0, 1000000000) %timeout.1) unnamed_addr #1 {
start:
  %_5 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5)
; call <std::time::Instant>::now
  %0 = tail call { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant3now()
  %1 = extractvalue { i64, i32 } %0, 0
  %2 = extractvalue { i64, i32 } %0, 1
  store i64 %1, ptr %_5, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store i32 %2, ptr %3, align 8
; call <std::time::Instant>::checked_add
  %4 = call { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant11checked_add(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %_5, i64 noundef %timeout.0, i32 noundef %timeout.1)
  %5 = extractvalue { i64, i32 } %4, 1
  %.not = icmp eq i32 %5, 1000000000
  br i1 %.not, label %bb4, label %bb5

bb5:                                              ; preds = %start
  %6 = extractvalue { i64, i32 } %4, 0
  call void @llvm.experimental.noalias.scope.decl(metadata !137)
  %_6.i = load ptr, ptr %self, align 8, !alias.scope !137, !nonnull !26, !noundef !26
  %_3.i = getelementptr inbounds nuw i8, ptr %_6.i, i64 16
; call <crossbeam_utils::sync::parker::Inner>::park
  call fastcc void @_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner4park(ptr noundef nonnull align 8 %_3.i, i64 %6, i32 noundef range(i32 0, 1000000000) %5), !noalias !137
  br label %bb6

bb4:                                              ; preds = %start
  call void @llvm.experimental.noalias.scope.decl(metadata !140)
  %_5.i = load ptr, ptr %self, align 8, !alias.scope !140, !nonnull !26, !noundef !26
  %_3.i3 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
; call <crossbeam_utils::sync::parker::Inner>::park
  call fastcc void @_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner4park(ptr noundef nonnull align 8 %_3.i3, i64 undef, i32 noundef 1000000000), !noalias !140
  br label %bb6

bb6:                                              ; preds = %bb5, %bb4
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  ret void
}

; <crossbeam_utils::sync::parker::Parker>::park_deadline
; Function Attrs: uwtable
define void @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker13park_deadline(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, i64 noundef %deadline.0, i32 noundef range(i32 0, 1000000000) %deadline.1) unnamed_addr #1 {
start:
  %_6 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_3 = getelementptr inbounds nuw i8, ptr %_6, i64 16
; call <crossbeam_utils::sync::parker::Inner>::park
  tail call fastcc void @_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner4park(ptr noundef nonnull align 8 %_3, i64 %deadline.0, i32 noundef %deadline.1)
  ret void
}

; <crossbeam_utils::sync::parker::Parker>::new
; Function Attrs: uwtable
define noalias noundef nonnull ptr @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker3new() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_17.i = alloca [56 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_17.i)
  store <2 x i64> splat (i64 1), ptr %_17.i, align 16
  %0 = getelementptr inbounds nuw i8, ptr %_17.i, i64 16
  store i64 0, ptr %0, align 16
  %_3.sroa.0.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_17.i, i64 24
  store i8 0, ptr %_3.sroa.0.sroa.4.0..sroa_idx.i, align 8
  %_3.sroa.0.sroa.6.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_17.i, i64 32
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_3.sroa.0.sroa.6.0..sroa_idx.i, i8 0, i64 24, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #24, !noalias !143
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(56) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 56, i64 noundef 8) #24, !noalias !143
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i.i, label %_RNvXs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB4_6ParkerNtNtCsjMrxcFdYDNN_4core7default7Default7default.exit, !prof !5

bb2.i.i:                                          ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 56) #25
          to label %.noexc.i unwind label %cleanup.i.i

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb2.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<alloc::sync::ArcInner<crossbeam_utils::sync::parker::Inner>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEEB1p_(ptr noalias noundef nonnull align 8 dereferenceable(56) %_17.i) #27
          to label %bb3.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

bb3.i.i:                                          ; preds = %cleanup.i.i
  resume { ptr, i32 } %3

_RNvXs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB4_6ParkerNtNtCsjMrxcFdYDNN_4core7default7Default7default.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %1, ptr noundef nonnull align 8 dereferenceable(56) %_17.i, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_17.i)
  ret ptr %1
}

; <crossbeam_utils::sync::parker::Parker>::park
; Function Attrs: uwtable
define void @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker4park(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 {
start:
  %_5 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_3 = getelementptr inbounds nuw i8, ptr %_5, i64 16
; call <crossbeam_utils::sync::parker::Inner>::park
  tail call fastcc void @_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner4park(ptr noundef nonnull align 8 %_3, i64 undef, i32 noundef 1000000000)
  ret void
}

; <crossbeam_utils::sync::parker::Parker>::from_raw
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define noundef nonnull ptr @_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker8from_raw(ptr noundef readnone captures(ret: address, provenance) %ptr) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %_7.i.i = getelementptr inbounds i8, ptr %ptr, i64 -16
  ret ptr %_7.i.i
}

; <crossbeam_utils::thread::ScopedThreadBuilder>::name
; Function Attrs: uwtable
define void @_RNvMs1_NtCs8jD91Rl7RDZ_15crossbeam_utils6threadNtB5_19ScopedThreadBuilder4name(ptr dead_on_unwind noalias noundef writable writeonly sret([56 x i8]) align 8 captures(none) dereferenceable(56) initializes((0, 56)) %_0, ptr dead_on_return noalias noundef align 8 captures(none) dereferenceable(56) %self, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(24) %name) unnamed_addr #1 {
start:
  %_4 = alloca [48 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_4)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_4, ptr noundef nonnull align 8 dereferenceable(48) %self, i64 48, i1 false)
; call <std::thread::builder::Builder>::name
  call void @_RNvMNtNtCs5sEH5CPMdak_3std6thread7builderNtB2_7Builder4name(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(none) dereferenceable(48) %_3, ptr noalias noundef nonnull align 8 captures(address) dereferenceable(48) %_4, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %name)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_4)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %self, ptr noundef nonnull align 8 dereferenceable(48) %_3, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %_0, ptr noundef nonnull align 8 dereferenceable(56) %self, i64 56, i1 false)
  ret void
}

; <alloc::raw_vec::RawVec<usize>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #5 {
start:
  %self1 = load i64, ptr %self, align 8, !range !146, !noundef !26
; call <alloc::raw_vec::RawVecInner>::grow_amortized
  %0 = tail call fastcc { i64, i64 } @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef align 8 dereferenceable(16) %self, i64 noundef %self1)
  %1 = extractvalue { i64, i64 } %0, 0
  %.not = icmp eq i64 %1, -9223372036854775807
  br i1 %.not, label %bb3, label %bb2, !prof !12

bb2:                                              ; preds = %start
  %2 = extractvalue { i64, i64 } %0, 1
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %1, i64 %2) #25
  unreachable

bb3:                                              ; preds = %start
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs8jD91Rl7RDZ_15crossbeam_utils(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef range(i64 0, -1) %cap) unnamed_addr #6 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %_24.i = add i64 %cap, -1
  %_27.0.i = shl i64 %_24.i, 3
  %_27.1.i = icmp ugt i64 %_24.i, 2305843009213693951
  %_32.i = icmp ugt i64 %_27.0.i, 9223372036854775800
  %or.cond.i = or i1 %_27.1.i, %_32.i
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !147

bb11.i:                                           ; preds = %bb6.i
  %new_size2.i = add nuw i64 %_27.0.i, 8
  %_40.i = icmp samesign ugt i64 %_27.0.i, 9223372036854775799
  br i1 %_40.i, label %bb11, label %bb14

bb14:                                             ; preds = %bb11.i
  %0 = icmp eq i64 %self.0.val, 0
  br i1 %0, label %bb4.i.i11, label %bb3.i.i

bb14.thread:                                      ; preds = %start
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb9, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb14.thread, %bb14
  %_27.sroa.7.01321 = phi i64 [ %new_size2.i, %bb14 ], [ 0, %bb14.thread ]
  %2 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %2)
  %alloc_size.i23 = shl nuw i64 %self.0.val, 3
  %cond.i.i = icmp uge i64 %_27.sroa.7.01321, %alloc_size.i23
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) 8, i64 noundef %_27.sroa.7.01321) #24
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #24
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) 8) #24
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %bb3.i.i
  %_27.sroa.7.012 = phi i64 [ %_27.sroa.7.01321, %bb3.i.i ], [ %new_size2.i, %bb4.i.i11 ]
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3.i.i ], [ %3, %bb4.i.i11 ]
  %4 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 8, ptr %5, align 8
  br label %bb11

bb9:                                              ; preds = %bb14.thread, %bb7
  %raw_ptr.i.i.pn31 = phi ptr [ %raw_ptr.i.i.pn, %bb7 ], [ inttoptr (i64 8 to ptr), %bb14.thread ]
  %_27.sroa.7.01230 = phi i64 [ %_27.sroa.7.012, %bb7 ], [ 0, %bb14.thread ]
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn31, ptr %6, align 8
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb11.i, %bb9, %bb8
  %.sink32 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %bb11.i ], [ 8, %bb6.i ]
  %_27.sroa.7.01230.sink = phi i64 [ %_27.sroa.7.01230, %bb9 ], [ %_27.sroa.7.012, %bb8 ], [ 0, %bb11.i ], [ 0, %bb6.i ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %bb11.i ], [ 1, %bb6.i ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink32
  store i64 %_27.sroa.7.01230.sink, ptr %7, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <alloc::raw_vec::RawVecInner>::grow_amortized
; Function Attrs: cold nounwind uwtable
define internal fastcc { i64, i64 } @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %self, i64 noundef range(i64 0, -9223372036854775808) %len) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %self3 = alloca [24 x i8], align 8
  %_25.0 = add nuw i64 %len, 1
  %self5 = load i64, ptr %self, align 8, !range !146, !noundef !26
  %v16 = shl nuw i64 %self5, 1
  %_0.sroa.0.0.i = tail call noundef range(i64 0, -1) i64 @llvm.umax.i64(i64 range(i64 0, -1) %_25.0, i64 range(i64 0, -1) %v16)
  %_0.sroa.0.0.i16 = tail call noundef range(i64 0, -1) i64 @llvm.umax.i64(i64 range(i64 0, -1) %_0.sroa.0.0.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15 = load ptr, ptr %0, align 8
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3, i64 %self5, ptr %self.val15, i64 noundef %_0.sroa.0.0.i16)
  %_35 = load i64, ptr %self3, align 8, !range !148, !noundef !26
  %1 = trunc nuw i64 %_35 to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3, i64 8
  br i1 %1, label %bb18, label %bb19

bb6:                                              ; preds = %bb18, %bb19
  %_0.sroa.5.0 = phi i64 [ %e.1, %bb18 ], [ undef, %bb19 ]
  %_0.sroa.0.0 = phi i64 [ %e.0, %bb18 ], [ -9223372036854775807, %bb19 ]
  %3 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0, 0
  %4 = insertvalue { i64, i64 } %3, i64 %_0.sroa.5.0, 1
  ret { i64, i64 } %4

bb18:                                             ; preds = %start
  %e.0 = load i64, ptr %2, align 8, !range !149, !noundef !26
  %5 = getelementptr inbounds nuw i8, ptr %self3, i64 16
  %e.1 = load i64, ptr %5, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3)
  br label %bb6

bb19:                                             ; preds = %start
  %v.0 = load ptr, ptr %2, align 8, !nonnull !26, !noundef !26
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3)
  store ptr %v.0, ptr %0, align 8
  %6 = icmp sgt i64 %_0.sroa.0.0.i16, -1
  tail call void @llvm.assume(i1 %6)
  store i64 %_0.sroa.0.0.i16, ptr %self, align 8
  br label %bb6
}

; <crossbeam_utils::sync::parker::Unparker>::unpark
; Function Attrs: uwtable
define void @_RNvMs4_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_8Unparker6unpark(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [16 x i8], align 8
  %_4 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_2 = getelementptr inbounds nuw i8, ptr %_4, i64 16
  %_11.i = getelementptr inbounds nuw i8, ptr %_4, i64 48
  %0 = atomicrmw xchg ptr %_11.i, i64 2 seq_cst, align 8
  switch i64 %0, label %bb1.i [
    i64 0, label %_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner6unpark.exit
    i64 2, label %_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner6unpark.exit
    i64 1, label %bb2.i
  ], !prof !150

bb1.i:                                            ; preds = %start
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_4abcb6df1c22df8ca969c35c4c0d7ea2, ptr noundef nonnull inttoptr (i64 57 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5f763a94cb0d078baa2ec601f473d62) #29
  unreachable

bb2.i:                                            ; preds = %start
  %1 = load atomic ptr, ptr %_2 acquire, align 8, !noalias !151
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb7.i.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i, !prof !5

bb7.i.i.i:                                        ; preds = %bb2.i
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %3 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_2), !noalias !151
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i: ; preds = %bb7.i.i.i, %bb2.i
  %_0.sroa.0.0.i.i.i = phi ptr [ %3, %bb7.i.i.i ], [ %1, %bb2.i ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i.i), !noalias !151
  %4 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !151
  %_6.i.i.i = and i64 %4, 9223372036854775807
  %5 = icmp eq i64 %_6.i.i.i, 0
  br i1 %5, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !12

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %6 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !151
  %7 = xor i1 %6, true
  %_7.i.i = getelementptr inbounds nuw i8, ptr %_4, i64 24
  %8 = load atomic i8, ptr %_7.i.i monotonic, align 1, !noalias !151
  %.not.i = icmp eq i8 %8, 0
  %_0.sroa.3.0.i.i.i = zext i1 %7 to i8
  br i1 %.not.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb2.i.i, !prof !12

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i.i
  %_7.i4.i = getelementptr inbounds nuw i8, ptr %_4, i64 24
  %9 = load atomic i8, ptr %_7.i4.i monotonic, align 1, !noalias !151
  %.not13.i = icmp eq i8 %9, 0
  br i1 %.not13.i, label %bb1.i.i.i.i, label %bb2.i.i, !prof !12

bb2.i.i:                                          ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i, %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %_0.sroa.3.0.i.i9.i = phi i8 [ 0, %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i ], [ %_0.sroa.3.0.i.i.i, %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i.i), !noalias !154
  store ptr %_2, ptr %e.i.i, align 8, !noalias !154
  %10 = getelementptr inbounds nuw i8, ptr %e.i.i, i64 8
  store i8 %_0.sroa.3.0.i.i9.i, ptr %10, align 8, !noalias !154
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_80b7b76bc8174414d036e884de381c56) #25
          to label %unreachable.i.i unwind label %cleanup.i.i, !noalias !158

cleanup.i.i:                                      ; preds = %bb2.i.i
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i.i) #27
          to label %bb5.i.i unwind label %terminate.i.i, !noalias !158

unreachable.i.i:                                  ; preds = %bb2.i.i
  unreachable

terminate.i.i:                                    ; preds = %cleanup.i.i
  %12 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !158
  unreachable

bb5.i.i:                                          ; preds = %cleanup.i.i
  resume { ptr, i32 } %11

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  br i1 %6, label %bb1.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

bb1.i.i.i.i:                                      ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i
  %_7.i812.i = phi ptr [ %_7.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit.i ], [ %_7.i4.i, %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit.thread.i ]
  %13 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i.i = and i64 %13, 9223372036854775807
  %14 = icmp eq i64 %_7.i.i.i.i, 0
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb6.i.i.i.i, !prof !12

bb6.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb6.i.i.i.i
  store atomic i8 1, ptr %_7.i812.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb2.i.i.i.i, %bb6.i.i.i.i, %bb1.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %15 = load atomic ptr, ptr %_2 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %15)
  %_9.i = getelementptr inbounds nuw i8, ptr %_4, i64 32
; call <std::sync::poison::condvar::Condvar>::notify_one
  tail call void @_RNvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB2_7Condvar10notify_one(ptr noundef nonnull align 8 %_9.i)
  br label %_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner6unpark.exit

_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner6unpark.exit: ; preds = %start, %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  ret void
}

; <crossbeam_utils::sync::parker::Inner>::park
; Function Attrs: uwtable
define internal fastcc void @_RNvMs7_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_5Inner4park(ptr noundef nonnull align 8 %self, i64 %0, i32 noundef range(i32 0, 1000000001) %1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i26 = alloca [24 x i8], align 8
  %e.i16 = alloca [16 x i8], align 8
  %e.i = alloca [16 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %n1 = alloca [8 x i8], align 8
  %args = alloca [16 x i8], align 8
  %n = alloca [8 x i8], align 8
  %old = alloca [8 x i8], align 8
  %_50 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %2 = cmpxchg ptr %_50, i64 2, i64 0 seq_cst seq_cst, align 8
  %_8.sroa.18.0.in.i = extractvalue { i64, i1 } %2, 1
  %args.sink.sroa.gep = getelementptr inbounds nuw i8, ptr %args, i64 8
  %args.sink.sroa.gep126 = getelementptr inbounds nuw i8, ptr %args2, i64 8
  br i1 %_8.sroa.18.0.in.i, label %bb35, label %bb2

bb2:                                              ; preds = %start
  %.not = icmp eq i32 %1, 1000000000
  br i1 %.not, label %bb7, label %bb3

bb35:                                             ; preds = %bb43, %bb42, %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit
  ret void

bb3:                                              ; preds = %bb2
; call <std::time::Instant>::now
  %3 = tail call { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant3now()
  %_6.0 = extractvalue { i64, i32 } %3, 0
  %4 = icmp eq i64 %0, %_6.0
  br i1 %4, label %bb43, label %bb42

bb7:                                              ; preds = %bb43, %bb42, %bb2
  %5 = load atomic ptr, ptr %self acquire, align 8, !noalias !159
  %6 = icmp eq ptr %5, null
  br i1 %6, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %bb7
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %7 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %self), !noalias !159
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %bb7
  %_0.sroa.0.0.i.i = phi ptr [ %7, %bb7.i.i ], [ %5, %bb7 ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !159
  %8 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !159
  %_6.i.i = and i64 %8, 9223372036854775807
  %9 = icmp eq i64 %_6.i.i, 0
  br i1 %9, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %10 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !159
  %11 = xor i1 %10, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %11, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %12 = load atomic i8, ptr %_7.i monotonic, align 8, !noalias !159
  %.not94 = icmp eq i8 %12, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not94, label %bb44, label %bb2.i20, !prof !12

bb2.i20:                                          ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i16), !noalias !162
  store ptr %self, ptr %e.i16, align 8, !noalias !162
  %13 = getelementptr inbounds nuw i8, ptr %e.i16, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %13, align 8, !noalias !162
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i16, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5b7852eb24eb1fd922b857fc00d93d48) #25
          to label %unreachable.i24 unwind label %cleanup.i21, !noalias !166

cleanup.i21:                                      ; preds = %bb2.i20
  %14 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i16) #27
          to label %common.resume unwind label %terminate.i22, !noalias !166

unreachable.i24:                                  ; preds = %bb2.i20
  unreachable

terminate.i22:                                    ; preds = %cleanup.i21
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !166
  unreachable

common.resume:                                    ; preds = %bb39, %cleanup.i44, %cleanup.i, %cleanup.i29, %bb37, %cleanup.i54, %cleanup.i21
  %common.resume.op = phi { ptr, i32 } [ %14, %cleanup.i21 ], [ %lpad.phi, %bb39 ], [ %lpad.phi101, %cleanup.i44 ], [ %29, %cleanup.i ], [ %43, %cleanup.i29 ], [ %46, %bb37 ], [ %40, %cleanup.i54 ]
  resume { ptr, i32 } %common.resume.op

bb43:                                             ; preds = %bb3
  %_6.1 = extractvalue { i64, i32 } %3, 1
  %16 = icmp ult i32 %_6.1, 1000000000
  tail call void @llvm.assume(i1 %16)
  %.not93 = icmp samesign ugt i32 %1, %_6.1
  br i1 %.not93, label %bb7, label %bb35

bb42:                                             ; preds = %bb3
  %.not92 = icmp sgt i64 %0, %_6.0
  br i1 %.not92, label %bb7, label %bb35

bb44:                                             ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %17 = cmpxchg ptr %_50, i64 0, i64 1 seq_cst seq_cst, align 8
  %_8.sroa.18.0.in.i40 = extractvalue { i64, i1 } %17, 1
  %_8.sroa.0.0.i43 = extractvalue { i64, i1 } %17, 0
  br i1 %_8.sroa.18.0.in.i40, label %bb17.preheader, label %bb11

bb17.preheader:                                   ; preds = %bb44
  %_34 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  br i1 %.not, label %bb17.us, label %bb17

bb17.us:                                          ; preds = %bb17.preheader, %bb49.us
  %18 = load atomic ptr, ptr %self monotonic, align 8, !noalias !167
  %addr.i.i.i.us = ptrtoint ptr %18 to i64
  %19 = cmpxchg ptr %_18.i.i.i, i64 0, i64 %addr.i.i.i.us monotonic monotonic, align 8, !noalias !167
  %_8.sroa.18.0.in.i.i.i.i.us = extractvalue { i64, i1 } %19, 1
  %_8.sroa.0.0.i.i.i.i.us = extractvalue { i64, i1 } %19, 0
  %_11.i.i.i.us = icmp eq i64 %_8.sroa.0.0.i.i.i.i.us, %addr.i.i.i.us
  %or.cond.not.not.i.i.i.us = select i1 %_8.sroa.18.0.in.i.i.i.i.us, i1 true, i1 %_11.i.i.i.us
  br i1 %or.cond.not.not.i.i.i.us, label %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i.us, label %bb3.i.i.i, !prof !170

_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i.us: ; preds = %bb17.us
  %20 = load atomic ptr, ptr %_34 acquire, align 8, !noalias !167
  %21 = icmp eq ptr %20, null
  br i1 %21, label %bb7.i.i.i.us, label %bb20.us, !prof !5

bb7.i.i.i.us:                                     ; preds = %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i.us
; invoke <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::condvar::Condvar>>::initialize::<<std::sys::sync::condvar::pthread::Condvar>::get::{closure#0}>
  %22 = invoke fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync7condvar7CondvarE10initializeNCNvMNtNtB5_7condvar7pthreadNtB1W_7Condvar3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_34)
          to label %bb20.us unwind label %cleanup.i44.loopexit.split.us, !noalias !167

bb20.us:                                          ; preds = %bb7.i.i.i.us, %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i.us
  %_0.sroa.0.0.i.i.i.us = phi ptr [ %20, %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i.us ], [ %22, %bb7.i.i.i.us ]
  %r.i.i.us = tail call noundef i32 @pthread_cond_wait(ptr noundef nonnull %_0.sroa.0.0.i.i.i.us, ptr noundef nonnull %18) #24, !noalias !167
  %23 = load atomic i8, ptr %_7.i monotonic, align 8, !noalias !167
  %.not96.us = icmp eq i8 %23, 0
  br i1 %.not96.us, label %bb49.us, label %bb2.i, !prof !12

bb49.us:                                          ; preds = %bb20.us
  %24 = cmpxchg ptr %_50, i64 2, i64 0 seq_cst seq_cst, align 8
  %_8.sroa.18.0.in.i50.us = extractvalue { i64, i1 } %24, 1
  br i1 %_8.sroa.18.0.in.i50.us, label %bb33, label %bb17.us

cleanup.i44.loopexit.split.us:                    ; preds = %bb7.i.i.i.us
  %lpad.loopexit99.us = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i44

bb11:                                             ; preds = %bb44
  %25 = icmp eq i64 %_8.sroa.0.0.i43, 2
  br i1 %25, label %bb13, label %bb12, !prof !12

bb17:                                             ; preds = %bb17.preheader, %bb49
; invoke <std::time::Instant>::now
  %26 = invoke { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant3now()
          to label %bb22 unwind label %bb39.loopexit

bb3.i.i.i:                                        ; preds = %bb17.us
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_da75be5cf355a0c62ad51607063f583d, ptr noundef nonnull inttoptr (i64 109 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33c80814996188919076c1dd2d93215e) #29
          to label %.noexc.i unwind label %cleanup.i44.loopexit.split-lp, !noalias !167

.noexc.i:                                         ; preds = %bb3.i.i.i
  unreachable

cleanup.i44.loopexit.split-lp:                    ; preds = %bb3.i.i.i
  %lpad.loopexit.split-lp100 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i44

cleanup.i44:                                      ; preds = %cleanup.i44.loopexit.split-lp, %cleanup.i44.loopexit.split.us
  %lpad.phi101 = phi { ptr, i32 } [ %lpad.loopexit99.us, %cleanup.i44.loopexit.split.us ], [ %lpad.loopexit.split-lp100, %cleanup.i44.loopexit.split-lp ]
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull align 8 %self, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate.i45, !noalias !167

terminate.i45:                                    ; preds = %cleanup.i44
  %27 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !167
  unreachable

bb2.i:                                            ; preds = %bb20.us
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !171
  store ptr %self, ptr %e.i, align 8, !noalias !171
  %28 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %28, align 8, !noalias !171
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_5dd4f91c097d8856a8d5b8665ac69700) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !175

cleanup.i:                                        ; preds = %bb2.i
  %29 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %common.resume unwind label %terminate.i, !noalias !175

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %30 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !175
  unreachable

bb22:                                             ; preds = %bb17
  %now.0 = extractvalue { i64, i32 } %26, 0
  %now.1 = extractvalue { i64, i32 } %26, 1
  %31 = icmp eq i64 %now.0, %0
  %32 = icmp slt i64 %now.0, %0
  %33 = icmp samesign ult i32 %now.1, %1
  %spec.select = select i1 %31, i1 %33, i1 %32
  br i1 %spec.select, label %bb23, label %bb27

bb27:                                             ; preds = %bb22
  %34 = atomicrmw xchg ptr %_50, i64 0 seq_cst, align 8
  %.off = add i64 %34, -1
  %switch = icmp ult i64 %.off, 2
  br i1 %switch, label %bb33, label %bb28, !prof !176

bb23:                                             ; preds = %bb22
; invoke <std::time::Instant as core::ops::arith::Sub>::sub
  %35 = invoke { i64, i32 } @_RNvXs3_NtCs5sEH5CPMdak_3std4timeNtB5_7InstantNtNtNtCsjMrxcFdYDNN_4core3ops5arith3Sub3sub(i64 noundef %0, i32 noundef %1, i64 noundef %now.0, i32 noundef %now.1)
          to label %bb24 unwind label %bb37

bb28:                                             ; preds = %bb27
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n1)
  store i64 %34, ptr %n1, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %n1, ptr %args2, align 8
  br label %bb28.invoke

bb28.invoke:                                      ; preds = %bb12, %bb28
  %args.sink.sroa.phi = phi ptr [ %args.sink.sroa.gep, %bb12 ], [ %args.sink.sroa.gep126, %bb28 ]
  %args.sink = phi ptr [ %args, %bb12 ], [ %args2, %bb28 ]
  %36 = phi ptr [ @alloc_2dc5e81b43b3aed87b3d95c30c679ec4, %bb12 ], [ @alloc_6f84445f1996a99b81bfc2712928887a, %bb28 ]
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %args.sink.sroa.phi, align 8
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_4995b9999b13a44a131c11df8c199b04, ptr noundef nonnull %args.sink, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %36) #25
          to label %bb28.cont unwind label %bb39.loopexit.split-lp

bb28.cont:                                        ; preds = %bb28.invoke
  unreachable

bb33:                                             ; preds = %bb49, %bb49.us, %bb27, %bb15
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb33
  %37 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %37, 9223372036854775807
  %38 = icmp eq i64 %_7.i.i.i, 0
  br i1 %38, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb33, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %39 = load atomic ptr, ptr %self monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %39)
  br label %bb35

unreachable:                                      ; preds = %bb16
  unreachable

bb24:                                             ; preds = %bb23
  %_36.0 = extractvalue { i64, i32 } %35, 0
  %_36.1 = extractvalue { i64, i32 } %35, 1
; invoke <std::sys::sync::condvar::pthread::Condvar>::wait_timeout
  %success.i = invoke noundef zeroext i1 @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar12wait_timeout(ptr noundef nonnull align 8 %_34, ptr noundef nonnull align 8 %self, i64 noundef %_36.0, i32 noundef range(i32 0, 1000000000) %_36.1)
          to label %bb25 unwind label %cleanup.i54, !noalias !177

cleanup.i54:                                      ; preds = %bb24
  %40 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull align 8 %self, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate.i55, !noalias !177

terminate.i55:                                    ; preds = %cleanup.i54
  %41 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !177
  unreachable

bb25:                                             ; preds = %bb24
  %42 = load atomic i8, ptr %_7.i monotonic, align 8, !noalias !177
  %.not95 = icmp eq i8 %42, 0
  br i1 %.not95, label %bb49, label %bb2.i28, !prof !12

bb2.i28:                                          ; preds = %bb25
  %_12.i = xor i1 %success.i, true
  %.sink.i = zext i1 %_12.i to i8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %e.i26), !noalias !180
  store ptr %self, ptr %e.i26, align 8, !noalias !184
  %_33.sroa.7.8.e.i26.sroa_idx = getelementptr inbounds nuw i8, ptr %e.i26, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %_33.sroa.7.8.e.i26.sroa_idx, align 8, !noalias !184
  %_33.sroa.10.8.e.i26.sroa_idx = getelementptr inbounds nuw i8, ptr %e.i26, i64 16
  store i8 %.sink.i, ptr %_33.sroa.10.8.e.i26.sroa_idx, align 8, !noalias !184
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i26, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_dc35fa7e21dfb2345fd82d2a720ab282) #25
          to label %unreachable.i32 unwind label %cleanup.i29, !noalias !180

cleanup.i29:                                      ; preds = %bb2.i28
  %43 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<(std::sync::poison::mutex::MutexGuard<()>, std::sync::WaitTimeoutResult)>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorTINtNtBJ_5mutex10MutexGuarduENtBL_17WaitTimeoutResultEEECs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 dereferenceable(24) %e.i26) #27
          to label %common.resume unwind label %terminate.i30, !noalias !180

unreachable.i32:                                  ; preds = %bb2.i28
  unreachable

terminate.i30:                                    ; preds = %cleanup.i29
  %44 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !180
  unreachable

bb49:                                             ; preds = %bb25
  %45 = cmpxchg ptr %_50, i64 2, i64 0 seq_cst seq_cst, align 8
  %_8.sroa.18.0.in.i50 = extractvalue { i64, i1 } %45, 1
  br i1 %_8.sroa.18.0.in.i50, label %bb33, label %bb17

bb37:                                             ; preds = %bb23
  %46 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull %self, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate

terminate:                                        ; preds = %bb37, %bb39
  %47 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

bb13:                                             ; preds = %bb11
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %old)
  %48 = atomicrmw xchg ptr %_50, i64 0 seq_cst, align 8
  store i64 %48, ptr %old, align 8
  %49 = icmp eq i64 %48, 2
  br i1 %49, label %bb15, label %bb16, !prof !12

bb12:                                             ; preds = %bb11
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %n)
  store i64 %_8.sroa.0.0.i43, ptr %n, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %n, ptr %args, align 8
  br label %bb28.invoke

bb15:                                             ; preds = %bb13
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %old)
  br label %bb33

bb16:                                             ; preds = %bb13
; invoke core::panicking::assert_failed::<usize, usize>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %old, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) @alloc_f85406d658e94b9c37cd2112f10307ee, ptr noundef nonnull @alloc_4956e422da54fdcef75db47b62671705, ptr nonnull inttoptr (i64 63 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ce448046d8ffdf2bb268a748fb5a871) #25
          to label %unreachable unwind label %bb39.loopexit.split-lp

bb39.loopexit:                                    ; preds = %bb17
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %bb39

bb39.loopexit.split-lp:                           ; preds = %bb28.invoke, %bb16
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb39

bb39:                                             ; preds = %bb39.loopexit.split-lp, %bb39.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %bb39.loopexit ], [ %lpad.loopexit.split-lp, %bb39.loopexit.split-lp ]
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull %self, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate
}

; <crossbeam_utils::sync::wait_group::WaitGroup>::new
; Function Attrs: uwtable
define noalias noundef nonnull ptr @_RNvMs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB4_9WaitGroup3new() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_15.i = alloca [56 x i8], align 16
  call void @llvm.lifetime.start.p0(i64 56, ptr nonnull %_15.i)
  store <2 x i64> splat (i64 1), ptr %_15.i, align 16
  %0 = getelementptr inbounds nuw i8, ptr %_15.i, i64 16
  %_2.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_15.i, i64 48
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(25) %0, i8 0, i64 25, i1 false)
  store i64 1, ptr %_2.sroa.5.0..sroa_idx.i, align 16
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #24, !noalias !185
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(56) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 56, i64 noundef 8) #24, !noalias !185
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i.i, label %_RNvXNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB2_9WaitGroupNtNtCsjMrxcFdYDNN_4core7default7Default7default.exit, !prof !5

bb2.i.i:                                          ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 56) #25
          to label %.noexc.i unwind label %cleanup.i.i

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

cleanup.i.i:                                      ; preds = %bb2.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<crossbeam_utils::sync::wait_group::Inner>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEBM_(ptr noalias noundef align 8 dereferenceable(40) %0)
          to label %bb3.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

bb3.i.i:                                          ; preds = %cleanup.i.i
  resume { ptr, i32 } %3

_RNvXNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB2_9WaitGroupNtNtCsjMrxcFdYDNN_4core7default7Default7default.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(56) %1, ptr noundef nonnull align 8 dereferenceable(56) %_15.i, i64 56, i1 false)
  call void @llvm.lifetime.end.p0(i64 56, ptr nonnull %_15.i)
  ret ptr %1
}

; <crossbeam_utils::sync::wait_group::WaitGroup>::wait
; Function Attrs: uwtable
define void @_RNvMs_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB4_9WaitGroup4wait(ptr noundef nonnull %0) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i16 = alloca [16 x i8], align 8
  %e.i4 = alloca [16 x i8], align 8
  %e.i = alloca [16 x i8], align 8
  %_7 = alloca [8 x i8], align 8
  %inner = alloca [8 x i8], align 8
  %self = alloca [8 x i8], align 8
  store ptr %0, ptr %self, align 8
  %_5 = getelementptr inbounds nuw i8, ptr %0, i64 32
  %1 = load atomic ptr, ptr %_5 acquire, align 8, !noalias !188
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %start
; invoke <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %3 = invoke fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_5)
          to label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i unwind label %bb20.thread107

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %start
  %_0.sroa.0.0.i.i = phi ptr [ %1, %start ], [ %3, %bb7.i.i ]
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::lock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i)
          to label %.noexc31 unwind label %bb20.thread107

.noexc31:                                         ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %4 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !188
  %_6.i.i = and i64 %4, 9223372036854775807
  %5 = icmp eq i64 %_6.i.i, 0
  br i1 %5, label %bb1, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %.noexc31
; invoke std::panicking::panic_count::is_zero_slow_path
  %6 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
          to label %.noexc32 unwind label %bb20.thread107

.noexc32:                                         ; preds = %bb6.i.i
  %7 = xor i1 %6, true
  br label %bb1

bb20.thread107:                                   ; preds = %bb7.i.i, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i, %bb6.i.i.i, %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i.i39, %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
  br label %bb19

bb1:                                              ; preds = %.noexc32, %.noexc31
  %_5.sroa.0.0.off0.i.i = phi i1 [ %7, %.noexc32 ], [ false, %.noexc31 ]
  %_7.i = getelementptr inbounds nuw i8, ptr %0, i64 40
  %8 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !188
  %.not = icmp eq i8 %8, 0
  br i1 %.not, label %bb2, label %bb2.i20, !prof !12

bb2.i20:                                          ; preds = %bb1
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i16), !noalias !191
  store ptr %_5, ptr %e.i16, align 8, !noalias !191
  %9 = getelementptr inbounds nuw i8, ptr %e.i16, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %9, align 8, !noalias !191
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i16, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9067e1d5a668f509be145c44a0a64f14) #25
          to label %unreachable.i24 unwind label %cleanup.i21, !noalias !195

cleanup.i21:                                      ; preds = %bb2.i20
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i16) #27
          to label %bb19 unwind label %terminate.i22, !noalias !195

unreachable.i24:                                  ; preds = %bb2.i20
  unreachable

terminate.i22:                                    ; preds = %cleanup.i21
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !195
  unreachable

bb2:                                              ; preds = %bb1
  %_22 = getelementptr inbounds nuw i8, ptr %0, i64 48
  %_2 = load i64, ptr %_22, align 8, !noundef !26
  %12 = icmp eq i64 %_2, 1
  br i1 %12, label %bb3, label %bb5

bb3:                                              ; preds = %bb2
  br i1 %_5.sroa.0.0.off0.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb3
  %13 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %13, 9223372036854775807
  %14 = icmp eq i64 %_7.i.i.i, 0
  br i1 %14, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; invoke std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i33 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
          to label %_6.i.i.i.noexc unwind label %bb20.thread107

_6.i.i.i.noexc:                                   ; preds = %bb6.i.i.i
  br i1 %_6.i.i.i33, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %_6.i.i.i.noexc
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb2.i.i.i, %_6.i.i.i.noexc, %bb1.i.i.i, %bb3
  %15 = load atomic ptr, ptr %_5 monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %15)
          to label %bb4 unwind label %bb20.thread107

bb5:                                              ; preds = %bb2
  br i1 %_5.sroa.0.0.off0.i.i, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41, label %bb1.i.i.i37

bb1.i.i.i37:                                      ; preds = %bb5
  %16 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i38 = and i64 %16, 9223372036854775807
  %17 = icmp eq i64 %_7.i.i.i38, 0
  br i1 %17, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41, label %bb6.i.i.i39, !prof !12

bb6.i.i.i39:                                      ; preds = %bb1.i.i.i37
; invoke std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i43 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
          to label %_6.i.i.i.noexc42 unwind label %bb20.thread107

_6.i.i.i.noexc42:                                 ; preds = %bb6.i.i.i39
  br i1 %_6.i.i.i43, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41, label %bb2.i.i.i40

bb2.i.i.i40:                                      ; preds = %_6.i.i.i.noexc42
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41: ; preds = %bb2.i.i.i40, %_6.i.i.i.noexc42, %bb1.i.i.i37, %bb5
  %18 = load atomic ptr, ptr %_5 monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %18)
          to label %bb6 unwind label %bb20.thread107

bb4:                                              ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; invoke <crossbeam_utils::sync::wait_group::WaitGroup as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %self)
          to label %bb4.i unwind label %cleanup.i46

cleanup.i46:                                      ; preds = %bb4
  %19 = landingpad { ptr, i32 }
          cleanup
  %20 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !196
  %21 = icmp eq i64 %20, 1
  br i1 %21, label %bb2.i.i.i47, label %common.resume

bb2.i.i.i47:                                      ; preds = %cleanup.i46
  fence acquire
; invoke <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %self) #28
          to label %common.resume unwind label %terminate.i48

bb4.i:                                            ; preds = %bb4
  %22 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !203
  %23 = icmp eq i64 %22, 1
  br i1 %23, label %bb2.i.i2.i, label %bb16

bb2.i.i2.i:                                       ; preds = %bb4.i
  fence acquire
; call <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %self) #28
  br label %bb16

terminate.i48:                                    ; preds = %bb2.i.i.i47
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !208
  unreachable

common.resume:                                    ; preds = %bb19, %cleanup1.body, %bb2.i.i, %cleanup.i46, %bb2.i.i.i47
  %common.resume.op = phi { ptr, i32 } [ %19, %bb2.i.i.i47 ], [ %19, %cleanup.i46 ], [ %eh.lpad-body25106, %bb19 ], [ %eh.lpad-body, %cleanup1.body ], [ %eh.lpad-body, %bb2.i.i ]
  resume { ptr, i32 } %common.resume.op

bb16:                                             ; preds = %bb2.i.i2.i, %bb4.i, %bb15
  ret void

bb6:                                              ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i41
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %inner)
  %25 = atomicrmw add ptr %0, i64 1 monotonic, align 8
  %_24 = icmp slt i64 %25, 0
  br i1 %_24, label %bb21, label %bb22

bb22:                                             ; preds = %bb6
  store ptr %0, ptr %inner, align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %0, ptr %_7, align 8
; invoke <crossbeam_utils::sync::wait_group::WaitGroup as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_7)
          to label %bb4.i54 unwind label %cleanup.i49

cleanup.i49:                                      ; preds = %bb22
  %26 = landingpad { ptr, i32 }
          cleanup
  %27 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !209
  %28 = icmp eq i64 %27, 1
  br i1 %28, label %bb2.i.i.i52, label %cleanup1.body

bb2.i.i.i52:                                      ; preds = %cleanup.i49
  fence acquire
; invoke <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_7) #28
          to label %cleanup1.body unwind label %terminate.i53

bb4.i54:                                          ; preds = %bb22
  %29 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !216
  %30 = icmp eq i64 %29, 1
  br i1 %30, label %bb2.i.i2.i56, label %bb24

bb2.i.i2.i56:                                     ; preds = %bb4.i54
  fence acquire
; invoke <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %_7) #28
          to label %bb24 unwind label %cleanup1

terminate.i53:                                    ; preds = %bb2.i.i.i52
  %31 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !221
  unreachable

bb21:                                             ; preds = %bb6
  tail call void @llvm.trap()
  unreachable

cleanup1:                                         ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81, %bb6.i.i.i79, %bb6.i.i65, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i62, %bb7.i.i70, %bb2.i.i2.i56
  %32 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup1.body

cleanup1.body:                                    ; preds = %cleanup.i, %cleanup.i86, %cleanup.i9, %cleanup1, %cleanup.i49, %bb2.i.i.i52
  %eh.lpad-body = phi { ptr, i32 } [ %26, %bb2.i.i.i52 ], [ %26, %cleanup.i49 ], [ %32, %cleanup1 ], [ %44, %cleanup.i9 ], [ %lpad.phi, %cleanup.i86 ], [ %61, %cleanup.i ]
  %33 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !222
  %34 = icmp eq i64 %33, 1
  br i1 %34, label %bb2.i.i, label %common.resume

bb2.i.i:                                          ; preds = %cleanup1.body
  fence acquire
; invoke <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  invoke void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %inner) #28
          to label %common.resume unwind label %terminate

bb24:                                             ; preds = %bb4.i54, %bb2.i.i2.i56
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  %35 = load atomic ptr, ptr %_5 acquire, align 8, !noalias !227
  %36 = icmp eq ptr %35, null
  br i1 %36, label %bb7.i.i70, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i62, !prof !5

bb7.i.i70:                                        ; preds = %bb24
; invoke <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %37 = invoke fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_5)
          to label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i62 unwind label %cleanup1

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i62: ; preds = %bb7.i.i70, %bb24
  %_0.sroa.0.0.i.i63 = phi ptr [ %35, %bb24 ], [ %37, %bb7.i.i70 ]
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::lock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i63)
          to label %.noexc72 unwind label %cleanup1

.noexc72:                                         ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i62
  %38 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !227
  %_6.i.i64 = and i64 %38, 9223372036854775807
  %39 = icmp eq i64 %_6.i.i64, 0
  br i1 %39, label %bb7, label %bb6.i.i65, !prof !12

bb6.i.i65:                                        ; preds = %.noexc72
; invoke std::panicking::panic_count::is_zero_slow_path
  %40 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
          to label %.noexc73 unwind label %cleanup1

.noexc73:                                         ; preds = %bb6.i.i65
  %41 = xor i1 %40, true
  br label %bb7

bb7:                                              ; preds = %.noexc73, %.noexc72
  %_5.sroa.0.0.off0.i.i66 = phi i1 [ %41, %.noexc73 ], [ false, %.noexc72 ]
  %42 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !227
  %.not110 = icmp eq i8 %42, 0
  br i1 %.not110, label %bb9.preheader, label %bb2.i8, !prof !12

bb9.preheader:                                    ; preds = %bb7
  %_15 = getelementptr inbounds nuw i8, ptr %0, i64 16
  %_18.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 24
  br label %bb9

bb2.i8:                                           ; preds = %bb7
  %_0.sroa.3.0.i.i68 = zext i1 %_5.sroa.0.0.off0.i.i66 to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i4), !noalias !230
  store ptr %_5, ptr %e.i4, align 8, !noalias !230
  %43 = getelementptr inbounds nuw i8, ptr %e.i4, i64 8
  store i8 %_0.sroa.3.0.i.i68, ptr %43, align 8, !noalias !230
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_af70a3b648b95ea60237fd7c7e73ca28) #25
          to label %unreachable.i12 unwind label %cleanup.i9, !noalias !234

cleanup.i9:                                       ; preds = %bb2.i8
  %44 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i4) #27
          to label %cleanup1.body unwind label %terminate.i10, !noalias !234

unreachable.i12:                                  ; preds = %bb2.i8
  unreachable

terminate.i10:                                    ; preds = %cleanup.i9
  %45 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !234
  unreachable

bb9:                                              ; preds = %bb9.preheader, %bb11
  %_12 = load i64, ptr %_22, align 8, !noundef !26
  %_11.not = icmp eq i64 %_12, 0
  br i1 %_11.not, label %bb13, label %bb10

bb13:                                             ; preds = %bb9
  br i1 %_5.sroa.0.0.off0.i.i66, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81, label %bb1.i.i.i77

bb1.i.i.i77:                                      ; preds = %bb13
  %46 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i78 = and i64 %46, 9223372036854775807
  %47 = icmp eq i64 %_7.i.i.i78, 0
  br i1 %47, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81, label %bb6.i.i.i79, !prof !12

bb6.i.i.i79:                                      ; preds = %bb1.i.i.i77
; invoke std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i83 = invoke noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
          to label %_6.i.i.i.noexc82 unwind label %cleanup1

_6.i.i.i.noexc82:                                 ; preds = %bb6.i.i.i79
  br i1 %_6.i.i.i83, label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81, label %bb2.i.i.i80

bb2.i.i.i80:                                      ; preds = %_6.i.i.i.noexc82
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81

_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81: ; preds = %bb2.i.i.i80, %_6.i.i.i.noexc82, %bb1.i.i.i77, %bb13
  %48 = load atomic ptr, ptr %_5 monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %48)
          to label %bb14 unwind label %cleanup1

bb10:                                             ; preds = %bb9
  %49 = load atomic ptr, ptr %_5 monotonic, align 8, !noalias !235
  %addr.i.i.i = ptrtoint ptr %49 to i64
  %50 = cmpxchg ptr %_18.i.i.i, i64 0, i64 %addr.i.i.i monotonic monotonic, align 8, !noalias !235
  %_8.sroa.18.0.in.i.i.i.i = extractvalue { i64, i1 } %50, 1
  %_8.sroa.0.0.i.i.i.i = extractvalue { i64, i1 } %50, 0
  %_11.i.i.i = icmp eq i64 %_8.sroa.0.0.i.i.i.i, %addr.i.i.i
  %or.cond.not.not.i.i.i = select i1 %_8.sroa.18.0.in.i.i.i.i, i1 true, i1 %_11.i.i.i
  br i1 %or.cond.not.not.i.i.i, label %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i, label %bb3.i.i.i, !prof !170

bb3.i.i.i:                                        ; preds = %bb10
; invoke core::panicking::panic_fmt
  invoke void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_da75be5cf355a0c62ad51607063f583d, ptr noundef nonnull inttoptr (i64 109 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_33c80814996188919076c1dd2d93215e) #29
          to label %.noexc.i unwind label %cleanup.i86.loopexit.split-lp, !noalias !235

.noexc.i:                                         ; preds = %bb3.i.i.i
  unreachable

_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i: ; preds = %bb10
  %51 = load atomic ptr, ptr %_15 acquire, align 8, !noalias !235
  %52 = icmp eq ptr %51, null
  br i1 %52, label %bb7.i.i.i, label %bb11, !prof !5

bb7.i.i.i:                                        ; preds = %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i
; invoke <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::condvar::Condvar>>::initialize::<<std::sys::sync::condvar::pthread::Condvar>::get::{closure#0}>
  %53 = invoke fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync7condvar7CondvarE10initializeNCNvMNtNtB5_7condvar7pthreadNtB1W_7Condvar3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_15)
          to label %bb11 unwind label %cleanup.i86.loopexit, !noalias !235

cleanup.i86.loopexit:                             ; preds = %bb7.i.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i86

cleanup.i86.loopexit.split-lp:                    ; preds = %bb3.i.i.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i86

cleanup.i86:                                      ; preds = %cleanup.i86.loopexit.split-lp, %cleanup.i86.loopexit
  %lpad.phi = phi { ptr, i32 } [ %lpad.loopexit, %cleanup.i86.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.i86.loopexit.split-lp ]
  %54 = zext i1 %_5.sroa.0.0.off0.i.i66 to i8
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull align 8 %_5, i8 %54) #27
          to label %cleanup1.body unwind label %terminate.i87, !noalias !235

terminate.i87:                                    ; preds = %cleanup.i86
  %55 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !235
  unreachable

bb14:                                             ; preds = %_RNvXsc_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_10MutexGuardjENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils.exit.i81
  %56 = atomicrmw sub ptr %0, i64 1 release, align 8, !noalias !238
  %57 = icmp eq i64 %56, 1
  br i1 %57, label %bb2.i.i92, label %bb15

bb2.i.i92:                                        ; preds = %bb14
  fence acquire
; call <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
  call void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef nonnull readonly align 8 dereferenceable(8) %inner) #28
  br label %bb15

bb15:                                             ; preds = %bb2.i.i92, %bb14
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %inner)
  br label %bb16

bb11:                                             ; preds = %bb7.i.i.i, %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i
  %_0.sroa.0.0.i.i.i = phi ptr [ %51, %_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar6verify.exit.i.i ], [ %53, %bb7.i.i.i ]
  %r.i.i = tail call noundef i32 @pthread_cond_wait(ptr noundef nonnull %_0.sroa.0.0.i.i.i, ptr noundef nonnull %49) #24, !noalias !235
  %58 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !235
  %.not112 = icmp eq i8 %58, 0
  br i1 %.not112, label %bb9, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %bb11
  %59 = zext i1 %_5.sroa.0.0.off0.i.i66 to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !243
  store ptr %_5, ptr %e.i, align 8, !noalias !243
  %60 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %59, ptr %60, align 8, !noalias !243
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f6524b7a65f7681dc4cb2656dc88f418) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !247

cleanup.i:                                        ; preds = %bb2.i
  %61 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %cleanup1.body unwind label %terminate.i, !noalias !247

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %62 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !247
  unreachable

terminate:                                        ; preds = %bb2.i.i, %bb19
  %63 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable

bb19:                                             ; preds = %cleanup.i21, %bb20.thread107
  %eh.lpad-body25106 = phi { ptr, i32 } [ %lpad.thr_comm, %bb20.thread107 ], [ %10, %cleanup.i21 ]
; invoke core::ptr::drop_in_place::<crossbeam_utils::sync::wait_group::WaitGroup>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_(ptr noalias noundef align 8 dereferenceable(8) %self) #27
          to label %common.resume unwind label %terminate
}

; <alloc::sync::Arc<crossbeam_utils::sync::wait_group::Inner>>::drop_slow
; Function Attrs: noinline uwtable
define void @_RNvMsn_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerE9drop_slowBM_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_6 = getelementptr inbounds nuw i8, ptr %_3, i64 16
; invoke core::ptr::drop_in_place::<crossbeam_utils::sync::wait_group::Inner>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEBM_(ptr noalias noundef align 8 dereferenceable(40) %_6)
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
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 56, i64 noundef 8) #24
  br label %bb4

bb1:                                              ; preds = %start
  %_16.i.i3 = icmp eq ptr %_3, inttoptr (i64 -1 to ptr)
  br i1 %_16.i.i3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerRNtNtBL_5alloc6GlobalEEB1l_.exit7, label %bb8.i.i4

bb8.i.i4:                                         ; preds = %bb1
  %_20.i.i5 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %3 = atomicrmw sub ptr %_20.i.i5, i64 1 release, align 8
  %4 = icmp eq i64 %3, 1
  br i1 %4, label %bb1.i.i6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerRNtNtBL_5alloc6GlobalEEB1l_.exit7

bb1.i.i6:                                         ; preds = %bb8.i.i4
  fence acquire
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3, i64 noundef 56, i64 noundef 8) #24
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerRNtNtBL_5alloc6GlobalEEB1l_.exit7

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync4WeakNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerRNtNtBL_5alloc6GlobalEEB1l_.exit7: ; preds = %bb1, %bb8.i.i4, %bb1.i.i6
  ret void

bb4:                                              ; preds = %bb1.i.i, %bb8.i.i, %cleanup
  resume { ptr, i32 } %0
}

; crossbeam_utils::sync::sharded_lock::thread_indices
; Function Attrs: uwtable
define noundef nonnull align 8 ptr @_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices() unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load atomic ptr, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 96) acquire, align 8
  %_3.i = icmp eq ptr %0, null
  br i1 %_3.i, label %_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE11get_or_initNvNvB20_14thread_indices4initEBa_.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %start
; call <crossbeam_utils::sync::once_lock::OnceLock<std::sync::poison::mutex::Mutex<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>::initialize::<crossbeam_utils::sync::sharded_lock::thread_indices::init>
  tail call fastcc void @_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE10initializeNvNvB20_14thread_indices4initEBa_()
  br label %_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE11get_or_initNvNvB20_14thread_indices4initEBa_.exit

_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE11get_or_initNvNvB20_14thread_indices4initEBa_.exit: ; preds = %start, %bb2.i
  ret ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES
}

; <crossbeam_utils::thread::scope::AbortOnPanic as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXNvNtCs8jD91Rl7RDZ_15crossbeam_utils6thread5scopeNtB2_12AbortOnPanicNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull readnone align 1 captures(none) %self) unnamed_addr #1 {
start:
  %0 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_5 = and i64 %0, 9223372036854775807
  %1 = icmp eq i64 %_5, 0
  br i1 %1, label %bb2, label %bb5, !prof !12

bb5:                                              ; preds = %start
; call std::panicking::panic_count::is_zero_slow_path
  %_4 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_4, label %bb2, label %bb1, !prof !12

bb2:                                              ; preds = %start, %bb5
  ret void

bb1:                                              ; preds = %bb5
; call std::process::abort
  tail call void @_RNvNtCs5sEH5CPMdak_3std7process5abort() #25
  unreachable
}

; <<crossbeam_utils::sync::sharded_lock::ShardedLock<_> as core::fmt::Debug>::fmt::LockedPlaceholder as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXNvXs4_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lockINtB8_11ShardedLockpENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtNtB2_17LockedPlaceholderB1l_3fmt(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_779f4c4c227c35122e8522ff6a6f2abf, i64 noundef 8)
  ret i1 %_0
}

; <crossbeam_utils::thread::Scope as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs0_NtCs8jD91Rl7RDZ_15crossbeam_utils6threadNtB5_5ScopeNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_467ef1e0da97d216b212e4a4e865bb86, i64 noundef 12)
  ret i1 %_0
}

; <crossbeam_utils::sync::wait_group::WaitGroup as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_9 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_4 = getelementptr inbounds nuw i8, ptr %_9, i64 32
  %0 = load atomic ptr, ptr %_4 acquire, align 8, !noalias !248
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %start
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %2 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_4), !noalias !248
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %start
  %_0.sroa.0.0.i.i = phi ptr [ %2, %bb7.i.i ], [ %0, %start ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !248
  %3 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !248
  %_6.i.i = and i64 %3, 9223372036854775807
  %4 = icmp eq i64 %_6.i.i, 0
  br i1 %4, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %5 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !248
  %6 = xor i1 %5, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %6, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_9, i64 40
  %7 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !248
  %.not = icmp eq i8 %7, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !251
  store ptr %_4, ptr %e.i, align 8, !noalias !251
  %8 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %8, align 8, !noalias !251
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_509d68a55f7b98fb47e78a4fe26f43b4) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !255

cleanup.i:                                        ; preds = %bb2.i
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %common.resume unwind label %terminate.i, !noalias !255

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !255
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %9, %cleanup.i ], [ %14, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_12 = getelementptr inbounds nuw i8, ptr %_9, i64 48
  %11 = load i64, ptr %_12, align 8, !noundef !26
  %12 = add i64 %11, -1
  store i64 %12, ptr %_12, align 8
  %13 = icmp eq i64 %12, 0
  br i1 %13, label %bb3, label %bb6

bb3:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_7 = getelementptr inbounds nuw i8, ptr %_9, i64 16
; invoke <std::sync::poison::condvar::Condvar>::notify_all
  invoke void @_RNvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB2_7Condvar10notify_all(ptr noundef nonnull align 8 %_7)
          to label %bb6 unwind label %cleanup

cleanup:                                          ; preds = %bb3
  %14 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull %_4, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate

bb6:                                              ; preds = %bb3, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb6
  %15 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %15, 9223372036854775807
  %16 = icmp eq i64 %_7.i.i.i, 0
  br i1 %16, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb6, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %17 = load atomic ptr, ptr %_4 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %17)
  ret void

terminate:                                        ; preds = %cleanup
  %18 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable
}

; <crossbeam_utils::sync::wait_group::WaitGroup as core::clone::Clone>::clone
; Function Attrs: uwtable
define noundef nonnull ptr @_RNvXs1_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_7 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_4 = getelementptr inbounds nuw i8, ptr %_7, i64 32
  %0 = load atomic ptr, ptr %_4 acquire, align 8, !noalias !256
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %start
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %2 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_4), !noalias !256
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %start
  %_0.sroa.0.0.i.i = phi ptr [ %2, %bb7.i.i ], [ %0, %start ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !256
  %3 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !256
  %_6.i.i = and i64 %3, 9223372036854775807
  %4 = icmp eq i64 %_6.i.i, 0
  br i1 %4, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %5 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !256
  %6 = xor i1 %5, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %6, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_7, i64 40
  %7 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !256
  %.not = icmp eq i8 %7, 0
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !259
  store ptr %_4, ptr %e.i, align 8, !noalias !259
  %8 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %8, align 8, !noalias !259
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1d58ea84bbabeff3efbb0146cc2da551) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !263

cleanup.i:                                        ; preds = %bb2.i
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %bb5.i unwind label %terminate.i, !noalias !263

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !263
  unreachable

bb5.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %9

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_10 = getelementptr inbounds nuw i8, ptr %_7, i64 48
  %11 = load i64, ptr %_10, align 8, !noundef !26
  %12 = add i64 %11, 1
  store i64 %12, ptr %_10, align 8
  %13 = atomicrmw add ptr %_7, i64 1 monotonic, align 8
  %_12 = icmp slt i64 %13, 0
  br i1 %_12, label %bb4, label %bb5

bb5:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb5
  %14 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %14, 9223372036854775807
  %15 = icmp eq i64 %_7.i.i.i, 0
  br i1 %15, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb5, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %16 = load atomic ptr, ptr %_4 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %16)
  ret ptr %_7

bb4:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  tail call void @llvm.trap()
  unreachable
}

; <crossbeam_utils::sync::parker::Parker as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs1_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6ParkerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_bd08e1f6e55d704b221670e7013f731d, i64 noundef 13)
  ret i1 %_0
}

; <crossbeam_utils::sync::wait_group::WaitGroup as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_groupNtB5_9WaitGroupNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %_9 = alloca [16 x i8], align 8
  %_12 = load ptr, ptr %self, align 8, !nonnull !26, !noundef !26
  %_6 = getelementptr inbounds nuw i8, ptr %_12, i64 32
  %0 = load atomic ptr, ptr %_6 acquire, align 8, !noalias !264
  %1 = icmp eq ptr %0, null
  br i1 %1, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %start
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %2 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 %_6), !noalias !264
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %start
  %_0.sroa.0.0.i.i = phi ptr [ %2, %bb7.i.i ], [ %0, %start ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !264
  %3 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !264
  %_6.i.i = and i64 %3, 9223372036854775807
  %4 = icmp eq i64 %_6.i.i, 0
  br i1 %4, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %5 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !264
  %6 = xor i1 %5, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %6, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  %_7.i = getelementptr inbounds nuw i8, ptr %_12, i64 40
  %7 = load atomic i8, ptr %_7.i monotonic, align 1, !noalias !264
  %.not = icmp eq i8 %7, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !267
  store ptr %_6, ptr %e.i, align 8, !noalias !267
  %8 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %8, align 8, !noalias !267
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6e7eea196793eb89cef9a8d7b56ad060) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !271

cleanup.i:                                        ; preds = %bb2.i
  %9 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %common.resume unwind label %terminate.i, !noalias !271

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !271
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %9, %cleanup.i ], [ %11, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_9)
; invoke <core::fmt::Formatter>::debug_struct
  invoke void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_9, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_040143ec29e8487e393481894bc3d0d2, i64 noundef 9)
          to label %bb3 unwind label %cleanup

cleanup:                                          ; preds = %bb4, %bb3, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %11 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull %_6, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate

bb3:                                              ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils.exit
  %_15 = getelementptr inbounds nuw i8, ptr %_12, i64 48
; invoke <core::fmt::builders::DebugStruct>::field
  %_7 = invoke noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cf49a750d6fbb7528728207623c220d8, i64 noundef 5, ptr noundef nonnull align 1 %_15, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.6)
          to label %bb4 unwind label %cleanup

bb4:                                              ; preds = %bb3
; invoke <core::fmt::builders::DebugStruct>::finish
  %_0 = invoke noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_7)
          to label %bb5 unwind label %cleanup

bb5:                                              ; preds = %bb4
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb5
  %12 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %12, 9223372036854775807
  %13 = icmp eq i64 %_7.i.i.i, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr %_7.i monotonic, align 1
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEECs8jD91Rl7RDZ_15crossbeam_utils.exit: ; preds = %bb5, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %14 = load atomic ptr, ptr %_6 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %14)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_9)
  ret i1 %_0

terminate:                                        ; preds = %cleanup
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable
}

; <crossbeam_utils::sync::parker::Unparker as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs5_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_8UnparkerNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
; call <core::fmt::Formatter>::pad
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_12266e6d1429d56a93188ba1015940af, i64 noundef 15)
  ret i1 %_0
}

; <u32 as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsW_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_5Debug3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !26
  %_3 = and i32 %_4, 33554432
  %1 = icmp eq i32 %_3, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %_5 = and i32 %_4, 67108864
  %2 = icmp eq i32 %_5, 0
  br i1 %2, label %bb4, label %bb3

bb1:                                              ; preds = %start
; call <u32 as core::fmt::LowerHex>::fmt
  %3 = tail call noundef zeroext i1 @_RNvXsu_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb4:                                              ; preds = %bb2
; call <u32 as core::fmt::Display>::fmt
  %4 = tail call noundef zeroext i1 @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb3:                                              ; preds = %bb2
; call <u32 as core::fmt::UpperHex>::fmt
  %5 = tail call noundef zeroext i1 @_RNvXsw_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb3, %bb1
  %_0.sroa.0.0.in = phi i1 [ %4, %bb4 ], [ %5, %bb3 ], [ %3, %bb1 ]
  ret i1 %_0.sroa.0.0.in
}

; <usize as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !26
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

; <crossbeam_utils::backoff::Backoff as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs_NtCs8jD91Rl7RDZ_15crossbeam_utils7backoffNtB4_7BackoffNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noundef nonnull align 4 %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_11 = alloca [1 x i8], align 1
  %_6 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_6)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_6, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3ce6a1413887fb9dce1532808b933899, i64 noundef 7)
; call <core::fmt::builders::DebugStruct>::field
  %_4 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e8c8888d17bac1081896fdf7761479e9, i64 noundef 4, ptr noundef nonnull align 1 %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.7)
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %_11)
  %_12 = load i32, ptr %self, align 4, !noundef !26
  %0 = icmp ugt i32 %_12, 10
  %1 = zext i1 %0 to i8
  store i8 %1, ptr %_11, align 1
; call <core::fmt::builders::DebugStruct>::field
  %_3 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_38120bd0eb09393889e2fcc8a0887b4a, i64 noundef 12, ptr noundef nonnull align 1 %_11, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.8)
; call <core::fmt::builders::DebugStruct>::finish
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_3)
  call void @llvm.lifetime.end.p0(i64 1, ptr nonnull %_11)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_6)
  ret i1 %_0
}

; <std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtNtCs5sEH5CPMdak_3std4sync6poisonINtB4_11PoisonErrorINtNtB4_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB1r_(ptr noalias readonly align 8 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_8e2410b80645266732854088d21653bc, i64 noundef 11)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; <bool as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsf_NtCsjMrxcFdYDNN_4core3fmtbNtB5_5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #3 {
start:
; call <bool as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <crossbeam_utils::sync::sharded_lock::Registration as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXsh_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lockNtB5_12RegistrationNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [16 x i8], align 8
  %0 = load atomic ptr, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 96) acquire, align 8
  %_3.i.i = icmp eq ptr %0, null
  br i1 %_3.i.i, label %_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit, label %bb2.i.i, !prof !12

bb2.i.i:                                          ; preds = %start
; call <crossbeam_utils::sync::once_lock::OnceLock<std::sync::poison::mutex::Mutex<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>::initialize::<crossbeam_utils::sync::sharded_lock::thread_indices::init>
  tail call fastcc void @_RINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB6_8OnceLockINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexNtNtB8_12sharded_lock13ThreadIndicesEE10initializeNvNvB20_14thread_indices4initEBa_()
  br label %_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit

_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit: ; preds = %start, %bb2.i.i
  %1 = load atomic ptr, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES acquire, align 8, !noalias !272
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb7.i.i, label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, !prof !5

bb7.i.i:                                          ; preds = %_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit
; call <std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>>::initialize::<<std::sys::sync::mutex::pthread::Mutex>::get::{closure#0}>
  %3 = tail call fastcc noundef nonnull align 8 ptr @_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE10initializeNCNvMNtNtB5_5mutex7pthreadNtB1S_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull align 8 @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES), !noalias !272
  br label %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb7.i.i, %_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit
  %_0.sroa.0.0.i.i = phi ptr [ %3, %bb7.i.i ], [ %1, %_RNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices.exit ]
; call <std::sys::pal::unix::sync::mutex::Mutex>::lock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8 %_0.sroa.0.0.i.i), !noalias !272
  %4 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8, !noalias !272
  %_6.i.i = and i64 %4, 9223372036854775807
  %5 = icmp eq i64 %_6.i.i, 0
  br i1 %5, label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_.exit, label %bb6.i.i, !prof !12

bb6.i.i:                                          ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
; call std::panicking::panic_count::is_zero_slow_path
  %6 = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28, !noalias !272
  %7 = xor i1 %6, true
  br label %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_.exit

_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_.exit: ; preds = %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, %bb6.i.i
  %_5.sroa.0.0.off0.i.i = phi i1 [ %7, %bb6.i.i ], [ false, %_RINvMNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB3_7OnceBoxNtNtNtNtNtB7_3pal4unix4sync5mutex5MutexE11get_or_initNCNvMNtNtB5_5mutex7pthreadNtB1T_5Mutex3get0ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i ]
  %8 = load atomic i8, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 8) monotonic, align 8, !noalias !272
  %.not = icmp eq i8 %8, 0
  %_0.sroa.3.0.i.i = zext i1 %_5.sroa.0.0.off0.i.i to i8
  br i1 %.not, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_.exit, label %bb2.i, !prof !12

bb2.i:                                            ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_.exit
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i), !noalias !275
  store ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, ptr %e.i, align 8, !noalias !275
  %9 = getelementptr inbounds nuw i8, ptr %e.i, i64 8
  store i8 %_0.sroa.3.0.i.i, ptr %9, align 8, !noalias !275
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_9734e5216d7378fcd643f75c93ad6161) #25
          to label %unreachable.i unwind label %cleanup.i, !noalias !275

cleanup.i:                                        ; preds = %bb2.i
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::PoisonError<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync6poison11PoisonErrorINtNtBJ_5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEEB20_(ptr noalias noundef nonnull align 8 dereferenceable(16) %e.i) #27
          to label %common.resume unwind label %terminate.i, !noalias !275

unreachable.i:                                    ; preds = %bb2.i
  unreachable

terminate.i:                                      ; preds = %cleanup.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26, !noalias !275
  unreachable

common.resume:                                    ; preds = %cleanup, %cleanup.i
  %common.resume.op = phi { ptr, i32 } [ %10, %cleanup.i ], [ %94, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_.exit: ; preds = %_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_.exit
  %self.val = load i64, ptr %self, align 8, !range !278, !alias.scope !279, !noalias !282, !noundef !26
  tail call void @llvm.experimental.noalias.scope.decl(metadata !285)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !288)
  %hash_builder.val.i.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 72), align 8, !alias.scope !291, !noundef !26
  %hash_builder.val1.i.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 80), align 8, !alias.scope !291, !noundef !26
  %12 = xor i64 %hash_builder.val.i.i, 8317987319222330741
  %13 = xor i64 %hash_builder.val1.i.i, 7237128888997146477
  %14 = xor i64 %hash_builder.val.i.i, 7816392313619706465
  %15 = xor i64 %self.val, %hash_builder.val1.i.i
  %16 = xor i64 %15, 8387220255154660723
  %_5.i41.i.i.i.i.i.i.i.i = add i64 %16, %14
  %17 = tail call noundef i64 @llvm.fshl.i64(i64 %16, i64 %16, i64 16)
  %18 = xor i64 %_5.i41.i.i.i.i.i.i.i.i, %17
  %_2.i38.i.i.i.i.i.i.i.i = add i64 %13, %12
  %19 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i38.i.i.i.i.i.i.i.i, i64 %_2.i38.i.i.i.i.i.i.i.i, i64 32)
  %_19.i43.i.i.i.i.i.i.i.i = add i64 %18, %19
  %20 = xor i64 %_19.i43.i.i.i.i.i.i.i.i, %self.val
  %21 = tail call noundef i64 @llvm.fshl.i64(i64 %13, i64 %13, i64 13)
  %22 = xor i64 %_2.i38.i.i.i.i.i.i.i.i, %21
  %_16.i42.i.i.i.i.i.i.i.i = add i64 %22, %_5.i41.i.i.i.i.i.i.i.i
  %23 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i42.i.i.i.i.i.i.i.i, i64 %_16.i42.i.i.i.i.i.i.i.i, i64 32)
  %24 = tail call noundef i64 @llvm.fshl.i64(i64 %18, i64 %18, i64 21)
  %25 = tail call noundef i64 @llvm.fshl.i64(i64 %22, i64 %22, i64 17)
  %26 = xor i64 %_16.i42.i.i.i.i.i.i.i.i, %25
  %27 = xor i64 %_19.i43.i.i.i.i.i.i.i.i, %24
  %28 = xor i64 %27, 576460752303423488
  %_2.i.i.i.i.i.i = add i64 %20, %26
  %_5.i.i.i4.i.i.i = add i64 %28, %23
  %29 = tail call noundef i64 @llvm.fshl.i64(i64 %26, i64 %26, i64 13)
  %30 = xor i64 %_2.i.i.i.i.i.i, %29
  %31 = tail call noundef i64 @llvm.fshl.i64(i64 %28, i64 %28, i64 16)
  %32 = xor i64 %31, %_5.i.i.i4.i.i.i
  %33 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i.i.i.i.i.i, i64 %_2.i.i.i.i.i.i, i64 32)
  %_16.i.i.i.i.i.i = add i64 %_5.i.i.i4.i.i.i, %30
  %_19.i.i.i.i.i.i = add i64 %32, %33
  %34 = tail call noundef i64 @llvm.fshl.i64(i64 %30, i64 %30, i64 17)
  %35 = xor i64 %_16.i.i.i.i.i.i, %34
  %36 = tail call noundef i64 @llvm.fshl.i64(i64 %32, i64 %32, i64 21)
  %37 = xor i64 %36, %_19.i.i.i.i.i.i
  %38 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i.i.i.i.i.i, i64 %_16.i.i.i.i.i.i, i64 32)
  %39 = xor i64 %_19.i.i.i.i.i.i, 576460752303423488
  %40 = xor i64 %38, 255
  %_2.i3.i.i.i.i.i = add i64 %39, %35
  %_5.i6.i.i.i.i.i = add i64 %37, %40
  %41 = tail call noundef i64 @llvm.fshl.i64(i64 %35, i64 %35, i64 13)
  %42 = xor i64 %_2.i3.i.i.i.i.i, %41
  %43 = tail call noundef i64 @llvm.fshl.i64(i64 %37, i64 %37, i64 16)
  %44 = xor i64 %43, %_5.i6.i.i.i.i.i
  %45 = tail call noundef i64 @llvm.fshl.i64(i64 %_2.i3.i.i.i.i.i, i64 %_2.i3.i.i.i.i.i, i64 32)
  %_16.i7.i.i.i.i.i = add i64 %42, %_5.i6.i.i.i.i.i
  %_19.i8.i.i.i.i.i = add i64 %44, %45
  %46 = tail call noundef i64 @llvm.fshl.i64(i64 %42, i64 %42, i64 17)
  %47 = xor i64 %_16.i7.i.i.i.i.i, %46
  %48 = tail call noundef i64 @llvm.fshl.i64(i64 %44, i64 %44, i64 21)
  %49 = xor i64 %48, %_19.i8.i.i.i.i.i
  %50 = tail call noundef i64 @llvm.fshl.i64(i64 %_16.i7.i.i.i.i.i, i64 %_16.i7.i.i.i.i.i, i64 32)
  %_30.i.i.i.i.i.i = add i64 %47, %_19.i8.i.i.i.i.i
  %_33.i.i.i.i.i.i = add i64 %49, %50
  %51 = tail call noundef i64 @llvm.fshl.i64(i64 %47, i64 %47, i64 13)
  %52 = xor i64 %51, %_30.i.i.i.i.i.i
  %53 = tail call noundef i64 @llvm.fshl.i64(i64 %49, i64 %49, i64 16)
  %54 = xor i64 %53, %_33.i.i.i.i.i.i
  %55 = tail call noundef i64 @llvm.fshl.i64(i64 %_30.i.i.i.i.i.i, i64 %_30.i.i.i.i.i.i, i64 32)
  %_44.i.i.i.i.i.i = add i64 %52, %_33.i.i.i.i.i.i
  %_47.i.i.i.i.i.i = add i64 %54, %55
  %56 = tail call noundef i64 @llvm.fshl.i64(i64 %52, i64 %52, i64 17)
  %57 = xor i64 %56, %_44.i.i.i.i.i.i
  %58 = tail call noundef i64 @llvm.fshl.i64(i64 %54, i64 %54, i64 21)
  %59 = xor i64 %58, %_47.i.i.i.i.i.i
  %60 = tail call noundef i64 @llvm.fshl.i64(i64 %_44.i.i.i.i.i.i, i64 %_44.i.i.i.i.i.i, i64 32)
  %_58.i.i.i.i.i.i = add i64 %57, %_47.i.i.i.i.i.i
  %_61.i.i.i.i.i.i = add i64 %59, %60
  %61 = tail call noundef i64 @llvm.fshl.i64(i64 %57, i64 %57, i64 13)
  %62 = xor i64 %61, %_58.i.i.i.i.i.i
  %63 = tail call noundef i64 @llvm.fshl.i64(i64 %59, i64 %59, i64 16)
  %64 = xor i64 %63, %_61.i.i.i.i.i.i
  %_72.i.i.i.i.i.i = add i64 %62, %_61.i.i.i.i.i.i
  %65 = tail call noundef i64 @llvm.fshl.i64(i64 %62, i64 %62, i64 17)
  %66 = tail call noundef i64 @llvm.fshl.i64(i64 %64, i64 %64, i64 21)
  %67 = tail call noundef i64 @llvm.fshl.i64(i64 %_72.i.i.i.i.i.i, i64 %_72.i.i.i.i.i.i, i64 32)
  %68 = xor i64 %66, %65
  %69 = xor i64 %68, %67
  %_0.i.i.i.i.i = xor i64 %69, %_72.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !292)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !295)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !298)
  %_24.i.i.i.i.i = lshr i64 %_0.i.i.i.i.i, 57
  %_25.i.i.i.i.i = trunc nuw nsw i64 %_24.i.i.i.i.i to i8
  %_29.i.i.i.i.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 48), align 8, !alias.scope !301, !noalias !302, !noundef !26
  %_32.i.i.i.i.i = load ptr, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 40), align 8, !alias.scope !301, !noalias !302, !nonnull !26, !noundef !26
  %70 = insertelement <1 x i8> poison, i8 %_25.i.i.i.i.i, i64 0
  %71 = shufflevector <1 x i8> %70, <1 x i8> poison, <8 x i32> zeroinitializer
  %invariant.gep.i.i.i.i = getelementptr i8, ptr %_32.i.i.i.i.i, i64 -16
  br label %bb1.i.i.i.i.i

bb1.i.i.i.i.i:                                    ; preds = %bb16.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_.exit
  %probe_seq.sroa.9.0.i.i.i.i.i = phi i64 [ 0, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_.exit ], [ %80, %bb16.i.i.i.i.i ]
  %hash.pn.i.i.i.i = phi i64 [ %_0.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_.exit ], [ %81, %bb16.i.i.i.i.i ]
  %probe_seq.sroa.0.0.i.i.i.i.i = and i64 %hash.pn.i.i.i.i, %_29.i.i.i.i.i
  %_30.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_32.i.i.i.i.i, i64 %probe_seq.sroa.0.0.i.i.i.i.i
  %tmp.sroa.0.0.copyload.i.i.i.i.i.i = load <8 x i8>, ptr %_30.i.i.i.i.i, align 1, !noalias !304
  %72 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i.i, %71
  %73 = sext <8 x i1> %72 to <8 x i8>
  %74 = bitcast <8 x i8> %73 to i64
  %_36.i.i.i.i.i = and i64 %74, -9187201950435737472
  %.not.i.not10.i.i.i.i = icmp eq i64 %_36.i.i.i.i.i, 0
  br i1 %.not.i.not10.i.i.i.i, label %bb9.i.i.i.i.i, label %bb8.i.i.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.i.i, %bb13.i.i.i.i.i
  %iter.sroa.0.0.i11.i.i.i.i = phi i64 [ %_53.i.i.i.i.i, %bb13.i.i.i.i.i ], [ %_36.i.i.i.i.i, %bb1.i.i.i.i.i ]
  %75 = tail call range(i64 1, 65) i64 @llvm.cttz.i64(i64 %iter.sroa.0.0.i11.i.i.i.i, i1 true)
  %_4315.i.i.i.i.i = lshr i64 %75, 3
  %_15.i.i.i.i.i = add i64 %_4315.i.i.i.i.i, %probe_seq.sroa.0.0.i.i.i.i.i
  %index6.i.i.i.i.i = and i64 %_15.i.i.i.i.i, %_29.i.i.i.i.i
  %_18.i.i.i.i.i = sub nsw i64 0, %index6.i.i.i.i.i
  %gep.i.i.i.i = getelementptr { i64, i64 }, ptr %invariant.gep.i.i.i.i, i64 %_18.i.i.i.i.i
  %.val.i.i.i.i.i = load i64, ptr %gep.i.i.i.i, align 8, !range !278, !noalias !309, !noundef !26
  %_0.i.i.i.i.i.i.i.i = icmp eq i64 %self.val, %.val.i.i.i.i.i
  br i1 %_0.i.i.i.i.i.i.i.i, label %bb4.i.i.i, label %bb13.i.i.i.i.i, !prof !12

bb9.i.i.i.i.i:                                    ; preds = %bb13.i.i.i.i.i, %bb1.i.i.i.i.i
  %76 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i.i, splat (i8 -1)
  %77 = bitcast <8 x i1> %76 to i8
  %78 = icmp eq i8 %77, 0
  br i1 %78, label %bb16.i.i.i.i.i, label %bb7, !prof !5

bb13.i.i.i.i.i:                                   ; preds = %bb8.i.i.i.i.i
  %79 = add i64 %iter.sroa.0.0.i11.i.i.i.i, -1
  %_53.i.i.i.i.i = and i64 %79, %iter.sroa.0.0.i11.i.i.i.i
  %.not.i.not.i.i.i.i = icmp eq i64 %_53.i.i.i.i.i, 0
  br i1 %.not.i.not.i.i.i.i, label %bb9.i.i.i.i.i, label %bb8.i.i.i.i.i

bb16.i.i.i.i.i:                                   ; preds = %bb9.i.i.i.i.i
  %80 = add i64 %probe_seq.sroa.9.0.i.i.i.i.i, 8
  %81 = add i64 %probe_seq.sroa.0.0.i.i.i.i.i, %80
  br label %bb1.i.i.i.i.i

bb4.i.i.i:                                        ; preds = %bb8.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !312)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !315)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !318)
  %_6.i.i.i.i.i.i = add nsw i64 %index6.i.i.i.i.i, -8
  %index_before.i.i.i.i.i.i = and i64 %_6.i.i.i.i.i.i, %_29.i.i.i.i.i
  %_19.i.i.i.i2.i.i = getelementptr inbounds nuw i8, ptr %_32.i.i.i.i.i, i64 %index_before.i.i.i.i.i.i
  %tmp.sroa.0.0.copyload.i.i.i.i.i.i.i.i = load <8 x i8>, ptr %_19.i.i.i.i2.i.i, align 1, !noalias !321
  %82 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i.i.i.i.i.i.i, splat (i8 -1)
  %83 = sext <8 x i1> %82 to <8 x i8>
  %84 = bitcast <8 x i8> %83 to i64
  %_24.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_32.i.i.i.i.i, i64 %index6.i.i.i.i.i
  %tmp.sroa.0.0.copyload.i.i6.i.i.i.i.i.i = load <8 x i8>, ptr %_24.i.i.i.i.i.i, align 1, !noalias !327
  %85 = icmp eq <8 x i8> %tmp.sroa.0.0.copyload.i.i6.i.i.i.i.i.i, splat (i8 -1)
  %86 = sext <8 x i1> %85 to <8 x i8>
  %87 = bitcast <8 x i8> %86 to i64
  %88 = tail call range(i64 0, 65) i64 @llvm.ctlz.i64(i64 %84, i1 false)
  %_173.i.i.i.i.i.i = lshr i64 %88, 3
  %89 = tail call range(i64 0, 65) i64 @llvm.cttz.i64(i64 %87, i1 false)
  %_184.i.i.i.i.i.i = lshr i64 %89, 3
  %_16.i.i.i.i3.i.i = add nuw nsw i64 %_184.i.i.i.i.i.i, %_173.i.i.i.i.i.i
  %_15.i.i.i.i.i.i = icmp samesign ugt i64 %_16.i.i.i.i3.i.i, 7
  br i1 %_15.i.i.i.i.i.i, label %_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i, label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i
  %90 = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 56), align 8, !alias.scope !332, !noalias !333, !noundef !26
  %91 = add i64 %90, 1
  store i64 %91, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 56), align 8, !alias.scope !332, !noalias !333
  br label %_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i

_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i: ; preds = %bb2.i.i.i.i.i.i, %bb4.i.i.i
  %ctrl.sroa.0.0.i.i.i.i.i.i = phi i8 [ -1, %bb2.i.i.i.i.i.i ], [ -128, %bb4.i.i.i ]
  store i8 %ctrl.sroa.0.0.i.i.i.i.i.i, ptr %_24.i.i.i.i.i.i, align 1, !noalias !334
  %_44.i.i.i.i4.i.i = getelementptr i8, ptr %_19.i.i.i.i2.i.i, i64 8
  store i8 %ctrl.sroa.0.0.i.i.i.i.i.i, ptr %_44.i.i.i.i4.i.i, align 1, !noalias !334
  %92 = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 64), align 8, !alias.scope !332, !noalias !333, !noundef !26
  %93 = add i64 %92, -1
  store i64 %93, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 64), align 8, !alias.scope !332, !noalias !333
  br label %bb7

cleanup:                                          ; preds = %bb1.i
  %94 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<std::sync::poison::mutex::MutexGuard<crossbeam_utils::sync::sharded_lock::ThreadIndices>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_(ptr nonnull @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i8 %_0.sroa.3.0.i.i) #27
          to label %common.resume unwind label %terminate

bb7:                                              ; preds = %bb9.i.i.i.i.i, %_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils.exit.i
  %95 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load i64, ptr %95, align 8, !noundef !26
  tail call void @llvm.experimental.noalias.scope.decl(metadata !335)
  %len.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 32), align 8, !alias.scope !335, !noundef !26
  %self1.i = load i64, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 16), align 8, !range !146, !alias.scope !335, !noundef !26
  %_4.i = icmp eq i64 %len.i, %self1.i
  br i1 %_4.i, label %bb1.i, label %bb8

bb1.i:                                            ; preds = %bb7
; invoke <alloc::raw_vec::RawVec<usize>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecjE8grow_oneCs8jD91Rl7RDZ_15crossbeam_utils(ptr noalias noundef nonnull align 8 dereferenceable(24) getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 16))
          to label %bb8 unwind label %cleanup

bb8:                                              ; preds = %bb7, %bb1.i
  %_14.i = load ptr, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 24), align 8, !alias.scope !335, !nonnull !26, !noundef !26
  %end.i = getelementptr inbounds nuw i64, ptr %_14.i, i64 %len.i
  store i64 %_8, ptr %end.i, align 8, !noalias !335
  %96 = add i64 %len.i, 1
  store i64 %96, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 32), align 8, !alias.scope !335
  br i1 %_5.sroa.0.0.off0.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb1.i.i.i

bb1.i.i.i:                                        ; preds = %bb8
  %97 = load atomic i64, ptr @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count18GLOBAL_PANIC_COUNT monotonic, align 8
  %_7.i.i.i = and i64 %97, 9223372036854775807
  %98 = icmp eq i64 %_7.i.i.i, 0
  br i1 %98, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb6.i.i.i, !prof !12

bb6.i.i.i:                                        ; preds = %bb1.i.i.i
; call std::panicking::panic_count::is_zero_slow_path
  %_6.i.i.i = tail call noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() #28
  br i1 %_6.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb6.i.i.i
  store atomic i8 1, ptr getelementptr inbounds nuw (i8, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES, i64 8) monotonic, align 8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEEB1H_.exit: ; preds = %bb8, %bb1.i.i.i, %bb6.i.i.i, %bb2.i.i.i
  %99 = load atomic ptr, ptr @_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices14THREAD_INDICES monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %99)
  ret void

terminate:                                        ; preds = %cleanup
  %100 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #26
  unreachable
}

; <core::cell::Cell<u32> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXsu_NtCsjMrxcFdYDNN_4core3fmtINtNtB7_4cell4CellmENtB5_5Debug3fmtCs8jD91Rl7RDZ_15crossbeam_utils(ptr noundef nonnull readonly align 4 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_8 = alloca [4 x i8], align 4
  %_5 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_5, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_30f876edaa9ab0a5b89f19bb4d7a0800, i64 noundef 4)
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_8)
  %0 = load i32, ptr %self, align 4, !noundef !26
  store i32 %0, ptr %_8, align 4
; call <core::fmt::builders::DebugStruct>::field
  %_3 = call noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef nonnull align 8 dereferenceable(16) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_2fce15d1a77c62e67d5eacceaee24476, i64 noundef 5, ptr noundef nonnull align 1 %_8, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.9)
; call <core::fmt::builders::DebugStruct>::finish
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef nonnull align 8 dereferenceable(16) %_3)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_8)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  ret i1 %_0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #9

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #2

; <std::sys::sync::condvar::pthread::Condvar>::wait_timeout
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthreadNtB2_7Condvar12wait_timeout(ptr noundef nonnull align 8, ptr noundef nonnull align 8, i64 noundef, i32 noundef range(i32 0, 1000000000)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #10

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #11

; <std::sys::sync::once::queue::Once>::call
; Function Attrs: cold uwtable
declare void @_RNvMNtNtNtNtCs5sEH5CPMdak_3std3sys4sync4once5queueNtB2_4Once4call(ptr noundef nonnull align 8, i1 noundef zeroext, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(40), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.fshl.i64(i64, i64, i64) #12

; <std::sys::sync::mutex::pthread::Mutex as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs_NtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthreadNtB4_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #1

; <std::sys::pal::unix::sync::mutex::Mutex as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs2_NtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB5_5MutexNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noundef nonnull align 8) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #13

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #13

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare nonnull ptr @llvm.threadlocal.address.p0(ptr nonnull) #12

; <std::sys::pal::unix::sync::mutex::Mutex>::init
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4init(ptr noundef nonnull align 8) unnamed_addr #1

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #14

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #15

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #2

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #16

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #13

; std::panicking::panic_count::is_zero_slow_path
; Function Attrs: cold noinline uwtable
declare noundef zeroext i1 @_RNvNtNtCs5sEH5CPMdak_3std9panicking11panic_count17is_zero_slow_path() unnamed_addr #5

; Function Attrs: nounwind uwtable
declare noundef i32 @pthread_cond_wait(ptr noundef, ptr noundef) unnamed_addr #2

; <std::time::Instant>::now
; Function Attrs: uwtable
declare { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant3now() unnamed_addr #1

; <std::time::Instant>::checked_add
; Function Attrs: uwtable
declare { i64, i32 } @_RNvMNtCs5sEH5CPMdak_3std4timeNtB2_7Instant11checked_add(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16), i64 noundef, i32 noundef range(i32 0, 1000000000)) unnamed_addr #1

; <std::thread::builder::Builder>::name
; Function Attrs: uwtable
declare void @_RNvMNtNtCs5sEH5CPMdak_3std6thread7builderNtB2_7Builder4name(ptr dead_on_unwind noalias noundef writable sret([48 x i8]) align 8 captures(none) dereferenceable(48), ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(48), ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(24)) unnamed_addr #1

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #17

; <std::sys::pal::unix::sync::mutex::Mutex>::lock
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex4lock(ptr noundef nonnull align 8) unnamed_addr #1

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <std::time::Instant as core::ops::arith::Sub>::sub
; Function Attrs: uwtable
declare { i64, i32 } @_RNvXs3_NtCs5sEH5CPMdak_3std4timeNtB5_7InstantNtNtNtCsjMrxcFdYDNN_4core3ops5arith3Sub3sub(i64 noundef, i32 noundef range(i32 0, 1000000000), i64 noundef, i32 noundef range(i32 0, 1000000000)) unnamed_addr #1

; core::panicking::assert_failed::<usize, usize>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedjjEB4_(i8 noundef range(i8 0, 3), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noundef, ptr, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #18

; <std::sync::poison::condvar::Condvar>::notify_one
; Function Attrs: uwtable
declare void @_RNvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB2_7Condvar10notify_one(ptr noundef nonnull align 8) unnamed_addr #1

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #19

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.cttz.i64(i64, i1 immarg) #12

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.ctlz.i64(i64, i1 immarg) #12

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #17

; std::sys::random::hashmap_random_keys
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCs5sEH5CPMdak_3std3sys6random19hashmap_random_keys() unnamed_addr #1

; std::process::abort
; Function Attrs: cold noreturn uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std7process5abort() unnamed_addr #20

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::fmt::Formatter>::pad
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter3pad(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <std::sync::poison::condvar::Condvar>::notify_all
; Function Attrs: uwtable
declare void @_RNvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB2_7Condvar10notify_all(ptr noundef nonnull align 8) unnamed_addr #1

; <core::fmt::Formatter>::debug_struct
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::fmt::builders::DebugStruct>::field
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct5field(ptr noalias noundef align 8 dereferenceable(16), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #1

; <core::fmt::builders::DebugStruct>::finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct6finish(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #1

; Function Attrs: nounwind uwtable
declare noundef i32 @pthread_cond_destroy(ptr noundef) unnamed_addr #2

; <u32 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <u32 as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsw_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <u32 as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsu_NtNtCsjMrxcFdYDNN_4core3fmt3nummNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <usize as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <usize as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs6_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <core::fmt::builders::DebugStruct>::finish_non_exhaustive
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #1

; <std::sys::pal::unix::sync::mutex::Mutex>::unlock
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8) unnamed_addr #1

; <bool as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #21

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #22

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #23

attributes #0 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #9 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #10 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #11 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #13 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #17 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #20 = { cold noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #21 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #22 = { nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #23 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #24 = { nounwind }
attributes #25 = { noreturn }
attributes #26 = { cold noreturn nounwind }
attributes #27 = { cold }
attributes #28 = { noinline }
attributes #29 = { noinline noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils: %x"}
!4 = distinct !{!4, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutex5MutexE3newCs8jD91Rl7RDZ_15crossbeam_utils"}
!5 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync7condvar7CondvarE3newCs8jD91Rl7RDZ_15crossbeam_utils: %x"}
!8 = distinct !{!8, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync7condvar7CondvarE3newCs8jD91Rl7RDZ_15crossbeam_utils"}
!9 = !{!10}
!10 = distinct !{!10, !11, !"_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB15_8OnceLockINtNtNtB8_6poison5mutex5MutexNtNtB17_12sharded_lock13ThreadIndicesEE10initializeNvNvB2E_14thread_indices4initE0EB19_: %f"}
!11 = distinct !{!11, !"_RINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB6_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB15_8OnceLockINtNtNtB8_6poison5mutex5MutexNtNtB17_12sharded_lock13ThreadIndicesEE10initializeNvNvB2E_14thread_indices4initE0EB19_"}
!12 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!13 = !{!14}
!14 = distinct !{!14, !15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!15 = distinct !{!15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils"}
!16 = !{!17}
!17 = distinct !{!17, !18, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!18 = distinct !{!18, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!19 = !{!17, !14, !20, !22, !24}
!20 = distinct !{!20, !21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!21 = distinct !{!21, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECs8jD91Rl7RDZ_15crossbeam_utils"}
!22 = distinct !{!22, !23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!23 = distinct !{!23, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexuEECs8jD91Rl7RDZ_15crossbeam_utils"}
!24 = distinct !{!24, !25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEBM_: %_1"}
!25 = distinct !{!25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEBM_"}
!26 = !{}
!27 = !{!17, !14}
!28 = !{!29}
!29 = distinct !{!29, !30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!30 = distinct !{!30, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils"}
!31 = !{!32}
!32 = distinct !{!32, !33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!33 = distinct !{!33, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils"}
!34 = !{!35}
!35 = distinct !{!35, !36, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!36 = distinct !{!36, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils"}
!37 = !{!38}
!38 = distinct !{!38, !39, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!39 = distinct !{!39, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!40 = !{!38, !35, !32, !29, !24}
!41 = !{!38, !35, !32, !29}
!42 = !{i64 8}
!43 = !{i8 0, i8 2}
!44 = !{!45}
!45 = distinct !{!45, !46, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!46 = distinct !{!46, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!47 = !{!48}
!48 = distinct !{!48, !49, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!49 = distinct !{!49, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvar7CondvarECs8jD91Rl7RDZ_15crossbeam_utils"}
!50 = !{!51}
!51 = distinct !{!51, !52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!52 = distinct !{!52, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils"}
!53 = !{!54}
!54 = distinct !{!54, !55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!55 = distinct !{!55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils"}
!56 = !{!57}
!57 = distinct !{!57, !58, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!58 = distinct !{!58, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!59 = !{!57, !54, !51, !48}
!60 = !{!61}
!61 = distinct !{!61, !62, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!62 = distinct !{!62, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync5mutex5MutexEECs8jD91Rl7RDZ_15crossbeam_utils"}
!63 = !{!64}
!64 = distinct !{!64, !65, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!65 = distinct !{!65, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync5mutex5MutexENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!66 = !{!64, !61, !67, !69}
!67 = distinct !{!67, !68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!68 = distinct !{!68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync5mutex7pthread5MutexECs8jD91Rl7RDZ_15crossbeam_utils"}
!69 = distinct !{!69, !70, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexjEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!70 = distinct !{!70, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex5MutexjEECs8jD91Rl7RDZ_15crossbeam_utils"}
!71 = !{!64, !61}
!72 = !{!73}
!73 = distinct !{!73, !74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!74 = distinct !{!74, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!75 = !{!76}
!76 = distinct !{!76, !77, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!77 = distinct !{!77, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!78 = !{!76, !73}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!81 = distinct !{!81, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!82 = !{!83}
!83 = distinct !{!83, !84, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!84 = distinct !{!84, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!85 = !{!83, !80}
!86 = !{!87}
!87 = distinct !{!87, !88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!88 = distinct !{!88, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtNtCs5sEH5CPMdak_3std3sys4sync7condvar7pthread7CondvarECs8jD91Rl7RDZ_15crossbeam_utils"}
!89 = !{!90}
!90 = distinct !{!90, !91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!91 = distinct !{!91, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtCs5sEH5CPMdak_3std3sys4sync8once_box7OnceBoxNtNtNtNtNtBN_3pal4unix4sync7condvar7CondvarEECs8jD91Rl7RDZ_15crossbeam_utils"}
!92 = !{!93}
!93 = distinct !{!93, !94, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!94 = distinct !{!94, !"_RNvXs1_NtNtNtCs5sEH5CPMdak_3std3sys4sync8once_boxINtB5_7OnceBoxNtNtNtNtNtB9_3pal4unix4sync7condvar7CondvarENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!95 = !{!93, !90, !87}
!96 = !{!97, !99, !101, !103, !105, !107}
!97 = distinct !{!97, !98, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils: %i"}
!98 = distinct !{!98, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils"}
!99 = distinct !{!99, !100, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs8jD91Rl7RDZ_15crossbeam_utils: %__rust_std_internal_init"}
!100 = distinct !{!100, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs8jD91Rl7RDZ_15crossbeam_utils"}
!101 = distinct !{!101, !102, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils: argument 0"}
!102 = distinct !{!102, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils"}
!103 = distinct !{!103, !104, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!104 = distinct !{!104, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs8jD91Rl7RDZ_15crossbeam_utils"}
!105 = distinct !{!105, !106, !"_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4init: %_0"}
!106 = distinct !{!106, !"_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4init"}
!107 = distinct !{!107, !108, !"_RNvYNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4initINtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceBa_: %_0"}
!108 = distinct !{!108, !"_RNvYNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4initINtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceBa_"}
!109 = !{!103, !105, !107}
!110 = !{!111, !103, !105, !107}
!111 = distinct !{!111, !112, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils: argument 0"}
!112 = distinct !{!112, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1a_8OnceLockINtNtNtBd_6poison5mutex5MutexNtNtB1c_12sharded_lock13ThreadIndicesEE10initializeNvNvB2J_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_: argument 0"}
!115 = distinct !{!115, !"_RNvYNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtBb_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB1a_8OnceLockINtNtNtBd_6poison5mutex5MutexNtNtB1c_12sharded_lock13ThreadIndicesEE10initializeNvNvB2J_14thread_indices4initE0E0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTRNtBb_9OnceStateEE9call_onceB1e_"}
!116 = !{!117}
!117 = distinct !{!117, !118, !"_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB17_8OnceLockINtNtNtBa_6poison5mutex5MutexNtNtB19_12sharded_lock13ThreadIndicesEE10initializeNvNvB2G_14thread_indices4initE0E0B1b_: %_1"}
!118 = distinct !{!118, !"_RNCINvMs0_NtNtCs5sEH5CPMdak_3std4sync4onceNtB8_4Once9call_onceNCINvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync9once_lockINtB17_8OnceLockINtNtNtBa_6poison5mutex5MutexNtNtB19_12sharded_lock13ThreadIndicesEE10initializeNvNvB2G_14thread_indices4initE0E0B1b_"}
!119 = !{!117, !114}
!120 = !{!121, !123, !125, !127, !129, !131, !117, !114}
!121 = distinct !{!121, !122, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils: %i"}
!122 = distinct !{!122, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE11get_or_initNvNvNvMNtNtBe_4hash6randomNtB2d_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils"}
!123 = distinct !{!123, !124, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs8jD91Rl7RDZ_15crossbeam_utils: %__rust_std_internal_init"}
!124 = distinct !{!124, !"_RNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtB8_11RandomState3new4KEYS0s_0Cs8jD91Rl7RDZ_15crossbeam_utils"}
!125 = distinct !{!125, !126, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils: argument 0"}
!126 = distinct !{!126, !"_RNvYNCNKNvNvMNtNtCs5sEH5CPMdak_3std4hash6randomNtBb_11RandomState3new4KEYS0s_0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceTINtNtB1l_6option6OptionQIB20_INtNtB1l_4cell4CellTyyEEEEEE9call_onceCs8jD91Rl7RDZ_15crossbeam_utils"}
!127 = distinct !{!127, !128, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!128 = distinct !{!128, !"_RINvMs2_NtNtCs5sEH5CPMdak_3std6thread5localINtB6_8LocalKeyINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEE8try_withNCNvMNtNtBa_4hash6randomNtB1M_11RandomState3new0B25_ECs8jD91Rl7RDZ_15crossbeam_utils"}
!129 = distinct !{!129, !130, !"_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4init: %_0"}
!130 = distinct !{!130, !"_RNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4init"}
!131 = distinct !{!131, !132, !"_RNvYNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4initINtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceBa_: %_0"}
!132 = distinct !{!132, !"_RNvYNvNvNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock14thread_indices4initINtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_onceBa_"}
!133 = !{!127, !129, !131, !117, !114}
!134 = !{!135, !127, !129, !131, !117, !114}
!135 = distinct !{!135, !136, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils: argument 0"}
!136 = distinct !{!136, !"_RINvMs0_NtNtNtNtCs5sEH5CPMdak_3std3sys12thread_local6native4lazyINtB6_7StorageINtNtCsjMrxcFdYDNN_4core4cell4CellTyyEEzE16get_or_init_slowNvNvNvMNtNtBe_4hash6randomNtB2i_11RandomState3new4KEYS27___rust_std_internal_init_fnECs8jD91Rl7RDZ_15crossbeam_utils"}
!137 = !{!138}
!138 = distinct !{!138, !139, !"_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker13park_deadline: %self"}
!139 = distinct !{!139, !"_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker13park_deadline"}
!140 = !{!141}
!141 = distinct !{!141, !142, !"_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker4park: %self"}
!142 = distinct !{!142, !"_RNvMs0_NtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parkerNtB5_6Parker4park"}
!143 = !{!144}
!144 = distinct !{!144, !145, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEE3newB16_: %x"}
!145 = distinct !{!145, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync6parker5InnerEE3newB16_"}
!146 = !{i64 0, i64 -9223372036854775808}
!147 = !{!"branch_weights", i32 2002, i32 2000}
!148 = !{i64 0, i64 2}
!149 = !{i64 0, i64 -9223372036854775807}
!150 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000}
!151 = !{!152}
!152 = distinct !{!152, !153, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!153 = distinct !{!153, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!154 = !{!155, !157}
!155 = distinct !{!155, !156, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!156 = distinct !{!156, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!157 = distinct !{!157, !156, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!158 = !{!155}
!159 = !{!160}
!160 = distinct !{!160, !161, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!161 = distinct !{!161, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexuE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!162 = !{!163, !165}
!163 = distinct !{!163, !164, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!164 = distinct !{!164, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!165 = distinct !{!165, !164, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!166 = !{!163}
!167 = !{!168}
!168 = distinct !{!168, !169, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar4waituECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!169 = distinct !{!169, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar4waituECs8jD91Rl7RDZ_15crossbeam_utils"}
!170 = !{!"branch_weights", i32 4001, i32 1}
!171 = !{!172, !174}
!172 = distinct !{!172, !173, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!173 = distinct !{!173, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!174 = distinct !{!174, !173, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!175 = !{!172}
!176 = !{!"branch_weights", i32 4000, i32 1}
!177 = !{!178}
!178 = distinct !{!178, !179, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar12wait_timeoutuECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!179 = distinct !{!179, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar12wait_timeoutuECs8jD91Rl7RDZ_15crossbeam_utils"}
!180 = !{!181, !183}
!181 = distinct !{!181, !182, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBP_17WaitTimeoutResultEINtBN_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %t"}
!182 = distinct !{!182, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBP_17WaitTimeoutResultEINtBN_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!183 = distinct !{!183, !182, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultTINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuarduENtBP_17WaitTimeoutResultEINtBN_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!184 = !{!181}
!185 = !{!186}
!186 = distinct !{!186, !187, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEE3newB16_: %x"}
!187 = distinct !{!187, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtB4_4sync8ArcInnerNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEE3newB16_"}
!188 = !{!189}
!189 = distinct !{!189, !190, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!190 = distinct !{!190, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!191 = !{!192, !194}
!192 = distinct !{!192, !193, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!193 = distinct !{!193, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!194 = distinct !{!194, !193, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!195 = !{!192}
!196 = !{!197, !199, !201}
!197 = distinct !{!197, !198, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!198 = distinct !{!198, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!199 = distinct !{!199, !200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!200 = distinct !{!200, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!201 = distinct !{!201, !202, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_: %_1"}
!202 = distinct !{!202, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_"}
!203 = !{!204, !206, !201}
!204 = distinct !{!204, !205, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!205 = distinct !{!205, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!206 = distinct !{!206, !207, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!207 = distinct !{!207, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!208 = !{!201}
!209 = !{!210, !212, !214}
!210 = distinct !{!210, !211, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!211 = distinct !{!211, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!212 = distinct !{!212, !213, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!213 = distinct !{!213, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!214 = distinct !{!214, !215, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_: %_1"}
!215 = distinct !{!215, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group9WaitGroupEBM_"}
!216 = !{!217, !219, !214}
!217 = distinct !{!217, !218, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!218 = distinct !{!218, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!219 = distinct !{!219, !220, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!220 = distinct !{!220, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!221 = !{!214}
!222 = !{!223, !225}
!223 = distinct !{!223, !224, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!224 = distinct !{!224, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!225 = distinct !{!225, !226, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!226 = distinct !{!226, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!227 = !{!228}
!228 = distinct !{!228, !229, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!229 = distinct !{!229, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!230 = !{!231, !233}
!231 = distinct !{!231, !232, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!232 = distinct !{!232, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!233 = distinct !{!233, !232, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!234 = !{!231}
!235 = !{!236}
!236 = distinct !{!236, !237, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar4waitjECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!237 = distinct !{!237, !"_RINvMNtNtNtCs5sEH5CPMdak_3std4sync6poison7condvarNtB3_7Condvar4waitjECs8jD91Rl7RDZ_15crossbeam_utils"}
!238 = !{!239, !241}
!239 = distinct !{!239, !240, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_: %self"}
!240 = distinct !{!240, !"_RNvXsE_NtCsdJPVW0sQgAG_5alloc4syncINtB5_3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropBM_"}
!241 = distinct !{!241, !242, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_: %_1"}
!242 = distinct !{!242, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc4sync3ArcNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync10wait_group5InnerEEB1k_"}
!243 = !{!244, !246}
!244 = distinct !{!244, !245, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!245 = distinct !{!245, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!246 = distinct !{!246, !245, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!247 = !{!244}
!248 = !{!249}
!249 = distinct !{!249, !250, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!250 = distinct !{!250, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!251 = !{!252, !254}
!252 = distinct !{!252, !253, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!253 = distinct !{!253, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!254 = distinct !{!254, !253, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!255 = !{!252}
!256 = !{!257}
!257 = distinct !{!257, !258, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!258 = distinct !{!258, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!259 = !{!260, !262}
!260 = distinct !{!260, !261, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!261 = distinct !{!261, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!262 = distinct !{!262, !261, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!263 = !{!260}
!264 = !{!265}
!265 = distinct !{!265, !266, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!266 = distinct !{!266, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexjE4lockCs8jD91Rl7RDZ_15crossbeam_utils"}
!267 = !{!268, !270}
!268 = distinct !{!268, !269, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!269 = distinct !{!269, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils"}
!270 = distinct !{!270, !269, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardjEINtBM_11PoisonErrorBH_EE6unwrapCs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!271 = !{!268}
!272 = !{!273}
!273 = distinct !{!273, !274, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_: %_0"}
!274 = distinct !{!274, !"_RNvMs5_NtNtNtCs5sEH5CPMdak_3std4sync6poison5mutexINtB5_5MutexNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesE4lockB13_"}
!275 = !{!276}
!276 = distinct !{!276, !277, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_: %self"}
!277 = distinct !{!277, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultINtNtNtNtCs5sEH5CPMdak_3std4sync6poison5mutex10MutexGuardNtNtNtCs8jD91Rl7RDZ_15crossbeam_utils4sync12sharded_lock13ThreadIndicesEINtBM_11PoisonErrorBH_EE6unwrapB1I_"}
!278 = !{i64 1, i64 0}
!279 = !{!280}
!280 = distinct !{!280, !281, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRNtNtNtB9_6thread2id8ThreadIdECs8jD91Rl7RDZ_15crossbeam_utils: argument 0"}
!281 = distinct !{!281, !"_RINvYNtNtNtCs5sEH5CPMdak_3std4hash6random11RandomStateNtNtCsjMrxcFdYDNN_4core4hash11BuildHasher8hash_oneRNtNtNtB9_6thread2id8ThreadIdECs8jD91Rl7RDZ_15crossbeam_utils"}
!282 = !{!283}
!283 = distinct !{!283, !284, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdNtB8_4Hash4hashNtNtNtBL_4hash6random13DefaultHasherECs8jD91Rl7RDZ_15crossbeam_utils: %state"}
!284 = distinct !{!284, !"_RINvXs3_NtNtCsjMrxcFdYDNN_4core4hash5implsRNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdNtB8_4Hash4hashNtNtNtBL_4hash6random13DefaultHasherECs8jD91Rl7RDZ_15crossbeam_utils"}
!285 = !{!286}
!286 = distinct !{!286, !287, !"_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE6removeBO_ECs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!287 = distinct !{!287, !"_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE6removeBO_ECs8jD91Rl7RDZ_15crossbeam_utils"}
!288 = !{!289}
!289 = distinct !{!289, !290, !"_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!290 = distinct !{!290, !"_RINvMs1_NtCsh9QrOU9e3Ke_9hashbrown3mapINtB6_7HashMapNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjNtNtNtBU_4hash6random11RandomStateE12remove_entryBO_ECs8jD91Rl7RDZ_15crossbeam_utils"}
!291 = !{!289, !286}
!292 = !{!293}
!293 = distinct !{!293, !294, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE12remove_entryNCINvNtB8_3map14equivalent_keyBQ_BQ_jE0ECs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!294 = distinct !{!294, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE12remove_entryNCINvNtB8_3map14equivalent_keyBQ_BQ_jE0ECs8jD91Rl7RDZ_15crossbeam_utils"}
!295 = !{!296}
!296 = distinct !{!296, !297, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE4findNCINvNtB8_3map14equivalent_keyBQ_BQ_jE0ECs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!297 = distinct !{!297, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE4findNCINvNtB8_3map14equivalent_keyBQ_BQ_jE0ECs8jD91Rl7RDZ_15crossbeam_utils"}
!298 = !{!299}
!299 = distinct !{!299, !300, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner10find_inner: %self"}
!300 = distinct !{!300, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner10find_inner"}
!301 = !{!299, !296, !293, !289, !286}
!302 = !{!303}
!303 = distinct !{!303, !297, !"_RINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB6_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE4findNCINvNtB8_3map14equivalent_keyBQ_BQ_jE0ECs8jD91Rl7RDZ_15crossbeam_utils: argument 1"}
!304 = !{!305, !307, !299, !296, !303, !293, !289, !286}
!305 = distinct !{!305, !306, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!306 = distinct !{!306, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils"}
!307 = distinct !{!307, !308, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!308 = distinct !{!308, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!309 = !{!310, !299, !296, !303, !293, !289, !286}
!310 = distinct !{!310, !311, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE4findNCINvNtBa_3map14equivalent_keyBS_BS_jE0E0Cs8jD91Rl7RDZ_15crossbeam_utils: %_1"}
!311 = distinct !{!311, !"_RNCINvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB8_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE4findNCINvNtBa_3map14equivalent_keyBS_BS_jE0E0Cs8jD91Rl7RDZ_15crossbeam_utils"}
!312 = !{!313}
!313 = distinct !{!313, !314, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE6removeCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!314 = distinct !{!314, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE6removeCs8jD91Rl7RDZ_15crossbeam_utils"}
!315 = !{!316}
!316 = distinct !{!316, !317, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE13erase_no_dropCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!317 = distinct !{!317, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE13erase_no_dropCs8jD91Rl7RDZ_15crossbeam_utils"}
!318 = !{!319}
!319 = distinct !{!319, !320, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner5erase: %self"}
!320 = distinct !{!320, !"_RNvMsa_NtCsh9QrOU9e3Ke_9hashbrown3rawNtB5_13RawTableInner5erase"}
!321 = !{!322, !324, !319, !316, !326, !313, !293, !289, !286}
!322 = distinct !{!322, !323, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!323 = distinct !{!323, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils"}
!324 = distinct !{!324, !325, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!325 = distinct !{!325, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!326 = distinct !{!326, !314, !"_RNvMs6_NtCsh9QrOU9e3Ke_9hashbrown3rawINtB5_8RawTableTNtNtNtCs5sEH5CPMdak_3std6thread2id8ThreadIdjEE6removeCs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!327 = !{!328, !330, !319, !316, !326, !313, !293, !289, !286}
!328 = distinct !{!328, !329, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils: %_0"}
!329 = distinct !{!329, !"_RINvNtCsjMrxcFdYDNN_4core3ptr14read_unalignedNtNtNtNtB4_9core_arch10arm_shared4neon9uint8x8_tECs8jD91Rl7RDZ_15crossbeam_utils"}
!330 = distinct !{!330, !331, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8: %_0"}
!331 = distinct !{!331, !"_RNvNtNtNtNtCsjMrxcFdYDNN_4core9core_arch7aarch644neon9generated7vld1_u8"}
!332 = !{!319, !316, !313, !293, !289, !286}
!333 = !{!326}
!334 = !{!319, !316, !326, !313, !293, !289, !286}
!335 = !{!336}
!336 = distinct !{!336, !337, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecjE8push_mutCs8jD91Rl7RDZ_15crossbeam_utils: %self"}
!337 = distinct !{!337, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecjE8push_mutCs8jD91Rl7RDZ_15crossbeam_utils"}
