; ModuleID = 'build_script_build.89df0ee823c14401-cgu.0'
source_filename = "build_script_build.89df0ee823c14401-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCsbPSV2LqDF3p_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CsbPSV2LqDF3p_18build_script_build, ptr @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CsbPSV2LqDF3p_18build_script_build }>, align 8
@alloc_742f06589122110502429e832b81e8bd = private unnamed_addr constant [32 x i8] c"cargo:rerun-if-changed=build.rs\0A", align 1
@alloc_8d78a40e246da6716843e9ec1abdbf4a = private unnamed_addr constant [63 x i8] c"cargo:rustc-check-cfg=cfg(fast_arithmetic, values(\2232\22, \2264\22))\0A", align 1
@alloc_0d3bcf6fb685f000bc18304ea76cbac4 = private unnamed_addr constant [21 x i8] c"CARGO_CFG_TARGET_ARCH", align 1
@alloc_3b3853a8db8410014be1f888d8237dac = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/serde_json-1.0.149/build.rs\00", align 1
@alloc_d8fa4c8f9193a807f6dccdc4e4271c68 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3b3853a8db8410014be1f888d8237dac, [16 x i8] c"c\00\00\00\00\00\00\00\0A\00\00\00<\00\00\00" }>, align 8
@alloc_6508c675143a2a16e0690055cd395724 = private unnamed_addr constant [30 x i8] c"CARGO_CFG_TARGET_POINTER_WIDTH", align 1
@alloc_fe32eecabd9c1013c7a769c735630542 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_3b3853a8db8410014be1f888d8237dac, [16 x i8] c"c\00\00\00\00\00\00\00\0B\00\00\00N\00\00\00" }>, align 8
@alloc_a1239a7299ff261e47dc48b0b02c8ca1 = private unnamed_addr constant [7 x i8] c"aarch64", align 1
@alloc_be0c7e2eb8d81d67a6db9a856123bb7e = private unnamed_addr constant [11 x i8] c"loongarch64", align 1
@alloc_5fb3b0f50388bf85c95c974cdb111fa2 = private unnamed_addr constant [6 x i8] c"mips64", align 1
@alloc_fa1130f2f45123ef906740f12b430906 = private unnamed_addr constant [9 x i8] c"powerpc64", align 1
@alloc_8f8dc58223e03021a07a335aedb98959 = private unnamed_addr constant [7 x i8] c"riscv64", align 1
@alloc_55df10dc7797e63df69596598a679c19 = private unnamed_addr constant [6 x i8] c"wasm32", align 1
@alloc_4a29a4faa0904cd7ff982831f2813e90 = private unnamed_addr constant [6 x i8] c"x86_64", align 1
@alloc_8092ccd99cb94b0213fd5864ca7ee6ea = private unnamed_addr constant [2 x i8] c"64", align 1
@alloc_7a26f645c74905f3b4beb62547c7b9da = private unnamed_addr constant [37 x i8] c"cargo:rustc-cfg=fast_arithmetic=\2232\22\0A", align 1
@alloc_73dc34a66d663abb565b3e9208b39a8d = private unnamed_addr constant [37 x i8] c"cargo:rustc-cfg=fast_arithmetic=\2264\22\0A", align 1

; std::rt::lang_start::<()>
; Function Attrs: uwtable
define hidden noundef i64 @_RINvNtCs5sEH5CPMdak_3std2rt10lang_startuECsbPSV2LqDF3p_18build_script_build(ptr noundef nonnull %main, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe) unnamed_addr #0 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  store ptr %main, ptr %_7, align 8
; call std::rt::lang_start_internal
  %_0 = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %argc, ptr noundef %argv, i8 noundef %sigpipe)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i64 %_0
}

; std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
; Function Attrs: noinline uwtable
define internal fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbPSV2LqDF3p_18build_script_build(ptr noundef nonnull readonly captures(none) %f) unnamed_addr #1 {
start:
  tail call void %f()
  tail call void asm sideeffect "", "~{memory}"() #10, !srcloc !3
  ret void
}

; std::rt::lang_start::<()>::{closure#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CsbPSV2LqDF3p_18build_script_build(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %_1) unnamed_addr #2 {
start:
  %_4 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbPSV2LqDF3p_18build_script_build(ptr noundef nonnull %_4) #11
  ret i32 0
}

; <std::rt::lang_start<()>::{closure#0} as core::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
; Function Attrs: inlinehint uwtable
define internal noundef i32 @_RNSNvYNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0INtNtNtCsjMrxcFdYDNN_4core3ops8function6FnOnceuE9call_once6vtableCsbPSV2LqDF3p_18build_script_build(ptr noundef readonly captures(none) %_1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = load ptr, ptr %_1, align 8, !nonnull !4, !noundef !4
; call std::sys::backtrace::__rust_begin_short_backtrace::<fn(), ()>
  tail call fastcc void @_RINvNtNtCs5sEH5CPMdak_3std3sys9backtrace28___rust_begin_short_backtraceFEuuECsbPSV2LqDF3p_18build_script_build(ptr noundef nonnull readonly %0) #11, !noalias !5
  ret i32 0
}

; build_script_build::main
; Function Attrs: uwtable
define hidden void @_RNvCsbPSV2LqDF3p_18build_script_build4main() unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_8 = alloca [24 x i8], align 8
  %_6 = alloca [24 x i8], align 8
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_742f06589122110502429e832b81e8bd, ptr noundef nonnull inttoptr (i64 65 to ptr))
; call std::io::stdio::_print
  tail call void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull @alloc_8d78a40e246da6716843e9ec1abdbf4a, ptr noundef nonnull inttoptr (i64 127 to ptr))
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0d3bcf6fb685f000bc18304ea76cbac4, i64 noundef 21)
  %0 = load i64, ptr %_6, align 8, !range !8, !noundef !4
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb23, label %bb24, !prof !9

bb24:                                             ; preds = %start
  %target_arch.sroa.5.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %target_arch.sroa.5.0.copyload = load ptr, ptr %target_arch.sroa.5.0._6.sroa_idx, align 8
  %target_arch.sroa.14.0._6.sroa_idx = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %target_arch.sroa.14.0.copyload = load i64, ptr %target_arch.sroa.14.0._6.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_8)
; invoke std::env::_var_os
  invoke void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_8, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_6508c675143a2a16e0690055cd395724, i64 noundef 30)
          to label %bb4 unwind label %cleanup

bb23:                                             ; preds = %start
; call core::option::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_d8fa4c8f9193a807f6dccdc4e4271c68) #12
  unreachable

bb20:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i10, %cleanup1, %cleanup
  %.pn = phi { ptr, i32 } [ %3, %cleanup ], [ %6, %cleanup1 ], [ %6, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i10 ]
  %1 = icmp eq i64 %0, 0
  br i1 %1, label %bb21, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %bb20
  %2 = icmp ne ptr %target_arch.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %2)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_arch.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #10
  br label %bb21

cleanup:                                          ; preds = %bb24, %bb25
  %3 = landingpad { ptr, i32 }
          cleanup
  br label %bb20

bb4:                                              ; preds = %bb24
  %4 = load i64, ptr %_8, align 8, !range !8, !noundef !4
  %.not2 = icmp eq i64 %4, -9223372036854775808
  br i1 %.not2, label %bb25, label %bb26, !prof !9

bb26:                                             ; preds = %bb4
  %target_pointer_width.sroa.5.0._8.sroa_idx = getelementptr inbounds nuw i8, ptr %_8, i64 8
  %target_pointer_width.sroa.5.0.copyload = load ptr, ptr %target_pointer_width.sroa.5.0._8.sroa_idx, align 8
  %target_pointer_width.sroa.8.0._8.sroa_idx = getelementptr inbounds nuw i8, ptr %_8, i64 16
  %target_pointer_width.sroa.8.0.copyload = load i64, ptr %target_pointer_width.sroa.8.0._8.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_8)
  %5 = icmp ne ptr %target_arch.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %5)
  switch i64 %target_arch.sroa.14.0.copyload, label %bb11 [
    i64 7, label %bb27
    i64 11, label %bb28
    i64 6, label %bb29
    i64 9, label %bb30
  ]

bb25:                                             ; preds = %bb4
; invoke core::option::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fe32eecabd9c1013c7a769c735630542) #13
          to label %unreachable unwind label %cleanup

unreachable:                                      ; preds = %bb25
  unreachable

cleanup1:                                         ; preds = %bb12.invoke
  %6 = landingpad { ptr, i32 }
          cleanup
  %7 = icmp eq i64 %4, 0
  br i1 %7, label %bb20, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i10

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i10: ; preds = %cleanup1
  %8 = icmp ne ptr %target_pointer_width.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %8)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_pointer_width.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #10
  br label %bb20

bb27:                                             ; preds = %bb26
  %9 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_a1239a7299ff261e47dc48b0b02c8ca1, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !10
  %10 = icmp eq i32 %9, 0
  br i1 %10, label %bb12.invoke, label %bb31

bb12.invoke:                                      ; preds = %bb27, %bb28, %bb29, %bb30, %bb31, %bb8.thread71.thread, %bb33, %bb34, %bb14
  %11 = phi ptr [ @alloc_7a26f645c74905f3b4beb62547c7b9da, %bb14 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb34 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb33 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb8.thread71.thread ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb31 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb30 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb29 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb28 ], [ @alloc_73dc34a66d663abb565b3e9208b39a8d, %bb27 ]
; invoke std::io::stdio::_print
  invoke void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull %11, ptr noundef nonnull inttoptr (i64 75 to ptr))
          to label %bb16 unwind label %cleanup1

bb28:                                             ; preds = %bb26
  %12 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(11) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(11) @alloc_be0c7e2eb8d81d67a6db9a856123bb7e, i64 range(i64 0, -9223372036854775808) 11), !alias.scope !14
  %13 = icmp eq i32 %12, 0
  br i1 %13, label %bb12.invoke, label %bb11

bb29:                                             ; preds = %bb26
  %14 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_5fb3b0f50388bf85c95c974cdb111fa2, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !18
  %15 = icmp eq i32 %14, 0
  br i1 %15, label %bb12.invoke, label %bb8.thread71.thread

bb8.thread71.thread:                              ; preds = %bb29
  %16 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_55df10dc7797e63df69596598a679c19, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !22
  %17 = icmp eq i32 %16, 0
  br i1 %17, label %bb12.invoke, label %bb33

bb30:                                             ; preds = %bb26
  %18 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(9) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(9) @alloc_fa1130f2f45123ef906740f12b430906, i64 range(i64 0, -9223372036854775808) 9), !alias.scope !26
  %19 = icmp eq i32 %18, 0
  br i1 %19, label %bb12.invoke, label %bb11

bb31:                                             ; preds = %bb27
  %20 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(7) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(7) @alloc_8f8dc58223e03021a07a335aedb98959, i64 range(i64 0, -9223372036854775808) 7), !alias.scope !30
  %21 = icmp eq i32 %20, 0
  br i1 %21, label %bb12.invoke, label %bb11

bb33:                                             ; preds = %bb8.thread71.thread
  %22 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(6) %target_arch.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(6) @alloc_4a29a4faa0904cd7ff982831f2813e90, i64 range(i64 0, -9223372036854775808) 6), !alias.scope !34
  %23 = icmp eq i32 %22, 0
  br i1 %23, label %bb12.invoke, label %bb11

bb11:                                             ; preds = %bb30, %bb26, %bb28, %bb31, %bb33
  %24 = icmp ne ptr %target_pointer_width.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %24)
  %_3.not.i36 = icmp eq i64 %target_pointer_width.sroa.8.0.copyload, 2
  br i1 %_3.not.i36, label %bb34, label %bb14

bb34:                                             ; preds = %bb11
  %25 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %target_pointer_width.sroa.5.0.copyload, ptr noundef nonnull dereferenceable(2) @alloc_8092ccd99cb94b0213fd5864ca7ee6ea, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !38
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %bb12.invoke, label %bb14

bb14:                                             ; preds = %bb11, %bb34
  br label %bb12.invoke

bb16:                                             ; preds = %bb12.invoke
  %27 = icmp eq i64 %4, 0
  br i1 %27, label %bb17, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i40

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i40: ; preds = %bb16
  %28 = icmp ne ptr %target_pointer_width.sroa.5.0.copyload, null
  call void @llvm.assume(i1 %28)
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_pointer_width.sroa.5.0.copyload, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #10
  br label %bb17

bb17:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i40, %bb16
  %29 = icmp eq i64 %0, 0
  br i1 %29, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsbPSV2LqDF3p_18build_script_build.exit43, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i42

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i42: ; preds = %bb17
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %target_arch.sroa.5.0.copyload, i64 noundef %0, i64 noundef range(i64 1, -9223372036854775807) 1) #10
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsbPSV2LqDF3p_18build_script_build.exit43

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECsbPSV2LqDF3p_18build_script_build.exit43: ; preds = %bb17, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i42
  ret void

bb21:                                             ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %bb20
  resume { ptr, i32 } %.pn
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #3

; std::rt::lang_start_internal
; Function Attrs: uwtable
declare noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), i64 noundef, ptr noundef, i8 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #3

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #4

; std::env::_var_os
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #5

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6__print(ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #6

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #7

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #8

define noundef i32 @main(i32 %0, ptr %1) unnamed_addr #9 {
top:
  %_7.i = alloca [8 x i8], align 8
  %2 = sext i32 %0 to i64
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7.i)
  store ptr @_RNvCsbPSV2LqDF3p_18build_script_build4main, ptr %_7.i, align 8
; call std::rt::lang_start_internal
  %_0.i = call noundef i64 @_RNvNtCs5sEH5CPMdak_3std2rt19lang_start_internal(ptr noundef nonnull align 1 %_7.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, i64 noundef %2, ptr noundef %1, i8 noundef 0)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7.i)
  %3 = trunc i64 %_0.i to i32
  ret i32 %3
}

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #4 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #6 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #8 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { nounwind }
attributes #11 = { noinline }
attributes #12 = { noinline noreturn }
attributes #13 = { noreturn }

!llvm.module.flags = !{!0, !1}
!llvm.ident = !{!2}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{i32 7, !"PIE Level", i32 2}
!2 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!3 = !{i64 13199890497594648}
!4 = !{}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CsbPSV2LqDF3p_18build_script_build: %_1"}
!7 = distinct !{!7, !"_RNCINvNtCs5sEH5CPMdak_3std2rt10lang_startuE0CsbPSV2LqDF3p_18build_script_build"}
!8 = !{i64 0, i64 -9223372036854775807}
!9 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!10 = !{!11, !13}
!11 = distinct !{!11, !12, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!12 = distinct !{!12, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!13 = distinct !{!13, !12, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!14 = !{!15, !17}
!15 = distinct !{!15, !16, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!16 = distinct !{!16, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!17 = distinct !{!17, !16, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!18 = !{!19, !21}
!19 = distinct !{!19, !20, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!20 = distinct !{!20, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!21 = distinct !{!21, !20, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!22 = !{!23, !25}
!23 = distinct !{!23, !24, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!24 = distinct !{!24, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!25 = distinct !{!25, !24, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!26 = !{!27, !29}
!27 = distinct !{!27, !28, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!28 = distinct !{!28, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!29 = distinct !{!29, !28, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!30 = !{!31, !33}
!31 = distinct !{!31, !32, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!32 = distinct !{!32, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!33 = distinct !{!33, !32, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!34 = !{!35, !37}
!35 = distinct !{!35, !36, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!36 = distinct !{!36, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!37 = distinct !{!37, !36, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
!38 = !{!39, !41}
!39 = distinct !{!39, !40, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %self.0"}
!40 = distinct !{!40, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build"}
!41 = distinct !{!41, !40, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsbPSV2LqDF3p_18build_script_build: %other.0"}
