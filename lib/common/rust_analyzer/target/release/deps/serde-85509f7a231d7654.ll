; ModuleID = 'serde.5073ef404d042dbb-cgu.0'
source_filename = "serde.5073ef404d042dbb-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

%"serde_core::private::content::Content<'_>" = type { i8, [31 x i8] }

@alloc_a32ee92aa163a424fb5f24d7445c00a9 = private unnamed_addr constant [8 x i8] c"a string", align 1
@alloc_4b2e1ac4e77881e01aa50df408195ab8 = private unnamed_addr constant [12 x i8] c"a byte array", align 1
@alloc_64c46d602bcdf61cecebd9c07a8ffbde = private unnamed_addr constant [9 x i8] c"a boolean", align 1
@alloc_a45aab79b1349dc8dcfa9a0871485b38 = private unnamed_addr constant [10 x i8] c"an integer", align 1
@alloc_d65406d2c9a3b2f5e537fca7bc28c259 = private unnamed_addr constant [7 x i8] c"a float", align 1
@alloc_0f28191399eb63227c9545128039b625 = private unnamed_addr constant [6 x i8] c"a char", align 1
@alloc_55183e09502581a3b4be406985e5433a = private unnamed_addr constant [11 x i8] c"an optional", align 1
@alloc_c705cb38a8523735dfb9d88c7a0743fd = private unnamed_addr constant [10 x i8] c"a sequence", align 1
@alloc_42b547da467ce6ab10816f050e5805ac = private unnamed_addr constant [7 x i8] c"a tuple", align 1
@alloc_aa9f3e4e91af539e6d709a9ddaa9a5d6 = private unnamed_addr constant [14 x i8] c"a tuple struct", align 1
@alloc_b8f0e9f5a55e3399052e7f51171d5f73 = private unnamed_addr constant [16 x i8] c"1 element in map", align 1
@alloc_bb2978a5d2b5523fedc4230268ea2bca = private unnamed_addr constant [19 x i8] c"\C0\10 elements in map\00", align 1
@alloc_38b751980a67eae055f527bc65124ed1 = private unnamed_addr constant [20 x i8] c"\0Dunit variant \C0\02::\C0\00", align 1
@alloc_b4672af18d56d6b08faa87389daaee94 = private unnamed_addr constant [9 x i8] c"any value", align 1
@alloc_b8b61a38e78c2df660def71a70f41227 = private unnamed_addr constant [36 x i8] c"\0Ca type tag `\C0\14` or any other value\00", align 1
@alloc_47bf8b2623f12665ad0eb57968a6fa8a = private unnamed_addr constant [8 x i8] c"\C0\04 or \C0\00", align 1
@alloc_7f44bc6f09057a780564215ee717c5f7 = private unnamed_addr constant [32 x i8] c"\C0\02, \C0\19, or other ignored fields\00", align 1
@alloc_e71a28b1125008ec4c9de9e536a49431 = private unnamed_addr constant [21 x i8] c"1 element in sequence", align 1
@alloc_8c1db4387b4656bb4b356dc02c3f8d30 = private unnamed_addr constant [24 x i8] c"\C0\15 elements in sequence\00", align 1

@_RNvXsS_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_19UntaggedUnitVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting = unnamed_addr alias i1 (ptr, ptr), ptr @_RNvXsR_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_27InternallyTaggedUnitVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting

; core::ptr::drop_in_place::<alloc::vec::Vec<serde_core::private::content::Content>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %0, align 8, !nonnull !2, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val5 = load i64, ptr %1, align 8, !noundef !2
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb5.i.i, %start
  %_3.sroa.0.0.i.i = phi i64 [ 0, %start ], [ %2, %bb5.i.i ]
  %_7.i.i = icmp eq i64 %_3.sroa.0.0.i.i, %_1.val5
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb6.i.i
  %_6.i.i = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %_1.val4, i64 %_3.sroa.0.0.i.i
  %2 = add i64 %_3.sroa.0.0.i.i, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 dereferenceable(32) %_6.i.i)
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
  %_4.i.i = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %_1.val4, i64 %_3.sroa.0.1.i.i
  %4 = add i64 %_3.sroa.0.1.i.i, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 dereferenceable(32) %_4.i.i) #10
          to label %bb4.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %bb3.i.i
  %5 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val = load i64, ptr %_1, align 8, !range !3, !noundef !2
  %6 = icmp eq i64 %_1.val, 0
  br i1 %6, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %bb1

bb4:                                              ; preds = %bb6.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !3, !noundef !2
  %7 = icmp eq i64 %_1.val2, 0
  br i1 %7, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val2, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %3
}

; core::ptr::drop_in_place::<alloc::vec::Vec<(serde_core::private::content::Content, serde_core::private::content::Content)>>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val4 = load ptr, ptr %0, align 8, !nonnull !2, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_1.val5 = load i64, ptr %1, align 8, !noundef !2
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb4.i3.i, %start
  %_3.sroa.0.0.i.i = phi i64 [ 0, %start ], [ %2, %bb4.i3.i ]
  %_7.i.i = icmp eq i64 %_3.sroa.0.0.i.i, %_1.val5
  br i1 %_7.i.i, label %bb4, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb6.i.i
  %_6.i.i = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %_1.val4, i64 %_3.sroa.0.0.i.i
  %2 = add i64 %_3.sroa.0.0.i.i, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull readonly align 8 dereferenceable(64) %_6.i.i) #13
          to label %bb4.i3.i unwind label %cleanup.i1.i

cleanup.i1.i:                                     ; preds = %bb5.i.i
  %3 = landingpad { ptr, i32 }
          cleanup
  %4 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 32
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 dereferenceable(32) %4) #14
          to label %cleanup.i.body.i unwind label %terminate.i2.i

bb4.i3.i:                                         ; preds = %bb5.i.i
  %5 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 32
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 dereferenceable(32) %5) #13
          to label %bb6.i.i unwind label %cleanup.i.i

terminate.i2.i:                                   ; preds = %cleanup.i1.i
  %6 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !4
  unreachable

bb4.i.i:                                          ; preds = %bb3.i.i, %cleanup.i.body.i
  %_3.sroa.0.1.i.i = phi i64 [ %2, %cleanup.i.body.i ], [ %8, %bb3.i.i ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %_1.val5
  br i1 %_5.i.i, label %cleanup.body, label %bb3.i.i

cleanup.i.i:                                      ; preds = %bb4.i3.i
  %7 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.body.i

cleanup.i.body.i:                                 ; preds = %cleanup.i.i, %cleanup.i1.i
  %eh.lpad-body.i = phi { ptr, i32 } [ %7, %cleanup.i.i ], [ %3, %cleanup.i1.i ]
  br label %bb4.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %_1.val4, i64 %_3.sroa.0.1.i.i
  %8 = add i64 %_3.sroa.0.1.i.i, 1
; invoke core::ptr::drop_in_place::<(serde_core::private::content::Content, serde_core::private::content::Content)>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBH_EECs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 dereferenceable(64) %_4.i.i) #10
          to label %bb4.i.i unwind label %terminate.i.i

terminate.i.i:                                    ; preds = %bb3.i.i
  %9 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11
  unreachable

cleanup.body:                                     ; preds = %bb4.i.i
  %_1.val = load i64, ptr %_1, align 8, !range !3, !noundef !2
  %10 = icmp eq i64 %_1.val, 0
  br i1 %10, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %cleanup.body
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val, 6
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %bb1

bb4:                                              ; preds = %bb6.i.i
  %_1.val2 = load i64, ptr %_1, align 8, !range !3, !noundef !2
  %11 = icmp eq i64 %_1.val2, 0
  br i1 %11, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1l_EEECs6Ufq8Na2eJH_5serde.exit8, label %bb2.i.i.i6

bb2.i.i.i6:                                       ; preds = %bb4
  %alloc_size.i.i.i.i7 = shl nuw i64 %_1.val2, 6
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val4, i64 noundef %alloc_size.i.i.i.i7, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1l_EEECs6Ufq8Na2eJH_5serde.exit8

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc7raw_vec6RawVecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1l_EEECs6Ufq8Na2eJH_5serde.exit8: ; preds = %bb4, %bb2.i.i.i6
  ret void

bb1:                                              ; preds = %bb2.i.i.i, %cleanup.body
  resume { ptr, i32 } %eh.lpad-body.i
}

; core::ptr::drop_in_place::<serde_core::private::content::Content>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(32) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %0 = load i8, ptr %_1, align 8, !range !7, !noundef !2
  switch i8 %0, label %bb7 [
    i8 0, label %bb1
    i8 1, label %bb1
    i8 2, label %bb1
    i8 3, label %bb1
    i8 4, label %bb1
    i8 5, label %bb1
    i8 6, label %bb1
    i8 7, label %bb1
    i8 8, label %bb1
    i8 9, label %bb1
    i8 10, label %bb1
    i8 11, label %bb1
    i8 12, label %bb2
    i8 13, label %bb1
    i8 14, label %bb3
    i8 15, label %bb1
    i8 16, label %bb1
    i8 17, label %bb4
    i8 18, label %bb1
    i8 19, label %bb5
    i8 20, label %bb6
  ]

bb7:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !8)
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_5.i = load ptr, ptr %2, align 8, !alias.scope !8, !nonnull !2, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %len.i = load i64, ptr %3, align 8, !alias.scope !8, !noundef !2
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb4.i, %bb7
  %_3.sroa.0.0.i.i = phi i64 [ 0, %bb7 ], [ %4, %bb4.i ]
  %_7.i.i = icmp eq i64 %_3.sroa.0.0.i.i, %len.i
  br i1 %_7.i.i, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde.exit, label %bb5.i.i

bb5.i.i:                                          ; preds = %bb6.i.i
  %_6.i.i = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %_5.i, i64 %_3.sroa.0.0.i.i
  %4 = add i64 %_3.sroa.0.0.i.i, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull align 8 dereferenceable(64) %_6.i.i)
          to label %bb4.i unwind label %cleanup.i47, !noalias !8

cleanup.i47:                                      ; preds = %bb5.i.i
  %5 = landingpad { ptr, i32 }
          cleanup
  %6 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 32
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %6) #10
          to label %cleanup.i.i.body unwind label %terminate.i48, !noalias !8

bb4.i:                                            ; preds = %bb5.i.i
  %7 = getelementptr inbounds nuw i8, ptr %_6.i.i, i64 32
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %7)
          to label %bb6.i.i unwind label %cleanup.i.i

terminate.i48:                                    ; preds = %cleanup.i47
  %8 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !8
  unreachable

bb4.i.i:                                          ; preds = %bb3.i.i, %cleanup.i.i.body
  %_3.sroa.0.1.i.i = phi i64 [ %4, %cleanup.i.i.body ], [ %10, %bb3.i.i ]
  %_5.i.i = icmp eq i64 %_3.sroa.0.1.i.i, %len.i
  br i1 %_5.i.i, label %cleanup.i.body, label %bb3.i.i

cleanup.i.i:                                      ; preds = %bb4.i
  %9 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i.i.body

cleanup.i.i.body:                                 ; preds = %cleanup.i47, %cleanup.i.i
  %eh.lpad-body50 = phi { ptr, i32 } [ %9, %cleanup.i.i ], [ %5, %cleanup.i47 ]
  br label %bb4.i.i

bb3.i.i:                                          ; preds = %bb4.i.i
  %_4.i.i = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %_5.i, i64 %_3.sroa.0.1.i.i
  %10 = add i64 %_3.sroa.0.1.i.i, 1
; invoke core::ptr::drop_in_place::<(serde_core::private::content::Content, serde_core::private::content::Content)>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBH_EECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(64) %_4.i.i) #10
          to label %bb4.i.i unwind label %terminate.i.i, !noalias !8

terminate.i.i:                                    ; preds = %bb3.i.i
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !8
  unreachable

cleanup.i.body:                                   ; preds = %bb4.i.i
  %_1.val.i = load i64, ptr %1, align 8, !range !3, !alias.scope !11, !noundef !2
  %12 = icmp eq i64 %_1.val.i, 0
  br i1 %12, label %common.resume, label %bb2.i.i.i23

bb2.i.i.i23:                                      ; preds = %cleanup.i.body
  %alloc_size.i.i.i.i24 = shl nuw i64 %_1.val.i, 6
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.i, i64 noundef %alloc_size.i.i.i.i24, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %common.resume

common.resume:                                    ; preds = %cleanup.i16.body, %bb2.i.i.i28, %cleanup.i.body, %bb2.i.i.i23, %bb1.i13, %bb1.i7
  %common.resume.op = phi { ptr, i32 } [ %21, %bb1.i7 ], [ %23, %bb1.i13 ], [ %eh.lpad-body50, %bb2.i.i.i23 ], [ %eh.lpad-body50, %cleanup.i.body ], [ %28, %bb2.i.i.i28 ], [ %28, %cleanup.i16.body ]
  resume { ptr, i32 } %common.resume.op

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde.exit: ; preds = %bb6.i.i
  %_1.val2.i = load i64, ptr %1, align 8, !range !3, !alias.scope !11, !noundef !2
  %13 = icmp eq i64 %_1.val2.i, 0
  br i1 %13, label %bb1, label %bb2.i.i.i

bb2.i.i.i:                                        ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde.exit
  %alloc_size.i.i.i.i = shl nuw i64 %_1.val2.i, 6
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.i, i64 noundef %alloc_size.i.i.i.i, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %bb1

bb1:                                              ; preds = %bb2.i.i.i26, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit, %bb2.i.i.i4.i, %bb3, %bb2.i.i.i4.i.i, %bb2, %bb2.i.i.i, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde.exit, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit15, %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start, %start
  ret void

bb2:                                              ; preds = %start
  %14 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %.val2 = load i64, ptr %14, align 8
  %15 = icmp eq i64 %.val2, 0
  br i1 %15, label %bb1, label %bb2.i.i.i4.i.i

bb2.i.i.i4.i.i:                                   ; preds = %bb2
  %16 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val3 = load ptr, ptr %16, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val3, i64 noundef %.val2, i64 noundef range(i64 1, -9223372036854775807) 1) #12
  br label %bb1

bb3:                                              ; preds = %start
  %17 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %.val = load i64, ptr %17, align 8
  %18 = icmp eq i64 %.val, 0
  br i1 %18, label %bb1, label %bb2.i.i.i4.i

bb2.i.i.i4.i:                                     ; preds = %bb3
  %19 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %.val1 = load ptr, ptr %19, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %.val1, i64 noundef %.val, i64 noundef range(i64 1, -9223372036854775807) 1) #12
  br label %bb1

bb4:                                              ; preds = %start
  %20 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !14)
  %_6.i = load ptr, ptr %20, align 8, !alias.scope !14, !nonnull !2, !noundef !2
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_6.i)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit unwind label %bb1.i7, !noalias !14

bb1.i7:                                           ; preds = %bb4
  %21 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.i, i64 noundef 32, i64 noundef 8) #12, !noalias !14
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit: ; preds = %bb4
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.i, i64 noundef 32, i64 noundef 8) #12, !noalias !14
  br label %bb1

bb5:                                              ; preds = %start
  %22 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !17)
  %_6.i9 = load ptr, ptr %22, align 8, !alias.scope !17, !nonnull !2, !noundef !2
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_6.i9)
          to label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit15 unwind label %bb1.i13, !noalias !17

bb1.i13:                                          ; preds = %bb5
  %23 = landingpad { ptr, i32 }
          cleanup
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.i9, i64 noundef 32, i64 noundef 8) #12, !noalias !17
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit15: ; preds = %bb5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_6.i9, i64 noundef 32, i64 noundef 8) #12, !noalias !17
  br label %bb1

bb6:                                              ; preds = %start
  %24 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !20)
  %25 = getelementptr inbounds nuw i8, ptr %_1, i64 16
  %_5.i31 = load ptr, ptr %25, align 8, !alias.scope !20, !nonnull !2, !noundef !2
  %26 = getelementptr inbounds nuw i8, ptr %_1, i64 24
  %len.i32 = load i64, ptr %26, align 8, !alias.scope !20, !noundef !2
  br label %bb6.i.i33

bb6.i.i33:                                        ; preds = %bb5.i.i36, %bb6
  %_3.sroa.0.0.i.i34 = phi i64 [ 0, %bb6 ], [ %27, %bb5.i.i36 ]
  %_7.i.i35 = icmp eq i64 %_3.sroa.0.0.i.i34, %len.i32
  br i1 %_7.i.i35, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit, label %bb5.i.i36

bb5.i.i36:                                        ; preds = %bb6.i.i33
  %_6.i.i37 = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %_5.i31, i64 %_3.sroa.0.0.i.i34
  %27 = add i64 %_3.sroa.0.0.i.i34, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_6.i.i37)
          to label %bb6.i.i33 unwind label %cleanup.i.i38, !noalias !20

bb4.i.i39:                                        ; preds = %bb3.i.i42, %cleanup.i.i38
  %_3.sroa.0.1.i.i40 = phi i64 [ %27, %cleanup.i.i38 ], [ %29, %bb3.i.i42 ]
  %_5.i.i41 = icmp eq i64 %_3.sroa.0.1.i.i40, %len.i32
  br i1 %_5.i.i41, label %cleanup.i16.body, label %bb3.i.i42

cleanup.i.i38:                                    ; preds = %bb5.i.i36
  %28 = landingpad { ptr, i32 }
          cleanup
  br label %bb4.i.i39

bb3.i.i42:                                        ; preds = %bb4.i.i39
  %_4.i.i43 = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %_5.i31, i64 %_3.sroa.0.1.i.i40
  %29 = add i64 %_3.sroa.0.1.i.i40, 1
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_4.i.i43) #10
          to label %bb4.i.i39 unwind label %terminate.i.i44, !noalias !20

terminate.i.i44:                                  ; preds = %bb3.i.i42
  %30 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !20
  unreachable

cleanup.i16.body:                                 ; preds = %bb4.i.i39
  %_1.val.i17 = load i64, ptr %24, align 8, !range !3, !alias.scope !23, !noundef !2
  %31 = icmp eq i64 %_1.val.i17, 0
  br i1 %31, label %common.resume, label %bb2.i.i.i28

bb2.i.i.i28:                                      ; preds = %cleanup.i16.body
  %alloc_size.i.i.i.i29 = shl nuw i64 %_1.val.i17, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.i31, i64 noundef %alloc_size.i.i.i.i29, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %common.resume

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit: ; preds = %bb6.i.i33
  %_1.val2.i21 = load i64, ptr %24, align 8, !range !3, !alias.scope !23, !noundef !2
  %32 = icmp eq i64 %_1.val2.i21, 0
  br i1 %32, label %bb1, label %bb2.i.i.i26

bb2.i.i.i26:                                      ; preds = %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde.exit
  %alloc_size.i.i.i.i27 = shl nuw i64 %_1.val2.i21, 5
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_5.i31, i64 noundef %alloc_size.i.i.i.i27, i64 noundef range(i64 1, -9223372036854775807) 8) #12
  br label %bb1
}

; core::ptr::drop_in_place::<(serde_core::private::content::Content, serde_core::private::content::Content)>
; Function Attrs: uwtable
define internal fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBH_EECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull readonly align 8 captures(none) dereferenceable(64) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_1)
          to label %bb4 unwind label %cleanup

cleanup:                                          ; preds = %start
  %0 = landingpad { ptr, i32 }
          cleanup
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %1) #10
          to label %bb1 unwind label %terminate

bb4:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %_1, i64 32
; call core::ptr::drop_in_place::<serde_core::private::content::Content>
  tail call fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %2)
  ret void

terminate:                                        ; preds = %cleanup
  %3 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11
  unreachable

bb1:                                              ; preds = %cleanup
  resume { ptr, i32 } %0
}

; serde::private::de::flat_map_take_entry
; Function Attrs: uwtable
define void @_RNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry(ptr dead_on_unwind noalias noundef writable writeonly sret([64 x i8]) align 8 captures(none) dereferenceable(64) %_0, ptr noalias noundef align 8 captures(none) dereferenceable(64) %entry, ptr noalias noundef nonnull readonly align 8 captures(address) %recognized.0, i64 noundef range(i64 0, 576460752303423488) %recognized.1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_9.i = alloca [24 x i8], align 8
  %_7.i = alloca [24 x i8], align 8
  %0 = load i8, ptr %entry, align 8, !range !26, !noundef !2
  %.not = icmp eq i8 %0, 22
  br i1 %.not, label %bb8, label %bb2

bb2:                                              ; preds = %start
  tail call void @llvm.experimental.noalias.scope.decl(metadata !27)
  switch i8 %0, label %_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit [
    i8 12, label %bb4.i
    i8 13, label %bb5.i
    i8 14, label %bb2.i
    i8 15, label %bb3.i
  ]

bb4.i:                                            ; preds = %bb2
  %1 = getelementptr inbounds nuw i8, ptr %entry, i64 16
  %_19.i = load ptr, ptr %1, align 8, !alias.scope !27, !nonnull !2, !noundef !2
  %2 = getelementptr inbounds nuw i8, ptr %entry, i64 24
  %_18.i = load i64, ptr %2, align 8, !alias.scope !27, !noundef !2
  br label %bb3.i1

bb5.i:                                            ; preds = %bb2
  %3 = getelementptr inbounds nuw i8, ptr %entry, i64 8
  %x.0.i = load ptr, ptr %3, align 8, !alias.scope !27, !nonnull !2, !align !30, !noundef !2
  %4 = getelementptr inbounds nuw i8, ptr %entry, i64 16
  %x.1.i = load i64, ptr %4, align 8, !alias.scope !27, !noundef !2
  br label %bb3.i1

bb2.i:                                            ; preds = %bb2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_9.i), !noalias !27
  %5 = getelementptr inbounds nuw i8, ptr %entry, i64 16
  %_14.i = load ptr, ptr %5, align 8, !alias.scope !27, !nonnull !2, !noundef !2
  %6 = getelementptr inbounds nuw i8, ptr %entry, i64 24
  %_13.i = load i64, ptr %6, align 8, !alias.scope !27, !noundef !2
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_14.i, i64 noundef %_13.i), !noalias !27
  %_22.i = load i64, ptr %_9.i, align 8, !range !31, !noalias !27, !noundef !2
  %7 = trunc nuw i64 %_22.i to i1
  %8 = getelementptr inbounds nuw i8, ptr %_9.i, i64 8
  %_23.0.i = load ptr, ptr %8, align 8, !noalias !27, !nonnull !2, !align !30
  %9 = getelementptr inbounds nuw i8, ptr %_9.i, i64 16
  %_23.1.i = load i64, ptr %9, align 8, !noalias !27
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_9.i), !noalias !27
  br i1 %7, label %_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit, label %bb3.i1

bb3.i:                                            ; preds = %bb2
  %10 = getelementptr inbounds nuw i8, ptr %entry, i64 8
  %x.01.i = load ptr, ptr %10, align 8, !alias.scope !27, !nonnull !2, !align !30, !noundef !2
  %11 = getelementptr inbounds nuw i8, ptr %entry, i64 16
  %x.12.i = load i64, ptr %11, align 8, !alias.scope !27, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_7.i), !noalias !27
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_7.i, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %x.01.i, i64 noundef %x.12.i), !noalias !27
  %_20.i = load i64, ptr %_7.i, align 8, !range !31, !noalias !27, !noundef !2
  %12 = trunc nuw i64 %_20.i to i1
  %13 = getelementptr inbounds nuw i8, ptr %_7.i, i64 8
  %_21.0.i = load ptr, ptr %13, align 8, !noalias !27, !nonnull !2, !align !30
  %14 = getelementptr inbounds nuw i8, ptr %_7.i, i64 16
  %_21.1.i = load i64, ptr %14, align 8, !noalias !27
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7.i), !noalias !27
  br i1 %12, label %_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit, label %bb3.i1

_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit: ; preds = %bb3.i, %bb2.i, %bb2
  tail call void @llvm.experimental.noalias.scope.decl(metadata !32)
  br label %bb8

bb3.i1:                                           ; preds = %bb4.i, %bb5.i, %bb2.i, %bb3.i
  %_0.sroa.8.0.i.ph = phi i64 [ %_21.1.i, %bb3.i ], [ %_23.1.i, %bb2.i ], [ %x.1.i, %bb5.i ], [ %_18.i, %bb4.i ]
  %_0.sroa.0.0.i.ph = phi ptr [ %_21.0.i, %bb3.i ], [ %_23.0.i, %bb2.i ], [ %x.0.i, %bb5.i ], [ %_19.i, %bb4.i ]
  tail call void @llvm.experimental.noalias.scope.decl(metadata !35)
  %_11.idx.i.i = shl nuw nsw i64 %recognized.1, 4
  %_11.i.i = getelementptr inbounds nuw i8, ptr %recognized.0, i64 %_11.idx.i.i
  %_123.not.i.i.i = icmp eq i64 %recognized.1, 0
  br i1 %_123.not.i.i.i, label %bb8, label %bb13.i.i.i

bb13.i.i.i:                                       ; preds = %bb3.i1, %bb1.backedge.i.i.i
  %_2224.i.i.i = phi ptr [ %_22.i.i.i, %bb1.backedge.i.i.i ], [ %recognized.0, %bb3.i1 ]
  %_22.i.i.i = getelementptr inbounds nuw i8, ptr %_2224.i.i.i, i64 16
  %15 = getelementptr i8, ptr %_2224.i.i.i, i64 8
  %ptr.val1.i.i.i = load i64, ptr %15, align 8, !alias.scope !38, !noalias !39, !noundef !2
  %_3.not.i.i.i.i.i.i.i = icmp eq i64 %ptr.val1.i.i.i, %_0.sroa.8.0.i.ph
  br i1 %_3.not.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i, label %bb1.backedge.i.i.i

bb2.i.i.i.i.i.i.i:                                ; preds = %bb13.i.i.i
  %ptr.val.i.i.i = load ptr, ptr %_2224.i.i.i, align 8, !alias.scope !38, !noalias !39, !nonnull !2, !align !30, !noundef !2
  %16 = tail call i32 @memcmp(ptr nonnull readonly align 1 %ptr.val.i.i.i, ptr nonnull readonly align 1 %_0.sroa.0.0.i.ph, i64 range(i64 0, -9223372036854775808) %_0.sroa.8.0.i.ph), !alias.scope !45, !noalias !52
  %17 = icmp eq i32 %16, 0
  br i1 %17, label %bb6, label %bb1.backedge.i.i.i

bb1.backedge.i.i.i:                               ; preds = %bb2.i.i.i.i.i.i.i, %bb13.i.i.i
  %_12.not.i.i.i = icmp eq ptr %_22.i.i.i, %_11.i.i
  br i1 %_12.not.i.i.i, label %bb8, label %bb13.i.i.i

bb6:                                              ; preds = %bb2.i.i.i.i.i.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_0, ptr noundef nonnull align 8 dereferenceable(64) %entry, i64 64, i1 false)
  br label %bb8

bb8:                                              ; preds = %bb1.backedge.i.i.i, %start, %_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit, %bb3.i1, %bb6
  %entry.sink = phi ptr [ %entry, %bb6 ], [ %_0, %bb3.i1 ], [ %_0, %_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str.exit ], [ %_0, %start ], [ %_0, %bb1.backedge.i.i.i ]
  store i8 22, ptr %entry.sink, align 8
  ret void
}

; serde::private::de::content::content_clone
; Function Attrs: uwtable
define void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr dead_on_unwind noalias noundef writable writeonly sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %content) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_6.i = alloca [32 x i8], align 8
  %_5.i100 = alloca [32 x i8], align 8
  %_7.i.i.i68 = alloca [64 x i8], align 8
  %_7.i.i.i = alloca [32 x i8], align 8
  %vector.i16 = alloca [24 x i8], align 8
  %vector.i = alloca [24 x i8], align 8
  %_27 = alloca [32 x i8], align 8
  %_23 = alloca [32 x i8], align 8
  %0 = load i8, ptr %content, align 8, !range !7, !noundef !2
  switch i8 %0, label %default.unreachable123 [
    i8 0, label %bb23
    i8 1, label %bb22
    i8 2, label %bb21
    i8 3, label %bb20
    i8 4, label %bb19
    i8 5, label %bb18
    i8 6, label %bb17
    i8 7, label %bb16
    i8 8, label %bb15
    i8 9, label %bb14
    i8 10, label %bb13
    i8 11, label %bb12
    i8 12, label %bb11
    i8 13, label %bb10
    i8 14, label %bb9
    i8 15, label %bb8
    i8 16, label %bb7
    i8 17, label %bb6
    i8 18, label %bb5
    i8 19, label %bb4
    i8 20, label %bb3.i13
    i8 21, label %bb3.i22
  ]

default.unreachable123:                           ; preds = %start
  unreachable

bb23:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb22:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb21:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb20:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb19:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb18:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb17:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb16:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb15:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb14:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb13:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb12:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb11:                                             ; preds = %start
  %s = getelementptr inbounds nuw i8, ptr %content, i64 8
  %1 = getelementptr inbounds nuw i8, ptr %_0, i64 8
; call <alloc::string::String as core::clone::Clone>::clone
  tail call void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %1, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %s)
  store i8 12, ptr %_0, align 8
  br label %bb30

bb10:                                             ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb9:                                              ; preds = %start
  %2 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %b.val = load ptr, ptr %2, align 8, !nonnull !2, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %content, i64 24
  %b.val6 = load i64, ptr %3, align 8, !noundef !2
  %_23.i.i.i.i.i = icmp eq i64 %b.val6, 0
  br i1 %_23.i.i.i.i.i, label %_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i: ; preds = %bb9
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #12, !noalias !53
; call __rustc::__rust_alloc
  %4 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef range(i64 0, -9223372036854775808) %b.val6, i64 noundef range(i64 1, 9) 1) #12, !noalias !53
  %5 = icmp eq ptr %4, null
  br i1 %5, label %bb3.i.i.i, label %_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde.exit

bb3.i.i.i:                                        ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef 1, i64 range(i64 0, -9223372036854775808) %b.val6) #15, !noalias !61
  unreachable

_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde.exit: ; preds = %bb9, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i
  %_4.sroa.10.0.i.i.i = phi ptr [ inttoptr (i64 1 to ptr), %bb9 ], [ %4, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i.i.i ]
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %_4.sroa.10.0.i.i.i, ptr nonnull readonly align 1 %b.val, i64 range(i64 0, -9223372036854775808) %b.val6, i1 false), !noalias !62
  %6 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store i64 %b.val6, ptr %6, align 8
  %_19.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 16
  store ptr %_4.sroa.10.0.i.i.i, ptr %_19.sroa.4.0..sroa_idx, align 8
  %_19.sroa.5.0..sroa_idx = getelementptr inbounds nuw i8, ptr %_0, i64 24
  store i64 %b.val6, ptr %_19.sroa.5.0..sroa_idx, align 8
  store i8 14, ptr %_0, align 8
  br label %bb30

bb8:                                              ; preds = %start
  tail call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_0, ptr noundef nonnull align 8 dereferenceable(32) %content, i64 32, i1 false)
  br label %bb30

bb7:                                              ; preds = %start
  store i8 16, ptr %_0, align 8
  br label %bb30

bb6:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_23)
  %7 = getelementptr inbounds nuw i8, ptr %content, i64 8
  %_60 = load ptr, ptr %7, align 8, !nonnull !2, !noundef !2
; call serde::private::de::content::content_clone
  call void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_23, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(32) %_60)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #12, !noalias !63
; call __rustc::__rust_alloc
  %8 = tail call noundef align 8 dereferenceable_or_null(32) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 32, i64 noundef 8) #12, !noalias !63
  %9 = icmp eq ptr %8, null
  br i1 %9, label %bb2.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit5, !prof !66

bb2.i:                                            ; preds = %bb6
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 32) #15
          to label %.noexc unwind label %cleanup.i2

.noexc:                                           ; preds = %bb2.i
  unreachable

cleanup.i2:                                       ; preds = %bb2.i
  %10 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull align 8 dereferenceable(32) %_23) #10
          to label %common.resume unwind label %terminate.i3

terminate.i3:                                     ; preds = %cleanup.i2
  %11 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11
  unreachable

common.resume:                                    ; preds = %cleanup1.i26.body, %cleanup1.i.body, %cleanup.i, %cleanup.i2
  %common.resume.op = phi { ptr, i32 } [ %10, %cleanup.i2 ], [ %16, %cleanup.i ], [ %29, %cleanup1.i.body ], [ %eh.lpad-body105, %cleanup1.i26.body ]
  resume { ptr, i32 } %common.resume.op

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit5: ; preds = %bb6
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %8, ptr noundef nonnull align 8 dereferenceable(32) %_23, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_23)
  %12 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %8, ptr %12, align 8
  store i8 17, ptr %_0, align 8
  br label %bb30

bb5:                                              ; preds = %start
  store i8 18, ptr %_0, align 8
  br label %bb30

bb4:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_27)
  %13 = getelementptr inbounds nuw i8, ptr %content, i64 8
  %_59 = load ptr, ptr %13, align 8, !nonnull !2, !noundef !2
; call serde::private::de::content::content_clone
  call void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_27, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(32) %_59)
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #12, !noalias !67
; call __rustc::__rust_alloc
  %14 = tail call noundef align 8 dereferenceable_or_null(32) ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef 32, i64 noundef 8) #12, !noalias !67
  %15 = icmp eq ptr %14, null
  br i1 %15, label %bb2.i9, label %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit, !prof !66

bb2.i9:                                           ; preds = %bb4
; invoke alloc::alloc::handle_alloc_error
  invoke void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef 8, i64 noundef 32) #15
          to label %.noexc10 unwind label %cleanup.i

.noexc10:                                         ; preds = %bb2.i9
  unreachable

cleanup.i:                                        ; preds = %bb2.i9
  %16 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef nonnull align 8 dereferenceable(32) %_27) #10
          to label %common.resume unwind label %terminate.i

terminate.i:                                      ; preds = %cleanup.i
  %17 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11
  unreachable

_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit: ; preds = %bb4
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %14, ptr noundef nonnull align 8 dereferenceable(32) %_27, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_27)
  %18 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  store ptr %14, ptr %18, align 8
  store i8 19, ptr %_0, align 8
  br label %bb30

bb3.i13:                                          ; preds = %start
  %19 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %_46 = load ptr, ptr %19, align 8, !nonnull !2, !noundef !2
  %20 = getelementptr inbounds nuw i8, ptr %content, i64 24
  %_45 = load i64, ptr %20, align 8, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i), !noalias !70
  %_23.i.i.i = icmp eq i64 %_45, 0
  br i1 %_23.i.i.i, label %.noexc30.thread, label %bb6.i.i.i

.noexc30.thread:                                  ; preds = %bb3.i13
  store i64 %_45, ptr %vector.i, align 8, !noalias !70
  %21 = getelementptr inbounds nuw i8, ptr %vector.i, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %21, align 8, !noalias !70
  %22 = getelementptr inbounds nuw i8, ptr %vector.i, i64 16
  store i64 0, ptr %22, align 8, !noalias !70
  br label %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_.exit

bb6.i.i.i:                                        ; preds = %bb3.i13
  %_24.i.i.i = shl i64 %_45, 5
  %23 = add i64 %_24.i.i.i, -9223372036854775801
  %or.cond = icmp ult i64 %23, -9223372036854775769
  br i1 %or.cond, label %bb3.i32, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i, !prof !73

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i: ; preds = %bb6.i.i.i
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #12, !noalias !74
; call __rustc::__rust_alloc
  %24 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %_24.i.i.i, i64 noundef range(i64 1, 9) 8) #12, !noalias !74
  %25 = icmp eq ptr %24, null
  br i1 %25, label %bb3.i32, label %bb9.i.i.preheader

bb3.i32:                                          ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i, %bb6.i.i.i
  %_4.sroa.4.0.ph.i = phi i64 [ 8, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i ], [ 0, %bb6.i.i.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i, i64 %_24.i.i.i) #15, !noalias !70
  unreachable

bb9.i.i.preheader:                                ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i
  store i64 %_45, ptr %vector.i, align 8, !noalias !70
  %26 = getelementptr inbounds nuw i8, ptr %vector.i, i64 8
  store ptr %24, ptr %26, align 8, !noalias !70
  %27 = getelementptr inbounds nuw i8, ptr %vector.i, i64 16
  br label %bb9.i.i

bb9.i.i:                                          ; preds = %bb9.i.i.preheader, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBV_uNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3Y_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1T_EE0E0E0B21_.exit.i.i
  %_5.i.i63.sroa.6.0 = phi i64 [ %28, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBV_uNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3Y_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1T_EE0E0E0B21_.exit.i.i ], [ 0, %bb9.i.i.preheader ]
  %_46.i.i = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %_46, i64 %_5.i.i63.sroa.6.0
; invoke serde::private::de::content::content_clone
  invoke void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_7.i.i.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(32) %_46.i.i)
          to label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBV_uNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3Y_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1T_EE0E0E0B21_.exit.i.i unwind label %cleanup1.i.body

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBV_uNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3Y_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1T_EE0E0E0B21_.exit.i.i: ; preds = %bb9.i.i
  %_3.i.i = getelementptr inbounds nuw %"serde_core::private::content::Content<'_>", ptr %24, i64 %_5.i.i63.sroa.6.0
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %_3.i.i, ptr noundef nonnull align 8 dereferenceable(32) %_7.i.i.i, i64 32, i1 false)
  %28 = add nuw i64 %_5.i.i63.sroa.6.0, 1
  %_28.i.i = icmp eq i64 %28, %_45
  br i1 %_28.i.i, label %bb11.i.i, label %bb9.i.i

bb11.i.i:                                         ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBV_uNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneNCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB3Y_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1T_EE0E0E0B21_.exit.i.i
  store i64 %_45, ptr %27, align 8, !noalias !77
  br label %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_.exit

cleanup1.i.body:                                  ; preds = %bb9.i.i
  %29 = landingpad { ptr, i32 }
          cleanup
  store i64 %_5.i.i63.sroa.6.0, ptr %27, align 8, !noalias !77
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<serde_core::private::content::Content>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(24) %vector.i) #10
          to label %common.resume unwind label %terminate.i14, !noalias !70

terminate.i14:                                    ; preds = %cleanup1.i.body
  %30 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !70
  unreachable

_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_.exit: ; preds = %.noexc30.thread, %bb11.i.i
  %31 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %31, ptr noundef nonnull align 8 dereferenceable(24) %vector.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i), !noalias !70
  store i8 20, ptr %_0, align 8
  br label %bb30

bb3.i22:                                          ; preds = %start
  %32 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %_42 = load ptr, ptr %32, align 8, !nonnull !2, !noundef !2
  %33 = getelementptr inbounds nuw i8, ptr %content, i64 24
  %_41 = load i64, ptr %33, align 8, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %vector.i16), !noalias !82
  %_23.i.i.i45 = icmp eq i64 %_41, 0
  br i1 %_23.i.i.i45, label %.noexc43.thread, label %bb6.i.i.i46

.noexc43.thread:                                  ; preds = %bb3.i22
  store i64 %_41, ptr %vector.i16, align 8, !noalias !82
  %34 = getelementptr inbounds nuw i8, ptr %vector.i16, i64 8
  store ptr inttoptr (i64 8 to ptr), ptr %34, align 8, !noalias !82
  %35 = getelementptr inbounds nuw i8, ptr %vector.i16, i64 16
  store i64 0, ptr %35, align 8, !noalias !82
  br label %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_.exit

bb6.i.i.i46:                                      ; preds = %bb3.i22
  %_24.i.i.i47 = shl i64 %_41, 6
  %36 = add i64 %_24.i.i.i47, -9223372036854775801
  %or.cond112 = icmp ult i64 %36, -9223372036854775737
  br i1 %or.cond112, label %bb3.i58, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i55, !prof !73

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i55: ; preds = %bb6.i.i.i46
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #12, !noalias !85
; call __rustc::__rust_alloc
  %37 = tail call noundef align 8 ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %_24.i.i.i47, i64 noundef range(i64 1, 9) 8) #12, !noalias !85
  %38 = icmp eq ptr %37, null
  br i1 %38, label %bb3.i58, label %bb9.i.i72.preheader

bb3.i58:                                          ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i55, %bb6.i.i.i46
  %_4.sroa.4.0.ph.i59 = phi i64 [ 8, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i55 ], [ 0, %bb6.i.i.i46 ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_4.sroa.4.0.ph.i59, i64 %_24.i.i.i47) #15, !noalias !82
  unreachable

bb9.i.i72.preheader:                              ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator8allocate.exit.i.i55
  store i64 %_41, ptr %vector.i16, align 8, !noalias !82
  %39 = getelementptr inbounds nuw i8, ptr %vector.i16, i64 8
  store ptr %37, ptr %39, align 8, !noalias !82
  %40 = getelementptr inbounds nuw i8, ptr %vector.i16, i64 16
  %41 = getelementptr inbounds nuw i8, ptr %_7.i.i.i68, i64 32
  br label %bb9.i.i72

cleanup.i.i76:                                    ; preds = %bb9.i.i72
  %42 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup1.i26.body

bb9.i.i72:                                        ; preds = %bb9.i.i72.preheader, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBW_EBV_uNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB46_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1Y_EE0E0E0B28_.exit.i.i
  %_5.i.i70.sroa.6.0 = phi i64 [ %45, %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBW_EBV_uNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB46_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1Y_EE0E0E0B28_.exit.i.i ], [ 0, %bb9.i.i72.preheader ]
  %_46.i.i74 = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %_42, i64 %_5.i.i70.sroa.6.0
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_5.i100), !noalias !88
; invoke serde::private::de::content::content_clone
  invoke void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_5.i100, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(64) %_46.i.i74)
          to label %.noexc104 unwind label %cleanup.i.i76

.noexc104:                                        ; preds = %bb9.i.i72
  %v.i = getelementptr inbounds nuw i8, ptr %_46.i.i74, i64 32
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_6.i), !noalias !88
; invoke serde::private::de::content::content_clone
  invoke void @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone(ptr noalias noundef nonnull sret([32 x i8]) align 8 captures(none) dereferenceable(32) %_6.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(32) %v.i)
          to label %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBW_EBV_uNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB46_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1Y_EE0E0E0B28_.exit.i.i unwind label %cleanup.i101, !noalias !96

cleanup.i101:                                     ; preds = %.noexc104
  %43 = landingpad { ptr, i32 }
          cleanup
; invoke core::ptr::drop_in_place::<serde_core::private::content::Content>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(32) %_5.i100) #10
          to label %cleanup1.i26.body unwind label %terminate.i102, !noalias !96

terminate.i102:                                   ; preds = %cleanup.i101
  %44 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !96
  unreachable

_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBW_EBV_uNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB46_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1Y_EE0E0E0B28_.exit.i.i: ; preds = %.noexc104
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_7.i.i.i68, ptr noundef nonnull align 8 dereferenceable(32) %_5.i100, i64 32, i1 false)
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(32) %41, ptr noundef nonnull align 8 dereferenceable(32) %_6.i, i64 32, i1 false)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_6.i), !noalias !88
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_5.i100), !noalias !88
  %_3.i.i99 = getelementptr inbounds nuw { %"serde_core::private::content::Content<'_>", %"serde_core::private::content::Content<'_>" }, ptr %37, i64 %_5.i.i70.sroa.6.0
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(64) %_3.i.i99, ptr noundef nonnull align 8 dereferenceable(64) %_7.i.i.i68, i64 64, i1 false)
  %45 = add nuw i64 %_5.i.i70.sroa.6.0, 1
  %_28.i.i80 = icmp eq i64 %45, %_41
  br i1 %_28.i.i80, label %bb11.i.i81, label %bb9.i.i72

bb11.i.i81:                                       ; preds = %_RNCINvNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map8map_foldRTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBW_EBV_uNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0NCINvNvNtNtNtB8_6traits8iterator8Iterator8for_each4callBV_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB46_3VecBV_E14extend_trustedINtB4_3MapINtNtNtBa_5slice4iter4IterBV_EB1Y_EE0E0E0B28_.exit.i.i
  store i64 %_41, ptr %40, align 8, !noalias !97
  br label %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_.exit

cleanup1.i26.body:                                ; preds = %cleanup.i.i76, %cleanup.i101
  %eh.lpad-body105 = phi { ptr, i32 } [ %42, %cleanup.i.i76 ], [ %43, %cleanup.i101 ]
  store i64 %_5.i.i70.sroa.6.0, ptr %40, align 8, !noalias !97
; invoke core::ptr::drop_in_place::<alloc::vec::Vec<(serde_core::private::content::Content, serde_core::private::content::Content)>>
  invoke fastcc void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde(ptr noalias noundef align 8 dereferenceable(24) %vector.i16) #10
          to label %common.resume unwind label %terminate.i27, !noalias !82

terminate.i27:                                    ; preds = %cleanup1.i26.body
  %46 = landingpad { ptr, i32 }
          filter [0 x ptr] zeroinitializer
; call core::panicking::panic_in_cleanup
  tail call void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() #11, !noalias !82
  unreachable

_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_.exit: ; preds = %.noexc43.thread, %bb11.i.i81
  %47 = getelementptr inbounds nuw i8, ptr %_0, i64 8
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %47, ptr noundef nonnull align 8 dereferenceable(24) %vector.i16, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %vector.i16), !noalias !82
  store i8 21, ptr %_0, align 8
  br label %bb30

bb30:                                             ; preds = %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_.exit, %_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_.exit, %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit, %bb5, %_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde.exit5, %bb7, %bb8, %_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde.exit, %bb10, %bb11, %bb12, %bb13, %bb14, %bb15, %bb16, %bb17, %bb18, %bb19, %bb20, %bb21, %bb22, %bb23
  ret void
}

; serde::private::de::content::content_as_str
; Function Attrs: uwtable
define { ptr, i64 } @_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str(ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %content) unnamed_addr #0 {
start:
  %_9 = alloca [24 x i8], align 8
  %_7 = alloca [24 x i8], align 8
  %0 = load i8, ptr %content, align 8, !range !7, !noundef !2
  switch i8 %0, label %bb8 [
    i8 12, label %bb4
    i8 13, label %bb5
    i8 14, label %bb2
    i8 15, label %bb3
  ]

bb4:                                              ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %_19 = load ptr, ptr %1, align 8, !nonnull !2, !noundef !2
  %2 = getelementptr inbounds nuw i8, ptr %content, i64 24
  %_18 = load i64, ptr %2, align 8, !noundef !2
  br label %bb8

bb5:                                              ; preds = %start
  %3 = getelementptr inbounds nuw i8, ptr %content, i64 8
  %x.0 = load ptr, ptr %3, align 8, !nonnull !2, !align !30, !noundef !2
  %4 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %x.1 = load i64, ptr %4, align 8, !noundef !2
  br label %bb8

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_9)
  %5 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %_14 = load ptr, ptr %5, align 8, !nonnull !2, !noundef !2
  %6 = getelementptr inbounds nuw i8, ptr %content, i64 24
  %_13 = load i64, ptr %6, align 8, !noundef !2
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_9, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_14, i64 noundef %_13)
  %_22 = load i64, ptr %_9, align 8, !range !31, !noundef !2
  %7 = trunc nuw i64 %_22 to i1
  %8 = getelementptr inbounds nuw i8, ptr %_9, i64 8
  %_23.0 = load ptr, ptr %8, align 8, !nonnull !2, !align !30
  %9 = getelementptr inbounds nuw i8, ptr %_9, i64 16
  %_23.1 = load i64, ptr %9, align 8
  %_0.sroa.8.1 = select i1 %7, i64 undef, i64 %_23.1
  %_0.sroa.0.1 = select i1 %7, ptr null, ptr %_23.0
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_9)
  br label %bb8

bb3:                                              ; preds = %start
  %10 = getelementptr inbounds nuw i8, ptr %content, i64 8
  %x.01 = load ptr, ptr %10, align 8, !nonnull !2, !align !30, !noundef !2
  %11 = getelementptr inbounds nuw i8, ptr %content, i64 16
  %x.12 = load i64, ptr %11, align 8, !noundef !2
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %_7)
; call core::str::converts::from_utf8
  call void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr noalias noundef nonnull sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_7, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %x.01, i64 noundef %x.12)
  %_20 = load i64, ptr %_7, align 8, !range !31, !noundef !2
  %12 = trunc nuw i64 %_20 to i1
  %13 = getelementptr inbounds nuw i8, ptr %_7, i64 8
  %_21.0 = load ptr, ptr %13, align 8, !nonnull !2, !align !30
  %14 = getelementptr inbounds nuw i8, ptr %_7, i64 16
  %_21.1 = load i64, ptr %14, align 8
  %_0.sroa.8.2 = select i1 %12, i64 undef, i64 %_21.1
  %_0.sroa.0.2 = select i1 %12, ptr null, ptr %_21.0
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %_7)
  br label %bb8

bb8:                                              ; preds = %start, %bb3, %bb2, %bb5, %bb4
  %_0.sroa.8.0 = phi i64 [ %_18, %bb4 ], [ %x.1, %bb5 ], [ %_0.sroa.8.1, %bb2 ], [ %_0.sroa.8.2, %bb3 ], [ undef, %start ]
  %_0.sroa.0.0 = phi ptr [ %_19, %bb4 ], [ %x.0, %bb5 ], [ %_0.sroa.0.1, %bb2 ], [ %_0.sroa.0.2, %bb3 ], [ null, %start ]
  %15 = insertvalue { ptr, i64 } poison, ptr %_0.sroa.0.0, 0
  %16 = insertvalue { ptr, i64 } %15, i64 %_0.sroa.8.0, 1
  ret { ptr, i64 } %16
}

; <serde::private::de::borrow_cow_str::CowStrVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXNvNtNtCs6Ufq8Na2eJH_5serde7private2de14borrow_cow_strNtB2_13CowStrVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a32ee92aa163a424fb5f24d7445c00a9, i64 noundef 8)
  ret i1 %_0
}

; <serde::private::de::borrow_cow_bytes::CowBytesVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXNvNtNtCs6Ufq8Na2eJH_5serde7private2de16borrow_cow_bytesNtB2_15CowBytesVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4b2e1ac4e77881e01aa50df408195ab8, i64 noundef 12)
  ret i1 %_0
}

; <&str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !30, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
; call <str as core::fmt::Debug>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <&str as core::fmt::Display>::fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6Ufq8Na2eJH_5serde(ptr noalias noundef readonly align 8 captures(none) dereferenceable(16) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #0 {
start:
  %_3.0 = load ptr, ptr %self, align 8, !nonnull !2, !align !30, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_3.1 = load i64, ptr %0, align 8, !noundef !2
; call <str as core::fmt::Display>::fmt
  %_0 = tail call noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_3.0, i64 noundef %_3.1, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  ret i1 %_0
}

; <serde::private::ser::Unsupported as core::fmt::Display>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXs2_NtNtCs6Ufq8Na2eJH_5serde7private3serNtB5_11UnsupportedNtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt(ptr noalias noundef readonly align 1 captures(none) dereferenceable(1) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %0 = load i8, ptr %self, align 1, !range !98, !noundef !2
  switch i8 %0, label %default.unreachable1 [
    i8 0, label %bb11
    i8 1, label %bb10
    i8 2, label %bb9
    i8 3, label %bb8
    i8 4, label %bb7
    i8 5, label %bb6
    i8 6, label %bb5
    i8 7, label %bb4
    i8 8, label %bb3
    i8 9, label %bb2
  ]

default.unreachable1:                             ; preds = %start
  unreachable

bb11:                                             ; preds = %start
; call <core::fmt::Formatter>::write_str
  %1 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_64c46d602bcdf61cecebd9c07a8ffbde, i64 noundef 9)
  br label %bb12

bb10:                                             ; preds = %start
; call <core::fmt::Formatter>::write_str
  %2 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a45aab79b1349dc8dcfa9a0871485b38, i64 noundef 10)
  br label %bb12

bb9:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %3 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_d65406d2c9a3b2f5e537fca7bc28c259, i64 noundef 7)
  br label %bb12

bb8:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %4 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_0f28191399eb63227c9545128039b625, i64 noundef 6)
  br label %bb12

bb7:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %5 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_a32ee92aa163a424fb5f24d7445c00a9, i64 noundef 8)
  br label %bb12

bb6:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %6 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_4b2e1ac4e77881e01aa50df408195ab8, i64 noundef 12)
  br label %bb12

bb5:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %7 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_55183e09502581a3b4be406985e5433a, i64 noundef 11)
  br label %bb12

bb4:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %8 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_c705cb38a8523735dfb9d88c7a0743fd, i64 noundef 10)
  br label %bb12

bb3:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %9 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_42b547da467ce6ab10816f050e5805ac, i64 noundef 7)
  br label %bb12

bb2:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %10 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_aa9f3e4e91af539e6d709a9ddaa9a5d6, i64 noundef 14)
  br label %bb12

bb12:                                             ; preds = %bb2, %bb3, %bb4, %bb5, %bb6, %bb7, %bb8, %bb9, %bb10, %bb11
  %_0.sroa.0.0.in = phi i1 [ %1, %bb11 ], [ %2, %bb10 ], [ %3, %bb9 ], [ %4, %bb8 ], [ %5, %bb7 ], [ %6, %bb6 ], [ %7, %bb5 ], [ %8, %bb4 ], [ %9, %bb3 ], [ %10, %bb2 ]
  ret i1 %_0.sroa.0.0.in
}

; <serde::private::de::content::ExpectedInMap as serde_core::de::Expected>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsA_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_13ExpectedInMapNtNtCsasqbmkB9jT1_10serde_core2de8Expected3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  %_3 = load i64, ptr %self, align 8, !noundef !2
  %0 = icmp eq i64 %_3, 1
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %1 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b8f0e9f5a55e3399052e7f51171d5f73, i64 noundef 16)
  br label %bb3

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_7.sroa.4.0..sroa_idx, align 8
  %_20.0 = load ptr, ptr %formatter, align 8, !nonnull !2, !align !30, !noundef !2
  %2 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_20.1 = load ptr, ptr %2, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %3 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_20.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_20.1, ptr noundef nonnull @alloc_bb2978a5d2b5523fedc4230268ea2bca, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_0.sroa.0.0.in = phi i1 [ %1, %bb1 ], [ %3, %bb2 ]
  ret i1 %_0.sroa.0.0.in
}

; <serde::private::de::content::InternallyTaggedUnitVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsR_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_27InternallyTaggedUnitVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %args = alloca [32 x i8], align 8
  %args1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6Ufq8Na2eJH_5serde, ptr %_7.sroa.4.0..sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %args1, ptr %0, align 8
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6Ufq8Na2eJH_5serde, ptr %_8.sroa.4.0..sroa_idx, align 8
  %_27.0 = load ptr, ptr %formatter, align 8, !nonnull !2, !align !30, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_27.1 = load ptr, ptr %1, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %2 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_27.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_27.1, ptr noundef nonnull @alloc_38b751980a67eae055f527bc65124ed1, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  ret i1 %2
}

; <serde::private::de::content::ContentVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsj_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_14ContentVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %fmt) unnamed_addr #0 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %fmt, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_b4672af18d56d6b08faa87389daaee94, i64 noundef 9)
  ret i1 %_0
}

; <serde::private::de::content::TagOrContentVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsl_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_19TagOrContentVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(16) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %fmt) unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_6.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1i_NtCsjMrxcFdYDNN_4core3fmtReNtB6_7Display3fmtCs6Ufq8Na2eJH_5serde, ptr %_6.sroa.4.0..sroa_idx, align 8
  %_19.0 = load ptr, ptr %fmt, align 8, !nonnull !2, !align !30, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %fmt, i64 8
  %_19.1 = load ptr, ptr %0, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %1 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_19.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_19.1, ptr noundef nonnull @alloc_b8b61a38e78c2df660def71a70f41227, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  ret i1 %1
}

; <serde::private::de::content::TagOrContentFieldVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXso_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_24TagOrContentFieldVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %args = alloca [32 x i8], align 8
  %args1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCs6Ufq8Na2eJH_5serde, ptr %_7.sroa.4.0..sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %args1, ptr %0, align 8
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCs6Ufq8Na2eJH_5serde, ptr %_8.sroa.4.0..sroa_idx, align 8
  %_27.0 = load ptr, ptr %formatter, align 8, !nonnull !2, !align !30, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_27.1 = load ptr, ptr %1, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %2 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_27.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_27.1, ptr noundef nonnull @alloc_47bf8b2623f12665ad0eb57968a6fa8a, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  ret i1 %2
}

; <serde::private::de::content::TagContentOtherFieldVisitor as serde_core::de::Visitor>::expecting
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsq_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_27TagContentOtherFieldVisitorNtNtCsasqbmkB9jT1_10serde_core2de7Visitor9expecting(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) %self, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %args = alloca [32 x i8], align 8
  %args1 = getelementptr inbounds nuw i8, ptr %self, i64 16
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCs6Ufq8Na2eJH_5serde, ptr %_7.sroa.4.0..sroa_idx, align 8
  %0 = getelementptr inbounds nuw i8, ptr %args, i64 16
  store ptr %args1, ptr %0, align 8
  %_8.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 24
  store ptr @_RNvXs1g_NtCsjMrxcFdYDNN_4core3fmtReNtB6_5Debug3fmtCs6Ufq8Na2eJH_5serde, ptr %_8.sroa.4.0..sroa_idx, align 8
  %_27.0 = load ptr, ptr %formatter, align 8, !nonnull !2, !align !30, !noundef !2
  %1 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_27.1 = load ptr, ptr %1, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %2 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_27.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_27.1, ptr noundef nonnull @alloc_7f44bc6f09057a780564215ee717c5f7, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %args)
  ret i1 %2
}

; <serde::private::de::content::ExpectedInSeq as serde_core::de::Expected>::fmt
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvXsu_NtNtNtCs6Ufq8Na2eJH_5serde7private2de7contentNtB5_13ExpectedInSeqNtNtCsasqbmkB9jT1_10serde_core2de8Expected3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8) %self, ptr noalias noundef align 8 dereferenceable(24) %formatter) unnamed_addr #0 {
start:
  %args = alloca [16 x i8], align 8
  %_3 = load i64, ptr %self, align 8, !noundef !2
  %0 = icmp eq i64 %_3, 1
  br i1 %0, label %bb1, label %bb2

bb1:                                              ; preds = %start
; call <core::fmt::Formatter>::write_str
  %1 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %formatter, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_e71a28b1125008ec4c9de9e536a49431, i64 noundef 21)
  br label %bb3

bb2:                                              ; preds = %start
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %self, ptr %args, align 8
  %_7.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt, ptr %_7.sroa.4.0..sroa_idx, align 8
  %_20.0 = load ptr, ptr %formatter, align 8, !nonnull !2, !align !30, !noundef !2
  %2 = getelementptr inbounds nuw i8, ptr %formatter, i64 8
  %_20.1 = load ptr, ptr %2, align 8, !nonnull !2, !align !99, !noundef !2
; call core::fmt::write
  %3 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_20.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_20.1, ptr noundef nonnull @alloc_8c1db4387b4656bb4b356dc02c3f8d30, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  br label %bb3

bb3:                                              ; preds = %bb2, %bb1
  %_0.sroa.0.0.in = phi i1 [ %1, %bb1 ], [ %3, %bb2 ]
  ret i1 %_0.sroa.0.0.in
}

; Function Attrs: nounwind uwtable
declare noundef range(i32 0, 10) i32 @rust_eh_personality(i32 noundef, i32 noundef, i64 noundef, ptr noundef, ptr noundef) unnamed_addr #1

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #2

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #2

; core::panicking::panic_in_cleanup
; Function Attrs: cold minsize noinline noreturn nounwind optsize uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core9panicking16panic_in_cleanup() unnamed_addr #3

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #4

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #5

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #6

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #1

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #7

; alloc::alloc::handle_alloc_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc5alloc18handle_alloc_error(i64 noundef range(i64 1, -9223372036854775807), i64 noundef) unnamed_addr #4

; <alloc::string::String as core::clone::Clone>::clone
; Function Attrs: uwtable
declare void @_RNvXs4_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core5clone5Clone5clone(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #0

; core::str::converts::from_utf8
; Function Attrs: uwtable
declare void @_RNvNtNtCsjMrxcFdYDNN_4core3str8converts9from_utf8(ptr dead_on_unwind noalias noundef writable sret([24 x i8]) align 8 captures(none) dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef range(i64 0, -9223372036854775808)) unnamed_addr #0

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #0

; <str as core::fmt::Debug>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsh_NtCsjMrxcFdYDNN_4core3fmteNtB5_5Debug3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; <str as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtCsjMrxcFdYDNN_4core3fmteNtB5_7Display3fmt(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: read)
declare i32 @memcmp(ptr captures(none), ptr captures(none), i64) local_unnamed_addr #8

; <usize as core::fmt::Display>::fmt
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsi_NtNtNtCsjMrxcFdYDNN_4core3fmt3num3impjNtB9_7Display3fmt(ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(8), ptr noalias noundef align 8 dereferenceable(24)) unnamed_addr #0

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #0

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #9

attributes #0 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #3 = { cold minsize noinline noreturn nounwind optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #6 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #7 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #8 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: read) }
attributes #9 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #10 = { cold }
attributes #11 = { cold noreturn nounwind }
attributes #12 = { nounwind }
attributes #13 = { "function-inline-cost-multiplier"="2" }
attributes #14 = { cold "function-inline-cost-multiplier"="2" }
attributes #15 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{}
!3 = !{i64 0, i64 -9223372036854775808}
!4 = !{!5}
!5 = distinct !{!5, !6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBH_EECs6Ufq8Na2eJH_5serde: %_1"}
!6 = distinct !{!6, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBH_EECs6Ufq8Na2eJH_5serde"}
!7 = !{i8 0, i8 22}
!8 = !{!9}
!9 = distinct !{!9, !10, !"_RNvXso_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBG_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6Ufq8Na2eJH_5serde: %self"}
!10 = distinct !{!10, !"_RNvXso_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentBG_EENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6Ufq8Na2eJH_5serde"}
!11 = !{!12}
!12 = distinct !{!12, !13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde: %_1"}
!13 = distinct !{!13, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1e_EEECs6Ufq8Na2eJH_5serde"}
!14 = !{!15}
!15 = distinct !{!15, !16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde: %_1"}
!16 = distinct !{!16, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde"}
!17 = !{!18}
!18 = distinct !{!18, !19, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde: %_1"}
!19 = distinct !{!19, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc5boxed3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde"}
!20 = !{!21}
!21 = distinct !{!21, !22, !"_RNvXso_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6Ufq8Na2eJH_5serde: %self"}
!22 = distinct !{!22, !"_RNvXso_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENtNtNtCsjMrxcFdYDNN_4core3ops4drop4Drop4dropCs6Ufq8Na2eJH_5serde"}
!23 = !{!24}
!24 = distinct !{!24, !25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde: %_1"}
!25 = distinct !{!25, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEECs6Ufq8Na2eJH_5serde"}
!26 = !{i8 0, i8 23}
!27 = !{!28}
!28 = distinct !{!28, !29, !"_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str: %content"}
!29 = distinct !{!29, !"_RNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content14content_as_str"}
!30 = !{i64 1}
!31 = !{i64 0, i64 2}
!32 = !{!33}
!33 = distinct !{!33, !34, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionReE6map_orbNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0EB11_: %f.0"}
!34 = distinct !{!34, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionReE6map_orbNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0EB11_"}
!35 = !{!36}
!36 = distinct !{!36, !37, !"_RNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0B7_: %_1.0"}
!37 = distinct !{!37, !"_RNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0B7_"}
!38 = !{!36, !33}
!39 = !{!40, !42, !43, !44}
!40 = distinct !{!40, !41, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs6Ufq8Na2eJH_5serde: %self"}
!41 = distinct !{!41, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs6Ufq8Na2eJH_5serde"}
!42 = distinct !{!42, !41, !"_RINvXs2E_NtNtCsjMrxcFdYDNN_4core5slice4iterINtB7_4IterReENtNtNtNtBb_4iter6traits8iterator8Iterator3anyNCNvXsf_NtB9_3cmpBQ_NtB1K_13SliceContains14slice_contains0ECs6Ufq8Na2eJH_5serde: argument 1"}
!43 = distinct !{!43, !37, !"_RNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0B7_: argument 1"}
!44 = distinct !{!44, !34, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionReE6map_orbNCNvNtNtCs6Ufq8Na2eJH_5serde7private2de19flat_map_take_entry0EB11_: argument 0"}
!45 = !{!46, !48, !49, !51}
!46 = distinct !{!46, !47, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6Ufq8Na2eJH_5serde: %self.0"}
!47 = distinct !{!47, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6Ufq8Na2eJH_5serde"}
!48 = distinct !{!48, !47, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core5slice3cmpShINtB5_14SlicePartialEqhE5equalCs6Ufq8Na2eJH_5serde: %other.0"}
!49 = distinct !{!49, !50, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %self.0"}
!50 = distinct !{!50, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq"}
!51 = distinct !{!51, !50, !"_RNvXs_NtNtCsjMrxcFdYDNN_4core3str6traitseNtNtB8_3cmp9PartialEq2eq: %other.0"}
!52 = !{!40, !42, !36, !33}
!53 = !{!54, !56, !58, !59}
!54 = distinct !{!54, !55, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde: %_0"}
!55 = distinct !{!55, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde"}
!56 = distinct !{!56, !57, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECs6Ufq8Na2eJH_5serde: %v"}
!57 = distinct !{!57, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECs6Ufq8Na2eJH_5serde"}
!58 = distinct !{!58, !57, !"_RINvXs_NvMNtCsdJPVW0sQgAG_5alloc5sliceSp9to_vec_inhNtB5_10ConvertVec6to_vecNtNtBa_5alloc6GlobalECs6Ufq8Na2eJH_5serde: %s.0"}
!59 = distinct !{!59, !60, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde: %_0"}
!60 = distinct !{!60, !"_RNvXsa_NtCsdJPVW0sQgAG_5alloc3vecINtB5_3VechENtNtCsjMrxcFdYDNN_4core5clone5Clone5cloneCs6Ufq8Na2eJH_5serde"}
!61 = !{!56, !58, !59}
!62 = !{!56, !59}
!63 = !{!64}
!64 = distinct !{!64, !65, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde: %x"}
!65 = distinct !{!65, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde"}
!66 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!67 = !{!68}
!68 = distinct !{!68, !69, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde: %x"}
!69 = distinct !{!69, !"_RNvMNtCsdJPVW0sQgAG_5alloc5boxedINtB2_3BoxNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentE3newCs6Ufq8Na2eJH_5serde"}
!70 = !{!71}
!71 = distinct !{!71, !72, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_: %_0"}
!72 = distinct !{!72, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2B_5slice4iter4IterB13_ENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneEE9from_iterB3U_"}
!73 = !{!"branch_weights", i32 6004, i32 2000}
!74 = !{!75, !71}
!75 = distinct !{!75, !76, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde: %_0"}
!76 = distinct !{!76, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde"}
!77 = !{!78, !80}
!78 = distinct !{!78, !79, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callB1n_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4z_3VecB1n_E14extend_trustedBN_E0E0EB2q_: %g"}
!79 = distinct !{!79, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3j_8for_each4callB1n_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4z_3VecB1n_E14extend_trustedBN_E0E0EB2q_"}
!80 = distinct !{!80, !81, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtBc_5slice4iter4IterNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB43_3VecB1h_E14extend_trustedB3_E0EB2k_: %f"}
!81 = distinct !{!81, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtBc_5slice4iter4IterNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentENvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_cloneENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB43_3VecB1h_E14extend_trustedB3_E0EB2k_"}
!82 = !{!83}
!83 = distinct !{!83, !84, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_: %_0"}
!84 = distinct !{!84, !"_RNvXs_NtNtCsdJPVW0sQgAG_5alloc3vec21spec_from_iter_nestedINtB6_3VecTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB14_EEINtB4_18SpecFromIterNestedB13_INtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtB2H_5slice4iter4IterB13_ENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0EE9from_iterB42_"}
!85 = !{!86, !83}
!86 = distinct !{!86, !87, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde: %_0"}
!87 = distinct !{!87, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner15try_allocate_inCs6Ufq8Na2eJH_5serde"}
!88 = !{!89, !91, !92, !94}
!89 = distinct !{!89, !90, !"_RNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0B9_: %_0"}
!90 = distinct !{!90, !"_RNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0B9_"}
!91 = distinct !{!91, !90, !"_RNCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0B9_: %_2"}
!92 = distinct !{!92, !93, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1o_EENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3s_8for_each4callB1n_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4I_3VecB1n_E14extend_trustedBN_E0E0EB2y_: %g"}
!93 = distinct !{!93, !"_RINvXs0_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters3mapINtB6_3MapINtNtNtBc_5slice4iter4IterTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1o_EENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0ENtNtNtBa_6traits8iterator8Iterator4folduNCINvNvB3s_8for_each4callB1n_NCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4I_3VecB1n_E14extend_trustedBN_E0E0EB2y_"}
!94 = distinct !{!94, !95, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtBc_5slice4iter4IterTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1i_EENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4c_3VecB1h_E14extend_trustedB3_E0EB2s_: %f"}
!95 = distinct !{!95, !"_RINvYINtNtNtNtCsjMrxcFdYDNN_4core4iter8adapters3map3MapINtNtNtBc_5slice4iter4IterTNtNtNtCsasqbmkB9jT1_10serde_core7private7content7ContentB1i_EENCNvNtNtNtCs6Ufq8Na2eJH_5serde7private2de7content13content_clone0ENtNtNtBa_6traits8iterator8Iterator8for_eachNCINvMsj_NtCsdJPVW0sQgAG_5alloc3vecINtB4c_3VecB1h_E14extend_trustedB3_E0EB2s_"}
!96 = !{!89, !92, !94}
!97 = !{!92, !94}
!98 = !{i8 0, i8 10}
!99 = !{i64 8}
