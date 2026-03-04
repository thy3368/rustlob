; ModuleID = 'clap_lex.5b63e3d8f81198-cgu.0'
source_filename = "clap_lex.5b63e3d8f81198-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"std::ffi::os_str::OsString" = type { %"std::sys::os_str::bytes::Buf" }
%"std::sys::os_str::bytes::Buf" = type { %"alloc::vec::Vec<u8>" }
%"alloc::vec::Vec<u8>" = type { %"alloc::raw_vec::RawVec<u8>", i64 }
%"alloc::raw_vec::RawVec<u8>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<u8>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<u8>" = type {}

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex }>, align 8
@alloc_b5cd4d979846a196645a2f424070a341 = private unnamed_addr constant [98 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-0.7.6/src/ext.rs\00", align 1
@alloc_8a01b8cc769b8fb73f4fe5e2b9a72b0d = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b5cd4d979846a196645a2f424070a341, [16 x i8] c"a\00\00\00\00\00\00\00\C6\00\00\00\1D\00\00\00" }>, align 8
@alloc_9f81aaaf7b04a46d3f186d0b7b34a866 = private unnamed_addr constant [98 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/clap_lex-0.7.6/src/lib.rs\00", align 1
@alloc_f7c6bd120cdd150e1d7183e14240e646 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9f81aaaf7b04a46d3f186d0b7b34a866, [16 x i8] c"a\00\00\00\00\00\00\00\E6\01\00\00)\00\00\00" }>, align 8
@alloc_743d0ef24a5897c38fa9d15d15a1122b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9f81aaaf7b04a46d3f186d0b7b34a866, [16 x i8] c"a\00\00\00\00\00\00\00\E4\00\00\00#\00\00\00" }>, align 8
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str5errorNtB5_9Utf8ErrorNtNtB9_3fmt5Debug3fmt }>, align 8
@alloc_00ae4b301f7fab8ac9617c03fcbd7274 = private unnamed_addr constant [43 x i8] c"called `Result::unwrap()` on an `Err` value", align 1
@alloc_05ac4674d88601cc843f438cc0a6c56a = private unnamed_addr constant [2 x i8] c"--", align 1
@alloc_219cd7325be98984893e9f7eb51114d4 = private unnamed_addr constant [1 x i8] c"=", align 1
@alloc_80a360ff56096edb1899e5a3621ce404 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b5cd4d979846a196645a2f424070a341, [16 x i8] c"a\00\00\00\00\00\00\00\16\01\00\00%\00\00\00" }>, align 8
@alloc_de83dc5a83c52017857da74c6afa4864 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b5cd4d979846a196645a2f424070a341, [16 x i8] c"a\00\00\00\00\00\00\00\E4\00\00\00\1F\00\00\00" }>, align 8
@alloc_fc30b8b9e6926e97249f2ed625cdad25 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b5cd4d979846a196645a2f424070a341, [16 x i8] c"a\00\00\00\00\00\00\00\E3\00\00\00\1E\00\00\00" }>, align 8
@alloc_eab5d04767146d7d9b93b60d28ef530a = private unnamed_addr constant <{ ptr, [8 x i8] }> <{ ptr inttoptr (i64 1 to ptr), [8 x i8] zeroinitializer }>, align 8
@alloc_1da1cbfd3a38ab605c33306f33c36a20 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b5cd4d979846a196645a2f424070a341, [16 x i8] c"a\00\00\00\00\00\00\00\D8\00\00\00\09\00\00\00" }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt }>, align 8
@vtable.3 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex }>, align 8
@alloc_8e685ef482aec04a2d7a8ed5ef1228a3 = private unnamed_addr constant [9 x i8] c"Utf8Error", align 1
@alloc_f34017a1538f19bf68b6d6294eec0bb3 = private unnamed_addr constant [11 x i8] c"valid_up_to", align 1
@alloc_91eca80c47235190e5fbed3d6d8be36c = private unnamed_addr constant [9 x i8] c"error_len", align 1
@alloc_37d2e53432a03a1f90b3e7253015eaf9 = private unnamed_addr constant [4 x i8] c"None", align 1
@vtable.4 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRhNtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex }>, align 8
@alloc_9535bf4c204f3eb9b19ec2c83e446e52 = private unnamed_addr constant [4 x i8] c"Some", align 1

; core::ptr::drop_in_place::<alloc::vec::Vec<std::ffi::os_str::OsString>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %0, align 8, !nonnull !2, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val5 = load i64, ptr %1, align 8, !noundef !2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3)
  %_710.i.i = icmp eq i64 %_1.val5, 0
  br i1 %_710.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i
  %_3.sroa.0.011.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_1.val4, i64 %_3.sroa.0.011.i.i
  %2 = add nuw i64 %_3.sroa.0.011.i.i, 1
  %_6.val.i.i = load i64, ptr %_6.i.i, align 8, !alias.scope !3
  %3 = icmp eq i64 %_6.val.i.i, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb5.i.i
  %4 = getelementptr i8, ptr %_6.i.i, i64 8
  %_6.val7.i.i = load ptr, ptr %4, align 8, !alias.scope !3, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i, i64 noundef %_6.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !3
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i, %bb5.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val5
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i, %start
  %_1.val2 = load i64, ptr %_1, align 8, !range !6, !noundef !2
  %5 = icmp eq i64 %_1.val2, 0
  br i1 %5, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = mul nuw i64 %_1.val2, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #27
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void
}

; core::ptr::drop_in_place::<core::iter::adapters::map::Map<std::env::ArgsOs, <clap_lex::RawArgs as core::convert::From<std::env::ArgsOs>>::from::{closure#0}>>
; Function Attrs: nounwind uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !13)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !16)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self.val.i.i.i.i = load ptr, ptr %0, align 8, !alias.scope !19, !nonnull !2, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %self.val1.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !19, !nonnull !2, !noundef !2
  %2 = ptrtoint ptr %self.val1.i.i.i.i to i64
  %3 = ptrtoint ptr %self.val.i.i.i.i to i64
  %4 = sub nuw i64 %2, %3
  %5 = udiv exact i64 %4, 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20)
  %_710.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i, %self.val.i.i.i.i
  br i1 %_710.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i = phi i64 [ %6, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i ], [ 0, %start ]
  %_6.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i
  %6 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i, align 8, !alias.scope !20, !noalias !19
  %7 = icmp eq i64 %_6.val.i.i.i.i.i, 0
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb5.i.i.i.i.i
  %8 = getelementptr i8, ptr %_6.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !20, !noalias !19, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !23
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i
  %_7.i.i.i.i.i = icmp eq i64 %6, %5
  br i1 %_7.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i, %start
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %capacity1.i.i3.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !19, !noundef !2
  %10 = icmp eq i64 %capacity1.i.i3.i.i.i.i, 0
  br i1 %10, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex.exit, label %bb2.i.i.i.i.i4.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i:                           ; preds = %bb2.i.i.i.i
  %ptr.i.i5.i.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !19, !nonnull !2, !noundef !2
  %alloc_size.i.i.i.i.i.i6.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !19
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex.exit: ; preds = %bb2.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i
  ret void
}

; core::panicking::assert_failed::<&str, &str>
; Function Attrs: cold minsize noinline noreturn optsize uwtable
define void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedReBM_ECs1TOCnChXxQ_8clap_lex(i8 noundef range(i8 0, 3) %kind, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %1, ptr noundef %args.0, ptr %args.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %2) unnamed_addr #1 {
start:
  %right = alloca [8 x i8], align 8
  %left = alloca [8 x i8], align 8
  store ptr %0, ptr %left, align 8
  store ptr %1, ptr %right, align 8
; call core::panicking::assert_failed_inner
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking19assert_failed_inner(i8 noundef %kind, ptr noundef nonnull align 1 %left, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noundef nonnull align 1 %right, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noundef %args.0, ptr %args.1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %2) #28
  unreachable
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1TOCnChXxQ_8clap_lex(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 1, 0) %additional) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !6, !alias.scope !24, !noundef !2
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !24
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !24
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1TOCnChXxQ_8clap_lex(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !27, !noalias !24, !noundef !2
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !28, !noalias !24, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !24
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !24
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #29
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !24, !nonnull !2, !noundef !2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !24
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !24
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !24
  ret void
}

; <clap_lex::RawArgs>::peek
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(read, inaccessiblemem: none) uwtable
define { ptr, i64 } @_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs4peek(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %cursor) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val2 = load i64, ptr %0, align 8, !noundef !2
  %cursor.val = load i64, ptr %cursor, align 8, !noundef !2
  %_9.i = icmp ult i64 %cursor.val, %self.val2
  br i1 %_9.i, label %bb1.i, label %_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs7peek_os.exit

bb1.i:                                            ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val = load ptr, ptr %1, align 8, !nonnull !2, !noundef !2
  %_10.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val, i64 %cursor.val
  %2 = getelementptr inbounds nuw i8, ptr %_10.i, i64 8
  %_18.i = load ptr, ptr %2, align 8, !nonnull !2, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %_10.i, i64 16
  %_17.i = load i64, ptr %3, align 8, !noundef !2
  br label %_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs7peek_os.exit

_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs7peek_os.exit: ; preds = %start, %bb1.i
  %_0.sroa.3.0.i = phi i64 [ %_17.i, %bb1.i ], [ undef, %start ]
  %_0.sroa.0.0.i = phi ptr [ %_18.i, %bb1.i ], [ null, %start ]
  %4 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i, 0
  %5 = insertvalue { ptr, i64 } %4, i64 %_0.sroa.3.0.i, 1
  ret { ptr, i64 } %5
}

; <clap_lex::RawArgs>::seek
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable
define void @_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs4seek(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 captures(none) dereferenceable(8) %cursor, i64 noundef range(i64 0, 3) %0, i64 noundef %1) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  switch i64 %0, label %default.unreachable11 [
    i64 0, label %bb7
    i64 1, label %bb3
    i64 2, label %bb2
  ]

default.unreachable11:                            ; preds = %start
  unreachable

bb3:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_10 = load i64, ptr %2, align 8, !noundef !2
  %_19 = icmp ult i64 %_10, 384307168202282326
  tail call void @llvm.assume(i1 %_19)
  %3 = tail call i64 @llvm.sadd.sat.i64(i64 %_10, i64 %1)
  %_0.sroa.0.0.i = tail call noundef range(i64 0, -9223372036854775808) i64 @llvm.smax.i64(i64 %3, i64 0)
  br label %bb7

bb2:                                              ; preds = %start
  %_15 = load i64, ptr %cursor, align 8, !noundef !2
  %4 = tail call i64 @llvm.sadd.sat.i64(i64 %_15, i64 %1)
  %_0.sroa.0.0.i9 = tail call noundef range(i64 0, -9223372036854775808) i64 @llvm.smax.i64(i64 %4, i64 0)
  br label %bb7

bb7:                                              ; preds = %start, %bb2, %bb3
  %pos1.sroa.0.0 = phi i64 [ %_0.sroa.0.0.i, %bb3 ], [ %_0.sroa.0.0.i9, %bb2 ], [ %1, %start ]
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_18 = load i64, ptr %5, align 8, !noundef !2
  %_20 = icmp ult i64 %_18, 384307168202282326
  tail call void @llvm.assume(i1 %_20)
  %_0.sroa.0.0.i10 = tail call noundef range(i64 0, 384307168202282326) i64 @llvm.umin.i64(i64 range(i64 0, 384307168202282326) %_18, i64 %pos1.sroa.0.0)
  store i64 %_0.sroa.0.0.i10, ptr %cursor, align 8
  ret void
}

; <clap_lex::RawArgs>::is_end
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define noundef zeroext i1 @_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs6is_end(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %cursor) unnamed_addr #5 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1 = load i64, ptr %0, align 8, !noundef !2
  %cursor.val = load i64, ptr %cursor, align 8, !noundef !2
  %_9.i = icmp uge i64 %cursor.val, %self.val1
  ret i1 %_9.i
}

; <clap_lex::RawArgs>::from_args
; Function Attrs: uwtable
define void @_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs9from_args(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %_19.i.i.i = alloca [32 x i8], align 8
  %vector.i.i.i = alloca [24 x i8], align 8
  %_1 = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_1)
; call std::env::args_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7args_os(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_1)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !29)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !32)
  %iter1.sroa.0.0.copyload.i = load ptr, ptr %_1, align 8, !alias.scope !34, !noalias !29
  %iter1.sroa.2.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %iter1.sroa.2.0.copyload.i = load ptr, ptr %iter1.sroa.2.0.iter.sroa_idx.i, align 8, !alias.scope !34, !noalias !29, !nonnull !2, !noundef !2
  %iter1.sroa.3.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %iter1.sroa.3.0.copyload.i = load i64, ptr %iter1.sroa.3.0.iter.sroa_idx.i, align 8, !alias.scope !34, !noalias !29
  %iter1.sroa.4.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %iter1.sroa.4.0.copyload.i = load ptr, ptr %iter1.sroa.4.0.iter.sroa_idx.i, align 8, !alias.scope !34, !noalias !29, !nonnull !2, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i.i.i), !noalias !38
  %_9.i.i.i.i.i.i = icmp eq ptr %iter1.sroa.2.0.copyload.i, %iter1.sroa.4.0.copyload.i
  br i1 %_9.i.i.i.i.i.i, label %bb12.i.i.i, label %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i

_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i: ; preds = %start
  %_23.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.2.0.copyload.i, i64 24
  %self1.sroa.0.0.copyload2.i.i.i.i = load i64, ptr %iter1.sroa.2.0.copyload.i, align 8, !noalias !45
  %.not.i.i.i.i = icmp eq i64 %self1.sroa.0.0.copyload2.i.i.i.i, -9223372036854775808
  br i1 %.not.i.i.i.i, label %bb12.i.i.i, label %bb14.i.i.i

bb12.i.i.i:                                       ; preds = %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i, %start
  %self.val.i.i.i.i.i.i.i.i = phi ptr [ %iter1.sroa.2.0.copyload.i, %start ], [ %_23.i.i.i.i.i.i, %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i), !noalias !38
  %0 = ptrtoint ptr %iter1.sroa.4.0.copyload.i to i64
  %1 = ptrtoint ptr %self.val.i.i.i.i.i.i.i.i to i64
  %2 = sub nuw i64 %0, %1
  %3 = udiv exact i64 %2, 24
  tail call void @llvm.experimental.noalias.scope.decl(metadata !53)
  %_710.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter1.sroa.4.0.copyload.i, %self.val.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i:                            ; preds = %bb12.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i = phi i64 [ %4, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i ], [ 0, %bb12.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i
  %4 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !53, !noalias !56
  %5 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i, 0
  br i1 %5, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb5.i.i.i.i.i.i.i.i.i
  %6 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i = load ptr, ptr %6, align 8, !alias.scope !53, !noalias !56, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !67
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = icmp eq i64 %4, %3
  br i1 %_7.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i:                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i, %bb12.i.i.i
  %7 = icmp eq i64 %iter1.sroa.3.0.copyload.i, 0
  br i1 %7, label %_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_.exit, label %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i:                   ; preds = %bb2.i.i.i.i.i.i.i.i
  %8 = icmp ne ptr %iter1.sroa.0.0.copyload.i, null
  tail call void @llvm.assume(i1 %8)
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i = mul nuw i64 %iter1.sroa.3.0.copyload.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %iter1.sroa.0.0.copyload.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !56
  br label %_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_.exit

cleanup2.i.i.i:                                   ; preds = %bb3.i.i.i.i
  %9 = landingpad { ptr, i32 }
          cleanup
  %10 = icmp eq i64 %self1.sroa.0.0.copyload2.i.i.i.i, 0
  br i1 %10, label %bb10.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i:                           ; preds = %cleanup2.i.i.i
  %11 = icmp ne ptr %_3.sroa.6.sroa.0.0.copyload.i.i.i, null
  tail call void @llvm.assume(i1 %11)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.sroa.6.sroa.0.0.copyload.i.i.i, i64 noundef %self1.sroa.0.0.copyload2.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !38
  br label %bb10.i.i.i

bb14.i.i.i:                                       ; preds = %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i
  %self1.sroa.6.0._21.i.i.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.2.0.copyload.i, i64 8
  %_3.sroa.6.sroa.0.0.copyload.i.i.i = load ptr, ptr %self1.sroa.6.0._21.i.i.sroa_idx.i.i.i.i, align 8, !noalias !68
  %_3.sroa.6.sroa.5.0.self1.sroa.6.0._21.i.i.sroa_idx.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %iter1.sroa.2.0.copyload.i, i64 16
  %_3.sroa.6.sroa.5.0.copyload.i.i.i = load i64, ptr %_3.sroa.6.sroa.5.0.self1.sroa.6.0._21.i.i.sroa_idx.i.sroa_idx.i.i.i, align 8, !noalias !68
  %12 = ptrtoint ptr %iter1.sroa.4.0.copyload.i to i64
  %13 = ptrtoint ptr %_23.i.i.i.i.i.i to i64
  %14 = sub nuw i64 %12, %13
  %15 = udiv exact i64 %14, 24
  %16 = tail call i64 @llvm.umax.i64(i64 %15, i64 3)
  %_0.sroa.0.0.i.i.i.i = add nuw nsw i64 %16, 1
  %or.cond.i.i.i.i = icmp ugt i64 %14, 9223372036854775776
  br i1 %or.cond.i.i.i.i, label %bb3.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i, !prof !69

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i: ; preds = %bb14.i.i.i
  %_27.0.i.i.i.i.i.i = mul nuw nsw i64 %16, 24
  %new_size2.i.i.i.i.i.i = add nuw nsw i64 %_27.0.i.i.i.i.i.i, 24
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !70
; call __rustc::__rust_alloc
  %17 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !70
  %18 = icmp eq ptr %17, null
  br i1 %18, label %bb3.i.i.i.i, label %bb15.i.i.i

bb3.i.i.i.i:                                      ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i, %bb14.i.i.i
  %_4.sroa.4.0.ph.i.i.i.i = phi i64 [ 8, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i ], [ 0, %bb14.i.i.i ]
  %_4.sroa.10.0.ph.i.i.i.i = phi i64 [ %new_size2.i.i.i.i.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i ], [ undef, %bb14.i.i.i ]
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i.i.i.i, i64 %_4.sroa.10.0.ph.i.i.i.i) #29
          to label %.noexc.i.i.i unwind label %cleanup2.i.i.i, !noalias !38

.noexc.i.i.i:                                     ; preds = %bb3.i.i.i.i
  unreachable

bb15.i.i.i:                                       ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i.i
  store i64 %self1.sroa.0.0.copyload2.i.i.i.i, ptr %17, align 8, !noalias !38
  %src.sroa.4.0._28.1.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %17, i64 8
  store ptr %_3.sroa.6.sroa.0.0.copyload.i.i.i, ptr %src.sroa.4.0._28.1.sroa_idx.i.i.i, align 8, !noalias !38
  %src.sroa.5.0._28.1.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %17, i64 16
  store i64 %_3.sroa.6.sroa.5.0.copyload.i.i.i, ptr %src.sroa.5.0._28.1.sroa_idx.i.i.i, align 8, !noalias !38
  store i64 %_0.sroa.0.0.i.i.i.i, ptr %vector.i.i.i, align 8, !noalias !38
  %vector1.sroa.4.0.vector.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i, i64 8
  store ptr %17, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i, align 8, !noalias !38
  %vector1.sroa.6.0.vector.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %vector.i.i.i, i64 16
  store i64 1, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i, align 8, !noalias !38
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_19.i.i.i), !noalias !38
  store ptr %iter1.sroa.0.0.copyload.i, ptr %_19.i.i.i, align 8, !noalias !73
  %_4.sroa.6.0._19.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i, i64 8
  store ptr %_23.i.i.i.i.i.i, ptr %_4.sroa.6.0._19.i.i.sroa_idx.i, align 8, !noalias !73
  %_4.sroa.8.0._19.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i, i64 16
  store i64 %iter1.sroa.3.0.copyload.i, ptr %_4.sroa.8.0._19.i.i.sroa_idx.i, align 8, !noalias !73
  %_4.sroa.10.0._19.i.i.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_19.i.i.i, i64 24
  store ptr %iter1.sroa.4.0.copyload.i, ptr %_4.sroa.10.0._19.i.i.sroa_idx.i, align 8, !noalias !73
  tail call void @llvm.experimental.noalias.scope.decl(metadata !74)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !77)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !79)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !82)
  %_9.i.i.i14.i.i.i.i.i = icmp eq ptr %_23.i.i.i.i.i.i, %iter1.sroa.4.0.copyload.i
  br i1 %_9.i.i.i14.i.i.i.i.i, label %bb9.i.i.i.i.i, label %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i

_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i: ; preds = %bb15.i.i.i, %bb8.i.i.i.i.i
  %_23.i.i25.i.i.i = phi ptr [ %_23.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %17, %bb15.i.i.i ]
  %len.i.i.i.i.i = phi i64 [ %new_len.i.i.i.i.i, %bb8.i.i.i.i.i ], [ 1, %bb15.i.i.i ]
  %iterator.val1315.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_23.i.i.i.i.i.i, %bb15.i.i.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !84)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !87)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !90)
  %_23.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.val1315.i.i.i.i.i, i64 24
  %self1.sroa.0.0.copyload2.i.i.i.i.i.i = load i64, ptr %iterator.val1315.i.i.i.i.i, align 8, !noalias !93
  %.not.i.i.i.i.i.i = icmp eq i64 %self1.sroa.0.0.copyload2.i.i.i.i.i.i, -9223372036854775808
  br i1 %.not.i.i.i.i.i.i, label %bb9.i.i.i.i.loopexit.i, label %bb3.i.i.i.i.i

bb11.i.i.i.i.i:                                   ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i, %cleanup2.i.i.i.i.i
; call core::ptr::drop_in_place::<core::iter::adapters::map::Map<std::env::ArgsOs, <clap_lex::RawArgs as core::convert::From<std::env::ArgsOs>>::from::{closure#0}>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_(ptr noalias noundef nonnull align 8 dereferenceable(32) %_19.i.i.i) #30, !noalias !95
; call core::ptr::drop_in_place::<alloc::vec::Vec<std::ffi::os_str::OsString>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex(ptr noalias noundef align 8 dereferenceable(24) %vector.i.i.i) #30, !noalias !38
  br label %bb8.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i
  %self1.sroa.6.0._21.i.i.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.val1315.i.i.i.i.i, i64 8
  %_3.sroa.6.sroa.0.0.copyload.i.i.i.i.i = load ptr, ptr %self1.sroa.6.0._21.i.i.sroa_idx.i.i.i.i.i.i, align 8, !noalias !96
  %_3.sroa.6.sroa.5.0.self1.sroa.6.0._21.i.i.sroa_idx.i.sroa_idx.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iterator.val1315.i.i.i.i.i, i64 16
  %_3.sroa.6.sroa.5.0.copyload.i.i.i.i.i = load i64, ptr %_3.sroa.6.sroa.5.0.self1.sroa.6.0._21.i.i.sroa_idx.i.sroa_idx.i.i.i.i.i, align 8, !noalias !96
  %_19.i.i.i.i.i = icmp samesign ult i64 %len.i.i.i.i.i, 384307168202282326
  tail call void @llvm.assume(i1 %_19.i.i.i.i.i)
  %self1.i.i.i.i.i = load i64, ptr %vector.i.i.i, align 8, !range !6, !alias.scope !97, !noalias !98, !noundef !2
  %_8.i.i.i.i.i = icmp eq i64 %len.i.i.i.i.i, %self1.i.i.i.i.i
  br i1 %_8.i.i.i.i.i, label %bb1.i.i.i.i.i.i, label %bb8.i.i.i.i.i

bb9.i.i.i.i.loopexit.i:                           ; preds = %bb8.i.i.i.i.i, %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i
  %_3.sroa.6.0.copyload413.i = phi i64 [ %len.i.i.i.i.i, %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i ], [ %new_len.i.i.i.i.i, %bb8.i.i.i.i.i ]
  %.pre.i = ptrtoint ptr %_23.i.i.i.i.i.i.i.i to i64
  %.pre15.i = sub nuw i64 %12, %.pre.i
  %.pre17.i = udiv exact i64 %.pre15.i, 24
  br label %bb9.i.i.i.i.i

bb9.i.i.i.i.i:                                    ; preds = %bb9.i.i.i.i.loopexit.i, %bb15.i.i.i
  %.pre-phi18.i = phi i64 [ %.pre17.i, %bb9.i.i.i.i.loopexit.i ], [ 0, %bb15.i.i.i ]
  %_3.sroa.6.0.copyload4.i = phi i64 [ %_3.sroa.6.0.copyload413.i, %bb9.i.i.i.i.loopexit.i ], [ 1, %bb15.i.i.i ]
  %self.val.i.i.i.i.i.i.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.loopexit.i ], [ %_23.i.i.i.i.i.i, %bb15.i.i.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !99)
  %_710.i.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter1.sroa.4.0.copyload.i, %self.val.i.i.i.i.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %bb9.i.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i = phi i64 [ %19, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb9.i.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %self.val.i.i.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i
  %19 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !99, !noalias !102
  %20 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %20, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i:           ; preds = %bb5.i.i.i.i.i.i.i.i.i.i.i
  %21 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %21, align 8, !alias.scope !99, !noalias !102, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !113
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %19, %.pre-phi18.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i.i.i.i, %bb9.i.i.i.i.i
  %22 = icmp eq i64 %iter1.sroa.3.0.copyload.i, 0
  br i1 %22, label %bb5.i.i.i, label %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i.i.i:               ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i.i.i = mul nuw i64 %iter1.sroa.3.0.copyload.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %iter1.sroa.0.0.copyload.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !102
  br label %bb5.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i, %bb3.i.i.i.i.i
  %_23.i.i.i.i.i = phi ptr [ %_23.i.i.pre.i.i.i, %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i ], [ %_23.i.i25.i.i.i, %bb3.i.i.i.i.i ]
  %dst.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_23.i.i.i.i.i, i64 %len.i.i.i.i.i
  store i64 %self1.sroa.0.0.copyload2.i.i.i.i.i.i, ptr %dst.i.i.i.i.i, align 8, !noalias !114
  %src.sroa.4.0.dst.sroa_idx.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i.i, i64 8
  store ptr %_3.sroa.6.sroa.0.0.copyload.i.i.i.i.i, ptr %src.sroa.4.0.dst.sroa_idx.i.i.i.i.i, align 8, !noalias !114
  %src.sroa.5.0.dst.sroa_idx.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %dst.i.i.i.i.i, i64 16
  store i64 %_3.sroa.6.sroa.5.0.copyload.i.i.i.i.i, ptr %src.sroa.5.0.dst.sroa_idx.i.i.i.i.i, align 8, !noalias !114
  %new_len.i.i.i.i.i = add nuw nsw i64 %len.i.i.i.i.i, 1
  store i64 %new_len.i.i.i.i.i, ptr %vector1.sroa.6.0.vector.sroa_idx.i.i.i, align 8, !alias.scope !97, !noalias !98
  %_9.i.i.i.i.i.i.i.i = icmp eq ptr %_23.i.i.i.i.i.i.i.i, %iter1.sroa.4.0.copyload.i
  br i1 %_9.i.i.i.i.i.i.i.i, label %bb9.i.i.i.i.loopexit.i, label %_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next.exit.i.i.i.i.i.i

cleanup2.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i.i.i
  %23 = landingpad { ptr, i32 }
          cleanup
  store ptr %_23.i.i.i.i.i.i.i.i, ptr %_4.sroa.6.0._19.i.i.sroa_idx.i, align 8, !alias.scope !115, !noalias !116
  %24 = icmp eq i64 %self1.sroa.0.0.copyload2.i.i.i.i.i.i, 0
  br i1 %24, label %bb11.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %cleanup2.i.i.i.i.i
  %25 = icmp ne ptr %_3.sroa.6.sroa.0.0.copyload.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %25)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.sroa.6.sroa.0.0.copyload.i.i.i.i.i, i64 noundef %self1.sroa.0.0.copyload2.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !114
  br label %bb11.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i
  %26 = ptrtoint ptr %_23.i.i.i.i.i.i.i.i to i64
  %27 = sub nuw i64 %12, %26
  %28 = udiv exact i64 %27, 24
  %29 = add nuw nsw i64 %28, 1
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1TOCnChXxQ_8clap_lex(ptr noalias noundef nonnull align 8 dereferenceable(24) %vector.i.i.i, i64 noundef %len.i.i.i.i.i, i64 noundef range(i64 1, 0) %29)
          to label %bb1.i.i.i.bb8.i.i_crit_edge.i.i.i unwind label %cleanup2.i.i.i.i.i

bb1.i.i.i.bb8.i.i_crit_edge.i.i.i:                ; preds = %bb1.i.i.i.i.i.i
  %_23.i.i.pre.i.i.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i, align 8, !alias.scope !97, !noalias !98
  br label %bb8.i.i.i.i.i

bb5.i.i.i:                                        ; preds = %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_19.i.i.i), !noalias !38
  %_3.sroa.0.0.copyload2.i = load i64, ptr %vector.i.i.i, align 8, !noalias !119
  %_3.sroa.5.0.copyload3.i = load ptr, ptr %vector1.sroa.4.0.vector.sroa_idx.i.i.i, align 8, !noalias !119
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i.i.i), !noalias !38
  br label %_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_.exit

bb8.i.i.i:                                        ; preds = %bb2.i.i.i.i.i4.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i, %bb11.i.i.i.i.i
  %.pn12.i.i.i = phi { ptr, i32 } [ %23, %bb11.i.i.i.i.i ], [ %9, %bb2.i.i.i.i.i.i.i ], [ %9, %bb2.i.i.i.i.i4.i.i.i.i.i.i.i ]
  resume { ptr, i32 } %.pn12.i.i.i

bb10.i.i.i:                                       ; preds = %bb2.i.i.i4.i.i.i.i.i.i, %cleanup2.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !120), !noalias !123
  %_710.i.i.i.i.i.i.i.i = icmp eq ptr %iter1.sroa.4.0.copyload.i, %_23.i.i.i.i.i.i
  br i1 %_710.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i:                              ; preds = %bb10.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i.i.i.i = phi i64 [ %30, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i ], [ 0, %bb10.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_23.i.i.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i
  %30 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i.i.i.i, 1
  %_6.val.i.i.i.i.i.i.i.i = load i64, ptr %_6.i.i.i.i.i.i.i.i, align 8, !alias.scope !120, !noalias !124
  %31 = icmp eq i64 %_6.val.i.i.i.i.i.i.i.i, 0
  br i1 %31, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i:                 ; preds = %bb5.i.i.i.i.i.i.i.i
  %32 = getelementptr i8, ptr %_6.i.i.i.i.i.i.i.i, i64 8
  %_6.val7.i.i.i.i.i.i.i.i = load ptr, ptr %32, align 8, !alias.scope !120, !noalias !124, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val7.i.i.i.i.i.i.i.i, i64 noundef %_6.val.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !135
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i = icmp eq i64 %30, %15
  br i1 %_7.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex.exit.i.i.i.i.i.i.i.i, %bb10.i.i.i
  %33 = icmp eq i64 %iter1.sroa.3.0.copyload.i, 0
  br i1 %33, label %bb8.i.i.i, label %bb2.i.i.i.i.i4.i.i.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i.i.i:                     ; preds = %bb2.i.i.i.i.i.i.i
  %34 = icmp ne ptr %iter1.sroa.0.0.copyload.i, null
  tail call void @llvm.assume(i1 %34)
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i = mul nuw i64 %iter1.sroa.3.0.copyload.i, 24
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %iter1.sroa.0.0.copyload.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !124
  br label %bb8.i.i.i

_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_.exit: ; preds = %bb2.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i, %bb5.i.i.i
  %_3.sroa.6.0.i = phi i64 [ 0, %bb2.i.i.i.i.i.i.i.i ], [ 0, %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i ], [ %_3.sroa.6.0.copyload4.i, %bb5.i.i.i ]
  %_3.sroa.5.0.i = phi ptr [ inttoptr (i64 8 to ptr), %bb2.i.i.i.i.i.i.i.i ], [ inttoptr (i64 8 to ptr), %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i ], [ %_3.sroa.5.0.copyload3.i, %bb5.i.i.i ]
  %_3.sroa.0.0.i = phi i64 [ 0, %bb2.i.i.i.i.i.i.i.i ], [ 0, %bb2.i.i.i.i.i4.i.i.i.i.i.i.i.i ], [ %_3.sroa.0.0.copyload2.i, %bb5.i.i.i ]
  store i64 %_3.sroa.0.0.i, ptr %_0, align 8, !alias.scope !29, !noalias !32
  %_3.sroa.5.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_3.sroa.5.0.i, ptr %_3.sroa.5.0._0.sroa_idx.i, align 8, !alias.scope !29, !noalias !32
  %_3.sroa.6.0._0.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_3.sroa.6.0.i, ptr %_3.sroa.6.0._0.sroa_idx.i, align 8, !alias.scope !29, !noalias !32
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_1)
  ret void
}

; <clap_lex::RawArgs>::remaining
; Function Attrs: uwtable
define { ptr, ptr } @_RNvMCs1TOCnChXxQ_8clap_lexNtB2_7RawArgs9remaining(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 captures(none) dereferenceable(8) %cursor) unnamed_addr #6 {
start:
  %_4 = load i64, ptr %cursor, align 8, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7 = load i64, ptr %0, align 8, !noundef !2
  %_9 = icmp ugt i64 %_4, %_7
  br i1 %_9, label %bb1, label %bb2, !prof !136

bb2:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load ptr, ptr %1, align 8, !nonnull !2, !noundef !2
  %_15 = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_8, i64 %_4
  %_18 = getelementptr inbounds nuw %"std::ffi::os_str::OsString", ptr %_8, i64 %_7
  %_22 = icmp ult i64 %_7, 384307168202282326
  tail call void @llvm.assume(i1 %_22)
  store i64 %_7, ptr %cursor, align 8
  %2 = insertvalue { ptr, ptr } poison, ptr %_15, 0
  %3 = insertvalue { ptr, ptr } %2, ptr %_18, 1
  ret { ptr, ptr } %3

bb1:                                              ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %_4, i64 noundef %_7, i64 noundef %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_743d0ef24a5897c38fa9d15d15a1122b) #28
  unreachable
}

; <clap_lex::ParsedArg>::is_negative_number
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg18is_negative_number(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %_5.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !137)
  %_3.0.i = load ptr, ptr %self, align 8, !alias.scope !137, !noalias !140, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1.i = load i64, ptr %0, align 8, !alias.scope !137, !noalias !140, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !143
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0.i, i64 noundef %_3.1.i), !noalias !143
  %_7.i = load i64, ptr %_5.i, align 8, !range !27, !noalias !143, !noundef !2
  %1 = trunc nuw i64 %_7.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %_5.i, i64 8
  %_8.0.i = load ptr, ptr %2, align 8, !noalias !143, !nonnull !2, !align !142
  %3 = getelementptr inbounds nuw i8, ptr %_5.i, i64 16
  %_8.1.i = load i64, ptr %3, align 8, !noalias !143
  %_8.1.sink.i = select i1 %1, i64 %_3.1.i, i64 %_8.1.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !143
  %_4.not.i.i = icmp eq i64 %_8.1.i, 0
  %or.cond = select i1 %1, i1 true, i1 %_4.not.i.i
  br i1 %or.cond, label %bb6, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit.i: ; preds = %start
  %rhsc.i = load i8, ptr %_8.0.i, align 1, !alias.scope !144
  %4 = icmp eq i8 %rhsc.i, 45
  br i1 %4, label %bb4.i, label %bb6

bb4.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit.i
  %_22.i = add i64 %_8.1.i, -1
  %_27.i.i = getelementptr i8, ptr %_8.0.i, i64 %_8.1.i
  %_6.i.i22.i.i = icmp samesign eq i64 %_22.i, 0
  br i1 %_6.i.i22.i.i, label %bb6, label %bb4.preheader.i.i

bb4.preheader.i.i:                                ; preds = %bb4.i
  %_24.i = getelementptr inbounds nuw i8, ptr %_8.0.i, i64 1
  %5 = load i8, ptr %_24.i, align 1, !alias.scope !147, !noundef !2
  %6 = add i8 %5, -48
  %or.cond14.peel.i.i = icmp ult i8 %6, 10
  br i1 %or.cond14.peel.i.i, label %bb27.peel.i.i, label %bb6

bb27.peel.i.i:                                    ; preds = %bb4.preheader.i.i
  %_6.i.i.peel.i.i = icmp samesign eq i64 %_22.i, 1
  br i1 %_6.i.i.peel.i.i, label %bb6, label %bb4.i2.preheader.i

bb4.i2.preheader.i:                               ; preds = %bb27.peel.i.i
  %_16.i.i.peel.i.i = getelementptr inbounds nuw i8, ptr %_8.0.i, i64 2
  br label %bb4.i2.i

bb4.i2.i:                                         ; preds = %bb27.i.i, %bb4.i2.preheader.i
  %seen_dot.sroa.0.0.off027.i.i = phi i1 [ %seen_dot.sroa.0.1.off0.i.i, %bb27.i.i ], [ false, %bb4.i2.preheader.i ]
  %position_of_e.sroa.6.026.i.i = phi i64 [ %position_of_e.sroa.6.1.i.i, %bb27.i.i ], [ undef, %bb4.i2.preheader.i ]
  %position_of_e.sroa.0.025.i.i = phi i64 [ %position_of_e.sroa.0.1.i.i, %bb27.i.i ], [ 0, %bb4.i2.preheader.i ]
  %iter.sroa.0.024.i.i = phi ptr [ %_16.i.i.i.i, %bb27.i.i ], [ %_16.i.i.peel.i.i, %bb4.i2.preheader.i ]
  %iter.sroa.8.023.i.i = phi i64 [ %_8.0.i.i.i, %bb27.i.i ], [ 1, %bb4.i2.preheader.i ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.024.i.i, i64 1
  %_8.0.i.i.i = add nuw i64 %iter.sroa.8.023.i.i, 1
  %7 = load i8, ptr %iter.sroa.0.024.i.i, align 1, !alias.scope !147, !noundef !2
  %8 = add i8 %7, -48
  %or.cond14.i.i = icmp ult i8 %8, 10
  br i1 %or.cond14.i.i, label %bb27.i.i, label %bb7.i.i

bb5.i.i:                                          ; preds = %bb27.i.i
  %9 = trunc nuw i64 %position_of_e.sroa.0.1.i.i to i1
  br i1 %9, label %bb29.i.i, label %bb6

bb29.i.i:                                         ; preds = %bb5.i.i
  %_23.i.i = add i64 %_8.1.sink.i, -2
  %10 = icmp ne i64 %position_of_e.sroa.6.1.i.i, %_23.i.i
  br label %bb6

bb7.i.i:                                          ; preds = %bb4.i2.i
  switch i8 %7, label %bb6 [
    i8 46, label %bb11.i.i
    i8 101, label %bb10.i.i
    i8 69, label %bb9.i.i
  ]

bb27.i.i:                                         ; preds = %bb9.i.i, %bb10.i.i, %bb11.i.i, %bb4.i2.i
  %position_of_e.sroa.0.1.i.i = phi i64 [ %position_of_e.sroa.0.025.i.i, %bb4.i2.i ], [ 1, %bb9.i.i ], [ 1, %bb10.i.i ], [ %position_of_e.sroa.0.025.i.i, %bb11.i.i ]
  %position_of_e.sroa.6.1.i.i = phi i64 [ %position_of_e.sroa.6.026.i.i, %bb4.i2.i ], [ %iter.sroa.8.023.i.i, %bb9.i.i ], [ %iter.sroa.8.023.i.i, %bb10.i.i ], [ %position_of_e.sroa.6.026.i.i, %bb11.i.i ]
  %seen_dot.sroa.0.1.off0.i.i = phi i1 [ %seen_dot.sroa.0.0.off027.i.i, %bb4.i2.i ], [ %seen_dot.sroa.0.0.off027.i.i, %bb9.i.i ], [ %seen_dot.sroa.0.0.off027.i.i, %bb10.i.i ], [ true, %bb11.i.i ]
  %_6.i.i.i.i = icmp eq ptr %_16.i.i.i.i, %_27.i.i
  br i1 %_6.i.i.i.i, label %bb5.i.i, label %bb4.i2.i, !llvm.loop !150

bb11.i.i:                                         ; preds = %bb7.i.i
  %_35.not.i.i = icmp eq i64 %position_of_e.sroa.0.025.i.i, 1
  %or.cond.i.i = select i1 %seen_dot.sroa.0.0.off027.i.i, i1 true, i1 %_35.not.i.i
  br i1 %or.cond.i.i, label %bb6, label %bb27.i.i

bb10.i.i:                                         ; preds = %bb7.i.i
  %_33.not.i.i = icmp eq i64 %position_of_e.sroa.0.025.i.i, 1
  br i1 %_33.not.i.i, label %bb6, label %bb27.i.i

bb9.i.i:                                          ; preds = %bb7.i.i
  %_31.not.i.i = icmp eq i64 %position_of_e.sroa.0.025.i.i, 1
  br i1 %_31.not.i.i, label %bb6, label %bb27.i.i

bb6:                                              ; preds = %bb9.i.i, %bb10.i.i, %bb11.i.i, %bb7.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit.i, %bb29.i.i, %bb5.i.i, %bb27.peel.i.i, %bb4.preheader.i.i, %bb4.i, %start
  %_0.sroa.0.0.off0 = phi i1 [ false, %start ], [ false, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit.i ], [ %10, %bb29.i.i ], [ true, %bb5.i.i ], [ true, %bb4.i ], [ true, %bb27.peel.i.i ], [ false, %bb4.preheader.i.i ], [ false, %bb7.i.i ], [ false, %bb11.i.i ], [ false, %bb10.i.i ], [ false, %bb9.i.i ]
  ret i1 %_0.sroa.0.0.off0
}

; <clap_lex::ParsedArg>::display
; Function Attrs: uwtable
define void @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg7display(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #6 {
start:
  %_2.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_2.1 = load i64, ptr %0, align 8, !noundef !2
; call <alloc::string::String>::from_utf8_lossy
  tail call void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_2.0, i64 noundef %_2.1)
  ret void
}

; <clap_lex::ParsedArg>::is_long
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(read, inaccessiblemem: none) uwtable
define noundef zeroext i1 @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg7is_long(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #7 {
start:
  %_4.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1 = load i64, ptr %0, align 8, !noundef !2
  %_4.not.i.i = icmp samesign ult i64 %_4.1, 2
  br i1 %_4.not.i.i, label %bb5, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit

_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit: ; preds = %start
  %1 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_05ac4674d88601cc843f438cc0a6c56a, ptr noundef nonnull readonly align 1 dereferenceable(2) %_4.0, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !152
  %2 = icmp eq i32 %1, 0
  br i1 %2, label %bb2, label %bb5

bb2:                                              ; preds = %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  %_3.not.i.i = icmp eq i64 %_4.1, 2
  br i1 %_3.not.i.i, label %bb2.i.i, label %bb5

bb2.i.i:                                          ; preds = %bb2
  %3 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_4.0, ptr noundef nonnull dereferenceable(2) @alloc_05ac4674d88601cc843f438cc0a6c56a, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !162, !noalias !166
  %4 = icmp ne i32 %3, 0
  br label %bb5

bb5:                                              ; preds = %bb2.i.i, %bb2, %start, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  %_0.sroa.0.0.off0 = phi i1 [ false, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit ], [ false, %start ], [ %4, %bb2.i.i ], [ true, %bb2 ]
  ret i1 %_0.sroa.0.0.off0
}

; <clap_lex::ParsedArg>::to_long
; Function Attrs: uwtable
define void @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg7to_long(ptr dead_on_unwind noalias noundef writable writeonly sret([40 x i8]) align 8 captures(none) dereferenceable(40) initializes((0, 8)) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #6 {
start:
  %_17 = alloca [24 x i8], align 8
  %_6 = alloca [32 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %raw.1 = load i64, ptr %0, align 8, !noundef !2
  %_5.not.i.i = icmp samesign ult i64 %raw.1, 2
  br i1 %_5.not.i.i, label %bb10, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i: ; preds = %start
  %raw.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %raw.0, i64 2
  %len.i.i.i.i = add nsw i64 %raw.1, -2
  %1 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %raw.0, ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_05ac4674d88601cc843f438cc0a6c56a, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !169
  %2 = icmp eq i32 %1, 0
  br i1 %2, label %bb11, label %bb10

bb11:                                             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i
  %3 = icmp eq i64 %len.i.i.i.i, 0
  br i1 %3, label %bb3, label %bb4

bb10:                                             ; preds = %start, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i
  store i64 2, ptr %_0, align 8
  br label %bb9

bb9:                                              ; preds = %bb8, %bb3, %bb10
  ret void

bb3:                                              ; preds = %bb11
  store i64 2, ptr %_0, align 8
  br label %bb9

bb4:                                              ; preds = %bb11
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_6)
; call <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::split_once
  call void @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt10split_once(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i.i.i, i64 noundef %len.i.i.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_219cd7325be98984893e9f7eb51114d4, i64 noundef 1)
  %4 = load ptr, ptr %_6, align 8, !noundef !2
  %.not6 = icmp eq ptr %4, null
  br i1 %.not6, label %bb8, label %bb6

bb6:                                              ; preds = %bb4
  %5 = getelementptr inbounds nuw i8, ptr %_6, i64 8
  %6 = load i64, ptr %5, align 8, !noundef !2
  %7 = getelementptr inbounds nuw i8, ptr %_6, i64 16
  %p1.0 = load ptr, ptr %7, align 8, !nonnull !2, !align !142, !noundef !2
  %8 = getelementptr inbounds nuw i8, ptr %_6, i64 24
  %p1.1 = load i64, ptr %8, align 8, !noundef !2
  br label %bb8

bb8:                                              ; preds = %bb4, %bb6
  %value.sroa.0.0 = phi ptr [ %p1.0, %bb6 ], [ null, %bb4 ]
  %remainder.sroa.6.0 = phi i64 [ %6, %bb6 ], [ %len.i.i.i.i, %bb4 ]
  %remainder.sroa.0.0 = phi ptr [ %4, %bb6 ], [ %data.i.i.i.i, %bb4 ]
  %value.sroa.3.0 = phi i64 [ %p1.1, %bb6 ], [ undef, %bb4 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_6)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_17)
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_17, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %remainder.sroa.0.0, i64 noundef %remainder.sroa.6.0)
  %_19 = load i64, ptr %_17, align 8, !range !27, !noundef !2
  %9 = trunc nuw i64 %_19 to i1
  %10 = getelementptr inbounds nuw i8, ptr %_17, i64 8
  %_20.0 = load ptr, ptr %10, align 8, !nonnull !2, !align !142
  %11 = getelementptr inbounds nuw i8, ptr %_17, i64 16
  %_20.1 = load i64, ptr %11, align 8
  %flag.sroa.3.0 = select i1 %9, ptr %remainder.sroa.0.0, ptr %_20.0
  %flag.sroa.5.0 = select i1 %9, i64 %remainder.sroa.6.0, i64 %_20.1
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_17)
  store i64 %_19, ptr %_0, align 8
  %_12.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %flag.sroa.3.0, ptr %_12.sroa.4.0._0.sroa_idx, align 8
  %_12.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %flag.sroa.5.0, ptr %_12.sroa.5.0._0.sroa_idx, align 8
  %_12.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store ptr %value.sroa.0.0, ptr %_12.sroa.6.0._0.sroa_idx, align 8
  %_12.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 %value.sroa.3.0, ptr %_12.sroa.7.0._0.sroa_idx, align 8
  br label %bb9
}

; <clap_lex::ParsedArg>::is_short
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(read, inaccessiblemem: none) uwtable
define noundef zeroext i1 @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8is_short(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #7 {
start:
  %_5.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_5.1 = load i64, ptr %0, align 8, !noundef !2
  %_4.not.i.i = icmp eq i64 %_5.1, 0
  br i1 %_4.not.i.i, label %bb7, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit

_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit: ; preds = %start
  %rhsc = load i8, ptr %_5.0, align 1
  %1 = icmp ne i8 %rhsc, 45
  %_3.not.i.i = icmp eq i64 %_5.1, 1
  %or.cond = or i1 %1, %_3.not.i.i
  br i1 %or.cond, label %bb7, label %bb4.i.i3

bb4.i.i3:                                         ; preds = %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  %2 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) @alloc_05ac4674d88601cc843f438cc0a6c56a, ptr noundef nonnull readonly align 1 dereferenceable(2) %_5.0, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !179
  %3 = icmp ne i32 %2, 0
  br label %bb7

bb7:                                              ; preds = %bb4.i.i3, %start, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  %_0.sroa.0.0.off0 = phi i1 [ false, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit ], [ false, %start ], [ %3, %bb4.i.i3 ]
  ret i1 %_0.sroa.0.0.off0
}

; <clap_lex::ParsedArg>::is_stdio
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(read, inaccessiblemem: none) uwtable
define noundef zeroext i1 @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8is_stdio(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #3 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
  %_3.not.i = icmp eq i64 %_3.1, 1
  br i1 %_3.not.i, label %bb2.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit

bb2.i:                                            ; preds = %start
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %lhsc = load i8, ptr %_3.0, align 1
  %1 = icmp eq i8 %lhsc, 45
  br label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit: ; preds = %start, %bb2.i
  %_0.sroa.0.0.off0.i = phi i1 [ %1, %bb2.i ], [ false, %start ]
  ret i1 %_0.sroa.0.0.off0.i
}

; <clap_lex::ParsedArg>::to_short
; Function Attrs: uwtable
define void @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8to_short(ptr dead_on_unwind noalias noundef writable writeonly sret([56 x i8]) align 8 captures(none) dereferenceable(56) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %e.i.i.i = alloca [16 x i8], align 8
  %_9.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_7.1 = load i64, ptr %0, align 8, !noundef !2
  %_5.not.i.i = icmp eq i64 %_7.1, 0
  br i1 %_5.not.i.i, label %bb11, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i: ; preds = %start
  %_7.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %data.i.i.i.i = getelementptr inbounds nuw i8, ptr %_7.0, i64 1
  %lhsc = load i8, ptr %_7.0, align 1
  %1 = icmp eq i8 %lhsc, 45
  br i1 %1, label %bb2, label %bb11

bb2:                                              ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i
  %len.i.i.i.i = add nsw i64 %_7.1, -1
  %_4.not.i.i = icmp eq i64 %len.i.i.i.i, 0
  br i1 %_4.not.i.i, label %bb6, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit

_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit: ; preds = %bb2
  %rhsc = load i8, ptr %data.i.i.i.i, align 1
  %2 = icmp eq i8 %rhsc, 45
  br i1 %2, label %bb4, label %bb7

bb11:                                             ; preds = %start, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i.i
  store ptr null, ptr %_0, align 8
  br label %bb12

bb4:                                              ; preds = %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  store ptr null, ptr %_0, align 8
  br label %bb12

bb6:                                              ; preds = %bb2
  store ptr null, ptr %_0, align 8
  br label %bb12

bb7:                                              ; preds = %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i.i), !noalias !189
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_2.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i.i.i, i64 noundef range(i64 1, 0) %len.i.i.i.i), !noalias !196
  %_3.i.i = load i64, ptr %_2.i.i, align 8, !range !27, !noalias !189, !noundef !2
  %3 = trunc nuw i64 %_3.i.i to i1
  %4 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 8
  br i1 %3, label %bb3.i.i, label %bb4.i.i2

bb3.i.i:                                          ; preds = %bb7
  %err.i.i = load i64, ptr %4, align 8, !noalias !189, !noundef !2
  %_6.not.i.i.i.i = icmp ugt i64 %err.i.i, %len.i.i.i.i
  br i1 %_6.not.i.i.i.i, label %bb3.i.i.i.i, label %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit.i.i, !prof !136

bb3.i.i.i.i:                                      ; preds = %bb3.i.i
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_80a360ff56096edb1899e5a3621ce404) #28, !noalias !197
  unreachable

_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit.i.i: ; preds = %bb3.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_9.i.i), !noalias !189
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9.i.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i.i.i, i64 noundef %err.i.i), !noalias !196
  tail call void @llvm.experimental.noalias.scope.decl(metadata !204)
  %_2.i.i.i = load i64, ptr %_9.i.i, align 8, !range !27, !alias.scope !204, !noalias !189, !noundef !2
  %5 = trunc nuw i64 %_2.i.i.i to i1
  br i1 %5, label %bb2.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i, !prof !136

bb2.i.i.i:                                        ; preds = %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit.i.i
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %e.i.i.i), !noalias !207
  %6 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %e.i.i.i, ptr noundef nonnull readonly align 8 dereferenceable(16) %6, i64 16, i1 false), !noalias !189
; call core::result::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_00ae4b301f7fab8ac9617c03fcbd7274, i64 noundef 43, ptr noundef nonnull align 1 %e.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f7c6bd120cdd150e1d7183e14240e646) #29, !noalias !208
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i: ; preds = %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit.i.i
  %len.i.i.i.i.i = sub nuw nsw i64 %len.i.i.i.i, %err.i.i
  %data.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i.i.i.i, i64 %err.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 8
  %t.0.i.i.i = load ptr, ptr %7, align 8, !alias.scope !204, !noalias !189, !nonnull !2, !align !142, !noundef !2
  %8 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 16
  %t.1.i.i.i = load i64, ptr %8, align 8, !alias.scope !204, !noalias !189, !noundef !2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_9.i.i), !noalias !189
  br label %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new.exit

bb4.i.i2:                                         ; preds = %bb7
  %s.0.i.i = load ptr, ptr %4, align 8, !noalias !189, !nonnull !2, !align !142, !noundef !2
  %9 = getelementptr inbounds nuw i8, ptr %_2.i.i, i64 16
  %s.1.i.i = load i64, ptr %9, align 8, !noalias !189, !noundef !2
  br label %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new.exit

_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new.exit: ; preds = %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i, %bb4.i.i2
  %_4.sroa.11.0.i = phi i64 [ %len.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i ], [ undef, %bb4.i.i2 ]
  %_4.sroa.8.0.i = phi ptr [ %data.i.i.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i ], [ null, %bb4.i.i2 ]
  %_4.sroa.5.0.i = phi i64 [ %t.1.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i ], [ %s.1.i.i, %bb4.i.i2 ]
  %_4.sroa.0.0.i = phi ptr [ %t.0.i.i.i, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex.exit.i.i ], [ %s.0.i.i, %bb4.i.i2 ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i.i), !noalias !189
  %_12.i = getelementptr inbounds nuw i8, ptr %_4.sroa.0.0.i, i64 %_4.sroa.5.0.i
  store ptr %data.i.i.i.i, ptr %_0, align 8
  %_6.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %len.i.i.i.i, ptr %_6.sroa.4.0._0.sroa_idx, align 8
  %_6.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %_4.sroa.0.0.i, ptr %_6.sroa.5.0._0.sroa_idx, align 8
  %_6.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store ptr %_12.i, ptr %_6.sroa.6.0._0.sroa_idx, align 8
  %_6.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 0, ptr %_6.sroa.7.0._0.sroa_idx, align 8
  %_6.sroa.8.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  store ptr %_4.sroa.8.0.i, ptr %_6.sroa.8.0._0.sroa_idx, align 8
  %_6.sroa.9.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 48
  store i64 %_4.sroa.11.0.i, ptr %_6.sroa.9.0._0.sroa_idx, align 8
  br label %bb12

bb12:                                             ; preds = %bb4, %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new.exit, %bb6, %bb11
  ret void
}

; <clap_lex::ParsedArg>::to_value
; Function Attrs: uwtable
define void @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8to_value(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) initializes((0, 24)) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #6 {
start:
  %_5 = alloca [24 x i8], align 8
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1)
  %_7 = load i64, ptr %_5, align 8, !range !27, !noundef !2
  %1 = trunc nuw i64 %_7 to i1
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %_8.0 = load ptr, ptr %2, align 8, !nonnull !2, !align !142
  %3 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %_8.1 = load i64, ptr %3, align 8
  %_8.0.sink = select i1 %1, ptr %_3.0, ptr %_8.0
  %_8.1.sink = select i1 %1, i64 %_3.1, i64 %_8.1
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_8.0.sink, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_8.1.sink, ptr %5, align 8
  store i64 %_7, ptr %_0, align 8
  ret void
}

; <clap_lex::ParsedArg>::is_escape
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(read, inaccessiblemem: none) uwtable
define noundef zeroext i1 @_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg9is_escape(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #7 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
  %_3.not.i = icmp eq i64 %_3.1, 2
  br i1 %_3.not.i, label %bb2.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit

bb2.i:                                            ; preds = %start
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %1 = tail call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(2) %_3.0, ptr noundef nonnull dereferenceable(2) @alloc_05ac4674d88601cc843f438cc0a6c56a, i64 range(i64 0, -9223372036854775808) 2), !alias.scope !209
  %2 = icmp eq i32 %1, 0
  br label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit: ; preds = %start, %bb2.i
  %_0.sroa.0.0.off0.i = phi i1 [ %2, %bb2.i ], [ false, %start ]
  ret i1 %_0.sroa.0.0.off0.i
}

; <clap_lex::ShortFlags>::advance_by
; Function Attrs: nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define { i64, i64 } @_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags10advance_by(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self, i64 noundef %n) unnamed_addr #8 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %.promoted = load ptr, ptr %0, align 8
  %_3.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.i3.i.i.i = load ptr, ptr %1, align 8, !nonnull !2
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %exitcond.not21 = icmp eq i64 %n, 0
  br i1 %exitcond.not21, label %bb5, label %bb6.lr.ph

bb6.lr.ph:                                        ; preds = %start
  %.promoted15 = load i64, ptr %2, align 8
  %_3.i.i.promoted = load ptr, ptr %_3.i.i, align 8
  br label %bb6

bb6:                                              ; preds = %bb6.lr.ph, %bb9
  %iter.sroa.0.023 = phi i64 [ 0, %bb6.lr.ph ], [ %4, %bb9 ]
  %_16.i26.i.i.i.i1422 = phi ptr [ %_3.i.i.promoted, %bb6.lr.ph ], [ %subtracted.i.i.i.i, %bb9 ]
  %3 = phi i64 [ %.promoted15, %bb6.lr.ph ], [ %13, %bb9 ]
  %4 = add i64 %iter.sroa.0.023, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !213)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !216)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !219)
  %5 = ptrtoint ptr %_16.i26.i.i.i.i1422 to i64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !222)
  %_6.i.i.i.i.i.not = icmp eq ptr %_16.i26.i.i.i.i1422, %_4.i3.i.i.i
  br i1 %_6.i.i.i.i.i.not, label %bb3.i.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb6
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i1422, i64 1
  store ptr %_16.i.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !225, !noalias !228
  %x.i.i.i.i = load i8, ptr %_16.i26.i.i.i.i1422, align 1, !noalias !231, !noundef !2
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i1422, i64 2
  store ptr %_16.i12.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !232, !noalias !228
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !231, !noundef !2
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i.i, label %bb9

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb9

bb6.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i1422, i64 3
  store ptr %_16.i19.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !235, !noalias !228
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !231, !noundef !2
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %7 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb9

bb8.i.i.i.i:                                      ; preds = %bb6.i.i.i.i
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_4.i3.i.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i.i1422, i64 4
  store ptr %_16.i26.i.i.i.i, ptr %_3.i.i, align 8, !alias.scope !238, !noalias !228
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !231, !noundef !2
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %8 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb9

bb3.i.i:                                          ; preds = %bb6
  %.not2.i.i = icmp eq ptr %.promoted, null
  br i1 %.not2.i.i, label %bb5, label %bb9.thread

bb9.thread:                                       ; preds = %bb3.i.i
  store ptr null, ptr %0, align 8, !alias.scope !241, !noalias !228
  br label %bb5

bb5:                                              ; preds = %bb9, %start, %bb9.thread, %bb3.i.i
  %iter.sroa.0.0.lcssa = phi i64 [ %iter.sroa.0.023, %bb3.i.i ], [ %iter.sroa.0.023, %bb9.thread ], [ %n, %start ], [ %n, %bb9 ]
  %_0.sroa.0.0 = phi i64 [ 1, %bb3.i.i ], [ 1, %bb9.thread ], [ 0, %start ], [ 0, %bb9 ]
  %9 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.0, 0
  %10 = insertvalue { i64, i64 } %9, i64 %iter.sroa.0.0.lcssa, 1
  ret { i64, i64 } %10

bb9:                                              ; preds = %bb4.i.i.i.i, %bb3.i.i.i.i, %bb6.i.i.i.i, %bb8.i.i.i.i
  %subtracted.i.i.i.i = phi ptr [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i.i ], [ %_16.i26.i.i.i.i, %bb8.i.i.i.i ], [ %_16.i.i.i.i.i, %bb3.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %6, %bb4.i.i.i.i ], [ %7, %bb6.i.i.i.i ], [ %8, %bb8.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i.i ]
  %11 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  tail call void @llvm.assume(i1 %11)
  %12 = ptrtoint ptr %subtracted.i.i.i.i to i64
  %_10.i.i.i = sub i64 %12, %5
  %13 = add i64 %_10.i.i.i, %3
  store i64 %13, ptr %2, align 8, !alias.scope !242, !noalias !228
  %exitcond.not = icmp eq i64 %4, %n
  br i1 %exitcond.not, label %bb5, label %bb6
}

; <clap_lex::ShortFlags>::next_value_os
; Function Attrs: uwtable
define { ptr, i64 } @_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags13next_value_os(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self) unnamed_addr #6 {
start:
  %_3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !243)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.i3.i = load ptr, ptr %0, align 8, !alias.scope !246, !nonnull !2, !noundef !2
  %subtracted.i4.i = load ptr, ptr %_3, align 8, !alias.scope !246, !nonnull !2, !noundef !2
  %_6.i.i.i = icmp eq ptr %subtracted.i4.i, %_4.i3.i
  br i1 %_6.i.i.i, label %bb4, label %bb14.i.i

bb14.i.i:                                         ; preds = %start
  %x.i.i = load i8, ptr %subtracted.i4.i, align 1, !noalias !249, !noundef !2
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 1
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i10.i.i)
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !noalias !249, !noundef !2
  %_33.i.i = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %1 = or disjoint i32 %_33.i.i, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i, label %bb2

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb2

bb6.i.i:                                          ; preds = %bb4.i.i
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 2
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i17.i.i)
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !noalias !249, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %2 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb2

bb8.i.i:                                          ; preds = %bb6.i.i
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 3
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i24.i.i)
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !noalias !249, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %3 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb2

bb2:                                              ; preds = %bb8.i.i, %bb6.i.i, %bb3.i.i, %bb4.i.i
  %_0.sroa.4.0.i.ph.i = phi i32 [ %1, %bb4.i.i ], [ %2, %bb6.i.i ], [ %3, %bb8.i.i ], [ %_7.i.i, %bb3.i.i ]
  %4 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  tail call void @llvm.assume(i1 %4)
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %index.i = load i64, ptr %5, align 8, !alias.scope !243, !noundef !2
  store ptr inttoptr (i64 1 to ptr), ptr %_3, align 8
  store ptr inttoptr (i64 1 to ptr), ptr %0, align 8
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 8
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %5, i8 0, i64 16, i1 false)
  %_11.1 = load i64, ptr %6, align 8, !noundef !2
  %_6.not.i.i = icmp ugt i64 %index.i, %_11.1
  br i1 %_6.not.i.i, label %bb3.i.i3, label %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit, !prof !136

bb3.i.i3:                                         ; preds = %bb2
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_80a360ff56096edb1899e5a3621ce404) #28, !noalias !252
  unreachable

_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit:   ; preds = %bb2
  %_11.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %_11.0, i64 %index.i
  %len.i.i.i = sub nuw nsw i64 %_11.1, %index.i
  br label %bb7

bb4:                                              ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %8 = load ptr, ptr %7, align 8, !align !142, !noundef !2
  %.not2 = icmp eq ptr %8, null
  br i1 %.not2, label %bb7, label %bb6

bb7:                                              ; preds = %bb4, %bb6, %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit
  %_0.sroa.4.0 = phi i64 [ %len.i.i.i, %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit ], [ %suffix.1, %bb6 ], [ undef, %bb4 ]
  %_0.sroa.0.0 = phi ptr [ %data.i.i.i, %_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at.exit ], [ %8, %bb6 ], [ null, %bb4 ]
  %9 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0, 0
  %10 = insertvalue { ptr, i64 } %9, i64 %_0.sroa.4.0, 1
  ret { ptr, i64 } %10

bb6:                                              ; preds = %bb4
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %suffix.1 = load i64, ptr %11, align 8, !noundef !2
  store ptr null, ptr %7, align 8
  br label %bb7
}

; <clap_lex::ShortFlags>::is_negative_number
; Function Attrs: nofree norecurse nosync nounwind memory(read, inaccessiblemem: none) uwtable
define noundef zeroext i1 @_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags18is_negative_number(ptr noalias noundef readonly align 8 captures(none) dereferenceable(56) %self) unnamed_addr #9 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %1 = load ptr, ptr %0, align 8, !align !142, !noundef !2
  %.not = icmp eq ptr %1, null
  br i1 %.not, label %bb1, label %bb3

bb1:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_8 = load ptr, ptr %2, align 8, !nonnull !2, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_11 = load ptr, ptr %3, align 8, !nonnull !2, !noundef !2
  %4 = ptrtoint ptr %_11 to i64
  %5 = ptrtoint ptr %_8 to i64
  %6 = sub nuw i64 %4, %5
  %_6.i.i22.i = icmp eq ptr %_11, %_8
  br i1 %_6.i.i22.i, label %bb3, label %bb4.preheader.i

bb4.preheader.i:                                  ; preds = %bb1
  %7 = load i8, ptr %_8, align 1, !alias.scope !259, !noundef !2
  %8 = add i8 %7, -48
  %or.cond14.peel.i = icmp ult i8 %8, 10
  br i1 %or.cond14.peel.i, label %bb27.peel.i, label %bb3

bb27.peel.i:                                      ; preds = %bb4.preheader.i
  %_6.i.i.peel.i = icmp samesign eq i64 %6, 1
  br i1 %_6.i.i.peel.i, label %bb3, label %bb4.i.preheader

bb4.i.preheader:                                  ; preds = %bb27.peel.i
  %_16.i.i.peel.i = getelementptr inbounds nuw i8, ptr %_8, i64 1
  br label %bb4.i

bb4.i:                                            ; preds = %bb4.i.preheader, %bb27.i
  %seen_dot.sroa.0.0.off027.i = phi i1 [ %seen_dot.sroa.0.1.off0.i, %bb27.i ], [ false, %bb4.i.preheader ]
  %position_of_e.sroa.6.026.i = phi i64 [ %position_of_e.sroa.6.1.i, %bb27.i ], [ undef, %bb4.i.preheader ]
  %position_of_e.sroa.0.025.i = phi i64 [ %position_of_e.sroa.0.1.i, %bb27.i ], [ 0, %bb4.i.preheader ]
  %iter.sroa.0.024.i = phi ptr [ %_16.i.i.i, %bb27.i ], [ %_16.i.i.peel.i, %bb4.i.preheader ]
  %iter.sroa.8.023.i = phi i64 [ %_8.0.i.i, %bb27.i ], [ 1, %bb4.i.preheader ]
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.024.i, i64 1
  %_8.0.i.i = add nuw i64 %iter.sroa.8.023.i, 1
  %9 = load i8, ptr %iter.sroa.0.024.i, align 1, !alias.scope !259, !noundef !2
  %10 = add i8 %9, -48
  %or.cond14.i = icmp ult i8 %10, 10
  br i1 %or.cond14.i, label %bb27.i, label %bb7.i

bb5.i:                                            ; preds = %bb27.i
  %11 = trunc nuw i64 %position_of_e.sroa.0.1.i to i1
  br i1 %11, label %bb29.i, label %bb3

bb29.i:                                           ; preds = %bb5.i
  %_23.i = add i64 %6, -1
  %12 = icmp ne i64 %position_of_e.sroa.6.1.i, %_23.i
  br label %bb3

bb7.i:                                            ; preds = %bb4.i
  switch i8 %9, label %bb3 [
    i8 46, label %bb11.i
    i8 101, label %bb10.i
    i8 69, label %bb9.i
  ]

bb27.i:                                           ; preds = %bb9.i, %bb10.i, %bb11.i, %bb4.i
  %position_of_e.sroa.0.1.i = phi i64 [ %position_of_e.sroa.0.025.i, %bb4.i ], [ 1, %bb9.i ], [ 1, %bb10.i ], [ %position_of_e.sroa.0.025.i, %bb11.i ]
  %position_of_e.sroa.6.1.i = phi i64 [ %position_of_e.sroa.6.026.i, %bb4.i ], [ %iter.sroa.8.023.i, %bb9.i ], [ %iter.sroa.8.023.i, %bb10.i ], [ %position_of_e.sroa.6.026.i, %bb11.i ]
  %seen_dot.sroa.0.1.off0.i = phi i1 [ %seen_dot.sroa.0.0.off027.i, %bb4.i ], [ %seen_dot.sroa.0.0.off027.i, %bb9.i ], [ %seen_dot.sroa.0.0.off027.i, %bb10.i ], [ true, %bb11.i ]
  %_6.i.i.i = icmp eq ptr %_16.i.i.i, %_11
  br i1 %_6.i.i.i, label %bb5.i, label %bb4.i, !llvm.loop !150

bb11.i:                                           ; preds = %bb7.i
  %_35.not.i = icmp eq i64 %position_of_e.sroa.0.025.i, 1
  %or.cond.i = select i1 %seen_dot.sroa.0.0.off027.i, i1 true, i1 %_35.not.i
  br i1 %or.cond.i, label %bb3, label %bb27.i

bb10.i:                                           ; preds = %bb7.i
  %_33.not.i = icmp eq i64 %position_of_e.sroa.0.025.i, 1
  br i1 %_33.not.i, label %bb3, label %bb27.i

bb9.i:                                            ; preds = %bb7.i
  %_31.not.i = icmp eq i64 %position_of_e.sroa.0.025.i, 1
  br i1 %_31.not.i, label %bb3, label %bb27.i

bb3:                                              ; preds = %bb9.i, %bb10.i, %bb11.i, %bb7.i, %bb29.i, %bb5.i, %bb27.peel.i, %bb4.preheader.i, %bb1, %start
  %_0.sroa.0.0.off0 = phi i1 [ false, %start ], [ %12, %bb29.i ], [ true, %bb5.i ], [ true, %bb1 ], [ true, %bb27.peel.i ], [ false, %bb4.preheader.i ], [ false, %bb7.i ], [ false, %bb11.i ], [ false, %bb10.i ], [ false, %bb9.i ]
  ret i1 %_0.sroa.0.0.off0
}

; <clap_lex::ShortFlags>::next_flag
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define void @_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, ptr noalias noundef align 8 captures(none) dereferenceable(56) %self) unnamed_addr #10 {
start:
  %_3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !262)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.i3.i = load ptr, ptr %0, align 8, !alias.scope !265, !nonnull !2, !noundef !2
  %subtracted.i4.i = load ptr, ptr %_3, align 8, !alias.scope !265, !nonnull !2, !noundef !2
  %1 = ptrtoint ptr %subtracted.i4.i to i64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !268)
  %_6.i.i.i = icmp eq ptr %subtracted.i4.i, %_4.i3.i
  br i1 %_6.i.i.i, label %bb3, label %bb14.i.i

bb14.i.i:                                         ; preds = %start
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 1
  store ptr %_16.i.i.i, ptr %_3, align 8, !alias.scope !271
  %x.i.i = load i8, ptr %subtracted.i4.i, align 1, !noalias !274, !noundef !2
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i10.i.i)
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 2
  store ptr %_16.i12.i.i, ptr %_3, align 8, !alias.scope !275
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !noalias !274, !noundef !2
  %_33.i.i = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %2 = or disjoint i32 %_33.i.i, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i, label %bb2

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb2

bb6.i.i:                                          ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i17.i.i)
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 3
  store ptr %_16.i19.i.i, ptr %_3, align 8, !alias.scope !278
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !noalias !274, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %3 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb2

bb8.i.i:                                          ; preds = %bb6.i.i
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %_4.i3.i
  tail call void @llvm.assume(i1 %_6.i24.i.i)
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i, i64 4
  store ptr %_16.i26.i.i, ptr %_3, align 8, !alias.scope !281
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !noalias !274, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %4 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb2

bb2:                                              ; preds = %bb8.i.i, %bb6.i.i, %bb3.i.i, %bb4.i.i
  %subtracted.i.i = phi ptr [ %_16.i12.i.i, %bb4.i.i ], [ %_16.i19.i.i, %bb6.i.i ], [ %_16.i26.i.i, %bb8.i.i ], [ %_16.i.i.i, %bb3.i.i ]
  %_0.sroa.4.0.i.ph.i = phi i32 [ %2, %bb4.i.i ], [ %3, %bb6.i.i ], [ %4, %bb8.i.i ], [ %_7.i.i, %bb3.i.i ]
  %5 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  tail call void @llvm.assume(i1 %5)
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %index.i = load i64, ptr %6, align 8, !alias.scope !262, !noundef !2
  %7 = ptrtoint ptr %subtracted.i.i to i64
  %_10.i = sub i64 %7, %1
  %8 = add i64 %_10.i, %index.i
  store i64 %8, ptr %6, align 8, !alias.scope !262
  %9 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr null, ptr %9, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 %_0.sroa.4.0.i.ph.i, ptr %_6.sroa.4.0..sroa_idx, align 8
  br label %bb6

bb3:                                              ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %11 = load ptr, ptr %10, align 8, !align !142, !noundef !2
  %.not2 = icmp eq ptr %11, null
  br i1 %.not2, label %bb6, label %bb5

bb6:                                              ; preds = %bb3, %bb5, %bb2
  %.sink = phi i64 [ 1, %bb5 ], [ 1, %bb2 ], [ 0, %bb3 ]
  store i64 %.sink, ptr %_0, align 8
  ret void

bb5:                                              ; preds = %bb3
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %suffix.1 = load i64, ptr %12, align 8, !noundef !2
  store ptr null, ptr %10, align 8
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %11, ptr %13, align 8
  %_9.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %suffix.1, ptr %_9.sroa.4.0..sroa_idx, align 8
  br label %bb6
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1TOCnChXxQ_8clap_lex(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #11 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %_24.i = add i64 %cap, -1
  %or.cond.i = icmp ugt i64 %_24.i, 384307168202282325
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !284

bb11.i:                                           ; preds = %bb6.i
  %_27.0.i = mul nuw nsw i64 %_24.i, 24
  %new_size2.i = add nuw i64 %_27.0.i, 24
  %_40.i = icmp eq i64 %_24.i, 384307168202282325
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
  %alloc_size.i23 = mul nuw i64 %self.0.val, 24
  %cond.i.i = icmp uge i64 %_27.sroa.7.01321, %alloc_size.i23
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) 8, i64 noundef %_27.sroa.7.01321) #27
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27
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

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::split_once
; Function Attrs: uwtable
define void @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt10split_once(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(none) %needle.0, i64 noundef %needle.1) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !285)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !288)
  %_14.i = icmp ult i64 %self.1, %needle.1
  br i1 %_14.i, label %bb4, label %bb4.preheader.i.i

bb4.preheader.i.i:                                ; preds = %start
  %_15.i = sub nuw i64 %self.1, %needle.1
  %_0.i932.i.not.i = icmp eq i64 %_15.i, 0
  br i1 %_0.i932.i.not.i, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb4.preheader.i.i, %bb4.backedge.i.i
  %_3.i73133.i.i = phi i64 [ %_0.i11.i.i, %bb4.backedge.i.i ], [ 0, %bb4.preheader.i.i ]
  %_0.i11.i.i = add nuw i64 %_3.i73133.i.i, 1
  %_8.i.i.i.i = icmp ugt i64 %_3.i73133.i.i, %self.1
  br i1 %_8.i.i.i.i, label %bb2.i.i.i.i, label %bb3.i.i.i.i, !prof !136

bb3.i.i.i.i:                                      ; preds = %bb6.i.i
  %_11.i.i.i.i = sub nuw i64 %self.1, %_3.i73133.i.i
  %_4.not.i.i.i.i.i = icmp samesign ult i64 %_11.i.i.i.i, %needle.1
  br i1 %_4.not.i.i.i.i.i, label %bb4.backedge.i.i, label %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb6.i.i
  %0 = add nuw i64 %self.1, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %0, i64 noundef %self.1, i64 noundef %self.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a01b8cc769b8fb73f4fe5e2b9a72b0d) #28, !noalias !290
  unreachable

_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i: ; preds = %bb3.i.i.i.i
  %_15.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_3.i73133.i.i
  %1 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i.i.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !298, !noalias !305
  %.fr.i.i.i = freeze i32 %1
  %2 = icmp eq i32 %.fr.i.i.i, 0
  br i1 %2, label %bb5, label %bb4.backedge.i.i

bb4.backedge.i.i:                                 ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i, %bb3.i.i.i.i
  %exitcond.not.i.i = icmp eq i64 %_0.i11.i.i, %_15.i
  br i1 %exitcond.not.i.i, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit, label %bb6.i.i

_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit: ; preds = %bb4.backedge.i.i, %bb4.preheader.i.i
  %_15.i.i21.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_15.i
  %3 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i21.i.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !306, !noalias !313
  %.fr.i22.i.i = freeze i32 %3
  %4 = icmp eq i32 %.fr.i22.i.i, 0
  br i1 %4, label %bb5, label %bb4

bb5:                                              ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit
  %_0.sroa.4.1.i5 = phi i64 [ %_15.i, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit ], [ %_3.i73133.i.i, %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i ]
  %end = add i64 %_0.sroa.4.1.i5, %needle.1
  %_17.not = icmp ugt i64 %_0.sroa.4.1.i5, %self.1
  br i1 %_17.not, label %bb7, label %bb6, !prof !318

bb4:                                              ; preds = %start, %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit
  store ptr null, ptr %_0, align 8
  br label %bb3

bb3:                                              ; preds = %bb11, %bb4
  ret void

bb7:                                              ; preds = %bb5
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef %_0.sroa.4.1.i5, i64 noundef %self.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_fc30b8b9e6926e97249f2ed625cdad25) #28
  unreachable

bb6:                                              ; preds = %bb5
  %_25 = icmp ugt i64 %end, %self.1
  br i1 %_25, label %bb10, label %bb11, !prof !136

bb11:                                             ; preds = %bb6
  %_27 = sub nuw i64 %self.1, %end
  %_31 = getelementptr inbounds nuw i8, ptr %self.0, i64 %end
  store ptr %self.0, ptr %_0, align 8
  %_11.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_0.sroa.4.1.i5, ptr %_11.sroa.4.0._0.sroa_idx, align 8
  %_11.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %_31, ptr %_11.sroa.5.0._0.sroa_idx, align 8
  %_11.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %_27, ptr %_11.sroa.6.0._0.sroa_idx, align 8
  br label %bb3

bb10:                                             ; preds = %bb6
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %end, i64 noundef %self.1, i64 noundef %self.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_de83dc5a83c52017857da74c6afa4864) #28
  unreachable
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::starts_with
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(argmem: read) uwtable
define noundef zeroext i1 @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with(ptr noalias noundef nonnull readonly align 1 captures(none) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(none) %prefix.0, i64 noundef %prefix.1) unnamed_addr #12 {
start:
  %_4.not.i = icmp samesign ult i64 %self.1, %prefix.1
  br i1 %_4.not.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit, label %bb4.i

bb4.i:                                            ; preds = %start
  %0 = tail call i32 @memcmp(ptr nonnull readonly align 1 %prefix.0, ptr nonnull readonly align 1 %self.0, i64 range(i64 0, -9223372036854775808) %prefix.1), !alias.scope !319
  %1 = icmp eq i32 %0, 0
  br label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex.exit: ; preds = %start, %bb4.i
  %_0.sroa.0.0.off0.i = phi i1 [ %1, %bb4.i ], [ false, %start ]
  ret i1 %_0.sroa.0.0.off0.i
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::strip_prefix
; Function Attrs: mustprogress nofree norecurse nounwind willreturn memory(argmem: read) uwtable
define { ptr, i64 } @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt12strip_prefix(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(none) %prefix.0, i64 noundef %prefix.1) unnamed_addr #12 {
start:
  %_5.not.i = icmp samesign ugt i64 %prefix.1, %self.1
  br i1 %_5.not.i, label %_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex.exit, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i: ; preds = %start
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %prefix.1
  %len.i.i.i = sub nuw nsw i64 %self.1, %prefix.1
  %0 = tail call i32 @memcmp(ptr nonnull readonly align 1 %self.0, ptr nonnull readonly align 1 %prefix.0, i64 range(i64 0, -9223372036854775808) %prefix.1), !alias.scope !326
  %1 = icmp eq i32 %0, 0
  %spec.select.i = select i1 %1, i64 %len.i.i.i, i64 undef
  %spec.select3.i = select i1 %1, ptr %data.i.i.i, ptr null
  br label %_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex.exit

_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex.exit: ; preds = %start, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i
  %_0.sroa.3.0.i = phi i64 [ undef, %start ], [ %spec.select.i, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i ]
  %_0.sroa.0.0.i = phi ptr [ null, %start ], [ %spec.select3.i, %_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex.exit.i ]
  %2 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0.i, 0
  %.not = icmp eq ptr %_0.sroa.0.0.i, null
  %_0.sroa.3.0 = select i1 %.not, i64 undef, i64 %_0.sroa.3.0.i
  %3 = insertvalue { ptr, i64 } %2, i64 %_0.sroa.3.0, 1
  ret { ptr, i64 } %3
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::find
; Function Attrs: uwtable
define { i64, i64 } @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find(ptr noalias noundef nonnull readonly align 1 captures(none) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(none) %needle.0, i64 noundef %needle.1) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  %_14 = icmp ult i64 %self.1, %needle.1
  br i1 %_14, label %bb2, label %bb4.preheader.i

bb4.preheader.i:                                  ; preds = %start
  %_15 = sub nuw i64 %self.1, %needle.1
  %_0.i932.i.not = icmp eq i64 %_15, 0
  br i1 %_0.i932.i.not, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i, label %bb6.i

bb6.i:                                            ; preds = %bb4.preheader.i, %bb4.backedge.i
  %_3.i73133.i = phi i64 [ %_0.i11.i, %bb4.backedge.i ], [ 0, %bb4.preheader.i ]
  %_0.i11.i = add nuw i64 %_3.i73133.i, 1
  %_8.i.i.i = icmp ugt i64 %_3.i73133.i, %self.1
  br i1 %_8.i.i.i, label %bb2.i.i.i, label %bb3.i.i.i, !prof !136

bb3.i.i.i:                                        ; preds = %bb6.i
  %_11.i.i.i = sub nuw i64 %self.1, %_3.i73133.i
  %_4.not.i.i.i.i = icmp samesign ult i64 %_11.i.i.i, %needle.1
  br i1 %_4.not.i.i.i.i, label %bb4.backedge.i, label %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i

bb2.i.i.i:                                        ; preds = %bb6.i
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %_3.i73133.i, i64 noundef %self.1, i64 noundef %self.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a01b8cc769b8fb73f4fe5e2b9a72b0d) #28, !noalias !333
  unreachable

_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i: ; preds = %bb3.i.i.i
  %_15.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_3.i73133.i
  %0 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !341, !noalias !333
  %.fr.i.i = freeze i32 %0
  %1 = icmp eq i32 %.fr.i.i, 0
  br i1 %1, label %bb2, label %bb4.backedge.i

bb4.backedge.i:                                   ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i, %bb3.i.i.i
  %exitcond.not.i = icmp eq i64 %_0.i11.i, %_15
  br i1 %exitcond.not.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i, label %bb6.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i: ; preds = %bb4.backedge.i, %bb4.preheader.i
  %_15.i.i21.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_15
  %2 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i21.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !348, !noalias !355
  %.fr.i22.i = freeze i32 %2
  %3 = icmp eq i32 %.fr.i22.i, 0
  %4 = zext i1 %3 to i64
  br label %bb2

bb2:                                              ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i, %start
  %_0.sroa.4.1 = phi i64 [ undef, %start ], [ %_15, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i ], [ %_3.i73133.i, %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i ]
  %_0.sroa.0.1 = phi i64 [ 0, %start ], [ %4, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i ], [ 1, %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i ]
  %5 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1, 0
  %6 = insertvalue { i64, i64 } %5, i64 %_0.sroa.4.1, 1
  ret { i64, i64 } %6
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::split
; Function Attrs: uwtable
define void @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt5split(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %0, i64 noundef %1) unnamed_addr #6 {
start:
  %needle = alloca [16 x i8], align 8
  store ptr %0, ptr %needle, align 8
  %2 = getelementptr inbounds nuw i8, ptr %needle, i64 8
  store i64 %1, ptr %2, align 8
  %_3.not.i = icmp eq i64 %1, 0
  br i1 %_3.not.i, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %self.0, ptr %3, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %self.1, ptr %4, align 8
  store ptr %0, ptr %_0, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %1, ptr %5, align 8
  ret void

bb1:                                              ; preds = %start
; call core::panicking::assert_failed::<&str, &str>
  call void @_RINvNtCsjMrxcFdYDNN_4core9panicking13assert_failedReBM_ECs1TOCnChXxQ_8clap_lex(i8 noundef 1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %needle, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) @alloc_eab5d04767146d7d9b93b60d28ef530a, ptr noundef null, ptr undef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1da1cbfd3a38ab605c33306f33c36a20) #28
  unreachable
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::try_str
; Function Attrs: uwtable
define void @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt7try_str(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #6 {
start:
; call core::str::converts::from_utf8
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1)
  ret void
}

; <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::contains
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt8contains(ptr noalias noundef nonnull readonly align 1 captures(none) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull readonly align 1 captures(none) %needle.0, i64 noundef %needle.1) unnamed_addr #6 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !360)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !363)
  %_14.i = icmp ult i64 %self.1, %needle.1
  br i1 %_14.i, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit, label %bb4.preheader.i.i

bb4.preheader.i.i:                                ; preds = %start
  %_15.i = sub nuw i64 %self.1, %needle.1
  %_0.i932.i.not.i = icmp eq i64 %self.1, %needle.1
  br i1 %_0.i932.i.not.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i.i, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb4.preheader.i.i, %bb4.backedge.i.i
  %_3.i73133.i.i = phi i64 [ %_0.i11.i.i, %bb4.backedge.i.i ], [ 0, %bb4.preheader.i.i ]
  %_0.i11.i.i = add nuw i64 %_3.i73133.i.i, 1
  %_8.i.i.i.i = icmp ugt i64 %_3.i73133.i.i, %self.1
  br i1 %_8.i.i.i.i, label %bb2.i.i.i.i, label %bb3.i.i.i.i, !prof !136

bb3.i.i.i.i:                                      ; preds = %bb6.i.i
  %_11.i.i.i.i = sub nuw i64 %self.1, %_3.i73133.i.i
  %_4.not.i.i.i.i.i = icmp samesign ult i64 %_11.i.i.i.i, %needle.1
  br i1 %_4.not.i.i.i.i.i, label %bb4.backedge.i.i, label %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb6.i.i
  %0 = add nuw i64 %self.1, 1
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %0, i64 noundef %self.1, i64 noundef %self.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8a01b8cc769b8fb73f4fe5e2b9a72b0d) #28, !noalias !365
  unreachable

_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i: ; preds = %bb3.i.i.i.i
  %_15.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_3.i73133.i.i
  %1 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i.i.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !373, !noalias !380
  %.fr.i.i.i = freeze i32 %1
  %2 = icmp eq i32 %.fr.i.i.i, 0
  br i1 %2, label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit, label %bb4.backedge.i.i

bb4.backedge.i.i:                                 ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i, %bb3.i.i.i.i
  %exitcond.not.i.i = icmp eq i64 %_0.i11.i.i, %_15.i
  br i1 %exitcond.not.i.i, label %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i.i, label %bb6.i.i

_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i.i: ; preds = %bb4.backedge.i.i, %bb4.preheader.i.i
  %_15.i.i21.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %_15.i
  %3 = tail call i32 @memcmp(ptr nonnull readonly align 1 %needle.0, ptr nonnull readonly align 1 %_15.i.i21.i.i, i64 range(i64 0, -9223372036854775808) %needle.1), !alias.scope !381, !noalias !388
  %.fr.i22.i.i = freeze i32 %3
  %4 = icmp eq i32 %.fr.i22.i.i, 0
  br label %_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit

_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find.exit: ; preds = %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i, %start, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i.i
  %_0.sroa.0.1.i = phi i1 [ false, %start ], [ %4, %_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_.exit25.i.i ], [ true, %_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_.exit.i.i.i ]
  ret i1 %_0.sroa.0.1.i
}

; <&core::option::Option<u8> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRINtNtB8_6option6OptionhENtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #6 {
start:
  %__self_0.i = alloca [8 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !393)
  %0 = load i8, ptr %_3, align 1, !range !396, !alias.scope !393, !noalias !397, !noundef !2
  %1 = trunc nuw i8 %0 to i1
  br i1 %1, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_3, i64 1
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %__self_0.i), !noalias !399
  store ptr %2, ptr %__self_0.i, align 8, !noalias !399
; call <core::fmt::Formatter>::debug_tuple_field1_finish
  %3 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9535bf4c204f3eb9b19ec2c83e446e52, i64 noundef 4, ptr noundef nonnull align 1 %__self_0.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.4)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %__self_0.i), !noalias !399
  br label %_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex.exit

bb3.i:                                            ; preds = %start
; call <core::fmt::Formatter>::write_str
  %4 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_37d2e53432a03a1f90b3e7253015eaf9, i64 noundef 4), !noalias !393
  br label %_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex.exit

_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex.exit: ; preds = %bb2.i, %bb3.i
  %_0.sroa.0.0.in.i = phi i1 [ %3, %bb2.i ], [ %4, %bb3.i ]
  ret i1 %_0.sroa.0.0.in.i
}

; <&&str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRReNtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #6 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !2, !align !400, !noundef !2
  %_3.val = load ptr, ptr %_3, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr i8, ptr %_3, i64 8
  %_3.val1 = load i64, ptr %0, align 8, !noundef !2
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.val, i64 noundef %_3.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <&u8 as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRhNtB6_5Debug3fmtCs1TOCnChXxQ_8clap_lex(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #6 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4.i = load i32, ptr %0, align 8, !alias.scope !401, !noalias !404, !noundef !2
  %_3.i = and i32 %_4.i, 33554432
  %1 = icmp eq i32 %_3.i, 0
  br i1 %1, label %bb2.i, label %bb1.i

bb2.i:                                            ; preds = %start
  %_5.i = and i32 %_4.i, 67108864
  %2 = icmp eq i32 %_5.i, 0
  br i1 %2, label %bb4.i, label %bb3.i

bb1.i:                                            ; preds = %start
; call <u8 as core::fmt::LowerHex>::fmt
  %3 = tail call noundef zeroext i1 @_RNvXse_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8LowerHex3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt.exit

bb4.i:                                            ; preds = %bb2.i
; call <u8 as core::fmt::Display>::fmt
  %4 = tail call noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt.exit

bb3.i:                                            ; preds = %bb2.i
; call <u8 as core::fmt::UpperHex>::fmt
  %5 = tail call noundef zeroext i1 @_RNvXsg_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8UpperHex3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %_3, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br label %_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt.exit

_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt.exit: ; preds = %bb1.i, %bb4.i, %bb3.i
  %_0.sroa.0.0.in.i = phi i1 [ %4, %bb4.i ], [ %5, %bb3.i ], [ %3, %bb1.i ]
  ret i1 %_0.sroa.0.0.in.i
}

; <clap_lex::ShortFlags as core::iter::traits::iterator::Iterator>::next
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable
define void @_RNvXs3_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlagsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, ptr noalias noundef align 8 captures(none) dereferenceable(56) %self) unnamed_addr #10 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !406)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !409)
  %_3.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  tail call void @llvm.experimental.noalias.scope.decl(metadata !411)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_4.i3.i.i = load ptr, ptr %0, align 8, !alias.scope !414, !noalias !406, !nonnull !2, !noundef !2
  %subtracted.i4.i.i = load ptr, ptr %_3.i, align 8, !alias.scope !414, !noalias !406, !nonnull !2, !noundef !2
  %1 = ptrtoint ptr %subtracted.i4.i.i to i64
  tail call void @llvm.experimental.noalias.scope.decl(metadata !417)
  %_6.i.i.i.i = icmp eq ptr %subtracted.i4.i.i, %_4.i3.i.i
  br i1 %_6.i.i.i.i, label %bb3.i, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %start
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i.i, i64 1
  store ptr %_16.i.i.i.i, ptr %_3.i, align 8, !alias.scope !420, !noalias !406
  %x.i.i.i = load i8, ptr %subtracted.i4.i.i, align 1, !noalias !423, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_4.i3.i.i
  tail call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i.i, i64 2
  store ptr %_16.i12.i.i.i, ptr %_3.i, align 8, !alias.scope !424, !noalias !406
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !noalias !423, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %2 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb2.i

bb3.i.i.i:                                        ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb2.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_4.i3.i.i
  tail call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i.i, i64 3
  store ptr %_16.i19.i.i.i, ptr %_3.i, align 8, !alias.scope !427, !noalias !406
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !noalias !423, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %3 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i, label %bb2.i

bb8.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_4.i3.i.i
  tail call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %subtracted.i4.i.i, i64 4
  store ptr %_16.i26.i.i.i, ptr %_3.i, align 8, !alias.scope !430, !noalias !406
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !noalias !423, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %4 = or disjoint i32 %_27.i.i.i, %_25.i.i.i
  br label %bb2.i

bb2.i:                                            ; preds = %bb8.i.i.i, %bb6.i.i.i, %bb3.i.i.i, %bb4.i.i.i
  %subtracted.i.i.i = phi ptr [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i.i ], [ %_16.i.i.i.i, %bb3.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %2, %bb4.i.i.i ], [ %3, %bb6.i.i.i ], [ %4, %bb8.i.i.i ], [ %_7.i.i.i, %bb3.i.i.i ]
  %5 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  tail call void @llvm.assume(i1 %5)
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 32
  %index.i.i = load i64, ptr %6, align 8, !alias.scope !433, !noalias !406, !noundef !2
  %7 = ptrtoint ptr %subtracted.i.i.i to i64
  %_10.i.i = sub i64 %7, %1
  %8 = add i64 %_10.i.i, %index.i.i
  store i64 %8, ptr %6, align 8, !alias.scope !433, !noalias !406
  %9 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr null, ptr %9, align 8, !alias.scope !406, !noalias !409
  %_6.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i32 %_0.sroa.4.0.i.ph.i.i, ptr %_6.sroa.4.0..sroa_idx.i, align 8, !alias.scope !406, !noalias !409
  br label %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag.exit

bb3.i:                                            ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %11 = load ptr, ptr %10, align 8, !alias.scope !409, !noalias !406, !align !142, !noundef !2
  %.not2.i = icmp eq ptr %11, null
  br i1 %.not2.i, label %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag.exit, label %bb5.i

bb5.i:                                            ; preds = %bb3.i
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 48
  %suffix.1.i = load i64, ptr %12, align 8, !alias.scope !409, !noalias !406, !noundef !2
  store ptr null, ptr %10, align 8, !alias.scope !409, !noalias !406
  %13 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %11, ptr %13, align 8, !alias.scope !406, !noalias !409
  %_9.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %suffix.1.i, ptr %_9.sroa.4.0..sroa_idx.i, align 8, !alias.scope !406, !noalias !409
  br label %_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag.exit

_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag.exit: ; preds = %bb2.i, %bb3.i, %bb5.i
  %.sink.i = phi i64 [ 1, %bb5.i ], [ 1, %bb2.i ], [ 0, %bb3.i ]
  store i64 %.sink.i, ptr %_0, align 8, !alias.scope !406, !noalias !409
  ret void
}

; <core::str::error::Utf8Error as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str5errorNtB5_9Utf8ErrorNtNtB9_3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #13 {
start:
  %_7 = alloca [8 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_7)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  store ptr %0, ptr %_7, align 8
; call <core::fmt::Formatter>::debug_struct_field2_finish
  %_0 = call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_8e685ef482aec04a2d7a8ed5ef1228a3, i64 noundef 9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_f34017a1538f19bf68b6d6294eec0bb3, i64 noundef 11, ptr noundef nonnull align 1 %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.2, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_91eca80c47235190e5fbed3d6d8be36c, i64 noundef 9, ptr noundef nonnull align 1 %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.3)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_7)
  ret i1 %_0
}

; <usize as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_5Debug3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #13 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_4 = load i32, ptr %0, align 8, !noundef !2
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

; <clap_lex::ext::Split as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
define { ptr, i64 } @_RNvXs_NtCs1TOCnChXxQ_8clap_lex3extNtB4_5SplitNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self) unnamed_addr #6 {
start:
  %_5 = alloca [32 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load ptr, ptr %0, align 8, !align !142, !noundef !2
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %.not = icmp eq ptr %1, null
  br i1 %.not, label %bb9, label %bb11

bb11:                                             ; preds = %start
  %3 = load i64, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5)
  %_13.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !142, !noundef !2
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_13.1 = load i64, ptr %4, align 8, !noundef !2
; call <std::ffi::os_str::OsStr as clap_lex::ext::OsStrExt>::split_once
  call void @_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt10split_once(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %1, i64 noundef %3, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_13.0, i64 noundef %_13.1)
  %5 = load ptr, ptr %_5, align 8, !noundef !2
  %.not2 = icmp eq ptr %5, null
  br i1 %.not2, label %bb8, label %bb3

bb9:                                              ; preds = %start, %bb8
  %_0.sroa.4.0 = phi i64 [ %_0.sroa.4.1, %bb8 ], [ undef, %start ]
  %_0.sroa.0.0 = phi ptr [ %_0.sroa.0.1, %bb8 ], [ null, %start ]
  %6 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0, 0
  %7 = insertvalue { ptr, i64 } %6, i64 %_0.sroa.4.0, 1
  ret { ptr, i64 } %7

bb3:                                              ; preds = %bb11
  %8 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %first.1 = load i64, ptr %8, align 8, !noundef !2
  %9 = getelementptr inbounds nuw i8, ptr %_5, i64 16
  %second.0 = load ptr, ptr %9, align 8, !nonnull !2, !align !142, !noundef !2
  %10 = getelementptr inbounds nuw i8, ptr %_5, i64 24
  %second.1 = load i64, ptr %10, align 8, !noundef !2
  store i64 %second.1, ptr %2, align 8
  br label %bb8

bb8:                                              ; preds = %bb11, %bb3
  %second.0.sink = phi ptr [ %second.0, %bb3 ], [ null, %bb11 ]
  %_0.sroa.4.1 = phi i64 [ %first.1, %bb3 ], [ %3, %bb11 ]
  %_0.sroa.0.1 = phi ptr [ %5, %bb3 ], [ %1, %bb11 ]
  store ptr %second.0.sink, ptr %0, align 8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  br label %bb9
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #14

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #15

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #14

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #16

; core::panicking::assert_failed_inner
; Function Attrs: cold minsize noinline noreturn optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking19assert_failed_inner(i8 noundef range(i8 0, 3), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noundef, ptr, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #1

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #17

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #18

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #19

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.sadd.sat.i64(i64, i64) #20

; std::env::args_os
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env7args_os(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32)) unnamed_addr #6

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #21

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #22

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #0

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #23

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #19

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #19

; <alloc::string::String>::from_utf8_lossy
; Function Attrs: uwtable
declare void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String15from_utf8_lossy(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #6

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #6

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #24

; <core::fmt::Formatter>::debug_struct_field2_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter26debug_struct_field2_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #6

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #6

; <core::fmt::Formatter>::debug_tuple_field1_finish
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter25debug_tuple_field1_finish(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32)) unnamed_addr #6

; <u8 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; <u8 as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsg_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; <u8 as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXse_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; <usize as core::fmt::UpperHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8UpperHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; <usize as core::fmt::LowerHex>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs6_NtNtCsjMrxcFdYDNN_4core3fmt3numjNtB7_8LowerHex3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #6

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #25

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.smax.i64(i64, i64) #26

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #26

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #26

attributes #0 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { cold minsize noinline noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { mustprogress nofree norecurse nosync nounwind willreturn memory(read, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nofree norecurse nounwind willreturn memory(read, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { nofree norecurse nosync nounwind memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { nofree norecurse nosync nounwind memory(read, inaccessiblemem: none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nofree norecurse nosync nounwind willreturn memory(read, argmem: readwrite, inaccessiblemem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { mustprogress nofree norecurse nounwind willreturn memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #15 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #16 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #17 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #19 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #21 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #22 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #23 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #24 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #25 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #26 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #27 = { nounwind }
attributes #28 = { noinline noreturn }
attributes #29 = { noreturn }
attributes #30 = { cold }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{}
!3 = !{!4}
!4 = distinct !{!4, !5, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex: %_1.0"}
!5 = distinct !{!5, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex"}
!6 = !{i64 0, i64 -9223372036854775808}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex: %_1"}
!9 = distinct !{!9, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex"}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex: %_1"}
!12 = distinct !{!12, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex"}
!13 = !{!14}
!14 = distinct !{!14, !15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex: %_1"}
!15 = distinct !{!15, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex"}
!16 = !{!17}
!17 = distinct !{!17, !18, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex: %self"}
!18 = distinct !{!18, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex"}
!19 = !{!17, !14, !11, !8}
!20 = !{!21}
!21 = distinct !{!21, !22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex: %_1.0"}
!22 = distinct !{!22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex"}
!23 = !{!21, !17, !14, !11, !8}
!24 = !{!25}
!25 = distinct !{!25, !26, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1TOCnChXxQ_8clap_lex: %self"}
!26 = distinct !{!26, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1TOCnChXxQ_8clap_lex"}
!27 = !{i64 0, i64 2}
!28 = !{i64 0, i64 -9223372036854775807}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_: %_0"}
!31 = distinct !{!31, !"_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_"}
!32 = !{!33}
!33 = distinct !{!33, !31, !"_RINvMCs1TOCnChXxQ_8clap_lexNtB3_7RawArgs3newNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringNtNtBM_3env6ArgsOsEB3_: %iter"}
!34 = !{!35, !37, !33}
!35 = distinct !{!35, !36, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collectNtNtCs5sEH5CPMdak_3std3env6ArgsOsNtB2_12IntoIterator9into_iterCs1TOCnChXxQ_8clap_lex: %_0"}
!36 = distinct !{!36, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collectNtNtCs5sEH5CPMdak_3std3env6ArgsOsNtB2_12IntoIterator9into_iterCs1TOCnChXxQ_8clap_lex"}
!37 = distinct !{!37, !36, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collectNtNtCs5sEH5CPMdak_3std3env6ArgsOsNtB2_12IntoIterator9into_iterCs1TOCnChXxQ_8clap_lex: %self"}
!38 = !{!39, !41, !42, !44, !30, !33}
!39 = distinct !{!39, !40, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_18SpecFromIterNestedB11_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB17_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3s_7RawArgsINtNtB2n_7convert4FromB32_E4from0EE9from_iterB3s_: %_0"}
!40 = distinct !{!40, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_18SpecFromIterNestedB11_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB17_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3s_7RawArgsINtNtB2n_7convert4FromB32_E4from0EE9from_iterB3s_"}
!41 = distinct !{!41, !40, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_18SpecFromIterNestedB11_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB17_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3s_7RawArgsINtNtB2n_7convert4FromB32_E4from0EE9from_iterB3s_: %iterator"}
!42 = distinct !{!42, !43, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_12SpecFromIterBU_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB10_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3e_7RawArgsINtNtB29_7convert4FromB2O_E4from0EE9from_iterB3e_: %_0"}
!43 = distinct !{!43, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_12SpecFromIterBU_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB10_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3e_7RawArgsINtNtB29_7convert4FromB2O_E4from0EE9from_iterB3e_"}
!44 = distinct !{!44, !43, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec14spec_from_iterINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_12SpecFromIterBU_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtB10_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB3e_7RawArgsINtNtB29_7convert4FromB2O_E4from0EE9from_iterB3e_: %iterator"}
!45 = !{!46, !48, !50, !52, !39, !41, !42, !44, !30, !33}
!46 = distinct !{!46, !47, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!47 = distinct !{!47, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!48 = distinct !{!48, !49, !"_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!49 = distinct !{!49, !"_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!50 = distinct !{!50, !51, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_: %_0"}
!51 = distinct !{!51, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_"}
!52 = distinct !{!52, !51, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_: %self"}
!53 = !{!54}
!54 = distinct !{!54, !55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex: %_1.0"}
!55 = distinct !{!55, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex"}
!56 = !{!57, !59, !61, !63, !65, !39, !41, !42, !44, !30, !33}
!57 = distinct !{!57, !58, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex: %self"}
!58 = distinct !{!58, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex"}
!59 = distinct !{!59, !60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex: %_1"}
!60 = distinct !{!60, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex"}
!61 = distinct !{!61, !62, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex: %_1"}
!62 = distinct !{!62, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex"}
!63 = distinct !{!63, !64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex: %_1"}
!64 = distinct !{!64, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex"}
!65 = distinct !{!65, !66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_: %_1"}
!66 = distinct !{!66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_"}
!67 = !{!54, !57, !59, !61, !63, !65, !39, !41, !42, !44, !30, !33}
!68 = !{!52, !39, !41, !42, !44, !30, !33}
!69 = !{!"branch_weights", i32 6004, i32 2000}
!70 = !{!71, !39, !41, !42, !44, !30, !33}
!71 = distinct !{!71, !72, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1TOCnChXxQ_8clap_lex: %_0"}
!72 = distinct !{!72, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1TOCnChXxQ_8clap_lex"}
!73 = !{!39, !42, !30, !33}
!74 = !{!75}
!75 = distinct !{!75, !76, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBX_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB38_7RawArgsINtNtB24_7convert4FromB2J_E4from0EE11spec_extendB38_: %self"}
!76 = distinct !{!76, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBX_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB38_7RawArgsINtNtB24_7convert4FromB2J_E4from0EE11spec_extendB38_"}
!77 = !{!78}
!78 = distinct !{!78, !76, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBX_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB38_7RawArgsINtNtB24_7convert4FromB2J_E4from0EE11spec_extendB38_: %iter"}
!79 = !{!80}
!80 = distinct !{!80, !81, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBM_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB2U_7RawArgsINtNtB1Q_7convert4FromB2v_E4from0EEB2U_: %self"}
!81 = distinct !{!81, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBM_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB2U_7RawArgsINtNtB1Q_7convert4FromB2v_E4from0EEB2U_"}
!82 = !{!83}
!83 = distinct !{!83, !81, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapNtNtBM_3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB2U_7RawArgsINtNtB1Q_7convert4FromB2v_E4from0EEB2U_: %iterator"}
!84 = !{!85}
!85 = distinct !{!85, !86, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_: %self"}
!86 = distinct !{!86, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_"}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!89 = distinct !{!89, !"_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!92 = distinct !{!92, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!93 = !{!91, !88, !94, !85, !80, !83, !75, !78, !39, !41, !42, !44, !30, !33}
!94 = distinct !{!94, !86, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1A_7RawArgsINtNtBb_7convert4FromBW_E4from0ENtNtNtB9_6traits8iterator8Iterator4nextB1A_: %_0"}
!95 = !{!80, !75, !39, !41, !42, !44, !30, !33}
!96 = !{!85, !80, !83, !75, !78, !39, !41, !42, !44, !30, !33}
!97 = !{!80, !75}
!98 = !{!83, !78, !39, !41, !42, !44, !30, !33}
!99 = !{!100}
!100 = distinct !{!100, !101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex: %_1.0"}
!101 = distinct !{!101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex"}
!102 = !{!103, !105, !107, !109, !111, !80, !83, !75, !78, !39, !41, !42, !44, !30, !33}
!103 = distinct !{!103, !104, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex: %self"}
!104 = distinct !{!104, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex"}
!105 = distinct !{!105, !106, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex: %_1"}
!106 = distinct !{!106, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex"}
!107 = distinct !{!107, !108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex: %_1"}
!108 = distinct !{!108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex"}
!109 = distinct !{!109, !110, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex: %_1"}
!110 = distinct !{!110, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex"}
!111 = distinct !{!111, !112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_: %_1"}
!112 = distinct !{!112, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_"}
!113 = !{!100, !103, !105, !107, !109, !111, !80, !83, !75, !78, !39, !41, !42, !44, !30, !33}
!114 = !{!80, !83, !75, !78, !39, !41, !42, !44, !30, !33}
!115 = !{!91, !88, !85, !83, !78}
!116 = !{!117, !118, !94, !80, !75, !39, !41, !42, !44, !30, !33}
!117 = distinct !{!117, !92, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %_0"}
!118 = distinct !{!118, !89, !"_RNvXsg_NtCs5sEH5CPMdak_3std3envNtB5_6ArgsOsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!119 = !{!41, !44, !30, !33}
!120 = !{!121}
!121 = distinct !{!121, !122, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex: %_1.0"}
!122 = distinct !{!122, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringECs1TOCnChXxQ_8clap_lex"}
!123 = !{!39}
!124 = !{!125, !127, !129, !131, !133, !39, !42, !44, !30, !33}
!125 = distinct !{!125, !126, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex: %self"}
!126 = distinct !{!126, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1TOCnChXxQ_8clap_lex"}
!127 = distinct !{!127, !128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex: %_1"}
!128 = distinct !{!128, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECs1TOCnChXxQ_8clap_lex"}
!129 = distinct !{!129, !130, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex: %_1"}
!130 = distinct !{!130, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtNtCs5sEH5CPMdak_3std3sys4args6common4ArgsECs1TOCnChXxQ_8clap_lex"}
!131 = distinct !{!131, !132, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex: %_1"}
!132 = distinct !{!132, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std3env6ArgsOsECs1TOCnChXxQ_8clap_lex"}
!133 = distinct !{!133, !134, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_: %_1"}
!134 = distinct !{!134, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapNtNtCs5sEH5CPMdak_3std3env6ArgsOsNCNvXs_Cs1TOCnChXxQ_8clap_lexNtB1S_7RawArgsINtNtB4_7convert4FromB1e_E4from0EEB1S_"}
!135 = !{!121, !125, !127, !129, !131, !133, !39, !42, !44, !30, !33}
!136 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!137 = !{!138}
!138 = distinct !{!138, !139, !"_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8to_value: %self"}
!139 = distinct !{!139, !"_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8to_value"}
!140 = !{!141}
!141 = distinct !{!141, !139, !"_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg8to_value: %_0"}
!142 = !{i64 1}
!143 = !{!141, !138}
!144 = !{!145}
!145 = distinct !{!145, !146, !"_RNCNvMs1_Cs1TOCnChXxQ_8clap_lexNtB7_9ParsedArg18is_negative_number0B7_: %s.0"}
!146 = distinct !{!146, !"_RNCNvMs1_Cs1TOCnChXxQ_8clap_lexNtB7_9ParsedArg18is_negative_number0B7_"}
!147 = !{!148, !145}
!148 = distinct !{!148, !149, !"_RNvCs1TOCnChXxQ_8clap_lex9is_number: %arg.0"}
!149 = distinct !{!149, !"_RNvCs1TOCnChXxQ_8clap_lex9is_number"}
!150 = distinct !{!150, !151}
!151 = !{!"llvm.loop.peeled.count", i32 1}
!152 = !{!153, !155, !156, !158, !159, !161}
!153 = distinct !{!153, !154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!154 = distinct !{!154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!155 = distinct !{!155, !154, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!156 = distinct !{!156, !157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!157 = distinct !{!157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!158 = distinct !{!158, !157, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!159 = distinct !{!159, !160, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with: %self.0"}
!160 = distinct !{!160, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with"}
!161 = distinct !{!161, !160, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with: %prefix.0"}
!162 = !{!163, !165}
!163 = distinct !{!163, !164, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!164 = distinct !{!164, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!165 = distinct !{!165, !164, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!166 = !{!167}
!167 = distinct !{!167, !168, !"_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg9is_escape: %self"}
!168 = distinct !{!168, !"_RNvMs1_Cs1TOCnChXxQ_8clap_lexNtB5_9ParsedArg9is_escape"}
!169 = !{!170, !172, !173, !175, !176, !178}
!170 = distinct !{!170, !171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!171 = distinct !{!171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!172 = distinct !{!172, !171, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!173 = distinct !{!173, !174, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex: %self.0"}
!174 = distinct !{!174, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex"}
!175 = distinct !{!175, !174, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex: %prefix.0"}
!176 = distinct !{!176, !177, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt12strip_prefix: %self.0"}
!177 = distinct !{!177, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt12strip_prefix"}
!178 = distinct !{!178, !177, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt12strip_prefix: %prefix.0"}
!179 = !{!180, !182, !183, !185, !186, !188}
!180 = distinct !{!180, !181, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!181 = distinct !{!181, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!182 = distinct !{!182, !181, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!183 = distinct !{!183, !184, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!184 = distinct !{!184, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!185 = distinct !{!185, !184, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!186 = distinct !{!186, !187, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with: %self.0"}
!187 = distinct !{!187, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with"}
!188 = distinct !{!188, !187, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt11starts_with: %prefix.0"}
!189 = !{!190, !192, !193, !195}
!190 = distinct !{!190, !191, !"_RNvCs1TOCnChXxQ_8clap_lex18split_nonutf8_once: %_0"}
!191 = distinct !{!191, !"_RNvCs1TOCnChXxQ_8clap_lex18split_nonutf8_once"}
!192 = distinct !{!192, !191, !"_RNvCs1TOCnChXxQ_8clap_lex18split_nonutf8_once: %b.0"}
!193 = distinct !{!193, !194, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new: %_0"}
!194 = distinct !{!194, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new"}
!195 = distinct !{!195, !194, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags3new: %inner.0"}
!196 = !{!190, !193}
!197 = !{!198, !200, !201, !203, !190, !193}
!198 = distinct !{!198, !199, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex: %pair"}
!199 = distinct !{!199, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex"}
!200 = distinct !{!200, !199, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex: %self.0"}
!201 = distinct !{!201, !202, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at: %_0"}
!202 = distinct !{!202, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at"}
!203 = distinct !{!203, !202, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at: %os.0"}
!204 = !{!205}
!205 = distinct !{!205, !206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex: %self"}
!206 = distinct !{!206, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultReNtNtNtB4_3str5error9Utf8ErrorE6unwrapCs1TOCnChXxQ_8clap_lex"}
!207 = !{!205, !190, !192, !193, !195}
!208 = !{!205, !190, !193}
!209 = !{!210, !212}
!210 = distinct !{!210, !211, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!211 = distinct !{!211, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!212 = distinct !{!212, !211, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!213 = !{!214}
!214 = distinct !{!214, !215, !"_RNvXs3_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlagsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!215 = distinct !{!215, !"_RNvXs3_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlagsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!216 = !{!217}
!217 = distinct !{!217, !218, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag: %self"}
!218 = distinct !{!218, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag"}
!219 = !{!220}
!220 = distinct !{!220, !221, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!221 = distinct !{!221, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!222 = !{!223}
!223 = distinct !{!223, !224, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex: %bytes"}
!224 = distinct !{!224, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex"}
!225 = !{!226, !223, !220, !217, !214}
!226 = distinct !{!226, !227, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!227 = distinct !{!227, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!228 = !{!229, !230}
!229 = distinct !{!229, !218, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag: %_0"}
!230 = distinct !{!230, !215, !"_RNvXs3_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlagsNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %_0"}
!231 = !{!223, !220, !229, !217, !230, !214}
!232 = !{!233, !223, !220, !217, !214}
!233 = distinct !{!233, !234, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!234 = distinct !{!234, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!235 = !{!236, !223, !220, !217, !214}
!236 = distinct !{!236, !237, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!237 = distinct !{!237, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!238 = !{!239, !223, !220, !217, !214}
!239 = distinct !{!239, !240, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!240 = distinct !{!240, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!241 = !{!217, !214}
!242 = !{!220, !217, !214}
!243 = !{!244}
!244 = distinct !{!244, !245, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!245 = distinct !{!245, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!246 = !{!247, !244}
!247 = distinct !{!247, !248, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex: %self"}
!248 = distinct !{!248, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex"}
!249 = !{!250, !244}
!250 = distinct !{!250, !251, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex: %bytes"}
!251 = distinct !{!251, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex"}
!252 = !{!253, !255, !256, !258}
!253 = distinct !{!253, !254, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex: %pair"}
!254 = distinct !{!254, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex"}
!255 = distinct !{!255, !254, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCs1TOCnChXxQ_8clap_lex: %self.0"}
!256 = distinct !{!256, !257, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at: %_0"}
!257 = distinct !{!257, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at"}
!258 = distinct !{!258, !257, !"_RNvNtCs1TOCnChXxQ_8clap_lex3ext8split_at: %os.0"}
!259 = !{!260}
!260 = distinct !{!260, !261, !"_RNvCs1TOCnChXxQ_8clap_lex9is_number: %arg.0"}
!261 = distinct !{!261, !"_RNvCs1TOCnChXxQ_8clap_lex9is_number"}
!262 = !{!263}
!263 = distinct !{!263, !264, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!264 = distinct !{!264, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!265 = !{!266, !263}
!266 = distinct !{!266, !267, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex: %self"}
!267 = distinct !{!267, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex"}
!268 = !{!269}
!269 = distinct !{!269, !270, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex: %bytes"}
!270 = distinct !{!270, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex"}
!271 = !{!272, !269, !263}
!272 = distinct !{!272, !273, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!273 = distinct !{!273, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!274 = !{!269, !263}
!275 = !{!276, !269, !263}
!276 = distinct !{!276, !277, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!277 = distinct !{!277, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!278 = !{!279, !269, !263}
!279 = distinct !{!279, !280, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!280 = distinct !{!280, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!281 = !{!282, !269, !263}
!282 = distinct !{!282, !283, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!283 = distinct !{!283, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!284 = !{!"branch_weights", i32 2002, i32 2000}
!285 = !{!286}
!286 = distinct !{!286, !287, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find: %self.0"}
!287 = distinct !{!287, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find"}
!288 = !{!289}
!289 = distinct !{!289, !287, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find: %needle.0"}
!290 = !{!291, !293, !295, !297, !286, !289}
!291 = distinct !{!291, !292, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!292 = distinct !{!292, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!293 = distinct !{!293, !294, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!294 = distinct !{!294, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!295 = distinct !{!295, !296, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %self"}
!296 = distinct !{!296, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_"}
!297 = distinct !{!297, !296, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %f"}
!298 = !{!299, !301, !302, !304, !286, !289}
!299 = distinct !{!299, !300, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!300 = distinct !{!300, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!301 = distinct !{!301, !300, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!302 = distinct !{!302, !303, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!303 = distinct !{!303, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!304 = distinct !{!304, !303, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!305 = !{!291, !293, !295, !297}
!306 = !{!307, !309, !310, !312, !286, !289}
!307 = distinct !{!307, !308, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!308 = distinct !{!308, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!309 = distinct !{!309, !308, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!310 = distinct !{!310, !311, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!311 = distinct !{!311, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!312 = distinct !{!312, !311, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!313 = !{!314, !316, !295, !297}
!314 = distinct !{!314, !315, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!315 = distinct !{!315, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!316 = distinct !{!316, !317, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!317 = distinct !{!317, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!318 = !{!"branch_weights", i32 4001, i32 4000000}
!319 = !{!320, !322, !323, !325}
!320 = distinct !{!320, !321, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!321 = distinct !{!321, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!322 = distinct !{!322, !321, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!323 = distinct !{!323, !324, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!324 = distinct !{!324, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!325 = distinct !{!325, !324, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!326 = !{!327, !329, !330, !332}
!327 = distinct !{!327, !328, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!328 = distinct !{!328, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!329 = distinct !{!329, !328, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!330 = distinct !{!330, !331, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex: %self.0"}
!331 = distinct !{!331, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex"}
!332 = distinct !{!332, !331, !"_RINvMNtCsjMrxcFdYDNN_4core5sliceSh12strip_prefixBu_ECs1TOCnChXxQ_8clap_lex: %prefix.0"}
!333 = !{!334, !336, !338, !340}
!334 = distinct !{!334, !335, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!335 = distinct !{!335, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!336 = distinct !{!336, !337, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!337 = distinct !{!337, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!338 = distinct !{!338, !339, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %self"}
!339 = distinct !{!339, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_"}
!340 = distinct !{!340, !339, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %f"}
!341 = !{!342, !344, !345, !347}
!342 = distinct !{!342, !343, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!343 = distinct !{!343, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!344 = distinct !{!344, !343, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!345 = distinct !{!345, !346, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!346 = distinct !{!346, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!347 = distinct !{!347, !346, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!348 = !{!349, !351, !352, !354}
!349 = distinct !{!349, !350, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!350 = distinct !{!350, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!351 = distinct !{!351, !350, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!352 = distinct !{!352, !353, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!353 = distinct !{!353, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!354 = distinct !{!354, !353, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!355 = !{!356, !358, !338, !340}
!356 = distinct !{!356, !357, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!357 = distinct !{!357, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!358 = distinct !{!358, !359, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!359 = distinct !{!359, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!360 = !{!361}
!361 = distinct !{!361, !362, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find: %self.0"}
!362 = distinct !{!362, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find"}
!363 = !{!364}
!364 = distinct !{!364, !362, !"_RNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2_8OsStrExt4find: %needle.0"}
!365 = !{!366, !368, !370, !372, !361, !364}
!366 = distinct !{!366, !367, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!367 = distinct !{!367, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!368 = distinct !{!368, !369, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!369 = distinct !{!369, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!370 = distinct !{!370, !371, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %self"}
!371 = distinct !{!371, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_"}
!372 = distinct !{!372, !371, !"_RINvXsc_NtNtCsjMrxcFdYDNN_4core4iter5rangeINtNtNtBa_3ops5range14RangeInclusivejENtB6_26RangeInclusiveIteratorImpl13spec_try_folduNCINvNvNtNtNtB8_6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB2Z_8OsStrExt4find0E0INtNtBJ_12control_flow11ControlFlowjEEB31_: %f"}
!373 = !{!374, !376, !377, !379, !361, !364}
!374 = distinct !{!374, !375, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!375 = distinct !{!375, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!376 = distinct !{!376, !375, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!377 = distinct !{!377, !378, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!378 = distinct !{!378, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!379 = distinct !{!379, !378, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!380 = !{!366, !368, !370, !372}
!381 = !{!382, !384, !385, !387, !361, !364}
!382 = distinct !{!382, !383, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %self.0"}
!383 = distinct !{!383, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex"}
!384 = distinct !{!384, !383, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs1TOCnChXxQ_8clap_lex: %other.0"}
!385 = distinct !{!385, !386, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %self.0"}
!386 = distinct !{!386, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex"}
!387 = distinct !{!387, !386, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1TOCnChXxQ_8clap_lex: %needle.0"}
!388 = !{!389, !391, !370, !372}
!389 = distinct !{!389, !390, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_: %_1"}
!390 = distinct !{!390, !"_RNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB4_8OsStrExt4find0B6_"}
!391 = distinct !{!391, !392, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_: %_1"}
!392 = distinct !{!392, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4find5checkjNCNvXNtCs1TOCnChXxQ_8clap_lex3extNtNtNtCs5sEH5CPMdak_3std3ffi6os_str5OsStrNtB1i_8OsStrExt4find0E0B1k_"}
!393 = !{!394}
!394 = distinct !{!394, !395, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex: %self"}
!395 = distinct !{!395, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex"}
!396 = !{i8 0, i8 2}
!397 = !{!398}
!398 = distinct !{!398, !395, !"_RNvXsR_NtCsjMrxcFdYDNN_4core6optionINtB5_6OptionhENtNtB7_3fmt5Debug3fmtCs1TOCnChXxQ_8clap_lex: %f"}
!399 = !{!394, !398}
!400 = !{i64 8}
!401 = !{!402}
!402 = distinct !{!402, !403, !"_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt: %f"}
!403 = distinct !{!403, !"_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt"}
!404 = !{!405}
!405 = distinct !{!405, !403, !"_RNvXsU_NtNtCsjMrxcFdYDNN_4core3fmt3numhNtB7_5Debug3fmt: %self"}
!406 = !{!407}
!407 = distinct !{!407, !408, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag: %_0"}
!408 = distinct !{!408, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag"}
!409 = !{!410}
!410 = distinct !{!410, !408, !"_RNvMs2_Cs1TOCnChXxQ_8clap_lexNtB5_10ShortFlags9next_flag: %self"}
!411 = !{!412}
!412 = distinct !{!412, !413, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!413 = distinct !{!413, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!414 = !{!415, !412, !410}
!415 = distinct !{!415, !416, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex: %self"}
!416 = distinct !{!416, !"_RNvXs2D_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits10exact_size17ExactSizeIterator3lenCs1TOCnChXxQ_8clap_lex"}
!417 = !{!418}
!418 = distinct !{!418, !419, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex: %bytes"}
!419 = distinct !{!419, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs1TOCnChXxQ_8clap_lex"}
!420 = !{!421, !418, !412, !410}
!421 = distinct !{!421, !422, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!422 = distinct !{!422, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!423 = !{!418, !412, !407, !410}
!424 = !{!425, !418, !412, !410}
!425 = distinct !{!425, !426, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!426 = distinct !{!426, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!427 = !{!428, !418, !412, !410}
!428 = distinct !{!428, !429, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!429 = distinct !{!429, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!430 = !{!431, !418, !412, !410}
!431 = distinct !{!431, !432, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex: %self"}
!432 = distinct !{!432, !"_RNvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB6_4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator4nextCs1TOCnChXxQ_8clap_lex"}
!433 = !{!412, !410}
