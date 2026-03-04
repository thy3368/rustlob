; ModuleID = 'quote.15ecd39294d94c2a-cgu.0'
source_filename = "quote.15ecd39294d94c2a-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"proc_macro2::TokenTree" = type { i32, [7 x i32] }
%"proc_macro::TokenTree" = type { [16 x i8], i8, [3 x i8] }
%"proc_macro::bridge::TokenTree<proc_macro::bridge::client::TokenStream, proc_macro::bridge::client::Span, proc_macro::bridge::symbol::Symbol>" = type { [16 x i8], i8, [3 x i8] }
%"core::mem::maybe_uninit::MaybeUninit<proc_macro2::TokenTree>" = type { [4 x i64] }
%"core::mem::maybe_uninit::MaybeUninit<proc_macro::TokenTree>" = type { [5 x i32] }

@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsb_Cs8M6BBVNvC7a_11proc_macro2NtB5_8LexErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt }>, align 8
@vtable.1 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt }>, align 8
@alloc_972c6ba976e8ba9d5a48ee7ba5235094 = private unnamed_addr constant [112 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/rc.rs\00", align 1
@alloc_34ccc9a2f61247d7615f0501afc88ea6 = private unnamed_addr constant [20 x i8] c"invalid token stream", align 1
@alloc_5184e6b45a026d33d30ffa740e9bcdd5 = private unnamed_addr constant [100 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/quote-1.0.42/src/runtime.rs\00", align 1
@alloc_c2b3f6463b0a8a8095dd8eea675c8d69 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00\1D\01\00\00$\00\00\00" }>, align 8
@alloc_bc590f7bbdf9d5cf9c9fc92d5b74ebb4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00B\01\00\00\12\00\00\00" }>, align 8
@alloc_a215eae7f8dd0221659ca93936e57bd6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00A\01\00\00$\00\00\00" }>, align 8
@alloc_26741d7be2999f870b5ef1ed52a52387 = private unnamed_addr constant [2 x i8] c"r#", align 1
@alloc_45cdb82e1f6d7e9f36108024db2c250f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00\C7\01\00\00\09\00\00\00" }>, align 8
@alloc_1ab7e69cb0be14b6daa10eaf0accb519 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00\C5\01\00\00\09\00\00\00" }>, align 8
@alloc_194591c686f9a063bbeab3b098b3fcb1 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00N\01\00\008\00\00\00" }>, align 8
@alloc_1c96afc0f227aaec172597eef4da1a54 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00N\01\00\00$\00\00\00" }>, align 8
@alloc_27cca3636828088e60ce450d2eca2060 = private unnamed_addr constant [1 x i8] c"_", align 1
@alloc_53c7a51dd4d2107fe69463bcafa37cb4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00\B8\01\00\00\13\00\00\00" }>, align 8
@alloc_8177639792cada302c0caf96986c44a5 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_5184e6b45a026d33d30ffa740e9bcdd5, [16 x i8] c"c\00\00\00\00\00\00\00\17\01\00\00$\00\00\00" }>, align 8
@alloc_8707c2166604de2a48294c7fc96dc805 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_972c6ba976e8ba9d5a48ee7ba5235094, [16 x i8] c"o\00\00\00\00\00\00\00\9B\11\00\00\1F\00\00\00" }>, align 8
@vtable.2 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str, ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char, ptr @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs1SHOYL7dTh6_5quote }>, align 8
@alloc_cc656815297f75969399c3f4b1ad3de4 = private unnamed_addr constant [55 x i8] c"a Display implementation returned an error unexpectedly", align 1
@alloc_82687d2573a6ed8a4d1fcf733f159466 = private unnamed_addr constant [116 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs\00", align 1
@alloc_f3c70bf9d2724ff8f638341943ddf3c8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_82687d2573a6ed8a4d1fcf733f159466, [16 x i8] c"s\00\00\00\00\00\00\00f\0B\00\00\0E\00\00\00" }>, align 8
@alloc_99ac8a81a24cac863217ce4a5cbfabea = private unnamed_addr constant [5 x i8] c"Error", align 1
@alloc_6f4357e3a3c9006d5d6d935934a9de54 = private unnamed_addr constant [5 x i8] c"false", align 1
@alloc_c9ee9951a160df092319190fa06505e4 = private unnamed_addr constant [4 x i8] c"true", align 1
@alloc_b83fa9c5a29b84f2261c4cb83b2825fb = private unnamed_addr constant [102 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/quote-1.0.42/src/to_tokens.rs\00", align 1
@alloc_e1fe06b9b3e627577c940ae5d8dbadd4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_b83fa9c5a29b84f2261c4cb83b2825fb, [16 x i8] c"e\00\00\00\00\00\00\00\D9\00\00\00\17\00\00\00" }>, align 8

; core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !2)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !5)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %self.val.i.i = load ptr, ptr %0, align 8, !alias.scope !8, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %self.val1.i.i = load ptr, ptr %1, align 8, !alias.scope !8, !nonnull !9, !noundef !9
  %2 = ptrtoint ptr %self.val1.i.i to i64
  %3 = ptrtoint ptr %self.val.i.i to i64
  %4 = sub nuw i64 %2, %3
  %5 = lshr exact i64 %4, 5
  br label %bb6.i.i.i

cleanup.body.i.i:                                 ; preds = %bb4.i.i.i
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %capacity1.i.i.i.i = load i64, ptr %6, align 8, !alias.scope !8, !noundef !9
  %7 = icmp eq i64 %capacity1.i.i.i.i, 0
  br i1 %7, label %bb5.i.i, label %bb2.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %cleanup.body.i.i
  %ptr.i.i.i.i = load ptr, ptr %_1, align 8, !alias.scope !8, !nonnull !9, !noundef !9
  %alloc_size.i.i.i.i.i.i.i.i = shl nuw i64 %capacity1.i.i.i.i, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !8
  br label %bb5.i.i

bb6.i.i.i:                                        ; preds = %bb5.i.i.i, %start
  %_3.sroa.0.0.i.i.i = phi i64 [ 0, %start ], [ %8, %bb5.i.i.i ]
  %_7.i.i.i = icmp eq i64 %_3.sroa.0.0.i.i.i, %5
  br i1 %_7.i.i.i, label %bb2.i.i, label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i, i64 %_3.sroa.0.0.i.i.i
  %8 = add nuw nsw i64 %_3.sroa.0.0.i.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_6.i.i.i)
          to label %bb6.i.i.i unwind label %cleanup.i.i.i, !noalias !8

bb4.i.i.i:                                        ; preds = %bb3.i.i.i, %cleanup.i.i.i
  %_3.sroa.0.1.i.i.i = phi i64 [ %8, %cleanup.i.i.i ], [ %10, %bb3.i.i.i ]
  %_5.i.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i, %5
  br i1 %_5.i.i.i, label %cleanup.body.i.i, label %bb3.i.i.i

cleanup.i.i.i:                                    ; preds = %bb5.i.i.i
  %9 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i.i

bb3.i.i.i:                                        ; preds = %bb4.i.i.i
  %_4.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i, i64 %_3.sroa.0.1.i.i.i
  %10 = add i64 %_3.sroa.0.1.i.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i.i) #22
          to label %bb4.i.i.i unwind label %terminate.i.i.i, !noalias !8

terminate.i.i.i:                                  ; preds = %bb3.i.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !8
  unreachable

bb2.i.i:                                          ; preds = %bb6.i.i.i
  %12 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %capacity1.i.i3.i.i = load i64, ptr %12, align 8, !alias.scope !8, !noundef !9
  %13 = icmp eq i64 %capacity1.i.i3.i.i, 0
  br i1 %13, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit, label %bb2.i.i.i.i.i4.i.i

bb2.i.i.i.i.i4.i.i:                               ; preds = %bb2.i.i
  %ptr.i.i5.i.i = load ptr, ptr %_1, align 8, !alias.scope !8, !nonnull !9, !noundef !9
  %alloc_size.i.i.i.i.i.i6.i.i = shl nuw i64 %capacity1.i.i3.i.i, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !8
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit

bb5.i.i:                                          ; preds = %bb2.i.i.i.i.i.i.i, %cleanup.body.i.i
  resume { ptr, i32 } %9

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit: ; preds = %bb2.i.i, %bb2.i.i.i.i.i4.i.i
  ret void
}

; core::ptr::drop_in_place::<alloc::rc::UniqueRcUninit<alloc::vec::Vec<proc_macro2::TokenTree>, alloc::alloc::Global>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc14UniqueRcUninitINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBL_5alloc6GlobalEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !10)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %1 = load i8, ptr %0, align 8, !range !13, !alias.scope !10, !noundef !9
  %2 = trunc nuw i8 %1 to i1
  store i8 0, ptr %0, align 8, !alias.scope !10
  br i1 %2, label %bb8.i, label %bb7.i, !prof !14

bb8.i:                                            ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self2.i = load ptr, ptr %3, align 8, !alias.scope !10, !nonnull !9, !noundef !9
  %_10.0.i = load i64, ptr %_1, align 8, !range !15, !alias.scope !10, !noundef !9
  %4 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_10.1.i = load i64, ptr %4, align 8, !alias.scope !10, !noundef !9
; call alloc::rc::rc_inner_layout_for_value_layout
  %5 = tail call { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef %_10.0.i, i64 noundef %_10.1.i), !noalias !10
  %_9.1.i = extractvalue { i64, i64 } %5, 1
  %6 = icmp eq i64 %_9.1.i, 0
  br i1 %6, label %_RNvXs1O_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote.exit, label %bb1.i.i

bb1.i.i:                                          ; preds = %bb8.i
  %_9.0.i = extractvalue { i64, i64 } %5, 0
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %self2.i, i64 noundef %_9.1.i, i64 noundef range(i64 1, -9223372036854775807) %_9.0.i) #21, !noalias !10
  br label %_RNvXs1O_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote.exit

bb7.i:                                            ; preds = %start
; call core::option::unwrap_failed
  tail call void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8707c2166604de2a48294c7fc96dc805) #24, !noalias !10
  unreachable

_RNvXs1O_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote.exit: ; preds = %bb8.i, %bb1.i.i
  ret void
}

; core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro2::TokenTree>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %0, align 8, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val5 = load i64, ptr %1, align 8, !noundef !9
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb5.i.i, %start
  %_3.sroa.0.0.i.i = phi i64 [ 0, %start ], [ %2, %bb5.i.i ]
  %_7.i.i = icmp eq i64 %_3.sroa.0.0.i.i, %_1.val5
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb6.i.i
  %_6.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %_1.val4, i64 %_3.sroa.0.0.i.i
  %2 = add i64 %_3.sroa.0.0.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_6.i.i)
          to label %bb6.i.i unwind label %cleanup.i.i

bb4.i.i:                                          ; preds = %bb3.i.i, %cleanup.i.i
  %_3.sroa.0.1.i.i = phi i64 [ %2, %cleanup.i.i ], [ %4, %bb3.i.i ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %_1.val5
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

cleanup.i.i:                                      ; preds = %bb5.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %_1.val4, i64 %_3.sroa.0.1.i.i
  %4 = add i64 %_3.sroa.0.1.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i) #22
          to label %bb4.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %bb3.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val = load i64, ptr %_1, align 8, !range !16, !noundef !9
  %6 = icmp eq i64 %_1.val, 0
  br i1 %6, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21
  br label %bb1

bb4:                                              ; preds = %bb6.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !16, !noundef !9
  %7 = icmp eq i64 %_1.val2, 0
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val2, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #21
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %3
}

; core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro::TokenTree>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val = load ptr, ptr %0, align 8, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1 = load i64, ptr %1, align 8, !noundef !9
  %_710.i.i = icmp eq i64 %_1.val1, 0
  br i1 %_710.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %start, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i
  %_3.sroa.0.011.i.i = phi i64 [ %2, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i ], [ 0, %start ]
  %_6.i.i = getelementptr inbounds nuw %"proc_macro::TokenTree", ptr %_1.val, i64 %_3.sroa.0.011.i.i
  %2 = add nuw i64 %_3.sroa.0.011.i.i, 1
  %3 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 16
  %4 = load i8, ptr %3, align 4, !range !17, !alias.scope !18, !noundef !9
  %5 = icmp samesign ult i8 %4, 4
  br i1 %5, label %bb2.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i

bb2.i.i.i:                                        ; preds = %bb5.i.i
  %6 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 12
  %7 = load i32, ptr %6, align 4, !alias.scope !23, !noundef !9
  %8 = icmp eq i32 %7, 0
  br i1 %8, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb2.i.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %6)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i unwind label %cleanup.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i: ; preds = %bb2.i.i.i.i.i.i, %bb2.i.i.i, %bb5.i.i
  %_7.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

cleanup.i.i:                                      ; preds = %bb2.i.i.i.i.i.i
  %9 = landingpad { ptr, i32 }
          cleanup
  %_512.i.i = icmp eq i64 %2, %_1.val1
  br i1 %_512.i.i, label %cleanup.body, label %bb3.i.i

bb3.i.i:                                          ; preds = %cleanup.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i
  %_3.sroa.0.113.i.i = phi i64 [ %10, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i ], [ %2, %cleanup.i.i ]
  %_4.i.i = getelementptr inbounds nuw %"proc_macro::TokenTree", ptr %_1.val, i64 %_3.sroa.0.113.i.i
  %10 = add i64 %_3.sroa.0.113.i.i, 1
  %11 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 16
  %12 = load i8, ptr %11, align 4, !range !17, !alias.scope !30, !noundef !9
  %13 = icmp samesign ult i8 %12, 4
  br i1 %13, label %bb2.i6.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i

bb2.i6.i.i:                                       ; preds = %bb3.i.i
  %14 = getelementptr inbounds nuw i8, ptr %_4.i.i, i64 12
  %15 = load i32, ptr %14, align 4, !alias.scope !33, !noundef !9
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i, label %bb2.i.i.i.i7.i.i

bb2.i.i.i.i7.i.i:                                 ; preds = %bb2.i6.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %14)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i unwind label %terminate.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i: ; preds = %bb2.i.i.i.i7.i.i, %bb2.i6.i.i, %bb3.i.i
  %_5.i.i = icmp eq i64 %10, %_1.val1
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

terminate.i.i:                                    ; preds = %bb2.i.i.i.i7.i.i
  %17 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

cleanup.body:                                     ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit9.i.i, %cleanup.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !16, !noundef !9
  %18 = icmp eq i64 %_1.val2, 0
  br i1 %18, label %bb1, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %cleanup.body
  %alloc_size.i.i.i.i = mul nuw i64 %_1.val2, 20
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #21
  br label %bb1

bb4:                                              ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, %start
  %_1.val4 = load i64, ptr %_1, align 8, !range !16, !noundef !9
  %19 = icmp eq i64 %_1.val4, 0
  br i1 %19, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote.exit9, label %bb2.i.i.i7

bb2.i.i.i7:                                       ; preds = %bb4
  %alloc_size.i.i.i.i8 = mul nuw i64 %_1.val4, 20
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val, i64 noundef %alloc_size.i.i.i.i8, i64 noundef range(i64 1, -9223372036854775807) 4) #21
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote.exit9

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote.exit9: ; preds = %bb4, %bb2.i.i.i7
  ret void

bb1:                                              ; preds = %bb2.i.i.i6, %cleanup.body
  resume { ptr, i32 } %9
}

; core::ptr::drop_in_place::<core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::sources::once::Once<proc_macro2::TokenStream>, <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend<core::iter::sources::once::Once<proc_macro2::TokenStream>>::{closure#0}>, <proc_macro2::imp::TokenStream>::unwrap_stable>>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten7FlattenINtNtBL_3map3MapIB1n_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2b_B29_INtNtNtBN_6traits7collect6ExtendB29_E6extendB1H_E0ENvMs_NtB2b_3impNtB41_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(96) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !40, !alias.scope !41, !noundef !9
  %.off.i.i.i = add i64 %0, 9223372036854775807
  %switch.i.i.i = icmp ult i64 %.off.i.i.i, 2
  br i1 %switch.i.i.i, label %bb6.i, label %bb2.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i:                            ; preds = %start
; invoke core::ptr::drop_in_place::<proc_macro2::TokenStream>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(96) %_1)
          to label %bb6.i unwind label %cleanup.i

cleanup.i:                                        ; preds = %bb2.i.i.i.i.i.i.i.i.i
  %1 = landingpad { ptr, i32 }
          cleanup
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %3 = load ptr, ptr %2, align 8, !alias.scope !48, !noundef !9
  %4 = icmp eq ptr %3, null
  br i1 %4, label %bb3.i, label %bb2.i.i

bb2.i.i:                                          ; preds = %cleanup.i
; invoke core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %2)
          to label %bb3.i unwind label %terminate.i

bb6.i:                                            ; preds = %bb2.i.i.i.i.i.i.i.i.i, %start
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %6 = load ptr, ptr %5, align 8, !alias.scope !51, !noundef !9
  %7 = icmp eq ptr %6, null
  br i1 %7, label %bb5.i, label %bb2.i4.i

bb2.i4.i:                                         ; preds = %bb6.i
; invoke core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %5)
          to label %bb5.i unwind label %cleanup1.i

bb3.i:                                            ; preds = %cleanup1.i, %bb2.i.i, %cleanup.i
  %.pn.i = phi { ptr, i32 } [ %11, %cleanup1.i ], [ %1, %bb2.i.i ], [ %1, %cleanup.i ]
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %9 = load ptr, ptr %8, align 8, !alias.scope !54, !noundef !9
  %10 = icmp eq ptr %9, null
  br i1 %10, label %bb1.i, label %bb2.i7.i

bb2.i7.i:                                         ; preds = %bb3.i
; invoke core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %8)
          to label %bb1.i unwind label %terminate.i

cleanup1.i:                                       ; preds = %bb2.i4.i
  %11 = landingpad { ptr, i32 }
          cleanup
  br label %bb3.i

bb5.i:                                            ; preds = %bb2.i4.i, %bb6.i
  %12 = getelementptr inbounds nuw i8, ptr %_1, i64 64
  %13 = load ptr, ptr %12, align 8, !alias.scope !57, !noundef !9
  %14 = icmp eq ptr %13, null
  br i1 %14, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten13FlattenCompatINtNtBL_3map3MapIB1u_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2i_B2g_INtNtNtBN_6traits7collect6ExtendB2g_E6extendB1O_E0ENvMs_NtB2i_3impNtB48_11TokenStream13unwrap_stableEINtNtB2i_5rcvec13RcVecIntoIterNtB2i_9TokenTreeEEECs1SHOYL7dTh6_5quote.exit, label %bb2.i10.i

bb2.i10.i:                                        ; preds = %bb5.i
; call core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(32) %12)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten13FlattenCompatINtNtBL_3map3MapIB1u_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2i_B2g_INtNtNtBN_6traits7collect6ExtendB2g_E6extendB1O_E0ENvMs_NtB2i_3impNtB48_11TokenStream13unwrap_stableEINtNtB2i_5rcvec13RcVecIntoIterNtB2i_9TokenTreeEEECs1SHOYL7dTh6_5quote.exit

terminate.i:                                      ; preds = %bb2.i7.i, %bb2.i.i
  %15 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb1.i:                                            ; preds = %bb2.i7.i, %bb3.i
  resume { ptr, i32 } %.pn.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten13FlattenCompatINtNtBL_3map3MapIB1u_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2i_B2g_INtNtNtBN_6traits7collect6ExtendB2g_E6extendB1O_E0ENvMs_NtB2i_3impNtB48_11TokenStream13unwrap_stableEINtNtB2i_5rcvec13RcVecIntoIterNtB2i_9TokenTreeEEECs1SHOYL7dTh6_5quote.exit: ; preds = %bb5.i, %bb2.i10.i
  ret void
}

; core::ptr::drop_in_place::<proc_macro2::TokenStream>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load i64, ptr %_1, align 8, !range !60, !alias.scope !61, !noundef !9
  %.not.i = icmp eq i64 %0, -9223372036854775808
  br i1 %.not.i, label %bb3.i, label %bb2.i

bb2.i:                                            ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %2 = load i32, ptr %1, align 8, !alias.scope !64, !noundef !9
  %3 = icmp eq i32 %2, 0
  br i1 %3, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote.exit.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb2.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %1)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote.exit.i unwind label %cleanup.i.i

cleanup.i.i:                                      ; preds = %bb2.i.i.i.i
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_1) #22
          to label %common.resume.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %cleanup.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

common.resume.i:                                  ; preds = %bb1.i.i.i.i.i, %cleanup.i1.i, %cleanup.i.i
  %common.resume.op.i = phi { ptr, i32 } [ %4, %cleanup.i.i ], [ %7, %bb1.i.i.i.i.i ], [ %7, %cleanup.i1.i ]
  resume { ptr, i32 } %common.resume.op.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote.exit.i: ; preds = %bb2.i.i.i.i, %bb2.i
; call core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro::TokenTree>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_1)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote.exit

bb3.i:                                            ; preds = %start
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(8) %6)
          to label %bb4.i.i unwind label %cleanup.i1.i

cleanup.i1.i:                                     ; preds = %bb3.i
  %7 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !71)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !74)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !77)
  %_5.i.i.i.i.i = load ptr, ptr %6, align 8, !alias.scope !80, !nonnull !9, !noundef !9
  %_8.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i, align 8, !noalias !83, !noundef !9
  %val.i.i.i.i.i = add i64 %_8.i.i.i.i.i, -1
  store i64 %val.i.i.i.i.i, ptr %_5.i.i.i.i.i, align 8, !noalias !83
  %8 = icmp eq i64 %val.i.i.i.i.i, 0
  br i1 %8, label %bb1.i.i.i.i.i, label %common.resume.i

bb1.i.i.i.i.i:                                    ; preds = %cleanup.i1.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(8) %6) #25
          to label %common.resume.i unwind label %terminate.i3.i

bb4.i.i:                                          ; preds = %bb3.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !84)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !87)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !90)
  %_5.i.i.i1.i.i = load ptr, ptr %6, align 8, !alias.scope !93, !nonnull !9, !noundef !9
  %_8.i.i.i2.i.i = load i64, ptr %_5.i.i.i1.i.i, align 8, !noalias !94, !noundef !9
  %val.i.i.i3.i.i = add i64 %_8.i.i.i2.i.i, -1
  store i64 %val.i.i.i3.i.i, ptr %_5.i.i.i1.i.i, align 8, !noalias !94
  %9 = icmp eq i64 %val.i.i.i3.i.i, 0
  br i1 %9, label %bb1.i.i.i4.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote.exit

bb1.i.i.i4.i.i:                                   ; preds = %bb4.i.i
; call <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  tail call void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(8) %6) #25
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote.exit

terminate.i3.i:                                   ; preds = %bb1.i.i.i.i.i
  %10 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote.exit: ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote.exit.i, %bb4.i.i, %bb1.i.i.i4.i.i
  ret void
}

; core::ptr::drop_in_place::<proc_macro2::Group>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load i32, ptr %_1, align 8, !range !95, !alias.scope !96, !noundef !9
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %3 = load i32, ptr %2, align 8, !alias.scope !99, !noundef !9
  %4 = icmp eq i32 %3, 0
  br i1 %4, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote.exit, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb2.i
; call <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  tail call void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %2)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote.exit

bb3.i:                                            ; preds = %start
  %5 = getelementptr inbounds nuw i8, ptr %_1, i64 8
; invoke <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %5)
          to label %bb4.i.i.i unwind label %cleanup.i.i.i

cleanup.i.i.i:                                    ; preds = %bb3.i
  %6 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !106)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !109)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !112)
  %_5.i.i.i.i.i.i = load ptr, ptr %5, align 8, !alias.scope !115, !nonnull !9, !noundef !9
  %_8.i.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i.i, align 8, !noalias !120, !noundef !9
  %val.i.i.i.i.i.i = add i64 %_8.i.i.i.i.i.i, -1
  store i64 %val.i.i.i.i.i.i, ptr %_5.i.i.i.i.i.i, align 8, !noalias !120
  %7 = icmp eq i64 %val.i.i.i.i.i.i, 0
  br i1 %7, label %bb1.i.i.i.i.i.i, label %bb1.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %cleanup.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %5) #25
          to label %bb1.i.i.i unwind label %terminate.i.i.i

bb4.i.i.i:                                        ; preds = %bb3.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !121)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !124)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !127)
  %_5.i.i.i1.i.i.i = load ptr, ptr %5, align 8, !alias.scope !130, !nonnull !9, !noundef !9
  %_8.i.i.i2.i.i.i = load i64, ptr %_5.i.i.i1.i.i.i, align 8, !noalias !131, !noundef !9
  %val.i.i.i3.i.i.i = add i64 %_8.i.i.i2.i.i.i, -1
  store i64 %val.i.i.i3.i.i.i, ptr %_5.i.i.i1.i.i.i, align 8, !noalias !131
  %8 = icmp eq i64 %val.i.i.i3.i.i.i, 0
  br i1 %8, label %bb1.i.i.i4.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote.exit

bb1.i.i.i4.i.i.i:                                 ; preds = %bb4.i.i.i
; call <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  tail call void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %5) #25
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote.exit

terminate.i.i.i:                                  ; preds = %bb1.i.i.i.i.i.i
  %9 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb1.i.i.i:                                        ; preds = %bb1.i.i.i.i.i.i, %cleanup.i.i.i
  resume { ptr, i32 } %6

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote.exit: ; preds = %bb2.i, %bb2.i.i.i.i, %bb4.i.i.i, %bb1.i.i.i4.i.i.i
  ret void
}

; core::ptr::drop_in_place::<proc_macro2::TokenTree>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load i32, ptr %_1, align 8, !range !132, !noundef !9
  switch i32 %0, label %default.unreachable3 [
    i32 0, label %bb2
    i32 1, label %bb3
    i32 2, label %bb1
    i32 3, label %bb4
  ]

default.unreachable3:                             ; preds = %start
  unreachable

bb4:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %.val = load i64, ptr %1, align 8, !range !60, !noundef !9
  switch i64 %.val, label %bb2.i.i.i4.i.i.i.i.i [
    i64 -9223372036854775808, label %bb1
    i64 0, label %bb1
  ]

bb2.i.i.i4.i.i.i.i.i:                             ; preds = %bb4
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val1 = load ptr, ptr %2, align 8, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #21, !noalias !133
  br label %bb1

bb2:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %4 = load i32, ptr %3, align 8, !range !95, !alias.scope !136, !noundef !9
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb2.i.i, label %bb3.i.i

bb2.i.i:                                          ; preds = %bb2
  %6 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %7 = load i32, ptr %6, align 8, !alias.scope !141, !noundef !9
  %8 = icmp eq i32 %7, 0
  br i1 %8, label %bb1, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb2.i.i
; call <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  tail call void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %6)
  br label %bb1

bb3.i.i:                                          ; preds = %bb2
  %9 = getelementptr inbounds nuw i8, ptr %_1, i64 16
; invoke <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %9)
          to label %bb4.i.i.i.i unwind label %cleanup.i.i.i.i

cleanup.i.i.i.i:                                  ; preds = %bb3.i.i
  %10 = landingpad { ptr, i32 }
          cleanup
  tail call void @llvm.experimental.noalias.scope.decl(metadata !148)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !151)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !154)
  %_5.i.i.i.i.i.i.i = load ptr, ptr %9, align 8, !alias.scope !157, !nonnull !9, !noundef !9
  %_8.i.i.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i.i.i, align 8, !noalias !162, !noundef !9
  %val.i.i.i.i.i.i.i = add i64 %_8.i.i.i.i.i.i.i, -1
  store i64 %val.i.i.i.i.i.i.i, ptr %_5.i.i.i.i.i.i.i, align 8, !noalias !162
  %11 = icmp eq i64 %val.i.i.i.i.i.i.i, 0
  br i1 %11, label %bb1.i.i.i.i.i.i.i, label %bb1.i.i.i.i

bb1.i.i.i.i.i.i.i:                                ; preds = %cleanup.i.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %9) #25
          to label %bb1.i.i.i.i unwind label %terminate.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb3.i.i
  tail call void @llvm.experimental.noalias.scope.decl(metadata !163)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !166)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !169)
  %_5.i.i.i1.i.i.i.i = load ptr, ptr %9, align 8, !alias.scope !172, !nonnull !9, !noundef !9
  %_8.i.i.i2.i.i.i.i = load i64, ptr %_5.i.i.i1.i.i.i.i, align 8, !noalias !173, !noundef !9
  %val.i.i.i3.i.i.i.i = add i64 %_8.i.i.i2.i.i.i.i, -1
  store i64 %val.i.i.i3.i.i.i.i, ptr %_5.i.i.i1.i.i.i.i, align 8, !noalias !173
  %12 = icmp eq i64 %val.i.i.i3.i.i.i.i, 0
  br i1 %12, label %bb1.i.i.i4.i.i.i.i, label %bb1

bb1.i.i.i4.i.i.i.i:                               ; preds = %bb4.i.i.i.i
; call <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  tail call void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %9) #25
  br label %bb1

terminate.i.i.i.i:                                ; preds = %bb1.i.i.i.i.i.i.i
  %13 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb1.i.i.i.i:                                      ; preds = %bb1.i.i.i.i.i.i.i, %cleanup.i.i.i.i
  resume { ptr, i32 } %10

bb3:                                              ; preds = %start
  %14 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !174)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !177)
  %15 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %16 = load i8, ptr %15, align 8, !range !180, !alias.scope !181, !noundef !9
  %17 = icmp eq i8 %16, 2
  br i1 %17, label %bb1, label %bb2.i.i2

bb2.i.i2:                                         ; preds = %bb3
  %18 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i.i = load i64, ptr %18, align 8, !alias.scope !181, !noundef !9
  %19 = icmp eq i64 %_1.val1.i.i, 0
  br i1 %19, label %bb1, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i: ; preds = %bb2.i.i2
  %_1.val.i.i = load ptr, ptr %14, align 8, !alias.scope !181, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i, i64 noundef %_1.val1.i.i, i64 noundef 1) #21, !noalias !181
  br label %bb1

bb1:                                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i, %bb2.i.i2, %bb3, %bb1.i.i.i4.i.i.i.i, %bb4.i.i.i.i, %bb2.i.i.i.i.i, %bb2.i.i, %bb2.i.i.i4.i.i.i.i.i, %bb4, %bb4, %start
  ret void
}

; core::ptr::drop_in_place::<proc_macro::ConcatStreamsHelper>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro19ConcatStreamsHelperECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !182)
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val.i = load ptr, ptr %0, align 8, !alias.scope !182, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val1.i = load i64, ptr %1, align 8, !alias.scope !182, !noundef !9
  br label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb5.i.i.i, %start
  %_3.sroa.0.0.i.i.i = phi i64 [ 0, %start ], [ %2, %bb5.i.i.i ]
  %_7.i.i.i = icmp eq i64 %_3.sroa.0.0.i.i.i, %_1.val1.i
  br i1 %_7.i.i.i, label %bb4.i, label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb6.i.i.i
  %_6.i.i.i = getelementptr inbounds nuw i32, ptr %_1.val.i, i64 %_3.sroa.0.0.i.i.i
  %2 = add i64 %_3.sroa.0.0.i.i.i, 1
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %_6.i.i.i)
          to label %bb6.i.i.i unwind label %cleanup.i.i.i, !noalias !182

bb4.i.i.i:                                        ; preds = %bb3.i.i.i, %cleanup.i.i.i
  %_3.sroa.0.1.i.i.i = phi i64 [ %2, %cleanup.i.i.i ], [ %4, %bb3.i.i.i ]
  %_5.i.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i, %_1.val1.i
  br i1 %_5.i.i.i, label %cleanup.body.i, label %bb3.i.i.i

cleanup.i.i.i:                                    ; preds = %bb5.i.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i.i

bb3.i.i.i:                                        ; preds = %bb4.i.i.i
  %_4.i.i.i = getelementptr inbounds nuw i32, ptr %_1.val.i, i64 %_3.sroa.0.1.i.i.i
  %4 = add i64 %_3.sroa.0.1.i.i.i, 1
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %_4.i.i.i)
          to label %bb4.i.i.i unwind label %terminate.i.i.i, !noalias !182

terminate.i.i.i:                                  ; preds = %bb3.i.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !182
  unreachable

cleanup.body.i:                                   ; preds = %bb4.i.i.i
  %_1.val2.i = load i64, ptr %_1, align 8, !range !16, !alias.scope !182, !noundef !9
  %6 = icmp eq i64 %_1.val2.i, 0
  br i1 %6, label %bb1.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %cleanup.body.i
  %alloc_size.i.i.i.i.i = shl nuw i64 %_1.val2.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i, i64 noundef %alloc_size.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #21, !noalias !182
  br label %bb1.i

bb4.i:                                            ; preds = %bb6.i.i.i
  %_1.val4.i = load i64, ptr %_1, align 8, !range !16, !alias.scope !182, !noundef !9
  %7 = icmp eq i64 %_1.val4.i, 0
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote.exit, label %bb2.i.i.i6.i

bb2.i.i.i6.i:                                     ; preds = %bb4.i
  %alloc_size.i.i.i.i7.i = shl nuw i64 %_1.val4.i, 2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i, i64 noundef %alloc_size.i.i.i.i7.i, i64 noundef range(i64 1, -9223372036854775807) 4) #21, !noalias !182
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote.exit

bb1.i:                                            ; preds = %bb2.i.i.i.i, %cleanup.body.i
  resume { ptr, i32 } %3

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote.exit: ; preds = %bb4.i, %bb2.i.i.i6.i
  ret void
}

; core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(40) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !185)
  %_2.i = load i64, ptr %_1, align 8, !range !188, !alias.scope !185, !noundef !9
  %0 = icmp eq i64 %_2.i, 0
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  br i1 %0, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !189)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !192)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !195)
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %self.val.i.i.i.i = load ptr, ptr %2, align 8, !alias.scope !198, !nonnull !9, !noundef !9
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 32
  %self.val1.i.i.i.i = load ptr, ptr %3, align 8, !alias.scope !198, !nonnull !9, !noundef !9
  %4 = ptrtoint ptr %self.val1.i.i.i.i to i64
  %5 = ptrtoint ptr %self.val.i.i.i.i to i64
  %6 = sub nuw i64 %4, %5
  %7 = udiv exact i64 %6, 20
  %_710.i.i.i.i.i = icmp eq ptr %self.val1.i.i.i.i, %self.val.i.i.i.i
  br i1 %_710.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

cleanup.body.i.i.i.i:                             ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i, %cleanup.i.i.i.i.i
  %8 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %capacity1.i.i.i.i.i.i = load i64, ptr %8, align 8, !alias.scope !198, !noundef !9
  %9 = icmp eq i64 %capacity1.i.i.i.i.i.i, 0
  br i1 %9, label %bb5.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i:                            ; preds = %cleanup.body.i.i.i.i
  %ptr.i.i.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !198, !nonnull !9, !noundef !9
  %alloc_size.i.i.i.i.i.i.i.i.i.i = mul nuw i64 %capacity1.i.i.i.i.i.i, 20
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #21, !noalias !198
  br label %bb5.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb2.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i
  %_3.sroa.0.011.i.i.i.i.i = phi i64 [ %10, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ 0, %bb2.i ]
  %_6.i.i.i.i.i = getelementptr inbounds nuw %"proc_macro::bridge::TokenTree<proc_macro::bridge::client::TokenStream, proc_macro::bridge::client::Span, proc_macro::bridge::symbol::Symbol>", ptr %self.val.i.i.i.i, i64 %_3.sroa.0.011.i.i.i.i.i
  %10 = add nuw i64 %_3.sroa.0.011.i.i.i.i.i, 1
  %11 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i, i64 16
  %12 = load i8, ptr %11, align 4, !range !17, !alias.scope !199, !noalias !198, !noundef !9
  %13 = icmp samesign ult i8 %12, 4
  br i1 %13, label %bb2.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb5.i.i.i.i.i
  %14 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i.i, i64 12
  %15 = load i32, ptr %14, align 4, !alias.scope !204, !noalias !198, !noundef !9
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i:                              ; preds = %bb2.i.i.i.i.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %14)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i unwind label %cleanup.i.i.i.i.i, !noalias !198

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i, %bb5.i.i.i.i.i
  %_7.i.i.i.i.i = icmp eq i64 %10, %7
  br i1 %_7.i.i.i.i.i, label %bb2.i.i.i.i, label %bb5.i.i.i.i.i

cleanup.i.i.i.i.i:                                ; preds = %bb2.i.i.i.i.i.i.i.i
  %17 = landingpad { ptr, i32 }
          cleanup
  %_512.i.i.i.i.i = icmp eq i64 %10, %7
  br i1 %_512.i.i.i.i.i, label %cleanup.body.i.i.i.i, label %bb3.i.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %cleanup.i.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i
  %_3.sroa.0.113.i.i.i.i.i = phi i64 [ %18, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i ], [ %10, %cleanup.i.i.i.i.i ]
  %_4.i.i.i.i.i = getelementptr inbounds nuw %"proc_macro::bridge::TokenTree<proc_macro::bridge::client::TokenStream, proc_macro::bridge::client::Span, proc_macro::bridge::symbol::Symbol>", ptr %self.val.i.i.i.i, i64 %_3.sroa.0.113.i.i.i.i.i
  %18 = add i64 %_3.sroa.0.113.i.i.i.i.i, 1
  %19 = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i.i, i64 16
  %20 = load i8, ptr %19, align 4, !range !17, !alias.scope !209, !noalias !198, !noundef !9
  %21 = icmp samesign ult i8 %20, 4
  br i1 %21, label %bb2.i6.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i

bb2.i6.i.i.i.i.i:                                 ; preds = %bb3.i.i.i.i.i
  %22 = getelementptr inbounds nuw i8, ptr %_4.i.i.i.i.i, i64 12
  %23 = load i32, ptr %22, align 4, !alias.scope !212, !noalias !198, !noundef !9
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i, label %bb2.i.i.i7.i.i.i.i.i

bb2.i.i.i7.i.i.i.i.i:                             ; preds = %bb2.i6.i.i.i.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %22)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i unwind label %terminate.i.i.i.i.i, !noalias !198

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit9.i.i.i.i.i: ; preds = %bb2.i.i.i7.i.i.i.i.i, %bb2.i6.i.i.i.i.i, %bb3.i.i.i.i.i
  %_5.i.i.i.i.i = icmp eq i64 %18, %7
  br i1 %_5.i.i.i.i.i, label %cleanup.body.i.i.i.i, label %bb3.i.i.i.i.i

terminate.i.i.i.i.i:                              ; preds = %bb2.i.i.i7.i.i.i.i.i
  %25 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !198
  unreachable

bb2.i.i.i.i:                                      ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote.exit.i.i.i.i.i, %bb2.i
  %26 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %capacity1.i.i3.i.i.i.i = load i64, ptr %26, align 8, !alias.scope !198, !noundef !9
  %27 = icmp eq i64 %capacity1.i.i3.i.i.i.i, 0
  br i1 %27, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote.exit, label %bb2.i.i.i.i.i4.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i:                           ; preds = %bb2.i.i.i.i
  %ptr.i.i5.i.i.i.i = load ptr, ptr %1, align 8, !alias.scope !198, !nonnull !9, !noundef !9
  %alloc_size.i.i.i.i.i.i6.i.i.i.i = mul nuw i64 %capacity1.i.i3.i.i.i.i, 20
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 4) #21, !noalias !198
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote.exit

bb5.i.i.i.i:                                      ; preds = %bb2.i.i.i.i.i.i.i.i.i, %cleanup.body.i.i.i.i
  resume { ptr, i32 } %17

bb3.i:                                            ; preds = %start
; call core::ptr::drop_in_place::<proc_macro2::rcvec::RcVecIntoIter<proc_macro2::TokenTree>>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef readonly align 8 dereferenceable(32) %1)
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote.exit: ; preds = %bb2.i.i.i.i, %bb2.i.i.i.i.i4.i.i.i.i, %bb3.i
  ret void
}

; core::ptr::drop_in_place::<alloc::string::String>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs1SHOYL7dTh6_5quote.exit, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %1, align 8, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #21
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs1SHOYL7dTh6_5quote.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs1SHOYL7dTh6_5quote.exit: ; preds = %start, %bb2.i.i.i4.i
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 33) %elem_layout.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !217)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !16, !alias.scope !217, !noundef !9
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %0 = icmp eq i64 %elem_layout.1, 1
  %v1.sroa.0.0.i = select i1 %0, i64 8, i64 4
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 range(i64 0, -1) %v1.sroa.0.0.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !217
  %1 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !217
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 33) %elem_layout.1), !noalias !217
  %_35.i = load i64, ptr %self3.i, align 8, !range !188, !noalias !217, !noundef !9
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %3, align 8, !range !60, !noalias !217, !noundef !9
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !217
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !217
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #26
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %3, align 8, !noalias !217, !nonnull !9, !noundef !9
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !217
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !217
  %5 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !217
  ret void
}

; <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenStream>>
; Function Attrs: uwtable
define internal fastcc void @_RINvXs5_Cs8M6BBVNvC7a_11proc_macro2NtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBx_E6extendINtNtNtBW_7sources4once4OnceBx_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, ptr dead_on_return noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %streams) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_2.i.i.i.i.i.i.i.i1.i = alloca [32 x i8], align 8
  %x.i.sroa.6.i.i.i.i.i.i = alloca [28 x i8], align 4
  %_10.i.i.i.i.i.i = alloca [32 x i8], align 8
  %element.i.i.i.i = alloca [32 x i8], align 8
  %_3.sroa.8.i.i.i.i = alloca [28 x i8], align 4
  %_3.i.i = alloca [96 x i8], align 8
  %_2.i.i.i.i.i.i.i.i.i = alloca [32 x i8], align 8
  %_13.i.i = alloca [24 x i8], align 8
  %builder.i.i = alloca [24 x i8], align 8
  %iter.i.i = alloca [32 x i8], align 8
  %_13.i = alloca [32 x i8], align 8
  %_5 = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_5, ptr noundef nonnull readonly align 8 dereferenceable(32) %streams, i64 32, i1 false), !alias.scope !220
  tail call void @llvm.experimental.noalias.scope.decl(metadata !224)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !227)
  %0 = load i64, ptr %self, align 8, !range !60, !alias.scope !224, !noalias !227, !noundef !9
  %1 = icmp eq i64 %0, -9223372036854775808
  br i1 %1, label %bb9.i, label %bb3.i

bb3.i:                                            ; preds = %start
; invoke <proc_macro2::imp::DeferredTokenStream>::evaluate_now
  invoke void @_RNvMNtCs8M6BBVNvC7a_11proc_macro23impNtB2_19DeferredTokenStream12evaluate_now(ptr noalias noundef nonnull align 8 dereferenceable(32) %self)
          to label %bb6.i unwind label %bb13.i, !noalias !227

bb6.i:                                            ; preds = %bb3.i
  %_7.i = getelementptr inbounds nuw i8, ptr %self, i64 24
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter.i.i), !noalias !229
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %iter.i.i, ptr noundef nonnull align 8 dereferenceable(32) %streams, i64 32, i1 false)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %builder.i.i), !noalias !229
  %iter.val.i.i = load i64, ptr %iter.i.i, align 8, !range !233, !noalias !229, !noundef !9
  %2 = icmp ne i64 %iter.val.i.i, -9223372036854775807
  %_4.i.i.i.i.i = zext i1 %2 to i64
; invoke <proc_macro::ConcatStreamsHelper>::new
  invoke void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %builder.i.i, i64 noundef %_4.i.i.i.i.i)
          to label %bb3.i.i unwind label %bb9.i.i, !noalias !234

bb3.i.i:                                          ; preds = %bb6.i
  %.not7.i.i.i.i.i.i = icmp eq i64 %iter.val.i.i, -9223372036854775807
  br i1 %.not7.i.i.i.i.i.i, label %bb4.i.i, label %bb3.i.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb3.i.i
  %_11.sroa.4.0._2.i.i.i.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i.i.i.i.i.i, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_2.i.i.i.i.i.i.i.i.i), !noalias !235
  %3 = getelementptr inbounds nuw i8, ptr %streams, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_11.sroa.4.0._2.i.i.i.sroa_idx.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(24) %3, i64 24, i1 false)
  store i64 %iter.val.i.i, ptr %_2.i.i.i.i.i.i.i.i.i, align 8, !noalias !252
; invoke <proc_macro2::imp::TokenStream>::unwrap_nightly
  %_0.i.i.i1.i.i.i.i3.i.i = invoke noundef i32 @_RNvMs_NtCs8M6BBVNvC7a_11proc_macro23impNtB4_11TokenStream14unwrap_nightly(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(32) %_2.i.i.i.i.i.i.i.i.i)
          to label %_0.i.i.i1.i.i.i.i.noexc.i.i unwind label %bb7.i.i, !noalias !234

_0.i.i.i1.i.i.i.i.noexc.i.i:                      ; preds = %bb3.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2.i.i.i.i.i.i.i.i.i), !noalias !235
; invoke <proc_macro::ConcatStreamsHelper>::push
  invoke void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper4push(ptr noalias noundef nonnull align 8 dereferenceable(24) %builder.i.i, i32 noundef %_0.i.i.i1.i.i.i.i3.i.i)
          to label %bb4.i.i unwind label %bb7.i.i, !noalias !234

bb4.i.i:                                          ; preds = %_0.i.i.i1.i.i.i.i.noexc.i.i, %bb3.i.i
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_13.i.i), !noalias !229
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_13.i.i, ptr noundef nonnull align 8 dereferenceable(24) %builder.i.i, i64 24, i1 false), !noalias !229
; call <proc_macro::ConcatStreamsHelper>::append_to
  call void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper9append_to(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(24) %_13.i.i, ptr noalias noundef nonnull align 4 dereferenceable(4) %_7.i), !noalias !227
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_13.i.i), !noalias !229
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %builder.i.i), !noalias !229
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter.i.i), !noalias !229
  br label %_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote.exit

bb7.i.i:                                          ; preds = %_0.i.i.i1.i.i.i.i.noexc.i.i, %bb3.i.i.i.i.i.i
  %lpad.thr_comm.i.i = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro::ConcatStreamsHelper>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro19ConcatStreamsHelperECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(24) %builder.i.i) #22
          to label %bb12.i unwind label %terminate.i.i, !noalias !234

terminate.i.i:                                    ; preds = %bb2.i.i.i.i.i.i.i.i, %bb7.i.i
  %4 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !234
  unreachable

bb9.i.i:                                          ; preds = %bb6.i
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = icmp eq i64 %iter.val.i.i, -9223372036854775807
  br i1 %6, label %bb12.i, label %bb2.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i:                              ; preds = %bb9.i.i
; invoke core::ptr::drop_in_place::<proc_macro2::TokenStream>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %iter.i.i)
          to label %bb12.i unwind label %terminate.i.i, !noalias !234

bb9.i:                                            ; preds = %start
  %tts.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i), !noalias !253
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_13.i, ptr noundef nonnull align 8 dereferenceable(32) %streams, i64 32, i1 false)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !254)
  call void @llvm.lifetime.start.p0(i64 96, ptr nonnull %_3.i.i), !noalias !253
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::make_mut
  %_7.i.i = invoke fastcc noundef nonnull align 8 ptr @_RNvMsh_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE8make_mutCs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(8) %tts.i)
          to label %bb2.i.i unwind label %bb4.i2.i, !noalias !257

bb2.i.i:                                          ; preds = %bb9.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_3.i.i, ptr noundef nonnull align 8 dereferenceable(32) %streams, i64 32, i1 false)
  %_2.sroa.4.0._0.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 32
  store ptr null, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !258, !noalias !261
  %_2.sroa.6.0._0.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 64
  store ptr null, ptr %_2.sroa.6.0._0.sroa_idx.i.i.i, align 8, !alias.scope !258, !noalias !261
  tail call void @llvm.experimental.noalias.scope.decl(metadata !264)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !267)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !269)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !272)
  %_14.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 56
  %7 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 40
  %self1.sroa.5.0.self.sroa_idx.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 8
  %args.sroa.4.0._2.i.sroa_idx.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_2.i.i.i.i.i.i.i.i1.i, i64 8
  %_14.i.i.i.i7.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 88
  %8 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 72
  %_3.sroa.8.0.element.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %element.i.i.i.i, i64 4
  %9 = getelementptr inbounds nuw i8, ptr %_7.i.i, i64 16
  %10 = getelementptr inbounds nuw i8, ptr %_7.i.i, i64 8
  %11 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 48
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb8.i.i.i.i, %bb2.i.i
  %12 = phi ptr [ %68, %bb8.i.i.i.i ], [ null, %bb2.i.i ]
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_3.sroa.8.i.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !274)
  call void @llvm.experimental.noalias.scope.decl(metadata !277)
  %self.promoted.i.i.i.i.i.i = load i64, ptr %_3.i.i, align 8, !alias.scope !280, !noalias !281
  call void @llvm.experimental.noalias.scope.decl(metadata !284)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %x.i.sroa.6.i.i.i.i.i.i)
  %.not.i.peel.i.i.i.i.i.i = icmp eq ptr %12, null
  br i1 %.not.i.peel.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit.peel.i.i.i.i.i.i, label %bb11.i.peel.i.i.i.i.i.i

bb11.i.peel.i.i.i.i.i.i:                          ; preds = %bb1.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !287)
  call void @llvm.experimental.noalias.scope.decl(metadata !290)
  call void @llvm.experimental.noalias.scope.decl(metadata !293)
  %_12.i.i.i.i.peel.i.i.i.i.i.i = load ptr, ptr %_14.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !296, !noalias !297, !nonnull !9, !noundef !9
  %_21.i.i.i.i.peel.i.i.i.i.i.i = load ptr, ptr %7, align 8, !alias.scope !296, !noalias !297, !nonnull !9, !noundef !9
  %_9.i.i.i.i.peel.i.i.i.i.i.i = icmp eq ptr %_21.i.i.i.i.peel.i.i.i.i.i.i, %_12.i.i.i.i.peel.i.i.i.i.i.i
  br i1 %_9.i.i.i.i.peel.i.i.i.i.i.i, label %bb2.i.i.peel.i.i.i.i.i.i, label %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i

_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i: ; preds = %bb11.i.peel.i.i.i.i.i.i
  %_23.i.i.i.i.peel.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i.peel.i.i.i.i.i.i, i64 32
  store ptr %_23.i.i.i.i.peel.i.i.i.i.i.i, ptr %7, align 8, !alias.scope !296, !noalias !297
  %x.i.sroa.0.0.copyload.peel.i.i.i.i.i.i = load i32, ptr %_21.i.i.i.i.peel.i.i.i.i.i.i, align 8, !noalias !302
  %x.i.sroa.6.0._21.i.i.i.i.sroa_idx.peel.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i.peel.i.i.i.i.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %x.i.sroa.6.i.i.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %x.i.sroa.6.0._21.i.i.i.i.sroa_idx.peel.i.i.i.i.i.i, i64 28, i1 false), !noalias !302
  %.not3.i.peel.i.i.i.i.i.i = icmp eq i32 %x.i.sroa.0.0.copyload.peel.i.i.i.i.i.i, 4
  br i1 %.not3.i.peel.i.i.i.i.i.i, label %bb2.i.i.peel.i.i.i.i.i.i, label %bb2.thread.i.i.i.i

bb2.i.i.peel.i.i.i.i.i.i:                         ; preds = %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i, %bb11.i.peel.i.i.i.i.i.i
  %self.val.i.i.i30.i.i = phi ptr [ %_23.i.i.i.i.peel.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i ], [ %_21.i.i.i.i.peel.i.i.i.i.i.i, %bb11.i.peel.i.i.i.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !303)
  call void @llvm.experimental.noalias.scope.decl(metadata !306), !noalias !309
  call void @llvm.experimental.noalias.scope.decl(metadata !310), !noalias !309
  %13 = ptrtoint ptr %_12.i.i.i.i.peel.i.i.i.i.i.i to i64
  %14 = ptrtoint ptr %self.val.i.i.i30.i.i to i64
  %15 = sub nuw i64 %13, %14
  %16 = lshr exact i64 %15, 5
  %_7.i.i.i.i3481.i.i = icmp eq ptr %_12.i.i.i.i.peel.i.i.i.i.i.i, %self.val.i.i.i30.i.i
  br i1 %_7.i.i.i.i3481.i.i, label %bb2.i.i.i50.i.i, label %bb5.i.i.i.i35.i.i

cleanup.body.i.i.i44.i.i:                         ; preds = %bb4.i.i.i.i38.i.i
  %capacity1.i.i.i.i.i45.i.i = load i64, ptr %11, align 8, !alias.scope !313, !noalias !314, !noundef !9
  %17 = icmp eq i64 %capacity1.i.i.i.i.i45.i.i, 0
  br i1 %17, label %bb9.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i46.i.i

bb2.i.i.i.i.i.i.i.i46.i.i:                        ; preds = %cleanup.body.i.i.i44.i.i
  %ptr.i.i.i.i.i47.i.i = load ptr, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !313, !noalias !314, !nonnull !9, !noundef !9
  br label %bb9.i.i.i.i.i.sink.split.i.i

bb5.i.i.i.i35.i.i:                                ; preds = %bb2.i.i.peel.i.i.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i
  %_3.sroa.0.0.i.i.i.i3382.i.i = phi i64 [ %18, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i ], [ 0, %bb2.i.i.peel.i.i.i.i.i.i ]
  %_6.i.i.i.i36.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i30.i.i, i64 %_3.sroa.0.0.i.i.i.i3382.i.i
  %18 = add nuw nsw i64 %_3.sroa.0.0.i.i.i.i3382.i.i, 1
  call void @llvm.experimental.noalias.scope.decl(metadata !315)
  %19 = load i32, ptr %_6.i.i.i.i36.i.i, align 8, !range !132, !alias.scope !315, !noalias !318, !noundef !9
  switch i32 %19, label %default.unreachable [
    i32 0, label %bb2.i.i.i
    i32 1, label %bb3.i.i.i
    i32 2, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i
    i32 3, label %bb4.i.i.i
  ]

default.unreachable:                              ; preds = %bb5.i.i.i.i35.i.i
  unreachable

bb4.i.i.i:                                        ; preds = %bb5.i.i.i.i35.i.i
  %20 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 8
  %.val.i.i.i = load i64, ptr %20, align 8, !range !60, !alias.scope !315, !noalias !318, !noundef !9
  switch i64 %.val.i.i.i, label %bb2.i.i.i4.i.i.i.i.i.i.i.i [
    i64 -9223372036854775808, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i
    i64 0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i
  ]

bb2.i.i.i4.i.i.i.i.i.i.i.i:                       ; preds = %bb4.i.i.i
  %21 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 16
  %.val1.i.i.i = load ptr, ptr %21, align 8, !alias.scope !315, !noalias !318, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1.i.i.i, i64 noundef %.val.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #21, !noalias !319
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i

bb2.i.i.i:                                        ; preds = %bb5.i.i.i.i35.i.i
  %22 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 8
  %23 = load i32, ptr %22, align 8, !range !95, !alias.scope !322, !noalias !318, !noundef !9
  %24 = icmp eq i32 %23, 0
  br i1 %24, label %bb2.i.i.i57.i.i, label %bb3.i.i.i.i.i

bb2.i.i.i57.i.i:                                  ; preds = %bb2.i.i.i
  %25 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 24
  %26 = load i32, ptr %25, align 4, !alias.scope !327, !noalias !318, !noundef !9
  %27 = icmp eq i32 %26, 0
  br i1 %27, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, label %bb2.i.i.i.i.i.i58.i.i

bb2.i.i.i.i.i.i58.i.i:                            ; preds = %bb2.i.i.i57.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %25)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i unwind label %cleanup.i.i.i.i37.i.i, !noalias !257

bb3.i.i.i.i.i:                                    ; preds = %bb2.i.i.i
  %28 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 16
; invoke <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %28)
          to label %bb4.i.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i.i, !noalias !318

cleanup.i.i.i.i.i.i.i:                            ; preds = %bb3.i.i.i.i.i
  %29 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !334), !noalias !337
  call void @llvm.experimental.noalias.scope.decl(metadata !338), !noalias !337
  call void @llvm.experimental.noalias.scope.decl(metadata !341), !noalias !337
  %_5.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %28, align 8, !alias.scope !344, !noalias !318, !nonnull !9, !noundef !9
  %_8.i.i.i.i.i.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !349, !noundef !9
  %val.i.i.i.i.i.i.i.i.i.i = add i64 %_8.i.i.i.i.i.i.i.i.i.i, -1
  store i64 %val.i.i.i.i.i.i.i.i.i.i, ptr %_5.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !349
  %30 = icmp eq i64 %val.i.i.i.i.i.i.i.i.i.i, 0
  br i1 %30, label %bb1.i.i.i.i.i.i.i.i.i.i, label %cleanup.i.i.i.i37.body.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %cleanup.i.i.i.i.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %28) #25
          to label %cleanup.i.i.i.i37.body.i.i unwind label %terminate.i.i.i.i.i.i.i, !noalias !318

bb4.i.i.i.i.i.i.i:                                ; preds = %bb3.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !350), !noalias !337
  call void @llvm.experimental.noalias.scope.decl(metadata !353), !noalias !337
  call void @llvm.experimental.noalias.scope.decl(metadata !356), !noalias !337
  %_5.i.i.i1.i.i.i.i.i.i.i = load ptr, ptr %28, align 8, !alias.scope !359, !noalias !318, !nonnull !9, !noundef !9
  %_8.i.i.i2.i.i.i.i.i.i.i = load i64, ptr %_5.i.i.i1.i.i.i.i.i.i.i, align 8, !noalias !360, !noundef !9
  %val.i.i.i3.i.i.i.i.i.i.i = add i64 %_8.i.i.i2.i.i.i.i.i.i.i, -1
  store i64 %val.i.i.i3.i.i.i.i.i.i.i, ptr %_5.i.i.i1.i.i.i.i.i.i.i, align 8, !noalias !360
  %31 = icmp eq i64 %val.i.i.i3.i.i.i.i.i.i.i, 0
  br i1 %31, label %bb1.i.i.i4.i.i.i.i.i.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i

bb1.i.i.i4.i.i.i.i.i.i.i:                         ; preds = %bb4.i.i.i.i.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %28) #25
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i unwind label %cleanup.i.i.i.i37.i.i, !noalias !257

terminate.i.i.i.i.i.i.i:                          ; preds = %bb1.i.i.i.i.i.i.i.i.i.i
  %32 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !318
  unreachable

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i35.i.i
  %33 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 8
  call void @llvm.experimental.noalias.scope.decl(metadata !361), !noalias !337
  call void @llvm.experimental.noalias.scope.decl(metadata !364), !noalias !337
  %34 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 24
  %35 = load i8, ptr %34, align 8, !range !180, !alias.scope !367, !noalias !318, !noundef !9
  %36 = icmp eq i8 %35, 2
  br i1 %36, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, label %bb2.i.i2.i.i.i

bb2.i.i2.i.i.i:                                   ; preds = %bb3.i.i.i
  %37 = getelementptr inbounds nuw i8, ptr %_6.i.i.i.i36.i.i, i64 16
  %_1.val1.i.i.i.i.i = load i64, ptr %37, align 8, !alias.scope !367, !noalias !318, !noundef !9
  %38 = icmp eq i64 %_1.val1.i.i.i.i.i, 0
  br i1 %38, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i: ; preds = %bb2.i.i2.i.i.i
  %_1.val.i.i.i.i.i = load ptr, ptr %33, align 8, !alias.scope !367, !noalias !318, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val.i.i.i.i.i, i64 noundef %_1.val1.i.i.i.i.i, i64 noundef 1) #21, !noalias !368
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i: ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i.i.i.i.i.i, %bb2.i.i2.i.i.i, %bb3.i.i.i, %bb1.i.i.i4.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i58.i.i, %bb2.i.i.i57.i.i, %bb2.i.i.i4.i.i.i.i.i.i.i.i, %bb4.i.i.i, %bb4.i.i.i, %bb5.i.i.i.i35.i.i
  %_7.i.i.i.i34.i.i = icmp eq i64 %18, %16
  br i1 %_7.i.i.i.i34.i.i, label %bb2.i.i.i50.i.i, label %bb5.i.i.i.i35.i.i

bb4.i.i.i.i38.i.i:                                ; preds = %bb3.i.i.i.i41.i.i, %cleanup.i.i.i.i37.body.i.i
  %_3.sroa.0.1.i.i.i.i39.i.i = phi i64 [ %18, %cleanup.i.i.i.i37.body.i.i ], [ %40, %bb3.i.i.i.i41.i.i ]
  %_5.i.i.i.i40.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i.i39.i.i, %16
  br i1 %_5.i.i.i.i40.i.i, label %cleanup.body.i.i.i44.i.i, label %bb3.i.i.i.i41.i.i

cleanup.i.i.i.i37.i.i:                            ; preds = %bb1.i.i.i4.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i58.i.i
  %39 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i.i.i37.body.i.i

cleanup.i.i.i.i37.body.i.i:                       ; preds = %cleanup.i.i.i.i37.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %cleanup.i.i.i.i.i.i.i
  %eh.lpad-body61.i.i = phi { ptr, i32 } [ %39, %cleanup.i.i.i.i37.i.i ], [ %29, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %29, %cleanup.i.i.i.i.i.i.i ]
  br label %bb4.i.i.i.i38.i.i

bb3.i.i.i.i41.i.i:                                ; preds = %bb4.i.i.i.i38.i.i
  %_4.i.i.i.i42.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i30.i.i, i64 %_3.sroa.0.1.i.i.i.i39.i.i
  %40 = add i64 %_3.sroa.0.1.i.i.i.i39.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i.i.i42.i.i) #22
          to label %bb4.i.i.i.i38.i.i unwind label %terminate.i.i.i.i43.i.i, !noalias !318

terminate.i.i.i.i43.i.i:                          ; preds = %bb3.i.i.i.i41.i.i
  %41 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !318
  unreachable

bb2.i.i.i50.i.i:                                  ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote.exit.i.i, %bb2.i.i.peel.i.i.i.i.i.i
  %capacity1.i.i3.i.i.i51.i.i = load i64, ptr %11, align 8, !alias.scope !313, !noalias !314, !noundef !9
  %42 = icmp eq i64 %capacity1.i.i3.i.i.i51.i.i, 0
  br i1 %42, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.peel.i.i.i.i.i.i, label %bb2.i.i.i.i.i4.i.i.i52.i.i

bb2.i.i.i.i.i4.i.i.i52.i.i:                       ; preds = %bb2.i.i.i50.i.i
  %ptr.i.i5.i.i.i53.i.i = load ptr, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !313, !noalias !314, !nonnull !9, !noundef !9
  %alloc_size.i.i.i.i.i.i6.i.i.i54.i.i = shl nuw i64 %capacity1.i.i3.i.i.i51.i.i, 5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i5.i.i.i53.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i54.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !318
  br label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.peel.i.i.i.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.peel.i.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i4.i.i.i52.i.i, %bb2.i.i.i50.i.i
  store ptr null, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !369, !noalias !314
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit.peel.i.i.i.i.i.i

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit.peel.i.i.i.i.i.i: ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.peel.i.i.i.i.i.i, %bb1.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %x.i.sroa.6.i.i.i.i.i.i)
  call void @llvm.experimental.noalias.scope.decl(metadata !370)
  %.not.i2.peel.i.i.i.i.i.i = icmp eq i64 %self.promoted.i.i.i.i.i.i, -9223372036854775806
  br i1 %.not.i2.peel.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, label %bb5.i.peel.i.i.i.i.i.i

bb5.i.peel.i.i.i.i.i.i:                           ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit.peel.i.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !373)
  store i64 -9223372036854775807, ptr %_3.i.i, align 8, !alias.scope !376, !noalias !381
  %.not.i.i.i.peel.i.i.i.i.i.i = icmp eq i64 %self.promoted.i.i.i.i.i.i, -9223372036854775807
  br i1 %.not.i.i.i.peel.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i

bb1.i.i.i.i.i.i:                                  ; preds = %bb5.i.peel.i.i.i.i.i.i
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_2.i.i.i.i.i.i.i.i1.i), !noalias !384
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %args.sroa.4.0._2.i.sroa_idx.i.i.i.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(24) %self1.sroa.5.0.self.sroa_idx.i.i.i.i.i.i.i.i.i, i64 24, i1 false), !noalias !281
  store i64 %self.promoted.i.i.i.i.i.i, ptr %_2.i.i.i.i.i.i.i.i1.i, align 8, !noalias !384
; invoke <proc_macro2::imp::TokenStream>::unwrap_stable
  %_0.i.i.i.peel.i.i5.i.i.i.i = invoke noundef nonnull ptr @_RNvMs_NtCs8M6BBVNvC7a_11proc_macro23impNtB4_11TokenStream13unwrap_stable(ptr noalias noundef nonnull align 8 captures(address) dereferenceable(32) %_2.i.i.i.i.i.i.i.i1.i)
          to label %_0.i.i.i.peel.i.i.noexc.i.i.i.i unwind label %cleanup.i.i.i.i, !noalias !385

_0.i.i.i.peel.i.i.noexc.i.i.i.i:                  ; preds = %bb1.i.i.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_2.i.i.i.i.i.i.i.i1.i), !noalias !384
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10.i.i.i.i.i.i), !noalias !386
; invoke <proc_macro2::fallback::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
  invoke void @_RNvXsc_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_10.i.i.i.i.i.i, ptr noundef nonnull %_0.i.i.i.peel.i.i5.i.i.i.i)
          to label %.noexc.i.i.i.i unwind label %cleanup.i.i.i.i, !noalias !385

.noexc.i.i.i.i:                                   ; preds = %_0.i.i.i.peel.i.i.noexc.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_2.sroa.4.0._0.sroa_idx.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %_10.i.i.i.i.i.i, i64 32, i1 false), !noalias !281
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10.i.i.i.i.i.i), !noalias !386
  %.pre.i.i.i.i.i.i = load ptr, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !387, !noalias !389
  call void @llvm.experimental.noalias.scope.decl(metadata !391)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %x.i.sroa.6.i.i.i.i.i.i)
  %.not.i.i.i.i.i.i.i = icmp eq ptr %.pre.i.i.i.i.i.i, null
  br i1 %.not.i.i.i.i.i.i.i, label %bb8.loopexit.i.i.i.i.i.i, label %bb11.i.i.i.i.i.i.i

bb11.i.i.i.i.i.i.i:                               ; preds = %.noexc.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !392)
  call void @llvm.experimental.noalias.scope.decl(metadata !394)
  call void @llvm.experimental.noalias.scope.decl(metadata !396)
  %_12.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %_14.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !398, !noalias !399, !nonnull !9, !noundef !9
  %_21.i.i.i.i.i.i.i.i.i.i = load ptr, ptr %7, align 8, !alias.scope !398, !noalias !399, !nonnull !9, !noundef !9
  %_9.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %_21.i.i.i.i.i.i.i.i.i.i, %_12.i.i.i.i.i.i.i.i.i.i
  br i1 %_9.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i5.i, label %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i

_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i: ; preds = %bb11.i.i.i.i.i.i.i
  %_23.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i.i.i.i.i.i.i, i64 32
  store ptr %_23.i.i.i.i.i.i.i.i.i.i, ptr %7, align 8, !alias.scope !398, !noalias !399
  %x.i.sroa.0.0.copyload.i.i.i.i.i.i = load i32, ptr %_21.i.i.i.i.i.i.i.i.i.i, align 8, !noalias !400
  %x.i.sroa.6.0._21.i.i.i.i.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i.i.i.i.i.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %x.i.sroa.6.i.i.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %x.i.sroa.6.0._21.i.i.i.i.sroa_idx.i.i.i.i.i.i, i64 28, i1 false), !noalias !400
  %.not3.i.i.i.i.i.i.i = icmp eq i32 %x.i.sroa.0.0.copyload.i.i.i.i.i.i, 4
  br i1 %.not3.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i5.i, label %bb2.thread.i.i.i.i

bb2.i.i.i.i.i.i.i5.i:                             ; preds = %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i
  %self.val.i.i.i3.i.i = phi ptr [ %_23.i.i.i.i.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i ], [ %_21.i.i.i.i.i.i.i.i.i.i, %bb11.i.i.i.i.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !401)
  call void @llvm.experimental.noalias.scope.decl(metadata !404), !noalias !407
  call void @llvm.experimental.noalias.scope.decl(metadata !408), !noalias !407
  %43 = ptrtoint ptr %_12.i.i.i.i.i.i.i.i.i.i to i64
  %44 = ptrtoint ptr %self.val.i.i.i3.i.i to i64
  %45 = sub nuw i64 %43, %44
  %46 = lshr exact i64 %45, 5
  br label %bb6.i.i.i.i5.i.i

cleanup.body.i.i.i17.i.i:                         ; preds = %bb4.i.i.i.i11.i.i
  %capacity1.i.i.i.i.i18.i.i = load i64, ptr %11, align 8, !alias.scope !411, !noalias !389, !noundef !9
  %47 = icmp eq i64 %capacity1.i.i.i.i.i18.i.i, 0
  br i1 %47, label %bb9.i.i.i.i.i.i.i, label %bb9.i.i.i.i.i.sink.split.i.i

bb6.i.i.i.i5.i.i:                                 ; preds = %bb5.i.i.i.i8.i.i, %bb2.i.i.i.i.i.i.i5.i
  %_3.sroa.0.0.i.i.i.i6.i.i = phi i64 [ 0, %bb2.i.i.i.i.i.i.i5.i ], [ %48, %bb5.i.i.i.i8.i.i ]
  %_7.i.i.i.i7.i.i = icmp eq i64 %_3.sroa.0.0.i.i.i.i6.i.i, %46
  br i1 %_7.i.i.i.i7.i.i, label %bb2.i.i.i23.i.i, label %bb5.i.i.i.i8.i.i

bb5.i.i.i.i8.i.i:                                 ; preds = %bb6.i.i.i.i5.i.i
  %_6.i.i.i.i9.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i3.i.i, i64 %_3.sroa.0.0.i.i.i.i6.i.i
  %48 = add nuw nsw i64 %_3.sroa.0.0.i.i.i.i6.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_6.i.i.i.i9.i.i)
          to label %bb6.i.i.i.i5.i.i unwind label %cleanup.i.i.i.i10.i.i, !noalias !412

bb4.i.i.i.i11.i.i:                                ; preds = %bb3.i.i.i.i14.i.i, %cleanup.i.i.i.i10.i.i
  %_3.sroa.0.1.i.i.i.i12.i.i = phi i64 [ %48, %cleanup.i.i.i.i10.i.i ], [ %50, %bb3.i.i.i.i14.i.i ]
  %_5.i.i.i.i13.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i.i12.i.i, %46
  br i1 %_5.i.i.i.i13.i.i, label %cleanup.body.i.i.i17.i.i, label %bb3.i.i.i.i14.i.i

cleanup.i.i.i.i10.i.i:                            ; preds = %bb5.i.i.i.i8.i.i
  %49 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i.i.i11.i.i

bb3.i.i.i.i14.i.i:                                ; preds = %bb4.i.i.i.i11.i.i
  %_4.i.i.i.i15.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i3.i.i, i64 %_3.sroa.0.1.i.i.i.i12.i.i
  %50 = add i64 %_3.sroa.0.1.i.i.i.i12.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i.i.i15.i.i) #22
          to label %bb4.i.i.i.i11.i.i unwind label %terminate.i.i.i.i16.i.i, !noalias !412

terminate.i.i.i.i16.i.i:                          ; preds = %bb3.i.i.i.i14.i.i
  %51 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !412
  unreachable

bb2.i.i.i23.i.i:                                  ; preds = %bb6.i.i.i.i5.i.i
  %capacity1.i.i3.i.i.i24.i.i = load i64, ptr %11, align 8, !alias.scope !411, !noalias !389, !noundef !9
  %52 = icmp eq i64 %capacity1.i.i3.i.i.i24.i.i, 0
  br i1 %52, label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.i.i.i.i.i.i, label %bb2.i.i.i.i.i4.i.i.i25.i.i

bb2.i.i.i.i.i4.i.i.i25.i.i:                       ; preds = %bb2.i.i.i23.i.i
  %alloc_size.i.i.i.i.i.i6.i.i.i27.i.i = shl nuw i64 %capacity1.i.i3.i.i.i24.i.i, 5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.pre.i.i.i.i.i.i, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i27.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !412
  br label %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.i.i.i.i.i.i

_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.i.i.i.i.i.i: ; preds = %bb2.i.i.i.i.i4.i.i.i25.i.i, %bb2.i.i.i23.i.i
  store ptr null, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !387, !noalias !389
  br label %bb8.loopexit.i.i.i.i.i.i

bb9.i.i.i.i.i.sink.split.i.i:                     ; preds = %cleanup.body.i.i.i17.i.i, %bb2.i.i.i.i.i.i.i.i46.i.i
  %capacity1.i.i.i.i.i45.sink.i.i = phi i64 [ %capacity1.i.i.i.i.i45.i.i, %bb2.i.i.i.i.i.i.i.i46.i.i ], [ %capacity1.i.i.i.i.i18.i.i, %cleanup.body.i.i.i17.i.i ]
  %ptr.i.i.i.i.i47.sink.i.i = phi ptr [ %ptr.i.i.i.i.i47.i.i, %bb2.i.i.i.i.i.i.i.i46.i.i ], [ %.pre.i.i.i.i.i.i, %cleanup.body.i.i.i17.i.i ]
  %lpad.phi.i.i.i.i.ph.i.i = phi { ptr, i32 } [ %eh.lpad-body61.i.i, %bb2.i.i.i.i.i.i.i.i46.i.i ], [ %49, %cleanup.body.i.i.i17.i.i ]
  %alloc_size.i.i.i.i.i.i.i.i.i48.i.i = shl nuw i64 %capacity1.i.i.i.i.i45.sink.i.i, 5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %ptr.i.i.i.i.i47.sink.i.i, i64 noundef %alloc_size.i.i.i.i.i.i.i.i.i48.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !413
  br label %bb9.i.i.i.i.i.i.i

bb9.i.i.i.i.i.i.i:                                ; preds = %bb9.i.i.i.i.i.sink.split.i.i, %cleanup.body.i.i.i17.i.i, %cleanup.body.i.i.i44.i.i
  %lpad.phi.i.i.i.i.i.i = phi { ptr, i32 } [ %49, %cleanup.body.i.i.i17.i.i ], [ %eh.lpad-body61.i.i, %cleanup.body.i.i.i44.i.i ], [ %lpad.phi.i.i.i.i.ph.i.i, %bb9.i.i.i.i.i.sink.split.i.i ]
  store ptr null, ptr %_2.sroa.4.0._0.sroa_idx.i.i.i, align 8, !alias.scope !387, !noalias !389
  br label %bb11.i.i.i.i

bb2.thread.i.i.i.i:                               ; preds = %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i
  %53 = phi ptr [ %12, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i ], [ %.pre.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i ]
  %x.i.sroa.0.0.copyload.lcssa.i.i.i.i.i.i = phi i32 [ %x.i.sroa.0.0.copyload.peel.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.peel.i.i.i.i.i.i ], [ %x.i.sroa.0.0.copyload.i.i.i.i.i.i, %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_3.sroa.8.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %x.i.sroa.6.i.i.i.i.i.i, i64 28, i1 false), !noalias !414
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %x.i.sroa.6.i.i.i.i.i.i)
  br label %bb3.i.i.i.i

bb8.loopexit.i.i.i.i.i.i:                         ; preds = %_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote.exit.thread25.i.i.i.i.i.i, %.noexc.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %x.i.sroa.6.i.i.i.i.i.i)
  store i64 -9223372036854775807, ptr %_3.i.i, align 8, !alias.scope !415, !noalias !381
  br label %bb8.i.i.i.i.i.i

bb8.i.i.i.i.i.i:                                  ; preds = %bb8.loopexit.i.i.i.i.i.i, %bb5.i.peel.i.i.i.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit.peel.i.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !418)
  %54 = load ptr, ptr %_2.sroa.6.0._0.sroa_idx.i.i.i, align 8, !alias.scope !421, !noalias !422, !noundef !9
  %.not.i5.i.i.i.i.i.i = icmp eq ptr %54, null
  br i1 %.not.i5.i.i.i.i.i.i, label %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote.exit.i.i.i, label %bb11.i6.i.i.i.i.i.i

bb11.i6.i.i.i.i.i.i:                              ; preds = %bb8.i.i.i.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !424)
  call void @llvm.experimental.noalias.scope.decl(metadata !427)
  call void @llvm.experimental.noalias.scope.decl(metadata !430)
  %_12.i.i.i.i8.i.i.i.i.i.i = load ptr, ptr %_14.i.i.i.i7.i.i.i.i.i.i, align 8, !alias.scope !433, !noalias !434, !nonnull !9, !noundef !9
  %_21.i.i.i.i9.i.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !433, !noalias !434, !nonnull !9, !noundef !9
  %_9.i.i.i.i10.i.i.i.i.i.i = icmp eq ptr %_21.i.i.i.i9.i.i.i.i.i.i, %_12.i.i.i.i8.i.i.i.i.i.i
  br i1 %_9.i.i.i.i10.i.i.i.i.i.i, label %bb2.i.i16.i.i.i.i.i.i, label %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i11.i.i.i.i.i.i

_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i11.i.i.i.i.i.i: ; preds = %bb11.i6.i.i.i.i.i.i
  %_23.i.i.i.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i9.i.i.i.i.i.i, i64 32
  store ptr %_23.i.i.i.i12.i.i.i.i.i.i, ptr %8, align 8, !alias.scope !433, !noalias !434
  %x.i4.sroa.0.0.copyload.i.i.i.i.i.i = load i32, ptr %_21.i.i.i.i9.i.i.i.i.i.i, align 8, !noalias !438
  %.not3.i14.i.i.i.i.i.i = icmp eq i32 %x.i4.sroa.0.0.copyload.i.i.i.i.i.i, 4
  br i1 %.not3.i14.i.i.i.i.i.i, label %bb2.i.i16.i.i.i.i.i.i.split.loop.exit54, label %bb2.i.i.i.i

bb2.i.i16.i.i.i.i.i.i.split.loop.exit54:          ; preds = %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i11.i.i.i.i.i.i
  %_23.i.i.i.i12.i.i.i.i.i.i.le = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i9.i.i.i.i.i.i, i64 32
  br label %bb2.i.i16.i.i.i.i.i.i

bb2.i.i16.i.i.i.i.i.i:                            ; preds = %bb11.i6.i.i.i.i.i.i, %bb2.i.i16.i.i.i.i.i.i.split.loop.exit54
  %self.val.i.i.i.i.i = phi ptr [ %_23.i.i.i.i12.i.i.i.i.i.i.le, %bb2.i.i16.i.i.i.i.i.i.split.loop.exit54 ], [ %_21.i.i.i.i9.i.i.i.i.i.i, %bb11.i6.i.i.i.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !439)
  call void @llvm.experimental.noalias.scope.decl(metadata !442), !noalias !445
  call void @llvm.experimental.noalias.scope.decl(metadata !446), !noalias !445
  %55 = ptrtoint ptr %_12.i.i.i.i8.i.i.i.i.i.i to i64
  %56 = ptrtoint ptr %self.val.i.i.i.i.i to i64
  %57 = sub nuw i64 %55, %56
  %58 = lshr exact i64 %57, 5
  br label %bb6.i.i.i.i.i.i

cleanup.body.i.i.i.i.i:                           ; preds = %bb4.i.i.i.i.i.i
  %59 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 80
  %capacity1.i.i.i.i.i.i.i = load i64, ptr %59, align 8, !alias.scope !449, !noalias !422, !noundef !9
  %60 = icmp eq i64 %capacity1.i.i.i.i.i.i.i, 0
  br i1 %60, label %bb9.i17.i.i.i.i.body.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %cleanup.body.i.i.i.i.i
  %alloc_size.i.i.i.i.i.i.i.i.i.i.i = shl nuw i64 %capacity1.i.i.i.i.i.i.i, 5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %54, i64 noundef %alloc_size.i.i.i.i.i.i.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !450
  br label %bb9.i17.i.i.i.i.body.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb5.i.i.i.i.i.i, %bb2.i.i16.i.i.i.i.i.i
  %_3.sroa.0.0.i.i.i.i.i.i = phi i64 [ 0, %bb2.i.i16.i.i.i.i.i.i ], [ %61, %bb5.i.i.i.i.i.i ]
  %_7.i.i.i.i.i.i = icmp eq i64 %_3.sroa.0.0.i.i.i.i.i.i, %58
  br i1 %_7.i.i.i.i.i.i, label %bb2.i.i.i.i.i, label %bb5.i.i.i.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i.i
  %_6.i.i.i.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i.i.i, i64 %_3.sroa.0.0.i.i.i.i.i.i
  %61 = add nuw nsw i64 %_3.sroa.0.0.i.i.i.i.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_6.i.i.i.i.i.i)
          to label %bb6.i.i.i.i.i.i unwind label %cleanup.i.i.i.i.i.i, !noalias !450

bb4.i.i.i.i.i.i:                                  ; preds = %bb3.i.i.i.i.i6.i, %cleanup.i.i.i.i.i.i
  %_3.sroa.0.1.i.i.i.i.i.i = phi i64 [ %61, %cleanup.i.i.i.i.i.i ], [ %63, %bb3.i.i.i.i.i6.i ]
  %_5.i.i.i.i.i.i = icmp eq i64 %_3.sroa.0.1.i.i.i.i.i.i, %58
  br i1 %_5.i.i.i.i.i.i, label %cleanup.body.i.i.i.i.i, label %bb3.i.i.i.i.i6.i

cleanup.i.i.i.i.i.i:                              ; preds = %bb5.i.i.i.i.i.i
  %62 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i.i.i.i.i

bb3.i.i.i.i.i6.i:                                 ; preds = %bb4.i.i.i.i.i.i
  %_4.i.i.i.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %self.val.i.i.i.i.i, i64 %_3.sroa.0.1.i.i.i.i.i.i
  %63 = add i64 %_3.sroa.0.1.i.i.i.i.i.i, 1
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i.i.i.i.i) #22
          to label %bb4.i.i.i.i.i.i unwind label %terminate.i.i.i.i.i.i, !noalias !450

terminate.i.i.i.i.i.i:                            ; preds = %bb3.i.i.i.i.i6.i
  %64 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !450
  unreachable

bb2.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i.i
  %65 = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 80
  %capacity1.i.i3.i.i.i.i.i = load i64, ptr %65, align 8, !alias.scope !449, !noalias !422, !noundef !9
  %66 = icmp eq i64 %capacity1.i.i3.i.i.i.i.i, 0
  br i1 %66, label %bb2.thread13.i.i.i.i, label %bb2.i.i.i.i.i4.i.i.i.i.i

bb2.i.i.i.i.i4.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i
  %alloc_size.i.i.i.i.i.i6.i.i.i.i.i = shl nuw i64 %capacity1.i.i3.i.i.i.i.i, 5
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %54, i64 noundef %alloc_size.i.i.i.i.i.i6.i.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #21, !noalias !450
  br label %bb2.thread13.i.i.i.i

bb2.thread13.i.i.i.i:                             ; preds = %bb2.i.i.i.i.i4.i.i.i.i.i, %bb2.i.i.i.i.i
  store ptr null, ptr %_2.sroa.6.0._0.sroa_idx.i.i.i, align 8, !alias.scope !421, !noalias !422
  br label %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote.exit.i.i.i

bb9.i17.i.i.i.i.body.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i, %cleanup.body.i.i.i.i.i
  store ptr null, ptr %_2.sroa.6.0._0.sroa_idx.i.i.i, align 8, !alias.scope !421, !noalias !422
  br label %bb11.i.i.i.i

bb11.i.i.i.i:                                     ; preds = %cleanup2.i.i.i.i, %cleanup.i.i.i.i, %bb9.i17.i.i.i.i.body.i.i, %bb9.i.i.i.i.i.i.i
  %.pn.i.i.i.i = phi { ptr, i32 } [ %69, %cleanup2.i.i.i.i ], [ %67, %cleanup.i.i.i.i ], [ %lpad.phi.i.i.i.i.i.i, %bb9.i.i.i.i.i.i.i ], [ %62, %bb9.i17.i.i.i.i.body.i.i ]
; invoke core::ptr::drop_in_place::<core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::sources::once::Once<proc_macro2::TokenStream>, <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend<core::iter::sources::once::Once<proc_macro2::TokenStream>>::{closure#0}>, <proc_macro2::imp::TokenStream>::unwrap_stable>>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten7FlattenINtNtBL_3map3MapIB1n_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2b_B29_INtNtNtBN_6traits7collect6ExtendB29_E6extendB1H_E0ENvMs_NtB2b_3impNtB41_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(96) %_3.i.i) #22
          to label %bb12.i unwind label %terminate.i.i.i.i, !noalias !451

cleanup.i.i.i.i:                                  ; preds = %_0.i.i.i.peel.i.i.noexc.i.i.i.i, %bb1.i.i.i.i.i.i
  %67 = landingpad { ptr, i32 }
          cleanup
  br label %bb11.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote.exit.i11.i.i.i.i.i.i
  %x.i4.sroa.6.0._21.i.i.i.i9.sroa_idx.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_21.i.i.i.i9.i.i.i.i.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_3.sroa.8.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %x.i4.sroa.6.0._21.i.i.i.i9.sroa_idx.i.i.i.i.i.i, i64 28, i1 false), !noalias !385
  br label %bb3.i.i.i.i

bb3.i.i.i.i:                                      ; preds = %bb2.i.i.i.i, %bb2.thread.i.i.i.i
  %68 = phi ptr [ %53, %bb2.thread.i.i.i.i ], [ null, %bb2.i.i.i.i ]
  %.not.i.i.i.i.i.i = phi i1 [ false, %bb2.thread.i.i.i.i ], [ true, %bb2.i.i.i.i ]
  %_3.sroa.0.19.i.i.i.i = phi i32 [ %x.i.sroa.0.0.copyload.lcssa.i.i.i.i.i.i, %bb2.thread.i.i.i.i ], [ %x.i4.sroa.0.0.copyload.i.i.i.i.i.i, %bb2.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %element.i.i.i.i), !noalias !452
  store i32 %_3.sroa.0.19.i.i.i.i, ptr %element.i.i.i.i, align 8, !noalias !452
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_3.sroa.8.0.element.sroa_idx.i.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %_3.sroa.8.i.i.i.i, i64 28, i1 false), !noalias !452
  %len.i.i.i.i = load i64, ptr %9, align 8, !alias.scope !453, !noalias !454, !noundef !9
  %_19.i.i.i.i = icmp ult i64 %len.i.i.i.i, 288230376151711744
  call void @llvm.assume(i1 %_19.i.i.i.i)
  %self1.i.i.i.i = load i64, ptr %_7.i.i, align 8, !range !16, !alias.scope !453, !noalias !454, !noundef !9
  %_8.i.i.i.i = icmp eq i64 %len.i.i.i.i, %self1.i.i.i.i
  br i1 %_8.i.i.i.i, label %bb1.i.i.i.i.i, label %bb8.i.i.i.i

bb8.i.i.i.i:                                      ; preds = %bb1.i.i.i.i.i, %bb3.i.i.i.i
  %_23.i.i.i.i = load ptr, ptr %10, align 8, !alias.scope !453, !noalias !454, !nonnull !9, !noundef !9
  %dst.i.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %_23.i.i.i.i, i64 %len.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %dst.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %element.i.i.i.i, i64 32, i1 false), !noalias !385
  %new_len.i.i.i.i = add nuw nsw i64 %len.i.i.i.i, 1
  store i64 %new_len.i.i.i.i, ptr %9, align 8, !alias.scope !453, !noalias !454
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %element.i.i.i.i), !noalias !452
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_3.sroa.8.i.i.i.i)
  br label %bb1.i.i.i.i

cleanup2.i.i.i.i:                                 ; preds = %bb1.i.i.i.i.i
  %69 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %element.i.i.i.i) #22
          to label %bb11.i.i.i.i unwind label %terminate.i.i.i.i, !noalias !385

bb1.i.i.i.i.i:                                    ; preds = %bb3.i.i.i.i
  %70 = load ptr, ptr %_2.sroa.6.0._0.sroa_idx.i.i.i, align 8, !alias.scope !455, !noalias !460, !noundef !9
  %.not38.i.i.i.i.i.i = icmp eq ptr %70, null
  %.val1.i48.i.i.i.i.i.i = load ptr, ptr %_14.i.i.i.i7.i.i.i.i.i.i, align 8, !alias.scope !455, !noalias !460, !nonnull !9
  %71 = ptrtoint ptr %.val1.i48.i.i.i.i.i.i to i64
  %.val.i47.i.i.i.i.i.i = load ptr, ptr %8, align 8, !alias.scope !455, !noalias !460, !nonnull !9
  %72 = ptrtoint ptr %.val.i47.i.i.i.i.i.i to i64
  %73 = sub nuw i64 %71, %72
  %74 = lshr exact i64 %73, 5
  %.val1.i.i.i.i.i.i.i = load ptr, ptr %_14.i.i.i.i.i.i.i.i.i.i, align 8, !alias.scope !455, !noalias !460, !nonnull !9
  %75 = ptrtoint ptr %.val1.i.i.i.i.i.i.i to i64
  %.val.i.i.i.i.i.i.i = load ptr, ptr %7, align 8, !alias.scope !455, !noalias !460, !nonnull !9
  %76 = ptrtoint ptr %.val.i.i.i.i.i.i.i to i64
  %77 = sub nuw i64 %75, %76
  %78 = lshr exact i64 %77, 5
  %_4.sroa.7.0.i.i.i.i.i.i = select i1 %.not.i.i.i.i.i.i, i64 0, i64 %78
  %79 = add nuw nsw i64 %74, 1
  %80 = select i1 %.not38.i.i.i.i.i.i, i64 1, i64 %79
  %81 = add nuw nsw i64 %_4.sroa.7.0.i.i.i.i.i.i, %80
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(24) %_7.i.i, i64 noundef %len.i.i.i.i, i64 noundef range(i64 1, 0) %81, i64 noundef 8, i64 noundef 32)
          to label %bb8.i.i.i.i unwind label %cleanup2.i.i.i.i, !noalias !454

terminate.i.i.i.i:                                ; preds = %cleanup2.i.i.i.i, %bb11.i.i.i.i
  %82 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !451
  unreachable

_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote.exit.i.i.i: ; preds = %bb8.i.i.i.i.i.i, %bb2.thread13.i.i.i.i
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_3.sroa.8.i.i.i.i)
; call core::ptr::drop_in_place::<core::iter::adapters::flatten::Flatten<core::iter::adapters::map::Map<core::iter::adapters::map::Map<core::iter::sources::once::Once<proc_macro2::TokenStream>, <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend<core::iter::sources::once::Once<proc_macro2::TokenStream>>::{closure#0}>, <proc_macro2::imp::TokenStream>::unwrap_stable>>>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten7FlattenINtNtBL_3map3MapIB1n_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2b_B29_INtNtNtBN_6traits7collect6ExtendB29_E6extendB1H_E0ENvMs_NtB2b_3impNtB41_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(96) %_3.i.i), !noalias !227
  call void @llvm.lifetime.end.p0(i64 96, ptr nonnull %_3.i.i), !noalias !253
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i), !noalias !253
  br label %_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote.exit

bb4.i2.i:                                         ; preds = %bb9.i
  %83 = landingpad { ptr, i32 }
          cleanup
  %84 = load i64, ptr %_13.i, align 8, !range !233, !alias.scope !463, !noalias !476, !noundef !9
  %85 = icmp eq i64 %84, -9223372036854775807
  br i1 %85, label %bb12.i, label %bb2.i.i.i.i.i.i1.i.i

bb2.i.i.i.i.i.i1.i.i:                             ; preds = %bb4.i2.i
; invoke core::ptr::drop_in_place::<proc_macro2::TokenStream>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_13.i)
          to label %bb12.i unwind label %terminate.i3.i, !noalias !227

terminate.i3.i:                                   ; preds = %bb2.i.i.i.i.i.i1.i.i
  %86 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !227
  unreachable

bb12.i:                                           ; preds = %bb2.i.i.i.i.i.i, %bb13.i, %bb2.i.i.i.i.i.i1.i.i, %bb4.i2.i, %bb11.i.i.i.i, %bb2.i.i.i.i.i.i.i.i, %bb9.i.i, %bb7.i.i
  %eh.lpad-body12.i = phi { ptr, i32 } [ %lpad.thr_comm.split-lp.i, %bb2.i.i.i.i.i.i ], [ %lpad.thr_comm.split-lp.i, %bb13.i ], [ %lpad.thr_comm.i.i, %bb7.i.i ], [ %5, %bb2.i.i.i.i.i.i.i.i ], [ %5, %bb9.i.i ], [ %.pn.i.i.i.i, %bb11.i.i.i.i ], [ %83, %bb2.i.i.i.i.i.i1.i.i ], [ %83, %bb4.i2.i ]
  resume { ptr, i32 } %eh.lpad-body12.i

bb13.i:                                           ; preds = %bb3.i
  %lpad.thr_comm.split-lp.i = landingpad { ptr, i32 }
          cleanup
  %87 = load i64, ptr %_5, align 8, !range !233, !alias.scope !477, !noalias !224, !noundef !9
  %88 = icmp eq i64 %87, -9223372036854775807
  br i1 %88, label %bb12.i, label %bb2.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb13.i
; invoke core::ptr::drop_in_place::<proc_macro2::TokenStream>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_5)
          to label %bb12.i unwind label %terminate.i

terminate.i:                                      ; preds = %bb2.i.i.i.i.i.i
  %89 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote.exit: ; preds = %bb4.i.i, %_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote.exit.i.i.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  ret void
}

; <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
; Function Attrs: uwtable
define internal fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %self, ptr dead_on_return noalias noundef nonnull align 8 captures(address) dereferenceable(32) %stream) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_5.i.i.i.i = alloca [32 x i8], align 8
  %_12 = alloca [20 x i8], align 4
  %token = alloca [32 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !60, !noundef !9
  %1 = icmp eq i64 %0, -9223372036854775808
  br i1 %1, label %bb2, label %bb3

bb2:                                              ; preds = %start
  %tts = getelementptr inbounds nuw i8, ptr %self, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !488)
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::make_mut
  %_10.i = invoke fastcc noundef nonnull align 8 ptr @_RNvMsh_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE8make_mutCs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(8) %tts)
          to label %bb1.i unwind label %bb4.i, !noalias !488

bb1.i:                                            ; preds = %bb2
  %_6.sroa.0.0.copyload.i = load i32, ptr %stream, align 8, !alias.scope !488, !noalias !491
  %.not6.i.i.i = icmp eq i32 %_6.sroa.0.0.copyload.i, 4
  br i1 %.not6.i.i.i, label %bb11, label %bb3.i.i.i

bb3.i.i.i:                                        ; preds = %bb1.i
  %_6.sroa.4.0.tokens.sroa_idx.i = getelementptr inbounds nuw i8, ptr %stream, i64 4
  %_11.sroa.4.0._5.i.sroa_idx.i.i.i = getelementptr inbounds nuw i8, ptr %_5.i.i.i.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5.i.i.i.i), !noalias !493
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_11.sroa.4.0._5.i.sroa_idx.i.i.i, ptr noundef nonnull align 4 dereferenceable(28) %_6.sroa.4.0.tokens.sroa_idx.i, i64 28, i1 false), !noalias !491
  store i32 %_6.sroa.0.0.copyload.i, ptr %_5.i.i.i.i, align 8, !noalias !501
; call proc_macro2::fallback::push_token_from_proc_macro
  call void @_RNvNtCs8M6BBVNvC7a_11proc_macro28fallback26push_token_from_proc_macro(ptr noalias noundef nonnull align 8 dereferenceable(24) %_10.i, ptr noalias noundef nonnull align 8 captures(address) dereferenceable(32) %_5.i.i.i.i), !noalias !488
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5.i.i.i.i), !noalias !493
  br label %bb11

common.resume:                                    ; preds = %bb2.i.i.i.i.i5, %bb2.i.i, %cleanup.i, %bb4.i, %bb2.i.i.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %2, %bb2.i.i.i.i.i ], [ %2, %bb4.i ], [ %10, %bb2.i.i.i.i.i5 ], [ %10, %bb2.i.i ], [ %10, %cleanup.i ]
  resume { ptr, i32 } %common.resume.op

bb4.i:                                            ; preds = %bb2
  %2 = landingpad { ptr, i32 }
          cleanup
  %3 = load i32, ptr %stream, align 8, !range !502, !alias.scope !503, !noalias !491, !noundef !9
  %4 = icmp eq i32 %3, 4
  br i1 %4, label %common.resume, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb4.i
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %stream)
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %bb2.i.i.i.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb3:                                              ; preds = %start
  %iter.sroa.0.0.copyload = load i32, ptr %stream, align 8
  %.not9 = icmp eq i32 %iter.sroa.0.0.copyload, 4
  br i1 %.not9, label %bb11, label %bb7

bb7:                                              ; preds = %bb3
  %6 = getelementptr inbounds nuw i8, ptr %stream, i64 4
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_7.sroa.6.0.token.sroa_idx = getelementptr inbounds nuw i8, ptr %token, i64 4
  store i32 %iter.sroa.0.0.copyload, ptr %token, align 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_7.sroa.6.0.token.sroa_idx, ptr noundef nonnull align 4 dereferenceable(28) %6, i64 28, i1 false)
  call void @llvm.lifetime.start.p0(i64 20, ptr nonnull %_12)
; call proc_macro2::imp::into_compiler_token
  call void @_RNvNtCs8M6BBVNvC7a_11proc_macro23imp19into_compiler_token(ptr noalias noundef nonnull sret([20 x i8]) align 4 captures(none) dereferenceable(20) %_12, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %token)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !512)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !515)
  %len.i = load i64, ptr %7, align 8, !alias.scope !512, !noalias !515, !noundef !9
  %self1.i = load i64, ptr %self, align 8, !range !16, !alias.scope !512, !noalias !515, !noundef !9
  %_4.i = icmp eq i64 %len.i, %self1.i
  br i1 %_4.i, label %bb1.i4, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit2.loopexit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit2.loopexit: ; preds = %bb7, %bb1.i4
  %_14.i = load ptr, ptr %8, align 8, !alias.scope !512, !noalias !515, !nonnull !9, !noundef !9
  %end.i = getelementptr inbounds nuw %"proc_macro::TokenTree", ptr %_14.i, i64 %len.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(20) %end.i, ptr noundef nonnull align 4 dereferenceable(20) %_12, i64 20, i1 false), !noalias !512
  %9 = add i64 %len.i, 1
  store i64 %9, ptr %7, align 8, !alias.scope !512, !noalias !515
  call void @llvm.lifetime.end.p0(i64 20, ptr nonnull %_12)
  br label %bb11

bb11:                                             ; preds = %bb3, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit2.loopexit, %bb3.i.i.i, %bb1.i
  ret void

bb1.i4:                                           ; preds = %bb7
; invoke <alloc::raw_vec::RawVec<proc_macro::TokenTree>>::grow_one
  invoke void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeE8grow_oneCs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(24) %self)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote.exit2.loopexit unwind label %cleanup.i, !noalias !515

cleanup.i:                                        ; preds = %bb1.i4
  %10 = landingpad { ptr, i32 }
          cleanup
  %11 = getelementptr inbounds nuw i8, ptr %_12, i64 16
  %12 = load i8, ptr %11, align 4, !range !17, !alias.scope !517, !noalias !512, !noundef !9
  %13 = icmp samesign ult i8 %12, 4
  br i1 %13, label %bb2.i.i, label %common.resume

bb2.i.i:                                          ; preds = %cleanup.i
  %14 = getelementptr inbounds nuw i8, ptr %_12, i64 12
  %15 = load i32, ptr %14, align 4, !alias.scope !520, !noalias !512, !noundef !9
  %16 = icmp eq i32 %15, 0
  br i1 %16, label %common.resume, label %bb2.i.i.i.i.i5

bb2.i.i.i.i.i5:                                   ; preds = %bb2.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %14)
          to label %common.resume unwind label %terminate.i6, !noalias !512

terminate.i6:                                     ; preds = %bb2.i.i.i.i.i5
  %17 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !512
  unreachable
}

; <alloc::raw_vec::RawVec<proc_macro::TokenTree>>::grow_one
; Function Attrs: cold noinline uwtable
define void @_RNvMs3_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB5_6RawVecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeE8grow_oneCs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 captures(none) dereferenceable(16) %self) unnamed_addr #3 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  %self1 = load i64, ptr %self, align 8, !range !16, !noundef !9
  tail call void @llvm.experimental.noalias.scope.decl(metadata !527)
  %v16.i = shl nuw i64 %self1, 1
  %0 = tail call i64 @llvm.umax.i64(i64 %v16.i, i64 range(i64 0, -1) 4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !527
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val15.i = load ptr, ptr %1, align 8, !alias.scope !527
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self1, ptr %self.val15.i, i64 noundef %0, i64 noundef 4, i64 noundef 20), !noalias !527
  %_35.i = load i64, ptr %self3.i, align 8, !range !188, !noalias !527, !noundef !9
  %2 = trunc nuw i64 %_35.i to i1
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %2, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %start
  %e.0.i = load i64, ptr %3, align 8, !range !60, !noalias !527, !noundef !9
  %4 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %4, align 8, !noalias !527
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !527
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %e.0.i, i64 %e.1.i) #26
  unreachable

bb3:                                              ; preds = %start
  %v.0.i = load ptr, ptr %3, align 8, !noalias !527, !nonnull !9, !noundef !9
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !527
  store ptr %v.0.i, ptr %1, align 8, !alias.scope !527
  %5 = icmp sgt i64 %0, -1
  tail call void @llvm.assume(i1 %5)
  store i64 %0, ptr %self, align 8, !alias.scope !527
  ret void
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs1SHOYL7dTh6_5quote(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap, i64 noundef range(i64 1, 9) %elem_layout.0, i64 noundef range(i64 1, 33) %elem_layout.1) unnamed_addr #4 {
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
  br i1 %or.cond.i, label %bb11, label %bb11.i, !prof !530

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
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %alloc_size.i23, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0, i64 noundef %_27.sroa.7.01321) #21
  br label %bb7

bb7.thread:                                       ; preds = %bb14.thread
  %_16.i.i = inttoptr i64 %elem_layout.0 to ptr
  br label %bb9

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #21
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %new_size2.i, i64 noundef range(i64 1, -9223372036854775807) %elem_layout.0) #21
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

; <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::make_mut
; Function Attrs: inlinehint uwtable
define internal fastcc noundef nonnull align 8 ptr @_RNvMsh_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE8make_mutCs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(8) %this) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %val.i.i.i.i.i = alloca [32 x i8], align 8
  %vec.i.i.i.i.i = alloca [24 x i8], align 8
  %src1.i.i.i = alloca [24 x i8], align 8
  %in_progress.i = alloca [32 x i8], align 16
  %_22 = load ptr, ptr %this, align 8, !nonnull !9, !noundef !9
  %_4 = load i64, ptr %_22, align 8, !noundef !9
  %0 = icmp eq i64 %_4, 1
  br i1 %0, label %bb6, label %bb1

bb6:                                              ; preds = %start
  %_34 = getelementptr inbounds nuw i8, ptr %_22, i64 8
  %_30 = load i64, ptr %_34, align 8, !noundef !9
  %1 = icmp eq i64 %_30, 1
  br i1 %1, label %bb13, label %bb7

bb1:                                              ; preds = %start
  %2 = getelementptr i8, ptr %_22, i64 24
  %_6.val = load ptr, ptr %2, align 8
  %3 = getelementptr i8, ptr %_22, i64 32
  %_6.val4 = load i64, ptr %3, align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %in_progress.i)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !531)
; call alloc::rc::rc_inner_layout_for_value_layout
  %4 = tail call { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef 8, i64 noundef 24), !noalias !531
; call alloc::rc::rc_inner_layout_for_value_layout
  %5 = tail call { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef 8, i64 noundef 24), !noalias !531
  %layout.0.i.i.i.i = extractvalue { i64, i64 } %5, 0
  %layout.1.i.i.i.i = extractvalue { i64, i64 } %5, 1
  %6 = icmp eq i64 %layout.1.i.i.i.i, 0
  br i1 %6, label %bb2.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %bb1
  %_16.i.i.i.i.i.i.i = inttoptr i64 %layout.0.i.i.i.i to ptr
  br label %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb1
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #21, !noalias !531
; call __rustc::__rust_alloc
  %7 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %layout.1.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) %layout.0.i.i.i.i) #21, !noalias !531
  br label %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i.i

_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i ], [ %7, %bb4.i.i.i.i.i.i.i ]
  %8 = icmp eq ptr %_0.sroa.0.0.i.i.i.i.i.i.i, null
  br i1 %8, label %bb9.i.i.i, label %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote.exit.i

bb9.i.i.i:                                        ; preds = %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i.i
  %layout.1.i.i.i = extractvalue { i64, i64 } %4, 1
  %layout.0.i.i.i = extractvalue { i64, i64 } %4, 0
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef %layout.0.i.i.i, i64 noundef %layout.1.i.i.i) #26, !noalias !531
  unreachable

_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote.exit.i: ; preds = %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i.i
  store <2 x i64> splat (i64 1), ptr %_0.sroa.0.0.i.i.i.i.i.i.i, align 8, !noalias !531
  %9 = getelementptr inbounds nuw i8, ptr %in_progress.i, i64 16
  store ptr %_0.sroa.0.0.i.i.i.i.i.i.i, ptr %9, align 16, !alias.scope !531
  store <2 x i64> <i64 8, i64 24>, ptr %in_progress.i, align 16, !alias.scope !531
  %10 = getelementptr inbounds nuw i8, ptr %in_progress.i, i64 24
  store i8 1, ptr %10, align 8, !alias.scope !531
  %11 = icmp ne ptr %_6.val, null
  tail call void @llvm.assume(i1 %11)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %src1.i.i.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %val.i.i.i.i.i), !noalias !534
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vec.i.i.i.i.i), !noalias !537
  %_23.i.i.i.i.i.i.i.i = icmp eq i64 %_6.val4, 0
  br i1 %_23.i.i.i.i.i.i.i.i, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i

_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i.i: ; preds = %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote.exit.i
  store i64 0, ptr %vec.i.i.i.i.i, align 8, !noalias !537
  %12 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i.i, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %12, align 8, !noalias !537
  %13 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i.i, i64 16
  br label %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit

bb6.i.i.i.i.i.i.i.i:                              ; preds = %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote.exit.i
  %_24.i.i.i.i.i.i.i.i = shl nuw nsw i64 %_6.val4, 5
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #21, !noalias !541
; call __rustc::__rust_alloc
  %14 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %_24.i.i.i.i.i.i.i.i, i64 noundef range(i64 4, 9) 8) #21, !noalias !541
  %15 = icmp eq ptr %14, null
  br i1 %15, label %bb3.i.i.i.i.i.i, label %bb20.preheader.i.i.i.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb6.i.i.i.i.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 8, i64 %_24.i.i.i.i.i.i.i.i) #26
          to label %.noexc.i unwind label %cleanup.body.thread5.i

cleanup.body.thread5.i:                           ; preds = %bb3.i.i.i.i.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  br label %bb5.i

.noexc.i:                                         ; preds = %bb3.i.i.i.i.i.i
  unreachable

bb20.preheader.i.i.i.i.i:                         ; preds = %bb6.i.i.i.i.i.i.i.i
  store i64 %_6.val4, ptr %vec.i.i.i.i.i, align 8, !noalias !537
  %17 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i.i, i64 8
  store ptr %14, ptr %17, align 8, !noalias !537
  %18 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i.i, i64 16
  %_39.i.i.i.i.i = getelementptr inbounds nuw %"proc_macro2::TokenTree", ptr %_6.val, i64 %_6.val4
  br label %bb20.i.i.i.i.i

bb20.i.i.i.i.i:                                   ; preds = %bb6.i.i.i.i.i, %bb20.preheader.i.i.i.i.i
  %iter.sroa.10.018.i.i.i.i.i = phi i64 [ %19, %bb6.i.i.i.i.i ], [ %_6.val4, %bb20.preheader.i.i.i.i.i ]
  %iter.sroa.0.017.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i3.i, %bb6.i.i.i.i.i ], [ %_6.val, %bb20.preheader.i.i.i.i.i ]
  %iter.sroa.7.016.i.i.i.i.i = phi i64 [ %_8.0.i.i.i.i.i.i, %bb6.i.i.i.i.i ], [ 0, %bb20.preheader.i.i.i.i.i ]
  %19 = add nsw i64 %iter.sroa.10.018.i.i.i.i.i, -1
  %_6.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.0.017.i.i.i.i.i, %_39.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i, label %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit, label %bb5.i.i.i.i.i

bb5.i.i.i.i.i:                                    ; preds = %bb20.i.i.i.i.i
; invoke <proc_macro2::TokenTree as core::clone::Clone>::clone
  invoke fastcc void @_RNvXsK_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef align 8 captures(none) dereferenceable(32) %val.i.i.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %iter.sroa.0.017.i.i.i.i.i)
          to label %bb6.i.i.i.i.i unwind label %bb7.i.i.i.i.i, !noalias !544

bb6.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i.i
  %_8.0.i.i.i.i.i.i = add nuw nsw i64 %iter.sroa.7.016.i.i.i.i.i, 1
  %_16.i.i.i.i.i.i3.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i.i.i.i, i64 32
  %self5.i.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<proc_macro2::TokenTree>", ptr %14, i64 %iter.sroa.7.016.i.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %self5.i.i.i.i.i, ptr noundef nonnull align 8 dereferenceable(32) %val.i.i.i.i.i, i64 32, i1 false), !noalias !544
  %20 = icmp eq i64 %19, 0
  br i1 %20, label %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit, label %bb20.i.i.i.i.i

terminate.i.i.i.i.i:                              ; preds = %bb7.i.i.i.i.i
  %21 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !544
  unreachable

bb7.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i.i
  %lpad.loopexit.i.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  store i64 %iter.sroa.7.016.i.i.i.i.i, ptr %18, align 8, !noalias !537
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(24) %vec.i.i.i.i.i) #22
          to label %bb5.i unwind label %terminate.i.i.i.i.i, !noalias !544

common.resume:                                    ; preds = %bb5.i, %cleanup
  %common.resume.op = phi { ptr, i32 } [ %32, %cleanup ], [ %eh.lpad-body4.i, %bb5.i ]
  resume { ptr, i32 } %common.resume.op

bb5.i:                                            ; preds = %bb7.i.i.i.i.i, %cleanup.body.thread5.i
  %eh.lpad-body4.i = phi { ptr, i32 } [ %16, %cleanup.body.thread5.i ], [ %lpad.loopexit.i.i.i.i.i, %bb7.i.i.i.i.i ]
; invoke core::ptr::drop_in_place::<alloc::rc::UniqueRcUninit<alloc::vec::Vec<proc_macro2::TokenTree>, alloc::alloc::Global>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc14UniqueRcUninitINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBL_5alloc6GlobalEECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %in_progress.i) #22
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %bb5.i
  %22 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit: ; preds = %bb20.i.i.i.i.i, %bb6.i.i.i.i.i, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i.i
  %23 = phi ptr [ %13, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i.i ], [ %18, %bb6.i.i.i.i.i ], [ %18, %bb20.i.i.i.i.i ]
  %_19.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i.i.i.i.i.i.i, i64 16
  store i64 %_6.val4, ptr %23, align 8, !noalias !537
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %src1.i.i.i, ptr noundef nonnull align 8 dereferenceable(24) %vec.i.i.i.i.i, i64 24, i1 false), !noalias !545
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vec.i.i.i.i.i), !noalias !537
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %val.i.i.i.i.i), !noalias !534
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_19.i, ptr noundef nonnull align 8 dereferenceable(24) %src1.i.i.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %src1.i.i.i)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %in_progress.i)
  %_8.i.i = load i64, ptr %_22, align 8, !noalias !546, !noundef !9
  %val.i.i = add i64 %_8.i.i, -1
  store i64 %val.i.i, ptr %_22, align 8, !noalias !546
  %24 = icmp eq i64 %val.i.i, 0
  br i1 %24, label %bb1.i.i, label %bb13.sink.split

bb1.i.i:                                          ; preds = %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(8) %this) #25
          to label %bb13.sink.split unwind label %cleanup

bb7:                                              ; preds = %bb6
; call alloc::rc::rc_inner_layout_for_value_layout
  %25 = tail call { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef 8, i64 noundef 24), !noalias !551
; call alloc::rc::rc_inner_layout_for_value_layout
  %26 = tail call { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef 8, i64 noundef 24), !noalias !551
  %layout.0.i.i.i5 = extractvalue { i64, i64 } %26, 0
  %layout.1.i.i.i6 = extractvalue { i64, i64 } %26, 1
  %27 = icmp eq i64 %layout.1.i.i.i6, 0
  br i1 %27, label %bb2.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb2.i.i.i.i.i.i:                                  ; preds = %bb7
  %_16.i.i.i.i.i.i = inttoptr i64 %layout.0.i.i.i5 to ptr
  br label %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb7
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #21, !noalias !551
; call __rustc::__rust_alloc
  %28 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %layout.1.i.i.i6, i64 noundef range(i64 1, -9223372036854775807) %layout.0.i.i.i5) #21, !noalias !551
  br label %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i

_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i: ; preds = %bb4.i.i.i.i.i.i, %bb2.i.i.i.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb2.i.i.i.i.i.i ], [ %28, %bb4.i.i.i.i.i.i ]
  %29 = icmp eq ptr %_0.sroa.0.0.i.i.i.i.i.i, null
  br i1 %29, label %bb9.i.i, label %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE7into_rcCs1SHOYL7dTh6_5quote.exit

bb9.i.i:                                          ; preds = %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i
  %layout.1.i.i = extractvalue { i64, i64 } %25, 1
  %layout.0.i.i = extractvalue { i64, i64 } %25, 0
; call alloc::alloc::handle_alloc_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef %layout.0.i.i, i64 noundef %layout.1.i.i) #26, !noalias !551
  unreachable

_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE7into_rcCs1SHOYL7dTh6_5quote.exit: ; preds = %_RNCNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB8_14UniqueRcUninitINtNtBa_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtBa_5alloc6GlobalE3new0Cs1SHOYL7dTh6_5quote.exit.i.i.i
  store <2 x i64> splat (i64 1), ptr %_0.sroa.0.0.i.i.i.i.i.i, align 8, !noalias !551
  %self = getelementptr inbounds nuw i8, ptr %_22, i64 16
  %dst = getelementptr inbounds nuw i8, ptr %_0.sroa.0.0.i.i.i.i.i.i, i64 16
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(24) %dst, ptr noundef nonnull align 1 dereferenceable(24) %self, i64 24, i1 false)
  %30 = load <2 x i64>, ptr %_22, align 8
  %31 = add <2 x i64> %30, splat (i64 -1)
  store <2 x i64> %31, ptr %_22, align 8
  br label %bb13.sink.split

bb13.sink.split:                                  ; preds = %bb1.i.i, %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit, %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE7into_rcCs1SHOYL7dTh6_5quote.exit
  %_0.sroa.0.0.i.i.i.i.i.i.sink = phi ptr [ %_0.sroa.0.0.i.i.i.i.i.i, %_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE7into_rcCs1SHOYL7dTh6_5quote.exit ], [ %_0.sroa.0.0.i.i.i.i.i.i.i, %_RNvMsd_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE17clone_from_ref_inCs1SHOYL7dTh6_5quote.exit ], [ %_0.sroa.0.0.i.i.i.i.i.i.i, %bb1.i.i ]
  store ptr %_0.sroa.0.0.i.i.i.i.i.i.sink, ptr %this, align 8
  br label %bb13

bb13:                                             ; preds = %bb13.sink.split, %bb6
  %_77 = phi ptr [ %_22, %bb6 ], [ %_0.sroa.0.0.i.i.i.i.i.i.sink, %bb13.sink.split ]
  %_0 = getelementptr inbounds nuw i8, ptr %_77, i64 16
  ret ptr %_0

cleanup:                                          ; preds = %bb1.i.i
  %32 = landingpad { ptr, i32 }
          cleanup
  store ptr %_0.sroa.0.0.i.i.i.i.i.i.i, ptr %this, align 8
  br label %common.resume
}

; quote::spanned::join_spans
; Function Attrs: uwtable
define noundef i32 @_RNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans(ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32) %tokens) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_9.i.i = alloca [32 x i8], align 8
  %_5.i = alloca [32 x i8], align 8
  %_20 = alloca [40 x i8], align 8
  %_16 = alloca [32 x i8], align 8
  %_10 = alloca [32 x i8], align 8
  %first = alloca [4 x i8], align 4
  %iter = alloca [40 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %iter)
; call <proc_macro2::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
  call void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro212token_streamNtB7_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr noalias noundef nonnull sret([40 x i8]) align 8 captures(none) dereferenceable(40) %iter, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %tokens)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10)
; invoke <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_10, ptr noalias noundef nonnull align 8 dereferenceable(40) %iter)
          to label %bb8 unwind label %bb6

bb8:                                              ; preds = %start
  %0 = load i32, ptr %_10, align 8, !range !502, !noundef !9
  %.not = icmp eq i32 %0, 4
  br i1 %.not, label %bb10, label %bb11

bb11:                                             ; preds = %bb8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_16)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_16, ptr noundef nonnull align 8 dereferenceable(32) %_10, i64 32, i1 false)
  %1 = load i32, ptr %_16, align 8, !range !132, !alias.scope !554, !noundef !9
  switch i32 %1, label %default.unreachable [
    i32 0, label %bb5.i.i
    i32 1, label %bb4.i.i
    i32 2, label %bb3.i.i
    i32 3, label %bb2.i.i
  ]

default.unreachable:                              ; preds = %bb3.i, %bb3.peel.i, %bb11
  unreachable

bb5.i.i:                                          ; preds = %bb11
  %2 = getelementptr inbounds nuw i8, ptr %_16, i64 8
  %3 = load i32, ptr %2, align 8, !range !95, !alias.scope !554, !noundef !9
  %4 = trunc nuw i32 %3 to i1
  %5 = getelementptr inbounds nuw i8, ptr %_16, i64 20
  %_18.i.i = load i32, ptr %5, align 4, !range !559, !alias.scope !554
  %_15.sroa.0.0.i.i = select i1 %4, i32 0, i32 %_18.i.i
  br label %bb1.i

bb4.i.i:                                          ; preds = %bb11
  %6 = getelementptr inbounds nuw i8, ptr %_16, i64 24
  %7 = load i8, ptr %6, align 8, !range !180, !alias.scope !554, !noundef !9
  %.not1.i.i = icmp eq i8 %7, 2
  %8 = getelementptr inbounds nuw i8, ptr %_16, i64 12
  %_14.i.i = load i32, ptr %8, align 4, !range !559, !alias.scope !554
  %_11.sroa.0.0.i.i = select i1 %.not1.i.i, i32 %_14.i.i, i32 0
  br label %bb1.i

bb3.i.i:                                          ; preds = %bb11
  %9 = getelementptr inbounds nuw i8, ptr %_16, i64 8
  %10 = load i32, ptr %9, align 8, !alias.scope !554, !noundef !9
  br label %bb1.i

bb2.i.i:                                          ; preds = %bb11
  %11 = getelementptr inbounds nuw i8, ptr %_16, i64 8
  %12 = load i64, ptr %11, align 8, !range !60, !alias.scope !554, !noundef !9
  %.not.i.i = icmp eq i64 %12, -9223372036854775808
  %13 = getelementptr inbounds nuw i8, ptr %_16, i64 20
  %_10.i.i = load i32, ptr %13, align 4, !range !559, !alias.scope !554
  %_7.sroa.0.0.i.i = select i1 %.not.i.i, i32 %_10.i.i, i32 0
  br label %bb1.i

bb1.i:                                            ; preds = %bb2.i.i, %bb3.i.i, %bb4.i.i, %bb5.i.i
  %_0.sroa.0.0.i.i = phi i32 [ %_15.sroa.0.0.i.i, %bb5.i.i ], [ %_11.sroa.0.0.i.i, %bb4.i.i ], [ %10, %bb3.i.i ], [ %_7.sroa.0.0.i.i, %bb2.i.i ]
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_16)
          to label %bb12 unwind label %bb6

bb10:                                             ; preds = %bb8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10)
; invoke <proc_macro2::Span>::call_site
  %14 = invoke noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
          to label %bb2 unwind label %bb6

bb2:                                              ; preds = %bb10
; call core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(40) %iter)
  br label %bb4

bb4:                                              ; preds = %bb5.i, %bb5.i.thread, %bb2
  %_0.sroa.0.0 = phi i32 [ %14, %bb2 ], [ %spec.select, %bb5.i ], [ %_0.sroa.0.0.i.i, %bb5.i.thread ]
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %iter)
  ret i32 %_0.sroa.0.0

bb12:                                             ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_16)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10)
  store i32 %_0.sroa.0.0.i.i, ptr %first, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %_20, ptr noundef nonnull align 8 dereferenceable(40) %iter, i64 40, i1 false)
  %15 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 8
  %16 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 20
  %17 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 24
  %18 = getelementptr inbounds nuw i8, ptr %_9.i.i, i64 12
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5.i), !noalias !560
; invoke <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(40) %_20)
          to label %bb2.peel.i unwind label %cleanup.loopexit.split-lp.i

bb2.peel.i:                                       ; preds = %bb12
  %19 = load i32, ptr %_5.i, align 8, !range !502, !noalias !560, !noundef !9
  %.not.peel.i.not = icmp eq i32 %19, 4
  br i1 %.not.peel.i.not, label %bb5.i.thread, label %bb3.peel.i

bb5.i.thread:                                     ; preds = %bb2.peel.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5.i), !noalias !560
; call core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(40) %_20)
  br label %bb4

bb3.peel.i:                                       ; preds = %bb2.peel.i
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i), !noalias !563
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i, ptr noundef nonnull align 8 dereferenceable(32) %_5.i, i64 32, i1 false), !noalias !560
  %20 = load i32, ptr %_9.i.i, align 8, !range !132, !alias.scope !566, !noalias !563, !noundef !9
  switch i32 %20, label %default.unreachable [
    i32 0, label %bb5.i.i.i.peel.i
    i32 1, label %bb4.i.i.i.peel.i
    i32 2, label %bb3.i.i.i.peel.i
    i32 3, label %bb2.i.i.i.peel.i
  ]

bb2.i.i.i.peel.i:                                 ; preds = %bb3.peel.i
  %21 = load i64, ptr %15, align 8, !range !60, !alias.scope !566, !noalias !563, !noundef !9
  %.not.i.i.i.peel.i = icmp eq i64 %21, -9223372036854775808
  %_10.i.i.i.peel.i = load i32, ptr %16, align 4, !range !559, !alias.scope !566, !noalias !563
  %_7.sroa.0.0.i.i.i.peel.i = select i1 %.not.i.i.i.peel.i, i32 %_10.i.i.i.peel.i, i32 0
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i

bb3.i.i.i.peel.i:                                 ; preds = %bb3.peel.i
  %22 = load i32, ptr %15, align 8, !alias.scope !566, !noalias !563, !noundef !9
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i

bb4.i.i.i.peel.i:                                 ; preds = %bb3.peel.i
  %23 = load i8, ptr %17, align 8, !range !180, !alias.scope !566, !noalias !563, !noundef !9
  %.not1.i.i.i.peel.i = icmp eq i8 %23, 2
  %_14.i.i.i.peel.i = load i32, ptr %18, align 4, !range !559, !alias.scope !566, !noalias !563
  %_11.sroa.0.0.i.i.i.peel.i = select i1 %.not1.i.i.i.peel.i, i32 %_14.i.i.i.peel.i, i32 0
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i

bb5.i.i.i.peel.i:                                 ; preds = %bb3.peel.i
  %24 = load i32, ptr %15, align 8, !range !95, !alias.scope !566, !noalias !563, !noundef !9
  %25 = trunc nuw i32 %24 to i1
  %_18.i.i.i.peel.i = load i32, ptr %16, align 4, !range !559, !alias.scope !566, !noalias !563
  %_15.sroa.0.0.i.i.i.peel.i = select i1 %25, i32 0, i32 %_18.i.i.i.peel.i
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i

_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i: ; preds = %bb5.i.i.i.peel.i, %bb4.i.i.i.peel.i, %bb3.i.i.i.peel.i, %bb2.i.i.i.peel.i
  %_0.sroa.0.0.i.i.i.peel.i = phi i32 [ %_15.sroa.0.0.i.i.i.peel.i, %bb5.i.i.i.peel.i ], [ %_11.sroa.0.0.i.i.i.peel.i, %bb4.i.i.i.peel.i ], [ %22, %bb3.i.i.i.peel.i ], [ %_7.sroa.0.0.i.i.i.peel.i, %bb2.i.i.i.peel.i ]
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_9.i.i)
          to label %bb1.i3 unwind label %cleanup.loopexit.split-lp.i

bb1.i3:                                           ; preds = %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i, %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i
  %accum.sroa.6.0.i = phi i32 [ %_0.sroa.0.0.i.i.i.i, %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i ], [ %_0.sroa.0.0.i.i.i.peel.i, %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9.i.i), !noalias !563
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5.i), !noalias !560
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5.i), !noalias !560
; invoke <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_5.i, ptr noalias noundef nonnull align 8 dereferenceable(40) %_20)
          to label %bb2.i unwind label %cleanup.loopexit.i

cleanup.loopexit.i:                               ; preds = %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i, %bb1.i3
  %lpad.loopexit.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.i:                      ; preds = %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.peel.i, %bb12
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.split-lp.i, %cleanup.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit.i, %cleanup.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.i ]
; invoke core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(40) %_20) #22
          to label %bb5 unwind label %terminate.i

bb2.i:                                            ; preds = %bb1.i3
  %26 = load i32, ptr %_5.i, align 8, !range !502, !noalias !560, !noundef !9
  %.not.i = icmp eq i32 %26, 4
  br i1 %.not.i, label %bb5.i, label %bb3.i

bb3.i:                                            ; preds = %bb2.i
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9.i.i), !noalias !563
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_9.i.i, ptr noundef nonnull align 8 dereferenceable(32) %_5.i, i64 32, i1 false), !noalias !560
  %27 = load i32, ptr %_9.i.i, align 8, !range !132, !alias.scope !566, !noalias !563, !noundef !9
  switch i32 %27, label %default.unreachable [
    i32 0, label %bb5.i.i.i.i
    i32 1, label %bb4.i.i.i.i
    i32 2, label %bb3.i.i.i.i
    i32 3, label %bb2.i.i.i.i
  ]

bb5.i.i.i.i:                                      ; preds = %bb3.i
  %28 = load i32, ptr %15, align 8, !range !95, !alias.scope !566, !noalias !563, !noundef !9
  %29 = trunc nuw i32 %28 to i1
  %_18.i.i.i.i = load i32, ptr %16, align 4, !range !559, !alias.scope !566, !noalias !563
  %_15.sroa.0.0.i.i.i.i = select i1 %29, i32 0, i32 %_18.i.i.i.i
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i

bb4.i.i.i.i:                                      ; preds = %bb3.i
  %30 = load i8, ptr %17, align 8, !range !180, !alias.scope !566, !noalias !563, !noundef !9
  %.not1.i.i.i.i = icmp eq i8 %30, 2
  %_14.i.i.i.i = load i32, ptr %18, align 4, !range !559, !alias.scope !566, !noalias !563
  %_11.sroa.0.0.i.i.i.i = select i1 %.not1.i.i.i.i, i32 %_14.i.i.i.i, i32 0
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i

bb3.i.i.i.i:                                      ; preds = %bb3.i
  %31 = load i32, ptr %15, align 8, !alias.scope !566, !noalias !563, !noundef !9
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i

bb2.i.i.i.i:                                      ; preds = %bb3.i
  %32 = load i64, ptr %15, align 8, !range !60, !alias.scope !566, !noalias !563, !noundef !9
  %.not.i.i.i.i = icmp eq i64 %32, -9223372036854775808
  %_10.i.i.i.i = load i32, ptr %16, align 4, !range !559, !alias.scope !566, !noalias !563
  %_7.sroa.0.0.i.i.i.i = select i1 %.not.i.i.i.i, i32 %_10.i.i.i.i, i32 0
  br label %_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i

_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_.exit.i.i: ; preds = %bb2.i.i.i.i, %bb3.i.i.i.i, %bb4.i.i.i.i, %bb5.i.i.i.i
  %_0.sroa.0.0.i.i.i.i = phi i32 [ %_15.sroa.0.0.i.i.i.i, %bb5.i.i.i.i ], [ %_11.sroa.0.0.i.i.i.i, %bb4.i.i.i.i ], [ %31, %bb3.i.i.i.i ], [ %_7.sroa.0.0.i.i.i.i, %bb2.i.i.i.i ]
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %_9.i.i)
          to label %bb1.i3 unwind label %cleanup.loopexit.i, !llvm.loop !571

bb5.i:                                            ; preds = %bb2.i
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5.i), !noalias !560
; call core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(40) %_20)
; call <proc_macro2::Span>::join
  %33 = call { i32, i32 } @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span4join(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %first, i32 noundef %accum.sroa.6.0.i)
  %34 = extractvalue { i32, i32 } %33, 0
  %35 = trunc i32 %34 to i1
  %36 = extractvalue { i32, i32 } %33, 1
  %spec.select = select i1 %35, i32 %36, i32 %_0.sroa.0.0.i.i
  br label %bb4

terminate.i:                                      ; preds = %cleanup.i
  %37 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb5:                                              ; preds = %bb6, %cleanup.i
  %eh.lpad-body7 = phi { ptr, i32 } [ %lpad.phi.i, %cleanup.i ], [ %lpad.thr_comm, %bb6 ]
  resume { ptr, i32 } %eh.lpad-body7

bb6:                                              ; preds = %bb10, %start, %bb1.i
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull readonly align 8 dereferenceable(40) %iter)
          to label %bb5 unwind label %terminate

terminate:                                        ; preds = %bb6
  %38 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable
}

; quote::__private::push_caret
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_caret(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !573
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 94, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !573
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !577
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !573
  ret void
}

; quote::__private::push_colon
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_colon(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !578
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 58, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !578
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !582
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !578
  ret void
}

; quote::__private::push_comma
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_comma(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !583
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 44, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !583
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !587
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !583
  ret void
}

; quote::__private::push_eq_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_eq_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !588
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 61, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !588
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !592
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !588
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !593
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !593
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !597
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !593
  ret void
}

; quote::__private::push_group
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_group(ptr noalias noundef align 8 dereferenceable(32) %tokens, i8 noundef range(i8 0, 4) %delimiter, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32) %inner) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_5 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
; call <proc_macro2::Group>::new
  call void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5, i8 noundef %delimiter, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %inner)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !598
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_5, i64 24, i1 false), !noalias !602
  store i32 0, ptr %_3.i, align 8, !noalias !598
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !598
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !603
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !598
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  ret void
}

; quote::__private::push_ident
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_ident(ptr noalias noundef align 8 dereferenceable(32) %tokens, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #0 {
start:
  %_4.sroa.4.i.i = alloca [28 x i8], align 4
  %_3.i.i = alloca [32 x i8], align 8
  %_5.i = alloca [24 x i8], align 8
; call <proc_macro2::Span>::call_site
  %span = tail call noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5.i), !noalias !604
  %_4.i.i.i = icmp samesign ugt i64 %s.1, 1
  br i1 %_4.i.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i.i, label %bb3.i.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i.i: ; preds = %start
  %0 = tail call i32 @memcmp(ptr noundef nonnull dereferenceable(2) @alloc_26741d7be2999f870b5ef1ed52a52387, ptr noundef nonnull readonly align 1 dereferenceable(2) %s.0, i64 2), !noalias !608
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %bb2.i.i, label %bb3.i.i

bb3.i.i:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i.i, %start
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_45cdb82e1f6d7e9f36108024db2c250f), !noalias !611
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned.exit

bb2.i.i:                                          ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i.i
  %_13.i.i = add i64 %s.1, -2
  %_15.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 2
; call <proc_macro2::Ident>::new_raw
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident7new_raw(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_15.i.i, i64 noundef %_13.i.i, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ab7e69cb0be14b6daa10eaf0accb519), !noalias !611
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned.exit

_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned.exit: ; preds = %bb3.i.i, %bb2.i.i
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i.i), !noalias !612
  %_4.sroa.4.8.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_5.i, i64 24, i1 false), !noalias !616
  store i32 1, ptr %_3.i.i, align 8, !noalias !612
  %_5.sroa.4.0._3.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i.i, i64 28, i1 false), !noalias !612
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i.i), !noalias !617
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i.i), !noalias !612
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5.i), !noalias !604
  ret void
}

; quote::__private::push_or_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_or_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !618
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 124, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !618
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !622
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !618
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !623
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !623
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !627
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !623
  ret void
}

; quote::__private::push_or_or
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_or_or(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !628
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 124, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !628
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !632
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !628
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !633
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 124, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !633
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !637
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !633
  ret void
}

; quote::__private::push_pound
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private10push_pound(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !638
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 35, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !638
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !642
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !638
  ret void
}

; quote::__private::push_add_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_add_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !643
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 43, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !643
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !647
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !643
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !648
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !648
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !652
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !648
  ret void
}

; quote::__private::push_and_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_and_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !653
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 38, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !653
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !657
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !653
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !658
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !658
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !662
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !658
  ret void
}

; quote::__private::push_colon2
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_colon2(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !663
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 58, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !663
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !667
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !663
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !668
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 58, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !668
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !672
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !668
  ret void
}

; quote::__private::push_div_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_div_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !673
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 47, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !673
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !677
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !673
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !678
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !678
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !682
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !678
  ret void
}

; quote::__private::push_larrow
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_larrow(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !683
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 60, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !683
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !687
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !683
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !688
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 45, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !688
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !692
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !688
  ret void
}

; quote::__private::push_mul_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_mul_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !693
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 42, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !693
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !697
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !693
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !698
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !698
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !702
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !698
  ret void
}

; quote::__private::push_rarrow
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_rarrow(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !703
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 45, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !703
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !707
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !703
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !708
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 62, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !708
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !712
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !708
  ret void
}

; quote::__private::push_rem_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_rem_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !713
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 37, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !713
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !717
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !713
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !718
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !718
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !722
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !718
  ret void
}

; quote::__private::push_shl_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_shl_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !723
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 60, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !723
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !727
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !723
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !728
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 60, i1 noundef zeroext true)
  store i32 2, ptr %_3.i1, align 8, !noalias !728
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !732
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !728
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !733
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i3, align 8, !noalias !733
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !737
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !733
  ret void
}

; quote::__private::push_shr_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_shr_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !738
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 62, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !738
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !742
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !738
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !743
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 62, i1 noundef zeroext true)
  store i32 2, ptr %_3.i1, align 8, !noalias !743
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !747
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !743
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !748
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i3, align 8, !noalias !748
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !752
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !748
  ret void
}

; quote::__private::push_sub_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private11push_sub_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !753
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 45, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !753
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !757
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !753
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !758
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !758
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !762
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !758
  ret void
}

; quote::__private::push_and_and
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private12push_and_and(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !763
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 38, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !763
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !767
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !763
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !768
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 38, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !768
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !772
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !768
  ret void
}

; quote::__private::parse_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private13parse_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [32 x i8], align 8
  %e.i = alloca [1 x i8], align 1
  %_13 = alloca [32 x i8], align 8
  %token = alloca [32 x i8], align 8
  %_8 = alloca [32 x i8], align 8
  %iter = alloca [40 x i8], align 8
  %_5 = alloca [32 x i8], align 8
  %s = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5)
; call <proc_macro2::TokenStream as core::str::traits::FromStr>::from_str
  call void @_RNvXs0_Cs8M6BBVNvC7a_11proc_macro2NtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3str6traits7FromStr8from_str(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !773)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !776)
  %0 = load i64, ptr %_5, align 8, !range !233, !alias.scope !776, !noalias !778, !noundef !9
  %1 = icmp eq i64 %0, -9223372036854775807
  br i1 %1, label %bb2.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote.exit, !prof !780

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %e.i), !noalias !781
  %2 = getelementptr inbounds nuw i8, ptr %_5, i64 8
  %3 = load i8, ptr %2, align 8, !range !180, !alias.scope !776, !noalias !778, !noundef !9
  store i8 %3, ptr %e.i, align 1, !noalias !781
; call core::result::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_34ccc9a2f61247d7615f0501afc88ea6, i64 noundef 20, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_c2b3f6463b0a8a8095dd8eea675c8d69) #26, !noalias !782
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote.exit: ; preds = %start
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %s, ptr noundef nonnull readonly align 8 dereferenceable(32) %_5, i64 32, i1 false), !alias.scope !782, !noalias !783
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5)
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %iter)
; call <proc_macro2::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
  call void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro212token_streamNtB7_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr noalias noundef nonnull sret([40 x i8]) align 8 captures(none) dereferenceable(40) %iter, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %s)
  br label %bb3

bb3:                                              ; preds = %bb9, %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote.exit
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_8)
; invoke <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_8, ptr noalias noundef nonnull align 8 dereferenceable(40) %iter)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %bb8, %bb6, %bb3
  %4 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(40) %iter) #22
          to label %bb12 unwind label %terminate

bb4:                                              ; preds = %bb3
  %5 = load i32, ptr %_8, align 8, !range !502, !noundef !9
  %.not = icmp eq i32 %5, 4
  br i1 %.not, label %bb7, label %bb6

bb6:                                              ; preds = %bb4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %token, ptr noundef nonnull align 8 dereferenceable(32) %_8, i64 32, i1 false)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13)
; invoke quote::__private::respan_token_tree
  invoke fastcc void @_RNvNtCs1SHOYL7dTh6_5quote9___private17respan_token_tree(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_13, ptr noalias noundef align 8 captures(address) dereferenceable(32) %token, i32 noundef %span)
          to label %bb8 unwind label %cleanup

bb7:                                              ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_8)
; call core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(40) %iter)
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %iter)
  ret void

bb8:                                              ; preds = %bb6
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !784
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_3.i, ptr noundef nonnull readonly align 8 dereferenceable(32) %_13, i64 32, i1 false), !noalias !788
; invoke <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i)
          to label %bb9 unwind label %cleanup

bb9:                                              ; preds = %bb8
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !784
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_8)
  br label %bb3

terminate:                                        ; preds = %cleanup
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb12:                                             ; preds = %cleanup
  resume { ptr, i32 } %4
}

; quote::__private::push_caret_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private13push_caret_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !789
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 94, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !789
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !793
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !789
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !794
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !794
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !798
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !794
  ret void
}

; quote::__private::push_lifetime
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private13push_lifetime(ptr noalias noundef align 8 dereferenceable(32) %tokens, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %lifetime.0, i64 noundef %lifetime.1) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_8 = alloca [24 x i8], align 8
  %_7.sroa.4 = alloca [28 x i8], align 4
  %_4.sroa.4.0._3.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !799
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_4.sroa.4.0._3.i.sroa_idx, i32 noundef 39, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !803
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !804
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !799
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_7.sroa.4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_8)
  %_8.i = icmp ult i64 %lifetime.1, 2
  br i1 %_8.i, label %bb6.i, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit

bb6.i:                                            ; preds = %start
  %0 = icmp eq i64 %lifetime.1, 1
  br i1 %0, label %bb9, label %bb8, !prof !14

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit: ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %lifetime.0, i64 1
  %self1.i = load i8, ptr %1, align 1, !alias.scope !805, !noundef !9
  %2 = icmp sgt i8 %self1.i, -65
  br i1 %2, label %bb9, label %bb8, !prof !14

bb9:                                              ; preds = %bb6.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit
  %data.i = getelementptr inbounds nuw i8, ptr %lifetime.0, i64 1
  %new_len.i = add i64 %lifetime.1, -1
; call <proc_macro2::Span>::call_site
  %_10 = call noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_8, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i, i64 noundef %new_len.i, i32 noundef %_10, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_a215eae7f8dd0221659ca93936e57bd6)
  %_7.sroa.4.8.sroa_idx = getelementptr inbounds nuw i8, ptr %_7.sroa.4, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_7.sroa.4.8.sroa_idx, ptr noundef nonnull align 8 dereferenceable(24) %_8, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_8)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !808
  store i32 1, ptr %_3.i2, align 8, !noalias !812
  %_7.sroa.4.0._3.i2.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_7.sroa.4.0._3.i2.sroa_idx, ptr noundef nonnull align 4 dereferenceable(28) %_7.sroa.4, i64 28, i1 false), !noalias !812
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !813
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !808
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_7.sroa.4)
  ret void

bb8:                                              ; preds = %bb6.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit
; call core::str::slice_error_fail
  call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %lifetime.0, i64 noundef %lifetime.1, i64 noundef 1, i64 noundef %lifetime.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_bc590f7bbdf9d5cf9c9fc92d5b74ebb4) #24
  unreachable
}

; quote::__private::push_question
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private13push_question(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !814
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 63, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !814
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !818
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !814
  ret void
}

; quote::__private::push_fat_arrow
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private14push_fat_arrow(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !819
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 61, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !819
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !823
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !819
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !824
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 62, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !824
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !828
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !824
  ret void
}

; quote::__private::push_at_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_at_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 64, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !829
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !833
  store i32 2, ptr %_3.i, align 8, !noalias !829
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !834
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !829
  ret void
}

; quote::__private::push_dot_dot_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_dot_dot_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !835
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 46, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !835
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !839
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !835
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !840
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 46, i1 noundef zeroext true)
  store i32 2, ptr %_3.i1, align 8, !noalias !840
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !844
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !840
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !845
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i3, align 8, !noalias !845
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !849
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !845
  ret void
}

; quote::__private::push_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 61, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !850
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !854
  store i32 2, ptr %_3.i, align 8, !noalias !850
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !855
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !850
  ret void
}

; quote::__private::push_ge_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_ge_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 62, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !856
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !860
  store i32 2, ptr %_3.i, align 8, !noalias !856
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !861
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !856
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !862
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !866
  store i32 2, ptr %_3.i2, align 8, !noalias !862
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !867
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !862
  ret void
}

; quote::__private::push_gt_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_gt_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 62, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !868
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !872
  store i32 2, ptr %_3.i, align 8, !noalias !868
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !873
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !868
  ret void
}

; quote::__private::push_le_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_le_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 60, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !874
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !878
  store i32 2, ptr %_3.i, align 8, !noalias !874
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !879
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !874
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !880
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !884
  store i32 2, ptr %_3.i2, align 8, !noalias !880
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !885
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !880
  ret void
}

; quote::__private::push_lt_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_lt_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 60, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !886
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !890
  store i32 2, ptr %_3.i, align 8, !noalias !886
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !891
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !886
  ret void
}

; quote::__private::push_ne_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_ne_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 33, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !892
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !896
  store i32 2, ptr %_3.i, align 8, !noalias !892
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !897
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !892
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !898
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !902
  store i32 2, ptr %_3.i2, align 8, !noalias !898
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !903
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !898
  ret void
}

; quote::__private::push_or_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_or_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 124, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !904
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !908
  store i32 2, ptr %_3.i, align 8, !noalias !904
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !909
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !904
  ret void
}

; quote::__private::push_underscore
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private15push_underscore(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i.i = alloca [28 x i8], align 4
  %_3.i.i = alloca [32 x i8], align 8
  %_4.i = alloca [24 x i8], align 8
; call <proc_macro2::Span>::call_site
  %_3 = tail call noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i), !noalias !910
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_27cca3636828088e60ce450d2eca2060, i64 noundef 1, i32 noundef %_3, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53c7a51dd4d2107fe69463bcafa37cb4), !noalias !910
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i.i), !noalias !913
  %_4.sroa.4.8.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4.i, i64 24, i1 false), !noalias !917
  store i32 1, ptr %_3.i.i, align 8, !noalias !913
  %_5.sroa.4.0._3.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i.i, i64 28, i1 false), !noalias !913
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i.i), !noalias !918
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i.i), !noalias !913
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i), !noalias !910
  ret void
}

; quote::__private::push_add_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_add_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 43, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !919
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !923
  store i32 2, ptr %_3.i, align 8, !noalias !919
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !924
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !919
  ret void
}

; quote::__private::push_and_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_and_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 38, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !925
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !929
  store i32 2, ptr %_3.i, align 8, !noalias !925
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !930
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !925
  ret void
}

; quote::__private::push_div_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_div_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 47, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !931
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !935
  store i32 2, ptr %_3.i, align 8, !noalias !931
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !936
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !931
  ret void
}

; quote::__private::push_dot_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_dot_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 46, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !937
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !941
  store i32 2, ptr %_3.i, align 8, !noalias !937
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !942
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !937
  ret void
}

; quote::__private::push_rem_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_rem_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 37, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !943
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !947
  store i32 2, ptr %_3.i, align 8, !noalias !943
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !948
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !943
  ret void
}

; quote::__private::push_shl_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_shl_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 60, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !949
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !953
  store i32 2, ptr %_3.i, align 8, !noalias !949
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !954
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !949
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 60, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !955
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !959
  store i32 2, ptr %_3.i2, align 8, !noalias !955
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !960
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !955
  ret void
}

; quote::__private::push_shr_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_shr_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 62, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !961
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !965
  store i32 2, ptr %_3.i, align 8, !noalias !961
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !966
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !961
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 62, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !967
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !971
  store i32 2, ptr %_3.i2, align 8, !noalias !967
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !972
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !967
  ret void
}

; quote::__private::push_sub_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private16push_sub_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 45, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !973
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !977
  store i32 2, ptr %_3.i, align 8, !noalias !973
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !978
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !973
  ret void
}

; quote::__private::push_bang_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private17push_bang_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 33, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !979
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !983
  store i32 2, ptr %_3.i, align 8, !noalias !979
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !984
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !979
  ret void
}

; quote::__private::push_dot2_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private17push_dot2_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 46, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !985
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !989
  store i32 2, ptr %_3.i, align 8, !noalias !985
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !990
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !985
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 46, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !991
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !995
  store i32 2, ptr %_3.i2, align 8, !noalias !991
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !996
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !991
  ret void
}

; quote::__private::push_dot3_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private17push_dot3_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i5 = alloca [32 x i8], align 8
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct2 = alloca [12 x i8], align 4
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 46, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !997
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1001
  store i32 2, ptr %_3.i, align 8, !noalias !997
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1002
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !997
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 46, i1 noundef zeroext true)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !1003
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1007
  store i32 2, ptr %_3.i3, align 8, !noalias !1003
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !1008
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !1003
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct2, i32 noundef 46, i1 noundef zeroext false)
  %2 = getelementptr inbounds nuw i8, ptr %punct2, i64 4
  store i32 %span, ptr %2, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i5), !noalias !1009
  %_5.sroa.4.0._3.sroa_idx.i6 = getelementptr inbounds nuw i8, ptr %_3.i5, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i6, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct2, i64 12, i1 false), !noalias !1013
  store i32 2, ptr %_3.i5, align 8, !noalias !1009
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i5), !noalias !1014
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i5), !noalias !1009
  ret void
}

; quote::__private::push_semi_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private17push_semi_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 59, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1015
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1019
  store i32 2, ptr %_3.i, align 8, !noalias !1015
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1020
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1015
  ret void
}

; quote::__private::push_star_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private17push_star_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 42, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1021
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1025
  store i32 2, ptr %_3.i, align 8, !noalias !1021
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1026
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1021
  ret void
}

; quote::__private::respan_token_tree
; Function Attrs: uwtable
define internal fastcc void @_RNvNtCs1SHOYL7dTh6_5quote9___private17respan_token_tree(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(32) %_0, ptr dead_on_return noalias noundef nonnull align 8 captures(address) dereferenceable(32) %token, i32 noundef %span) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [32 x i8], align 8
  %_21 = alloca [32 x i8], align 8
  %_19 = alloca [24 x i8], align 8
  %_18 = alloca [32 x i8], align 8
  %token1 = alloca [32 x i8], align 8
  %_12 = alloca [32 x i8], align 8
  %iter = alloca [40 x i8], align 8
  %_9 = alloca [32 x i8], align 8
  %_8 = alloca [40 x i8], align 8
  %tokens = alloca [32 x i8], align 8
  %0 = load i32, ptr %token, align 8, !range !132, !noundef !9
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %bb2, label %bb1

bb2:                                              ; preds = %start
  %g = getelementptr inbounds nuw i8, ptr %token, i64 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %tokens)
; invoke <proc_macro2::TokenStream>::new
  invoke void @_RNvMCs8M6BBVNvC7a_11proc_macro2NtB2_11TokenStream3new(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %tokens)
          to label %bb3 unwind label %cleanup

bb1:                                              ; preds = %start
; invoke <proc_macro2::TokenTree>::set_span
  invoke void @_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree8set_span(ptr noalias noundef nonnull align 8 dereferenceable(32) %token, i32 noundef %span)
          to label %bb19 unwind label %cleanup

bb21:                                             ; preds = %bb24, %bb24.thread12, %bb23, %cleanup
  %.pn.pn = phi { ptr, i32 } [ %.pn11, %bb23 ], [ %lpad.thr_comm.split-lp, %bb24 ], [ %2, %cleanup ], [ %eh.lpad-body, %bb24.thread12 ]
; invoke core::ptr::drop_in_place::<proc_macro2::TokenTree>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %token) #22
          to label %bb22 unwind label %terminate

cleanup:                                          ; preds = %bb1, %bb2
  %2 = landingpad { ptr, i32 }
          cleanup
  br label %bb21

bb3:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %_8)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_9)
; invoke <proc_macro2::Group>::stream
  invoke void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group6stream(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_9, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %g)
          to label %bb4 unwind label %bb24.thread15

bb24.thread15:                                    ; preds = %bb10, %bb4, %bb3
  %lpad.thr_comm = landingpad { ptr, i32 }
          cleanup
  br label %bb23

bb24:                                             ; preds = %bb13, %bb15
  %lpad.thr_comm.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %bb21

bb4:                                              ; preds = %bb3
; invoke <proc_macro2::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro212token_streamNtB7_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr noalias noundef nonnull sret([40 x i8]) align 8 captures(none) dereferenceable(40) %_8, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_9)
          to label %bb5 unwind label %bb24.thread15

bb5:                                              ; preds = %bb4
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_9)
  call void @llvm.lifetime.start.p0(i64 40, ptr nonnull %iter)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(40) %iter, ptr noundef nonnull align 8 dereferenceable(40) %_8, i64 40, i1 false)
  br label %bb6

bb6:                                              ; preds = %bb12, %bb5
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_12)
; invoke <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
  invoke void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(address) dereferenceable(32) %_12, ptr noalias noundef nonnull align 8 dereferenceable(40) %iter)
          to label %bb7 unwind label %cleanup3

cleanup3:                                         ; preds = %bb11, %bb9, %bb6
  %3 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(40) %iter) #22
          to label %bb23 unwind label %terminate

bb7:                                              ; preds = %bb6
  %4 = load i32, ptr %_12, align 8, !range !502, !noundef !9
  %.not = icmp eq i32 %4, 4
  br i1 %.not, label %bb10, label %bb9

bb9:                                              ; preds = %bb7
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %token1, ptr noundef nonnull align 8 dereferenceable(32) %_12, i64 32, i1 false)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_18)
; invoke quote::__private::respan_token_tree
  invoke fastcc void @_RNvNtCs1SHOYL7dTh6_5quote9___private17respan_token_tree(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_18, ptr noalias noundef align 8 captures(address) dereferenceable(32) %token1, i32 noundef %span)
          to label %bb11 unwind label %cleanup3

bb10:                                             ; preds = %bb7
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_12)
; invoke core::ptr::drop_in_place::<proc_macro2::token_stream::IntoIter>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(40) %iter)
          to label %bb13 unwind label %bb24.thread15

bb13:                                             ; preds = %bb10
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %iter)
  call void @llvm.lifetime.end.p0(i64 40, ptr nonnull %_8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_19)
  %5 = load i32, ptr %g, align 8, !range !95, !noundef !9
  %6 = trunc nuw i32 %5 to i1
  %7 = getelementptr inbounds nuw i8, ptr %token, i64 24
  %8 = load i8, ptr %7, align 8, !range !1027
  %9 = getelementptr inbounds nuw i8, ptr %token, i64 28
  %_25 = load i8, ptr %9, align 4, !range !1027
  %_20.sroa.0.0 = select i1 %6, i8 %8, i8 %_25
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_21)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_21, ptr noundef nonnull align 8 dereferenceable(32) %tokens, i64 32, i1 false)
; invoke <proc_macro2::Group>::new
  invoke void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_19, i8 noundef %_20.sroa.0.0, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_21)
          to label %bb14 unwind label %bb24

bb14:                                             ; preds = %bb13
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_21)
  %10 = load i32, ptr %g, align 8, !range !95, !alias.scope !1028, !noundef !9
  %11 = icmp eq i32 %10, 0
  br i1 %11, label %bb2.i.i, label %bb3.i.i

bb2.i.i:                                          ; preds = %bb14
  %12 = getelementptr inbounds nuw i8, ptr %token, i64 24
  %13 = load i32, ptr %12, align 8, !alias.scope !1033, !noundef !9
  %14 = icmp eq i32 %13, 0
  br i1 %14, label %bb15, label %bb2.i.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb2.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %12)
          to label %bb15 unwind label %cleanup4

bb3.i.i:                                          ; preds = %bb14
  %15 = getelementptr inbounds nuw i8, ptr %token, i64 16
; invoke <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 8 dereferenceable(16) %15)
          to label %bb4.i.i.i.i unwind label %cleanup.i.i.i.i

cleanup.i.i.i.i:                                  ; preds = %bb3.i.i
  %16 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !1040)
  call void @llvm.experimental.noalias.scope.decl(metadata !1043)
  call void @llvm.experimental.noalias.scope.decl(metadata !1046)
  %_5.i.i.i.i.i.i.i = load ptr, ptr %15, align 8, !alias.scope !1049, !nonnull !9, !noundef !9
  %_8.i.i.i.i.i.i.i = load i64, ptr %_5.i.i.i.i.i.i.i, align 8, !noalias !1054, !noundef !9
  %val.i.i.i.i.i.i.i = add i64 %_8.i.i.i.i.i.i.i, -1
  store i64 %val.i.i.i.i.i.i.i, ptr %_5.i.i.i.i.i.i.i, align 8, !noalias !1054
  %17 = icmp eq i64 %val.i.i.i.i.i.i.i, 0
  br i1 %17, label %bb1.i.i.i.i.i.i.i, label %bb24.thread12

bb1.i.i.i.i.i.i.i:                                ; preds = %cleanup.i.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %15) #25
          to label %bb24.thread12 unwind label %terminate.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb3.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1055)
  call void @llvm.experimental.noalias.scope.decl(metadata !1058)
  call void @llvm.experimental.noalias.scope.decl(metadata !1061)
  %_5.i.i.i1.i.i.i.i = load ptr, ptr %15, align 8, !alias.scope !1064, !nonnull !9, !noundef !9
  %_8.i.i.i2.i.i.i.i = load i64, ptr %_5.i.i.i1.i.i.i.i, align 8, !noalias !1065, !noundef !9
  %val.i.i.i3.i.i.i.i = add i64 %_8.i.i.i2.i.i.i.i, -1
  store i64 %val.i.i.i3.i.i.i.i, ptr %_5.i.i.i1.i.i.i.i, align 8, !noalias !1065
  %18 = icmp eq i64 %val.i.i.i3.i.i.i.i, 0
  br i1 %18, label %bb1.i.i.i4.i.i.i.i, label %bb15

bb1.i.i.i4.i.i.i.i:                               ; preds = %bb4.i.i.i.i
; invoke <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
  invoke void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef nonnull align 8 dereferenceable(16) %15) #25
          to label %bb15 unwind label %cleanup4

terminate.i.i.i.i:                                ; preds = %bb1.i.i.i.i.i.i.i
  %19 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

cleanup4:                                         ; preds = %bb1.i.i.i4.i.i.i.i, %bb2.i.i.i.i.i
  %20 = landingpad { ptr, i32 }
          cleanup
  br label %bb24.thread12

bb24.thread12:                                    ; preds = %cleanup4, %bb1.i.i.i.i.i.i.i, %cleanup.i.i.i.i
  %eh.lpad-body = phi { ptr, i32 } [ %20, %cleanup4 ], [ %16, %bb1.i.i.i.i.i.i.i ], [ %16, %cleanup.i.i.i.i ]
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %g, ptr noundef nonnull align 8 dereferenceable(24) %_19, i64 24, i1 false)
  br label %bb21

bb15:                                             ; preds = %bb4.i.i.i.i, %bb2.i.i, %bb2.i.i.i.i.i, %bb1.i.i.i4.i.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %g, ptr noundef nonnull align 8 dereferenceable(24) %_19, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_19)
; invoke <proc_macro2::Group>::set_span
  invoke void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group8set_span(ptr noalias noundef nonnull align 8 dereferenceable(24) %g, i32 noundef %span)
          to label %bb17 unwind label %bb24

bb17:                                             ; preds = %bb15
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %tokens)
  br label %bb19

bb19:                                             ; preds = %bb1, %bb17
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %token, i64 32, i1 false)
  ret void

bb11:                                             ; preds = %bb9
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1066
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_3.i, ptr noundef nonnull readonly align 8 dereferenceable(32) %_18, i64 32, i1 false), !noalias !1070
; invoke <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  invoke fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i)
          to label %bb12 unwind label %cleanup3

bb12:                                             ; preds = %bb11
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1066
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_18)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_12)
  br label %bb6

terminate:                                        ; preds = %bb21, %bb23, %cleanup3
  %21 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable

bb23:                                             ; preds = %cleanup3, %bb24.thread15
  %.pn11 = phi { ptr, i32 } [ %lpad.thr_comm, %bb24.thread15 ], [ %3, %cleanup3 ]
; invoke core::ptr::drop_in_place::<proc_macro2::TokenStream>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro211TokenStreamECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %tokens) #22
          to label %bb21 unwind label %terminate

bb22:                                             ; preds = %bb21
  resume { ptr, i32 } %.pn.pn
}

; quote::__private::push_caret_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_caret_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 94, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1071
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1075
  store i32 2, ptr %_3.i, align 8, !noalias !1071
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1076
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1071
  ret void
}

; quote::__private::push_colon_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_colon_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 58, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1077
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1081
  store i32 2, ptr %_3.i, align 8, !noalias !1077
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1082
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1077
  ret void
}

; quote::__private::push_comma_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_comma_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 44, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1083
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1087
  store i32 2, ptr %_3.i, align 8, !noalias !1083
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1088
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1083
  ret void
}

; quote::__private::push_eq_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_eq_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 61, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1089
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1093
  store i32 2, ptr %_3.i, align 8, !noalias !1089
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1094
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1089
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1095
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1099
  store i32 2, ptr %_3.i2, align 8, !noalias !1095
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1100
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1095
  ret void
}

; quote::__private::push_group_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_group_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span, i8 noundef range(i8 0, 4) %delimiter, ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32) %inner) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %g = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %g)
; call <proc_macro2::Group>::new
  call void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %g, i8 noundef %delimiter, ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %inner)
; invoke <proc_macro2::Group>::set_span
  invoke void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group8set_span(ptr noalias noundef nonnull align 8 dereferenceable(24) %g, i32 noundef %span)
          to label %bb2 unwind label %bb5

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1101
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(24) %g, i64 24, i1 false)
  store i32 0, ptr %_3.i, align 8, !noalias !1101
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1101
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1101
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %g)
  ret void

bb4:                                              ; preds = %bb5
  resume { ptr, i32 } %0

bb5:                                              ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<proc_macro2::Group>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(24) %g) #22
          to label %bb4 unwind label %terminate

terminate:                                        ; preds = %bb5
  %1 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23
  unreachable
}

; quote::__private::push_ident_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_5 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_5)
  %_4.i.i = icmp samesign ugt i64 %s.1, 1
  br i1 %_4.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i, label %bb3.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i: ; preds = %start
  %0 = tail call i32 @memcmp(ptr noundef nonnull dereferenceable(2) @alloc_26741d7be2999f870b5ef1ed52a52387, ptr noundef nonnull readonly align 1 dereferenceable(2) %s.0, i64 2), !noalias !1105
  %1 = icmp eq i32 %0, 0
  br i1 %1, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i, %start
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_45cdb82e1f6d7e9f36108024db2c250f)
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit

bb2.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i
  %_13.i = add i64 %s.1, -2
  %_15.i = getelementptr inbounds nuw i8, ptr %s.0, i64 2
; call <proc_macro2::Ident>::new_raw
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident7new_raw(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_5, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_15.i, i64 noundef %_13.i, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ab7e69cb0be14b6daa10eaf0accb519)
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit

_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit: ; preds = %bb3.i, %bb2.i
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1108
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_5, i64 24, i1 false), !noalias !1112
  store i32 1, ptr %_3.i, align 8, !noalias !1108
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1108
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1113
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1108
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_5)
  ret void
}

; quote::__private::push_or_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_or_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 124, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1114
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1118
  store i32 2, ptr %_3.i, align 8, !noalias !1114
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1119
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1114
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1120
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1124
  store i32 2, ptr %_3.i2, align 8, !noalias !1120
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1125
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1120
  ret void
}

; quote::__private::push_or_or_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_or_or_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 124, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1126
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1130
  store i32 2, ptr %_3.i, align 8, !noalias !1126
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1131
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1126
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 124, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1132
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1136
  store i32 2, ptr %_3.i2, align 8, !noalias !1132
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1137
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1132
  ret void
}

; quote::__private::push_pound_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private18push_pound_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 35, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1138
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1142
  store i32 2, ptr %_3.i, align 8, !noalias !1138
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1143
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1138
  ret void
}

; quote::__private::push_add_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_add_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 43, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1144
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1148
  store i32 2, ptr %_3.i, align 8, !noalias !1144
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1149
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1144
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1150
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1154
  store i32 2, ptr %_3.i2, align 8, !noalias !1150
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1155
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1150
  ret void
}

; quote::__private::push_and_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_and_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 38, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1156
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1160
  store i32 2, ptr %_3.i, align 8, !noalias !1156
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1161
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1156
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1162
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1166
  store i32 2, ptr %_3.i2, align 8, !noalias !1162
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1167
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1162
  ret void
}

; quote::__private::push_colon2_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_colon2_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 58, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1168
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1172
  store i32 2, ptr %_3.i, align 8, !noalias !1168
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1173
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1168
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 58, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1174
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1178
  store i32 2, ptr %_3.i2, align 8, !noalias !1174
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1179
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1174
  ret void
}

; quote::__private::push_div_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_div_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 47, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1180
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1184
  store i32 2, ptr %_3.i, align 8, !noalias !1180
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1185
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1180
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1186
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1190
  store i32 2, ptr %_3.i2, align 8, !noalias !1186
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1191
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1186
  ret void
}

; quote::__private::push_larrow_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_larrow_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 60, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1192
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1196
  store i32 2, ptr %_3.i, align 8, !noalias !1192
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1197
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1192
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 45, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1198
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1202
  store i32 2, ptr %_3.i2, align 8, !noalias !1198
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1203
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1198
  ret void
}

; quote::__private::push_mul_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_mul_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 42, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1204
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1208
  store i32 2, ptr %_3.i, align 8, !noalias !1204
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1209
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1204
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1210
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1214
  store i32 2, ptr %_3.i2, align 8, !noalias !1210
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1215
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1210
  ret void
}

; quote::__private::push_rarrow_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_rarrow_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 45, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1216
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1220
  store i32 2, ptr %_3.i, align 8, !noalias !1216
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1221
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1216
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 62, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1222
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1226
  store i32 2, ptr %_3.i2, align 8, !noalias !1222
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1227
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1222
  ret void
}

; quote::__private::push_rem_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_rem_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 37, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1228
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1232
  store i32 2, ptr %_3.i, align 8, !noalias !1228
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1233
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1228
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1234
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1238
  store i32 2, ptr %_3.i2, align 8, !noalias !1234
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1239
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1234
  ret void
}

; quote::__private::push_shl_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_shl_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i5 = alloca [32 x i8], align 8
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct2 = alloca [12 x i8], align 4
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 60, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1240
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1244
  store i32 2, ptr %_3.i, align 8, !noalias !1240
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1245
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1240
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 60, i1 noundef zeroext true)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !1246
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1250
  store i32 2, ptr %_3.i3, align 8, !noalias !1246
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !1251
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !1246
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct2, i32 noundef 61, i1 noundef zeroext false)
  %2 = getelementptr inbounds nuw i8, ptr %punct2, i64 4
  store i32 %span, ptr %2, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i5), !noalias !1252
  %_5.sroa.4.0._3.sroa_idx.i6 = getelementptr inbounds nuw i8, ptr %_3.i5, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i6, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct2, i64 12, i1 false), !noalias !1256
  store i32 2, ptr %_3.i5, align 8, !noalias !1252
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i5), !noalias !1257
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i5), !noalias !1252
  ret void
}

; quote::__private::push_shr_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_shr_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i5 = alloca [32 x i8], align 8
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct2 = alloca [12 x i8], align 4
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 62, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1258
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1262
  store i32 2, ptr %_3.i, align 8, !noalias !1258
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1263
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1258
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 62, i1 noundef zeroext true)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !1264
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1268
  store i32 2, ptr %_3.i3, align 8, !noalias !1264
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !1269
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !1264
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct2, i32 noundef 61, i1 noundef zeroext false)
  %2 = getelementptr inbounds nuw i8, ptr %punct2, i64 4
  store i32 %span, ptr %2, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i5), !noalias !1270
  %_5.sroa.4.0._3.sroa_idx.i6 = getelementptr inbounds nuw i8, ptr %_3.i5, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i6, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct2, i64 12, i1 false), !noalias !1274
  store i32 2, ptr %_3.i5, align 8, !noalias !1270
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i5), !noalias !1275
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i5), !noalias !1270
  ret void
}

; quote::__private::push_sub_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private19push_sub_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 45, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1276
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1280
  store i32 2, ptr %_3.i, align 8, !noalias !1276
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1281
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1276
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1282
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1286
  store i32 2, ptr %_3.i2, align 8, !noalias !1282
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1287
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1282
  ret void
}

; quote::__private::push_and_and_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private20push_and_and_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 38, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1288
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1292
  store i32 2, ptr %_3.i, align 8, !noalias !1288
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1293
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1288
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 38, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1294
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1298
  store i32 2, ptr %_3.i2, align 8, !noalias !1294
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1299
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1294
  ret void
}

; quote::__private::push_caret_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private21push_caret_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 94, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1300
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1304
  store i32 2, ptr %_3.i, align 8, !noalias !1300
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1305
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1300
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 61, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1306
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1310
  store i32 2, ptr %_3.i2, align 8, !noalias !1306
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1311
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1306
  ret void
}

; quote::__private::push_lifetime_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private21push_lifetime_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %lifetime.0, i64 noundef %lifetime.1) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_9 = alloca [24 x i8], align 8
  %_8.sroa.4 = alloca [28 x i8], align 4
  %apostrophe = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %apostrophe, i32 noundef 39, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %apostrophe, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1312
  store i32 2, ptr %_3.i, align 8, !noalias !1316
  %_5.sroa.4.0._3.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.i.sroa_idx, ptr noundef nonnull align 4 dereferenceable(12) %apostrophe, i64 12, i1 false)
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1317
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1312
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_8.sroa.4)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_9)
  %_8.i = icmp ult i64 %lifetime.1, 2
  br i1 %_8.i, label %bb6.i, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit

bb6.i:                                            ; preds = %start
  %1 = icmp eq i64 %lifetime.1, 1
  br i1 %1, label %bb8, label %bb7, !prof !14

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit: ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %lifetime.0, i64 1
  %self1.i = load i8, ptr %2, align 1, !alias.scope !1318, !noundef !9
  %3 = icmp sgt i8 %self1.i, -65
  br i1 %3, label %bb8, label %bb7, !prof !14

bb8:                                              ; preds = %bb6.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit
  %data.i = getelementptr inbounds nuw i8, ptr %lifetime.0, i64 1
  %new_len.i = add i64 %lifetime.1, -1
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i, i64 noundef %new_len.i, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1c96afc0f227aaec172597eef4da1a54)
  %_8.sroa.4.8.sroa_idx = getelementptr inbounds nuw i8, ptr %_8.sroa.4, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_8.sroa.4.8.sroa_idx, ptr noundef nonnull align 8 dereferenceable(24) %_9, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_9)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1321
  store i32 1, ptr %_3.i2, align 8, !noalias !1325
  %_8.sroa.4.0._3.i2.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_8.sroa.4.0._3.i2.sroa_idx, ptr noundef nonnull align 4 dereferenceable(28) %_8.sroa.4, i64 28, i1 false), !noalias !1325
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1326
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1321
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_8.sroa.4)
  ret void

bb7:                                              ; preds = %bb6.i, %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit
; call core::str::slice_error_fail
  call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %lifetime.0, i64 noundef %lifetime.1, i64 noundef 1, i64 noundef %lifetime.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_194591c686f9a063bbeab3b098b3fcb1) #24
  unreachable
}

; quote::__private::push_question_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private21push_question_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 63, i1 noundef zeroext false)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1327
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1331
  store i32 2, ptr %_3.i, align 8, !noalias !1327
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1332
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1327
  ret void
}

; quote::__private::push_fat_arrow_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private22push_fat_arrow_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i2 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 61, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1333
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1337
  store i32 2, ptr %_3.i, align 8, !noalias !1333
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1338
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1333
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 62, i1 noundef zeroext false)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i2), !noalias !1339
  %_5.sroa.4.0._3.sroa_idx.i3 = getelementptr inbounds nuw i8, ptr %_3.i2, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i3, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1343
  store i32 2, ptr %_3.i2, align 8, !noalias !1339
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i2), !noalias !1344
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i2), !noalias !1339
  ret void
}

; quote::__private::push_dot_dot_eq_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private23push_dot_dot_eq_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_3.i5 = alloca [32 x i8], align 8
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %punct2 = alloca [12 x i8], align 4
  %punct1 = alloca [12 x i8], align 4
  %punct = alloca [12 x i8], align 4
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct, i32 noundef 46, i1 noundef zeroext true)
  %0 = getelementptr inbounds nuw i8, ptr %punct, i64 4
  store i32 %span, ptr %0, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1345
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct, i64 12, i1 false), !noalias !1349
  store i32 2, ptr %_3.i, align 8, !noalias !1345
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1350
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1345
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct1, i32 noundef 46, i1 noundef zeroext true)
  %1 = getelementptr inbounds nuw i8, ptr %punct1, i64 4
  store i32 %span, ptr %1, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !1351
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct1, i64 12, i1 false), !noalias !1355
  store i32 2, ptr %_3.i3, align 8, !noalias !1351
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !1356
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !1351
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %punct2, i32 noundef 61, i1 noundef zeroext false)
  %2 = getelementptr inbounds nuw i8, ptr %punct2, i64 4
  store i32 %span, ptr %2, align 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i5), !noalias !1357
  %_5.sroa.4.0._3.sroa_idx.i6 = getelementptr inbounds nuw i8, ptr %_3.i5, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i6, ptr noundef nonnull readonly align 4 dereferenceable(12) %punct2, i64 12, i1 false), !noalias !1361
  store i32 2, ptr %_3.i5, align 8, !noalias !1357
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i5), !noalias !1362
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i5), !noalias !1357
  ret void
}

; quote::__private::push_underscore_spanned
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private23push_underscore_spanned(ptr noalias noundef align 8 dereferenceable(32) %tokens, i32 noundef %span) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_27cca3636828088e60ce450d2eca2060, i64 noundef 1, i32 noundef %span, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_53c7a51dd4d2107fe69463bcafa37cb4)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1363
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1367
  store i32 1, ptr %_3.i, align 8, !noalias !1363
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1363
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1368
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1363
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; quote::__private::parse
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private5parse(ptr noalias noundef align 8 dereferenceable(32) %tokens, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %e.i = alloca [1 x i8], align 1
  %_6 = alloca [32 x i8], align 8
  %_4 = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_4)
; call <proc_macro2::TokenStream as core::str::traits::FromStr>::from_str
  call void @_RNvXs0_Cs8M6BBVNvC7a_11proc_macro2NtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3str6traits7FromStr8from_str(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1369)
  %0 = load i64, ptr %_4, align 8, !range !233, !alias.scope !1369, !noalias !1372, !noundef !9
  %1 = icmp eq i64 %0, -9223372036854775807
  br i1 %1, label %bb2.i, label %_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote.exit, !prof !780

bb2.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 1, ptr nonnull %e.i), !noalias !1375
  %2 = getelementptr inbounds nuw i8, ptr %_4, i64 8
  %3 = load i8, ptr %2, align 8, !range !180, !alias.scope !1369, !noalias !1372, !noundef !9
  store i8 %3, ptr %e.i, align 1, !noalias !1375
; call core::result::unwrap_failed
  call void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_34ccc9a2f61247d7615f0501afc88ea6, i64 noundef 20, ptr noundef nonnull align 1 %e.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_8177639792cada302c0caf96986c44a5) #26, !noalias !1376
  unreachable

_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote.exit: ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_6)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_6, ptr noundef nonnull readonly align 8 dereferenceable(32) %_4, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_4)
; call <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenStream>>
  call fastcc void @_RINvXs5_Cs8M6BBVNvC7a_11proc_macro2NtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBx_E6extendINtNtNtBW_7sources4once4OnceBx_EECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_6)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_6)
  ret void
}

; quote::__private::push_at
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_at(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1377
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 64, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1377
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1381
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1377
  ret void
}

; quote::__private::push_eq
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_eq(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1382
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1382
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1386
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1382
  ret void
}

; quote::__private::push_ge
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_ge(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1387
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 62, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1387
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1391
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1387
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1392
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1392
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1396
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1392
  ret void
}

; quote::__private::push_gt
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_gt(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1397
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 62, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1397
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1401
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1397
  ret void
}

; quote::__private::push_le
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_le(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1402
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 60, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1402
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1406
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1402
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1407
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1407
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1411
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1407
  ret void
}

; quote::__private::push_lt
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_lt(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1412
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 60, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1412
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1416
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1412
  ret void
}

; quote::__private::push_ne
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_ne(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1417
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 33, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1417
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1421
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1417
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1422
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 61, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1422
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1426
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1422
  ret void
}

; quote::__private::push_or
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private7push_or(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1427
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 124, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1427
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1431
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1427
  ret void
}

; quote::__private::mk_ident
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8mk_ident(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %id.0, i64 noundef %id.1, i32 noundef range(i32 0, 2) %0, i32 %1) unnamed_addr #0 {
start:
  %2 = trunc nuw i32 %0 to i1
  br i1 %2, label %bb2, label %bb4

bb4:                                              ; preds = %start
; call <proc_macro2::Span>::call_site
  %3 = tail call noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
  br label %bb2

bb2:                                              ; preds = %start, %bb4
  %span1.sroa.0.0 = phi i32 [ %3, %bb4 ], [ %1, %start ]
  %_4.i.i = icmp samesign ugt i64 %id.1, 1
  br i1 %_4.i.i, label %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i, label %bb3.i

_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i: ; preds = %bb2
  %4 = tail call i32 @memcmp(ptr noundef nonnull dereferenceable(2) @alloc_26741d7be2999f870b5ef1ed52a52387, ptr noundef nonnull readonly align 1 dereferenceable(2) %id.0, i64 2), !noalias !1432
  %5 = icmp eq i32 %4, 0
  br i1 %5, label %bb2.i, label %bb3.i

bb3.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i, %bb2
; call <proc_macro2::Ident>::new
  tail call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %id.0, i64 noundef %id.1, i32 noundef %span1.sroa.0.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_45cdb82e1f6d7e9f36108024db2c250f)
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit

bb2.i:                                            ; preds = %_RNvMNtCsjMrxcFdYDNN_4core5sliceSh11starts_withCs1SHOYL7dTh6_5quote.exit.i
  %_13.i = add i64 %id.1, -2
  %_15.i = getelementptr inbounds nuw i8, ptr %id.0, i64 2
; call <proc_macro2::Ident>::new_raw
  tail call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident7new_raw(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_15.i, i64 noundef %_13.i, i32 noundef %span1.sroa.0.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_1ab7e69cb0be14b6daa10eaf0accb519)
  br label %_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit

_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw.exit: ; preds = %bb3.i, %bb2.i
  ret void
}

; quote::__private::push_add
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_add(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1435
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 43, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1435
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1439
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1435
  ret void
}

; quote::__private::push_and
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_and(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1440
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 38, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1440
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1444
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1440
  ret void
}

; quote::__private::push_div
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_div(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1445
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 47, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1445
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1449
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1445
  ret void
}

; quote::__private::push_dot
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_dot(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1450
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 46, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1450
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1454
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1450
  ret void
}

; quote::__private::push_rem
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_rem(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1455
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 37, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1455
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1459
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1455
  ret void
}

; quote::__private::push_shl
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_shl(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1460
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 60, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1460
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1464
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1460
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1465
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 60, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1465
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1469
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1465
  ret void
}

; quote::__private::push_shr
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_shr(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1470
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 62, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1470
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1474
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1470
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1475
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 62, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1475
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1479
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1475
  ret void
}

; quote::__private::push_sub
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private8push_sub(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1480
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 45, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1480
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1484
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1480
  ret void
}

; quote::__private::push_bang
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private9push_bang(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1485
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 33, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1485
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1489
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1485
  ret void
}

; quote::__private::push_dot2
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private9push_dot2(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1490
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 46, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1490
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1494
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1490
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1495
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 46, i1 noundef zeroext false)
  store i32 2, ptr %_3.i1, align 8, !noalias !1495
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1499
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1495
  ret void
}

; quote::__private::push_dot3
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private9push_dot3(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i3 = alloca [32 x i8], align 8
  %_3.i1 = alloca [32 x i8], align 8
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1500
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 46, i1 noundef zeroext true)
  store i32 2, ptr %_3.i, align 8, !noalias !1500
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1504
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1500
  %_5.sroa.4.0._3.sroa_idx.i2 = getelementptr inbounds nuw i8, ptr %_3.i1, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i1), !noalias !1505
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i2, i32 noundef 46, i1 noundef zeroext true)
  store i32 2, ptr %_3.i1, align 8, !noalias !1505
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i1), !noalias !1509
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i1), !noalias !1505
  %_5.sroa.4.0._3.sroa_idx.i4 = getelementptr inbounds nuw i8, ptr %_3.i3, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i3), !noalias !1510
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i4, i32 noundef 46, i1 noundef zeroext false)
  store i32 2, ptr %_3.i3, align 8, !noalias !1510
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i3), !noalias !1514
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i3), !noalias !1510
  ret void
}

; quote::__private::push_semi
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private9push_semi(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1515
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 59, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1515
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1519
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1515
  ret void
}

; quote::__private::push_star
; Function Attrs: uwtable
define void @_RNvNtCs1SHOYL7dTh6_5quote9___private9push_star(ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1520
; call <proc_macro2::Punct>::new
  call void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, i32 noundef 42, i1 noundef zeroext false)
  store i32 2, ptr %_3.i, align 8, !noalias !1520
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1524
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1520
  ret void
}

; <proc_macro2::Ident as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs0_NtCs1SHOYL7dTh6_5quote14ident_fragmentNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !1525
  store i64 0, ptr %buf.i, align 8, !noalias !1525
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !1525
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !1525
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !1525
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !1525
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !1525
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !1525
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !1525
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.2, ptr %1, align 8, !noalias !1525
; invoke <proc_macro2::Ident as core::fmt::Display>::fmt
  %_8.i = invoke noundef zeroext i1 @_RNvXsA_Cs8M6BBVNvC7a_11proc_macro2NtB5_5IdentNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter.i)
          to label %bb1.i unwind label %cleanup.i, !noalias !1529

cleanup.i:                                        ; preds = %bb2.i.i, %start
  %2 = landingpad { ptr, i32 }
          cleanup
  call void @llvm.experimental.noalias.scope.decl(metadata !1530)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !1530, !noalias !1525
  %3 = icmp eq i64 %_1.val.i.i, 0
  br i1 %3, label %common.resume, label %bb2.i.i.i4.i.i.i

bb2.i.i.i4.i.i.i:                                 ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1530, !noalias !1525, !nonnull !9, !noundef !9
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #21, !noalias !1533
  br label %common.resume

bb1.i:                                            ; preds = %start
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !1525
  br i1 %_8.i, label %bb2.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit, !prof !780

bb2.i.i:                                          ; preds = %bb1.i
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #26
          to label %.noexc.i unwind label %cleanup.i, !noalias !1529

.noexc.i:                                         ; preds = %bb2.i.i
  unreachable

common.resume:                                    ; preds = %cleanup, %bb2.i.i.i4.i.i, %cleanup.i, %bb2.i.i.i4.i.i.i
  %common.resume.op = phi { ptr, i32 } [ %2, %bb2.i.i.i4.i.i.i ], [ %2, %cleanup.i ], [ %4, %bb2.i.i.i4.i.i ], [ %4, %cleanup ]
  resume { ptr, i32 } %common.resume.op

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit: ; preds = %bb1.i
  call void @llvm.lifetime.end.p0(i64 0, ptr nonnull %e.i.i), !noalias !1525
  %id.sroa.0.0.copyload = load i64, ptr %buf.i, align 8, !noalias !1534
  %id.sroa.5.0.copyload = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !1534, !nonnull !9, !noundef !9
  %id.sroa.9.0.copyload = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !1534
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !1525
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !1525
  %_4.i = icmp samesign ugt i64 %id.sroa.9.0.copyload, 1
  br i1 %_4.i, label %bb8, label %bb6.invoke

cleanup:                                          ; preds = %bb6.invoke
  %4 = landingpad { ptr, i32 }
          cleanup
  %5 = icmp eq i64 %id.sroa.0.0.copyload, 0
  br i1 %5, label %common.resume, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %cleanup
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %id.sroa.5.0.copyload, i64 noundef %id.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #21, !noalias !1535
  br label %common.resume

bb8:                                              ; preds = %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit
  %6 = call i32 @memcmp(ptr noundef nonnull dereferenceable(2) @alloc_26741d7be2999f870b5ef1ed52a52387, ptr noundef nonnull readonly align 1 dereferenceable(2) %id.sroa.5.0.copyload, i64 2)
  %7 = icmp eq i32 %6, 0
  br i1 %7, label %bb6, label %bb6.invoke

bb6:                                              ; preds = %bb8
  %_20 = add i64 %id.sroa.9.0.copyload, -2
  %_22 = getelementptr inbounds nuw i8, ptr %id.sroa.5.0.copyload, i64 2
  br label %bb6.invoke

bb6.invoke:                                       ; preds = %bb8, %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit, %bb6
  %8 = phi ptr [ %_22, %bb6 ], [ %id.sroa.5.0.copyload, %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit ], [ %id.sroa.5.0.copyload, %bb8 ]
  %9 = phi i64 [ %_20, %bb6 ], [ %id.sroa.9.0.copyload, %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote.exit ], [ %id.sroa.9.0.copyload, %bb8 ]
; invoke <str as core::fmt::Display>::fmt
  %10 = invoke noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %8, i64 noundef %9, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
          to label %bb1 unwind label %cleanup

bb1:                                              ; preds = %bb6.invoke
  %11 = icmp eq i64 %id.sroa.0.0.copyload, 0
  br i1 %11, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote.exit5, label %bb2.i.i.i4.i.i3

bb2.i.i.i4.i.i3:                                  ; preds = %bb1
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %id.sroa.5.0.copyload, i64 noundef %id.sroa.0.0.copyload, i64 noundef range(i64 1, -9223372036854775807) 1) #21, !noalias !1538
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote.exit5

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote.exit5: ; preds = %bb1, %bb2.i.i.i4.i.i3
  ret i1 %10
}

; <bool as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtCs1SHOYL7dTh6_5quote14ident_fragmentbNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <bool as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <str as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs3_NtCs1SHOYL7dTh6_5quote14ident_fragmenteNtB5_13IdentFragment3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <alloc::string::String as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs4_NtCs1SHOYL7dTh6_5quote14ident_fragmentNtNtCsdJPVW0sQgAG_5alloc6string6StringNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_8 = load ptr, ptr %0, align 8, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7 = load i64, ptr %1, align 8, !noundef !9
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_8, i64 noundef %_7, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <str as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs4_NtCs1SHOYL7dTh6_5quote9to_tokenseNtB5_8ToTokens9to_tokens(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
; call <proc_macro2::Literal>::string
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal6string(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1541
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1545
  store i32 3, ptr %_3.i, align 8, !noalias !1541
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1541
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1546
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1541
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <char as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs5_NtCs1SHOYL7dTh6_5quote14ident_fragmentcNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <char as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsk_NtCsjMrxcFdYDNN_4core3fmtcNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <alloc::string::String as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs5_NtCs1SHOYL7dTh6_5quote9to_tokensNtNtCsdJPVW0sQgAG_5alloc6string6StringNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i.i = alloca [28 x i8], align 4
  %_3.i.i = alloca [32 x i8], align 8
  %_4.i = alloca [24 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_9 = load ptr, ptr %0, align 8, !nonnull !9, !noundef !9
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_8 = load i64, ptr %1, align 8, !noundef !9
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4.i), !noalias !1547
; call <proc_macro2::Literal>::string
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal6string(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_9, i64 noundef %_8), !noalias !1551
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i.i), !noalias !1552
  %_4.sroa.4.8.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4.i, i64 24, i1 false), !noalias !1556
  store i32 3, ptr %_3.i.i, align 8, !noalias !1552
  %_5.sroa.4.0._3.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_3.i.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i.i, i64 28, i1 false), !noalias !1552
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i.i), !noalias !1557
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i.i), !noalias !1552
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4.i), !noalias !1547
  ret void
}

; <u8 as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs6_NtCs1SHOYL7dTh6_5quote14ident_fragmenthNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <u8 as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) dereferenceable(1) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <i8 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs6_NtCs1SHOYL7dTh6_5quote9to_tokensaNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i8, ptr %self, align 1, !noundef !9
; call <proc_macro2::Literal>::i8_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal11i8_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i8 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1558
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1562
  store i32 3, ptr %_3.i, align 8, !noalias !1558
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1558
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1563
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1558
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u16 as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs7_NtCs1SHOYL7dTh6_5quote14ident_fragmenttNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(2) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <u16 as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3imptNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 2 captures(address, read_provenance) dereferenceable(2) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <i16 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs7_NtCs1SHOYL7dTh6_5quote9to_tokenssNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 2 captures(none) dereferenceable(2) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i16, ptr %self, align 2, !noundef !9
; call <proc_macro2::Literal>::i16_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i16_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i16 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1564
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1568
  store i32 3, ptr %_3.i, align 8, !noalias !1564
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1564
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1569
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1564
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u32 as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs8_NtCs1SHOYL7dTh6_5quote14ident_fragmentmNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <u32 as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <i32 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs8_NtCs1SHOYL7dTh6_5quote9to_tokenslNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 4 captures(none) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i32, ptr %self, align 4, !noundef !9
; call <proc_macro2::Literal>::i32_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i32_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i32 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1570
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1574
  store i32 3, ptr %_3.i, align 8, !noalias !1570
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1570
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1575
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1570
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u64 as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs9_NtCs1SHOYL7dTh6_5quote14ident_fragmentyNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <u64 as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsd_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impyNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <i64 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXs9_NtCs1SHOYL7dTh6_5quote9to_tokensxNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i64, ptr %self, align 8, !noundef !9
; call <proc_macro2::Literal>::i64_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i64_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i64 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1576
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1580
  store i32 3, ptr %_3.i, align 8, !noalias !1576
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1576
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1581
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1576
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <proc_macro2::TokenTree as core::clone::Clone>::clone
; Function Attrs: inlinehint uwtable
define internal fastcc void @_RNvXsK_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(32) %_0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(32) %self) unnamed_addr #5 personality ptr @rust_eh_personality {
start:
  %_14 = alloca [24 x i8], align 8
  %0 = load i32, ptr %self, align 8, !range !132, !noundef !9
  switch i32 %0, label %default.unreachable12 [
    i32 0, label %bb5
    i32 1, label %bb4
    i32 2, label %bb3
    i32 3, label %bb2
  ]

default.unreachable12:                            ; preds = %start
  unreachable

bb5:                                              ; preds = %start
  %_28 = getelementptr inbounds nuw i8, ptr %self, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1582)
  %1 = load i32, ptr %_28, align 8, !range !95, !alias.scope !1582, !noalias !1585, !noundef !9
  %2 = trunc nuw i32 %1 to i1
  br i1 %2, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %bb5
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %_7.i = load i8, ptr %4, align 8, !range !1027, !alias.scope !1582, !noalias !1585, !noundef !9
  %_13.i = load ptr, ptr %3, align 8, !alias.scope !1582, !noalias !1585, !nonnull !9, !noundef !9
  %_14.i = load i64, ptr %_13.i, align 8, !noalias !1587, !noundef !9
  %_15.i = icmp ne i64 %_14.i, 0
  tail call void @llvm.assume(i1 %_15.i)
  %_16.i = add i64 %_14.i, 1
  store i64 %_16.i, ptr %_13.i, align 8, !noalias !1587
  %5 = icmp eq i64 %_16.i, 0
  br i1 %5, label %bb5.i, label %_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit, !prof !780

bb3.i:                                            ; preds = %bb5
  %_24.i = getelementptr inbounds nuw i8, ptr %self, i64 12
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 28
  %_2.i.i = load i8, ptr %6, align 4, !range !1027, !alias.scope !1588, !noalias !1591, !noundef !9
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %8 = load i32, ptr %7, align 8, !alias.scope !1588, !noalias !1591, !noundef !9
  %.not.i.i = icmp eq i32 %8, 0
  br i1 %.not.i.i, label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb3.i
; call <proc_macro::bridge::client::TokenStream as core::clone::Clone>::clone
  %_10.i.i = tail call noundef i32 @_RNvXNtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB2_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %7), !noalias !1591
  br label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i

_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i: ; preds = %bb7.i.i, %bb3.i
  %storemerge.i.i = phi i32 [ %_10.i.i, %bb7.i.i ], [ 0, %bb3.i ]
  %self.val.i.i.i = load i32, ptr %_24.i, align 4, !range !559, !alias.scope !1593, !noalias !1596, !noundef !9
  %_5.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_5.val.i.i.i = load i64, ptr %_5.i.i.i, align 8, !alias.scope !1593, !noalias !1596
  %9 = inttoptr i64 %_5.val.i.i.i to ptr
  %_27.sroa.8.sroa.0.0.extract.trunc = trunc i32 %storemerge.i.i to i8
  %_27.sroa.8.sroa.5.0.extract.shift = and i32 %storemerge.i.i, -256
  br label %_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit

bb5.i:                                            ; preds = %bb2.i
  tail call void @llvm.trap()
  unreachable

_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit: ; preds = %bb2.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i
  %_27.sroa.4.0 = phi i32 [ %self.val.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ undef, %bb2.i ]
  %_27.sroa.10.0 = phi i8 [ %_2.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ undef, %bb2.i ]
  %_27.sroa.8.sroa.0.0 = phi i8 [ %_27.sroa.8.sroa.0.0.extract.trunc, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ %_7.i, %bb2.i ]
  %_27.sroa.8.sroa.5.sroa.0.0 = phi i32 [ %_27.sroa.8.sroa.5.0.extract.shift, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ 0, %bb2.i ]
  %_27.sroa.5.0 = phi ptr [ %9, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ %_13.i, %bb2.i ]
  %storemerge.i = phi i32 [ 0, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ 1, %bb2.i ]
  %_27.sroa.8.sroa.0.0.insert.ext = zext i8 %_27.sroa.8.sroa.0.0 to i32
  %_27.sroa.8.sroa.0.0.insert.insert = or disjoint i32 %_27.sroa.8.sroa.5.sroa.0.0, %_27.sroa.8.sroa.0.0.insert.ext
  %10 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i32 %storemerge.i, ptr %10, align 8
  %_4.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 12
  store i32 %_27.sroa.4.0, ptr %_4.sroa.4.0..sroa_idx, align 4
  %_4.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %_27.sroa.5.0, ptr %_4.sroa.5.0..sroa_idx, align 8
  %_4.sroa.6.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i32 %_27.sroa.8.sroa.0.0.insert.insert, ptr %_4.sroa.6.0..sroa_idx, align 8
  %_4.sroa.7.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 28
  store i8 %_27.sroa.10.0, ptr %_4.sroa.7.0..sroa_idx, align 4
  store i32 0, ptr %_0, align 8
  br label %bb6

bb4:                                              ; preds = %start
  %11 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %12 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %13 = load i8, ptr %12, align 8, !range !180, !noundef !9
  %.not5 = icmp eq i8 %13, 2
  br i1 %.not5, label %bb13, label %bb12

bb3:                                              ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %self, i64 32, i1 false)
  br label %bb6

bb2:                                              ; preds = %start
  %14 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %15 = load i64, ptr %14, align 8, !range !60, !noundef !9
  %.not = icmp eq i64 %15, -9223372036854775808
  br i1 %.not, label %bb8, label %bb7

bb6:                                              ; preds = %bb9, %bb3, %bb14, %_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit
  ret void

bb12:                                             ; preds = %bb4
; call <alloc::boxed::Box<str> as core::clone::Clone>::clone
  %16 = tail call { ptr, i64 } @_RNvXsf_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxeENtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %11)
  %_22.0 = extractvalue { ptr, i64 } %16, 0
  %_22.1 = extractvalue { ptr, i64 } %16, 1
  br label %bb14

bb13:                                             ; preds = %bb4
  %self.val.i = load i64, ptr %11, align 8, !alias.scope !1598, !noalias !1601
  %17 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %18 = load i8, ptr %17, align 8, !range !13, !alias.scope !1598, !noalias !1601, !noundef !9
  %19 = inttoptr i64 %self.val.i to ptr
  %_18.sroa.0.sroa.5.0.insert.ext = zext nneg i8 %18 to i64
  br label %bb14

bb14:                                             ; preds = %bb12, %bb13
  %_18.sroa.0.sroa.0.0 = phi ptr [ %_22.0, %bb12 ], [ %19, %bb13 ]
  %_18.sroa.0.sroa.5.0 = phi i64 [ %_22.1, %bb12 ], [ %_18.sroa.0.sroa.5.0.insert.ext, %bb13 ]
  %20 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %_18.sroa.0.sroa.0.0, ptr %20, align 8
  %_6.sroa.0.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store i64 %_18.sroa.0.sroa.5.0, ptr %_6.sroa.0.sroa.4.0..sroa_idx, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i8 %13, ptr %_6.sroa.4.0..sroa_idx, align 8
  store i32 1, ptr %_0, align 8
  br label %bb6

bb7:                                              ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_14)
; call <alloc::string::String as core::clone::Clone>::clone
  call void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_14, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %14)
  %_13.sroa.0.0.copyload = load i64, ptr %_14, align 8
  %_13.sroa.4.0._14.sroa_idx = getelementptr inbounds nuw i8, ptr %_14, i64 8
  %21 = load <2 x i32>, ptr %_13.sroa.4.0._14.sroa_idx, align 8
  %_13.sroa.4.sroa.5.0._13.sroa.4.0._14.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_14, i64 16
  %_13.sroa.4.sroa.5.0.copyload = load i32, ptr %_13.sroa.4.sroa.5.0._13.sroa.4.0._14.sroa_idx.sroa_idx, align 8
  %_13.sroa.4.sroa.6.0._13.sroa.4.0._14.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_14, i64 20
  %_13.sroa.4.sroa.6.0.copyload = load i8, ptr %_13.sroa.4.sroa.6.0._13.sroa.4.0._14.sroa_idx.sroa_idx, align 4
  %_13.sroa.4.sroa.7.0._13.sroa.4.0._14.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_14, i64 21
  %_13.sroa.4.sroa.7.0.copyload = load i8, ptr %_13.sroa.4.sroa.7.0._13.sroa.4.0._14.sroa_idx.sroa_idx, align 1
  %_13.sroa.4.sroa.8.0._13.sroa.4.0._14.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_14, i64 22
  %22 = load i16, ptr %_13.sroa.4.sroa.8.0._13.sroa.4.0._14.sroa_idx.sroa_idx, align 2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_14)
  br label %bb9

bb8:                                              ; preds = %bb2
  %_17 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %23 = getelementptr inbounds nuw i8, ptr %self, i64 28
  %_2.0.i = load i8, ptr %23, align 4, !range !1603, !alias.scope !1604, !noalias !1607, !noundef !9
  %24 = getelementptr inbounds nuw i8, ptr %self, i64 29
  %_2.1.i = load i8, ptr %24, align 1, !alias.scope !1604, !noalias !1607
  %25 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %26 = load i32, ptr %25, align 8, !alias.scope !1604, !noalias !1607, !noundef !9
  %27 = load <2 x i32>, ptr %_17, align 8, !alias.scope !1604, !noalias !1607
  br label %bb9

bb9:                                              ; preds = %bb7, %bb8
  %_10.sroa.5.sroa.9.sroa.0.0 = phi i16 [ undef, %bb8 ], [ %22, %bb7 ]
  %_10.sroa.5.sroa.6.0 = phi i32 [ %26, %bb8 ], [ %_13.sroa.4.sroa.5.0.copyload, %bb7 ]
  %_10.sroa.5.sroa.7.0 = phi i8 [ %_2.0.i, %bb8 ], [ %_13.sroa.4.sroa.6.0.copyload, %bb7 ]
  %_10.sroa.5.sroa.8.0 = phi i8 [ %_2.1.i, %bb8 ], [ %_13.sroa.4.sroa.7.0.copyload, %bb7 ]
  %_10.sroa.0.0 = phi i64 [ -9223372036854775808, %bb8 ], [ %_13.sroa.0.0.copyload, %bb7 ]
  %28 = phi <2 x i32> [ %27, %bb8 ], [ %21, %bb7 ]
  %29 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %_10.sroa.0.0, ptr %29, align 8
  %_9.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store <2 x i32> %28, ptr %_9.sroa.4.0..sroa_idx, align 8
  %_9.sroa.4.sroa.5.0._9.sroa.4.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i32 %_10.sroa.5.sroa.6.0, ptr %_9.sroa.4.sroa.5.0._9.sroa.4.0..sroa_idx.sroa_idx, align 8
  %_9.sroa.4.sroa.6.0._9.sroa.4.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 28
  store i8 %_10.sroa.5.sroa.7.0, ptr %_9.sroa.4.sroa.6.0._9.sroa.4.0..sroa_idx.sroa_idx, align 4
  %_9.sroa.4.sroa.7.0._9.sroa.4.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 29
  store i8 %_10.sroa.5.sroa.8.0, ptr %_9.sroa.4.sroa.7.0._9.sroa.4.0..sroa_idx.sroa_idx, align 1
  %_9.sroa.4.sroa.8.0._9.sroa.4.0..sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 30
  store i16 %_10.sroa.5.sroa.9.sroa.0.0, ptr %_9.sroa.4.sroa.8.0._9.sroa.4.0..sroa_idx.sroa_idx, align 2
  store i32 3, ptr %_0, align 8
  br label %bb6
}

; <core::fmt::Error as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt(ptr noalias nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #5 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_99ac8a81a24cac863217ce4a5cbfabea, i64 noundef 5)
  ret i1 %_0
}

; <alloc::string::String as core::fmt::Write>::write_char
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #5 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1609)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i = load i64, ptr %0, align 8, !alias.scope !1609, !noundef !9
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
  %self2.i.i = load i64, ptr %self, align 8, !range !16, !alias.scope !1612, !noundef !9
  %_9.i.i = sub nsw i64 %self2.i.i, %len.i
  %_7.i.i = icmp ugt i64 %ch_len.sroa.0.0.i, %_9.i.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote.exit.i, !prof !780

bb1.i.i:                                          ; preds = %bb2.i
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i, i64 noundef %ch_len.sroa.0.0.i, i64 noundef 1, i64 noundef 1)
  %count.pre.i = load i64, ptr %0, align 8, !alias.scope !1609
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote.exit.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote.exit.i: ; preds = %bb1.i.i, %bb2.i
  %count.i = phi i64 [ %len.i, %bb2.i ], [ %count.pre.i, %bb1.i.i ]
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20.i = load ptr, ptr %1, align 8, !alias.scope !1609, !nonnull !9, !noundef !9
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  br i1 %_16.i, label %bb12.i.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote.exit.i
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

bb12.i.i:                                         ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote.exit.i
  %5 = trunc nuw nsw i32 %c to i8
  store i8 %5, ptr %_8.i, align 1, !noalias !1609
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb1.i2.i:                                         ; preds = %bb7.i.i
  %6 = or disjoint i8 %3, -64
  store i8 %6, ptr %_8.i, align 1, !noalias !1609
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !noalias !1609
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb2.i.i:                                          ; preds = %bb7.i.i
  %_28.i.i = icmp samesign ult i32 %c, 65536
  br i1 %_28.i.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %7 = or disjoint i8 %4, -32
  store i8 %7, ptr %_8.i, align 1, !noalias !1609
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !noalias !1609
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 1, !noalias !1609
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_8.i, align 1, !noalias !1609
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !noalias !1609
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 1, !noalias !1609
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !noalias !1609
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit: ; preds = %bb12.i.i, %bb1.i2.i, %bb3.i.i, %bb4.i.i
  %new_len.i = add nuw i64 %ch_len.sroa.0.0.i, %len.i
  store i64 %new_len.i, ptr %0, align 8, !alias.scope !1609
  ret i1 false
}

; <alloc::string::String as core::fmt::Write>::write_str
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef nonnull readonly align 1 captures(none) %s.0, i64 noundef %s.1) unnamed_addr #5 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1615)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1618)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i.i.i = load i64, ptr %0, align 8, !alias.scope !1621, !noalias !1624, !noundef !9
  %self2.i.i.i = load i64, ptr %self, align 8, !range !16, !alias.scope !1621, !noalias !1624, !noundef !9
  %_9.i.i.i = sub i64 %self2.i.i.i, %len.i.i.i
  %_7.i.i.i = icmp ugt i64 %s.1, %_9.i.i.i
  br i1 %_7.i.i.i, label %bb1.i.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit, !prof !780

bb1.i.i.i:                                        ; preds = %start
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i.i.i, i64 noundef %s.1, i64 noundef 1, i64 noundef 1), !noalias !1624
  %len.pre.i.i = load i64, ptr %0, align 8, !alias.scope !1626, !noalias !1624
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit: ; preds = %start, %bb1.i.i.i
  %len.i.i = phi i64 [ %len.i.i.i, %start ], [ %len.pre.i.i, %bb1.i.i.i ]
  %_9.i.i = icmp sgt i64 %len.i.i, -1
  tail call void @llvm.assume(i1 %_9.i.i)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_10.i.i = load ptr, ptr %1, align 8, !alias.scope !1626, !noalias !1624, !nonnull !9, !noundef !9
  %dst.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 %len.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i, ptr nonnull readonly align 1 %s.0, i64 %s.1, i1 false), !noalias !1626
  %2 = add i64 %len.i.i, %s.1
  store i64 %2, ptr %0, align 8, !alias.scope !1626, !noalias !1624
  ret i1 false
}

; <u128 as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsa_NtCs1SHOYL7dTh6_5quote14ident_fragmentoNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 16 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <u128 as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXNtNtCsjMrxcFdYDNN_4core3fmt3numoNtB4_7Display3fmt(ptr noalias noundef nonnull readonly align 16 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <i128 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsa_NtCs1SHOYL7dTh6_5quote9to_tokensnNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 16 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i128, ptr %self, align 16, !noundef !9
; call <proc_macro2::Literal>::i128_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal13i128_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i128 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1627
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1631
  store i32 3, ptr %_3.i, align 8, !noalias !1627
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1627
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1632
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1627
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <usize as quote::ident_fragment::IdentFragment>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsb_NtCs1SHOYL7dTh6_5quote14ident_fragmentjNtB5_13IdentFragment3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
; call <usize as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <isize as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsb_NtCs1SHOYL7dTh6_5quote9to_tokensiNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i64, ptr %self, align 8, !noundef !9
; call <proc_macro2::Literal>::isize_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal14isize_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i64 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1633
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1637
  store i32 3, ptr %_3.i, align 8, !noalias !1633
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1633
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1638
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1633
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u8 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsc_NtCs1SHOYL7dTh6_5quote9to_tokenshNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i8, ptr %self, align 1, !noundef !9
; call <proc_macro2::Literal>::u8_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal11u8_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i8 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1639
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1643
  store i32 3, ptr %_3.i, align 8, !noalias !1639
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1639
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1644
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1639
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u16 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsd_NtCs1SHOYL7dTh6_5quote9to_tokenstNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 2 captures(none) dereferenceable(2) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i16, ptr %self, align 2, !noundef !9
; call <proc_macro2::Literal>::u16_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u16_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i16 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1645
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1649
  store i32 3, ptr %_3.i, align 8, !noalias !1645
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1645
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1650
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1645
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u32 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXse_NtCs1SHOYL7dTh6_5quote9to_tokensmNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 4 captures(none) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i32, ptr %self, align 4, !noundef !9
; call <proc_macro2::Literal>::u32_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u32_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i32 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1651
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1655
  store i32 3, ptr %_3.i, align 8, !noalias !1651
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1651
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1656
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1651
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u64 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsf_NtCs1SHOYL7dTh6_5quote9to_tokensyNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i64, ptr %self, align 8, !noundef !9
; call <proc_macro2::Literal>::u64_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u64_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i64 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1657
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1661
  store i32 3, ptr %_3.i, align 8, !noalias !1657
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1657
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1662
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1657
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <u128 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsg_NtCs1SHOYL7dTh6_5quote9to_tokensoNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 16 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i128, ptr %self, align 16, !noundef !9
; call <proc_macro2::Literal>::u128_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal13u128_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i128 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1663
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1667
  store i32 3, ptr %_3.i, align 8, !noalias !1663
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1663
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1668
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1663
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <usize as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsh_NtCs1SHOYL7dTh6_5quote9to_tokensjNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i64, ptr %self, align 8, !noundef !9
; call <proc_macro2::Literal>::usize_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal14usize_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i64 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1669
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1673
  store i32 3, ptr %_3.i, align 8, !noalias !1669
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1669
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1674
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1669
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <f32 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsi_NtCs1SHOYL7dTh6_5quote9to_tokensfNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 4 captures(none) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load float, ptr %self, align 4, !noundef !9
; call <proc_macro2::Literal>::f32_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12f32_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, float noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1675
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1679
  store i32 3, ptr %_3.i, align 8, !noalias !1675
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1675
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1680
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1675
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <f64 as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsj_NtCs1SHOYL7dTh6_5quote9to_tokensdNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load double, ptr %self, align 8, !noundef !9
; call <proc_macro2::Literal>::f64_suffixed
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12f64_suffixed(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, double noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1681
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1685
  store i32 3, ptr %_3.i, align 8, !noalias !1681
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1681
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1686
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1681
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <char as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsk_NtCs1SHOYL7dTh6_5quote9to_tokenscNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 4 captures(none) dereferenceable(4) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_5 = load i32, ptr %self, align 4, !range !1687, !noundef !9
; call <proc_macro2::Literal>::character
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal9character(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, i32 noundef %_5)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1688
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1692
  store i32 3, ptr %_3.i, align 8, !noalias !1688
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1688
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1693
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1688
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <bool as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsl_NtCs1SHOYL7dTh6_5quote9to_tokensbNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_6 = alloca [24 x i8], align 8
  %0 = load i8, ptr %self, align 1, !range !13, !noundef !9
  %_4 = trunc nuw i8 %0 to i1
  %. = select i1 %_4, i64 4, i64 5
  %alloc_c9ee9951a160df092319190fa06505e4.alloc_6f4357e3a3c9006d5d6d935934a9de54 = select i1 %_4, ptr @alloc_c9ee9951a160df092319190fa06505e4, ptr @alloc_6f4357e3a3c9006d5d6d935934a9de54
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_6)
; call <proc_macro2::Span>::call_site
  %_7 = tail call noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site()
; call <proc_macro2::Ident>::new
  call void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_6, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %alloc_c9ee9951a160df092319190fa06505e4.alloc_6f4357e3a3c9006d5d6d935934a9de54, i64 noundef %., i32 noundef %_7, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_e1fe06b9b3e627577c940ae5d8dbadd4)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1694
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_6, i64 24, i1 false), !noalias !1698
  store i32 1, ptr %_3.i, align 8, !noalias !1694
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1694
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1699
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1694
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_6)
  ret void
}

; <core::ffi::c_str::CStr as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsm_NtCs1SHOYL7dTh6_5quote9to_tokensNtNtNtCsjMrxcFdYDNN_4core3ffi5c_str4CStrNtB5_8ToTokens9to_tokens(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
; call <proc_macro2::Literal>::c_string
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal8c_string(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1700
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1704
  store i32 3, ptr %_3.i, align 8, !noalias !1700
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1700
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1705
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1700
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <alloc::ffi::c_str::CString as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsn_NtCs1SHOYL7dTh6_5quote9to_tokensNtNtNtCsdJPVW0sQgAG_5alloc3ffi5c_str7CStringNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_4.sroa.4.i = alloca [28 x i8], align 4
  %_3.i = alloca [32 x i8], align 8
  %_4 = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_4)
  %_6.0 = load ptr, ptr %self, align 8, !nonnull !9, !noundef !9
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_6.1 = load i64, ptr %0, align 8, !noundef !9
; call <proc_macro2::Literal>::c_string
  call void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal8c_string(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_4, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_6.0, i64 noundef %_6.1)
  call void @llvm.lifetime.start.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1706
  %_4.sroa.4.8.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_4.sroa.4.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(24) %_4.sroa.4.8.sroa_idx.i, ptr noundef nonnull readonly align 8 dereferenceable(24) %_4, i64 24, i1 false), !noalias !1710
  store i32 3, ptr %_3.i, align 8, !noalias !1706
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(28) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(28) %_4.sroa.4.i, i64 28, i1 false), !noalias !1706
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1711
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1706
  call void @llvm.lifetime.end.p0(i64 28, ptr nonnull %_4.sroa.4.i)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_4)
  ret void
}

; <proc_macro2::Group as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXso_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro25GroupNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [32 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1712)
  %0 = load i32, ptr %self, align 8, !range !95, !alias.scope !1712, !noalias !1715, !noundef !9
  %1 = trunc nuw i32 %0 to i1
  br i1 %1, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_7.i = load i8, ptr %3, align 8, !range !1027, !alias.scope !1712, !noalias !1715, !noundef !9
  %_13.i = load ptr, ptr %2, align 8, !alias.scope !1712, !noalias !1715, !nonnull !9, !noundef !9
  %_14.i = load i64, ptr %_13.i, align 8, !noalias !1717, !noundef !9
  %_15.i = icmp ne i64 %_14.i, 0
  tail call void @llvm.assume(i1 %_15.i)
  %_16.i = add i64 %_14.i, 1
  store i64 %_16.i, ptr %_13.i, align 8, !noalias !1717
  %4 = icmp eq i64 %_16.i, 0
  br i1 %4, label %bb5.i, label %_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit, !prof !780

bb3.i:                                            ; preds = %start
  %_24.i = getelementptr inbounds nuw i8, ptr %self, i64 4
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 20
  %_2.i.i = load i8, ptr %5, align 4, !range !1027, !alias.scope !1718, !noalias !1721, !noundef !9
  %6 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %7 = load i32, ptr %6, align 8, !alias.scope !1718, !noalias !1721, !noundef !9
  %.not.i.i = icmp eq i32 %7, 0
  br i1 %.not.i.i, label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb3.i
; call <proc_macro::bridge::client::TokenStream as core::clone::Clone>::clone
  %_10.i.i = tail call noundef i32 @_RNvXNtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB2_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %6), !noalias !1721
  br label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i

_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i: ; preds = %bb7.i.i, %bb3.i
  %storemerge.i.i = phi i32 [ %_10.i.i, %bb7.i.i ], [ 0, %bb3.i ]
  %self.val.i.i.i = load i32, ptr %_24.i, align 4, !range !559, !alias.scope !1723, !noalias !1726, !noundef !9
  %_5.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_5.val.i.i.i = load i64, ptr %_5.i.i.i, align 8, !alias.scope !1723, !noalias !1726
  %8 = inttoptr i64 %_5.val.i.i.i to ptr
  %_5.sroa.8.sroa.0.0.extract.trunc = trunc i32 %storemerge.i.i to i8
  %_5.sroa.8.sroa.5.0.extract.shift = and i32 %storemerge.i.i, -256
  br label %_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit

bb5.i:                                            ; preds = %bb2.i
  tail call void @llvm.trap()
  unreachable

_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit: ; preds = %bb2.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i
  %_5.sroa.8.sroa.5.sroa.0.0 = phi i32 [ %_5.sroa.8.sroa.5.0.extract.shift, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ 0, %bb2.i ]
  %_5.sroa.8.sroa.0.0 = phi i8 [ %_5.sroa.8.sroa.0.0.extract.trunc, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ %_7.i, %bb2.i ]
  %_5.sroa.10.0 = phi i8 [ %_2.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ undef, %bb2.i ]
  %_5.sroa.4.0 = phi i32 [ %self.val.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ undef, %bb2.i ]
  %_5.sroa.5.0 = phi ptr [ %8, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ %_13.i, %bb2.i ]
  %storemerge.i = phi i32 [ 0, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i ], [ 1, %bb2.i ]
  %_5.sroa.8.sroa.0.0.insert.ext = zext i8 %_5.sroa.8.sroa.0.0 to i32
  %_5.sroa.8.sroa.0.0.insert.insert = or disjoint i32 %_5.sroa.8.sroa.5.sroa.0.0, %_5.sroa.8.sroa.0.0.insert.ext
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1728
  store i32 0, ptr %_3.i, align 8, !noalias !1728
  %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  store i32 %storemerge.i, ptr %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1728
  %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 12
  store i32 %_5.sroa.4.0, ptr %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 4, !noalias !1728
  %_4.sroa.4.i.sroa.5.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  store ptr %_5.sroa.5.0, ptr %_4.sroa.4.i.sroa.5.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1728
  %_4.sroa.4.i.sroa.6.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 24
  store i32 %_5.sroa.8.sroa.0.0.insert.insert, ptr %_4.sroa.4.i.sroa.6.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1728
  %_4.sroa.4.i.sroa.7.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 28
  store i8 %_5.sroa.10.0, ptr %_4.sroa.4.i.sroa.7.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 4, !noalias !1728
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1732
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1728
  ret void
}

; <proc_macro2::Ident as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsp_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [32 x i8], align 8
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %1 = load i8, ptr %0, align 8, !range !180, !noundef !9
  %.not = icmp eq i8 %1, 2
  br i1 %.not, label %bb4, label %bb3

bb3:                                              ; preds = %start
; call <alloc::boxed::Box<str> as core::clone::Clone>::clone
  %2 = tail call { ptr, i64 } @_RNvXsf_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxeENtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(16) %self)
  %_9.0 = extractvalue { ptr, i64 } %2, 0
  %_9.1 = extractvalue { ptr, i64 } %2, 1
  br label %bb5

bb4:                                              ; preds = %start
  %self.val.i = load i64, ptr %self, align 8, !alias.scope !1733, !noalias !1736
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %4 = load i8, ptr %3, align 8, !range !13, !alias.scope !1733, !noalias !1736, !noundef !9
  %5 = inttoptr i64 %self.val.i to ptr
  %_5.sroa.0.sroa.5.0.insert.ext = zext nneg i8 %4 to i64
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %_5.sroa.0.sroa.0.0 = phi ptr [ %_9.0, %bb3 ], [ %5, %bb4 ]
  %_5.sroa.0.sroa.5.0 = phi i64 [ %_9.1, %bb3 ], [ %_5.sroa.0.sroa.5.0.insert.ext, %bb4 ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1738
  store i32 1, ptr %_3.i, align 8, !noalias !1738
  %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  store ptr %_5.sroa.0.sroa.0.0, ptr %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1738
  %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  store i64 %_5.sroa.0.sroa.5.0, ptr %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1738
  %_4.sroa.4.i.sroa.5.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 24
  store i8 %1, ptr %_4.sroa.4.i.sroa.5.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1738
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1742
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1738
  ret void
}

; <proc_macro2::Punct as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsq_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro25PunctNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 4 captures(none) dereferenceable(12) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1743
  %_5.sroa.4.0._3.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_3.i, i64 4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 4 dereferenceable(12) %_5.sroa.4.0._3.sroa_idx.i, ptr noundef nonnull readonly align 4 dereferenceable(12) %self, i64 12, i1 false), !noalias !1747
  store i32 2, ptr %_3.i, align 8, !noalias !1743
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1748
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1743
  ret void
}

; <proc_macro2::Literal as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXsr_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro27LiteralNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_3.i = alloca [32 x i8], align 8
  %_9 = alloca [24 x i8], align 8
  %0 = load i64, ptr %self, align 8, !range !60, !noundef !9
  %.not = icmp eq i64 %0, -9223372036854775808
  br i1 %.not, label %bb4, label %bb3

bb3:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_9)
; call <alloc::string::String as core::clone::Clone>::clone
  call void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %self)
  %_8.sroa.0.0.copyload = load i64, ptr %_9, align 8
  %_8.sroa.4.0._9.sroa_idx = getelementptr inbounds nuw i8, ptr %_9, i64 8
  %1 = load <2 x i32>, ptr %_8.sroa.4.0._9.sroa_idx, align 8
  %_8.sroa.4.sroa.5.0._8.sroa.4.0._9.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_9, i64 16
  %_8.sroa.4.sroa.5.0.copyload = load i32, ptr %_8.sroa.4.sroa.5.0._8.sroa.4.0._9.sroa_idx.sroa_idx, align 8
  %_8.sroa.4.sroa.6.0._8.sroa.4.0._9.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_9, i64 20
  %_8.sroa.4.sroa.6.0.copyload = load i8, ptr %_8.sroa.4.sroa.6.0._8.sroa.4.0._9.sroa_idx.sroa_idx, align 4
  %_8.sroa.4.sroa.7.0._8.sroa.4.0._9.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_9, i64 21
  %_8.sroa.4.sroa.7.0.copyload = load i8, ptr %_8.sroa.4.sroa.7.0._8.sroa.4.0._9.sroa_idx.sroa_idx, align 1
  %_8.sroa.4.sroa.8.0._8.sroa.4.0._9.sroa_idx.sroa_idx = getelementptr inbounds nuw i8, ptr %_9, i64 22
  %2 = load i16, ptr %_8.sroa.4.sroa.8.0._8.sroa.4.0._9.sroa_idx.sroa_idx, align 2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_9)
  br label %bb5

bb4:                                              ; preds = %start
  %_12 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %self, i64 20
  %_2.0.i = load i8, ptr %3, align 4, !range !1603, !alias.scope !1749, !noalias !1752, !noundef !9
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 21
  %_2.1.i = load i8, ptr %4, align 1, !alias.scope !1749, !noalias !1752
  %5 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %6 = load i32, ptr %5, align 8, !alias.scope !1749, !noalias !1752, !noundef !9
  %7 = load <2 x i32>, ptr %_12, align 8, !alias.scope !1749, !noalias !1752
  br label %bb5

bb5:                                              ; preds = %bb3, %bb4
  %_5.sroa.5.sroa.9.sroa.0.0 = phi i16 [ undef, %bb4 ], [ %2, %bb3 ]
  %_5.sroa.5.sroa.6.0 = phi i32 [ %6, %bb4 ], [ %_8.sroa.4.sroa.5.0.copyload, %bb3 ]
  %_5.sroa.5.sroa.7.0 = phi i8 [ %_2.0.i, %bb4 ], [ %_8.sroa.4.sroa.6.0.copyload, %bb3 ]
  %_5.sroa.5.sroa.8.0 = phi i8 [ %_2.1.i, %bb4 ], [ %_8.sroa.4.sroa.7.0.copyload, %bb3 ]
  %_5.sroa.0.0 = phi i64 [ -9223372036854775808, %bb4 ], [ %_8.sroa.0.0.copyload, %bb3 ]
  %8 = phi <2 x i32> [ %7, %bb4 ], [ %1, %bb3 ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1754
  store i32 3, ptr %_3.i, align 8, !noalias !1754
  %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 8
  store i64 %_5.sroa.0.0, ptr %_4.sroa.4.i.sroa.3.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1754
  %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 16
  store <2 x i32> %8, ptr %_4.sroa.4.i.sroa.4.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1754
  %_4.sroa.4.i.sroa.6.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 24
  store i32 %_5.sroa.5.sroa.6.0, ptr %_4.sroa.4.i.sroa.6.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 8, !noalias !1754
  %_4.sroa.4.i.sroa.7.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 28
  store i8 %_5.sroa.5.sroa.7.0, ptr %_4.sroa.4.i.sroa.7.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 4, !noalias !1754
  %_4.sroa.4.i.sroa.8.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 29
  store i8 %_5.sroa.5.sroa.8.0, ptr %_4.sroa.4.i.sroa.8.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 1, !noalias !1754
  %_4.sroa.4.i.sroa.9.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx = getelementptr inbounds nuw i8, ptr %_3.i, i64 30
  store i16 %_5.sroa.5.sroa.9.sroa.0.0, ptr %_4.sroa.4.i.sroa.9.0._5.sroa.4.0._3.sroa_idx.i.sroa_idx, align 2, !noalias !1754
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1758
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1754
  ret void
}

; <proc_macro2::TokenTree as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXss_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 {
start:
  %_3.i = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_3.i), !noalias !1759
; call <proc_macro2::TokenTree as core::clone::Clone>::clone
  call fastcc void @_RNvXsK_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef align 8 captures(none) dereferenceable(32) %_3.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self) #27
; call <proc_macro2::imp::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenTree>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenTree>>
  call fastcc void @_RINvXs7_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB12_7sources4once4OnceB1O_EECs1SHOYL7dTh6_5quote(ptr noalias noundef nonnull align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_3.i), !noalias !1763
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_3.i), !noalias !1759
  ret void
}

; <proc_macro2::TokenStream as quote::to_tokens::ToTokens>::to_tokens
; Function Attrs: uwtable
define void @_RNvXst_NtCs1SHOYL7dTh6_5quote9to_tokensNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB5_8ToTokens9to_tokens(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef align 8 dereferenceable(32) %tokens) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %val.sroa.19.i.i.i.i = alloca [3 x i8], align 1
  %vec.i.i.i.i = alloca [24 x i8], align 8
  %_2.i.i = alloca [4 x i8], align 4
  %_4 = alloca [32 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_4)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1764)
  %0 = load i64, ptr %self, align 8, !range !60, !alias.scope !1764, !noalias !1767, !noundef !9
  %1 = icmp eq i64 %0, -9223372036854775808
  br i1 %1, label %bb2.i, label %bb3.i

bb2.i:                                            ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_11.i = load ptr, ptr %2, align 8, !alias.scope !1764, !noalias !1767, !nonnull !9, !noundef !9
  %_12.i = load i64, ptr %_11.i, align 8, !noalias !1769, !noundef !9
  %_13.i = icmp ne i64 %_12.i, 0
  tail call void @llvm.assume(i1 %_13.i)
  %_14.i = add i64 %_12.i, 1
  store i64 %_14.i, ptr %_11.i, align 8, !noalias !1769
  %3 = icmp eq i64 %_14.i, 0
  br i1 %3, label %bb6.i, label %_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit, !prof !780

bb3.i:                                            ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1770)
  call void @llvm.lifetime.start.p0(i64 4, ptr nonnull %_2.i.i), !noalias !1773
  %4 = getelementptr inbounds nuw i8, ptr %self, i64 24
  %5 = load i32, ptr %4, align 8, !alias.scope !1775, !noalias !1776, !noundef !9
  %.not.i.i = icmp eq i32 %5, 0
  br i1 %.not.i.i, label %bb4.i.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %bb3.i
; call <proc_macro::bridge::client::TokenStream as core::clone::Clone>::clone
  %_8.i.i = tail call noundef i32 @_RNvXNtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB2_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %4), !noalias !1776
  br label %bb4.i.i

bb4.i.i:                                          ; preds = %bb7.i.i, %bb3.i
  %6 = phi i32 [ %_8.i.i, %bb7.i.i ], [ 0, %bb3.i ]
  store i32 %6, ptr %_2.i.i, align 4, !noalias !1773
  %7 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %self.val.i.i = load ptr, ptr %7, align 8, !alias.scope !1775, !noalias !1776, !nonnull !9, !noundef !9
  %8 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %self.val1.i.i = load i64, ptr %8, align 8, !alias.scope !1775, !noalias !1776, !noundef !9
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1777)
  call void @llvm.lifetime.start.p0(i64 3, ptr nonnull %val.sroa.19.i.i.i.i)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vec.i.i.i.i), !noalias !1780
  %_23.i.i.i.i.i.i.i = icmp eq i64 %self.val1.i.i, 0
  br i1 %_23.i.i.i.i.i.i.i, label %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i, label %bb6.i.i.i.i.i.i.i

_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i: ; preds = %bb4.i.i
  %9 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i, i64 16
  br label %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i
  %10 = mul nuw nsw i64 %self.val1.i.i, 20
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #21, !noalias !1784
; call __rustc::__rust_alloc
  %11 = tail call noundef align 4 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %10, i64 noundef range(i64 4, 9) 4) #21, !noalias !1784
  %12 = icmp eq ptr %11, null
  br i1 %12, label %bb3.i.i.i.i.i, label %bb20.preheader.i.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i.i.i
; invoke alloc::raw_vec::handle_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 4, i64 %10) #26
          to label %.noexc.i.i unwind label %cleanup.i.i, !noalias !1776

.noexc.i.i:                                       ; preds = %bb3.i.i.i.i.i
  unreachable

bb20.preheader.i.i.i.i:                           ; preds = %bb6.i.i.i.i.i.i.i
  store i64 %self.val1.i.i, ptr %vec.i.i.i.i, align 8, !noalias !1780
  %13 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i, i64 8
  store ptr %11, ptr %13, align 8, !noalias !1780
  %14 = getelementptr inbounds nuw i8, ptr %vec.i.i.i.i, i64 16
  %_39.i.i.i.i = getelementptr inbounds nuw %"proc_macro::TokenTree", ptr %self.val.i.i, i64 %self.val1.i.i
  br label %bb20.i.i.i.i

bb20.i.i.i.i:                                     ; preds = %bb6.i.i.i.i, %bb20.preheader.i.i.i.i
  %val.sroa.10.sroa.7.sroa.0.044.i.i.i.i = phi i24 [ %val.sroa.10.sroa.7.sroa.0.1.i.i.i.i, %bb6.i.i.i.i ], [ undef, %bb20.preheader.i.i.i.i ]
  %iter.sroa.0.043.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb6.i.i.i.i ], [ %self.val.i.i, %bb20.preheader.i.i.i.i ]
  %iter.sroa.7.041.i.i.i.i = phi i64 [ %_8.0.i.i.i.i.i, %bb6.i.i.i.i ], [ 0, %bb20.preheader.i.i.i.i ]
  %iter.sroa.10.040.i.i.i.i = phi i64 [ %15, %bb6.i.i.i.i ], [ %self.val1.i.i, %bb20.preheader.i.i.i.i ]
  %val.sroa.13.sroa.0.039.i.i.i.i = phi i8 [ %val.sroa.13.sroa.0.1.i.i.i.i, %bb6.i.i.i.i ], [ undef, %bb20.preheader.i.i.i.i ]
  %val.sroa.13.sroa.6.038.i.i.i.i = phi i8 [ %val.sroa.13.sroa.6.1.i.i.i.i, %bb6.i.i.i.i ], [ undef, %bb20.preheader.i.i.i.i ]
  %val.sroa.13.sroa.7.sroa.0.037.i.i.i.i = phi i16 [ %val.sroa.13.sroa.7.sroa.0.1.i.i.i.i, %bb6.i.i.i.i ], [ undef, %bb20.preheader.i.i.i.i ]
  %15 = add nsw i64 %iter.sroa.10.040.i.i.i.i, -1
  %_6.i.i.i.i.i.i = icmp eq ptr %iter.sroa.0.043.i.i.i.i, %_39.i.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb20.i.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 20
  %_8.0.i.i.i.i.i = add nuw nsw i64 %iter.sroa.7.041.i.i.i.i, 1
  tail call void @llvm.experimental.noalias.scope.decl(metadata !1787)
  %16 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 16
  %17 = load i8, ptr %16, align 4, !range !17, !alias.scope !1790, !noalias !1792, !noundef !9
  %18 = icmp samesign ugt i8 %17, 3
  %19 = zext nneg i8 %17 to i64
  %20 = add nsw i64 %19, -3
  %_2.i.i.i.i.i = select i1 %18, i64 %20, i64 0
  switch i64 %_2.i.i.i.i.i, label %bb1.i.i.i.i.i [
    i64 0, label %bb5.i.i.i.i.i
    i64 1, label %bb4.i.i.i.i.i
    i64 2, label %bb3.i8.i.i.i.i
    i64 3, label %bb2.i.i.i.i.i
  ]

bb1.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i
  unreachable

bb5.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i
  %21 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 12
  %22 = load i32, ptr %21, align 4, !alias.scope !1793, !noalias !1796, !noundef !9
  %.not.i.i.i.i.i.i = icmp eq i32 %22, 0
  br i1 %.not.i.i.i.i.i.i, label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i, label %bb7.i.i.i.i.i.i

bb7.i.i.i.i.i.i:                                  ; preds = %bb5.i.i.i.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::clone::Clone>::clone
  %_10.i.i10.i.i.i.i = invoke noundef i32 @_RNvXNtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB2_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull readonly align 4 captures(address, read_provenance) dereferenceable(4) %21)
          to label %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i unwind label %bb7.i.i.i.i, !noalias !1798

_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i: ; preds = %bb7.i.i.i.i.i.i, %bb5.i.i.i.i.i
  %storemerge.i.i.i.i.i.i = phi i32 [ 0, %bb5.i.i.i.i.i ], [ %_10.i.i10.i.i.i.i, %bb7.i.i.i.i.i.i ]
  %23 = load <2 x i32>, ptr %iter.sroa.0.043.i.i.i.i, align 4, !alias.scope !1799, !noalias !1802
  %_7.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 8
  %_7.val.i.i.i.i.i.i.i = load i32, ptr %_7.i.i.i.i.i.i.i, align 4, !range !559, !alias.scope !1799, !noalias !1802, !noundef !9
  %val.sroa.10.sroa.0.0.extract.trunc22.i.i.i.i = trunc i32 %_7.val.i.i.i.i.i.i.i to i8
  %val.sroa.10.sroa.7.0.extract.shift25.i.i.i.i = lshr i32 %_7.val.i.i.i.i.i.i.i, 8
  %val.sroa.10.sroa.7.0.extract.trunc26.i.i.i.i = trunc nuw i32 %val.sroa.10.sroa.7.0.extract.shift25.i.i.i.i to i24
  %val.sroa.13.sroa.0.0.extract.trunc16.i.i.i.i = trunc i32 %storemerge.i.i.i.i.i.i to i8
  %val.sroa.13.sroa.6.0.extract.shift17.i.i.i.i = lshr i32 %storemerge.i.i.i.i.i.i, 8
  %val.sroa.13.sroa.6.0.extract.trunc18.i.i.i.i = trunc i32 %val.sroa.13.sroa.6.0.extract.shift17.i.i.i.i to i8
  %val.sroa.13.sroa.7.0.extract.shift19.i.i.i.i = lshr i32 %storemerge.i.i.i.i.i.i, 16
  %val.sroa.13.sroa.7.0.extract.trunc20.i.i.i.i = trunc nuw i32 %val.sroa.13.sroa.7.0.extract.shift19.i.i.i.i to i16
  br label %bb6.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i
  %24 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 8
  %25 = load i8, ptr %24, align 4, !range !13, !alias.scope !1804, !noalias !1807, !noundef !9
  %26 = load <2 x i32>, ptr %iter.sroa.0.043.i.i.i.i, align 4, !alias.scope !1804, !noalias !1807
  br label %bb6.i.i.i.i

bb3.i8.i.i.i.i:                                   ; preds = %bb5.i.i.i.i
  %27 = load <2 x i32>, ptr %iter.sroa.0.043.i.i.i.i, align 4, !alias.scope !1809, !noalias !1798
  %val.sroa.10.0..sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 8
  %val.sroa.10.0.copyload13.i.i.i.i = load i32, ptr %val.sroa.10.0..sroa_idx.i.i.i.i, align 4, !alias.scope !1809, !noalias !1798
  %val.sroa.10.sroa.0.0.extract.trunc.i.i.i.i = trunc i32 %val.sroa.10.0.copyload13.i.i.i.i to i8
  %val.sroa.10.sroa.7.0.extract.shift.i.i.i.i = lshr i32 %val.sroa.10.0.copyload13.i.i.i.i, 8
  %val.sroa.10.sroa.7.0.extract.trunc.i.i.i.i = trunc nuw i32 %val.sroa.10.sroa.7.0.extract.shift.i.i.i.i to i24
  %val.sroa.13.0..sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 12
  %val.sroa.13.0.copyload14.i.i.i.i = load i32, ptr %val.sroa.13.0..sroa_idx.i.i.i.i, align 4, !alias.scope !1809, !noalias !1798
  %val.sroa.13.sroa.0.0.extract.trunc.i.i.i.i = trunc i32 %val.sroa.13.0.copyload14.i.i.i.i to i8
  %val.sroa.13.sroa.6.0.extract.shift.i.i.i.i = lshr i32 %val.sroa.13.0.copyload14.i.i.i.i, 8
  %val.sroa.13.sroa.6.0.extract.trunc.i.i.i.i = trunc i32 %val.sroa.13.sroa.6.0.extract.shift.i.i.i.i to i8
  %val.sroa.13.sroa.7.0.extract.shift.i.i.i.i = lshr i32 %val.sroa.13.0.copyload14.i.i.i.i, 16
  %val.sroa.13.sroa.7.0.extract.trunc.i.i.i.i = trunc nuw i32 %val.sroa.13.sroa.7.0.extract.shift.i.i.i.i to i16
  %val.sroa.19.0..sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 17
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %val.sroa.19.i.i.i.i, ptr noundef nonnull align 1 dereferenceable(3) %val.sroa.19.0..sroa_idx.i.i.i.i, i64 3, i1 false), !noalias !1798
  br label %bb6.i.i.i.i

bb2.i.i.i.i.i:                                    ; preds = %bb5.i.i.i.i
  %28 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 12
  %_2.0.i.i.i.i.i.i = load i8, ptr %28, align 4, !range !1603, !alias.scope !1810, !noalias !1813, !noundef !9
  %29 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 13
  %_2.1.i.i.i.i.i.i = load i8, ptr %29, align 1, !alias.scope !1810, !noalias !1813
  %30 = getelementptr inbounds nuw i8, ptr %iter.sroa.0.043.i.i.i.i, i64 8
  %31 = load i32, ptr %30, align 4, !alias.scope !1810, !noalias !1813, !noundef !9
  %32 = load <2 x i32>, ptr %iter.sroa.0.043.i.i.i.i, align 4, !alias.scope !1810, !noalias !1813
  %val.sroa.10.sroa.0.0.extract.trunc21.i.i.i.i = trunc i32 %31 to i8
  %val.sroa.10.sroa.7.0.extract.shift23.i.i.i.i = lshr i32 %31, 8
  %val.sroa.10.sroa.7.0.extract.trunc24.i.i.i.i = trunc nuw i32 %val.sroa.10.sroa.7.0.extract.shift23.i.i.i.i to i24
  br label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb2.i.i.i.i.i, %bb3.i8.i.i.i.i, %bb4.i.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i
  %val.sroa.13.sroa.7.sroa.0.1.i.i.i.i = phi i16 [ %val.sroa.13.sroa.7.0.extract.trunc20.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %val.sroa.13.sroa.7.sroa.0.037.i.i.i.i, %bb4.i.i.i.i.i ], [ %val.sroa.13.sroa.7.0.extract.trunc.i.i.i.i, %bb3.i8.i.i.i.i ], [ %val.sroa.13.sroa.7.sroa.0.037.i.i.i.i, %bb2.i.i.i.i.i ]
  %val.sroa.13.sroa.6.1.i.i.i.i = phi i8 [ %val.sroa.13.sroa.6.0.extract.trunc18.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %val.sroa.13.sroa.6.038.i.i.i.i, %bb4.i.i.i.i.i ], [ %val.sroa.13.sroa.6.0.extract.trunc.i.i.i.i, %bb3.i8.i.i.i.i ], [ %_2.1.i.i.i.i.i.i, %bb2.i.i.i.i.i ]
  %val.sroa.13.sroa.0.1.i.i.i.i = phi i8 [ %val.sroa.13.sroa.0.0.extract.trunc16.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %val.sroa.13.sroa.0.039.i.i.i.i, %bb4.i.i.i.i.i ], [ %val.sroa.13.sroa.0.0.extract.trunc.i.i.i.i, %bb3.i8.i.i.i.i ], [ %_2.0.i.i.i.i.i.i, %bb2.i.i.i.i.i ]
  %val.sroa.16.0.i.i.i.i = phi i8 [ %17, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ 4, %bb4.i.i.i.i.i ], [ %17, %bb3.i8.i.i.i.i ], [ 6, %bb2.i.i.i.i.i ]
  %val.sroa.10.sroa.0.0.i.i.i.i = phi i8 [ %val.sroa.10.sroa.0.0.extract.trunc22.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i ], [ %val.sroa.10.sroa.0.0.extract.trunc.i.i.i.i, %bb3.i8.i.i.i.i ], [ %val.sroa.10.sroa.0.0.extract.trunc21.i.i.i.i, %bb2.i.i.i.i.i ]
  %val.sroa.10.sroa.7.sroa.0.1.i.i.i.i = phi i24 [ %val.sroa.10.sroa.7.0.extract.trunc26.i.i.i.i, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %val.sroa.10.sroa.7.sroa.0.044.i.i.i.i, %bb4.i.i.i.i.i ], [ %val.sroa.10.sroa.7.0.extract.trunc.i.i.i.i, %bb3.i8.i.i.i.i ], [ %val.sroa.10.sroa.7.0.extract.trunc24.i.i.i.i, %bb2.i.i.i.i.i ]
  %33 = phi <2 x i32> [ %23, %_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote.exit.i.i.i.i.i ], [ %26, %bb4.i.i.i.i.i ], [ %27, %bb3.i8.i.i.i.i ], [ %32, %bb2.i.i.i.i.i ]
  %self5.i.i.i.i = getelementptr inbounds nuw %"core::mem::maybe_uninit::MaybeUninit<proc_macro::TokenTree>", ptr %11, i64 %iter.sroa.7.041.i.i.i.i
  %val.sroa.10.sroa.7.0.insert.ext.i.i.i.i = zext i24 %val.sroa.10.sroa.7.sroa.0.1.i.i.i.i to i32
  %val.sroa.10.sroa.7.0.insert.shift.i.i.i.i = shl nuw i32 %val.sroa.10.sroa.7.0.insert.ext.i.i.i.i, 8
  %val.sroa.10.sroa.0.0.insert.ext.i.i.i.i = zext i8 %val.sroa.10.sroa.0.0.i.i.i.i to i32
  %val.sroa.10.sroa.0.0.insert.insert.i.i.i.i = or disjoint i32 %val.sroa.10.sroa.7.0.insert.shift.i.i.i.i, %val.sroa.10.sroa.0.0.insert.ext.i.i.i.i
  %val.sroa.13.sroa.7.0.insert.ext.i.i.i.i = zext i16 %val.sroa.13.sroa.7.sroa.0.1.i.i.i.i to i32
  %val.sroa.13.sroa.7.0.insert.shift.i.i.i.i = shl nuw i32 %val.sroa.13.sroa.7.0.insert.ext.i.i.i.i, 16
  %val.sroa.13.sroa.6.0.insert.ext.i.i.i.i = zext i8 %val.sroa.13.sroa.6.1.i.i.i.i to i32
  %val.sroa.13.sroa.6.0.insert.shift.i.i.i.i = shl nuw nsw i32 %val.sroa.13.sroa.6.0.insert.ext.i.i.i.i, 8
  %val.sroa.13.sroa.6.0.insert.insert.i.i.i.i = or disjoint i32 %val.sroa.13.sroa.6.0.insert.shift.i.i.i.i, %val.sroa.13.sroa.7.0.insert.shift.i.i.i.i
  %val.sroa.13.sroa.0.0.insert.ext.i.i.i.i = zext i8 %val.sroa.13.sroa.0.1.i.i.i.i to i32
  %val.sroa.13.sroa.0.0.insert.insert.i.i.i.i = or disjoint i32 %val.sroa.13.sroa.6.0.insert.insert.i.i.i.i, %val.sroa.13.sroa.0.0.insert.ext.i.i.i.i
  store <2 x i32> %33, ptr %self5.i.i.i.i, align 4, !noalias !1798
  %_45.sroa.5.0.self5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %self5.i.i.i.i, i64 8
  store i32 %val.sroa.10.sroa.0.0.insert.insert.i.i.i.i, ptr %_45.sroa.5.0.self5.sroa_idx.i.i.i.i, align 4, !noalias !1798
  %_45.sroa.6.0.self5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %self5.i.i.i.i, i64 12
  store i32 %val.sroa.13.sroa.0.0.insert.insert.i.i.i.i, ptr %_45.sroa.6.0.self5.sroa_idx.i.i.i.i, align 4, !noalias !1798
  %_45.sroa.7.0.self5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %self5.i.i.i.i, i64 16
  store i8 %val.sroa.16.0.i.i.i.i, ptr %_45.sroa.7.0.self5.sroa_idx.i.i.i.i, align 4, !noalias !1798
  %_45.sroa.8.0.self5.sroa_idx.i.i.i.i = getelementptr inbounds nuw i8, ptr %self5.i.i.i.i, i64 17
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 1 dereferenceable(3) %_45.sroa.8.0.self5.sroa_idx.i.i.i.i, ptr noundef nonnull align 1 dereferenceable(3) %val.sroa.19.i.i.i.i, i64 3, i1 false), !noalias !1798
  %34 = icmp eq i64 %15, 0
  br i1 %34, label %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i, label %bb20.i.i.i.i

terminate.i.i.i.i:                                ; preds = %bb7.i.i.i.i
  %35 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !1798
  unreachable

bb7.i.i.i.i:                                      ; preds = %bb7.i.i.i.i.i.i
  %lpad.loopexit.i.i.i.i = landingpad { ptr, i32 }
          cleanup
  store i64 %iter.sroa.7.041.i.i.i.i, ptr %14, align 8, !noalias !1780
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<proc_macro::TokenTree>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeEECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(24) %vec.i.i.i.i) #22
          to label %cleanup.body.i.i unwind label %terminate.i.i.i.i, !noalias !1798

cleanup.i.i:                                      ; preds = %bb3.i.i.i.i.i
  %36 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.body.i.i

cleanup.body.i.i:                                 ; preds = %cleanup.i.i, %bb7.i.i.i.i
  %eh.lpad-body.i.i = phi { ptr, i32 } [ %36, %cleanup.i.i ], [ %lpad.loopexit.i.i.i.i, %bb7.i.i.i.i ]
  %37 = icmp eq i32 %6, 0
  br i1 %37, label %bb3.i.i, label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %cleanup.body.i.i
; invoke <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
  invoke void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef nonnull align 4 dereferenceable(4) %_2.i.i)
          to label %bb3.i.i unwind label %terminate.i.i, !noalias !1776

terminate.i.i:                                    ; preds = %bb2.i.i.i.i
  %38 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #23, !noalias !1776
  unreachable

bb3.i.i:                                          ; preds = %bb2.i.i.i.i, %cleanup.body.i.i
  resume { ptr, i32 } %eh.lpad-body.i.i

_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i: ; preds = %bb6.i.i.i.i, %bb20.i.i.i.i, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i
  %39 = phi ptr [ inttoptr (i64 4 to ptr), %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i ], [ %11, %bb20.i.i.i.i ], [ %11, %bb6.i.i.i.i ]
  %40 = phi ptr [ %9, %_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner16with_capacity_inCs1SHOYL7dTh6_5quote.exit.thread.i.i.i.i ], [ %14, %bb20.i.i.i.i ], [ %14, %bb6.i.i.i.i ]
  store i64 %self.val1.i.i, ptr %40, align 8, !noalias !1780
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vec.i.i.i.i), !noalias !1780
  call void @llvm.lifetime.end.p0(i64 3, ptr nonnull %val.sroa.19.i.i.i.i)
  call void @llvm.lifetime.end.p0(i64 4, ptr nonnull %_2.i.i), !noalias !1773
  br label %_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit

bb6.i:                                            ; preds = %bb2.i
  tail call void @llvm.trap()
  unreachable

_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit: ; preds = %bb2.i, %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i
  %_6.sroa.64.0 = phi i32 [ %6, %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i ], [ undef, %bb2.i ]
  %_6.sroa.6.0 = phi i64 [ %self.val1.i.i, %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i ], [ undef, %bb2.i ]
  %_6.sroa.5.0 = phi ptr [ %39, %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i ], [ %_11.i, %bb2.i ]
  %_6.sroa.0.0 = phi i64 [ %self.val1.i.i, %_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone.exit.i ], [ -9223372036854775808, %bb2.i ]
  store i64 %_6.sroa.0.0, ptr %_4, align 8
  %_8.sroa.4.0._4.sroa_idx = getelementptr inbounds nuw i8, ptr %_4, i64 8
  store ptr %_6.sroa.5.0, ptr %_8.sroa.4.0._4.sroa_idx, align 8
  %_8.sroa.5.0._4.sroa_idx = getelementptr inbounds nuw i8, ptr %_4, i64 16
  store i64 %_6.sroa.6.0, ptr %_8.sroa.5.0._4.sroa_idx, align 8
  %_8.sroa.6.0._4.sroa_idx = getelementptr inbounds nuw i8, ptr %_4, i64 24
  store i32 %_6.sroa.64.0, ptr %_8.sroa.6.0._4.sroa_idx, align 8
; call <proc_macro2::TokenStream as core::iter::traits::collect::Extend<proc_macro2::TokenStream>>::extend::<core::iter::sources::once::Once<proc_macro2::TokenStream>>
  call fastcc void @_RINvXs5_Cs8M6BBVNvC7a_11proc_macro2NtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBx_E6extendINtNtNtBW_7sources4once4OnceBx_EECs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(32) %tokens, ptr noalias noundef align 8 captures(address) dereferenceable(32) %_4)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_4)
  ret void
}

; <alloc::string::String as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs1SHOYL7dTh6_5quote(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #0 {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(24) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.2, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #8

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #9

; alloc::rc::rc_inner_layout_for_value_layout
; Function Attrs: uwtable
declare { i64, i64 } @_RNvNtCsdJPVW0sQgAG_5alloc2rc32rc_inner_layout_for_value_layout(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #0

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #10

; <proc_macro2::fallback::TokenStream as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #0

; <proc_macro::bridge::client::TokenStream as core::ops::drop::Drop>::drop
; Function Attrs: uwtable
declare void @_RNvXsi_NtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4drop(ptr noalias noundef align 4 dereferenceable(4)) unnamed_addr #0

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #10

; proc_macro2::imp::into_compiler_token
; Function Attrs: uwtable
declare void @_RNvNtCs8M6BBVNvC7a_11proc_macro23imp19into_compiler_token(ptr dead_on_unwind noalias noundef writable sret([20 x i8]) align 4 captures(none) dereferenceable(20), ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32)) unnamed_addr #0

; <proc_macro2::imp::DeferredTokenStream>::evaluate_now
; Function Attrs: uwtable
declare void @_RNvMNtCs8M6BBVNvC7a_11proc_macro23impNtB2_19DeferredTokenStream12evaluate_now(ptr noalias noundef align 8 dereferenceable(32)) unnamed_addr #0

; <proc_macro::ConcatStreamsHelper>::new
; Function Attrs: uwtable
declare void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper3new(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i64 noundef) unnamed_addr #0

; <proc_macro::ConcatStreamsHelper>::append_to
; Function Attrs: uwtable
declare void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper9append_to(ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(24), ptr noalias noundef align 4 dereferenceable(4)) unnamed_addr #0

; <proc_macro2::token_stream::IntoIter as core::iter::traits::iterator::Iterator>::next
; Function Attrs: uwtable
declare void @_RNvXNtCs8M6BBVNvC7a_11proc_macro212token_streamNtB2_8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4next(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(address) dereferenceable(32), ptr noalias noundef align 8 dereferenceable(40)) unnamed_addr #0

; proc_macro2::fallback::push_token_from_proc_macro
; Function Attrs: uwtable
declare void @_RNvNtCs8M6BBVNvC7a_11proc_macro28fallback26push_token_from_proc_macro(ptr noalias noundef align 8 dereferenceable(24), ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(32)) unnamed_addr #0

; <proc_macro::ConcatStreamsHelper>::push
; Function Attrs: uwtable
declare void @_RNvMsf_CsbtZo8rQoR5w_10proc_macroNtB5_19ConcatStreamsHelper4push(ptr noalias noundef align 8 dereferenceable(24), i32 noundef) unnamed_addr #0

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #11

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #12

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #13

; <proc_macro2::LexError as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsb_Cs8M6BBVNvC7a_11proc_macro2NtB5_8LexErrorNtNtCsjMrxcFdYDNN_4core3fmt5Debug3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; Function Attrs: mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare { i64, i1 } @llvm.umul.with.overflow.i64(i64, i64) #15

; core::option::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6option13unwrap_failed(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; <proc_macro2::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: uwtable
declare void @_RNvXs0_NtCs8M6BBVNvC7a_11proc_macro212token_streamNtB7_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr dead_on_unwind noalias noundef writable sret([40 x i8]) align 8 captures(none) dereferenceable(40), ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32)) unnamed_addr #0

; <proc_macro2::Span>::call_site
; Function Attrs: uwtable
declare noundef i32 @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span9call_site() unnamed_addr #0

; <proc_macro2::Span>::join
; Function Attrs: uwtable
declare { i32, i32 } @_RNvMse_Cs8M6BBVNvC7a_11proc_macro2NtB5_4Span4join(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), i32 noundef) unnamed_addr #0

; <proc_macro2::Punct>::new
; Function Attrs: uwtable
declare void @_RNvMsq_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Punct3new(ptr dead_on_unwind noalias noundef writable sret([12 x i8]) align 4 captures(none) dereferenceable(12), i32 noundef range(i32 0, 1114112), i1 noundef zeroext) unnamed_addr #0

; <proc_macro2::Group>::new
; Function Attrs: uwtable
declare void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group3new(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i8 noundef range(i8 0, 4), ptr dead_on_return noalias noundef readonly align 8 captures(none) dereferenceable(32)) unnamed_addr #0

; <proc_macro2::TokenStream as core::str::traits::FromStr>::from_str
; Function Attrs: uwtable
declare void @_RNvXs0_Cs8M6BBVNvC7a_11proc_macro2NtB5_11TokenStreamNtNtNtCsjMrxcFdYDNN_4core3str6traits7FromStr8from_str(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #14

; <proc_macro2::Ident>::new
; Function Attrs: uwtable
declare void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident3new(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i32 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Ident>::new_raw
; Function Attrs: uwtable
declare void @_RNvMst_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Ident7new_raw(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i32 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; <proc_macro2::TokenStream>::new
; Function Attrs: uwtable
declare void @_RNvMCs8M6BBVNvC7a_11proc_macro2NtB2_11TokenStream3new(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32)) unnamed_addr #0

; <proc_macro2::Group>::stream
; Function Attrs: uwtable
declare void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group6stream(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Group>::set_span
; Function Attrs: uwtable
declare void @_RNvMsn_Cs8M6BBVNvC7a_11proc_macro2NtB5_5Group8set_span(ptr noalias noundef align 8 dereferenceable(24), i32 noundef) unnamed_addr #0

; <proc_macro2::TokenTree>::set_span
; Function Attrs: uwtable
declare void @_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree8set_span(ptr noalias noundef align 8 dereferenceable(32), i32 noundef) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro::bridge::client::TokenStream as core::clone::Clone>::clone
; Function Attrs: uwtable
declare noundef range(i32 1, 0) i32 @_RNvXNtNtCsbtZo8rQoR5w_10proc_macro6bridge6clientNtB2_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4)) unnamed_addr #0

; <bool as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsg_NtCsjMrxcFdYDNN_4core3fmtbNtB5_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #16

; <proc_macro2::Literal>::string
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal6string(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <char as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsk_NtCsjMrxcFdYDNN_4core3fmtcNtB5_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <u8 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtNtCsjMrxcFdYDNN_4core3fmt3num3imphNtB6_7Display3fmt(ptr noalias noundef readonly align 1 captures(address, read_provenance) dereferenceable(1), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::i8_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal11i8_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i8 noundef) unnamed_addr #0

; <u16 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3imptNtB9_7Display3fmt(ptr noalias noundef readonly align 2 captures(address, read_provenance) dereferenceable(2), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::i16_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i16_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i16 noundef) unnamed_addr #0

; <u32 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXs8_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impmNtB9_7Display3fmt(ptr noalias noundef readonly align 4 captures(address, read_provenance) dereferenceable(4), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::i32_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i32_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i32 noundef) unnamed_addr #0

; <u64 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsd_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impyNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::i64_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12i64_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i64 noundef) unnamed_addr #0

; <proc_macro2::Ident as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsA_Cs8M6BBVNvC7a_11proc_macro2NtB5_5IdentNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: cold noreturn nounwind memory(inaccessiblemem: write)
declare void @llvm.trap() #17

; <alloc::boxed::Box<str> as core::clone::Clone>::clone
; Function Attrs: uwtable
declare { ptr, i64 } @_RNvXsf_NtCsdJPVW0sQgAG_5alloc5boxedINtB5_3BoxeENtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16)) unnamed_addr #0

; <alloc::string::String as core::clone::Clone>::clone
; Function Attrs: uwtable
declare void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; <u128 as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXNtNtCsjMrxcFdYDNN_4core3fmt3numoNtB4_7Display3fmt(ptr noalias noundef readonly align 16 captures(address, read_provenance) dereferenceable(16), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::i128_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal13i128_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i128 noundef) unnamed_addr #0

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <proc_macro2::Literal>::isize_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal14isize_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i64 noundef) unnamed_addr #0

; <proc_macro2::Literal>::u8_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal11u8_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i8 noundef) unnamed_addr #0

; <proc_macro2::Literal>::u16_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u16_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i16 noundef) unnamed_addr #0

; <proc_macro2::Literal>::u32_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u32_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i32 noundef) unnamed_addr #0

; <proc_macro2::Literal>::u64_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12u64_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i64 noundef) unnamed_addr #0

; <proc_macro2::Literal>::u128_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal13u128_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i128 noundef) unnamed_addr #0

; <proc_macro2::Literal>::usize_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal14usize_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i64 noundef) unnamed_addr #0

; <proc_macro2::Literal>::f32_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12f32_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), float noundef) unnamed_addr #0

; <proc_macro2::fallback::TokenStream as core::iter::traits::collect::IntoIterator>::into_iter
; Function Attrs: uwtable
declare void @_RNvXsc_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB5_11TokenStreamNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect12IntoIterator9into_iter(ptr dead_on_unwind noalias noundef writable sret([32 x i8]) align 8 captures(none) dereferenceable(32), ptr noundef nonnull) unnamed_addr #0

; <proc_macro2::Literal>::f64_suffixed
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal12f64_suffixed(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), double noundef) unnamed_addr #0

; <proc_macro2::Literal>::character
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal9character(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), i32 noundef range(i32 0, 1114112)) unnamed_addr #0

; <proc_macro2::Literal>::c_string
; Function Attrs: uwtable
declare void @_RNvMsC_Cs8M6BBVNvC7a_11proc_macro2NtB5_7Literal8c_string(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <alloc::rc::Rc<alloc::vec::Vec<proc_macro2::TokenTree>>>::drop_slow
; Function Attrs: noinline uwtable
declare void @_RNvMs6_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEE9drop_slowBV_(ptr noalias noundef align 8 dereferenceable(8)) unnamed_addr #18

; <proc_macro2::imp::TokenStream>::unwrap_stable
; Function Attrs: uwtable
declare noundef nonnull ptr @_RNvMs_NtCs8M6BBVNvC7a_11proc_macro23impNtB4_11TokenStream13unwrap_stable(ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(32)) unnamed_addr #0

; <proc_macro2::imp::TokenStream>::unwrap_nightly
; Function Attrs: uwtable
declare noundef i32 @_RNvMs_NtCs8M6BBVNvC7a_11proc_macro23impNtB4_11TokenStream14unwrap_nightly(ptr dead_on_return noalias noundef align 8 captures(address) dereferenceable(32)) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #19

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #20

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #6 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #9 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { mustprogress nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #16 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #17 = { cold noreturn nounwind memory(inaccessiblemem: write) }
attributes #18 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #19 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #20 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #21 = { nounwind }
attributes #22 = { cold }
attributes #23 = { cold noreturn nounwind }
attributes #24 = { noinline noreturn }
attributes #25 = { noinline }
attributes #26 = { noreturn }
attributes #27 = { inlinehint }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{!3}
!3 = distinct !{!3, !4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!4 = distinct !{!4, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!5 = !{!6}
!6 = distinct !{!6, !7, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!7 = distinct !{!7, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!8 = !{!6, !3}
!9 = !{}
!10 = !{!11}
!11 = distinct !{!11, !12, !"_RNvXs1O_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!12 = distinct !{!12, !"_RNvXs1O_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!13 = !{i8 0, i8 2}
!14 = !{!"branch_weights", !"expected", i32 2000, i32 1}
!15 = !{i64 1, i64 -9223372036854775807}
!16 = !{i64 0, i64 -9223372036854775808}
!17 = !{i8 0, i8 7}
!18 = !{!19, !21}
!19 = distinct !{!19, !20, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote: %_1"}
!20 = distinct !{!20, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote"}
!21 = distinct !{!21, !22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote: %_1.0"}
!22 = distinct !{!22, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote"}
!23 = !{!24, !26, !28, !19, !21}
!24 = distinct !{!24, !25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!25 = distinct !{!25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!26 = distinct !{!26, !27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!27 = distinct !{!27, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!28 = distinct !{!28, !29, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!29 = distinct !{!29, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!30 = !{!31, !21}
!31 = distinct !{!31, !32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote: %_1"}
!32 = distinct !{!32, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote"}
!33 = !{!34, !36, !38, !31, !21}
!34 = distinct !{!34, !35, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!35 = distinct !{!35, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!36 = distinct !{!36, !37, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!37 = distinct !{!37, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!38 = distinct !{!38, !39, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!39 = distinct !{!39, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!40 = !{i64 0, i64 -9223372036854775805}
!41 = !{!42, !44, !46}
!42 = distinct !{!42, !43, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters3map3MapIB13_INtNtNtB19_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2a_B28_INtNtNtB19_6traits7collect6ExtendB28_E6extendB1F_E0ENvMs_NtB2a_3impNtB41_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote: %_1"}
!43 = distinct !{!43, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtNtNtB4_4iter8adapters3map3MapIB13_INtNtNtB19_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2a_B28_INtNtNtB19_6traits7collect6ExtendB28_E6extendB1F_E0ENvMs_NtB2a_3impNtB41_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote"}
!44 = distinct !{!44, !45, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4fuse4FuseINtNtBL_3map3MapIB1h_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B25_B23_INtNtNtBN_6traits7collect6ExtendB23_E6extendB1B_E0ENvMs_NtB25_3impNtB3V_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote: %_1"}
!45 = distinct !{!45, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters4fuse4FuseINtNtBL_3map3MapIB1h_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B25_B23_INtNtNtBN_6traits7collect6ExtendB23_E6extendB1B_E0ENvMs_NtB25_3impNtB3V_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote"}
!46 = distinct !{!46, !47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten13FlattenCompatINtNtBL_3map3MapIB1u_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2i_B2g_INtNtNtBN_6traits7collect6ExtendB2g_E6extendB1O_E0ENvMs_NtB2i_3impNtB48_11TokenStream13unwrap_stableEINtNtB2i_5rcvec13RcVecIntoIterNtB2i_9TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!47 = distinct !{!47, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters7flatten13FlattenCompatINtNtBL_3map3MapIB1u_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2i_B2g_INtNtNtBN_6traits7collect6ExtendB2g_E6extendB1O_E0ENvMs_NtB2i_3impNtB48_11TokenStream13unwrap_stableEINtNtB2i_5rcvec13RcVecIntoIterNtB2i_9TokenTreeEEECs1SHOYL7dTh6_5quote"}
!48 = !{!49, !46}
!49 = distinct !{!49, !50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!50 = distinct !{!50, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote"}
!51 = !{!52, !46}
!52 = distinct !{!52, !53, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!53 = distinct !{!53, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote"}
!54 = !{!55, !46}
!55 = distinct !{!55, !56, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!56 = distinct !{!56, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote"}
!57 = !{!58, !46}
!58 = distinct !{!58, !59, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!59 = distinct !{!59, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB17_9TokenTreeEEECs1SHOYL7dTh6_5quote"}
!60 = !{i64 0, i64 -9223372036854775807}
!61 = !{!62}
!62 = distinct !{!62, !63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!63 = distinct !{!63, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamECs1SHOYL7dTh6_5quote"}
!64 = !{!65, !67, !69, !62}
!65 = distinct !{!65, !66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!66 = distinct !{!66, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!67 = distinct !{!67, !68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!68 = distinct !{!68, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro11TokenStreamECs1SHOYL7dTh6_5quote"}
!69 = distinct !{!69, !70, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!70 = distinct !{!70, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp19DeferredTokenStreamECs1SHOYL7dTh6_5quote"}
!71 = !{!72}
!72 = distinct !{!72, !73, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!73 = distinct !{!73, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!74 = !{!75}
!75 = distinct !{!75, !76, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!76 = distinct !{!76, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!77 = !{!78}
!78 = distinct !{!78, !79, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!79 = distinct !{!79, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!80 = !{!78, !75, !72, !81, !62}
!81 = distinct !{!81, !82, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!82 = distinct !{!82, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote"}
!83 = !{!78, !75, !72}
!84 = !{!85}
!85 = distinct !{!85, !86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!86 = distinct !{!86, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!87 = !{!88}
!88 = distinct !{!88, !89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!89 = distinct !{!89, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!90 = !{!91}
!91 = distinct !{!91, !92, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!92 = distinct !{!92, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!93 = !{!91, !88, !85, !81, !62}
!94 = !{!91, !88, !85}
!95 = !{i32 0, i32 2}
!96 = !{!97}
!97 = distinct !{!97, !98, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote: %_1"}
!98 = distinct !{!98, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote"}
!99 = !{!100, !102, !104, !97}
!100 = distinct !{!100, !101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!101 = distinct !{!101, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!102 = distinct !{!102, !103, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!103 = distinct !{!103, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!104 = distinct !{!104, !105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!105 = distinct !{!105, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!106 = !{!107}
!107 = distinct !{!107, !108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!108 = distinct !{!108, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!109 = !{!110}
!110 = distinct !{!110, !111, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!111 = distinct !{!111, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!112 = !{!113}
!113 = distinct !{!113, !114, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!114 = distinct !{!114, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!115 = !{!113, !110, !107, !116, !118, !97}
!116 = distinct !{!116, !117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!117 = distinct !{!117, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote"}
!118 = distinct !{!118, !119, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote: %_1"}
!119 = distinct !{!119, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote"}
!120 = !{!113, !110, !107}
!121 = !{!122}
!122 = distinct !{!122, !123, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!123 = distinct !{!123, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!124 = !{!125}
!125 = distinct !{!125, !126, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!126 = distinct !{!126, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!127 = !{!128}
!128 = distinct !{!128, !129, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!129 = distinct !{!129, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!130 = !{!128, !125, !122, !116, !118, !97}
!131 = !{!128, !125, !122}
!132 = !{i32 0, i32 4}
!133 = !{!134}
!134 = distinct !{!134, !135, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote: %_1"}
!135 = distinct !{!135, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote"}
!136 = !{!137, !139}
!137 = distinct !{!137, !138, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote: %_1"}
!138 = distinct !{!138, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote"}
!139 = distinct !{!139, !140, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote: %_1"}
!140 = distinct !{!140, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote"}
!141 = !{!142, !144, !146, !137, !139}
!142 = distinct !{!142, !143, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!143 = distinct !{!143, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!144 = distinct !{!144, !145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!145 = distinct !{!145, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!146 = distinct !{!146, !147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!147 = distinct !{!147, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!148 = !{!149}
!149 = distinct !{!149, !150, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!150 = distinct !{!150, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!151 = !{!152}
!152 = distinct !{!152, !153, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!153 = distinct !{!153, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!154 = !{!155}
!155 = distinct !{!155, !156, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!156 = distinct !{!156, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!157 = !{!155, !152, !149, !158, !160, !137, !139}
!158 = distinct !{!158, !159, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!159 = distinct !{!159, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote"}
!160 = distinct !{!160, !161, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote: %_1"}
!161 = distinct !{!161, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote"}
!162 = !{!155, !152, !149}
!163 = !{!164}
!164 = distinct !{!164, !165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!165 = distinct !{!165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!166 = !{!167}
!167 = distinct !{!167, !168, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!168 = distinct !{!168, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!169 = !{!170}
!170 = distinct !{!170, !171, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!171 = distinct !{!171, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!172 = !{!170, !167, !164, !158, !160, !137, !139}
!173 = !{!170, !167, !164}
!174 = !{!175}
!175 = distinct !{!175, !176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25IdentECs1SHOYL7dTh6_5quote: %_1"}
!176 = distinct !{!176, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25IdentECs1SHOYL7dTh6_5quote"}
!177 = !{!178}
!178 = distinct !{!178, !179, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5IdentECs1SHOYL7dTh6_5quote: %_1"}
!179 = distinct !{!179, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5IdentECs1SHOYL7dTh6_5quote"}
!180 = !{i8 0, i8 3}
!181 = !{!178, !175}
!182 = !{!183}
!183 = distinct !{!183, !184, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!184 = distinct !{!184, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!185 = !{!186}
!186 = distinct !{!186, !187, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote: %_1"}
!187 = distinct !{!187, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp13TokenTreeIterECs1SHOYL7dTh6_5quote"}
!188 = !{i64 0, i64 2}
!189 = !{!190}
!190 = distinct !{!190, !191, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbtZo8rQoR5w_10proc_macro12token_stream8IntoIterECs1SHOYL7dTh6_5quote: %_1"}
!191 = distinct !{!191, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsbtZo8rQoR5w_10proc_macro12token_stream8IntoIterECs1SHOYL7dTh6_5quote"}
!192 = !{!193}
!193 = distinct !{!193, !194, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtB1x_6client11TokenStreamNtB2i_4SpanNtNtB1x_6symbol6SymbolEEECs1SHOYL7dTh6_5quote: %_1"}
!194 = distinct !{!194, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtB1x_6client11TokenStreamNtB2i_4SpanNtNtB1x_6symbol6SymbolEEECs1SHOYL7dTh6_5quote"}
!195 = !{!196}
!196 = distinct !{!196, !197, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBZ_6client11TokenStreamNtB1K_4SpanNtNtBZ_6symbol6SymbolEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!197 = distinct !{!197, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBZ_6client11TokenStreamNtB1K_4SpanNtNtBZ_6symbol6SymbolEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!198 = !{!196, !193, !190, !186}
!199 = !{!200, !202}
!200 = distinct !{!200, !201, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote: %_1"}
!201 = distinct !{!201, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote"}
!202 = distinct !{!202, !203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBK_6client11TokenStreamNtB1v_4SpanNtNtBK_6symbol6SymbolEECs1SHOYL7dTh6_5quote: %_1.0"}
!203 = distinct !{!203, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeSINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBK_6client11TokenStreamNtB1v_4SpanNtNtBK_6symbol6SymbolEECs1SHOYL7dTh6_5quote"}
!204 = !{!205, !207, !200, !202}
!205 = distinct !{!205, !206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!206 = distinct !{!206, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!207 = distinct !{!207, !208, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!208 = distinct !{!208, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!209 = !{!210, !202}
!210 = distinct !{!210, !211, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote: %_1"}
!211 = distinct !{!211, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge9TokenTreeNtNtBJ_6client11TokenStreamNtB1u_4SpanNtNtBJ_6symbol6SymbolEECs1SHOYL7dTh6_5quote"}
!212 = !{!213, !215, !210, !202}
!213 = distinct !{!213, !214, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!214 = distinct !{!214, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!215 = distinct !{!215, !216, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!216 = distinct !{!216, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!217 = !{!218}
!218 = distinct !{!218, !219, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1SHOYL7dTh6_5quote: %self"}
!219 = distinct !{!219, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1SHOYL7dTh6_5quote"}
!220 = !{!221, !223}
!221 = distinct !{!221, !222, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator3mapNtNtBU_3imp11TokenStreamNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0ECs1SHOYL7dTh6_5quote: %_0"}
!222 = distinct !{!222, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator3mapNtNtBU_3imp11TokenStreamNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0ECs1SHOYL7dTh6_5quote"}
!223 = distinct !{!223, !222, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator3mapNtNtBU_3imp11TokenStreamNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0ECs1SHOYL7dTh6_5quote: %self"}
!224 = !{!225}
!225 = distinct !{!225, !226, !"_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote: %self"}
!226 = distinct !{!226, !"_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote"}
!227 = !{!228}
!228 = distinct !{!228, !226, !"_RINvXs8_NtCs8M6BBVNvC7a_11proc_macro23impNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBD_E6extendINtNtNtB12_8adapters3map3MapINtNtNtB12_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B2U_IBW_B2U_E6extendB2r_E0EECs1SHOYL7dTh6_5quote: %streams"}
!229 = !{!230, !232, !225, !228}
!230 = distinct !{!230, !231, !"_RINvXsj_CsbtZo8rQoR5w_10proc_macroNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBw_E6extendINtNtNtBV_8adapters3map3MapIB1T_INtNtNtBV_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2S_B2Q_IBP_B2Q_E6extendB2o_E0ENvMs_NtB2S_3impNtB4g_11TokenStream14unwrap_nightlyEECs1SHOYL7dTh6_5quote: %self"}
!231 = distinct !{!231, !"_RINvXsj_CsbtZo8rQoR5w_10proc_macroNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBw_E6extendINtNtNtBV_8adapters3map3MapIB1T_INtNtNtBV_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2S_B2Q_IBP_B2Q_E6extendB2o_E0ENvMs_NtB2S_3impNtB4g_11TokenStream14unwrap_nightlyEECs1SHOYL7dTh6_5quote"}
!232 = distinct !{!232, !231, !"_RINvXsj_CsbtZo8rQoR5w_10proc_macroNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBw_E6extendINtNtNtBV_8adapters3map3MapIB1T_INtNtNtBV_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B2S_B2Q_IBP_B2Q_E6extendB2o_E0ENvMs_NtB2S_3impNtB4g_11TokenStream14unwrap_nightlyEECs1SHOYL7dTh6_5quote: %streams"}
!233 = !{i64 0, i64 -9223372036854775806}
!234 = !{!230, !232, !228}
!235 = !{!236, !238, !240, !242, !243, !245, !246, !248, !249, !251, !230, !232, !225, !228}
!236 = distinct !{!236, !237, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamNtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_BW_BU_14unwrap_nightlyNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1G_NCINvXsj_B1I_B1G_INtNtB2Y_7collect6ExtendB1G_E6extendINtB4_3MapIB4C_INtNtNtB8_7sources4once4OnceNtBY_11TokenStreamENCINvXs5_BY_B5i_IB42_B5i_E6extendB4Q_E0EB2m_EE0E0E0Cs1SHOYL7dTh6_5quote: %elt"}
!237 = distinct !{!237, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtNtCs8M6BBVNvC7a_11proc_macro23imp11TokenStreamNtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_BW_BU_14unwrap_nightlyNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callB1G_NCINvXsj_B1I_B1G_INtNtB2Y_7collect6ExtendB1G_E6extendINtB4_3MapIB4C_INtNtNtB8_7sources4once4OnceNtBY_11TokenStreamENCINvXs5_BY_B5i_IB42_B5i_E6extendB4Q_E0EB2m_EE0E0E0Cs1SHOYL7dTh6_5quote"}
!238 = distinct !{!238, !239, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtNtBW_3imp11TokenStreamuNCINvXs5_BW_BU_INtNtNtB8_6traits7collect6ExtendBU_E6extendINtNtNtB8_7sources4once4OnceBU_EE0NCIB2_B1A_NtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B1C_B1A_14unwrap_nightlyNCINvNvNtNtB2j_8iterator8Iterator8for_each4callB3D_NCINvXsj_B3F_B3D_IB2f_B3D_E6extendINtB4_3MapIB6a_B2V_B1Z_EB4j_EE0E0E0E0Cs1SHOYL7dTh6_5quote: %elt"}
!239 = distinct !{!239, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtNtBW_3imp11TokenStreamuNCINvXs5_BW_BU_INtNtNtB8_6traits7collect6ExtendBU_E6extendINtNtNtB8_7sources4once4OnceBU_EE0NCIB2_B1A_NtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B1C_B1A_14unwrap_nightlyNCINvNvNtNtB2j_8iterator8Iterator8for_each4callB3D_NCINvXsj_B3F_B3D_IB2f_B3D_E6extendINtB4_3MapIB6a_B2V_B1Z_EB4j_EE0E0E0E0Cs1SHOYL7dTh6_5quote"}
!240 = distinct !{!240, !241, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator4folduNCINvNtNtBa_8adapters3map8map_foldBS_NtNtBU_3imp11TokenStreamuNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0NCIB2g_B2O_NtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B2Q_B2O_14unwrap_nightlyNCINvNvB1z_8for_each4callB4h_NCINvXsj_B4j_B4h_IB3t_B4h_E6extendINtB2i_3MapIB6s_B3_B3d_EB4X_EE0E0E0E0ECs1SHOYL7dTh6_5quote: %self"}
!241 = distinct !{!241, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator4folduNCINvNtNtBa_8adapters3map8map_foldBS_NtNtBU_3imp11TokenStreamuNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0NCIB2g_B2O_NtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B2Q_B2O_14unwrap_nightlyNCINvNvB1z_8for_each4callB4h_NCINvXsj_B4j_B4h_IB3t_B4h_E6extendINtB2i_3MapIB6s_B3_B3d_EB4X_EE0E0E0E0ECs1SHOYL7dTh6_5quote"}
!242 = distinct !{!242, !241, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtBa_6traits8iterator8Iterator4folduNCINvNtNtBa_8adapters3map8map_foldBS_NtNtBU_3imp11TokenStreamuNCINvXs5_BU_BS_INtNtB1D_7collect6ExtendBS_E6extendB3_E0NCIB2g_B2O_NtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B2Q_B2O_14unwrap_nightlyNCINvNvB1z_8for_each4callB4h_NCINvXsj_B4j_B4h_IB3t_B4h_E6extendINtB2i_3MapIB6s_B3_B3d_EB4X_EE0E0E0E0ECs1SHOYL7dTh6_5quote: argument 1"}
!243 = distinct !{!243, !244, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1r_B1p_INtNtNtBa_6traits7collect6ExtendB1p_E6extendBX_E0ENtNtB2s_8iterator8Iterator4folduNCINvB6_8map_foldNtNtB1r_3imp11TokenStreamNtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B40_B3Y_14unwrap_nightlyNCINvNvB3b_8for_each4callB4n_NCINvXsj_B4p_B4n_IB2o_B4n_E6extendIBO_BN_B53_EE0E0E0ECs1SHOYL7dTh6_5quote: %self"}
!244 = distinct !{!244, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1r_B1p_INtNtNtBa_6traits7collect6ExtendB1p_E6extendBX_E0ENtNtB2s_8iterator8Iterator4folduNCINvB6_8map_foldNtNtB1r_3imp11TokenStreamNtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B40_B3Y_14unwrap_nightlyNCINvNvB3b_8for_each4callB4n_NCINvXsj_B4p_B4n_IB2o_B4n_E6extendIBO_BN_B53_EE0E0E0ECs1SHOYL7dTh6_5quote"}
!245 = distinct !{!245, !244, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1r_B1p_INtNtNtBa_6traits7collect6ExtendB1p_E6extendBX_E0ENtNtB2s_8iterator8Iterator4folduNCINvB6_8map_foldNtNtB1r_3imp11TokenStreamNtCsbtZo8rQoR5w_10proc_macro11TokenStreamuNvMs_B40_B3Y_14unwrap_nightlyNCINvNvB3b_8for_each4callB4n_NCINvXsj_B4p_B4n_IB2o_B4n_E6extendIBO_BN_B53_EE0E0E0ECs1SHOYL7dTh6_5quote: %g"}
!246 = distinct !{!246, !247, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapIBO_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1v_B1t_INtNtNtBa_6traits7collect6ExtendB1t_E6extendB11_E0ENvMs_NtB1v_3impNtB3l_11TokenStream14unwrap_nightlyENtNtB2w_8iterator8Iterator4folduNCINvNvB45_8for_each4callNtCsbtZo8rQoR5w_10proc_macro11TokenStreamNCINvXsj_B52_B50_IB2s_B50_E6extendBN_E0E0ECs1SHOYL7dTh6_5quote: %self"}
!247 = distinct !{!247, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapIBO_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1v_B1t_INtNtNtBa_6traits7collect6ExtendB1t_E6extendB11_E0ENvMs_NtB1v_3impNtB3l_11TokenStream14unwrap_nightlyENtNtB2w_8iterator8Iterator4folduNCINvNvB45_8for_each4callNtCsbtZo8rQoR5w_10proc_macro11TokenStreamNCINvXsj_B52_B50_IB2s_B50_E6extendBN_E0E0ECs1SHOYL7dTh6_5quote"}
!248 = distinct !{!248, !247, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapIBO_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1v_B1t_INtNtNtBa_6traits7collect6ExtendB1t_E6extendB11_E0ENvMs_NtB1v_3impNtB3l_11TokenStream14unwrap_nightlyENtNtB2w_8iterator8Iterator4folduNCINvNvB45_8for_each4callNtCsbtZo8rQoR5w_10proc_macro11TokenStreamNCINvXsj_B52_B50_IB2s_B50_E6extendBN_E0E0ECs1SHOYL7dTh6_5quote: %g"}
!249 = distinct !{!249, !250, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB4_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1p_B1n_INtNtNtBa_6traits7collect6ExtendB1n_E6extendBV_E0ENvMs_NtB1p_3impNtB3e_11TokenStream14unwrap_nightlyENtNtB2q_8iterator8Iterator8for_eachNCINvXsj_CsbtZo8rQoR5w_10proc_macroNtB4G_11TokenStreamIB2m_B56_E6extendB3_E0ECs1SHOYL7dTh6_5quote: %self"}
!250 = distinct !{!250, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB4_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1p_B1n_INtNtNtBa_6traits7collect6ExtendB1n_E6extendBV_E0ENvMs_NtB1p_3impNtB3e_11TokenStream14unwrap_nightlyENtNtB2q_8iterator8Iterator8for_eachNCINvXsj_CsbtZo8rQoR5w_10proc_macroNtB4G_11TokenStreamIB2m_B56_E6extendB3_E0ECs1SHOYL7dTh6_5quote"}
!251 = distinct !{!251, !250, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB4_INtNtNtBa_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1p_B1n_INtNtNtBa_6traits7collect6ExtendB1n_E6extendBV_E0ENvMs_NtB1p_3impNtB3e_11TokenStream14unwrap_nightlyENtNtB2q_8iterator8Iterator8for_eachNCINvXsj_CsbtZo8rQoR5w_10proc_macroNtB4G_11TokenStreamIB2m_B56_E6extendB3_E0ECs1SHOYL7dTh6_5quote: %f"}
!252 = !{!240, !242, !243, !245, !246, !248, !249, !251, !230, !232, !225, !228}
!253 = !{!225, !228}
!254 = !{!255}
!255 = distinct !{!255, !256, !"_RINvXsb_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBI_E6extendINtNtNtB17_8adapters3map3MapIB25_INtNtNtB17_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B34_IB11_B34_E6extendB2B_E0ENvMs_NtB8_3impNtB46_11TokenStream13unwrap_stableEECs1SHOYL7dTh6_5quote: %streams"}
!256 = distinct !{!256, !"_RINvXsb_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBI_E6extendINtNtNtB17_8adapters3map3MapIB25_INtNtNtB17_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B34_IB11_B34_E6extendB2B_E0ENvMs_NtB8_3impNtB46_11TokenStream13unwrap_stableEECs1SHOYL7dTh6_5quote"}
!257 = !{!255, !228}
!258 = !{!259}
!259 = distinct !{!259, !260, !"_RNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB3_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1o_B1m_INtNtNtB9_6traits7collect6ExtendB1m_E6extendBU_E0ENvMs_NtB1o_3impNtB3d_11TokenStream13unwrap_stableENtNtB2p_8iterator8Iterator7flattenCs1SHOYL7dTh6_5quote: %_0"}
!260 = distinct !{!260, !"_RNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB3_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1o_B1m_INtNtNtB9_6traits7collect6ExtendB1m_E6extendBU_E0ENvMs_NtB1o_3impNtB3d_11TokenStream13unwrap_stableENtNtB2p_8iterator8Iterator7flattenCs1SHOYL7dTh6_5quote"}
!261 = !{!262, !263, !255, !225, !228}
!262 = distinct !{!262, !260, !"_RNvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapIB3_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1o_B1m_INtNtNtB9_6traits7collect6ExtendB1m_E6extendBU_E0ENvMs_NtB1o_3impNtB3d_11TokenStream13unwrap_stableENtNtB2p_8iterator8Iterator7flattenCs1SHOYL7dTh6_5quote: %self"}
!263 = distinct !{!263, !256, !"_RINvXsb_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendBI_E6extendINtNtNtB17_8adapters3map3MapIB25_INtNtNtB17_7sources4once4OnceNtB8_11TokenStreamENCINvXs5_B8_B34_IB11_B34_E6extendB2B_E0ENvMs_NtB8_3impNtB46_11TokenStream13unwrap_stableEECs1SHOYL7dTh6_5quote: %self"}
!264 = !{!265}
!265 = distinct !{!265, !266, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1V_3map3MapIB2N_INtNtNtB1X_7sources4once4OnceNtBT_11TokenStreamENCINvXs5_BT_B3B_INtNtNtB1X_6traits7collect6ExtendB3B_E6extendB38_E0ENvMs_NtBT_3impNtB55_11TokenStream13unwrap_stableEEE11spec_extendCs1SHOYL7dTh6_5quote: %self"}
!266 = distinct !{!266, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1V_3map3MapIB2N_INtNtNtB1X_7sources4once4OnceNtBT_11TokenStreamENCINvXs5_BT_B3B_INtNtNtB1X_6traits7collect6ExtendB3B_E6extendB38_E0ENvMs_NtBT_3impNtB55_11TokenStream13unwrap_stableEEE11spec_extendCs1SHOYL7dTh6_5quote"}
!267 = !{!268}
!268 = distinct !{!268, !266, !"_RNvXNtNtCsdJPVW0sQgAG_5alloc3vec11spec_extendINtB4_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEINtB2_10SpecExtendBR_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1V_3map3MapIB2N_INtNtNtB1X_7sources4once4OnceNtBT_11TokenStreamENCINvXs5_BT_B3B_INtNtNtB1X_6traits7collect6ExtendB3B_E6extendB38_E0ENvMs_NtBT_3impNtB55_11TokenStream13unwrap_stableEEE11spec_extendCs1SHOYL7dTh6_5quote: %iter"}
!269 = !{!270}
!270 = distinct !{!270, !271, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote: %self"}
!271 = distinct !{!271, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote"}
!272 = !{!273}
!273 = distinct !{!273, !271, !"_RINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB6_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeE16extend_desugaredINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten7FlattenINtNtB1H_3map3MapIB2z_INtNtNtB1J_7sources4once4OnceNtBI_11TokenStreamENCINvXs5_BI_B3n_INtNtNtB1J_6traits7collect6ExtendB3n_E6extendB2U_E0ENvMs_NtBI_3impNtB4R_11TokenStream13unwrap_stableEEECs1SHOYL7dTh6_5quote: %iterator"}
!274 = !{!275}
!275 = distinct !{!275, !276, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!276 = distinct !{!276, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!277 = !{!278}
!278 = distinct !{!278, !279, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!279 = distinct !{!279, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!280 = !{!278, !275, !273, !268}
!281 = !{!282, !283, !270, !265, !263, !255, !225, !228}
!282 = distinct !{!282, !279, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %elt"}
!283 = distinct !{!283, !276, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!284 = !{!285}
!285 = distinct !{!285, !286, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %opt:Peel0"}
!286 = distinct !{!286, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote"}
!287 = !{!288}
!288 = distinct !{!288, !289, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote: argument 1:Peel0"}
!289 = distinct !{!289, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote"}
!290 = !{!291}
!291 = distinct !{!291, !292, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self:Peel0"}
!292 = distinct !{!292, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!293 = !{!294}
!294 = distinct !{!294, !295, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self:Peel0"}
!295 = distinct !{!295, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!296 = !{!294, !291, !288, !285, !278, !275, !273, !268}
!297 = !{!298, !299, !300, !301, !282, !283, !270, !265, !263, !255, !225, !228}
!298 = distinct !{!298, !295, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!299 = distinct !{!299, !292, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!300 = distinct !{!300, !289, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote: %_0"}
!301 = distinct !{!301, !286, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %_0:Peel0"}
!302 = !{!294, !291, !288, !301, !285, !282, !278, !283, !275, !270, !273, !265, !268, !255, !228}
!303 = !{!304}
!304 = distinct !{!304, !305, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!305 = distinct !{!305, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!306 = !{!307}
!307 = distinct !{!307, !308, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!308 = distinct !{!308, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!309 = !{!301, !282, !283, !270, !265}
!310 = !{!311}
!311 = distinct !{!311, !312, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!312 = distinct !{!312, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!313 = !{!311, !307, !304}
!314 = !{!301, !282, !283, !270, !265, !263, !255, !225, !228}
!315 = !{!316}
!316 = distinct !{!316, !317, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote: %_1"}
!317 = distinct !{!317, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro29TokenTreeECs1SHOYL7dTh6_5quote"}
!318 = !{!311, !307, !304, !301, !282, !283, !270, !265, !255, !228}
!319 = !{!320, !316, !311, !307, !304, !301, !282, !283, !270, !265, !255, !228}
!320 = distinct !{!320, !321, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote: %_1"}
!321 = distinct !{!321, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote"}
!322 = !{!323, !325, !316}
!323 = distinct !{!323, !324, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote: %_1"}
!324 = distinct !{!324, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote"}
!325 = distinct !{!325, !326, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote: %_1"}
!326 = distinct !{!326, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote"}
!327 = !{!328, !330, !332, !323, !325, !316}
!328 = distinct !{!328, !329, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!329 = distinct !{!329, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!330 = distinct !{!330, !331, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!331 = distinct !{!331, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!332 = distinct !{!332, !333, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!333 = distinct !{!333, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!334 = !{!335}
!335 = distinct !{!335, !336, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!336 = distinct !{!336, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!337 = !{!311, !307, !304, !301, !282, !283, !270, !265}
!338 = !{!339}
!339 = distinct !{!339, !340, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!340 = distinct !{!340, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!341 = !{!342}
!342 = distinct !{!342, !343, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!343 = distinct !{!343, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!344 = !{!342, !339, !335, !345, !347, !323, !325, !316}
!345 = distinct !{!345, !346, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!346 = distinct !{!346, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote"}
!347 = distinct !{!347, !348, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote: %_1"}
!348 = distinct !{!348, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote"}
!349 = !{!342, !339, !335, !311, !307, !304, !301, !282, !283, !270, !265, !255, !228}
!350 = !{!351}
!351 = distinct !{!351, !352, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!352 = distinct !{!352, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!353 = !{!354}
!354 = distinct !{!354, !355, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!355 = distinct !{!355, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!356 = !{!357}
!357 = distinct !{!357, !358, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!358 = distinct !{!358, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!359 = !{!357, !354, !351, !345, !347, !323, !325, !316}
!360 = !{!357, !354, !351, !311, !307, !304, !301, !282, !283, !270, !265, !255, !228}
!361 = !{!362}
!362 = distinct !{!362, !363, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25IdentECs1SHOYL7dTh6_5quote: %_1"}
!363 = distinct !{!363, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25IdentECs1SHOYL7dTh6_5quote"}
!364 = !{!365}
!365 = distinct !{!365, !366, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5IdentECs1SHOYL7dTh6_5quote: %_1"}
!366 = distinct !{!366, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5IdentECs1SHOYL7dTh6_5quote"}
!367 = !{!365, !362, !316}
!368 = !{!365, !362, !316, !311, !307, !304, !301, !282, !283, !270, !265, !255, !228}
!369 = !{!285, !278, !275, !273, !268}
!370 = !{!371}
!371 = distinct !{!371, !372, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4fuseINtB5_4FuseINtNtB7_3map3MapIBZ_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1M_B1K_INtNtNtB9_6traits7collect6ExtendB1K_E6extendB1i_E0ENvMs_NtB1M_3impNtB3C_11TokenStream13unwrap_stableEEINtB5_8FuseImplBY_E4nextCs1SHOYL7dTh6_5quote: %self:Peel0"}
!372 = distinct !{!372, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4fuseINtB5_4FuseINtNtB7_3map3MapIBZ_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1M_B1K_INtNtNtB9_6traits7collect6ExtendB1K_E6extendB1i_E0ENvMs_NtB1M_3impNtB3C_11TokenStream13unwrap_stableEEINtB5_8FuseImplBY_E4nextCs1SHOYL7dTh6_5quote"}
!373 = !{!374}
!374 = distinct !{!374, !375, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapIBN_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1u_B1s_INtNtNtB9_6traits7collect6ExtendB1s_E6extendB10_E0ENvMs_NtB1u_3impNtB3k_11TokenStream13unwrap_stableENtNtB2v_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self:Peel0"}
!375 = distinct !{!375, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapIBN_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1u_B1s_INtNtNtB9_6traits7collect6ExtendB1s_E6extendB10_E0ENvMs_NtB1u_3impNtB3k_11TokenStream13unwrap_stableENtNtB2v_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!376 = !{!377, !379, !374, !371, !278, !275, !273, !268}
!377 = distinct !{!377, !378, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtB6_6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!378 = distinct !{!378, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtB6_6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!379 = distinct !{!379, !380, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1q_B1o_INtNtNtB9_6traits7collect6ExtendB1o_E6extendBW_E0ENtNtB2r_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!380 = distinct !{!380, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1q_B1o_INtNtNtB9_6traits7collect6ExtendB1o_E6extendBW_E0ENtNtB2r_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!381 = !{!382, !383, !282, !283, !270, !265, !263, !255, !225, !228}
!382 = distinct !{!382, !378, !"_RNvXNtNtNtCsjMrxcFdYDNN_4core4iter7sources4onceINtB2_4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENtNtNtB6_6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!383 = distinct !{!383, !380, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapINtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1q_B1o_INtNtNtB9_6traits7collect6ExtendB1o_E6extendBW_E0ENtNtB2r_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!384 = !{!374, !371, !282, !278, !283, !275, !270, !273, !265, !268, !263, !255, !225, !228}
!385 = !{!270, !273, !265, !268, !255, !228}
!386 = !{!282, !278, !283, !275, !270, !273, !265, !268, !263, !255, !225, !228}
!387 = !{!388, !278, !275, !273, !268}
!388 = distinct !{!388, !286, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %opt"}
!389 = !{!390, !282, !283, !270, !265, !263, !255, !225, !228}
!390 = distinct !{!390, !286, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %_0"}
!391 = !{!388}
!392 = !{!393}
!393 = distinct !{!393, !289, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote: argument 1"}
!394 = !{!395}
!395 = distinct !{!395, !292, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!396 = !{!397}
!397 = distinct !{!397, !295, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!398 = !{!397, !395, !393, !388, !278, !275, !273, !268}
!399 = !{!298, !299, !300, !390, !282, !283, !270, !265, !263, !255, !225, !228}
!400 = !{!397, !395, !393, !390, !388, !282, !278, !283, !275, !270, !273, !265, !268, !255, !228}
!401 = !{!402}
!402 = distinct !{!402, !403, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!403 = distinct !{!403, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!404 = !{!405}
!405 = distinct !{!405, !406, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!406 = distinct !{!406, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!407 = !{!390, !282, !283, !270, !265}
!408 = !{!409}
!409 = distinct !{!409, !410, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!410 = distinct !{!410, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!411 = !{!409, !405, !402}
!412 = !{!409, !405, !402, !390, !282, !283, !270, !265, !255, !228}
!413 = !{!282, !283, !270, !265, !255, !228}
!414 = !{!278, !275, !270, !273, !265, !268, !263, !255, !225, !228}
!415 = !{!377, !379, !416, !417, !278, !275, !273, !268}
!416 = distinct !{!416, !375, !"_RNvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB5_3MapIBN_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1u_B1s_INtNtNtB9_6traits7collect6ExtendB1s_E6extendB10_E0ENvMs_NtB1u_3impNtB3k_11TokenStream13unwrap_stableENtNtB2v_8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!417 = distinct !{!417, !372, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters4fuseINtB5_4FuseINtNtB7_3map3MapIBZ_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1M_B1K_INtNtNtB9_6traits7collect6ExtendB1K_E6extendB1i_E0ENvMs_NtB1M_3impNtB3C_11TokenStream13unwrap_stableEEINtB5_8FuseImplBY_E4nextCs1SHOYL7dTh6_5quote: %self"}
!418 = !{!419}
!419 = distinct !{!419, !420, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %opt"}
!420 = distinct !{!420, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote"}
!421 = !{!419, !278, !275, !273, !268}
!422 = !{!423, !282, !283, !270, !265, !263, !255, !225, !228}
!423 = distinct !{!423, !420, !"_RINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flatten17and_then_or_clearINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtB1b_9TokenTreeEB1X_NvYB16_NtNtNtB6_6traits8iterator8Iterator4nextECs1SHOYL7dTh6_5quote: %_0"}
!424 = !{!425}
!425 = distinct !{!425, !426, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote: argument 1"}
!426 = distinct !{!426, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote"}
!427 = !{!428}
!428 = distinct !{!428, !429, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!429 = distinct !{!429, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!430 = !{!431}
!431 = distinct !{!431, !432, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %self"}
!432 = distinct !{!432, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote"}
!433 = !{!431, !428, !425, !419, !278, !275, !273, !268}
!434 = !{!435, !436, !437, !423, !282, !283, !270, !265, !263, !255, !225, !228}
!435 = distinct !{!435, !432, !"_RNvXs4_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!436 = distinct !{!436, !429, !"_RNvXs3_NtCs8M6BBVNvC7a_11proc_macro25rcvecINtB5_13RcVecIntoIterNtB7_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextCs1SHOYL7dTh6_5quote: %_0"}
!437 = distinct !{!437, !426, !"_RNvYNvYINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBa_9TokenTreeENtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4nextINtNtNtB1k_3ops8function6FnOnceTQB5_EE9call_onceCs1SHOYL7dTh6_5quote: %_0"}
!438 = !{!431, !428, !425, !423, !419, !282, !278, !283, !275, !270, !273, !265, !268, !255, !228}
!439 = !{!440}
!440 = distinct !{!440, !441, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!441 = distinct !{!441, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec13RcVecIntoIterNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!442 = !{!443}
!443 = distinct !{!443, !444, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!444 = distinct !{!444, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtCsdJPVW0sQgAG_5alloc3vec9into_iter8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!445 = !{!423, !282, !283, !270, !265}
!446 = !{!447}
!447 = distinct !{!447, !448, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!448 = distinct !{!448, !"_RNvXse_NtNtCsdJPVW0sQgAG_5alloc3vec9into_iterINtB5_8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!449 = !{!447, !443, !440}
!450 = !{!447, !443, !440, !423, !282, !283, !270, !265, !255, !228}
!451 = !{!270, !265, !255, !228}
!452 = !{!270, !273, !265, !268, !263, !255, !225, !228}
!453 = !{!270, !265}
!454 = !{!273, !268, !255, !228}
!455 = !{!456, !458, !273, !268}
!456 = distinct !{!456, !457, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote: %self"}
!457 = distinct !{!457, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote"}
!458 = distinct !{!458, !459, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote: %self"}
!459 = distinct !{!459, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote"}
!460 = !{!461, !462, !270, !265, !263, !255, !225, !228}
!461 = distinct !{!461, !457, !"_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_13FlattenCompatINtNtB7_3map3MapIB1c_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B20_B1Y_INtNtNtB9_6traits7collect6ExtendB1Y_E6extendB1w_E0ENvMs_NtB20_3impNtB3Q_11TokenStream13unwrap_stableEINtNtB20_5rcvec13RcVecIntoIterNtB20_9TokenTreeEENtNtB31_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote: %_0"}
!462 = distinct !{!462, !459, !"_RNvXs9_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters7flattenINtB5_7FlattenINtNtB7_3map3MapIB15_INtNtNtB9_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1T_B1R_INtNtNtB9_6traits7collect6ExtendB1R_E6extendB1p_E0ENvMs_NtB1T_3impNtB3J_11TokenStream13unwrap_stableEENtNtB2U_8iterator8Iterator9size_hintCs1SHOYL7dTh6_5quote: %_0"}
!463 = !{!464, !466, !468, !470, !472, !474, !255}
!464 = distinct !{!464, !465, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!465 = distinct !{!465, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!466 = distinct !{!466, !467, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!467 = distinct !{!467, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!468 = distinct !{!468, !469, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!469 = distinct !{!469, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!470 = distinct !{!470, !471, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!471 = distinct !{!471, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!472 = distinct !{!472, !473, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapINtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1I_B1G_INtNtNtBN_6traits7collect6ExtendB1G_E6extendB1e_E0EECs1SHOYL7dTh6_5quote: %_1"}
!473 = distinct !{!473, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapINtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1I_B1G_INtNtNtBN_6traits7collect6ExtendB1G_E6extendB1e_E0EECs1SHOYL7dTh6_5quote"}
!474 = distinct !{!474, !475, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapIBH_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1M_B1K_INtNtNtBN_6traits7collect6ExtendB1K_E6extendB1i_E0ENvMs_NtB1M_3impNtB3C_11TokenStream13unwrap_stableEECs1SHOYL7dTh6_5quote: %_1"}
!475 = distinct !{!475, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapIBH_INtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1M_B1K_INtNtNtBN_6traits7collect6ExtendB1K_E6extendB1i_E0ENvMs_NtB1M_3impNtB3C_11TokenStream13unwrap_stableEECs1SHOYL7dTh6_5quote"}
!476 = !{!263, !225, !228}
!477 = !{!478, !480, !482, !484, !486, !228}
!478 = distinct !{!478, !479, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!479 = distinct !{!479, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!480 = distinct !{!480, !481, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!481 = distinct !{!481, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!482 = distinct !{!482, !483, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!483 = distinct !{!483, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!484 = distinct !{!484, !485, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!485 = distinct !{!485, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamEECs1SHOYL7dTh6_5quote"}
!486 = distinct !{!486, !487, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapINtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1I_B1G_INtNtNtBN_6traits7collect6ExtendB1G_E6extendB1e_E0EECs1SHOYL7dTh6_5quote: %_1"}
!487 = distinct !{!487, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter8adapters3map3MapINtNtNtBN_7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro211TokenStreamENCINvXs5_B1I_B1G_INtNtNtBN_6traits7collect6ExtendB1G_E6extendB1e_E0EECs1SHOYL7dTh6_5quote"}
!488 = !{!489}
!489 = distinct !{!489, !490, !"_RINvXsa_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB17_7sources4once4OnceB1T_EECs1SHOYL7dTh6_5quote: %tokens"}
!490 = distinct !{!490, !"_RINvXsa_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB17_7sources4once4OnceB1T_EECs1SHOYL7dTh6_5quote"}
!491 = !{!492}
!492 = distinct !{!492, !490, !"_RINvXsa_NtCs8M6BBVNvC7a_11proc_macro28fallbackNtB6_11TokenStreamINtNtNtNtCsjMrxcFdYDNN_4core4iter6traits7collect6ExtendNtB8_9TokenTreeE6extendINtNtNtB17_7sources4once4OnceB1T_EECs1SHOYL7dTh6_5quote: %self"}
!493 = !{!494, !496, !498, !499, !492, !489}
!494 = distinct !{!494, !495, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNCINvXsa_NtB1h_8fallbackNtB21_11TokenStreamINtNtBa_7collect6ExtendB1f_E6extendINtNtNtBc_7sources4once4OnceB1f_EE0E0Cs1SHOYL7dTh6_5quote: %item"}
!495 = distinct !{!495, !"_RNCINvNvNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator8for_each4callNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNCINvXsa_NtB1h_8fallbackNtB21_11TokenStreamINtNtBa_7collect6ExtendB1f_E6extendINtNtNtBc_7sources4once4OnceB1f_EE0E0Cs1SHOYL7dTh6_5quote"}
!496 = distinct !{!496, !497, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB1w_8for_each4callBS_NCINvXsa_NtBU_8fallbackNtB2L_11TokenStreamINtNtB1A_7collect6ExtendBS_E6extendB3_E0E0ECs1SHOYL7dTh6_5quote: %self"}
!497 = distinct !{!497, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB1w_8for_each4callBS_NCINvXsa_NtBU_8fallbackNtB2L_11TokenStreamINtNtB1A_7collect6ExtendBS_E6extendB3_E0E0ECs1SHOYL7dTh6_5quote"}
!498 = distinct !{!498, !497, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB1w_8for_each4callBS_NCINvXsa_NtBU_8fallbackNtB2L_11TokenStreamINtNtB1A_7collect6ExtendBS_E6extendB3_E0E0ECs1SHOYL7dTh6_5quote: argument 1"}
!499 = distinct !{!499, !500, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsa_NtBU_8fallbackNtB2m_11TokenStreamINtNtB1A_7collect6ExtendBS_E6extendB3_E0ECs1SHOYL7dTh6_5quote: %self"}
!500 = distinct !{!500, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvXsa_NtBU_8fallbackNtB2m_11TokenStreamINtNtB1A_7collect6ExtendBS_E6extendB3_E0ECs1SHOYL7dTh6_5quote"}
!501 = !{!496, !498, !499, !492, !489}
!502 = !{i32 0, i32 5}
!503 = !{!504, !506, !508, !510, !489}
!504 = distinct !{!504, !505, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!505 = distinct !{!505, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!506 = distinct !{!506, !507, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!507 = distinct !{!507, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option4ItemNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!508 = distinct !{!508, !509, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!509 = distinct !{!509, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option8IntoIterNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!510 = distinct !{!510, !511, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!511 = distinct !{!511, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtNtNtB4_4iter7sources4once4OnceNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEECs1SHOYL7dTh6_5quote"}
!512 = !{!513}
!513 = distinct !{!513, !514, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeE8push_mutCs1SHOYL7dTh6_5quote: %self"}
!514 = distinct !{!514, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeE8push_mutCs1SHOYL7dTh6_5quote"}
!515 = !{!516}
!516 = distinct !{!516, !514, !"_RNvMsF_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeE8push_mutCs1SHOYL7dTh6_5quote: %value"}
!517 = !{!518, !516}
!518 = distinct !{!518, !519, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote: %_1"}
!519 = distinct !{!519, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro9TokenTreeECs1SHOYL7dTh6_5quote"}
!520 = !{!521, !523, !525, !518, !516}
!521 = distinct !{!521, !522, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!522 = distinct !{!522, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!523 = distinct !{!523, !524, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!524 = distinct !{!524, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!525 = distinct !{!525, !526, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!526 = distinct !{!526, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!527 = !{!528}
!528 = distinct !{!528, !529, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1SHOYL7dTh6_5quote: %self"}
!529 = distinct !{!529, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs1SHOYL7dTh6_5quote"}
!530 = !{!"branch_weights", i32 2002, i32 2000}
!531 = !{!532}
!532 = distinct !{!532, !533, !"_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote: %_0"}
!533 = distinct !{!533, !"_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote"}
!534 = !{!535}
!535 = distinct !{!535, !536, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!536 = distinct !{!536, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!537 = !{!538, !540, !535}
!538 = distinct !{!538, !539, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote: %_0"}
!539 = distinct !{!539, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote"}
!540 = distinct !{!540, !539, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote: %s.0"}
!541 = !{!542, !538, !540, !535}
!542 = distinct !{!542, !543, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1SHOYL7dTh6_5quote: %_0"}
!543 = distinct !{!543, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1SHOYL7dTh6_5quote"}
!544 = !{!538, !535}
!545 = !{!540}
!546 = !{!547, !549}
!547 = distinct !{!547, !548, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!548 = distinct !{!548, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!549 = distinct !{!549, !550, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!550 = distinct !{!550, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!551 = !{!552}
!552 = distinct !{!552, !553, !"_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote: %_0"}
!553 = distinct !{!553, !"_RNvMs1N_NtCsdJPVW0sQgAG_5alloc2rcINtB6_14UniqueRcUninitINtNtB8_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeENtNtB8_5alloc6GlobalE3newCs1SHOYL7dTh6_5quote"}
!554 = !{!555, !557}
!555 = distinct !{!555, !556, !"_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree4span: %self"}
!556 = distinct !{!556, !"_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree4span"}
!557 = distinct !{!557, !558, !"_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_: %tt"}
!558 = distinct !{!558, !"_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_"}
!559 = !{i32 1, i32 0}
!560 = !{!561}
!561 = distinct !{!561, !562, !"_RINvYNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB13_6option6OptionNtB7_4SpanENCINvNtNtB11_8adapters3map8map_foldNtB7_9TokenTreeB2i_B1V_NCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0NCB3r_s_0E0EB3v_: %self"}
!562 = distinct !{!562, !"_RINvYNtNtCs8M6BBVNvC7a_11proc_macro212token_stream8IntoIterNtNtNtNtCsjMrxcFdYDNN_4core4iter6traits8iterator8Iterator4foldINtNtB13_6option6OptionNtB7_4SpanENCINvNtNtB11_8adapters3map8map_foldNtB7_9TokenTreeB2i_B1V_NCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0NCB3r_s_0E0EB3v_"}
!563 = !{!564, !561}
!564 = distinct !{!564, !565, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtBW_4SpanINtNtBa_6option6OptionB1x_ENCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0NCB2a_s_0E0B2e_: %elt"}
!565 = distinct !{!565, !"_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldNtCs8M6BBVNvC7a_11proc_macro29TokenTreeNtBW_4SpanINtNtBa_6option6OptionB1x_ENCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0NCB2a_s_0E0B2e_"}
!566 = !{!567, !569}
!567 = distinct !{!567, !568, !"_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree4span: %self"}
!568 = distinct !{!568, !"_RNvMsg_Cs8M6BBVNvC7a_11proc_macro2NtB5_9TokenTree4span"}
!569 = distinct !{!569, !570, !"_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_: %tt"}
!570 = distinct !{!570, !"_RNCNvNtCs1SHOYL7dTh6_5quote7spanned10join_spans0B5_"}
!571 = distinct !{!571, !572}
!572 = !{!"llvm.loop.peeled.count", i32 1}
!573 = !{!574, !576}
!574 = distinct !{!574, !575, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!575 = distinct !{!575, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!576 = distinct !{!576, !575, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!577 = !{!576}
!578 = !{!579, !581}
!579 = distinct !{!579, !580, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!580 = distinct !{!580, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!581 = distinct !{!581, !580, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!582 = !{!581}
!583 = !{!584, !586}
!584 = distinct !{!584, !585, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!585 = distinct !{!585, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!586 = distinct !{!586, !585, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!587 = !{!586}
!588 = !{!589, !591}
!589 = distinct !{!589, !590, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!590 = distinct !{!590, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!591 = distinct !{!591, !590, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!592 = !{!591}
!593 = !{!594, !596}
!594 = distinct !{!594, !595, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!595 = distinct !{!595, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!596 = distinct !{!596, !595, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!597 = !{!596}
!598 = !{!599, !601}
!599 = distinct !{!599, !600, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %self"}
!600 = distinct !{!600, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_"}
!601 = distinct !{!601, !600, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %token"}
!602 = !{!599}
!603 = !{!601}
!604 = !{!605, !607}
!605 = distinct !{!605, !606, !"_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned: %tokens"}
!606 = distinct !{!606, !"_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned"}
!607 = distinct !{!607, !606, !"_RNvNtCs1SHOYL7dTh6_5quote9___private18push_ident_spanned: %s.0"}
!608 = !{!609, !605}
!609 = distinct !{!609, !610, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw: %_0"}
!610 = distinct !{!610, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw"}
!611 = !{!605}
!612 = !{!613, !615, !605, !607}
!613 = distinct !{!613, !614, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!614 = distinct !{!614, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!615 = distinct !{!615, !614, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!616 = !{!613, !605, !607}
!617 = !{!615}
!618 = !{!619, !621}
!619 = distinct !{!619, !620, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!620 = distinct !{!620, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!621 = distinct !{!621, !620, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!622 = !{!621}
!623 = !{!624, !626}
!624 = distinct !{!624, !625, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!625 = distinct !{!625, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!626 = distinct !{!626, !625, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!627 = !{!626}
!628 = !{!629, !631}
!629 = distinct !{!629, !630, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!630 = distinct !{!630, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!631 = distinct !{!631, !630, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!632 = !{!631}
!633 = !{!634, !636}
!634 = distinct !{!634, !635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!635 = distinct !{!635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!636 = distinct !{!636, !635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!637 = !{!636}
!638 = !{!639, !641}
!639 = distinct !{!639, !640, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!640 = distinct !{!640, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!641 = distinct !{!641, !640, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!642 = !{!641}
!643 = !{!644, !646}
!644 = distinct !{!644, !645, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!645 = distinct !{!645, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!646 = distinct !{!646, !645, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!647 = !{!646}
!648 = !{!649, !651}
!649 = distinct !{!649, !650, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!650 = distinct !{!650, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!651 = distinct !{!651, !650, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!652 = !{!651}
!653 = !{!654, !656}
!654 = distinct !{!654, !655, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!655 = distinct !{!655, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!656 = distinct !{!656, !655, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!657 = !{!656}
!658 = !{!659, !661}
!659 = distinct !{!659, !660, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!660 = distinct !{!660, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!661 = distinct !{!661, !660, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!662 = !{!661}
!663 = !{!664, !666}
!664 = distinct !{!664, !665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!665 = distinct !{!665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!666 = distinct !{!666, !665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!667 = !{!666}
!668 = !{!669, !671}
!669 = distinct !{!669, !670, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!670 = distinct !{!670, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!671 = distinct !{!671, !670, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!672 = !{!671}
!673 = !{!674, !676}
!674 = distinct !{!674, !675, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!675 = distinct !{!675, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!676 = distinct !{!676, !675, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!677 = !{!676}
!678 = !{!679, !681}
!679 = distinct !{!679, !680, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!680 = distinct !{!680, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!681 = distinct !{!681, !680, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!682 = !{!681}
!683 = !{!684, !686}
!684 = distinct !{!684, !685, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!685 = distinct !{!685, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!686 = distinct !{!686, !685, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!687 = !{!686}
!688 = !{!689, !691}
!689 = distinct !{!689, !690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!690 = distinct !{!690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!691 = distinct !{!691, !690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!692 = !{!691}
!693 = !{!694, !696}
!694 = distinct !{!694, !695, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!695 = distinct !{!695, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!696 = distinct !{!696, !695, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!697 = !{!696}
!698 = !{!699, !701}
!699 = distinct !{!699, !700, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!700 = distinct !{!700, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!701 = distinct !{!701, !700, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!702 = !{!701}
!703 = !{!704, !706}
!704 = distinct !{!704, !705, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!705 = distinct !{!705, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!706 = distinct !{!706, !705, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!707 = !{!706}
!708 = !{!709, !711}
!709 = distinct !{!709, !710, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!710 = distinct !{!710, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!711 = distinct !{!711, !710, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!712 = !{!711}
!713 = !{!714, !716}
!714 = distinct !{!714, !715, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!715 = distinct !{!715, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!716 = distinct !{!716, !715, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!717 = !{!716}
!718 = !{!719, !721}
!719 = distinct !{!719, !720, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!720 = distinct !{!720, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!721 = distinct !{!721, !720, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!722 = !{!721}
!723 = !{!724, !726}
!724 = distinct !{!724, !725, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!725 = distinct !{!725, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!726 = distinct !{!726, !725, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!727 = !{!726}
!728 = !{!729, !731}
!729 = distinct !{!729, !730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!730 = distinct !{!730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!731 = distinct !{!731, !730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!732 = !{!731}
!733 = !{!734, !736}
!734 = distinct !{!734, !735, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!735 = distinct !{!735, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!736 = distinct !{!736, !735, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!737 = !{!736}
!738 = !{!739, !741}
!739 = distinct !{!739, !740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!740 = distinct !{!740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!741 = distinct !{!741, !740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!742 = !{!741}
!743 = !{!744, !746}
!744 = distinct !{!744, !745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!745 = distinct !{!745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!746 = distinct !{!746, !745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!747 = !{!746}
!748 = !{!749, !751}
!749 = distinct !{!749, !750, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!750 = distinct !{!750, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!751 = distinct !{!751, !750, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!752 = !{!751}
!753 = !{!754, !756}
!754 = distinct !{!754, !755, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!755 = distinct !{!755, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!756 = distinct !{!756, !755, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!757 = !{!756}
!758 = !{!759, !761}
!759 = distinct !{!759, !760, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!760 = distinct !{!760, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!761 = distinct !{!761, !760, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!762 = !{!761}
!763 = !{!764, !766}
!764 = distinct !{!764, !765, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!765 = distinct !{!765, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!766 = distinct !{!766, !765, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!767 = !{!766}
!768 = !{!769, !771}
!769 = distinct !{!769, !770, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!770 = distinct !{!770, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!771 = distinct !{!771, !770, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!772 = !{!771}
!773 = !{!774}
!774 = distinct !{!774, !775, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: %t"}
!775 = distinct !{!775, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote"}
!776 = !{!777}
!777 = distinct !{!777, !775, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: %self"}
!778 = !{!774, !779}
!779 = distinct !{!779, !775, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: argument 2"}
!780 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!781 = !{!774, !777, !779}
!782 = !{!774, !777}
!783 = !{!779}
!784 = !{!785, !787}
!785 = distinct !{!785, !786, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!786 = distinct !{!786, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!787 = distinct !{!787, !786, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!788 = !{!785}
!789 = !{!790, !792}
!790 = distinct !{!790, !791, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!791 = distinct !{!791, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!792 = distinct !{!792, !791, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!793 = !{!792}
!794 = !{!795, !797}
!795 = distinct !{!795, !796, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!796 = distinct !{!796, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!797 = distinct !{!797, !796, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!798 = !{!797}
!799 = !{!800, !802}
!800 = distinct !{!800, !801, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!801 = distinct !{!801, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!802 = distinct !{!802, !801, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!803 = !{!800}
!804 = !{!802}
!805 = !{!806}
!806 = distinct !{!806, !807, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!807 = distinct !{!807, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!808 = !{!809, !811}
!809 = distinct !{!809, !810, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!810 = distinct !{!810, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!811 = distinct !{!811, !810, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!812 = !{!809}
!813 = !{!811}
!814 = !{!815, !817}
!815 = distinct !{!815, !816, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!816 = distinct !{!816, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!817 = distinct !{!817, !816, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!818 = !{!817}
!819 = !{!820, !822}
!820 = distinct !{!820, !821, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!821 = distinct !{!821, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!822 = distinct !{!822, !821, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!823 = !{!822}
!824 = !{!825, !827}
!825 = distinct !{!825, !826, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!826 = distinct !{!826, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!827 = distinct !{!827, !826, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!828 = !{!827}
!829 = !{!830, !832}
!830 = distinct !{!830, !831, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!831 = distinct !{!831, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!832 = distinct !{!832, !831, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!833 = !{!830}
!834 = !{!832}
!835 = !{!836, !838}
!836 = distinct !{!836, !837, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!837 = distinct !{!837, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!838 = distinct !{!838, !837, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!839 = !{!838}
!840 = !{!841, !843}
!841 = distinct !{!841, !842, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!842 = distinct !{!842, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!843 = distinct !{!843, !842, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!844 = !{!843}
!845 = !{!846, !848}
!846 = distinct !{!846, !847, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!847 = distinct !{!847, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!848 = distinct !{!848, !847, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!849 = !{!848}
!850 = !{!851, !853}
!851 = distinct !{!851, !852, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!852 = distinct !{!852, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!853 = distinct !{!853, !852, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!854 = !{!851}
!855 = !{!853}
!856 = !{!857, !859}
!857 = distinct !{!857, !858, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!858 = distinct !{!858, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!859 = distinct !{!859, !858, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!860 = !{!857}
!861 = !{!859}
!862 = !{!863, !865}
!863 = distinct !{!863, !864, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!864 = distinct !{!864, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!865 = distinct !{!865, !864, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!866 = !{!863}
!867 = !{!865}
!868 = !{!869, !871}
!869 = distinct !{!869, !870, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!870 = distinct !{!870, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!871 = distinct !{!871, !870, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!872 = !{!869}
!873 = !{!871}
!874 = !{!875, !877}
!875 = distinct !{!875, !876, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!876 = distinct !{!876, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!877 = distinct !{!877, !876, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!878 = !{!875}
!879 = !{!877}
!880 = !{!881, !883}
!881 = distinct !{!881, !882, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!882 = distinct !{!882, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!883 = distinct !{!883, !882, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!884 = !{!881}
!885 = !{!883}
!886 = !{!887, !889}
!887 = distinct !{!887, !888, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!888 = distinct !{!888, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!889 = distinct !{!889, !888, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!890 = !{!887}
!891 = !{!889}
!892 = !{!893, !895}
!893 = distinct !{!893, !894, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!894 = distinct !{!894, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!895 = distinct !{!895, !894, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!896 = !{!893}
!897 = !{!895}
!898 = !{!899, !901}
!899 = distinct !{!899, !900, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!900 = distinct !{!900, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!901 = distinct !{!901, !900, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!902 = !{!899}
!903 = !{!901}
!904 = !{!905, !907}
!905 = distinct !{!905, !906, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!906 = distinct !{!906, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!907 = distinct !{!907, !906, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!908 = !{!905}
!909 = !{!907}
!910 = !{!911}
!911 = distinct !{!911, !912, !"_RNvNtCs1SHOYL7dTh6_5quote9___private23push_underscore_spanned: %tokens"}
!912 = distinct !{!912, !"_RNvNtCs1SHOYL7dTh6_5quote9___private23push_underscore_spanned"}
!913 = !{!914, !916, !911}
!914 = distinct !{!914, !915, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!915 = distinct !{!915, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!916 = distinct !{!916, !915, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!917 = !{!914, !911}
!918 = !{!916}
!919 = !{!920, !922}
!920 = distinct !{!920, !921, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!921 = distinct !{!921, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!922 = distinct !{!922, !921, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!923 = !{!920}
!924 = !{!922}
!925 = !{!926, !928}
!926 = distinct !{!926, !927, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!927 = distinct !{!927, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!928 = distinct !{!928, !927, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!929 = !{!926}
!930 = !{!928}
!931 = !{!932, !934}
!932 = distinct !{!932, !933, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!933 = distinct !{!933, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!934 = distinct !{!934, !933, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!935 = !{!932}
!936 = !{!934}
!937 = !{!938, !940}
!938 = distinct !{!938, !939, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!939 = distinct !{!939, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!940 = distinct !{!940, !939, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!941 = !{!938}
!942 = !{!940}
!943 = !{!944, !946}
!944 = distinct !{!944, !945, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!945 = distinct !{!945, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!946 = distinct !{!946, !945, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!947 = !{!944}
!948 = !{!946}
!949 = !{!950, !952}
!950 = distinct !{!950, !951, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!951 = distinct !{!951, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!952 = distinct !{!952, !951, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!953 = !{!950}
!954 = !{!952}
!955 = !{!956, !958}
!956 = distinct !{!956, !957, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!957 = distinct !{!957, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!958 = distinct !{!958, !957, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!959 = !{!956}
!960 = !{!958}
!961 = !{!962, !964}
!962 = distinct !{!962, !963, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!963 = distinct !{!963, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!964 = distinct !{!964, !963, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!965 = !{!962}
!966 = !{!964}
!967 = !{!968, !970}
!968 = distinct !{!968, !969, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!969 = distinct !{!969, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!970 = distinct !{!970, !969, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!971 = !{!968}
!972 = !{!970}
!973 = !{!974, !976}
!974 = distinct !{!974, !975, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!975 = distinct !{!975, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!976 = distinct !{!976, !975, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!977 = !{!974}
!978 = !{!976}
!979 = !{!980, !982}
!980 = distinct !{!980, !981, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!981 = distinct !{!981, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!982 = distinct !{!982, !981, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!983 = !{!980}
!984 = !{!982}
!985 = !{!986, !988}
!986 = distinct !{!986, !987, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!987 = distinct !{!987, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!988 = distinct !{!988, !987, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!989 = !{!986}
!990 = !{!988}
!991 = !{!992, !994}
!992 = distinct !{!992, !993, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!993 = distinct !{!993, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!994 = distinct !{!994, !993, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!995 = !{!992}
!996 = !{!994}
!997 = !{!998, !1000}
!998 = distinct !{!998, !999, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!999 = distinct !{!999, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1000 = distinct !{!1000, !999, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1001 = !{!998}
!1002 = !{!1000}
!1003 = !{!1004, !1006}
!1004 = distinct !{!1004, !1005, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1005 = distinct !{!1005, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1006 = distinct !{!1006, !1005, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1007 = !{!1004}
!1008 = !{!1006}
!1009 = !{!1010, !1012}
!1010 = distinct !{!1010, !1011, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1011 = distinct !{!1011, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1012 = distinct !{!1012, !1011, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1013 = !{!1010}
!1014 = !{!1012}
!1015 = !{!1016, !1018}
!1016 = distinct !{!1016, !1017, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1017 = distinct !{!1017, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1018 = distinct !{!1018, !1017, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1019 = !{!1016}
!1020 = !{!1018}
!1021 = !{!1022, !1024}
!1022 = distinct !{!1022, !1023, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1023 = distinct !{!1023, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1024 = distinct !{!1024, !1023, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1025 = !{!1022}
!1026 = !{!1024}
!1027 = !{i8 0, i8 4}
!1028 = !{!1029, !1031}
!1029 = distinct !{!1029, !1030, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote: %_1"}
!1030 = distinct !{!1030, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro23imp5GroupECs1SHOYL7dTh6_5quote"}
!1031 = distinct !{!1031, !1032, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote: %_1"}
!1032 = distinct !{!1032, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCs8M6BBVNvC7a_11proc_macro25GroupECs1SHOYL7dTh6_5quote"}
!1033 = !{!1034, !1036, !1038, !1029, !1031}
!1034 = distinct !{!1034, !1035, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote: %_1"}
!1035 = distinct !{!1035, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtB4_6option6OptionNtNtNtCsbtZo8rQoR5w_10proc_macro6bridge6client11TokenStreamEECs1SHOYL7dTh6_5quote"}
!1036 = distinct !{!1036, !1037, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote: %_1"}
!1037 = distinct !{!1037, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsbtZo8rQoR5w_10proc_macro6bridge5GroupNtNtBJ_6client11TokenStreamNtB1q_4SpanEECs1SHOYL7dTh6_5quote"}
!1038 = distinct !{!1038, !1039, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote: %_1"}
!1039 = distinct !{!1039, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtCsbtZo8rQoR5w_10proc_macro5GroupECs1SHOYL7dTh6_5quote"}
!1040 = !{!1041}
!1041 = distinct !{!1041, !1042, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!1042 = distinct !{!1042, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!1043 = !{!1044}
!1044 = distinct !{!1044, !1045, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!1045 = distinct !{!1045, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!1046 = !{!1047}
!1047 = distinct !{!1047, !1048, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!1048 = distinct !{!1048, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!1049 = !{!1047, !1044, !1041, !1050, !1052, !1029, !1031}
!1050 = distinct !{!1050, !1051, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote: %_1"}
!1051 = distinct !{!1051, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback11TokenStreamECs1SHOYL7dTh6_5quote"}
!1052 = distinct !{!1052, !1053, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote: %_1"}
!1053 = distinct !{!1053, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCs8M6BBVNvC7a_11proc_macro28fallback5GroupECs1SHOYL7dTh6_5quote"}
!1054 = !{!1047, !1044, !1041}
!1055 = !{!1056}
!1056 = distinct !{!1056, !1057, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote: %_1"}
!1057 = distinct !{!1057, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCs8M6BBVNvC7a_11proc_macro25rcvec5RcVecNtBL_9TokenTreeEECs1SHOYL7dTh6_5quote"}
!1058 = !{!1059}
!1059 = distinct !{!1059, !1060, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote: %_1"}
!1060 = distinct !{!1060, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc2rc2RcINtNtBL_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEEECs1SHOYL7dTh6_5quote"}
!1061 = !{!1062}
!1062 = distinct !{!1062, !1063, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote: %self"}
!1063 = distinct !{!1063, !"_RNvXsx_NtCsdJPVW0sQgAG_5alloc2rcINtB5_2RcINtNtB7_3vec3VecNtCs8M6BBVNvC7a_11proc_macro29TokenTreeEENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs1SHOYL7dTh6_5quote"}
!1064 = !{!1062, !1059, !1056, !1050, !1052, !1029, !1031}
!1065 = !{!1062, !1059, !1056}
!1066 = !{!1067, !1069}
!1067 = distinct !{!1067, !1068, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!1068 = distinct !{!1068, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!1069 = distinct !{!1069, !1068, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!1070 = !{!1067}
!1071 = !{!1072, !1074}
!1072 = distinct !{!1072, !1073, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1073 = distinct !{!1073, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1074 = distinct !{!1074, !1073, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1075 = !{!1072}
!1076 = !{!1074}
!1077 = !{!1078, !1080}
!1078 = distinct !{!1078, !1079, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1079 = distinct !{!1079, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1080 = distinct !{!1080, !1079, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1081 = !{!1078}
!1082 = !{!1080}
!1083 = !{!1084, !1086}
!1084 = distinct !{!1084, !1085, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1085 = distinct !{!1085, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1086 = distinct !{!1086, !1085, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1087 = !{!1084}
!1088 = !{!1086}
!1089 = !{!1090, !1092}
!1090 = distinct !{!1090, !1091, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1091 = distinct !{!1091, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1092 = distinct !{!1092, !1091, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1093 = !{!1090}
!1094 = !{!1092}
!1095 = !{!1096, !1098}
!1096 = distinct !{!1096, !1097, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1097 = distinct !{!1097, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1098 = distinct !{!1098, !1097, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1099 = !{!1096}
!1100 = !{!1098}
!1101 = !{!1102, !1104}
!1102 = distinct !{!1102, !1103, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %self"}
!1103 = distinct !{!1103, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_"}
!1104 = distinct !{!1104, !1103, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %token"}
!1105 = !{!1106}
!1106 = distinct !{!1106, !1107, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw: %_0"}
!1107 = distinct !{!1107, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw"}
!1108 = !{!1109, !1111}
!1109 = distinct !{!1109, !1110, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!1110 = distinct !{!1110, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!1111 = distinct !{!1111, !1110, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!1112 = !{!1109}
!1113 = !{!1111}
!1114 = !{!1115, !1117}
!1115 = distinct !{!1115, !1116, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1116 = distinct !{!1116, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1117 = distinct !{!1117, !1116, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1118 = !{!1115}
!1119 = !{!1117}
!1120 = !{!1121, !1123}
!1121 = distinct !{!1121, !1122, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1122 = distinct !{!1122, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1123 = distinct !{!1123, !1122, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1124 = !{!1121}
!1125 = !{!1123}
!1126 = !{!1127, !1129}
!1127 = distinct !{!1127, !1128, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1128 = distinct !{!1128, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1129 = distinct !{!1129, !1128, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1130 = !{!1127}
!1131 = !{!1129}
!1132 = !{!1133, !1135}
!1133 = distinct !{!1133, !1134, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1134 = distinct !{!1134, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1135 = distinct !{!1135, !1134, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1136 = !{!1133}
!1137 = !{!1135}
!1138 = !{!1139, !1141}
!1139 = distinct !{!1139, !1140, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1140 = distinct !{!1140, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1141 = distinct !{!1141, !1140, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1142 = !{!1139}
!1143 = !{!1141}
!1144 = !{!1145, !1147}
!1145 = distinct !{!1145, !1146, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1146 = distinct !{!1146, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1147 = distinct !{!1147, !1146, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1148 = !{!1145}
!1149 = !{!1147}
!1150 = !{!1151, !1153}
!1151 = distinct !{!1151, !1152, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1152 = distinct !{!1152, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1153 = distinct !{!1153, !1152, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1154 = !{!1151}
!1155 = !{!1153}
!1156 = !{!1157, !1159}
!1157 = distinct !{!1157, !1158, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1158 = distinct !{!1158, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1159 = distinct !{!1159, !1158, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1160 = !{!1157}
!1161 = !{!1159}
!1162 = !{!1163, !1165}
!1163 = distinct !{!1163, !1164, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1164 = distinct !{!1164, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1165 = distinct !{!1165, !1164, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1166 = !{!1163}
!1167 = !{!1165}
!1168 = !{!1169, !1171}
!1169 = distinct !{!1169, !1170, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1170 = distinct !{!1170, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1171 = distinct !{!1171, !1170, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1172 = !{!1169}
!1173 = !{!1171}
!1174 = !{!1175, !1177}
!1175 = distinct !{!1175, !1176, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1176 = distinct !{!1176, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1177 = distinct !{!1177, !1176, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1178 = !{!1175}
!1179 = !{!1177}
!1180 = !{!1181, !1183}
!1181 = distinct !{!1181, !1182, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1182 = distinct !{!1182, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1183 = distinct !{!1183, !1182, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1184 = !{!1181}
!1185 = !{!1183}
!1186 = !{!1187, !1189}
!1187 = distinct !{!1187, !1188, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1188 = distinct !{!1188, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1189 = distinct !{!1189, !1188, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1190 = !{!1187}
!1191 = !{!1189}
!1192 = !{!1193, !1195}
!1193 = distinct !{!1193, !1194, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1194 = distinct !{!1194, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1195 = distinct !{!1195, !1194, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1196 = !{!1193}
!1197 = !{!1195}
!1198 = !{!1199, !1201}
!1199 = distinct !{!1199, !1200, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1200 = distinct !{!1200, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1201 = distinct !{!1201, !1200, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1202 = !{!1199}
!1203 = !{!1201}
!1204 = !{!1205, !1207}
!1205 = distinct !{!1205, !1206, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1206 = distinct !{!1206, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1207 = distinct !{!1207, !1206, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1208 = !{!1205}
!1209 = !{!1207}
!1210 = !{!1211, !1213}
!1211 = distinct !{!1211, !1212, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1212 = distinct !{!1212, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1213 = distinct !{!1213, !1212, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1214 = !{!1211}
!1215 = !{!1213}
!1216 = !{!1217, !1219}
!1217 = distinct !{!1217, !1218, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1218 = distinct !{!1218, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1219 = distinct !{!1219, !1218, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1220 = !{!1217}
!1221 = !{!1219}
!1222 = !{!1223, !1225}
!1223 = distinct !{!1223, !1224, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1224 = distinct !{!1224, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1225 = distinct !{!1225, !1224, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1226 = !{!1223}
!1227 = !{!1225}
!1228 = !{!1229, !1231}
!1229 = distinct !{!1229, !1230, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1230 = distinct !{!1230, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1231 = distinct !{!1231, !1230, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1232 = !{!1229}
!1233 = !{!1231}
!1234 = !{!1235, !1237}
!1235 = distinct !{!1235, !1236, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1236 = distinct !{!1236, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1237 = distinct !{!1237, !1236, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1238 = !{!1235}
!1239 = !{!1237}
!1240 = !{!1241, !1243}
!1241 = distinct !{!1241, !1242, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1242 = distinct !{!1242, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1243 = distinct !{!1243, !1242, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1244 = !{!1241}
!1245 = !{!1243}
!1246 = !{!1247, !1249}
!1247 = distinct !{!1247, !1248, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1248 = distinct !{!1248, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1249 = distinct !{!1249, !1248, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1250 = !{!1247}
!1251 = !{!1249}
!1252 = !{!1253, !1255}
!1253 = distinct !{!1253, !1254, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1254 = distinct !{!1254, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1255 = distinct !{!1255, !1254, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1256 = !{!1253}
!1257 = !{!1255}
!1258 = !{!1259, !1261}
!1259 = distinct !{!1259, !1260, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1260 = distinct !{!1260, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1261 = distinct !{!1261, !1260, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1262 = !{!1259}
!1263 = !{!1261}
!1264 = !{!1265, !1267}
!1265 = distinct !{!1265, !1266, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1266 = distinct !{!1266, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1267 = distinct !{!1267, !1266, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1268 = !{!1265}
!1269 = !{!1267}
!1270 = !{!1271, !1273}
!1271 = distinct !{!1271, !1272, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1272 = distinct !{!1272, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1273 = distinct !{!1273, !1272, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1274 = !{!1271}
!1275 = !{!1273}
!1276 = !{!1277, !1279}
!1277 = distinct !{!1277, !1278, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1278 = distinct !{!1278, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1279 = distinct !{!1279, !1278, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1280 = !{!1277}
!1281 = !{!1279}
!1282 = !{!1283, !1285}
!1283 = distinct !{!1283, !1284, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1284 = distinct !{!1284, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1285 = distinct !{!1285, !1284, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1286 = !{!1283}
!1287 = !{!1285}
!1288 = !{!1289, !1291}
!1289 = distinct !{!1289, !1290, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1290 = distinct !{!1290, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1291 = distinct !{!1291, !1290, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1292 = !{!1289}
!1293 = !{!1291}
!1294 = !{!1295, !1297}
!1295 = distinct !{!1295, !1296, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1296 = distinct !{!1296, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1297 = distinct !{!1297, !1296, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1298 = !{!1295}
!1299 = !{!1297}
!1300 = !{!1301, !1303}
!1301 = distinct !{!1301, !1302, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1302 = distinct !{!1302, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1303 = distinct !{!1303, !1302, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1304 = !{!1301}
!1305 = !{!1303}
!1306 = !{!1307, !1309}
!1307 = distinct !{!1307, !1308, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1308 = distinct !{!1308, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1309 = distinct !{!1309, !1308, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1310 = !{!1307}
!1311 = !{!1309}
!1312 = !{!1313, !1315}
!1313 = distinct !{!1313, !1314, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!1314 = distinct !{!1314, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!1315 = distinct !{!1315, !1314, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!1316 = !{!1313}
!1317 = !{!1315}
!1318 = !{!1319}
!1319 = distinct !{!1319, !1320, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1320 = distinct !{!1320, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1321 = !{!1322, !1324}
!1322 = distinct !{!1322, !1323, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!1323 = distinct !{!1323, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!1324 = distinct !{!1324, !1323, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!1325 = !{!1322}
!1326 = !{!1324}
!1327 = !{!1328, !1330}
!1328 = distinct !{!1328, !1329, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1329 = distinct !{!1329, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1330 = distinct !{!1330, !1329, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1331 = !{!1328}
!1332 = !{!1330}
!1333 = !{!1334, !1336}
!1334 = distinct !{!1334, !1335, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1335 = distinct !{!1335, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1336 = distinct !{!1336, !1335, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1337 = !{!1334}
!1338 = !{!1336}
!1339 = !{!1340, !1342}
!1340 = distinct !{!1340, !1341, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1341 = distinct !{!1341, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1342 = distinct !{!1342, !1341, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1343 = !{!1340}
!1344 = !{!1342}
!1345 = !{!1346, !1348}
!1346 = distinct !{!1346, !1347, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1347 = distinct !{!1347, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1348 = distinct !{!1348, !1347, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1349 = !{!1346}
!1350 = !{!1348}
!1351 = !{!1352, !1354}
!1352 = distinct !{!1352, !1353, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1353 = distinct !{!1353, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1354 = distinct !{!1354, !1353, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1355 = !{!1352}
!1356 = !{!1354}
!1357 = !{!1358, !1360}
!1358 = distinct !{!1358, !1359, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1359 = distinct !{!1359, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1360 = distinct !{!1360, !1359, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1361 = !{!1358}
!1362 = !{!1360}
!1363 = !{!1364, !1366}
!1364 = distinct !{!1364, !1365, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!1365 = distinct !{!1365, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!1366 = distinct !{!1366, !1365, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!1367 = !{!1364}
!1368 = !{!1366}
!1369 = !{!1370}
!1370 = distinct !{!1370, !1371, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: %self"}
!1371 = distinct !{!1371, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote"}
!1372 = !{!1373, !1374}
!1373 = distinct !{!1373, !1371, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: %t"}
!1374 = distinct !{!1374, !1371, !"_RNvMNtCsjMrxcFdYDNN_4core6resultINtB2_6ResultNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtBJ_8LexErrorE6expectCs1SHOYL7dTh6_5quote: argument 2"}
!1375 = !{!1373, !1370, !1374}
!1376 = !{!1373, !1370}
!1377 = !{!1378, !1380}
!1378 = distinct !{!1378, !1379, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1379 = distinct !{!1379, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1380 = distinct !{!1380, !1379, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1381 = !{!1380}
!1382 = !{!1383, !1385}
!1383 = distinct !{!1383, !1384, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1384 = distinct !{!1384, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1385 = distinct !{!1385, !1384, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1386 = !{!1385}
!1387 = !{!1388, !1390}
!1388 = distinct !{!1388, !1389, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1389 = distinct !{!1389, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1390 = distinct !{!1390, !1389, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1391 = !{!1390}
!1392 = !{!1393, !1395}
!1393 = distinct !{!1393, !1394, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1394 = distinct !{!1394, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1395 = distinct !{!1395, !1394, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1396 = !{!1395}
!1397 = !{!1398, !1400}
!1398 = distinct !{!1398, !1399, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1399 = distinct !{!1399, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1400 = distinct !{!1400, !1399, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1401 = !{!1400}
!1402 = !{!1403, !1405}
!1403 = distinct !{!1403, !1404, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1404 = distinct !{!1404, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1405 = distinct !{!1405, !1404, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1406 = !{!1405}
!1407 = !{!1408, !1410}
!1408 = distinct !{!1408, !1409, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1409 = distinct !{!1409, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1410 = distinct !{!1410, !1409, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1411 = !{!1410}
!1412 = !{!1413, !1415}
!1413 = distinct !{!1413, !1414, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1414 = distinct !{!1414, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1415 = distinct !{!1415, !1414, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1416 = !{!1415}
!1417 = !{!1418, !1420}
!1418 = distinct !{!1418, !1419, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1419 = distinct !{!1419, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1420 = distinct !{!1420, !1419, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1421 = !{!1420}
!1422 = !{!1423, !1425}
!1423 = distinct !{!1423, !1424, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1424 = distinct !{!1424, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1425 = distinct !{!1425, !1424, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1426 = !{!1425}
!1427 = !{!1428, !1430}
!1428 = distinct !{!1428, !1429, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1429 = distinct !{!1429, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1430 = distinct !{!1430, !1429, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1431 = !{!1430}
!1432 = !{!1433}
!1433 = distinct !{!1433, !1434, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw: %_0"}
!1434 = distinct !{!1434, !"_RNvNtCs1SHOYL7dTh6_5quote9___private15ident_maybe_raw"}
!1435 = !{!1436, !1438}
!1436 = distinct !{!1436, !1437, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1437 = distinct !{!1437, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1438 = distinct !{!1438, !1437, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1439 = !{!1438}
!1440 = !{!1441, !1443}
!1441 = distinct !{!1441, !1442, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1442 = distinct !{!1442, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1443 = distinct !{!1443, !1442, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1444 = !{!1443}
!1445 = !{!1446, !1448}
!1446 = distinct !{!1446, !1447, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1447 = distinct !{!1447, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1448 = distinct !{!1448, !1447, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1449 = !{!1448}
!1450 = !{!1451, !1453}
!1451 = distinct !{!1451, !1452, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1452 = distinct !{!1452, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1453 = distinct !{!1453, !1452, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1454 = !{!1453}
!1455 = !{!1456, !1458}
!1456 = distinct !{!1456, !1457, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1457 = distinct !{!1457, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1458 = distinct !{!1458, !1457, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1459 = !{!1458}
!1460 = !{!1461, !1463}
!1461 = distinct !{!1461, !1462, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1462 = distinct !{!1462, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1463 = distinct !{!1463, !1462, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1464 = !{!1463}
!1465 = !{!1466, !1468}
!1466 = distinct !{!1466, !1467, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1467 = distinct !{!1467, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1468 = distinct !{!1468, !1467, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1469 = !{!1468}
!1470 = !{!1471, !1473}
!1471 = distinct !{!1471, !1472, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1472 = distinct !{!1472, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1473 = distinct !{!1473, !1472, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1474 = !{!1473}
!1475 = !{!1476, !1478}
!1476 = distinct !{!1476, !1477, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1477 = distinct !{!1477, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1478 = distinct !{!1478, !1477, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1479 = !{!1478}
!1480 = !{!1481, !1483}
!1481 = distinct !{!1481, !1482, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1482 = distinct !{!1482, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1483 = distinct !{!1483, !1482, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1484 = !{!1483}
!1485 = !{!1486, !1488}
!1486 = distinct !{!1486, !1487, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1487 = distinct !{!1487, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1488 = distinct !{!1488, !1487, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1489 = !{!1488}
!1490 = !{!1491, !1493}
!1491 = distinct !{!1491, !1492, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1492 = distinct !{!1492, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1493 = distinct !{!1493, !1492, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1494 = !{!1493}
!1495 = !{!1496, !1498}
!1496 = distinct !{!1496, !1497, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1497 = distinct !{!1497, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1498 = distinct !{!1498, !1497, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1499 = !{!1498}
!1500 = !{!1501, !1503}
!1501 = distinct !{!1501, !1502, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1502 = distinct !{!1502, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1503 = distinct !{!1503, !1502, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1504 = !{!1503}
!1505 = !{!1506, !1508}
!1506 = distinct !{!1506, !1507, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1507 = distinct !{!1507, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1508 = distinct !{!1508, !1507, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1509 = !{!1508}
!1510 = !{!1511, !1513}
!1511 = distinct !{!1511, !1512, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1512 = distinct !{!1512, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1513 = distinct !{!1513, !1512, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1514 = !{!1513}
!1515 = !{!1516, !1518}
!1516 = distinct !{!1516, !1517, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1517 = distinct !{!1517, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1518 = distinct !{!1518, !1517, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1519 = !{!1518}
!1520 = !{!1521, !1523}
!1521 = distinct !{!1521, !1522, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1522 = distinct !{!1522, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1523 = distinct !{!1523, !1522, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1524 = !{!1523}
!1525 = !{!1526, !1528}
!1526 = distinct !{!1526, !1527, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote: %_0"}
!1527 = distinct !{!1527, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote"}
!1528 = distinct !{!1528, !1527, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringNtCs8M6BBVNvC7a_11proc_macro25IdentNtB5_12SpecToString14spec_to_stringCs1SHOYL7dTh6_5quote: %self"}
!1529 = !{!1526}
!1530 = !{!1531}
!1531 = distinct !{!1531, !1532, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote: %_1"}
!1532 = distinct !{!1532, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote"}
!1533 = !{!1531, !1526}
!1534 = !{!1528}
!1535 = !{!1536}
!1536 = distinct !{!1536, !1537, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote: %_1"}
!1537 = distinct !{!1537, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote"}
!1538 = !{!1539}
!1539 = distinct !{!1539, !1540, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote: %_1"}
!1540 = distinct !{!1540, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs1SHOYL7dTh6_5quote"}
!1541 = !{!1542, !1544}
!1542 = distinct !{!1542, !1543, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1543 = distinct !{!1543, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1544 = distinct !{!1544, !1543, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1545 = !{!1542}
!1546 = !{!1544}
!1547 = !{!1548, !1550}
!1548 = distinct !{!1548, !1549, !"_RNvXs4_NtCs1SHOYL7dTh6_5quote9to_tokenseNtB5_8ToTokens9to_tokens: %self.0"}
!1549 = distinct !{!1549, !"_RNvXs4_NtCs1SHOYL7dTh6_5quote9to_tokenseNtB5_8ToTokens9to_tokens"}
!1550 = distinct !{!1550, !1549, !"_RNvXs4_NtCs1SHOYL7dTh6_5quote9to_tokenseNtB5_8ToTokens9to_tokens: %tokens"}
!1551 = !{!1550}
!1552 = !{!1553, !1555, !1548, !1550}
!1553 = distinct !{!1553, !1554, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1554 = distinct !{!1554, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1555 = distinct !{!1555, !1554, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1556 = !{!1553, !1548, !1550}
!1557 = !{!1555}
!1558 = !{!1559, !1561}
!1559 = distinct !{!1559, !1560, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1560 = distinct !{!1560, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1561 = distinct !{!1561, !1560, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1562 = !{!1559}
!1563 = !{!1561}
!1564 = !{!1565, !1567}
!1565 = distinct !{!1565, !1566, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1566 = distinct !{!1566, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1567 = distinct !{!1567, !1566, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1568 = !{!1565}
!1569 = !{!1567}
!1570 = !{!1571, !1573}
!1571 = distinct !{!1571, !1572, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1572 = distinct !{!1572, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1573 = distinct !{!1573, !1572, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1574 = !{!1571}
!1575 = !{!1573}
!1576 = !{!1577, !1579}
!1577 = distinct !{!1577, !1578, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1578 = distinct !{!1578, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1579 = distinct !{!1579, !1578, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1580 = !{!1577}
!1581 = !{!1579}
!1582 = !{!1583}
!1583 = distinct !{!1583, !1584, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %self"}
!1584 = distinct !{!1584, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone"}
!1585 = !{!1586}
!1586 = distinct !{!1586, !1584, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %_0"}
!1587 = !{!1586, !1583}
!1588 = !{!1589, !1583}
!1589 = distinct !{!1589, !1590, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1590 = distinct !{!1590, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1591 = !{!1592, !1586}
!1592 = distinct !{!1592, !1590, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1593 = !{!1594, !1589, !1583}
!1594 = distinct !{!1594, !1595, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1595 = distinct !{!1595, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1596 = !{!1597, !1592, !1586}
!1597 = distinct !{!1597, !1595, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1598 = !{!1599}
!1599 = distinct !{!1599, !1600, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1600 = distinct !{!1600, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1601 = !{!1602}
!1602 = distinct !{!1602, !1600, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1603 = !{i8 0, i8 11}
!1604 = !{!1605}
!1605 = distinct !{!1605, !1606, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1606 = distinct !{!1606, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1607 = !{!1608}
!1608 = distinct !{!1608, !1606, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1609 = !{!1610}
!1610 = distinct !{!1610, !1611, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!1611 = distinct !{!1611, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!1612 = !{!1613, !1610}
!1613 = distinct !{!1613, !1614, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote: %self"}
!1614 = distinct !{!1614, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote"}
!1615 = !{!1616}
!1616 = distinct !{!1616, !1617, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!1617 = distinct !{!1617, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!1618 = !{!1619}
!1619 = distinct !{!1619, !1620, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs1SHOYL7dTh6_5quote: %self"}
!1620 = distinct !{!1620, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs1SHOYL7dTh6_5quote"}
!1621 = !{!1622, !1619, !1616}
!1622 = distinct !{!1622, !1623, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote: %self"}
!1623 = distinct !{!1623, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs1SHOYL7dTh6_5quote"}
!1624 = !{!1625}
!1625 = distinct !{!1625, !1617, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!1626 = !{!1619, !1616}
!1627 = !{!1628, !1630}
!1628 = distinct !{!1628, !1629, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1629 = distinct !{!1629, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1630 = distinct !{!1630, !1629, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1631 = !{!1628}
!1632 = !{!1630}
!1633 = !{!1634, !1636}
!1634 = distinct !{!1634, !1635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1635 = distinct !{!1635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1636 = distinct !{!1636, !1635, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1637 = !{!1634}
!1638 = !{!1636}
!1639 = !{!1640, !1642}
!1640 = distinct !{!1640, !1641, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1641 = distinct !{!1641, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1642 = distinct !{!1642, !1641, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1643 = !{!1640}
!1644 = !{!1642}
!1645 = !{!1646, !1648}
!1646 = distinct !{!1646, !1647, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1647 = distinct !{!1647, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1648 = distinct !{!1648, !1647, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1649 = !{!1646}
!1650 = !{!1648}
!1651 = !{!1652, !1654}
!1652 = distinct !{!1652, !1653, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1653 = distinct !{!1653, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1654 = distinct !{!1654, !1653, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1655 = !{!1652}
!1656 = !{!1654}
!1657 = !{!1658, !1660}
!1658 = distinct !{!1658, !1659, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1659 = distinct !{!1659, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1660 = distinct !{!1660, !1659, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1661 = !{!1658}
!1662 = !{!1660}
!1663 = !{!1664, !1666}
!1664 = distinct !{!1664, !1665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1665 = distinct !{!1665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1666 = distinct !{!1666, !1665, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1667 = !{!1664}
!1668 = !{!1666}
!1669 = !{!1670, !1672}
!1670 = distinct !{!1670, !1671, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1671 = distinct !{!1671, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1672 = distinct !{!1672, !1671, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1673 = !{!1670}
!1674 = !{!1672}
!1675 = !{!1676, !1678}
!1676 = distinct !{!1676, !1677, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1677 = distinct !{!1677, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1678 = distinct !{!1678, !1677, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1679 = !{!1676}
!1680 = !{!1678}
!1681 = !{!1682, !1684}
!1682 = distinct !{!1682, !1683, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1683 = distinct !{!1683, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1684 = distinct !{!1684, !1683, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1685 = !{!1682}
!1686 = !{!1684}
!1687 = !{i32 0, i32 1114112}
!1688 = !{!1689, !1691}
!1689 = distinct !{!1689, !1690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1690 = distinct !{!1690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1691 = distinct !{!1691, !1690, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1692 = !{!1689}
!1693 = !{!1691}
!1694 = !{!1695, !1697}
!1695 = distinct !{!1695, !1696, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!1696 = distinct !{!1696, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!1697 = distinct !{!1697, !1696, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!1698 = !{!1695}
!1699 = !{!1697}
!1700 = !{!1701, !1703}
!1701 = distinct !{!1701, !1702, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1702 = distinct !{!1702, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1703 = distinct !{!1703, !1702, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1704 = !{!1701}
!1705 = !{!1703}
!1706 = !{!1707, !1709}
!1707 = distinct !{!1707, !1708, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1708 = distinct !{!1708, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1709 = distinct !{!1709, !1708, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1710 = !{!1707}
!1711 = !{!1709}
!1712 = !{!1713}
!1713 = distinct !{!1713, !1714, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %self"}
!1714 = distinct !{!1714, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone"}
!1715 = !{!1716}
!1716 = distinct !{!1716, !1714, !"_RNvXsF_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_5GroupNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %_0"}
!1717 = !{!1716, !1713}
!1718 = !{!1719, !1713}
!1719 = distinct !{!1719, !1720, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1720 = distinct !{!1720, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1721 = !{!1722, !1716}
!1722 = distinct !{!1722, !1720, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1723 = !{!1724, !1719, !1713}
!1724 = distinct !{!1724, !1725, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1725 = distinct !{!1725, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1726 = !{!1727, !1722, !1716}
!1727 = distinct !{!1727, !1725, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1728 = !{!1729, !1731}
!1729 = distinct !{!1729, !1730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %self"}
!1730 = distinct !{!1730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_"}
!1731 = distinct !{!1731, !1730, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5GroupEB5_: %token"}
!1732 = !{!1731}
!1733 = !{!1734}
!1734 = distinct !{!1734, !1735, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1735 = distinct !{!1735, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1736 = !{!1737}
!1737 = distinct !{!1737, !1735, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1738 = !{!1739, !1741}
!1739 = distinct !{!1739, !1740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %self"}
!1740 = distinct !{!1740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_"}
!1741 = distinct !{!1741, !1740, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5IdentEB5_: %token"}
!1742 = !{!1741}
!1743 = !{!1744, !1746}
!1744 = distinct !{!1744, !1745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %self"}
!1745 = distinct !{!1745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_"}
!1746 = distinct !{!1746, !1745, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_5PunctEB5_: %token"}
!1747 = !{!1744}
!1748 = !{!1746}
!1749 = !{!1750}
!1750 = distinct !{!1750, !1751, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1751 = distinct !{!1751, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1752 = !{!1753}
!1753 = distinct !{!1753, !1751, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1754 = !{!1755, !1757}
!1755 = distinct !{!1755, !1756, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %self"}
!1756 = distinct !{!1756, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_"}
!1757 = distinct !{!1757, !1756, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_7LiteralEB5_: %token"}
!1758 = !{!1757}
!1759 = !{!1760, !1762}
!1760 = distinct !{!1760, !1761, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %self"}
!1761 = distinct !{!1761, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_"}
!1762 = distinct !{!1762, !1761, !"_RINvXNtCs1SHOYL7dTh6_5quote3extNtCs8M6BBVNvC7a_11proc_macro211TokenStreamNtB3_14TokenStreamExt6appendNtBv_9TokenTreeEB5_: %token"}
!1763 = !{!1762}
!1764 = !{!1765}
!1765 = distinct !{!1765, !1766, !"_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %self"}
!1766 = distinct !{!1766, !"_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone"}
!1767 = !{!1768}
!1768 = distinct !{!1768, !1766, !"_RNvXsz_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_11TokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %_0"}
!1769 = !{!1768, !1765}
!1770 = !{!1771}
!1771 = distinct !{!1771, !1772, !"_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %self"}
!1772 = distinct !{!1772, !"_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone"}
!1773 = !{!1774, !1771, !1768, !1765}
!1774 = distinct !{!1774, !1772, !"_RNvXsA_NtCs8M6BBVNvC7a_11proc_macro23impNtB5_19DeferredTokenStreamNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %_0"}
!1775 = !{!1771, !1765}
!1776 = !{!1774, !1768}
!1777 = !{!1778}
!1778 = distinct !{!1778, !1779, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCsbtZo8rQoR5w_10proc_macro9TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote: %s.0"}
!1779 = distinct !{!1779, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCsbtZo8rQoR5w_10proc_macro9TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote"}
!1780 = !{!1781, !1778, !1782, !1774, !1771, !1768, !1765}
!1781 = distinct !{!1781, !1779, !"_RINvXNvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inNtCsbtZo8rQoR5w_10proc_macro9TokenTreeNtB3_10ConvertVec6to_vecNtNtB8_5alloc6GlobalECs1SHOYL7dTh6_5quote: %_0"}
!1782 = distinct !{!1782, !1783, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1783 = distinct !{!1783, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtCsbtZo8rQoR5w_10proc_macro9TokenTreeENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1784 = !{!1785, !1781, !1778, !1782, !1774, !1768}
!1785 = distinct !{!1785, !1786, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1SHOYL7dTh6_5quote: %_0"}
!1786 = distinct !{!1786, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs1SHOYL7dTh6_5quote"}
!1787 = !{!1788}
!1788 = distinct !{!1788, !1789, !"_RNvXs14_CsbtZo8rQoR5w_10proc_macroNtB6_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %_0"}
!1789 = distinct !{!1789, !"_RNvXs14_CsbtZo8rQoR5w_10proc_macroNtB6_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone"}
!1790 = !{!1791, !1778}
!1791 = distinct !{!1791, !1789, !"_RNvXs14_CsbtZo8rQoR5w_10proc_macroNtB6_9TokenTreeNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone: %self"}
!1792 = !{!1788, !1781, !1782, !1774, !1768}
!1793 = !{!1794, !1791, !1778}
!1794 = distinct !{!1794, !1795, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1795 = distinct !{!1795, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1796 = !{!1797, !1788, !1781, !1782, !1774, !1768}
!1797 = distinct !{!1797, !1795, !"_RNvXs18_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5GroupNtNtB6_6client11TokenStreamNtBT_4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1798 = !{!1781, !1782, !1774, !1768}
!1799 = !{!1800, !1794, !1791, !1778}
!1800 = distinct !{!1800, !1801, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1801 = distinct !{!1801, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1802 = !{!1803, !1797, !1788, !1781, !1782, !1774, !1768}
!1803 = distinct !{!1803, !1801, !"_RNvXs13_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_9DelimSpanNtNtB6_6client4SpanENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1804 = !{!1805, !1791, !1778}
!1805 = distinct !{!1805, !1806, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1806 = distinct !{!1806, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1807 = !{!1808, !1788, !1781, !1782, !1774, !1768}
!1808 = distinct !{!1808, !1806, !"_RNvXs1j_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_5IdentNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
!1809 = !{!1788, !1791, !1778}
!1810 = !{!1811, !1791, !1778}
!1811 = distinct !{!1811, !1812, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %self"}
!1812 = distinct !{!1812, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote"}
!1813 = !{!1814, !1788, !1781, !1782, !1774, !1768}
!1814 = distinct !{!1814, !1812, !"_RNvXs1r_NtCsbtZo8rQoR5w_10proc_macro6bridgeINtB6_7LiteralNtNtB6_6client4SpanNtNtB6_6symbol6SymbolENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs1SHOYL7dTh6_5quote: %_0"}
