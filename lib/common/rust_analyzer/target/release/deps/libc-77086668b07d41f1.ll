; ModuleID = 'libc.cc503752d7356084-cgu.0'
source_filename = "libc.cc503752d7356084-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_1410eeb0c69ddafe193efe2ed87690b8 = private unnamed_addr constant [5 x i8] c"semun", align 1
@alloc_d010b4359500b08c3f6e7f182d075773 = private unnamed_addr constant [22 x i8] c"__c_anonymous_ifk_data", align 1
@alloc_ece41c8a2237903c83c928aebeb987ae = private unnamed_addr constant [22 x i8] c"__c_anonymous_ifr_ifru", align 1
@alloc_9bc4c0a63662aee4023e2ef268393d32 = private unnamed_addr constant [22 x i8] c"__c_anonymous_ifc_ifcu", align 1
@alloc_3f096efd96963f7035082bfa1288c1d8 = private unnamed_addr constant [23 x i8] c"__c_anonymous_ifr_ifru6", align 1

; <libc::unix::bsd::apple::semun as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8H_NtNtNtCshxypWE0Cazg_4libc4unix3bsd5appleNtB6_5semunNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_1410eeb0c69ddafe193efe2ed87690b8, i64 noundef 5)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; <libc::unix::bsd::apple::__c_anonymous_ifk_data as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8f_NtNtNtCshxypWE0Cazg_4libc4unix3bsd5appleNtB6_22___c_anonymous_ifk_dataNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d010b4359500b08c3f6e7f182d075773, i64 noundef 22)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; <libc::unix::bsd::apple::__c_anonymous_ifr_ifru as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8n_NtNtNtCshxypWE0Cazg_4libc4unix3bsd5appleNtB6_22___c_anonymous_ifr_ifruNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ece41c8a2237903c83c928aebeb987ae, i64 noundef 22)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; <libc::unix::bsd::apple::__c_anonymous_ifc_ifcu as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8v_NtNtNtCshxypWE0Cazg_4libc4unix3bsd5appleNtB6_22___c_anonymous_ifc_ifcuNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_9bc4c0a63662aee4023e2ef268393d32, i64 noundef 22)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; <libc::unix::bsd::apple::__c_anonymous_ifr_ifru6 as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8z_NtNtNtCshxypWE0Cazg_4libc4unix3bsd5appleNtB6_23___c_anonymous_ifr_ifru6NtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(272) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_4 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_4)
; call <core::fmt::Formatter>::debug_struct
  call void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr noalias noundef nonnull sret([16 x i8]) align 8 captures(address) dereferenceable(16) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3f096efd96963f7035082bfa1288c1d8, i64 noundef 23)
; call <core::fmt::builders::DebugStruct>::finish_non_exhaustive
  %_0 = call noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef nonnull align 8 dereferenceable(16) %_4)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_4)
  ret i1 %_0
}

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #1

; <core::fmt::Formatter>::debug_struct
; Function Attrs: uwtable
declare void @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter12debug_struct(ptr dead_on_unwind noalias noundef writable sret([16 x i8]) align 8 captures(address) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <core::fmt::builders::DebugStruct>::finish_non_exhaustive
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMs1_NtNtCsjMrxcFdYDNN_4core3fmt8buildersNtB5_11DebugStruct21finish_non_exhaustive(ptr noalias noundef align 8 dereferenceable(16)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #1

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
