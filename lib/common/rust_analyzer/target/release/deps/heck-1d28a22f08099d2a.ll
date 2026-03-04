; ModuleID = 'heck.1a1dfad340048966-cgu.0'
source_filename = "heck.1a1dfad340048966-cgu.0"
target datalayout = "e-m:o-p270:32:32-p271:32:32-p272:64:64-i64:64-i128:128-n32:64-S128-Fn32"
target triple = "arm64-apple-macosx11.0.0"

@alloc_cea3b8959a83461a5d607431cd4e6334 = private unnamed_addr constant [94 x i8] c"/Users/hongyaotang/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/heck-0.5.0/src/lib.rs\00", align 1
@alloc_13b4bf5d328eab9b8df3f7027955d33b = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_cea3b8959a83461a5d607431cd4e6334, [16 x i8] c"]\00\00\00\00\00\00\00\98\00\00\00 \00\00\00" }>, align 8
@alloc_308f8938e9762d461fc13622ca97483f = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_cea3b8959a83461a5d607431cd4e6334, [16 x i8] c"]\00\00\00\00\00\00\00\89\00\00\00$\00\00\00" }>, align 8
@alloc_a50a5070980a36c272121b09121457de = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_cea3b8959a83461a5d607431cd4e6334, [16 x i8] c"]\00\00\00\00\00\00\00|\00\00\00$\00\00\00" }>, align 8
@alloc_0c812808379efded5a4fb82d2790b556 = private unnamed_addr constant [2 x i8] c"\C0\00", align 1
@alloc_4e2f5c99210b7b3b4346cc992c5fffc9 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_cea3b8959a83461a5d607431cd4e6334, [16 x i8] c"]\00\00\00\00\00\00\00\BB\00\00\00\19\00\00\00" }>, align 8
@alloc_ce246300cd91ffc3b77a40cb1bee254d = private unnamed_addr constant [2 x i8] c"\CF\82", align 1
@vtable.0 = private unnamed_addr constant <{ [24 x i8], ptr }> <{ [24 x i8] c"\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\00\01\00\00\00\00\00\00\00", ptr @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt }>, align 8
@vtable.1 = private unnamed_addr constant <{ ptr, [16 x i8], ptr, ptr, ptr }> <{ ptr @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck, [16 x i8] c"\18\00\00\00\00\00\00\00\08\00\00\00\00\00\00\00", ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str, ptr @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char, ptr @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs2f1hMIlW3Va_4heck }>, align 8
@alloc_cc656815297f75969399c3f4b1ad3de4 = private unnamed_addr constant [55 x i8] c"a Display implementation returned an error unexpectedly", align 1
@alloc_82687d2573a6ed8a4d1fcf733f159466 = private unnamed_addr constant [116 x i8] c"/Users/hongyaotang/.rustup/toolchains/nightly-aarch64-apple-darwin/lib/rustlib/src/rust/library/alloc/src/string.rs\00", align 1
@alloc_f3c70bf9d2724ff8f638341943ddf3c8 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc_82687d2573a6ed8a4d1fcf733f159466, [16 x i8] c"s\00\00\00\00\00\00\00f\0B\00\00\0E\00\00\00" }>, align 8
@alloc_99ac8a81a24cac863217ce4a5cbfabea = private unnamed_addr constant [5 x i8] c"Error", align 1

; core::ptr::drop_in_place::<alloc::string::String>
; Function Attrs: nounwind uwtable
define internal void @_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck(ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %_1) unnamed_addr #0 personality ptr @rust_eh_personality {
start:
  %_1.val = load i64, ptr %_1, align 8
  %0 = icmp eq i64 %_1.val, 0
  br i1 %0, label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2f1hMIlW3Va_4heck.exit, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i: ; preds = %start
  %1 = getelementptr inbounds nuw i8, ptr %_1, i64 8
  %_1.val1 = load ptr, ptr %1, align 8, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  tail call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1, i64 noundef %_1.val, i64 noundef range(i64 1, -9223372036854775807) 1) #17
  br label %_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2f1hMIlW3Va_4heck.exit

_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeINtNtCsdJPVW0sQgAG_5alloc3vec3VechEECs2f1hMIlW3Va_4heck.exit: ; preds = %start, %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i
  ret void
}

; <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
; Function Attrs: cold uwtable
define internal fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 captures(none) dereferenceable(16) %slf, i64 noundef %len, i64 noundef %additional) unnamed_addr #1 personality ptr @rust_eh_personality {
start:
  %self3.i = alloca [24 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !3)
  %_25.0.i = add i64 %additional, %len
  %_25.1.i = icmp ult i64 %_25.0.i, %len
  br i1 %_25.1.i, label %bb2, label %bb9.i

bb9.i:                                            ; preds = %start
  %self5.i = load i64, ptr %slf, align 8, !range !6, !alias.scope !3, !noundef !2
  %v16.i = shl nuw i64 %self5.i, 1
  %_0.sroa.0.0.i.i = tail call noundef i64 @llvm.umax.i64(i64 %_25.0.i, i64 range(i64 0, -1) %v16.i)
  %_0.sroa.0.0.i16.i = tail call noundef i64 @llvm.umax.i64(i64 %_0.sroa.0.0.i.i, i64 8)
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %self3.i), !noalias !3
  %0 = getelementptr inbounds nuw i8, ptr %slf, i64 8
  %self.val15.i = load ptr, ptr %0, align 8, !alias.scope !3
; call <alloc::raw_vec::RawVecInner>::finish_grow
  call fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs2f1hMIlW3Va_4heck(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self3.i, i64 %self5.i, ptr %self.val15.i, i64 noundef %_0.sroa.0.0.i16.i)
  %_35.i = load i64, ptr %self3.i, align 8, !range !7, !noalias !3, !noundef !2
  %1 = trunc nuw i64 %_35.i to i1
  %2 = getelementptr inbounds nuw i8, ptr %self3.i, i64 8
  br i1 %1, label %bb18.i, label %bb3

bb18.i:                                           ; preds = %bb9.i
  %e.0.i = load i64, ptr %2, align 8, !range !8, !noalias !3, !noundef !2
  %3 = getelementptr inbounds nuw i8, ptr %self3.i, i64 16
  %e.1.i = load i64, ptr %3, align 8, !noalias !3
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !3
  br label %bb2

bb2:                                              ; preds = %bb18.i, %start
  %_0.sroa.5.0.i.ph = phi i64 [ undef, %start ], [ %e.1.i, %bb18.i ]
  %_0.sroa.0.0.i.ph = phi i64 [ 0, %start ], [ %e.0.i, %bb18.i ]
; call alloc::raw_vec::handle_error
  tail call void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef %_0.sroa.0.0.i.ph, i64 %_0.sroa.5.0.i.ph) #18
  unreachable

bb3:                                              ; preds = %bb9.i
  %v.0.i = load ptr, ptr %2, align 8, !noalias !3, !nonnull !2, !noundef !2
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %self3.i), !noalias !3
  store ptr %v.0.i, ptr %0, align 8, !alias.scope !3
  %4 = icmp sgt i64 %_0.sroa.0.0.i16.i, -1
  tail call void @llvm.assume(i1 %4)
  store i64 %_0.sroa.0.0.i16.i, ptr %slf, align 8, !alias.scope !3
  ret void
}

; heck::capitalize
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_33 = alloca [12 x i8], align 8
  %args = alloca [16 x i8], align 8
  %_11 = alloca [32 x i8], align 8
  %_28 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  %0 = ptrtoint ptr %s.0 to i64
  %_6.i.i.i = icmp samesign eq i64 %s.1, 0
  br i1 %_6.i.i.i, label %bb12, label %bb14.i.i

bb14.i.i:                                         ; preds = %start
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 1
  %x.i.i = load i8, ptr %s.0, align 1, !noalias !9, !noundef !2
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp samesign ne i64 %s.1, 1
  tail call void @llvm.assume(i1 %_6.i10.i.i)
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 2
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !noalias !9, !noundef !2
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
  %_6.i17.i.i = icmp samesign ne i64 %s.1, 2
  tail call void @llvm.assume(i1 %_6.i17.i.i)
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 3
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !noalias !9, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %2 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb2

bb8.i.i:                                          ; preds = %bb6.i.i
  %_6.i24.i.i = icmp samesign ne i64 %s.1, 3
  tail call void @llvm.assume(i1 %_6.i24.i.i)
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %s.0, i64 4
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !noalias !9, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %3 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb2

bb2:                                              ; preds = %bb8.i.i, %bb6.i.i, %bb3.i.i, %bb4.i.i
  %char_indices.sroa.0.0 = phi ptr [ %_16.i.i.i, %bb3.i.i ], [ %_16.i26.i.i, %bb8.i.i ], [ %_16.i19.i.i, %bb6.i.i ], [ %_16.i12.i.i, %bb4.i.i ]
  %_0.sroa.4.0.i.ph.i = phi i32 [ %_7.i.i, %bb3.i.i ], [ %3, %bb8.i.i ], [ %2, %bb6.i.i ], [ %1, %bb4.i.i ]
  %4 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  tail call void @llvm.assume(i1 %4)
  %5 = ptrtoint ptr %char_indices.sroa.0.0 to i64
  %6 = sub i64 %5, %0
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11)
; call core::unicode::unicode_data::conversions::to_upper
  call void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33, i32 noundef %_0.sroa.4.0.i.ph.i)
  %7 = getelementptr inbounds nuw i8, ptr %_33, i64 8
  %_3.i = load i32, ptr %7, align 8, !range !14, !noundef !2
  %8 = icmp eq i32 %_3.i, 0
  %9 = getelementptr inbounds nuw i8, ptr %_33, i64 4
  %_6.i = load i32, ptr %9, align 4, !range !14
  %10 = icmp eq i32 %_6.i, 0
  %spec.select.i = select i1 %10, i64 1, i64 2
  %iter.sroa.4.0.i = select i1 %8, i64 %spec.select.i, i64 3
  store i64 0, ptr %_11, align 8
  %_32.sroa.4.0._11.sroa_idx = getelementptr inbounds nuw i8, ptr %_11, i64 8
  store i64 %iter.sroa.4.0.i, ptr %_32.sroa.4.0._11.sroa_idx, align 8
  %_32.sroa.5.0._11.sroa_idx = getelementptr inbounds nuw i8, ptr %_11, i64 16
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx, ptr noundef nonnull align 8 dereferenceable(12) %_33, i64 12, i1 false)
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_11, ptr %args, align 8
  %_13.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx, align 8
  %_45.0 = load ptr, ptr %f, align 8, !nonnull !2, !align !15, !noundef !2
  %11 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_45.1 = load ptr, ptr %11, align 8, !nonnull !2, !align !16, !noundef !2
; call core::fmt::write
  %12 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_45.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_45.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11)
  br i1 %12, label %bb11, label %bb23

bb23:                                             ; preds = %bb2
  %_6.i.i.i14 = icmp eq ptr %char_indices.sroa.0.0, %_28
  br i1 %_6.i.i.i14, label %bb12, label %bb14.i.i15

bb14.i.i15:                                       ; preds = %bb23
  %x.i.i17 = load i8, ptr %char_indices.sroa.0.0, align 1, !noalias !17, !noundef !2
  %_6.i.i18 = icmp sgt i8 %x.i.i17, -1
  br i1 %_6.i.i18, label %bb3.i.i56, label %bb4.i.i19

bb4.i.i19:                                        ; preds = %bb14.i.i15
  %_16.i.i.i16 = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0, i64 1
  %_30.i.i20 = and i8 %x.i.i17, 31
  %init.i.i21 = zext nneg i8 %_30.i.i20 to i32
  %_6.i10.i.i22 = icmp ne ptr %_16.i.i.i16, %_28
  call void @llvm.assume(i1 %_6.i10.i.i22)
  %y.i.i24 = load i8, ptr %_16.i.i.i16, align 1, !noalias !17, !noundef !2
  %_33.i.i25 = shl nuw nsw i32 %init.i.i21, 6
  %_35.i.i26 = and i8 %y.i.i24, 63
  %_34.i.i27 = zext nneg i8 %_35.i.i26 to i32
  %13 = or disjoint i32 %_33.i.i25, %_34.i.i27
  %_13.i.i28 = icmp samesign ugt i8 %x.i.i17, -33
  br i1 %_13.i.i28, label %bb6.i.i36, label %bb6

bb3.i.i56:                                        ; preds = %bb14.i.i15
  %_7.i.i57 = zext nneg i8 %x.i.i17 to i32
  br label %bb6

bb6.i.i36:                                        ; preds = %bb4.i.i19
  %_16.i12.i.i23 = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0, i64 2
  %_6.i17.i.i37 = icmp ne ptr %_16.i12.i.i23, %_28
  call void @llvm.assume(i1 %_6.i17.i.i37)
  %z.i.i39 = load i8, ptr %_16.i12.i.i23, align 1, !noalias !17, !noundef !2
  %_38.i.i40 = shl nuw nsw i32 %_34.i.i27, 6
  %_40.i.i41 = and i8 %z.i.i39, 63
  %_39.i.i42 = zext nneg i8 %_40.i.i41 to i32
  %y_z.i.i43 = or disjoint i32 %_38.i.i40, %_39.i.i42
  %_20.i.i44 = shl nuw nsw i32 %init.i.i21, 12
  %14 = or disjoint i32 %y_z.i.i43, %_20.i.i44
  %_21.i.i45 = icmp samesign ugt i8 %x.i.i17, -17
  br i1 %_21.i.i45, label %bb8.i.i46, label %bb6

bb8.i.i46:                                        ; preds = %bb6.i.i36
  %_16.i19.i.i38 = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0, i64 3
  %_6.i24.i.i47 = icmp ne ptr %_16.i19.i.i38, %_28
  call void @llvm.assume(i1 %_6.i24.i.i47)
  %w.i.i49 = load i8, ptr %_16.i19.i.i38, align 1, !noalias !17, !noundef !2
  %_26.i.i50 = shl nuw nsw i32 %init.i.i21, 18
  %_25.i.i51 = and i32 %_26.i.i50, 1835008
  %_43.i.i52 = shl nuw nsw i32 %y_z.i.i43, 6
  %_45.i.i53 = and i8 %w.i.i49, 63
  %_44.i.i54 = zext nneg i8 %_45.i.i53 to i32
  %_27.i.i55 = or disjoint i32 %_43.i.i52, %_44.i.i54
  %15 = or disjoint i32 %_27.i.i55, %_25.i.i51
  br label %bb6

bb6:                                              ; preds = %bb8.i.i46, %bb6.i.i36, %bb3.i.i56, %bb4.i.i19
  %_0.sroa.4.0.i.ph.i31 = phi i32 [ %13, %bb4.i.i19 ], [ %14, %bb6.i.i36 ], [ %15, %bb8.i.i46 ], [ %_7.i.i57, %bb3.i.i56 ]
  %16 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31, 1114112
  call void @llvm.assume(i1 %16)
  %_8.not.i = icmp ult i64 %6, %s.1
  br i1 %_8.not.i, label %bb9.i, label %bb6.i

bb6.i:                                            ; preds = %bb6
  %17 = icmp eq i64 %6, %s.1
  br i1 %17, label %bb26, label %bb25

bb9.i:                                            ; preds = %bb6
  %18 = getelementptr inbounds nuw i8, ptr %s.0, i64 %6
  %self1.i = load i8, ptr %18, align 1, !alias.scope !22, !noundef !2
  %19 = icmp sgt i8 %self1.i, -65
  br i1 %19, label %bb26, label %bb25

bb26:                                             ; preds = %bb9.i, %bb6.i
  %new_len.i = sub nuw i64 %s.1, %6
  %data.i = getelementptr inbounds nuw i8, ptr %s.0, i64 %6
; call heck::lowercase
  %_19 = call noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i, i64 noundef %new_len.i, ptr noalias noundef nonnull align 8 dereferenceable(24) %f)
  br i1 %_19, label %bb11, label %bb12

bb25:                                             ; preds = %bb9.i, %bb6.i
; call core::str::slice_error_fail
  call void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %s.0, i64 noundef %s.1, i64 noundef %6, i64 noundef %s.1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
  unreachable

bb11:                                             ; preds = %bb2, %bb26
  br label %bb12

bb12:                                             ; preds = %bb23, %start, %bb26, %bb11
  %_0.sroa.0.1.off0 = phi i1 [ true, %bb11 ], [ false, %bb26 ], [ false, %start ], [ false, %bb23 ]
  ret i1 %_0.sroa.0.1.off0
}

; heck::lowercase
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address) %s.0, i64 noundef %s.1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_63 = alloca [12 x i8], align 4
  %args = alloca [16 x i8], align 8
  %_13 = alloca [32 x i8], align 8
  %_22 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  %0 = getelementptr inbounds nuw i8, ptr %_63, i64 8
  %1 = getelementptr inbounds nuw i8, ptr %_63, i64 4
  %_62.sroa.4.0._13.sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 8
  %_62.sroa.5.0._13.sroa_idx = getelementptr inbounds nuw i8, ptr %_13, i64 16
  %_15.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_75.0 = load ptr, ptr %f, align 8, !nonnull !2, !align !15
  %2 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_75.1 = load ptr, ptr %2, align 8, !nonnull !2, !align !16
  %3 = getelementptr inbounds nuw i8, ptr %_75.1, i64 24
  br label %bb1

bb1:                                              ; preds = %bb1.backedge, %start
  %chars.sroa.17.0 = phi i32 [ 1114113, %start ], [ %chars.sroa.17.0.be, %bb1.backedge ]
  %chars.sroa.0.0 = phi ptr [ %s.0, %start ], [ %chars.sroa.0.0.be, %bb1.backedge ]
  switch i32 %chars.sroa.17.0, label %bb2 [
    i32 1114113, label %bb14
    i32 1114112, label %bb11
  ]

bb14:                                             ; preds = %bb1
  %_6.i.i = icmp eq ptr %chars.sroa.0.0, %_22
  br i1 %_6.i.i, label %bb11, label %bb14.i

bb14.i:                                           ; preds = %bb14
  %_16.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0, i64 1
  %x.i = load i8, ptr %chars.sroa.0.0, align 1, !noalias !25, !noundef !2
  %_6.i = icmp sgt i8 %x.i, -1
  br i1 %_6.i, label %bb3.i, label %bb4.i

bb4.i:                                            ; preds = %bb14.i
  %_30.i = and i8 %x.i, 31
  %init.i = zext nneg i8 %_30.i to i32
  %_6.i10.i = icmp ne ptr %_16.i.i, %_22
  call void @llvm.assume(i1 %_6.i10.i)
  %_16.i12.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0, i64 2
  %y.i = load i8, ptr %_16.i.i, align 1, !noalias !25, !noundef !2
  %_33.i = shl nuw nsw i32 %init.i, 6
  %_35.i = and i8 %y.i, 63
  %_34.i = zext nneg i8 %_35.i to i32
  %4 = or disjoint i32 %_33.i, %_34.i
  %_13.i = icmp samesign ugt i8 %x.i, -33
  br i1 %_13.i, label %bb6.i, label %bb18

bb3.i:                                            ; preds = %bb14.i
  %_7.i = zext nneg i8 %x.i to i32
  br label %bb18

bb6.i:                                            ; preds = %bb4.i
  %_6.i17.i = icmp ne ptr %_16.i12.i, %_22
  call void @llvm.assume(i1 %_6.i17.i)
  %_16.i19.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0, i64 3
  %z.i = load i8, ptr %_16.i12.i, align 1, !noalias !25, !noundef !2
  %_38.i = shl nuw nsw i32 %_34.i, 6
  %_40.i = and i8 %z.i, 63
  %_39.i = zext nneg i8 %_40.i to i32
  %y_z.i = or disjoint i32 %_38.i, %_39.i
  %_20.i = shl nuw nsw i32 %init.i, 12
  %5 = or disjoint i32 %y_z.i, %_20.i
  %_21.i = icmp samesign ugt i8 %x.i, -17
  br i1 %_21.i, label %bb8.i, label %bb18

bb8.i:                                            ; preds = %bb6.i
  %_6.i24.i = icmp ne ptr %_16.i19.i, %_22
  call void @llvm.assume(i1 %_6.i24.i)
  %_16.i26.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0, i64 4
  %w.i = load i8, ptr %_16.i19.i, align 1, !noalias !25, !noundef !2
  %_26.i = shl nuw nsw i32 %init.i, 18
  %_25.i = and i32 %_26.i, 1835008
  %_43.i = shl nuw nsw i32 %y_z.i, 6
  %_45.i = and i8 %w.i, 63
  %_44.i = zext nneg i8 %_45.i to i32
  %_27.i = or disjoint i32 %_43.i, %_44.i
  %6 = or disjoint i32 %_27.i, %_25.i
  br label %bb18

bb18:                                             ; preds = %bb3.i, %bb8.i, %bb6.i, %bb4.i
  %chars.sroa.0.4.ph = phi ptr [ %_16.i12.i, %bb4.i ], [ %_16.i19.i, %bb6.i ], [ %_16.i26.i, %bb8.i ], [ %_16.i.i, %bb3.i ]
  %_0.sroa.4.0.i.ph = phi i32 [ %4, %bb4.i ], [ %5, %bb6.i ], [ %6, %bb8.i ], [ %_7.i, %bb3.i ]
  %7 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 1114112
  call void @llvm.assume(i1 %7)
  br label %bb2

bb2:                                              ; preds = %bb1, %bb18
  %chars.sroa.0.1 = phi ptr [ %chars.sroa.0.0, %bb1 ], [ %chars.sroa.0.4.ph, %bb18 ]
  %_5.sroa.0.0 = phi i32 [ %chars.sroa.17.0, %bb1 ], [ %_0.sroa.4.0.i.ph, %bb18 ]
  %8 = icmp eq i32 %_5.sroa.0.0, 931
  br i1 %8, label %bb1.i, label %bb5

bb1.i:                                            ; preds = %bb2
  %_6.i.i.not.i.i.i = icmp eq ptr %chars.sroa.0.1, %_22
  br i1 %_6.i.i.not.i.i.i, label %bb47, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb1.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1, i64 1
  %x.i.i.i.i = load i8, ptr %chars.sroa.0.1, align 1, !noalias !28, !noundef !2
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i, %_22
  call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1, i64 2
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !noalias !28, !noundef !2
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %9 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i.i, label %bb5

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb5

bb6.i.i.i.i:                                      ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i, %_22
  call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1, i64 3
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !noalias !28, !noundef !2
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %10 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit, label %bb5

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit: ; preds = %bb6.i.i.i.i
  %_6.i24.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i, %_22
  call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1, i64 4
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !noalias !28, !noundef !2
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %11 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  %.not12 = icmp eq i32 %11, 1114112
  br i1 %.not12, label %bb47, label %bb5

bb5:                                              ; preds = %bb4.i.i.i.i, %bb6.i.i.i.i, %bb3.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit, %bb2
  %chars.sroa.17.1 = phi i32 [ %11, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit ], [ 1114113, %bb2 ], [ %9, %bb4.i.i.i.i ], [ %10, %bb6.i.i.i.i ], [ %_7.i.i.i.i, %bb3.i.i.i.i ]
  %chars.sroa.0.2 = phi ptr [ %_16.i26.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit ], [ %chars.sroa.0.1, %bb2 ], [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i.i ], [ %_16.i.i.i.i.i, %bb3.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63)
; call core::unicode::unicode_data::conversions::to_lower
  call void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63, i32 noundef %_5.sroa.0.0)
  %_3.i = load i32, ptr %0, align 4, !range !14, !alias.scope !38, !noalias !41, !noundef !2
  %12 = icmp eq i32 %_3.i, 0
  %_6.i13 = load i32, ptr %1, align 4, !range !14, !alias.scope !38, !noalias !41
  %13 = icmp eq i32 %_6.i13, 0
  %spec.select.i = select i1 %13, i64 1, i64 2
  %iter.sroa.4.0.i = select i1 %12, i64 %spec.select.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx, ptr noundef nonnull align 4 dereferenceable(12) %_63, i64 12, i1 false)
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63)
  store i64 0, ptr %_13, align 8
  store i64 %iter.sroa.4.0.i, ptr %_62.sroa.4.0._13.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_13, ptr %args, align 8
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx, align 8
; call core::fmt::write
  %14 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_75.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_75.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13)
  br i1 %14, label %bb11, label %bb1.backedge

bb1.backedge:                                     ; preds = %bb5, %bb47
  %chars.sroa.17.0.be = phi i32 [ 1114112, %bb47 ], [ %chars.sroa.17.1, %bb5 ]
  %chars.sroa.0.0.be = phi ptr [ %chars.sroa.0.528, %bb47 ], [ %chars.sroa.0.2, %bb5 ]
  br label %bb1

bb47:                                             ; preds = %bb1.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit
  %chars.sroa.0.528 = phi ptr [ %_16.i26.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit ], [ %_22, %bb1.i ]
  %15 = load ptr, ptr %3, align 8, !invariant.load !2, !nonnull !2
  %16 = call noundef zeroext i1 %15(ptr noundef nonnull align 1 %_75.0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_ce246300cd91ffc3b77a40cb1bee254d, i64 noundef 2) #20
  br i1 %16, label %bb11, label %bb1.backedge

bb11:                                             ; preds = %bb14, %bb47, %bb1, %bb5
  %_0.sroa.0.1.off0 = phi i1 [ true, %bb5 ], [ false, %bb14 ], [ true, %bb47 ], [ false, %bb1 ]
  ret i1 %_0.sroa.0.1.off0
}

; heck::uppercase
; Function Attrs: uwtable
define noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9uppercase(ptr noalias noundef nonnull readonly align 1 captures(address) %s.0, i64 noundef %s.1, ptr noalias noundef readonly align 8 captures(none) dereferenceable(24) %f) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %_29 = alloca [12 x i8], align 4
  %args = alloca [16 x i8], align 8
  %_10 = alloca [32 x i8], align 8
  %_19 = getelementptr inbounds nuw i8, ptr %s.0, i64 %s.1
  %_6.i.i16.not = icmp samesign eq i64 %s.1, 0
  br i1 %_6.i.i16.not, label %bb5, label %bb14.i.lr.ph

bb14.i.lr.ph:                                     ; preds = %start
  %0 = getelementptr inbounds nuw i8, ptr %_29, i64 8
  %1 = getelementptr inbounds nuw i8, ptr %_29, i64 4
  %_28.sroa.4.0._10.sroa_idx = getelementptr inbounds nuw i8, ptr %_10, i64 8
  %_28.sroa.5.0._10.sroa_idx = getelementptr inbounds nuw i8, ptr %_10, i64 16
  %_12.sroa.4.0..sroa_idx = getelementptr inbounds nuw i8, ptr %args, i64 8
  %_41.0 = load ptr, ptr %f, align 8, !nonnull !2, !align !15
  %2 = getelementptr inbounds nuw i8, ptr %f, i64 8
  %_41.1 = load ptr, ptr %2, align 8, !nonnull !2, !align !16
  br label %bb14.i

bb14.i:                                           ; preds = %bb10, %bb14.i.lr.ph
  %iter.sroa.0.017 = phi ptr [ %s.0, %bb14.i.lr.ph ], [ %iter.sroa.0.1.ph, %bb10 ]
  %_16.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017, i64 1
  %x.i = load i8, ptr %iter.sroa.0.017, align 1, !noalias !43, !noundef !2
  %_6.i = icmp sgt i8 %x.i, -1
  br i1 %_6.i, label %bb3.i, label %bb4.i

bb4.i:                                            ; preds = %bb14.i
  %_30.i = and i8 %x.i, 31
  %init.i = zext nneg i8 %_30.i to i32
  %_6.i10.i = icmp ne ptr %_16.i.i, %_19
  call void @llvm.assume(i1 %_6.i10.i)
  %_16.i12.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017, i64 2
  %y.i = load i8, ptr %_16.i.i, align 1, !noalias !43, !noundef !2
  %_33.i = shl nuw nsw i32 %init.i, 6
  %_35.i = and i8 %y.i, 63
  %_34.i = zext nneg i8 %_35.i to i32
  %3 = or disjoint i32 %_33.i, %_34.i
  %_13.i = icmp samesign ugt i8 %x.i, -33
  br i1 %_13.i, label %bb6.i, label %bb10

bb3.i:                                            ; preds = %bb14.i
  %_7.i = zext nneg i8 %x.i to i32
  br label %bb10

bb6.i:                                            ; preds = %bb4.i
  %_6.i17.i = icmp ne ptr %_16.i12.i, %_19
  call void @llvm.assume(i1 %_6.i17.i)
  %_16.i19.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017, i64 3
  %z.i = load i8, ptr %_16.i12.i, align 1, !noalias !43, !noundef !2
  %_38.i = shl nuw nsw i32 %_34.i, 6
  %_40.i = and i8 %z.i, 63
  %_39.i = zext nneg i8 %_40.i to i32
  %y_z.i = or disjoint i32 %_38.i, %_39.i
  %_20.i = shl nuw nsw i32 %init.i, 12
  %4 = or disjoint i32 %y_z.i, %_20.i
  %_21.i = icmp samesign ugt i8 %x.i, -17
  br i1 %_21.i, label %bb8.i, label %bb10

bb8.i:                                            ; preds = %bb6.i
  %_6.i24.i = icmp ne ptr %_16.i19.i, %_19
  call void @llvm.assume(i1 %_6.i24.i)
  %_16.i26.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017, i64 4
  %w.i = load i8, ptr %_16.i19.i, align 1, !noalias !43, !noundef !2
  %_26.i = shl nuw nsw i32 %init.i, 18
  %_25.i = and i32 %_26.i, 1835008
  %_43.i = shl nuw nsw i32 %y_z.i, 6
  %_45.i = and i8 %w.i, 63
  %_44.i = zext nneg i8 %_45.i to i32
  %_27.i = or disjoint i32 %_43.i, %_44.i
  %5 = or disjoint i32 %_27.i, %_25.i
  br label %bb10

bb10:                                             ; preds = %bb3.i, %bb8.i, %bb6.i, %bb4.i
  %iter.sroa.0.1.ph = phi ptr [ %_16.i12.i, %bb4.i ], [ %_16.i19.i, %bb6.i ], [ %_16.i26.i, %bb8.i ], [ %_16.i.i, %bb3.i ]
  %_0.sroa.4.0.i.ph = phi i32 [ %3, %bb4.i ], [ %4, %bb6.i ], [ %5, %bb8.i ], [ %_7.i, %bb3.i ]
  %6 = icmp samesign ult i32 %_0.sroa.4.0.i.ph, 1114112
  call void @llvm.assume(i1 %6)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_29)
; call core::unicode::unicode_data::conversions::to_upper
  call void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_29, i32 noundef %_0.sroa.4.0.i.ph)
  %_3.i = load i32, ptr %0, align 4, !range !14, !alias.scope !46, !noalias !49, !noundef !2
  %7 = icmp eq i32 %_3.i, 0
  %_6.i8 = load i32, ptr %1, align 4, !range !14, !alias.scope !46, !noalias !49
  %8 = icmp eq i32 %_6.i8, 0
  %spec.select.i = select i1 %8, i64 1, i64 2
  %iter.sroa.4.0.i = select i1 %7, i64 %spec.select.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_28.sroa.5.0._10.sroa_idx, ptr noundef nonnull align 4 dereferenceable(12) %_29, i64 12, i1 false)
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_29)
  store i64 0, ptr %_10, align 8
  store i64 %iter.sroa.4.0.i, ptr %_28.sroa.4.0._10.sroa_idx, align 8
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args)
  store ptr %_10, ptr %args, align 8
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_12.sroa.4.0..sroa_idx, align 8
; call core::fmt::write
  %9 = call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %_41.0, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) %_41.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args)
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args)
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10)
  %_6.i.i.not = icmp eq ptr %iter.sroa.0.1.ph, %_19
  %or.cond = select i1 %9, i1 true, i1 %_6.i.i.not
  br i1 %or.cond, label %bb5, label %bb14.i

bb5:                                              ; preds = %bb10, %start
  %_6.i.i15 = phi i1 [ false, %start ], [ %9, %bb10 ]
  ret i1 %_6.i.i15
}

; <alloc::raw_vec::RawVecInner>::finish_grow
; Function Attrs: cold nounwind uwtable
define internal fastcc void @_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner11finish_growCs2f1hMIlW3Va_4heck(ptr dead_on_unwind noalias noundef nonnull writable writeonly align 8 captures(none) dereferenceable(24) initializes((0, 8)) %_0, i64 %self.0.val, ptr %self.8.val, i64 noundef %cap) unnamed_addr #3 {
start:
  %_23.i = icmp eq i64 %cap, 0
  br i1 %_23.i, label %bb14.thread, label %bb6.i

bb6.i:                                            ; preds = %start
  %or.cond.not = icmp sgt i64 %cap, 0
  br i1 %or.cond.not, label %bb14, label %bb11, !prof !51

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
  %raw_ptr.i.i = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr noundef nonnull %self.8.val, i64 noundef %self.0.val, i64 noundef range(i64 1, -9223372036854775807) 1, i64 noundef %cap) #17
  br label %bb7

bb4.i.i11:                                        ; preds = %bb14
; call __rustc::__rust_no_alloc_shim_is_unstable_v2
  tail call void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() #17
; call __rustc::__rust_alloc
  %3 = tail call noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef %cap, i64 noundef range(i64 1, -9223372036854775807) 1) #17
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

; <str as heck::lower_camel::ToLowerCamelCase>::to_lower_camel_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck11lower_cameleNtB2_16ToLowerCamelCase19to_lower_camel_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_63.i = alloca [12 x i8], align 4
  %args.i8 = alloca [16 x i8], align 8
  %_13.i = alloca [32 x i8], align 8
  %_33.i = alloca [12 x i8], align 8
  %args.i = alloca [16 x i8], align 8
  %_11.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !52
  store i64 0, ptr %buf.i, align 8, !noalias !52
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !52
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !52
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !52
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !52
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !52
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !52
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !52
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !52
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_33.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_33.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 16
  %_13.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  %4 = getelementptr inbounds nuw i8, ptr %_63.i, i64 8
  %5 = getelementptr inbounds nuw i8, ptr %_63.i, i64 4
  %_62.sroa.4.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 8
  %_62.sroa.5.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 16
  %_15.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i8, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %iter.sroa.0.0165.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0164.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1187.0163.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1187.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0161.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %.promoted155160.off0.i.i.i = phi i1 [ true, %start ], [ %.promoted153.off0.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1187.0163.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %6 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0161.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1187.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1187.0163.i.i.i, %bb2.i.i.i.i ]
  %7 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !58, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !58, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %8 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %9 = add i64 %6, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !58, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %10 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !58, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %11 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1187.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %11, %bb8.i.i.i.i.i.i.i.i.i ], [ %10, %bb6.i.i.i.i.i.i.i.i.i ], [ %8, %bb4.i.i.i.i.i.i.i.i.i ]
  %12 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %12)
  %13 = ptrtoint ptr %iter.sroa.1187.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %13, %7
  %14 = add i64 %_10.i.i.i.i.i.i.i.i, %6
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %9, %bb3.thread.i.i.i.i.i.i.i ], [ %14, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1187.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %15 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %15, 10
  %16 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %17 = add nsw i32 %16, -65
  %18 = icmp ult i32 %17, 26
  %19 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %18
  br i1 %19, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !52

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtBJ_11lower_camelINtB1j_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1e_s_0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtBJ_11lower_camelINtB1j_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1e_s_0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %20 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !52

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtBJ_11lower_camelINtB1j_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1e_s_0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %20, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %14, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %14, %.noexc.i ]
  %iter.sroa.1187.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1187.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1187.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1187.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0161.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %14, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %14, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %14, %.noexc.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ]
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ]
  %iter.sroa.1187.6.ph.i.i.i = phi ptr [ %iter.sroa.1187.0163.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1187.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1187.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1187.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1187.1.i.i.i, %.noexc.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0165.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.0.0165.i.i.i, %bb5.i.i.i.i.i.i ], [ %14, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %14, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %14, %.noexc.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ], [ %6, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %6, %.noexc.i ], [ %6, %bb1.i.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0165.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0165.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb3.i.i.i
  %.promoted154.off0.i.i.i = phi i1 [ %.promoted155160.off0.i.i.i, %bb3.i.i.i ], [ %.promoted154.off0.i.i.i.be, %bb5.i.i.i.backedge ]
  %.off0.i.i.i = phi i1 [ %.promoted155160.off0.i.i.i, %bb3.i.i.i ], [ %.off0.i.i.i.be, %bb5.i.i.i.backedge ]
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %32, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.0164.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %init.sroa.0.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb3.i.i.i ], [ %mode.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %21 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !55, !noalias !78, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !78, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %22 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread225.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i39.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread225.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !55, !noalias !78, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %23 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread225.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !55, !noalias !78, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %24 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread225.i.i.i

bb50.thread225.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i39.i.i.i, %bb3.i.i.i.i.i ], [ %24, %bb8.i.i.i.i.i ], [ %23, %bb6.i.i.i.i.i ], [ %22, %bb4.i.i.i.i.i ]
  %25 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %21
  %26 = add i64 %_10.i.i.i.i, %25
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread225.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %25, %bb50.thread225.i.i.i ], [ %21, %bb5.i.i.i ]
  %_15.sroa.8.0234.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread225.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0233.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread225.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1232.i.i.i = phi i64 [ %26, %bb50.thread225.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1231.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread225.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1231.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb33.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1231.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1231.i.i.i, align 1, !alias.scope !55, !noalias !83, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i40.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1231.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !83, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %27 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i40.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1231.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !83, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %28 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1231.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !55, !noalias !83, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %29 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %.noexc4.i, %.noexc3.i
  %.promoted153.off0.i.i.i = phi i1 [ false, %.noexc4.i ], [ false, %.noexc3.i ], [ %.promoted154.off0.i.i.i, %bb47.i.i.i ]
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %.noexc4.i ], [ undef, %.noexc3.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11lower_camel16AsLowerCamelCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i40.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i40.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i40.i.i.i ], [ %29, %bb8.i.i.i.i.i.i.i ], [ %28, %bb6.i.i.i.i.i.i.i ], [ %27, %bb4.i.i.i.i.i.i.i ]
  %30 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %30)
  %31 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1232.i.i.i, %.pre-phi.i.i
  %32 = add i64 %_10.i.i.i.i.i.i, %31
  %33 = add nsw i32 %_15.sroa.8.0234.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %33, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb33.i.i.i:                                       ; preds = %bb1.i.i.i.i
  %34 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %34, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %35 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %35, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %36 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self1.i.i.i.i = load i8, ptr %36, align 1, !alias.scope !93, !noalias !96, !noundef !2
  %37 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %37, label %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb101.i.i.invoke.i

_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i: ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %new_len.i43.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i
  %data.i44.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  br i1 %.promoted154.off0.i.i.i, label %bb1.i47.i.i.i, label %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit.i.i.i

bb1.i47.i.i.i:                                    ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i
; invoke heck::lowercase
  %38 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i44.i.i.i, i64 noundef %new_len.i43.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %.noexc3.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !52

.noexc3.i:                                        ; preds = %bb1.i47.i.i.i
  br i1 %38, label %bb2.i.i, label %bb35.i.i.i, !prof !97

_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit.i.i.i: ; preds = %_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i
; invoke heck::capitalize
  %39 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i44.i.i.i, i64 noundef %new_len.i43.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %.noexc4.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !52

.noexc4.i:                                        ; preds = %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit.i.i.i
  br i1 %39, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i76.i.i.i, %bb16.i72.i.i.i, %bb13.i78.i.i.i, %bb10.i65.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %40 = phi i64 [ %_15.sroa.0.0233.i.i.i, %bb19.i.i.i.i ], [ %_15.sroa.0.0233.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0233.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0233.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0233.i.i.i, %bb24.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb19.i76.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb16.i72.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb13.i78.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb10.i65.i.i.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb15.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %41 = phi ptr [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i76.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i72.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i78.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i65.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i, i64 noundef %40, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %41) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !52

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0234.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0234.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %42 = add nsw i32 %_15.sroa.8.0234.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0234.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %43 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %43, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %44 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %44, 26
  br i1 %or.cond2.i.i.i, label %bb15.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb15.i.i.i, label %bb17.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %45 = add nsw i32 %_15.sroa.8.0234.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %45, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0234.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0234.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %46 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %46, 26
  br i1 %or.cond4.i.i.i, label %bb24.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb24.i.i.i, label %bb5.i.i.i.backedge

bb24.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %_15.sroa.0.0233.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %47 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %47, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %48 = icmp eq i64 %_15.sroa.0.0233.i.i.i, 0
  br i1 %48, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %49 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %49, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %50 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i.i.i.i = load i8, ptr %50, align 1, !alias.scope !98, !noalias !96, !noundef !2
  %51 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %51, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0233.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %52 = icmp eq i64 %_15.sroa.0.0233.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %52, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %53 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0233.i.i.i
  %self2.i.i.i.i = load i8, ptr %53, align 1, !alias.scope !98, !noalias !96, !noundef !2
  %54 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %54, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i, label %bb101.i.i.invoke.i

_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i: ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0233.i.i.i, %init.sroa.0.0.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  br i1 %.off0.i.i.i, label %bb1.i59.i.i.i, label %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit60.i.i.i

bb1.i59.i.i.i:                                    ; preds = %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i
; invoke heck::lowercase
  %55 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %.noexc11.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

.noexc11.i:                                       ; preds = %bb1.i59.i.i.i
  br i1 %55, label %bb2.i.i, label %bb5.i.i.i.backedge, !prof !97

_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit60.i.i.i: ; preds = %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit.i.i.i
; invoke heck::capitalize
  %56 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %.noexc12.i unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit, !noalias !52

.noexc12.i:                                       ; preds = %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit60.i.i.i
  br i1 %56, label %bb2.i.i, label %bb5.i.i.i.backedge, !prof !97

bb15.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  %_3.not.i61.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %char_indices.sroa.17.1232.i.i.i
  br i1 %_3.not.i61.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i62.i.i.i

bb1.i62.i.i.i:                                    ; preds = %bb15.i.i.i
  %57 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %57, label %bb2.i69.i.i.i, label %bb9.i63.i.i.i

bb9.i63.i.i.i:                                    ; preds = %bb1.i62.i.i.i
  %_11.not.i64.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i64.i.i.i, label %bb13.i78.i.i.i, label %bb10.i65.i.i.i

bb2.i69.i.i.i:                                    ; preds = %bb13.i78.i.i.i, %bb10.i65.i.i.i, %bb1.i62.i.i.i
  %58 = icmp eq i64 %char_indices.sroa.17.1232.i.i.i, 0
  br i1 %58, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i, label %bb15.i70.i.i.i

bb10.i65.i.i.i:                                   ; preds = %bb9.i63.i.i.i
  %59 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %59, label %bb2.i69.i.i.i, label %bb101.i.i.invoke.i

bb13.i78.i.i.i:                                   ; preds = %bb9.i63.i.i.i
  %60 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i79.i.i.i = load i8, ptr %60, align 1, !alias.scope !101, !noalias !96, !noundef !2
  %61 = icmp sgt i8 %self.i79.i.i.i, -65
  br i1 %61, label %bb2.i69.i.i.i, label %bb101.i.i.invoke.i

bb15.i70.i.i.i:                                   ; preds = %bb2.i69.i.i.i
  %_17.not.i71.i.i.i = icmp ult i64 %char_indices.sroa.17.1232.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i71.i.i.i, label %bb19.i76.i.i.i, label %bb16.i72.i.i.i

bb16.i72.i.i.i:                                   ; preds = %bb15.i70.i.i.i
  %62 = icmp eq i64 %char_indices.sroa.17.1232.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %62, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i, label %bb101.i.i.invoke.i

bb19.i76.i.i.i:                                   ; preds = %bb15.i70.i.i.i
  %63 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1232.i.i.i
  %self2.i77.i.i.i = load i8, ptr %63, align 1, !alias.scope !101, !noalias !96, !noundef !2
  %64 = icmp sgt i8 %self2.i77.i.i.i, -65
  br i1 %64, label %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i, label %bb101.i.i.invoke.i

_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i: ; preds = %bb19.i76.i.i.i, %bb16.i72.i.i.i, %bb2.i69.i.i.i
  %new_len.i74.i.i.i = sub nuw i64 %char_indices.sroa.17.1232.i.i.i, %init.sroa.0.0.i.i.i
  %data.i75.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  br i1 %.off0.i.i.i, label %bb1.i84.i.i.i, label %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit85.i.i.i

bb1.i84.i.i.i:                                    ; preds = %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !104)
  %_22.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1232.i.i.i
  br label %bb1.i

bb1.i:                                            ; preds = %bb1.i.backedge, %bb1.i84.i.i.i
  %chars.sroa.17.0.i = phi i32 [ 1114113, %bb1.i84.i.i.i ], [ %chars.sroa.17.0.i.be, %bb1.i.backedge ]
  %chars.sroa.0.0.i = phi ptr [ %data.i75.i.i.i, %bb1.i84.i.i.i ], [ %chars.sroa.0.0.i.be, %bb1.i.backedge ]
  switch i32 %chars.sroa.17.0.i, label %bb2.i13 [
    i32 1114113, label %bb14.i
    i32 1114112, label %bb5.i.i.i.backedge
  ]

bb14.i:                                           ; preds = %bb1.i
  %_6.i.i.i11 = icmp eq ptr %chars.sroa.0.0.i, %_22.i
  br i1 %_6.i.i.i11, label %bb5.i.i.i.backedge, label %bb14.i.i

bb14.i.i:                                         ; preds = %bb14.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 1
  %x.i.i = load i8, ptr %chars.sroa.0.0.i, align 1, !alias.scope !104, !noalias !107, !noundef !2
  %_6.i.i12 = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i12, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %_22.i
  call void @llvm.assume(i1 %_6.i10.i.i), !noalias !52
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 2
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !alias.scope !104, !noalias !107, !noundef !2
  %_33.i.i = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %65 = or disjoint i32 %_33.i.i, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i52, label %bb18.i

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb18.i

bb6.i.i52:                                        ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %_22.i
  call void @llvm.assume(i1 %_6.i17.i.i), !noalias !52
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 3
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !alias.scope !104, !noalias !107, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %66 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb18.i

bb8.i.i:                                          ; preds = %bb6.i.i52
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %_22.i
  call void @llvm.assume(i1 %_6.i24.i.i), !noalias !52
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 4
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !alias.scope !104, !noalias !107, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %67 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb18.i

bb18.i:                                           ; preds = %bb8.i.i, %bb6.i.i52, %bb3.i.i, %bb4.i.i
  %chars.sroa.0.4.ph.i = phi ptr [ %_16.i12.i.i, %bb4.i.i ], [ %_16.i19.i.i, %bb6.i.i52 ], [ %_16.i26.i.i, %bb8.i.i ], [ %_16.i.i.i, %bb3.i.i ]
  %_0.sroa.4.0.i.ph.i = phi i32 [ %65, %bb4.i.i ], [ %66, %bb6.i.i52 ], [ %67, %bb8.i.i ], [ %_7.i.i, %bb3.i.i ]
  %68 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  call void @llvm.assume(i1 %68), !noalias !52
  br label %bb2.i13

bb2.i13:                                          ; preds = %bb18.i, %bb1.i
  %chars.sroa.0.1.i = phi ptr [ %chars.sroa.0.0.i, %bb1.i ], [ %chars.sroa.0.4.ph.i, %bb18.i ]
  %_5.sroa.0.0.i = phi i32 [ %chars.sroa.17.0.i, %bb1.i ], [ %_0.sroa.4.0.i.ph.i, %bb18.i ]
  %69 = icmp eq i32 %_5.sroa.0.0.i, 931
  br i1 %69, label %bb1.i.i, label %bb5.i

bb1.i.i:                                          ; preds = %bb2.i13
  %_6.i.i.not.i.i.i.i = icmp eq ptr %chars.sroa.0.1.i, %_22.i
  br i1 %_6.i.i.not.i.i.i.i, label %bb47.i, label %bb14.i.i.i.i.i17

bb14.i.i.i.i.i17:                                 ; preds = %bb1.i.i
  %_16.i.i.i.i.i.i18 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 1
  %x.i.i.i.i.i19 = load i8, ptr %chars.sroa.0.1.i, align 1, !alias.scope !104, !noalias !111, !noundef !2
  %_6.i.i.i.i.i20 = icmp sgt i8 %x.i.i.i.i.i19, -1
  br i1 %_6.i.i.i.i.i20, label %bb3.i.i.i.i.i50, label %bb4.i.i.i.i.i21

bb4.i.i.i.i.i21:                                  ; preds = %bb14.i.i.i.i.i17
  %_30.i.i.i.i.i22 = and i8 %x.i.i.i.i.i19, 31
  %init.i.i.i.i.i23 = zext nneg i8 %_30.i.i.i.i.i22 to i32
  %_6.i10.i.i.i.i.i24 = icmp ne ptr %_16.i.i.i.i.i.i18, %_22.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i24), !noalias !52
  %_16.i12.i.i.i.i.i25 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 2
  %y.i.i.i.i.i26 = load i8, ptr %_16.i.i.i.i.i.i18, align 1, !alias.scope !104, !noalias !111, !noundef !2
  %_33.i.i.i.i.i27 = shl nuw nsw i32 %init.i.i.i.i.i23, 6
  %_35.i.i.i.i.i28 = and i8 %y.i.i.i.i.i26, 63
  %_34.i.i.i.i.i29 = zext nneg i8 %_35.i.i.i.i.i28 to i32
  %70 = or disjoint i32 %_33.i.i.i.i.i27, %_34.i.i.i.i.i29
  %_13.i.i.i.i.i30 = icmp samesign ugt i8 %x.i.i.i.i.i19, -33
  br i1 %_13.i.i.i.i.i30, label %bb6.i.i.i.i.i31, label %bb5.i

bb3.i.i.i.i.i50:                                  ; preds = %bb14.i.i.i.i.i17
  %_7.i.i.i.i.i51 = zext nneg i8 %x.i.i.i.i.i19 to i32
  br label %bb5.i

bb6.i.i.i.i.i31:                                  ; preds = %bb4.i.i.i.i.i21
  %_6.i17.i.i.i.i.i32 = icmp ne ptr %_16.i12.i.i.i.i.i25, %_22.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i32), !noalias !52
  %_16.i19.i.i.i.i.i33 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 3
  %z.i.i.i.i.i34 = load i8, ptr %_16.i12.i.i.i.i.i25, align 1, !alias.scope !104, !noalias !111, !noundef !2
  %_38.i.i.i.i.i35 = shl nuw nsw i32 %_34.i.i.i.i.i29, 6
  %_40.i.i.i.i.i36 = and i8 %z.i.i.i.i.i34, 63
  %_39.i.i.i.i.i37 = zext nneg i8 %_40.i.i.i.i.i36 to i32
  %y_z.i.i.i.i.i38 = or disjoint i32 %_38.i.i.i.i.i35, %_39.i.i.i.i.i37
  %_20.i.i.i.i.i39 = shl nuw nsw i32 %init.i.i.i.i.i23, 12
  %71 = or disjoint i32 %y_z.i.i.i.i.i38, %_20.i.i.i.i.i39
  %_21.i.i.i.i.i40 = icmp samesign ugt i8 %x.i.i.i.i.i19, -17
  br i1 %_21.i.i.i.i.i40, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, label %bb5.i

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i: ; preds = %bb6.i.i.i.i.i31
  %_6.i24.i.i.i.i.i41 = icmp ne ptr %_16.i19.i.i.i.i.i33, %_22.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i41), !noalias !52
  %_16.i26.i.i.i.i.i42 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 4
  %w.i.i.i.i.i43 = load i8, ptr %_16.i19.i.i.i.i.i33, align 1, !alias.scope !104, !noalias !111, !noundef !2
  %_26.i.i.i.i.i44 = shl nuw nsw i32 %init.i.i.i.i.i23, 18
  %_25.i.i.i.i.i45 = and i32 %_26.i.i.i.i.i44, 1835008
  %_43.i.i.i.i.i46 = shl nuw nsw i32 %y_z.i.i.i.i.i38, 6
  %_45.i.i.i.i.i47 = and i8 %w.i.i.i.i.i43, 63
  %_44.i.i.i.i.i48 = zext nneg i8 %_45.i.i.i.i.i47 to i32
  %_27.i.i.i.i.i49 = or disjoint i32 %_43.i.i.i.i.i46, %_44.i.i.i.i.i48
  %72 = or disjoint i32 %_27.i.i.i.i.i49, %_25.i.i.i.i.i45
  %.not12.i = icmp eq i32 %72, 1114112
  br i1 %.not12.i, label %bb47.i, label %bb5.i

bb5.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb6.i.i.i.i.i31, %bb3.i.i.i.i.i50, %bb4.i.i.i.i.i21, %bb2.i13
  %chars.sroa.17.1.i = phi i32 [ %72, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ 1114113, %bb2.i13 ], [ %70, %bb4.i.i.i.i.i21 ], [ %71, %bb6.i.i.i.i.i31 ], [ %_7.i.i.i.i.i51, %bb3.i.i.i.i.i50 ]
  %chars.sroa.0.2.i = phi ptr [ %_16.i26.i.i.i.i.i42, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %chars.sroa.0.1.i, %bb2.i13 ], [ %_16.i12.i.i.i.i.i25, %bb4.i.i.i.i.i21 ], [ %_16.i19.i.i.i.i.i33, %bb6.i.i.i.i.i31 ], [ %_16.i.i.i.i.i.i18, %bb3.i.i.i.i.i50 ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i), !noalias !121
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63.i), !noalias !121
; invoke core::unicode::unicode_data::conversions::to_lower
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63.i, i32 noundef %_5.sroa.0.0.i)
          to label %.noexc53 unwind label %cleanup.loopexit.i.loopexit

.noexc53:                                         ; preds = %bb5.i
  %_3.i.i14 = load i32, ptr %4, align 4, !range !14, !alias.scope !122, !noalias !125, !noundef !2
  %73 = icmp eq i32 %_3.i.i14, 0
  %_6.i13.i = load i32, ptr %5, align 4, !range !14, !alias.scope !122, !noalias !125
  %74 = icmp eq i32 %_6.i13.i, 0
  %spec.select.i.i15 = select i1 %74, i64 1, i64 2
  %iter.sroa.4.0.i.i16 = select i1 %73, i64 %spec.select.i.i15, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(12) %_63.i, i64 12, i1 false), !noalias !121
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63.i), !noalias !121
  store i64 0, ptr %_13.i, align 8, !noalias !121
  store i64 %iter.sroa.4.0.i.i16, ptr %_62.sroa.4.0._13.sroa_idx.i, align 8, !noalias !121
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i8), !noalias !121
  store ptr %_13.i, ptr %args.i8, align 8, !noalias !121
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx.i, align 8, !noalias !121
; invoke core::fmt::write
  %75 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i8)
          to label %.noexc54 unwind label %cleanup.loopexit.i.loopexit

.noexc54:                                         ; preds = %.noexc53
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i8), !noalias !121
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i), !noalias !121
  br i1 %75, label %bb2.i.i, label %bb1.i.backedge, !prof !97

bb47.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb1.i.i
  %chars.sroa.0.528.i = phi ptr [ %_16.i26.i.i.i.i.i42, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %_22.i, %bb1.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !127)
  call void @llvm.experimental.noalias.scope.decl(metadata !130)
  call void @llvm.experimental.noalias.scope.decl(metadata !133)
  %len.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !136, !noalias !139, !noundef !2
  %self2.i.i.i.i241 = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !136, !noalias !139, !noundef !2
  %_9.i.i.i.i = sub i64 %self2.i.i.i.i241, %len.i.i.i.i
  %_7.i.i.i.i = icmp ult i64 %_9.i.i.i.i, 2
  br i1 %_7.i.i.i.i, label %bb1.i.i.i.i242, label %.noexc55, !prof !142

bb1.i.i.i.i242:                                   ; preds = %bb47.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i, i64 noundef 2)
          to label %.noexc243 unwind label %cleanup.loopexit.i.loopexit

.noexc243:                                        ; preds = %bb1.i.i.i.i242
  %len.pre.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !143, !noalias !139
  br label %.noexc55

.noexc55:                                         ; preds = %.noexc243, %bb47.i
  %len.i.i.i = phi i64 [ %len.i.i.i.i, %bb47.i ], [ %len.pre.i.i.i, %.noexc243 ]
  %_9.i.i.i = icmp sgt i64 %len.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i)
  %_10.i.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !143, !noalias !139, !nonnull !2, !noundef !2
  %dst.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i, i64 %len.i.i.i
  store i16 -32049, ptr %dst.i.i.i, align 1, !noalias !143
  %76 = add nuw i64 %len.i.i.i, 2
  store i64 %76, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !143, !noalias !139
  br label %bb1.i.backedge

bb1.i.backedge:                                   ; preds = %.noexc55, %.noexc54
  %chars.sroa.17.0.i.be = phi i32 [ 1114112, %.noexc55 ], [ %chars.sroa.17.1.i, %.noexc54 ]
  %chars.sroa.0.0.i.be = phi ptr [ %chars.sroa.0.528.i, %.noexc55 ], [ %chars.sroa.0.2.i, %.noexc54 ]
  br label %bb1.i

_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit85.i.i.i: ; preds = %_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get.exit80.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !144)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i)
  %_28.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1232.i.i.i
  %77 = ptrtoint ptr %data.i75.i.i.i to i64
  %_6.i.i.i.i = icmp eq i64 %char_indices.sroa.17.1232.i.i.i, %init.sroa.0.0.i.i.i
  br i1 %_6.i.i.i.i, label %.noexc15.i.thread, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit85.i.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 1
  %x.i.i.i = load i8, ptr %data.i75.i.i.i, align 1, !alias.scope !144, !noalias !147, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i4, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp samesign ne i64 %new_len.i74.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i), !noalias !52
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !144, !noalias !147, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %78 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb2.i

bb3.i.i.i4:                                       ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb2.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp samesign ne i64 %new_len.i74.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i), !noalias !52
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !144, !noalias !147, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %79 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i2, label %bb2.i

bb8.i.i.i2:                                       ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp samesign ne i64 %new_len.i74.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i), !noalias !52
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !144, !noalias !147, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i3 = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %80 = or disjoint i32 %_27.i.i.i, %_25.i.i.i3
  br label %bb2.i

bb2.i:                                            ; preds = %bb8.i.i.i2, %bb6.i.i.i, %bb3.i.i.i4, %bb4.i.i.i
  %char_indices.sroa.0.0.i = phi ptr [ %_16.i.i.i.i, %bb3.i.i.i4 ], [ %_16.i26.i.i.i, %bb8.i.i.i2 ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i12.i.i.i, %bb4.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %_7.i.i.i, %bb3.i.i.i4 ], [ %80, %bb8.i.i.i2 ], [ %79, %bb6.i.i.i ], [ %78, %bb4.i.i.i ]
  %81 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %81), !noalias !52
  %82 = ptrtoint ptr %char_indices.sroa.0.0.i to i64
  %83 = sub i64 %82, %77
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i), !noalias !153
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit

.noexc:                                           ; preds = %bb2.i
  %_3.i.i = load i32, ptr %2, align 8, !range !14, !noalias !153, !noundef !2
  %84 = icmp eq i32 %_3.i.i, 0
  %_6.i.i = load i32, ptr %3, align 4, !range !14, !noalias !153
  %85 = icmp eq i32 %_6.i.i, 0
  %spec.select.i.i = select i1 %85, i64 1, i64 2
  %iter.sroa.4.0.i.i = select i1 %84, i64 %spec.select.i.i, i64 3
  store i64 0, ptr %_11.i, align 8, !noalias !153
  store i64 %iter.sroa.4.0.i.i, ptr %_32.sroa.4.0._11.sroa_idx.i, align 8, !noalias !153
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i, i64 12, i1 false), !noalias !153
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !153
  store ptr %_11.i, ptr %args.i, align 8, !noalias !153
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i, align 8, !noalias !153
; invoke core::fmt::write
  %86 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i)
          to label %.noexc5 unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit

.noexc5:                                          ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !153
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i), !noalias !153
  br i1 %86, label %.noexc15.i, label %bb23.i

bb23.i:                                           ; preds = %.noexc5
  %_6.i.i.i14.i = icmp eq ptr %char_indices.sroa.0.0.i, %_28.i
  br i1 %_6.i.i.i14.i, label %.noexc15.i.thread, label %bb14.i.i15.i

bb14.i.i15.i:                                     ; preds = %bb23.i
  %x.i.i17.i = load i8, ptr %char_indices.sroa.0.0.i, align 1, !alias.scope !144, !noalias !154, !noundef !2
  %_6.i.i18.i = icmp sgt i8 %x.i.i17.i, -1
  br i1 %_6.i.i18.i, label %bb3.i.i56.i, label %bb4.i.i19.i

bb4.i.i19.i:                                      ; preds = %bb14.i.i15.i
  %_16.i.i.i16.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 1
  %_30.i.i20.i = and i8 %x.i.i17.i, 31
  %init.i.i21.i = zext nneg i8 %_30.i.i20.i to i32
  %_6.i10.i.i22.i = icmp ne ptr %_16.i.i.i16.i, %_28.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i), !noalias !52
  %y.i.i24.i = load i8, ptr %_16.i.i.i16.i, align 1, !alias.scope !144, !noalias !154, !noundef !2
  %_33.i.i25.i = shl nuw nsw i32 %init.i.i21.i, 6
  %_35.i.i26.i = and i8 %y.i.i24.i, 63
  %_34.i.i27.i = zext nneg i8 %_35.i.i26.i to i32
  %87 = or disjoint i32 %_33.i.i25.i, %_34.i.i27.i
  %_13.i.i28.i = icmp samesign ugt i8 %x.i.i17.i, -33
  br i1 %_13.i.i28.i, label %bb6.i.i36.i, label %bb6.i

bb3.i.i56.i:                                      ; preds = %bb14.i.i15.i
  %_7.i.i57.i = zext nneg i8 %x.i.i17.i to i32
  br label %bb6.i

bb6.i.i36.i:                                      ; preds = %bb4.i.i19.i
  %_16.i12.i.i23.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 2
  %_6.i17.i.i37.i = icmp ne ptr %_16.i12.i.i23.i, %_28.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i), !noalias !52
  %z.i.i39.i = load i8, ptr %_16.i12.i.i23.i, align 1, !alias.scope !144, !noalias !154, !noundef !2
  %_38.i.i40.i = shl nuw nsw i32 %_34.i.i27.i, 6
  %_40.i.i41.i = and i8 %z.i.i39.i, 63
  %_39.i.i42.i = zext nneg i8 %_40.i.i41.i to i32
  %y_z.i.i43.i = or disjoint i32 %_38.i.i40.i, %_39.i.i42.i
  %_20.i.i44.i = shl nuw nsw i32 %init.i.i21.i, 12
  %88 = or disjoint i32 %y_z.i.i43.i, %_20.i.i44.i
  %_21.i.i45.i = icmp samesign ugt i8 %x.i.i17.i, -17
  br i1 %_21.i.i45.i, label %bb8.i.i46.i, label %bb6.i

bb8.i.i46.i:                                      ; preds = %bb6.i.i36.i
  %_16.i19.i.i38.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 3
  %_6.i24.i.i47.i = icmp ne ptr %_16.i19.i.i38.i, %_28.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i), !noalias !52
  %w.i.i49.i = load i8, ptr %_16.i19.i.i38.i, align 1, !alias.scope !144, !noalias !154, !noundef !2
  %_26.i.i50.i = shl nuw nsw i32 %init.i.i21.i, 18
  %_25.i.i51.i = and i32 %_26.i.i50.i, 1835008
  %_43.i.i52.i = shl nuw nsw i32 %y_z.i.i43.i, 6
  %_45.i.i53.i = and i8 %w.i.i49.i, 63
  %_44.i.i54.i = zext nneg i8 %_45.i.i53.i to i32
  %_27.i.i55.i = or disjoint i32 %_43.i.i52.i, %_44.i.i54.i
  %89 = or disjoint i32 %_27.i.i55.i, %_25.i.i51.i
  br label %bb6.i

bb6.i:                                            ; preds = %bb8.i.i46.i, %bb6.i.i36.i, %bb3.i.i56.i, %bb4.i.i19.i
  %_0.sroa.4.0.i.ph.i31.i = phi i32 [ %87, %bb4.i.i19.i ], [ %88, %bb6.i.i36.i ], [ %89, %bb8.i.i46.i ], [ %_7.i.i57.i, %bb3.i.i56.i ]
  %90 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i, 1114112
  call void @llvm.assume(i1 %90), !noalias !52
  %_8.not.i.i = icmp ult i64 %83, %new_len.i74.i.i.i
  br i1 %_8.not.i.i, label %bb9.i.i, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb6.i
  %91 = icmp eq i64 %83, %new_len.i74.i.i.i
  br i1 %91, label %bb26.i, label %bb25.i

bb9.i.i:                                          ; preds = %bb6.i
  %92 = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 %83
  %self1.i.i = load i8, ptr %92, align 1, !alias.scope !159, !noalias !162, !noundef !2
  %93 = icmp sgt i8 %self1.i.i, -65
  br i1 %93, label %bb26.i, label %bb25.i

bb26.i:                                           ; preds = %bb9.i.i, %bb6.i.i
  %new_len.i.i = sub nuw i64 %new_len.i74.i.i.i, %83
  %data.i.i = getelementptr inbounds nuw i8, ptr %data.i75.i.i.i, i64 %83
; invoke heck::lowercase
  %_19.i6 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i, i64 noundef %new_len.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_19.i.noexc unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit

_19.i.noexc:                                      ; preds = %bb26.i
  br i1 %_19.i6, label %.noexc15.i, label %.noexc15.i.thread

bb25.i:                                           ; preds = %bb9.i.i, %bb6.i.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i75.i.i.i, i64 noundef %new_len.i74.i.i.i, i64 noundef %83, i64 noundef %new_len.i74.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc7 unwind label %cleanup.loopexit.i.loopexit.split-lp.loopexit.split-lp

.noexc7:                                          ; preds = %bb25.i
  unreachable

.noexc15.i.thread:                                ; preds = %_19.i.noexc, %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit85.i.i.i, %bb23.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %bb14.i, %bb1.i, %.noexc15.i.thread, %.noexc12.i, %.noexc11.i, %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  %.promoted154.off0.i.i.i.be = phi i1 [ %.promoted154.off0.i.i.i, %bb85.i.i.i ], [ %.promoted154.off0.i.i.i, %bb79.i.i.i ], [ %.promoted154.off0.i.i.i, %bb17.i.i.i ], [ %.promoted154.off0.i.i.i, %_36.i.i.noexc.i ], [ %.promoted154.off0.i.i.i, %_35.i.i.noexc.i ], [ %.promoted154.off0.i.i.i, %.noexc12.i ], [ false, %.noexc11.i ], [ %.promoted154.off0.i.i.i, %.noexc15.i.thread ], [ false, %bb1.i ], [ false, %bb14.i ]
  %.off0.i.i.i.be = phi i1 [ %.off0.i.i.i, %bb85.i.i.i ], [ %.off0.i.i.i, %bb79.i.i.i ], [ %.off0.i.i.i, %bb17.i.i.i ], [ %.off0.i.i.i, %_36.i.i.noexc.i ], [ %.off0.i.i.i, %_35.i.i.noexc.i ], [ false, %.noexc12.i ], [ false, %.noexc11.i ], [ false, %.noexc15.i.thread ], [ false, %bb1.i ], [ false, %bb14.i ]
  %init.sroa.0.0.i.i.i.be = phi i64 [ %init.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %init.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %init.sroa.0.0.i.i.i, %bb17.i.i.i ], [ %init.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ %_15.sroa.0.0233.i.i.i, %.noexc12.i ], [ %_15.sroa.0.0233.i.i.i, %.noexc11.i ], [ %char_indices.sroa.17.1232.i.i.i, %.noexc15.i.thread ], [ %char_indices.sroa.17.1232.i.i.i, %bb1.i ], [ %char_indices.sroa.17.1232.i.i.i, %bb14.i ]
  %mode.sroa.0.0.i.i.i.be = phi i8 [ %next_mode.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %bb17.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ 0, %.noexc12.i ], [ 0, %.noexc11.i ], [ 0, %.noexc15.i.thread ], [ 0, %bb1.i ], [ 0, %bb14.i ]
  br label %bb5.i.i.i

.noexc15.i:                                       ; preds = %.noexc5, %_19.i.noexc
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb2.i.i

cleanup.loopexit.i.loopexit:                      ; preds = %bb1.i.i.i.i242, %bb5.i, %.noexc53
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.i.loopexit.split-lp.loopexit:    ; preds = %bb26.i, %.noexc, %bb2.i, %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit60.i.i.i, %bb1.i59.i.i.i, %bb88.i.i.i, %bb82.i.i.i, %bb69.i.i.i, %bb63.i.i.i, %bb57.i.i.i
  %lpad.loopexit58 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.i.loopexit.split-lp.loopexit.split-lp: ; preds = %bb25.i
  %lpad.loopexit.split-lp59 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtBJ_11lower_camelINtB1j_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1e_s_0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %_RNCNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB6_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0B8_.exit.i.i.i, %bb1.i47.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.i.loopexit, %cleanup.loopexit.i.loopexit.split-lp.loopexit.split-lp, %cleanup.loopexit.i.loopexit.split-lp.loopexit, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit, %cleanup.loopexit.i.loopexit ], [ %lpad.loopexit58, %cleanup.loopexit.i.loopexit.split-lp.loopexit ], [ %lpad.loopexit.split-lp59, %cleanup.loopexit.i.loopexit.split-lp.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !163)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !163, !noalias !52
  %94 = icmp eq i64 %_1.val.i.i, 0
  br i1 %94, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !163, !noalias !52, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !166
  br label %bb4.i

bb2.i.i:                                          ; preds = %.noexc4.i, %.noexc3.i, %.noexc12.i, %.noexc11.i, %.noexc54, %.noexc15.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !52
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !52

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11lower_camel16AsLowerCamelCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !52
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !52
  ret void
}

; <str as heck::upper_camel::ToUpperCamelCase>::to_upper_camel_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck11upper_cameleNtB2_16ToUpperCamelCase19to_upper_camel_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_33.i = alloca [12 x i8], align 8
  %args.i = alloca [16 x i8], align 8
  %_11.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !167
  store i64 0, ptr %buf.i, align 8, !noalias !167
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !167
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !167
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !167
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !167
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !167
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !167
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !167
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !167
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_33.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_33.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 16
  %_13.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %iter.sroa.0.0142.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0141.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1174.0140.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1174.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0139.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1174.0140.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %4 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0139.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1174.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1174.0140.i.i.i, %bb2.i.i.i.i ]
  %5 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !173, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !173, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %7 = add i64 %4, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !173, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %8 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !173, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %9 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1174.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %9, %bb8.i.i.i.i.i.i.i.i.i ], [ %8, %bb6.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %10)
  %11 = ptrtoint ptr %iter.sroa.1174.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %11, %5
  %12 = add i64 %_10.i.i.i.i.i.i.i.i, %4
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %7, %bb3.thread.i.i.i.i.i.i.i ], [ %12, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1174.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %13 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %13, 10
  %14 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %15 = add nsw i32 %14, -65
  %16 = icmp ult i32 %15, 26
  %17 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %16
  br i1 %17, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !167

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs0_NtBJ_11upper_camelINtB1B_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs0_NtBJ_11upper_camelINtB1B_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %18 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !167

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs0_NtBJ_11upper_camelINtB1B_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %18, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %12, %.noexc.i ]
  %iter.sroa.1174.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1174.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1174.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1174.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0139.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.1174.6.ph.i.i.i = phi ptr [ %iter.sroa.1174.0140.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1174.1.i.i.i, %.noexc.i ], [ %iter.sroa.1174.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1174.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1174.3.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0142.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.0.0142.i.i.i, %bb5.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %4, %.noexc.i ], [ %4, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0142.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0142.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i.outer

bb5.i.i.i.outer:                                  ; preds = %bb5.i.i.i.outer.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i.ph = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.17.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %30, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.22.1.i.i.i.ph = phi i64 [ %char_indices.sroa.22.0141.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1182.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.25.0.i.i.i.ph = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.outer.backedge ]
  %init.sroa.0.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.ph.be, %bb5.i.i.i.outer.backedge ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb5.i.i.i.outer
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %char_indices.sroa.0.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %30, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.1.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.17.1182.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ %char_indices.sroa.25.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb5.i.i.i.outer ], [ %next_mode.sroa.0.0.i.i.i, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %19 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !170, !noalias !192, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !192, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %20 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread175.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i37.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread175.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !170, !noalias !192, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %21 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread175.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !170, !noalias !192, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %22 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread175.i.i.i

bb50.thread175.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i37.i.i.i, %bb3.i.i.i.i.i ], [ %22, %bb8.i.i.i.i.i ], [ %21, %bb6.i.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %23 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %19
  %24 = add i64 %_10.i.i.i.i, %23
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread175.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %23, %bb50.thread175.i.i.i ], [ %19, %bb5.i.i.i ]
  %_15.sroa.8.0184.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread175.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0183.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread175.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1182.i.i.i = phi i64 [ %24, %bb50.thread175.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1181.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread175.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1181.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb33.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1181.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1181.i.i.i, align 1, !alias.scope !170, !noalias !197, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i38.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1181.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !197, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i38.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1181.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !197, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %26 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1181.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !170, !noalias !197, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %27 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11upper_camel16AsUpperCamelCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i38.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i38.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i38.i.i.i ], [ %27, %bb8.i.i.i.i.i.i.i ], [ %26, %bb6.i.i.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %28)
  %29 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1182.i.i.i, %.pre-phi.i.i
  %30 = add i64 %_10.i.i.i.i.i.i, %29
  %31 = add nsw i32 %_15.sroa.8.0184.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %31, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb33.i.i.i:                                       ; preds = %bb1.i.i.i.i
  %32 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %32, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %33 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %33, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %34 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self1.i.i.i.i = load i8, ptr %34, align 1, !alias.scope !207, !noalias !210, !noundef !2
  %35 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %35, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i42.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i41.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::capitalize
  %_0.i.i.i3.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i42.i.i.i, i64 noundef %new_len.i41.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !167

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i3.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i67.i.i.i, %bb16.i63.i.i.i, %bb13.i69.i.i.i, %bb10.i56.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %36 = phi i64 [ %char_indices.sroa.17.1182.i.i.i, %bb15.i.i.i ], [ %char_indices.sroa.17.1182.i.i.i, %bb10.i56.i.i.i ], [ %char_indices.sroa.17.1182.i.i.i, %bb13.i69.i.i.i ], [ %char_indices.sroa.17.1182.i.i.i, %bb16.i63.i.i.i ], [ %char_indices.sroa.17.1182.i.i.i, %bb19.i67.i.i.i ], [ %_15.sroa.0.0183.i.i.i, %bb24.i.i.i ], [ %_15.sroa.0.0183.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0183.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0183.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0183.i.i.i, %bb19.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %37 = phi ptr [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i56.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i69.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i63.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i67.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i.ph, i64 noundef %36, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %37) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !167

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0184.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i5.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0184.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit, !noalias !167

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i5.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %38 = add nsw i32 %_15.sroa.8.0184.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %38, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0184.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit, !noalias !167

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i6.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %39 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %39, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %40 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %40, 26
  br i1 %or.cond2.i.i.i, label %bb15.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit, !noalias !167

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i7.i, label %bb15.i.i.i, label %bb17.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %41 = add nsw i32 %_15.sroa.8.0184.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %41, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0184.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0184.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit, !noalias !167

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i8.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %42 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond4.i.i.i, label %bb24.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit, !noalias !167

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i9.i, label %bb24.i.i.i, label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  br label %bb5.i.i.i

bb24.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %_15.sroa.0.0183.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i43.i.i.i

bb1.i43.i.i.i:                                    ; preds = %bb24.i.i.i
  %43 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %43, label %bb2.i48.i.i.i, label %bb9.i44.i.i.i

bb9.i44.i.i.i:                                    ; preds = %bb1.i43.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i48.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i43.i.i.i
  %44 = icmp eq i64 %_15.sroa.0.0183.i.i.i, 0
  br i1 %44, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i44.i.i.i
  %45 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %45, label %bb2.i48.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i44.i.i.i
  %46 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i.i.i.i = load i8, ptr %46, align 1, !alias.scope !211, !noalias !210, !noundef !2
  %47 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %47, label %bb2.i48.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i48.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0183.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %48 = icmp eq i64 %_15.sroa.0.0183.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %48, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %49 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0183.i.i.i
  %self2.i.i.i.i = load i8, ptr %49, align 1, !alias.scope !211, !noalias !210, !noundef !2
  %50 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %50, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i48.i.i.i
  %data.i50.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i49.i.i.i = sub nuw i64 %_15.sroa.0.0183.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::capitalize
  %_0.i51.i.i10.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i50.i.i.i, i64 noundef %new_len.i49.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i51.i.i.noexc.i unwind label %cleanup.loopexit.i.loopexit.loopexit.split-lp, !noalias !167

_0.i51.i.i.noexc.i:                               ; preds = %bb95.i.i.i
  br i1 %_0.i51.i.i10.i, label %bb2.i.i, label %bb5.i.i.i.outer.backedge, !prof !97

bb15.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  %_3.not.i52.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %char_indices.sroa.17.1182.i.i.i
  br i1 %_3.not.i52.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i53.i.i.i

bb1.i53.i.i.i:                                    ; preds = %bb15.i.i.i
  %51 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %51, label %bb2.i60.i.i.i, label %bb9.i54.i.i.i

bb9.i54.i.i.i:                                    ; preds = %bb1.i53.i.i.i
  %_11.not.i55.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i55.i.i.i, label %bb13.i69.i.i.i, label %bb10.i56.i.i.i

bb2.i60.i.i.i:                                    ; preds = %bb13.i69.i.i.i, %bb10.i56.i.i.i, %bb1.i53.i.i.i
  %52 = icmp eq i64 %char_indices.sroa.17.1182.i.i.i, 0
  br i1 %52, label %bb76.i.i.i, label %bb15.i61.i.i.i

bb10.i56.i.i.i:                                   ; preds = %bb9.i54.i.i.i
  %53 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %53, label %bb2.i60.i.i.i, label %bb101.i.i.invoke.i

bb13.i69.i.i.i:                                   ; preds = %bb9.i54.i.i.i
  %54 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i70.i.i.i = load i8, ptr %54, align 1, !alias.scope !214, !noalias !210, !noundef !2
  %55 = icmp sgt i8 %self.i70.i.i.i, -65
  br i1 %55, label %bb2.i60.i.i.i, label %bb101.i.i.invoke.i

bb15.i61.i.i.i:                                   ; preds = %bb2.i60.i.i.i
  %_17.not.i62.i.i.i = icmp ult i64 %char_indices.sroa.17.1182.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i62.i.i.i, label %bb19.i67.i.i.i, label %bb16.i63.i.i.i

bb16.i63.i.i.i:                                   ; preds = %bb15.i61.i.i.i
  %56 = icmp eq i64 %char_indices.sroa.17.1182.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %56, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i67.i.i.i:                                   ; preds = %bb15.i61.i.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1182.i.i.i
  %self2.i68.i.i.i = load i8, ptr %57, align 1, !alias.scope !214, !noalias !210, !noundef !2
  %58 = icmp sgt i8 %self2.i68.i.i.i, -65
  br i1 %58, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i67.i.i.i, %bb16.i63.i.i.i, %bb2.i60.i.i.i
  %data.i66.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i65.i.i.i = sub nuw i64 %char_indices.sroa.17.1182.i.i.i, %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !217)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i)
  %_28.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1182.i.i.i
  %59 = ptrtoint ptr %data.i66.i.i.i to i64
  %_6.i.i.i.i = icmp eq i64 %char_indices.sroa.17.1182.i.i.i, %init.sroa.0.0.i.i.i.ph
  br i1 %_6.i.i.i.i, label %_0.i72.i.i.noexc.i.thread, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb76.i.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 1
  %x.i.i.i = load i8, ptr %data.i66.i.i.i, align 1, !alias.scope !217, !noalias !220, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i.i4, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp samesign ne i64 %new_len.i65.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i), !noalias !167
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !217, !noalias !220, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %60 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb2.i

bb3.i.i.i4:                                       ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb2.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp samesign ne i64 %new_len.i65.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i), !noalias !167
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !217, !noalias !220, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %61 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i2, label %bb2.i

bb8.i.i.i2:                                       ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp samesign ne i64 %new_len.i65.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i), !noalias !167
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !217, !noalias !220, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i3 = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %62 = or disjoint i32 %_27.i.i.i, %_25.i.i.i3
  br label %bb2.i

bb2.i:                                            ; preds = %bb8.i.i.i2, %bb6.i.i.i, %bb3.i.i.i4, %bb4.i.i.i
  %char_indices.sroa.0.0.i = phi ptr [ %_16.i.i.i.i, %bb3.i.i.i4 ], [ %_16.i26.i.i.i, %bb8.i.i.i2 ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i12.i.i.i, %bb4.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %_7.i.i.i, %bb3.i.i.i4 ], [ %62, %bb8.i.i.i2 ], [ %61, %bb6.i.i.i ], [ %60, %bb4.i.i.i ]
  %63 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %63), !noalias !167
  %64 = ptrtoint ptr %char_indices.sroa.0.0.i to i64
  %65 = sub i64 %64, %59
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i), !noalias !226
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc unwind label %cleanup.loopexit.i.loopexit.loopexit.split-lp

.noexc:                                           ; preds = %bb2.i
  %_3.i.i = load i32, ptr %2, align 8, !range !14, !noalias !226, !noundef !2
  %66 = icmp eq i32 %_3.i.i, 0
  %_6.i.i = load i32, ptr %3, align 4, !range !14, !noalias !226
  %67 = icmp eq i32 %_6.i.i, 0
  %spec.select.i.i = select i1 %67, i64 1, i64 2
  %iter.sroa.4.0.i.i = select i1 %66, i64 %spec.select.i.i, i64 3
  store i64 0, ptr %_11.i, align 8, !noalias !226
  store i64 %iter.sroa.4.0.i.i, ptr %_32.sroa.4.0._11.sroa_idx.i, align 8, !noalias !226
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i, i64 12, i1 false), !noalias !226
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !226
  store ptr %_11.i, ptr %args.i, align 8, !noalias !226
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i, align 8, !noalias !226
; invoke core::fmt::write
  %68 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i)
          to label %.noexc5 unwind label %cleanup.loopexit.i.loopexit.loopexit.split-lp

.noexc5:                                          ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !226
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i), !noalias !226
  br i1 %68, label %_0.i72.i.i.noexc.i, label %bb23.i

bb23.i:                                           ; preds = %.noexc5
  %_6.i.i.i14.i = icmp eq ptr %char_indices.sroa.0.0.i, %_28.i
  br i1 %_6.i.i.i14.i, label %_0.i72.i.i.noexc.i.thread, label %bb14.i.i15.i

bb14.i.i15.i:                                     ; preds = %bb23.i
  %x.i.i17.i = load i8, ptr %char_indices.sroa.0.0.i, align 1, !alias.scope !217, !noalias !227, !noundef !2
  %_6.i.i18.i = icmp sgt i8 %x.i.i17.i, -1
  br i1 %_6.i.i18.i, label %bb3.i.i56.i, label %bb4.i.i19.i

bb4.i.i19.i:                                      ; preds = %bb14.i.i15.i
  %_16.i.i.i16.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 1
  %_30.i.i20.i = and i8 %x.i.i17.i, 31
  %init.i.i21.i = zext nneg i8 %_30.i.i20.i to i32
  %_6.i10.i.i22.i = icmp ne ptr %_16.i.i.i16.i, %_28.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i), !noalias !167
  %y.i.i24.i = load i8, ptr %_16.i.i.i16.i, align 1, !alias.scope !217, !noalias !227, !noundef !2
  %_33.i.i25.i = shl nuw nsw i32 %init.i.i21.i, 6
  %_35.i.i26.i = and i8 %y.i.i24.i, 63
  %_34.i.i27.i = zext nneg i8 %_35.i.i26.i to i32
  %69 = or disjoint i32 %_33.i.i25.i, %_34.i.i27.i
  %_13.i.i28.i = icmp samesign ugt i8 %x.i.i17.i, -33
  br i1 %_13.i.i28.i, label %bb6.i.i36.i, label %bb6.i

bb3.i.i56.i:                                      ; preds = %bb14.i.i15.i
  %_7.i.i57.i = zext nneg i8 %x.i.i17.i to i32
  br label %bb6.i

bb6.i.i36.i:                                      ; preds = %bb4.i.i19.i
  %_16.i12.i.i23.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 2
  %_6.i17.i.i37.i = icmp ne ptr %_16.i12.i.i23.i, %_28.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i), !noalias !167
  %z.i.i39.i = load i8, ptr %_16.i12.i.i23.i, align 1, !alias.scope !217, !noalias !227, !noundef !2
  %_38.i.i40.i = shl nuw nsw i32 %_34.i.i27.i, 6
  %_40.i.i41.i = and i8 %z.i.i39.i, 63
  %_39.i.i42.i = zext nneg i8 %_40.i.i41.i to i32
  %y_z.i.i43.i = or disjoint i32 %_38.i.i40.i, %_39.i.i42.i
  %_20.i.i44.i = shl nuw nsw i32 %init.i.i21.i, 12
  %70 = or disjoint i32 %y_z.i.i43.i, %_20.i.i44.i
  %_21.i.i45.i = icmp samesign ugt i8 %x.i.i17.i, -17
  br i1 %_21.i.i45.i, label %bb8.i.i46.i, label %bb6.i

bb8.i.i46.i:                                      ; preds = %bb6.i.i36.i
  %_16.i19.i.i38.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 3
  %_6.i24.i.i47.i = icmp ne ptr %_16.i19.i.i38.i, %_28.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i), !noalias !167
  %w.i.i49.i = load i8, ptr %_16.i19.i.i38.i, align 1, !alias.scope !217, !noalias !227, !noundef !2
  %_26.i.i50.i = shl nuw nsw i32 %init.i.i21.i, 18
  %_25.i.i51.i = and i32 %_26.i.i50.i, 1835008
  %_43.i.i52.i = shl nuw nsw i32 %y_z.i.i43.i, 6
  %_45.i.i53.i = and i8 %w.i.i49.i, 63
  %_44.i.i54.i = zext nneg i8 %_45.i.i53.i to i32
  %_27.i.i55.i = or disjoint i32 %_43.i.i52.i, %_44.i.i54.i
  %71 = or disjoint i32 %_27.i.i55.i, %_25.i.i51.i
  br label %bb6.i

bb6.i:                                            ; preds = %bb8.i.i46.i, %bb6.i.i36.i, %bb3.i.i56.i, %bb4.i.i19.i
  %_0.sroa.4.0.i.ph.i31.i = phi i32 [ %69, %bb4.i.i19.i ], [ %70, %bb6.i.i36.i ], [ %71, %bb8.i.i46.i ], [ %_7.i.i57.i, %bb3.i.i56.i ]
  %72 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i, 1114112
  call void @llvm.assume(i1 %72), !noalias !167
  %_8.not.i.i = icmp ult i64 %65, %new_len.i65.i.i.i
  br i1 %_8.not.i.i, label %bb9.i.i, label %bb6.i.i

bb6.i.i:                                          ; preds = %bb6.i
  %73 = icmp eq i64 %65, %new_len.i65.i.i.i
  br i1 %73, label %bb26.i, label %bb25.i

bb9.i.i:                                          ; preds = %bb6.i
  %74 = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 %65
  %self1.i.i = load i8, ptr %74, align 1, !alias.scope !232, !noalias !235, !noundef !2
  %75 = icmp sgt i8 %self1.i.i, -65
  br i1 %75, label %bb26.i, label %bb25.i

bb26.i:                                           ; preds = %bb9.i.i, %bb6.i.i
  %new_len.i.i = sub nuw i64 %new_len.i65.i.i.i, %65
  %data.i.i = getelementptr inbounds nuw i8, ptr %data.i66.i.i.i, i64 %65
; invoke heck::lowercase
  %_19.i6 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i, i64 noundef %new_len.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_19.i.noexc unwind label %cleanup.loopexit.i.loopexit.loopexit.split-lp

_19.i.noexc:                                      ; preds = %bb26.i
  br i1 %_19.i6, label %_0.i72.i.i.noexc.i, label %_0.i72.i.i.noexc.i.thread

bb25.i:                                           ; preds = %bb9.i.i, %bb6.i.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i66.i.i.i, i64 noundef %new_len.i65.i.i.i, i64 noundef %65, i64 noundef %new_len.i65.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc7 unwind label %cleanup.loopexit.i.loopexit.split-lp

.noexc7:                                          ; preds = %bb25.i
  unreachable

_0.i72.i.i.noexc.i.thread:                        ; preds = %_19.i.noexc, %bb76.i.i.i, %bb23.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb5.i.i.i.outer.backedge

bb5.i.i.i.outer.backedge:                         ; preds = %_0.i72.i.i.noexc.i.thread, %_0.i51.i.i.noexc.i
  %init.sroa.0.0.i.i.i.ph.be = phi i64 [ %_15.sroa.0.0183.i.i.i, %_0.i51.i.i.noexc.i ], [ %char_indices.sroa.17.1182.i.i.i, %_0.i72.i.i.noexc.i.thread ]
  br label %bb5.i.i.i.outer

_0.i72.i.i.noexc.i:                               ; preds = %.noexc5, %_19.i.noexc
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb2.i.i

cleanup.loopexit.i.loopexit.loopexit:             ; preds = %bb88.i.i.i, %bb82.i.i.i, %bb69.i.i.i, %bb63.i.i.i, %bb57.i.i.i
  %lpad.loopexit166 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.i.loopexit.loopexit.split-lp:    ; preds = %bb26.i, %.noexc, %bb2.i, %bb95.i.i.i
  %lpad.loopexit.split-lp167 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.i.loopexit.split-lp:             ; preds = %bb25.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs0_NtBJ_11upper_camelINtB1B_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.i.loopexit.loopexit, %cleanup.loopexit.i.loopexit.loopexit.split-lp, %cleanup.loopexit.i.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.i.loopexit.split-lp ], [ %lpad.loopexit166, %cleanup.loopexit.i.loopexit.loopexit ], [ %lpad.loopexit.split-lp167, %cleanup.loopexit.i.loopexit.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !236)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !236, !noalias !167
  %76 = icmp eq i64 %_1.val.i.i, 0
  br i1 %76, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !236, !noalias !167, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !239
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i51.i.i.noexc.i, %_0.i72.i.i.noexc.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !167
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc14.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !167

.noexc14.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11upper_camel16AsUpperCamelCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !167
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !167
  ret void
}

; <str as heck::shouty_kebab::ToShoutyKebabCase>::to_shouty_kebab_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck12shouty_kebabeNtB2_17ToShoutyKebabCase20to_shouty_kebab_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_29.i.i = alloca [12 x i8], align 4
  %args.i.i = alloca [16 x i8], align 8
  %_10.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !240
  store i64 0, ptr %buf.i, align 8, !noalias !240
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !240
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !240
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !240
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !240
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !240
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !240
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !240
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !240
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_29.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_29.i.i, i64 4
  %_28.sroa.4.0._10.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 8
  %_28.sroa.5.0._10.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 16
  %_12.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %4 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %5 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !246, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !246, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %7 = add i64 %4, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !246, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %8 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !246, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %9 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %9, %bb8.i.i.i.i.i.i.i.i.i ], [ %8, %bb6.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %10)
  %11 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %11, %5
  %12 = add i64 %_10.i.i.i.i.i.i.i.i, %4
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %7, %bb3.thread.i.i.i.i.i.i.i ], [ %12, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %13 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %13, 10
  %14 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %15 = add nsw i32 %14, -65
  %16 = icmp ult i32 %15, 26
  %17 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %16
  br i1 %17, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !240

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs_NtBJ_12shouty_kebabINtB1y_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs_NtBJ_12shouty_kebabINtB1y_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %18 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !240

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs_NtBJ_12shouty_kebabINtB1y_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %18, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %.noexc.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %12, %.noexc.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ], [ %4, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %4, %.noexc.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %30, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %init.sroa.0.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb3.i.i.i ], [ %mode.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %first_word.sroa.0.1.off0.i.i.i = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i.be, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %19 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !243, !noalias !265, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !265, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %20 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !243, !noalias !265, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %21 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !243, !noalias !265, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %22 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %22, %bb8.i.i.i.i.i ], [ %21, %bb6.i.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %23 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %19
  %24 = add i64 %_10.i.i.i.i, %23
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %23, %bb50.thread189.i.i.i ], [ %19, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %24, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !243, !noalias !270, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !270, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !270, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %26 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !243, !noalias !270, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %27 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_kebab17AsShoutyKebabCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %27, %bb8.i.i.i.i.i.i.i ], [ %26, %bb6.i.i.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %28)
  %29 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %30 = add i64 %_10.i.i.i.i.i.i, %29
  %31 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %31, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !280)
  call void @llvm.experimental.noalias.scope.decl(metadata !283)
  call void @llvm.experimental.noalias.scope.decl(metadata !286)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !289, !noalias !292, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !289, !noalias !292, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !295, !noalias !292
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !295, !noalias !292, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 45, ptr %dst.i.i.i.i, align 1, !noalias !296
  %32 = add nuw i64 %len.i.i.i.i, 1
  store i64 %32, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !295, !noalias !292
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %33 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %33, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %34 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %34, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %35 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self1.i.i.i.i = load i8, ptr %35, align 1, !alias.scope !297, !noalias !300, !noundef !2
  %36 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %36, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i
; invoke heck::uppercase
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9uppercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !240

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %37 = phi i64 [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %38 = phi ptr [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i, i64 noundef %37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %38) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !240

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %39 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %39, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %40 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %40, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %41 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %41, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %42 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %43 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !301)
  call void @llvm.experimental.noalias.scope.decl(metadata !304)
  call void @llvm.experimental.noalias.scope.decl(metadata !307)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !310, !noalias !313, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !310, !noalias !313, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.split-lp.i

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !316, !noalias !313
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !316, !noalias !313, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 45, ptr %dst.i.i.i137.i, align 1, !noalias !317
  %44 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %44, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !316, !noalias !313
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %45 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %45, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %46 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %46, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %47 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %47, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %48 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i.i.i.i = load i8, ptr %48, align 1, !alias.scope !318, !noalias !300, !noundef !2
  %49 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %49, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %50, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %51 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %51, align 1, !alias.scope !318, !noalias !300, !noundef !2
  %52 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %52, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i
; invoke heck::uppercase
  %_0.i56.i.i12.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9uppercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i56.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !240

_0.i56.i.i.noexc.i:                               ; preds = %bb95.i.i.i
  br i1 %_0.i56.i.i12.i, label %bb2.i.i, label %bb5.i.i.i.backedge, !prof !97

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !321)
  call void @llvm.experimental.noalias.scope.decl(metadata !324)
  call void @llvm.experimental.noalias.scope.decl(metadata !327)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !330, !noalias !333, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !330, !noalias !333, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.split-lp.i

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !336, !noalias !333
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !336, !noalias !333, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 45, ptr %dst.i.i.i149.i, align 1, !noalias !337
  %53 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %53, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !336, !noalias !333
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %54 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %54, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %55 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %55, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %56 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %56, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i75.i.i.i = load i8, ptr %57, align 1, !alias.scope !338, !noalias !300, !noundef !2
  %58 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %58, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %59 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %59, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %60 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %60, align 1, !alias.scope !338, !noalias !300, !noundef !2
  %61 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %61, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !341)
  %_19.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %_6.i.i16.not.i.i = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i
  br i1 %_6.i.i16.not.i.i, label %bb5.i.i.i.backedge, label %bb14.i.lr.ph.i.i

bb5.i.i.i.backedge:                               ; preds = %bb19.i.i, %bb76.i.i.i, %_0.i56.i.i.noexc.i, %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  %init.sroa.0.0.i.i.i.be = phi i64 [ %init.sroa.0.0.i.i.i, %bb76.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %init.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %init.sroa.0.0.i.i.i, %bb17.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i.i ]
  %mode.sroa.0.0.i.i.i.be = phi i8 [ 0, %bb76.i.i.i ], [ 0, %_0.i56.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %bb17.i.i.i ], [ 0, %bb19.i.i ]
  %first_word.sroa.0.1.off0.i.i.i.be = phi i1 [ false, %bb76.i.i.i ], [ false, %_0.i56.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %_36.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb85.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i, %_35.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb79.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb17.i.i.i ], [ false, %bb19.i.i ]
  br label %bb5.i.i.i

bb14.i.lr.ph.i.i:                                 ; preds = %bb76.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  br label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb19.i.i, %bb14.i.lr.ph.i.i
  %iter.sroa.0.017.i.i = phi ptr [ %data.i71.i.i.i, %bb14.i.lr.ph.i.i ], [ %iter.sroa.0.1.ph.i.i, %bb19.i.i ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 1
  %x.i.i.i = load i8, ptr %iter.sroa.0.017.i.i, align 1, !alias.scope !341, !noalias !344, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i156.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !341, !noalias !344, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %62 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb10.i.i

bb3.i.i156.i:                                     ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb10.i.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !341, !noalias !344, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %63 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i154.i, label %bb10.i.i

bb8.i.i154.i:                                     ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !341, !noalias !344, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i155.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %64 = or disjoint i32 %_27.i.i.i, %_25.i.i155.i
  br label %bb10.i.i

bb10.i.i:                                         ; preds = %bb8.i.i154.i, %bb6.i.i.i, %bb3.i.i156.i, %bb4.i.i.i
  %iter.sroa.0.1.ph.i.i = phi ptr [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i154.i ], [ %_16.i.i.i.i, %bb3.i.i156.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %62, %bb4.i.i.i ], [ %63, %bb6.i.i.i ], [ %64, %bb8.i.i154.i ], [ %_7.i.i.i, %bb3.i.i156.i ]
  %65 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %65)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10.i.i), !noalias !348
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_29.i.i), !noalias !348
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_29.i.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc157.i unwind label %cleanup.loopexit.loopexit.i, !noalias !240

.noexc157.i:                                      ; preds = %bb10.i.i
  %_3.i.i.i = load i32, ptr %2, align 4, !range !14, !alias.scope !349, !noalias !352, !noundef !2
  %66 = icmp eq i32 %_3.i.i.i, 0
  %_6.i8.i.i = load i32, ptr %3, align 4, !range !14, !alias.scope !349, !noalias !352
  %67 = icmp eq i32 %_6.i8.i.i, 0
  %spec.select.i.i.i = select i1 %67, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %66, i64 %spec.select.i.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_28.sroa.5.0._10.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(12) %_29.i.i, i64 12, i1 false), !noalias !348
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_29.i.i), !noalias !348
  store i64 0, ptr %_10.i.i, align 8, !noalias !348
  store i64 %iter.sroa.4.0.i.i.i, ptr %_28.sroa.4.0._10.sroa_idx.i.i, align 8, !noalias !348
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !348
  store ptr %_10.i.i, ptr %args.i.i, align 8, !noalias !348
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_12.sroa.4.0..sroa_idx.i.i, align 8, !noalias !348
; invoke core::fmt::write
  %68 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc158.i unwind label %cleanup.loopexit.loopexit.i, !noalias !240

.noexc158.i:                                      ; preds = %.noexc157.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !348
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10.i.i), !noalias !348
  br i1 %68, label %bb2.i.i, label %bb19.i.i, !prof !97

bb19.i.i:                                         ; preds = %.noexc158.i
  %_6.i.i.not.i.i = icmp eq ptr %iter.sroa.0.1.ph.i.i, %_19.i.i
  br i1 %_6.i.i.not.i.i, label %bb5.i.i.i.backedge, label %bb14.i.i.i

cleanup.loopexit.loopexit.i:                      ; preds = %.noexc157.i, %bb10.i.i
  %lpad.loopexit160.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i:             ; preds = %bb1.i.i.i.i138.i, %bb1.i.i.i.i150.i, %bb95.i.i.i, %bb88.i.i.i, %bb82.i.i.i, %bb69.i.i.i, %bb63.i.i.i, %bb57.i.i.i
  %lpad.loopexit.split-lp161.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs_NtBJ_12shouty_kebabINtB1y_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.split-lp.i, %cleanup.loopexit.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit160.i, %cleanup.loopexit.loopexit.i ], [ %lpad.loopexit.split-lp161.i, %cleanup.loopexit.loopexit.split-lp.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !354)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !354, !noalias !240
  %69 = icmp eq i64 %_1.val.i.i, 0
  br i1 %69, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !354, !noalias !240, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !357
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %.noexc158.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !240
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !240

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_kebab17AsShoutyKebabCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !240
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !240
  ret void
}

; <str as heck::kebab::ToKebabCase>::to_kebab_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck5kebabeNtB2_11ToKebabCase13to_kebab_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_63.i.i = alloca [12 x i8], align 4
  %args.i.i = alloca [16 x i8], align 8
  %_13.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !358
  store i64 0, ptr %buf.i, align 8, !noalias !358
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !358
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !358
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !358
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !358
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !358
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !358
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !358
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !358
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_63.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_63.i.i, i64 4
  %_62.sroa.4.0._13.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_13.i.i, i64 8
  %_62.sroa.5.0._13.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_13.i.i, i64 16
  %_15.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %4 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %5 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !364, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !364, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %7 = add i64 %4, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !364, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %8 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !364, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %9 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %9, %bb8.i.i.i.i.i.i.i.i.i ], [ %8, %bb6.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %10)
  %11 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %11, %5
  %12 = add i64 %_10.i.i.i.i.i.i.i.i, %4
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %7, %bb3.thread.i.i.i.i.i.i.i ], [ %12, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %13 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %13, 10
  %14 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %15 = add nsw i32 %14, -65
  %16 = icmp ult i32 %15, 26
  %17 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %16
  br i1 %17, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !358

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs_NtBJ_5kebabINtB1y_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs_NtBJ_5kebabINtB1y_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %18 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !358

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs_NtBJ_5kebabINtB1y_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %18, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %4, %.noexc.i ], [ %4, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i.outer

bb5.i.i.i.outer:                                  ; preds = %bb5.i.i.i.outer.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i.ph = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.17.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %30, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.22.1.i.i.i.ph = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.25.0.i.i.i.ph = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.outer.backedge ]
  %init.sroa.0.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.ph.be, %bb5.i.i.i.outer.backedge ]
  %first_word.sroa.0.1.off0.i.i.i.ph = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ false, %bb5.i.i.i.outer.backedge ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb5.i.i.i.outer
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %char_indices.sroa.0.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %30, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.1.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ %char_indices.sroa.25.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb5.i.i.i.outer ], [ %next_mode.sroa.0.0.i.i.i, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %19 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !361, !noalias !383, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !383, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %20 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !361, !noalias !383, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %21 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !361, !noalias !383, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %22 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %22, %bb8.i.i.i.i.i ], [ %21, %bb6.i.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %23 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %19
  %24 = add i64 %_10.i.i.i.i, %23
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %23, %bb50.thread189.i.i.i ], [ %19, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %24, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !361, !noalias !388, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !388, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !388, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %26 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !361, !noalias !388, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %27 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i.ph, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5kebab11AsKebabCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %27, %bb8.i.i.i.i.i.i.i ], [ %26, %bb6.i.i.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %28)
  %29 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %30 = add i64 %_10.i.i.i.i.i.i, %29
  %31 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %31, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !398)
  call void @llvm.experimental.noalias.scope.decl(metadata !401)
  call void @llvm.experimental.noalias.scope.decl(metadata !404)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !407, !noalias !410, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !407, !noalias !410, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !413, !noalias !410
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !413, !noalias !410, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 45, ptr %dst.i.i.i.i, align 1, !noalias !414
  %32 = add nuw i64 %len.i.i.i.i, 1
  store i64 %32, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !413, !noalias !410
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %33 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %33, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %34 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %34, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %35 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self1.i.i.i.i = load i8, ptr %35, align 1, !alias.scope !415, !noalias !418, !noundef !2
  %36 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %36, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::lowercase
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !358

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %37 = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %38 = phi ptr [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i.ph, i64 noundef %37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %38) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !358

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !358

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %39 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %39, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !358

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %40 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %40, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %41 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %41, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !358

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %42 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !358

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %43 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !358

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  br label %bb5.i.i.i

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !419)
  call void @llvm.experimental.noalias.scope.decl(metadata !422)
  call void @llvm.experimental.noalias.scope.decl(metadata !425)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !428, !noalias !431, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !428, !noalias !431, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !434, !noalias !431
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !434, !noalias !431, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 45, ptr %dst.i.i.i137.i, align 1, !noalias !435
  %44 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %44, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !434, !noalias !431
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %45 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %45, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %46 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %46, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %47 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %47, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %48 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i.i.i.i = load i8, ptr %48, align 1, !alias.scope !436, !noalias !418, !noundef !2
  %49 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %49, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %50, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %51 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %51, align 1, !alias.scope !436, !noalias !418, !noundef !2
  %52 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %52, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::lowercase
  %_0.i56.i.i12.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i56.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp, !noalias !358

_0.i56.i.i.noexc.i:                               ; preds = %bb95.i.i.i
  br i1 %_0.i56.i.i12.i, label %bb2.i.i, label %bb5.i.i.i.outer.backedge, !prof !97

bb5.i.i.i.outer.backedge:                         ; preds = %bb1.i.i, %bb14.i.i, %_0.i56.i.i.noexc.i
  %init.sroa.0.0.i.i.i.ph.be = phi i64 [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb14.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb1.i.i ]
  br label %bb5.i.i.i.outer

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !439)
  call void @llvm.experimental.noalias.scope.decl(metadata !442)
  call void @llvm.experimental.noalias.scope.decl(metadata !445)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !448, !noalias !451, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !448, !noalias !451, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !454, !noalias !451
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !454, !noalias !451, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 45, ptr %dst.i.i.i149.i, align 1, !noalias !455
  %53 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %53, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !454, !noalias !451
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %54 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %54, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %55 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %55, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %56 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %56, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i75.i.i.i = load i8, ptr %57, align 1, !alias.scope !456, !noalias !418, !noundef !2
  %58 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %58, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %59 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %59, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %60 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %60, align 1, !alias.scope !456, !noalias !418, !noundef !2
  %61 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %61, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !459)
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb1.i.i.backedge, %bb76.i.i.i
  %chars.sroa.17.0.i.i = phi i32 [ 1114113, %bb76.i.i.i ], [ %chars.sroa.17.0.i.i.be, %bb1.i.i.backedge ]
  %chars.sroa.0.0.i.i = phi ptr [ %data.i71.i.i.i, %bb76.i.i.i ], [ %chars.sroa.0.0.i.i.be, %bb1.i.i.backedge ]
  switch i32 %chars.sroa.17.0.i.i, label %bb2.i154.i [
    i32 1114113, label %bb14.i.i
    i32 1114112, label %bb5.i.i.i.outer.backedge
  ]

bb14.i.i:                                         ; preds = %bb1.i.i
  %_6.i.i.i.i = icmp eq ptr %chars.sroa.0.0.i.i, %_22.i.i
  br i1 %_6.i.i.i.i, label %bb5.i.i.i.outer.backedge, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb14.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 1
  %x.i.i.i = load i8, ptr %chars.sroa.0.0.i.i, align 1, !alias.scope !459, !noalias !462, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i158.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !459, !noalias !462, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %62 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb18.i.i

bb3.i.i158.i:                                     ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb18.i.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !459, !noalias !462, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %63 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i156.i, label %bb18.i.i

bb8.i.i156.i:                                     ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !459, !noalias !462, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i157.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %64 = or disjoint i32 %_27.i.i.i, %_25.i.i157.i
  br label %bb18.i.i

bb18.i.i:                                         ; preds = %bb8.i.i156.i, %bb6.i.i.i, %bb3.i.i158.i, %bb4.i.i.i
  %chars.sroa.0.4.ph.i.i = phi ptr [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i156.i ], [ %_16.i.i.i.i, %bb3.i.i158.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %62, %bb4.i.i.i ], [ %63, %bb6.i.i.i ], [ %64, %bb8.i.i156.i ], [ %_7.i.i.i, %bb3.i.i158.i ]
  %65 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %65)
  br label %bb2.i154.i

bb2.i154.i:                                       ; preds = %bb18.i.i, %bb1.i.i
  %chars.sroa.0.1.i.i = phi ptr [ %chars.sroa.0.0.i.i, %bb1.i.i ], [ %chars.sroa.0.4.ph.i.i, %bb18.i.i ]
  %_5.sroa.0.0.i.i = phi i32 [ %chars.sroa.17.0.i.i, %bb1.i.i ], [ %_0.sroa.4.0.i.ph.i.i, %bb18.i.i ]
  %66 = icmp eq i32 %_5.sroa.0.0.i.i, 931
  br i1 %66, label %bb1.i.i.i, label %bb5.i.i

bb1.i.i.i:                                        ; preds = %bb2.i154.i
  %_6.i.i.not.i.i.i.i.i = icmp eq ptr %chars.sroa.0.1.i.i, %_22.i.i
  br i1 %_6.i.i.not.i.i.i.i.i, label %bb47.i.i, label %bb14.i.i.i.i.i.i

bb14.i.i.i.i.i.i:                                 ; preds = %bb1.i.i.i
  %_16.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 1
  %x.i.i.i.i.i.i = load i8, ptr %chars.sroa.0.1.i.i, align 1, !alias.scope !459, !noalias !466, !noundef !2
  %_6.i.i.i.i.i155.i = icmp sgt i8 %x.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i155.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_30.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 2
  %y.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !459, !noalias !466, !noundef !2
  %_33.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i to i32
  %67 = or disjoint i32 %_33.i.i.i.i.i.i, %_34.i.i.i.i.i.i
  %_13.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %bb5.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i to i32
  br label %bb5.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 3
  %z.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i, align 1, !alias.scope !459, !noalias !466, !noundef !2
  %_38.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i, %_39.i.i.i.i.i.i
  %_20.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 12
  %68 = or disjoint i32 %y_z.i.i.i.i.i.i, %_20.i.i.i.i.i.i
  %_21.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, label %bb5.i.i

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i: ; preds = %bb6.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 4
  %w.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i, align 1, !alias.scope !459, !noalias !466, !noundef !2
  %_26.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i, %_44.i.i.i.i.i.i
  %69 = or disjoint i32 %_27.i.i.i.i.i.i, %_25.i.i.i.i.i.i
  %.not12.i.i = icmp eq i32 %69, 1114112
  br i1 %.not12.i.i, label %bb47.i.i, label %bb5.i.i

bb5.i.i:                                          ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, %bb6.i.i.i.i.i.i, %bb3.i.i.i.i.i.i, %bb4.i.i.i.i.i.i, %bb2.i154.i
  %chars.sroa.17.1.i.i = phi i32 [ %69, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ 1114113, %bb2.i154.i ], [ %67, %bb4.i.i.i.i.i.i ], [ %68, %bb6.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %chars.sroa.0.2.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ %chars.sroa.0.1.i.i, %bb2.i154.i ], [ %_16.i12.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i.i), !noalias !476
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63.i.i), !noalias !476
; invoke core::unicode::unicode_data::conversions::to_lower
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63.i.i, i32 noundef %_5.sroa.0.0.i.i)
          to label %.noexc159.i unwind label %cleanup.loopexit.loopexit.i, !noalias !358

.noexc159.i:                                      ; preds = %bb5.i.i
  %_3.i.i.i = load i32, ptr %2, align 4, !range !14, !alias.scope !477, !noalias !480, !noundef !2
  %70 = icmp eq i32 %_3.i.i.i, 0
  %_6.i13.i.i = load i32, ptr %3, align 4, !range !14, !alias.scope !477, !noalias !480
  %71 = icmp eq i32 %_6.i13.i.i, 0
  %spec.select.i.i.i = select i1 %71, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %70, i64 %spec.select.i.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(12) %_63.i.i, i64 12, i1 false), !noalias !476
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63.i.i), !noalias !476
  store i64 0, ptr %_13.i.i, align 8, !noalias !476
  store i64 %iter.sroa.4.0.i.i.i, ptr %_62.sroa.4.0._13.sroa_idx.i.i, align 8, !noalias !476
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !476
  store ptr %_13.i.i, ptr %args.i.i, align 8, !noalias !476
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx.i.i, align 8, !noalias !476
; invoke core::fmt::write
  %72 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc160.i unwind label %cleanup.loopexit.loopexit.i, !noalias !358

.noexc160.i:                                      ; preds = %.noexc159.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !476
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i.i), !noalias !476
  br i1 %72, label %bb2.i.i, label %bb1.i.i.backedge, !prof !97

bb47.i.i:                                         ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, %bb1.i.i.i
  %chars.sroa.0.528.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ %_22.i.i, %bb1.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !482)
  call void @llvm.experimental.noalias.scope.decl(metadata !485)
  call void @llvm.experimental.noalias.scope.decl(metadata !488)
  %len.i.i.i.i295.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !491, !noalias !494, !noundef !2
  %self2.i.i.i.i296.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !491, !noalias !494, !noundef !2
  %_9.i.i.i.i.i = sub i64 %self2.i.i.i.i296.i, %len.i.i.i.i295.i
  %_7.i.i.i.i297.i = icmp ult i64 %_9.i.i.i.i.i, 2
  br i1 %_7.i.i.i.i297.i, label %bb1.i.i.i.i302.i, label %.noexc161.i, !prof !142

bb1.i.i.i.i302.i:                                 ; preds = %bb47.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i295.i, i64 noundef 2)
          to label %.noexc304.i unwind label %cleanup.loopexit.loopexit.i

.noexc304.i:                                      ; preds = %bb1.i.i.i.i302.i
  %len.pre.i.i.i303.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !497, !noalias !494
  br label %.noexc161.i

.noexc161.i:                                      ; preds = %.noexc304.i, %bb47.i.i
  %len.i.i.i298.i = phi i64 [ %len.i.i.i.i295.i, %bb47.i.i ], [ %len.pre.i.i.i303.i, %.noexc304.i ]
  %_9.i.i.i299.i = icmp sgt i64 %len.i.i.i298.i, -1
  call void @llvm.assume(i1 %_9.i.i.i299.i)
  %_10.i.i.i300.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !497, !noalias !494, !nonnull !2, !noundef !2
  %dst.i.i.i301.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i300.i, i64 %len.i.i.i298.i
  store i16 -32049, ptr %dst.i.i.i301.i, align 1, !noalias !498
  %73 = add nuw i64 %len.i.i.i298.i, 2
  store i64 %73, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !497, !noalias !494
  br label %bb1.i.i.backedge

bb1.i.i.backedge:                                 ; preds = %.noexc161.i, %.noexc160.i
  %chars.sroa.17.0.i.i.be = phi i32 [ 1114112, %.noexc161.i ], [ %chars.sroa.17.1.i.i, %.noexc160.i ]
  %chars.sroa.0.0.i.i.be = phi ptr [ %chars.sroa.0.528.i.i, %.noexc161.i ], [ %chars.sroa.0.2.i.i, %.noexc160.i ]
  br label %bb1.i.i

cleanup.loopexit.loopexit.i:                      ; preds = %bb1.i.i.i.i302.i, %.noexc159.i, %bb5.i.i
  %lpad.loopexit163.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i.loopexit:    ; preds = %bb57.i.i.i, %bb63.i.i.i, %bb69.i.i.i, %bb82.i.i.i, %bb88.i.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp: ; preds = %bb95.i.i.i, %bb1.i.i.i.i150.i, %bb1.i.i.i.i138.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs_NtBJ_5kebabINtB1y_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.loopexit.split-lp.i.loopexit, %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit163.i, %cleanup.loopexit.loopexit.i ], [ %lpad.loopexit, %cleanup.loopexit.loopexit.split-lp.i.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !499)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !499, !noalias !358
  %74 = icmp eq i64 %_1.val.i.i, 0
  br i1 %74, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !499, !noalias !358, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !502
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %.noexc160.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !358
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !358

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5kebab11AsKebabCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !358
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !358
  ret void
}

; <str as heck::title::ToTitleCase>::to_title_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck5titleeNtB2_11ToTitleCase13to_title_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_33.i = alloca [12 x i8], align 8
  %args.i42 = alloca [16 x i8], align 8
  %_11.i = alloca [32 x i8], align 8
  %_63.i = alloca [12 x i8], align 4
  %args.i = alloca [16 x i8], align 8
  %_13.i = alloca [32 x i8], align 8
  %_33.i.i = alloca [12 x i8], align 8
  %args.i.i = alloca [16 x i8], align 8
  %_11.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !503
  store i64 0, ptr %buf.i, align 8, !noalias !503
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !503
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !503
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !503
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !503
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !503
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !503
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !503
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !503
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_33.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_33.i.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 16
  %_13.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  %4 = getelementptr inbounds nuw i8, ptr %_33.i, i64 8
  %5 = getelementptr inbounds nuw i8, ptr %_33.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 16
  %_13.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i42, i64 8
  %6 = getelementptr inbounds nuw i8, ptr %_63.i, i64 8
  %7 = getelementptr inbounds nuw i8, ptr %_63.i, i64 4
  %_62.sroa.4.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 8
  %_62.sroa.5.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 16
  %_15.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %8 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %9 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !509, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !509, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %10 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %11 = add i64 %8, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !509, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %12 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !509, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %13 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %13, %bb8.i.i.i.i.i.i.i.i.i ], [ %12, %bb6.i.i.i.i.i.i.i.i.i ], [ %10, %bb4.i.i.i.i.i.i.i.i.i ]
  %14 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %14)
  %15 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %15, %9
  %16 = add i64 %_10.i.i.i.i.i.i.i.i, %8
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %11, %bb3.thread.i.i.i.i.i.i.i ], [ %16, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %17 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %17, 10
  %18 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %19 = add nsw i32 %18, -65
  %20 = icmp ult i32 %19, 26
  %21 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %20
  br i1 %21, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !503

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5titleINtB1A_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5titleINtB1A_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %22 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !503

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5titleINtB1A_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %22, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %16, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %16, %.noexc.i ], [ %16, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %16, %.noexc.i ], [ %16, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %8, %.noexc.i ], [ %8, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %8, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %8, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i.outer

bb5.i.i.i.outer:                                  ; preds = %bb5.i.i.i.outer.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i.ph = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.17.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %34, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.22.1.i.i.i.ph = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.25.0.i.i.i.ph = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.outer.backedge ]
  %init.sroa.0.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.ph.be, %bb5.i.i.i.outer.backedge ]
  %first_word.sroa.0.1.off0.i.i.i.ph = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ false, %bb5.i.i.i.outer.backedge ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb5.i.i.i.outer
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %char_indices.sroa.0.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %34, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.1.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ %char_indices.sroa.25.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb5.i.i.i.outer ], [ %next_mode.sroa.0.0.i.i.i, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %23 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !506, !noalias !528, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !528, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %24 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !506, !noalias !528, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %25 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !506, !noalias !528, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %26 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %26, %bb8.i.i.i.i.i ], [ %25, %bb6.i.i.i.i.i ], [ %24, %bb4.i.i.i.i.i ]
  %27 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %23
  %28 = add i64 %_10.i.i.i.i, %27
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %27, %bb50.thread189.i.i.i ], [ %23, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %28, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !506, !noalias !533, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !533, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %29 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !533, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %30 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !506, !noalias !533, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %31 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i.ph, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5title11AsTitleCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %31, %bb8.i.i.i.i.i.i.i ], [ %30, %bb6.i.i.i.i.i.i.i ], [ %29, %bb4.i.i.i.i.i.i.i ]
  %32 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %32)
  %33 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %34 = add i64 %_10.i.i.i.i.i.i, %33
  %35 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %35, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !543)
  call void @llvm.experimental.noalias.scope.decl(metadata !546)
  call void @llvm.experimental.noalias.scope.decl(metadata !549)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !552, !noalias !555, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !552, !noalias !555, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !558, !noalias !555
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !558, !noalias !555, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 32, ptr %dst.i.i.i.i, align 1, !noalias !559
  %36 = add nuw i64 %len.i.i.i.i, 1
  store i64 %36, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !558, !noalias !555
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %37 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %37, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %38 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %38, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %39 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self1.i.i.i.i = load i8, ptr %39, align 1, !alias.scope !560, !noalias !563, !noundef !2
  %40 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %40, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::capitalize
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !503

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %41 = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %42 = phi ptr [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i.ph, i64 noundef %41, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %42) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !503

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !503

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %43 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !503

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %44 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %44, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %45 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %45, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !503

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %46 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %46, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !503

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %47 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %47, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !503

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  br label %bb5.i.i.i

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !564)
  call void @llvm.experimental.noalias.scope.decl(metadata !567)
  call void @llvm.experimental.noalias.scope.decl(metadata !570)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !573, !noalias !576, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !573, !noalias !576, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !579, !noalias !576
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !579, !noalias !576, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 32, ptr %dst.i.i.i137.i, align 1, !noalias !580
  %48 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %48, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !579, !noalias !576
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %49 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %49, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %50, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %51 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %51, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %52 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i.i.i.i = load i8, ptr %52, align 1, !alias.scope !581, !noalias !563, !noundef !2
  %53 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %53, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %54 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %54, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %55 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %55, align 1, !alias.scope !581, !noalias !563, !noundef !2
  %56 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %56, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !584)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i)
  %_28.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %57 = ptrtoint ptr %data.i55.i.i.i to i64
  %_6.i.i.i.i43 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
  br i1 %_6.i.i.i.i43, label %_0.i56.i.i.noexc.i.thread, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb95.i.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 1
  %x.i.i.i = load i8, ptr %data.i55.i.i.i, align 1, !alias.scope !584, !noalias !587, !noundef !2
  %_6.i.i.i44 = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i44, label %bb3.i.i.i55, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i), !noalias !503
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !584, !noalias !587, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %58 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i52, label %bb2.i45

bb3.i.i.i55:                                      ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb2.i45

bb6.i.i.i52:                                      ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i), !noalias !503
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !584, !noalias !587, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %59 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i53, label %bb2.i45

bb8.i.i.i53:                                      ; preds = %bb6.i.i.i52
  %_6.i24.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i), !noalias !503
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !584, !noalias !587, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i54 = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %60 = or disjoint i32 %_27.i.i.i, %_25.i.i.i54
  br label %bb2.i45

bb2.i45:                                          ; preds = %bb8.i.i.i53, %bb6.i.i.i52, %bb3.i.i.i55, %bb4.i.i.i
  %char_indices.sroa.0.0.i = phi ptr [ %_16.i.i.i.i, %bb3.i.i.i55 ], [ %_16.i26.i.i.i, %bb8.i.i.i53 ], [ %_16.i19.i.i.i, %bb6.i.i.i52 ], [ %_16.i12.i.i.i, %bb4.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %_7.i.i.i, %bb3.i.i.i55 ], [ %60, %bb8.i.i.i53 ], [ %59, %bb6.i.i.i52 ], [ %58, %bb4.i.i.i ]
  %61 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %61), !noalias !503
  %62 = ptrtoint ptr %char_indices.sroa.0.0.i to i64
  %63 = sub i64 %62, %57
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i), !noalias !593
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc56 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc56:                                         ; preds = %bb2.i45
  %_3.i.i46 = load i32, ptr %4, align 8, !range !14, !noalias !593, !noundef !2
  %64 = icmp eq i32 %_3.i.i46, 0
  %_6.i.i47 = load i32, ptr %5, align 4, !range !14, !noalias !593
  %65 = icmp eq i32 %_6.i.i47, 0
  %spec.select.i.i48 = select i1 %65, i64 1, i64 2
  %iter.sroa.4.0.i.i49 = select i1 %64, i64 %spec.select.i.i48, i64 3
  store i64 0, ptr %_11.i, align 8, !noalias !593
  store i64 %iter.sroa.4.0.i.i49, ptr %_32.sroa.4.0._11.sroa_idx.i, align 8, !noalias !593
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i, i64 12, i1 false), !noalias !593
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i42), !noalias !593
  store ptr %_11.i, ptr %args.i42, align 8, !noalias !593
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i, align 8, !noalias !593
; invoke core::fmt::write
  %66 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i42)
          to label %.noexc57 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc57:                                         ; preds = %.noexc56
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i42), !noalias !593
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i), !noalias !593
  br i1 %66, label %_0.i56.i.i.noexc.i, label %bb23.i

bb23.i:                                           ; preds = %.noexc57
  %_6.i.i.i14.i = icmp eq ptr %char_indices.sroa.0.0.i, %_28.i
  br i1 %_6.i.i.i14.i, label %_0.i56.i.i.noexc.i.thread, label %bb14.i.i15.i

bb14.i.i15.i:                                     ; preds = %bb23.i
  %x.i.i17.i = load i8, ptr %char_indices.sroa.0.0.i, align 1, !alias.scope !584, !noalias !594, !noundef !2
  %_6.i.i18.i = icmp sgt i8 %x.i.i17.i, -1
  br i1 %_6.i.i18.i, label %bb3.i.i56.i, label %bb4.i.i19.i

bb4.i.i19.i:                                      ; preds = %bb14.i.i15.i
  %_16.i.i.i16.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 1
  %_30.i.i20.i = and i8 %x.i.i17.i, 31
  %init.i.i21.i = zext nneg i8 %_30.i.i20.i to i32
  %_6.i10.i.i22.i = icmp ne ptr %_16.i.i.i16.i, %_28.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i), !noalias !503
  %y.i.i24.i = load i8, ptr %_16.i.i.i16.i, align 1, !alias.scope !584, !noalias !594, !noundef !2
  %_33.i.i25.i = shl nuw nsw i32 %init.i.i21.i, 6
  %_35.i.i26.i = and i8 %y.i.i24.i, 63
  %_34.i.i27.i = zext nneg i8 %_35.i.i26.i to i32
  %67 = or disjoint i32 %_33.i.i25.i, %_34.i.i27.i
  %_13.i.i28.i = icmp samesign ugt i8 %x.i.i17.i, -33
  br i1 %_13.i.i28.i, label %bb6.i.i36.i, label %bb6.i

bb3.i.i56.i:                                      ; preds = %bb14.i.i15.i
  %_7.i.i57.i = zext nneg i8 %x.i.i17.i to i32
  br label %bb6.i

bb6.i.i36.i:                                      ; preds = %bb4.i.i19.i
  %_16.i12.i.i23.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 2
  %_6.i17.i.i37.i = icmp ne ptr %_16.i12.i.i23.i, %_28.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i), !noalias !503
  %z.i.i39.i = load i8, ptr %_16.i12.i.i23.i, align 1, !alias.scope !584, !noalias !594, !noundef !2
  %_38.i.i40.i = shl nuw nsw i32 %_34.i.i27.i, 6
  %_40.i.i41.i = and i8 %z.i.i39.i, 63
  %_39.i.i42.i = zext nneg i8 %_40.i.i41.i to i32
  %y_z.i.i43.i = or disjoint i32 %_38.i.i40.i, %_39.i.i42.i
  %_20.i.i44.i = shl nuw nsw i32 %init.i.i21.i, 12
  %68 = or disjoint i32 %y_z.i.i43.i, %_20.i.i44.i
  %_21.i.i45.i = icmp samesign ugt i8 %x.i.i17.i, -17
  br i1 %_21.i.i45.i, label %bb8.i.i46.i, label %bb6.i

bb8.i.i46.i:                                      ; preds = %bb6.i.i36.i
  %_16.i19.i.i38.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 3
  %_6.i24.i.i47.i = icmp ne ptr %_16.i19.i.i38.i, %_28.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i), !noalias !503
  %w.i.i49.i = load i8, ptr %_16.i19.i.i38.i, align 1, !alias.scope !584, !noalias !594, !noundef !2
  %_26.i.i50.i = shl nuw nsw i32 %init.i.i21.i, 18
  %_25.i.i51.i = and i32 %_26.i.i50.i, 1835008
  %_43.i.i52.i = shl nuw nsw i32 %y_z.i.i43.i, 6
  %_45.i.i53.i = and i8 %w.i.i49.i, 63
  %_44.i.i54.i = zext nneg i8 %_45.i.i53.i to i32
  %_27.i.i55.i = or disjoint i32 %_43.i.i52.i, %_44.i.i54.i
  %69 = or disjoint i32 %_27.i.i55.i, %_25.i.i51.i
  br label %bb6.i

bb6.i:                                            ; preds = %bb8.i.i46.i, %bb6.i.i36.i, %bb3.i.i56.i, %bb4.i.i19.i
  %_0.sroa.4.0.i.ph.i31.i = phi i32 [ %67, %bb4.i.i19.i ], [ %68, %bb6.i.i36.i ], [ %69, %bb8.i.i46.i ], [ %_7.i.i57.i, %bb3.i.i56.i ]
  %70 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i, 1114112
  call void @llvm.assume(i1 %70), !noalias !503
  %_8.not.i.i = icmp ult i64 %63, %new_len.i54.i.i.i
  br i1 %_8.not.i.i, label %bb9.i.i, label %bb6.i.i50

bb6.i.i50:                                        ; preds = %bb6.i
  %71 = icmp eq i64 %63, %new_len.i54.i.i.i
  br i1 %71, label %bb26.i, label %bb25.i

bb9.i.i:                                          ; preds = %bb6.i
  %72 = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 %63
  %self1.i.i = load i8, ptr %72, align 1, !alias.scope !599, !noalias !602, !noundef !2
  %73 = icmp sgt i8 %self1.i.i, -65
  br i1 %73, label %bb26.i, label %bb25.i

bb26.i:                                           ; preds = %bb9.i.i, %bb6.i.i50
  %new_len.i.i = sub nuw i64 %new_len.i54.i.i.i, %63
  %data.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 %63
; invoke heck::lowercase
  %_19.i58 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i, i64 noundef %new_len.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_19.i.noexc unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

_19.i.noexc:                                      ; preds = %bb26.i
  br i1 %_19.i58, label %_0.i56.i.i.noexc.i, label %_0.i56.i.i.noexc.i.thread

bb25.i:                                           ; preds = %bb9.i.i, %bb6.i.i50
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, i64 noundef %63, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc59 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp

.noexc59:                                         ; preds = %bb25.i
  unreachable

_0.i56.i.i.noexc.i.thread:                        ; preds = %_19.i.noexc, %bb95.i.i.i, %bb23.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb5.i.i.i.outer.backedge

bb5.i.i.i.outer.backedge:                         ; preds = %_0.i56.i.i.noexc.i.thread, %_0.i77.i.i.noexc.thread.i
  %init.sroa.0.0.i.i.i.ph.be = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %_0.i77.i.i.noexc.thread.i ], [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i.thread ]
  br label %bb5.i.i.i.outer

_0.i56.i.i.noexc.i:                               ; preds = %.noexc57, %_19.i.noexc
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb2.i.i

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !603)
  call void @llvm.experimental.noalias.scope.decl(metadata !606)
  call void @llvm.experimental.noalias.scope.decl(metadata !609)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !612, !noalias !615, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !612, !noalias !615, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !618, !noalias !615
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !618, !noalias !615, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 32, ptr %dst.i.i.i149.i, align 1, !noalias !619
  %74 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %74, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !618, !noalias !615
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %75 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %75, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %76 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %76, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %77 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %77, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %78 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i75.i.i.i = load i8, ptr %78, align 1, !alias.scope !620, !noalias !563, !noundef !2
  %79 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %79, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %80 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %80, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %81 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %81, align 1, !alias.scope !620, !noalias !563, !noundef !2
  %82 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %82, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i70.i.i.i = sub nuw i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !623)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i.i), !noalias !503
  %_28.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %83 = ptrtoint ptr %data.i71.i.i.i to i64
  %_6.i.i.i.i154.i = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i.ph
  br i1 %_6.i.i.i.i154.i, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb76.i.i.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 1
  %x.i.i.i.i = load i8, ptr %data.i71.i.i.i, align 1, !alias.scope !623, !noalias !626, !noundef !2
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 2
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !alias.scope !623, !noalias !626, !noundef !2
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %84 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i156.i, label %bb2.i155.i

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb2.i155.i

bb6.i.i.i156.i:                                   ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 3
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !alias.scope !623, !noalias !626, !noundef !2
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %85 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb2.i155.i

bb8.i.i.i.i:                                      ; preds = %bb6.i.i.i156.i
  %_6.i24.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 4
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !alias.scope !623, !noalias !626, !noundef !2
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %86 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb2.i155.i

bb2.i155.i:                                       ; preds = %bb8.i.i.i.i, %bb6.i.i.i156.i, %bb3.i.i.i.i, %bb4.i.i.i.i
  %char_indices.sroa.0.0.i.i = phi ptr [ %_16.i.i.i.i.i, %bb3.i.i.i.i ], [ %_16.i26.i.i.i.i, %bb8.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i156.i ], [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %_7.i.i.i.i, %bb3.i.i.i.i ], [ %86, %bb8.i.i.i.i ], [ %85, %bb6.i.i.i156.i ], [ %84, %bb4.i.i.i.i ]
  %87 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  call void @llvm.assume(i1 %87)
  %88 = ptrtoint ptr %char_indices.sroa.0.0.i.i to i64
  %89 = sub i64 %88, %83
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i.i), !noalias !632
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i.i, i32 noundef %_0.sroa.4.0.i.ph.i.i.i)
          to label %.noexc157.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, !noalias !503

.noexc157.i:                                      ; preds = %bb2.i155.i
  %_3.i.i.i = load i32, ptr %2, align 8, !range !14, !noalias !632, !noundef !2
  %90 = icmp eq i32 %_3.i.i.i, 0
  %_6.i.i.i = load i32, ptr %3, align 4, !range !14, !noalias !632
  %91 = icmp eq i32 %_6.i.i.i, 0
  %spec.select.i.i.i = select i1 %91, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %90, i64 %spec.select.i.i.i, i64 3
  store i64 0, ptr %_11.i.i, align 8, !noalias !632
  store i64 %iter.sroa.4.0.i.i.i, ptr %_32.sroa.4.0._11.sroa_idx.i.i, align 8, !noalias !632
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i.i, i64 12, i1 false), !noalias !632
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !632
  store ptr %_11.i.i, ptr %args.i.i, align 8, !noalias !632
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i.i, align 8, !noalias !632
; invoke core::fmt::write
  %92 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc158.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, !noalias !503

.noexc158.i:                                      ; preds = %.noexc157.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !632
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i.i), !noalias !632
  br i1 %92, label %_0.i77.i.i.noexc.i, label %bb23.i.i

bb23.i.i:                                         ; preds = %.noexc158.i
  %_6.i.i.i14.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i, %_28.i.i
  br i1 %_6.i.i.i14.i.i, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i15.i.i

bb14.i.i15.i.i:                                   ; preds = %bb23.i.i
  %x.i.i17.i.i = load i8, ptr %char_indices.sroa.0.0.i.i, align 1, !alias.scope !623, !noalias !633, !noundef !2
  %_6.i.i18.i.i = icmp sgt i8 %x.i.i17.i.i, -1
  br i1 %_6.i.i18.i.i, label %bb3.i.i56.i.i, label %bb4.i.i19.i.i

bb4.i.i19.i.i:                                    ; preds = %bb14.i.i15.i.i
  %_16.i.i.i16.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 1
  %_30.i.i20.i.i = and i8 %x.i.i17.i.i, 31
  %init.i.i21.i.i = zext nneg i8 %_30.i.i20.i.i to i32
  %_6.i10.i.i22.i.i = icmp ne ptr %_16.i.i.i16.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i.i)
  %y.i.i24.i.i = load i8, ptr %_16.i.i.i16.i.i, align 1, !alias.scope !623, !noalias !633, !noundef !2
  %_33.i.i25.i.i = shl nuw nsw i32 %init.i.i21.i.i, 6
  %_35.i.i26.i.i = and i8 %y.i.i24.i.i, 63
  %_34.i.i27.i.i = zext nneg i8 %_35.i.i26.i.i to i32
  %93 = or disjoint i32 %_33.i.i25.i.i, %_34.i.i27.i.i
  %_13.i.i28.i.i = icmp samesign ugt i8 %x.i.i17.i.i, -33
  br i1 %_13.i.i28.i.i, label %bb6.i.i36.i.i, label %bb6.i.i

bb3.i.i56.i.i:                                    ; preds = %bb14.i.i15.i.i
  %_7.i.i57.i.i = zext nneg i8 %x.i.i17.i.i to i32
  br label %bb6.i.i

bb6.i.i36.i.i:                                    ; preds = %bb4.i.i19.i.i
  %_16.i12.i.i23.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 2
  %_6.i17.i.i37.i.i = icmp ne ptr %_16.i12.i.i23.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i.i)
  %z.i.i39.i.i = load i8, ptr %_16.i12.i.i23.i.i, align 1, !alias.scope !623, !noalias !633, !noundef !2
  %_38.i.i40.i.i = shl nuw nsw i32 %_34.i.i27.i.i, 6
  %_40.i.i41.i.i = and i8 %z.i.i39.i.i, 63
  %_39.i.i42.i.i = zext nneg i8 %_40.i.i41.i.i to i32
  %y_z.i.i43.i.i = or disjoint i32 %_38.i.i40.i.i, %_39.i.i42.i.i
  %_20.i.i44.i.i = shl nuw nsw i32 %init.i.i21.i.i, 12
  %94 = or disjoint i32 %y_z.i.i43.i.i, %_20.i.i44.i.i
  %_21.i.i45.i.i = icmp samesign ugt i8 %x.i.i17.i.i, -17
  br i1 %_21.i.i45.i.i, label %bb8.i.i46.i.i, label %bb6.i.i

bb8.i.i46.i.i:                                    ; preds = %bb6.i.i36.i.i
  %_16.i19.i.i38.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 3
  %_6.i24.i.i47.i.i = icmp ne ptr %_16.i19.i.i38.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i.i)
  %w.i.i49.i.i = load i8, ptr %_16.i19.i.i38.i.i, align 1, !alias.scope !623, !noalias !633, !noundef !2
  %_26.i.i50.i.i = shl nuw nsw i32 %init.i.i21.i.i, 18
  %_25.i.i51.i.i = and i32 %_26.i.i50.i.i, 1835008
  %_43.i.i52.i.i = shl nuw nsw i32 %y_z.i.i43.i.i, 6
  %_45.i.i53.i.i = and i8 %w.i.i49.i.i, 63
  %_44.i.i54.i.i = zext nneg i8 %_45.i.i53.i.i to i32
  %_27.i.i55.i.i = or disjoint i32 %_43.i.i52.i.i, %_44.i.i54.i.i
  %95 = or disjoint i32 %_27.i.i55.i.i, %_25.i.i51.i.i
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb8.i.i46.i.i, %bb6.i.i36.i.i, %bb3.i.i56.i.i, %bb4.i.i19.i.i
  %_0.sroa.4.0.i.ph.i31.i.i = phi i32 [ %93, %bb4.i.i19.i.i ], [ %94, %bb6.i.i36.i.i ], [ %95, %bb8.i.i46.i.i ], [ %_7.i.i57.i.i, %bb3.i.i56.i.i ]
  %96 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i.i, 1114112
  call void @llvm.assume(i1 %96)
  %_8.not.i.i.i = icmp ult i64 %89, %new_len.i70.i.i.i
  br i1 %_8.not.i.i.i, label %bb9.i.i.i, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb6.i.i
  %97 = icmp eq i64 %89, %new_len.i70.i.i.i
  br i1 %97, label %bb26.i.i, label %bb25.i.i

bb9.i.i.i:                                        ; preds = %bb6.i.i
  %98 = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 %89
  %self1.i.i.i = load i8, ptr %98, align 1, !alias.scope !638, !noalias !641, !noundef !2
  %99 = icmp sgt i8 %self1.i.i.i, -65
  br i1 %99, label %bb26.i.i, label %bb25.i.i

bb26.i.i:                                         ; preds = %bb9.i.i.i, %bb6.i.i.i
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 %89
  call void @llvm.experimental.noalias.scope.decl(metadata !642)
  br label %bb1.i

bb1.i:                                            ; preds = %bb1.i.backedge, %bb26.i.i
  %chars.sroa.17.0.i = phi i32 [ 1114113, %bb26.i.i ], [ %chars.sroa.17.0.i.be, %bb1.i.backedge ]
  %chars.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb26.i.i ], [ %chars.sroa.0.0.i.be, %bb1.i.backedge ]
  switch i32 %chars.sroa.17.0.i, label %bb2.i [
    i32 1114113, label %bb14.i
    i32 1114112, label %_0.i77.i.i.noexc.thread.i
  ]

bb14.i:                                           ; preds = %bb1.i
  %_6.i.i.i2 = icmp eq ptr %chars.sroa.0.0.i, %_28.i.i
  br i1 %_6.i.i.i2, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i

bb14.i.i:                                         ; preds = %bb14.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 1
  %x.i.i = load i8, ptr %chars.sroa.0.0.i, align 1, !alias.scope !642, !noalias !645, !noundef !2
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i), !noalias !503
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 2
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !alias.scope !642, !noalias !645, !noundef !2
  %_33.i.i3 = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %100 = or disjoint i32 %_33.i.i3, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i39, label %bb18.i

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb18.i

bb6.i.i39:                                        ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i), !noalias !503
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 3
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !alias.scope !642, !noalias !645, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %101 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb18.i

bb8.i.i:                                          ; preds = %bb6.i.i39
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i), !noalias !503
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 4
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !alias.scope !642, !noalias !645, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %102 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb18.i

bb18.i:                                           ; preds = %bb8.i.i, %bb6.i.i39, %bb3.i.i, %bb4.i.i
  %chars.sroa.0.4.ph.i = phi ptr [ %_16.i12.i.i, %bb4.i.i ], [ %_16.i19.i.i, %bb6.i.i39 ], [ %_16.i26.i.i, %bb8.i.i ], [ %_16.i.i.i, %bb3.i.i ]
  %_0.sroa.4.0.i.ph.i = phi i32 [ %100, %bb4.i.i ], [ %101, %bb6.i.i39 ], [ %102, %bb8.i.i ], [ %_7.i.i, %bb3.i.i ]
  %103 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  call void @llvm.assume(i1 %103), !noalias !503
  br label %bb2.i

bb2.i:                                            ; preds = %bb18.i, %bb1.i
  %chars.sroa.0.1.i = phi ptr [ %chars.sroa.0.0.i, %bb1.i ], [ %chars.sroa.0.4.ph.i, %bb18.i ]
  %_5.sroa.0.0.i = phi i32 [ %chars.sroa.17.0.i, %bb1.i ], [ %_0.sroa.4.0.i.ph.i, %bb18.i ]
  %104 = icmp eq i32 %_5.sroa.0.0.i, 931
  br i1 %104, label %bb1.i.i, label %bb5.i

bb1.i.i:                                          ; preds = %bb2.i
  %_6.i.i.not.i.i.i.i = icmp eq ptr %chars.sroa.0.1.i, %_28.i.i
  br i1 %_6.i.i.not.i.i.i.i, label %bb47.i, label %bb14.i.i.i.i.i4

bb14.i.i.i.i.i4:                                  ; preds = %bb1.i.i
  %_16.i.i.i.i.i.i5 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 1
  %x.i.i.i.i.i6 = load i8, ptr %chars.sroa.0.1.i, align 1, !alias.scope !642, !noalias !649, !noundef !2
  %_6.i.i.i.i.i7 = icmp sgt i8 %x.i.i.i.i.i6, -1
  br i1 %_6.i.i.i.i.i7, label %bb3.i.i.i.i.i37, label %bb4.i.i.i.i.i8

bb4.i.i.i.i.i8:                                   ; preds = %bb14.i.i.i.i.i4
  %_30.i.i.i.i.i9 = and i8 %x.i.i.i.i.i6, 31
  %init.i.i.i.i.i10 = zext nneg i8 %_30.i.i.i.i.i9 to i32
  %_6.i10.i.i.i.i.i11 = icmp ne ptr %_16.i.i.i.i.i.i5, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i11), !noalias !503
  %_16.i12.i.i.i.i.i12 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 2
  %y.i.i.i.i.i13 = load i8, ptr %_16.i.i.i.i.i.i5, align 1, !alias.scope !642, !noalias !649, !noundef !2
  %_33.i.i.i.i.i14 = shl nuw nsw i32 %init.i.i.i.i.i10, 6
  %_35.i.i.i.i.i15 = and i8 %y.i.i.i.i.i13, 63
  %_34.i.i.i.i.i16 = zext nneg i8 %_35.i.i.i.i.i15 to i32
  %105 = or disjoint i32 %_33.i.i.i.i.i14, %_34.i.i.i.i.i16
  %_13.i.i.i.i.i17 = icmp samesign ugt i8 %x.i.i.i.i.i6, -33
  br i1 %_13.i.i.i.i.i17, label %bb6.i.i.i.i.i18, label %bb5.i

bb3.i.i.i.i.i37:                                  ; preds = %bb14.i.i.i.i.i4
  %_7.i.i.i.i.i38 = zext nneg i8 %x.i.i.i.i.i6 to i32
  br label %bb5.i

bb6.i.i.i.i.i18:                                  ; preds = %bb4.i.i.i.i.i8
  %_6.i17.i.i.i.i.i19 = icmp ne ptr %_16.i12.i.i.i.i.i12, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i19), !noalias !503
  %_16.i19.i.i.i.i.i20 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 3
  %z.i.i.i.i.i21 = load i8, ptr %_16.i12.i.i.i.i.i12, align 1, !alias.scope !642, !noalias !649, !noundef !2
  %_38.i.i.i.i.i22 = shl nuw nsw i32 %_34.i.i.i.i.i16, 6
  %_40.i.i.i.i.i23 = and i8 %z.i.i.i.i.i21, 63
  %_39.i.i.i.i.i24 = zext nneg i8 %_40.i.i.i.i.i23 to i32
  %y_z.i.i.i.i.i25 = or disjoint i32 %_38.i.i.i.i.i22, %_39.i.i.i.i.i24
  %_20.i.i.i.i.i26 = shl nuw nsw i32 %init.i.i.i.i.i10, 12
  %106 = or disjoint i32 %y_z.i.i.i.i.i25, %_20.i.i.i.i.i26
  %_21.i.i.i.i.i27 = icmp samesign ugt i8 %x.i.i.i.i.i6, -17
  br i1 %_21.i.i.i.i.i27, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, label %bb5.i

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i: ; preds = %bb6.i.i.i.i.i18
  %_6.i24.i.i.i.i.i28 = icmp ne ptr %_16.i19.i.i.i.i.i20, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i28), !noalias !503
  %_16.i26.i.i.i.i.i29 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 4
  %w.i.i.i.i.i30 = load i8, ptr %_16.i19.i.i.i.i.i20, align 1, !alias.scope !642, !noalias !649, !noundef !2
  %_26.i.i.i.i.i31 = shl nuw nsw i32 %init.i.i.i.i.i10, 18
  %_25.i.i.i.i.i32 = and i32 %_26.i.i.i.i.i31, 1835008
  %_43.i.i.i.i.i33 = shl nuw nsw i32 %y_z.i.i.i.i.i25, 6
  %_45.i.i.i.i.i34 = and i8 %w.i.i.i.i.i30, 63
  %_44.i.i.i.i.i35 = zext nneg i8 %_45.i.i.i.i.i34 to i32
  %_27.i.i.i.i.i36 = or disjoint i32 %_43.i.i.i.i.i33, %_44.i.i.i.i.i35
  %107 = or disjoint i32 %_27.i.i.i.i.i36, %_25.i.i.i.i.i32
  %.not12.i = icmp eq i32 %107, 1114112
  br i1 %.not12.i, label %bb47.i, label %bb5.i

bb5.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb6.i.i.i.i.i18, %bb3.i.i.i.i.i37, %bb4.i.i.i.i.i8, %bb2.i
  %chars.sroa.17.1.i = phi i32 [ %107, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ 1114113, %bb2.i ], [ %105, %bb4.i.i.i.i.i8 ], [ %106, %bb6.i.i.i.i.i18 ], [ %_7.i.i.i.i.i38, %bb3.i.i.i.i.i37 ]
  %chars.sroa.0.2.i = phi ptr [ %_16.i26.i.i.i.i.i29, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %chars.sroa.0.1.i, %bb2.i ], [ %_16.i12.i.i.i.i.i12, %bb4.i.i.i.i.i8 ], [ %_16.i19.i.i.i.i.i20, %bb6.i.i.i.i.i18 ], [ %_16.i.i.i.i.i.i5, %bb3.i.i.i.i.i37 ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i), !noalias !659
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63.i), !noalias !659
; invoke core::unicode::unicode_data::conversions::to_lower
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63.i, i32 noundef %_5.sroa.0.0.i)
          to label %.noexc unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc:                                           ; preds = %bb5.i
  %_3.i.i = load i32, ptr %6, align 4, !range !14, !alias.scope !660, !noalias !663, !noundef !2
  %108 = icmp eq i32 %_3.i.i, 0
  %_6.i13.i = load i32, ptr %7, align 4, !range !14, !alias.scope !660, !noalias !663
  %109 = icmp eq i32 %_6.i13.i, 0
  %spec.select.i.i = select i1 %109, i64 1, i64 2
  %iter.sroa.4.0.i.i = select i1 %108, i64 %spec.select.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(12) %_63.i, i64 12, i1 false), !noalias !659
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63.i), !noalias !659
  store i64 0, ptr %_13.i, align 8, !noalias !659
  store i64 %iter.sroa.4.0.i.i, ptr %_62.sroa.4.0._13.sroa_idx.i, align 8, !noalias !659
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !659
  store ptr %_13.i, ptr %args.i, align 8, !noalias !659
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx.i, align 8, !noalias !659
; invoke core::fmt::write
  %110 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i)
          to label %.noexc40 unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc40:                                         ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !659
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i), !noalias !659
  br i1 %110, label %_0.i77.i.i.noexc.i, label %bb1.i.backedge

bb47.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb1.i.i
  %chars.sroa.0.528.i = phi ptr [ %_16.i26.i.i.i.i.i29, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %_28.i.i, %bb1.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !665)
  call void @llvm.experimental.noalias.scope.decl(metadata !668)
  call void @llvm.experimental.noalias.scope.decl(metadata !671)
  %len.i.i.i.i275 = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !674, !noalias !677, !noundef !2
  %self2.i.i.i.i276 = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !674, !noalias !677, !noundef !2
  %_9.i.i.i.i277 = sub i64 %self2.i.i.i.i276, %len.i.i.i.i275
  %_7.i.i.i.i278 = icmp ult i64 %_9.i.i.i.i277, 2
  br i1 %_7.i.i.i.i278, label %bb1.i.i.i.i279, label %.noexc41, !prof !142

bb1.i.i.i.i279:                                   ; preds = %bb47.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i275, i64 noundef 2)
          to label %.noexc280 unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc280:                                        ; preds = %bb1.i.i.i.i279
  %len.pre.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !680, !noalias !677
  br label %.noexc41

.noexc41:                                         ; preds = %.noexc280, %bb47.i
  %len.i.i.i = phi i64 [ %len.i.i.i.i275, %bb47.i ], [ %len.pre.i.i.i, %.noexc280 ]
  %_9.i.i.i = icmp sgt i64 %len.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i)
  %_10.i.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !680, !noalias !677, !nonnull !2, !noundef !2
  %dst.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i, i64 %len.i.i.i
  store i16 -32049, ptr %dst.i.i.i, align 1, !noalias !680
  %111 = add nuw i64 %len.i.i.i, 2
  store i64 %111, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !680, !noalias !677
  br label %bb1.i.backedge

bb1.i.backedge:                                   ; preds = %.noexc41, %.noexc40
  %chars.sroa.17.0.i.be = phi i32 [ 1114112, %.noexc41 ], [ %chars.sroa.17.1.i, %.noexc40 ]
  %chars.sroa.0.0.i.be = phi ptr [ %chars.sroa.0.528.i, %.noexc41 ], [ %chars.sroa.0.2.i, %.noexc40 ]
  br label %bb1.i

bb25.i.i:                                         ; preds = %bb9.i.i.i, %bb6.i.i.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i71.i.i.i, i64 noundef %new_len.i70.i.i.i, i64 noundef %89, i64 noundef %new_len.i70.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc160.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !503

.noexc160.i:                                      ; preds = %bb25.i.i
  unreachable

_0.i77.i.i.noexc.thread.i:                        ; preds = %bb1.i, %bb14.i, %bb23.i.i, %bb76.i.i.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i.i), !noalias !503
  br label %bb5.i.i.i.outer.backedge

_0.i77.i.i.noexc.i:                               ; preds = %.noexc158.i, %.noexc40
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i.i), !noalias !503
  br label %bb2.i.i

cleanup.loopexit.loopexit.i.loopexit:             ; preds = %bb1.i.i.i.i279, %bb5.i, %.noexc
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit: ; preds = %bb57.i.i.i, %bb63.i.i.i, %bb69.i.i.i, %bb82.i.i.i, %bb88.i.i.i
  %lpad.loopexit563 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp: ; preds = %bb2.i155.i, %.noexc157.i, %bb2.i45, %.noexc56, %bb26.i, %bb1.i.i.i.i150.i, %bb1.i.i.i.i138.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp: ; preds = %bb25.i
  %lpad.loopexit.split-lp63 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i:             ; preds = %bb25.i.i
  %lpad.loopexit.split-lp163.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5titleINtB1A_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, %cleanup.loopexit.loopexit.i.loopexit, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.split-lp.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit.split-lp163.i, %cleanup.loopexit.loopexit.split-lp.i ], [ %lpad.loopexit, %cleanup.loopexit.loopexit.i.loopexit ], [ %lpad.loopexit.split-lp63, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp ], [ %lpad.loopexit563, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !681)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !681, !noalias !503
  %112 = icmp eq i64 %_1.val.i.i, 0
  br i1 %112, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !681, !noalias !503, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !684
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %_0.i77.i.i.noexc.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !503
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !503

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5title11AsTitleCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !503
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !503
  ret void
}

; <str as heck::train::ToTrainCase>::to_train_case
; Function Attrs: uwtable
define void @_RNvXNtCs2f1hMIlW3Va_4heck5traineNtB2_11ToTrainCase13to_train_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_33.i = alloca [12 x i8], align 8
  %args.i42 = alloca [16 x i8], align 8
  %_11.i = alloca [32 x i8], align 8
  %_63.i = alloca [12 x i8], align 4
  %args.i = alloca [16 x i8], align 8
  %_13.i = alloca [32 x i8], align 8
  %_33.i.i = alloca [12 x i8], align 8
  %args.i.i = alloca [16 x i8], align 8
  %_11.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !685
  store i64 0, ptr %buf.i, align 8, !noalias !685
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !685
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !685
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !685
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !685
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !685
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !685
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !685
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !685
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_33.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_33.i.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_11.i.i, i64 16
  %_13.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  %4 = getelementptr inbounds nuw i8, ptr %_33.i, i64 8
  %5 = getelementptr inbounds nuw i8, ptr %_33.i, i64 4
  %_32.sroa.4.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 8
  %_32.sroa.5.0._11.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_11.i, i64 16
  %_13.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i42, i64 8
  %6 = getelementptr inbounds nuw i8, ptr %_63.i, i64 8
  %7 = getelementptr inbounds nuw i8, ptr %_63.i, i64 4
  %_62.sroa.4.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 8
  %_62.sroa.5.0._13.sroa_idx.i = getelementptr inbounds nuw i8, ptr %_13.i, i64 16
  %_15.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %args.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %8 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %9 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %10 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %11 = add i64 %8, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %12 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !691, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %13 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %13, %bb8.i.i.i.i.i.i.i.i.i ], [ %12, %bb6.i.i.i.i.i.i.i.i.i ], [ %10, %bb4.i.i.i.i.i.i.i.i.i ]
  %14 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %14)
  %15 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %15, %9
  %16 = add i64 %_10.i.i.i.i.i.i.i.i, %8
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %11, %bb3.thread.i.i.i.i.i.i.i ], [ %16, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %17 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %17, 10
  %18 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %19 = add nsw i32 %18, -65
  %20 = icmp ult i32 %19, 26
  %21 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %20
  br i1 %21, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !685

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5trainINtB1A_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5trainINtB1A_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %22 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !685

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5trainINtB1A_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %22, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %16, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %16, %.noexc.i ], [ %16, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %16, %.noexc.i ], [ %16, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %16, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %8, %.noexc.i ], [ %8, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %8, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %8, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i.outer

bb5.i.i.i.outer:                                  ; preds = %bb5.i.i.i.outer.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i.ph = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.17.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %34, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.22.1.i.i.i.ph = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.25.0.i.i.i.ph = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.outer.backedge ]
  %init.sroa.0.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.ph.be, %bb5.i.i.i.outer.backedge ]
  %first_word.sroa.0.1.off0.i.i.i.ph = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ false, %bb5.i.i.i.outer.backedge ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb5.i.i.i.outer
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %char_indices.sroa.0.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %34, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.1.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ %char_indices.sroa.25.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb5.i.i.i.outer ], [ %next_mode.sroa.0.0.i.i.i, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %23 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !688, !noalias !710, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !710, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %24 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !688, !noalias !710, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %25 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !688, !noalias !710, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %26 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %26, %bb8.i.i.i.i.i ], [ %25, %bb6.i.i.i.i.i ], [ %24, %bb4.i.i.i.i.i ]
  %27 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %23
  %28 = add i64 %_10.i.i.i.i, %27
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %27, %bb50.thread189.i.i.i ], [ %23, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %28, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !688, !noalias !715, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !715, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %29 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !715, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %30 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !688, !noalias !715, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %31 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i.ph, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5train11AsTrainCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %31, %bb8.i.i.i.i.i.i.i ], [ %30, %bb6.i.i.i.i.i.i.i ], [ %29, %bb4.i.i.i.i.i.i.i ]
  %32 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %32)
  %33 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %34 = add i64 %_10.i.i.i.i.i.i, %33
  %35 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %35, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !725)
  call void @llvm.experimental.noalias.scope.decl(metadata !728)
  call void @llvm.experimental.noalias.scope.decl(metadata !731)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !734, !noalias !737, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !734, !noalias !737, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !740, !noalias !737
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !740, !noalias !737, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 45, ptr %dst.i.i.i.i, align 1, !noalias !741
  %36 = add nuw i64 %len.i.i.i.i, 1
  store i64 %36, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !740, !noalias !737
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %37 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %37, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %38 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %38, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %39 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self1.i.i.i.i = load i8, ptr %39, align 1, !alias.scope !742, !noalias !745, !noundef !2
  %40 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %40, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::capitalize
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck10capitalize(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !685

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %41 = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %42 = phi ptr [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i.ph, i64 noundef %41, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %42) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !685

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !685

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %43 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !685

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %44 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %44, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %45 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %45, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !685

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %46 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %46, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !685

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %47 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %47, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, !noalias !685

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  br label %bb5.i.i.i

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !746)
  call void @llvm.experimental.noalias.scope.decl(metadata !749)
  call void @llvm.experimental.noalias.scope.decl(metadata !752)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !755, !noalias !758, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !755, !noalias !758, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !761, !noalias !758
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !761, !noalias !758, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 45, ptr %dst.i.i.i137.i, align 1, !noalias !762
  %48 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %48, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !761, !noalias !758
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %49 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %49, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %50, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %51 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %51, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %52 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i.i.i.i = load i8, ptr %52, align 1, !alias.scope !763, !noalias !745, !noundef !2
  %53 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %53, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %54 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %54, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %55 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %55, align 1, !alias.scope !763, !noalias !745, !noundef !2
  %56 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %56, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !766)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i)
  %_28.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %57 = ptrtoint ptr %data.i55.i.i.i to i64
  %_6.i.i.i.i43 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
  br i1 %_6.i.i.i.i43, label %_0.i56.i.i.noexc.i.thread, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb95.i.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 1
  %x.i.i.i = load i8, ptr %data.i55.i.i.i, align 1, !alias.scope !766, !noalias !769, !noundef !2
  %_6.i.i.i44 = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i44, label %bb3.i.i.i55, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i), !noalias !685
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !766, !noalias !769, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %58 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i52, label %bb2.i45

bb3.i.i.i55:                                      ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb2.i45

bb6.i.i.i52:                                      ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i), !noalias !685
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !766, !noalias !769, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %59 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i.i53, label %bb2.i45

bb8.i.i.i53:                                      ; preds = %bb6.i.i.i52
  %_6.i24.i.i.i = icmp samesign ne i64 %new_len.i54.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i), !noalias !685
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !766, !noalias !769, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i.i54 = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %60 = or disjoint i32 %_27.i.i.i, %_25.i.i.i54
  br label %bb2.i45

bb2.i45:                                          ; preds = %bb8.i.i.i53, %bb6.i.i.i52, %bb3.i.i.i55, %bb4.i.i.i
  %char_indices.sroa.0.0.i = phi ptr [ %_16.i.i.i.i, %bb3.i.i.i55 ], [ %_16.i26.i.i.i, %bb8.i.i.i53 ], [ %_16.i19.i.i.i, %bb6.i.i.i52 ], [ %_16.i12.i.i.i, %bb4.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %_7.i.i.i, %bb3.i.i.i55 ], [ %60, %bb8.i.i.i53 ], [ %59, %bb6.i.i.i52 ], [ %58, %bb4.i.i.i ]
  %61 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %61), !noalias !685
  %62 = ptrtoint ptr %char_indices.sroa.0.0.i to i64
  %63 = sub i64 %62, %57
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i), !noalias !775
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc56 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc56:                                         ; preds = %bb2.i45
  %_3.i.i46 = load i32, ptr %4, align 8, !range !14, !noalias !775, !noundef !2
  %64 = icmp eq i32 %_3.i.i46, 0
  %_6.i.i47 = load i32, ptr %5, align 4, !range !14, !noalias !775
  %65 = icmp eq i32 %_6.i.i47, 0
  %spec.select.i.i48 = select i1 %65, i64 1, i64 2
  %iter.sroa.4.0.i.i49 = select i1 %64, i64 %spec.select.i.i48, i64 3
  store i64 0, ptr %_11.i, align 8, !noalias !775
  store i64 %iter.sroa.4.0.i.i49, ptr %_32.sroa.4.0._11.sroa_idx.i, align 8, !noalias !775
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i, i64 12, i1 false), !noalias !775
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i42), !noalias !775
  store ptr %_11.i, ptr %args.i42, align 8, !noalias !775
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i, align 8, !noalias !775
; invoke core::fmt::write
  %66 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i42)
          to label %.noexc57 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc57:                                         ; preds = %.noexc56
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i42), !noalias !775
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i), !noalias !775
  br i1 %66, label %_0.i56.i.i.noexc.i, label %bb23.i

bb23.i:                                           ; preds = %.noexc57
  %_6.i.i.i14.i = icmp eq ptr %char_indices.sroa.0.0.i, %_28.i
  br i1 %_6.i.i.i14.i, label %_0.i56.i.i.noexc.i.thread, label %bb14.i.i15.i

bb14.i.i15.i:                                     ; preds = %bb23.i
  %x.i.i17.i = load i8, ptr %char_indices.sroa.0.0.i, align 1, !alias.scope !766, !noalias !776, !noundef !2
  %_6.i.i18.i = icmp sgt i8 %x.i.i17.i, -1
  br i1 %_6.i.i18.i, label %bb3.i.i56.i, label %bb4.i.i19.i

bb4.i.i19.i:                                      ; preds = %bb14.i.i15.i
  %_16.i.i.i16.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 1
  %_30.i.i20.i = and i8 %x.i.i17.i, 31
  %init.i.i21.i = zext nneg i8 %_30.i.i20.i to i32
  %_6.i10.i.i22.i = icmp ne ptr %_16.i.i.i16.i, %_28.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i), !noalias !685
  %y.i.i24.i = load i8, ptr %_16.i.i.i16.i, align 1, !alias.scope !766, !noalias !776, !noundef !2
  %_33.i.i25.i = shl nuw nsw i32 %init.i.i21.i, 6
  %_35.i.i26.i = and i8 %y.i.i24.i, 63
  %_34.i.i27.i = zext nneg i8 %_35.i.i26.i to i32
  %67 = or disjoint i32 %_33.i.i25.i, %_34.i.i27.i
  %_13.i.i28.i = icmp samesign ugt i8 %x.i.i17.i, -33
  br i1 %_13.i.i28.i, label %bb6.i.i36.i, label %bb6.i

bb3.i.i56.i:                                      ; preds = %bb14.i.i15.i
  %_7.i.i57.i = zext nneg i8 %x.i.i17.i to i32
  br label %bb6.i

bb6.i.i36.i:                                      ; preds = %bb4.i.i19.i
  %_16.i12.i.i23.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 2
  %_6.i17.i.i37.i = icmp ne ptr %_16.i12.i.i23.i, %_28.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i), !noalias !685
  %z.i.i39.i = load i8, ptr %_16.i12.i.i23.i, align 1, !alias.scope !766, !noalias !776, !noundef !2
  %_38.i.i40.i = shl nuw nsw i32 %_34.i.i27.i, 6
  %_40.i.i41.i = and i8 %z.i.i39.i, 63
  %_39.i.i42.i = zext nneg i8 %_40.i.i41.i to i32
  %y_z.i.i43.i = or disjoint i32 %_38.i.i40.i, %_39.i.i42.i
  %_20.i.i44.i = shl nuw nsw i32 %init.i.i21.i, 12
  %68 = or disjoint i32 %y_z.i.i43.i, %_20.i.i44.i
  %_21.i.i45.i = icmp samesign ugt i8 %x.i.i17.i, -17
  br i1 %_21.i.i45.i, label %bb8.i.i46.i, label %bb6.i

bb8.i.i46.i:                                      ; preds = %bb6.i.i36.i
  %_16.i19.i.i38.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i, i64 3
  %_6.i24.i.i47.i = icmp ne ptr %_16.i19.i.i38.i, %_28.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i), !noalias !685
  %w.i.i49.i = load i8, ptr %_16.i19.i.i38.i, align 1, !alias.scope !766, !noalias !776, !noundef !2
  %_26.i.i50.i = shl nuw nsw i32 %init.i.i21.i, 18
  %_25.i.i51.i = and i32 %_26.i.i50.i, 1835008
  %_43.i.i52.i = shl nuw nsw i32 %y_z.i.i43.i, 6
  %_45.i.i53.i = and i8 %w.i.i49.i, 63
  %_44.i.i54.i = zext nneg i8 %_45.i.i53.i to i32
  %_27.i.i55.i = or disjoint i32 %_43.i.i52.i, %_44.i.i54.i
  %69 = or disjoint i32 %_27.i.i55.i, %_25.i.i51.i
  br label %bb6.i

bb6.i:                                            ; preds = %bb8.i.i46.i, %bb6.i.i36.i, %bb3.i.i56.i, %bb4.i.i19.i
  %_0.sroa.4.0.i.ph.i31.i = phi i32 [ %67, %bb4.i.i19.i ], [ %68, %bb6.i.i36.i ], [ %69, %bb8.i.i46.i ], [ %_7.i.i57.i, %bb3.i.i56.i ]
  %70 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i, 1114112
  call void @llvm.assume(i1 %70), !noalias !685
  %_8.not.i.i = icmp ult i64 %63, %new_len.i54.i.i.i
  br i1 %_8.not.i.i, label %bb9.i.i, label %bb6.i.i50

bb6.i.i50:                                        ; preds = %bb6.i
  %71 = icmp eq i64 %63, %new_len.i54.i.i.i
  br i1 %71, label %bb26.i, label %bb25.i

bb9.i.i:                                          ; preds = %bb6.i
  %72 = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 %63
  %self1.i.i = load i8, ptr %72, align 1, !alias.scope !781, !noalias !784, !noundef !2
  %73 = icmp sgt i8 %self1.i.i, -65
  br i1 %73, label %bb26.i, label %bb25.i

bb26.i:                                           ; preds = %bb9.i.i, %bb6.i.i50
  %new_len.i.i = sub nuw i64 %new_len.i54.i.i.i, %63
  %data.i.i = getelementptr inbounds nuw i8, ptr %data.i55.i.i.i, i64 %63
; invoke heck::lowercase
  %_19.i58 = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i.i, i64 noundef %new_len.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_19.i.noexc unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

_19.i.noexc:                                      ; preds = %bb26.i
  br i1 %_19.i58, label %_0.i56.i.i.noexc.i, label %_0.i56.i.i.noexc.i.thread

bb25.i:                                           ; preds = %bb9.i.i, %bb6.i.i50
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, i64 noundef %63, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc59 unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp

.noexc59:                                         ; preds = %bb25.i
  unreachable

_0.i56.i.i.noexc.i.thread:                        ; preds = %_19.i.noexc, %bb95.i.i.i, %bb23.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb5.i.i.i.outer.backedge

bb5.i.i.i.outer.backedge:                         ; preds = %_0.i56.i.i.noexc.i.thread, %_0.i77.i.i.noexc.thread.i
  %init.sroa.0.0.i.i.i.ph.be = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %_0.i77.i.i.noexc.thread.i ], [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i.thread ]
  br label %bb5.i.i.i.outer

_0.i56.i.i.noexc.i:                               ; preds = %.noexc57, %_19.i.noexc
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i)
  br label %bb2.i.i

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !785)
  call void @llvm.experimental.noalias.scope.decl(metadata !788)
  call void @llvm.experimental.noalias.scope.decl(metadata !791)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !794, !noalias !797, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !794, !noalias !797, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !800, !noalias !797
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !800, !noalias !797, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 45, ptr %dst.i.i.i149.i, align 1, !noalias !801
  %74 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %74, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !800, !noalias !797
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %75 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %75, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %76 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %76, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %77 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %77, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %78 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i75.i.i.i = load i8, ptr %78, align 1, !alias.scope !802, !noalias !745, !noundef !2
  %79 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %79, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %80 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %80, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %81 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %81, align 1, !alias.scope !802, !noalias !745, !noundef !2
  %82 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %82, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i70.i.i.i = sub nuw i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !805)
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_33.i.i), !noalias !685
  %_28.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %83 = ptrtoint ptr %data.i71.i.i.i to i64
  %_6.i.i.i.i154.i = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i.ph
  br i1 %_6.i.i.i.i154.i, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i.i.i

bb14.i.i.i.i:                                     ; preds = %bb76.i.i.i
  %_16.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 1
  %x.i.i.i.i = load i8, ptr %data.i71.i.i.i, align 1, !alias.scope !805, !noalias !808, !noundef !2
  %_6.i.i.i.i = icmp sgt i8 %x.i.i.i.i, -1
  br i1 %_6.i.i.i.i, label %bb3.i.i.i.i, label %bb4.i.i.i.i

bb4.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_30.i.i.i.i = and i8 %x.i.i.i.i, 31
  %init.i.i.i.i = zext nneg i8 %_30.i.i.i.i to i32
  %_6.i10.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 1
  call void @llvm.assume(i1 %_6.i10.i.i.i.i)
  %_16.i12.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 2
  %y.i.i.i.i = load i8, ptr %_16.i.i.i.i.i, align 1, !alias.scope !805, !noalias !808, !noundef !2
  %_33.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 6
  %_35.i.i.i.i = and i8 %y.i.i.i.i, 63
  %_34.i.i.i.i = zext nneg i8 %_35.i.i.i.i to i32
  %84 = or disjoint i32 %_33.i.i.i.i, %_34.i.i.i.i
  %_13.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -33
  br i1 %_13.i.i.i.i, label %bb6.i.i.i156.i, label %bb2.i155.i

bb3.i.i.i.i:                                      ; preds = %bb14.i.i.i.i
  %_7.i.i.i.i = zext nneg i8 %x.i.i.i.i to i32
  br label %bb2.i155.i

bb6.i.i.i156.i:                                   ; preds = %bb4.i.i.i.i
  %_6.i17.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 2
  call void @llvm.assume(i1 %_6.i17.i.i.i.i)
  %_16.i19.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 3
  %z.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i, align 1, !alias.scope !805, !noalias !808, !noundef !2
  %_38.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i, 6
  %_40.i.i.i.i = and i8 %z.i.i.i.i, 63
  %_39.i.i.i.i = zext nneg i8 %_40.i.i.i.i to i32
  %y_z.i.i.i.i = or disjoint i32 %_38.i.i.i.i, %_39.i.i.i.i
  %_20.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 12
  %85 = or disjoint i32 %y_z.i.i.i.i, %_20.i.i.i.i
  %_21.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i, -17
  br i1 %_21.i.i.i.i, label %bb8.i.i.i.i, label %bb2.i155.i

bb8.i.i.i.i:                                      ; preds = %bb6.i.i.i156.i
  %_6.i24.i.i.i.i = icmp samesign ne i64 %new_len.i70.i.i.i, 3
  call void @llvm.assume(i1 %_6.i24.i.i.i.i)
  %_16.i26.i.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 4
  %w.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i, align 1, !alias.scope !805, !noalias !808, !noundef !2
  %_26.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i, 18
  %_25.i.i.i.i = and i32 %_26.i.i.i.i, 1835008
  %_43.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i, 6
  %_45.i.i.i.i = and i8 %w.i.i.i.i, 63
  %_44.i.i.i.i = zext nneg i8 %_45.i.i.i.i to i32
  %_27.i.i.i.i = or disjoint i32 %_43.i.i.i.i, %_44.i.i.i.i
  %86 = or disjoint i32 %_27.i.i.i.i, %_25.i.i.i.i
  br label %bb2.i155.i

bb2.i155.i:                                       ; preds = %bb8.i.i.i.i, %bb6.i.i.i156.i, %bb3.i.i.i.i, %bb4.i.i.i.i
  %char_indices.sroa.0.0.i.i = phi ptr [ %_16.i.i.i.i.i, %bb3.i.i.i.i ], [ %_16.i26.i.i.i.i, %bb8.i.i.i.i ], [ %_16.i19.i.i.i.i, %bb6.i.i.i156.i ], [ %_16.i12.i.i.i.i, %bb4.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i = phi i32 [ %_7.i.i.i.i, %bb3.i.i.i.i ], [ %86, %bb8.i.i.i.i ], [ %85, %bb6.i.i.i156.i ], [ %84, %bb4.i.i.i.i ]
  %87 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i, 1114112
  call void @llvm.assume(i1 %87)
  %88 = ptrtoint ptr %char_indices.sroa.0.0.i.i to i64
  %89 = sub i64 %88, %83
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_11.i.i), !noalias !814
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_33.i.i, i32 noundef %_0.sroa.4.0.i.ph.i.i.i)
          to label %.noexc157.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, !noalias !685

.noexc157.i:                                      ; preds = %bb2.i155.i
  %_3.i.i.i = load i32, ptr %2, align 8, !range !14, !noalias !814, !noundef !2
  %90 = icmp eq i32 %_3.i.i.i, 0
  %_6.i.i.i = load i32, ptr %3, align 4, !range !14, !noalias !814
  %91 = icmp eq i32 %_6.i.i.i, 0
  %spec.select.i.i.i = select i1 %91, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %90, i64 %spec.select.i.i.i, i64 3
  store i64 0, ptr %_11.i.i, align 8, !noalias !814
  store i64 %iter.sroa.4.0.i.i.i, ptr %_32.sroa.4.0._11.sroa_idx.i.i, align 8, !noalias !814
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_32.sroa.5.0._11.sroa_idx.i.i, ptr noundef nonnull align 8 dereferenceable(12) %_33.i.i, i64 12, i1 false), !noalias !814
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !814
  store ptr %_11.i.i, ptr %args.i.i, align 8, !noalias !814
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_13.sroa.4.0..sroa_idx.i.i, align 8, !noalias !814
; invoke core::fmt::write
  %92 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc158.i unwind label %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, !noalias !685

.noexc158.i:                                      ; preds = %.noexc157.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !814
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_11.i.i), !noalias !814
  br i1 %92, label %_0.i77.i.i.noexc.i, label %bb23.i.i

bb23.i.i:                                         ; preds = %.noexc158.i
  %_6.i.i.i14.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i, %_28.i.i
  br i1 %_6.i.i.i14.i.i, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i15.i.i

bb14.i.i15.i.i:                                   ; preds = %bb23.i.i
  %x.i.i17.i.i = load i8, ptr %char_indices.sroa.0.0.i.i, align 1, !alias.scope !805, !noalias !815, !noundef !2
  %_6.i.i18.i.i = icmp sgt i8 %x.i.i17.i.i, -1
  br i1 %_6.i.i18.i.i, label %bb3.i.i56.i.i, label %bb4.i.i19.i.i

bb4.i.i19.i.i:                                    ; preds = %bb14.i.i15.i.i
  %_16.i.i.i16.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 1
  %_30.i.i20.i.i = and i8 %x.i.i17.i.i, 31
  %init.i.i21.i.i = zext nneg i8 %_30.i.i20.i.i to i32
  %_6.i10.i.i22.i.i = icmp ne ptr %_16.i.i.i16.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i22.i.i)
  %y.i.i24.i.i = load i8, ptr %_16.i.i.i16.i.i, align 1, !alias.scope !805, !noalias !815, !noundef !2
  %_33.i.i25.i.i = shl nuw nsw i32 %init.i.i21.i.i, 6
  %_35.i.i26.i.i = and i8 %y.i.i24.i.i, 63
  %_34.i.i27.i.i = zext nneg i8 %_35.i.i26.i.i to i32
  %93 = or disjoint i32 %_33.i.i25.i.i, %_34.i.i27.i.i
  %_13.i.i28.i.i = icmp samesign ugt i8 %x.i.i17.i.i, -33
  br i1 %_13.i.i28.i.i, label %bb6.i.i36.i.i, label %bb6.i.i

bb3.i.i56.i.i:                                    ; preds = %bb14.i.i15.i.i
  %_7.i.i57.i.i = zext nneg i8 %x.i.i17.i.i to i32
  br label %bb6.i.i

bb6.i.i36.i.i:                                    ; preds = %bb4.i.i19.i.i
  %_16.i12.i.i23.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 2
  %_6.i17.i.i37.i.i = icmp ne ptr %_16.i12.i.i23.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i37.i.i)
  %z.i.i39.i.i = load i8, ptr %_16.i12.i.i23.i.i, align 1, !alias.scope !805, !noalias !815, !noundef !2
  %_38.i.i40.i.i = shl nuw nsw i32 %_34.i.i27.i.i, 6
  %_40.i.i41.i.i = and i8 %z.i.i39.i.i, 63
  %_39.i.i42.i.i = zext nneg i8 %_40.i.i41.i.i to i32
  %y_z.i.i43.i.i = or disjoint i32 %_38.i.i40.i.i, %_39.i.i42.i.i
  %_20.i.i44.i.i = shl nuw nsw i32 %init.i.i21.i.i, 12
  %94 = or disjoint i32 %y_z.i.i43.i.i, %_20.i.i44.i.i
  %_21.i.i45.i.i = icmp samesign ugt i8 %x.i.i17.i.i, -17
  br i1 %_21.i.i45.i.i, label %bb8.i.i46.i.i, label %bb6.i.i

bb8.i.i46.i.i:                                    ; preds = %bb6.i.i36.i.i
  %_16.i19.i.i38.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i, i64 3
  %_6.i24.i.i47.i.i = icmp ne ptr %_16.i19.i.i38.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i47.i.i)
  %w.i.i49.i.i = load i8, ptr %_16.i19.i.i38.i.i, align 1, !alias.scope !805, !noalias !815, !noundef !2
  %_26.i.i50.i.i = shl nuw nsw i32 %init.i.i21.i.i, 18
  %_25.i.i51.i.i = and i32 %_26.i.i50.i.i, 1835008
  %_43.i.i52.i.i = shl nuw nsw i32 %y_z.i.i43.i.i, 6
  %_45.i.i53.i.i = and i8 %w.i.i49.i.i, 63
  %_44.i.i54.i.i = zext nneg i8 %_45.i.i53.i.i to i32
  %_27.i.i55.i.i = or disjoint i32 %_43.i.i52.i.i, %_44.i.i54.i.i
  %95 = or disjoint i32 %_27.i.i55.i.i, %_25.i.i51.i.i
  br label %bb6.i.i

bb6.i.i:                                          ; preds = %bb8.i.i46.i.i, %bb6.i.i36.i.i, %bb3.i.i56.i.i, %bb4.i.i19.i.i
  %_0.sroa.4.0.i.ph.i31.i.i = phi i32 [ %93, %bb4.i.i19.i.i ], [ %94, %bb6.i.i36.i.i ], [ %95, %bb8.i.i46.i.i ], [ %_7.i.i57.i.i, %bb3.i.i56.i.i ]
  %96 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i31.i.i, 1114112
  call void @llvm.assume(i1 %96)
  %_8.not.i.i.i = icmp ult i64 %89, %new_len.i70.i.i.i
  br i1 %_8.not.i.i.i, label %bb9.i.i.i, label %bb6.i.i.i

bb6.i.i.i:                                        ; preds = %bb6.i.i
  %97 = icmp eq i64 %89, %new_len.i70.i.i.i
  br i1 %97, label %bb26.i.i, label %bb25.i.i

bb9.i.i.i:                                        ; preds = %bb6.i.i
  %98 = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 %89
  %self1.i.i.i = load i8, ptr %98, align 1, !alias.scope !820, !noalias !823, !noundef !2
  %99 = icmp sgt i8 %self1.i.i.i, -65
  br i1 %99, label %bb26.i.i, label %bb25.i.i

bb26.i.i:                                         ; preds = %bb9.i.i.i, %bb6.i.i.i
  %data.i.i.i = getelementptr inbounds nuw i8, ptr %data.i71.i.i.i, i64 %89
  call void @llvm.experimental.noalias.scope.decl(metadata !824)
  br label %bb1.i

bb1.i:                                            ; preds = %bb1.i.backedge, %bb26.i.i
  %chars.sroa.17.0.i = phi i32 [ 1114113, %bb26.i.i ], [ %chars.sroa.17.0.i.be, %bb1.i.backedge ]
  %chars.sroa.0.0.i = phi ptr [ %data.i.i.i, %bb26.i.i ], [ %chars.sroa.0.0.i.be, %bb1.i.backedge ]
  switch i32 %chars.sroa.17.0.i, label %bb2.i [
    i32 1114113, label %bb14.i
    i32 1114112, label %_0.i77.i.i.noexc.thread.i
  ]

bb14.i:                                           ; preds = %bb1.i
  %_6.i.i.i2 = icmp eq ptr %chars.sroa.0.0.i, %_28.i.i
  br i1 %_6.i.i.i2, label %_0.i77.i.i.noexc.thread.i, label %bb14.i.i

bb14.i.i:                                         ; preds = %bb14.i
  %_16.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 1
  %x.i.i = load i8, ptr %chars.sroa.0.0.i, align 1, !alias.scope !824, !noalias !827, !noundef !2
  %_6.i.i = icmp sgt i8 %x.i.i, -1
  br i1 %_6.i.i, label %bb3.i.i, label %bb4.i.i

bb4.i.i:                                          ; preds = %bb14.i.i
  %_30.i.i = and i8 %x.i.i, 31
  %init.i.i = zext nneg i8 %_30.i.i to i32
  %_6.i10.i.i = icmp ne ptr %_16.i.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i), !noalias !685
  %_16.i12.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 2
  %y.i.i = load i8, ptr %_16.i.i.i, align 1, !alias.scope !824, !noalias !827, !noundef !2
  %_33.i.i3 = shl nuw nsw i32 %init.i.i, 6
  %_35.i.i = and i8 %y.i.i, 63
  %_34.i.i = zext nneg i8 %_35.i.i to i32
  %100 = or disjoint i32 %_33.i.i3, %_34.i.i
  %_13.i.i = icmp samesign ugt i8 %x.i.i, -33
  br i1 %_13.i.i, label %bb6.i.i39, label %bb18.i

bb3.i.i:                                          ; preds = %bb14.i.i
  %_7.i.i = zext nneg i8 %x.i.i to i32
  br label %bb18.i

bb6.i.i39:                                        ; preds = %bb4.i.i
  %_6.i17.i.i = icmp ne ptr %_16.i12.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i), !noalias !685
  %_16.i19.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 3
  %z.i.i = load i8, ptr %_16.i12.i.i, align 1, !alias.scope !824, !noalias !827, !noundef !2
  %_38.i.i = shl nuw nsw i32 %_34.i.i, 6
  %_40.i.i = and i8 %z.i.i, 63
  %_39.i.i = zext nneg i8 %_40.i.i to i32
  %y_z.i.i = or disjoint i32 %_38.i.i, %_39.i.i
  %_20.i.i = shl nuw nsw i32 %init.i.i, 12
  %101 = or disjoint i32 %y_z.i.i, %_20.i.i
  %_21.i.i = icmp samesign ugt i8 %x.i.i, -17
  br i1 %_21.i.i, label %bb8.i.i, label %bb18.i

bb8.i.i:                                          ; preds = %bb6.i.i39
  %_6.i24.i.i = icmp ne ptr %_16.i19.i.i, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i), !noalias !685
  %_16.i26.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i, i64 4
  %w.i.i = load i8, ptr %_16.i19.i.i, align 1, !alias.scope !824, !noalias !827, !noundef !2
  %_26.i.i = shl nuw nsw i32 %init.i.i, 18
  %_25.i.i = and i32 %_26.i.i, 1835008
  %_43.i.i = shl nuw nsw i32 %y_z.i.i, 6
  %_45.i.i = and i8 %w.i.i, 63
  %_44.i.i = zext nneg i8 %_45.i.i to i32
  %_27.i.i = or disjoint i32 %_43.i.i, %_44.i.i
  %102 = or disjoint i32 %_27.i.i, %_25.i.i
  br label %bb18.i

bb18.i:                                           ; preds = %bb8.i.i, %bb6.i.i39, %bb3.i.i, %bb4.i.i
  %chars.sroa.0.4.ph.i = phi ptr [ %_16.i12.i.i, %bb4.i.i ], [ %_16.i19.i.i, %bb6.i.i39 ], [ %_16.i26.i.i, %bb8.i.i ], [ %_16.i.i.i, %bb3.i.i ]
  %_0.sroa.4.0.i.ph.i = phi i32 [ %100, %bb4.i.i ], [ %101, %bb6.i.i39 ], [ %102, %bb8.i.i ], [ %_7.i.i, %bb3.i.i ]
  %103 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i, 1114112
  call void @llvm.assume(i1 %103), !noalias !685
  br label %bb2.i

bb2.i:                                            ; preds = %bb18.i, %bb1.i
  %chars.sroa.0.1.i = phi ptr [ %chars.sroa.0.0.i, %bb1.i ], [ %chars.sroa.0.4.ph.i, %bb18.i ]
  %_5.sroa.0.0.i = phi i32 [ %chars.sroa.17.0.i, %bb1.i ], [ %_0.sroa.4.0.i.ph.i, %bb18.i ]
  %104 = icmp eq i32 %_5.sroa.0.0.i, 931
  br i1 %104, label %bb1.i.i, label %bb5.i

bb1.i.i:                                          ; preds = %bb2.i
  %_6.i.i.not.i.i.i.i = icmp eq ptr %chars.sroa.0.1.i, %_28.i.i
  br i1 %_6.i.i.not.i.i.i.i, label %bb47.i, label %bb14.i.i.i.i.i4

bb14.i.i.i.i.i4:                                  ; preds = %bb1.i.i
  %_16.i.i.i.i.i.i5 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 1
  %x.i.i.i.i.i6 = load i8, ptr %chars.sroa.0.1.i, align 1, !alias.scope !824, !noalias !831, !noundef !2
  %_6.i.i.i.i.i7 = icmp sgt i8 %x.i.i.i.i.i6, -1
  br i1 %_6.i.i.i.i.i7, label %bb3.i.i.i.i.i37, label %bb4.i.i.i.i.i8

bb4.i.i.i.i.i8:                                   ; preds = %bb14.i.i.i.i.i4
  %_30.i.i.i.i.i9 = and i8 %x.i.i.i.i.i6, 31
  %init.i.i.i.i.i10 = zext nneg i8 %_30.i.i.i.i.i9 to i32
  %_6.i10.i.i.i.i.i11 = icmp ne ptr %_16.i.i.i.i.i.i5, %_28.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i11), !noalias !685
  %_16.i12.i.i.i.i.i12 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 2
  %y.i.i.i.i.i13 = load i8, ptr %_16.i.i.i.i.i.i5, align 1, !alias.scope !824, !noalias !831, !noundef !2
  %_33.i.i.i.i.i14 = shl nuw nsw i32 %init.i.i.i.i.i10, 6
  %_35.i.i.i.i.i15 = and i8 %y.i.i.i.i.i13, 63
  %_34.i.i.i.i.i16 = zext nneg i8 %_35.i.i.i.i.i15 to i32
  %105 = or disjoint i32 %_33.i.i.i.i.i14, %_34.i.i.i.i.i16
  %_13.i.i.i.i.i17 = icmp samesign ugt i8 %x.i.i.i.i.i6, -33
  br i1 %_13.i.i.i.i.i17, label %bb6.i.i.i.i.i18, label %bb5.i

bb3.i.i.i.i.i37:                                  ; preds = %bb14.i.i.i.i.i4
  %_7.i.i.i.i.i38 = zext nneg i8 %x.i.i.i.i.i6 to i32
  br label %bb5.i

bb6.i.i.i.i.i18:                                  ; preds = %bb4.i.i.i.i.i8
  %_6.i17.i.i.i.i.i19 = icmp ne ptr %_16.i12.i.i.i.i.i12, %_28.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i19), !noalias !685
  %_16.i19.i.i.i.i.i20 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 3
  %z.i.i.i.i.i21 = load i8, ptr %_16.i12.i.i.i.i.i12, align 1, !alias.scope !824, !noalias !831, !noundef !2
  %_38.i.i.i.i.i22 = shl nuw nsw i32 %_34.i.i.i.i.i16, 6
  %_40.i.i.i.i.i23 = and i8 %z.i.i.i.i.i21, 63
  %_39.i.i.i.i.i24 = zext nneg i8 %_40.i.i.i.i.i23 to i32
  %y_z.i.i.i.i.i25 = or disjoint i32 %_38.i.i.i.i.i22, %_39.i.i.i.i.i24
  %_20.i.i.i.i.i26 = shl nuw nsw i32 %init.i.i.i.i.i10, 12
  %106 = or disjoint i32 %y_z.i.i.i.i.i25, %_20.i.i.i.i.i26
  %_21.i.i.i.i.i27 = icmp samesign ugt i8 %x.i.i.i.i.i6, -17
  br i1 %_21.i.i.i.i.i27, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, label %bb5.i

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i: ; preds = %bb6.i.i.i.i.i18
  %_6.i24.i.i.i.i.i28 = icmp ne ptr %_16.i19.i.i.i.i.i20, %_28.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i28), !noalias !685
  %_16.i26.i.i.i.i.i29 = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i, i64 4
  %w.i.i.i.i.i30 = load i8, ptr %_16.i19.i.i.i.i.i20, align 1, !alias.scope !824, !noalias !831, !noundef !2
  %_26.i.i.i.i.i31 = shl nuw nsw i32 %init.i.i.i.i.i10, 18
  %_25.i.i.i.i.i32 = and i32 %_26.i.i.i.i.i31, 1835008
  %_43.i.i.i.i.i33 = shl nuw nsw i32 %y_z.i.i.i.i.i25, 6
  %_45.i.i.i.i.i34 = and i8 %w.i.i.i.i.i30, 63
  %_44.i.i.i.i.i35 = zext nneg i8 %_45.i.i.i.i.i34 to i32
  %_27.i.i.i.i.i36 = or disjoint i32 %_43.i.i.i.i.i33, %_44.i.i.i.i.i35
  %107 = or disjoint i32 %_27.i.i.i.i.i36, %_25.i.i.i.i.i32
  %.not12.i = icmp eq i32 %107, 1114112
  br i1 %.not12.i, label %bb47.i, label %bb5.i

bb5.i:                                            ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb6.i.i.i.i.i18, %bb3.i.i.i.i.i37, %bb4.i.i.i.i.i8, %bb2.i
  %chars.sroa.17.1.i = phi i32 [ %107, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ 1114113, %bb2.i ], [ %105, %bb4.i.i.i.i.i8 ], [ %106, %bb6.i.i.i.i.i18 ], [ %_7.i.i.i.i.i38, %bb3.i.i.i.i.i37 ]
  %chars.sroa.0.2.i = phi ptr [ %_16.i26.i.i.i.i.i29, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %chars.sroa.0.1.i, %bb2.i ], [ %_16.i12.i.i.i.i.i12, %bb4.i.i.i.i.i8 ], [ %_16.i19.i.i.i.i.i20, %bb6.i.i.i.i.i18 ], [ %_16.i.i.i.i.i.i5, %bb3.i.i.i.i.i37 ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i), !noalias !841
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63.i), !noalias !841
; invoke core::unicode::unicode_data::conversions::to_lower
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63.i, i32 noundef %_5.sroa.0.0.i)
          to label %.noexc unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc:                                           ; preds = %bb5.i
  %_3.i.i = load i32, ptr %6, align 4, !range !14, !alias.scope !842, !noalias !845, !noundef !2
  %108 = icmp eq i32 %_3.i.i, 0
  %_6.i13.i = load i32, ptr %7, align 4, !range !14, !alias.scope !842, !noalias !845
  %109 = icmp eq i32 %_6.i13.i, 0
  %spec.select.i.i = select i1 %109, i64 1, i64 2
  %iter.sroa.4.0.i.i = select i1 %108, i64 %spec.select.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx.i, ptr noundef nonnull align 4 dereferenceable(12) %_63.i, i64 12, i1 false), !noalias !841
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63.i), !noalias !841
  store i64 0, ptr %_13.i, align 8, !noalias !841
  store i64 %iter.sroa.4.0.i.i, ptr %_62.sroa.4.0._13.sroa_idx.i, align 8, !noalias !841
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i), !noalias !841
  store ptr %_13.i, ptr %args.i, align 8, !noalias !841
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx.i, align 8, !noalias !841
; invoke core::fmt::write
  %110 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i)
          to label %.noexc40 unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc40:                                         ; preds = %.noexc
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i), !noalias !841
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i), !noalias !841
  br i1 %110, label %_0.i77.i.i.noexc.i, label %bb1.i.backedge

bb47.i:                                           ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i, %bb1.i.i
  %chars.sroa.0.528.i = phi ptr [ %_16.i26.i.i.i.i.i29, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i ], [ %_28.i.i, %bb1.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !847)
  call void @llvm.experimental.noalias.scope.decl(metadata !850)
  call void @llvm.experimental.noalias.scope.decl(metadata !853)
  %len.i.i.i.i275 = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !856, !noalias !859, !noundef !2
  %self2.i.i.i.i276 = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !856, !noalias !859, !noundef !2
  %_9.i.i.i.i277 = sub i64 %self2.i.i.i.i276, %len.i.i.i.i275
  %_7.i.i.i.i278 = icmp ult i64 %_9.i.i.i.i277, 2
  br i1 %_7.i.i.i.i278, label %bb1.i.i.i.i279, label %.noexc41, !prof !142

bb1.i.i.i.i279:                                   ; preds = %bb47.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i275, i64 noundef 2)
          to label %.noexc280 unwind label %cleanup.loopexit.loopexit.i.loopexit

.noexc280:                                        ; preds = %bb1.i.i.i.i279
  %len.pre.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !862, !noalias !859
  br label %.noexc41

.noexc41:                                         ; preds = %.noexc280, %bb47.i
  %len.i.i.i = phi i64 [ %len.i.i.i.i275, %bb47.i ], [ %len.pre.i.i.i, %.noexc280 ]
  %_9.i.i.i = icmp sgt i64 %len.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i)
  %_10.i.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !862, !noalias !859, !nonnull !2, !noundef !2
  %dst.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i, i64 %len.i.i.i
  store i16 -32049, ptr %dst.i.i.i, align 1, !noalias !862
  %111 = add nuw i64 %len.i.i.i, 2
  store i64 %111, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !862, !noalias !859
  br label %bb1.i.backedge

bb1.i.backedge:                                   ; preds = %.noexc41, %.noexc40
  %chars.sroa.17.0.i.be = phi i32 [ 1114112, %.noexc41 ], [ %chars.sroa.17.1.i, %.noexc40 ]
  %chars.sroa.0.0.i.be = phi ptr [ %chars.sroa.0.528.i, %.noexc41 ], [ %chars.sroa.0.2.i, %.noexc40 ]
  br label %bb1.i

bb25.i.i:                                         ; preds = %bb9.i.i.i, %bb6.i.i.i
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i71.i.i.i, i64 noundef %new_len.i70.i.i.i, i64 noundef %89, i64 noundef %new_len.i70.i.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_4e2f5c99210b7b3b4346cc992c5fffc9) #19
          to label %.noexc160.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !685

.noexc160.i:                                      ; preds = %bb25.i.i
  unreachable

_0.i77.i.i.noexc.thread.i:                        ; preds = %bb1.i, %bb14.i, %bb23.i.i, %bb76.i.i.i
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i.i), !noalias !685
  br label %bb5.i.i.i.outer.backedge

_0.i77.i.i.noexc.i:                               ; preds = %.noexc158.i, %.noexc40
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_33.i.i), !noalias !685
  br label %bb2.i.i

cleanup.loopexit.loopexit.i.loopexit:             ; preds = %bb1.i.i.i.i279, %bb5.i, %.noexc
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit: ; preds = %bb57.i.i.i, %bb63.i.i.i, %bb69.i.i.i, %bb82.i.i.i, %bb88.i.i.i
  %lpad.loopexit563 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp: ; preds = %bb2.i155.i, %.noexc157.i, %bb2.i45, %.noexc56, %bb26.i, %bb1.i.i.i.i150.i, %bb1.i.i.i.i138.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp: ; preds = %bb25.i
  %lpad.loopexit.split-lp63 = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i:             ; preds = %bb25.i.i
  %lpad.loopexit.split-lp163.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_10capitalizeNCNvXs_NtBJ_5trainINtB1A_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp, %cleanup.loopexit.loopexit.i.loopexit, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.split-lp.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit.split-lp163.i, %cleanup.loopexit.loopexit.split-lp.i ], [ %lpad.loopexit, %cleanup.loopexit.loopexit.i.loopexit ], [ %lpad.loopexit.split-lp63, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.split-lp ], [ %lpad.loopexit563, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.loopexit.i.loopexit.split-lp.loopexit.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !863)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !863, !noalias !685
  %112 = icmp eq i64 %_1.val.i.i, 0
  br i1 %112, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !863, !noalias !685, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !866
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %_0.i77.i.i.noexc.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !685
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !685

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5train11AsTrainCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !685
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !685
  ret void
}

; <core::char::ToLowercase as core::fmt::Display>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt(ptr noalias noundef readonly align 8 captures(none) dereferenceable(32) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 personality ptr @rust_eh_personality {
start:
  %iter.i = alloca [32 x i8], align 8
  tail call void @llvm.experimental.noalias.scope.decl(metadata !867)
  %index.0.i.i.i = load i64, ptr %self, align 8, !alias.scope !870, !noalias !875, !noundef !2
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %index.1.i.i.i = load i64, ptr %0, align 8, !alias.scope !870, !noalias !875, !noundef !2
  %_2.i5.not.i.i.i = icmp eq i64 %index.1.i.i.i, %index.0.i.i.i
  br i1 %_2.i5.not.i.i.i, label %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i.thread, label %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i

_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i.thread: ; preds = %start
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter.i), !noalias !879
  br label %_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt.exit

_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i: ; preds = %start
  %len.i.i.i = sub nuw i64 %index.1.i.i.i, %index.0.i.i.i
  %_0.sroa.0.0.i.i.i.i.i.i = tail call noundef i64 @llvm.umin.i64(i64 range(i64 0, 2305843009213693952) %len.i.i.i, i64 3)
  %slice.0.i.i.i = getelementptr inbounds nuw i8, ptr %self, i64 16
  %_32.i.i.i = getelementptr i32, ptr %slice.0.i.i.i, i64 %index.0.i.i.i
  %1 = shl nuw nsw i64 %_0.sroa.0.0.i.i.i.i.i.i, 2
  %self1.sroa.3.0.iter.sroa_idx.i = getelementptr inbounds nuw i8, ptr %iter.i, i64 16
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %iter.i), !noalias !879
  call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 8 %self1.sroa.3.0.iter.sroa_idx.i, ptr readonly align 4 %_32.i.i.i, i64 %1, i1 false), !noalias !880
  br label %bb3.i

bb3.i:                                            ; preds = %bb3.i, %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i
  %_10.i513.i = phi i64 [ 0, %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i ], [ %_11.i.i, %bb3.i ]
  %self3.i.i = getelementptr inbounds nuw i32, ptr %self1.sroa.3.0.iter.sroa_idx.i, i64 %_10.i513.i
  %_14.i.i = load i32, ptr %self3.i.i, align 4, !range !14, !alias.scope !881, !noalias !879, !noundef !2
; call <core::fmt::Formatter as core::fmt::Write>::write_char
  %self2.i = tail call noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write10write_char(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, i32 noundef %_14.i.i), !noalias !867
  %_11.i.i = add nuw nsw i64 %_10.i513.i, 1
  %_5.not.i.not.i = icmp eq i64 %_0.sroa.0.0.i.i.i.i.i.i, %_11.i.i
  %or.cond = select i1 %self2.i, i1 true, i1 %_5.not.i.not.i
  br i1 %or.cond, label %_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt.exit, label %bb3.i

_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt.exit: ; preds = %bb3.i, %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i.thread
  %_5.not.i10.i = phi i1 [ false, %_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck.exit.i.thread ], [ %self2.i, %bb3.i ]
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %iter.i), !noalias !879
  ret i1 %_5.not.i10.i
}

; <core::fmt::Error as core::fmt::Debug>::fmt
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsK_NtCsjMrxcFdYDNN_4core3fmtNtB5_5ErrorNtB5_5Debug3fmt(ptr noalias nonnull readonly align 1 captures(none) %self, ptr noalias noundef align 8 dereferenceable(24) %f) unnamed_addr #4 {
start:
; call <core::fmt::Formatter>::write_str
  %_0 = tail call noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef nonnull align 8 dereferenceable(24) %f, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_99ac8a81a24cac863217ce4a5cbfabea, i64 noundef 5)
  ret i1 %_0
}

; <alloc::string::String as core::fmt::Write>::write_char
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write10write_char(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, i32 noundef range(i32 0, 1114112) %c) unnamed_addr #4 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !884)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i = load i64, ptr %0, align 8, !alias.scope !884, !noundef !2
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
  %self2.i.i = load i64, ptr %self, align 8, !range !6, !alias.scope !887, !noundef !2
  %_9.i.i = sub nsw i64 %self2.i.i, %len.i
  %_7.i.i = icmp ugt i64 %ch_len.sroa.0.0.i, %_9.i.i
  br i1 %_7.i.i, label %bb1.i.i, label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck.exit.i, !prof !142

bb1.i.i:                                          ; preds = %bb2.i
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i, i64 noundef %ch_len.sroa.0.0.i)
  %count.pre.i = load i64, ptr %0, align 8, !alias.scope !884
  br label %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck.exit.i

_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck.exit.i: ; preds = %bb1.i.i, %bb2.i
  %count.i = phi i64 [ %len.i, %bb2.i ], [ %count.pre.i, %bb1.i.i ]
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_20.i = load ptr, ptr %1, align 8, !alias.scope !884, !nonnull !2, !noundef !2
  %_21.i = icmp sgt i64 %count.i, -1
  tail call void @llvm.assume(i1 %_21.i)
  %_8.i = getelementptr inbounds nuw i8, ptr %_20.i, i64 %count.i
  br i1 %_16.i, label %bb12.i.i, label %bb7.i.i

bb7.i.i:                                          ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck.exit.i
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

bb12.i.i:                                         ; preds = %_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck.exit.i
  %5 = trunc nuw nsw i32 %c to i8
  store i8 %5, ptr %_8.i, align 1, !noalias !884
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb1.i2.i:                                         ; preds = %bb7.i.i
  %6 = or disjoint i8 %3, -64
  store i8 %6, ptr %_8.i, align 1, !noalias !884
  %_20.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last1.i.i, ptr %_20.i.i, align 1, !noalias !884
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb2.i.i:                                          ; preds = %bb7.i.i
  %_28.i.i = icmp samesign ult i32 %c, 65536
  br i1 %_28.i.i, label %bb3.i.i, label %bb4.i.i

bb3.i.i:                                          ; preds = %bb2.i.i
  %7 = or disjoint i8 %4, -32
  store i8 %7, ptr %_8.i, align 1, !noalias !884
  %_21.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last2.i.i, ptr %_21.i.i, align 1, !noalias !884
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last1.i.i, ptr %_22.i.i, align 1, !noalias !884
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

bb4.i.i:                                          ; preds = %bb2.i.i
  store i8 %last4.i.i, ptr %_8.i, align 1, !noalias !884
  %_23.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 1
  store i8 %last3.i.i, ptr %_23.i.i, align 1, !noalias !884
  %_24.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 2
  store i8 %last2.i.i, ptr %_24.i.i, align 1, !noalias !884
  %_25.i.i = getelementptr inbounds nuw i8, ptr %_8.i, i64 3
  store i8 %last1.i.i, ptr %_25.i.i, align 1, !noalias !884
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push.exit: ; preds = %bb12.i.i, %bb1.i2.i, %bb3.i.i, %bb4.i.i
  %new_len.i = add nuw i64 %ch_len.sroa.0.0.i, %len.i
  store i64 %new_len.i, ptr %0, align 8, !alias.scope !884
  ret i1 false
}

; <alloc::string::String as core::fmt::Write>::write_str
; Function Attrs: inlinehint uwtable
define internal noundef zeroext i1 @_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str(ptr noalias noundef align 8 captures(none) dereferenceable(24) %self, ptr noalias noundef nonnull readonly align 1 captures(none) %s.0, i64 noundef %s.1) unnamed_addr #4 {
start:
  tail call void @llvm.experimental.noalias.scope.decl(metadata !890)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !893)
  %0 = getelementptr inbounds nuw i8, ptr %self, i64 16
  %len.i.i.i = load i64, ptr %0, align 8, !alias.scope !896, !noalias !899, !noundef !2
  %self2.i.i.i = load i64, ptr %self, align 8, !range !6, !alias.scope !896, !noalias !899, !noundef !2
  %_9.i.i.i = sub i64 %self2.i.i.i, %len.i.i.i
  %_7.i.i.i = icmp ugt i64 %s.1, %_9.i.i.i
  br i1 %_7.i.i.i, label %bb1.i.i.i, label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit, !prof !142

bb1.i.i.i:                                        ; preds = %start
; call <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  tail call fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %self, i64 noundef %len.i.i.i, i64 noundef %s.1)
  %len.pre.i.i = load i64, ptr %0, align 8, !alias.scope !901, !noalias !899
  br label %_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit

_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str.exit: ; preds = %start, %bb1.i.i.i
  %len.i.i = phi i64 [ %len.i.i.i, %start ], [ %len.pre.i.i, %bb1.i.i.i ]
  %_9.i.i = icmp sgt i64 %len.i.i, -1
  tail call void @llvm.assume(i1 %_9.i.i)
  %1 = getelementptr inbounds nuw i8, ptr %self, i64 8
  %_10.i.i = load ptr, ptr %1, align 8, !alias.scope !901, !noalias !899, !nonnull !2, !noundef !2
  %dst.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 %len.i.i
  tail call void @llvm.memcpy.p0.p0.i64(ptr nonnull align 1 %dst.i.i, ptr nonnull readonly align 1 %s.0, i64 %s.1, i1 false), !noalias !901
  %2 = add i64 %len.i.i, %s.1
  store i64 %2, ptr %0, align 8, !alias.scope !901, !noalias !899
  ret i1 false
}

; <str as heck::shouty_snake::ToShoutySnakeCase>::to_shouty_snake_case
; Function Attrs: uwtable
define void @_RNvXs_NtCs2f1hMIlW3Va_4heck12shouty_snakeeNtB4_17ToShoutySnakeCase20to_shouty_snake_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_29.i.i = alloca [12 x i8], align 4
  %args.i.i = alloca [16 x i8], align 8
  %_10.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !902
  store i64 0, ptr %buf.i, align 8, !noalias !902
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !902
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !902
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !902
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !902
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !902
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !902
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !902
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !902
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_29.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_29.i.i, i64 4
  %_28.sroa.4.0._10.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 8
  %_28.sroa.5.0._10.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i, i64 16
  %_12.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %4 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %5 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %7 = add i64 %4, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %8 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !908, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %9 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %9, %bb8.i.i.i.i.i.i.i.i.i ], [ %8, %bb6.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %10)
  %11 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %11, %5
  %12 = add i64 %_10.i.i.i.i.i.i.i.i, %4
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %7, %bb3.thread.i.i.i.i.i.i.i ], [ %12, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %13 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %13, 10
  %14 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %15 = add nsw i32 %14, -65
  %16 = icmp ult i32 %15, 26
  %17 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %16
  br i1 %17, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !902

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs0_NtBJ_12shouty_snakeINtB1z_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs0_NtBJ_12shouty_snakeINtB1z_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %18 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !902

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs0_NtBJ_12shouty_snakeINtB1z_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %18, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %.noexc.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %12, %.noexc.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ], [ %4, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %4, %.noexc.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %30, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %init.sroa.0.0.i.i.i = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb3.i.i.i ], [ %mode.sroa.0.0.i.i.i.be, %bb5.i.i.i.backedge ]
  %first_word.sroa.0.1.off0.i.i.i = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i.be, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %19 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !905, !noalias !927, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !927, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %20 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !905, !noalias !927, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %21 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !905, !noalias !927, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %22 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %22, %bb8.i.i.i.i.i ], [ %21, %bb6.i.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %23 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %19
  %24 = add i64 %_10.i.i.i.i, %23
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %23, %bb50.thread189.i.i.i ], [ %19, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %24, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !905, !noalias !932, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !932, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !932, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %26 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !905, !noalias !932, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %27 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_snake17AsShoutySnakeCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %27, %bb8.i.i.i.i.i.i.i ], [ %26, %bb6.i.i.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %28)
  %29 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %30 = add i64 %_10.i.i.i.i.i.i, %29
  %31 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %31, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !942)
  call void @llvm.experimental.noalias.scope.decl(metadata !945)
  call void @llvm.experimental.noalias.scope.decl(metadata !948)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !951, !noalias !954, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !951, !noalias !954, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !957, !noalias !954
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !957, !noalias !954, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 95, ptr %dst.i.i.i.i, align 1, !noalias !958
  %32 = add nuw i64 %len.i.i.i.i, 1
  store i64 %32, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !957, !noalias !954
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %33 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %33, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %34 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %34, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %35 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self1.i.i.i.i = load i8, ptr %35, align 1, !alias.scope !959, !noalias !962, !noundef !2
  %36 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %36, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i
; invoke heck::uppercase
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9uppercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !902

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %37 = phi i64 [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %38 = phi ptr [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i, i64 noundef %37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %38) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !902

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %39 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %39, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %40 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %40, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %41 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %41, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %42 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %43 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !963)
  call void @llvm.experimental.noalias.scope.decl(metadata !966)
  call void @llvm.experimental.noalias.scope.decl(metadata !969)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !972, !noalias !975, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !972, !noalias !975, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.split-lp.i

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !978, !noalias !975
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !978, !noalias !975, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 95, ptr %dst.i.i.i137.i, align 1, !noalias !979
  %44 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %44, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !978, !noalias !975
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %45 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %45, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %46 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %46, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %47 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %47, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %48 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i.i.i.i = load i8, ptr %48, align 1, !alias.scope !980, !noalias !962, !noundef !2
  %49 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %49, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %50, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %51 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %51, align 1, !alias.scope !980, !noalias !962, !noundef !2
  %52 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %52, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i
; invoke heck::uppercase
  %_0.i56.i.i12.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9uppercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i56.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i, !noalias !902

_0.i56.i.i.noexc.i:                               ; preds = %bb95.i.i.i
  br i1 %_0.i56.i.i12.i, label %bb2.i.i, label %bb5.i.i.i.backedge, !prof !97

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !983)
  call void @llvm.experimental.noalias.scope.decl(metadata !986)
  call void @llvm.experimental.noalias.scope.decl(metadata !989)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !992, !noalias !995, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !992, !noalias !995, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.split-lp.i

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !998, !noalias !995
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !998, !noalias !995, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 95, ptr %dst.i.i.i149.i, align 1, !noalias !999
  %53 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %53, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !998, !noalias !995
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %54 = icmp eq i64 %init.sroa.0.0.i.i.i, 0
  br i1 %54, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %55 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %55, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %56 = icmp eq i64 %init.sroa.0.0.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %56, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  %self.i75.i.i.i = load i8, ptr %57, align 1, !alias.scope !1000, !noalias !962, !noundef !2
  %58 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %58, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %59 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %59, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %60 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %60, align 1, !alias.scope !1000, !noalias !962, !noundef !2
  %61 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %61, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1003)
  %_19.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %_6.i.i16.not.i.i = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %init.sroa.0.0.i.i.i
  br i1 %_6.i.i16.not.i.i, label %bb5.i.i.i.backedge, label %bb14.i.lr.ph.i.i

bb5.i.i.i.backedge:                               ; preds = %bb19.i.i, %bb76.i.i.i, %_0.i56.i.i.noexc.i, %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  %init.sroa.0.0.i.i.i.be = phi i64 [ %init.sroa.0.0.i.i.i, %bb76.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %init.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ %init.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %init.sroa.0.0.i.i.i, %bb17.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i.i ]
  %mode.sroa.0.0.i.i.i.be = phi i8 [ 0, %bb76.i.i.i ], [ 0, %_0.i56.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %_36.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %bb85.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %_35.i.i.noexc.i ], [ %next_mode.sroa.0.0.i.i.i, %bb79.i.i.i ], [ %next_mode.sroa.0.0.i.i.i, %bb17.i.i.i ], [ 0, %bb19.i.i ]
  %first_word.sroa.0.1.off0.i.i.i.be = phi i1 [ false, %bb76.i.i.i ], [ false, %_0.i56.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %_36.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb85.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i, %_35.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb79.i.i.i ], [ %first_word.sroa.0.1.off0.i.i.i, %bb17.i.i.i ], [ false, %bb19.i.i ]
  br label %bb5.i.i.i

bb14.i.lr.ph.i.i:                                 ; preds = %bb76.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i
  br label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb19.i.i, %bb14.i.lr.ph.i.i
  %iter.sroa.0.017.i.i = phi ptr [ %data.i71.i.i.i, %bb14.i.lr.ph.i.i ], [ %iter.sroa.0.1.ph.i.i, %bb19.i.i ]
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 1
  %x.i.i.i = load i8, ptr %iter.sroa.0.017.i.i, align 1, !alias.scope !1003, !noalias !1006, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i156.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !1003, !noalias !1006, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %62 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb10.i.i

bb3.i.i156.i:                                     ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb10.i.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !1003, !noalias !1006, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %63 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i154.i, label %bb10.i.i

bb8.i.i154.i:                                     ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_19.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %iter.sroa.0.017.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !1003, !noalias !1006, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i155.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %64 = or disjoint i32 %_27.i.i.i, %_25.i.i155.i
  br label %bb10.i.i

bb10.i.i:                                         ; preds = %bb8.i.i154.i, %bb6.i.i.i, %bb3.i.i156.i, %bb4.i.i.i
  %iter.sroa.0.1.ph.i.i = phi ptr [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i154.i ], [ %_16.i.i.i.i, %bb3.i.i156.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %62, %bb4.i.i.i ], [ %63, %bb6.i.i.i ], [ %64, %bb8.i.i154.i ], [ %_7.i.i.i, %bb3.i.i156.i ]
  %65 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %65)
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_10.i.i), !noalias !1010
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_29.i.i), !noalias !1010
; invoke core::unicode::unicode_data::conversions::to_upper
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_29.i.i, i32 noundef %_0.sroa.4.0.i.ph.i.i)
          to label %.noexc157.i unwind label %cleanup.loopexit.loopexit.i, !noalias !902

.noexc157.i:                                      ; preds = %bb10.i.i
  %_3.i.i.i = load i32, ptr %2, align 4, !range !14, !alias.scope !1011, !noalias !1014, !noundef !2
  %66 = icmp eq i32 %_3.i.i.i, 0
  %_6.i8.i.i = load i32, ptr %3, align 4, !range !14, !alias.scope !1011, !noalias !1014
  %67 = icmp eq i32 %_6.i8.i.i, 0
  %spec.select.i.i.i = select i1 %67, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %66, i64 %spec.select.i.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_28.sroa.5.0._10.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(12) %_29.i.i, i64 12, i1 false), !noalias !1010
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_29.i.i), !noalias !1010
  store i64 0, ptr %_10.i.i, align 8, !noalias !1010
  store i64 %iter.sroa.4.0.i.i.i, ptr %_28.sroa.4.0._10.sroa_idx.i.i, align 8, !noalias !1010
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !1010
  store ptr %_10.i.i, ptr %args.i.i, align 8, !noalias !1010
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_12.sroa.4.0..sroa_idx.i.i, align 8, !noalias !1010
; invoke core::fmt::write
  %68 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc158.i unwind label %cleanup.loopexit.loopexit.i, !noalias !902

.noexc158.i:                                      ; preds = %.noexc157.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !1010
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_10.i.i), !noalias !1010
  br i1 %68, label %bb2.i.i, label %bb19.i.i, !prof !97

bb19.i.i:                                         ; preds = %.noexc158.i
  %_6.i.i.not.i.i = icmp eq ptr %iter.sroa.0.1.ph.i.i, %_19.i.i
  br i1 %_6.i.i.not.i.i, label %bb5.i.i.i.backedge, label %bb14.i.i.i

cleanup.loopexit.loopexit.i:                      ; preds = %.noexc157.i, %bb10.i.i
  %lpad.loopexit160.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i:             ; preds = %bb1.i.i.i.i138.i, %bb1.i.i.i.i150.i, %bb95.i.i.i, %bb88.i.i.i, %bb82.i.i.i, %bb69.i.i.i, %bb63.i.i.i, %bb57.i.i.i
  %lpad.loopexit.split-lp161.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9uppercaseNCNvXs0_NtBJ_12shouty_snakeINtB1z_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.split-lp.i, %cleanup.loopexit.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit160.i, %cleanup.loopexit.loopexit.i ], [ %lpad.loopexit.split-lp161.i, %cleanup.loopexit.loopexit.split-lp.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !1016)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !1016, !noalias !902
  %69 = icmp eq i64 %_1.val.i.i, 0
  br i1 %69, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1016, !noalias !902, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !1019
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %.noexc158.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !902
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !902

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_snake17AsShoutySnakeCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !902
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !902
  ret void
}

; <str as heck::snake::ToSnakeCase>::to_snake_case
; Function Attrs: uwtable
define void @_RNvXs_NtCs2f1hMIlW3Va_4heck5snakeeNtB4_11ToSnakeCase13to_snake_case(ptr dead_on_unwind noalias noundef writable writeonly sret([24 x i8]) align 8 captures(none) dereferenceable(24) %_0, ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %self.0, i64 noundef %self.1) unnamed_addr #2 personality ptr @rust_eh_personality {
start:
  %e.i.i = alloca [0 x i8], align 1
  %_63.i.i = alloca [12 x i8], align 4
  %args.i.i = alloca [16 x i8], align 8
  %_13.i.i = alloca [32 x i8], align 8
  %formatter.i = alloca [24 x i8], align 8
  %buf.i = alloca [24 x i8], align 8
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %buf.i), !noalias !1020
  store i64 0, ptr %buf.i, align 8, !noalias !1020
  %_10.sroa.4.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 8
  store ptr inttoptr (i64 1 to ptr), ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !noalias !1020
  %_10.sroa.5.0.buf.sroa_idx.i = getelementptr inbounds nuw i8, ptr %buf.i, i64 16
  store i64 0, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !noalias !1020
  call void @llvm.lifetime.start.p0(i64 24, ptr nonnull %formatter.i), !noalias !1020
  %0 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 16
  store i32 1610612768, ptr %0, align 8, !noalias !1020
  %options.sroa.4.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 20
  store i16 0, ptr %options.sroa.4.0..sroa_idx.i, align 4, !noalias !1020
  %options.sroa.5.0..sroa_idx.i = getelementptr inbounds nuw i8, ptr %formatter.i, i64 22
  store i16 0, ptr %options.sroa.5.0..sroa_idx.i, align 2, !noalias !1020
  store ptr %buf.i, ptr %formatter.i, align 8, !noalias !1020
  %1 = getelementptr inbounds nuw i8, ptr %formatter.i, i64 8
  store ptr @vtable.1, ptr %1, align 8, !noalias !1020
  %_7.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %self.1
  %2 = getelementptr inbounds nuw i8, ptr %_63.i.i, i64 8
  %3 = getelementptr inbounds nuw i8, ptr %_63.i.i, i64 4
  %_62.sroa.4.0._13.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_13.i.i, i64 8
  %_62.sroa.5.0._13.sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %_13.i.i, i64 16
  %_15.sroa.4.0..sroa_idx.i.i = getelementptr inbounds nuw i8, ptr %args.i.i, i64 8
  br label %bb2.i.i.i.i

bb2.i.i.i.i:                                      ; preds = %bb35.i.i.i, %start
  %first_word.sroa.0.0.off0153.i.i.i = phi i1 [ true, %start ], [ %first_word.sroa.0.2.off0.i.i.i, %bb35.i.i.i ]
  %iter.sroa.0.0151.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.0.1.ph.i.i.i, %bb35.i.i.i ]
  %char_indices.sroa.22.0150.i.i.i = phi i64 [ undef, %start ], [ %char_indices.sroa.22.2.i.i.i, %bb35.i.i.i ]
  %iter.sroa.1179.0149.i.i.i = phi ptr [ %self.0, %start ], [ %iter.sroa.1179.6.ph.i.i.i, %bb35.i.i.i ]
  %iter.sroa.17.0148.i.i.i = phi i64 [ 0, %start ], [ %iter.sroa.17.5.ph.i.i.i, %bb35.i.i.i ]
  %_6.i.i.i.i15.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.0149.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i15.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i.i.i:                           ; preds = %bb2.i.i.i.i, %bb5.i.i.i.i.i.i
  %4 = phi i64 [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ]
  %_16.i26.i.i.i1216.i.i.i.i.i.i = phi ptr [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ], [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ]
  %5 = ptrtoint ptr %_16.i26.i.i.i1216.i.i.i.i.i.i to i64
  %_16.i.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 1
  %x.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1026, !noundef !2
  %_6.i.i.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i.i.i, label %bb3.thread.i.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i:                            ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 2
  %y.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1026, !noundef !2
  %_33.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i.i.i to i32
  %6 = or disjoint i32 %_33.i.i.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb3.thread.i.i.i.i.i.i.i:                         ; preds = %bb14.i.i.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i.i.i to i32
  %7 = add i64 %4, 1
  br label %bb1.i.i.i.i.i.i.i.i.i.i

bb6.i.i.i.i.i.i.i.i.i:                            ; preds = %bb4.i.i.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 3
  %z.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1026, !noundef !2
  %_38.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 12
  %8 = or disjoint i32 %y_z.i.i.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i.i.i.i.i

bb8.i.i.i.i.i.i.i.i.i:                            ; preds = %bb6.i.i.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i.i.i, %_7.i.i.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %_16.i26.i.i.i1216.i.i.i.i.i.i, i64 4
  %w.i.i.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1026, !noundef !2
  %_26.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i.i.i
  %9 = or disjoint i32 %_27.i.i.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i.i.i
  br label %bb3.i.i.i.i.i.i.i

bb3.i.i.i.i.i.i.i:                                ; preds = %bb8.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i
  %iter.sroa.1179.1.i.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i = phi i32 [ %9, %bb8.i.i.i.i.i.i.i.i.i ], [ %8, %bb6.i.i.i.i.i.i.i.i.i ], [ %6, %bb4.i.i.i.i.i.i.i.i.i ]
  %10 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %10)
  %11 = ptrtoint ptr %iter.sroa.1179.1.i.i.i to i64
  %_10.i.i.i.i.i.i.i.i = sub i64 %11, %5
  %12 = add i64 %_10.i.i.i.i.i.i.i.i, %4
  %_2.i.i.i.i.i.i.i.i.i.i = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 128
  br i1 %_2.i.i.i.i.i.i.i.i.i.i, label %bb1.i.i.i.i.i.i.i.i.i.i, label %bb2.i.i.i.i.i.i.i.i.i.i

bb2.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i
  %_15.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 169
  br i1 %_15.i.i.i.i.i.i.i.i.i.i, label %bb16.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i

bb1.i.i.i.i.i.i.i.i.i.i:                          ; preds = %bb3.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i
  %iter.sroa.17.4.i.i.i = phi i64 [ %7, %bb3.thread.i.i.i.i.i.i.i ], [ %12, %bb3.i.i.i.i.i.i.i ]
  %_16.i26.i.i.i14.i.i.i.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i.i.i, %bb3.thread.i.i.i.i.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i.i ]
  %13 = add nsw i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, -48
  %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i = icmp ult i32 %13, 10
  %14 = and i32 %_0.sroa.4.0.i.ph.i19.i.i.i.i.i.i.i, 95
  %15 = add nsw i32 %14, -65
  %16 = icmp ult i32 %15, 26
  %17 = or i1 %_9.sroa.0.0.off0.i.i.i.i.i.i.i.i.i.i, %16
  br i1 %17, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb16.i.i.i.i.i.i.i.i.i.i:                         ; preds = %bb2.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::alphabetic::lookup_slow
  %_5.i.i.i.i.i.i.i.i.i2.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %_5.i.i.i.i.i.i.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !1020

_5.i.i.i.i.i.i.i.i.i.noexc.i:                     ; preds = %bb16.i.i.i.i.i.i.i.i.i.i
  br i1 %_5.i.i.i.i.i.i.i.i.i2.i, label %bb5.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i.i.i.i:                          ; preds = %_5.i.i.i.i.i.i.i.i.i.noexc.i
  %_17.i.i.i.i.i.i.i.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i, 177
  br i1 %_17.i.i.i.i.i.i.i.i.i.i, label %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs0_NtBJ_5snakeINtB1z_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, label %bb3.i.i.i

_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs0_NtBJ_5snakeINtB1z_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i: ; preds = %bb4.i.i.i.i.i.i.i.i.i.i
; invoke core::unicode::unicode_data::n::lookup_slow
  %18 = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112) %_0.sroa.4.0.i.ph.i.i.i.i.i.i.i.i) #21
          to label %.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.i, !noalias !1020

.noexc.i:                                         ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs0_NtBJ_5snakeINtB1z_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i
  br i1 %18, label %bb5.i.i.i.i.i.i, label %bb3.i.i.i

bb5.i.i.i.i.i.i:                                  ; preds = %.noexc.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i, %bb1.i.i.i.i.i.i.i.i.i.i
  %iter.sroa.17.2.i.i.i = phi i64 [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %12, %.noexc.i ]
  %iter.sroa.1179.3.i.i.i = phi ptr [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %_5.i.i.i.i.i.i.i.i.i.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ]
  %_6.i.i.i.i.i.i.i.i.i.i = icmp eq ptr %iter.sroa.1179.3.i.i.i, %_7.i.i.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i.i.i, label %bb3.i.i.i, label %bb14.i.i.i.i.i.i.i.i.i

bb3.i.i.i:                                        ; preds = %bb5.i.i.i.i.i.i, %.noexc.i, %bb4.i.i.i.i.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i, %bb2.i.i.i.i
  %iter.sroa.22.1.ph.off0.i.i.i = phi i1 [ true, %bb2.i.i.i.i ], [ false, %.noexc.i ], [ false, %bb4.i.i.i.i.i.i.i.i.i.i ], [ false, %bb1.i.i.i.i.i.i.i.i.i.i ], [ false, %bb2.i.i.i.i.i.i.i.i.i.i ], [ true, %bb5.i.i.i.i.i.i ]
  %iter.sroa.17.5.ph.i.i.i = phi i64 [ %iter.sroa.17.0148.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.2.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.1179.6.ph.i.i.i = phi ptr [ %iter.sroa.1179.0149.i.i.i, %bb2.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %.noexc.i ], [ %iter.sroa.1179.1.i.i.i, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %_16.i26.i.i.i14.i.i.i.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.1.i.i.i, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.1179.3.i.i.i, %bb5.i.i.i.i.i.i ]
  %iter.sroa.0.1.ph.i.i.i = phi i64 [ %iter.sroa.0.0151.i.i.i, %bb2.i.i.i.i ], [ %12, %.noexc.i ], [ %12, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.17.4.i.i.i, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %12, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %iter.sroa.0.0151.i.i.i, %bb5.i.i.i.i.i.i ]
  %s.1.pn.i.i.i = phi i64 [ %self.1, %bb2.i.i.i.i ], [ %4, %.noexc.i ], [ %4, %bb4.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb1.i.i.i.i.i.i.i.i.i.i ], [ %4, %bb2.i.i.i.i.i.i.i.i.i.i ], [ %self.1, %bb5.i.i.i.i.i.i ]
  %_0.sroa.0.1.i.ph.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %iter.sroa.0.0151.i.i.i
  %_0.sroa.4.1.i.ph.i.i.i = sub nuw i64 %s.1.pn.i.i.i, %iter.sroa.0.0151.i.i.i
  %_69.i.i.i = getelementptr inbounds nuw i8, ptr %self.0, i64 %s.1.pn.i.i.i
  br label %bb5.i.i.i.outer

bb5.i.i.i.outer:                                  ; preds = %bb5.i.i.i.outer.backedge, %bb3.i.i.i
  %char_indices.sroa.0.0.i.i.i.ph = phi ptr [ %_0.sroa.0.1.i.ph.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.17.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %30, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.22.1.i.i.i.ph = phi i64 [ %char_indices.sroa.22.0150.i.i.i, %bb3.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.outer.backedge ]
  %char_indices.sroa.25.0.i.i.i.ph = phi i32 [ 1114113, %bb3.i.i.i ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.outer.backedge ]
  %init.sroa.0.0.i.i.i.ph = phi i64 [ 0, %bb3.i.i.i ], [ %init.sroa.0.0.i.i.i.ph.be, %bb5.i.i.i.outer.backedge ]
  %first_word.sroa.0.1.off0.i.i.i.ph = phi i1 [ %first_word.sroa.0.0.off0153.i.i.i, %bb3.i.i.i ], [ false, %bb5.i.i.i.outer.backedge ]
  br label %bb5.i.i.i

bb5.i.i.i:                                        ; preds = %bb5.i.i.i.backedge, %bb5.i.i.i.outer
  %char_indices.sroa.0.0.i.i.i = phi ptr [ %char_indices.sroa.0.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.0.4.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.17.0.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %30, %bb5.i.i.i.backedge ]
  %char_indices.sroa.22.1.i.i.i = phi i64 [ %char_indices.sroa.22.1.i.i.i.ph, %bb5.i.i.i.outer ], [ %char_indices.sroa.17.1196.i.i.i, %bb5.i.i.i.backedge ]
  %char_indices.sroa.25.0.i.i.i = phi i32 [ %char_indices.sroa.25.0.i.i.i.ph, %bb5.i.i.i.outer ], [ %_0.sroa.4.0.i.ph.i.i.i.i.i.i, %bb5.i.i.i.backedge ]
  %mode.sroa.0.0.i.i.i = phi i8 [ 0, %bb5.i.i.i.outer ], [ %next_mode.sroa.0.0.i.i.i, %bb5.i.i.i.backedge ]
  %cond.i.i = icmp eq i32 %char_indices.sroa.25.0.i.i.i, 1114113
  %19 = ptrtoint ptr %char_indices.sroa.0.0.i.i.i to i64
  br i1 %cond.i.i, label %bb47.i.i.i, label %bb1.i.i.i.i

bb47.i.i.i:                                       ; preds = %bb5.i.i.i
  %_6.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.0.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i, label %bb35.i.i.i, label %bb14.i.i.i.i.i

bb14.i.i.i.i.i:                                   ; preds = %bb47.i.i.i
  %_16.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 1
  %x.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.0.i.i.i, align 1, !alias.scope !1023, !noalias !1045, !noundef !2
  %_6.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i, label %bb3.i.i.i.i.i, label %bb4.i.i.i.i.i

bb4.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_30.i.i.i.i.i = and i8 %x.i.i.i.i.i, 31
  %init.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i)
  %_16.i12.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 2
  %y.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1045, !noundef !2
  %_33.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 6
  %_35.i.i.i.i.i = and i8 %y.i.i.i.i.i, 63
  %_34.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i to i32
  %20 = or disjoint i32 %_33.i.i.i.i.i, %_34.i.i.i.i.i
  %_13.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i, label %bb6.i.i.i.i.i, label %bb50.thread189.i.i.i

bb3.i.i.i.i.i:                                    ; preds = %bb14.i.i.i.i.i
  %_7.i.i42.i.i.i = zext nneg i8 %x.i.i.i.i.i to i32
  br label %bb50.thread189.i.i.i

bb6.i.i.i.i.i:                                    ; preds = %bb4.i.i.i.i.i
  %_6.i17.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i)
  %_16.i19.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 3
  %z.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1045, !noundef !2
  %_38.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i, 6
  %_40.i.i.i.i.i = and i8 %z.i.i.i.i.i, 63
  %_39.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i to i32
  %y_z.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i, %_39.i.i.i.i.i
  %_20.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 12
  %21 = or disjoint i32 %y_z.i.i.i.i.i, %_20.i.i.i.i.i
  %_21.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i, label %bb8.i.i.i.i.i, label %bb50.thread189.i.i.i

bb8.i.i.i.i.i:                                    ; preds = %bb6.i.i.i.i.i
  %_6.i24.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i)
  %_16.i26.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.0.i.i.i, i64 4
  %w.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1045, !noundef !2
  %_26.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i, 18
  %_25.i.i.i.i.i = and i32 %_26.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i, 6
  %_45.i.i.i.i.i = and i8 %w.i.i.i.i.i, 63
  %_44.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i to i32
  %_27.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i, %_44.i.i.i.i.i
  %22 = or disjoint i32 %_27.i.i.i.i.i, %_25.i.i.i.i.i
  br label %bb50.thread189.i.i.i

bb50.thread189.i.i.i:                             ; preds = %bb8.i.i.i.i.i, %bb6.i.i.i.i.i, %bb3.i.i.i.i.i, %bb4.i.i.i.i.i
  %char_indices.sroa.0.2.i.i.i = phi ptr [ %_16.i.i.i.i.i.i, %bb3.i.i.i.i.i ], [ %_16.i26.i.i.i.i.i, %bb8.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i, %bb6.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i, %bb4.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i = phi i32 [ %_7.i.i42.i.i.i, %bb3.i.i.i.i.i ], [ %22, %bb8.i.i.i.i.i ], [ %21, %bb6.i.i.i.i.i ], [ %20, %bb4.i.i.i.i.i ]
  %23 = ptrtoint ptr %char_indices.sroa.0.2.i.i.i to i64
  %_10.i.i.i.i = sub i64 %char_indices.sroa.17.0.i.i.i, %19
  %24 = add i64 %_10.i.i.i.i, %23
  br label %bb1.i.i.i.i

bb1.i.i.i.i:                                      ; preds = %bb50.thread189.i.i.i, %bb5.i.i.i
  %.pre-phi.i.i = phi i64 [ %23, %bb50.thread189.i.i.i ], [ %19, %bb5.i.i.i ]
  %_15.sroa.8.0198.i.i.i = phi i32 [ %_0.sroa.4.0.i.ph.i.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.25.0.i.i.i, %bb5.i.i.i ]
  %_15.sroa.0.0197.i.i.i = phi i64 [ %char_indices.sroa.17.0.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.22.1.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.17.1196.i.i.i = phi i64 [ %24, %bb50.thread189.i.i.i ], [ %char_indices.sroa.17.0.i.i.i, %bb5.i.i.i ]
  %char_indices.sroa.0.1195.i.i.i = phi ptr [ %char_indices.sroa.0.2.i.i.i, %bb50.thread189.i.i.i ], [ %char_indices.sroa.0.0.i.i.i, %bb5.i.i.i ]
  %_6.i.i.i.i.i.i.i.i = icmp eq ptr %char_indices.sroa.0.1195.i.i.i, %_69.i.i.i
  br i1 %_6.i.i.i.i.i.i.i.i, label %bb52.i.i.i, label %bb14.i.i.i.i.i.i.i

bb14.i.i.i.i.i.i.i:                               ; preds = %bb1.i.i.i.i
  %_16.i.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 1
  %x.i.i.i.i.i.i.i = load i8, ptr %char_indices.sroa.0.1195.i.i.i, align 1, !alias.scope !1023, !noalias !1050, !noundef !2
  %_6.i.i.i.i.i.i.i = icmp sgt i8 %x.i.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i.i.i, label %bb3.i.i.i.i43.i.i.i, label %bb4.i.i.i.i.i.i.i

bb4.i.i.i.i.i.i.i:                                ; preds = %bb14.i.i.i.i.i.i.i
  %_30.i.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 2
  %y.i.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1050, !noundef !2
  %_33.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i.i to i32
  %25 = or disjoint i32 %_33.i.i.i.i.i.i.i, %_34.i.i.i.i.i.i.i
  %_13.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i.i, label %bb53.i.i.i

bb3.i.i.i.i43.i.i.i:                              ; preds = %bb14.i.i.i.i.i.i.i
  %_7.i.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i.i to i32
  br label %bb53.i.i.i

bb6.i.i.i.i.i.i.i:                                ; preds = %bb4.i.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 3
  %z.i.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1050, !noundef !2
  %_38.i.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i.i, %_39.i.i.i.i.i.i.i
  %_20.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 12
  %26 = or disjoint i32 %y_z.i.i.i.i.i.i.i, %_20.i.i.i.i.i.i.i
  %_21.i.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i.i, label %bb8.i.i.i.i.i.i.i, label %bb53.i.i.i

bb8.i.i.i.i.i.i.i:                                ; preds = %bb6.i.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i.i, %_69.i.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %char_indices.sroa.0.1195.i.i.i, i64 4
  %w.i.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i.i, align 1, !alias.scope !1023, !noalias !1050, !noundef !2
  %_26.i.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i.i, %_44.i.i.i.i.i.i.i
  %27 = or disjoint i32 %_27.i.i.i.i.i.i.i, %_25.i.i.i.i.i.i.i
  br label %bb53.i.i.i

bb35.i.i.i:                                       ; preds = %bb47.i.i.i, %_0.i.i.i.noexc.i
  %char_indices.sroa.22.2.i.i.i = phi i64 [ undef, %_0.i.i.i.noexc.i ], [ %char_indices.sroa.22.1.i.i.i, %bb47.i.i.i ]
  %first_word.sroa.0.2.off0.i.i.i = phi i1 [ false, %_0.i.i.i.noexc.i ], [ %first_word.sroa.0.1.off0.i.i.i.ph, %bb47.i.i.i ]
  br i1 %iter.sroa.22.1.ph.off0.i.i.i, label %_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5snake11AsSnakeCaseReENtB5_12SpecToString14spec_to_stringBD_.exit, label %bb2.i.i.i.i

bb53.i.i.i:                                       ; preds = %bb8.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i, %bb4.i.i.i.i.i.i.i
  %char_indices.sroa.0.4.i.i.i = phi ptr [ %_16.i.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %_16.i26.i.i.i.i.i.i.i, %bb8.i.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i.i, %bb6.i.i.i.i.i.i.i ], [ %_16.i12.i.i.i.i.i.i.i, %bb4.i.i.i.i.i.i.i ]
  %_0.sroa.4.0.i.ph.i.i.i.i.i.i = phi i32 [ %_7.i.i.i.i.i.i.i, %bb3.i.i.i.i43.i.i.i ], [ %27, %bb8.i.i.i.i.i.i.i ], [ %26, %bb6.i.i.i.i.i.i.i ], [ %25, %bb4.i.i.i.i.i.i.i ]
  %28 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 1114112
  call void @llvm.assume(i1 %28)
  %29 = ptrtoint ptr %char_indices.sroa.0.4.i.i.i to i64
  %_10.i.i.i.i.i.i = sub i64 %char_indices.sroa.17.1196.i.i.i, %.pre-phi.i.i
  %30 = add i64 %_10.i.i.i.i.i.i, %29
  %31 = add nsw i32 %_15.sroa.8.0198.i.i.i, -97
  %or.cond.i.i.i = icmp ult i32 %31, 26
  br i1 %or.cond.i.i.i, label %bb11.i.i.i, label %bb54.i.i.i

bb52.i.i.i:                                       ; preds = %bb1.i.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb33.i.i.i, label %bb30.i.i.i

bb30.i.i.i:                                       ; preds = %bb52.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1060)
  call void @llvm.experimental.noalias.scope.decl(metadata !1063)
  call void @llvm.experimental.noalias.scope.decl(metadata !1066)
  %len.i.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1069, !noalias !1072, !noundef !2
  %self2.i.i.i.i.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !1069, !noalias !1072, !noundef !2
  %_7.i.i.i.i127.i = icmp eq i64 %self2.i.i.i.i.i, %len.i.i.i.i.i
  br i1 %_7.i.i.i.i127.i, label %bb1.i.i.i.i.i, label %.noexc3.i, !prof !142

bb1.i.i.i.i.i:                                    ; preds = %bb30.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i.i, i64 noundef 1)
          to label %.noexc129.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i

.noexc129.i:                                      ; preds = %bb1.i.i.i.i.i
  %len.pre.i.i.i.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1075, !noalias !1072
  br label %.noexc3.i

.noexc3.i:                                        ; preds = %.noexc129.i, %bb30.i.i.i
  %len.i.i.i.i = phi i64 [ %len.i.i.i.i.i, %bb30.i.i.i ], [ %len.pre.i.i.i.i, %.noexc129.i ]
  %_9.i.i.i.i = icmp sgt i64 %len.i.i.i.i, -1
  call void @llvm.assume(i1 %_9.i.i.i.i)
  %_10.i.i.i128.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1075, !noalias !1072, !nonnull !2, !noundef !2
  %dst.i.i.i.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i128.i, i64 %len.i.i.i.i
  store i8 95, ptr %dst.i.i.i.i, align 1, !noalias !1076
  %32 = add nuw i64 %len.i.i.i.i, 1
  store i64 %32, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1075, !noalias !1072
  br label %bb33.i.i.i

bb33.i.i.i:                                       ; preds = %.noexc3.i, %bb52.i.i.i
  %33 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %33, label %bb102.i.i.i, label %bb5.i.i.i.i

bb5.i.i.i.i:                                      ; preds = %bb33.i.i.i
  %_8.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_8.not.i.i.i.i, label %bb9.i.i.i.i, label %bb6.i.i.i.i

bb6.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %34 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %34, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb9.i.i.i.i:                                      ; preds = %bb5.i.i.i.i
  %35 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self1.i.i.i.i = load i8, ptr %35, align 1, !alias.scope !1077, !noalias !1080, !noundef !2
  %36 = icmp sgt i8 %self1.i.i.i.i, -65
  br i1 %36, label %bb102.i.i.i, label %bb101.i.i.invoke.i

bb102.i.i.i:                                      ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb33.i.i.i
  %data.i47.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i46.i.i.i = sub nuw i64 %_0.sroa.4.1.i.ph.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::lowercase
  %_0.i.i.i4.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i47.i.i.i, i64 noundef %new_len.i46.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i.i.i.noexc.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, !noalias !1020

_0.i.i.i.noexc.i:                                 ; preds = %bb102.i.i.i
  br i1 %_0.i.i.i4.i, label %bb2.i.i, label %bb35.i.i.i, !prof !97

bb101.i.i.invoke.i:                               ; preds = %bb9.i.i.i.i, %bb6.i.i.i.i, %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb15.i.i.i, %bb19.i.i.i.i, %bb16.i.i.i.i, %bb13.i.i.i.i, %bb10.i.i.i.i, %bb24.i.i.i
  %37 = phi i64 [ %char_indices.sroa.17.1196.i.i.i, %bb15.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb10.i61.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb13.i74.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb16.i68.i.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb19.i72.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb24.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb10.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb13.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb16.i.i.i.i ], [ %_15.sroa.0.0197.i.i.i, %bb19.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb6.i.i.i.i ], [ %_0.sroa.4.1.i.ph.i.i.i, %bb9.i.i.i.i ]
  %38 = phi ptr [ @alloc_a50a5070980a36c272121b09121457de, %bb15.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb10.i61.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb13.i74.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb16.i68.i.i.i ], [ @alloc_a50a5070980a36c272121b09121457de, %bb19.i72.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb24.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb10.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb13.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb16.i.i.i.i ], [ @alloc_308f8938e9762d461fc13622ca97483f, %bb19.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb6.i.i.i.i ], [ @alloc_13b4bf5d328eab9b8df3f7027955d33b, %bb9.i.i.i.i ]
; invoke core::str::slice_error_fail
  invoke void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %_0.sroa.0.1.i.ph.i.i.i, i64 noundef %_0.sroa.4.1.i.ph.i.i.i, i64 noundef %init.sroa.0.0.i.i.i.ph, i64 noundef %37, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(24) %38) #18
          to label %bb101.i.i.cont.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !1020

bb101.i.i.cont.i:                                 ; preds = %bb101.i.i.invoke.i
  unreachable

bb54.i.i.i:                                       ; preds = %bb53.i.i.i
  %_84.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_84.i.i.i, label %bb57.i.i.i, label %bb8.i.i.i

bb57.i.i.i:                                       ; preds = %bb54.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_23.i.i6.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_23.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !1020

_23.i.i.noexc.i:                                  ; preds = %bb57.i.i.i
  br i1 %_23.i.i6.i, label %bb11.i.i.i, label %bb63.i.i.i

bb8.i.i.i:                                        ; preds = %bb54.i.i.i
  %39 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond1.i.i.i = icmp ult i32 %39, 26
  br i1 %or.cond1.i.i.i, label %bb17.i.i.i, label %bb10.i.i.i

bb63.i.i.i:                                       ; preds = %_23.i.i.noexc.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_24.i.i7.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_24.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !1020

_24.i.i.noexc.i:                                  ; preds = %bb63.i.i.i
  br i1 %_24.i.i7.i, label %bb17.i.i.i, label %bb10.i.i.i

bb10.i.i.i:                                       ; preds = %_24.i.i.noexc.i, %bb8.i.i.i
  %_25.i.i.i = icmp eq i8 %mode.sroa.0.0.i.i.i, 1
  br i1 %_25.i.i.i, label %bb11.i.i.i, label %bb17.i.i.i

bb17.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb66.i.i.i, %bb10.i.i.i, %_24.i.i.noexc.i, %bb8.i.i.i
  %next_mode.sroa.0.0.i.i.i = phi i8 [ 1, %_26.i.i.noexc.i ], [ 1, %bb66.i.i.i ], [ %mode.sroa.0.0.i.i.i, %bb10.i.i.i ], [ 2, %bb8.i.i.i ], [ 2, %_24.i.i.noexc.i ]
  %40 = icmp eq i8 %mode.sroa.0.0.i.i.i, 2
  br i1 %40, label %bb18.i.i.i, label %bb5.i.i.i.backedge

bb11.i.i.i:                                       ; preds = %bb10.i.i.i, %_23.i.i.noexc.i, %bb53.i.i.i
  %41 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -65
  %or.cond2.i.i.i = icmp ult i32 %41, 26
  br i1 %or.cond2.i.i.i, label %bb12.i.i.i, label %bb66.i.i.i

bb66.i.i.i:                                       ; preds = %bb11.i.i.i
  %_91.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_91.i.i.i, label %bb69.i.i.i, label %bb17.i.i.i

bb69.i.i.i:                                       ; preds = %bb66.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_26.i.i8.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_26.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !1020

_26.i.i.noexc.i:                                  ; preds = %bb69.i.i.i
  br i1 %_26.i.i8.i, label %bb12.i.i.i, label %bb17.i.i.i

bb12.i.i.i:                                       ; preds = %_26.i.i.noexc.i, %bb11.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb15.i.i.i, label %bb13.i.i.i

bb18.i.i.i:                                       ; preds = %bb17.i.i.i
  %42 = add nsw i32 %_15.sroa.8.0198.i.i.i, -65
  %or.cond3.i.i.i = icmp ult i32 %42, 26
  br i1 %or.cond3.i.i.i, label %bb19.i.i.i, label %bb79.i.i.i

bb79.i.i.i:                                       ; preds = %bb18.i.i.i
  %_100.i.i.i = icmp samesign ugt i32 %_15.sroa.8.0198.i.i.i, 127
  br i1 %_100.i.i.i, label %bb82.i.i.i, label %bb5.i.i.i.backedge

bb82.i.i.i:                                       ; preds = %bb79.i.i.i
; invoke core::unicode::unicode_data::uppercase::lookup
  %_35.i.i9.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef %_15.sroa.8.0198.i.i.i)
          to label %_35.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !1020

_35.i.i.noexc.i:                                  ; preds = %bb82.i.i.i
  br i1 %_35.i.i9.i, label %bb19.i.i.i, label %bb5.i.i.i.backedge

bb19.i.i.i:                                       ; preds = %_35.i.i.noexc.i, %bb18.i.i.i
  %43 = add nsw i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, -97
  %or.cond4.i.i.i = icmp ult i32 %43, 26
  br i1 %or.cond4.i.i.i, label %bb20.i.i.i, label %bb85.i.i.i

bb85.i.i.i:                                       ; preds = %bb19.i.i.i
  %_103.i.i.i = icmp samesign ugt i32 %_0.sroa.4.0.i.ph.i.i.i.i.i.i, 127
  br i1 %_103.i.i.i, label %bb88.i.i.i, label %bb5.i.i.i.backedge

bb88.i.i.i:                                       ; preds = %bb85.i.i.i
; invoke core::unicode::unicode_data::lowercase::lookup
  %_36.i.i10.i = invoke noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef %_0.sroa.4.0.i.ph.i.i.i.i.i.i)
          to label %_36.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit, !noalias !1020

_36.i.i.noexc.i:                                  ; preds = %bb88.i.i.i
  br i1 %_36.i.i10.i, label %bb20.i.i.i, label %bb5.i.i.i.backedge

bb5.i.i.i.backedge:                               ; preds = %_36.i.i.noexc.i, %bb85.i.i.i, %_35.i.i.noexc.i, %bb79.i.i.i, %bb17.i.i.i
  br label %bb5.i.i.i

bb20.i.i.i:                                       ; preds = %_36.i.i.noexc.i, %bb19.i.i.i
  br i1 %first_word.sroa.0.1.off0.i.i.i.ph, label %bb24.i.i.i, label %bb21.i.i.i

bb21.i.i.i:                                       ; preds = %bb20.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1081)
  call void @llvm.experimental.noalias.scope.decl(metadata !1084)
  call void @llvm.experimental.noalias.scope.decl(metadata !1087)
  %len.i.i.i.i130.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1090, !noalias !1093, !noundef !2
  %self2.i.i.i.i131.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !1090, !noalias !1093, !noundef !2
  %_7.i.i.i.i133.i = icmp eq i64 %self2.i.i.i.i131.i, %len.i.i.i.i130.i
  br i1 %_7.i.i.i.i133.i, label %bb1.i.i.i.i138.i, label %.noexc11.i, !prof !142

bb1.i.i.i.i138.i:                                 ; preds = %bb21.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i130.i, i64 noundef 1)
          to label %.noexc140.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp

.noexc140.i:                                      ; preds = %bb1.i.i.i.i138.i
  %len.pre.i.i.i139.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1096, !noalias !1093
  br label %.noexc11.i

.noexc11.i:                                       ; preds = %.noexc140.i, %bb21.i.i.i
  %len.i.i.i134.i = phi i64 [ %len.i.i.i.i130.i, %bb21.i.i.i ], [ %len.pre.i.i.i139.i, %.noexc140.i ]
  %_9.i.i.i135.i = icmp sgt i64 %len.i.i.i134.i, -1
  call void @llvm.assume(i1 %_9.i.i.i135.i)
  %_10.i.i.i136.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1096, !noalias !1093, !nonnull !2, !noundef !2
  %dst.i.i.i137.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i136.i, i64 %len.i.i.i134.i
  store i8 95, ptr %dst.i.i.i137.i, align 1, !noalias !1097
  %44 = add nuw i64 %len.i.i.i134.i, 1
  store i64 %44, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1096, !noalias !1093
  br label %bb24.i.i.i

bb24.i.i.i:                                       ; preds = %.noexc11.i, %bb20.i.i.i
  %_3.not.i.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %_15.sroa.0.0197.i.i.i
  br i1 %_3.not.i.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i48.i.i.i

bb1.i48.i.i.i:                                    ; preds = %bb24.i.i.i
  %45 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %45, label %bb2.i53.i.i.i, label %bb9.i49.i.i.i

bb9.i49.i.i.i:                                    ; preds = %bb1.i48.i.i.i
  %_11.not.i.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i.i.i.i, label %bb13.i.i.i.i, label %bb10.i.i.i.i

bb2.i53.i.i.i:                                    ; preds = %bb13.i.i.i.i, %bb10.i.i.i.i, %bb1.i48.i.i.i
  %46 = icmp eq i64 %_15.sroa.0.0197.i.i.i, 0
  br i1 %46, label %bb95.i.i.i, label %bb15.i.i.i.i

bb10.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %47 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %47, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb13.i.i.i.i:                                     ; preds = %bb9.i49.i.i.i
  %48 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i.i.i.i = load i8, ptr %48, align 1, !alias.scope !1098, !noalias !1080, !noundef !2
  %49 = icmp sgt i8 %self.i.i.i.i, -65
  br i1 %49, label %bb2.i53.i.i.i, label %bb101.i.i.invoke.i

bb15.i.i.i.i:                                     ; preds = %bb2.i53.i.i.i
  %_17.not.i.i.i.i = icmp ult i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i.i.i.i, label %bb19.i.i.i.i, label %bb16.i.i.i.i

bb16.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %50 = icmp eq i64 %_15.sroa.0.0197.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %50, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb19.i.i.i.i:                                     ; preds = %bb15.i.i.i.i
  %51 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %_15.sroa.0.0197.i.i.i
  %self2.i.i.i.i = load i8, ptr %51, align 1, !alias.scope !1098, !noalias !1080, !noundef !2
  %52 = icmp sgt i8 %self2.i.i.i.i, -65
  br i1 %52, label %bb95.i.i.i, label %bb101.i.i.invoke.i

bb95.i.i.i:                                       ; preds = %bb19.i.i.i.i, %bb16.i.i.i.i, %bb2.i53.i.i.i
  %data.i55.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %new_len.i54.i.i.i = sub nuw i64 %_15.sroa.0.0197.i.i.i, %init.sroa.0.0.i.i.i.ph
; invoke heck::lowercase
  %_0.i56.i.i12.i = invoke noundef zeroext i1 @_RNvCs2f1hMIlW3Va_4heck9lowercase(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) %data.i55.i.i.i, i64 noundef %new_len.i54.i.i.i, ptr noalias noundef nonnull readonly align 8 dereferenceable(24) %formatter.i)
          to label %_0.i56.i.i.noexc.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp, !noalias !1020

_0.i56.i.i.noexc.i:                               ; preds = %bb95.i.i.i
  br i1 %_0.i56.i.i12.i, label %bb2.i.i, label %bb5.i.i.i.outer.backedge, !prof !97

bb5.i.i.i.outer.backedge:                         ; preds = %bb1.i.i, %bb14.i.i, %_0.i56.i.i.noexc.i
  %init.sroa.0.0.i.i.i.ph.be = phi i64 [ %_15.sroa.0.0197.i.i.i, %_0.i56.i.i.noexc.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb14.i.i ], [ %char_indices.sroa.17.1196.i.i.i, %bb1.i.i ]
  br label %bb5.i.i.i.outer

bb13.i.i.i:                                       ; preds = %bb12.i.i.i
  call void @llvm.experimental.noalias.scope.decl(metadata !1101)
  call void @llvm.experimental.noalias.scope.decl(metadata !1104)
  call void @llvm.experimental.noalias.scope.decl(metadata !1107)
  %len.i.i.i.i142.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1110, !noalias !1113, !noundef !2
  %self2.i.i.i.i143.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !1110, !noalias !1113, !noundef !2
  %_7.i.i.i.i145.i = icmp eq i64 %self2.i.i.i.i143.i, %len.i.i.i.i142.i
  br i1 %_7.i.i.i.i145.i, label %bb1.i.i.i.i150.i, label %.noexc14.i, !prof !142

bb1.i.i.i.i150.i:                                 ; preds = %bb13.i.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i142.i, i64 noundef 1)
          to label %.noexc152.i unwind label %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp

.noexc152.i:                                      ; preds = %bb1.i.i.i.i150.i
  %len.pre.i.i.i151.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1116, !noalias !1113
  br label %.noexc14.i

.noexc14.i:                                       ; preds = %.noexc152.i, %bb13.i.i.i
  %len.i.i.i146.i = phi i64 [ %len.i.i.i.i142.i, %bb13.i.i.i ], [ %len.pre.i.i.i151.i, %.noexc152.i ]
  %_9.i.i.i147.i = icmp sgt i64 %len.i.i.i146.i, -1
  call void @llvm.assume(i1 %_9.i.i.i147.i)
  %_10.i.i.i148.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1116, !noalias !1113, !nonnull !2, !noundef !2
  %dst.i.i.i149.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i148.i, i64 %len.i.i.i146.i
  store i8 95, ptr %dst.i.i.i149.i, align 1, !noalias !1117
  %53 = add nuw i64 %len.i.i.i146.i, 1
  store i64 %53, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1116, !noalias !1113
  br label %bb15.i.i.i

bb15.i.i.i:                                       ; preds = %.noexc14.i, %bb12.i.i.i
  %_3.not.i57.i.i.i = icmp ugt i64 %init.sroa.0.0.i.i.i.ph, %char_indices.sroa.17.1196.i.i.i
  br i1 %_3.not.i57.i.i.i, label %bb101.i.i.invoke.i, label %bb1.i58.i.i.i

bb1.i58.i.i.i:                                    ; preds = %bb15.i.i.i
  %54 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, 0
  br i1 %54, label %bb2.i65.i.i.i, label %bb9.i59.i.i.i

bb9.i59.i.i.i:                                    ; preds = %bb1.i58.i.i.i
  %_11.not.i60.i.i.i = icmp ult i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_11.not.i60.i.i.i, label %bb13.i74.i.i.i, label %bb10.i61.i.i.i

bb2.i65.i.i.i:                                    ; preds = %bb13.i74.i.i.i, %bb10.i61.i.i.i, %bb1.i58.i.i.i
  %55 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, 0
  br i1 %55, label %bb76.i.i.i, label %bb15.i66.i.i.i

bb10.i61.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %56 = icmp eq i64 %init.sroa.0.0.i.i.i.ph, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %56, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb13.i74.i.i.i:                                   ; preds = %bb9.i59.i.i.i
  %57 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  %self.i75.i.i.i = load i8, ptr %57, align 1, !alias.scope !1118, !noalias !1080, !noundef !2
  %58 = icmp sgt i8 %self.i75.i.i.i, -65
  br i1 %58, label %bb2.i65.i.i.i, label %bb101.i.i.invoke.i

bb15.i66.i.i.i:                                   ; preds = %bb2.i65.i.i.i
  %_17.not.i67.i.i.i = icmp ult i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %_17.not.i67.i.i.i, label %bb19.i72.i.i.i, label %bb16.i68.i.i.i

bb16.i68.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %59 = icmp eq i64 %char_indices.sroa.17.1196.i.i.i, %_0.sroa.4.1.i.ph.i.i.i
  br i1 %59, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb19.i72.i.i.i:                                   ; preds = %bb15.i66.i.i.i
  %60 = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  %self2.i73.i.i.i = load i8, ptr %60, align 1, !alias.scope !1118, !noalias !1080, !noundef !2
  %61 = icmp sgt i8 %self2.i73.i.i.i, -65
  br i1 %61, label %bb76.i.i.i, label %bb101.i.i.invoke.i

bb76.i.i.i:                                       ; preds = %bb19.i72.i.i.i, %bb16.i68.i.i.i, %bb2.i65.i.i.i
  %data.i71.i.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %init.sroa.0.0.i.i.i.ph
  call void @llvm.experimental.noalias.scope.decl(metadata !1121)
  %_22.i.i = getelementptr inbounds nuw i8, ptr %_0.sroa.0.1.i.ph.i.i.i, i64 %char_indices.sroa.17.1196.i.i.i
  br label %bb1.i.i

bb1.i.i:                                          ; preds = %bb1.i.i.backedge, %bb76.i.i.i
  %chars.sroa.17.0.i.i = phi i32 [ 1114113, %bb76.i.i.i ], [ %chars.sroa.17.0.i.i.be, %bb1.i.i.backedge ]
  %chars.sroa.0.0.i.i = phi ptr [ %data.i71.i.i.i, %bb76.i.i.i ], [ %chars.sroa.0.0.i.i.be, %bb1.i.i.backedge ]
  switch i32 %chars.sroa.17.0.i.i, label %bb2.i154.i [
    i32 1114113, label %bb14.i.i
    i32 1114112, label %bb5.i.i.i.outer.backedge
  ]

bb14.i.i:                                         ; preds = %bb1.i.i
  %_6.i.i.i.i = icmp eq ptr %chars.sroa.0.0.i.i, %_22.i.i
  br i1 %_6.i.i.i.i, label %bb5.i.i.i.outer.backedge, label %bb14.i.i.i

bb14.i.i.i:                                       ; preds = %bb14.i.i
  %_16.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 1
  %x.i.i.i = load i8, ptr %chars.sroa.0.0.i.i, align 1, !alias.scope !1121, !noalias !1124, !noundef !2
  %_6.i.i.i = icmp sgt i8 %x.i.i.i, -1
  br i1 %_6.i.i.i, label %bb3.i.i158.i, label %bb4.i.i.i

bb4.i.i.i:                                        ; preds = %bb14.i.i.i
  %_30.i.i.i = and i8 %x.i.i.i, 31
  %init.i.i.i = zext nneg i8 %_30.i.i.i to i32
  %_6.i10.i.i.i = icmp ne ptr %_16.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i)
  %_16.i12.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 2
  %y.i.i.i = load i8, ptr %_16.i.i.i.i, align 1, !alias.scope !1121, !noalias !1124, !noundef !2
  %_33.i.i.i = shl nuw nsw i32 %init.i.i.i, 6
  %_35.i.i.i = and i8 %y.i.i.i, 63
  %_34.i.i.i = zext nneg i8 %_35.i.i.i to i32
  %62 = or disjoint i32 %_33.i.i.i, %_34.i.i.i
  %_13.i.i.i = icmp samesign ugt i8 %x.i.i.i, -33
  br i1 %_13.i.i.i, label %bb6.i.i.i, label %bb18.i.i

bb3.i.i158.i:                                     ; preds = %bb14.i.i.i
  %_7.i.i.i = zext nneg i8 %x.i.i.i to i32
  br label %bb18.i.i

bb6.i.i.i:                                        ; preds = %bb4.i.i.i
  %_6.i17.i.i.i = icmp ne ptr %_16.i12.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i)
  %_16.i19.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 3
  %z.i.i.i = load i8, ptr %_16.i12.i.i.i, align 1, !alias.scope !1121, !noalias !1124, !noundef !2
  %_38.i.i.i = shl nuw nsw i32 %_34.i.i.i, 6
  %_40.i.i.i = and i8 %z.i.i.i, 63
  %_39.i.i.i = zext nneg i8 %_40.i.i.i to i32
  %y_z.i.i.i = or disjoint i32 %_38.i.i.i, %_39.i.i.i
  %_20.i.i.i = shl nuw nsw i32 %init.i.i.i, 12
  %63 = or disjoint i32 %y_z.i.i.i, %_20.i.i.i
  %_21.i.i.i = icmp samesign ugt i8 %x.i.i.i, -17
  br i1 %_21.i.i.i, label %bb8.i.i156.i, label %bb18.i.i

bb8.i.i156.i:                                     ; preds = %bb6.i.i.i
  %_6.i24.i.i.i = icmp ne ptr %_16.i19.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i)
  %_16.i26.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.0.i.i, i64 4
  %w.i.i.i = load i8, ptr %_16.i19.i.i.i, align 1, !alias.scope !1121, !noalias !1124, !noundef !2
  %_26.i.i.i = shl nuw nsw i32 %init.i.i.i, 18
  %_25.i.i157.i = and i32 %_26.i.i.i, 1835008
  %_43.i.i.i = shl nuw nsw i32 %y_z.i.i.i, 6
  %_45.i.i.i = and i8 %w.i.i.i, 63
  %_44.i.i.i = zext nneg i8 %_45.i.i.i to i32
  %_27.i.i.i = or disjoint i32 %_43.i.i.i, %_44.i.i.i
  %64 = or disjoint i32 %_27.i.i.i, %_25.i.i157.i
  br label %bb18.i.i

bb18.i.i:                                         ; preds = %bb8.i.i156.i, %bb6.i.i.i, %bb3.i.i158.i, %bb4.i.i.i
  %chars.sroa.0.4.ph.i.i = phi ptr [ %_16.i12.i.i.i, %bb4.i.i.i ], [ %_16.i19.i.i.i, %bb6.i.i.i ], [ %_16.i26.i.i.i, %bb8.i.i156.i ], [ %_16.i.i.i.i, %bb3.i.i158.i ]
  %_0.sroa.4.0.i.ph.i.i = phi i32 [ %62, %bb4.i.i.i ], [ %63, %bb6.i.i.i ], [ %64, %bb8.i.i156.i ], [ %_7.i.i.i, %bb3.i.i158.i ]
  %65 = icmp samesign ult i32 %_0.sroa.4.0.i.ph.i.i, 1114112
  call void @llvm.assume(i1 %65)
  br label %bb2.i154.i

bb2.i154.i:                                       ; preds = %bb18.i.i, %bb1.i.i
  %chars.sroa.0.1.i.i = phi ptr [ %chars.sroa.0.0.i.i, %bb1.i.i ], [ %chars.sroa.0.4.ph.i.i, %bb18.i.i ]
  %_5.sroa.0.0.i.i = phi i32 [ %chars.sroa.17.0.i.i, %bb1.i.i ], [ %_0.sroa.4.0.i.ph.i.i, %bb18.i.i ]
  %66 = icmp eq i32 %_5.sroa.0.0.i.i, 931
  br i1 %66, label %bb1.i.i.i, label %bb5.i.i

bb1.i.i.i:                                        ; preds = %bb2.i154.i
  %_6.i.i.not.i.i.i.i.i = icmp eq ptr %chars.sroa.0.1.i.i, %_22.i.i
  br i1 %_6.i.i.not.i.i.i.i.i, label %bb47.i.i, label %bb14.i.i.i.i.i.i

bb14.i.i.i.i.i.i:                                 ; preds = %bb1.i.i.i
  %_16.i.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 1
  %x.i.i.i.i.i.i = load i8, ptr %chars.sroa.0.1.i.i, align 1, !alias.scope !1121, !noalias !1128, !noundef !2
  %_6.i.i.i.i.i155.i = icmp sgt i8 %x.i.i.i.i.i.i, -1
  br i1 %_6.i.i.i.i.i155.i, label %bb3.i.i.i.i.i.i, label %bb4.i.i.i.i.i.i

bb4.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_30.i.i.i.i.i.i = and i8 %x.i.i.i.i.i.i, 31
  %init.i.i.i.i.i.i = zext nneg i8 %_30.i.i.i.i.i.i to i32
  %_6.i10.i.i.i.i.i.i = icmp ne ptr %_16.i.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i10.i.i.i.i.i.i)
  %_16.i12.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 2
  %y.i.i.i.i.i.i = load i8, ptr %_16.i.i.i.i.i.i.i, align 1, !alias.scope !1121, !noalias !1128, !noundef !2
  %_33.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 6
  %_35.i.i.i.i.i.i = and i8 %y.i.i.i.i.i.i, 63
  %_34.i.i.i.i.i.i = zext nneg i8 %_35.i.i.i.i.i.i to i32
  %67 = or disjoint i32 %_33.i.i.i.i.i.i, %_34.i.i.i.i.i.i
  %_13.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -33
  br i1 %_13.i.i.i.i.i.i, label %bb6.i.i.i.i.i.i, label %bb5.i.i

bb3.i.i.i.i.i.i:                                  ; preds = %bb14.i.i.i.i.i.i
  %_7.i.i.i.i.i.i = zext nneg i8 %x.i.i.i.i.i.i to i32
  br label %bb5.i.i

bb6.i.i.i.i.i.i:                                  ; preds = %bb4.i.i.i.i.i.i
  %_6.i17.i.i.i.i.i.i = icmp ne ptr %_16.i12.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i17.i.i.i.i.i.i)
  %_16.i19.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 3
  %z.i.i.i.i.i.i = load i8, ptr %_16.i12.i.i.i.i.i.i, align 1, !alias.scope !1121, !noalias !1128, !noundef !2
  %_38.i.i.i.i.i.i = shl nuw nsw i32 %_34.i.i.i.i.i.i, 6
  %_40.i.i.i.i.i.i = and i8 %z.i.i.i.i.i.i, 63
  %_39.i.i.i.i.i.i = zext nneg i8 %_40.i.i.i.i.i.i to i32
  %y_z.i.i.i.i.i.i = or disjoint i32 %_38.i.i.i.i.i.i, %_39.i.i.i.i.i.i
  %_20.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 12
  %68 = or disjoint i32 %y_z.i.i.i.i.i.i, %_20.i.i.i.i.i.i
  %_21.i.i.i.i.i.i = icmp samesign ugt i8 %x.i.i.i.i.i.i, -17
  br i1 %_21.i.i.i.i.i.i, label %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, label %bb5.i.i

_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i: ; preds = %bb6.i.i.i.i.i.i
  %_6.i24.i.i.i.i.i.i = icmp ne ptr %_16.i19.i.i.i.i.i.i, %_22.i.i
  call void @llvm.assume(i1 %_6.i24.i.i.i.i.i.i)
  %_16.i26.i.i.i.i.i.i = getelementptr inbounds nuw i8, ptr %chars.sroa.0.1.i.i, i64 4
  %w.i.i.i.i.i.i = load i8, ptr %_16.i19.i.i.i.i.i.i, align 1, !alias.scope !1121, !noalias !1128, !noundef !2
  %_26.i.i.i.i.i.i = shl nuw nsw i32 %init.i.i.i.i.i.i, 18
  %_25.i.i.i.i.i.i = and i32 %_26.i.i.i.i.i.i, 1835008
  %_43.i.i.i.i.i.i = shl nuw nsw i32 %y_z.i.i.i.i.i.i, 6
  %_45.i.i.i.i.i.i = and i8 %w.i.i.i.i.i.i, 63
  %_44.i.i.i.i.i.i = zext nneg i8 %_45.i.i.i.i.i.i to i32
  %_27.i.i.i.i.i.i = or disjoint i32 %_43.i.i.i.i.i.i, %_44.i.i.i.i.i.i
  %69 = or disjoint i32 %_27.i.i.i.i.i.i, %_25.i.i.i.i.i.i
  %.not12.i.i = icmp eq i32 %69, 1114112
  br i1 %.not12.i.i, label %bb47.i.i, label %bb5.i.i

bb5.i.i:                                          ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, %bb6.i.i.i.i.i.i, %bb3.i.i.i.i.i.i, %bb4.i.i.i.i.i.i, %bb2.i154.i
  %chars.sroa.17.1.i.i = phi i32 [ %69, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ 1114113, %bb2.i154.i ], [ %67, %bb4.i.i.i.i.i.i ], [ %68, %bb6.i.i.i.i.i.i ], [ %_7.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  %chars.sroa.0.2.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ %chars.sroa.0.1.i.i, %bb2.i154.i ], [ %_16.i12.i.i.i.i.i.i, %bb4.i.i.i.i.i.i ], [ %_16.i19.i.i.i.i.i.i, %bb6.i.i.i.i.i.i ], [ %_16.i.i.i.i.i.i.i, %bb3.i.i.i.i.i.i ]
  call void @llvm.lifetime.start.p0(i64 32, ptr nonnull %_13.i.i), !noalias !1138
  call void @llvm.lifetime.start.p0(i64 12, ptr nonnull %_63.i.i), !noalias !1138
; invoke core::unicode::unicode_data::conversions::to_lower
  invoke void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr noalias noundef nonnull sret([12 x i8]) align 4 captures(none) dereferenceable(12) %_63.i.i, i32 noundef %_5.sroa.0.0.i.i)
          to label %.noexc159.i unwind label %cleanup.loopexit.loopexit.i, !noalias !1020

.noexc159.i:                                      ; preds = %bb5.i.i
  %_3.i.i.i = load i32, ptr %2, align 4, !range !14, !alias.scope !1139, !noalias !1142, !noundef !2
  %70 = icmp eq i32 %_3.i.i.i, 0
  %_6.i13.i.i = load i32, ptr %3, align 4, !range !14, !alias.scope !1139, !noalias !1142
  %71 = icmp eq i32 %_6.i13.i.i, 0
  %spec.select.i.i.i = select i1 %71, i64 1, i64 2
  %iter.sroa.4.0.i.i.i = select i1 %70, i64 %spec.select.i.i.i, i64 3
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(12) %_62.sroa.5.0._13.sroa_idx.i.i, ptr noundef nonnull align 4 dereferenceable(12) %_63.i.i, i64 12, i1 false), !noalias !1138
  call void @llvm.lifetime.end.p0(i64 12, ptr nonnull %_63.i.i), !noalias !1138
  store i64 0, ptr %_13.i.i, align 8, !noalias !1138
  store i64 %iter.sroa.4.0.i.i.i, ptr %_62.sroa.4.0._13.sroa_idx.i.i, align 8, !noalias !1138
  call void @llvm.lifetime.start.p0(i64 16, ptr nonnull %args.i.i), !noalias !1138
  store ptr %_13.i.i, ptr %args.i.i, align 8, !noalias !1138
  store ptr @_RNvXsB_NtCsjMrxcFdYDNN_4core4charNtB5_11ToLowercaseNtNtB7_3fmt7Display3fmt, ptr %_15.sroa.4.0..sroa_idx.i.i, align 8, !noalias !1138
; invoke core::fmt::write
  %72 = invoke noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1 %buf.i, ptr noalias noundef nonnull readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull @alloc_0c812808379efded5a4fb82d2790b556, ptr noundef nonnull %args.i.i)
          to label %.noexc160.i unwind label %cleanup.loopexit.loopexit.i, !noalias !1020

.noexc160.i:                                      ; preds = %.noexc159.i
  call void @llvm.lifetime.end.p0(i64 16, ptr nonnull %args.i.i), !noalias !1138
  call void @llvm.lifetime.end.p0(i64 32, ptr nonnull %_13.i.i), !noalias !1138
  br i1 %72, label %bb2.i.i, label %bb1.i.i.backedge, !prof !97

bb47.i.i:                                         ; preds = %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i, %bb1.i.i.i
  %chars.sroa.0.528.i.i = phi ptr [ %_16.i26.i.i.i.i.i.i, %_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck.exit.i.i ], [ %_22.i.i, %bb1.i.i.i ]
  call void @llvm.experimental.noalias.scope.decl(metadata !1144)
  call void @llvm.experimental.noalias.scope.decl(metadata !1147)
  call void @llvm.experimental.noalias.scope.decl(metadata !1150)
  %len.i.i.i.i295.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1153, !noalias !1156, !noundef !2
  %self2.i.i.i.i296.i = load i64, ptr %buf.i, align 8, !range !6, !alias.scope !1153, !noalias !1156, !noundef !2
  %_9.i.i.i.i.i = sub i64 %self2.i.i.i.i296.i, %len.i.i.i.i295.i
  %_7.i.i.i.i297.i = icmp ult i64 %_9.i.i.i.i.i, 2
  br i1 %_7.i.i.i.i297.i, label %bb1.i.i.i.i302.i, label %.noexc161.i, !prof !142

bb1.i.i.i.i302.i:                                 ; preds = %bb47.i.i
; invoke <alloc::raw_vec::RawVecInner<_>>::reserve::do_reserve_and_handle::<alloc::alloc::Global>
  invoke fastcc void @_RINvNvMs2_NtCsdJPVW0sQgAG_5alloc7raw_vecINtB8_11RawVecInnerpE7reserve21do_reserve_and_handleNtNtBa_5alloc6GlobalECs2f1hMIlW3Va_4heck(ptr noalias noundef nonnull align 8 dereferenceable(24) %buf.i, i64 noundef %len.i.i.i.i295.i, i64 noundef 2)
          to label %.noexc304.i unwind label %cleanup.loopexit.loopexit.i

.noexc304.i:                                      ; preds = %bb1.i.i.i.i302.i
  %len.pre.i.i.i303.i = load i64, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1159, !noalias !1156
  br label %.noexc161.i

.noexc161.i:                                      ; preds = %.noexc304.i, %bb47.i.i
  %len.i.i.i298.i = phi i64 [ %len.i.i.i.i295.i, %bb47.i.i ], [ %len.pre.i.i.i303.i, %.noexc304.i ]
  %_9.i.i.i299.i = icmp sgt i64 %len.i.i.i298.i, -1
  call void @llvm.assume(i1 %_9.i.i.i299.i)
  %_10.i.i.i300.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1159, !noalias !1156, !nonnull !2, !noundef !2
  %dst.i.i.i301.i = getelementptr inbounds nuw i8, ptr %_10.i.i.i300.i, i64 %len.i.i.i298.i
  store i16 -32049, ptr %dst.i.i.i301.i, align 1, !noalias !1160
  %73 = add nuw i64 %len.i.i.i298.i, 2
  store i64 %73, ptr %_10.sroa.5.0.buf.sroa_idx.i, align 8, !alias.scope !1159, !noalias !1156
  br label %bb1.i.i.backedge

bb1.i.i.backedge:                                 ; preds = %.noexc161.i, %.noexc160.i
  %chars.sroa.17.0.i.i.be = phi i32 [ 1114112, %.noexc161.i ], [ %chars.sroa.17.1.i.i, %.noexc160.i ]
  %chars.sroa.0.0.i.i.be = phi ptr [ %chars.sroa.0.528.i.i, %.noexc161.i ], [ %chars.sroa.0.2.i.i, %.noexc160.i ]
  br label %bb1.i.i

cleanup.loopexit.loopexit.i:                      ; preds = %bb1.i.i.i.i302.i, %.noexc159.i, %bb5.i.i
  %lpad.loopexit163.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i.loopexit:    ; preds = %bb57.i.i.i, %bb63.i.i.i, %bb69.i.i.i, %bb82.i.i.i, %bb88.i.i.i
  %lpad.loopexit = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp: ; preds = %bb95.i.i.i, %bb1.i.i.i.i150.i, %bb1.i.i.i.i138.i
  %lpad.loopexit.split-lp = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.i:             ; preds = %_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str7patternNCINvCs2f1hMIlW3Va_4heck9transformNvBJ_9lowercaseNCNvXs0_NtBJ_5snakeINtB1z_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0NtB5_11MultiCharEq7matchesBJ_.exit.i.i.i.i.i.i.i, %bb16.i.i.i.i.i.i.i.i.i.i
  %lpad.loopexit2.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i: ; preds = %bb1.i.i.i.i.i, %bb102.i.i.i
  %lpad.loopexit5.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i: ; preds = %bb2.i.i, %bb101.i.i.invoke.i
  %lpad.loopexit.split-lp.i = landingpad { ptr, i32 }
          cleanup
  br label %cleanup.i

cleanup.i:                                        ; preds = %cleanup.loopexit.loopexit.split-lp.i.loopexit, %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i, %cleanup.loopexit.split-lp.loopexit.i, %cleanup.loopexit.loopexit.i
  %lpad.phi.i = phi { ptr, i32 } [ %lpad.loopexit2.i, %cleanup.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit5.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.i ], [ %lpad.loopexit.split-lp.i, %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i ], [ %lpad.loopexit163.i, %cleanup.loopexit.loopexit.i ], [ %lpad.loopexit, %cleanup.loopexit.loopexit.split-lp.i.loopexit ], [ %lpad.loopexit.split-lp, %cleanup.loopexit.loopexit.split-lp.i.loopexit.split-lp ]
  call void @llvm.experimental.noalias.scope.decl(metadata !1161)
  %_1.val.i.i = load i64, ptr %buf.i, align 8, !alias.scope !1161, !noalias !1020
  %74 = icmp eq i64 %_1.val.i.i, 0
  br i1 %74, label %bb4.i, label %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i

_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i: ; preds = %cleanup.i
  %_1.val1.i.i = load ptr, ptr %_10.sroa.4.0.buf.sroa_idx.i, align 8, !alias.scope !1161, !noalias !1020, !nonnull !2, !noundef !2
; call __rustc::__rust_dealloc
  call void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr noundef nonnull %_1.val1.i.i, i64 noundef %_1.val.i.i, i64 noundef range(i64 1, -9223372036854775807) 1) #17, !noalias !1164
  br label %bb4.i

bb2.i.i:                                          ; preds = %_0.i.i.i.noexc.i, %_0.i56.i.i.noexc.i, %.noexc160.i
  call void @llvm.lifetime.start.p0(i64 0, ptr nonnull %e.i.i), !noalias !1020
; invoke core::result::unwrap_failed
  invoke void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance) @alloc_cc656815297f75969399c3f4b1ad3de4, i64 noundef 55, ptr noundef nonnull align 1 %e.i.i, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32) @vtable.0, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24) @alloc_f3c70bf9d2724ff8f638341943ddf3c8) #18
          to label %.noexc17.i unwind label %cleanup.loopexit.split-lp.loopexit.split-lp.loopexit.split-lp.i, !noalias !1020

.noexc17.i:                                       ; preds = %bb2.i.i
  unreachable

bb4.i:                                            ; preds = %_RNvXs_NtCsdJPVW0sQgAG_5alloc5allocNtB4_6GlobalNtNtCsjMrxcFdYDNN_4core5alloc9Allocator10deallocate.exit.i.i.i4.i.i.i, %cleanup.i
  resume { ptr, i32 } %lpad.phi.i

_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5snake11AsSnakeCaseReENtB5_12SpecToString14spec_to_stringBD_.exit: ; preds = %bb35.i.i.i
  call void @llvm.memcpy.p0.p0.i64(ptr noundef nonnull align 8 dereferenceable(24) %_0, ptr noundef nonnull align 8 dereferenceable(24) %buf.i, i64 24, i1 false)
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %formatter.i), !noalias !1020
  call void @llvm.lifetime.end.p0(i64 24, ptr nonnull %buf.i), !noalias !1020
  ret void
}

; <alloc::string::String as core::fmt::Write>::write_fmt
; Function Attrs: uwtable
define internal noundef zeroext i1 @_RNvYNtNtCsdJPVW0sQgAG_5alloc6string6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_fmtCs2f1hMIlW3Va_4heck(ptr noalias noundef align 8 dereferenceable(24) %self, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1) unnamed_addr #2 {
start:
; call core::fmt::write
  %0 = tail call noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 8 dereferenceable(24) %self, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48) @vtable.1, ptr noundef nonnull %args.0, ptr noundef nonnull %args.1)
  ret i1 %0
}

declare i32 @rust_eh_personality(...) unnamed_addr #5

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.start.p0(i64 immarg, ptr captures(none)) #6

; Function Attrs: mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias writeonly captures(none), ptr noalias readonly captures(none), i64, i1 immarg) #7

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite)
declare void @llvm.lifetime.end.p0(i64 immarg, ptr captures(none)) #6

; Function Attrs: mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write)
declare void @llvm.assume(i1 noundef) #8

; core::str::slice_error_fail
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core3str16slice_error_fail(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, i64 noundef, i64 noundef, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; core::unicode::unicode_data::lowercase::lookup
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9lowercase6lookup(i32 noundef range(i32 0, 1114112)) unnamed_addr #2

; core::unicode::unicode_data::uppercase::lookup
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data9uppercase6lookup(i32 noundef range(i32 0, 1114112)) unnamed_addr #2

; alloc::raw_vec::handle_error
; Function Attrs: cold minsize noreturn optsize uwtable
declare void @_RNvNtCsdJPVW0sQgAG_5alloc7raw_vec12handle_error(i64 noundef range(i64 0, -9223372036854775807), i64) unnamed_addr #10

; core::fmt::write
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvNtCsjMrxcFdYDNN_4core3fmt5write(ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(48), ptr noundef nonnull, ptr noundef nonnull) unnamed_addr #2

; core::unicode::unicode_data::conversions::to_upper
; Function Attrs: uwtable
declare void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_upper(ptr dead_on_unwind noalias noundef writable sret([12 x i8]) align 4 captures(none) dereferenceable(12), i32 noundef range(i32 0, 1114112)) unnamed_addr #2

; core::unicode::unicode_data::conversions::to_lower
; Function Attrs: uwtable
declare void @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data11conversions8to_lower(ptr dead_on_unwind noalias noundef writable sret([12 x i8]) align 4 captures(none) dereferenceable(12), i32 noundef range(i32 0, 1114112)) unnamed_addr #2

; __rustc::__rust_dealloc
; Function Attrs: nounwind allockind("free") uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc14___rust_dealloc(ptr allocptr noundef, i64 noundef, i64 noundef) unnamed_addr #11

; __rustc::__rust_realloc
; Function Attrs: nounwind allockind("realloc,aligned") allocsize(3) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc14___rust_realloc(ptr allocptr noundef, i64 noundef, i64 allocalign noundef, i64 noundef) unnamed_addr #12

; __rustc::__rust_no_alloc_shim_is_unstable_v2
; Function Attrs: nounwind uwtable
declare void @_RNvCsKhRCbHf33p_7___rustc35___rust_no_alloc_shim_is_unstable_v2() unnamed_addr #0

; __rustc::__rust_alloc
; Function Attrs: nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable
declare noalias noundef ptr @_RNvCsKhRCbHf33p_7___rustc12___rust_alloc(i64 noundef, i64 allocalign noundef) unnamed_addr #13

; core::result::unwrap_failed
; Function Attrs: cold noinline noreturn uwtable
declare void @_RNvNtCsjMrxcFdYDNN_4core6result13unwrap_failed(ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef, ptr noundef nonnull align 1, ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(32), ptr noalias noundef readonly align 8 captures(address, read_provenance) dereferenceable(24)) unnamed_addr #9

; core::unicode::unicode_data::alphabetic::lookup_slow
; Function Attrs: noinline uwtable
declare noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data10alphabetic11lookup_slow(i32 noundef range(i32 0, 1114112)) unnamed_addr #14

; core::unicode::unicode_data::n::lookup_slow
; Function Attrs: noinline uwtable
declare noundef zeroext i1 @_RNvNtNtNtCsjMrxcFdYDNN_4core7unicode12unicode_data1n11lookup_slow(i32 noundef range(i32 0, 1114112)) unnamed_addr #14

; <core::fmt::Formatter>::write_str
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvMsa_NtCsjMrxcFdYDNN_4core3fmtNtB5_9Formatter9write_str(ptr noalias noundef align 8 dereferenceable(24), ptr noalias noundef nonnull readonly align 1 captures(address, read_provenance), i64 noundef) unnamed_addr #2

; <core::fmt::Formatter as core::fmt::Write>::write_char
; Function Attrs: uwtable
declare noundef zeroext i1 @_RNvXsb_NtCsjMrxcFdYDNN_4core3fmtNtB5_9FormatterNtB5_5Write10write_char(ptr noalias noundef align 8 dereferenceable(24), i32 noundef range(i32 0, 1114112)) unnamed_addr #2

; Function Attrs: nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite)
declare void @llvm.experimental.noalias.scope.decl(metadata) #15

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umin.i64(i64, i64) #16

; Function Attrs: nocallback nofree nosync nounwind speculatable willreturn memory(none)
declare i64 @llvm.umax.i64(i64, i64) #16

attributes #0 = { nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #1 = { cold uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #2 = { uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #3 = { cold nounwind uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #4 = { inlinehint uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #5 = { "target-cpu"="apple-m1" }
attributes #6 = { mustprogress nocallback nofree nosync nounwind willreturn memory(argmem: readwrite) }
attributes #7 = { mustprogress nocallback nofree nounwind willreturn memory(argmem: readwrite) }
attributes #8 = { mustprogress nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: write) }
attributes #9 = { cold noinline noreturn uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #10 = { cold minsize noreturn optsize uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #11 = { nounwind allockind("free") uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #12 = { nounwind allockind("realloc,aligned") allocsize(3) uwtable "alloc-family"="__rust_alloc" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #13 = { nounwind allockind("alloc,uninitialized,aligned") allocsize(0) uwtable "alloc-family"="__rust_alloc" "alloc-variant-zeroed"="_RNvCsKhRCbHf33p_7___rustc19___rust_alloc_zeroed" "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #14 = { noinline uwtable "frame-pointer"="non-leaf" "probe-stack"="inline-asm" "target-cpu"="apple-m1" }
attributes #15 = { nocallback nofree nosync nounwind willreturn memory(inaccessiblemem: readwrite) }
attributes #16 = { nocallback nofree nosync nounwind speculatable willreturn memory(none) }
attributes #17 = { nounwind }
attributes #18 = { noreturn }
attributes #19 = { noinline noreturn }
attributes #20 = { inlinehint }
attributes #21 = { noinline }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 8, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.95.0-nightly (39052daf9 2026-01-22)"}
!2 = !{}
!3 = !{!4}
!4 = distinct !{!4, !5, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs2f1hMIlW3Va_4heck: %self"}
!5 = distinct !{!5, !"_RNvMs4_NtCsdJPVW0sQgAG_5alloc7raw_vecNtB5_11RawVecInner14grow_amortizedCs2f1hMIlW3Va_4heck"}
!6 = !{i64 0, i64 -9223372036854775808}
!7 = !{i64 0, i64 2}
!8 = !{i64 0, i64 -9223372036854775807}
!9 = !{!10, !12}
!10 = distinct !{!10, !11, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!11 = distinct !{!11, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!12 = distinct !{!12, !13, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!13 = distinct !{!13, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!14 = !{i32 0, i32 1114112}
!15 = !{i64 1}
!16 = !{i64 8}
!17 = !{!18, !20}
!18 = distinct !{!18, !19, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!19 = distinct !{!19, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!20 = distinct !{!20, !21, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!21 = distinct !{!21, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!22 = !{!23}
!23 = distinct !{!23, !24, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!24 = distinct !{!24, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!25 = !{!26}
!26 = distinct !{!26, !27, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!27 = distinct !{!27, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!28 = !{!29, !31, !33, !35, !37}
!29 = distinct !{!29, !30, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!30 = distinct !{!30, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!31 = distinct !{!31, !32, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!32 = distinct !{!32, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!33 = distinct !{!33, !34, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!34 = distinct !{!34, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!35 = distinct !{!35, !36, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!36 = distinct !{!36, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!37 = distinct !{!37, !36, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!38 = !{!39}
!39 = distinct !{!39, !40, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!40 = distinct !{!40, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!41 = !{!42}
!42 = distinct !{!42, !40, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!43 = !{!44}
!44 = distinct !{!44, !45, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!45 = distinct !{!45, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!46 = !{!47}
!47 = distinct !{!47, !48, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!48 = distinct !{!48, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!49 = !{!50}
!50 = distinct !{!50, !48, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!51 = !{!"branch_weights", i32 2000, i32 6004}
!52 = !{!53}
!53 = distinct !{!53, !54, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11lower_camel16AsLowerCamelCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!54 = distinct !{!54, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11lower_camel16AsLowerCamelCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!55 = !{!56}
!56 = distinct !{!56, !57, !"_RINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB2_11lower_camelINtBC_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0NCBx_s_0EB2_: %s.0"}
!57 = distinct !{!57, !"_RINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB2_11lower_camelINtBC_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0NCBx_s_0EB2_"}
!58 = !{!59, !61, !63, !65, !66, !68, !69, !71, !72, !74, !75, !76, !53}
!59 = distinct !{!59, !60, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!60 = distinct !{!60, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!61 = distinct !{!61, !62, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!62 = distinct !{!62, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!63 = distinct !{!63, !64, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1a_11lower_camelINtB1K_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1F_s_0E0ENtB5_8Searcher4nextB1a_: %_0"}
!64 = distinct !{!64, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1a_11lower_camelINtB1K_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1F_s_0E0ENtB5_8Searcher4nextB1a_"}
!65 = distinct !{!65, !64, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1a_11lower_camelINtB1K_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1F_s_0E0ENtB5_8Searcher4nextB1a_: %self"}
!66 = distinct !{!66, !67, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB14_11lower_camelINtB1E_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1z_s_0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!67 = distinct !{!67, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB14_11lower_camelINtB1E_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1z_s_0E0ENtB5_8Searcher10next_matchB14_"}
!68 = distinct !{!68, !67, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB14_11lower_camelINtB1E_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1z_s_0E0ENtB5_8Searcher10next_matchB14_: %self"}
!69 = distinct !{!69, !70, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1c_11lower_camelINtB1M_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1H_s_0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!70 = distinct !{!70, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1c_11lower_camelINtB1M_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1H_s_0E0ENtB5_8Searcher10next_matchB1c_"}
!71 = distinct !{!71, !70, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB1c_11lower_camelINtB1M_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1H_s_0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!72 = distinct !{!72, !73, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB11_11lower_camelINtB1B_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1w_s_0E0E4nextB11_: %self"}
!73 = distinct !{!73, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB11_11lower_camelINtB1B_16AsLowerCamelCaseReENtNtB9_3fmt7Display3fmt0NCB1w_s_0E0E4nextB11_"}
!74 = distinct !{!74, !57, !"_RINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB2_11lower_camelINtBC_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0NCBx_s_0EB2_: argument 1"}
!75 = distinct !{!75, !57, !"_RINvCs2f1hMIlW3Va_4heck9transformNCNvXs_NtB2_11lower_camelINtBC_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0NCBx_s_0EB2_: %f"}
!76 = distinct !{!76, !77, !"_RNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB4_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_: %f"}
!77 = distinct !{!77, !"_RNvXs_NtCs2f1hMIlW3Va_4heck11lower_camelINtB4_16AsLowerCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_"}
!78 = !{!79, !81, !74, !75, !76, !53}
!79 = distinct !{!79, !80, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!80 = distinct !{!80, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!81 = distinct !{!81, !82, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!82 = distinct !{!82, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!83 = !{!84, !86, !88, !90, !92, !74, !75, !76, !53}
!84 = distinct !{!84, !85, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!85 = distinct !{!85, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!86 = distinct !{!86, !87, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!87 = distinct !{!87, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!88 = distinct !{!88, !89, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!89 = distinct !{!89, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!90 = distinct !{!90, !91, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!91 = distinct !{!91, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!92 = distinct !{!92, !91, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!93 = !{!94, !56}
!94 = distinct !{!94, !95, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!95 = distinct !{!95, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!96 = !{!74, !75, !76, !53}
!97 = !{!"branch_weights", i32 1073205, i32 2146410443}
!98 = !{!99, !56}
!99 = distinct !{!99, !100, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!100 = distinct !{!100, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!101 = !{!102, !56}
!102 = distinct !{!102, !103, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!103 = distinct !{!103, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!104 = !{!105}
!105 = distinct !{!105, !106, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %s.0"}
!106 = distinct !{!106, !"_RNvCs2f1hMIlW3Va_4heck9lowercase"}
!107 = !{!108, !110, !53}
!108 = distinct !{!108, !109, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!109 = distinct !{!109, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!110 = distinct !{!110, !106, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %f"}
!111 = !{!112, !114, !116, !118, !120, !110, !53}
!112 = distinct !{!112, !113, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!113 = distinct !{!113, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!114 = distinct !{!114, !115, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!115 = distinct !{!115, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!116 = distinct !{!116, !117, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!117 = distinct !{!117, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!118 = distinct !{!118, !119, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!119 = distinct !{!119, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!120 = distinct !{!120, !119, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!121 = !{!105, !110, !53}
!122 = !{!123}
!123 = distinct !{!123, !124, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!124 = distinct !{!124, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!125 = !{!126, !105, !110, !53}
!126 = distinct !{!126, !124, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!127 = !{!128}
!128 = distinct !{!128, !129, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!129 = distinct !{!129, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!130 = !{!131}
!131 = distinct !{!131, !132, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!132 = distinct !{!132, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!133 = !{!134}
!134 = distinct !{!134, !135, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!135 = distinct !{!135, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!136 = !{!137, !134, !131, !128}
!137 = distinct !{!137, !138, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!138 = distinct !{!138, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!139 = !{!140, !141}
!140 = distinct !{!140, !132, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!141 = distinct !{!141, !129, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!142 = !{!"branch_weights", !"expected", i32 1, i32 2000}
!143 = !{!134, !131, !128}
!144 = !{!145}
!145 = distinct !{!145, !146, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!146 = distinct !{!146, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!147 = !{!148, !150, !152, !53}
!148 = distinct !{!148, !149, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!149 = distinct !{!149, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!150 = distinct !{!150, !151, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!151 = distinct !{!151, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!152 = distinct !{!152, !146, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!153 = !{!145, !152, !53}
!154 = !{!155, !157, !152, !53}
!155 = distinct !{!155, !156, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!156 = distinct !{!156, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!157 = distinct !{!157, !158, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!158 = distinct !{!158, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!159 = !{!160, !145}
!160 = distinct !{!160, !161, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!161 = distinct !{!161, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!162 = !{!152, !53}
!163 = !{!164}
!164 = distinct !{!164, !165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!165 = distinct !{!165, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!166 = !{!164, !53}
!167 = !{!168}
!168 = distinct !{!168, !169, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11upper_camel16AsUpperCamelCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!169 = distinct !{!169, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck11upper_camel16AsUpperCamelCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!170 = !{!171}
!171 = distinct !{!171, !172, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs0_NtB2_11upper_camelINtBU_16AsUpperCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!172 = distinct !{!172, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs0_NtB2_11upper_camelINtBU_16AsUpperCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!173 = !{!174, !176, !178, !180, !181, !183, !184, !186, !187, !189, !190, !168}
!174 = distinct !{!174, !175, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!175 = distinct !{!175, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!176 = distinct !{!176, !177, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!177 = distinct !{!177, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!178 = distinct !{!178, !179, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs0_NtB1a_11upper_camelINtB23_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!179 = distinct !{!179, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs0_NtB1a_11upper_camelINtB23_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!180 = distinct !{!180, !179, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs0_NtB1a_11upper_camelINtB23_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!181 = distinct !{!181, !182, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs0_NtB14_11upper_camelINtB1X_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!182 = distinct !{!182, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs0_NtB14_11upper_camelINtB1X_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!183 = distinct !{!183, !182, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs0_NtB14_11upper_camelINtB1X_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!184 = distinct !{!184, !185, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs0_NtB1c_11upper_camelINtB25_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!185 = distinct !{!185, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs0_NtB1c_11upper_camelINtB25_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!186 = distinct !{!186, !185, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs0_NtB1c_11upper_camelINtB25_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!187 = distinct !{!187, !188, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs0_NtB11_11upper_camelINtB1U_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!188 = distinct !{!188, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs0_NtB11_11upper_camelINtB1U_16AsUpperCamelCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!189 = distinct !{!189, !172, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs0_NtB2_11upper_camelINtBU_16AsUpperCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!190 = distinct !{!190, !191, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck11upper_camelINtB5_16AsUpperCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_: %f"}
!191 = distinct !{!191, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck11upper_camelINtB5_16AsUpperCamelCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_"}
!192 = !{!193, !195, !189, !190, !168}
!193 = distinct !{!193, !194, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!194 = distinct !{!194, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!195 = distinct !{!195, !196, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!196 = distinct !{!196, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!197 = !{!198, !200, !202, !204, !206, !189, !190, !168}
!198 = distinct !{!198, !199, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!199 = distinct !{!199, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!200 = distinct !{!200, !201, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!201 = distinct !{!201, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!202 = distinct !{!202, !203, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!203 = distinct !{!203, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!204 = distinct !{!204, !205, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!205 = distinct !{!205, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!206 = distinct !{!206, !205, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!207 = !{!208, !171}
!208 = distinct !{!208, !209, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!209 = distinct !{!209, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!210 = !{!189, !190, !168}
!211 = !{!212, !171}
!212 = distinct !{!212, !213, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!213 = distinct !{!213, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!214 = !{!215, !171}
!215 = distinct !{!215, !216, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!216 = distinct !{!216, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!217 = !{!218}
!218 = distinct !{!218, !219, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!219 = distinct !{!219, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!220 = !{!221, !223, !225, !168}
!221 = distinct !{!221, !222, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!222 = distinct !{!222, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!223 = distinct !{!223, !224, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!224 = distinct !{!224, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!225 = distinct !{!225, !219, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!226 = !{!218, !225, !168}
!227 = !{!228, !230, !225, !168}
!228 = distinct !{!228, !229, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!229 = distinct !{!229, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!230 = distinct !{!230, !231, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!231 = distinct !{!231, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!232 = !{!233, !218}
!233 = distinct !{!233, !234, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!234 = distinct !{!234, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!235 = !{!225, !168}
!236 = !{!237}
!237 = distinct !{!237, !238, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!238 = distinct !{!238, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!239 = !{!237, !168}
!240 = !{!241}
!241 = distinct !{!241, !242, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_kebab17AsShoutyKebabCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!242 = distinct !{!242, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_kebab17AsShoutyKebabCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!243 = !{!244}
!244 = distinct !{!244, !245, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs_NtB2_12shouty_kebabINtBR_17AsShoutyKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!245 = distinct !{!245, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs_NtB2_12shouty_kebabINtBR_17AsShoutyKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!246 = !{!247, !249, !251, !253, !254, !256, !257, !259, !260, !262, !263, !241}
!247 = distinct !{!247, !248, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!248 = distinct !{!248, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!249 = distinct !{!249, !250, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!250 = distinct !{!250, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!251 = distinct !{!251, !252, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs_NtB1a_12shouty_kebabINtB20_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!252 = distinct !{!252, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs_NtB1a_12shouty_kebabINtB20_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!253 = distinct !{!253, !252, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs_NtB1a_12shouty_kebabINtB20_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!254 = distinct !{!254, !255, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs_NtB14_12shouty_kebabINtB1U_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!255 = distinct !{!255, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs_NtB14_12shouty_kebabINtB1U_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!256 = distinct !{!256, !255, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs_NtB14_12shouty_kebabINtB1U_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!257 = distinct !{!257, !258, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs_NtB1c_12shouty_kebabINtB22_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!258 = distinct !{!258, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs_NtB1c_12shouty_kebabINtB22_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!259 = distinct !{!259, !258, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs_NtB1c_12shouty_kebabINtB22_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!260 = distinct !{!260, !261, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9uppercaseNCNvXs_NtB11_12shouty_kebabINtB1R_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!261 = distinct !{!261, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9uppercaseNCNvXs_NtB11_12shouty_kebabINtB1R_17AsShoutyKebabCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!262 = distinct !{!262, !245, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs_NtB2_12shouty_kebabINtBR_17AsShoutyKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!263 = distinct !{!263, !264, !"_RNvXs_NtCs2f1hMIlW3Va_4heck12shouty_kebabINtB4_17AsShoutyKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_: %f"}
!264 = distinct !{!264, !"_RNvXs_NtCs2f1hMIlW3Va_4heck12shouty_kebabINtB4_17AsShoutyKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_"}
!265 = !{!266, !268, !262, !263, !241}
!266 = distinct !{!266, !267, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!267 = distinct !{!267, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!268 = distinct !{!268, !269, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!269 = distinct !{!269, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!270 = !{!271, !273, !275, !277, !279, !262, !263, !241}
!271 = distinct !{!271, !272, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!272 = distinct !{!272, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!273 = distinct !{!273, !274, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!274 = distinct !{!274, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!275 = distinct !{!275, !276, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!276 = distinct !{!276, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!277 = distinct !{!277, !278, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!278 = distinct !{!278, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!279 = distinct !{!279, !278, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!280 = !{!281}
!281 = distinct !{!281, !282, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!282 = distinct !{!282, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!283 = !{!284}
!284 = distinct !{!284, !285, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!285 = distinct !{!285, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!286 = !{!287}
!287 = distinct !{!287, !288, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!288 = distinct !{!288, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!289 = !{!290, !287, !284, !281}
!290 = distinct !{!290, !291, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!291 = distinct !{!291, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!292 = !{!293, !294, !241}
!293 = distinct !{!293, !285, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!294 = distinct !{!294, !282, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!295 = !{!287, !284, !281}
!296 = !{!287, !284, !281, !241}
!297 = !{!298, !244}
!298 = distinct !{!298, !299, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!299 = distinct !{!299, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!300 = !{!262, !263, !241}
!301 = !{!302}
!302 = distinct !{!302, !303, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!303 = distinct !{!303, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!304 = !{!305}
!305 = distinct !{!305, !306, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!306 = distinct !{!306, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!307 = !{!308}
!308 = distinct !{!308, !309, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!309 = distinct !{!309, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!310 = !{!311, !308, !305, !302}
!311 = distinct !{!311, !312, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!312 = distinct !{!312, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!313 = !{!314, !315, !241}
!314 = distinct !{!314, !306, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!315 = distinct !{!315, !303, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!316 = !{!308, !305, !302}
!317 = !{!308, !305, !302, !241}
!318 = !{!319, !244}
!319 = distinct !{!319, !320, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!320 = distinct !{!320, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!321 = !{!322}
!322 = distinct !{!322, !323, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!323 = distinct !{!323, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!324 = !{!325}
!325 = distinct !{!325, !326, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!326 = distinct !{!326, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!327 = !{!328}
!328 = distinct !{!328, !329, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!329 = distinct !{!329, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!330 = !{!331, !328, !325, !322}
!331 = distinct !{!331, !332, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!332 = distinct !{!332, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!333 = !{!334, !335, !241}
!334 = distinct !{!334, !326, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!335 = distinct !{!335, !323, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!336 = !{!328, !325, !322}
!337 = !{!328, !325, !322, !241}
!338 = !{!339, !244}
!339 = distinct !{!339, !340, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!340 = distinct !{!340, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!341 = !{!342}
!342 = distinct !{!342, !343, !"_RNvCs2f1hMIlW3Va_4heck9uppercase: %s.0"}
!343 = distinct !{!343, !"_RNvCs2f1hMIlW3Va_4heck9uppercase"}
!344 = !{!345, !347, !241}
!345 = distinct !{!345, !346, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!346 = distinct !{!346, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!347 = distinct !{!347, !343, !"_RNvCs2f1hMIlW3Va_4heck9uppercase: %f"}
!348 = !{!342, !347, !241}
!349 = !{!350}
!350 = distinct !{!350, !351, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!351 = distinct !{!351, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!352 = !{!353, !342, !347, !241}
!353 = distinct !{!353, !351, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!354 = !{!355}
!355 = distinct !{!355, !356, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!356 = distinct !{!356, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!357 = !{!355, !241}
!358 = !{!359}
!359 = distinct !{!359, !360, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5kebab11AsKebabCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!360 = distinct !{!360, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5kebab11AsKebabCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!361 = !{!362}
!362 = distinct !{!362, !363, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs_NtB2_5kebabINtBR_11AsKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!363 = distinct !{!363, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs_NtB2_5kebabINtBR_11AsKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!364 = !{!365, !367, !369, !371, !372, !374, !375, !377, !378, !380, !381, !359}
!365 = distinct !{!365, !366, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!366 = distinct !{!366, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!367 = distinct !{!367, !368, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!368 = distinct !{!368, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!369 = distinct !{!369, !370, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs_NtB1a_5kebabINtB20_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!370 = distinct !{!370, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs_NtB1a_5kebabINtB20_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!371 = distinct !{!371, !370, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs_NtB1a_5kebabINtB20_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!372 = distinct !{!372, !373, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs_NtB14_5kebabINtB1U_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!373 = distinct !{!373, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs_NtB14_5kebabINtB1U_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!374 = distinct !{!374, !373, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs_NtB14_5kebabINtB1U_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!375 = distinct !{!375, !376, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs_NtB1c_5kebabINtB22_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!376 = distinct !{!376, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs_NtB1c_5kebabINtB22_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!377 = distinct !{!377, !376, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs_NtB1c_5kebabINtB22_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!378 = distinct !{!378, !379, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9lowercaseNCNvXs_NtB11_5kebabINtB1R_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!379 = distinct !{!379, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9lowercaseNCNvXs_NtB11_5kebabINtB1R_11AsKebabCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!380 = distinct !{!380, !363, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs_NtB2_5kebabINtBR_11AsKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!381 = distinct !{!381, !382, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5kebabINtB4_11AsKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_: %f"}
!382 = distinct !{!382, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5kebabINtB4_11AsKebabCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_"}
!383 = !{!384, !386, !380, !381, !359}
!384 = distinct !{!384, !385, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!385 = distinct !{!385, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!386 = distinct !{!386, !387, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!387 = distinct !{!387, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!388 = !{!389, !391, !393, !395, !397, !380, !381, !359}
!389 = distinct !{!389, !390, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!390 = distinct !{!390, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!391 = distinct !{!391, !392, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!392 = distinct !{!392, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!393 = distinct !{!393, !394, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!394 = distinct !{!394, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!395 = distinct !{!395, !396, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!396 = distinct !{!396, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!397 = distinct !{!397, !396, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!398 = !{!399}
!399 = distinct !{!399, !400, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!400 = distinct !{!400, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!401 = !{!402}
!402 = distinct !{!402, !403, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!403 = distinct !{!403, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!404 = !{!405}
!405 = distinct !{!405, !406, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!406 = distinct !{!406, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!407 = !{!408, !405, !402, !399}
!408 = distinct !{!408, !409, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!409 = distinct !{!409, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!410 = !{!411, !412, !359}
!411 = distinct !{!411, !403, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!412 = distinct !{!412, !400, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!413 = !{!405, !402, !399}
!414 = !{!405, !402, !399, !359}
!415 = !{!416, !362}
!416 = distinct !{!416, !417, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!417 = distinct !{!417, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!418 = !{!380, !381, !359}
!419 = !{!420}
!420 = distinct !{!420, !421, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!421 = distinct !{!421, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!422 = !{!423}
!423 = distinct !{!423, !424, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!424 = distinct !{!424, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!425 = !{!426}
!426 = distinct !{!426, !427, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!427 = distinct !{!427, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!428 = !{!429, !426, !423, !420}
!429 = distinct !{!429, !430, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!430 = distinct !{!430, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!431 = !{!432, !433, !359}
!432 = distinct !{!432, !424, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!433 = distinct !{!433, !421, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!434 = !{!426, !423, !420}
!435 = !{!426, !423, !420, !359}
!436 = !{!437, !362}
!437 = distinct !{!437, !438, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!438 = distinct !{!438, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!439 = !{!440}
!440 = distinct !{!440, !441, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!441 = distinct !{!441, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!442 = !{!443}
!443 = distinct !{!443, !444, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!444 = distinct !{!444, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!445 = !{!446}
!446 = distinct !{!446, !447, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!447 = distinct !{!447, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!448 = !{!449, !446, !443, !440}
!449 = distinct !{!449, !450, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!450 = distinct !{!450, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!451 = !{!452, !453, !359}
!452 = distinct !{!452, !444, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!453 = distinct !{!453, !441, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!454 = !{!446, !443, !440}
!455 = !{!446, !443, !440, !359}
!456 = !{!457, !362}
!457 = distinct !{!457, !458, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!458 = distinct !{!458, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!459 = !{!460}
!460 = distinct !{!460, !461, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %s.0"}
!461 = distinct !{!461, !"_RNvCs2f1hMIlW3Va_4heck9lowercase"}
!462 = !{!463, !465, !359}
!463 = distinct !{!463, !464, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!464 = distinct !{!464, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!465 = distinct !{!465, !461, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %f"}
!466 = !{!467, !469, !471, !473, !475, !465, !359}
!467 = distinct !{!467, !468, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!468 = distinct !{!468, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!469 = distinct !{!469, !470, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!470 = distinct !{!470, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!471 = distinct !{!471, !472, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!472 = distinct !{!472, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!473 = distinct !{!473, !474, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!474 = distinct !{!474, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!475 = distinct !{!475, !474, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!476 = !{!460, !465, !359}
!477 = !{!478}
!478 = distinct !{!478, !479, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!479 = distinct !{!479, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!480 = !{!481, !460, !465, !359}
!481 = distinct !{!481, !479, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!482 = !{!483}
!483 = distinct !{!483, !484, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!484 = distinct !{!484, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!485 = !{!486}
!486 = distinct !{!486, !487, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!487 = distinct !{!487, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!488 = !{!489}
!489 = distinct !{!489, !490, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!490 = distinct !{!490, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!491 = !{!492, !489, !486, !483}
!492 = distinct !{!492, !493, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!493 = distinct !{!493, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!494 = !{!495, !496, !359}
!495 = distinct !{!495, !487, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!496 = distinct !{!496, !484, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!497 = !{!489, !486, !483}
!498 = !{!489, !486, !483, !359}
!499 = !{!500}
!500 = distinct !{!500, !501, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!501 = distinct !{!501, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!502 = !{!500, !359}
!503 = !{!504}
!504 = distinct !{!504, !505, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5title11AsTitleCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!505 = distinct !{!505, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5title11AsTitleCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!506 = !{!507}
!507 = distinct !{!507, !508, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5titleINtBT_11AsTitleCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!508 = distinct !{!508, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5titleINtBT_11AsTitleCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!509 = !{!510, !512, !514, !516, !517, !519, !520, !522, !523, !525, !526, !504}
!510 = distinct !{!510, !511, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!511 = distinct !{!511, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!512 = distinct !{!512, !513, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!513 = distinct !{!513, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!514 = distinct !{!514, !515, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5titleINtB22_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!515 = distinct !{!515, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5titleINtB22_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!516 = distinct !{!516, !515, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5titleINtB22_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!517 = distinct !{!517, !518, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5titleINtB1W_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!518 = distinct !{!518, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5titleINtB1W_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!519 = distinct !{!519, !518, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5titleINtB1W_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!520 = distinct !{!520, !521, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5titleINtB24_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!521 = distinct !{!521, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5titleINtB24_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!522 = distinct !{!522, !521, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5titleINtB24_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!523 = distinct !{!523, !524, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs_NtB11_5titleINtB1T_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!524 = distinct !{!524, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs_NtB11_5titleINtB1T_11AsTitleCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!525 = distinct !{!525, !508, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5titleINtBT_11AsTitleCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!526 = distinct !{!526, !527, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5titleINtB4_11AsTitleCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_: %f"}
!527 = distinct !{!527, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5titleINtB4_11AsTitleCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_"}
!528 = !{!529, !531, !525, !526, !504}
!529 = distinct !{!529, !530, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!530 = distinct !{!530, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!531 = distinct !{!531, !532, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!532 = distinct !{!532, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!533 = !{!534, !536, !538, !540, !542, !525, !526, !504}
!534 = distinct !{!534, !535, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!535 = distinct !{!535, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!536 = distinct !{!536, !537, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!537 = distinct !{!537, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!538 = distinct !{!538, !539, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!539 = distinct !{!539, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!540 = distinct !{!540, !541, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!541 = distinct !{!541, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!542 = distinct !{!542, !541, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!543 = !{!544}
!544 = distinct !{!544, !545, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!545 = distinct !{!545, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!546 = !{!547}
!547 = distinct !{!547, !548, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!548 = distinct !{!548, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!549 = !{!550}
!550 = distinct !{!550, !551, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!551 = distinct !{!551, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!552 = !{!553, !550, !547, !544}
!553 = distinct !{!553, !554, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!554 = distinct !{!554, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!555 = !{!556, !557, !504}
!556 = distinct !{!556, !548, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!557 = distinct !{!557, !545, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!558 = !{!550, !547, !544}
!559 = !{!550, !547, !544, !504}
!560 = !{!561, !507}
!561 = distinct !{!561, !562, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!562 = distinct !{!562, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!563 = !{!525, !526, !504}
!564 = !{!565}
!565 = distinct !{!565, !566, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!566 = distinct !{!566, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!567 = !{!568}
!568 = distinct !{!568, !569, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!569 = distinct !{!569, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!570 = !{!571}
!571 = distinct !{!571, !572, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!572 = distinct !{!572, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!573 = !{!574, !571, !568, !565}
!574 = distinct !{!574, !575, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!575 = distinct !{!575, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!576 = !{!577, !578, !504}
!577 = distinct !{!577, !569, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!578 = distinct !{!578, !566, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!579 = !{!571, !568, !565}
!580 = !{!571, !568, !565, !504}
!581 = !{!582, !507}
!582 = distinct !{!582, !583, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!583 = distinct !{!583, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!584 = !{!585}
!585 = distinct !{!585, !586, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!586 = distinct !{!586, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!587 = !{!588, !590, !592, !504}
!588 = distinct !{!588, !589, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!589 = distinct !{!589, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!590 = distinct !{!590, !591, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!591 = distinct !{!591, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!592 = distinct !{!592, !586, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!593 = !{!585, !592, !504}
!594 = !{!595, !597, !592, !504}
!595 = distinct !{!595, !596, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!596 = distinct !{!596, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!597 = distinct !{!597, !598, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!598 = distinct !{!598, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!599 = !{!600, !585}
!600 = distinct !{!600, !601, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!601 = distinct !{!601, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!602 = !{!592, !504}
!603 = !{!604}
!604 = distinct !{!604, !605, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!605 = distinct !{!605, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!606 = !{!607}
!607 = distinct !{!607, !608, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!608 = distinct !{!608, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!609 = !{!610}
!610 = distinct !{!610, !611, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!611 = distinct !{!611, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!612 = !{!613, !610, !607, !604}
!613 = distinct !{!613, !614, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!614 = distinct !{!614, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!615 = !{!616, !617, !504}
!616 = distinct !{!616, !608, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!617 = distinct !{!617, !605, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!618 = !{!610, !607, !604}
!619 = !{!610, !607, !604, !504}
!620 = !{!621, !507}
!621 = distinct !{!621, !622, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!622 = distinct !{!622, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!623 = !{!624}
!624 = distinct !{!624, !625, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!625 = distinct !{!625, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!626 = !{!627, !629, !631, !504}
!627 = distinct !{!627, !628, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!628 = distinct !{!628, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!629 = distinct !{!629, !630, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!630 = distinct !{!630, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!631 = distinct !{!631, !625, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!632 = !{!624, !631, !504}
!633 = !{!634, !636, !631, !504}
!634 = distinct !{!634, !635, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!635 = distinct !{!635, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!636 = distinct !{!636, !637, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!637 = distinct !{!637, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!638 = !{!639, !624}
!639 = distinct !{!639, !640, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!640 = distinct !{!640, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!641 = !{!631, !504}
!642 = !{!643}
!643 = distinct !{!643, !644, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %s.0"}
!644 = distinct !{!644, !"_RNvCs2f1hMIlW3Va_4heck9lowercase"}
!645 = !{!646, !648, !504}
!646 = distinct !{!646, !647, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!647 = distinct !{!647, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!648 = distinct !{!648, !644, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %f"}
!649 = !{!650, !652, !654, !656, !658, !648, !504}
!650 = distinct !{!650, !651, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!651 = distinct !{!651, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!652 = distinct !{!652, !653, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!653 = distinct !{!653, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!654 = distinct !{!654, !655, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!655 = distinct !{!655, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!656 = distinct !{!656, !657, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!657 = distinct !{!657, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!658 = distinct !{!658, !657, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!659 = !{!643, !648, !504}
!660 = !{!661}
!661 = distinct !{!661, !662, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!662 = distinct !{!662, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!663 = !{!664, !643, !648, !504}
!664 = distinct !{!664, !662, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!665 = !{!666}
!666 = distinct !{!666, !667, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!667 = distinct !{!667, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!668 = !{!669}
!669 = distinct !{!669, !670, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!670 = distinct !{!670, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!671 = !{!672}
!672 = distinct !{!672, !673, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!673 = distinct !{!673, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!674 = !{!675, !672, !669, !666}
!675 = distinct !{!675, !676, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!676 = distinct !{!676, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!677 = !{!678, !679}
!678 = distinct !{!678, !670, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!679 = distinct !{!679, !667, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!680 = !{!672, !669, !666}
!681 = !{!682}
!682 = distinct !{!682, !683, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!683 = distinct !{!683, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!684 = !{!682, !504}
!685 = !{!686}
!686 = distinct !{!686, !687, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5train11AsTrainCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!687 = distinct !{!687, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5train11AsTrainCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!688 = !{!689}
!689 = distinct !{!689, !690, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5trainINtBT_11AsTrainCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!690 = distinct !{!690, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5trainINtBT_11AsTrainCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!691 = !{!692, !694, !696, !698, !699, !701, !702, !704, !705, !707, !708, !686}
!692 = distinct !{!692, !693, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!693 = distinct !{!693, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!694 = distinct !{!694, !695, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!695 = distinct !{!695, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!696 = distinct !{!696, !697, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5trainINtB22_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!697 = distinct !{!697, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5trainINtB22_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!698 = distinct !{!698, !697, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_10capitalizeNCNvXs_NtB1a_5trainINtB22_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!699 = distinct !{!699, !700, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5trainINtB1W_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!700 = distinct !{!700, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5trainINtB1W_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!701 = distinct !{!701, !700, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_10capitalizeNCNvXs_NtB14_5trainINtB1W_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!702 = distinct !{!702, !703, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5trainINtB24_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!703 = distinct !{!703, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5trainINtB24_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!704 = distinct !{!704, !703, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_10capitalizeNCNvXs_NtB1c_5trainINtB24_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!705 = distinct !{!705, !706, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs_NtB11_5trainINtB1T_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!706 = distinct !{!706, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_10capitalizeNCNvXs_NtB11_5trainINtB1T_11AsTrainCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!707 = distinct !{!707, !690, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_10capitalizeNCNvXs_NtB2_5trainINtBT_11AsTrainCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!708 = distinct !{!708, !709, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5trainINtB4_11AsTrainCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_: %f"}
!709 = distinct !{!709, !"_RNvXs_NtCs2f1hMIlW3Va_4heck5trainINtB4_11AsTrainCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB6_"}
!710 = !{!711, !713, !707, !708, !686}
!711 = distinct !{!711, !712, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!712 = distinct !{!712, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!713 = distinct !{!713, !714, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!714 = distinct !{!714, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!715 = !{!716, !718, !720, !722, !724, !707, !708, !686}
!716 = distinct !{!716, !717, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!717 = distinct !{!717, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!718 = distinct !{!718, !719, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!719 = distinct !{!719, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!720 = distinct !{!720, !721, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!721 = distinct !{!721, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!722 = distinct !{!722, !723, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!723 = distinct !{!723, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!724 = distinct !{!724, !723, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!725 = !{!726}
!726 = distinct !{!726, !727, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!727 = distinct !{!727, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!728 = !{!729}
!729 = distinct !{!729, !730, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!730 = distinct !{!730, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!731 = !{!732}
!732 = distinct !{!732, !733, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!733 = distinct !{!733, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!734 = !{!735, !732, !729, !726}
!735 = distinct !{!735, !736, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!736 = distinct !{!736, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!737 = !{!738, !739, !686}
!738 = distinct !{!738, !730, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!739 = distinct !{!739, !727, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!740 = !{!732, !729, !726}
!741 = !{!732, !729, !726, !686}
!742 = !{!743, !689}
!743 = distinct !{!743, !744, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!744 = distinct !{!744, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!745 = !{!707, !708, !686}
!746 = !{!747}
!747 = distinct !{!747, !748, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!748 = distinct !{!748, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!749 = !{!750}
!750 = distinct !{!750, !751, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!751 = distinct !{!751, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!752 = !{!753}
!753 = distinct !{!753, !754, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!754 = distinct !{!754, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!755 = !{!756, !753, !750, !747}
!756 = distinct !{!756, !757, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!757 = distinct !{!757, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!758 = !{!759, !760, !686}
!759 = distinct !{!759, !751, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!760 = distinct !{!760, !748, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!761 = !{!753, !750, !747}
!762 = !{!753, !750, !747, !686}
!763 = !{!764, !689}
!764 = distinct !{!764, !765, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!765 = distinct !{!765, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!766 = !{!767}
!767 = distinct !{!767, !768, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!768 = distinct !{!768, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!769 = !{!770, !772, !774, !686}
!770 = distinct !{!770, !771, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!771 = distinct !{!771, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!772 = distinct !{!772, !773, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!773 = distinct !{!773, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!774 = distinct !{!774, !768, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!775 = !{!767, !774, !686}
!776 = !{!777, !779, !774, !686}
!777 = distinct !{!777, !778, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!778 = distinct !{!778, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!779 = distinct !{!779, !780, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!780 = distinct !{!780, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!781 = !{!782, !767}
!782 = distinct !{!782, !783, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!783 = distinct !{!783, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!784 = !{!774, !686}
!785 = !{!786}
!786 = distinct !{!786, !787, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!787 = distinct !{!787, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!788 = !{!789}
!789 = distinct !{!789, !790, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!790 = distinct !{!790, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!791 = !{!792}
!792 = distinct !{!792, !793, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!793 = distinct !{!793, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!794 = !{!795, !792, !789, !786}
!795 = distinct !{!795, !796, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!796 = distinct !{!796, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!797 = !{!798, !799, !686}
!798 = distinct !{!798, !790, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!799 = distinct !{!799, !787, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!800 = !{!792, !789, !786}
!801 = !{!792, !789, !786, !686}
!802 = !{!803, !689}
!803 = distinct !{!803, !804, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!804 = distinct !{!804, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!805 = !{!806}
!806 = distinct !{!806, !807, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %s.0"}
!807 = distinct !{!807, !"_RNvCs2f1hMIlW3Va_4heck10capitalize"}
!808 = !{!809, !811, !813, !686}
!809 = distinct !{!809, !810, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!810 = distinct !{!810, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!811 = distinct !{!811, !812, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!812 = distinct !{!812, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!813 = distinct !{!813, !807, !"_RNvCs2f1hMIlW3Va_4heck10capitalize: %f"}
!814 = !{!806, !813, !686}
!815 = !{!816, !818, !813, !686}
!816 = distinct !{!816, !817, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!817 = distinct !{!817, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!818 = distinct !{!818, !819, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!819 = distinct !{!819, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!820 = !{!821, !806}
!821 = distinct !{!821, !822, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!822 = distinct !{!822, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!823 = !{!813, !686}
!824 = !{!825}
!825 = distinct !{!825, !826, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %s.0"}
!826 = distinct !{!826, !"_RNvCs2f1hMIlW3Va_4heck9lowercase"}
!827 = !{!828, !830, !686}
!828 = distinct !{!828, !829, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!829 = distinct !{!829, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!830 = distinct !{!830, !826, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %f"}
!831 = !{!832, !834, !836, !838, !840, !830, !686}
!832 = distinct !{!832, !833, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!833 = distinct !{!833, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!834 = distinct !{!834, !835, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!835 = distinct !{!835, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!836 = distinct !{!836, !837, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!837 = distinct !{!837, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!838 = distinct !{!838, !839, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!839 = distinct !{!839, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!840 = distinct !{!840, !839, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!841 = !{!825, !830, !686}
!842 = !{!843}
!843 = distinct !{!843, !844, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!844 = distinct !{!844, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!845 = !{!846, !825, !830, !686}
!846 = distinct !{!846, !844, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!847 = !{!848}
!848 = distinct !{!848, !849, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!849 = distinct !{!849, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!850 = !{!851}
!851 = distinct !{!851, !852, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!852 = distinct !{!852, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!853 = !{!854}
!854 = distinct !{!854, !855, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!855 = distinct !{!855, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!856 = !{!857, !854, !851, !848}
!857 = distinct !{!857, !858, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!858 = distinct !{!858, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!859 = !{!860, !861}
!860 = distinct !{!860, !852, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!861 = distinct !{!861, !849, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!862 = !{!854, !851, !848}
!863 = !{!864}
!864 = distinct !{!864, !865, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!865 = distinct !{!865, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!866 = !{!864, !686}
!867 = !{!868}
!868 = distinct !{!868, !869, !"_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt: %self"}
!869 = distinct !{!869, !"_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt"}
!870 = !{!871, !873, !868}
!871 = distinct !{!871, !872, !"_RINvNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB8_15PolymorphicIterAINtNtNtBe_3mem12maybe_uninit11MaybeUninitpEpENtNtBe_5clone5Clone5clone14clone_into_newcECs2f1hMIlW3Va_4heck: %source.0"}
!872 = distinct !{!872, !"_RINvNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB8_15PolymorphicIterAINtNtNtBe_3mem12maybe_uninit11MaybeUninitpEpENtNtBe_5clone5Clone5clone14clone_into_newcECs2f1hMIlW3Va_4heck"}
!873 = distinct !{!873, !874, !"_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck: %self"}
!874 = distinct !{!874, !"_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck"}
!875 = !{!876, !877, !878}
!876 = distinct !{!876, !872, !"_RINvNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB8_15PolymorphicIterAINtNtNtBe_3mem12maybe_uninit11MaybeUninitpEpENtNtBe_5clone5Clone5clone14clone_into_newcECs2f1hMIlW3Va_4heck: %target.0"}
!877 = distinct !{!877, !874, !"_RNvXs3_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterAINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEj3_ENtNtBb_5clone5Clone5cloneCs2f1hMIlW3Va_4heck: %_0"}
!878 = distinct !{!878, !869, !"_RNvXsl_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIterNtNtB7_3fmt7Display3fmt: %f"}
!879 = !{!868, !878}
!880 = !{!878}
!881 = !{!882}
!882 = distinct !{!882, !883, !"_RNvMs6_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterSINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEE4nextCs2f1hMIlW3Va_4heck: %self.0"}
!883 = distinct !{!883, !"_RNvMs6_NtNtNtCsjMrxcFdYDNN_4core5array4iter10iter_innerINtB5_15PolymorphicIterSINtNtNtBb_3mem12maybe_uninit11MaybeUninitcEE4nextCs2f1hMIlW3Va_4heck"}
!884 = !{!885}
!885 = distinct !{!885, !886, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push: %self"}
!886 = distinct !{!886, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String4push"}
!887 = !{!888, !885}
!888 = distinct !{!888, !889, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!889 = distinct !{!889, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!890 = !{!891}
!891 = distinct !{!891, !892, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!892 = distinct !{!892, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!893 = !{!894}
!894 = distinct !{!894, !895, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!895 = distinct !{!895, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!896 = !{!897, !894, !891}
!897 = distinct !{!897, !898, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!898 = distinct !{!898, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!899 = !{!900}
!900 = distinct !{!900, !892, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!901 = !{!894, !891}
!902 = !{!903}
!903 = distinct !{!903, !904, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_snake17AsShoutySnakeCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!904 = distinct !{!904, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck12shouty_snake17AsShoutySnakeCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!905 = !{!906}
!906 = distinct !{!906, !907, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs0_NtB2_12shouty_snakeINtBS_17AsShoutySnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!907 = distinct !{!907, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs0_NtB2_12shouty_snakeINtBS_17AsShoutySnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!908 = !{!909, !911, !913, !915, !916, !918, !919, !921, !922, !924, !925, !903}
!909 = distinct !{!909, !910, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!910 = distinct !{!910, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!911 = distinct !{!911, !912, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!912 = distinct !{!912, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!913 = distinct !{!913, !914, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs0_NtB1a_12shouty_snakeINtB21_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!914 = distinct !{!914, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs0_NtB1a_12shouty_snakeINtB21_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!915 = distinct !{!915, !914, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9uppercaseNCNvXs0_NtB1a_12shouty_snakeINtB21_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!916 = distinct !{!916, !917, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs0_NtB14_12shouty_snakeINtB1V_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!917 = distinct !{!917, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs0_NtB14_12shouty_snakeINtB1V_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!918 = distinct !{!918, !917, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9uppercaseNCNvXs0_NtB14_12shouty_snakeINtB1V_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!919 = distinct !{!919, !920, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs0_NtB1c_12shouty_snakeINtB23_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!920 = distinct !{!920, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs0_NtB1c_12shouty_snakeINtB23_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!921 = distinct !{!921, !920, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9uppercaseNCNvXs0_NtB1c_12shouty_snakeINtB23_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!922 = distinct !{!922, !923, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9uppercaseNCNvXs0_NtB11_12shouty_snakeINtB1S_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!923 = distinct !{!923, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9uppercaseNCNvXs0_NtB11_12shouty_snakeINtB1S_17AsShoutySnakeCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!924 = distinct !{!924, !907, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9uppercaseNCNvXs0_NtB2_12shouty_snakeINtBS_17AsShoutySnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!925 = distinct !{!925, !926, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck12shouty_snakeINtB5_17AsShoutySnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_: %f"}
!926 = distinct !{!926, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck12shouty_snakeINtB5_17AsShoutySnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_"}
!927 = !{!928, !930, !924, !925, !903}
!928 = distinct !{!928, !929, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!929 = distinct !{!929, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!930 = distinct !{!930, !931, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!931 = distinct !{!931, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!932 = !{!933, !935, !937, !939, !941, !924, !925, !903}
!933 = distinct !{!933, !934, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!934 = distinct !{!934, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!935 = distinct !{!935, !936, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!936 = distinct !{!936, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!937 = distinct !{!937, !938, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!938 = distinct !{!938, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!939 = distinct !{!939, !940, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!940 = distinct !{!940, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!941 = distinct !{!941, !940, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!942 = !{!943}
!943 = distinct !{!943, !944, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!944 = distinct !{!944, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!945 = !{!946}
!946 = distinct !{!946, !947, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!947 = distinct !{!947, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!948 = !{!949}
!949 = distinct !{!949, !950, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!950 = distinct !{!950, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!951 = !{!952, !949, !946, !943}
!952 = distinct !{!952, !953, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!953 = distinct !{!953, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!954 = !{!955, !956, !903}
!955 = distinct !{!955, !947, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!956 = distinct !{!956, !944, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!957 = !{!949, !946, !943}
!958 = !{!949, !946, !943, !903}
!959 = !{!960, !906}
!960 = distinct !{!960, !961, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!961 = distinct !{!961, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!962 = !{!924, !925, !903}
!963 = !{!964}
!964 = distinct !{!964, !965, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!965 = distinct !{!965, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!966 = !{!967}
!967 = distinct !{!967, !968, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!968 = distinct !{!968, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!969 = !{!970}
!970 = distinct !{!970, !971, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!971 = distinct !{!971, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!972 = !{!973, !970, !967, !964}
!973 = distinct !{!973, !974, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!974 = distinct !{!974, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!975 = !{!976, !977, !903}
!976 = distinct !{!976, !968, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!977 = distinct !{!977, !965, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!978 = !{!970, !967, !964}
!979 = !{!970, !967, !964, !903}
!980 = !{!981, !906}
!981 = distinct !{!981, !982, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!982 = distinct !{!982, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!983 = !{!984}
!984 = distinct !{!984, !985, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!985 = distinct !{!985, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!986 = !{!987}
!987 = distinct !{!987, !988, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!988 = distinct !{!988, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!989 = !{!990}
!990 = distinct !{!990, !991, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!991 = distinct !{!991, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!992 = !{!993, !990, !987, !984}
!993 = distinct !{!993, !994, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!994 = distinct !{!994, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!995 = !{!996, !997, !903}
!996 = distinct !{!996, !988, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!997 = distinct !{!997, !985, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!998 = !{!990, !987, !984}
!999 = !{!990, !987, !984, !903}
!1000 = !{!1001, !906}
!1001 = distinct !{!1001, !1002, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1002 = distinct !{!1002, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1003 = !{!1004}
!1004 = distinct !{!1004, !1005, !"_RNvCs2f1hMIlW3Va_4heck9uppercase: %s.0"}
!1005 = distinct !{!1005, !"_RNvCs2f1hMIlW3Va_4heck9uppercase"}
!1006 = !{!1007, !1009, !903}
!1007 = distinct !{!1007, !1008, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1008 = distinct !{!1008, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1009 = distinct !{!1009, !1005, !"_RNvCs2f1hMIlW3Va_4heck9uppercase: %f"}
!1010 = !{!1004, !1009, !903}
!1011 = !{!1012}
!1012 = distinct !{!1012, !1013, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!1013 = distinct !{!1013, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!1014 = !{!1015, !1004, !1009, !903}
!1015 = distinct !{!1015, !1013, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!1016 = !{!1017}
!1017 = distinct !{!1017, !1018, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!1018 = distinct !{!1018, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!1019 = !{!1017, !903}
!1020 = !{!1021}
!1021 = distinct !{!1021, !1022, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5snake11AsSnakeCaseReENtB5_12SpecToString14spec_to_stringBD_: %_0"}
!1022 = distinct !{!1022, !"_RNvXsC_NtCsdJPVW0sQgAG_5alloc6stringINtNtCs2f1hMIlW3Va_4heck5snake11AsSnakeCaseReENtB5_12SpecToString14spec_to_stringBD_"}
!1023 = !{!1024}
!1024 = distinct !{!1024, !1025, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs0_NtB2_5snakeINtBS_11AsSnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %s.0"}
!1025 = distinct !{!1025, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs0_NtB2_5snakeINtBS_11AsSnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_"}
!1026 = !{!1027, !1029, !1031, !1033, !1034, !1036, !1037, !1039, !1040, !1042, !1043, !1021}
!1027 = distinct !{!1027, !1028, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1028 = distinct !{!1028, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1029 = distinct !{!1029, !1030, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!1030 = distinct !{!1030, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!1031 = distinct !{!1031, !1032, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs0_NtB1a_5snakeINtB21_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %_0"}
!1032 = distinct !{!1032, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs0_NtB1a_5snakeINtB21_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_"}
!1033 = distinct !{!1033, !1032, !"_RNvXs8_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1a_9lowercaseNCNvXs0_NtB1a_5snakeINtB21_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher4nextB1a_: %self"}
!1034 = distinct !{!1034, !1035, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs0_NtB14_5snakeINtB1V_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %_0"}
!1035 = distinct !{!1035, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs0_NtB14_5snakeINtB1V_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_"}
!1036 = distinct !{!1036, !1035, !"_RNvYINtNtNtCsjMrxcFdYDNN_4core3str7pattern19MultiCharEqSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB14_9lowercaseNCNvXs0_NtB14_5snakeINtB1V_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB14_: %self"}
!1037 = distinct !{!1037, !1038, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs0_NtB1c_5snakeINtB23_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %_0"}
!1038 = distinct !{!1038, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs0_NtB1c_5snakeINtB23_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_"}
!1039 = distinct !{!1039, !1038, !"_RNvXso_NtNtCsjMrxcFdYDNN_4core3str7patternINtB5_21CharPredicateSearcherNCINvCs2f1hMIlW3Va_4heck9transformNvB1c_9lowercaseNCNvXs0_NtB1c_5snakeINtB23_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0ENtB5_8Searcher10next_matchB1c_: %self"}
!1040 = distinct !{!1040, !1041, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9lowercaseNCNvXs0_NtB11_5snakeINtB1S_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_: %self"}
!1041 = distinct !{!1041, !"_RNvMsf_NtNtCsjMrxcFdYDNN_4core3str4iterINtB5_13SplitInternalNCINvCs2f1hMIlW3Va_4heck9transformNvB11_9lowercaseNCNvXs0_NtB11_5snakeINtB1S_11AsSnakeCaseReENtNtB9_3fmt7Display3fmt0E0E4nextB11_"}
!1042 = distinct !{!1042, !1025, !"_RINvCs2f1hMIlW3Va_4heck9transformNvB2_9lowercaseNCNvXs0_NtB2_5snakeINtBS_11AsSnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmt0EB2_: %f"}
!1043 = distinct !{!1043, !1044, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck5snakeINtB5_11AsSnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_: %f"}
!1044 = distinct !{!1044, !"_RNvXs0_NtCs2f1hMIlW3Va_4heck5snakeINtB5_11AsSnakeCaseReENtNtCsjMrxcFdYDNN_4core3fmt7Display3fmtB7_"}
!1045 = !{!1046, !1048, !1042, !1043, !1021}
!1046 = distinct !{!1046, !1047, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1047 = distinct !{!1047, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1048 = distinct !{!1048, !1049, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!1049 = distinct !{!1049, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!1050 = !{!1051, !1053, !1055, !1057, !1059, !1042, !1043, !1021}
!1051 = distinct !{!1051, !1052, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1052 = distinct !{!1052, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1053 = distinct !{!1053, !1054, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next: %self"}
!1054 = distinct !{!1054, !"_RNvXs3_NtNtCsjMrxcFdYDNN_4core3str4iterNtB5_11CharIndicesNtNtNtNtB9_4iter6traits8iterator8Iterator4next"}
!1055 = distinct !{!1055, !1056, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!1056 = distinct !{!1056, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter11CharIndicesE4peek0Cs2f1hMIlW3Va_4heck"}
!1057 = distinct !{!1057, !1058, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!1058 = distinct !{!1058, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck"}
!1059 = distinct !{!1059, !1058, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_TjcEEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1k_8PeekableNtNtNtB5_3str4iter11CharIndicesE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!1060 = !{!1061}
!1061 = distinct !{!1061, !1062, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!1062 = distinct !{!1062, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!1063 = !{!1064}
!1064 = distinct !{!1064, !1065, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!1065 = distinct !{!1065, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!1066 = !{!1067}
!1067 = distinct !{!1067, !1068, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!1068 = distinct !{!1068, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!1069 = !{!1070, !1067, !1064, !1061}
!1070 = distinct !{!1070, !1071, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!1071 = distinct !{!1071, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!1072 = !{!1073, !1074, !1021}
!1073 = distinct !{!1073, !1065, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!1074 = distinct !{!1074, !1062, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!1075 = !{!1067, !1064, !1061}
!1076 = !{!1067, !1064, !1061, !1021}
!1077 = !{!1078, !1024}
!1078 = distinct !{!1078, !1079, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1079 = distinct !{!1079, !"_RNvXs9_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range9RangeFromjEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1080 = !{!1042, !1043, !1021}
!1081 = !{!1082}
!1082 = distinct !{!1082, !1083, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!1083 = distinct !{!1083, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!1084 = !{!1085}
!1085 = distinct !{!1085, !1086, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!1086 = distinct !{!1086, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!1087 = !{!1088}
!1088 = distinct !{!1088, !1089, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!1089 = distinct !{!1089, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!1090 = !{!1091, !1088, !1085, !1082}
!1091 = distinct !{!1091, !1092, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!1092 = distinct !{!1092, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!1093 = !{!1094, !1095, !1021}
!1094 = distinct !{!1094, !1086, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!1095 = distinct !{!1095, !1083, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!1096 = !{!1088, !1085, !1082}
!1097 = !{!1088, !1085, !1082, !1021}
!1098 = !{!1099, !1024}
!1099 = distinct !{!1099, !1100, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1100 = distinct !{!1100, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1101 = !{!1102}
!1102 = distinct !{!1102, !1103, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!1103 = distinct !{!1103, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!1104 = !{!1105}
!1105 = distinct !{!1105, !1106, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!1106 = distinct !{!1106, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!1107 = !{!1108}
!1108 = distinct !{!1108, !1109, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!1109 = distinct !{!1109, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!1110 = !{!1111, !1108, !1105, !1102}
!1111 = distinct !{!1111, !1112, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!1112 = distinct !{!1112, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!1113 = !{!1114, !1115, !1021}
!1114 = distinct !{!1114, !1106, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!1115 = distinct !{!1115, !1103, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!1116 = !{!1108, !1105, !1102}
!1117 = !{!1108, !1105, !1102, !1021}
!1118 = !{!1119, !1024}
!1119 = distinct !{!1119, !1120, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get: %slice.0"}
!1120 = distinct !{!1120, !"_RNvXs5_NtNtCsjMrxcFdYDNN_4core3str6traitsINtNtNtB9_3ops5range5RangejEINtNtNtB9_5slice5index10SliceIndexeE3get"}
!1121 = !{!1122}
!1122 = distinct !{!1122, !1123, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %s.0"}
!1123 = distinct !{!1123, !"_RNvCs2f1hMIlW3Va_4heck9lowercase"}
!1124 = !{!1125, !1127, !1021}
!1125 = distinct !{!1125, !1126, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1126 = distinct !{!1126, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1127 = distinct !{!1127, !1123, !"_RNvCs2f1hMIlW3Va_4heck9lowercase: %f"}
!1128 = !{!1129, !1131, !1133, !1135, !1137, !1127, !1021}
!1129 = distinct !{!1129, !1130, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck: %bytes"}
!1130 = distinct !{!1130, !"_RINvNtNtCsjMrxcFdYDNN_4core3str11validations15next_code_pointINtNtNtB6_5slice4iter4IterhEECs2f1hMIlW3Va_4heck"}
!1131 = distinct !{!1131, !1132, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next: %self"}
!1132 = distinct !{!1132, !"_RNvXNtNtCsjMrxcFdYDNN_4core3str4iterNtB2_5CharsNtNtNtNtB6_4iter6traits8iterator8Iterator4next"}
!1133 = distinct !{!1133, !1134, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck: %_1"}
!1134 = distinct !{!1134, !"_RNCNvMs3_NtNtNtCsjMrxcFdYDNN_4core4iter8adapters8peekableINtB7_8PeekableNtNtNtBd_3str4iter5CharsE4peek0Cs2f1hMIlW3Va_4heck"}
!1135 = distinct !{!1135, !1136, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %self"}
!1136 = distinct !{!1136, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck"}
!1137 = distinct !{!1137, !1136, !"_RINvMNtCsjMrxcFdYDNN_4core6optionINtB3_6OptionIBw_cEE18get_or_insert_withNCNvMs3_NtNtNtB5_4iter8adapters8peekableINtB1h_8PeekableNtNtNtB5_3str4iter5CharsE4peek0ECs2f1hMIlW3Va_4heck: %f"}
!1138 = !{!1122, !1127, !1021}
!1139 = !{!1140}
!1140 = distinct !{!1140, !1141, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %chars"}
!1141 = distinct !{!1141, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new"}
!1142 = !{!1143, !1122, !1127, !1021}
!1143 = distinct !{!1143, !1141, !"_RNvMsd_NtCsjMrxcFdYDNN_4core4charNtB5_15CaseMappingIter3new: %_0"}
!1144 = !{!1145}
!1145 = distinct !{!1145, !1146, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %self"}
!1146 = distinct !{!1146, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str"}
!1147 = !{!1148}
!1148 = distinct !{!1148, !1149, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %self"}
!1149 = distinct !{!1149, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str"}
!1150 = !{!1151}
!1151 = distinct !{!1151, !1152, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck: %self"}
!1152 = distinct !{!1152, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE15append_elementsCs2f1hMIlW3Va_4heck"}
!1153 = !{!1154, !1151, !1148, !1145}
!1154 = distinct !{!1154, !1155, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck: %self"}
!1155 = distinct !{!1155, !"_RNvMs_NtCsdJPVW0sQgAG_5alloc3vecINtB4_3VechE7reserveCs2f1hMIlW3Va_4heck"}
!1156 = !{!1157, !1158, !1021}
!1157 = distinct !{!1157, !1149, !"_RNvMNtCsdJPVW0sQgAG_5alloc6stringNtB2_6String8push_str: %string.0"}
!1158 = distinct !{!1158, !1146, !"_RNvXsZ_NtCsdJPVW0sQgAG_5alloc6stringNtB5_6StringNtNtCsjMrxcFdYDNN_4core3fmt5Write9write_str: %s.0"}
!1159 = !{!1151, !1148, !1145}
!1160 = !{!1151, !1148, !1145, !1021}
!1161 = !{!1162}
!1162 = distinct !{!1162, !1163, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck: %_1"}
!1163 = distinct !{!1163, !"_RINvNtCsjMrxcFdYDNN_4core3ptr13drop_in_placeNtNtCsdJPVW0sQgAG_5alloc6string6StringECs2f1hMIlW3Va_4heck"}
!1164 = !{!1162, !1021}
