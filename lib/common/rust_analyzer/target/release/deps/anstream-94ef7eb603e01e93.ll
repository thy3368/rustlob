; ModuleID = 'anstream.a769963086c8ce7a-cgu.0'
source_filename = "anstream.a769963086c8ce7a-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>" = type { %"std::sys::sync::mutex::pthread::Mutex", %"std::sync::reentrant_lock::Tid", i32, [1 x i32], i64 }
%"std::sys::sync::mutex::pthread::Mutex" = type { %"std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>" }
%"std::sys::sync::once_box::OnceBox<std::sys::pal::unix::sync::mutex::Mutex>" = type { %"core::sync::atomic::AtomicPtr<std::sys::pal::unix::sync::mutex::Mutex>" }
%"core::sync::atomic::AtomicPtr<std::sys::pal::unix::sync::mutex::Mutex>" = type { ptr }
%"std::sync::reentrant_lock::Tid" = type { %"core::sync::atomic::AtomicU64" }
%"core::sync::atomic::AtomicU64" = type { i64 }

@_RNvNvNtNtCs5sEH5CPMdak_3std2io5stdio6stderr8INSTANCE = external global %"std::sync::reentrant_lock::ReentrantLock<core::cell::RefCell<std::io::stdio::StderrRaw>>"
@alloc_787344c135fb1ca228327e24d16f41dc = private unnamed_addr constant [14 x i8] c"CLICOLOR_FORCE", align 1
@alloc_c940f1872184b67533cde325d4eb7ceb = private unnamed_addr constant [4 x i8] c"TERM", align 1
@alloc_0cb071d804372b212169612c3e5a40ce = private unnamed_addr constant [4 x i8] c"dumb", align 1
@alloc_5c8c5330792b76d76bc036d6b9372d18 = private unnamed_addr constant [8 x i8] c"CLICOLOR", align 1
@alloc_9e8821ebf06809bdf585485e81a043ea = private unnamed_addr constant [8 x i8] c"NO_COLOR", align 1
@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtBL_5strip9write_fmt0EEBL_, [16 x i8] c" \00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtCsen8ds7JR2I0_8anstream3fmtINtB4_7AdapterNCNvNtB6_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write9write_strB6_, ptr @_RNvYINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtB7_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write10write_charB7_, ptr @_RNvYINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtB7_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtB7_ }>, align 8
@alloc_118e5dd62e18907a47aec3e2be501119 = private unnamed_addr constant [15 x i8] c"formatter error", align 1
@alloc_843aa7820c0e6dd6cc59e2cfc7675b31 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsf_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrNtB5_10IsTerminal11is_terminal }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5write, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write14write_vectored, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write17is_write_vectored, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5flush, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_all, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write18write_all_vectored, ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_fmt, ptr @_RNvXsf_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrNtB5_10IsTerminal11is_terminal, ptr @alloc_843aa7820c0e6dd6cc59e2cfc7675b31 }>, align 8
@alloc_027f9229565fef6467cc79397c5c8fb2 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsd_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutNtB5_10IsTerminal11is_terminal }>, align 8
@vtable.2 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5write, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write14write_vectored, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write17is_write_vectored, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5flush, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_all, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write18write_all_vectored, ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_fmt, ptr @_RNvXsd_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutNtB5_10IsTerminal11is_terminal, ptr @alloc_027f9229565fef6467cc79397c5c8fb2 }>, align 8
@alloc_d1084648e479974e70c9329824bf76f9 = private unnamed_addr constant [9 x i8] c"mid > len", align 1
@alloc_4140925dbf06006e7a29bfbbfbd49eb3 = private unnamed_addr constant [2 x i8] c"CI", align 1
@alloc_9db393b76629f54c457052ca2b4031ba = private unnamed_addr constant [101 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anstream-0.6.21/src/strip.rs\00", align 1
@alloc_90124f96c6cc96c4a64c53170ed42508 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9db393b76629f54c457052ca2b4031ba, [16 x i8] c"d\00\00\00\00\00\00\00\83\00\00\00 \00\00\00" }>, align 8
@alloc_de4980c7a9bc962828f1e8f8f9f3228c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_9db393b76629f54c457052ca2b4031ba, [16 x i8] c"d\00\00\00\00\00\00\00\81\00\00\00(\00\00\00" }>, align 8
@anon.24dc06a1e6c291b3480b2052ac225214.0 = private unnamed_addr constant [4096 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\\\00\\\0A\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP################\B4\B4\B4\B4\B4\B4\B4\B4\B4\B4\B4\B4$$$$<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPPpppppppppppppppppppppppppppppppp\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0C\0Cp\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP                \02\02\02\02\02\02\02\02\02\02\02\02\02\02\02\02<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP################\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\02\02\02\02<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<<p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00pppppppppppppppppppppppp\00p\00\00pppp''''''''''''''''\B8\B8\B8\B8\B8\B8\B8\B8\B8\B8\B8\B8((((\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00pppppppppppppppppppppppp\00p\00\00pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0C\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00pppppppppppppppppppppppp\00p\00\00pppp                \06\06\06\06\06\06\06\06\06\06\06\06\06\06\06\06\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00pppppppppppppppppppppppp\00p\00\00pppp''''''''''''''''\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\B0\06\06\06\06\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09\09p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\00\D0\00\00\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0\D0p\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0C\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP++++++++++++++++LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLL\05LLLLLLL\0ELL\01L\0D\0E\0ELLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLp\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP                LLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLLp\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00PPPPPPPPPPPPPPPPPPPPPPPP\00P\00\00PPPP\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0\C0PPPPPPPPPPPPPPPP\00PPPPPPPPPP\00P\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\FF\00\00\00\00\00\00\00\00\00\00\00ppppppp\0Cpppppppppppppppp\00p\00\00pppp\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90\90pppppppppppppppppppppppp\00p\00\00pppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppppp\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\0C\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00", align 1
@alloc_05a47f67729265ef2e42046509dc1810 = private unnamed_addr constant [109 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anstream-0.6.21/src/adapter/strip.rs\00", align 1
@alloc_af6f16ef59dc1471d141b3b938a3b7e6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_05a47f67729265ef2e42046509dc1810, [16 x i8] c"l\00\00\00\00\00\00\00,\01\00\00\1B\00\00\00" }>, align 8
@alloc_3d3bc0ab4df5551832e4c9b01825d63f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_05a47f67729265ef2e42046509dc1810, [16 x i8] c"l\00\00\00\00\00\00\00B\01\00\00#\00\00\00" }>, align 8
@alloc_7415e7cefd3b1fa46f24f7cbc1fe8f95 = private unnamed_addr constant [116 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/io/stdio.rs\00", align 1
@alloc_b5a27d2ad8ac297ce0dc342bcb05cc9c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7415e7cefd3b1fa46f24f7cbc1fe8f95, [16 x i8] c"s\00\00\00\00\00\00\00V\03\00\00\14\00\00\00" }>, align 8
@alloc_66d59cd97f643b0f3a5d9f737f46b52c = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_7415e7cefd3b1fa46f24f7cbc1fe8f95, [16 x i8] c"s\00\00\00\00\00\00\00>\04\00\00\14\00\00\00" }>, align 8

; core::ptr::drop_in_place::<anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtBL_5strip9write_fmt0EEBL_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %.val = load ptr, ptr %0, align 8, !noundef !2
  %bits.i.i.i.i.i = ptrtoint ptr %.val to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsen8ds7JR2I0_8anstream.exit, !prof !3

bb2.i2.i.i.i.i:                                   ; preds = %start
  %1 = getelementptr i8, ptr %.val, i64 -1
  %2 = icmp ne ptr %1, null
  tail call void @llvm.assume(i1 %2)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %1, align 8
  %3 = getelementptr i8, ptr %.val, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %3, align 8, !nonnull !2, !align !4, !noundef !2
  %4 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !2
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %4, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %5 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %5)
  invoke void %4(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %6 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %6)
  %7 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %8 = load i64, ptr %7, align 8, !range !5, !invariant.load !2
  %9 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %10 = load i64, ptr %9, align 8, !range !6, !invariant.load !2
  %11 = add i64 %10, -1
  %12 = icmp sgt i64 %11, -1
  tail call void @llvm.assume(i1 %12)
  %13 = icmp eq i64 %8, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) %10) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %14 = landingpad { ptr, i32 }
          cleanup
  %15 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !5, !invariant.load !2
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !6, !invariant.load !2
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  tail call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #22
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef 24, i64 noundef 8) #22
  resume { ptr, i32 } %14

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %1, i64 noundef 24, i64 noundef 8) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsen8ds7JR2I0_8anstream.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6result6ResultuNtNtNtCs5sEH5CPMdak_3std2io5error5ErrorEECsen8ds7JR2I0_8anstream.exit: ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsen8ds7JR2I0_8anstream(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef range(i64 0, -9223372036854775808) %additional) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !7)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !5, !alias.scope !7, !noundef !2
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !7
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !7
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsen8ds7JR2I0_8anstream(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !10, !noalias !7, !noundef !2
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !11, !noalias !7, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !7
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !7
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #23
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !7, !nonnull !2, !noundef !2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !7
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !7
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !7
  ret void
}

; anstream::stderr
; Function Attrs: uwtable
define void @_RNvCsen8ds7JR2I0_8anstream6stderr(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %raw.i = alloca [8 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !12)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %raw.i)
  store ptr @_RNvNvNtNtCs5sEH5CPMdak_3std2io5stdio6stderr8INSTANCE, ptr %raw.i, align 8, !noalias !12
; call anstream::auto::choice
  %choice.i = call noundef i8 @_RNvNtCsen8ds7JR2I0_8anstream4auto6choice(ptr noundef nonnull align 1 %raw.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(88) @vtable.1), !noalias !12
  %_7.i = load ptr, ptr %raw.i, align 8, !noalias !12, !nonnull !2, !align !4, !noundef !2
  %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 12
  switch i8 %choice.i, label %default.unreachable [
    i8 3, label %bb2.i.i
    i8 1, label %bb4.i.i
    i8 2, label %bb3.i.i
  ]

default.unreachable:                              ; preds = %start
  unreachable

bb4.i.i:                                          ; preds = %start
  call void @llvm.experimental.noalias.scope.decl(metadata !15)
  %_3.i.i.i1.i = call noundef i32 @isatty(i32 noundef 2) #22, !noalias !18
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !18
  store i8 8, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !18
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_.exit

bb3.i.i:                                          ; preds = %start
  call void @llvm.experimental.noalias.scope.decl(metadata !19)
  %_3.i.i.i.i = call noundef i32 @isatty(i32 noundef 2) #22, !noalias !22
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !22
  store i8 8, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !22
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_.exit

bb2.i.i:                                          ; preds = %start
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !23
  %_4.sroa.4.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i32 0, ptr %_4.sroa.4.0._0.sroa_idx.i.i, align 8, !alias.scope !23
  store i8 0, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !23
  %_4.sroa.4.sroa.6.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i8 12, ptr %_4.sroa.4.sroa.6.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !23
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_.exit

_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_.exit: ; preds = %bb4.i.i, %bb3.i.i, %bb2.i.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %raw.i)
  ret void
}

; anstream::stdout
; Function Attrs: uwtable
define void @_RNvCsen8ds7JR2I0_8anstream6stdout(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %raw.i = alloca [8 x i8], align 8
; call std::io::stdio::stdout
  %stdout = tail call noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6stdout()
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %raw.i)
  store ptr %stdout, ptr %raw.i, align 8, !noalias !26
; call anstream::auto::choice
  %choice.i = call noundef i8 @_RNvNtCsen8ds7JR2I0_8anstream4auto6choice(ptr noundef nonnull align 1 %raw.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(88) @vtable.2), !noalias !26
  %_7.i = load ptr, ptr %raw.i, align 8, !noalias !26, !nonnull !2, !align !4, !noundef !2
  %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 12
  switch i8 %choice.i, label %default.unreachable [
    i8 3, label %bb2.i.i
    i8 1, label %bb4.i.i
    i8 2, label %bb3.i.i
  ]

default.unreachable:                              ; preds = %start
  unreachable

bb4.i.i:                                          ; preds = %start
  call void @llvm.experimental.noalias.scope.decl(metadata !29)
  %_3.i.i.i1.i = call noundef i32 @isatty(i32 noundef 1) #22, !noalias !32
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !32
  store i8 8, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !32
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_.exit

bb3.i.i:                                          ; preds = %start
  call void @llvm.experimental.noalias.scope.decl(metadata !33)
  %_3.i.i.i.i = call noundef i32 @isatty(i32 noundef 1) #22, !noalias !36
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !36
  store i8 8, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !36
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_.exit

bb2.i.i:                                          ; preds = %start
  store ptr %_7.i, ptr %_0, align 8, !alias.scope !37
  %_4.sroa.4.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i32 0, ptr %_4.sroa.4.0._0.sroa_idx.i.i, align 8, !alias.scope !37
  store i8 0, ptr %_4.sroa.4.sroa.4.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 4, !alias.scope !37
  %_4.sroa.4.sroa.6.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i8 12, ptr %_4.sroa.4.sroa.6.0._4.sroa.4.0._0.sroa_idx.sroa_idx.i.i, align 8, !alias.scope !37
  br label %_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_.exit

_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_.exit: ; preds = %bb4.i.i, %bb3.i.i, %bb2.i.i
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %raw.i)
  ret void
}

; <anstream::adapter::wincon::WinconBytes>::extract_next
; Function Attrs: uwtable
define void @_RNvMNtNtCsen8ds7JR2I0_8anstream7adapter6winconNtB2_11WinconBytes12extract_next(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) initializes((0, 32)) %_0, ptr noalias noundef align 8 dereferenceable(480) initializes((462, 463)) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %bytes.0, i64 noundef range(i64 0, -9223372036854775808) %bytes.1) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 424
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 462
  store i8 4, ptr %1, align 2
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 440
  %len.i = load i64, ptr %2, align 8, !alias.scope !40, !noundef !2
  %self2.i = load i64, ptr %0, align 8, !range !5, !alias.scope !40, !noundef !2
  %_9.i = sub i64 %self2.i, %len.i
  %_7.i = icmp ugt i64 %bytes.1, %_9.i
  br i1 %_7.i, label %bb1.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit, !prof !43

bb1.i:                                            ; preds = %start
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsen8ds7JR2I0_8anstream(ptr noalias noundef nonnull align 8 dereferenceable(24) %0, i64 noundef %len.i, i64 noundef range(i64 0, -9223372036854775808) %bytes.1)
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit: ; preds = %start, %bb1.i
  store ptr %bytes.0, ptr %_0, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %bytes.1, ptr %3, align 8
  %4 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %self, ptr %4, align 8
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store ptr %0, ptr %5, align 8
  ret void
}

; <anstream::adapter::wincon::WinconBytes>::new
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define void @_RNvMNtNtCsen8ds7JR2I0_8anstream7adapter6winconNtB2_11WinconBytes3new(ptr dead_on_unwind noalias noundef writable writeonly sret([480 x i8]) align 8 captures(none) dereferenceable(480) initializes((0, 29), (32, 422), (424, 449), (452, 453), (456, 457), (460, 463)) %_0) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_1.sroa.12 = alloca [263 x i8], align 1
  call void @llvm.lifetime.start.p0(i64 263, ptr nonnull %_1.sroa.12)
  %_1.sroa.12.7.sroa_idx = getelementptr inbounds nuw i8, ptr %_1.sroa.12, i64 7
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 1 dereferenceable(256) %_1.sroa.12.7.sroa_idx, i8 0, i64 256, i1 false), !alias.scope !44
  %_1.sroa.9.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 40
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(96) %_1.sroa.9.0._0.sroa_idx, i8 0, i64 96, i1 false)
  store i64 0, ptr %_0, align 8
  %_1.sroa.4.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_1.sroa.4.0._0.sroa_idx, align 8
  %_1.sroa.5.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 0, ptr %_1.sroa.5.0._0.sroa_idx, align 8
  %_1.sroa.6.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i32 0, ptr %_1.sroa.6.0._0.sroa_idx, align 8
  %_1.sroa.7.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 28
  store i8 0, ptr %_1.sroa.7.0._0.sroa_idx, align 4
  %_1.sroa.83.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 32
  store i64 0, ptr %_1.sroa.83.0._0.sroa_idx, align 8
  %_1.sroa.10.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 136
  store i64 0, ptr %_1.sroa.10.0._0.sroa_idx, align 8
  %_1.sroa.11.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 144
  store i8 0, ptr %_1.sroa.11.0._0.sroa_idx, align 8
  %_1.sroa.12.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 145
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(263) %_1.sroa.12.0._0.sroa_idx, ptr noundef nonnull align 1 dereferenceable(263) %_1.sroa.12, i64 263, i1 false)
  %_1.sroa.13.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 408
  %_1.sroa.17.0._0.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 421
  tail call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(13) %_1.sroa.13.0._0.sroa_idx, i8 0, i64 13, i1 false)
  store i8 12, ptr %_1.sroa.17.0._0.sroa_idx, align 1
  %0 = getelementptr inbounds nuw i8, ptr %_0, i64 424
  store i64 0, ptr %0, align 8
  %_2.sroa.0.sroa.0.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 432
  store ptr inttoptr (i64 1 to ptr), ptr %_2.sroa.0.sroa.0.sroa.4.0..sroa_idx, align 8
  %_2.sroa.0.sroa.0.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 440
  store i64 0, ptr %_2.sroa.0.sroa.0.sroa.5.0..sroa_idx, align 8
  %_2.sroa.0.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 448
  store i8 3, ptr %_2.sroa.0.sroa.4.0..sroa_idx, align 8
  %_2.sroa.0.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 452
  store i8 3, ptr %_2.sroa.0.sroa.6.0..sroa_idx, align 4
  %_2.sroa.0.sroa.8.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 456
  store i8 3, ptr %_2.sroa.0.sroa.8.0..sroa_idx, align 8
  %_2.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 460
  store i16 0, ptr %_2.sroa.5.0..sroa_idx, align 4
  %_2.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 462
  store i8 4, ptr %_2.sroa.6.0..sroa_idx, align 2
  call void @llvm.lifetime.end.p0(i64 263, ptr nonnull %_1.sroa.12)
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCsen8ds7JR2I0_8anstream(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #3 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %or.cond.not = icmp sgt i64 %cap, 0
  br i1 %or.cond.not, label %bb14, label %bb11, !prof !47

bb14:                                             ; preds = %bb6.i
  %0 = icmp eq i64 %self.0.val, 0
  br i1 %0, label %bb4.i.i11, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit

bb14.thread:                                      ; preds = %start
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb9, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit: ; preds = %bb14.thread, %bb14
  %2 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %2)
  %cond.i.i = icmp uge i64 %cap, %self.0.val
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %self.0.val, i64 noundef range(i64 1, -9223372036854775807) 1, i64 noundef %cap) #22
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #22
; call __rustc::__rust_alloc
  %3 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %cap, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator4grow.exit ], [ %3, %bb4.i.i11 ]
  %4 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %4, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %5 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 1, ptr %5, align 8
  br label %bb11

bb9:                                              ; preds = %bb14.thread, %bb7
  %raw_ptr.i.i.pn22 = phi ptr [ %raw_ptr.i.i.pn, %bb7 ], [ inttoptr (i64 1 to ptr), %bb14.thread ]
  %_27.sroa.7.01121 = phi i64 [ %cap, %bb7 ], [ 0, %bb14.thread ]
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn22, ptr %6, align 8
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb9, %bb8
  %.sink23 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %bb6.i ]
  %_27.sroa.7.01121.sink = phi i64 [ %_27.sroa.7.01121, %bb9 ], [ %cap, %bb8 ], [ 0, %bb6.i ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %bb6.i ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink23
  store i64 %_27.sroa.7.01121.sink, ptr %7, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <anstream::adapter::strip::Utf8Parser>::add
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable
define noundef zeroext i1 @_RNvMs7_NtNtCsen8ds7JR2I0_8anstream7adapter5stripNtB5_10Utf8Parser3add(ptr noalias noundef align 4 captures(none) dereferenceable(8) %self, i8 noundef %byte) unnamed_addr #4 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 4
  %_7.i = load i8, ptr %0, align 4, !range !48, !alias.scope !49, !noundef !2
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
  br i1 %_13.i.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit, label %bb12.i.i

bb4.i.i:                                          ; preds = %start
  %or.cond5.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond5.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb3.i.i:                                          ; preds = %start
  %or.cond6.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond6.i.i, label %bb6.i5.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb2.i.i:                                          ; preds = %start
  %or.cond7.i.i = icmp slt i8 %byte, -64
  br i1 %or.cond7.i.i, label %bb7.i6.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb8.i.i:                                          ; preds = %start
  %1 = and i8 %byte, -32
  %or.cond8.i.i = icmp eq i8 %1, -96
  br i1 %or.cond8.i.i, label %bb6.i5.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb7.i.i:                                          ; preds = %start
  %or.cond9.i.i = icmp slt i8 %byte, -96
  br i1 %or.cond9.i.i, label %bb6.i5.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb6.i.i:                                          ; preds = %start
  %2 = add i8 %byte, 112
  %or.cond10.i.i = icmp ult i8 %2, 48
  br i1 %or.cond10.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb5.i.i:                                          ; preds = %start
  %or.cond11.i.i = icmp slt i8 %byte, -112
  br i1 %or.cond11.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb12.i.i:                                         ; preds = %bb9.i.i
  %3 = add nsw i8 %byte, 62
  %or.cond1.i.i = icmp ult i8 %3, 30
  br i1 %or.cond1.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i, label %bb14.i.i

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i: ; preds = %bb12.i.i
  %_20.i.i = and i8 %byte, 31
  %_19.i.i = zext nneg i8 %_20.i.i to i32
  %_18.i.i = shl nuw nsw i32 %_19.i.i, 6
  %4 = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %5 = or i32 %4, %_18.i.i
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

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
  br i1 %or.cond4.i.i, label %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i, label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i: ; preds = %bb5.i.i, %bb6.i.i, %bb4.i.i
  %_23.i.i = and i8 %byte, 63
  %_22.i.i = zext nneg i8 %_23.i.i to i32
  %_21.i.i = shl nuw nsw i32 %_22.i.i, 12
  %9 = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %10 = or i32 %9, %_21.i.i
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i: ; preds = %bb28.i.i, %bb15.i.i, %bb14.i.i
  %_0.sroa.0.0.i.ph16.i = phi i8 [ 2, %bb15.i.i ], [ 4, %bb14.i.i ], [ 5, %bb28.i.i ]
  %_26.i.i = and i8 %byte, 15
  %_25.i.i = zext nneg i8 %_26.i.i to i32
  %_24.i.i = shl nuw nsw i32 %_25.i.i, 12
  %11 = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %12 = or i32 %11, %_24.i.i
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i: ; preds = %bb19.i.i, %bb26.i.i, %bb14.i.i
  %_0.sroa.0.0.i.ph22.i = phi i8 [ 7, %bb26.i.i ], [ 6, %bb14.i.i ], [ 1, %bb19.i.i ]
  %_29.i.i = and i8 %byte, 7
  %_28.i.i = zext nneg i8 %_29.i.i to i32
  %_27.i.i = shl nuw nsw i32 %_28.i.i, 18
  %13 = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %14 = or i32 %13, %_27.i.i
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb7.i6.i:                                         ; preds = %bb2.i.i
  %_10.i.i = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %15 = icmp ult i32 %_10.i.i, 1114112
  tail call void @llvm.assume(i1 %15)
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

bb6.i5.i:                                         ; preds = %bb7.i.i, %bb8.i.i, %bb3.i.i
  %_17.i.i = and i8 %byte, 63
  %_16.i.i = zext nneg i8 %_17.i.i to i32
  %_15.i.i = shl nuw nsw i32 %_16.i.i, 6
  %16 = load i32, ptr %self, align 4, !alias.scope !52, !noundef !2
  %17 = or i32 %16, %_15.i.i
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split

_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split: ; preds = %bb4.i.i, %bb2.i.i, %bb6.i.i, %bb5.i.i, %bb19.i.i, %bb7.i.i, %bb8.i.i, %bb3.i.i, %bb6.i5.i, %bb7.i6.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i
  %.sink = phi i32 [ %5, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i ], [ %10, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i ], [ %12, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i ], [ %14, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i ], [ 0, %bb7.i6.i ], [ %17, %bb6.i5.i ], [ 0, %bb3.i.i ], [ 0, %bb8.i.i ], [ 0, %bb7.i.i ], [ 0, %bb19.i.i ], [ 0, %bb5.i.i ], [ 0, %bb6.i.i ], [ 0, %bb2.i.i ], [ 0, %bb4.i.i ]
  %b.sroa.0.0.off0.ph = phi i1 [ false, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i ], [ false, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i ], [ false, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i ], [ false, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i ], [ true, %bb7.i6.i ], [ false, %bb6.i5.i ], [ true, %bb3.i.i ], [ true, %bb8.i.i ], [ true, %bb7.i.i ], [ true, %bb19.i.i ], [ true, %bb5.i.i ], [ true, %bb6.i.i ], [ true, %bb2.i.i ], [ true, %bb4.i.i ]
  %_0.sroa.0.0.i7.i.ph = phi i8 [ 3, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread35.i ], [ 2, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread11.i ], [ %_0.sroa.0.0.i.ph16.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread17.i ], [ %_0.sroa.0.0.i.ph22.i, %_RNvMNtCs8N0ImSjNXUE_9utf8parse5typesNtB2_5State7advance.exit.thread23.i ], [ 0, %bb7.i6.i ], [ 3, %bb6.i5.i ], [ 0, %bb3.i.i ], [ 0, %bb8.i.i ], [ 0, %bb7.i.i ], [ 0, %bb19.i.i ], [ 0, %bb5.i.i ], [ 0, %bb6.i.i ], [ 0, %bb2.i.i ], [ 0, %bb4.i.i ]
  store i32 %.sink, ptr %self, align 4, !alias.scope !52
  br label %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit

_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit: ; preds = %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split, %bb9.i.i
  %b.sroa.0.0.off0 = phi i1 [ true, %bb9.i.i ], [ %b.sroa.0.0.off0.ph, %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split ]
  %_0.sroa.0.0.i7.i = phi i8 [ 0, %bb9.i.i ], [ %_0.sroa.0.0.i7.i.ph, %_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_.exit.sink.split ]
  store i8 %_0.sroa.0.0.i7.i, ptr %0, align 4, !alias.scope !49
  ret i1 %b.sroa.0.0.off0
}

; anstream::auto::choice
; Function Attrs: uwtable
define noundef range(i8 1, 4) i8 @_RNvNtCsen8ds7JR2I0_8anstream4auto6choice(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(88) %raw.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_1.i = alloca [24 x i8], align 8
  %_2.i24 = alloca [24 x i8], align 8
  %_2.i18 = alloca [24 x i8], align 8
  %_2.i11 = alloca [24 x i8], align 8
  %_2.i = alloca [24 x i8], align 8
  %_3.i = alloca [24 x i8], align 8
  %_13 = alloca [24 x i8], align 8
; call <colorchoice::ColorChoice>::global
  %0 = tail call noundef i8 @_RNvMCs314fcNNe6oa_11colorchoiceNtB2_11ColorChoice6global()
  %switch = icmp eq i8 %0, 0
  br i1 %switch, label %bb4, label %bb24

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_3.i)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_3.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_5c8c5330792b76d76bc036d6b9372d18, i64 noundef 8)
  %1 = load i64, ptr %_3.i, align 8, !range !11, !noundef !2
  %.not.i = icmp ne i64 %1, -9223372036854775808
  br i1 %.not.i, label %bb8.i, label %bb25

bb8.i:                                            ; preds = %bb4
  %_8.sroa.5.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  %_8.sroa.5.0.copyload.i = load ptr, ptr %_8.sroa.5.0._3.sroa_idx.i, align 8, !nonnull !2, !noundef !2
  %_8.sroa.6.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  %_8.sroa.6.0.copyload.i = load i64, ptr %_8.sroa.6.0._3.sroa_idx.i, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i)
  %_3.not.i.i = icmp eq i64 %_8.sroa.6.0.copyload.i, 1
  br i1 %_3.not.i.i, label %bb2.i.i, label %bb9.i

bb2.i.i:                                          ; preds = %bb8.i
  %lhsc.i = load i8, ptr %_8.sroa.5.0.copyload.i, align 1
  %2 = icmp eq i8 %lhsc.i, 48
  br label %bb9.i

bb9.i:                                            ; preds = %bb2.i.i, %bb8.i
  %_0.sroa.0.0.off0.i.i = phi i1 [ %2, %bb2.i.i ], [ false, %bb8.i ]
  %3 = icmp eq i64 %1, 0
  br i1 %3, label %bb26, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i4.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i4.i: ; preds = %bb9.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_8.sroa.5.0.copyload.i, i64 noundef %1, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %bb26

bb26:                                             ; preds = %bb9.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i4.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9e8821ebf06809bdf585485e81a043ea, i64 noundef 8)
  %4 = load i64, ptr %_2.i, align 8, !range !11, !noundef !2
  %5 = getelementptr inbounds nuw i8, ptr %_2.i, i64 16
  %_10.i = load i64, ptr %5, align 8
  switch i64 %4, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i [
    i64 -9223372036854775808, label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit
    i64 0, label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit
  ]

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i: ; preds = %bb26
  %6 = getelementptr inbounds nuw i8, ptr %_2.i, i64 8
  %_2.val1.i = load ptr, ptr %6, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val1.i, i64 noundef %4, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit

_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit:  ; preds = %bb26, %bb26, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i
  %.not.i10 = icmp ne i64 %4, -9223372036854775808
  %7 = icmp ne i64 %_10.i, 0
  %_15.sroa.6.0.i = select i1 %.not.i10, i1 %7, i1 false
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i)
  br i1 %_15.sroa.6.0.i, label %bb24, label %bb8

bb25:                                             ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_3.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i11)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i11, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9e8821ebf06809bdf585485e81a043ea, i64 noundef 8)
  %8 = load i64, ptr %_2.i11, align 8, !range !11, !noundef !2
  %9 = getelementptr inbounds nuw i8, ptr %_2.i11, i64 16
  %_10.i12 = load i64, ptr %9, align 8
  switch i64 %8, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i15 [
    i64 -9223372036854775808, label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17
    i64 0, label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17
  ]

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i15: ; preds = %bb25
  %10 = getelementptr inbounds nuw i8, ptr %_2.i11, i64 8
  %_2.val1.i16 = load ptr, ptr %10, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val1.i16, i64 noundef %8, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17

_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17: ; preds = %bb25, %bb25, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i15
  %.not.i13 = icmp ne i64 %8, -9223372036854775808
  %11 = icmp ne i64 %_10.i12, 0
  %_15.sroa.6.0.i14 = select i1 %.not.i13, i1 %11, i1 false
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i11)
  br i1 %_15.sroa.6.0.i14, label %bb24, label %bb30

bb30:                                             ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i18)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i18, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_787344c135fb1ca228327e24d16f41dc, i64 noundef 14)
  %12 = load i64, ptr %_2.i18, align 8, !range !11, !noundef !2
  %13 = getelementptr inbounds nuw i8, ptr %_2.i18, i64 16
  %_10.i19 = load i64, ptr %13, align 8
  switch i64 %12, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i22 [
    i64 -9223372036854775808, label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit
    i64 0, label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit
  ]

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i22: ; preds = %bb30
  %14 = getelementptr inbounds nuw i8, ptr %_2.i18, i64 8
  %_2.val1.i23 = load ptr, ptr %14, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val1.i23, i64 noundef %12, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit

_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit: ; preds = %bb30, %bb30, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i22
  %.not.i20 = icmp ne i64 %12, -9223372036854775808
  %15 = icmp ne i64 %_10.i19, 0
  %_15.sroa.6.0.i21 = select i1 %.not.i20, i1 %15, i1 false
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i18)
  br i1 %_15.sroa.6.0.i21, label %bb24, label %bb13

bb13:                                             ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30, %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit
  %16 = getelementptr inbounds nuw i8, ptr %raw.1, i64 80
  %17 = load ptr, ptr %16, align 8, !invariant.load !2, !nonnull !2
  %_9 = call noundef zeroext i1 %17(ptr noundef nonnull align 1 %raw.0) #24
  br i1 %_9, label %bb15, label %bb24

bb8:                                              ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_2.i24)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_2.i24, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_787344c135fb1ca228327e24d16f41dc, i64 noundef 14)
  %18 = load i64, ptr %_2.i24, align 8, !range !11, !noundef !2
  %19 = getelementptr inbounds nuw i8, ptr %_2.i24, i64 16
  %_10.i25 = load i64, ptr %19, align 8
  switch i64 %18, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i28 [
    i64 -9223372036854775808, label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30
    i64 0, label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30
  ]

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i28: ; preds = %bb8
  %20 = getelementptr inbounds nuw i8, ptr %_2.i24, i64 8
  %_2.val1.i29 = load ptr, ptr %20, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_2.val1.i29, i64 noundef %18, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30

_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30: ; preds = %bb8, %bb8, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i.i28
  %.not.i26 = icmp ne i64 %18, -9223372036854775808
  %21 = icmp ne i64 %_10.i25, 0
  %_15.sroa.6.0.i27 = select i1 %.not.i26, i1 %21, i1 false
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_2.i24)
  %brmerge = select i1 %_15.sroa.6.0.i27, i1 true, i1 %_0.sroa.0.0.off0.i.i
  %.mux = select i1 %_15.sroa.6.0.i27, i8 2, i8 3
  br i1 %brmerge, label %bb24, label %bb13

bb15:                                             ; preds = %bb13
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_1.i)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_1.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c940f1872184b67533cde325d4eb7ceb, i64 noundef 4)
  %22 = load i64, ptr %_1.i, align 8, !range !11, !noundef !2
  %.not.i31 = icmp eq i64 %22, -9223372036854775808
  br i1 %.not.i31, label %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit, label %bb3.i

bb3.i:                                            ; preds = %bb15
  %k.sroa.7.0._1.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_1.i, i64 8
  %k.sroa.7.0.copyload.i = load ptr, ptr %k.sroa.7.0._1.sroa_idx.i, align 8, !nonnull !2, !noundef !2
  %k.sroa.11.0._1.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_1.i, i64 16
  %k.sroa.11.0.copyload.i = load i64, ptr %k.sroa.11.0._1.sroa_idx.i, align 8
  %_3.not.i.i32 = icmp eq i64 %k.sroa.11.0.copyload.i, 4
  br i1 %_3.not.i.i32, label %bb13.i, label %bb6.i

bb13.i:                                           ; preds = %bb3.i
  %23 = call i32 @memcmp(ptr noundef nonnull readonly align 1 dereferenceable(4) %k.sroa.7.0.copyload.i, ptr noundef nonnull dereferenceable(4) @alloc_0cb071d804372b212169612c3e5a40ce, i64 range(i64 0, -9223372036854775808) 4), !alias.scope !55
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %bb5.i, label %bb6.i

bb6.i:                                            ; preds = %bb13.i, %bb3.i
  %25 = icmp eq i64 %22, 0
  br i1 %25, label %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit.thread, label %bb9.sink.split.i

_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit.thread: ; preds = %bb6.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_1.i)
  br label %bb19

bb5.i:                                            ; preds = %bb13.i
  %26 = icmp eq i64 %22, 0
  br i1 %26, label %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit, label %bb9.sink.split.i

bb9.sink.split.i:                                 ; preds = %bb5.i, %bb6.i
  %_0.sroa.0.1.off0.ph.i = phi i1 [ true, %bb6.i ], [ false, %bb5.i ]
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %k.sroa.7.0.copyload.i, i64 noundef %22, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit

_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit: ; preds = %bb15, %bb5.i, %bb9.sink.split.i
  %_0.sroa.0.1.off0.i = phi i1 [ false, %bb5.i ], [ false, %bb15 ], [ %_0.sroa.0.1.off0.ph.i, %bb9.sink.split.i ]
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_1.i)
  %or.cond = or i1 %.not.i, %_0.sroa.0.1.off0.i
  br i1 %or.cond, label %bb19, label %bb18

bb19:                                             ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit.thread, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit, %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit
  br label %bb24

bb18:                                             ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query19term_supports_color.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_13)
; call std::env::_var_os
  call void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(address) dereferenceable(24) %_13, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4140925dbf06006e7a29bfbbfbd49eb3, i64 noundef 2)
  %27 = load i64, ptr %_13, align 8, !range !11, !noundef !2
  %.not8 = icmp eq i64 %27, -9223372036854775808
  switch i64 %27, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit
  ]

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i: ; preds = %bb18
  %28 = getelementptr inbounds nuw i8, ptr %_13, i64 8
  %_13.val9 = load ptr, ptr %28, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_13.val9, i64 noundef %27, i64 noundef range(i64 1, -9223372036854775807) 1) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit: ; preds = %bb18, %bb18, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_13)
  br i1 %.not8, label %bb24, label %bb19

bb24:                                             ; preds = %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30, %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit, %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17, %bb13, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit, %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit, %bb19, %start
  %choice.sroa.0.2 = phi i8 [ %0, %start ], [ 3, %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit ], [ 3, %_RNvCsfNlaCBqeVed_13anstyle_query8no_color.exit17 ], [ 2, %bb19 ], [ %.mux, %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit30 ], [ 2, %_RNvCsfNlaCBqeVed_13anstyle_query14clicolor_force.exit ], [ 3, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCs5sEH5CPMdak_3std3ffi6os_str8OsStringEECsen8ds7JR2I0_8anstream.exit ], [ 3, %bb13 ]
  ret i8 %choice.sroa.0.2
}

; anstream::strip::write
; Function Attrs: uwtable
define { i64, ptr } @_RNvNtCsen8ds7JR2I0_8anstream5strip5write(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(80) %raw.1, ptr noalias noundef align 4 dereferenceable(12) %state, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_19 = alloca [32 x i8], align 8
  %iter = alloca [32 x i8], align 8
  %initial_state = alloca [12 x i8], align 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %initial_state, ptr noundef nonnull align 4 dereferenceable(12) %state, i64 12, i1 false)
  %_21 = getelementptr inbounds nuw i8, ptr %state, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter)
  store ptr %buf.0, ptr %iter, align 8
  %_5.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %buf.1, ptr %_5.sroa.2.0.iter.sroa_idx, align 8
  %_5.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %_21, ptr %_5.sroa.3.0.iter.sroa_idx, align 8
  %_5.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store ptr %state, ptr %_5.sroa.4.0.iter.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %raw.1, i64 24
  %1 = load ptr, ptr %0, align 8, !nonnull !2
  br label %bb1

bb1:                                              ; preds = %bb12, %start
  %_24 = load ptr, ptr %_5.sroa.3.0.iter.sroa_idx, align 8, !nonnull !2, !align !59, !noundef !2
  %_25 = load ptr, ptr %_5.sroa.4.0.iter.sroa_idx, align 8, !nonnull !2, !align !60, !noundef !2
; call anstream::adapter::strip::next_bytes
  %2 = call fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef align 8 dereferenceable(16) %iter, ptr noalias noundef align 1 dereferenceable(1) %_24, ptr noalias noundef align 4 dereferenceable(8) %_25) #24
  %3 = extractvalue { ptr, i64 } %2, 0
  %4 = extractvalue { ptr, i64 } %2, 1
  %.not = icmp eq ptr %3, null
  br i1 %.not, label %bb9, label %bb3

bb3:                                              ; preds = %bb1
  %5 = tail call { i64, ptr } %1(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %3, i64 noundef %4) #24
  %6 = extractvalue { i64, ptr } %5, 0
  %7 = extractvalue { i64, ptr } %5, 1
  %8 = ptrtoint ptr %7 to i64
  %9 = trunc nuw i64 %6 to i1
  br i1 %9, label %bb9, label %bb12

bb9:                                              ; preds = %bb1, %bb3, %_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit
  %_0.sroa.4.0 = phi i64 [ %offset, %_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit ], [ %8, %bb3 ], [ %buf.1, %bb1 ]
  %_0.sroa.0.0 = phi i64 [ 0, %_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit ], [ 1, %bb3 ], [ 0, %bb1 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter)
  %10 = inttoptr i64 %_0.sroa.4.0 to ptr
  %11 = insertvalue { i64, ptr } poison, i64 %_0.sroa.0.0, 0
  %12 = insertvalue { i64, ptr } %11, ptr %10, 1
  ret { i64, ptr } %12

bb12:                                             ; preds = %bb3
  %_15.not = icmp eq i64 %4, %8
  br i1 %_15.not, label %bb1, label %bb6

bb6:                                              ; preds = %bb12
  %_31 = icmp ult i64 %4, %8
  br i1 %_31, label %bb13, label %bb14, !prof !43

bb14:                                             ; preds = %bb6
  %_37 = getelementptr inbounds nuw i8, ptr %3, i64 %8
  %_40 = ptrtoint ptr %_37 to i64
  %_41 = ptrtoint ptr %buf.0 to i64
  %offset = sub i64 %_40, %_41
  %_44 = icmp ugt i64 %offset, %buf.1
  br i1 %_44, label %bb15, label %bb16, !prof !43

bb13:                                             ; preds = %bb6
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %8, i64 noundef %4, i64 noundef %4, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_de4980c7a9bc962828f1e8f8f9f3228c) #25
  unreachable

bb16:                                             ; preds = %bb14
  %_47 = sub nuw nsw i64 %buf.1, %offset
  %_51 = getelementptr inbounds nuw i8, ptr %buf.0, i64 %offset
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %state, ptr noundef nonnull align 4 dereferenceable(12) %initial_state, i64 12, i1 false)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_19)
  store ptr %_51, ptr %_19, align 8
  %13 = getelementptr inbounds nuw i8, ptr %_19, i64 8
  store i64 %_47, ptr %13, align 8
  %14 = getelementptr inbounds nuw i8, ptr %_19, i64 16
  store ptr %_21, ptr %14, align 8
  %15 = getelementptr inbounds nuw i8, ptr %_19, i64 24
  store ptr %state, ptr %15, align 8
; call anstream::adapter::strip::next_bytes
  %16 = call fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef nonnull align 8 dereferenceable(32) %_19, ptr noalias noundef align 1 dereferenceable(1) %_21, ptr noalias noundef align 4 dereferenceable(8) %state) #24
  %17 = extractvalue { ptr, i64 } %16, 0
  %.not3.i = icmp eq ptr %17, null
  br i1 %.not3.i, label %_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit, label %bb3.i

bb3.i:                                            ; preds = %bb16, %bb3.i
  %_3.i.i = load ptr, ptr %14, align 8, !alias.scope !61, !nonnull !2, !align !59, !noundef !2
  %_4.i.i = load ptr, ptr %15, align 8, !alias.scope !61, !nonnull !2, !align !60, !noundef !2
; call anstream::adapter::strip::next_bytes
  %18 = call fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef nonnull align 8 dereferenceable(32) %_19, ptr noalias noundef align 1 dereferenceable(1) %_3.i.i, ptr noalias noundef align 4 dereferenceable(8) %_4.i.i) #24
  %19 = extractvalue { ptr, i64 } %18, 0
  %.not.i = icmp eq ptr %19, null
  br i1 %.not.i, label %_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit, label %bb3.i

_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_.exit: ; preds = %bb3.i, %bb16
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_19)
  br label %bb9

bb15:                                             ; preds = %bb14
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef %offset, i64 noundef %buf.1, i64 noundef %buf.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_90124f96c6cc96c4a64c53170ed42508) #25
  unreachable
}

; anstream::strip::write_all
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsen8ds7JR2I0_8anstream5strip9write_all(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(80) %raw.1, ptr noalias noundef align 4 dereferenceable(12) %state, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %buf.0, i64 noundef range(i64 0, -9223372036854775808) %buf.1) unnamed_addr #0 {
start:
  %iter = alloca [32 x i8], align 8
  %_12 = getelementptr inbounds nuw i8, ptr %state, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter)
  store ptr %buf.0, ptr %iter, align 8
  %_4.sroa.2.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 %buf.1, ptr %_4.sroa.2.0.iter.sroa_idx, align 8
  %_4.sroa.3.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 16
  store ptr %_12, ptr %_4.sroa.3.0.iter.sroa_idx, align 8
  %_4.sroa.4.0.iter.sroa_idx = getelementptr inbounds nuw i8, ptr %iter, i64 24
  store ptr %state, ptr %_4.sroa.4.0.iter.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %raw.1, i64 56
  %1 = load ptr, ptr %0, align 8, !nonnull !2
  br label %bb1

bb1:                                              ; preds = %bb3, %start
  %_15 = load ptr, ptr %_4.sroa.3.0.iter.sroa_idx, align 8, !nonnull !2, !align !59, !noundef !2
  %_16 = load ptr, ptr %_4.sroa.4.0.iter.sroa_idx, align 8, !nonnull !2, !align !60, !noundef !2
; call anstream::adapter::strip::next_bytes
  %2 = call fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef align 8 dereferenceable(16) %iter, ptr noalias noundef align 1 dereferenceable(1) %_15, ptr noalias noundef align 4 dereferenceable(8) %_16) #24
  %3 = extractvalue { ptr, i64 } %2, 0
  %.not = icmp eq ptr %3, null
  br i1 %.not, label %bb6, label %bb3

bb3:                                              ; preds = %bb1
  %4 = extractvalue { ptr, i64 } %2, 1
  %5 = tail call noundef ptr %1(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %3, i64 noundef %4) #24
  %.not2 = icmp eq ptr %5, null
  br i1 %.not2, label %bb1, label %bb6

bb6:                                              ; preds = %bb1, %bb3
  %_0.sroa.0.0 = phi ptr [ %5, %bb3 ], [ null, %bb1 ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter)
  ret ptr %_0.sroa.0.0
}

; anstream::strip::write_fmt
; Function Attrs: uwtable
define noundef ptr @_RNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt(ptr noundef nonnull align 1 %raw.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(80) %raw.1, ptr noalias noundef align 4 dereferenceable(12) %state, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5 = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5)
  store ptr %raw.0, ptr %_5, align 8
  %write_all.sroa.2.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store ptr %raw.1, ptr %write_all.sroa.2.0._5.sroa_idx, align 8
  %write_all.sroa.3.0._5.sroa_idx = getelementptr inbounds nuw i8, ptr %_5, i64 16
  store ptr %state, ptr %write_all.sroa.3.0._5.sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 24
  store ptr null, ptr %0, align 8
; invoke core::fmt::write
  %_3.i = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(32) %_5, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
          to label %bb1.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb6.i, %start
  %1 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtBL_5strip9write_fmt0EEBL_(ptr noalias noundef nonnull align 8 dereferenceable(32) %_5) #26
          to label %common.resume.i unwind label %terminate.i

bb1.i:                                            ; preds = %start
  br i1 %_3.i, label %bb3.i, label %bb13.i

bb3.i:                                            ; preds = %bb1.i
  %2 = load ptr, ptr %0, align 8, !alias.scope !66, !noundef !2
  %.not.i = icmp eq ptr %2, null
  br i1 %.not.i, label %bb6.i, label %_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_.exit

bb6.i:                                            ; preds = %bb3.i
; invoke <std::io::error::Error>::new::<&str>
  %_8.i = invoke noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef 40, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_118e5dd62e18907a47aec3e2be501119, i64 noundef 15)
          to label %bb13.i unwind label %cleanup.i

bb13.i:                                           ; preds = %bb6.i, %bb1.i
  %_0.sroa.0.0.i = phi ptr [ null, %bb1.i ], [ %_8.i, %bb6.i ]
  %.val.i = load ptr, ptr %0, align 8, !alias.scope !66, !noundef !2
  %bits.i.i.i.i.i.i = ptrtoint ptr %.val.i to i64
  %_5.i.i.i.i.i.i = and i64 %bits.i.i.i.i.i.i, 3
  %switch.i.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i.i, label %bb2.i2.i.i.i.i.i, label %_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_.exit, !prof !3

bb2.i2.i.i.i.i.i:                                 ; preds = %bb13.i
  %3 = getelementptr i8, ptr %.val.i, i64 -1
  %4 = icmp ne ptr %3, null
  call void @llvm.assume(i1 %4)
  %_6.val.i.i.i.i.i.i.i = load ptr, ptr %3, align 8
  %5 = getelementptr i8, ptr %.val.i, i64 7
  %_6.val1.i.i.i.i.i.i.i = load ptr, ptr %5, align 8, !nonnull !2, !align !4, !noundef !2
  %6 = load ptr, ptr %_6.val1.i.i.i.i.i.i.i, align 8, !invariant.load !2
  %.not.i.i.i.i.i.i.i.i.i = icmp eq ptr %6, null
  br i1 %.not.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i2.i.i.i.i.i
  %7 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %7)
  invoke void %6(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i:                            ; preds = %is_not_null.i.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i.i
  %8 = icmp ne ptr %_6.val.i.i.i.i.i.i.i, null
  call void @llvm.assume(i1 %8)
  %9 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %10 = load i64, ptr %9, align 8, !range !5, !invariant.load !2
  %11 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %12 = load i64, ptr %11, align 8, !range !6, !invariant.load !2
  %13 = add i64 %12, -1
  %14 = icmp sgt i64 %13, -1
  call void @llvm.assume(i1 %14)
  %15 = icmp eq i64 %10, 0
  br i1 %15, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %10, i64 noundef range(i64 1, -9223372036854775807) %12) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i.i:                        ; preds = %is_not_null.i.i.i.i.i.i.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 8
  %18 = load i64, ptr %17, align 8, !range !5, !invariant.load !2
  %19 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i.i, i64 16
  %20 = load i64, ptr %19, align 8, !range !6, !invariant.load !2
  %21 = add i64 %20, -1
  %22 = icmp sgt i64 %21, -1
  call void @llvm.assume(i1 %22)
  %23 = icmp eq i64 %18, 0
  br i1 %23, label %bb1.i.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i.i, i64 noundef %18, i64 noundef range(i64 1, -9223372036854775807) %20) #22
  br label %bb1.i.i.i.i.i.i.i

common.resume.i:                                  ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i
  %common.resume.op.i = phi { ptr, i32 } [ %16, %bb1.i.i.i.i.i.i.i ], [ %1, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op.i

bb1.i.i.i.i.i.i.i:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %3, i64 noundef 24, i64 noundef 8) #22
  br label %common.resume.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %3, i64 noundef 24, i64 noundef 8) #22
  br label %_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_.exit

terminate.i:                                      ; preds = %cleanup.i
  %24 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #27
  unreachable

_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_.exit: ; preds = %bb3.i, %bb13.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i.i
  %_0.sroa.0.1.i = phi ptr [ %2, %bb3.i ], [ %_0.sroa.0.0.i, %bb13.i ], [ %_0.sroa.0.0.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i.i ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  ret ptr %_0.sroa.0.1.i
}

; anstream::adapter::strip::next_bytes
; Function Attrs: inlinehint uwtable
define internal fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %bytes, ptr noalias noundef nonnull align 1 captures(none) dereferenceable(1) %state, ptr noalias noundef nonnull align 4 captures(none) dereferenceable(8) %utf8parser) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %_19.0 = load ptr, ptr %bytes, align 8, !nonnull !2, !align !59, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %bytes, i64 8
  %_19.1 = load i64, ptr %0, align 8, !noundef !2
  %_28 = getelementptr inbounds nuw i8, ptr %_19.0, i64 %_19.1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !69)
  %_6.i13.i = icmp samesign eq i64 %_19.1, 0
  br i1 %_6.i13.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit, label %bb3.preheader.i

bb3.preheader.i:                                  ; preds = %start
  %.promoted.i = load i8, ptr %state, align 1, !alias.scope !69, !noalias !72
  br label %bb3.i

bb3.i:                                            ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i, %bb3.preheader.i
  %_16.i816.i = phi ptr [ %_16.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i ], [ %_19.0, %bb3.preheader.i ]
  %_13.i.i.i.i1015.i = phi i8 [ %_13.i.i.i.i9.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i ], [ %.promoted.i, %bb3.preheader.i ]
  %_7.i.i.i = phi i64 [ %_8.0.i.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i ], [ 0, %bb3.preheader.i ]
  %_16.i.i = getelementptr inbounds nuw i8, ptr %_16.i816.i, i64 1
  %.val.i = load i8, ptr %_16.i816.i, align 1, !noalias !75, !noundef !2
  %1 = icmp eq i8 %_13.i.i.i.i1015.i, 15
  br i1 %1, label %bb10, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb3.i
  %_5.i.i.i.i.i = zext i8 %.val.i to i64
  %2 = getelementptr inbounds nuw i8, ptr @anon.24dc06a1e6c291b3480b2052ac225214.0, i64 %_5.i.i.i.i.i
  %3 = load i8, ptr %2, align 1, !noalias !75, !noundef !2
  %4 = icmp eq i8 %3, 0
  br i1 %4, label %bb5.i.i.i.i.i, label %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb2.i.i.i.i
  %_8.i.i.i.i.i = zext nneg i8 %_13.i.i.i.i1015.i to i64
  %5 = getelementptr inbounds nuw [256 x i8], ptr @anon.24dc06a1e6c291b3480b2052ac225214.0, i64 %_8.i.i.i.i.i
  %6 = getelementptr inbounds nuw i8, ptr %5, i64 %_5.i.i.i.i.i
  %7 = load i8, ptr %6, align 1, !noalias !75, !noundef !2
  br label %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i

_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i: ; preds = %bb5.i.i.i.i.i, %bb2.i.i.i.i
  %change.sroa.0.0.i.i.i.i.i = phi i8 [ %7, %bb5.i.i.i.i.i ], [ %3, %bb2.i.i.i.i ]
  %_13.i.i.i.i.i = and i8 %change.sroa.0.0.i.i.i.i.i, 15
  %_15.i.i.i.i.i = lshr i8 %change.sroa.0.0.i.i.i.i.i, 4
  %_12.not.i.i.i.i = icmp eq i8 %_13.i.i.i.i.i, 0
  br i1 %_12.not.i.i.i.i, label %bb5.i.i.i.i, label %bb4.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb4.i.i.i.i, %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i
  %_13.i.i.i.i9.i = phi i8 [ %_13.i.i.i.i.i, %bb4.i.i.i.i ], [ %_13.i.i.i.i1015.i, %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i ]
  switch i8 %_15.i.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i [
    i8 12, label %bb7.i.i.i.i
    i8 15, label %bb10
    i8 5, label %bb11.i.i.i.i
  ]

bb4.i.i.i.i:                                      ; preds = %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i
  store i8 %_13.i.i.i.i.i, ptr %state, align 1, !alias.scope !69, !noalias !72
  br label %bb5.i.i.i.i

bb7.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %_14.not.i.i.i.i = icmp eq i8 %.val.i, 127
  br i1 %_14.not.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i, label %bb10

bb11.i.i.i.i:                                     ; preds = %bb5.i.i.i.i
  switch i8 %.val.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i [
    i8 9, label %bb10
    i8 10, label %bb10
    i8 12, label %bb10
    i8 13, label %bb10
    i8 32, label %bb10
  ]

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i: ; preds = %bb11.i.i.i.i, %bb7.i.i.i.i, %bb5.i.i.i.i
  %_8.0.i.i.i = add nuw i64 %_7.i.i.i, 1
  %_6.i.i = icmp eq ptr %_16.i.i, %_28
  br i1 %_6.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit.thread, label %bb3.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit.thread: ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2P_.exit.i
  %data.i.i55 = getelementptr inbounds nuw i8, ptr %_19.0, i64 %_19.1
  br label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread

bb10:                                             ; preds = %bb11.i.i.i.i, %bb11.i.i.i.i, %bb11.i.i.i.i, %bb11.i.i.i.i, %bb11.i.i.i.i, %bb7.i.i.i.i, %bb5.i.i.i.i, %bb3.i
  %_6.not.i = icmp ugt i64 %_7.i.i.i, %_19.1
  br i1 %_6.not.i, label %bb3.i6, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit, !prof !76

bb3.i6:                                           ; preds = %bb10
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_af6f16ef59dc1471d141b3b938a3b7e6) #25, !noalias !77
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit: ; preds = %start, %bb10
  %_10.sroa.0.041 = phi i64 [ %_7.i.i.i, %bb10 ], [ 0, %start ]
  %data.i.i = getelementptr inbounds nuw i8, ptr %_19.0, i64 %_10.sroa.0.041
  %len.i.i = sub nuw nsw i64 %_19.1, %_10.sroa.0.041
  store ptr %data.i.i, ptr %bytes, align 8
  store i64 %len.i.i, ptr %0, align 8
  %_6.i8.i = icmp samesign eq i64 %_10.sroa.0.041, %_19.1
  br i1 %_6.i8.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread, label %bb3.i9.preheader

bb3.i9.preheader:                                 ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit
  %state.promoted = load i8, ptr %state, align 1, !noalias !81
  br label %bb3.i9

bb3.i9:                                           ; preds = %bb3.i9.preheader, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
  %8 = phi i8 [ %17, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i ], [ %state.promoted, %bb3.i9.preheader ]
  %_47.sroa.0.0 = phi i64 [ %_8.0.i.i.i19, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i ], [ 0, %bb3.i9.preheader ]
  %_16.i79.i = phi ptr [ %_16.i.i10, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i ], [ %data.i.i, %bb3.i9.preheader ]
  %_16.i.i10 = getelementptr inbounds nuw i8, ptr %_16.i79.i, i64 1
  %.val.i11 = load i8, ptr %_16.i79.i, align 1, !noalias !89, !noundef !2
  %9 = icmp eq i8 %8, 15
  br i1 %9, label %bb1.i.i.i.i, label %bb5.i.i.i.i12

bb1.i.i.i.i:                                      ; preds = %bb3.i9
; call <anstream::adapter::strip::Utf8Parser>::add
  %_3.i.i.i.i = tail call noundef zeroext i1 @_RNvMs7_NtNtCsen8ds7JR2I0_8anstream7adapter5stripNtB5_10Utf8Parser3add(ptr noalias noundef nonnull align 4 dereferenceable(8) %utf8parser, i8 noundef %.val.i11), !noalias !81
  br i1 %_3.i.i.i.i, label %bb3.i.i.i.i, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i

bb5.i.i.i.i12:                                    ; preds = %bb3.i9
  %_5.i.i.i.i.i13 = zext i8 %.val.i11 to i64
  %10 = getelementptr inbounds nuw i8, ptr @anon.24dc06a1e6c291b3480b2052ac225214.0, i64 %_5.i.i.i.i.i13
  %11 = load i8, ptr %10, align 1, !noalias !81, !noundef !2
  %12 = icmp eq i8 %11, 0
  br i1 %12, label %bb5.i.i.i.i.i24, label %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14

bb5.i.i.i.i.i24:                                  ; preds = %bb5.i.i.i.i12
  %13 = getelementptr inbounds nuw i8, ptr getelementptr inbounds nuw (i8, ptr @anon.24dc06a1e6c291b3480b2052ac225214.0, i64 3072), i64 %_5.i.i.i.i.i13
  %14 = load i8, ptr %13, align 1, !noalias !81, !noundef !2
  br label %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14

_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14: ; preds = %bb5.i.i.i.i.i24, %bb5.i.i.i.i12
  %change.sroa.0.0.i.i.i.i.i15 = phi i8 [ %14, %bb5.i.i.i.i.i24 ], [ %11, %bb5.i.i.i.i12 ]
  %_13.i.i.i.i.i16 = and i8 %change.sroa.0.0.i.i.i.i.i15, 15
  %_15.i.i.i.i.i17 = lshr i8 %change.sroa.0.0.i.i.i.i.i15, 4
  %_17.not.i.i.i.i = icmp eq i8 %_13.i.i.i.i.i16, 0
  br i1 %_17.not.i.i.i.i, label %bb11.i.i.i.i18, label %bb8.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb1.i.i.i.i
  store i8 12, ptr %state, align 1, !noalias !81
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i

bb8.i.i.i.i:                                      ; preds = %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14
  store i8 %_13.i.i.i.i.i16, ptr %state, align 1, !noalias !81
  %15 = icmp eq i8 %_13.i.i.i.i.i16, 15
  br i1 %15, label %bb9.i.i.i.i, label %bb11.i.i.i.i18

bb9.i.i.i.i:                                      ; preds = %bb8.i.i.i.i
; call <anstream::adapter::strip::Utf8Parser>::add
  %_8.i.i.i.i = tail call noundef zeroext i1 @_RNvMs7_NtNtCsen8ds7JR2I0_8anstream7adapter5stripNtB5_10Utf8Parser3add(ptr noalias noundef nonnull align 4 dereferenceable(8) %utf8parser, i8 noundef %.val.i11), !noalias !81
  br label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i

bb11.i.i.i.i18:                                   ; preds = %bb8.i.i.i.i, %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14
  %16 = phi i8 [ %_13.i.i.i.i.i16, %bb8.i.i.i.i ], [ %8, %_RNvNtCsofRcISb9et_13anstyle_parse5state12state_change.exit.i.i.i.i14 ]
  switch i8 %_15.i.i.i.i.i17, label %bb14 [
    i8 12, label %bb14.i.i.i.i
    i8 15, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
    i8 5, label %bb18.i.i.i.i
  ]

bb14.i.i.i.i:                                     ; preds = %bb11.i.i.i.i18
  %_20.not.i.i.i.i = icmp eq i8 %.val.i11, 127
  br i1 %_20.not.i.i.i.i, label %bb14, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i

bb18.i.i.i.i:                                     ; preds = %bb11.i.i.i.i18
  switch i8 %.val.i11, label %bb14 [
    i8 9, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
    i8 10, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
    i8 12, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
    i8 13, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
    i8 32, label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i
  ]

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i: ; preds = %bb18.i.i.i.i, %bb18.i.i.i.i, %bb18.i.i.i.i, %bb18.i.i.i.i, %bb18.i.i.i.i, %bb14.i.i.i.i, %bb11.i.i.i.i18, %bb9.i.i.i.i, %bb3.i.i.i.i, %bb1.i.i.i.i
  %17 = phi i8 [ %16, %bb18.i.i.i.i ], [ %16, %bb18.i.i.i.i ], [ %16, %bb18.i.i.i.i ], [ %16, %bb18.i.i.i.i ], [ %16, %bb18.i.i.i.i ], [ %16, %bb14.i.i.i.i ], [ %16, %bb11.i.i.i.i18 ], [ 15, %bb9.i.i.i.i ], [ 12, %bb3.i.i.i.i ], [ 15, %bb1.i.i.i.i ]
  %_8.0.i.i.i19 = add nuw i64 %_47.sroa.0.0, 1
  %_6.i.i20 = icmp eq ptr %_16.i.i10, %_28
  br i1 %_6.i.i20, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32, label %bb3.i9

bb14:                                             ; preds = %bb11.i.i.i.i18, %bb14.i.i.i.i, %bb18.i.i.i.i
  %_6.not.i25 = icmp ugt i64 %_47.sroa.0.0, %len.i.i
  br i1 %_6.not.i25, label %bb3.i31, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32, !prof !90

bb3.i31:                                          ; preds = %bb14
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_d1084648e479974e70c9329824bf76f9, ptr noundef nonnull inttoptr (i64 19 to ptr), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_3d3bc0ab4df5551832e4c9b01825d63f) #25, !noalias !91
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread: ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit.thread
  %data.i.i58.ph = phi ptr [ %data.i.i55, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit.thread ], [ %data.i.i, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit ]
  store ptr %data.i.i58.ph, ptr %bytes, align 8
  store i64 0, ptr %0, align 8
  br label %19

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32: ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i, %bb14
  %_18.sroa.0.049 = phi i64 [ %_47.sroa.0.0, %bb14 ], [ %len.i.i, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_.exit.i ]
  %_18.sroa.0.049.fr = freeze i64 %_18.sroa.0.049
  %data.i.i26 = getelementptr inbounds nuw i8, ptr %data.i.i, i64 %_18.sroa.0.049.fr
  %len.i.i27 = sub nuw nsw i64 %len.i.i, %_18.sroa.0.049.fr
  store ptr %data.i.i26, ptr %bytes, align 8
  store i64 %len.i.i27, ptr %0, align 8
  %18 = icmp eq i64 %_18.sroa.0.049.fr, 0
  %spec.select = select i1 %18, ptr null, ptr %data.i.i
  br label %19

19:                                               ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread
  %_18.sroa.0.04966 = phi i64 [ 0, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread ], [ %_18.sroa.0.049.fr, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32 ]
  %20 = phi ptr [ null, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32.thread ], [ %spec.select, %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream.exit32 ]
  %21 = insertvalue { ptr, i64 } poison, ptr %20, 0
  %22 = insertvalue { ptr, i64 } %21, i64 %_18.sroa.0.04966, 1
  ret { ptr, i64 } %22
}

; <anstream::adapter::wincon::WinconCapture as anstyle_parse::Perform>::csi_dispatch
; Function Attrs: uwtable
define void @_RNvXs1_NtNtCsen8ds7JR2I0_8anstream7adapter6winconNtB5_13WinconCaptureNtCsofRcISb9et_13anstyle_parse7Perform12csi_dispatch(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(112) %params, ptr noalias noundef nonnull readonly align 1 captures(none) %_intermediates.0, i64 noundef range(i64 0, -9223372036854775808) %_intermediates.1, i1 noundef zeroext %ignore, i8 noundef %action) unnamed_addr #0 {
start:
  %iter = alloca [16 x i8], align 8
  %0 = icmp ne i8 %action, 109
  %or.cond11.not = or i1 %ignore, %0
  br i1 %or.cond11.not, label %bb70, label %bb3

bb70:                                             ; preds = %bb69, %start
  ret void

bb3:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %style.sroa.0.0.copyload = load i32, ptr %1, align 8
  %style.sroa.0.sroa.0.0.extract.trunc1279 = trunc i32 %style.sroa.0.0.copyload to i8
  %style.sroa.0.sroa.45.0.extract.shift1367 = lshr i32 %style.sroa.0.0.copyload, 8
  %style.sroa.0.sroa.45.0.extract.trunc1368 = trunc i32 %style.sroa.0.sroa.45.0.extract.shift1367 to i8
  %style.sroa.0.sroa.48.0.extract.shift1482 = lshr i32 %style.sroa.0.0.copyload, 16
  %style.sroa.0.sroa.48.0.extract.trunc1483 = trunc i32 %style.sroa.0.sroa.48.0.extract.shift1482 to i8
  %style.sroa.0.sroa.49.0.extract.shift1598 = lshr i32 %style.sroa.0.0.copyload, 24
  %style.sroa.0.sroa.49.0.extract.trunc1599 = trunc nuw i32 %style.sroa.0.sroa.49.0.extract.shift1598 to i8
  %style.sroa.50.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 28
  %style.sroa.50.0.copyload = load i32, ptr %style.sroa.50.0..sroa_idx, align 4
  %style.sroa.50.sroa.0.0.extract.trunc = trunc i32 %style.sroa.50.0.copyload to i8
  %style.sroa.50.sroa.45.0.extract.shift = lshr i32 %style.sroa.50.0.copyload, 8
  %style.sroa.50.sroa.45.0.extract.trunc = trunc i32 %style.sroa.50.sroa.45.0.extract.shift to i8
  %style.sroa.50.sroa.48.0.extract.shift = lshr i32 %style.sroa.50.0.copyload, 16
  %style.sroa.50.sroa.48.0.extract.trunc = trunc i32 %style.sroa.50.sroa.48.0.extract.shift to i8
  %style.sroa.50.sroa.49.0.extract.shift = lshr i32 %style.sroa.50.0.copyload, 24
  %style.sroa.50.sroa.49.0.extract.trunc = trunc nuw i32 %style.sroa.50.sroa.49.0.extract.shift to i8
  %style.sroa.72.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 32
  %style.sroa.72.0.copyload = load i32, ptr %style.sroa.72.0..sroa_idx, align 8
  %style.sroa.72.sroa.0.0.extract.trunc = trunc i32 %style.sroa.72.0.copyload to i8
  %style.sroa.72.sroa.48.0.extract.shift = lshr i32 %style.sroa.72.0.copyload, 8
  %style.sroa.72.sroa.48.0.extract.trunc = trunc i32 %style.sroa.72.sroa.48.0.extract.shift to i8
  %style.sroa.72.sroa.49.0.extract.shift = lshr i32 %style.sroa.72.0.copyload, 16
  %style.sroa.72.sroa.49.0.extract.trunc = trunc i32 %style.sroa.72.sroa.49.0.extract.shift to i8
  %style.sroa.72.sroa.50.0.extract.shift = lshr i32 %style.sroa.72.0.copyload, 24
  %style.sroa.72.sroa.50.0.extract.trunc = trunc nuw i32 %style.sroa.72.sroa.50.0.extract.shift to i8
  %style.sroa.89.0..sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 36
  %style.sroa.89.0.copyload = load i16, ptr %style.sroa.89.0..sroa_idx, align 4
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %iter)
  store ptr %params, ptr %iter, align 8
  %2 = getelementptr inbounds nuw i8, ptr %iter, i64 8
  store i64 0, ptr %2, align 8
; call <anstyle_parse::params::ParamsIter as core::iter::traits::iterator::Iterator>::next
  %3 = call { ptr, i64 } @_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull align 8 dereferenceable(16) %iter)
  %4 = extractvalue { ptr, i64 } %3, 0
  %.not1922 = icmp eq ptr %4, null
  br i1 %.not1922, label %bb8, label %bb7

bb7:                                              ; preds = %bb3, %bb64
  %5 = phi ptr [ %30, %bb64 ], [ %4, %bb3 ]
  %6 = phi { ptr, i64 } [ %29, %bb64 ], [ %3, %bb3 ]
  %state.sroa.0.01941 = phi i8 [ %state.sroa.0.11734, %bb64 ], [ 0, %bb3 ]
  %color_target.sroa.0.01940 = phi i8 [ %color_target.sroa.0.11748, %bb64 ], [ 0, %bb3 ]
  %g.sroa.4.0.off01939 = phi i8 [ %g.sroa.4.1.off01762, %bb64 ], [ undef, %bb3 ]
  %g.sroa.0.0.off01938 = phi i1 [ %g.sroa.0.1.off01776, %bb64 ], [ false, %bb3 ]
  %r.sroa.4.0.off01937 = phi i8 [ %r.sroa.4.1.off01790, %bb64 ], [ undef, %bb3 ]
  %r.sroa.0.0.off01936 = phi i1 [ %r.sroa.0.1.off01804, %bb64 ], [ false, %bb3 ]
  %style.sroa.89.01935 = phi i16 [ %style.sroa.89.2, %bb64 ], [ %style.sroa.89.0.copyload, %bb3 ]
  %style.sroa.72.sroa.0.01934 = phi i8 [ %style.sroa.72.sroa.0.2, %bb64 ], [ %style.sroa.72.sroa.0.0.extract.trunc, %bb3 ]
  %style.sroa.72.sroa.48.01933 = phi i8 [ %style.sroa.72.sroa.48.2, %bb64 ], [ %style.sroa.72.sroa.48.0.extract.trunc, %bb3 ]
  %style.sroa.72.sroa.49.01932 = phi i8 [ %style.sroa.72.sroa.49.2, %bb64 ], [ %style.sroa.72.sroa.49.0.extract.trunc, %bb3 ]
  %style.sroa.72.sroa.50.01931 = phi i8 [ %style.sroa.72.sroa.50.2, %bb64 ], [ %style.sroa.72.sroa.50.0.extract.trunc, %bb3 ]
  %style.sroa.50.sroa.0.01930 = phi i8 [ %style.sroa.50.sroa.0.2, %bb64 ], [ %style.sroa.50.sroa.0.0.extract.trunc, %bb3 ]
  %style.sroa.50.sroa.45.01929 = phi i8 [ %style.sroa.50.sroa.45.2, %bb64 ], [ %style.sroa.50.sroa.45.0.extract.trunc, %bb3 ]
  %style.sroa.50.sroa.48.01928 = phi i8 [ %style.sroa.50.sroa.48.2, %bb64 ], [ %style.sroa.50.sroa.48.0.extract.trunc, %bb3 ]
  %style.sroa.50.sroa.49.01927 = phi i8 [ %style.sroa.50.sroa.49.2, %bb64 ], [ %style.sroa.50.sroa.49.0.extract.trunc, %bb3 ]
  %style.sroa.0.sroa.0.01926 = phi i8 [ %style.sroa.0.sroa.0.2, %bb64 ], [ %style.sroa.0.sroa.0.0.extract.trunc1279, %bb3 ]
  %style.sroa.0.sroa.45.01925 = phi i8 [ %style.sroa.0.sroa.45.2, %bb64 ], [ %style.sroa.0.sroa.45.0.extract.trunc1368, %bb3 ]
  %style.sroa.0.sroa.48.01924 = phi i8 [ %style.sroa.0.sroa.48.2, %bb64 ], [ %style.sroa.0.sroa.48.0.extract.trunc1483, %bb3 ]
  %style.sroa.0.sroa.49.01923 = phi i8 [ %style.sroa.0.sroa.49.2, %bb64 ], [ %style.sroa.0.sroa.49.0.extract.trunc1599, %bb3 ]
  %7 = extractvalue { ptr, i64 } %6, 1
  %_111.idx = shl nuw nsw i64 %7, 1
  %_111 = getelementptr inbounds nuw i8, ptr %5, i64 %_111.idx
  %_1181874 = icmp eq i64 %7, 0
  br i1 %_1181874, label %bb64, label %bb73

bb8:                                              ; preds = %bb64, %bb3
  %style.sroa.0.sroa.49.0.lcssa = phi i8 [ %style.sroa.0.sroa.49.0.extract.trunc1599, %bb3 ], [ %style.sroa.0.sroa.49.2, %bb64 ]
  %style.sroa.0.sroa.48.0.lcssa = phi i8 [ %style.sroa.0.sroa.48.0.extract.trunc1483, %bb3 ], [ %style.sroa.0.sroa.48.2, %bb64 ]
  %style.sroa.0.sroa.45.0.lcssa = phi i8 [ %style.sroa.0.sroa.45.0.extract.trunc1368, %bb3 ], [ %style.sroa.0.sroa.45.2, %bb64 ]
  %style.sroa.0.sroa.0.0.lcssa = phi i8 [ %style.sroa.0.sroa.0.0.extract.trunc1279, %bb3 ], [ %style.sroa.0.sroa.0.2, %bb64 ]
  %style.sroa.50.sroa.49.0.lcssa = phi i8 [ %style.sroa.50.sroa.49.0.extract.trunc, %bb3 ], [ %style.sroa.50.sroa.49.2, %bb64 ]
  %style.sroa.50.sroa.48.0.lcssa = phi i8 [ %style.sroa.50.sroa.48.0.extract.trunc, %bb3 ], [ %style.sroa.50.sroa.48.2, %bb64 ]
  %style.sroa.50.sroa.45.0.lcssa = phi i8 [ %style.sroa.50.sroa.45.0.extract.trunc, %bb3 ], [ %style.sroa.50.sroa.45.2, %bb64 ]
  %style.sroa.50.sroa.0.0.lcssa = phi i8 [ %style.sroa.50.sroa.0.0.extract.trunc, %bb3 ], [ %style.sroa.50.sroa.0.2, %bb64 ]
  %style.sroa.72.sroa.50.0.lcssa = phi i8 [ %style.sroa.72.sroa.50.0.extract.trunc, %bb3 ], [ %style.sroa.72.sroa.50.2, %bb64 ]
  %style.sroa.72.sroa.49.0.lcssa = phi i8 [ %style.sroa.72.sroa.49.0.extract.trunc, %bb3 ], [ %style.sroa.72.sroa.49.2, %bb64 ]
  %style.sroa.72.sroa.48.0.lcssa = phi i8 [ %style.sroa.72.sroa.48.0.extract.trunc, %bb3 ], [ %style.sroa.72.sroa.48.2, %bb64 ]
  %style.sroa.72.sroa.0.0.lcssa = phi i8 [ %style.sroa.72.sroa.0.0.extract.trunc, %bb3 ], [ %style.sroa.72.sroa.0.2, %bb64 ]
  %style.sroa.89.0.lcssa = phi i16 [ %style.sroa.89.0.copyload, %bb3 ], [ %style.sroa.89.2, %bb64 ]
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %iter)
  %.not.i = icmp eq i8 %style.sroa.0.sroa.0.0.lcssa, 3
  br i1 %.not.i, label %bb6.i, label %bb8.i

bb8.i:                                            ; preds = %bb8
  %_5.i.i = icmp eq i8 %style.sroa.0.sroa.0.0.lcssa, %style.sroa.0.sroa.0.0.extract.trunc1279
  br i1 %_5.i.i, label %bb1.i.i, label %bb65

bb6.i:                                            ; preds = %bb8
  %.not1955 = icmp eq i8 %style.sroa.0.sroa.0.0.extract.trunc1279, 3
  br i1 %.not1955, label %bb1.i, label %bb65

bb1.i.i:                                          ; preds = %bb8.i
  %8 = icmp eq i8 %style.sroa.0.sroa.45.0.lcssa, %style.sroa.0.sroa.45.0.extract.trunc1368
  %switch.i.i = icmp samesign ult i8 %style.sroa.0.sroa.0.0.extract.trunc1279, 2
  %.not.i.i = xor i1 %8, true
  %brmerge.i.i = or i1 %switch.i.i, %.not.i.i
  br i1 %brmerge.i.i, label %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb1.i.i
  %_15.i.i = icmp eq i8 %style.sroa.0.sroa.48.0.lcssa, %style.sroa.0.sroa.48.0.extract.trunc1483
  %9 = icmp eq i8 %style.sroa.0.sroa.49.0.lcssa, %style.sroa.0.sroa.49.0.extract.trunc1599
  %or.cond1706 = select i1 %_15.i.i, i1 %9, i1 false
  br i1 %or.cond1706, label %bb1.i, label %bb65

_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit.i: ; preds = %bb1.i.i
  %.mux.i.i = and i1 %switch.i.i, %8
  br i1 %.mux.i.i, label %bb1.i, label %bb65

bb1.i:                                            ; preds = %bb7.i.i, %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit.i, %bb6.i
  %.not2.i = icmp eq i8 %style.sroa.50.sroa.0.0.lcssa, 3
  %10 = icmp ne i8 %style.sroa.50.sroa.0.0.extract.trunc, 3
  br i1 %.not2.i, label %bb12.i, label %bb13.i

bb13.i:                                           ; preds = %bb1.i
  %_5.i6.i = icmp eq i8 %style.sroa.50.sroa.0.0.lcssa, %style.sroa.50.sroa.0.0.extract.trunc
  %or.cond1717 = select i1 %10, i1 %_5.i6.i, i1 false
  br i1 %or.cond1717, label %bb1.i8.i, label %bb65

bb12.i:                                           ; preds = %bb1.i
  br i1 %10, label %bb65, label %bb2.i

bb1.i8.i:                                         ; preds = %bb13.i
  %11 = icmp eq i8 %style.sroa.50.sroa.45.0.lcssa, %style.sroa.50.sroa.45.0.extract.trunc
  %switch.i11.i = icmp samesign ult i8 %style.sroa.50.sroa.0.0.extract.trunc, 2
  %.not.i12.i = xor i1 %11, true
  %brmerge.i13.i = or i1 %switch.i11.i, %.not.i12.i
  br i1 %brmerge.i13.i, label %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit22.i, label %bb7.i15.i

bb7.i15.i:                                        ; preds = %bb1.i8.i
  %_15.i18.i = icmp eq i8 %style.sroa.50.sroa.48.0.lcssa, %style.sroa.50.sroa.48.0.extract.trunc
  %12 = icmp eq i8 %style.sroa.50.sroa.49.0.lcssa, %style.sroa.50.sroa.49.0.extract.trunc
  %or.cond1708 = select i1 %_15.i18.i, i1 %12, i1 false
  br i1 %or.cond1708, label %bb2.i, label %bb65

_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit22.i: ; preds = %bb1.i8.i
  %.mux.i14.i = and i1 %switch.i11.i, %11
  br i1 %.mux.i14.i, label %bb2.i, label %bb65

bb2.i:                                            ; preds = %bb7.i15.i, %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit22.i, %bb12.i
  %.not4.i = icmp eq i8 %style.sroa.72.sroa.0.0.lcssa, 3
  %13 = icmp eq i8 %style.sroa.72.sroa.0.0.extract.trunc, 3
  br i1 %.not4.i, label %bb17.i, label %bb18.i

bb18.i:                                           ; preds = %bb2.i
  %_5.i23.i = icmp ne i8 %style.sroa.72.sroa.0.0.lcssa, %style.sroa.72.sroa.0.0.extract.trunc
  %or.cond1719.not = select i1 %13, i1 true, i1 %_5.i23.i
  br i1 %or.cond1719.not, label %bb65, label %bb1.i25.i

bb17.i:                                           ; preds = %bb2.i
  %.old = icmp eq i16 %style.sroa.89.0.lcssa, %style.sroa.89.0.copyload
  %or.cond1712 = select i1 %13, i1 %.old, i1 false
  br i1 %or.cond1712, label %bb69, label %bb65

bb1.i25.i:                                        ; preds = %bb18.i
  %14 = icmp eq i8 %style.sroa.72.sroa.48.0.lcssa, %style.sroa.72.sroa.48.0.extract.trunc
  %switch.i28.i = icmp samesign ult i8 %style.sroa.72.sroa.0.0.extract.trunc, 2
  %.not.i29.i = xor i1 %14, true
  %brmerge.i30.i = or i1 %switch.i28.i, %.not.i29.i
  br i1 %brmerge.i30.i, label %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit39.i, label %bb7.i32.i

bb7.i32.i:                                        ; preds = %bb1.i25.i
  %_15.i35.i = icmp eq i8 %style.sroa.72.sroa.49.0.lcssa, %style.sroa.72.sroa.49.0.extract.trunc
  %15 = icmp eq i8 %style.sroa.72.sroa.50.0.lcssa, %style.sroa.72.sroa.50.0.extract.trunc
  %or.cond1710 = select i1 %_15.i35.i, i1 %15, i1 false
  %16 = icmp eq i16 %style.sroa.89.0.lcssa, %style.sroa.89.0.copyload
  %or.cond1711 = select i1 %or.cond1710, i1 %16, i1 false
  br i1 %or.cond1711, label %bb69, label %bb65

_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit39.i: ; preds = %bb1.i25.i
  %.mux.i31.i = and i1 %switch.i28.i, %14
  %.old.old = icmp eq i16 %style.sroa.89.0.lcssa, %style.sroa.89.0.copyload
  %or.cond1713 = select i1 %.mux.i31.i, i1 %.old.old, i1 false
  br i1 %or.cond1713, label %bb69, label %bb65

bb65:                                             ; preds = %bb7.i32.i, %bb7.i15.i, %bb7.i.i, %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit39.i, %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit22.i, %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit.i, %bb6.i, %bb12.i, %bb17.i, %bb8.i, %bb13.i, %bb18.i
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_150 = load i64, ptr %17, align 8, !noundef !2
  %_151 = icmp sgt i64 %_150, -1
  call void @llvm.assume(i1 %_151)
  %18 = icmp eq i64 %_150, 0
  br i1 %18, label %bb69, label %bb67

bb69:                                             ; preds = %_RNvXsi_NtCsds7IErrfYrg_7anstyle5colorNtB5_5ColorNtNtCsjMrxcFdYDNN_4core3cmp9PartialEq2eq.exit39.i, %bb17.i, %bb7.i32.i, %bb65, %bb67
  %style.sroa.0.sroa.49.0.insert.ext1600 = zext i8 %style.sroa.0.sroa.49.0.lcssa to i32
  %style.sroa.0.sroa.49.0.insert.shift1601 = shl nuw i32 %style.sroa.0.sroa.49.0.insert.ext1600, 24
  %style.sroa.0.sroa.48.0.insert.ext1484 = zext i8 %style.sroa.0.sroa.48.0.lcssa to i32
  %style.sroa.0.sroa.48.0.insert.shift1485 = shl nuw nsw i32 %style.sroa.0.sroa.48.0.insert.ext1484, 16
  %style.sroa.0.sroa.48.0.insert.insert1487 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.shift1485, %style.sroa.0.sroa.49.0.insert.shift1601
  %style.sroa.0.sroa.45.0.insert.ext1369 = zext i8 %style.sroa.0.sroa.45.0.lcssa to i32
  %style.sroa.0.sroa.45.0.insert.shift1370 = shl nuw nsw i32 %style.sroa.0.sroa.45.0.insert.ext1369, 8
  %style.sroa.0.sroa.45.0.insert.insert1372 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.insert1487, %style.sroa.0.sroa.45.0.insert.shift1370
  %style.sroa.0.sroa.0.0.insert.ext1280 = zext i8 %style.sroa.0.sroa.0.0.lcssa to i32
  %style.sroa.0.sroa.0.0.insert.insert1282 = or disjoint i32 %style.sroa.0.sroa.45.0.insert.insert1372, %style.sroa.0.sroa.0.0.insert.ext1280
  %style.sroa.50.sroa.49.0.insert.ext = zext i8 %style.sroa.50.sroa.49.0.lcssa to i32
  %style.sroa.50.sroa.49.0.insert.shift = shl nuw i32 %style.sroa.50.sroa.49.0.insert.ext, 24
  %style.sroa.50.sroa.48.0.insert.ext = zext i8 %style.sroa.50.sroa.48.0.lcssa to i32
  %style.sroa.50.sroa.48.0.insert.shift = shl nuw nsw i32 %style.sroa.50.sroa.48.0.insert.ext, 16
  %style.sroa.50.sroa.48.0.insert.insert = or disjoint i32 %style.sroa.50.sroa.48.0.insert.shift, %style.sroa.50.sroa.49.0.insert.shift
  %style.sroa.50.sroa.45.0.insert.ext = zext i8 %style.sroa.50.sroa.45.0.lcssa to i32
  %style.sroa.50.sroa.45.0.insert.shift = shl nuw nsw i32 %style.sroa.50.sroa.45.0.insert.ext, 8
  %style.sroa.50.sroa.45.0.insert.insert = or disjoint i32 %style.sroa.50.sroa.48.0.insert.insert, %style.sroa.50.sroa.45.0.insert.shift
  %style.sroa.50.sroa.0.0.insert.ext = zext i8 %style.sroa.50.sroa.0.0.lcssa to i32
  %style.sroa.50.sroa.0.0.insert.insert = or disjoint i32 %style.sroa.50.sroa.45.0.insert.insert, %style.sroa.50.sroa.0.0.insert.ext
  %style.sroa.72.sroa.50.0.insert.ext = zext i8 %style.sroa.72.sroa.50.0.lcssa to i32
  %style.sroa.72.sroa.50.0.insert.shift = shl nuw i32 %style.sroa.72.sroa.50.0.insert.ext, 24
  %style.sroa.72.sroa.49.0.insert.ext = zext i8 %style.sroa.72.sroa.49.0.lcssa to i32
  %style.sroa.72.sroa.49.0.insert.shift = shl nuw nsw i32 %style.sroa.72.sroa.49.0.insert.ext, 16
  %style.sroa.72.sroa.49.0.insert.insert = or disjoint i32 %style.sroa.72.sroa.49.0.insert.shift, %style.sroa.72.sroa.50.0.insert.shift
  %style.sroa.72.sroa.48.0.insert.ext = zext i8 %style.sroa.72.sroa.48.0.lcssa to i32
  %style.sroa.72.sroa.48.0.insert.shift = shl nuw nsw i32 %style.sroa.72.sroa.48.0.insert.ext, 8
  %style.sroa.72.sroa.48.0.insert.insert = or disjoint i32 %style.sroa.72.sroa.49.0.insert.insert, %style.sroa.72.sroa.48.0.insert.shift
  %style.sroa.72.sroa.0.0.insert.ext = zext i8 %style.sroa.72.sroa.0.0.lcssa to i32
  %style.sroa.72.sroa.0.0.insert.insert = or disjoint i32 %style.sroa.72.sroa.48.0.insert.insert, %style.sroa.72.sroa.0.0.insert.ext
  store i32 %style.sroa.0.sroa.0.0.insert.insert1282, ptr %1, align 8
  store i32 %style.sroa.50.sroa.0.0.insert.insert, ptr %style.sroa.50.0..sroa_idx, align 4
  store i32 %style.sroa.72.sroa.0.0.insert.insert, ptr %style.sroa.72.0..sroa_idx, align 8
  store i16 %style.sroa.89.0.lcssa, ptr %style.sroa.89.0..sroa_idx, align 4
  br label %bb70

bb67:                                             ; preds = %bb65
  %19 = getelementptr inbounds nuw i8, ptr %self, i64 38
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 2 dereferenceable(14) %19, ptr noundef nonnull align 8 dereferenceable(14) %1, i64 14, i1 false)
  br label %bb69

bb73:                                             ; preds = %bb7, %bb63
  %state.sroa.0.11882 = phi i8 [ %state.sroa.0.2, %bb63 ], [ %state.sroa.0.01941, %bb7 ]
  %color_target.sroa.0.11881 = phi i8 [ %color_target.sroa.0.2, %bb63 ], [ %color_target.sroa.0.01940, %bb7 ]
  %iter1.sroa.0.01880 = phi ptr [ %_124, %bb63 ], [ %5, %bb7 ]
  %g.sroa.4.1.off01879 = phi i8 [ %g.sroa.4.2.off0, %bb63 ], [ %g.sroa.4.0.off01939, %bb7 ]
  %g.sroa.0.1.off01878 = phi i1 [ %g.sroa.0.2.off0, %bb63 ], [ %g.sroa.0.0.off01938, %bb7 ]
  %r.sroa.4.1.off01877 = phi i8 [ %r.sroa.4.2.off0, %bb63 ], [ %r.sroa.4.0.off01937, %bb7 ]
  %r.sroa.0.1.off01876 = phi i1 [ %r.sroa.0.2.off0, %bb63 ], [ %r.sroa.0.0.off01936, %bb7 ]
  %style.sroa.89.11875 = phi i16 [ %style.sroa.89.3, %bb63 ], [ %style.sroa.89.01935, %bb7 ]
  %_124 = getelementptr inbounds nuw i8, ptr %iter1.sroa.0.01880, i64 2
  %n = load i16, ptr %iter1.sroa.0.01880, align 2, !noundef !2
  switch i8 %state.sroa.0.11882, label %default.unreachable2062 [
    i8 0, label %bb10
    i8 1, label %bb23
    i8 2, label %bb31
    i8 3, label %bb30
    i8 4, label %bb24
  ]

default.unreachable2062:                          ; preds = %bb55, %bb31, %bb73
  unreachable

default.unreachable:                              ; preds = %bb18, %bb16, %bb14, %bb12
  unreachable

bb10:                                             ; preds = %bb73
  switch i16 %n, label %bb11 [
    i16 0, label %bb64.loopexit
    i16 1, label %bb46
    i16 2, label %bb45
    i16 3, label %bb44
    i16 4, label %bb43
    i16 21, label %bb42
    i16 7, label %bb41
    i16 8, label %bb40
    i16 9, label %bb39
    i16 38, label %bb63
    i16 39, label %bb64.loopexit2248
    i16 48, label %bb36
    i16 49, label %bb64
    i16 58, label %bb34
  ]

bb23:                                             ; preds = %bb73
  switch i16 %n, label %bb64 [
    i16 5, label %bb63
    i16 2, label %bb32
  ]

bb31:                                             ; preds = %bb73
  switch i8 %color_target.sroa.0.11881, label %default.unreachable2062 [
    i8 0, label %bb52
    i8 1, label %bb51
    i8 2, label %bb50
  ]

bb30:                                             ; preds = %bb73
  br i1 %r.sroa.0.1.off01876, label %bb54, label %bb57

bb24:                                             ; preds = %bb73
  switch i16 %n, label %bb64 [
    i16 0, label %bb29
    i16 1, label %bb63
    i16 2, label %bb28
    i16 3, label %bb27
    i16 4, label %bb26
    i16 5, label %bb25
  ]

bb11:                                             ; preds = %bb10
  %20 = add i16 %n, -30
  %or.cond = icmp ult i16 %20, 8
  br i1 %or.cond, label %bb12, label %bb13

bb46:                                             ; preds = %bb10
  %21 = or i16 %style.sroa.89.11875, 1
  br label %bb64

bb45:                                             ; preds = %bb10
  %22 = or i16 %style.sroa.89.11875, 2
  br label %bb64

bb44:                                             ; preds = %bb10
  %23 = or i16 %style.sroa.89.11875, 4
  br label %bb64

bb43:                                             ; preds = %bb10
  %24 = or i16 %style.sroa.89.11875, 8
  br label %bb63

bb42:                                             ; preds = %bb10
  %25 = or i16 %style.sroa.89.11875, 16
  br label %bb64

bb41:                                             ; preds = %bb10
  %26 = or i16 %style.sroa.89.11875, 512
  br label %bb64

bb40:                                             ; preds = %bb10
  %27 = or i16 %style.sroa.89.11875, 1024
  br label %bb64

bb39:                                             ; preds = %bb10
  %28 = or i16 %style.sroa.89.11875, 2048
  br label %bb64

bb36:                                             ; preds = %bb10
  br label %bb63

bb34:                                             ; preds = %bb10
  br label %bb63

bb64.loopexit:                                    ; preds = %bb10
  br label %bb64

bb64.loopexit2248:                                ; preds = %bb10
  br label %bb64

bb64:                                             ; preds = %bb23, %bb24, %bb63, %bb10, %bb64.loopexit2248, %bb64.loopexit, %bb7, %bb85, %bb84, %bb83, %bb82, %bb81, %bb80, %bb79, %bb12, %bb94, %bb93, %bb92, %bb91, %bb90, %bb89, %bb88, %bb14, %bb11.i195, %bb12.i197, %bb13.i198, %bb14.i199, %bb15.i200, %bb16.i201, %bb116.thread, %bb16, %bb11.i190, %bb12.i191, %bb13.i192, %bb14.i, %bb15.i, %bb16.i193, %bb117.thread, %bb18, %bb61, %bb53, %bb17, %bb39, %bb40, %bb41, %bb42, %bb44, %bb45, %bb46
  %r.sroa.0.1.off01804 = phi i1 [ %r.sroa.0.1.off01876, %bb17 ], [ %r.sroa.0.1.off01876, %bb46 ], [ %r.sroa.0.1.off01876, %bb45 ], [ %r.sroa.0.1.off01876, %bb44 ], [ %r.sroa.0.1.off01876, %bb42 ], [ %r.sroa.0.1.off01876, %bb41 ], [ %r.sroa.0.1.off01876, %bb40 ], [ %r.sroa.0.1.off01876, %bb39 ], [ %r.sroa.0.1.off01876, %bb53 ], [ true, %bb61 ], [ %r.sroa.0.1.off01876, %bb18 ], [ %r.sroa.0.1.off01876, %bb117.thread ], [ %r.sroa.0.1.off01876, %bb16.i193 ], [ %r.sroa.0.1.off01876, %bb15.i ], [ %r.sroa.0.1.off01876, %bb14.i ], [ %r.sroa.0.1.off01876, %bb13.i192 ], [ %r.sroa.0.1.off01876, %bb12.i191 ], [ %r.sroa.0.1.off01876, %bb11.i190 ], [ %r.sroa.0.1.off01876, %bb16 ], [ %r.sroa.0.1.off01876, %bb116.thread ], [ %r.sroa.0.1.off01876, %bb16.i201 ], [ %r.sroa.0.1.off01876, %bb15.i200 ], [ %r.sroa.0.1.off01876, %bb14.i199 ], [ %r.sroa.0.1.off01876, %bb13.i198 ], [ %r.sroa.0.1.off01876, %bb12.i197 ], [ %r.sroa.0.1.off01876, %bb11.i195 ], [ %r.sroa.0.1.off01876, %bb14 ], [ %r.sroa.0.1.off01876, %bb88 ], [ %r.sroa.0.1.off01876, %bb89 ], [ %r.sroa.0.1.off01876, %bb90 ], [ %r.sroa.0.1.off01876, %bb91 ], [ %r.sroa.0.1.off01876, %bb92 ], [ %r.sroa.0.1.off01876, %bb93 ], [ %r.sroa.0.1.off01876, %bb94 ], [ %r.sroa.0.1.off01876, %bb12 ], [ %r.sroa.0.1.off01876, %bb79 ], [ %r.sroa.0.1.off01876, %bb80 ], [ %r.sroa.0.1.off01876, %bb81 ], [ %r.sroa.0.1.off01876, %bb82 ], [ %r.sroa.0.1.off01876, %bb83 ], [ %r.sroa.0.1.off01876, %bb84 ], [ %r.sroa.0.1.off01876, %bb85 ], [ %r.sroa.0.0.off01936, %bb7 ], [ %r.sroa.0.1.off01876, %bb64.loopexit ], [ %r.sroa.0.1.off01876, %bb10 ], [ %r.sroa.0.1.off01876, %bb23 ], [ %r.sroa.0.1.off01876, %bb24 ], [ %r.sroa.0.2.off0, %bb63 ], [ %r.sroa.0.1.off01876, %bb64.loopexit2248 ]
  %r.sroa.4.1.off01790 = phi i8 [ %r.sroa.4.1.off01877, %bb17 ], [ %r.sroa.4.1.off01877, %bb46 ], [ %r.sroa.4.1.off01877, %bb45 ], [ %r.sroa.4.1.off01877, %bb44 ], [ %r.sroa.4.1.off01877, %bb42 ], [ %r.sroa.4.1.off01877, %bb41 ], [ %r.sroa.4.1.off01877, %bb40 ], [ %r.sroa.4.1.off01877, %bb39 ], [ %r.sroa.4.1.off01877, %bb53 ], [ %r.sroa.4.1.off01877, %bb61 ], [ %r.sroa.4.1.off01877, %bb18 ], [ %r.sroa.4.1.off01877, %bb117.thread ], [ %r.sroa.4.1.off01877, %bb16.i193 ], [ %r.sroa.4.1.off01877, %bb15.i ], [ %r.sroa.4.1.off01877, %bb14.i ], [ %r.sroa.4.1.off01877, %bb13.i192 ], [ %r.sroa.4.1.off01877, %bb12.i191 ], [ %r.sroa.4.1.off01877, %bb11.i190 ], [ %r.sroa.4.1.off01877, %bb16 ], [ %r.sroa.4.1.off01877, %bb116.thread ], [ %r.sroa.4.1.off01877, %bb16.i201 ], [ %r.sroa.4.1.off01877, %bb15.i200 ], [ %r.sroa.4.1.off01877, %bb14.i199 ], [ %r.sroa.4.1.off01877, %bb13.i198 ], [ %r.sroa.4.1.off01877, %bb12.i197 ], [ %r.sroa.4.1.off01877, %bb11.i195 ], [ %r.sroa.4.1.off01877, %bb14 ], [ %r.sroa.4.1.off01877, %bb88 ], [ %r.sroa.4.1.off01877, %bb89 ], [ %r.sroa.4.1.off01877, %bb90 ], [ %r.sroa.4.1.off01877, %bb91 ], [ %r.sroa.4.1.off01877, %bb92 ], [ %r.sroa.4.1.off01877, %bb93 ], [ %r.sroa.4.1.off01877, %bb94 ], [ %r.sroa.4.1.off01877, %bb12 ], [ %r.sroa.4.1.off01877, %bb79 ], [ %r.sroa.4.1.off01877, %bb80 ], [ %r.sroa.4.1.off01877, %bb81 ], [ %r.sroa.4.1.off01877, %bb82 ], [ %r.sroa.4.1.off01877, %bb83 ], [ %r.sroa.4.1.off01877, %bb84 ], [ %r.sroa.4.1.off01877, %bb85 ], [ %r.sroa.4.0.off01937, %bb7 ], [ %r.sroa.4.1.off01877, %bb64.loopexit ], [ %r.sroa.4.1.off01877, %bb10 ], [ %r.sroa.4.1.off01877, %bb23 ], [ %r.sroa.4.1.off01877, %bb24 ], [ %r.sroa.4.2.off0, %bb63 ], [ %r.sroa.4.1.off01877, %bb64.loopexit2248 ]
  %g.sroa.0.1.off01776 = phi i1 [ %g.sroa.0.1.off01878, %bb17 ], [ %g.sroa.0.1.off01878, %bb46 ], [ %g.sroa.0.1.off01878, %bb45 ], [ %g.sroa.0.1.off01878, %bb44 ], [ %g.sroa.0.1.off01878, %bb42 ], [ %g.sroa.0.1.off01878, %bb41 ], [ %g.sroa.0.1.off01878, %bb40 ], [ %g.sroa.0.1.off01878, %bb39 ], [ %g.sroa.0.1.off01878, %bb53 ], [ true, %bb61 ], [ %g.sroa.0.1.off01878, %bb18 ], [ %g.sroa.0.1.off01878, %bb117.thread ], [ %g.sroa.0.1.off01878, %bb16.i193 ], [ %g.sroa.0.1.off01878, %bb15.i ], [ %g.sroa.0.1.off01878, %bb14.i ], [ %g.sroa.0.1.off01878, %bb13.i192 ], [ %g.sroa.0.1.off01878, %bb12.i191 ], [ %g.sroa.0.1.off01878, %bb11.i190 ], [ %g.sroa.0.1.off01878, %bb16 ], [ %g.sroa.0.1.off01878, %bb116.thread ], [ %g.sroa.0.1.off01878, %bb16.i201 ], [ %g.sroa.0.1.off01878, %bb15.i200 ], [ %g.sroa.0.1.off01878, %bb14.i199 ], [ %g.sroa.0.1.off01878, %bb13.i198 ], [ %g.sroa.0.1.off01878, %bb12.i197 ], [ %g.sroa.0.1.off01878, %bb11.i195 ], [ %g.sroa.0.1.off01878, %bb14 ], [ %g.sroa.0.1.off01878, %bb88 ], [ %g.sroa.0.1.off01878, %bb89 ], [ %g.sroa.0.1.off01878, %bb90 ], [ %g.sroa.0.1.off01878, %bb91 ], [ %g.sroa.0.1.off01878, %bb92 ], [ %g.sroa.0.1.off01878, %bb93 ], [ %g.sroa.0.1.off01878, %bb94 ], [ %g.sroa.0.1.off01878, %bb12 ], [ %g.sroa.0.1.off01878, %bb79 ], [ %g.sroa.0.1.off01878, %bb80 ], [ %g.sroa.0.1.off01878, %bb81 ], [ %g.sroa.0.1.off01878, %bb82 ], [ %g.sroa.0.1.off01878, %bb83 ], [ %g.sroa.0.1.off01878, %bb84 ], [ %g.sroa.0.1.off01878, %bb85 ], [ %g.sroa.0.0.off01938, %bb7 ], [ %g.sroa.0.1.off01878, %bb64.loopexit ], [ %g.sroa.0.1.off01878, %bb10 ], [ %g.sroa.0.1.off01878, %bb23 ], [ %g.sroa.0.1.off01878, %bb24 ], [ %g.sroa.0.2.off0, %bb63 ], [ %g.sroa.0.1.off01878, %bb64.loopexit2248 ]
  %g.sroa.4.1.off01762 = phi i8 [ %g.sroa.4.1.off01879, %bb17 ], [ %g.sroa.4.1.off01879, %bb46 ], [ %g.sroa.4.1.off01879, %bb45 ], [ %g.sroa.4.1.off01879, %bb44 ], [ %g.sroa.4.1.off01879, %bb42 ], [ %g.sroa.4.1.off01879, %bb41 ], [ %g.sroa.4.1.off01879, %bb40 ], [ %g.sroa.4.1.off01879, %bb39 ], [ %g.sroa.4.1.off01879, %bb53 ], [ %g.sroa.4.1.off01879, %bb61 ], [ %g.sroa.4.1.off01879, %bb18 ], [ %g.sroa.4.1.off01879, %bb117.thread ], [ %g.sroa.4.1.off01879, %bb16.i193 ], [ %g.sroa.4.1.off01879, %bb15.i ], [ %g.sroa.4.1.off01879, %bb14.i ], [ %g.sroa.4.1.off01879, %bb13.i192 ], [ %g.sroa.4.1.off01879, %bb12.i191 ], [ %g.sroa.4.1.off01879, %bb11.i190 ], [ %g.sroa.4.1.off01879, %bb16 ], [ %g.sroa.4.1.off01879, %bb116.thread ], [ %g.sroa.4.1.off01879, %bb16.i201 ], [ %g.sroa.4.1.off01879, %bb15.i200 ], [ %g.sroa.4.1.off01879, %bb14.i199 ], [ %g.sroa.4.1.off01879, %bb13.i198 ], [ %g.sroa.4.1.off01879, %bb12.i197 ], [ %g.sroa.4.1.off01879, %bb11.i195 ], [ %g.sroa.4.1.off01879, %bb14 ], [ %g.sroa.4.1.off01879, %bb88 ], [ %g.sroa.4.1.off01879, %bb89 ], [ %g.sroa.4.1.off01879, %bb90 ], [ %g.sroa.4.1.off01879, %bb91 ], [ %g.sroa.4.1.off01879, %bb92 ], [ %g.sroa.4.1.off01879, %bb93 ], [ %g.sroa.4.1.off01879, %bb94 ], [ %g.sroa.4.1.off01879, %bb12 ], [ %g.sroa.4.1.off01879, %bb79 ], [ %g.sroa.4.1.off01879, %bb80 ], [ %g.sroa.4.1.off01879, %bb81 ], [ %g.sroa.4.1.off01879, %bb82 ], [ %g.sroa.4.1.off01879, %bb83 ], [ %g.sroa.4.1.off01879, %bb84 ], [ %g.sroa.4.1.off01879, %bb85 ], [ %g.sroa.4.0.off01939, %bb7 ], [ %g.sroa.4.1.off01879, %bb64.loopexit ], [ %g.sroa.4.1.off01879, %bb10 ], [ %g.sroa.4.1.off01879, %bb23 ], [ %g.sroa.4.1.off01879, %bb24 ], [ %g.sroa.4.2.off0, %bb63 ], [ %g.sroa.4.1.off01879, %bb64.loopexit2248 ]
  %color_target.sroa.0.11748 = phi i8 [ %color_target.sroa.0.11881, %bb17 ], [ %color_target.sroa.0.11881, %bb46 ], [ %color_target.sroa.0.11881, %bb45 ], [ %color_target.sroa.0.11881, %bb44 ], [ %color_target.sroa.0.11881, %bb42 ], [ %color_target.sroa.0.11881, %bb41 ], [ %color_target.sroa.0.11881, %bb40 ], [ %color_target.sroa.0.11881, %bb39 ], [ %color_target.sroa.0.11881, %bb53 ], [ %color_target.sroa.0.11881, %bb61 ], [ %color_target.sroa.0.11881, %bb18 ], [ %color_target.sroa.0.11881, %bb117.thread ], [ %color_target.sroa.0.11881, %bb16.i193 ], [ %color_target.sroa.0.11881, %bb15.i ], [ %color_target.sroa.0.11881, %bb14.i ], [ %color_target.sroa.0.11881, %bb13.i192 ], [ %color_target.sroa.0.11881, %bb12.i191 ], [ %color_target.sroa.0.11881, %bb11.i190 ], [ %color_target.sroa.0.11881, %bb16 ], [ %color_target.sroa.0.11881, %bb116.thread ], [ %color_target.sroa.0.11881, %bb16.i201 ], [ %color_target.sroa.0.11881, %bb15.i200 ], [ %color_target.sroa.0.11881, %bb14.i199 ], [ %color_target.sroa.0.11881, %bb13.i198 ], [ %color_target.sroa.0.11881, %bb12.i197 ], [ %color_target.sroa.0.11881, %bb11.i195 ], [ %color_target.sroa.0.11881, %bb14 ], [ %color_target.sroa.0.11881, %bb88 ], [ %color_target.sroa.0.11881, %bb89 ], [ %color_target.sroa.0.11881, %bb90 ], [ %color_target.sroa.0.11881, %bb91 ], [ %color_target.sroa.0.11881, %bb92 ], [ %color_target.sroa.0.11881, %bb93 ], [ %color_target.sroa.0.11881, %bb94 ], [ %color_target.sroa.0.11881, %bb12 ], [ %color_target.sroa.0.11881, %bb79 ], [ %color_target.sroa.0.11881, %bb80 ], [ %color_target.sroa.0.11881, %bb81 ], [ %color_target.sroa.0.11881, %bb82 ], [ %color_target.sroa.0.11881, %bb83 ], [ %color_target.sroa.0.11881, %bb84 ], [ %color_target.sroa.0.11881, %bb85 ], [ %color_target.sroa.0.01940, %bb7 ], [ %color_target.sroa.0.11881, %bb64.loopexit ], [ %color_target.sroa.0.11881, %bb10 ], [ %color_target.sroa.0.11881, %bb23 ], [ %color_target.sroa.0.11881, %bb24 ], [ %color_target.sroa.0.2, %bb63 ], [ %color_target.sroa.0.11881, %bb64.loopexit2248 ]
  %state.sroa.0.11734 = phi i8 [ 0, %bb17 ], [ 0, %bb46 ], [ 0, %bb45 ], [ 0, %bb44 ], [ 0, %bb42 ], [ 0, %bb41 ], [ 0, %bb40 ], [ 0, %bb39 ], [ 2, %bb53 ], [ 3, %bb61 ], [ 0, %bb18 ], [ 0, %bb117.thread ], [ 0, %bb16.i193 ], [ 0, %bb15.i ], [ 0, %bb14.i ], [ 0, %bb13.i192 ], [ 0, %bb12.i191 ], [ 0, %bb11.i190 ], [ 0, %bb16 ], [ 0, %bb116.thread ], [ 0, %bb16.i201 ], [ 0, %bb15.i200 ], [ 0, %bb14.i199 ], [ 0, %bb13.i198 ], [ 0, %bb12.i197 ], [ 0, %bb11.i195 ], [ 0, %bb14 ], [ 0, %bb88 ], [ 0, %bb89 ], [ 0, %bb90 ], [ 0, %bb91 ], [ 0, %bb92 ], [ 0, %bb93 ], [ 0, %bb94 ], [ 0, %bb12 ], [ 0, %bb79 ], [ 0, %bb80 ], [ 0, %bb81 ], [ 0, %bb82 ], [ 0, %bb83 ], [ 0, %bb84 ], [ 0, %bb85 ], [ %state.sroa.0.01941, %bb7 ], [ 0, %bb64.loopexit ], [ 0, %bb10 ], [ 1, %bb23 ], [ 4, %bb24 ], [ %state.sroa.0.2, %bb63 ], [ 0, %bb64.loopexit2248 ]
  %style.sroa.0.sroa.49.2 = phi i8 [ %style.sroa.0.sroa.49.01923, %bb17 ], [ %style.sroa.0.sroa.49.01923, %bb46 ], [ %style.sroa.0.sroa.49.01923, %bb45 ], [ %style.sroa.0.sroa.49.01923, %bb44 ], [ %style.sroa.0.sroa.49.01923, %bb42 ], [ %style.sroa.0.sroa.49.01923, %bb41 ], [ %style.sroa.0.sroa.49.01923, %bb40 ], [ %style.sroa.0.sroa.49.01923, %bb39 ], [ %style.sroa.0.sroa.49.0.extract.trunc1587, %bb53 ], [ %style.sroa.0.sroa.49.0.extract.trunc1597, %bb61 ], [ %style.sroa.0.sroa.49.01923, %bb18 ], [ %style.sroa.0.sroa.49.01923, %bb117.thread ], [ %style.sroa.0.sroa.49.01923, %bb16.i193 ], [ %style.sroa.0.sroa.49.01923, %bb15.i ], [ %style.sroa.0.sroa.49.01923, %bb14.i ], [ %style.sroa.0.sroa.49.01923, %bb13.i192 ], [ %style.sroa.0.sroa.49.01923, %bb12.i191 ], [ %style.sroa.0.sroa.49.01923, %bb11.i190 ], [ 0, %bb16 ], [ 0, %bb116.thread ], [ 0, %bb16.i201 ], [ 0, %bb15.i200 ], [ 0, %bb14.i199 ], [ 0, %bb13.i198 ], [ 0, %bb12.i197 ], [ 0, %bb11.i195 ], [ %style.sroa.0.sroa.49.01923, %bb14 ], [ %style.sroa.0.sroa.49.01923, %bb88 ], [ %style.sroa.0.sroa.49.01923, %bb89 ], [ %style.sroa.0.sroa.49.01923, %bb90 ], [ %style.sroa.0.sroa.49.01923, %bb91 ], [ %style.sroa.0.sroa.49.01923, %bb92 ], [ %style.sroa.0.sroa.49.01923, %bb93 ], [ %style.sroa.0.sroa.49.01923, %bb94 ], [ 0, %bb12 ], [ 0, %bb79 ], [ 0, %bb80 ], [ 0, %bb81 ], [ 0, %bb82 ], [ 0, %bb83 ], [ 0, %bb84 ], [ 0, %bb85 ], [ %style.sroa.0.sroa.49.01923, %bb7 ], [ %style.sroa.0.sroa.49.01923, %bb64.loopexit ], [ %style.sroa.0.sroa.49.01923, %bb10 ], [ %style.sroa.0.sroa.49.01923, %bb63 ], [ %style.sroa.0.sroa.49.01923, %bb24 ], [ %style.sroa.0.sroa.49.01923, %bb23 ], [ %style.sroa.0.sroa.49.01923, %bb64.loopexit2248 ]
  %style.sroa.0.sroa.48.2 = phi i8 [ %style.sroa.0.sroa.48.01924, %bb17 ], [ %style.sroa.0.sroa.48.01924, %bb46 ], [ %style.sroa.0.sroa.48.01924, %bb45 ], [ %style.sroa.0.sroa.48.01924, %bb44 ], [ %style.sroa.0.sroa.48.01924, %bb42 ], [ %style.sroa.0.sroa.48.01924, %bb41 ], [ %style.sroa.0.sroa.48.01924, %bb40 ], [ %style.sroa.0.sroa.48.01924, %bb39 ], [ %style.sroa.0.sroa.48.0.extract.trunc1471, %bb53 ], [ %style.sroa.0.sroa.48.0.extract.trunc1481, %bb61 ], [ %style.sroa.0.sroa.48.01924, %bb18 ], [ %style.sroa.0.sroa.48.01924, %bb117.thread ], [ %style.sroa.0.sroa.48.01924, %bb16.i193 ], [ %style.sroa.0.sroa.48.01924, %bb15.i ], [ %style.sroa.0.sroa.48.01924, %bb14.i ], [ %style.sroa.0.sroa.48.01924, %bb13.i192 ], [ %style.sroa.0.sroa.48.01924, %bb12.i191 ], [ %style.sroa.0.sroa.48.01924, %bb11.i190 ], [ 0, %bb16 ], [ 0, %bb116.thread ], [ 0, %bb16.i201 ], [ 0, %bb15.i200 ], [ 0, %bb14.i199 ], [ 0, %bb13.i198 ], [ 0, %bb12.i197 ], [ 0, %bb11.i195 ], [ %style.sroa.0.sroa.48.01924, %bb14 ], [ %style.sroa.0.sroa.48.01924, %bb88 ], [ %style.sroa.0.sroa.48.01924, %bb89 ], [ %style.sroa.0.sroa.48.01924, %bb90 ], [ %style.sroa.0.sroa.48.01924, %bb91 ], [ %style.sroa.0.sroa.48.01924, %bb92 ], [ %style.sroa.0.sroa.48.01924, %bb93 ], [ %style.sroa.0.sroa.48.01924, %bb94 ], [ 0, %bb12 ], [ 0, %bb79 ], [ 0, %bb80 ], [ 0, %bb81 ], [ 0, %bb82 ], [ 0, %bb83 ], [ 0, %bb84 ], [ 0, %bb85 ], [ %style.sroa.0.sroa.48.01924, %bb7 ], [ %style.sroa.0.sroa.48.01924, %bb64.loopexit ], [ %style.sroa.0.sroa.48.01924, %bb10 ], [ %style.sroa.0.sroa.48.01924, %bb63 ], [ %style.sroa.0.sroa.48.01924, %bb24 ], [ %style.sroa.0.sroa.48.01924, %bb23 ], [ %style.sroa.0.sroa.48.01924, %bb64.loopexit2248 ]
  %style.sroa.0.sroa.45.2 = phi i8 [ %style.sroa.0.sroa.45.01925, %bb17 ], [ %style.sroa.0.sroa.45.01925, %bb46 ], [ %style.sroa.0.sroa.45.01925, %bb45 ], [ %style.sroa.0.sroa.45.01925, %bb44 ], [ %style.sroa.0.sroa.45.01925, %bb42 ], [ %style.sroa.0.sroa.45.01925, %bb41 ], [ %style.sroa.0.sroa.45.01925, %bb40 ], [ %style.sroa.0.sroa.45.01925, %bb39 ], [ %style.sroa.0.sroa.45.0.extract.trunc1356, %bb53 ], [ %style.sroa.0.sroa.45.0.extract.trunc1366, %bb61 ], [ %style.sroa.0.sroa.45.01925, %bb18 ], [ %style.sroa.0.sroa.45.01925, %bb117.thread ], [ %style.sroa.0.sroa.45.01925, %bb16.i193 ], [ %style.sroa.0.sroa.45.01925, %bb15.i ], [ %style.sroa.0.sroa.45.01925, %bb14.i ], [ %style.sroa.0.sroa.45.01925, %bb13.i192 ], [ %style.sroa.0.sroa.45.01925, %bb12.i191 ], [ %style.sroa.0.sroa.45.01925, %bb11.i190 ], [ 9, %bb16 ], [ 8, %bb116.thread ], [ 10, %bb16.i201 ], [ 11, %bb15.i200 ], [ 12, %bb14.i199 ], [ 13, %bb13.i198 ], [ 14, %bb12.i197 ], [ 15, %bb11.i195 ], [ %style.sroa.0.sroa.45.01925, %bb14 ], [ %style.sroa.0.sroa.45.01925, %bb88 ], [ %style.sroa.0.sroa.45.01925, %bb89 ], [ %style.sroa.0.sroa.45.01925, %bb90 ], [ %style.sroa.0.sroa.45.01925, %bb91 ], [ %style.sroa.0.sroa.45.01925, %bb92 ], [ %style.sroa.0.sroa.45.01925, %bb93 ], [ %style.sroa.0.sroa.45.01925, %bb94 ], [ 0, %bb12 ], [ 7, %bb79 ], [ 6, %bb80 ], [ 5, %bb81 ], [ 4, %bb82 ], [ 3, %bb83 ], [ 2, %bb84 ], [ 1, %bb85 ], [ %style.sroa.0.sroa.45.01925, %bb7 ], [ %style.sroa.0.sroa.45.01925, %bb64.loopexit ], [ %style.sroa.0.sroa.45.01925, %bb10 ], [ %style.sroa.0.sroa.45.01925, %bb63 ], [ %style.sroa.0.sroa.45.01925, %bb24 ], [ %style.sroa.0.sroa.45.01925, %bb23 ], [ %style.sroa.0.sroa.45.01925, %bb64.loopexit2248 ]
  %style.sroa.0.sroa.0.2 = phi i8 [ %style.sroa.0.sroa.0.01926, %bb17 ], [ %style.sroa.0.sroa.0.01926, %bb46 ], [ %style.sroa.0.sroa.0.01926, %bb45 ], [ %style.sroa.0.sroa.0.01926, %bb44 ], [ %style.sroa.0.sroa.0.01926, %bb42 ], [ %style.sroa.0.sroa.0.01926, %bb41 ], [ %style.sroa.0.sroa.0.01926, %bb40 ], [ %style.sroa.0.sroa.0.01926, %bb39 ], [ %style.sroa.0.sroa.0.0.extract.trunc1271, %bb53 ], [ %style.sroa.0.sroa.0.0.extract.trunc1278, %bb61 ], [ %style.sroa.0.sroa.0.01926, %bb18 ], [ %style.sroa.0.sroa.0.01926, %bb117.thread ], [ %style.sroa.0.sroa.0.01926, %bb16.i193 ], [ %style.sroa.0.sroa.0.01926, %bb15.i ], [ %style.sroa.0.sroa.0.01926, %bb14.i ], [ %style.sroa.0.sroa.0.01926, %bb13.i192 ], [ %style.sroa.0.sroa.0.01926, %bb12.i191 ], [ %style.sroa.0.sroa.0.01926, %bb11.i190 ], [ 0, %bb16 ], [ 0, %bb116.thread ], [ 0, %bb16.i201 ], [ 0, %bb15.i200 ], [ 0, %bb14.i199 ], [ 0, %bb13.i198 ], [ 0, %bb12.i197 ], [ 0, %bb11.i195 ], [ %style.sroa.0.sroa.0.01926, %bb14 ], [ %style.sroa.0.sroa.0.01926, %bb88 ], [ %style.sroa.0.sroa.0.01926, %bb89 ], [ %style.sroa.0.sroa.0.01926, %bb90 ], [ %style.sroa.0.sroa.0.01926, %bb91 ], [ %style.sroa.0.sroa.0.01926, %bb92 ], [ %style.sroa.0.sroa.0.01926, %bb93 ], [ %style.sroa.0.sroa.0.01926, %bb94 ], [ 0, %bb12 ], [ 0, %bb79 ], [ 0, %bb80 ], [ 0, %bb81 ], [ 0, %bb82 ], [ 0, %bb83 ], [ 0, %bb84 ], [ 0, %bb85 ], [ %style.sroa.0.sroa.0.01926, %bb7 ], [ 3, %bb64.loopexit ], [ %style.sroa.0.sroa.0.01926, %bb10 ], [ %style.sroa.0.sroa.0.01926, %bb23 ], [ %style.sroa.0.sroa.0.01926, %bb24 ], [ %style.sroa.0.sroa.0.01926, %bb63 ], [ 3, %bb64.loopexit2248 ]
  %style.sroa.50.sroa.49.2 = phi i8 [ %style.sroa.50.sroa.49.01927, %bb17 ], [ %style.sroa.50.sroa.49.01927, %bb46 ], [ %style.sroa.50.sroa.49.01927, %bb45 ], [ %style.sroa.50.sroa.49.01927, %bb44 ], [ %style.sroa.50.sroa.49.01927, %bb42 ], [ %style.sroa.50.sroa.49.01927, %bb41 ], [ %style.sroa.50.sroa.49.01927, %bb40 ], [ %style.sroa.50.sroa.49.01927, %bb39 ], [ %style.sroa.50.sroa.49.0.extract.trunc1224, %bb53 ], [ %style.sroa.50.sroa.49.0.extract.trunc1234, %bb61 ], [ 0, %bb18 ], [ 0, %bb117.thread ], [ 0, %bb16.i193 ], [ 0, %bb15.i ], [ 0, %bb14.i ], [ 0, %bb13.i192 ], [ 0, %bb12.i191 ], [ 0, %bb11.i190 ], [ %style.sroa.50.sroa.49.01927, %bb16 ], [ %style.sroa.50.sroa.49.01927, %bb116.thread ], [ %style.sroa.50.sroa.49.01927, %bb16.i201 ], [ %style.sroa.50.sroa.49.01927, %bb15.i200 ], [ %style.sroa.50.sroa.49.01927, %bb14.i199 ], [ %style.sroa.50.sroa.49.01927, %bb13.i198 ], [ %style.sroa.50.sroa.49.01927, %bb12.i197 ], [ %style.sroa.50.sroa.49.01927, %bb11.i195 ], [ 0, %bb14 ], [ 0, %bb88 ], [ 0, %bb89 ], [ 0, %bb90 ], [ 0, %bb91 ], [ 0, %bb92 ], [ 0, %bb93 ], [ 0, %bb94 ], [ %style.sroa.50.sroa.49.01927, %bb12 ], [ %style.sroa.50.sroa.49.01927, %bb79 ], [ %style.sroa.50.sroa.49.01927, %bb80 ], [ %style.sroa.50.sroa.49.01927, %bb81 ], [ %style.sroa.50.sroa.49.01927, %bb82 ], [ %style.sroa.50.sroa.49.01927, %bb83 ], [ %style.sroa.50.sroa.49.01927, %bb84 ], [ %style.sroa.50.sroa.49.01927, %bb85 ], [ %style.sroa.50.sroa.49.01927, %bb7 ], [ %style.sroa.50.sroa.49.01927, %bb64.loopexit ], [ %style.sroa.50.sroa.49.01927, %bb10 ], [ %style.sroa.50.sroa.49.01927, %bb63 ], [ %style.sroa.50.sroa.49.01927, %bb24 ], [ %style.sroa.50.sroa.49.01927, %bb23 ], [ %style.sroa.50.sroa.49.01927, %bb64.loopexit2248 ]
  %style.sroa.50.sroa.48.2 = phi i8 [ %style.sroa.50.sroa.48.01928, %bb17 ], [ %style.sroa.50.sroa.48.01928, %bb46 ], [ %style.sroa.50.sroa.48.01928, %bb45 ], [ %style.sroa.50.sroa.48.01928, %bb44 ], [ %style.sroa.50.sroa.48.01928, %bb42 ], [ %style.sroa.50.sroa.48.01928, %bb41 ], [ %style.sroa.50.sroa.48.01928, %bb40 ], [ %style.sroa.50.sroa.48.01928, %bb39 ], [ %style.sroa.50.sroa.48.0.extract.trunc1108, %bb53 ], [ %style.sroa.50.sroa.48.0.extract.trunc1118, %bb61 ], [ 0, %bb18 ], [ 0, %bb117.thread ], [ 0, %bb16.i193 ], [ 0, %bb15.i ], [ 0, %bb14.i ], [ 0, %bb13.i192 ], [ 0, %bb12.i191 ], [ 0, %bb11.i190 ], [ %style.sroa.50.sroa.48.01928, %bb16 ], [ %style.sroa.50.sroa.48.01928, %bb116.thread ], [ %style.sroa.50.sroa.48.01928, %bb16.i201 ], [ %style.sroa.50.sroa.48.01928, %bb15.i200 ], [ %style.sroa.50.sroa.48.01928, %bb14.i199 ], [ %style.sroa.50.sroa.48.01928, %bb13.i198 ], [ %style.sroa.50.sroa.48.01928, %bb12.i197 ], [ %style.sroa.50.sroa.48.01928, %bb11.i195 ], [ 0, %bb14 ], [ 0, %bb88 ], [ 0, %bb89 ], [ 0, %bb90 ], [ 0, %bb91 ], [ 0, %bb92 ], [ 0, %bb93 ], [ 0, %bb94 ], [ %style.sroa.50.sroa.48.01928, %bb12 ], [ %style.sroa.50.sroa.48.01928, %bb79 ], [ %style.sroa.50.sroa.48.01928, %bb80 ], [ %style.sroa.50.sroa.48.01928, %bb81 ], [ %style.sroa.50.sroa.48.01928, %bb82 ], [ %style.sroa.50.sroa.48.01928, %bb83 ], [ %style.sroa.50.sroa.48.01928, %bb84 ], [ %style.sroa.50.sroa.48.01928, %bb85 ], [ %style.sroa.50.sroa.48.01928, %bb7 ], [ %style.sroa.50.sroa.48.01928, %bb64.loopexit ], [ %style.sroa.50.sroa.48.01928, %bb10 ], [ %style.sroa.50.sroa.48.01928, %bb63 ], [ %style.sroa.50.sroa.48.01928, %bb24 ], [ %style.sroa.50.sroa.48.01928, %bb23 ], [ %style.sroa.50.sroa.48.01928, %bb64.loopexit2248 ]
  %style.sroa.50.sroa.45.2 = phi i8 [ %style.sroa.50.sroa.45.01929, %bb17 ], [ %style.sroa.50.sroa.45.01929, %bb46 ], [ %style.sroa.50.sroa.45.01929, %bb45 ], [ %style.sroa.50.sroa.45.01929, %bb44 ], [ %style.sroa.50.sroa.45.01929, %bb42 ], [ %style.sroa.50.sroa.45.01929, %bb41 ], [ %style.sroa.50.sroa.45.01929, %bb40 ], [ %style.sroa.50.sroa.45.01929, %bb39 ], [ %style.sroa.50.sroa.45.0.extract.trunc993, %bb53 ], [ %style.sroa.50.sroa.45.0.extract.trunc1003, %bb61 ], [ 9, %bb18 ], [ 8, %bb117.thread ], [ 10, %bb16.i193 ], [ 11, %bb15.i ], [ 12, %bb14.i ], [ 13, %bb13.i192 ], [ 14, %bb12.i191 ], [ 15, %bb11.i190 ], [ %style.sroa.50.sroa.45.01929, %bb16 ], [ %style.sroa.50.sroa.45.01929, %bb116.thread ], [ %style.sroa.50.sroa.45.01929, %bb16.i201 ], [ %style.sroa.50.sroa.45.01929, %bb15.i200 ], [ %style.sroa.50.sroa.45.01929, %bb14.i199 ], [ %style.sroa.50.sroa.45.01929, %bb13.i198 ], [ %style.sroa.50.sroa.45.01929, %bb12.i197 ], [ %style.sroa.50.sroa.45.01929, %bb11.i195 ], [ 0, %bb14 ], [ 7, %bb88 ], [ 6, %bb89 ], [ 5, %bb90 ], [ 4, %bb91 ], [ 3, %bb92 ], [ 2, %bb93 ], [ 1, %bb94 ], [ %style.sroa.50.sroa.45.01929, %bb12 ], [ %style.sroa.50.sroa.45.01929, %bb79 ], [ %style.sroa.50.sroa.45.01929, %bb80 ], [ %style.sroa.50.sroa.45.01929, %bb81 ], [ %style.sroa.50.sroa.45.01929, %bb82 ], [ %style.sroa.50.sroa.45.01929, %bb83 ], [ %style.sroa.50.sroa.45.01929, %bb84 ], [ %style.sroa.50.sroa.45.01929, %bb85 ], [ %style.sroa.50.sroa.45.01929, %bb7 ], [ %style.sroa.50.sroa.45.01929, %bb64.loopexit ], [ %style.sroa.50.sroa.45.01929, %bb10 ], [ %style.sroa.50.sroa.45.01929, %bb63 ], [ %style.sroa.50.sroa.45.01929, %bb24 ], [ %style.sroa.50.sroa.45.01929, %bb23 ], [ %style.sroa.50.sroa.45.01929, %bb64.loopexit2248 ]
  %style.sroa.50.sroa.0.2 = phi i8 [ %style.sroa.50.sroa.0.01930, %bb17 ], [ %style.sroa.50.sroa.0.01930, %bb46 ], [ %style.sroa.50.sroa.0.01930, %bb45 ], [ %style.sroa.50.sroa.0.01930, %bb44 ], [ %style.sroa.50.sroa.0.01930, %bb42 ], [ %style.sroa.50.sroa.0.01930, %bb41 ], [ %style.sroa.50.sroa.0.01930, %bb40 ], [ %style.sroa.50.sroa.0.01930, %bb39 ], [ %style.sroa.50.sroa.0.0.extract.trunc888, %bb53 ], [ %style.sroa.50.sroa.0.0.extract.trunc895, %bb61 ], [ 0, %bb18 ], [ 0, %bb117.thread ], [ 0, %bb16.i193 ], [ 0, %bb15.i ], [ 0, %bb14.i ], [ 0, %bb13.i192 ], [ 0, %bb12.i191 ], [ 0, %bb11.i190 ], [ %style.sroa.50.sroa.0.01930, %bb16 ], [ %style.sroa.50.sroa.0.01930, %bb116.thread ], [ %style.sroa.50.sroa.0.01930, %bb16.i201 ], [ %style.sroa.50.sroa.0.01930, %bb15.i200 ], [ %style.sroa.50.sroa.0.01930, %bb14.i199 ], [ %style.sroa.50.sroa.0.01930, %bb13.i198 ], [ %style.sroa.50.sroa.0.01930, %bb12.i197 ], [ %style.sroa.50.sroa.0.01930, %bb11.i195 ], [ 0, %bb14 ], [ 0, %bb88 ], [ 0, %bb89 ], [ 0, %bb90 ], [ 0, %bb91 ], [ 0, %bb92 ], [ 0, %bb93 ], [ 0, %bb94 ], [ %style.sroa.50.sroa.0.01930, %bb12 ], [ %style.sroa.50.sroa.0.01930, %bb79 ], [ %style.sroa.50.sroa.0.01930, %bb80 ], [ %style.sroa.50.sroa.0.01930, %bb81 ], [ %style.sroa.50.sroa.0.01930, %bb82 ], [ %style.sroa.50.sroa.0.01930, %bb83 ], [ %style.sroa.50.sroa.0.01930, %bb84 ], [ %style.sroa.50.sroa.0.01930, %bb85 ], [ %style.sroa.50.sroa.0.01930, %bb7 ], [ 3, %bb64.loopexit ], [ 3, %bb10 ], [ %style.sroa.50.sroa.0.01930, %bb63 ], [ %style.sroa.50.sroa.0.01930, %bb24 ], [ %style.sroa.50.sroa.0.01930, %bb23 ], [ %style.sroa.50.sroa.0.01930, %bb64.loopexit2248 ]
  %style.sroa.72.sroa.50.2 = phi i8 [ %style.sroa.72.sroa.50.01931, %bb17 ], [ %style.sroa.72.sroa.50.01931, %bb46 ], [ %style.sroa.72.sroa.50.01931, %bb45 ], [ %style.sroa.72.sroa.50.01931, %bb44 ], [ %style.sroa.72.sroa.50.01931, %bb42 ], [ %style.sroa.72.sroa.50.01931, %bb41 ], [ %style.sroa.72.sroa.50.01931, %bb40 ], [ %style.sroa.72.sroa.50.01931, %bb39 ], [ %style.sroa.72.sroa.50.0.extract.trunc803, %bb53 ], [ %style.sroa.72.sroa.50.0.extract.trunc813, %bb61 ], [ %style.sroa.72.sroa.50.01931, %bb18 ], [ %style.sroa.72.sroa.50.01931, %bb117.thread ], [ %style.sroa.72.sroa.50.01931, %bb16.i193 ], [ %style.sroa.72.sroa.50.01931, %bb15.i ], [ %style.sroa.72.sroa.50.01931, %bb14.i ], [ %style.sroa.72.sroa.50.01931, %bb13.i192 ], [ %style.sroa.72.sroa.50.01931, %bb12.i191 ], [ %style.sroa.72.sroa.50.01931, %bb11.i190 ], [ %style.sroa.72.sroa.50.01931, %bb16 ], [ %style.sroa.72.sroa.50.01931, %bb116.thread ], [ %style.sroa.72.sroa.50.01931, %bb16.i201 ], [ %style.sroa.72.sroa.50.01931, %bb15.i200 ], [ %style.sroa.72.sroa.50.01931, %bb14.i199 ], [ %style.sroa.72.sroa.50.01931, %bb13.i198 ], [ %style.sroa.72.sroa.50.01931, %bb12.i197 ], [ %style.sroa.72.sroa.50.01931, %bb11.i195 ], [ %style.sroa.72.sroa.50.01931, %bb14 ], [ %style.sroa.72.sroa.50.01931, %bb88 ], [ %style.sroa.72.sroa.50.01931, %bb89 ], [ %style.sroa.72.sroa.50.01931, %bb90 ], [ %style.sroa.72.sroa.50.01931, %bb91 ], [ %style.sroa.72.sroa.50.01931, %bb92 ], [ %style.sroa.72.sroa.50.01931, %bb93 ], [ %style.sroa.72.sroa.50.01931, %bb94 ], [ %style.sroa.72.sroa.50.01931, %bb12 ], [ %style.sroa.72.sroa.50.01931, %bb79 ], [ %style.sroa.72.sroa.50.01931, %bb80 ], [ %style.sroa.72.sroa.50.01931, %bb81 ], [ %style.sroa.72.sroa.50.01931, %bb82 ], [ %style.sroa.72.sroa.50.01931, %bb83 ], [ %style.sroa.72.sroa.50.01931, %bb84 ], [ %style.sroa.72.sroa.50.01931, %bb85 ], [ %style.sroa.72.sroa.50.01931, %bb7 ], [ %style.sroa.72.sroa.50.01931, %bb64.loopexit ], [ %style.sroa.72.sroa.50.01931, %bb10 ], [ %style.sroa.72.sroa.50.01931, %bb63 ], [ %style.sroa.72.sroa.50.01931, %bb24 ], [ %style.sroa.72.sroa.50.01931, %bb23 ], [ %style.sroa.72.sroa.50.01931, %bb64.loopexit2248 ]
  %style.sroa.72.sroa.49.2 = phi i8 [ %style.sroa.72.sroa.49.01932, %bb17 ], [ %style.sroa.72.sroa.49.01932, %bb46 ], [ %style.sroa.72.sroa.49.01932, %bb45 ], [ %style.sroa.72.sroa.49.01932, %bb44 ], [ %style.sroa.72.sroa.49.01932, %bb42 ], [ %style.sroa.72.sroa.49.01932, %bb41 ], [ %style.sroa.72.sroa.49.01932, %bb40 ], [ %style.sroa.72.sroa.49.01932, %bb39 ], [ %style.sroa.72.sroa.49.0.extract.trunc675, %bb53 ], [ %style.sroa.72.sroa.49.0.extract.trunc685, %bb61 ], [ %style.sroa.72.sroa.49.01932, %bb18 ], [ %style.sroa.72.sroa.49.01932, %bb117.thread ], [ %style.sroa.72.sroa.49.01932, %bb16.i193 ], [ %style.sroa.72.sroa.49.01932, %bb15.i ], [ %style.sroa.72.sroa.49.01932, %bb14.i ], [ %style.sroa.72.sroa.49.01932, %bb13.i192 ], [ %style.sroa.72.sroa.49.01932, %bb12.i191 ], [ %style.sroa.72.sroa.49.01932, %bb11.i190 ], [ %style.sroa.72.sroa.49.01932, %bb16 ], [ %style.sroa.72.sroa.49.01932, %bb116.thread ], [ %style.sroa.72.sroa.49.01932, %bb16.i201 ], [ %style.sroa.72.sroa.49.01932, %bb15.i200 ], [ %style.sroa.72.sroa.49.01932, %bb14.i199 ], [ %style.sroa.72.sroa.49.01932, %bb13.i198 ], [ %style.sroa.72.sroa.49.01932, %bb12.i197 ], [ %style.sroa.72.sroa.49.01932, %bb11.i195 ], [ %style.sroa.72.sroa.49.01932, %bb14 ], [ %style.sroa.72.sroa.49.01932, %bb88 ], [ %style.sroa.72.sroa.49.01932, %bb89 ], [ %style.sroa.72.sroa.49.01932, %bb90 ], [ %style.sroa.72.sroa.49.01932, %bb91 ], [ %style.sroa.72.sroa.49.01932, %bb92 ], [ %style.sroa.72.sroa.49.01932, %bb93 ], [ %style.sroa.72.sroa.49.01932, %bb94 ], [ %style.sroa.72.sroa.49.01932, %bb12 ], [ %style.sroa.72.sroa.49.01932, %bb79 ], [ %style.sroa.72.sroa.49.01932, %bb80 ], [ %style.sroa.72.sroa.49.01932, %bb81 ], [ %style.sroa.72.sroa.49.01932, %bb82 ], [ %style.sroa.72.sroa.49.01932, %bb83 ], [ %style.sroa.72.sroa.49.01932, %bb84 ], [ %style.sroa.72.sroa.49.01932, %bb85 ], [ %style.sroa.72.sroa.49.01932, %bb7 ], [ %style.sroa.72.sroa.49.01932, %bb64.loopexit ], [ %style.sroa.72.sroa.49.01932, %bb10 ], [ %style.sroa.72.sroa.49.01932, %bb63 ], [ %style.sroa.72.sroa.49.01932, %bb24 ], [ %style.sroa.72.sroa.49.01932, %bb23 ], [ %style.sroa.72.sroa.49.01932, %bb64.loopexit2248 ]
  %style.sroa.72.sroa.48.2 = phi i8 [ %style.sroa.72.sroa.48.01933, %bb17 ], [ %style.sroa.72.sroa.48.01933, %bb46 ], [ %style.sroa.72.sroa.48.01933, %bb45 ], [ %style.sroa.72.sroa.48.01933, %bb44 ], [ %style.sroa.72.sroa.48.01933, %bb42 ], [ %style.sroa.72.sroa.48.01933, %bb41 ], [ %style.sroa.72.sroa.48.01933, %bb40 ], [ %style.sroa.72.sroa.48.01933, %bb39 ], [ %style.sroa.72.sroa.48.0.extract.trunc547, %bb53 ], [ %style.sroa.72.sroa.48.0.extract.trunc557, %bb61 ], [ %style.sroa.72.sroa.48.01933, %bb18 ], [ %style.sroa.72.sroa.48.01933, %bb117.thread ], [ %style.sroa.72.sroa.48.01933, %bb16.i193 ], [ %style.sroa.72.sroa.48.01933, %bb15.i ], [ %style.sroa.72.sroa.48.01933, %bb14.i ], [ %style.sroa.72.sroa.48.01933, %bb13.i192 ], [ %style.sroa.72.sroa.48.01933, %bb12.i191 ], [ %style.sroa.72.sroa.48.01933, %bb11.i190 ], [ %style.sroa.72.sroa.48.01933, %bb16 ], [ %style.sroa.72.sroa.48.01933, %bb116.thread ], [ %style.sroa.72.sroa.48.01933, %bb16.i201 ], [ %style.sroa.72.sroa.48.01933, %bb15.i200 ], [ %style.sroa.72.sroa.48.01933, %bb14.i199 ], [ %style.sroa.72.sroa.48.01933, %bb13.i198 ], [ %style.sroa.72.sroa.48.01933, %bb12.i197 ], [ %style.sroa.72.sroa.48.01933, %bb11.i195 ], [ %style.sroa.72.sroa.48.01933, %bb14 ], [ %style.sroa.72.sroa.48.01933, %bb88 ], [ %style.sroa.72.sroa.48.01933, %bb89 ], [ %style.sroa.72.sroa.48.01933, %bb90 ], [ %style.sroa.72.sroa.48.01933, %bb91 ], [ %style.sroa.72.sroa.48.01933, %bb92 ], [ %style.sroa.72.sroa.48.01933, %bb93 ], [ %style.sroa.72.sroa.48.01933, %bb94 ], [ %style.sroa.72.sroa.48.01933, %bb12 ], [ %style.sroa.72.sroa.48.01933, %bb79 ], [ %style.sroa.72.sroa.48.01933, %bb80 ], [ %style.sroa.72.sroa.48.01933, %bb81 ], [ %style.sroa.72.sroa.48.01933, %bb82 ], [ %style.sroa.72.sroa.48.01933, %bb83 ], [ %style.sroa.72.sroa.48.01933, %bb84 ], [ %style.sroa.72.sroa.48.01933, %bb85 ], [ %style.sroa.72.sroa.48.01933, %bb7 ], [ %style.sroa.72.sroa.48.01933, %bb64.loopexit ], [ %style.sroa.72.sroa.48.01933, %bb10 ], [ %style.sroa.72.sroa.48.01933, %bb63 ], [ %style.sroa.72.sroa.48.01933, %bb24 ], [ %style.sroa.72.sroa.48.01933, %bb23 ], [ %style.sroa.72.sroa.48.01933, %bb64.loopexit2248 ]
  %style.sroa.72.sroa.0.2 = phi i8 [ %style.sroa.72.sroa.0.01934, %bb17 ], [ %style.sroa.72.sroa.0.01934, %bb46 ], [ %style.sroa.72.sroa.0.01934, %bb45 ], [ %style.sroa.72.sroa.0.01934, %bb44 ], [ %style.sroa.72.sroa.0.01934, %bb42 ], [ %style.sroa.72.sroa.0.01934, %bb41 ], [ %style.sroa.72.sroa.0.01934, %bb40 ], [ %style.sroa.72.sroa.0.01934, %bb39 ], [ %style.sroa.72.sroa.0.0.extract.trunc428, %bb53 ], [ %style.sroa.72.sroa.0.0.extract.trunc435, %bb61 ], [ %style.sroa.72.sroa.0.01934, %bb18 ], [ %style.sroa.72.sroa.0.01934, %bb117.thread ], [ %style.sroa.72.sroa.0.01934, %bb16.i193 ], [ %style.sroa.72.sroa.0.01934, %bb15.i ], [ %style.sroa.72.sroa.0.01934, %bb14.i ], [ %style.sroa.72.sroa.0.01934, %bb13.i192 ], [ %style.sroa.72.sroa.0.01934, %bb12.i191 ], [ %style.sroa.72.sroa.0.01934, %bb11.i190 ], [ %style.sroa.72.sroa.0.01934, %bb16 ], [ %style.sroa.72.sroa.0.01934, %bb116.thread ], [ %style.sroa.72.sroa.0.01934, %bb16.i201 ], [ %style.sroa.72.sroa.0.01934, %bb15.i200 ], [ %style.sroa.72.sroa.0.01934, %bb14.i199 ], [ %style.sroa.72.sroa.0.01934, %bb13.i198 ], [ %style.sroa.72.sroa.0.01934, %bb12.i197 ], [ %style.sroa.72.sroa.0.01934, %bb11.i195 ], [ %style.sroa.72.sroa.0.01934, %bb14 ], [ %style.sroa.72.sroa.0.01934, %bb88 ], [ %style.sroa.72.sroa.0.01934, %bb89 ], [ %style.sroa.72.sroa.0.01934, %bb90 ], [ %style.sroa.72.sroa.0.01934, %bb91 ], [ %style.sroa.72.sroa.0.01934, %bb92 ], [ %style.sroa.72.sroa.0.01934, %bb93 ], [ %style.sroa.72.sroa.0.01934, %bb94 ], [ %style.sroa.72.sroa.0.01934, %bb12 ], [ %style.sroa.72.sroa.0.01934, %bb79 ], [ %style.sroa.72.sroa.0.01934, %bb80 ], [ %style.sroa.72.sroa.0.01934, %bb81 ], [ %style.sroa.72.sroa.0.01934, %bb82 ], [ %style.sroa.72.sroa.0.01934, %bb83 ], [ %style.sroa.72.sroa.0.01934, %bb84 ], [ %style.sroa.72.sroa.0.01934, %bb85 ], [ %style.sroa.72.sroa.0.01934, %bb7 ], [ 3, %bb64.loopexit ], [ %style.sroa.72.sroa.0.01934, %bb10 ], [ %style.sroa.72.sroa.0.01934, %bb63 ], [ %style.sroa.72.sroa.0.01934, %bb24 ], [ %style.sroa.72.sroa.0.01934, %bb23 ], [ %style.sroa.72.sroa.0.01934, %bb64.loopexit2248 ]
  %style.sroa.89.2 = phi i16 [ %style.sroa.89.11875, %bb17 ], [ %21, %bb46 ], [ %22, %bb45 ], [ %23, %bb44 ], [ %25, %bb42 ], [ %26, %bb41 ], [ %27, %bb40 ], [ %28, %bb39 ], [ %style.sroa.89.11875, %bb53 ], [ %style.sroa.89.11875, %bb61 ], [ %style.sroa.89.11875, %bb18 ], [ %style.sroa.89.11875, %bb117.thread ], [ %style.sroa.89.11875, %bb16.i193 ], [ %style.sroa.89.11875, %bb15.i ], [ %style.sroa.89.11875, %bb14.i ], [ %style.sroa.89.11875, %bb13.i192 ], [ %style.sroa.89.11875, %bb12.i191 ], [ %style.sroa.89.11875, %bb11.i190 ], [ %style.sroa.89.11875, %bb16 ], [ %style.sroa.89.11875, %bb116.thread ], [ %style.sroa.89.11875, %bb16.i201 ], [ %style.sroa.89.11875, %bb15.i200 ], [ %style.sroa.89.11875, %bb14.i199 ], [ %style.sroa.89.11875, %bb13.i198 ], [ %style.sroa.89.11875, %bb12.i197 ], [ %style.sroa.89.11875, %bb11.i195 ], [ %style.sroa.89.11875, %bb14 ], [ %style.sroa.89.11875, %bb88 ], [ %style.sroa.89.11875, %bb89 ], [ %style.sroa.89.11875, %bb90 ], [ %style.sroa.89.11875, %bb91 ], [ %style.sroa.89.11875, %bb92 ], [ %style.sroa.89.11875, %bb93 ], [ %style.sroa.89.11875, %bb94 ], [ %style.sroa.89.11875, %bb12 ], [ %style.sroa.89.11875, %bb79 ], [ %style.sroa.89.11875, %bb80 ], [ %style.sroa.89.11875, %bb81 ], [ %style.sroa.89.11875, %bb82 ], [ %style.sroa.89.11875, %bb83 ], [ %style.sroa.89.11875, %bb84 ], [ %style.sroa.89.11875, %bb85 ], [ %style.sroa.89.01935, %bb7 ], [ %n, %bb64.loopexit ], [ %style.sroa.89.11875, %bb10 ], [ %style.sroa.89.11875, %bb23 ], [ %style.sroa.89.11875, %bb24 ], [ %style.sroa.89.3, %bb63 ], [ %style.sroa.89.11875, %bb64.loopexit2248 ]
; call <anstyle_parse::params::ParamsIter as core::iter::traits::iterator::Iterator>::next
  %29 = call { ptr, i64 } @_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull align 8 dereferenceable(16) %iter)
  %30 = extractvalue { ptr, i64 } %29, 0
  %.not = icmp eq ptr %30, null
  br i1 %.not, label %bb8, label %bb7

bb63:                                             ; preds = %bb23, %bb10, %bb57, %bb56, %bb25, %bb26, %bb27, %bb28, %bb29, %bb24, %bb32, %bb34, %bb36, %bb43
  %style.sroa.89.3 = phi i16 [ %24, %bb43 ], [ %style.sroa.89.11875, %bb10 ], [ %style.sroa.89.11875, %bb36 ], [ %style.sroa.89.11875, %bb34 ], [ %style.sroa.89.11875, %bb23 ], [ %style.sroa.89.11875, %bb32 ], [ %style.sroa.89.11875, %bb56 ], [ %style.sroa.89.11875, %bb57 ], [ %41, %bb29 ], [ %style.sroa.89.11875, %bb24 ], [ %43, %bb28 ], [ %45, %bb27 ], [ %47, %bb26 ], [ %49, %bb25 ]
  %r.sroa.0.2.off0 = phi i1 [ %r.sroa.0.1.off01876, %bb43 ], [ %r.sroa.0.1.off01876, %bb10 ], [ %r.sroa.0.1.off01876, %bb36 ], [ %r.sroa.0.1.off01876, %bb34 ], [ %r.sroa.0.1.off01876, %bb23 ], [ false, %bb32 ], [ true, %bb56 ], [ true, %bb57 ], [ %r.sroa.0.1.off01876, %bb29 ], [ %r.sroa.0.1.off01876, %bb24 ], [ %r.sroa.0.1.off01876, %bb28 ], [ %r.sroa.0.1.off01876, %bb27 ], [ %r.sroa.0.1.off01876, %bb26 ], [ %r.sroa.0.1.off01876, %bb25 ]
  %r.sroa.4.2.off0 = phi i8 [ %r.sroa.4.1.off01877, %bb43 ], [ %r.sroa.4.1.off01877, %bb10 ], [ %r.sroa.4.1.off01877, %bb36 ], [ %r.sroa.4.1.off01877, %bb34 ], [ %r.sroa.4.1.off01877, %bb23 ], [ undef, %bb32 ], [ %r.sroa.4.1.off01877, %bb56 ], [ %extract.t139, %bb57 ], [ %r.sroa.4.1.off01877, %bb29 ], [ %r.sroa.4.1.off01877, %bb24 ], [ %r.sroa.4.1.off01877, %bb28 ], [ %r.sroa.4.1.off01877, %bb27 ], [ %r.sroa.4.1.off01877, %bb26 ], [ %r.sroa.4.1.off01877, %bb25 ]
  %g.sroa.0.2.off0 = phi i1 [ %g.sroa.0.1.off01878, %bb43 ], [ %g.sroa.0.1.off01878, %bb10 ], [ %g.sroa.0.1.off01878, %bb36 ], [ %g.sroa.0.1.off01878, %bb34 ], [ %g.sroa.0.1.off01878, %bb23 ], [ false, %bb32 ], [ true, %bb56 ], [ %g.sroa.0.1.off01878, %bb57 ], [ %g.sroa.0.1.off01878, %bb29 ], [ %g.sroa.0.1.off01878, %bb24 ], [ %g.sroa.0.1.off01878, %bb28 ], [ %g.sroa.0.1.off01878, %bb27 ], [ %g.sroa.0.1.off01878, %bb26 ], [ %g.sroa.0.1.off01878, %bb25 ]
  %g.sroa.4.2.off0 = phi i8 [ %g.sroa.4.1.off01879, %bb43 ], [ %g.sroa.4.1.off01879, %bb10 ], [ %g.sroa.4.1.off01879, %bb36 ], [ %g.sroa.4.1.off01879, %bb34 ], [ %g.sroa.4.1.off01879, %bb23 ], [ undef, %bb32 ], [ %extract.t153, %bb56 ], [ %g.sroa.4.1.off01879, %bb57 ], [ %g.sroa.4.1.off01879, %bb29 ], [ %g.sroa.4.1.off01879, %bb24 ], [ %g.sroa.4.1.off01879, %bb28 ], [ %g.sroa.4.1.off01879, %bb27 ], [ %g.sroa.4.1.off01879, %bb26 ], [ %g.sroa.4.1.off01879, %bb25 ]
  %color_target.sroa.0.2 = phi i8 [ %color_target.sroa.0.11881, %bb43 ], [ 0, %bb10 ], [ 1, %bb36 ], [ 2, %bb34 ], [ %color_target.sroa.0.11881, %bb23 ], [ %color_target.sroa.0.11881, %bb32 ], [ %color_target.sroa.0.11881, %bb56 ], [ %color_target.sroa.0.11881, %bb57 ], [ %color_target.sroa.0.11881, %bb29 ], [ %color_target.sroa.0.11881, %bb24 ], [ %color_target.sroa.0.11881, %bb28 ], [ %color_target.sroa.0.11881, %bb27 ], [ %color_target.sroa.0.11881, %bb26 ], [ %color_target.sroa.0.11881, %bb25 ]
  %state.sroa.0.2 = phi i8 [ 4, %bb43 ], [ 1, %bb10 ], [ 1, %bb36 ], [ 1, %bb34 ], [ 2, %bb23 ], [ 3, %bb32 ], [ 3, %bb56 ], [ 3, %bb57 ], [ 4, %bb29 ], [ 4, %bb24 ], [ 4, %bb28 ], [ 4, %bb27 ], [ 4, %bb26 ], [ 4, %bb25 ]
  %_118 = icmp eq ptr %_124, %_111
  br i1 %_118, label %bb64, label %bb73

bb13:                                             ; preds = %bb11
  %31 = and i16 %n, -8
  %or.cond8 = icmp eq i16 %31, 40
  br i1 %or.cond8, label %bb14, label %bb15

bb12:                                             ; preds = %bb11
  switch i16 %n, label %default.unreachable [
    i16 30, label %bb64
    i16 31, label %bb85
    i16 32, label %bb84
    i16 33, label %bb83
    i16 34, label %bb82
    i16 35, label %bb81
    i16 36, label %bb80
    i16 37, label %bb79
  ], !prof !95

bb15:                                             ; preds = %bb13
  %32 = add i16 %n, -90
  %or.cond9 = icmp ult i16 %32, 8
  br i1 %or.cond9, label %bb16, label %bb17

bb14:                                             ; preds = %bb13
  switch i16 %n, label %default.unreachable [
    i16 40, label %bb64
    i16 41, label %bb94
    i16 42, label %bb93
    i16 43, label %bb92
    i16 44, label %bb91
    i16 45, label %bb90
    i16 46, label %bb89
    i16 47, label %bb88
  ], !prof !95

bb17:                                             ; preds = %bb15
  %33 = add i16 %n, -100
  %or.cond10 = icmp ult i16 %33, 8
  br i1 %or.cond10, label %bb18, label %bb64

bb16:                                             ; preds = %bb15
  switch i16 %n, label %default.unreachable [
    i16 90, label %bb116.thread
    i16 91, label %bb64
    i16 92, label %bb16.i201
    i16 93, label %bb15.i200
    i16 94, label %bb14.i199
    i16 95, label %bb13.i198
    i16 96, label %bb12.i197
    i16 97, label %bb11.i195
  ], !prof !95

bb116.thread:                                     ; preds = %bb16
  br label %bb64

bb18:                                             ; preds = %bb17
  switch i16 %n, label %default.unreachable [
    i16 100, label %bb117.thread
    i16 101, label %bb64
    i16 102, label %bb16.i193
    i16 103, label %bb15.i
    i16 104, label %bb14.i
    i16 105, label %bb13.i192
    i16 106, label %bb12.i191
    i16 107, label %bb11.i190
  ], !prof !95

bb117.thread:                                     ; preds = %bb18
  br label %bb64

bb16.i193:                                        ; preds = %bb18
  br label %bb64

bb15.i:                                           ; preds = %bb18
  br label %bb64

bb14.i:                                           ; preds = %bb18
  br label %bb64

bb13.i192:                                        ; preds = %bb18
  br label %bb64

bb12.i191:                                        ; preds = %bb18
  br label %bb64

bb11.i190:                                        ; preds = %bb18
  br label %bb64

bb16.i201:                                        ; preds = %bb16
  br label %bb64

bb15.i200:                                        ; preds = %bb16
  br label %bb64

bb14.i199:                                        ; preds = %bb16
  br label %bb64

bb13.i198:                                        ; preds = %bb16
  br label %bb64

bb12.i197:                                        ; preds = %bb16
  br label %bb64

bb11.i195:                                        ; preds = %bb16
  br label %bb64

bb94:                                             ; preds = %bb14
  br label %bb64

bb93:                                             ; preds = %bb14
  br label %bb64

bb92:                                             ; preds = %bb14
  br label %bb64

bb91:                                             ; preds = %bb14
  br label %bb64

bb90:                                             ; preds = %bb14
  br label %bb64

bb89:                                             ; preds = %bb14
  br label %bb64

bb88:                                             ; preds = %bb14
  br label %bb64

bb85:                                             ; preds = %bb12
  br label %bb64

bb84:                                             ; preds = %bb12
  br label %bb64

bb83:                                             ; preds = %bb12
  br label %bb64

bb82:                                             ; preds = %bb12
  br label %bb64

bb81:                                             ; preds = %bb12
  br label %bb64

bb80:                                             ; preds = %bb12
  br label %bb64

bb79:                                             ; preds = %bb12
  br label %bb64

bb32:                                             ; preds = %bb23
  br label %bb63

bb52:                                             ; preds = %bb31
  %style.sroa.50.sroa.49.0.insert.ext1215 = zext i8 %style.sroa.50.sroa.49.01927 to i32
  %style.sroa.50.sroa.49.0.insert.shift1216 = shl nuw i32 %style.sroa.50.sroa.49.0.insert.ext1215, 24
  %style.sroa.50.sroa.48.0.insert.ext1099 = zext i8 %style.sroa.50.sroa.48.01928 to i32
  %style.sroa.50.sroa.48.0.insert.shift1100 = shl nuw nsw i32 %style.sroa.50.sroa.48.0.insert.ext1099, 16
  %style.sroa.50.sroa.48.0.insert.insert1102 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.shift1100, %style.sroa.50.sroa.49.0.insert.shift1216
  %style.sroa.50.sroa.45.0.insert.ext984 = zext i8 %style.sroa.50.sroa.45.01929 to i32
  %style.sroa.50.sroa.45.0.insert.shift985 = shl nuw nsw i32 %style.sroa.50.sroa.45.0.insert.ext984, 8
  %style.sroa.50.sroa.45.0.insert.insert987 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.insert1102, %style.sroa.50.sroa.45.0.insert.shift985
  %style.sroa.50.sroa.0.0.insert.ext882 = zext i8 %style.sroa.50.sroa.0.01930 to i32
  %style.sroa.50.sroa.0.0.insert.insert884 = or disjoint i32 %style.sroa.50.sroa.45.0.insert.insert987, %style.sroa.50.sroa.0.0.insert.ext882
  %style.sroa.72.sroa.50.0.insert.ext794 = zext i8 %style.sroa.72.sroa.50.01931 to i32
  %style.sroa.72.sroa.50.0.insert.shift795 = shl nuw i32 %style.sroa.72.sroa.50.0.insert.ext794, 24
  %style.sroa.72.sroa.49.0.insert.ext666 = zext i8 %style.sroa.72.sroa.49.01932 to i32
  %style.sroa.72.sroa.49.0.insert.shift667 = shl nuw nsw i32 %style.sroa.72.sroa.49.0.insert.ext666, 16
  %style.sroa.72.sroa.49.0.insert.insert669 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.shift667, %style.sroa.72.sroa.50.0.insert.shift795
  %style.sroa.72.sroa.48.0.insert.ext538 = zext i8 %style.sroa.72.sroa.48.01933 to i32
  %style.sroa.72.sroa.48.0.insert.shift539 = shl nuw nsw i32 %style.sroa.72.sroa.48.0.insert.ext538, 8
  %style.sroa.72.sroa.48.0.insert.insert541 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.insert669, %style.sroa.72.sroa.48.0.insert.shift539
  %style.sroa.72.sroa.0.0.insert.ext422 = zext i8 %style.sroa.72.sroa.0.01934 to i32
  %style.sroa.72.sroa.0.0.insert.insert424 = or disjoint i32 %style.sroa.72.sroa.48.0.insert.insert541, %style.sroa.72.sroa.0.0.insert.ext422
  %34 = shl i16 %n, 8
  %35 = or disjoint i16 %34, 1
  %_72.sroa.0.0.insert.insert = zext i16 %35 to i32
  br label %bb53

bb51:                                             ; preds = %bb31
  %style.sroa.0.sroa.49.0.insert.ext1578 = zext i8 %style.sroa.0.sroa.49.01923 to i32
  %style.sroa.0.sroa.49.0.insert.shift1579 = shl nuw i32 %style.sroa.0.sroa.49.0.insert.ext1578, 24
  %style.sroa.0.sroa.48.0.insert.ext1462 = zext i8 %style.sroa.0.sroa.48.01924 to i32
  %style.sroa.0.sroa.48.0.insert.shift1463 = shl nuw nsw i32 %style.sroa.0.sroa.48.0.insert.ext1462, 16
  %style.sroa.0.sroa.48.0.insert.insert1465 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.shift1463, %style.sroa.0.sroa.49.0.insert.shift1579
  %style.sroa.0.sroa.45.0.insert.ext1347 = zext i8 %style.sroa.0.sroa.45.01925 to i32
  %style.sroa.0.sroa.45.0.insert.shift1348 = shl nuw nsw i32 %style.sroa.0.sroa.45.0.insert.ext1347, 8
  %style.sroa.0.sroa.45.0.insert.insert1350 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.insert1465, %style.sroa.0.sroa.45.0.insert.shift1348
  %style.sroa.0.sroa.0.0.insert.ext1265 = zext i8 %style.sroa.0.sroa.0.01926 to i32
  %style.sroa.0.sroa.0.0.insert.insert1267 = or disjoint i32 %style.sroa.0.sroa.45.0.insert.insert1350, %style.sroa.0.sroa.0.0.insert.ext1265
  %style.sroa.72.sroa.50.0.insert.ext798 = zext i8 %style.sroa.72.sroa.50.01931 to i32
  %style.sroa.72.sroa.50.0.insert.shift799 = shl nuw i32 %style.sroa.72.sroa.50.0.insert.ext798, 24
  %style.sroa.72.sroa.49.0.insert.ext670 = zext i8 %style.sroa.72.sroa.49.01932 to i32
  %style.sroa.72.sroa.49.0.insert.shift671 = shl nuw nsw i32 %style.sroa.72.sroa.49.0.insert.ext670, 16
  %style.sroa.72.sroa.49.0.insert.insert673 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.shift671, %style.sroa.72.sroa.50.0.insert.shift799
  %style.sroa.72.sroa.48.0.insert.ext542 = zext i8 %style.sroa.72.sroa.48.01933 to i32
  %style.sroa.72.sroa.48.0.insert.shift543 = shl nuw nsw i32 %style.sroa.72.sroa.48.0.insert.ext542, 8
  %style.sroa.72.sroa.48.0.insert.insert545 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.insert673, %style.sroa.72.sroa.48.0.insert.shift543
  %style.sroa.72.sroa.0.0.insert.ext425 = zext i8 %style.sroa.72.sroa.0.01934 to i32
  %style.sroa.72.sroa.0.0.insert.insert427 = or disjoint i32 %style.sroa.72.sroa.48.0.insert.insert545, %style.sroa.72.sroa.0.0.insert.ext425
  %36 = shl i16 %n, 8
  %37 = or disjoint i16 %36, 1
  %_74.sroa.0.0.insert.insert = zext i16 %37 to i32
  br label %bb53

bb50:                                             ; preds = %bb31
  %style.sroa.0.sroa.49.0.insert.ext1582 = zext i8 %style.sroa.0.sroa.49.01923 to i32
  %style.sroa.0.sroa.49.0.insert.shift1583 = shl nuw i32 %style.sroa.0.sroa.49.0.insert.ext1582, 24
  %style.sroa.0.sroa.48.0.insert.ext1466 = zext i8 %style.sroa.0.sroa.48.01924 to i32
  %style.sroa.0.sroa.48.0.insert.shift1467 = shl nuw nsw i32 %style.sroa.0.sroa.48.0.insert.ext1466, 16
  %style.sroa.0.sroa.48.0.insert.insert1469 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.shift1467, %style.sroa.0.sroa.49.0.insert.shift1583
  %style.sroa.0.sroa.45.0.insert.ext1351 = zext i8 %style.sroa.0.sroa.45.01925 to i32
  %style.sroa.0.sroa.45.0.insert.shift1352 = shl nuw nsw i32 %style.sroa.0.sroa.45.0.insert.ext1351, 8
  %style.sroa.0.sroa.45.0.insert.insert1354 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.insert1469, %style.sroa.0.sroa.45.0.insert.shift1352
  %style.sroa.0.sroa.0.0.insert.ext1268 = zext i8 %style.sroa.0.sroa.0.01926 to i32
  %style.sroa.0.sroa.0.0.insert.insert1270 = or disjoint i32 %style.sroa.0.sroa.45.0.insert.insert1354, %style.sroa.0.sroa.0.0.insert.ext1268
  %style.sroa.50.sroa.49.0.insert.ext1219 = zext i8 %style.sroa.50.sroa.49.01927 to i32
  %style.sroa.50.sroa.49.0.insert.shift1220 = shl nuw i32 %style.sroa.50.sroa.49.0.insert.ext1219, 24
  %style.sroa.50.sroa.48.0.insert.ext1103 = zext i8 %style.sroa.50.sroa.48.01928 to i32
  %style.sroa.50.sroa.48.0.insert.shift1104 = shl nuw nsw i32 %style.sroa.50.sroa.48.0.insert.ext1103, 16
  %style.sroa.50.sroa.48.0.insert.insert1106 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.shift1104, %style.sroa.50.sroa.49.0.insert.shift1220
  %style.sroa.50.sroa.45.0.insert.ext988 = zext i8 %style.sroa.50.sroa.45.01929 to i32
  %style.sroa.50.sroa.45.0.insert.shift989 = shl nuw nsw i32 %style.sroa.50.sroa.45.0.insert.ext988, 8
  %style.sroa.50.sroa.45.0.insert.insert991 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.insert1106, %style.sroa.50.sroa.45.0.insert.shift989
  %style.sroa.50.sroa.0.0.insert.ext885 = zext i8 %style.sroa.50.sroa.0.01930 to i32
  %style.sroa.50.sroa.0.0.insert.insert887 = or disjoint i32 %style.sroa.50.sroa.45.0.insert.insert991, %style.sroa.50.sroa.0.0.insert.ext885
  %38 = shl i16 %n, 8
  %39 = or disjoint i16 %38, 1
  %_76.sroa.0.0.insert.insert = zext i16 %39 to i32
  br label %bb53

bb53:                                             ; preds = %bb50, %bb51, %bb52
  %_70.sroa.0.0 = phi i32 [ %_72.sroa.0.0.insert.insert, %bb52 ], [ %style.sroa.0.sroa.0.0.insert.insert1267, %bb51 ], [ %style.sroa.0.sroa.0.0.insert.insert1270, %bb50 ]
  %_70.sroa.5.0 = phi i32 [ %style.sroa.50.sroa.0.0.insert.insert884, %bb52 ], [ %_74.sroa.0.0.insert.insert, %bb51 ], [ %style.sroa.50.sroa.0.0.insert.insert887, %bb50 ]
  %_70.sroa.6.0 = phi i32 [ %style.sroa.72.sroa.0.0.insert.insert424, %bb52 ], [ %style.sroa.72.sroa.0.0.insert.insert427, %bb51 ], [ %_76.sroa.0.0.insert.insert, %bb50 ]
  %style.sroa.0.sroa.0.0.extract.trunc1271 = trunc i32 %_70.sroa.0.0 to i8
  %style.sroa.0.sroa.45.0.extract.shift1355 = lshr i32 %_70.sroa.0.0, 8
  %style.sroa.0.sroa.45.0.extract.trunc1356 = trunc i32 %style.sroa.0.sroa.45.0.extract.shift1355 to i8
  %style.sroa.0.sroa.48.0.extract.shift1470 = lshr i32 %_70.sroa.0.0, 16
  %style.sroa.0.sroa.48.0.extract.trunc1471 = trunc i32 %style.sroa.0.sroa.48.0.extract.shift1470 to i8
  %style.sroa.0.sroa.49.0.extract.shift1586 = lshr i32 %_70.sroa.0.0, 24
  %style.sroa.0.sroa.49.0.extract.trunc1587 = trunc nuw i32 %style.sroa.0.sroa.49.0.extract.shift1586 to i8
  %style.sroa.50.sroa.0.0.extract.trunc888 = trunc i32 %_70.sroa.5.0 to i8
  %style.sroa.50.sroa.45.0.extract.shift992 = lshr i32 %_70.sroa.5.0, 8
  %style.sroa.50.sroa.45.0.extract.trunc993 = trunc i32 %style.sroa.50.sroa.45.0.extract.shift992 to i8
  %style.sroa.50.sroa.48.0.extract.shift1107 = lshr i32 %_70.sroa.5.0, 16
  %style.sroa.50.sroa.48.0.extract.trunc1108 = trunc i32 %style.sroa.50.sroa.48.0.extract.shift1107 to i8
  %style.sroa.50.sroa.49.0.extract.shift1223 = lshr i32 %_70.sroa.5.0, 24
  %style.sroa.50.sroa.49.0.extract.trunc1224 = trunc nuw i32 %style.sroa.50.sroa.49.0.extract.shift1223 to i8
  %style.sroa.72.sroa.0.0.extract.trunc428 = trunc i32 %_70.sroa.6.0 to i8
  %style.sroa.72.sroa.48.0.extract.shift546 = lshr i32 %_70.sroa.6.0, 8
  %style.sroa.72.sroa.48.0.extract.trunc547 = trunc i32 %style.sroa.72.sroa.48.0.extract.shift546 to i8
  %style.sroa.72.sroa.49.0.extract.shift674 = lshr i32 %_70.sroa.6.0, 16
  %style.sroa.72.sroa.49.0.extract.trunc675 = trunc i32 %style.sroa.72.sroa.49.0.extract.shift674 to i8
  %style.sroa.72.sroa.50.0.extract.shift802 = lshr i32 %_70.sroa.6.0, 24
  %style.sroa.72.sroa.50.0.extract.trunc803 = trunc nuw i32 %style.sroa.72.sroa.50.0.extract.shift802 to i8
  br label %bb64

bb54:                                             ; preds = %bb30
  br i1 %g.sroa.0.1.off01878, label %bb55, label %bb56

bb57:                                             ; preds = %bb30
  %extract.t139 = trunc i16 %n to i8
  br label %bb63

bb55:                                             ; preds = %bb54
  switch i8 %color_target.sroa.0.11881, label %default.unreachable2062 [
    i8 0, label %bb60
    i8 1, label %bb59
    i8 2, label %bb58
  ]

bb56:                                             ; preds = %bb54
  %extract.t153 = trunc i16 %n to i8
  br label %bb63

bb60:                                             ; preds = %bb55
  %style.sroa.50.sroa.49.0.insert.ext1225 = zext i8 %style.sroa.50.sroa.49.01927 to i32
  %style.sroa.50.sroa.49.0.insert.shift1226 = shl nuw i32 %style.sroa.50.sroa.49.0.insert.ext1225, 24
  %style.sroa.50.sroa.48.0.insert.ext1109 = zext i8 %style.sroa.50.sroa.48.01928 to i32
  %style.sroa.50.sroa.48.0.insert.shift1110 = shl nuw nsw i32 %style.sroa.50.sroa.48.0.insert.ext1109, 16
  %style.sroa.50.sroa.48.0.insert.insert1112 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.shift1110, %style.sroa.50.sroa.49.0.insert.shift1226
  %style.sroa.50.sroa.45.0.insert.ext994 = zext i8 %style.sroa.50.sroa.45.01929 to i32
  %style.sroa.50.sroa.45.0.insert.shift995 = shl nuw nsw i32 %style.sroa.50.sroa.45.0.insert.ext994, 8
  %style.sroa.50.sroa.45.0.insert.insert997 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.insert1112, %style.sroa.50.sroa.45.0.insert.shift995
  %style.sroa.50.sroa.0.0.insert.ext889 = zext i8 %style.sroa.50.sroa.0.01930 to i32
  %style.sroa.50.sroa.0.0.insert.insert891 = or disjoint i32 %style.sroa.50.sroa.45.0.insert.insert997, %style.sroa.50.sroa.0.0.insert.ext889
  %style.sroa.72.sroa.50.0.insert.ext804 = zext i8 %style.sroa.72.sroa.50.01931 to i32
  %style.sroa.72.sroa.50.0.insert.shift805 = shl nuw i32 %style.sroa.72.sroa.50.0.insert.ext804, 24
  %style.sroa.72.sroa.49.0.insert.ext676 = zext i8 %style.sroa.72.sroa.49.01932 to i32
  %style.sroa.72.sroa.49.0.insert.shift677 = shl nuw nsw i32 %style.sroa.72.sroa.49.0.insert.ext676, 16
  %style.sroa.72.sroa.49.0.insert.insert679 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.shift677, %style.sroa.72.sroa.50.0.insert.shift805
  %style.sroa.72.sroa.48.0.insert.ext548 = zext i8 %style.sroa.72.sroa.48.01933 to i32
  %style.sroa.72.sroa.48.0.insert.shift549 = shl nuw nsw i32 %style.sroa.72.sroa.48.0.insert.ext548, 8
  %style.sroa.72.sroa.48.0.insert.insert551 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.insert679, %style.sroa.72.sroa.48.0.insert.shift549
  %style.sroa.72.sroa.0.0.insert.ext429 = zext i8 %style.sroa.72.sroa.0.01934 to i32
  %style.sroa.72.sroa.0.0.insert.insert431 = or disjoint i32 %style.sroa.72.sroa.48.0.insert.insert551, %style.sroa.72.sroa.0.0.insert.ext429
  %40 = and i16 %n, 255
  %_92.sroa.6.0.insert.ext = zext nneg i16 %40 to i32
  %_92.sroa.6.0.insert.shift = shl nuw i32 %_92.sroa.6.0.insert.ext, 24
  %_92.sroa.5.0.insert.ext = zext i8 %g.sroa.4.1.off01879 to i32
  %_92.sroa.5.0.insert.shift = shl nuw nsw i32 %_92.sroa.5.0.insert.ext, 16
  %_92.sroa.5.0.insert.insert = or disjoint i32 %_92.sroa.6.0.insert.shift, %_92.sroa.5.0.insert.shift
  %_92.sroa.4.0.insert.ext = zext i8 %r.sroa.4.1.off01877 to i32
  %_92.sroa.4.0.insert.shift = shl nuw nsw i32 %_92.sroa.4.0.insert.ext, 8
  %_92.sroa.4.0.insert.insert = or disjoint i32 %_92.sroa.5.0.insert.insert, %_92.sroa.4.0.insert.shift
  %_92.sroa.0.0.insert.insert = or disjoint i32 %_92.sroa.4.0.insert.insert, 2
  br label %bb61

bb59:                                             ; preds = %bb55
  %style.sroa.0.sroa.49.0.insert.ext1588 = zext i8 %style.sroa.0.sroa.49.01923 to i32
  %style.sroa.0.sroa.49.0.insert.shift1589 = shl nuw i32 %style.sroa.0.sroa.49.0.insert.ext1588, 24
  %style.sroa.0.sroa.48.0.insert.ext1472 = zext i8 %style.sroa.0.sroa.48.01924 to i32
  %style.sroa.0.sroa.48.0.insert.shift1473 = shl nuw nsw i32 %style.sroa.0.sroa.48.0.insert.ext1472, 16
  %style.sroa.0.sroa.48.0.insert.insert1475 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.shift1473, %style.sroa.0.sroa.49.0.insert.shift1589
  %style.sroa.0.sroa.45.0.insert.ext1357 = zext i8 %style.sroa.0.sroa.45.01925 to i32
  %style.sroa.0.sroa.45.0.insert.shift1358 = shl nuw nsw i32 %style.sroa.0.sroa.45.0.insert.ext1357, 8
  %style.sroa.0.sroa.45.0.insert.insert1360 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.insert1475, %style.sroa.0.sroa.45.0.insert.shift1358
  %style.sroa.0.sroa.0.0.insert.ext1272 = zext i8 %style.sroa.0.sroa.0.01926 to i32
  %style.sroa.0.sroa.0.0.insert.insert1274 = or disjoint i32 %style.sroa.0.sroa.45.0.insert.insert1360, %style.sroa.0.sroa.0.0.insert.ext1272
  %style.sroa.72.sroa.50.0.insert.ext808 = zext i8 %style.sroa.72.sroa.50.01931 to i32
  %style.sroa.72.sroa.50.0.insert.shift809 = shl nuw i32 %style.sroa.72.sroa.50.0.insert.ext808, 24
  %style.sroa.72.sroa.49.0.insert.ext680 = zext i8 %style.sroa.72.sroa.49.01932 to i32
  %style.sroa.72.sroa.49.0.insert.shift681 = shl nuw nsw i32 %style.sroa.72.sroa.49.0.insert.ext680, 16
  %style.sroa.72.sroa.49.0.insert.insert683 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.shift681, %style.sroa.72.sroa.50.0.insert.shift809
  %style.sroa.72.sroa.48.0.insert.ext552 = zext i8 %style.sroa.72.sroa.48.01933 to i32
  %style.sroa.72.sroa.48.0.insert.shift553 = shl nuw nsw i32 %style.sroa.72.sroa.48.0.insert.ext552, 8
  %style.sroa.72.sroa.48.0.insert.insert555 = or disjoint i32 %style.sroa.72.sroa.49.0.insert.insert683, %style.sroa.72.sroa.48.0.insert.shift553
  %style.sroa.72.sroa.0.0.insert.ext432 = zext i8 %style.sroa.72.sroa.0.01934 to i32
  %style.sroa.72.sroa.0.0.insert.insert434 = or disjoint i32 %style.sroa.72.sroa.48.0.insert.insert555, %style.sroa.72.sroa.0.0.insert.ext432
  %_88.mask189 = and i16 %n, 255
  %_94.sroa.6.0.insert.ext = zext nneg i16 %_88.mask189 to i32
  %_94.sroa.6.0.insert.shift = shl nuw i32 %_94.sroa.6.0.insert.ext, 24
  %_94.sroa.5.0.insert.ext = zext i8 %g.sroa.4.1.off01879 to i32
  %_94.sroa.5.0.insert.shift = shl nuw nsw i32 %_94.sroa.5.0.insert.ext, 16
  %_94.sroa.5.0.insert.insert = or disjoint i32 %_94.sroa.6.0.insert.shift, %_94.sroa.5.0.insert.shift
  %_94.sroa.4.0.insert.ext = zext i8 %r.sroa.4.1.off01877 to i32
  %_94.sroa.4.0.insert.shift = shl nuw nsw i32 %_94.sroa.4.0.insert.ext, 8
  %_94.sroa.4.0.insert.insert = or disjoint i32 %_94.sroa.5.0.insert.insert, %_94.sroa.4.0.insert.shift
  %_94.sroa.0.0.insert.insert = or disjoint i32 %_94.sroa.4.0.insert.insert, 2
  br label %bb61

bb58:                                             ; preds = %bb55
  %style.sroa.0.sroa.49.0.insert.ext1592 = zext i8 %style.sroa.0.sroa.49.01923 to i32
  %style.sroa.0.sroa.49.0.insert.shift1593 = shl nuw i32 %style.sroa.0.sroa.49.0.insert.ext1592, 24
  %style.sroa.0.sroa.48.0.insert.ext1476 = zext i8 %style.sroa.0.sroa.48.01924 to i32
  %style.sroa.0.sroa.48.0.insert.shift1477 = shl nuw nsw i32 %style.sroa.0.sroa.48.0.insert.ext1476, 16
  %style.sroa.0.sroa.48.0.insert.insert1479 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.shift1477, %style.sroa.0.sroa.49.0.insert.shift1593
  %style.sroa.0.sroa.45.0.insert.ext1361 = zext i8 %style.sroa.0.sroa.45.01925 to i32
  %style.sroa.0.sroa.45.0.insert.shift1362 = shl nuw nsw i32 %style.sroa.0.sroa.45.0.insert.ext1361, 8
  %style.sroa.0.sroa.45.0.insert.insert1364 = or disjoint i32 %style.sroa.0.sroa.48.0.insert.insert1479, %style.sroa.0.sroa.45.0.insert.shift1362
  %style.sroa.0.sroa.0.0.insert.ext1275 = zext i8 %style.sroa.0.sroa.0.01926 to i32
  %style.sroa.0.sroa.0.0.insert.insert1277 = or disjoint i32 %style.sroa.0.sroa.45.0.insert.insert1364, %style.sroa.0.sroa.0.0.insert.ext1275
  %style.sroa.50.sroa.49.0.insert.ext1229 = zext i8 %style.sroa.50.sroa.49.01927 to i32
  %style.sroa.50.sroa.49.0.insert.shift1230 = shl nuw i32 %style.sroa.50.sroa.49.0.insert.ext1229, 24
  %style.sroa.50.sroa.48.0.insert.ext1113 = zext i8 %style.sroa.50.sroa.48.01928 to i32
  %style.sroa.50.sroa.48.0.insert.shift1114 = shl nuw nsw i32 %style.sroa.50.sroa.48.0.insert.ext1113, 16
  %style.sroa.50.sroa.48.0.insert.insert1116 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.shift1114, %style.sroa.50.sroa.49.0.insert.shift1230
  %style.sroa.50.sroa.45.0.insert.ext998 = zext i8 %style.sroa.50.sroa.45.01929 to i32
  %style.sroa.50.sroa.45.0.insert.shift999 = shl nuw nsw i32 %style.sroa.50.sroa.45.0.insert.ext998, 8
  %style.sroa.50.sroa.45.0.insert.insert1001 = or disjoint i32 %style.sroa.50.sroa.48.0.insert.insert1116, %style.sroa.50.sroa.45.0.insert.shift999
  %style.sroa.50.sroa.0.0.insert.ext892 = zext i8 %style.sroa.50.sroa.0.01930 to i32
  %style.sroa.50.sroa.0.0.insert.insert894 = or disjoint i32 %style.sroa.50.sroa.45.0.insert.insert1001, %style.sroa.50.sroa.0.0.insert.ext892
  %_88.mask = and i16 %n, 255
  %_96.sroa.6.0.insert.ext = zext nneg i16 %_88.mask to i32
  %_96.sroa.6.0.insert.shift = shl nuw i32 %_96.sroa.6.0.insert.ext, 24
  %_96.sroa.5.0.insert.ext = zext i8 %g.sroa.4.1.off01879 to i32
  %_96.sroa.5.0.insert.shift = shl nuw nsw i32 %_96.sroa.5.0.insert.ext, 16
  %_96.sroa.5.0.insert.insert = or disjoint i32 %_96.sroa.6.0.insert.shift, %_96.sroa.5.0.insert.shift
  %_96.sroa.4.0.insert.ext = zext i8 %r.sroa.4.1.off01877 to i32
  %_96.sroa.4.0.insert.shift = shl nuw nsw i32 %_96.sroa.4.0.insert.ext, 8
  %_96.sroa.4.0.insert.insert = or disjoint i32 %_96.sroa.5.0.insert.insert, %_96.sroa.4.0.insert.shift
  %_96.sroa.0.0.insert.insert = or disjoint i32 %_96.sroa.4.0.insert.insert, 2
  br label %bb61

bb61:                                             ; preds = %bb58, %bb59, %bb60
  %_90.sroa.0.0 = phi i32 [ %_92.sroa.0.0.insert.insert, %bb60 ], [ %style.sroa.0.sroa.0.0.insert.insert1274, %bb59 ], [ %style.sroa.0.sroa.0.0.insert.insert1277, %bb58 ]
  %_90.sroa.5.0 = phi i32 [ %style.sroa.50.sroa.0.0.insert.insert891, %bb60 ], [ %_94.sroa.0.0.insert.insert, %bb59 ], [ %style.sroa.50.sroa.0.0.insert.insert894, %bb58 ]
  %_90.sroa.6.0 = phi i32 [ %style.sroa.72.sroa.0.0.insert.insert431, %bb60 ], [ %style.sroa.72.sroa.0.0.insert.insert434, %bb59 ], [ %_96.sroa.0.0.insert.insert, %bb58 ]
  %style.sroa.0.sroa.0.0.extract.trunc1278 = trunc i32 %_90.sroa.0.0 to i8
  %style.sroa.0.sroa.45.0.extract.shift1365 = lshr i32 %_90.sroa.0.0, 8
  %style.sroa.0.sroa.45.0.extract.trunc1366 = trunc i32 %style.sroa.0.sroa.45.0.extract.shift1365 to i8
  %style.sroa.0.sroa.48.0.extract.shift1480 = lshr i32 %_90.sroa.0.0, 16
  %style.sroa.0.sroa.48.0.extract.trunc1481 = trunc i32 %style.sroa.0.sroa.48.0.extract.shift1480 to i8
  %style.sroa.0.sroa.49.0.extract.shift1596 = lshr i32 %_90.sroa.0.0, 24
  %style.sroa.0.sroa.49.0.extract.trunc1597 = trunc nuw i32 %style.sroa.0.sroa.49.0.extract.shift1596 to i8
  %style.sroa.50.sroa.0.0.extract.trunc895 = trunc i32 %_90.sroa.5.0 to i8
  %style.sroa.50.sroa.45.0.extract.shift1002 = lshr i32 %_90.sroa.5.0, 8
  %style.sroa.50.sroa.45.0.extract.trunc1003 = trunc i32 %style.sroa.50.sroa.45.0.extract.shift1002 to i8
  %style.sroa.50.sroa.48.0.extract.shift1117 = lshr i32 %_90.sroa.5.0, 16
  %style.sroa.50.sroa.48.0.extract.trunc1118 = trunc i32 %style.sroa.50.sroa.48.0.extract.shift1117 to i8
  %style.sroa.50.sroa.49.0.extract.shift1233 = lshr i32 %_90.sroa.5.0, 24
  %style.sroa.50.sroa.49.0.extract.trunc1234 = trunc nuw i32 %style.sroa.50.sroa.49.0.extract.shift1233 to i8
  %style.sroa.72.sroa.0.0.extract.trunc435 = trunc i32 %_90.sroa.6.0 to i8
  %style.sroa.72.sroa.48.0.extract.shift556 = lshr i32 %_90.sroa.6.0, 8
  %style.sroa.72.sroa.48.0.extract.trunc557 = trunc i32 %style.sroa.72.sroa.48.0.extract.shift556 to i8
  %style.sroa.72.sroa.49.0.extract.shift684 = lshr i32 %_90.sroa.6.0, 16
  %style.sroa.72.sroa.49.0.extract.trunc685 = trunc i32 %style.sroa.72.sroa.49.0.extract.shift684 to i8
  %style.sroa.72.sroa.50.0.extract.shift812 = lshr i32 %_90.sroa.6.0, 24
  %style.sroa.72.sroa.50.0.extract.trunc813 = trunc nuw i32 %style.sroa.72.sroa.50.0.extract.shift812 to i8
  br label %bb64

bb29:                                             ; preds = %bb24
  %41 = and i16 %style.sroa.89.11875, -9
  br label %bb63

bb28:                                             ; preds = %bb24
  %42 = and i16 %style.sroa.89.11875, -25
  %43 = or disjoint i16 %42, 16
  br label %bb63

bb27:                                             ; preds = %bb24
  %44 = and i16 %style.sroa.89.11875, -41
  %45 = or disjoint i16 %44, 32
  br label %bb63

bb26:                                             ; preds = %bb24
  %46 = and i16 %style.sroa.89.11875, -73
  %47 = or disjoint i16 %46, 64
  br label %bb63

bb25:                                             ; preds = %bb24
  %48 = and i16 %style.sroa.89.11875, -137
  %49 = or disjoint i16 %48, 128
  br label %bb63
}

; <anstream::adapter::wincon::WinconCapture as anstyle_parse::Perform>::print
; Function Attrs: uwtable
define void @_RNvXs1_NtNtCsen8ds7JR2I0_8anstream7adapter6winconNtB5_13WinconCaptureNtCsofRcISb9et_13anstyle_parse7Perform5print(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #0 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !96)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i = load i64, ptr %0, align 8, !alias.scope !96, !noundef !2
  %_14.i = icmp sgt i64 %len.i, -1
  tail call void @llvm.assume(i1 %_14.i)
  %_16.i = icmp samesign ult i32 %c, 128
  br i1 %_16.i, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %start
  %_17.i = icmp samesign ult i32 %c, 2048
  br i1 %_17.i, label %bb2.i, label %bb4.i

bb4.i:                                            ; preds = %bb3.i
  %_18.i = icmp samesign ult i32 %c, 65536
  %..i = select i1 %_18.i, i64 3, i64 4
  br label %bb2.i

bb2.i:                                            ; preds = %bb4.i, %bb3.i, %start
  %ch_len.sroa.0.0.i = phi i64 [ 1, %start ], [ %..i, %bb4.i ], [ 2, %bb3.i ]
  %self2.i.i = load i64, ptr %self, align 8, !range !5, !alias.scope !99, !noundef !2
  %_9.i.i = sub nsw i64 %self2.i.i, %len.i
  %_7.i.i = icmp ugt i64 %ch_len.sroa.0.0.i, %_9.i.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit.i, !prof !43

bb1.i.i:                                          ; preds = %bb2.i
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsen8ds7JR2I0_8anstream(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i, i64 noundef range(i64 0, -9223372036854775808) %ch_len.sroa.0.0.i)
  %count.pre.i = load i64, ptr %0, align 8, !alias.scope !96
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit.i: ; preds = %bb1.i.i, %bb2.i
  %count.i = phi i64 [ %len.i, %bb2.i ], [ %count.pre.i, %bb1.i.i ]
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20.i = load ptr, ptr %1, align 8, !alias.scope !96, !nonnull !2, !noundef !2
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  br i1 %_16.i, label %bb12.i.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit.i
  %_27.i.i = icmp samesign ult i32 %c, 2048
  %2 = trunc i32 %c to i8
  %_5.i.i = and i8 %2, 63
  %last1.i.i = or disjoint i8 %_5.i.i, -128
  %_10.i.i = lshr i32 %c, 6
  %3 = trunc i32 %_10.i.i to i8
  %_8.i.i = and i8 %3, 63
  %last2.i.i = or disjoint i8 %_8.i.i, -128
  %_14.i.i = lshr i32 %c, 12
  %4 = trunc i32 %_14.i.i to i8
  %_12.i.i = and i8 %4, 63
  %last3.i.i = or disjoint i8 %_12.i.i, -128
  %_18.i.i = lshr i32 %c, 18
  %_16.i.i = trunc nuw nsw i32 %_18.i.i to i8
  %last4.i.i = or disjoint i8 %_16.i.i, -16
  br i1 %_27.i.i, label %bb1.i2.i, label %bb2.i.i

bb12.i.i:                                         ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream.exit.i
  %5 = trunc nuw nsw i32 %c to i8
  store i8 %5, ptr %_8.i, align 1, !noalias !96
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb1.i2.i:                                         ; preds = %bb7.i.i
  %6 = or disjoint i8 %3, -64
  store i8 %6, ptr %_8.i, align 1, !noalias !96
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !noalias !96
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb2.i.i:                                          ; preds = %bb7.i.i
  %_28.i.i = icmp samesign ult i32 %c, 65536
  br i1 %_28.i.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %7 = or disjoint i8 %4, -32
  store i8 %7, ptr %_8.i, align 1, !noalias !96
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !noalias !96
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 1, !noalias !96
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_8.i, align 1, !noalias !96
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !noalias !96
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 1, !noalias !96
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !noalias !96
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit: ; preds = %bb12.i.i, %bb1.i2.i, %bb3.i.i, %bb4.i.i
  %new_len.i = add nuw i64 %ch_len.sroa.0.0.i, %len.i
  store i64 %new_len.i, ptr %0, align 8, !alias.scope !96
  ret void
}

; <anstream::adapter::wincon::WinconCapture as anstyle_parse::Perform>::execute
; Function Attrs: uwtable
define void @_RNvXs1_NtNtCsen8ds7JR2I0_8anstream7adapter6winconNtB5_13WinconCaptureNtCsofRcISb9et_13anstyle_parse7Perform7execute(ptr noalias noundef align 8 captures(none) dereferenceable(56) %self, i8 noundef %byte) unnamed_addr #0 {
start:
  switch i8 %byte, label %bb2 [
    i8 9, label %bb4
    i8 10, label %bb4
    i8 12, label %bb4
    i8 13, label %bb4
    i8 32, label %bb4
  ]

bb4:                                              ; preds = %start, %start, %start, %start, %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !102)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i = load i64, ptr %0, align 8, !alias.scope !102, !noundef !2
  %_14.i = icmp sgt i64 %len.i, -1
  tail call void @llvm.assume(i1 %_14.i)
  %self2.i.i = load i64, ptr %self, align 8, !range !5, !alias.scope !105, !noundef !2
  %_7.i.i = icmp eq i64 %self2.i.i, %len.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit, !prof !43

bb1.i.i:                                          ; preds = %bb4
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECsen8ds7JR2I0_8anstream(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i, i64 noundef range(i64 0, -9223372036854775808) 1)
  %count.pre.i = load i64, ptr %0, align 8, !alias.scope !102
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit: ; preds = %bb1.i.i, %bb4
  %count.i = phi i64 [ %len.i, %bb4 ], [ %count.pre.i, %bb1.i.i ]
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20.i = load ptr, ptr %1, align 8, !alias.scope !102, !nonnull !2, !noundef !2
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  store i8 %byte, ptr %_8.i, align 1, !noalias !102
  %new_len.i = add nuw i64 %len.i, 1
  store i64 %new_len.i, ptr %0, align 8, !alias.scope !102
  br label %bb2

bb2:                                              ; preds = %start, %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit
  ret void
}

; <anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}> as core::fmt::Write>::write_str
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtCsen8ds7JR2I0_8anstream3fmtINtB4_7AdapterNCNvNtB6_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write9write_strB6_(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %iter.i.i = alloca [32 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !108)
  %_3.0.i = load ptr, ptr %self, align 8, !alias.scope !108, !noalias !111, !nonnull !2, !align !59, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1.i = load ptr, ptr %0, align 8, !alias.scope !108, !noalias !111, !nonnull !2, !align !4, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4.i = load ptr, ptr %1, align 8, !alias.scope !108, !noalias !111, !nonnull !2, !align !60, !noundef !2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !113)
  %_12.i.i = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter.i.i), !noalias !116
  store ptr %s.0, ptr %iter.i.i, align 8, !noalias !116
  %_4.sroa.2.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 8
  store i64 %s.1, ptr %_4.sroa.2.0.iter.sroa_idx.i.i, align 8, !noalias !116
  %_4.sroa.3.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 16
  store ptr %_12.i.i, ptr %_4.sroa.3.0.iter.sroa_idx.i.i, align 8, !noalias !116
  %_4.sroa.4.0.iter.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %iter.i.i, i64 24
  store ptr %_4.i, ptr %_4.sroa.4.0.iter.sroa_idx.i.i, align 8, !noalias !116
  %2 = getelementptr inbounds nuw i8, ptr %_3.1.i, i64 56
  %3 = load ptr, ptr %2, align 8, !alias.scope !113, !noalias !119, !nonnull !2
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb3.i.i, %start
  %_15.i.i = load ptr, ptr %_4.sroa.3.0.iter.sroa_idx.i.i, align 8, !noalias !116, !nonnull !2, !align !59, !noundef !2
  %_16.i.i = load ptr, ptr %_4.sroa.4.0.iter.sroa_idx.i.i, align 8, !noalias !116, !nonnull !2, !align !60, !noundef !2
; call anstream::adapter::strip::next_bytes
  %4 = call fastcc { ptr, i64 } @_RNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes(ptr noalias noundef align 8 dereferenceable(16) %iter.i.i, ptr noalias noundef align 1 dereferenceable(1) %_15.i.i, ptr noalias noundef align 4 dereferenceable(8) %_16.i.i) #24, !noalias !120
  %5 = extractvalue { ptr, i64 } %4, 0
  %.not.i.i.not.not.not.not.not = icmp ne ptr %5, null
  br i1 %.not.i.i.not.not.not.not.not, label %bb3.i.i, label %_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_.exit.thread

_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_.exit.thread: ; preds = %bb1.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter.i.i), !noalias !116
  br label %bb7

bb3.i.i:                                          ; preds = %bb1.i.i
  %6 = extractvalue { ptr, i64 } %4, 1
  %7 = tail call noundef ptr %3(ptr noundef nonnull align 1 %_3.0.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %5, i64 noundef %6) #24, !noalias !120
  %.not2.i.i = icmp eq ptr %7, null
  br i1 %.not2.i.i, label %bb1.i.i, label %bb3

bb3:                                              ; preds = %bb3.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter.i.i), !noalias !116
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %.val = load ptr, ptr %8, align 8, !noundef !2
  %bits.i.i.i.i.i = ptrtoint ptr %.val to i64
  %_5.i.i.i.i.i = and i64 %bits.i.i.i.i.i, 3
  %switch.i.i.i.i = icmp eq i64 %_5.i.i.i.i.i, 1
  br i1 %switch.i.i.i.i, label %bb2.i2.i.i.i.i, label %bb5, !prof !3

bb2.i2.i.i.i.i:                                   ; preds = %bb3
  %9 = getelementptr i8, ptr %.val, i64 -1
  %10 = icmp ne ptr %9, null
  tail call void @llvm.assume(i1 %10)
  %_6.val.i.i.i.i.i.i = load ptr, ptr %9, align 8
  %11 = getelementptr i8, ptr %.val, i64 7
  %_6.val1.i.i.i.i.i.i = load ptr, ptr %11, align 8, !nonnull !2, !align !4, !noundef !2
  %12 = load ptr, ptr %_6.val1.i.i.i.i.i.i, align 8, !invariant.load !2
  %.not.i.i.i.i.i.i.i.i = icmp eq ptr %12, null
  br i1 %.not.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i, label %is_not_null.i.i.i.i.i.i.i.i

is_not_null.i.i.i.i.i.i.i.i:                      ; preds = %bb2.i2.i.i.i.i
  %13 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %13)
  invoke void %12(ptr noundef nonnull %_6.val.i.i.i.i.i.i)
          to label %bb3.i.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i:                              ; preds = %is_not_null.i.i.i.i.i.i.i.i, %bb2.i2.i.i.i.i
  %14 = icmp ne ptr %_6.val.i.i.i.i.i.i, null
  tail call void @llvm.assume(i1 %14)
  %15 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %16 = load i64, ptr %15, align 8, !range !5, !invariant.load !2
  %17 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %18 = load i64, ptr %17, align 8, !range !6, !invariant.load !2
  %19 = add i64 %18, -1
  %20 = icmp sgt i64 %19, -1
  tail call void @llvm.assume(i1 %20)
  %21 = icmp eq i64 %16, 0
  br i1 %21, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i: ; preds = %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %16, i64 noundef range(i64 1, -9223372036854775807) %18) #22
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i

cleanup.i.i.i.i.i.i.i.i:                          ; preds = %is_not_null.i.i.i.i.i.i.i.i
  %22 = landingpad { ptr, i32 }
          cleanup
  %23 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 8
  %24 = load i64, ptr %23, align 8, !range !5, !invariant.load !2
  %25 = getelementptr inbounds nuw i8, ptr %_6.val1.i.i.i.i.i.i, i64 16
  %26 = load i64, ptr %25, align 8, !range !6, !invariant.load !2
  %27 = add i64 %26, -1
  %28 = icmp sgt i64 %27, -1
  tail call void @llvm.assume(i1 %28)
  %29 = icmp eq i64 %24, 0
  br i1 %29, label %bb1.i.i.i.i.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i: ; preds = %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.val.i.i.i.i.i.i, i64 noundef %24, i64 noundef range(i64 1, -9223372036854775807) %26) #22
  br label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #22
  store ptr %7, ptr %8, align 8
  resume { ptr, i32 } %22

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %9, i64 noundef 24, i64 noundef 8) #22
  br label %bb5

bb7:                                              ; preds = %_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_.exit.thread, %bb5
  ret i1 %.not.i.i.not.not.not.not.not

bb5:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCs5sEH5CPMdak_3std2io5error6CustomEECsen8ds7JR2I0_8anstream.exit.i.i.i.i.i, %bb3
  store ptr %7, ptr %8, align 8
  br label %bb7
}

; <std::io::stdio::Stdout as anstream::stream::IsTerminal>::is_terminal
; Function Attrs: inlinehint nounwind uwtable
define internal noundef zeroext i1 @_RNvXsd_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutNtB5_10IsTerminal11is_terminal(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #6 {
start:
  %_3.i = tail call noundef i32 @isatty(i32 noundef 1) #22
  %_0.i = icmp ne i32 %_3.i, 0
  ret i1 %_0.i
}

; <std::io::stdio::Stdout as std::io::Write>::is_write_vectored
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write17is_write_vectored(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
; call <std::io::stdio::Stdout>::lock
  %0 = tail call noundef nonnull align 8 ptr @_RNvMs9_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6Stdout4lock(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self)
  %_7.i = getelementptr inbounds nuw i8, ptr %0, i64 24
  %_9.i.i = load i64, ptr %_7.i, align 8, !noundef !2
  %1 = icmp eq i64 %_9.i.i, 0
  br i1 %1, label %bb6.i, label %bb1.i.i, !prof !121

bb1.i.i:                                          ; preds = %start
; invoke core::cell::panic_already_borrowed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core4cell22panic_already_borrowed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_b5a27d2ad8ac297ce0dc342bcb05cc9c) #25
          to label %.noexc.i unwind label %cleanup.i

.noexc.i:                                         ; preds = %bb1.i.i
  unreachable

cleanup.i:                                        ; preds = %bb1.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
  %_14.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 16
  %3 = load i32, ptr %_14.i.i.i.i, align 8, !noundef !2
  %4 = add i32 %3, -1
  store i32 %4, ptr %_14.i.i.i.i, align 8
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb1.i.i.i.i, label %bb4.i

bb1.i.i.i.i:                                      ; preds = %cleanup.i
  %_21.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 8
  store atomic i64 0, ptr %_21.i.i.i.i monotonic, align 8
  %6 = load atomic ptr, ptr %0 monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %6)
          to label %bb4.i unwind label %terminate.i

bb6.i:                                            ; preds = %start
  %_14.i.i.i3.i = getelementptr inbounds nuw i8, ptr %0, i64 16
  %7 = load i32, ptr %_14.i.i.i3.i, align 8, !noundef !2
  %8 = add i32 %7, -1
  store i32 %8, ptr %_14.i.i.i3.i, align 8
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %bb1.i.i.i4.i, label %_RNvXse_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StdoutNtB7_5Write17is_write_vectored.exit

bb1.i.i.i4.i:                                     ; preds = %bb6.i
  %_21.i.i.i5.i = getelementptr inbounds nuw i8, ptr %0, i64 8
  store atomic i64 0, ptr %_21.i.i.i5.i monotonic, align 8
  %10 = load atomic ptr, ptr %0 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %10)
  br label %_RNvXse_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StdoutNtB7_5Write17is_write_vectored.exit

terminate.i:                                      ; preds = %bb1.i.i.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #27
  unreachable

bb4.i:                                            ; preds = %bb1.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %2

_RNvXse_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StdoutNtB7_5Write17is_write_vectored.exit: ; preds = %bb6.i, %bb1.i.i.i4.i
  ret i1 true
}

; <std::io::stdio::Stderr as anstream::stream::IsTerminal>::is_terminal
; Function Attrs: inlinehint nounwind uwtable
define internal noundef zeroext i1 @_RNvXsf_NtCsen8ds7JR2I0_8anstream6streamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrNtB5_10IsTerminal11is_terminal(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #6 {
start:
  %_3.i = tail call noundef i32 @isatty(i32 noundef 2) #22
  %_0.i = icmp ne i32 %_3.i, 0
  ret i1 %_0.i
}

; <std::io::stdio::Stderr as std::io::Write>::is_write_vectored
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write17is_write_vectored(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
; call <std::io::stdio::Stderr>::lock
  %0 = tail call noundef nonnull align 8 ptr @_RNvMsj_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6Stderr4lock(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self)
  %_7.i = getelementptr inbounds nuw i8, ptr %0, i64 24
  %_9.i.i = load i64, ptr %_7.i, align 8, !noundef !2
  %1 = icmp eq i64 %_9.i.i, 0
  br i1 %1, label %bb6.i, label %bb1.i.i, !prof !121

bb1.i.i:                                          ; preds = %start
; invoke core::cell::panic_already_borrowed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core4cell22panic_already_borrowed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_66d59cd97f643b0f3a5d9f737f46b52c) #25
          to label %.noexc.i unwind label %cleanup.i

.noexc.i:                                         ; preds = %bb1.i.i
  unreachable

cleanup.i:                                        ; preds = %bb1.i.i
  %2 = landingpad { ptr, i32 }
          cleanup
  %_14.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 16
  %3 = load i32, ptr %_14.i.i.i.i, align 8, !noundef !2
  %4 = add i32 %3, -1
  store i32 %4, ptr %_14.i.i.i.i, align 8
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb1.i.i.i.i, label %bb4.i

bb1.i.i.i.i:                                      ; preds = %cleanup.i
  %_21.i.i.i.i = getelementptr inbounds nuw i8, ptr %0, i64 8
  store atomic i64 0, ptr %_21.i.i.i.i monotonic, align 8
  %6 = load atomic ptr, ptr %0 monotonic, align 8
; invoke <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  invoke void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %6)
          to label %bb4.i unwind label %terminate.i

bb6.i:                                            ; preds = %start
  %_14.i.i.i3.i = getelementptr inbounds nuw i8, ptr %0, i64 16
  %7 = load i32, ptr %_14.i.i.i3.i, align 8, !noundef !2
  %8 = add i32 %7, -1
  store i32 %8, ptr %_14.i.i.i3.i, align 8
  %9 = icmp eq i32 %8, 0
  br i1 %9, label %bb1.i.i.i4.i, label %_RNvXso_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StderrNtB7_5Write17is_write_vectored.exit

bb1.i.i.i4.i:                                     ; preds = %bb6.i
  %_21.i.i.i5.i = getelementptr inbounds nuw i8, ptr %0, i64 8
  store atomic i64 0, ptr %_21.i.i.i5.i monotonic, align 8
  %10 = load atomic ptr, ptr %0 monotonic, align 8
; call <std::sys::pal::unix::sync::mutex::Mutex>::unlock
  tail call void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8 %10)
  br label %_RNvXso_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StderrNtB7_5Write17is_write_vectored.exit

terminate.i:                                      ; preds = %bb1.i.i.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #27
  unreachable

bb4.i:                                            ; preds = %bb1.i.i.i.i, %cleanup.i
  resume { ptr, i32 } %2

_RNvXso_NtNtCs5sEH5CPMdak_3std2io5stdioRNtB5_6StderrNtB7_5Write17is_write_vectored.exit: ; preds = %bb6.i, %bb1.i.i.i4.i
  ret i1 true
}

; <anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}> as core::fmt::Write>::write_char
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtB7_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write10write_charB7_(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #0 {
start:
  %_6 = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_6)
  store i32 0, ptr %_6, align 4
  %_11.i = icmp samesign ult i32 %c, 128
  br i1 %_11.i, label %bb12.i.i, label %bb5.i

bb5.i:                                            ; preds = %start
  %_12.i = icmp samesign ult i32 %c, 2048
  %0 = trunc i32 %c to i8
  %_5.i.i = and i8 %0, 63
  %last1.i.i = or disjoint i8 %_5.i.i, -128
  %_10.i.i = lshr i32 %c, 6
  %1 = trunc i32 %_10.i.i to i8
  %_8.i.i = and i8 %1, 63
  %last2.i.i = or disjoint i8 %_8.i.i, -128
  %_14.i.i = lshr i32 %c, 12
  %2 = trunc i32 %_14.i.i to i8
  %_12.i.i = and i8 %2, 63
  %last3.i.i = or disjoint i8 %_12.i.i, -128
  %_18.i.i = lshr i32 %c, 18
  %_16.i.i = trunc nuw nsw i32 %_18.i.i to i8
  %last4.i.i = or disjoint i8 %_16.i.i, -16
  br i1 %_12.i, label %bb1.i.i, label %bb2.i.i

bb12.i.i:                                         ; preds = %start
  %3 = trunc nuw nsw i32 %c to i8
  store i8 %3, ptr %_6, align 4, !alias.scope !122
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb1.i.i:                                          ; preds = %bb5.i
  %4 = or disjoint i8 %1, -64
  store i8 %4, ptr %_6, align 4, !alias.scope !122
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !alias.scope !122
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb2.i.i:                                          ; preds = %bb5.i
  %_13.i = icmp samesign ult i32 %c, 65536
  br i1 %_13.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %5 = or disjoint i8 %2, -32
  store i8 %5, ptr %_6, align 4, !alias.scope !122
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !alias.scope !122
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 2, !alias.scope !122
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_6, align 4, !alias.scope !122
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !alias.scope !122
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 2, !alias.scope !122
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !alias.scope !122
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit: ; preds = %bb12.i.i, %bb1.i.i, %bb3.i.i, %bb4.i.i
  %len.sroa.0.04.i = phi i64 [ 1, %bb12.i.i ], [ 2, %bb1.i.i ], [ 3, %bb3.i.i ], [ 4, %bb4.i.i ]
; call <anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}> as core::fmt::Write>::write_str
  %_0 = call noundef zeroext i1 @_RNvXs_NtCsen8ds7JR2I0_8anstream3fmtINtB4_7AdapterNCNvNtB6_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write9write_strB6_(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_6, i64 noundef %len.sroa.0.04.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_6)
  ret i1 %_0
}

; <anstream::fmt::Adapter<anstream::strip::write_fmt::{closure#0}> as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYINtNtCsen8ds7JR2I0_8anstream3fmt7AdapterNCNvNtB7_5strip9write_fmt0ENtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtB7_(ptr noalias noundef align 8 dereferenceable(32) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #0 {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.0, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #7

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #8

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #9

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #9

; std::env::_var_os
; Function Attrs: uwtable
declare void @_RNvNtCs5sEH5CPMdak_3std3env7__var_os(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(address) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #10

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; Function Attrs: nounwind uwtable
declare noundef i32 @isatty(i32 noundef) unnamed_addr #8

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #12

; std::io::stdio::stdout
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvNtNtCs5sEH5CPMdak_3std2io5stdio6stdout() unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #13

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #14

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #15

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #16

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #8

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #17

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; <std::io::error::Error>::new::<&str>
; Function Attrs: noinline uwtable
declare noundef nonnull ptr @_RINvMs5_NtNtCs5sEH5CPMdak_3std2io5errorNtB6_5Error3newReEBa_(i8 noundef range(i8 0, 42), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #18

; <std::io::stdio::Stderr as std::io::Write>::write
; Function Attrs: uwtable
declare { i64, ptr } @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5write(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <std::io::stdio::Stderr as std::io::Write>::write_vectored
; Function Attrs: uwtable
declare { i64, ptr } @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write14write_vectored(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance), i64 noundef range(i64 0, 576460752303423488)) unnamed_addr #0

; <std::io::stdio::Stderr as std::io::Write>::flush
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write5flush(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #0

; <std::io::stdio::Stderr as std::io::Write>::write_all
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_all(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <std::io::stdio::Stderr as std::io::Write>::write_all_vectored
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write18write_all_vectored(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull align 8, i64 noundef range(i64 0, 576460752303423488)) unnamed_addr #0

; <std::io::stdio::Stderr as std::io::Write>::write_fmt
; Function Attrs: uwtable
declare noundef ptr @_RNvXsn_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StderrNtB7_5Write9write_fmt(ptr noalias noundef align 8 dereferenceable(8), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::write
; Function Attrs: uwtable
declare { i64, ptr } @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5write(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::write_vectored
; Function Attrs: uwtable
declare { i64, ptr } @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write14write_vectored(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance), i64 noundef range(i64 0, 576460752303423488)) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::flush
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write5flush(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::write_all
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_all(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::write_all_vectored
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write18write_all_vectored(ptr noalias noundef align 8 dereferenceable(8), ptr noalias noundef nonnull align 8, i64 noundef range(i64 0, 576460752303423488)) unnamed_addr #0

; <std::io::stdio::Stdout as std::io::Write>::write_fmt
; Function Attrs: uwtable
declare noundef ptr @_RNvXsd_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6StdoutNtB7_5Write9write_fmt(ptr noalias noundef align 8 dereferenceable(8), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; core::cell::panic_already_borrowed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core4cell22panic_already_borrowed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; <colorchoice::ColorChoice>::global
; Function Attrs: uwtable
declare noundef range(i8 0, 4) i8 @_RNvMCs314fcNNe6oa_11colorchoiceNtB2_11ColorChoice6global() unnamed_addr #0

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #11

; <anstyle_parse::params::ParamsIter as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
declare { ptr, i64 } @_RNvXs1_NtCsofRcISb9et_13anstyle_parse6paramsNtB5_10ParamsIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #19

; <std::sys::pal::unix::sync::mutex::Mutex>::unlock
; Function Attrs: uwtable
declare void @_RNvMNtNtNtNtNtCs5sEH5CPMdak_3std3sys3pal4unix4sync5mutexNtB2_5Mutex6unlock(ptr noundef nonnull align 8) unnamed_addr #0

; <std::io::stdio::Stdout>::lock
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMs9_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6Stdout4lock(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8)) unnamed_addr #0

; <std::io::stdio::Stderr>::lock
; Function Attrs: uwtable
declare noundef nonnull align 8 ptr @_RNvMsj_NtNtCs5sEH5CPMdak_3std2io5stdioNtB5_6Stderr4lock(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8)) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #20

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #21

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { inlinehint nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #8 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #10 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #14 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #17 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #20 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #21 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #22 = { nounwind }
attributes #23 = { noreturn }
attributes #24 = { inlinehint }
attributes #25 = { noinline noreturn }
attributes #26 = { cold }
attributes #27 = { cold noreturn nounwind }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{}
!3 = !{!"branch_weights", i32 2000, i32 14002}
!4 = !{i64 8}
!5 = !{i64 0, i64 -9223372036854775808}
!6 = !{i64 1, i64 0}
!7 = !{!8}
!8 = distinct !{!8, !9, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsen8ds7JR2I0_8anstream: %self"}
!9 = distinct !{!9, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCsen8ds7JR2I0_8anstream"}
!10 = !{i64 0, i64 2}
!11 = !{i64 0, i64 -9223372036854775807}
!12 = !{!13}
!13 = distinct !{!13, !14, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_: %_0"}
!14 = distinct !{!14, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE4autoB4_"}
!15 = !{!16}
!16 = distinct !{!16, !17, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE11always_ansiB4_: %_0"}
!17 = distinct !{!17, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE11always_ansiB4_"}
!18 = !{!16, !13}
!19 = !{!20}
!20 = distinct !{!20, !21, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE11always_ansiB4_: %_0"}
!21 = distinct !{!21, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE11always_ansiB4_"}
!22 = !{!20, !13}
!23 = !{!24, !13}
!24 = distinct !{!24, !25, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE3newB4_: %_0"}
!25 = distinct !{!25, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StderrE3newB4_"}
!26 = !{!27}
!27 = distinct !{!27, !28, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_: %_0"}
!28 = distinct !{!28, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE4autoB4_"}
!29 = !{!30}
!30 = distinct !{!30, !31, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE11always_ansiB4_: %_0"}
!31 = distinct !{!31, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE11always_ansiB4_"}
!32 = !{!30, !27}
!33 = !{!34}
!34 = distinct !{!34, !35, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE11always_ansiB4_: %_0"}
!35 = distinct !{!35, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE11always_ansiB4_"}
!36 = !{!34, !27}
!37 = !{!38, !27}
!38 = distinct !{!38, !39, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE3newB4_: %_0"}
!39 = distinct !{!39, !"_RNvMNtCsen8ds7JR2I0_8anstream4autoINtB2_10AutoStreamNtNtNtCs5sEH5CPMdak_3std2io5stdio6StdoutE3newB4_"}
!40 = !{!41}
!41 = distinct !{!41, !42, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream: %self"}
!42 = distinct !{!42, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream"}
!43 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!44 = !{!45}
!45 = distinct !{!45, !46, !"_RNvXs2_CsofRcISb9et_13anstyle_parseNtB5_6ParserNtNtCsjMrxcFdYDNN_4core7default7Default7defaultCsen8ds7JR2I0_8anstream: %_0"}
!46 = distinct !{!46, !"_RNvXs2_CsofRcISb9et_13anstyle_parseNtB5_6ParserNtNtCsjMrxcFdYDNN_4core7default7Default7defaultCsen8ds7JR2I0_8anstream"}
!47 = !{!"branch_weights", i32 2000, i32 6004}
!48 = !{i8 0, i8 8}
!49 = !{!50}
!50 = distinct !{!50, !51, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_: %self"}
!51 = distinct !{!51, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser7advanceNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBR_"}
!52 = !{!53, !50}
!53 = distinct !{!53, !54, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser14perform_actionNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBZ_: %self"}
!54 = distinct !{!54, !"_RINvMCs8N0ImSjNXUE_9utf8parseNtB3_6Parser14perform_actionNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14VtUtf8ReceiverEBZ_"}
!55 = !{!56, !58}
!56 = distinct !{!56, !57, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsen8ds7JR2I0_8anstream: %self.0"}
!57 = distinct !{!57, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsen8ds7JR2I0_8anstream"}
!58 = distinct !{!58, !57, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCsen8ds7JR2I0_8anstream: %other.0"}
!59 = !{i64 1}
!60 = !{i64 4}
!61 = !{!62, !64}
!62 = distinct !{!62, !63, !"_RNvXs6_NtNtCsen8ds7JR2I0_8anstream7adapter5stripNtB5_14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!63 = distinct !{!63, !"_RNvXs6_NtNtCsen8ds7JR2I0_8anstream7adapter5stripNtB5_14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!64 = distinct !{!64, !65, !"_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_: %self"}
!65 = distinct !{!65, !"_RINvYNtNtNtCsen8ds7JR2I0_8anstream7adapter5strip14StripBytesIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB18_6option6OptionRShEINvNvB10_4last4someB2n_EEB9_"}
!66 = !{!67}
!67 = distinct !{!67, !68, !"_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_: %self"}
!68 = distinct !{!68, !"_RNvMNtCsen8ds7JR2I0_8anstream3fmtINtB2_7AdapterNCNvNtB4_5strip9write_fmt0E9write_fmtB4_"}
!69 = !{!70}
!70 = distinct !{!70, !71, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2j_EB3y_: argument 1"}
!71 = distinct !{!71, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2j_EB3y_"}
!72 = !{!73, !74}
!73 = distinct !{!73, !71, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2j_EB3y_: %self"}
!74 = distinct !{!74, !71, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytes0E0E0B2j_EB3y_: argument 2"}
!75 = !{!73, !70, !74}
!76 = !{!"branch_weights", !"expected", i32 2222941, i32 2145260707}
!77 = !{!78, !80}
!78 = distinct !{!78, !79, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream: %pair"}
!79 = distinct !{!79, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream"}
!80 = distinct !{!80, !79, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream: %self.0"}
!81 = !{!82, !84, !86, !88}
!82 = distinct !{!82, !83, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0B1p_: %_1"}
!83 = distinct !{!83, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0B1p_"}
!84 = distinct !{!84, !85, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_: %_1"}
!85 = distinct !{!85, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvNtNtNtB8_6traits8iterator8Iterator8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2P_"}
!86 = distinct !{!86, !87, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2j_EB3y_: %self"}
!87 = distinct !{!87, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2j_EB3y_"}
!88 = distinct !{!88, !87, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowjENCINvNvBL_8position5checkhNCNvNtNtCsen8ds7JR2I0_8anstream7adapter5strip10next_bytess_0E0E0B2j_EB3y_: %f"}
!89 = !{!86, !88}
!90 = !{!"branch_weights", !"expected", i32 2146410, i32 2145337238}
!91 = !{!92, !94}
!92 = distinct !{!92, !93, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream: %pair"}
!93 = distinct !{!93, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream"}
!94 = distinct !{!94, !93, !"_RNvMNtCsjMrxcFdYDNN_4core5sliceSh8split_atCsen8ds7JR2I0_8anstream: %self.0"}
!95 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000, i32 2000, i32 2000, i32 2000, i32 2000, i32 2000}
!96 = !{!97}
!97 = distinct !{!97, !98, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!98 = distinct !{!98, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!99 = !{!100, !97}
!100 = distinct !{!100, !101, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream: %self"}
!101 = distinct !{!101, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream"}
!102 = !{!103}
!103 = distinct !{!103, !104, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!104 = distinct !{!104, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!105 = !{!106, !103}
!106 = distinct !{!106, !107, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream: %self"}
!107 = distinct !{!107, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCsen8ds7JR2I0_8anstream"}
!108 = !{!109}
!109 = distinct !{!109, !110, !"_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_: %_1"}
!110 = distinct !{!110, !"_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_"}
!111 = !{!112}
!112 = distinct !{!112, !110, !"_RNCNvNtCsen8ds7JR2I0_8anstream5strip9write_fmt0B5_: %buf.0"}
!113 = !{!114}
!114 = distinct !{!114, !115, !"_RNvNtCsen8ds7JR2I0_8anstream5strip9write_all: %raw.1"}
!115 = distinct !{!115, !"_RNvNtCsen8ds7JR2I0_8anstream5strip9write_all"}
!116 = !{!114, !117, !118, !109, !112}
!117 = distinct !{!117, !115, !"_RNvNtCsen8ds7JR2I0_8anstream5strip9write_all: %state"}
!118 = distinct !{!118, !115, !"_RNvNtCsen8ds7JR2I0_8anstream5strip9write_all: %buf.0"}
!119 = !{!117, !118, !109}
!120 = !{!114, !109}
!121 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!122 = !{!123}
!123 = distinct !{!123, !124, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw: %dst.0"}
!124 = distinct !{!124, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw"}
