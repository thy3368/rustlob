; ModuleID = 'number_prefix.250353e037494a47-cgu.0'
source_filename = "number_prefix.250353e037494a47-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_3629cf37e45f4088c031426ac602bbb6 = private unnamed_addr constant [1 x i8] c"k", align 1
@alloc_f9b235d2a210ccd74228e3ef41714c41 = private unnamed_addr constant [1 x i8] c"M", align 1
@alloc_b9a77d17410e0d1e6ff9596fd12d3c82 = private unnamed_addr constant [1 x i8] c"G", align 1
@alloc_8488b46b1090aa1f460173af0eed1272 = private unnamed_addr constant [1 x i8] c"T", align 1
@alloc_c29e55c84125b0bc64bfbcda361e43c7 = private unnamed_addr constant [1 x i8] c"P", align 1
@alloc_c92bab2dec2582f45f478f51a76e64ac = private unnamed_addr constant [1 x i8] c"E", align 1
@alloc_38ea2406a8c8f86dee62ab1a553c7a1c = private unnamed_addr constant [1 x i8] c"Z", align 1
@alloc_ffc197f78acd086a7c8e7e4d7eac7b7e = private unnamed_addr constant [1 x i8] c"Y", align 1
@alloc_c803a913919cc4cc7c525869201ef304 = private unnamed_addr constant [2 x i8] c"Ki", align 1
@alloc_097382b845f263ee384021d6112e4d2c = private unnamed_addr constant [2 x i8] c"Mi", align 1
@alloc_676da1d27150f7d0703e7a03ed4da35b = private unnamed_addr constant [2 x i8] c"Gi", align 1
@alloc_b15e1a524b116a340771d58415be9c95 = private unnamed_addr constant [2 x i8] c"Ti", align 1
@alloc_c965274ee4533d30861cfd452c5d9473 = private unnamed_addr constant [2 x i8] c"Pi", align 1
@alloc_4a79c077d0f27bd5409e789937860481 = private unnamed_addr constant [2 x i8] c"Ei", align 1
@alloc_b4976af3659c3df9bc225ac1ca795bb9 = private unnamed_addr constant [2 x i8] c"Zi", align 1
@alloc_05cd67bd5c9793ff44a2f757ef24b216 = private unnamed_addr constant [2 x i8] c"Yi", align 1
@alloc_0c812808379efded5a4fb82d2790b556 = private unnamed_addr constant [2 x i8] c"\C0\00", align 1
@alloc_92e7eb43628b20a8febc7af0da627eb2 = private unnamed_addr constant [21 x i8] c"invalid prefix syntax", align 1
@switch.table._RNvXs_Cs3b1chdcSpG5_13number_prefixNtB4_6PrefixNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt = private unnamed_addr constant [16 x ptr] [ptr @alloc_3629cf37e45f4088c031426ac602bbb6, ptr @alloc_f9b235d2a210ccd74228e3ef41714c41, ptr @alloc_b9a77d17410e0d1e6ff9596fd12d3c82, ptr @alloc_8488b46b1090aa1f460173af0eed1272, ptr @alloc_c29e55c84125b0bc64bfbcda361e43c7, ptr @alloc_c92bab2dec2582f45f478f51a76e64ac, ptr @alloc_38ea2406a8c8f86dee62ab1a553c7a1c, ptr @alloc_ffc197f78acd086a7c8e7e4d7eac7b7e, ptr @alloc_c803a913919cc4cc7c525869201ef304, ptr @alloc_097382b845f263ee384021d6112e4d2c, ptr @alloc_676da1d27150f7d0703e7a03ed4da35b, ptr @alloc_b15e1a524b116a340771d58415be9c95, ptr @alloc_c965274ee4533d30861cfd452c5d9473, ptr @alloc_4a79c077d0f27bd5409e789937860481, ptr @alloc_b4976af3659c3df9bc225ac1ca795bb9, ptr @alloc_05cd67bd5c9793ff44a2f757ef24b216], align 8
@switch.table._RNvXs_Cs3b1chdcSpG5_13number_prefixNtB4_6PrefixNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt.1 = private unnamed_addr constant [16 x i64] [i64 1, i64 1, i64 1, i64 1, i64 1, i64 1, i64 1, i64 1, i64 2, i64 2, i64 2, i64 2, i64 2, i64 2, i64 2, i64 2], align 8

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs3b1chdcSpG5_13number_prefix(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !3, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <number_prefix::Prefix as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs_Cs3b1chdcSpG5_13number_prefixNtB4_6PrefixNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  %_5 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_5)
  %_6 = load i8, ptr %self, align 1, !range !4, !noundef !2
  %0 = zext nneg i8 %_6 to i64
  %switch.gep = getelementptr inbounds nuw [16 x ptr], ptr @switch.table._RNvXs_Cs3b1chdcSpG5_13number_prefixNtB4_6PrefixNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, i64 0, i64 %0
  %switch.load = load ptr, ptr %switch.gep, align 8
  %1 = zext nneg i8 %_6 to i64
  %switch.gep8 = getelementptr inbounds nuw [16 x i64], ptr @switch.table._RNvXs_Cs3b1chdcSpG5_13number_prefixNtB4_6PrefixNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt.1, i64 0, i64 %1
  %switch.load9 = load i64, ptr %switch.gep8, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  store ptr %switch.load, ptr %_5, align 8
  store i64 %switch.load9, ptr %2, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_5, ptr %args, align 8
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs3b1chdcSpG5_13number_prefix, ptr %_8.sroa.4.0..sroa_idx, align 8
  %_22.0 = load ptr, ptr %f, align 8, !nonnull !2, !align !3, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_22.1 = load ptr, ptr %3, align 8, !nonnull !2, !align !5, !noundef !2
; call core::fmt::write
  %4 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_22.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_22.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_5)
  ret i1 %4
}

; <number_prefix::parse::NumberPrefixParseError as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs_NtCs3b1chdcSpG5_13number_prefix5parseNtB4_22NumberPrefixParseErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %fmt) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %fmt, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_92e7eb43628b20a8febc7af0da627eb2, i64 noundef 21)
  ret i1 %_0
}

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #1

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{}
!3 = !{i64 1}
!4 = !{i8 0, i8 16}
!5 = !{i64 8}
