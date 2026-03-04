; ModuleID = 'anyhow.548eb04fbe53a2b6-cgu.0'
source_filename = "anyhow.548eb04fbe53a2b6-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"std::backtrace::BacktraceFrame" = type { %"std::backtrace::RawFrame::Actual", %"alloc::vec::Vec<std::backtrace::BacktraceSymbol>" }
%"std::backtrace::RawFrame::Actual" = type { %"std::backtrace_rs::backtrace::Frame" }
%"std::backtrace_rs::backtrace::Frame" = type { %"std::backtrace_rs::backtrace::libunwind::Frame" }
%"std::backtrace_rs::backtrace::libunwind::Frame" = type { i64, [3 x i64] }
%"alloc::vec::Vec<std::backtrace::BacktraceSymbol>" = type { %"alloc::raw_vec::RawVec<std::backtrace::BacktraceSymbol>", i64 }
%"alloc::raw_vec::RawVec<std::backtrace::BacktraceSymbol>" = type { %"alloc::raw_vec::RawVecInner", %"core::marker::PhantomData<std::backtrace::BacktraceSymbol>" }
%"alloc::raw_vec::RawVecInner" = type { i64, ptr, %"alloc::alloc::Global" }
%"alloc::alloc::Global" = type {}
%"core::marker::PhantomData<std::backtrace::BacktraceSymbol>" = type {}
%"std::backtrace::BacktraceSymbol" = type { %"core::option::Option<std::backtrace::BytesOrWide>", %"core::option::Option<alloc::vec::Vec<u8>>", %"core::option::Option<u32>", %"core::option::Option<u32>" }
%"core::option::Option<std::backtrace::BytesOrWide>" = type { i64, [3 x i64] }
%"core::option::Option<alloc::vec::Vec<u8>>" = type { i64, [2 x i64] }
%"core::option::Option<u32>" = type { i32, [1 x i32] }

@alloc_1c9e72ef4f3c4c925f006237b5e99045 = private unnamed_addr constant <{ ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastNtNtCsdJPVW0sQgAG_5alloc6string6StringEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error17object_drop_frontNtNtCsdJPVW0sQgAG_5alloc6string6StringEB4_ }>, align 8
@alloc_093facc8e7d37c1fd6d0b4f47cfc8572 = private unnamed_addr constant <{ ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropINtNtB4_7wrapper12MessageErrorReEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refINtNtB4_7wrapper12MessageErrorReEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedINtNtB4_7wrapper12MessageErrorReEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedINtNtB4_7wrapper12MessageErrorReEEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastReEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error17object_drop_frontINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1s_6marker4SendNtB1Z_4SyncEL_EEB4_ }>, align 8
@alloc_8f15763fe203e1c59ab9f400f5f8e454 = private unnamed_addr constant [43 x i8] c"end of range should be a character boundary", align 1
@anon.18f4560d4c35da8c0573dbe31cce292b.0 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 7067659346731365489 to ptr), ptr inttoptr (i64 8479862918148678107 to ptr) }>, align 8
@alloc_c17be871687e96237669ecdd04d5db53 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_ }>, align 8
@vtable.0 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXNtCs7g60D0Ppidu_6anyhow7wrapperINtB2_12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB4_, ptr @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_, ptr @alloc_c17be871687e96237669ecdd04d5db53, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_ }>, align 8
@alloc_17ba3436ef9bd8623bd10d9c0038d583 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_ }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXNtCs7g60D0Ppidu_6anyhow7wrapperINtB2_12MessageErrorReENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB4_, ptr @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_, ptr @alloc_17ba3436ef9bd8623bd10d9c0038d583, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorReENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_ }>, align 8
@alloc_72b3e39c9b1c22b79f7446e4197d0e22 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs5_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt }>, align 8
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_, [16 x i8] c"\10\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs4_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt, ptr @_RNvXs5_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr @alloc_72b3e39c9b1c22b79f7446e4197d0e22, ptr @_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source, ptr @_RNvYNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idB6_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error5causeB6_, ptr @_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error7provide }>, align 8
@alloc_f143f738944d0e214851f31944da179d = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_, [16 x i8] c"P\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_ }>, align 8
@vtable.3 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_, [16 x i8] c"P\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs8_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB7_, ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_, ptr @alloc_f143f738944d0e214851f31944da179d, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error6sourceB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_ }>, align 8
@alloc_0865dc4f52f588ca741a1a84d96662c2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorReEEEBL_, [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_ }>, align 8
@vtable.4 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorReEEEBL_, [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs8_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB7_, ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_, ptr @alloc_0865dc4f52f588ca741a1a84d96662c2, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error6sourceB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorReEENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_ }>, align 8
@alloc_88261d1b95f500c91ea8ac337e220479 = private unnamed_addr constant <{ ptr, [16 x i8], ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBL_7wrapper10BoxedErrorEEBL_, [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_ }>, align 8
@vtable.5 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBL_7wrapper10BoxedErrorEEBL_, [16 x i8] c"H\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs8_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB7_, ptr @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_, ptr @alloc_88261d1b95f500c91ea8ac337e220479, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error6sourceB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtB7_7wrapper10BoxedErrorENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_, ptr @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_ }>, align 8
@vtable.6 = private unnamed_addr constant [24 x i8] c"\00\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", align 8
@alloc_0c812808379efded5a4fb82d2790b556 = private unnamed_addr constant [2 x i8] c"\C0\00", align 1
@alloc_51acbd421ac85a9a399ca0e8aa8c9226 = private unnamed_addr constant [12 x i8] c"\0A\0ACaused by:", align 1
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant [1 x i8] c"\0A", align 1
@vtable.7 = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00 \00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs_NtCs7g60D0Ppidu_6anyhow3fmtINtB4_8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBM_5Write9write_strB6_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow3fmt8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBH_5Write10write_charB7_, ptr @_RNvYINtNtCs7g60D0Ppidu_6anyhow3fmt8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBH_5Write9write_fmtB7_ }>, align 8
@alloc_3f62f09340ec4217b72fe8840b861b6c = private unnamed_addr constant [2 x i8] c"\0A\0A", align 1
@alloc_86c033b424378768b420500f01350358 = private unnamed_addr constant [16 x i8] c"stack backtrace:", align 1
@alloc_a4f8140a525565a8ceb87e4312ebf4a0 = private unnamed_addr constant [17 x i8] c"Stack backtrace:\0A", align 1
@alloc_58d9f94aed38c4835a5113bfa030e38e = private unnamed_addr constant [98 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/fmt.rs\00", align 1
@alloc_8403077a0892b6773b81e3d2ab60e635 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_58d9f94aed38c4835a5113bfa030e38e, [16 x i8] c"a\00\00\00\00\00\00\006\00\00\00\1F\00\00\00" }>, align 8
@alloc_23466b28fd02317287360d508a0d60e6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_58d9f94aed38c4835a5113bfa030e38e, [16 x i8] c"a\00\00\00\00\00\00\00<\00\00\00\1B\00\00\00" }>, align 8
@alloc_2f1a690d3ef170f046a8bf34b77e8da0 = private unnamed_addr constant [5 x i8] c"\02: \C0\00", align 1
@alloc_62c6760a437bdae0bad12604deb7927d = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/anyhow-1.0.100/src/error.rs\00", align 1
@alloc_dcd4e5eaab0d1ce631e75800078b6782 = private unnamed_addr constant <{ ptr, ptr, ptr, ptr, ptr, ptr }> <{ ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropNtNtB4_7wrapper10BoxedErrorEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refNtNtB4_7wrapper10BoxedErrorEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedNtNtB4_7wrapper10BoxedErrorEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedNtNtB4_7wrapper10BoxedErrorEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1q_6marker4SendNtB1X_4SyncEL_EEB4_, ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error17object_drop_frontINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1s_6marker4SendNtB1Z_4SyncEL_EEB4_ }>, align 8
@alloc_84da818df01979596a9e7a52ed4fd1e4 = private unnamed_addr constant [48 x i8] c"assertion failed: self.is_char_boundary(new_len)", align 1
@vtable.9 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt }>, align 8
@alloc_953aeaebd783a84f9cd5e2f7a549ef80 = private unnamed_addr constant [24 x i8] c"backtrace capture failed", align 1
@alloc_6a9d8a56587fcfcc62095ae63dff67cb = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_62c6760a437bdae0bad12604deb7927d, [16 x i8] c"c\00\00\00\00\00\00\00g\04\00\00\0E\00\00\00" }>, align 8
@vtable.a = private unnamed_addr constant <{ [24 x i8], ptr, ptr, ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\000\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str, ptr @_RNvYNtNtCs7g60D0Ppidu_6anyhow6ensure3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_charB6_, ptr @_RNvYNtNtCs7g60D0Ppidu_6anyhow6ensure3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtB6_ }>, align 8
@_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP = external local_unnamed_addr global [256 x i8]
@alloc_a931397211c33a1c8fe0d17838460834 = private unnamed_addr constant [60 x i8] c"internal error: entered unreachable code: invalid Once state", align 1
@alloc_24b637cd9133284607a5bf8acdb509dc = private unnamed_addr constant [127 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/std/src/sys/sync/once/queue.rs\00", align 1
@alloc_74802a7f2e82cc98725e50da6efeabcf = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_24b637cd9133284607a5bf8acdb509dc, [16 x i8] c"~\00\00\00\00\00\00\00\8B\00\00\00\12\00\00\00" }>, align 8
@vtable.c = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str, ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char, ptr @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs7g60D0Ppidu_6anyhow }>, align 8
@alloc_cc656815297f75969399c3f4b1ad3de4 = private unnamed_addr constant [55 x i8] c"a Display implementation returned an error unexpectedly", align 1
@alloc_82687d2573a6ed8a4d1fcf733f159466 = private unnamed_addr constant [116 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs\00", align 1
@alloc_f3c70bf9d2724ff8f638341943ddf3c8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_82687d2573a6ed8a4d1fcf733f159466, [16 x i8] c"s\00\00\00\00\00\00\00f\0B\00\00\0E\00\00\00" }>, align 8
@alloc_99ac8a81a24cac863217ce4a5cbfabea = private unnamed_addr constant [5 x i8] c"Error", align 1
@alloc_7926774156b504d85a3c261767c6333c = private unnamed_addr constant [4 x i8] c"    ", align 1
@alloc_d107a7a48d7efb04418b9bba8c2c8fa5 = private unnamed_addr constant [11 x i8] c"\C3 \00\00(\05\00\02: \00", align 1
@alloc_b4e500608388755dc3eaea8bbe197789 = private unnamed_addr constant [7 x i8] c"       ", align 1
@alloc_04d7ce44d7c86a9a02b346ab945bf155 = private unnamed_addr constant [40 x i8] c"description() is deprecated; use Display", align 1
@anon.18f4560d4c35da8c0573dbe31cce292b.1 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 -640463609205808894 to ptr), ptr inttoptr (i64 1716322800860739650 to ptr) }>, align 8
@anon.18f4560d4c35da8c0573dbe31cce292b.2 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 -7318756893564100098 to ptr), ptr inttoptr (i64 -6530684861118026372 to ptr) }>, align 8
@anon.18f4560d4c35da8c0573dbe31cce292b.3 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 1474922122950398934 to ptr), ptr inttoptr (i64 1623364556305432069 to ptr) }>, align 8
@anon.18f4560d4c35da8c0573dbe31cce292b.4 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 -4549863735566508208 to ptr), ptr inttoptr (i64 -5940437046199485808 to ptr) }>, align 8
@anon.18f4560d4c35da8c0573dbe31cce292b.5 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 7388317162735459084 to ptr), ptr inttoptr (i64 -5316605608683314246 to ptr) }>, align 8
@anon.18f4560d4c35da8c0573dbe31cce292b.6 = private unnamed_addr constant <{ ptr, ptr }> <{ ptr inttoptr (i64 3610633705485972615 to ptr), ptr inttoptr (i64 -6707005957325237486 to ptr) }>, align 8

@_RNvXsb_NtCs7g60D0Ppidu_6anyhow5errorINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1c_6marker4SendEL_EINtNtB1c_7convert4FromNtB7_5ErrorE4from = unnamed_addr alias { ptr, ptr } (ptr), ptr @_RNvXsa_NtCs7g60D0Ppidu_6anyhow5errorINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1c_6marker4SendNtB1J_4SyncEL_EINtNtB1c_7convert4FromNtB7_5ErrorE4from
@_RNvXsc_NtCs7g60D0Ppidu_6anyhow5errorINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_EINtNtB1c_7convert4FromNtB7_5ErrorE4from = unnamed_addr alias { ptr, ptr } (ptr), ptr @_RNvXsa_NtCs7g60D0Ppidu_6anyhow5errorINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1c_6marker4SendNtB1J_4SyncEL_EINtNtB1c_7convert4FromNtB7_5ErrorE4from
@_RNvXse_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorINtNtCsjMrxcFdYDNN_4core7convert5AsRefDNtNtBO_5error5ErrorEL_E6as_ref = unnamed_addr alias { ptr, ptr } (ptr), ptr @_RNvXsd_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorINtNtCsjMrxcFdYDNN_4core7convert5AsRefDNtNtBO_5error5ErrorNtNtBO_6marker4SendNtB1H_4SyncEL_E6as_ref
@_RNvXs1_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops5deref8DerefMut9deref_mut = unnamed_addr alias { ptr, ptr } (ptr), ptr @_RNvXs0_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops5deref5Deref5deref
@_RNvMs3_NtCs7g60D0Ppidu_6anyhow4kindNtB5_5Boxed3new = unnamed_addr alias ptr (ptr, ptr), ptr @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error10from_boxed

; <anyhow::Error>::msg::<alloc::string::String>
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error3msgNtNtCsdJPVW0sQgAG_5alloc6string6StringEB5_(ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %message) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_4 = alloca [48 x i8], align 8
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_4)
; invoke <std::backtrace::Backtrace>::capture
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std9backtraceNtB5_9Backtrace7capture(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(address) dereferenceable(48) %_4)
          to label %bb1 unwind label %bb4

bb1:                                              ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %_3, ptr noundef nonnull align 8 dereferenceable(48) %_4, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_4)
; call <anyhow::Error>::construct::<anyhow::wrapper::MessageError<alloc::string::String>>
  %_0.i1 = call fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructINtNtB5_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB5_(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %message, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret ptr %_0.i1

bb3:                                              ; preds = %bb2.i.i.i4.i.i, %bb4
  resume { ptr, i32 } %0

bb4:                                              ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !2)
  %_1.val.i = load i64, ptr %message, align 8, !alias.scope !2
  %1 = icmp eq i64 %_1.val.i, 0
  br i1 %1, label %bb3, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb4
  %2 = getelementptr inbounds nuw i8, ptr %message, i64 8
  %_1.val1.i = load ptr, ptr %2, align 8, !alias.scope !2, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !2
  br label %bb3
}

; <anyhow::Error>::msg::<&str>
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error3msgReEB5_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %message.0, i64 noundef %message.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3 = alloca [48 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_3)
; call <std::backtrace::Backtrace>::capture
  call void @_RNvMs2_NtCs5sEH5CPMdak_3std9backtraceNtB5_9Backtrace7capture(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(address) dereferenceable(48) %_3)
; call <anyhow::Error>::construct::<anyhow::wrapper::MessageError<&str>>
  %_0.i = call fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructINtNtB5_7wrapper12MessageErrorReEEB5_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %message.0, i64 noundef %message.1, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_3)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_3)
  ret ptr %_0.i
}

; <anyhow::Error>::construct::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructINtNtB5_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB5_(ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %error, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5 = alloca [80 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 80, ptr nonnull %_5)
  store ptr @alloc_1c9e72ef4f3c4c925f006237b5e99045, ptr %_5, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) %backtrace, i64 48, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %_5, i64 56
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %1, ptr noundef nonnull align 8 dereferenceable(24) %error, i64 24, i1 false)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !6
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 dereferenceable_or_null(80) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 80, i64 noundef 8) #27, !noalias !6
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorNtNtB4_6string6StringEEE3newBJ_.exit, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 80) #28
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_(ptr noalias noundef nonnull align 8 dereferenceable(80) %_5) #29
          to label %bb3.i unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb3.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %4

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorNtNtB4_6string6StringEEE3newBJ_.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(80) %2, ptr noundef nonnull align 8 dereferenceable(80) %_5, i64 80, i1 false)
  call void @llvm.lifetime.end.p0(i64 80, ptr nonnull %_5)
  ret ptr %2
}

; <anyhow::Error>::construct::<anyhow::wrapper::MessageError<&str>>
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructINtNtB5_7wrapper12MessageErrorReEEB5_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %error.0, i64 noundef %error.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5 = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_5)
  store ptr @alloc_093facc8e7d37c1fd6d0b4f47cfc8572, ptr %_5, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) %backtrace, i64 48, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %_5, i64 56
  store ptr %error.0, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 64
  store i64 %error.1, ptr %2, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !10
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 dereferenceable_or_null(72) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 72, i64 noundef 8) #27, !noalias !10
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorReEEE3newBJ_.exit, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 72) #28
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %5 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %bb3.i unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb3.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %5

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorReEEE3newBJ_.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %3, ptr noundef nonnull align 8 dereferenceable(72) %_5, i64 72, i1 false)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_5)
  ret ptr %3
}

; <anyhow::Error>::construct::<anyhow::wrapper::BoxedError>
; Function Attrs: cold uwtable
define internal fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructNtNtB5_7wrapper10BoxedErrorEB5_(ptr noundef nonnull align 1 %error.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(80) %error.1, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5 = alloca [72 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 72, ptr nonnull %_5)
  store ptr @alloc_dcd4e5eaab0d1ce631e75800078b6782, ptr %_5, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %0, ptr noundef nonnull align 8 dereferenceable(48) %backtrace, i64 48, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %_5, i64 56
  store ptr %error.0, ptr %1, align 8
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 64
  store ptr %error.1, ptr %2, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !13
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 dereferenceable_or_null(72) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 72, i64 noundef 8) #27, !noalias !13
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBJ_7wrapper10BoxedErrorEE3newBJ_.exit, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 72) #28
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %5 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::BoxedError>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBL_7wrapper10BoxedErrorEEBL_(ptr noalias noundef nonnull align 8 dereferenceable(72) %_5) #29
          to label %bb3.i unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb3.i:                                            ; preds = %cleanup.i
  resume { ptr, i32 } %5

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBJ_7wrapper10BoxedErrorEE3newBJ_.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(72) %3, ptr noundef nonnull align 8 dereferenceable(72) %_5, i64 72, i1 false)
  call void @llvm.lifetime.end.p0(i64 72, ptr nonnull %_5)
  ret ptr %3
}

; <alloc::string::String>::replace_range::<core::ops::range::Range<usize>>
; Function Attrs: uwtable
define internal fastcc void @_RINvMNtCsdJPVW0sQgAG_5alloc6stringNtB3_6String13replace_rangeINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(24) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_6 = load i64, ptr %0, align 8, !noundef !5
  %_20 = icmp sgt i64 %_6, -1
  tail call void @llvm.assume(i1 %_20)
  %_10.i = icmp eq i64 %_6, 0
  br i1 %_10.i, label %bb8.i, label %bb15, !prof !9

bb8.i:                                            ; preds = %start
; call core::slice::index::slice_index_fail
  tail call void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef 0, i64 noundef 1, i64 noundef range(i64 0, -9223372036854775808) 0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8403077a0892b6773b81e3d2ab60e635) #28
  unreachable

bb15:                                             ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_25 = load ptr, ptr %1, align 8, !nonnull !5, !noundef !5
  %_46.not.not = icmp eq i64 %_6, 1
  br i1 %_46.not.not, label %bb1.i.i, label %bb19

bb1.i.i:                                          ; preds = %bb15
  store i64 0, ptr %0, align 8, !alias.scope !16, !noalias !21
  tail call void @llvm.experimental.noalias.scope.decl(metadata !24)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27)
  %self2.i.i.i.i.i = load i64, ptr %self, align 8, !range !30, !alias.scope !31, !noalias !34, !noundef !5
  %_7.i.i.i.i.i = icmp eq i64 %self2.i.i.i.i.i, 0
  br i1 %_7.i.i.i.i.i, label %bb1.i.i.i.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i.i.i.i, !prof !9

bb1.i.i.i.i.i:                                    ; preds = %bb1.i.i
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef 0, i64 noundef 1)
  %_27.pre.i.i.i.i = load i64, ptr %0, align 8, !alias.scope !41, !noalias !34
  %_26.i.i.i.i.pre = load ptr, ptr %1, align 8, !alias.scope !41, !noalias !34
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i.i.i.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i.i.i.i: ; preds = %bb1.i.i.i.i.i, %bb1.i.i
  %_26.i.i.i.i = phi ptr [ %_25, %bb1.i.i ], [ %_26.i.i.i.i.pre, %bb1.i.i.i.i.i ]
  %_27.i.i.i.i = phi i64 [ 0, %bb1.i.i ], [ %_27.pre.i.i.i.i, %bb1.i.i.i.i.i ]
  %2 = add i64 %_27.i.i.i.i, 1
  %_3.i.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_26.i.i.i.i, i64 %_27.i.i.i.i
  store i8 83, ptr %_3.i.i.i.i.i.i.i.i.i.i.i, align 1, !noalias !42
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec6splice6SpliceNtNtNtB4_3str4iter5BytesEECs7g60D0Ppidu_6anyhow.exit

bb3.i.i.i.i5.i:                                   ; preds = %bb19
  store i8 83, ptr %_25, align 1, !noalias !61
  store i64 1, ptr %0, align 8, !noalias !61
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec6splice6SpliceNtNtNtB4_3str4iter5BytesEECs7g60D0Ppidu_6anyhow.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec6splice6SpliceNtNtNtB4_3str4iter5BytesEECs7g60D0Ppidu_6anyhow.exit: ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i.i.i.i, %bb3.i.i.i.i5.i
  %storemerge = phi i64 [ %_6, %bb3.i.i.i.i5.i ], [ %2, %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i.i.i.i ]
  store i64 %storemerge, ptr %0, align 8, !noalias !64
  ret void

bb19:                                             ; preds = %bb15
  %3 = getelementptr inbounds nuw i8, ptr %_25, i64 1
  %self3 = load i8, ptr %3, align 1, !noundef !5
  %4 = icmp sgt i8 %self3, -65
  br i1 %4, label %bb3.i.i.i.i5.i, label %bb5, !prof !65

bb5:                                              ; preds = %bb19
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_8f15763fe203e1c59ab9f400f5f8e454, ptr noundef nonnull inttoptr (i64 87 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8403077a0892b6773b81e3d2ab60e635) #31
  unreachable
}

; <str>::trim_end_matches::<<char>::is_whitespace>
; Function Attrs: nofree norecurse nosync nounwind memory(read, inaccessiblemem: write) uwtable
define internal fastcc i64 @_RINvMNtCsjMrxcFdYDNN_4core3stre16trim_end_matchesNvMNtNtB5_4char7methodsc13is_whitespaceECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %0 = icmp samesign eq i64 %self.1, 0
  br i1 %0, label %bb11, label %bb17.i.i.i.i.i.preheader

bb17.i.i.i.i.i.preheader:                         ; preds = %start
  %_7.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  br label %bb17.i.i.i.i.i

bb17.i.i.i.i.i:                                   ; preds = %bb17.i.i.i.i.i.preheader, %bb5.i.i
  %_23.i25.i.i.i1213.i.i = phi ptr [ %_4.i.i.i.i, %bb5.i.i ], [ %_7.i.i.i, %bb17.i.i.i.i.i.preheader ]
  %_23.i.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i, i64 -1
  %w.i.i.i.i.i = load i8, ptr %_23.i.i.i.i.i.i, align 1, !noalias !66, !noundef !5
  %_6.i.i.i.i.i = icmp sgt i8 %w.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb17.i.i.i.i.i
  %1 = icmp ne ptr %self.0, %_23.i.i.i.i.i.i
  tail call void @llvm.assume(i1 %1)
  %_23.i13.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i, i64 -2
  %z.i.i.i.i.i = load i8, ptr %_23.i13.i.i.i.i.i, align 1, !noalias !66, !noundef !5
  %_25.i.i.i.i.i = and i8 %z.i.i.i.i.i, 31
  %2 = zext nneg i8 %_25.i.i.i.i.i to i32
  %_12.i.i.i.i.i = icmp slt i8 %z.i.i.i.i.i, -64
  br i1 %_12.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb13.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb17.i.i.i.i.i
  %_8.i.i.i.i.i = zext nneg i8 %w.i.i.i.i.i to i32
  br label %bb3.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %3 = icmp ne ptr %self.0, %_23.i13.i.i.i.i.i
  tail call void @llvm.assume(i1 %3)
  %_23.i19.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i, i64 -3
  %y.i.i.i.i.i = load i8, ptr %_23.i19.i.i.i.i.i, align 1, !noalias !66, !noundef !5
  %_29.i.i.i.i.i = and i8 %y.i.i.i.i.i, 15
  %4 = zext nneg i8 %_29.i.i.i.i.i to i32
  %_16.i.i.i.i.i = icmp slt i8 %y.i.i.i.i.i, -64
  br i1 %_16.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb11.i.i.i.i.i

bb13.i.i.i.i.i:                                   ; preds = %bb11.i.i.i.i.i, %bb4.i.i.i.i.i
  %_4.i14.i.i.i.i = phi ptr [ %_4.i15.i.i.i.i, %bb11.i.i.i.i.i ], [ %_23.i13.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %ch.sroa.0.0.i.i.i.i.i = phi i32 [ %9, %bb11.i.i.i.i.i ], [ %2, %bb4.i.i.i.i.i ]
  %_40.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.0.i.i.i.i.i, 6
  %_42.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_41.i.i.i.i.i = zext nneg i8 %_42.i.i.i.i.i to i32
  %5 = or disjoint i32 %_40.i.i.i.i.i, %_41.i.i.i.i.i
  br label %bb3.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %6 = icmp ne ptr %self.0, %_23.i19.i.i.i.i.i
  tail call void @llvm.assume(i1 %6)
  %_23.i25.i.i.i.i.i = getelementptr inbounds i8, ptr %_23.i25.i.i.i1213.i.i, i64 -4
  %x.i.i.i.i.i = load i8, ptr %_23.i25.i.i.i.i.i, align 1, !noalias !66, !noundef !5
  %_33.i.i.i.i.i = and i8 %x.i.i.i.i.i, 7
  %7 = zext nneg i8 %_33.i.i.i.i.i to i32
  %_34.i.i.i.i.i = shl nuw nsw i32 %7, 6
  %_36.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_35.i.i.i.i.i = zext nneg i8 %_36.i.i.i.i.i to i32
  %8 = or disjoint i32 %_34.i.i.i.i.i, %_35.i.i.i.i.i
  br label %bb11.i.i.i.i.i

bb11.i.i.i.i.i:                                   ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i
  %_4.i15.i.i.i.i = phi ptr [ %_23.i25.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_23.i19.i.i.i.i.i, %bb6.i.i.i.i.i ]
  %ch.sroa.0.1.i.i.i.i.i = phi i32 [ %8, %bb8.i.i.i.i.i ], [ %4, %bb6.i.i.i.i.i ]
  %_37.i.i.i.i.i = shl nuw nsw i32 %ch.sroa.0.1.i.i.i.i.i, 6
  %_39.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_38.i.i.i.i.i = zext nneg i8 %_39.i.i.i.i.i to i32
  %9 = or disjoint i32 %_37.i.i.i.i.i, %_38.i.i.i.i.i
  br label %bb13.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb13.i.i.i.i.i, %bb3.i.i.i.i.i
  %_4.i.i.i.i = phi ptr [ %_23.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_4.i14.i.i.i.i, %bb13.i.i.i.i.i ]
  %_0.sroa.4.1.i.ph.i.i.i.i = phi i32 [ %_8.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %5, %bb13.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.1.i.ph.i.i.i.i, 1114112
  tail call void @llvm.assume(i1 %10)
  switch i32 %_0.sroa.4.1.i.ph.i.i.i.i, label %bb1.i.i.i.i.i.i [
    i32 32, label %bb5.i.i
    i32 13, label %bb5.i.i
    i32 12, label %bb5.i.i
    i32 11, label %bb5.i.i
    i32 10, label %bb5.i.i
    i32 9, label %bb5.i.i
  ]

bb1.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i
  %_4.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.1.i.ph.i.i.i.i, 127
  br i1 %_4.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i, label %bb3

bb5.i.i.i.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i
  %_3.i.i.i.i.i.i.i = lshr i32 %_0.sroa.4.1.i.ph.i.i.i.i, 8
  switch i32 %_3.i.i.i.i.i.i.i, label %bb3 [
    i32 0, label %bb6.i.i.i.i.i.i.i
    i32 22, label %bb4.i.i.i.i.i.i.i
    i32 32, label %bb7.i.i.i.i.i.i.i
    i32 48, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs7g60D0Ppidu_6anyhow.exit.i.i.i
  ]

bb4.i.i.i.i.i.i.i:                                ; preds = %bb5.i.i.i.i.i.i
  %11 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i, 5760
  br i1 %11, label %bb5.i.i, label %bb3

bb6.i.i.i.i.i.i.i:                                ; preds = %bb5.i.i.i.i.i.i
  %12 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i, 255
  %_8.i.i.i.i.i.i.i = zext nneg i32 %12 to i64
  %13 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_8.i.i.i.i.i.i.i
  %_6.i.i.i.i.i.i.i = load i8, ptr %13, align 1, !noalias !80, !noundef !5
  %extract.t.i.i.i.i.i.i.i = trunc i8 %_6.i.i.i.i.i.i.i to i1
  br i1 %extract.t.i.i.i.i.i.i.i, label %bb5.i.i, label %bb3

bb7.i.i.i.i.i.i.i:                                ; preds = %bb5.i.i.i.i.i.i
  %14 = and i32 %_0.sroa.4.1.i.ph.i.i.i.i, 255
  %_14.i.i.i.i.i.i.i = zext nneg i32 %14 to i64
  %15 = getelementptr inbounds nuw i8, ptr @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11white_space14WHITESPACE_MAP, i64 %_14.i.i.i.i.i.i.i
  %_12.i.i.i.i.i.i.i = load i8, ptr %15, align 1, !noalias !80, !noundef !5
  %16 = and i8 %_12.i.i.i.i.i.i.i, 2
  %extract.t3.i.i.i.i.not.i.i.i = icmp eq i8 %16, 0
  br i1 %extract.t3.i.i.i.i.not.i.i.i, label %bb3, label %bb5.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs7g60D0Ppidu_6anyhow.exit.i.i.i: ; preds = %bb5.i.i.i.i.i.i
  %17 = icmp eq i32 %_0.sroa.4.1.i.ph.i.i.i.i, 12288
  br i1 %17, label %bb5.i.i, label %bb3

bb5.i.i:                                          ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs7g60D0Ppidu_6anyhow.exit.i.i.i, %bb7.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i, %bb3.i.i.i, %bb3.i.i.i, %bb3.i.i.i, %bb3.i.i.i, %bb3.i.i.i, %bb3.i.i.i
  %18 = icmp eq ptr %self.0, %_4.i.i.i.i
  br i1 %18, label %bb11, label %bb17.i.i.i.i.i

bb3:                                              ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNvMNtNtB9_4char7methodsc13is_whitespaceNtB5_11MultiCharEq7matchesCs7g60D0Ppidu_6anyhow.exit.i.i.i, %bb7.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i, %bb1.i.i.i.i.i.i
  %19 = ptrtoint ptr %_23.i25.i.i.i1213.i.i to i64
  %20 = ptrtoint ptr %self.0 to i64
  %_15.i6.i.i = sub i64 %19, %20
  br label %bb11

bb11:                                             ; preds = %bb5.i.i, %start, %bb3
  %j.sroa.0.0 = phi i64 [ %_15.i6.i.i, %bb3 ], [ 0, %start ], [ 0, %bb5.i.i ]
  ret i64 %j.sroa.0.0
}

; anyhow::error::object_ref::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %0 = insertvalue { ptr, ptr } poison, ptr %_5, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.0, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_ref::<anyhow::wrapper::MessageError<&str>>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refINtNtB4_7wrapper12MessageErrorReEEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %0 = insertvalue { ptr, ptr } poison, ptr %_5, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.1, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_ref::<anyhow::wrapper::BoxedError>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error10object_refNtNtB4_7wrapper10BoxedErrorEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %0 = insertvalue { ptr, ptr } poison, ptr %_5, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.2, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_drop::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: uwtable
define internal void @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !81)
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %bb4.i.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = getelementptr inbounds nuw i8, ptr %e, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !84)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !87)
  %_1.val.i.i.i.i = load i64, ptr %2, align 8, !alias.scope !90
  %3 = icmp eq i64 %_1.val.i.i.i.i, 0
  br i1 %3, label %bb1.i, label %bb2.i.i.i4.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %cleanup.i.i
  %4 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_1.val1.i.i.i.i = load ptr, ptr %4, align 8, !alias.scope !90, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i.i.i, i64 noundef %_1.val.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !90
  br label %bb1.i

bb4.i.i:                                          ; preds = %start
  %5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !91)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !94)
  %_1.val.i.i1.i.i = load i64, ptr %5, align 8, !alias.scope !97
  %6 = icmp eq i64 %_1.val.i.i1.i.i, 0
  br i1 %6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB1k_7wrapper12MessageErrorNtNtBL_6string6StringEEEEB1k_.exit, label %bb2.i.i.i4.i.i.i2.i.i

bb2.i.i.i4.i.i.i2.i.i:                            ; preds = %bb4.i.i
  %7 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_1.val1.i.i3.i.i = load ptr, ptr %7, align 8, !alias.scope !97, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i3.i.i, i64 noundef %_1.val.i.i1.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !97
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB1k_7wrapper12MessageErrorNtNtBL_6string6StringEEEEB1k_.exit

bb1.i:                                            ; preds = %bb2.i.i.i4.i.i.i.i.i, %cleanup.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB1k_7wrapper12MessageErrorNtNtBL_6string6StringEEEEB1k_.exit: ; preds = %bb4.i.i, %bb2.i.i.i4.i.i.i2.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  ret void
}

; anyhow::error::object_drop::<anyhow::wrapper::MessageError<&str>>
; Function Attrs: uwtable
define internal void @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropINtNtB4_7wrapper12MessageErrorReEEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB1k_7wrapper12MessageErrorReEEEEB1k_.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB1k_7wrapper12MessageErrorReEEEEB1k_.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  ret void
}

; anyhow::error::object_drop::<anyhow::wrapper::BoxedError>
; Function Attrs: uwtable
define internal void @_RINvNtCs7g60D0Ppidu_6anyhow5error11object_dropNtNtB4_7wrapper10BoxedErrorEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; invoke core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::BoxedError>>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBL_7wrapper10BoxedErrorEEBL_(ptr noalias noundef nonnull align 8 dereferenceable(72) %e)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtB1k_7wrapper10BoxedErrorEEEB1k_.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  resume { ptr, i32 } %0

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtB1k_7wrapper10BoxedErrorEEEB1k_.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  ret void
}

; anyhow::error::object_boxed::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %0 = insertvalue { ptr, ptr } poison, ptr %e, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.3, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_boxed::<anyhow::wrapper::MessageError<&str>>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedINtNtB4_7wrapper12MessageErrorReEEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %0 = insertvalue { ptr, ptr } poison, ptr %e, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.4, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_boxed::<anyhow::wrapper::BoxedError>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error12object_boxedNtNtB4_7wrapper10BoxedErrorEB4_(ptr noundef nonnull %e) unnamed_addr #3 {
start:
  %0 = insertvalue { ptr, ptr } poison, ptr %e, 0
  %1 = insertvalue { ptr, ptr } %0, ptr @vtable.5, 1
  ret { ptr, ptr } %1
}

; anyhow::error::object_downcast::<alloc::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal noundef ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1q_6marker4SendNtB1X_4SyncEL_EEB4_(ptr noundef nonnull readnone captures(ret: address, provenance) %e, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(16) %target) unnamed_addr #4 {
start:
  %_7 = load i128, ptr %target, align 8, !noundef !5
  %_3 = icmp eq i128 %_7, 82760478985003599859529898323219199352
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_0.sroa.0.0 = select i1 %_3, ptr %_5, ptr null
  ret ptr %_0.sroa.0.0
}

; anyhow::error::object_downcast::<alloc::string::String>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal noundef ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastNtNtCsdJPVW0sQgAG_5alloc6string6StringEB4_(ptr noundef nonnull readnone captures(ret: address, provenance) %e, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(16) %target) unnamed_addr #4 {
start:
  %_7 = load i128, ptr %target, align 8, !noundef !5
  %_3 = icmp eq i128 %_7, -123566602171620202126999985453424216142
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_0.sroa.0.0 = select i1 %_3, ptr %_5, ptr null
  ret ptr %_0.sroa.0.0
}

; anyhow::error::object_downcast::<&str>
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable
define internal noundef ptr @_RINvNtCs7g60D0Ppidu_6anyhow5error15object_downcastReEB4_(ptr noundef nonnull readnone captures(ret: address, provenance) %e, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(16) %target) unnamed_addr #4 {
start:
  %_7 = load i128, ptr %target, align 8, !noundef !5
  %_3 = icmp eq i128 %_7, -93652901832424836513689306266955195027
  %_5 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_0.sroa.0.0 = select i1 %_3, ptr %_5, ptr null
  ret ptr %_0.sroa.0.0
}

; anyhow::error::object_drop_front::<alloc::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal void @_RINvNtCs7g60D0Ppidu_6anyhow5error17object_drop_frontINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1s_6marker4SendNtB1Z_4SyncEL_EEB4_(ptr noundef nonnull %e, ptr dead_on_return noalias readonly align 8 captures(none) %target) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtNtB4_3mem13manually_drop12ManuallyDropIBH_DNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB32_4SyncEL_EEEEEB1k_.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtNtB4_3mem13manually_drop12ManuallyDropIBH_DNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB32_4SyncEL_EEEEEB1k_.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  ret void
}

; anyhow::error::object_drop_front::<alloc::string::String>
; Function Attrs: uwtable
define internal void @_RINvNtCs7g60D0Ppidu_6anyhow5error17object_drop_frontNtNtCsdJPVW0sQgAG_5alloc6string6StringEB4_(ptr noundef nonnull %e, ptr dead_on_return noalias readonly align 8 captures(none) %target) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 dereferenceable(48) %0)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtNtB4_3mem13manually_drop12ManuallyDropNtNtBL_6string6StringEEEEB1k_.exit unwind label %bb1.i

bb1.i:                                            ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  resume { ptr, i32 } %1

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtNtB4_3mem13manually_drop12ManuallyDropNtNtBL_6string6StringEEEEB1k_.exit: ; preds = %start
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  ret void
}

; anyhow::error::object_reallocate_boxed::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedINtNtB4_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_3.sroa.0.0.copyload = load i64, ptr %0, align 8
  %_3.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_3.sroa.5.0.copyload = load ptr, ptr %_3.sroa.5.0..sroa_idx, align 8
  %_3.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %e, i64 72
  %_3.sroa.6.0.copyload = load i64, ptr %_3.sroa.6.0..sroa_idx, align 8
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !98
; call __rustc::__rust_alloc
  %1 = tail call noundef align 8 dereferenceable_or_null(24) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 24, i64 noundef 8) #27, !noalias !98
  %2 = icmp eq ptr %1, null
  br i1 %2, label %bb2.i, label %bb1, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 24) #28
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = icmp eq i64 %_3.sroa.0.0.copyload, 0
  br i1 %4, label %cleanup.body, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
  %5 = icmp ne ptr %_3.sroa.5.0.copyload, null
  tail call void @llvm.assume(i1 %5)
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_3.sroa.5.0.copyload, i64 noundef %_3.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !101
  br label %cleanup.body

cleanup.body:                                     ; preds = %cleanup.i, %bb2.i.i.i4.i.i.i
  %6 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %6) #29
          to label %bb2 unwind label %terminate

bb1:                                              ; preds = %start
  store i64 %_3.sroa.0.0.copyload, ptr %1, align 8
  %_3.sroa.5.0..sroa_idx8 = getelementptr inbounds nuw i8, ptr %1, i64 8
  store ptr %_3.sroa.5.0.copyload, ptr %_3.sroa.5.0..sroa_idx8, align 8
  %_3.sroa.6.0..sroa_idx10 = getelementptr inbounds nuw i8, ptr %1, i64 16
  store i64 %_3.sroa.6.0.copyload, ptr %_3.sroa.6.0..sroa_idx10, align 8
  %7 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %7)
          to label %bb8 unwind label %cleanup1

cleanup1:                                         ; preds = %bb1
  %8 = landingpad { ptr, i32 }
          cleanup
  br label %bb2

bb8:                                              ; preds = %bb1
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  %9 = insertvalue { ptr, ptr } poison, ptr %1, 0
  %10 = insertvalue { ptr, ptr } %9, ptr @vtable.0, 1
  ret { ptr, ptr } %10

terminate:                                        ; preds = %cleanup.body
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb2:                                              ; preds = %cleanup.body, %cleanup1
  %.pn = phi { ptr, i32 } [ %8, %cleanup1 ], [ %3, %cleanup.body ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 80, i64 noundef 8) #27
  resume { ptr, i32 } %.pn
}

; anyhow::error::object_reallocate_boxed::<anyhow::wrapper::MessageError<&str>>
; Function Attrs: uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedINtNtB4_7wrapper12MessageErrorReEEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_3.0 = load ptr, ptr %0, align 8, !nonnull !5, !align !106, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_3.1 = load i64, ptr %1, align 8, !noundef !5
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27
; call __rustc::__rust_alloc
  %2 = tail call noundef align 8 dereferenceable_or_null(16) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 16, i64 noundef 8) #27
  %3 = icmp eq ptr %2, null
  br i1 %3, label %bb2.i, label %bb1, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 16) #28
          to label %.noexc unwind label %cleanup

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup:                                          ; preds = %bb2.i
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %5) #29
          to label %bb2 unwind label %terminate

bb1:                                              ; preds = %start
  store ptr %_3.0, ptr %2, align 8, !noalias !107
  %6 = getelementptr inbounds nuw i8, ptr %2, i64 8
  store i64 %_3.1, ptr %6, align 8
  %7 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %7)
          to label %bb8 unwind label %cleanup1

cleanup1:                                         ; preds = %bb1
  %8 = landingpad { ptr, i32 }
          cleanup
  br label %bb2

bb8:                                              ; preds = %bb1
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  %9 = insertvalue { ptr, ptr } poison, ptr %2, 0
  %10 = insertvalue { ptr, ptr } %9, ptr @vtable.1, 1
  ret { ptr, ptr } %10

terminate:                                        ; preds = %cleanup
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb2:                                              ; preds = %cleanup, %cleanup1
  %.pn = phi { ptr, i32 } [ %8, %cleanup1 ], [ %4, %cleanup ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  resume { ptr, i32 } %.pn
}

; anyhow::error::object_reallocate_boxed::<anyhow::wrapper::BoxedError>
; Function Attrs: uwtable
define internal { ptr, ptr } @_RINvNtCs7g60D0Ppidu_6anyhow5error23object_reallocate_boxedNtNtB4_7wrapper10BoxedErrorEB4_(ptr noundef nonnull %e) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %x.i = alloca [16 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %e, i64 56
  %_3.0 = load ptr, ptr %0, align 8, !nonnull !5, !align !106, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %e, i64 64
  %_3.1 = load ptr, ptr %1, align 8, !nonnull !5, !align !110, !noundef !5
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %x.i)
  store ptr %_3.0, ptr %x.i, align 8, !noalias !111
  %2 = getelementptr inbounds nuw i8, ptr %x.i, i64 8
  store ptr %_3.1, ptr %2, align 8, !noalias !111
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27
; call __rustc::__rust_alloc
  %3 = tail call noundef align 8 dereferenceable_or_null(16) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 16, i64 noundef 8) #27
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb2.i, label %bb1, !prof !9

bb2.i:                                            ; preds = %start
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 16) #28
          to label %.noexc unwind label %cleanup.i

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i:                                        ; preds = %bb2.i
  %5 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<anyhow::wrapper::BoxedError>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_(ptr noalias noundef nonnull align 8 dereferenceable(16) %x.i) #29
          to label %cleanup.body unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

cleanup.body:                                     ; preds = %cleanup.i
  %7 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %7) #29
          to label %bb2 unwind label %terminate

bb1:                                              ; preds = %start
  store ptr %_3.0, ptr %3, align 8
  %8 = getelementptr inbounds nuw i8, ptr %3, i64 8
  store ptr %_3.1, ptr %8, align 8
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %x.i)
  %9 = getelementptr inbounds nuw i8, ptr %e, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %9)
          to label %bb8 unwind label %cleanup1

cleanup1:                                         ; preds = %bb1
  %10 = landingpad { ptr, i32 }
          cleanup
  br label %bb2

bb8:                                              ; preds = %bb1
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  %11 = insertvalue { ptr, ptr } poison, ptr %3, 0
  %12 = insertvalue { ptr, ptr } %11, ptr @vtable.2, 1
  ret { ptr, ptr } %12

terminate:                                        ; preds = %cleanup.body
  %13 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable

bb2:                                              ; preds = %cleanup.body, %cleanup1
  %.pn = phi { ptr, i32 } [ %10, %cleanup1 ], [ %5, %cleanup.body ]
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %e, i64 noundef 72, i64 noundef 8) #27
  resume { ptr, i32 } %.pn
}

; core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !114, !noundef !5
  %1 = icmp eq i64 %0, 3
  br i1 %1, label %bb1, label %bb2

bb1:                                              ; preds = %bb2.i.i.i6.i.i.i.i.i, %bb4.i.i.i.i.i, %bb2.i.i, %bb2, %start
  ret void

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !115)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !118)
  %switch.i.i = icmp samesign ult i64 %0, 2
  br i1 %switch.i.i, label %bb1, label %bb2.i.i

bb2.i.i:                                          ; preds = %bb2
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !121)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !124)
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 40
  %self1.i.i.i.i = load ptr, ptr %3, align 8, !alias.scope !127, !noundef !5
  %_8.i.i.i.i = ptrtoint ptr %self1.i.i.i.i to i64
  switch i64 %_8.i.i.i.i, label %bb2.i.i.i.i [
    i64 3, label %bb1.sink.split.i.i.i.i
    i64 2, label %bb1
    i64 0, label %bb1.sink.split.i.i.i.i
  ], !prof !128

bb2.i.i.i.i:                                      ; preds = %bb2.i.i
; call core::panicking::panic_fmt
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull @alloc_a931397211c33a1c8fe0d17838460834, ptr noundef nonnull inttoptr (i64 121 to ptr), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_74802a7f2e82cc98725e50da6efeabcf) #31, !noalias !127
  unreachable

bb1.sink.split.i.i.i.i:                           ; preds = %bb2.i.i, %bb2.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !129)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !132)
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val.i.i.i.i.i = load ptr, ptr %4, align 8, !alias.scope !135, !nonnull !5, !noundef !5
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %_1.val1.i.i.i.i.i = load i64, ptr %5, align 8, !alias.scope !135, !noundef !5
  %_76.i.i.i.i.i.i.i = icmp eq i64 %_1.val1.i.i.i.i.i, 0
  br i1 %_76.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i:                                ; preds = %bb1.sink.split.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i
  %_3.sroa.0.07.i.i.i.i.i.i.i = phi i64 [ %6, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i ], [ 0, %bb1.sink.split.i.i.i.i ]
  %_6.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::backtrace::BacktraceFrame", ptr %_1.val.i.i.i.i.i, i64 %_3.sroa.0.07.i.i.i.i.i.i.i
  %6 = add nuw i64 %_3.sroa.0.07.i.i.i.i.i.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !136)
  %7 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i, i64 32
  tail call void @llvm.experimental.noalias.scope.decl(metadata !139)
  %8 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i, i64 40
  %_1.val.i.i.i.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !142, !noalias !135, !nonnull !5, !noundef !5
  %9 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i, i64 48
  %_1.val1.i.i.i.i.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !142, !noalias !135, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !143)
  %_76.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %_1.val1.i.i.i.i.i.i.i.i, 0
  br i1 %_76.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb5.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb5.i.i.i.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i
  %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i = phi i64 [ %10, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i ], [ 0, %bb5.i.i.i.i.i.i.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw %"std::backtrace::BacktraceSymbol", ptr %_1.val.i.i.i.i.i.i.i.i, i64 %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i
  %10 = add nuw i64 %_3.sroa.0.07.i.i.i.i.i.i.i.i.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !146)
  %11 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 32
  %.val.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %11, align 8, !range !149, !alias.scope !150, !noalias !151, !noundef !5
  switch i64 %.val.i.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %bb4.i.i.i.i.i.i.i.i.i.i.i
    i64 0, label %bb4.i.i.i.i.i.i.i.i.i.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i:             ; preds = %bb5.i.i.i.i.i.i.i.i.i.i
  %12 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 40
  %.val1.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %12, align 8, !alias.scope !150, !noalias !151, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %.val.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !152
  br label %bb4.i.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i.i:                        ; preds = %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !153)
  %13 = load i64, ptr %_6.i.i.i.i.i.i.i.i.i.i, align 8, !range !156, !alias.scope !157, !noalias !151, !noundef !5
  %14 = icmp eq i64 %13, 2
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i:                      ; preds = %bb4.i.i.i.i.i.i.i.i.i.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !158)
  %15 = icmp eq i64 %13, 0
  %16 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 8
  %.val.i.i.i.i.i.i.i.i.i.i.i.i.i = load i64, ptr %16, align 8, !alias.scope !161, !noalias !151
  %17 = icmp eq i64 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %15, label %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %17, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i:           ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i
  %18 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 16
  %.val1.i.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %18, align 8, !alias.scope !161, !noalias !151, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !162
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i.i.i.i.i.i.i:                    ; preds = %bb2.i.i.i.i.i.i.i.i.i.i.i.i
  br i1 %17, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i4.i4.i.i.i.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i4.i4.i.i.i.i.i.i.i.i.i.i.i.i.i:          ; preds = %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i
  %19 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i.i.i.i.i.i, i64 16
  %.val3.i.i.i.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %19, align 8, !alias.scope !161, !noalias !151, !nonnull !5, !noundef !5
  %alloc_size.i.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i.i = shl nuw i64 %.val.i.i.i.i.i.i.i.i.i.i.i.i.i, 1
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i5.i.i.i.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 2) #27, !noalias !162
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i.i4.i4.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i4.i.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i.i = icmp eq i64 %10, %_1.val1.i.i.i.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i:                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i.i.i.i.i, %bb5.i.i.i.i.i.i.i
  %_1.val4.i.i.i.i.i.i.i.i = load i64, ptr %7, align 8, !range !30, !alias.scope !142, !noalias !135, !noundef !5
  %20 = icmp eq i64 %_1.val4.i.i.i.i.i.i.i.i, 0
  br i1 %20, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i, label %bb2.i.i.i6.i.i.i.i.i.i.i.i

bb2.i.i.i6.i.i.i.i.i.i.i.i:                       ; preds = %bb4.i.i.i.i.i.i.i.i
  %alloc_size.i.i.i.i7.i.i.i.i.i.i.i.i = mul nuw i64 %_1.val4.i.i.i.i.i.i.i.i, 72
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i7.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !151
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i: ; preds = %bb2.i.i.i6.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = icmp eq i64 %6, %_1.val1.i.i.i.i.i
  br i1 %_7.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i, label %bb5.i.i.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow.exit.i.i.i.i.i.i, %bb1.sink.split.i.i.i.i
  %_1.val4.i.i.i.i.i = load i64, ptr %2, align 8, !range !30, !alias.scope !135, !noundef !5
  %21 = icmp eq i64 %_1.val4.i.i.i.i.i, 0
  br i1 %21, label %bb1, label %bb2.i.i.i6.i.i.i.i.i

bb2.i.i.i6.i.i.i.i.i:                             ; preds = %bb4.i.i.i.i.i
  %alloc_size.i.i.i.i7.i.i.i.i.i = mul nuw i64 %_1.val4.i.i.i.i.i, 56
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i7.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27, !noalias !135
  br label %bb1
}

; core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(80) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %0)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !163)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !166)
  %_1.val.i.i = load i64, ptr %2, align 8, !alias.scope !169
  %3 = icmp eq i64 %_1.val.i.i, 0
  br i1 %3, label %bb1, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %_1.val1.i.i = load ptr, ptr %4, align 8, !alias.scope !169, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !169
  br label %bb1

bb4:                                              ; preds = %start
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !170)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !173)
  %_1.val.i.i1 = load i64, ptr %5, align 8, !alias.scope !176
  %6 = icmp eq i64 %_1.val.i.i1, 0
  br i1 %6, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_.exit4, label %bb2.i.i.i4.i.i.i2

bb2.i.i.i4.i.i.i2:                                ; preds = %bb4
  %7 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %_1.val1.i.i3 = load ptr, ptr %7, align 8, !alias.scope !176, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i3, i64 noundef %_1.val.i.i1, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !176
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_.exit4

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_.exit4: ; preds = %bb4, %bb2.i.i.i4.i.i.i2
  ret void

bb1:                                              ; preds = %bb2.i.i.i4.i.i.i, %cleanup
  resume { ptr, i32 } %1
}

; core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<&str>>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorReEEEBL_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; call core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %0)
  ret void
}

; core::ptr::drop_in_place::<anyhow::error::ErrorImpl<anyhow::wrapper::BoxedError>>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBL_7wrapper10BoxedErrorEEBL_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(72) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke core::ptr::drop_in_place::<core::option::Option<std::backtrace::Backtrace>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(48) %0)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %start
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 56
; invoke core::ptr::drop_in_place::<anyhow::wrapper::BoxedError>
  invoke void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_(ptr noalias noundef nonnull align 8 dereferenceable(16) %2) #29
          to label %common.resume unwind label %terminate

bb4:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 56
  tail call void @llvm.experimental.noalias.scope.decl(metadata !177)
  %_1.val.i = load ptr, ptr %3, align 8, !alias.scope !177
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %_1.val1.i = load ptr, ptr %4, align 8, !alias.scope !177, !nonnull !5, !align !110, !noundef !5
  %5 = load ptr, ptr %_1.val1.i, align 8, !invariant.load !5, !noalias !177
  %.not.i.i = icmp eq ptr %5, null
  br i1 %.not.i.i, label %bb3.i.i, label %is_not_null.i.i

is_not_null.i.i:                                  ; preds = %bb4
  %6 = icmp ne ptr %_1.val.i, null
  tail call void @llvm.assume(i1 %6)
  invoke void %5(ptr noundef nonnull %_1.val.i)
          to label %bb3.i.i unwind label %cleanup.i.i, !noalias !177

bb3.i.i:                                          ; preds = %is_not_null.i.i, %bb4
  %7 = icmp ne ptr %_1.val.i, null
  tail call void @llvm.assume(i1 %7)
  %8 = getelementptr inbounds nuw i8, ptr %_1.val1.i, i64 8
  %9 = load i64, ptr %8, align 8, !range !30, !invariant.load !5, !noalias !177
  %10 = getelementptr inbounds nuw i8, ptr %_1.val1.i, i64 16
  %11 = load i64, ptr %10, align 8, !range !180, !invariant.load !5, !noalias !177
  %12 = add i64 %11, -1
  %13 = icmp sgt i64 %12, -1
  tail call void @llvm.assume(i1 %13)
  %14 = icmp eq i64 %9, 0
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i: ; preds = %bb3.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i, i64 noundef %9, i64 noundef range(i64 1, -9223372036854775807) %11) #27, !noalias !177
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_.exit

cleanup.i.i:                                      ; preds = %is_not_null.i.i
  %15 = landingpad { ptr, i32 }
          cleanup
  %16 = getelementptr inbounds nuw i8, ptr %_1.val1.i, i64 8
  %17 = load i64, ptr %16, align 8, !range !30, !invariant.load !5, !noalias !177
  %18 = getelementptr inbounds nuw i8, ptr %_1.val1.i, i64 16
  %19 = load i64, ptr %18, align 8, !range !180, !invariant.load !5, !noalias !177
  %20 = add i64 %19, -1
  %21 = icmp sgt i64 %20, -1
  tail call void @llvm.assume(i1 %21)
  %22 = icmp eq i64 %17, 0
  br i1 %22, label %common.resume, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i: ; preds = %cleanup.i.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i, i64 noundef %17, i64 noundef range(i64 1, -9223372036854775807) %19) #27, !noalias !177
  br label %common.resume

common.resume:                                    ; preds = %cleanup, %cleanup.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i
  %common.resume.op = phi { ptr, i32 } [ %15, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i.i ], [ %15, %cleanup.i.i ], [ %1, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_.exit: ; preds = %bb3.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i
  ret void

terminate:                                        ; preds = %cleanup
  %23 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable
}

; core::ptr::drop_in_place::<anyhow::wrapper::MessageError<alloc::string::String>>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !181)
  %_1.val.i = load i64, ptr %_1, align 8, !alias.scope !181
  %0 = icmp eq i64 %_1.val.i, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1.i = load ptr, ptr %1, align 8, !alias.scope !181, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !181
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit: ; preds = %start, %bb2.i.i.i4.i.i
  ret void
}

; core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1B_4SyncEL_EECs7g60D0Ppidu_6anyhow(ptr %_1.0.val, ptr readonly captures(address_is_null) %_1.8.val) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = icmp ne ptr %_1.8.val, null
  tail call void @llvm.assume(i1 %0)
  %1 = load ptr, ptr %_1.8.val, align 8, !invariant.load !5
  %.not = icmp eq ptr %1, null
  br i1 %.not, label %bb3, label %is_not_null

is_not_null:                                      ; preds = %start
  %2 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %2)
  invoke void %1(ptr noundef nonnull %_1.0.val)
          to label %bb3 unwind label %cleanup

bb3:                                              ; preds = %is_not_null, %start
  %3 = icmp ne ptr %_1.0.val, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %5 = load i64, ptr %4, align 8, !range !30, !invariant.load !5
  %6 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %7 = load i64, ptr %6, align 8, !range !180, !invariant.load !5
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtBM_6marker4SendNtB1j_4SyncEL_ENtNtNtBM_3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i: ; preds = %bb3
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #27
  br label %_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtBM_6marker4SendNtB1j_4SyncEL_ENtNtNtBM_3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow.exit

_RNvXs8_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtBM_6marker4SendNtB1j_4SyncEL_ENtNtNtBM_3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow.exit: ; preds = %bb3, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i
  ret void

cleanup:                                          ; preds = %is_not_null
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 8
  %13 = load i64, ptr %12, align 8, !range !30, !invariant.load !5
  %14 = getelementptr inbounds nuw i8, ptr %_1.8.val, i64 16
  %15 = load i64, ptr %14, align 8, !range !180, !invariant.load !5
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4: ; preds = %cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.0.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #27
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4, %cleanup
  resume { ptr, i32 } %11
}

; core::ptr::drop_in_place::<anyhow::wrapper::BoxedError>
; Function Attrs: uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load ptr, ptr %_1, align 8
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = load ptr, ptr %_1.val1, align 8, !invariant.load !5
  %.not.i = icmp eq ptr %1, null
  br i1 %.not.i, label %bb3.i, label %is_not_null.i

is_not_null.i:                                    ; preds = %start
  %2 = icmp ne ptr %_1.val, null
  tail call void @llvm.assume(i1 %2)
  invoke void %1(ptr noundef nonnull %_1.val)
          to label %bb3.i unwind label %cleanup.i

bb3.i:                                            ; preds = %is_not_null.i, %start
  %3 = icmp ne ptr %_1.val, null
  tail call void @llvm.assume(i1 %3)
  %4 = getelementptr inbounds nuw i8, ptr %_1.val1, i64 8
  %5 = load i64, ptr %4, align 8, !range !30, !invariant.load !5
  %6 = getelementptr inbounds nuw i8, ptr %_1.val1, i64 16
  %7 = load i64, ptr %6, align 8, !range !180, !invariant.load !5
  %8 = add i64 %7, -1
  %9 = icmp sgt i64 %8, -1
  tail call void @llvm.assume(i1 %9)
  %10 = icmp eq i64 %5, 0
  br i1 %10, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1B_4SyncEL_EECs7g60D0Ppidu_6anyhow.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i: ; preds = %bb3.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %5, i64 noundef range(i64 1, -9223372036854775807) %7) #27
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1B_4SyncEL_EECs7g60D0Ppidu_6anyhow.exit

cleanup.i:                                        ; preds = %is_not_null.i
  %11 = landingpad { ptr, i32 }
          cleanup
  %12 = getelementptr inbounds nuw i8, ptr %_1.val1, i64 8
  %13 = load i64, ptr %12, align 8, !range !30, !invariant.load !5
  %14 = getelementptr inbounds nuw i8, ptr %_1.val1, i64 16
  %15 = load i64, ptr %14, align 8, !range !180, !invariant.load !5
  %16 = add i64 %15, -1
  %17 = icmp sgt i64 %16, -1
  tail call void @llvm.assume(i1 %17)
  %18 = icmp eq i64 %13, 0
  br i1 %18, label %bb1.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i: ; preds = %cleanup.i
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %13, i64 noundef range(i64 1, -9223372036854775807) %15) #27
  br label %bb1.i

bb1.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i4.i, %cleanup.i
  resume { ptr, i32 } %11

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1B_4SyncEL_EECs7g60D0Ppidu_6anyhow.exit: ; preds = %bb3.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i
  ret void
}

; core::ptr::drop_in_place::<alloc::string::String>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7g60D0Ppidu_6anyhow.exit, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %1, align 8, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #27
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7g60D0Ppidu_6anyhow.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs7g60D0Ppidu_6anyhow.exit: ; preds = %start, %bb2.i.i.i4.i
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !184)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !30, !alias.scope !184, !noundef !5
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 range(i64 0, -1) 8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !184
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !184
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef 1, i64 noundef 1), !noalias !184
  %_35.i = load i64, ptr %self3.i, align 8, !range !187, !noalias !184, !noundef !5
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !149, !noalias !184, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !184
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !184
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #28
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !184, !nonnull !5, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !184
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !184
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !184
  ret void
}

; <anyhow::error::ErrorImpl>::debug
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl5debug(ptr noundef nonnull %this, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  %tagged.i.i.i.i.i = alloca [24 x i8], align 8
  %args2 = alloca [16 x i8], align 8
  %backtrace = alloca [24 x i8], align 8
  %args1 = alloca [16 x i8], align 8
  %indented = alloca [32 x i8], align 8
  %error = alloca [16 x i8], align 8
  %_21.sroa.6 = alloca i64, align 8
  %_21.sroa.11 = alloca i64, align 8
  %args = alloca [16 x i8], align 8
  %_3 = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_3)
  %_4.i = load ptr, ptr %this, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i(ptr noundef nonnull %this)
  %_2.0.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i, null
  tail call void @llvm.assume(i1 %2)
  %error.1 = extractvalue { ptr, ptr } %1, 1
  store ptr %_2.0.i, ptr %_3, align 8
  %3 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  store ptr %error.1, ptr %3, align 8
  %4 = getelementptr inbounds nuw i8, ptr %f, i64 16
  %_59 = load i32, ptr %4, align 8, !noundef !5
  %_58 = and i32 %_59, 8388608
  %5 = icmp eq i32 %_58, 0
  br i1 %5, label %bb4, label %bb2

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_3, ptr %args, align 8
  %_9.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow, ptr %_9.sroa.4.0..sroa_idx, align 8
  %_71.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %6 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_71.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
; call core::fmt::write
  %7 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_71.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_71.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br i1 %7, label %bb33, label %bb43

bb2:                                              ; preds = %start
  %8 = getelementptr inbounds nuw i8, ptr %error.1, i64 24
  %9 = load ptr, ptr %8, align 8, !invariant.load !5, !nonnull !5
  %10 = tail call noundef zeroext i1 %9(ptr noundef nonnull align 1 %_2.0.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f) #32
  br label %bb33

bb43:                                             ; preds = %bb4
  %11 = getelementptr inbounds nuw i8, ptr %error.1, i64 48
  %12 = load ptr, ptr %11, align 8, !invariant.load !5, !nonnull !5
  %13 = call { ptr, ptr } %12(ptr noundef nonnull align 1 %_2.0.i) #32
  %14 = extractvalue { ptr, ptr } %13, 0
  %15 = extractvalue { ptr, ptr } %13, 1
  %.not = icmp eq ptr %14, null
  br i1 %.not, label %bb18, label %bb7

bb7:                                              ; preds = %bb43
  %16 = icmp ne ptr %15, null
  call void @llvm.assume(i1 %16)
  %_94.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %_94.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
  %17 = getelementptr inbounds nuw i8, ptr %_94.1, i64 24
  %18 = load ptr, ptr %17, align 8, !invariant.load !5, !nonnull !5
  %19 = call noundef zeroext i1 %18(ptr noundef nonnull align 1 %_94.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_51acbd421ac85a9a399ca0e8aa8c9226, i64 noundef 12) #32
  br i1 %19, label %bb33, label %bb50, !prof !65

bb18:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit39, %bb43
  %20 = getelementptr inbounds nuw i8, ptr %this, i64 8
  %21 = load i64, ptr %20, align 8, !range !114, !noundef !5
  %.not.i = icmp eq i64 %21, 3
  br i1 %.not.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i: ; preds = %bb18
  %_4.i.i.i.i = load ptr, ptr %this, align 8, !nonnull !5, !align !110, !noundef !5
  %22 = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 8
  %_3.i.i.i.i = load ptr, ptr %22, align 8, !nonnull !5, !noundef !5
  %23 = call { ptr, ptr } %_3.i.i.i.i(ptr noundef nonnull %this)
  %_2.0.i.i.i.i = extractvalue { ptr, ptr } %23, 0
  %24 = icmp ne ptr %_2.0.i.i.i.i, null
  call void @llvm.assume(i1 %24)
  %_3.1.i.i.i = extractvalue { ptr, ptr } %23, 1
  call void @llvm.experimental.noalias.scope.decl(metadata !188)
  %25 = getelementptr inbounds nuw i8, ptr %_3.1.i.i.i, i64 80
  %err.1.val.i.i.i.i = load ptr, ptr %25, align 8, !alias.scope !188
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %tagged.i.i.i.i.i), !noalias !188
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %tagged.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.0, i64 16, i1 false), !noalias !188
  %26 = getelementptr inbounds nuw i8, ptr %tagged.i.i.i.i.i, i64 16
  store ptr null, ptr %26, align 8, !noalias !188
  call void %err.1.val.i.i.i.i(ptr noundef nonnull align 1 %_2.0.i.i.i.i, ptr noundef nonnull align 8 %tagged.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @vtable.6), !noalias !188
  %_0.i.i.i.i.i = load ptr, ptr %26, align 8, !noalias !188, !align !110, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %tagged.i.i.i.i.i), !noalias !188
  %.not1.i = icmp eq ptr %_0.i.i.i.i.i, null
  br i1 %.not1.i, label %bb6.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exitthread-pre-split, !prof !191

bb6.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i
; call core::option::expect_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_953aeaebd783a84f9cd5e2f7a549ef80, i64 noundef 24, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6a9d8a56587fcfcc62095ae63dff67cb) #31
  unreachable

_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exitthread-pre-split: ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i
  %_153.pr = load i64, ptr %_0.i.i.i.i.i, align 8
  br label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit

_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit: ; preds = %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exitthread-pre-split, %bb18
  %_153 = phi i64 [ %_153.pr, %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exitthread-pre-split ], [ %21, %bb18 ]
  %x.sroa.0.0.i4.i = phi ptr [ %_0.i.i.i.i.i, %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exitthread-pre-split ], [ %20, %bb18 ]
  %switch = icmp ult i64 %_153, 2
  br i1 %switch, label %bb33, label %bb66

bb50:                                             ; preds = %bb7
  %27 = getelementptr inbounds nuw i8, ptr %15, i64 48
  %28 = load ptr, ptr %27, align 8, !invariant.load !5, !nonnull !5
  %29 = call { ptr, ptr } %28(ptr noundef nonnull align 1 %14) #32
  %_17.0 = extractvalue { ptr, ptr } %29, 0
  %.not27 = icmp ne ptr %_17.0, null
  %30 = ptrtoint ptr %15 to i64
  %31 = getelementptr inbounds nuw i8, ptr %error, i64 8
  %. = zext i1 %.not27 to i64
  %32 = getelementptr inbounds nuw i8, ptr %indented, i64 16
  %33 = getelementptr inbounds nuw i8, ptr %indented, i64 8
  %34 = getelementptr inbounds nuw i8, ptr %indented, i64 24
  %_35.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  br label %bb3.i.i

bb3.i.i:                                          ; preds = %bb50, %bb65
  %iter.sroa.8.0 = phi ptr [ %14, %bb50 ], [ %iter.sroa.8.2, %bb65 ]
  %iter.sroa.13.0 = phi i64 [ %30, %bb50 ], [ %iter.sroa.13.3, %bb65 ]
  %iter.sroa.21.0 = phi i64 [ 0, %bb50 ], [ %iter.sroa.21.1, %bb65 ]
  %_21.sroa.0.0 = phi i64 [ undef, %bb50 ], [ %_21.sroa.0.1, %bb65 ]
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_21.sroa.6)
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %_21.sroa.11)
  call void @llvm.experimental.noalias.scope.decl(metadata !192)
  %.not1.i.i = icmp eq ptr %iter.sroa.8.0, null
  br i1 %.not1.i.i, label %bb11, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb3.i.i
  %35 = inttoptr i64 %iter.sroa.13.0 to ptr
  %36 = getelementptr inbounds nuw i8, ptr %35, i64 48
  %37 = load ptr, ptr %36, align 8, !invariant.load !5, !noalias !195, !nonnull !5
  %38 = call { ptr, ptr } %37(ptr noundef nonnull align 1 %iter.sroa.8.0) #32
  %_7.0.i.i = extractvalue { ptr, ptr } %38, 0
  %_7.1.i.i = extractvalue { ptr, ptr } %38, 1
  %39 = ptrtoint ptr %_7.1.i.i to i64
  %_8.0.i = add i64 %iter.sroa.21.0, 1
  %40 = ptrtoint ptr %iter.sroa.8.0 to i64
  store i64 %40, ptr %_21.sroa.6, align 8, !alias.scope !192, !noalias !199
  br label %bb11

bb11:                                             ; preds = %bb7.i.i, %bb3.i.i
  %iter.sroa.8.2 = phi ptr [ null, %bb3.i.i ], [ %_7.0.i.i, %bb7.i.i ]
  %iter.sroa.13.3 = phi i64 [ %iter.sroa.13.0, %bb3.i.i ], [ %39, %bb7.i.i ]
  %iter.sroa.21.1 = phi i64 [ %iter.sroa.21.0, %bb3.i.i ], [ %_8.0.i, %bb7.i.i ]
  %_21.sroa.0.1 = phi i64 [ %_21.sroa.0.0, %bb3.i.i ], [ %iter.sroa.21.0, %bb7.i.i ]
  %.sink.i.sroa.phi = phi ptr [ %_21.sroa.6, %bb3.i.i ], [ %_21.sroa.11, %bb7.i.i ]
  %_0.sroa.4.0.i.sink.i = phi ptr [ null, %bb3.i.i ], [ %35, %bb7.i.i ]
  store ptr %_0.sroa.4.0.i.sink.i, ptr %.sink.i.sroa.phi, align 8, !alias.scope !192, !noalias !199
  %_21.sroa.6.0._21.sroa.6.0._21.sroa.6.0._21.sroa.6.8. = load i64, ptr %_21.sroa.6, align 8, !noundef !5
  %.not28 = icmp eq i64 %_21.sroa.6.0._21.sroa.6.0._21.sroa.6.0._21.sroa.6.8., 0
  br i1 %.not28, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit39, label %bb12

bb12:                                             ; preds = %bb11
  %41 = inttoptr i64 %_21.sroa.6.0._21.sroa.6.0._21.sroa.6.0._21.sroa.6.8. to ptr
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %error)
  %_21.sroa.11.0._21.sroa.11.0._21.sroa.11.0._21.sroa.11.16. = load i64, ptr %_21.sroa.11, align 8, !range !180, !noundef !5
  %42 = inttoptr i64 %_21.sroa.11.0._21.sroa.11.0._21.sroa.11.0._21.sroa.11.16. to ptr
  store ptr %41, ptr %error, align 8
  store ptr %42, ptr %31, align 8
  %_119.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %_119.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
  %43 = getelementptr inbounds nuw i8, ptr %_119.1, i64 24
  %44 = load ptr, ptr %43, align 8, !invariant.load !5, !nonnull !5
  %45 = call noundef zeroext i1 %44(ptr noundef nonnull align 1 %_119.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_49a1e817e911805af64bbc7efb390101, i64 noundef 1)
  br i1 %45, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit43, label %bb57

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit39: ; preds = %bb11
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.11)
  br label %bb18

bb57:                                             ; preds = %bb12
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %indented)
  store ptr %f, ptr %32, align 8
  store i64 %., ptr %indented, align 8
  store i64 %_21.sroa.0.1, ptr %33, align 8
  store i8 0, ptr %34, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1)
  store ptr %error, ptr %args1, align 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow, ptr %_35.sroa.4.0..sroa_idx, align 8
; call core::fmt::write
  %46 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %indented, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.7, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %indented)
  br i1 %46, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit43, label %bb65

bb65:                                             ; preds = %bb57
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %error)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.11)
  br label %bb3.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit43: ; preds = %bb12, %bb57
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %error)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.6)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %_21.sroa.11)
  br label %bb33

common.resume:                                    ; preds = %cleanup.i, %bb2.i.i.i4.i.i.i, %cleanup4, %bb2.i.i.i4.i.i
  %common.resume.op = phi { ptr, i32 } [ %54, %cleanup4 ], [ %54, %bb2.i.i.i4.i.i ], [ %49, %bb2.i.i.i4.i.i.i ], [ %49, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op

bb66:                                             ; preds = %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %backtrace)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !200
  store i64 0, ptr %buf.i, align 8, !noalias !200
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !200
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !200
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !200
  %47 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %47, align 8, !noalias !200
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !200
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !200
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !200
  %48 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.c, ptr %48, align 8, !noalias !200
; invoke <std::backtrace::Backtrace as core::fmt::Display>::fmt
  %_8.i = invoke noundef zeroext i1 @_RNvXs4_NtCs5sEH5CPMdak_3std9backtraceNtB5_9BacktraceNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noundef nonnull align 8 %x.sroa.0.0.i4.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter.i)
          to label %bb1.i unwind label %cleanup.i, !noalias !200

cleanup.i:                                        ; preds = %bb2.i.i44, %bb66
  %49 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !203)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !203, !noalias !200
  %50 = icmp eq i64 %_1.val.i.i, 0
  br i1 %50, label %common.resume, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !203, !noalias !200, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !206
  br label %common.resume

bb1.i:                                            ; preds = %bb66
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !200
  br i1 %_8.i, label %bb2.i.i44, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow.exit, !prof !9

bb2.i.i44:                                        ; preds = %bb1.i
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.9, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #28
          to label %.noexc.i unwind label %cleanup.i, !noalias !200

.noexc.i:                                         ; preds = %bb2.i.i44
  unreachable

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow.exit: ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 0, ptr nonnull %e.i.i), !noalias !200
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %backtrace, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !200
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !200
  %_165.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %_165.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
  %51 = getelementptr inbounds nuw i8, ptr %_165.1, i64 24
  %52 = load ptr, ptr %51, align 8, !invariant.load !5, !nonnull !5
  %53 = invoke noundef zeroext i1 %52(ptr noundef nonnull align 1 %_165.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_3f62f09340ec4217b72fe8840b861b6c, i64 noundef 2)
          to label %bb70 unwind label %cleanup4

cleanup4:                                         ; preds = %bb20, %bb23, %bb24, %bb108, %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow.exit
  %54 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !207)
  %_1.val.i = load i64, ptr %backtrace, align 8, !alias.scope !207
  %55 = icmp eq i64 %_1.val.i, 0
  br i1 %55, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup4
  %56 = getelementptr inbounds nuw i8, ptr %backtrace, i64 8
  %_1.val1.i = load ptr, ptr %56, align 8, !alias.scope !207, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !207
  br label %common.resume

bb70:                                             ; preds = %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow.exit
  br i1 %53, label %bb27, label %bb76

bb76:                                             ; preds = %bb70
  %57 = getelementptr inbounds nuw i8, ptr %backtrace, i64 8
  %58 = getelementptr inbounds nuw i8, ptr %backtrace, i64 16
  %_181 = load i64, ptr %58, align 8, !noundef !5
  %_4.i45 = icmp samesign ugt i64 %_181, 15
  br i1 %_4.i45, label %bb77, label %bb108

bb77:                                             ; preds = %bb76
  %_182 = load ptr, ptr %57, align 8, !nonnull !5, !noundef !5
  %59 = call i32 @memcmp(ptr noundef nonnull dereferenceable(16) @alloc_86c033b424378768b420500f01350358, ptr noundef nonnull readonly align 1 dereferenceable(16) %_182, i64 16), !alias.scope !210
  %60 = icmp eq i32 %59, 0
  br i1 %60, label %bb20, label %bb108

bb20:                                             ; preds = %bb77
; invoke <alloc::string::String>::replace_range::<core::ops::range::Range<usize>>
  invoke fastcc void @_RINvMNtCsdJPVW0sQgAG_5alloc6stringNtB3_6String13replace_rangeINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(24) %backtrace)
          to label %bb20.bb23_crit_edge unwind label %cleanup4

bb20.bb23_crit_edge:                              ; preds = %bb20
  %_212.pre = load i64, ptr %58, align 8
  br label %bb23

bb108:                                            ; preds = %bb76, %bb77
  %_196.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %_196.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
  %61 = getelementptr inbounds nuw i8, ptr %_196.1, i64 24
  %62 = load ptr, ptr %61, align 8, !invariant.load !5, !nonnull !5
  %63 = invoke noundef zeroext i1 %62(ptr noundef nonnull align 1 %_196.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a4f8140a525565a8ceb87e4312ebf4a0, i64 noundef 17)
          to label %bb78 unwind label %cleanup4

bb78:                                             ; preds = %bb108
  br i1 %63, label %bb27, label %bb23

bb23:                                             ; preds = %bb20.bb23_crit_edge, %bb78
  %_212 = phi i64 [ %_212.pre, %bb20.bb23_crit_edge ], [ %_181, %bb78 ]
  %_213 = load ptr, ptr %57, align 8, !nonnull !5, !noundef !5
; call <str>::trim_end_matches::<<char>::is_whitespace>
  %64 = call fastcc i64 @_RINvMNtCsjMrxcFdYDNN_4core3stre16trim_end_matchesNvMNtNtB5_4char7methodsc13is_whitespaceECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_213, i64 noundef %_212)
; invoke <alloc::string::String>::truncate
  invoke fastcc void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8truncate(ptr noalias noundef align 8 dereferenceable(24) %backtrace, i64 noundef %64)
          to label %bb24 unwind label %cleanup4

bb27:                                             ; preds = %bb130, %bb78, %bb70
  call void @llvm.experimental.noalias.scope.decl(metadata !214)
  %_1.val.i47 = load i64, ptr %backtrace, align 8, !alias.scope !214
  %65 = icmp eq i64 %_1.val.i47, 0
  br i1 %65, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit50, label %bb2.i.i.i4.i.i48

bb2.i.i.i4.i.i48:                                 ; preds = %bb27
  %66 = getelementptr inbounds nuw i8, ptr %backtrace, i64 8
  %_1.val1.i49 = load ptr, ptr %66, align 8, !alias.scope !214, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i49, i64 noundef %_1.val.i47, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !214
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit50

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit50: ; preds = %bb27, %bb2.i.i.i4.i.i48
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %backtrace)
  br label %bb33

bb24:                                             ; preds = %bb23
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args2)
  store ptr %backtrace, ptr %args2, align 8
  %_56.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args2, i64 8
  store ptr @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt, ptr %_56.sroa.4.0..sroa_idx, align 8
  %_225.0 = load ptr, ptr %f, align 8, !nonnull !5, !align !106, !noundef !5
  %_225.1 = load ptr, ptr %6, align 8, !nonnull !5, !align !110, !noundef !5
; invoke core::fmt::write
  %67 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_225.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_225.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args2)
          to label %bb130 unwind label %cleanup4

bb130:                                            ; preds = %bb24
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args2)
  br i1 %67, label %bb27, label %bb92

bb92:                                             ; preds = %bb130
  call void @llvm.experimental.noalias.scope.decl(metadata !217)
  %_1.val.i51 = load i64, ptr %backtrace, align 8, !alias.scope !217
  %68 = icmp eq i64 %_1.val.i51, 0
  br i1 %68, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit54, label %bb2.i.i.i4.i.i52

bb2.i.i.i4.i.i52:                                 ; preds = %bb92
  %_1.val1.i53 = load ptr, ptr %57, align 8, !alias.scope !217, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i53, i64 noundef %_1.val.i51, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !217
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit54

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit54: ; preds = %bb92, %bb2.i.i.i4.i.i52
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %backtrace)
  br label %bb33

bb33:                                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit54, %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit50, %bb2, %bb7, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit43, %bb4
  %_0.sroa.0.4.off0 = phi i1 [ true, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit50 ], [ %10, %bb2 ], [ true, %bb7 ], [ true, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters9enumerate9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainEEB1s_.exit43 ], [ true, %bb4 ], [ false, %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit ], [ false, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow.exit54 ]
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_3)
  ret i1 %_0.sroa.0.4.off0
}

; <anyhow::Error>::from_boxed
; Function Attrs: cold uwtable
define noundef nonnull ptr @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error10from_boxed(ptr noundef nonnull align 1 %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(80) %1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %tagged.i.i = alloca [24 x i8], align 8
  %_7 = alloca [48 x i8], align 8
  %backtrace = alloca [48 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !220)
  %2 = getelementptr inbounds nuw i8, ptr %1, i64 80
  %err.1.val.i = load ptr, ptr %2, align 8, !alias.scope !220
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %tagged.i.i), !noalias !220
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %tagged.i.i, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.0, i64 16, i1 false), !noalias !220
  %3 = getelementptr inbounds nuw i8, ptr %tagged.i.i, i64 16
  store ptr null, ptr %3, align 8, !noalias !220
  invoke void %err.1.val.i(ptr noundef nonnull align 1 %0, ptr noundef nonnull align 8 %tagged.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @vtable.6)
          to label %bb1 unwind label %bb9

bb1:                                              ; preds = %start
  %_0.i.i = load ptr, ptr %3, align 8, !noalias !220, !align !110, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %tagged.i.i), !noalias !220
  %.not = icmp eq ptr %_0.i.i, null
  br i1 %.not, label %bb3, label %bb4, !prof !65

bb4:                                              ; preds = %bb1
  store i64 3, ptr %backtrace, align 8
  br label %bb6

bb3:                                              ; preds = %bb1
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %_7)
; invoke <std::backtrace::Backtrace>::capture
  invoke void @_RNvMs2_NtCs5sEH5CPMdak_3std9backtraceNtB5_9Backtrace7capture(ptr noalias noundef nonnull sret([48 x i8]) align 8 captures(address) dereferenceable(48) %_7)
          to label %bb5 unwind label %bb9

bb5:                                              ; preds = %bb3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %backtrace, ptr noundef nonnull align 8 dereferenceable(48) %_7, i64 48, i1 false)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %_7)
  br label %bb6

bb6:                                              ; preds = %bb4, %bb5
; call <anyhow::Error>::construct::<anyhow::wrapper::BoxedError>
  %_0.i2 = call fastcc noalias noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error9constructNtNtB5_7wrapper10BoxedErrorEB5_(ptr noundef nonnull align 1 %0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(80) %1, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(48) %backtrace)
  ret ptr %_0.i2

bb8:                                              ; preds = %bb9
  resume { ptr, i32 } %lpad.thr_comm

bb9:                                              ; preds = %bb3, %start
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<alloc::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtB4_5error5ErrorNtNtB4_6marker4SendNtB1B_4SyncEL_EECs7g60D0Ppidu_6anyhow(ptr nonnull %0, ptr nonnull %1) #29
          to label %bb8 unwind label %terminate

terminate:                                        ; preds = %bb9
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #30
  unreachable
}

; <anyhow::Error>::root_cause
; Function Attrs: uwtable
define { ptr, ptr } @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error10root_cause(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
bb7.i.us.preheader.i:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !223)
  %_3.i = load ptr, ptr %self, align 8, !alias.scope !223, !noalias !226, !nonnull !5, !noundef !5
  %_4.i.i.i = load ptr, ptr %_3.i, align 8, !noalias !228, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 8
  %_3.i.i.i = load ptr, ptr %0, align 8, !noalias !228, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i.i.i(ptr noundef nonnull %_3.i), !noalias !228
  %_2.0.i.i.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i.i.i, null
  tail call void @llvm.assume(i1 %2)
  br label %bb7.i.us.i

bb7.i.us.i:                                       ; preds = %bb7.i.us.i, %bb7.i.us.preheader.i
  %_23.i.i11.us25.i = phi ptr [ %_7.0.i.us.i, %bb7.i.us.i ], [ %_2.0.i.i.i, %bb7.i.us.preheader.i ]
  %.pn = phi { ptr, ptr } [ %5, %bb7.i.us.i ], [ %1, %bb7.i.us.preheader.i ]
  %_7.1.i15.us24.i = extractvalue { ptr, ptr } %.pn, 1
  %3 = getelementptr inbounds nuw i8, ptr %_7.1.i15.us24.i, i64 48
  %4 = load ptr, ptr %3, align 8, !invariant.load !5, !noalias !231, !nonnull !5
  %5 = tail call { ptr, ptr } %4(ptr noundef nonnull align 1 %_23.i.i11.us25.i) #32, !noalias !236
  %_7.0.i.us.i = extractvalue { ptr, ptr } %5, 0
  %.not1.i.us.i = icmp eq ptr %_7.0.i.us.i, null
  br i1 %.not1.i.us.i, label %bb5, label %bb7.i.us.i

bb5:                                              ; preds = %bb7.i.us.i
  %6 = insertvalue { ptr, ptr } poison, ptr %_23.i.i11.us25.i, 0
  %7 = insertvalue { ptr, ptr } %6, ptr %_7.1.i15.us24.i, 1
  ret { ptr, ptr } %7
}

; <anyhow::Error>::thiserror_provide
; Function Attrs: uwtable
define void @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error17thiserror_provide(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noundef nonnull align 8 %request.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !237)
  %_4.i = load ptr, ptr %self, align 8, !alias.scope !237, !noalias !240, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %1 = load i64, ptr %0, align 8, !range !114, !noalias !242, !noundef !5
  %.not.i.i = icmp eq i64 %1, 3
  br i1 %.not.i.i, label %_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide.exit, label %bb1.i.i

bb1.i.i:                                          ; preds = %start
  %_18.sroa.0.0.copyload.i.i.i.i = load i128, ptr %request.0, align 8, !noalias !245
  %_11.i.i.i.i = icmp eq i128 %_18.sroa.0.0.copyload.i.i.i.i, 156425861031228512447185639436817036401
  br i1 %_11.i.i.i.i, label %bb8.i.i.i.i, label %_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide.exit

bb8.i.i.i.i:                                      ; preds = %bb1.i.i
  %_14.i.i.i.i = getelementptr inbounds nuw i8, ptr %request.0, i64 16
  %2 = load ptr, ptr %_14.i.i.i.i, align 8, !noalias !245, !align !110, !noundef !5
  %.not.i.i.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i.i.i, label %bb1.i.i.i.i, label %_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide.exit

bb1.i.i.i.i:                                      ; preds = %bb8.i.i.i.i
  store ptr %0, ptr %_14.i.i.i.i, align 8, !noalias !245
  br label %_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide.exit

_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide.exit: ; preds = %start, %bb1.i.i, %bb8.i.i.i.i, %bb1.i.i.i.i
  %_4.i.i.i = load ptr, ptr %_4.i, align 8, !noalias !242, !nonnull !5, !align !110, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 8
  %_3.i.i.i = load ptr, ptr %3, align 8, !noalias !242, !nonnull !5, !noundef !5
  %4 = tail call { ptr, ptr } %_3.i.i.i(ptr noundef nonnull %_4.i), !noalias !242
  %_2.0.i.i.i = extractvalue { ptr, ptr } %4, 0
  %5 = icmp ne ptr %_2.0.i.i.i, null
  tail call void @llvm.assume(i1 %5)
  %_6.1.i.i = extractvalue { ptr, ptr } %4, 1
  %6 = getelementptr inbounds nuw i8, ptr %_6.1.i.i, i64 80
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !noalias !242, !nonnull !5
  tail call void %7(ptr noundef nonnull align 1 %_2.0.i.i.i, ptr noundef nonnull align 8 %request.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) #32, !noalias !237
  ret void
}

; <anyhow::Error>::into_boxed_dyn_error
; Function Attrs: uwtable
define { ptr, ptr } @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error20into_boxed_dyn_error(ptr noundef nonnull %self) unnamed_addr #1 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 16
  %_2 = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_2(ptr noundef nonnull %self)
  ret { ptr, ptr } %1
}

; <anyhow::Error>::reallocate_into_boxed_dyn_error_without_backtrace
; Function Attrs: uwtable
define { ptr, ptr } @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error49reallocate_into_boxed_dyn_error_without_backtrace(ptr noundef nonnull %self) unnamed_addr #1 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 24
  %_2 = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_2(ptr noundef nonnull %self)
  ret { ptr, ptr } %1
}

; <anyhow::Error>::chain
; Function Attrs: cold uwtable
define void @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error5chain(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) initializes((0, 24)) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #0 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !248)
  %_4.i.i = load ptr, ptr %_3, align 8, !noalias !248, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %0, align 8, !noalias !248, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull %_3), !noalias !248
  %_2.0.i.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %2)
  %_3.1.i = extractvalue { ptr, ptr } %1, 1
  store ptr null, ptr %_0, align 8, !alias.scope !251, !noalias !254
  %_2.sroa.4.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_2.0.i.i, ptr %_2.sroa.4.0._0.sroa_idx.i.i, align 8, !alias.scope !251, !noalias !254
  %_2.sroa.5.0._0.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %_3.1.i, ptr %_2.sroa.5.0._0.sroa_idx.i.i, align 8, !alias.scope !251, !noalias !254
  ret void
}

; <anyhow::Error>::provide
; Function Attrs: uwtable
define void @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noundef nonnull align 8 %request.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_4 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4, i64 8
  %1 = load i64, ptr %0, align 8, !range !114, !noalias !256, !noundef !5
  %.not.i = icmp eq i64 %1, 3
  br i1 %.not.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit, label %bb1.i

bb1.i:                                            ; preds = %start
  %_18.sroa.0.0.copyload.i.i.i = load i128, ptr %request.0, align 8, !noalias !259
  %_11.i.i.i = icmp eq i128 %_18.sroa.0.0.copyload.i.i.i, 156425861031228512447185639436817036401
  br i1 %_11.i.i.i, label %bb8.i.i.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

bb8.i.i.i:                                        ; preds = %bb1.i
  %_14.i.i.i = getelementptr inbounds nuw i8, ptr %request.0, i64 16
  %2 = load ptr, ptr %_14.i.i.i, align 8, !noalias !259, !align !110, !noundef !5
  %.not.i.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i.i, label %bb1.i.i.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

bb1.i.i.i:                                        ; preds = %bb8.i.i.i
  store ptr %0, ptr %_14.i.i.i, align 8, !noalias !259
  br label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit: ; preds = %start, %bb1.i, %bb8.i.i.i, %bb1.i.i.i
  %_4.i.i = load ptr, ptr %_4, align 8, !noalias !256, !nonnull !5, !align !110, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %3, align 8, !noalias !256, !nonnull !5, !noundef !5
  %4 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull %_4), !noalias !256
  %_2.0.i.i = extractvalue { ptr, ptr } %4, 0
  %5 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %5)
  %_6.1.i = extractvalue { ptr, ptr } %4, 1
  %6 = getelementptr inbounds nuw i8, ptr %_6.1.i, i64 80
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !noalias !256, !nonnull !5
  tail call void %7(ptr noundef nonnull align 1 %_2.0.i.i, ptr noundef nonnull align 8 %request.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) #32
  ret void
}

; <anyhow::Error>::backtrace
; Function Attrs: uwtable
define noundef nonnull align 8 ptr @_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error9backtrace(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %tagged.i.i.i.i.i = alloca [24 x i8], align 8
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_3, i64 8
  %1 = load i64, ptr %0, align 8, !range !114, !noundef !5
  %.not.i = icmp eq i64 %1, 3
  br i1 %.not.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i: ; preds = %start
  %_4.i.i.i.i = load ptr, ptr %_3, align 8, !nonnull !5, !align !110, !noundef !5
  %2 = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i, i64 8
  %_3.i.i.i.i = load ptr, ptr %2, align 8, !nonnull !5, !noundef !5
  %3 = tail call { ptr, ptr } %_3.i.i.i.i(ptr noundef nonnull %_3)
  %_2.0.i.i.i.i = extractvalue { ptr, ptr } %3, 0
  %4 = icmp ne ptr %_2.0.i.i.i.i, null
  tail call void @llvm.assume(i1 %4)
  %_3.1.i.i.i = extractvalue { ptr, ptr } %3, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !262)
  %5 = getelementptr inbounds nuw i8, ptr %_3.1.i.i.i, i64 80
  %err.1.val.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !262
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %tagged.i.i.i.i.i), !noalias !262
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %tagged.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.0, i64 16, i1 false), !noalias !262
  %6 = getelementptr inbounds nuw i8, ptr %tagged.i.i.i.i.i, i64 16
  store ptr null, ptr %6, align 8, !noalias !262
  call void %err.1.val.i.i.i.i(ptr noundef nonnull align 1 %_2.0.i.i.i.i, ptr noundef nonnull align 8 %tagged.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @vtable.6), !noalias !262
  %_0.i.i.i.i.i = load ptr, ptr %6, align 8, !noalias !262, !align !110, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %tagged.i.i.i.i.i), !noalias !262
  %.not1.i = icmp eq ptr %_0.i.i.i.i.i, null
  br i1 %.not1.i, label %bb6.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit, !prof !191

bb6.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i
; call core::option::expect_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_953aeaebd783a84f9cd5e2f7a549ef80, i64 noundef 24, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_6a9d8a56587fcfcc62095ae63dff67cb) #31
  unreachable

_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl9backtrace.exit: ; preds = %start, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i
  %x.sroa.0.0.i4.i = phi ptr [ %_0.i.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionRNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceE7or_elseNCNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB1G_9ErrorImpl9backtrace0EB1I_.exit.i ], [ %0, %start ]
  ret ptr %x.sroa.0.0.i4.i
}

; <alloc::string::String>::truncate
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8truncate(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(24) %self, i64 noundef %new_len) unnamed_addr #6 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_4 = load i64, ptr %0, align 8, !noundef !5
  %_9 = icmp sgt i64 %_4, -1
  tail call void @llvm.assume(i1 %_9)
  %_3.not = icmp ugt i64 %new_len, %_4
  br i1 %_3.not, label %bb5, label %bb1

bb1:                                              ; preds = %start
  %1 = icmp ne i64 %new_len, 0
  %_15.not = icmp samesign ult i64 %new_len, %_4
  %or.cond = select i1 %1, i1 %_15.not, i1 false
  br i1 %or.cond, label %bb11, label %bb14

bb5:                                              ; preds = %bb14, %start
  ret void

bb11:                                             ; preds = %bb1
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_14 = load ptr, ptr %2, align 8, !nonnull !5, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %_14, i64 %new_len
  %self1 = load i8, ptr %3, align 1, !noundef !5
  %4 = icmp sgt i8 %self1, -65
  br i1 %4, label %bb14, label %bb3, !prof !65

bb3:                                              ; preds = %bb11
; call core::panicking::panic
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_84da818df01979596a9e7a52ed4fd1e4, i64 noundef 48, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_23466b28fd02317287360d508a0d60e6) #31
  unreachable

bb14:                                             ; preds = %bb11, %bb1
  store i64 %new_len, ptr %0, align 8
  br label %bb5
}

; <alloc::raw_vec::RawVec<&dyn core::error::Error>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8grow_oneCs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #7 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !30, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !265)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 range(i64 0, -1) 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !265
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !265
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 8, i64 noundef 16), !noalias !265
  %_35.i = load i64, ptr %self3.i, align 8, !range !187, !noalias !265, !noundef !5
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !149, !noalias !265, !noundef !5
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !265
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !265
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #28
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !265, !nonnull !5, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !265
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !265
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !265
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs7g60D0Ppidu_6anyhow(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 17) %elem_layout.1) unnamed_addr #8 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %_15.i = add nsw i64 %elem_layout.0, -1
  %_17.i = add nuw nsw i64 %_15.i, %elem_layout.1
  %_19.i = sub nsw i64 0, %elem_layout.0
  %new_size.i = and i64 %_17.i, %_19.i
  %_24.i = add i64 %cap, -1
  %0 = tail call { i64, i1 } @llvm.umul.with.overflow.i64(i64 %new_size.i, i64 %_24.i)
  %_27.0.i = extractvalue { i64, i1 } %0, 0
  %_27.1.i = extractvalue { i64, i1 } %0, 1
  %_33.i = sub nuw i64 -9223372036854775808, %elem_layout.0
  %_32.i = icmp ugt i64 %_27.0.i, %_33.i
  %or.cond.i = select i1 %_27.1.i, i1 true, i1 %_32.i
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !268

bb11.i:                                           ; preds = %bb6.i
  %new_size2.i = add nuw i64 %_27.0.i, %elem_layout.1
  %_40.i = icmp ugt i64 %new_size2.i, %_33.i
  br i1 %_40.i, label %bb11, label %bb14

bb14:                                             ; preds = %bb11.i
  %1 = icmp eq i64 %self.0.val, 0
  br i1 %1, label %bb4.i.i11, label %bb3.i.i

bb14.thread:                                      ; preds = %start
  %2 = icmp eq i64 %self.0.val, 0
  br i1 %2, label %bb7.thread, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb14.thread, %bb14
  %_27.sroa.7.01321 = phi i64 [ %new_size2.i, %bb14 ], [ 0, %bb14.thread ]
  %3 = icmp ne ptr %self.8.val, null
  tail call void @llvm.assume(i1 %3)
  %alloc_size.i23 = mul nuw i64 %elem_layout.1, %self.0.val
  %cond.i.i = icmp uge i64 %_27.sroa.7.01321, %alloc_size.i23
  tail call void @llvm.assume(i1 %cond.i.i)
; call __rustc::__rust_realloc
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %_27.sroa.7.01321) #27
  br label %bb7

bb7.thread:                                       ; preds = %bb14.thread
  %_16.i.i = inttoptr i64 %elem_layout.0 to ptr
  br label %bb9

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #27
  br label %bb7

bb7:                                              ; preds = %bb4.i.i11, %bb3.i.i
  %_27.sroa.7.012 = phi i64 [ %_27.sroa.7.01321, %bb3.i.i ], [ %new_size2.i, %bb4.i.i11 ]
  %raw_ptr.i.i.pn = phi ptr [ %raw_ptr.i.i, %bb3.i.i ], [ %4, %bb4.i.i11 ]
  %5 = icmp eq ptr %raw_ptr.i.i.pn, null
  br i1 %5, label %bb8, label %bb9

bb8:                                              ; preds = %bb7
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %elem_layout.0, ptr %6, align 8
  br label %bb11

bb9:                                              ; preds = %bb7.thread, %bb7
  %raw_ptr.i.i.pn31 = phi ptr [ %_16.i.i, %bb7.thread ], [ %raw_ptr.i.i.pn, %bb7 ]
  %_27.sroa.7.01230 = phi i64 [ 0, %bb7.thread ], [ %_27.sroa.7.012, %bb7 ]
  %7 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %raw_ptr.i.i.pn31, ptr %7, align 8
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb11.i, %bb9, %bb8
  %.sink32 = phi i64 [ 16, %bb9 ], [ 16, %bb8 ], [ 8, %bb11.i ], [ 8, %bb6.i ]
  %_27.sroa.7.01230.sink = phi i64 [ %_27.sroa.7.01230, %bb9 ], [ %_27.sroa.7.012, %bb8 ], [ 0, %bb11.i ], [ 0, %bb6.i ]
  %storemerge8 = phi i64 [ 0, %bb9 ], [ 1, %bb8 ], [ 1, %bb11.i ], [ 1, %bb6.i ]
  %8 = getelementptr inbounds nuw i8, ptr %_0, i64 %.sink32
  store i64 %_27.sroa.7.01230.sink, ptr %8, align 8
  store i64 %storemerge8, ptr %_0, align 8
  ret void
}

; <anyhow::error::ErrorImpl>::error
; Function Attrs: uwtable
define { ptr, ptr } @_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5error(ptr noundef nonnull %this) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %this, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4, i64 8
  %_3 = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3(ptr noundef nonnull %this)
  %_2.0 = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0, null
  tail call void @llvm.assume(i1 %2)
  ret { ptr, ptr } %1
}

; <anyhow::error::ErrorImpl>::provide
; Function Attrs: uwtable
define void @_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide(ptr noundef nonnull %this, ptr noundef nonnull align 8 %request.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %this, i64 8
  %1 = load i64, ptr %0, align 8, !range !114, !noundef !5
  %.not = icmp eq i64 %1, 3
  br i1 %.not, label %bb2, label %bb1

bb1:                                              ; preds = %start
  %_18.sroa.0.0.copyload.i.i = load i128, ptr %request.0, align 8, !noalias !269
  %_11.i.i = icmp eq i128 %_18.sroa.0.0.copyload.i.i, 156425861031228512447185639436817036401
  br i1 %_11.i.i, label %bb8.i.i, label %bb2

bb8.i.i:                                          ; preds = %bb1
  %_14.i.i = getelementptr inbounds nuw i8, ptr %request.0, i64 16
  %2 = load ptr, ptr %_14.i.i, align 8, !noalias !269, !align !110, !noundef !5
  %.not.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i, label %bb1.i.i, label %bb2

bb1.i.i:                                          ; preds = %bb8.i.i
  store ptr %0, ptr %_14.i.i, align 8, !noalias !269
  br label %bb2

bb2:                                              ; preds = %bb1.i.i, %bb8.i.i, %bb1, %start
  %_4.i = load ptr, ptr %this, align 8, !nonnull !5, !align !110, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.i = load ptr, ptr %3, align 8, !nonnull !5, !noundef !5
  %4 = tail call { ptr, ptr } %_3.i(ptr noundef nonnull %this)
  %_2.0.i = extractvalue { ptr, ptr } %4, 0
  %5 = icmp ne ptr %_2.0.i, null
  tail call void @llvm.assume(i1 %5)
  %_6.1 = extractvalue { ptr, ptr } %4, 1
  %6 = getelementptr inbounds nuw i8, ptr %_6.1, i64 80
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !nonnull !5
  tail call void %7(ptr noundef nonnull align 1 %_2.0.i, ptr noundef nonnull align 8 %request.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) #32
  ret void
}

; anyhow::ensure::render
; Function Attrs: uwtable
define noalias noundef nonnull ptr @_RNvNtCs7g60D0Ppidu_6anyhow6ensure6render(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %msg.0, i64 noundef %msg.1, ptr noundef nonnull align 1 %0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %1, ptr noundef nonnull align 1 %2, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %3) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %string = alloca [24 x i8], align 8
  %args1 = alloca [16 x i8], align 8
  %rhs_buf = alloca [48 x i8], align 8
  %args = alloca [16 x i8], align 8
  %lhs_buf = alloca [48 x i8], align 8
  %rhs = alloca [16 x i8], align 8
  %lhs = alloca [16 x i8], align 8
  store ptr %0, ptr %lhs, align 8
  %4 = getelementptr inbounds nuw i8, ptr %lhs, i64 8
  store ptr %1, ptr %4, align 8
  store ptr %2, ptr %rhs, align 8
  %5 = getelementptr inbounds nuw i8, ptr %rhs, i64 8
  store ptr %3, ptr %5, align 8
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %lhs_buf)
  %6 = getelementptr inbounds nuw i8, ptr %lhs_buf, i64 40
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %lhs_buf, i8 0, i64 48, i1 false)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %lhs, ptr %args, align 8
  %_11.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRDNtB6_5DebugEL_Bx_3fmtCs7g60D0Ppidu_6anyhow, ptr %_11.sroa.4.0..sroa_idx, align 8
; call core::fmt::write
  %_5 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %lhs_buf, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.a, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br i1 %_5, label %bb9, label %bb2, !prof !9

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 48, ptr nonnull %rhs_buf)
  call void @llvm.memset.p0.i64(ptr noundef nonnull align 8 dereferenceable(48) %rhs_buf, i8 0, i64 48, i1 false)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1)
  store ptr %rhs, ptr %args1, align 8
  %_20.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args1, i64 8
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRDNtB6_5DebugEL_Bx_3fmtCs7g60D0Ppidu_6anyhow, ptr %_20.sroa.4.0..sroa_idx, align 8
; call core::fmt::write
  %_14 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %rhs_buf, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.a, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args1)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1)
  br i1 %_14, label %bb7, label %bb4, !prof !9

bb4:                                              ; preds = %bb2
  %7 = getelementptr inbounds nuw i8, ptr %rhs_buf, i64 40
  %_58 = load i64, ptr %6, align 8, !noundef !5
  %_65 = load i64, ptr %7, align 8, !noundef !5
  %_27 = add i64 %msg.1, 6
  %_26 = add i64 %_27, %_58
  %_25 = add i64 %_26, %_65
  %len = add i64 %_25, 1
  %_23.i.i = icmp eq i64 %len, 0
  br i1 %_23.i.i, label %bb18, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb4
  %or.cond.not.i = icmp ult i64 %_25, 9223372036854775807
  br i1 %or.cond.not.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i, label %bb17, !prof !272

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i: ; preds = %bb6.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #27, !noalias !273
; call __rustc::__rust_alloc
  %8 = call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %len, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !273
  %9 = icmp eq ptr %8, null
  br i1 %9, label %bb17, label %bb10.i

bb10.i:                                           ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %10 = ptrtoint ptr %8 to i64
  br label %bb18

bb7:                                              ; preds = %bb2
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %rhs_buf)
  br label %bb9

bb17:                                             ; preds = %bb6.i.i, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i
  %_74.sroa.4.0.ph = phi i64 [ 1, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i ], [ 0, %bb6.i.i ]
; call alloc::raw_vec::handle_error
  call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_74.sroa.4.0.ph, i64 %len) #28
  unreachable

bb18:                                             ; preds = %bb10.i, %bb4
  %_74.sroa.10.0 = phi i64 [ %10, %bb10.i ], [ 1, %bb4 ]
  %11 = inttoptr i64 %_74.sroa.10.0 to ptr
  store i64 %len, ptr %string, align 8
  %_71.sroa.4.0.string.sroa_idx = getelementptr inbounds nuw i8, ptr %string, i64 8
  store ptr %11, ptr %_71.sroa.4.0.string.sroa_idx, align 8
  %_71.sroa.5.0.string.sroa_idx = getelementptr inbounds nuw i8, ptr %string, i64 16
  store i64 0, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_7.i = icmp ugt i64 %msg.1, %len
  br i1 %_7.i, label %bb1.i, label %bb20, !prof !9

bb1.i:                                            ; preds = %bb18
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef 0, i64 noundef %msg.1)
          to label %bb1.i.bb20_crit_edge unwind label %bb13

bb1.i.bb20_crit_edge:                             ; preds = %bb1.i
  %_94.pre = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_99.pre = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8
  %self2.i9.pre = load i64, ptr %string, align 8, !range !30
  br label %bb20

bb20:                                             ; preds = %bb1.i.bb20_crit_edge, %bb18
  %self2.i9 = phi i64 [ %self2.i9.pre, %bb1.i.bb20_crit_edge ], [ %len, %bb18 ]
  %_99 = phi ptr [ %_99.pre, %bb1.i.bb20_crit_edge ], [ %11, %bb18 ]
  %_94 = phi i64 [ %_94.pre, %bb1.i.bb20_crit_edge ], [ 0, %bb18 ]
  %_98 = icmp sgt i64 %_94, -1
  call void @llvm.assume(i1 %_98)
  %_96 = getelementptr inbounds nuw i8, ptr %_99, i64 %_94
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_96, ptr nonnull align 1 %msg.0, i64 %msg.1, i1 false)
  %12 = add i64 %_94, %msg.1
  store i64 %12, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_9.i10 = sub i64 %self2.i9, %12
  %_7.i11 = icmp ult i64 %_9.i10, 2
  br i1 %_7.i11, label %bb1.i13, label %bb22, !prof !9

bb1.i13:                                          ; preds = %bb20
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef %12, i64 noundef 2)
          to label %bb1.i13.bb22_crit_edge unwind label %bb13

bb1.i13.bb22_crit_edge:                           ; preds = %bb1.i13
  %_113.pre = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_118.pre = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8
  br label %bb22

bb22:                                             ; preds = %bb1.i13.bb22_crit_edge, %bb20
  %_118 = phi ptr [ %_118.pre, %bb1.i13.bb22_crit_edge ], [ %_99, %bb20 ]
  %_113 = phi i64 [ %_113.pre, %bb1.i13.bb22_crit_edge ], [ %12, %bb20 ]
  %_117 = icmp sgt i64 %_113, -1
  call void @llvm.assume(i1 %_117)
  %_115 = getelementptr inbounds nuw i8, ptr %_118, i64 %_113
  store i16 10272, ptr %_115, align 1
  %13 = add nuw i64 %_113, 2
  store i64 %13, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %self2.i17 = load i64, ptr %string, align 8, !range !30, !noundef !5
  %_9.i18 = sub i64 %self2.i17, %13
  %_7.i19 = icmp ugt i64 %_58, %_9.i18
  br i1 %_7.i19, label %bb1.i21, label %bb24, !prof !9

bb1.i21:                                          ; preds = %bb22
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef %13, i64 noundef %_58)
          to label %bb1.i21.bb24_crit_edge unwind label %bb13

bb1.i21.bb24_crit_edge:                           ; preds = %bb1.i21
  %_130.pre = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %self2.i25.pre = load i64, ptr %string, align 8, !range !30
  br label %bb24

bb24:                                             ; preds = %bb1.i21.bb24_crit_edge, %bb22
  %self2.i25 = phi i64 [ %self2.i25.pre, %bb1.i21.bb24_crit_edge ], [ %self2.i17, %bb22 ]
  %_130 = phi i64 [ %_130.pre, %bb1.i21.bb24_crit_edge ], [ %13, %bb22 ]
  %_134 = icmp sgt i64 %_130, -1
  call void @llvm.assume(i1 %_134)
  %_135 = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8, !nonnull !5, !noundef !5
  %_132 = getelementptr inbounds nuw i8, ptr %_135, i64 %_130
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_132, ptr nonnull align 8 %lhs_buf, i64 %_58, i1 false)
  %14 = add i64 %_130, %_58
  store i64 %14, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_9.i26 = sub i64 %self2.i25, %14
  %_7.i27 = icmp ult i64 %_9.i26, 4
  br i1 %_7.i27, label %bb1.i29, label %bb26, !prof !9

bb1.i29:                                          ; preds = %bb24
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef %14, i64 noundef 4)
          to label %bb1.i29.bb26_crit_edge unwind label %bb13

bb1.i29.bb26_crit_edge:                           ; preds = %bb1.i29
  %_149.pre = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_154.pre = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8
  %self2.i33.pre = load i64, ptr %string, align 8, !range !30
  br label %bb26

bb26:                                             ; preds = %bb1.i29.bb26_crit_edge, %bb24
  %self2.i33 = phi i64 [ %self2.i33.pre, %bb1.i29.bb26_crit_edge ], [ %self2.i25, %bb24 ]
  %_154 = phi ptr [ %_154.pre, %bb1.i29.bb26_crit_edge ], [ %_135, %bb24 ]
  %_149 = phi i64 [ %_149.pre, %bb1.i29.bb26_crit_edge ], [ %14, %bb24 ]
  %_153 = icmp sgt i64 %_149, -1
  call void @llvm.assume(i1 %_153)
  %_151 = getelementptr inbounds nuw i8, ptr %_154, i64 %_149
  store i32 544437792, ptr %_151, align 1
  %15 = add nuw i64 %_149, 4
  store i64 %15, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_9.i34 = sub i64 %self2.i33, %15
  %_7.i35 = icmp ugt i64 %_65, %_9.i34
  br i1 %_7.i35, label %bb1.i37, label %bb28, !prof !9

bb1.i37:                                          ; preds = %bb26
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef %15, i64 noundef %_65)
          to label %bb1.i37.bb28_crit_edge unwind label %bb13

bb1.i37.bb28_crit_edge:                           ; preds = %bb1.i37
  %_166.pre = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_171.pre = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8
  br label %bb28

bb28:                                             ; preds = %bb1.i37.bb28_crit_edge, %bb26
  %_171 = phi ptr [ %_171.pre, %bb1.i37.bb28_crit_edge ], [ %_154, %bb26 ]
  %_166 = phi i64 [ %_166.pre, %bb1.i37.bb28_crit_edge ], [ %15, %bb26 ]
  %_170 = icmp sgt i64 %_166, -1
  call void @llvm.assume(i1 %_170)
  %_168 = getelementptr inbounds nuw i8, ptr %_171, i64 %_166
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_168, ptr nonnull align 8 %rhs_buf, i64 %_65, i1 false)
  %16 = add i64 %_166, %_65
  store i64 %16, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  %_14.i = icmp sgt i64 %16, -1
  call void @llvm.assume(i1 %_14.i)
  %self2.i.i = load i64, ptr %string, align 8, !range !30, !noundef !5
  %_7.i.i = icmp eq i64 %self2.i.i, %16
  br i1 %_7.i.i, label %bb1.i.i, label %bb5, !prof !9

bb1.i.i:                                          ; preds = %bb28
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %string, i64 noundef %16, i64 noundef 1)
          to label %.noexc42 unwind label %bb13

.noexc42:                                         ; preds = %bb1.i.i
  %count.pre.i = load i64, ptr %_71.sroa.5.0.string.sroa_idx, align 8
  br label %bb5

bb5:                                              ; preds = %.noexc42, %bb28
  %count.i = phi i64 [ %16, %bb28 ], [ %count.pre.i, %.noexc42 ]
  %_20.i = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8, !nonnull !5, !noundef !5
  %_21.i = icmp sgt i64 %count.i, -1
  call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  store i8 41, ptr %_8.i, align 1, !noalias !276
  %new_len.i = add nuw i64 %16, 1
  store i64 %new_len.i, ptr %_71.sroa.5.0.string.sroa_idx, align 8
; call <anyhow::Error>::msg::<alloc::string::String>
  %17 = call fastcc noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error3msgNtNtCsdJPVW0sQgAG_5alloc6string6StringEB5_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %string)
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %rhs_buf)
  br label %bb11

bb11:                                             ; preds = %bb9, %bb5
  %_0.sroa.0.0 = phi ptr [ %17, %bb5 ], [ %19, %bb9 ]
  call void @llvm.lifetime.end.p0(i64 48, ptr nonnull %lhs_buf)
  ret ptr %_0.sroa.0.0

bb12:                                             ; preds = %bb2.i.i.i4.i.i, %bb13
  resume { ptr, i32 } %lpad.thr_comm

bb13:                                             ; preds = %bb1.i, %bb1.i13, %bb1.i21, %bb1.i29, %bb1.i37, %bb1.i.i
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
  %_1.val.i = load i64, ptr %string, align 8
  %18 = icmp eq i64 %_1.val.i, 0
  br i1 %18, label %bb12, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb13
  %_1.val1.i = load ptr, ptr %_71.sroa.4.0.string.sroa_idx, align 8, !nonnull !5, !noundef !5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i, i64 noundef %_1.val.i, i64 noundef range(i64 1, -9223372036854775807) 1) #27, !noalias !279
  br label %bb12

bb9:                                              ; preds = %start, %bb7
; call <anyhow::Error>::msg::<&str>
  %19 = call fastcc noundef nonnull ptr @_RINvMNtCs7g60D0Ppidu_6anyhow5errorNtB5_5Error3msgReEB5_(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %msg.0, i64 noundef %msg.1)
  br label %bb11
}

; anyhow::nightly::request_ref_backtrace
; Function Attrs: uwtable
define noundef align 8 ptr @_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace(ptr noundef nonnull align 1 %err.0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(80) %err.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %tagged.i = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %err.1, i64 80
  %err.1.val = load ptr, ptr %0, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %tagged.i)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %tagged.i, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.0, i64 16, i1 false)
  %1 = getelementptr inbounds nuw i8, ptr %tagged.i, i64 16
  store ptr null, ptr %1, align 8
  call void %err.1.val(ptr noundef nonnull align 1 %err.0, ptr noundef nonnull align 8 %tagged.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @vtable.6)
  %_0.i = load ptr, ptr %1, align 8, !align !110, !noundef !5
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %tagged.i)
  ret ptr %_0.i
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXNtCs7g60D0Ppidu_6anyhow7wrapperINtB2_12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB4_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1 = load i64, ptr %1, align 8, !noundef !5
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <anyhow::wrapper::MessageError<&str> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXNtCs7g60D0Ppidu_6anyhow7wrapperINtB2_12MessageErrorReENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB4_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %self.val = load ptr, ptr %self, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val1 = load i64, ptr %0, align 8, !noundef !5
; call <str as core::fmt::Debug>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <anyhow::Chain as core::iter::traits::double_ended::DoubleEndedIterator>::next_back
; Function Attrs: uwtable
define { ptr, ptr } @_RNvXs0_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits12double_ended19DoubleEndedIterator9next_back(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %rest = alloca [24 x i8], align 8
  %0 = load ptr, ptr %self, align 8, !noundef !5
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !282)
  %_17.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_15.i = load ptr, ptr %_17.i, align 8, !alias.scope !282, !nonnull !5, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_25.i = load ptr, ptr %1, align 8, !alias.scope !282, !nonnull !5, !noundef !5
  %_12.i = icmp eq ptr %_25.i, %_15.i
  br i1 %_12.i, label %bb11, label %bb6.i

bb6.i:                                            ; preds = %bb2
  %2 = getelementptr inbounds i8, ptr %_15.i, i64 -16
  store ptr %2, ptr %_17.i, align 8, !alias.scope !282
  %_20.0.i = load ptr, ptr %2, align 8, !noalias !282, !nonnull !5, !align !106, !noundef !5
  %3 = getelementptr inbounds i8, ptr %_15.i, i64 -8
  %_20.1.i = load ptr, ptr %3, align 8, !noalias !282, !nonnull !5, !align !110, !noundef !5
  br label %bb11

bb3:                                              ; preds = %start
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %5 = load ptr, ptr %4, align 8, !align !106, !noundef !5
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %7 = load ptr, ptr %6, align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %rest)
  store i64 0, ptr %rest, align 8
  %8 = getelementptr inbounds nuw i8, ptr %rest, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %8, align 8
  %9 = getelementptr inbounds nuw i8, ptr %rest, i64 16
  store i64 0, ptr %9, align 8
  %.not435 = icmp eq ptr %5, null
  br i1 %.not435, label %bb9, label %bb5

bb5:                                              ; preds = %bb3, %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit
  %_14.i39 = phi ptr [ %_14.i, %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit ], [ inttoptr (i64 8 to ptr), %bb3 ]
  %len.i = phi i64 [ %19, %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit ], [ 0, %bb3 ]
  %next.sroa.0.037 = phi ptr [ %16, %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit ], [ %5, %bb3 ]
  %next.sroa.4.036 = phi ptr [ %17, %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit ], [ %7, %bb3 ]
  %10 = icmp ne ptr %next.sroa.4.036, null
  tail call void @llvm.assume(i1 %10)
  %11 = getelementptr inbounds nuw i8, ptr %next.sroa.4.036, i64 48
  %12 = load ptr, ptr %11, align 8, !invariant.load !5, !nonnull !5
  %13 = invoke { ptr, ptr } %12(ptr noundef nonnull align 1 %next.sroa.0.037)
          to label %bb6 unwind label %bb15

bb7:                                              ; preds = %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit
  %_25.pre = load ptr, ptr %8, align 8
  %_24.pre = load i64, ptr %rest, align 8, !range !30
  %_22 = icmp ult i64 %19, 576460752303423488
  tail call void @llvm.assume(i1 %_22)
  %_19.idx = shl nuw nsw i64 %19, 4
  %_19 = getelementptr inbounds nuw i8, ptr %_25.pre, i64 %_19.idx
  %_12.i16 = icmp eq i64 %19, 0
  br i1 %_12.i16, label %bb9, label %bb6.i17

bb6.i17:                                          ; preds = %bb7
  %14 = getelementptr inbounds i8, ptr %_19, i64 -16
  %_20.0.i18 = load ptr, ptr %14, align 8, !noalias !285, !nonnull !5, !align !106, !noundef !5
  %15 = getelementptr inbounds i8, ptr %_19, i64 -8
  %_20.1.i19 = load ptr, ptr %15, align 8, !noalias !285, !nonnull !5, !align !110, !noundef !5
  br label %bb9

bb6:                                              ; preds = %bb5
  %16 = extractvalue { ptr, ptr } %13, 0
  %17 = extractvalue { ptr, ptr } %13, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !288)
  %self1.i = load i64, ptr %rest, align 8, !range !30, !alias.scope !288, !noalias !291, !noundef !5
  %_4.i = icmp eq i64 %len.i, %self1.i
  br i1 %_4.i, label %bb1.i, label %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit

bb1.i:                                            ; preds = %bb6
; invoke <alloc::raw_vec::RawVec<&dyn core::error::Error>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8grow_oneCs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %rest)
          to label %bb1.i._RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit_crit_edge unwind label %bb15

bb1.i._RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit_crit_edge: ; preds = %bb1.i
  %_14.i.pre = load ptr, ptr %8, align 8, !alias.scope !288, !noalias !291
  br label %_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit

_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit: ; preds = %bb1.i._RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit_crit_edge, %bb6
  %_14.i = phi ptr [ %_14.i.pre, %bb1.i._RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow.exit_crit_edge ], [ %_14.i39, %bb6 ]
  %end.i = getelementptr inbounds nuw { ptr, ptr }, ptr %_14.i, i64 %len.i
  store ptr %next.sroa.0.037, ptr %end.i, align 8, !noalias !293
  %18 = getelementptr inbounds nuw i8, ptr %end.i, i64 8
  store ptr %next.sroa.4.036, ptr %18, align 8, !noalias !293
  %19 = add i64 %len.i, 1
  store i64 %19, ptr %9, align 8
  %.not4 = icmp eq ptr %16, null
  br i1 %.not4, label %bb7, label %bb5

bb9:                                              ; preds = %bb3, %bb6.i17, %bb7
  %_2552 = phi ptr [ %_25.pre, %bb7 ], [ %_25.pre, %bb6.i17 ], [ inttoptr (i64 8 to ptr), %bb3 ]
  %_2451 = phi i64 [ %_24.pre, %bb7 ], [ %_24.pre, %bb6.i17 ], [ 0, %bb3 ]
  %rest1.sroa.9.0 = phi ptr [ %_19, %bb7 ], [ %14, %bb6.i17 ], [ inttoptr (i64 8 to ptr), %bb3 ]
  %_0.sroa.3.0.i20 = phi ptr [ undef, %bb7 ], [ %_20.1.i19, %bb6.i17 ], [ undef, %bb3 ]
  %_0.sroa.0.0.i21 = phi ptr [ null, %bb7 ], [ %_20.0.i18, %bb6.i17 ], [ null, %bb3 ]
  store ptr %_2552, ptr %self, align 8
  store ptr %_2552, ptr %4, align 8
  store i64 %_2451, ptr %6, align 8
  %_10.sroa.7.0.self.sroa_idx = getelementptr inbounds nuw i8, ptr %self, i64 24
  store ptr %rest1.sroa.9.0, ptr %_10.sroa.7.0.self.sroa_idx, align 8
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %rest)
  br label %bb11

bb11:                                             ; preds = %bb6.i, %bb2, %bb9
  %_0.sroa.0.0.i.pn = phi ptr [ %_0.sroa.0.0.i21, %bb9 ], [ %_20.0.i, %bb6.i ], [ null, %bb2 ]
  %_0.sroa.3.0.i.pn = phi ptr [ %_0.sroa.3.0.i20, %bb9 ], [ %_20.1.i, %bb6.i ], [ undef, %bb2 ]
  %.pn = insertvalue { ptr, ptr } poison, ptr %_0.sroa.0.0.i.pn, 0
  %.pn8 = insertvalue { ptr, ptr } %.pn, ptr %_0.sroa.3.0.i.pn, 1
  ret { ptr, ptr } %.pn8

bb12:                                             ; preds = %bb2.i.i.i4.i, %bb15
  resume { ptr, i32 } %20

bb15:                                             ; preds = %bb1.i, %bb5
  %20 = landingpad { ptr, i32 }
          cleanup
  %rest.val = load i64, ptr %rest, align 8
  %21 = icmp eq i64 %rest.val, 0
  br i1 %21, label %bb12, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %bb15
  %rest.val12 = load ptr, ptr %8, align 8, !nonnull !5, !noundef !5
  %alloc_size.i.i.i.i5.i = shl nuw i64 %rest.val, 4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %rest.val12, i64 noundef %alloc_size.i.i.i.i5.i, i64 noundef range(i64 1, -9223372036854775807) 8) #27
  br label %bb12
}

; <anyhow::Error as core::ops::deref::Deref>::deref
; Function Attrs: uwtable
define { ptr, ptr } @_RNvXs0_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops5deref5Deref5deref(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 {
start:
  %_3 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %_4.i = load ptr, ptr %_3, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i(ptr noundef nonnull %_3)
  %_2.0.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i, null
  tail call void @llvm.assume(i1 %2)
  ret { ptr, ptr } %1
}

; <anyhow::Chain as core::iter::traits::exact_size::ExactSizeIterator>::len
; Function Attrs: uwtable
define noundef i64 @_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len(ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %self) unnamed_addr #1 {
start:
  %0 = load ptr, ptr %self, align 8, !noundef !5
  %.not = icmp eq ptr %0, null
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %2 = load ptr, ptr %1, align 8, !noundef !5
  br i1 %.not, label %bb3, label %bb10

bb3:                                              ; preds = %start
  %.not46 = icmp eq ptr %2, null
  br i1 %.not46, label %bb8, label %bb5.preheader

bb5.preheader:                                    ; preds = %bb3
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %4 = load ptr, ptr %3, align 8
  br label %bb5

bb5:                                              ; preds = %bb5.preheader, %bb5
  %len.sroa.0.09 = phi i64 [ %11, %bb5 ], [ 0, %bb5.preheader ]
  %next.sroa.0.08 = phi ptr [ %9, %bb5 ], [ %2, %bb5.preheader ]
  %next.sroa.4.07 = phi ptr [ %10, %bb5 ], [ %4, %bb5.preheader ]
  %5 = icmp ne ptr %next.sroa.4.07, null
  tail call void @llvm.assume(i1 %5)
  %6 = getelementptr inbounds nuw i8, ptr %next.sroa.4.07, i64 48
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !nonnull !5
  %8 = tail call { ptr, ptr } %7(ptr noundef nonnull align 1 %next.sroa.0.08) #32
  %9 = extractvalue { ptr, ptr } %8, 0
  %10 = extractvalue { ptr, ptr } %8, 1
  %11 = add i64 %len.sroa.0.09, 1
  %.not4 = icmp eq ptr %9, null
  br i1 %.not4, label %bb8, label %bb5

bb8:                                              ; preds = %bb5, %bb3, %bb10
  %len.sroa.0.1 = phi i64 [ %16, %bb10 ], [ 0, %bb3 ], [ %11, %bb5 ]
  ret i64 %len.sroa.0.1

bb10:                                             ; preds = %start
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self.val5 = load ptr, ptr %12, align 8, !nonnull !5, !noundef !5
  %13 = ptrtoint ptr %self.val5 to i64
  %14 = ptrtoint ptr %2 to i64
  %15 = sub nuw i64 %13, %14
  %16 = lshr exact i64 %15, 4
  br label %bb8
}

; <anyhow::ensure::Buf as core::fmt::Write>::write_str
; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable
define noundef zeroext i1 @_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str(ptr noalias noundef align 8 captures(none) dereferenceable(48) %self, ptr noalias noundef nonnull readonly align 1 captures(address) %s.0, i64 noundef %s.1) unnamed_addr #9 personality ptr @rust_eh_personality {
start:
  %_21 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  br label %bb1.i

bb1.i:                                            ; preds = %bb3.i, %start
  %_16.i3.i = phi ptr [ %_16.i.i, %bb3.i ], [ %s.0, %start ]
  %_6.i.not.not.not.i.not = icmp eq ptr %_16.i3.i, %_21
  br i1 %_6.i.not.not.not.i.not, label %bb2, label %bb3.i

bb3.i:                                            ; preds = %bb1.i
  %_16.i.i = getelementptr inbounds nuw i8, ptr %_16.i3.i, i64 1
  %.val.i = load i8, ptr %_16.i3.i, align 1, !noalias !294, !noundef !5
  switch i8 %.val.i, label %bb1.i [
    i8 32, label %bb5
    i8 10, label %bb5
  ]

bb2:                                              ; preds = %bb1.i
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %_6 = load i64, ptr %0, align 8, !noundef !5
  %remaining = sub i64 40, %_6
  %_7 = icmp ugt i64 %s.1, %remaining
  br i1 %_7, label %bb5, label %bb4

bb4:                                              ; preds = %bb2
  %_10 = getelementptr inbounds nuw i8, ptr %self, i64 %_6
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_10, ptr nonnull align 1 %s.0, i64 %s.1, i1 false)
  %1 = load i64, ptr %0, align 8, !noundef !5
  %2 = add i64 %1, %s.1
  store i64 %2, ptr %0, align 8
  br label %bb5

bb5:                                              ; preds = %bb3.i, %bb3.i, %bb2, %bb4
  %_0.sroa.0.0.off0 = phi i1 [ false, %bb4 ], [ true, %bb2 ], [ true, %bb3.i ], [ true, %bb3.i ]
  ret i1 %_0.sroa.0.0.off0
}

; <&dyn core::fmt::Debug as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtRDNtB6_5DebugEL_Bx_3fmtCs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_3.1, i64 24
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  %_0 = tail call noundef zeroext i1 %2(ptr noundef nonnull align 1 %_3.0, ptr noalias noundef nonnull align 8 dereferenceable(24) %f) #32
  ret i1 %_0
}

; <&dyn core::error::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_3.1, i64 32
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  %_0 = tail call noundef zeroext i1 %2(ptr noundef nonnull align 1 %_3.0, ptr noalias noundef nonnull align 8 dereferenceable(24) %f) #32
  ret i1 %_0
}

; <anyhow::Error as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %formatter) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args1.i = alloca [16 x i8], align 8
  %cause.i = alloca [16 x i8], align 8
  %args.i = alloca [16 x i8], align 8
  %_6.i = alloca [16 x i8], align 8
  %_4 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  tail call void @llvm.experimental.noalias.scope.decl(metadata !297)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %_6.i), !noalias !297
  %_4.i.i = load ptr, ptr %_4, align 8, !noalias !297, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %0, align 8, !noalias !297, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull %_4), !noalias !297
  %_2.0.i.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %2)
  %3 = extractvalue { ptr, ptr } %1, 1
  store ptr %_2.0.i.i, ptr %_6.i, align 8, !noalias !297
  %4 = getelementptr inbounds nuw i8, ptr %_6.i, i64 8
  store ptr %3, ptr %4, align 8, !noalias !297
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !297
  store ptr %_6.i, ptr %args.i, align 8, !noalias !297
  %_8.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow, ptr %_8.sroa.4.0..sroa_idx.i, align 8, !noalias !297
  %_33.0.i = load ptr, ptr %formatter, align 8, !alias.scope !297, !nonnull !5, !align !106, !noundef !5
  %5 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_33.1.i = load ptr, ptr %5, align 8, !alias.scope !297, !nonnull !5, !align !110, !noundef !5
; call core::fmt::write
  %6 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_33.0.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_33.1.i, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i), !noalias !297
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !297
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %_6.i), !noalias !297
  br i1 %6, label %_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit, label %bb21.i

bb21.i:                                           ; preds = %start
  %7 = getelementptr inbounds nuw i8, ptr %formatter, i64 16
  %_46.i = load i32, ptr %7, align 8, !alias.scope !297, !noundef !5
  %_45.i = and i32 %_46.i, 8388608
  %8 = icmp eq i32 %_45.i, 0
  br i1 %8, label %_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit, label %bb7.i.us.i.peel.i, !prof !65

bb7.i.us.i.peel.i:                                ; preds = %bb21.i
  %_4.i.i.i = load ptr, ptr %_4, align 8, !noalias !300, !nonnull !5, !align !110, !noundef !5
  %9 = getelementptr inbounds nuw i8, ptr %_4.i.i.i, i64 8
  %_3.i.i.i = load ptr, ptr %9, align 8, !noalias !300, !nonnull !5, !noundef !5
  %10 = call { ptr, ptr } %_3.i.i.i(ptr noundef nonnull %_4), !noalias !300
  %_2.0.i.i.i = extractvalue { ptr, ptr } %10, 0
  %11 = icmp ne ptr %_2.0.i.i.i, null
  call void @llvm.assume(i1 %11)
  %_3.1.i.i = extractvalue { ptr, ptr } %10, 1
  %12 = getelementptr inbounds nuw i8, ptr %cause.i, i64 8
  %_20.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args1.i, i64 8
  %13 = getelementptr inbounds nuw i8, ptr %_3.1.i.i, i64 48
  %14 = load ptr, ptr %13, align 8, !invariant.load !5, !noalias !303, !nonnull !5
  %15 = call { ptr, ptr } %14(ptr noundef nonnull align 1 %_2.0.i.i.i) #32, !noalias !297
  %_7.0.i.us.i.peel.i = extractvalue { ptr, ptr } %15, 0
  %.not1.i38.peel.i = icmp eq ptr %_7.0.i.us.i.peel.i, null
  br i1 %.not1.i38.peel.i, label %_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit, label %bb6.peel.i

bb6.peel.i:                                       ; preds = %bb7.i.us.i.peel.i
  %_7.1.i.us.i.peel.i = extractvalue { ptr, ptr } %15, 1
  %16 = getelementptr inbounds nuw i8, ptr %_7.1.i.us.i.peel.i, i64 48
  %17 = load ptr, ptr %16, align 8, !invariant.load !5, !noalias !308, !nonnull !5
  %18 = call { ptr, ptr } %17(ptr noundef nonnull align 1 %_7.0.i.us.i.peel.i) #32, !noalias !297
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cause.i), !noalias !297
  %19 = icmp ne ptr %_7.1.i.us.i.peel.i, null
  call void @llvm.assume(i1 %19)
  store ptr %_7.0.i.us.i.peel.i, ptr %cause.i, align 8, !noalias !297
  store ptr %_7.1.i.us.i.peel.i, ptr %12, align 8, !noalias !297
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i), !noalias !297
  store ptr %cause.i, ptr %args1.i, align 8, !noalias !297
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow, ptr %_20.sroa.4.0..sroa_idx.i, align 8, !noalias !297
; call core::fmt::write
  %20 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_33.0.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_33.1.i, ptr noundef nonnull @alloc_2f1a690d3ef170f046a8bf34b77e8da0, ptr noundef nonnull %args1.i), !noalias !297
  br i1 %20, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4skip4SkipNtCs7g60D0Ppidu_6anyhow5ChainEEB1i_.exit51.i, label %bb3.i.i

bb3.i.i:                                          ; preds = %bb6.peel.i, %bb6.i
  %.pn.i = phi { ptr, ptr } [ %23, %bb6.i ], [ %18, %bb6.peel.i ]
  %iter.sroa.10.0.i = extractvalue { ptr, ptr } %.pn.i, 0
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i), !noalias !297
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cause.i), !noalias !297
  %.not1.i.i = icmp eq ptr %iter.sroa.10.0.i, null
  br i1 %.not1.i.i, label %_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit, label %bb6.i

bb6.i:                                            ; preds = %bb3.i.i
  %iter.sroa.22.0.in.i = extractvalue { ptr, ptr } %.pn.i, 1
  %21 = getelementptr inbounds nuw i8, ptr %iter.sroa.22.0.in.i, i64 48
  %22 = load ptr, ptr %21, align 8, !invariant.load !5, !noalias !311, !nonnull !5
  %23 = call { ptr, ptr } %22(ptr noundef nonnull align 1 %iter.sroa.10.0.i) #32, !noalias !297
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %cause.i), !noalias !297
  %24 = icmp ne ptr %iter.sroa.22.0.in.i, null
  call void @llvm.assume(i1 %24)
  store ptr %iter.sroa.10.0.i, ptr %cause.i, align 8, !noalias !297
  store ptr %iter.sroa.22.0.in.i, ptr %12, align 8, !noalias !297
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args1.i), !noalias !297
  store ptr %cause.i, ptr %args1.i, align 8, !noalias !297
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtRDNtNtB8_5error5ErrorEL_NtB6_7Display3fmtCs7g60D0Ppidu_6anyhow, ptr %_20.sroa.4.0..sroa_idx.i, align 8, !noalias !297
; call core::fmt::write
  %25 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_33.0.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_33.1.i, ptr noundef nonnull @alloc_2f1a690d3ef170f046a8bf34b77e8da0, ptr noundef nonnull %args1.i), !noalias !297
  br i1 %25, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4skip4SkipNtCs7g60D0Ppidu_6anyhow5ChainEEB1i_.exit51.i, label %bb3.i.i, !llvm.loop !314

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4skip4SkipNtCs7g60D0Ppidu_6anyhow5ChainEEB1i_.exit51.i: ; preds = %bb6.i, %bb6.peel.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args1.i), !noalias !297
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %cause.i), !noalias !297
  br label %_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit

_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display.exit: ; preds = %bb3.i.i, %start, %bb21.i, %bb7.i.us.i.peel.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4skip4SkipNtCs7g60D0Ppidu_6anyhow5ChainEEB1i_.exit51.i
  %_0.sroa.0.0.off0.i = phi i1 [ true, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4skip4SkipNtCs7g60D0Ppidu_6anyhow5ChainEEB1i_.exit51.i ], [ false, %bb21.i ], [ false, %bb7.i.us.i.peel.i ], [ true, %start ], [ false, %bb3.i.i ]
  ret i1 %_0.sroa.0.0.off0.i
}

; <anyhow::Error as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs3_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #1 {
start:
  %_4 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
; call <anyhow::error::ErrorImpl>::debug
  %_0 = tail call noundef zeroext i1 @_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl5debug(ptr noundef nonnull %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter)
  ret i1 %_0
}

; <anyhow::Error as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
define void @_RNvXs4_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 {
start:
  %_5 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %_4 = load ptr, ptr %_5, align 8, !nonnull !5, !align !110, !noundef !5
  %_3 = load ptr, ptr %_4, align 8, !nonnull !5, !noundef !5
  tail call void %_3(ptr noundef nonnull %_5)
  ret void
}

; <anyhow::wrapper::BoxedError as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs4_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_5.0 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_5.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_5.1, i64 24
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  %_0 = tail call noundef zeroext i1 %2(ptr noundef nonnull align 1 %_5.0, ptr noalias noundef nonnull align 8 dereferenceable(24) %f) #32
  ret i1 %_0
}

; <anyhow::context::Quoted<&mut core::fmt::Formatter> as core::fmt::Write>::write_str
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs5_NtCs7g60D0Ppidu_6anyhow7contextINtB5_6QuotedQNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBQ_5Write9write_str(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 {
start:
  %_4 = alloca [120 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 120, ptr nonnull %_4)
; call <str>::escape_debug
  call void @_RNvMNtCsjMrxcFdYDNN_4core3stre12escape_debug(ptr noalias noundef nonnull sret([120 x i8]) align 8 captures(none) dereferenceable(120) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  %_5 = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
; call <core::str::iter::EscapeDebug as core::fmt::Display>::fmt
  %_0 = call noundef zeroext i1 @_RNvXs20_NtNtCsjMrxcFdYDNN_4core3str4iterNtB6_11EscapeDebugNtNtBa_3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(120) %_4, ptr noalias noundef nonnull align 8 dereferenceable(24) %_5)
  call void @llvm.lifetime.end.p0(i64 120, ptr nonnull %_4)
  ret i1 %_0
}

; <anyhow::wrapper::BoxedError as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs5_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %_5.0 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_5.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_5.1, i64 32
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  %_0 = tail call noundef zeroext i1 %2(ptr noundef nonnull align 1 %_5.0, ptr noalias noundef nonnull align 8 dereferenceable(24) %f) #32
  ret i1 %_0
}

; <anyhow::wrapper::BoxedError as core::error::Error>::source
; Function Attrs: uwtable
define { ptr, ptr } @_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
  %_4.0 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_4.1, i64 48
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  %3 = tail call { ptr, ptr } %2(ptr noundef nonnull align 1 %_4.0) #32
  ret { ptr, ptr } %3
}

; <anyhow::wrapper::BoxedError as core::error::Error>::provide
; Function Attrs: uwtable
define void @_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error7provide(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noundef nonnull align 8 %request.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) unnamed_addr #1 {
start:
  %_6.0 = load ptr, ptr %self, align 8, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_6.1 = load ptr, ptr %0, align 8, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_6.1, i64 80
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !nonnull !5
  tail call void %2(ptr noundef nonnull align 1 %_6.0, ptr noundef nonnull align 8 %request.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) #32
  ret void
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::error::Error>::source
; Function Attrs: uwtable
define internal { ptr, ptr } @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error6sourceB7_(ptr noundef nonnull align 8 %self) unnamed_addr #1 {
start:
  %_4.i = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i(ptr noundef nonnull %self)
  %_2.0.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i, null
  tail call void @llvm.assume(i1 %2)
  %_2.1 = extractvalue { ptr, ptr } %1, 1
  %3 = getelementptr inbounds nuw i8, ptr %_2.1, i64 48
  %4 = load ptr, ptr %3, align 8, !invariant.load !5, !nonnull !5
  %5 = tail call { ptr, ptr } %4(ptr noundef nonnull align 1 %_2.0.i) #32
  ret { ptr, ptr } %5
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::error::Error>::provide
; Function Attrs: uwtable
define internal void @_RNvXs7_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_(ptr noundef nonnull align 8 %self, ptr noundef nonnull align 8 %request.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %1 = load i64, ptr %0, align 8, !range !114, !noalias !316, !noundef !5
  %.not.i = icmp eq i64 %1, 3
  br i1 %.not.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit, label %bb1.i

bb1.i:                                            ; preds = %start
  %_18.sroa.0.0.copyload.i.i.i = load i128, ptr %request.0, align 8, !noalias !319
  %_11.i.i.i = icmp eq i128 %_18.sroa.0.0.copyload.i.i.i, 156425861031228512447185639436817036401
  br i1 %_11.i.i.i, label %bb8.i.i.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

bb8.i.i.i:                                        ; preds = %bb1.i
  %_14.i.i.i = getelementptr inbounds nuw i8, ptr %request.0, i64 16
  %2 = load ptr, ptr %_14.i.i.i, align 8, !noalias !319, !align !110, !noundef !5
  %.not.i.i.i = icmp eq ptr %2, null
  br i1 %.not.i.i.i, label %bb1.i.i.i, label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

bb1.i.i.i:                                        ; preds = %bb8.i.i.i
  store ptr %0, ptr %_14.i.i.i, align 8, !noalias !319
  br label %_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit

_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide.exit: ; preds = %start, %bb1.i, %bb8.i.i.i, %bb1.i.i.i
  %_4.i.i = load ptr, ptr %self, align 8, !noalias !316, !nonnull !5, !align !110, !noundef !5
  %3 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %3, align 8, !noalias !316, !nonnull !5, !noundef !5
  %4 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull %self), !noalias !316
  %_2.0.i.i = extractvalue { ptr, ptr } %4, 0
  %5 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %5)
  %_6.1.i = extractvalue { ptr, ptr } %4, 1
  %6 = getelementptr inbounds nuw i8, ptr %_6.1.i, i64 80
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !noalias !316, !nonnull !5
  tail call void %7(ptr noundef nonnull align 1 %_2.0.i.i, ptr noundef nonnull align 8 %request.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %request.1) #32
  ret void
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs8_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmtB7_(ptr noundef nonnull align 8 %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #1 {
start:
; call <anyhow::error::ErrorImpl>::debug
  %_0 = tail call noundef zeroext i1 @_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl5debug(ptr noundef nonnull %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter)
  ret i1 %_0
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs9_NtCs7g60D0Ppidu_6anyhow5errorINtB5_9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_(ptr noundef nonnull align 8 %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #1 {
start:
  %_4.i = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i, i64 8
  %_3.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i(ptr noundef nonnull %self)
  %_2.0.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i, null
  tail call void @llvm.assume(i1 %2)
  %_3.1 = extractvalue { ptr, ptr } %1, 1
  %3 = getelementptr inbounds nuw i8, ptr %_3.1, i64 32
  %4 = load ptr, ptr %3, align 8, !invariant.load !5, !nonnull !5
  %_0 = tail call noundef zeroext i1 %4(ptr noundef nonnull align 1 %_2.0.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter) #32
  ret i1 %_0
}

; <core::fmt::Error as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt(ptr noalias nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #6 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_99ac8a81a24cac863217ce4a5cbfabea, i64 noundef 5)
  ret i1 %_0
}

; <alloc::string::String as core::fmt::Write>::write_char
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #6 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !322)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i = load i64, ptr %0, align 8, !alias.scope !322, !noundef !5
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
  %self2.i.i = load i64, ptr %self, align 8, !range !30, !alias.scope !325, !noundef !5
  %_9.i.i = sub nsw i64 %self2.i.i, %len.i
  %_7.i.i = icmp ugt i64 %ch_len.sroa.0.0.i, %_9.i.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i, !prof !9

bb1.i.i:                                          ; preds = %bb2.i
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i, i64 noundef %ch_len.sroa.0.0.i)
  %count.pre.i = load i64, ptr %0, align 8, !alias.scope !322
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i: ; preds = %bb1.i.i, %bb2.i
  %count.i = phi i64 [ %len.i, %bb2.i ], [ %count.pre.i, %bb1.i.i ]
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20.i = load ptr, ptr %1, align 8, !alias.scope !322, !nonnull !5, !noundef !5
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  br i1 %_16.i, label %bb12.i.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i
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

bb12.i.i:                                         ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow.exit.i
  %5 = trunc nuw nsw i32 %c to i8
  store i8 %5, ptr %_8.i, align 1, !noalias !322
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb1.i2.i:                                         ; preds = %bb7.i.i
  %6 = or disjoint i8 %3, -64
  store i8 %6, ptr %_8.i, align 1, !noalias !322
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !noalias !322
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb2.i.i:                                          ; preds = %bb7.i.i
  %_28.i.i = icmp samesign ult i32 %c, 65536
  br i1 %_28.i.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %7 = or disjoint i8 %4, -32
  store i8 %7, ptr %_8.i, align 1, !noalias !322
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !noalias !322
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 1, !noalias !322
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_8.i, align 1, !noalias !322
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !noalias !322
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 1, !noalias !322
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !noalias !322
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit: ; preds = %bb12.i.i, %bb1.i2.i, %bb3.i.i, %bb4.i.i
  %new_len.i = add nuw i64 %ch_len.sroa.0.0.i, %len.i
  store i64 %new_len.i, ptr %0, align 8, !alias.scope !322
  ret i1 false
}

; <alloc::string::String as core::fmt::Write>::write_str
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef nonnull readonly align 1 captures(none) %s.0, i64 noundef %s.1) unnamed_addr #6 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !328)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !331)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i.i.i = load i64, ptr %0, align 8, !alias.scope !334, !noalias !337, !noundef !5
  %self2.i.i.i = load i64, ptr %self, align 8, !range !30, !alias.scope !334, !noalias !337, !noundef !5
  %_9.i.i.i = sub i64 %self2.i.i.i, %len.i.i.i
  %_7.i.i.i = icmp ugt i64 %s.1, %_9.i.i.i
  br i1 %_7.i.i.i, label %bb1.i.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit, !prof !9

bb1.i.i.i:                                        ; preds = %start
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs7g60D0Ppidu_6anyhow(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i.i.i, i64 noundef %s.1)
  %len.pre.i.i = load i64, ptr %0, align 8, !alias.scope !339, !noalias !337
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit: ; preds = %start, %bb1.i.i.i
  %len.i.i = phi i64 [ %len.i.i.i, %start ], [ %len.pre.i.i, %bb1.i.i.i ]
  %_9.i.i = icmp sgt i64 %len.i.i, -1
  tail call void @llvm.assume(i1 %_9.i.i)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_10.i.i = load ptr, ptr %1, align 8, !alias.scope !339, !noalias !337, !nonnull !5, !noundef !5
  %dst.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 %len.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i, ptr nonnull readonly align 1 %s.0, i64 %s.1, i1 false), !noalias !339
  %2 = add i64 %len.i.i, %s.1
  store i64 %2, ptr %0, align 8, !alias.scope !339, !noalias !337
  ret i1 false
}

; <anyhow::fmt::Indented<core::fmt::Formatter> as core::fmt::Write>::write_str
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtCs7g60D0Ppidu_6anyhow3fmtINtB4_8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBM_5Write9write_strB6_(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %args = alloca [16 x i8], align 8
  %number = alloca [8 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_12 = load i64, ptr %self, align 8, !range !187
  %1 = trunc nuw i64 %_12 to i1
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_26 = load ptr, ptr %2, align 8, !nonnull !5, !align !110
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %4 = load i64, ptr %3, align 8
  %_18.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  %5 = getelementptr i8, ptr %_26, i64 8
  %.not1 = icmp eq i64 %_12, 0
  %.promoted = load i8, ptr %0, align 8
  %extract.t = trunc i8 %.promoted to i1
  br label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %bb20, %start
  %.off0 = phi i1 [ %extract.t, %start ], [ true, %bb20 ]
  %6 = phi i64 [ 0, %start ], [ %_8.0.i, %bb20 ]
  %7 = phi i64 [ 0, %start ], [ %20, %bb20 ]
  %_3.sroa.2.sroa.3.sroa.3.0.copyload732 = phi i64 [ 0, %start ], [ %_3.sroa.2.sroa.3.sroa.3.0.copyload4, %bb20 ]
  %_4325.i.i.i.i = icmp ult i64 %s.1, %_3.sroa.2.sroa.3.sroa.3.0.copyload732
  br i1 %_4325.i.i.i.i, label %bb4, label %bb12.us.i.i.i.i

bb12.us.i.i.i.i:                                  ; preds = %bb2.i.i.i, %bb9.us.i.i.i.i
  %8 = phi i64 [ %17, %bb9.us.i.i.i.i ], [ %_3.sroa.2.sroa.3.sroa.3.0.copyload732, %bb2.i.i.i ]
  %new_len.us.i.i.i.i = sub nuw i64 %s.1, %8
  %_46.us.i.i.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %8
  %_3.i.us.i.i.i.i = icmp samesign ult i64 %new_len.us.i.i.i.i, 16
  br i1 %_3.i.us.i.i.i.i, label %bb5.preheader.i.us.i.i.i.i, label %bb2.i.us.i.i.i.i

bb2.i.us.i.i.i.i:                                 ; preds = %bb12.us.i.i.i.i
; call core::slice::memchr::memchr_aligned
  %9 = call { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef 10, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_46.us.i.i.i.i, i64 noundef range(i64 0, -9223372036854775808) %new_len.us.i.i.i.i), !noalias !340
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i

bb5.preheader.i.us.i.i.i.i:                       ; preds = %bb12.us.i.i.i.i
  %_64.not.i.us.i.i.i.i = icmp eq i64 %new_len.us.i.i.i.i, 0
  br i1 %_64.not.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb7.i.us.i.i.i.i

bb7.i.us.i.i.i.i:                                 ; preds = %bb5.preheader.i.us.i.i.i.i, %bb9.i.us.i.i.i.i
  %i.sroa.0.05.i.us.i.i.i.i = phi i64 [ %11, %bb9.i.us.i.i.i.i ], [ 0, %bb5.preheader.i.us.i.i.i.i ]
  %10 = getelementptr inbounds nuw i8, ptr %_46.us.i.i.i.i, i64 %i.sroa.0.05.i.us.i.i.i.i
  %_9.i.us.i.i.i.i = load i8, ptr %10, align 1, !alias.scope !351, !noalias !340, !noundef !5
  %_8.i.us.i.i.i.i = icmp eq i8 %_9.i.us.i.i.i.i, 10
  br i1 %_8.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb9.i.us.i.i.i.i

bb9.i.us.i.i.i.i:                                 ; preds = %bb7.i.us.i.i.i.i
  %11 = add nuw nsw i64 %i.sroa.0.05.i.us.i.i.i.i, 1
  %exitcond.not.i.us.i.i.i.i = icmp eq i64 %11, %new_len.us.i.i.i.i
  br i1 %exitcond.not.i.us.i.i.i.i, label %bb4.i.us.i.i.i.i, label %bb7.i.us.i.i.i.i

bb4.i.us.i.i.i.i:                                 ; preds = %bb9.i.us.i.i.i.i, %bb7.i.us.i.i.i.i, %bb5.preheader.i.us.i.i.i.i
  %i.sroa.0.0.lcssa.i.us.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i.i.i ], [ %new_len.us.i.i.i.i, %bb9.i.us.i.i.i.i ], [ %i.sroa.0.05.i.us.i.i.i.i, %bb7.i.us.i.i.i.i ]
  %_0.sroa.0.1.i.us.i.i.i.i = phi i64 [ 0, %bb5.preheader.i.us.i.i.i.i ], [ 0, %bb9.i.us.i.i.i.i ], [ 1, %bb7.i.us.i.i.i.i ]
  %12 = insertvalue { i64, i64 } poison, i64 %_0.sroa.0.1.i.us.i.i.i.i, 0
  %13 = insertvalue { i64, i64 } %12, i64 %i.sroa.0.0.lcssa.i.us.i.i.i.i, 1
  br label %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i

_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i: ; preds = %bb4.i.us.i.i.i.i, %bb2.i.us.i.i.i.i
  %.merged.i.us.i.i.i.i = phi { i64, i64 } [ %13, %bb4.i.us.i.i.i.i ], [ %9, %bb2.i.us.i.i.i.i ]
  %14 = extractvalue { i64, i64 } %.merged.i.us.i.i.i.i, 0
  %15 = trunc nuw i64 %14 to i1
  br i1 %15, label %bb4.us.i.i.i.i, label %bb4

bb4.us.i.i.i.i:                                   ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i
  %16 = extractvalue { i64, i64 } %.merged.i.us.i.i.i.i, 1
  %_16.us.i.i.i.i = add i64 %8, 1
  %17 = add i64 %_16.us.i.i.i.i, %16
  %_54.not.us.i.i.i.i = icmp ugt i64 %17, %s.1
  %18 = add i64 %16, %8
  %or.cond.i.i.i.i.not = icmp ult i64 %18, %s.1
  br i1 %or.cond.i.i.i.i.not, label %bb19.us.i.i.i.i, label %bb9.us.i.i.i.i

bb19.us.i.i.i.i:                                  ; preds = %bb4.us.i.i.i.i
  %_62.us.i.i.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %18
  %lhsc = load i8, ptr %_62.us.i.i.i.i, align 1
  %19 = icmp eq i8 %lhsc, 10
  br i1 %19, label %bb4, label %bb9.us.i.i.i.i

bb9.us.i.i.i.i:                                   ; preds = %bb19.us.i.i.i.i, %bb4.us.i.i.i.i
  br i1 %_54.not.us.i.i.i.i, label %bb4, label %bb12.us.i.i.i.i

bb4:                                              ; preds = %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i, %bb9.us.i.i.i.i, %bb19.us.i.i.i.i, %bb2.i.i.i
  %_3.sroa.2.sroa.3.sroa.3.0.copyload4 = phi i64 [ %_3.sroa.2.sroa.3.sroa.3.0.copyload732, %bb2.i.i.i ], [ %17, %bb19.us.i.i.i.i ], [ %s.1, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i ], [ %17, %bb9.us.i.i.i.i ]
  %.off013 = phi i1 [ true, %bb2.i.i.i ], [ false, %bb19.us.i.i.i.i ], [ true, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i ], [ true, %bb9.us.i.i.i.i ]
  %20 = phi i64 [ %7, %bb2.i.i.i ], [ %17, %bb19.us.i.i.i.i ], [ %7, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i ], [ %7, %bb9.us.i.i.i.i ]
  %found_char.us.i.i.i.i.pn = phi i64 [ %s.1, %bb2.i.i.i ], [ %18, %bb19.us.i.i.i.i ], [ %s.1, %_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr.exit.us.i.i.i.i ], [ %s.1, %bb9.us.i.i.i.i ]
  %_0.sroa.4.1.i.i.i = sub nuw i64 %found_char.us.i.i.i.i.pn, %7
  %_0.sroa.0.1.i.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %7
  %_8.0.i = add i64 %6, 1
  br i1 %.off0, label %bb11, label %bb6

bb26:                                             ; preds = %bb8, %bb7, %bb12, %bb14, %bb16, %bb20
  %_0.sroa.0.0.off0 = phi i1 [ true, %bb7 ], [ true, %bb12 ], [ true, %bb14 ], [ true, %bb16 ], [ %_25, %bb20 ], [ true, %bb8 ]
  ret i1 %_0.sroa.0.0.off0

bb6:                                              ; preds = %bb4
  store i8 1, ptr %0, align 8
  br i1 %1, label %bb8, label %bb7

bb11:                                             ; preds = %bb4
  %_21.not = icmp eq i64 %6, 0
  br i1 %_21.not, label %bb20, label %bb12

bb8:                                              ; preds = %bb6
  call void @llvm.lifetime.start.p0(i64 8, ptr nonnull %number)
  store i64 %4, ptr %number, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %number, ptr %args, align 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_18.sroa.4.0..sroa_idx, align 8
  %_27.val2 = load ptr, ptr %5, align 8, !nonnull !5, !noundef !5
  %_27.val = load ptr, ptr %_26, align 8, !nonnull !5, !noundef !5
; call core::fmt::write
  %21 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_27.val, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_27.val2, ptr noundef nonnull @alloc_d107a7a48d7efb04418b9bba8c2c8fa5, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 8, ptr nonnull %number)
  br i1 %21, label %bb26, label %bb20

bb7:                                              ; preds = %bb6
; call <core::fmt::Formatter as core::fmt::Write>::write_str
  %_20 = call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %_26, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_7926774156b504d85a3c261767c6333c, i64 noundef 4)
  br i1 %_20, label %bb26, label %bb20

bb20:                                             ; preds = %bb8, %bb16, %bb14, %bb11, %bb7
; call <core::fmt::Formatter as core::fmt::Write>::write_str
  %_25 = call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %_26, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.i.i, i64 noundef %_0.sroa.4.1.i.i.i)
  %brmerge = or i1 %_25, %.off013
  br i1 %brmerge, label %bb26, label %bb2.i.i.i

bb12:                                             ; preds = %bb11
; call <core::fmt::Formatter as core::fmt::Write>::write_char
  %_22 = call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write10write_char(ptr noalias noundef nonnull align 8 dereferenceable(24) %_26, i32 noundef 10)
  br i1 %_22, label %bb26, label %bb33

bb33:                                             ; preds = %bb12
  br i1 %.not1, label %bb16, label %bb14

bb14:                                             ; preds = %bb33
; call <core::fmt::Formatter as core::fmt::Write>::write_str
  %_23 = call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %_26, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b4e500608388755dc3eaea8bbe197789, i64 noundef 7)
  br i1 %_23, label %bb26, label %bb20

bb16:                                             ; preds = %bb33
; call <core::fmt::Formatter as core::fmt::Write>::write_str
  %_24 = call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %_26, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_7926774156b504d85a3c261767c6333c, i64 noundef 4)
  br i1 %_24, label %bb26, label %bb20
}

; <anyhow::Chain as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
define { ptr, ptr } @_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self) unnamed_addr #1 {
start:
  %0 = load ptr, ptr %self, align 8, !noundef !5
  %.not = icmp eq ptr %0, null
  br i1 %.not, label %bb3, label %bb2

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !354)
  %_14.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_12.i = load ptr, ptr %_14.i, align 8, !alias.scope !354, !nonnull !5, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_21.i = load ptr, ptr %1, align 8, !alias.scope !354, !nonnull !5, !noundef !5
  %_9.i = icmp eq ptr %_21.i, %_12.i
  br i1 %_9.i, label %bb5, label %bb6.i

bb6.i:                                            ; preds = %bb2
  %_23.i = getelementptr inbounds nuw i8, ptr %_21.i, i64 16
  store ptr %_23.i, ptr %1, align 8, !alias.scope !354
  %_17.0.i = load ptr, ptr %_21.i, align 8, !noalias !354, !nonnull !5, !align !106, !noundef !5
  %2 = getelementptr inbounds nuw i8, ptr %_21.i, i64 8
  %_17.1.i = load ptr, ptr %2, align 8, !noalias !354, !nonnull !5, !align !110, !noundef !5
  br label %bb5

bb3:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %4 = load ptr, ptr %3, align 8, !align !106, !noundef !5
  %.not1 = icmp eq ptr %4, null
  br i1 %.not1, label %bb5, label %bb7

bb7:                                              ; preds = %bb3
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %6 = load ptr, ptr %5, align 8, !nonnull !5, !noundef !5
  %7 = getelementptr inbounds nuw i8, ptr %6, i64 48
  %8 = load ptr, ptr %7, align 8, !invariant.load !5, !nonnull !5
  %9 = tail call { ptr, ptr } %8(ptr noundef nonnull align 1 %4) #32
  %_7.0 = extractvalue { ptr, ptr } %9, 0
  %_7.1 = extractvalue { ptr, ptr } %9, 1
  store ptr %_7.0, ptr %3, align 8
  store ptr %_7.1, ptr %5, align 8
  br label %bb5

bb5:                                              ; preds = %bb6.i, %bb2, %bb3, %bb7
  %_0.sroa.4.0 = phi ptr [ %6, %bb7 ], [ undef, %bb3 ], [ %_17.1.i, %bb6.i ], [ undef, %bb2 ]
  %_0.sroa.0.0 = phi ptr [ %4, %bb7 ], [ null, %bb3 ], [ %_17.0.i, %bb6.i ], [ null, %bb2 ]
  %10 = insertvalue { ptr, ptr } poison, ptr %_0.sroa.0.0, 0
  %11 = insertvalue { ptr, ptr } %10, ptr %_0.sroa.4.0, 1
  ret { ptr, ptr } %11
}

; <anyhow::Chain as core::iter::traits::iterator::Iterator>::size_hint
; Function Attrs: uwtable
define void @_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator9size_hint(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %self) unnamed_addr #1 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !357)
  %0 = load ptr, ptr %self, align 8, !alias.scope !357, !noundef !5
  %.not.i = icmp eq ptr %0, null
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %2 = load ptr, ptr %1, align 8, !alias.scope !357, !noundef !5
  br i1 %.not.i, label %bb3.i, label %bb10.i

bb3.i:                                            ; preds = %start
  %.not46.i = icmp eq ptr %2, null
  br i1 %.not46.i, label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len.exit, label %bb5.preheader.i

bb5.preheader.i:                                  ; preds = %bb3.i
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %4 = load ptr, ptr %3, align 8, !alias.scope !357
  br label %bb5.i

bb5.i:                                            ; preds = %bb5.i, %bb5.preheader.i
  %len.sroa.0.09.i = phi i64 [ %11, %bb5.i ], [ 0, %bb5.preheader.i ]
  %next.sroa.0.08.i = phi ptr [ %9, %bb5.i ], [ %2, %bb5.preheader.i ]
  %next.sroa.4.07.i = phi ptr [ %10, %bb5.i ], [ %4, %bb5.preheader.i ]
  %5 = icmp ne ptr %next.sroa.4.07.i, null
  tail call void @llvm.assume(i1 %5)
  %6 = getelementptr inbounds nuw i8, ptr %next.sroa.4.07.i, i64 48
  %7 = load ptr, ptr %6, align 8, !invariant.load !5, !noalias !357, !nonnull !5
  %8 = tail call { ptr, ptr } %7(ptr noundef nonnull align 1 %next.sroa.0.08.i) #32, !noalias !357
  %9 = extractvalue { ptr, ptr } %8, 0
  %10 = extractvalue { ptr, ptr } %8, 1
  %11 = add i64 %len.sroa.0.09.i, 1
  %.not4.i = icmp eq ptr %9, null
  br i1 %.not4.i, label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len.exit, label %bb5.i

bb10.i:                                           ; preds = %start
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %self.val5.i = load ptr, ptr %12, align 8, !alias.scope !357, !nonnull !5, !noundef !5
  %13 = ptrtoint ptr %self.val5.i to i64
  %14 = ptrtoint ptr %2 to i64
  %15 = sub nuw i64 %13, %14
  %16 = lshr exact i64 %15, 4
  br label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len.exit

_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len.exit: ; preds = %bb5.i, %bb3.i, %bb10.i
  %len.sroa.0.1.i = phi i64 [ %16, %bb10.i ], [ 0, %bb3.i ], [ %11, %bb5.i ]
  store i64 %len.sroa.0.1.i, ptr %_0, align 8
  %17 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 1, ptr %17, align 8
  %18 = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %len.sroa.0.1.i, ptr %18, align 8
  ret void
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !360)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8.i = load ptr, ptr %0, align 8, !alias.scope !360, !noalias !363, !nonnull !5, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7.i = load i64, ptr %1, align 8, !alias.scope !360, !noalias !363, !noundef !5
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8.i, i64 noundef %_7.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f), !noalias !360
  ret i1 %_0.i
}

; <anyhow::wrapper::MessageError<&str> as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs_NtCs7g60D0Ppidu_6anyhow7wrapperINtB4_12MessageErrorReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #1 {
start:
  %self.val = load ptr, ptr %self, align 8, !nonnull !5, !align !106, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val1 = load i64, ptr %0, align 8, !noundef !5
; call <str as core::fmt::Display>::fmt
  %_0.i = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.val, i64 noundef %self.val1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0.i
}

; <alloc::boxed::Box<dyn core::error::Error + core::marker::Send + core::marker::Sync> as core::convert::From<anyhow::Error>>::from
; Function Attrs: cold uwtable
define { ptr, ptr } @_RNvXsa_NtCs7g60D0Ppidu_6anyhow5errorINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxDNtNtCsjMrxcFdYDNN_4core5error5ErrorNtNtB1c_6marker4SendNtB1J_4SyncEL_EINtNtB1c_7convert4FromNtB7_5ErrorE4from(ptr noundef nonnull %error) unnamed_addr #0 {
start:
  %_3.i = load ptr, ptr %error, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  %_2.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_2.i(ptr noundef nonnull %error)
  ret { ptr, ptr } %1
}

; <anyhow::Error as core::convert::AsRef<dyn core::error::Error + core::marker::Send + core::marker::Sync>>::as_ref
; Function Attrs: uwtable
define { ptr, ptr } @_RNvXsd_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorINtNtCsjMrxcFdYDNN_4core7convert5AsRefDNtNtBO_5error5ErrorNtNtBO_6marker4SendNtB1H_4SyncEL_E6as_ref(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self) unnamed_addr #1 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !365)
  %_3.i = load ptr, ptr %self, align 8, !alias.scope !365, !nonnull !5, !noundef !5
  %_4.i.i = load ptr, ptr %_3.i, align 8, !noalias !365, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %0, align 8, !noalias !365, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull %_3.i), !noalias !365
  %_2.0.i.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %2)
  ret { ptr, ptr } %1
}

; <alloc::string::String as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #6 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7 = load i64, ptr %1, align 8, !noundef !5
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8, i64 noundef %_7, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <anyhow::fmt::Indented<core::fmt::Formatter> as core::fmt::Write>::write_char
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYINtNtCs7g60D0Ppidu_6anyhow3fmt8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBH_5Write10write_charB7_(ptr noalias noundef align 8 captures(none) dereferenceable(32) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #1 {
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
  store i8 %3, ptr %_6, align 4, !alias.scope !368
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb1.i.i:                                          ; preds = %bb5.i
  %4 = or disjoint i8 %1, -64
  store i8 %4, ptr %_6, align 4, !alias.scope !368
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !alias.scope !368
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb2.i.i:                                          ; preds = %bb5.i
  %_13.i = icmp samesign ult i32 %c, 65536
  br i1 %_13.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %5 = or disjoint i8 %2, -32
  store i8 %5, ptr %_6, align 4, !alias.scope !368
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !alias.scope !368
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 2, !alias.scope !368
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_6, align 4, !alias.scope !368
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !alias.scope !368
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 2, !alias.scope !368
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_6, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !alias.scope !368
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit: ; preds = %bb12.i.i, %bb1.i.i, %bb3.i.i, %bb4.i.i
  %len.sroa.0.04.i = phi i64 [ 1, %bb12.i.i ], [ 2, %bb1.i.i ], [ 3, %bb3.i.i ], [ 4, %bb4.i.i ]
; call <anyhow::fmt::Indented<core::fmt::Formatter> as core::fmt::Write>::write_str
  %_0 = call noundef zeroext i1 @_RNvXs_NtCs7g60D0Ppidu_6anyhow3fmtINtB4_8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBM_5Write9write_strB6_(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_6, i64 noundef %len.sroa.0.04.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_6)
  ret i1 %_0
}

; <anyhow::fmt::Indented<core::fmt::Formatter> as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYINtNtCs7g60D0Ppidu_6anyhow3fmt8IndentedNtNtCsjMrxcFdYDNN_4core3fmt9FormatterENtBH_5Write9write_fmtB7_(ptr noalias noundef align 8 dereferenceable(32) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #1 {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.7, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::error::Error>::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, i64 } @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_(ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #3 {
start:
  ret { ptr, i64 } { ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, i64 40 }
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::error::Error>::cause
; Function Attrs: uwtable
define internal { ptr, ptr } @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_(ptr noundef nonnull align 8 %self) unnamed_addr #1 {
start:
  %_4.i.i = load ptr, ptr %self, align 8, !nonnull !5, !align !110, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 8
  %_3.i.i = load ptr, ptr %0, align 8, !nonnull !5, !noundef !5
  %1 = tail call { ptr, ptr } %_3.i.i(ptr noundef nonnull align 8 %self)
  %_2.0.i.i = extractvalue { ptr, ptr } %1, 0
  %2 = icmp ne ptr %_2.0.i.i, null
  tail call void @llvm.assume(i1 %2)
  %_2.1.i = extractvalue { ptr, ptr } %1, 1
  %3 = getelementptr inbounds nuw i8, ptr %_2.1.i, i64 48
  %4 = load ptr, ptr %3, align 8, !invariant.load !5, !nonnull !5
  %5 = tail call { ptr, ptr } %4(ptr noundef nonnull align 1 %_2.0.i.i) #32
  ret { ptr, ptr } %5
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<alloc::string::String>> as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.1, i64 16, i1 false)
  ret void
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::MessageError<&str>> as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtB7_7wrapper12MessageErrorReEENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.2, i64 16, i1 false)
  ret void
}

; <anyhow::error::ErrorImpl<anyhow::wrapper::BoxedError> as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtB7_7wrapper10BoxedErrorENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr nonnull readnone align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.3, i64 16, i1 false)
  ret void
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::error::Error>::description
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, i64 } @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error11descriptionB7_(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #3 {
start:
  ret { ptr, i64 } { ptr @alloc_04d7ce44d7c86a9a02b346ab945bf155, i64 40 }
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::error::Error>::cause
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal { ptr, ptr } @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error5causeB7_(ptr noalias readonly align 8 captures(none) %self) unnamed_addr #3 {
start:
  ret { ptr, ptr } { ptr null, ptr undef }
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::error::Error>::provide
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error7provideB7_(ptr noalias readonly align 8 captures(none) %self, ptr nonnull readnone align 8 captures(none) %request.0, ptr noalias readonly align 8 captures(none) %request.1) unnamed_addr #3 {
start:
  ret void
}

; <anyhow::wrapper::MessageError<alloc::string::String> as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.4, i64 16, i1 false)
  ret void
}

; <anyhow::wrapper::MessageError<&str> as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorReENtNtCsjMrxcFdYDNN_4core5error5Error7type_idB7_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.5, i64 16, i1 false)
  ret void
}

; <anyhow::ensure::Buf as core::fmt::Write>::write_char
; Function Attrs: nofree norecurse nosync nounwind memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable
define internal noundef zeroext i1 @_RNvYNtNtCs7g60D0Ppidu_6anyhow6ensure3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_charB6_(ptr noalias noundef align 8 captures(none) dereferenceable(48) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #11 personality ptr @rust_eh_personality {
start:
  %_6 = alloca [4 x i8], align 4
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_6)
  store i32 0, ptr %_6, align 4
  %_11.i = icmp samesign ult i32 %c, 128
  %len.sroa.0.04.i.sroa.gep = getelementptr inbounds nuw i8, ptr %_6, i64 1
  %len.sroa.0.04.i.sroa.gep4 = getelementptr inbounds nuw i8, ptr %_6, i64 2
  %len.sroa.0.04.i.sroa.gep5 = getelementptr inbounds nuw i8, ptr %_6, i64 3
  %len.sroa.0.04.i.sroa.gep6 = getelementptr inbounds nuw i8, ptr %_6, i64 4
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
  store i8 %3, ptr %_6, align 4, !alias.scope !371
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb1.i.i:                                          ; preds = %bb5.i
  %4 = or disjoint i8 %1, -64
  store i8 %4, ptr %_6, align 4, !alias.scope !371
  store i8 %last1.i.i, ptr %len.sroa.0.04.i.sroa.gep, align 1, !alias.scope !371
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb2.i.i:                                          ; preds = %bb5.i
  %_13.i = icmp samesign ult i32 %c, 65536
  br i1 %_13.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %5 = or disjoint i8 %2, -32
  store i8 %5, ptr %_6, align 4, !alias.scope !371
  store i8 %last2.i.i, ptr %len.sroa.0.04.i.sroa.gep, align 1, !alias.scope !371
  store i8 %last1.i.i, ptr %len.sroa.0.04.i.sroa.gep4, align 2, !alias.scope !371
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_6, align 4, !alias.scope !371
  store i8 %last3.i.i, ptr %len.sroa.0.04.i.sroa.gep, align 1, !alias.scope !371
  store i8 %last2.i.i, ptr %len.sroa.0.04.i.sroa.gep4, align 2, !alias.scope !371
  store i8 %last1.i.i, ptr %len.sroa.0.04.i.sroa.gep5, align 1, !alias.scope !371
  br label %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit

_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit: ; preds = %bb12.i.i, %bb1.i.i, %bb3.i.i, %bb4.i.i
  %len.sroa.0.04.i.sroa.phi = phi ptr [ %len.sroa.0.04.i.sroa.gep, %bb12.i.i ], [ %len.sroa.0.04.i.sroa.gep4, %bb1.i.i ], [ %len.sroa.0.04.i.sroa.gep5, %bb3.i.i ], [ %len.sroa.0.04.i.sroa.gep6, %bb4.i.i ]
  %len.sroa.0.04.i = phi i64 [ 1, %bb12.i.i ], [ 2, %bb1.i.i ], [ 3, %bb3.i.i ], [ 4, %bb4.i.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !374)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !377)
  br label %bb1.i.i1

bb1.i.i1:                                         ; preds = %bb3.i.i2, %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit
  %_16.i3.i.i = phi ptr [ %_16.i.i.i, %bb3.i.i2 ], [ %_6, %_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw.exit ]
  %_6.i.not.not.not.i.not.i = icmp eq ptr %_16.i3.i.i, %len.sroa.0.04.i.sroa.phi
  br i1 %_6.i.not.not.not.i.not.i, label %bb2.i, label %bb3.i.i2

bb3.i.i2:                                         ; preds = %bb1.i.i1
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i3.i.i, i64 1
  %.val.i.i = load i8, ptr %_16.i3.i.i, align 1, !alias.scope !377, !noalias !379, !noundef !5
  switch i8 %.val.i.i, label %bb1.i.i1 [
    i8 32, label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str.exit
    i8 10, label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str.exit
  ]

bb2.i:                                            ; preds = %bb1.i.i1
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 40
  %_6.i = load i64, ptr %6, align 8, !alias.scope !374, !noalias !377, !noundef !5
  %remaining.i = sub i64 40, %_6.i
  %_7.i = icmp ugt i64 %len.sroa.0.04.i, %remaining.i
  br i1 %_7.i, label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str.exit, label %bb4.i

bb4.i:                                            ; preds = %bb2.i
  %_10.i = getelementptr inbounds nuw i8, ptr %self, i64 %_6.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(1) %_10.i, ptr noundef nonnull readonly align 4 dereferenceable(1) %_6, i64 %len.sroa.0.04.i, i1 false), !alias.scope !382
  %7 = load i64, ptr %6, align 8, !alias.scope !374, !noalias !377, !noundef !5
  %8 = add i64 %7, %len.sroa.0.04.i
  store i64 %8, ptr %6, align 8, !alias.scope !374, !noalias !377
  br label %_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str.exit

_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str.exit: ; preds = %bb3.i.i2, %bb3.i.i2, %bb2.i, %bb4.i
  %_0.sroa.0.0.off0.i = phi i1 [ false, %bb4.i ], [ true, %bb2.i ], [ true, %bb3.i.i2 ], [ true, %bb3.i.i2 ]
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_6)
  ret i1 %_0.sroa.0.0.off0.i
}

; <anyhow::ensure::Buf as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYNtNtCs7g60D0Ppidu_6anyhow6ensure3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtB6_(ptr noalias noundef align 8 dereferenceable(48) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(48) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.a, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

; <anyhow::wrapper::BoxedError as core::error::Error>::cause
; Function Attrs: uwtable
define internal { ptr, ptr } @_RNvYNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error5causeB6_(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self) unnamed_addr #1 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !383)
  %_4.0.i = load ptr, ptr %self, align 8, !alias.scope !383, !nonnull !5, !noundef !5
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_4.1.i = load ptr, ptr %0, align 8, !alias.scope !383, !nonnull !5, !align !110, !noundef !5
  %1 = getelementptr inbounds nuw i8, ptr %_4.1.i, i64 48
  %2 = load ptr, ptr %1, align 8, !invariant.load !5, !noalias !383, !nonnull !5
  %3 = tail call { ptr, ptr } %2(ptr noundef nonnull align 1 %_4.0.i) #32, !noalias !383
  ret { ptr, ptr } %3
}

; <anyhow::wrapper::BoxedError as core::error::Error>::type_id
; Function Attrs: mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable
define internal void @_RNvYNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error7type_idB6_(ptr dead_on_unwind noalias noundef writable writeonly sret([16 x i8]) align 8 captures(none) dereferenceable(16) initializes((0, 16)) %_0, ptr noalias readonly align 8 captures(none) %self) unnamed_addr #10 {
start:
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(16) %_0, ptr noundef nonnull align 8 dereferenceable(16) @anon.18f4560d4c35da8c0573dbe31cce292b.6, i64 16, i1 false)
  ret void
}

; <alloc::string::String as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs7g60D0Ppidu_6anyhow(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #1 {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(24) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.c, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #12

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #5

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #13

; <std::backtrace::Backtrace>::capture
; Function Attrs: noinline uwtable
declare void @_RNvMs2_NtCs5sEH5CPMdak_3std9backtraceNtB5_9Backtrace7capture(ptr dead_on_unwind noalias noundef writable sret([48 x i8]) align 8 captures(address) dereferenceable(48)) unnamed_addr #14

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #13

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #15

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #16

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking9panic_fmt(ptr noundef nonnull, ptr noundef nonnull, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #17

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #18

; core::slice::index::slice_index_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core5slice5index16slice_index_fail(i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #17

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #19

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #1

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #20

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: write)
declare void @llvm.memset.p0.i64(ptr writeonly captures(none), i8, i64, i1 immarg) #21

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #5

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #22

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking5panic(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #17

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #17

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #23

; core::option::expect_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13expect_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #17

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #19

; core::slice::memchr::memchr_aligned
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr14memchr_aligned(i8 noundef, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #1

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #24

; <str>::escape_debug
; Function Attrs: uwtable
declare void @_RNvMNtCsjMrxcFdYDNN_4core3stre12escape_debug(ptr dead_on_unwind noalias noundef writable sret([120 x i8]) align 8 captures(none) dereferenceable(120), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::str::iter::EscapeDebug as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs20_NtNtCsjMrxcFdYDNN_4core3str4iterNtB6_11EscapeDebugNtNtBa_3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(120), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <std::backtrace::Backtrace as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs4_NtCs5sEH5CPMdak_3std9backtraceNtB5_9BacktraceNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noundef nonnull align 8, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #1

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::fmt::Formatter as core::fmt::Write>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #1

; <core::fmt::Formatter as core::fmt::Write>::write_char
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write10write_char(ptr noalias noundef align 8 dereferenceable(24), i32 noundef range(i32 0, 1114112)) unnamed_addr #1

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #25

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #26

attributes #0 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { nofree norecurse nosync nounwind memory(read, inaccessiblemem: write) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { mustprogress nofree norecurse nosync nounwind willreturn memory(none) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: read) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #9 = { nofree norecurse nosync nounwind memory(argmem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { mustprogress nofree norecurse nosync nounwind willreturn memory(argmem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { nofree norecurse nosync nounwind memory(argmem: readwrite, inaccessiblemem: readwrite) uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #13 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #14 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #16 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #17 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #18 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #20 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #21 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: write) }
attributes #22 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #23 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #24 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #25 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #26 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #27 = { nounwind }
attributes #28 = { noreturn }
attributes #29 = { cold }
attributes #30 = { cold noreturn nounwind }
attributes #31 = { noinline noreturn }
attributes #32 = { inlinehint }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!4 = distinct !{!4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!5 = !{}
!6 = !{!7}
!7 = distinct !{!7, !8, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorNtNtB4_6string6StringEEE3newBJ_: %x"}
!8 = distinct !{!8, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorNtNtB4_6string6StringEEE3newBJ_"}
!9 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorReEEE3newBJ_: %x"}
!12 = distinct !{!12, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBJ_7wrapper12MessageErrorReEEE3newBJ_"}
!13 = !{!14}
!14 = distinct !{!14, !15, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBJ_7wrapper10BoxedErrorEE3newBJ_: %x"}
!15 = distinct !{!15, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplNtNtBJ_7wrapper10BoxedErrorEE3newBJ_"}
!16 = !{!17, !19}
!17 = distinct !{!17, !18, !"_RINvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechE5drainINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs7g60D0Ppidu_6anyhow: %self"}
!18 = distinct !{!18, !"_RINvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechE5drainINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs7g60D0Ppidu_6anyhow"}
!19 = distinct !{!19, !20, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE6spliceINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtBW_3str4iter5BytesECs7g60D0Ppidu_6anyhow: %self"}
!20 = distinct !{!20, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE6spliceINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtBW_3str4iter5BytesECs7g60D0Ppidu_6anyhow"}
!21 = !{!22, !23}
!22 = distinct !{!22, !18, !"_RINvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechE5drainINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejEECs7g60D0Ppidu_6anyhow: %_0"}
!23 = distinct !{!23, !20, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE6spliceINtNtNtCsjMrxcFdYDNN_4core3ops5range5RangejENtNtNtBW_3str4iter5BytesECs7g60D0Ppidu_6anyhow: %_0"}
!24 = !{!25}
!25 = distinct !{!25, !26, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VechEINtB4_10SpecExtendhQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesE11spec_extendCs7g60D0Ppidu_6anyhow: %self"}
!26 = distinct !{!26, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VechEINtB4_10SpecExtendhQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesE11spec_extendCs7g60D0Ppidu_6anyhow"}
!27 = !{!28}
!28 = distinct !{!28, !29, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE14extend_trustedQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesECs7g60D0Ppidu_6anyhow: %self"}
!29 = distinct !{!29, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE14extend_trustedQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesECs7g60D0Ppidu_6anyhow"}
!30 = !{i64 0, i64 -9223372036854775808}
!31 = !{!32, !28, !25}
!32 = distinct !{!32, !33, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow: %self"}
!33 = distinct !{!33, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow"}
!34 = !{!35, !36, !37, !39}
!35 = distinct !{!35, !29, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VechE14extend_trustedQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesECs7g60D0Ppidu_6anyhow: argument 1"}
!36 = distinct !{!36, !26, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB6_3VechEINtB4_10SpecExtendhQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesE11spec_extendCs7g60D0Ppidu_6anyhow: %iterator"}
!37 = distinct !{!37, !38, !"_RNvXs1_NtNtCsdJPVW0sQgAG_5alloc3vec6spliceINtB5_6SpliceNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesENtNtNtBX_3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow: %self"}
!38 = distinct !{!38, !"_RNvXs1_NtNtCsdJPVW0sQgAG_5alloc3vec6spliceINtB5_6SpliceNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesENtNtNtBX_3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow"}
!39 = distinct !{!39, !40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec6splice6SpliceNtNtNtB4_3str4iter5BytesEECs7g60D0Ppidu_6anyhow: %_1"}
!40 = distinct !{!40, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec6splice6SpliceNtNtNtB4_3str4iter5BytesEECs7g60D0Ppidu_6anyhow"}
!41 = !{!28, !25}
!42 = !{!43, !45, !47, !49, !51, !52, !54, !55, !57, !58, !60, !28, !35, !25, !36, !37, !39}
!43 = distinct !{!43, !44, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VechE14extend_trustedQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesE0Cs7g60D0Ppidu_6anyhow: %_1"}
!44 = distinct !{!44, !"_RNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB8_3VechE14extend_trustedQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesE0Cs7g60D0Ppidu_6anyhow"}
!45 = distinct !{!45, !46, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VechE14extend_trustedQNtNtNtBe_3str4iter5BytesE0E0Cs7g60D0Ppidu_6anyhow: %_1"}
!46 = distinct !{!46, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1p_3VechE14extend_trustedQNtNtNtBe_3str4iter5BytesE0E0Cs7g60D0Ppidu_6anyhow"}
!47 = distinct !{!47, !48, !"_RNCINvMs0_NtNtCsjMrxcFdYDNN_4core3ops9try_traitINtB8_17NeverShortCircuituE10wrap_mut_2uhNCINvNvNtNtNtNtBc_4iter6traits8iterator8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2y_3VechE14extend_trustedQNtNtNtBc_3str4iter5BytesE0E0E0Cs7g60D0Ppidu_6anyhow: %_1"}
!48 = distinct !{!48, !"_RNCINvMs0_NtNtCsjMrxcFdYDNN_4core3ops9try_traitINtB8_17NeverShortCircuituE10wrap_mut_2uhNCINvNvNtNtNtNtBc_4iter6traits8iterator8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2y_3VechE14extend_trustedQNtNtNtBc_3str4iter5BytesE0E0E0Cs7g60D0Ppidu_6anyhow"}
!49 = distinct !{!49, !50, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvMs0_NtNtB9_3ops9try_traitINtB1F_17NeverShortCircuituE10wrap_mut_2uhNCINvNvBH_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3e_3VechE14extend_trustedQB3_E0E0E0B20_ECs7g60D0Ppidu_6anyhow: %self"}
!50 = distinct !{!50, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvMs0_NtNtB9_3ops9try_traitINtB1F_17NeverShortCircuituE10wrap_mut_2uhNCINvNvBH_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3e_3VechE14extend_trustedQB3_E0E0E0B20_ECs7g60D0Ppidu_6anyhow"}
!51 = distinct !{!51, !50, !"_RINvYNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtB9_4iter6traits8iterator8Iterator8try_folduNCINvMs0_NtNtB9_3ops9try_traitINtB1F_17NeverShortCircuituE10wrap_mut_2uhNCINvNvBH_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3e_3VechE14extend_trustedQB3_E0E0E0B20_ECs7g60D0Ppidu_6anyhow: %f"}
!52 = distinct !{!52, !53, !"_RINvXs2_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_15IteratorRefSpec9spec_folduNCINvNvNtB6_8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2v_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow: %self"}
!53 = distinct !{!53, !"_RINvXs2_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_15IteratorRefSpec9spec_folduNCINvNvNtB6_8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2v_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow"}
!54 = distinct !{!54, !53, !"_RINvXs2_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_15IteratorRefSpec9spec_folduNCINvNvNtB6_8Iterator8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB2v_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow: %fold"}
!55 = distinct !{!55, !56, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_8Iterator4folduNCINvNvB1f_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB28_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow: %self"}
!56 = distinct !{!56, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_8Iterator4folduNCINvNvB1f_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB28_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow"}
!57 = distinct !{!57, !56, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter6traits8iteratorQNtNtNtBc_3str4iter5BytesNtB6_8Iterator4folduNCINvNvB1f_8for_each4callhNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB28_3VechE14extend_trustedBQ_E0E0ECs7g60D0Ppidu_6anyhow: %f"}
!58 = distinct !{!58, !59, !"_RINvYQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1F_3VechE14extend_trustedB3_E0ECs7g60D0Ppidu_6anyhow: %self"}
!59 = distinct !{!59, !"_RINvYQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1F_3VechE14extend_trustedB3_E0ECs7g60D0Ppidu_6anyhow"}
!60 = distinct !{!60, !59, !"_RINvYQNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesNtNtNtNtBa_4iter6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB1F_3VechE14extend_trustedB3_E0ECs7g60D0Ppidu_6anyhow: %f"}
!61 = !{!62, !37, !39}
!62 = distinct !{!62, !63, !"_RINvMs2_NtNtCsdJPVW0sQgAG_5alloc3vec6spliceINtNtB8_5drain5DrainhE4fillNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesECs7g60D0Ppidu_6anyhow: %replace_with"}
!63 = distinct !{!63, !"_RINvMs2_NtNtCsdJPVW0sQgAG_5alloc3vec6spliceINtNtB8_5drain5DrainhE4fillNtNtNtCsjMrxcFdYDNN_4core3str4iter5BytesECs7g60D0Ppidu_6anyhow"}
!64 = !{!39}
!65 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!66 = !{!67, !69, !71, !73, !74, !76, !77, !79}
!67 = distinct !{!67, !68, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECs7g60D0Ppidu_6anyhow: %bytes"}
!68 = distinct !{!68, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations23next_code_point_reverseINtNtNtB6_5slice4iter4IterhEECs7g60D0Ppidu_6anyhow"}
!69 = distinct !{!69, !70, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back: %self"}
!70 = distinct !{!70, !"_RNvXs4_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits12double_ended19DoubleEndedIterator9next_back"}
!71 = distinct !{!71, !72, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs7g60D0Ppidu_6anyhow: %_0"}
!72 = distinct !{!72, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs7g60D0Ppidu_6anyhow"}
!73 = distinct !{!73, !72, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher9next_backCs7g60D0Ppidu_6anyhow: %self"}
!74 = distinct !{!74, !75, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow: %_0"}
!75 = distinct !{!75, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow"}
!76 = distinct !{!76, !75, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow: %self"}
!77 = distinct !{!77, !78, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow: %_0"}
!78 = distinct !{!78, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow"}
!79 = distinct !{!79, !78, !"_RNvXsp_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNvMNtNtB9_4char7methodsc13is_whitespaceENtB5_15ReverseSearcher16next_reject_backCs7g60D0Ppidu_6anyhow: %self"}
!80 = !{!71, !73, !74, !76, !77, !79}
!81 = !{!82}
!82 = distinct !{!82, !83, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_: %_1"}
!83 = distinct !{!83, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow5error9ErrorImplINtNtBL_7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEEBL_"}
!84 = !{!85}
!85 = distinct !{!85, !86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_: %_1"}
!86 = distinct !{!86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_"}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!89 = distinct !{!89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!90 = !{!88, !85, !82}
!91 = !{!92}
!92 = distinct !{!92, !93, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_: %_1"}
!93 = distinct !{!93, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_"}
!94 = !{!95}
!95 = distinct !{!95, !96, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!96 = distinct !{!96, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!97 = !{!95, !92, !82}
!98 = !{!99}
!99 = distinct !{!99, !100, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtB4_6string6StringEE3newBJ_: %x"}
!100 = distinct !{!100, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtB4_6string6StringEE3newBJ_"}
!101 = !{!102, !104}
!102 = distinct !{!102, !103, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!103 = distinct !{!103, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!104 = distinct !{!104, !105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_: %_1"}
!105 = distinct !{!105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_"}
!106 = !{i64 1}
!107 = !{!108}
!108 = distinct !{!108, !109, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorReEE3newBJ_: %x.0"}
!109 = distinct !{!109, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorReEE3newBJ_"}
!110 = !{i64 8}
!111 = !{!112}
!112 = distinct !{!112, !113, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorE3newBI_: argument 0"}
!113 = distinct !{!113, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorE3newBI_"}
!114 = !{i64 0, i64 4}
!115 = !{!116}
!116 = distinct !{!116, !117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceECs7g60D0Ppidu_6anyhow: %_1"}
!117 = distinct !{!117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceECs7g60D0Ppidu_6anyhow"}
!118 = !{!119}
!119 = distinct !{!119, !120, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace5InnerECs7g60D0Ppidu_6anyhow: %_1"}
!120 = distinct !{!120, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace5InnerECs7g60D0Ppidu_6anyhow"}
!121 = !{!122}
!122 = distinct !{!122, !123, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync9lazy_lock8LazyLockNtNtBN_9backtrace7CaptureNCNvNtB1v_6helper12lazy_resolve0EECs7g60D0Ppidu_6anyhow: %_1"}
!123 = distinct !{!123, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCs5sEH5CPMdak_3std4sync9lazy_lock8LazyLockNtNtBN_9backtrace7CaptureNCNvNtB1v_6helper12lazy_resolve0EECs7g60D0Ppidu_6anyhow"}
!124 = !{!125}
!125 = distinct !{!125, !126, !"_RNvXs0_NtNtCs5sEH5CPMdak_3std4sync9lazy_lockINtB5_8LazyLockNtNtB9_9backtrace7CaptureNCNvNtBX_6helper12lazy_resolve0ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow: %self"}
!126 = distinct !{!126, !"_RNvXs0_NtNtCs5sEH5CPMdak_3std4sync9lazy_lockINtB5_8LazyLockNtNtB9_9backtrace7CaptureNCNvNtBX_6helper12lazy_resolve0ENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs7g60D0Ppidu_6anyhow"}
!127 = !{!125, !122, !119, !116}
!128 = !{!"branch_weights", i32 1, i32 2000, i32 2000, i32 2000}
!129 = !{!130}
!130 = distinct !{!130, !131, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace7CaptureECs7g60D0Ppidu_6anyhow: %_1"}
!131 = distinct !{!131, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace7CaptureECs7g60D0Ppidu_6anyhow"}
!132 = !{!133}
!133 = distinct !{!133, !134, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameEECs7g60D0Ppidu_6anyhow: %_1"}
!134 = distinct !{!134, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameEECs7g60D0Ppidu_6anyhow"}
!135 = !{!133, !130, !122, !119, !116}
!136 = !{!137}
!137 = distinct !{!137, !138, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow: %_1"}
!138 = distinct !{!138, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace14BacktraceFrameECs7g60D0Ppidu_6anyhow"}
!139 = !{!140}
!140 = distinct !{!140, !141, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolEECs7g60D0Ppidu_6anyhow: %_1"}
!141 = distinct !{!141, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolEECs7g60D0Ppidu_6anyhow"}
!142 = !{!140, !137}
!143 = !{!144}
!144 = distinct !{!144, !145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow: %_1.0"}
!145 = distinct !{!145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow"}
!146 = !{!147}
!147 = distinct !{!147, !148, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow: %_1"}
!148 = distinct !{!148, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace15BacktraceSymbolECs7g60D0Ppidu_6anyhow"}
!149 = !{i64 0, i64 -9223372036854775807}
!150 = !{!147, !144}
!151 = !{!140, !137, !133, !130, !122, !119, !116}
!152 = !{!147, !144, !140, !137, !133, !130, !122, !119, !116}
!153 = !{!154}
!154 = distinct !{!154, !155, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace11BytesOrWideEECs7g60D0Ppidu_6anyhow: %_1"}
!155 = distinct !{!155, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtCs5sEH5CPMdak_3std9backtrace11BytesOrWideEECs7g60D0Ppidu_6anyhow"}
!156 = !{i64 0, i64 3}
!157 = !{!154, !147, !144}
!158 = !{!159}
!159 = distinct !{!159, !160, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace11BytesOrWideECs7g60D0Ppidu_6anyhow: %_1"}
!160 = distinct !{!160, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs5sEH5CPMdak_3std9backtrace11BytesOrWideECs7g60D0Ppidu_6anyhow"}
!161 = !{!159, !154, !147, !144}
!162 = !{!159, !154, !147, !144, !140, !137, !133, !130, !122, !119, !116}
!163 = !{!164}
!164 = distinct !{!164, !165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_: %_1"}
!165 = distinct !{!165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_"}
!166 = !{!167}
!167 = distinct !{!167, !168, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!168 = distinct !{!168, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!169 = !{!167, !164}
!170 = !{!171}
!171 = distinct !{!171, !172, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_: %_1"}
!172 = distinct !{!172, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs7g60D0Ppidu_6anyhow7wrapper12MessageErrorNtNtCsdJPVW0sQgAG_5alloc6string6StringEEBL_"}
!173 = !{!174}
!174 = distinct !{!174, !175, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!175 = distinct !{!175, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!176 = !{!174, !171}
!177 = !{!178}
!178 = distinct !{!178, !179, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_: %_1"}
!179 = distinct !{!179, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs7g60D0Ppidu_6anyhow7wrapper10BoxedErrorEBK_"}
!180 = !{i64 1, i64 0}
!181 = !{!182}
!182 = distinct !{!182, !183, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!183 = distinct !{!183, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!184 = !{!185}
!185 = distinct !{!185, !186, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs7g60D0Ppidu_6anyhow: %self"}
!186 = distinct !{!186, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs7g60D0Ppidu_6anyhow"}
!187 = !{i64 0, i64 2}
!188 = !{!189}
!189 = distinct !{!189, !190, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace: %err.1"}
!190 = distinct !{!190, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace"}
!191 = !{!"branch_weights", !"expected", i32 2146410, i32 2145337238}
!192 = !{!193}
!193 = distinct !{!193, !194, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainENtNtNtB8_6traits8iterator8Iterator4nextB19_: %_0"}
!194 = distinct !{!194, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainENtNtNtB8_6traits8iterator8Iterator4nextB19_"}
!195 = !{!196, !193, !198}
!196 = distinct !{!196, !197, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!197 = distinct !{!197, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!198 = distinct !{!198, !194, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateNtCs7g60D0Ppidu_6anyhow5ChainENtNtNtB8_6traits8iterator8Iterator4nextB19_: %self"}
!199 = !{!198}
!200 = !{!201}
!201 = distinct !{!201, !202, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow: %_0"}
!202 = distinct !{!202, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceNtB5_12SpecToString14spec_to_stringCs7g60D0Ppidu_6anyhow"}
!203 = !{!204}
!204 = distinct !{!204, !205, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!205 = distinct !{!205, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!206 = !{!204, !201}
!207 = !{!208}
!208 = distinct !{!208, !209, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!209 = distinct !{!209, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!210 = !{!211, !213}
!211 = distinct !{!211, !212, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7g60D0Ppidu_6anyhow: %self.0"}
!212 = distinct !{!212, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7g60D0Ppidu_6anyhow"}
!213 = distinct !{!213, !212, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs7g60D0Ppidu_6anyhow: %other.0"}
!214 = !{!215}
!215 = distinct !{!215, !216, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!216 = distinct !{!216, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!217 = !{!218}
!218 = distinct !{!218, !219, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!219 = distinct !{!219, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!220 = !{!221}
!221 = distinct !{!221, !222, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace: %err.1"}
!222 = distinct !{!222, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace"}
!223 = !{!224}
!224 = distinct !{!224, !225, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error5chain: %self"}
!225 = distinct !{!225, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error5chain"}
!226 = !{!227}
!227 = distinct !{!227, !225, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error5chain: %_0"}
!228 = !{!229, !227, !224}
!229 = distinct !{!229, !230, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain: %_0"}
!230 = distinct !{!230, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain"}
!231 = !{!232, !234}
!232 = distinct !{!232, !233, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!233 = distinct !{!233, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!234 = distinct !{!234, !235, !"_RINvYNtCs7g60D0Ppidu_6anyhow5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtBE_6option6OptionRDNtNtBE_5error5ErrorEL_EINvNvBw_4last4someB1S_EEB5_: %self"}
!235 = distinct !{!235, !"_RINvYNtCs7g60D0Ppidu_6anyhow5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtBE_6option6OptionRDNtNtBE_5error5ErrorEL_EINvNvBw_4last4someB1S_EEB5_"}
!236 = !{!234}
!237 = !{!238}
!238 = distinct !{!238, !239, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide: %self"}
!239 = distinct !{!239, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide"}
!240 = !{!241}
!241 = distinct !{!241, !239, !"_RNvMNtCs7g60D0Ppidu_6anyhow5errorNtB4_5Error7provide: %request.1"}
!242 = !{!243, !238, !241}
!243 = distinct !{!243, !244, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide: %request.1"}
!244 = distinct !{!244, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide"}
!245 = !{!246, !243, !238, !241}
!246 = distinct !{!246, !247, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow: %self.1"}
!247 = distinct !{!247, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow"}
!248 = !{!249}
!249 = distinct !{!249, !250, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain: %_0"}
!250 = distinct !{!250, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain"}
!251 = !{!252, !249}
!252 = distinct !{!252, !253, !"_RNvMNtCs7g60D0Ppidu_6anyhow5chainNtB4_5Chain3new: %_0"}
!253 = distinct !{!253, !"_RNvMNtCs7g60D0Ppidu_6anyhow5chainNtB4_5Chain3new"}
!254 = !{!255}
!255 = distinct !{!255, !253, !"_RNvMNtCs7g60D0Ppidu_6anyhow5chainNtB4_5Chain3new: %head.1"}
!256 = !{!257}
!257 = distinct !{!257, !258, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide: %request.1"}
!258 = distinct !{!258, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide"}
!259 = !{!260, !257}
!260 = distinct !{!260, !261, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow: %self.1"}
!261 = distinct !{!261, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow"}
!262 = !{!263}
!263 = distinct !{!263, !264, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace: %err.1"}
!264 = distinct !{!264, !"_RNvNtCs7g60D0Ppidu_6anyhow7nightly21request_ref_backtrace"}
!265 = !{!266}
!266 = distinct !{!266, !267, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs7g60D0Ppidu_6anyhow: %self"}
!267 = distinct !{!267, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs7g60D0Ppidu_6anyhow"}
!268 = !{!"branch_weights", i32 2002, i32 2000}
!269 = !{!270}
!270 = distinct !{!270, !271, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow: %self.1"}
!271 = distinct !{!271, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow"}
!272 = !{!"branch_weights", i32 2000, i32 6004}
!273 = !{!274}
!274 = distinct !{!274, !275, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs7g60D0Ppidu_6anyhow: %_0"}
!275 = distinct !{!275, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs7g60D0Ppidu_6anyhow"}
!276 = !{!277}
!277 = distinct !{!277, !278, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!278 = distinct !{!278, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!279 = !{!280}
!280 = distinct !{!280, !281, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow: %_1"}
!281 = distinct !{!281, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs7g60D0Ppidu_6anyhow"}
!282 = !{!283}
!283 = distinct !{!283, !284, !"_RNvXs5_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits12double_ended19DoubleEndedIterator9next_backCs7g60D0Ppidu_6anyhow: %self"}
!284 = distinct !{!284, !"_RNvXs5_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits12double_ended19DoubleEndedIterator9next_backCs7g60D0Ppidu_6anyhow"}
!285 = !{!286}
!286 = distinct !{!286, !287, !"_RNvXs5_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits12double_ended19DoubleEndedIterator9next_backCs7g60D0Ppidu_6anyhow: %self"}
!287 = distinct !{!287, !"_RNvXs5_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits12double_ended19DoubleEndedIterator9next_backCs7g60D0Ppidu_6anyhow"}
!288 = !{!289}
!289 = distinct !{!289, !290, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow: %self"}
!290 = distinct !{!290, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow"}
!291 = !{!292}
!292 = distinct !{!292, !290, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_E8push_mutCs7g60D0Ppidu_6anyhow: %value.1"}
!293 = !{!289, !292}
!294 = !{!295}
!295 = distinct !{!295, !296, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowuENCINvNvBL_3any5checkhNCNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB3t_3BufNtNtBa_3fmt5Write9write_str0E0E0B2j_EB3v_: %self"}
!296 = distinct !{!296, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowuENCINvNvBL_3any5checkhNCNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB3t_3BufNtNtBa_3fmt5Write9write_str0E0E0B2j_EB3v_"}
!297 = !{!298}
!298 = distinct !{!298, !299, !"_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display: %f"}
!299 = distinct !{!299, !"_RNvMNtCs7g60D0Ppidu_6anyhow3fmtNtNtB4_5error9ErrorImpl7display"}
!300 = !{!301, !298}
!301 = distinct !{!301, !302, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain: %_0"}
!302 = distinct !{!302, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl5chain"}
!303 = !{!304, !306, !298}
!304 = distinct !{!304, !305, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!305 = distinct !{!305, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!306 = distinct !{!306, !307, !"_RINvYNtCs7g60D0Ppidu_6anyhow5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8try_foldINtNtNtBE_3num7nonzero7NonZerojENCNvXs_NvBw_10advance_byB3_NtB2d_13SpecAdvanceBy15spec_advance_by0INtNtBE_6option6OptionB1A_EEB5_: %self"}
!307 = distinct !{!307, !"_RINvYNtCs7g60D0Ppidu_6anyhow5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8try_foldINtNtNtBE_3num7nonzero7NonZerojENCNvXs_NvBw_10advance_byB3_NtB2d_13SpecAdvanceBy15spec_advance_by0INtNtBE_6option6OptionB1A_EEB5_"}
!308 = !{!309, !298}
!309 = distinct !{!309, !310, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!310 = distinct !{!310, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!311 = !{!312, !298}
!312 = distinct !{!312, !313, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next: %self"}
!313 = distinct !{!313, !"_RNvXs_NtCs7g60D0Ppidu_6anyhow5chainNtB6_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next"}
!314 = distinct !{!314, !315}
!315 = !{!"llvm.loop.peeled.count", i32 1}
!316 = !{!317}
!317 = distinct !{!317, !318, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide: %request.1"}
!318 = distinct !{!318, !"_RNvMs6_NtCs7g60D0Ppidu_6anyhow5errorNtB5_9ErrorImpl7provide"}
!319 = !{!320, !317}
!320 = distinct !{!320, !321, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow: %self.1"}
!321 = distinct !{!321, !"_RINvMs3_NtCsjMrxcFdYDNN_4core5errorNtB6_7Request7provideINtNtB6_4tags3RefINtBV_15MaybeSizedValueNtNtCs5sEH5CPMdak_3std9backtrace9BacktraceEEECs7g60D0Ppidu_6anyhow"}
!322 = !{!323}
!323 = distinct !{!323, !324, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!324 = distinct !{!324, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!325 = !{!326, !323}
!326 = distinct !{!326, !327, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow: %self"}
!327 = distinct !{!327, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow"}
!328 = !{!329}
!329 = distinct !{!329, !330, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!330 = distinct !{!330, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!331 = !{!332}
!332 = distinct !{!332, !333, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs7g60D0Ppidu_6anyhow: %self"}
!333 = distinct !{!333, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs7g60D0Ppidu_6anyhow"}
!334 = !{!335, !332, !329}
!335 = distinct !{!335, !336, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow: %self"}
!336 = distinct !{!336, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs7g60D0Ppidu_6anyhow"}
!337 = !{!338}
!338 = distinct !{!338, !330, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!339 = !{!332, !329}
!340 = !{!341, !343, !344, !346, !348, !350}
!341 = distinct !{!341, !342, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %_0"}
!342 = distinct !{!342, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match"}
!343 = distinct !{!343, !342, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str7patternNtB4_12CharSearcherNtB4_8Searcher10next_match: %self"}
!344 = distinct !{!344, !345, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7g60D0Ppidu_6anyhow: %self"}
!345 = distinct !{!345, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalcE4nextCs7g60D0Ppidu_6anyhow"}
!346 = distinct !{!346, !347, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow: %self"}
!347 = distinct !{!347, !"_RNvXsX_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_5SplitcENtNtNtNtB9_4iter6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow"}
!348 = distinct !{!348, !349, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_3str4iter5SplitcEENtNtNtB8_6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow: %_0"}
!349 = distinct !{!349, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_3str4iter5SplitcEENtNtNtB8_6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow"}
!350 = distinct !{!350, !349, !"_RNvXs_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters9enumerateINtB4_9EnumerateINtNtNtBa_3str4iter5SplitcEENtNtNtB8_6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow: %self"}
!351 = !{!352}
!352 = distinct !{!352, !353, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr: %text.0"}
!353 = distinct !{!353, !"_RNvNtNtCsjMrxcFdYDNN_4core5slice6memchr6memchr"}
!354 = !{!355}
!355 = distinct !{!355, !356, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow: %self"}
!356 = distinct !{!356, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterRDNtNtCsjMrxcFdYDNN_4core5error5ErrorEL_ENtNtNtNtB12_4iter6traits8iterator8Iterator4nextCs7g60D0Ppidu_6anyhow"}
!357 = !{!358}
!358 = distinct !{!358, !359, !"_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len: %self"}
!359 = distinct !{!359, !"_RNvXs1_NtCs7g60D0Ppidu_6anyhow5chainNtB7_5ChainNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits10exact_size17ExactSizeIterator3len"}
!360 = !{!361}
!361 = distinct !{!361, !362, !"_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt: %self"}
!362 = distinct !{!362, !"_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt"}
!363 = !{!364}
!364 = distinct !{!364, !362, !"_RNvXsq_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt: %f"}
!365 = !{!366}
!366 = distinct !{!366, !367, !"_RNvXs0_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops5deref5Deref5deref: %self"}
!367 = distinct !{!367, !"_RNvXs0_NtCs7g60D0Ppidu_6anyhow5errorNtB7_5ErrorNtNtNtCsjMrxcFdYDNN_4core3ops5deref5Deref5deref"}
!368 = !{!369}
!369 = distinct !{!369, !370, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw: %dst.0"}
!370 = distinct !{!370, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw"}
!371 = !{!372}
!372 = distinct !{!372, !373, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw: %dst.0"}
!373 = distinct !{!373, !"_RNvNtNtCsjMrxcFdYDNN_4core4char7methods15encode_utf8_raw"}
!374 = !{!375}
!375 = distinct !{!375, !376, !"_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!376 = distinct !{!376, !"_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!377 = !{!378}
!378 = distinct !{!378, !376, !"_RNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB5_3BufNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!379 = !{!380, !375}
!380 = distinct !{!380, !381, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowuENCINvNvBL_3any5checkhNCNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB3t_3BufNtNtBa_3fmt5Write9write_str0E0E0B2j_EB3v_: %self"}
!381 = distinct !{!381, !"_RINvYINtNtNtCsjMrxcFdYDNN_4core5slice4iter4IterhENtNtNtNtBa_4iter6traits8iterator8Iterator8try_folduNCINvNtNtBR_8adapters6copied13copy_try_foldhuINtNtNtBa_3ops12control_flow11ControlFlowuENCINvNvBL_3any5checkhNCNvXs1_NtCs7g60D0Ppidu_6anyhow6ensureNtB3t_3BufNtNtBa_3fmt5Write9write_str0E0E0B2j_EB3v_"}
!382 = !{!375, !378}
!383 = !{!384}
!384 = distinct !{!384, !385, !"_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source: %self"}
!385 = distinct !{!385, !"_RNvXs6_NtCs7g60D0Ppidu_6anyhow7wrapperNtB5_10BoxedErrorNtNtCsjMrxcFdYDNN_4core5error5Error6source"}
